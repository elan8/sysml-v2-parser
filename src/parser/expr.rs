//! Expression and path parsing for values and bind/connect.

use crate::ast::{
    Argument, BinaryOperator, CollectionOperator, Expression, FeatureChain, Node, TypeCheckKind,
    UnaryOperator,
};
use crate::parser::lex::{name, qualified_name, starts_with_keyword, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use nom::Parser;

/// Numeric literal text: optional sign, mantissa, optional exponent (`5E9`, `195.3`, `6.022e23`).
fn numeric_literal_text(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = ws_and_comments(input)?;
    let frag = input.fragment();
    let mut i = 0usize;
    if matches!(frag.first(), Some(b'+' | b'-')) {
        i += 1;
    }
    let digit_start = i;
    while i < frag.len() && frag[i].is_ascii_digit() {
        i += 1;
    }
    if i == digit_start {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )));
    }
    if i < frag.len() && frag[i] == b'.' {
        i += 1;
        while i < frag.len() && frag[i].is_ascii_digit() {
            i += 1;
        }
    }
    if i < frag.len() && matches!(frag[i], b'e' | b'E') {
        i += 1;
        if i < frag.len() && matches!(frag[i], b'+' | b'-') {
            i += 1;
        }
        let exp_start = i;
        while i < frag.len() && frag[i].is_ascii_digit() {
            i += 1;
        }
        if i == exp_start {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Digit,
            )));
        }
    }
    let text = String::from_utf8_lossy(&frag[..i]).to_string();
    let (input, _) = nom::bytes::complete::take(i).parse(input)?;
    Ok((input, text))
}

fn classify_numeric_literal(text: &str) -> Expression {
    let normalized = text.trim();
    if normalized.contains('.') || normalized.chars().skip(1).any(|c| c == 'e' || c == 'E') {
        Expression::LiteralReal(normalized.to_string())
    } else {
        Expression::LiteralInteger(normalized.parse().unwrap_or(0))
    }
}

/// Integer literal.
fn literal_integer(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, text) = numeric_literal_text(input)?;
    if text.contains('.') || text.chars().skip(1).any(|c| c == 'e' || c == 'E') {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )));
    }
    Ok((
        input,
        node_from_to(start, input, classify_numeric_literal(&text)),
    ))
}

/// Real literal (decimal or scientific notation).
fn literal_real(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, text) = numeric_literal_text(input)?;
    if !text.contains('.') && !text.chars().skip(1).any(|c| c == 'e' || c == 'E') {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )));
    }
    Ok((
        input,
        node_from_to(start, input, classify_numeric_literal(&text)),
    ))
}

/// String literal: double-quoted.
fn literal_string(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"\""[..]).parse(input)?;
    let frag = input.fragment();
    let mut i = 0;
    while i < frag.len() {
        if frag[i] == b'\\' && i + 1 < frag.len() {
            i += 2;
            continue;
        }
        if frag[i] == b'"' {
            let s = String::from_utf8_lossy(&frag[..i]).replace("\\\"", "\"");
            let (input, _) = nom::bytes::complete::take(i + 1).parse(input)?;
            return Ok((
                input,
                node_from_to(start, input, Expression::LiteralString(s)),
            ));
        }
        i += 1;
    }
    let s = String::from_utf8_lossy(frag).replace("\\\"", "\"");
    let (input, _) = nom::bytes::complete::take(frag.len()).parse(input)?;
    Ok((
        input,
        node_from_to(start, input, Expression::LiteralString(s)),
    ))
}

/// Boolean literal: true | false.
fn literal_boolean(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, v) = alt((
        map(tag(&b"true"[..]), |_| true),
        map(tag(&b"false"[..]), |_| false),
    ))
    .parse(input)?;
    Ok((
        input,
        node_from_to(start, input, Expression::LiteralBoolean(v)),
    ))
}

/// Feature reference: name or qualified name.
fn feature_ref_primary(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, n) = qualified_name(input)?;
    Ok((input, node_from_to(start, input, Expression::FeatureRef(n))))
}

/// Metadata reference: @ qualified_name (e.g. @Safety, @Security for filter expressions).
fn metadata_ref_primary(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"@"[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, n) = qualified_name(input)?;
    Ok((
        input,
        node_from_to(start, input, Expression::Classification { metaclass: n }),
    ))
}

/// A single `ArgumentList` entry: positional (`ArgumentValue`) or named
/// (`NAME '=' ArgumentValue`, KerML `NamedArgument`). Only treats `NAME '='` as named when the
/// `=` is a lone assignment token, not the start of `==`/`===`, so equality expressions like
/// `a == b` are never misread as a named argument.
fn argument(input: Input<'_>) -> IResult<Input<'_>, Argument> {
    let (input, _) = ws_and_comments(input)?;
    if let Ok((after_name, arg_name)) = name(input) {
        let (after_ws, _) = ws_and_comments(after_name)?;
        let frag = after_ws.fragment();
        if frag.first() == Some(&b'=') && frag.get(1) != Some(&b'=') {
            let (after_eq, _) = tag(&b"="[..]).parse(after_ws)?;
            let (after_eq, _) = ws_and_comments(after_eq)?;
            let (rest, value) = expression(after_eq)?;
            return Ok((
                rest,
                Argument {
                    name: Some(arg_name),
                    value,
                },
            ));
        }
    }
    let (rest, value) = expression(input)?;
    Ok((rest, Argument { name: None, value }))
}

/// Parses the interior of an `ArgumentList` (`PositionalArgumentList | NamedArgumentList`) after
/// the opening `(` has already been consumed, through and including the closing `)`.
fn argument_list_tail(input: Input<'_>) -> IResult<Input<'_>, Vec<Argument>> {
    let (next, _) = ws_and_comments(input)?;
    if next.fragment().starts_with(b")") {
        let (next, _) = tag(&b")"[..]).parse(next)?;
        return Ok((next, Vec::new()));
    }
    let mut args = Vec::new();
    let mut input = next;
    loop {
        let (next, arg) = argument(input)?;
        args.push(arg);
        let (next, _) = ws_and_comments(next)?;
        if next.fragment().starts_with(b")") {
            let (next, _) = tag(&b")"[..]).parse(next)?;
            return Ok((next, args));
        }
        let (next, _) = tag(&b","[..]).parse(next)?;
        let (next, _) = ws_and_comments(next)?;
        input = next;
    }
}

fn constructor_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"new"[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, type_name) = qualified_name(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, args) = if input.fragment().starts_with(b"(") {
        let (input, _) = tag(&b"("[..]).parse(input)?;
        argument_list_tail(input)?
    } else {
        (input, Vec::new())
    };
    let current = node_from_to(start, input, Expression::Constructor { type_name, args });
    postfix(input, start, current)
}

/// Literal only (no unit): integer, real, string, boolean.
fn literal_only(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        literal_boolean,
        literal_real,
        literal_integer,
        literal_string,
    ))
    .parse(input)
}

fn quoted_unit_string(input: Input<'_>) -> IResult<Input<'_>, String> {
    let quote = *input.fragment().first().ok_or_else(|| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag))
    })?;
    if quote != b'\'' && quote != b'"' {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, _) = nom::bytes::complete::take(1usize).parse(input)?;
    let frag = input.fragment();
    let mut i = 0usize;
    while i < frag.len() {
        if frag[i] == quote {
            let s = String::from_utf8_lossy(&frag[..i]).to_string();
            let (input, _) = nom::bytes::complete::take(i + 1).parse(input)?;
            return Ok((input, s));
        }
        if frag[i] == b'\\' && i + 1 < frag.len() {
            i += 2;
            continue;
        }
        i += 1;
    }
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Tag,
    )))
}

/// Unit text inside `[` … `]` (e.g. `kg`, `m/s`, `'$'`).
fn unit_name_in_brackets(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = ws_and_comments(input)?;
    if matches!(input.fragment().first(), Some(b'"' | b'\'')) {
        return quoted_unit_string(input);
    }
    let frag = input.fragment();
    let mut i = 0usize;
    while i < frag.len() {
        let c = frag[i];
        if c == b']' {
            break;
        }
        if c.is_ascii_whitespace() {
            break;
        }
        if c.is_ascii_alphanumeric() || matches!(c, b'_' | b'/' | b'-' | b'^' | b'.' | b'*' | b':')
        {
            i += 1;
            continue;
        }
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::AlphaNumeric,
        )));
    }
    if i == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::AlphaNumeric,
        )));
    }
    let s = String::from_utf8_lossy(&frag[..i]).trim().to_string();
    let (input, _) = nom::bytes::complete::take(i).parse(input)?;
    Ok((input, s))
}

/// Literal with optional [ unit ]: 1750 [kg] -> LiteralWithUnit(...).
fn literal_with_unit(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, value_node) = literal_only(input)?;
    let (input, _) = ws_and_comments(input)?;
    if !input.fragment().starts_with(b"[") {
        return Ok((input, value_node));
    }
    let (input, _) = tag(&b"["[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let unit_start = input;
    let (input, unit_name) = unit_name_in_brackets.parse(input)?;
    let unit_name_span = crate::parser::span_from_to(unit_start, input);
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"]"[..]).parse(input)?;
    let unit = Node::new(
        unit_name_span.clone(),
        Expression::Bracket(Box::new(Node::new(
            unit_name_span,
            Expression::FeatureRef(unit_name),
        ))),
    );
    let expr = Expression::LiteralWithUnit {
        value: Box::new(value_node),
        unit: Box::new(unit),
    };
    Ok((input, node_from_to(start, input, expr)))
}

/// Parenthesized expression: `( expression )` for grouping, or `( e1, e2, ... )` as [`Expression::Tuple`].
fn parenthesized(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"("[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, first) = expression(input)?;
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b")") {
        let (input, _) = tag(&b")"[..]).parse(input)?;
        // Include `(` … `)` in the span so consumers (e.g. Spec42 `text_from_span`) round-trip
        // the full parenthesized source, not only the inner expression. Wrap in `Parenthesized`
        // (PAR-005 item 6) so the fact source had explicit grouping parens survives parsing
        // instead of being lost when only the inner expression's span was recomputed.
        return Ok((
            input,
            node_from_to(start, input, Expression::Parenthesized(Box::new(first))),
        ));
    }
    let (input, _) = tag(&b","[..]).parse(input)?;
    let mut elements = vec![first];
    let mut input = input;
    loop {
        let (next, _) = ws_and_comments(input)?;
        if next.fragment().starts_with(b")") {
            let (input, _) = tag(&b")"[..]).parse(next)?;
            return Ok((
                input,
                node_from_to(start, input, Expression::Tuple(elements)),
            ));
        }
        let (next, expr) = expression(next)?;
        elements.push(expr);
        let (next, _) = ws_and_comments(next)?;
        if next.fragment().starts_with(b")") {
            let (input, _) = tag(&b")"[..]).parse(next)?;
            return Ok((
                input,
                node_from_to(start, input, Expression::Tuple(elements)),
            ));
        }
        if next.fragment().starts_with(b",") {
            let (next, _) = tag(&b","[..]).parse(next)?;
            input = next;
            continue;
        }
        return Err(nom::Err::Error(nom::error::Error::new(
            next,
            nom::error::ErrorKind::Tag,
        )));
    }
}

/// KerML null or empty sequence ().
fn null_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = alt((
        map(tag(&b"null"[..]), |_| ()),
        map(
            delimited(tag(&b"("[..]), ws_and_comments, tag(&b")"[..])),
            |_| (),
        ),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, Expression::Null)))
}

/// SelectExpression: base `.?` selector
fn select_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, base) = feature_ref_primary(input)?;
    let (input, _) = tag(&b".?"[..]).parse(input)?;
    let (input, selector) = preceded(ws_and_comments, name).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Expression::Select {
                base: Box::new(base),
                selector,
            },
        ),
    ))
}

/// CollectExpression: base `.**` selector
fn collect_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, base) = feature_ref_primary(input)?;
    let (input, _) = tag(&b".**"[..]).parse(input)?;
    let (input, selector) = preceded(ws_and_comments, name).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Expression::Collect {
                base: Box::new(base),
                selector,
            },
        ),
    ))
}

/// SequenceExpression: `(` expr (`,` expr)* `)`
fn sequence_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, _) = tag(&b"("[..]).parse(input)?;
    let (input, first) = preceded(ws_and_comments, expression).parse(input)?;
    let (input, rest) = nom::multi::many0(preceded(
        preceded(ws_and_comments, tag(&b","[..])),
        preceded(ws_and_comments, expression),
    ))
    .parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b")"[..])).parse(input)?;
    let mut exprs = vec![first];
    exprs.extend(rest);
    let args = exprs
        .into_iter()
        .map(|value| Argument { name: None, value })
        .collect();
    Ok((
        input,
        node_from_to(
            start,
            input,
            Expression::Invocation {
                callee: Box::new(node_from_to(start, start, Expression::Null)),
                args,
            },
        ),
    ))
}

/// KerML type test suffix: `istype Type`, `hastype Type`, or `as Type`.
fn type_check_kind_token(input: Input<'_>) -> IResult<Input<'_>, TypeCheckKind> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b"istype"[..]), |_| TypeCheckKind::Istype),
        map(tag(&b"hastype"[..]), |_| TypeCheckKind::Hastype),
        map(tag(&b"as"[..]), |_| TypeCheckKind::As),
    ))
    .parse(input)
}

fn type_check_primary(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, kind) = type_check_kind_token(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, type_name) = qualified_name(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Expression::TypeCheck {
                kind,
                operand: None,
                type_name,
            },
        ),
    ))
}

/// Primary expression: literal with unit, literal only, metadata ref, feature ref, null, or parenthesized.
fn primary(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        literal_with_unit,
        literal_only,
        null_expression,
        metadata_ref_primary,
        type_check_primary,
        constructor_expression,
        collect_expression,
        select_expression,
        feature_ref_primary,
        parenthesized,
        sequence_expression,
    ))
    .parse(input)
}

/// Apply postfix #( expr ), . name, or :: name (qualified member access) to an expression.
fn postfix<'a>(
    input: Input<'a>,
    start: Input<'a>,
    current: Node<Expression>,
) -> IResult<Input<'a>, Node<Expression>> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b"(") {
        let (input, _) = tag(&b"("[..]).parse(input)?;
        let (input, args) = argument_list_tail(input)?;
        let expr = Expression::Invocation {
            callee: Box::new(current),
            args,
        };
        return postfix(input, start, node_from_to(start, input, expr));
    }
    if input.fragment().starts_with(b"#") {
        let (input, _) = tag(&b"#"[..]).parse(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b"("[..])).parse(input)?;
        let (input, index_node) = preceded(ws_and_comments, expression).parse(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b")"[..])).parse(input)?;
        let expr = Expression::Index {
            base: Box::new(current),
            index: Box::new(index_node),
        };
        return postfix(input, start, node_from_to(start, input, expr));
    }
    if input.fragment().starts_with(b"::") {
        let (input, _) = tag(&b"::"[..]).parse(input)?;
        let (input, _) = ws_and_comments(input)?;
        let (input, member) = name(input)?;
        let expr = Expression::MemberAccess(Box::new(current), member);
        return postfix(input, start, node_from_to(start, input, expr));
    }
    if input.fragment().starts_with(b".") {
        let (input, _) = tag(&b"."[..]).parse(input)?;
        let (input, _) = ws_and_comments(input)?;
        let (input, member) = name(input)?;
        // `expr.metadata` is a dedicated KerML production (MetadataAccessExpression, BNF
        // 8.2.5.8.3: `ElementReferenceMember '.' 'metadata'`), distinct from ordinary member
        // access -- PAR-005 item 4.
        let expr = if member == "metadata" {
            Expression::MetadataAccess(Box::new(current))
        } else {
            Expression::MemberAccess(Box::new(current), member)
        };
        return postfix(input, start, node_from_to(start, input, expr));
    }
    if input.fragment().starts_with(b"->") {
        let (input, _) = tag(&b"->"[..]).parse(input)?;
        let (input, _) = ws_and_comments(input)?;
        let (input, member) = name(input)?;
        let (after_name, _) = ws_and_comments(input)?;
        // KerML arrow-invocation, e.g. `collection->size()`, `xs->select(p)`, `xs->collect(f)`.
        // When followed by a call, capture it as a dedicated `CollectionOp` (PAR-005 item 2) so
        // the specific operator survives without string-matching a generic Invocation's callee.
        if after_name.fragment().starts_with(b"(") {
            let (after_paren, _) = tag(&b"("[..]).parse(after_name)?;
            let (after_args, args) = argument_list_tail(after_paren)?;
            let expr = Expression::CollectionOp {
                op: CollectionOperator::from_name(&member),
                base: Box::new(current),
                args,
            };
            return postfix(after_args, start, node_from_to(start, after_args, expr));
        }
        // Bare arrow access with no call (rare) -- fall back to plain member access.
        let expr = Expression::MemberAccess(Box::new(current), member);
        return postfix(input, start, node_from_to(start, input, expr));
    }
    if let Ok((after_kind, kind)) = type_check_kind_token(input) {
        if let Ok((after_type, type_name)) = qualified_name(after_kind) {
            let expr = node_from_to(
                start,
                after_type,
                Expression::TypeCheck {
                    kind,
                    operand: Some(Box::new(current)),
                    type_name,
                },
            );
            return postfix(after_type, start, expr);
        }
    }
    if starts_with_keyword(input.fragment(), b"meta") {
        let (input, _) = tag(&b"meta"[..]).parse(input)?;
        let (input, _) = ws_and_comments(input)?;
        let (input, metaclass) = qualified_name(input)?;
        let expr = Expression::MetaCast {
            base: Box::new(current),
            metaclass,
        };
        return postfix(input, start, node_from_to(start, input, expr));
    }
    Ok((input, current))
}

fn logical_op_token(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b"and"[..]), |_| "&&".to_string()),
        map(tag(&b"or"[..]), |_| "||".to_string()),
        map(tag(&b"xor"[..]), |_| "xor".to_string()),
        map(tag(&b"&&"[..]), |_| "&&".to_string()),
        map(tag(&b"||"[..]), |_| "||".to_string()),
    ))
    .parse(input)
}

/// Implication: lower precedence than `or` / `and` (constraint and filter bodies).
fn implies_op_token(input: Input<'_>) -> IResult<Input<'_>, String> {
    preceded(ws_and_comments, tag(&b"implies"[..]))
        .map(|_| "implies".to_string())
        .parse(input)
}

fn equality_op_token(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b"==="[..]), |_| "===".to_string()),
        map(tag(&b"!=="[..]), |_| "!==".to_string()),
        map(tag(&b"=="[..]), |_| "==".to_string()),
        map(tag(&b"!="[..]), |_| "!=".to_string()),
    ))
    .parse(input)
}

fn comparison_op_token(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b">="[..]), |_| ">=".to_string()),
        map(tag(&b"<="[..]), |_| "<=".to_string()),
        map(tag(&b">"[..]), |_| ">".to_string()),
        map(tag(&b"<"[..]), |_| "<".to_string()),
        map(tag(&b".."[..]), |_| "..".to_string()),
    ))
    .parse(input)
}

fn additive_op_token(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b"+"[..]), |_| "+".to_string()),
        map(tag(&b"-"[..]), |_| "-".to_string()),
        map(tag(&b"|"[..]), |_| "|".to_string()),
        map(tag(&b"&"[..]), |_| "&".to_string()),
    ))
    .parse(input)
}

fn multiplicative_op_token(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b"**"[..]), |_| "**".to_string()),
        map(tag(&b"*"[..]), |_| "*".to_string()),
        map(tag(&b"/"[..]), |_| "/".to_string()),
        map(tag(&b"%"[..]), |_| "%".to_string()),
        map(tag(&b"^"[..]), |_| "^".to_string()),
    ))
    .parse(input)
}

fn binary_chain_with<'a, P, N>(
    mut input: Input<'a>,
    start: Input<'a>,
    mut left: Node<Expression>,
    mut op_parser: P,
    mut next_parser: N,
) -> IResult<Input<'a>, Node<Expression>>
where
    P: Parser<Input<'a>, Output = String, Error = nom::error::Error<Input<'a>>>,
    N: Parser<Input<'a>, Output = Node<Expression>, Error = nom::error::Error<Input<'a>>>,
{
    loop {
        let Ok((next_input, op)) = op_parser.parse(input) else {
            return Ok((input, left));
        };
        let (next_input, right) = next_parser.parse(next_input)?;
        left = node_from_to(
            start,
            next_input,
            Expression::BinaryOp {
                op: BinaryOperator::from_token(&op),
                left: Box::new(left),
                right: Box::new(right),
            },
        );
        input = next_input;
    }
}

/// Unary operator token: + - ~ not (KerML UnaryOperator).
fn unary_op_token(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b"not"[..]), |_| "not".to_string()),
        map(tag(&b"~"[..]), |_| "~".to_string()),
        map(tag(&b"+"[..]), |_| "+".to_string()),
        map(tag(&b"-"[..]), |_| "-".to_string()),
    ))
    .parse(input)
}

/// Parse unary prefixes then primary; build nested UnaryOp from the right.
fn unary_and_primary(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, prefixes) = nom::multi::many0(unary_op_token).parse(input)?;
    let primary_start = input;
    let (input, primary_node) = primary(input)?;
    let (input, after_postfix) = postfix(input, primary_start, primary_node)?;
    let mut expr = after_postfix;
    for op in prefixes.into_iter().rev() {
        expr = node_from_to(
            start,
            input,
            Expression::UnaryOp {
                op: UnaryOperator::from_token(&op),
                operand: Box::new(expr),
            },
        );
    }
    Ok((input, expr))
}

fn multiplicative_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, left) = unary_and_primary(input)?;
    binary_chain_with(
        input,
        start,
        left,
        multiplicative_op_token,
        unary_and_primary,
    )
}

fn additive_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, left) = multiplicative_expression(input)?;
    binary_chain_with(
        input,
        start,
        left,
        additive_op_token,
        multiplicative_expression,
    )
}

fn comparison_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, left) = additive_expression(input)?;
    binary_chain_with(input, start, left, comparison_op_token, additive_expression)
}

fn equality_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, left) = comparison_expression(input)?;
    binary_chain_with(input, start, left, equality_op_token, comparison_expression)
}

fn logical_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, left) = equality_expression(input)?;
    binary_chain_with(input, start, left, logical_op_token, equality_expression)
}

/// Full expression with precedence-aware binary parsing.
pub(crate) fn expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, left) = logical_expression(input)?;
    binary_chain_with(input, start, left, implies_op_token, logical_expression)
}

/// Path expression: qualified name and/or member access (for bind/connect).
/// Supports `A`, `A::B::C`, `A.B.C`, and combinations like `A::B.C`.
///
/// A single segment (no `.` chain) stays [`Expression::FeatureRef`]. A genuine multi-segment
/// dotted chain (`A.B.C`, or `A::B.C`) is captured as [`Expression::FeatureChainRef`] using the
/// standalone [`FeatureChain`] type built for exactly this by PAR-004 item 6 (PAR-005 item 3) --
/// the first segment carries the full leading qualified name (which may itself contain `::`),
/// and each subsequent `.`-separated segment is a plain feature name.
pub(crate) fn path_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    // `qualified_name` covers `::`-separated chains (common in SysML examples for feature chains).
    let (input, first) = crate::parser::lex::qualified_name(input)?;
    let mut segments = vec![first];
    let mut rest = input;
    loop {
        let (next, _) = ws_and_comments(rest)?;
        if !next.fragment().starts_with(b".") {
            break;
        }
        let (next, _) = tag(&b"."[..]).parse(next)?;
        let (next, _) = ws_and_comments(next)?;
        let (next, member) = name(next)?;
        segments.push(member);
        rest = next;
    }
    let span = crate::parser::span_from_to(start, rest);
    let expr = if segments.len() == 1 {
        Expression::FeatureRef(segments.remove(0))
    } else {
        Expression::FeatureChainRef(FeatureChain { segments, span })
    };
    Ok((rest, node_from_to(start, rest, expr)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn span_input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    #[test]
    fn expression_parses_implies_lower_than_or() {
        let input = span_input("a or b implies c");
        let (_, node) = expression(input).expect("expression");
        match &node.value {
            Expression::BinaryOp { op, left, right } => {
                assert_eq!(op, &BinaryOperator::Implies);
                match &left.value {
                    Expression::BinaryOp { op, .. } => assert_eq!(op, &BinaryOperator::Or),
                    other => panic!("expected or on lhs, got {other:?}"),
                }
                assert!(matches!(&right.value, Expression::FeatureRef(s) if s == "c"));
            }
            other => panic!("expected implies, got {other:?}"),
        }
    }

    #[test]
    fn expression_parses_arrow_invocation_as_collection_op() {
        let input = span_input("powerProfile->size()");
        let (_, node) = expression(input).expect("expression");
        match &node.value {
            Expression::CollectionOp { op, base, args } => {
                assert_eq!(op, &CollectionOperator::Size);
                assert!(args.is_empty());
                assert!(matches!(&base.value, Expression::FeatureRef(s) if s == "powerProfile"));
            }
            other => panic!("expected CollectionOp, got {other:?}"),
        }
    }

    #[test]
    fn expression_parses_chained_arrow_invocation_with_lower_precedence_minus() {
        let input = span_input("a->b->c()-1");
        let (_, node) = expression(input).expect("expression");
        match &node.value {
            Expression::BinaryOp { op, left, right } => {
                assert_eq!(op, &BinaryOperator::Sub);
                assert!(matches!(&right.value, Expression::LiteralInteger(1)));
                match &left.value {
                    Expression::CollectionOp { op, base, args } => {
                        assert_eq!(op, &CollectionOperator::Other("c".to_string()));
                        assert!(args.is_empty());
                        match &base.value {
                            Expression::MemberAccess(inner_base, inner_member) => {
                                assert_eq!(inner_member, "b");
                                assert!(matches!(&inner_base.value, Expression::FeatureRef(s) if s == "a"));
                            }
                            other => panic!("expected MemberAccess base (bare arrow access), got {other:?}"),
                        }
                    }
                    other => panic!("expected CollectionOp on lhs of subtraction, got {other:?}"),
                }
            }
            other => panic!("expected BinaryOp subtract, got {other:?}"),
        }
    }

    #[test]
    fn constructor_expression_parses_positional_args() {
        let input = span_input("new A(x, y)");
        let (_, node) = expression(input).expect("expression");
        match &node.value {
            Expression::Constructor { type_name, args } => {
                assert_eq!(type_name, "A");
                assert_eq!(args.len(), 2);
                assert!(args.iter().all(|a| a.name.is_none()));
                assert!(matches!(&args[0].value.value, Expression::FeatureRef(s) if s == "x"));
                assert!(matches!(&args[1].value.value, Expression::FeatureRef(s) if s == "y"));
            }
            other => panic!("expected Constructor, got {other:?}"),
        }
    }

    #[test]
    fn constructor_expression_parses_named_args() {
        // Real shape from the Systems Library (RiskMetadata.sysml):
        // `new RiskLevel(probability = LevelEnum::low)`.
        let input = span_input("new RiskLevel(probability = LevelEnum::low)");
        let (_, node) = expression(input).expect("expression");
        match &node.value {
            Expression::Constructor { type_name, args } => {
                assert_eq!(type_name, "RiskLevel");
                assert_eq!(args.len(), 1);
                assert_eq!(args[0].name.as_deref(), Some("probability"));
                assert!(
                    matches!(&args[0].value.value, Expression::FeatureRef(s) if s == "LevelEnum::low")
                );
            }
            other => panic!("expected Constructor, got {other:?}"),
        }
    }

    #[test]
    fn constructor_expression_without_args_has_empty_arg_list() {
        let input = span_input("new A");
        let (_, node) = expression(input).expect("expression");
        match &node.value {
            Expression::Constructor { type_name, args } => {
                assert_eq!(type_name, "A");
                assert!(args.is_empty());
            }
            other => panic!("expected Constructor, got {other:?}"),
        }
    }

    #[test]
    fn invocation_parses_mixed_named_and_positional_args() {
        // Real shape from ParameterTest.sysml: `F(q = 1, p = a)`.
        let input = span_input("F(q = 1, p = a)");
        let (_, node) = expression(input).expect("expression");
        match &node.value {
            Expression::Invocation { args, .. } => {
                assert_eq!(args.len(), 2);
                assert_eq!(args[0].name.as_deref(), Some("q"));
                assert!(matches!(&args[0].value.value, Expression::LiteralInteger(1)));
                assert_eq!(args[1].name.as_deref(), Some("p"));
                assert!(matches!(&args[1].value.value, Expression::FeatureRef(s) if s == "a"));
            }
            other => panic!("expected Invocation, got {other:?}"),
        }
    }

    #[test]
    fn invocation_named_argument_is_not_confused_with_equality() {
        // `a == b` must stay a plain positional equality expression, not `a`-named-`= b`.
        let input = span_input("F(a == b)");
        let (_, node) = expression(input).expect("expression");
        match &node.value {
            Expression::Invocation { args, .. } => {
                assert_eq!(args.len(), 1);
                assert!(args[0].name.is_none());
                assert!(matches!(
                    &args[0].value.value,
                    Expression::BinaryOp {
                        op: BinaryOperator::Eq,
                        ..
                    }
                ));
            }
            other => panic!("expected Invocation, got {other:?}"),
        }
    }

    #[test]
    fn collection_op_collect_parses_with_args() {
        let input = span_input("items->collect(f)");
        let (_, node) = expression(input).expect("expression");
        match &node.value {
            Expression::CollectionOp { op, base, args } => {
                assert_eq!(op, &CollectionOperator::Collect);
                assert_eq!(args.len(), 1);
                assert!(matches!(&base.value, Expression::FeatureRef(s) if s == "items"));
            }
            other => panic!("expected CollectionOp, got {other:?}"),
        }
    }

    #[test]
    fn path_expression_single_segment_stays_feature_ref() {
        let input = span_input("engine");
        let (_, node) = path_expression(input).expect("path_expression");
        assert!(matches!(&node.value, Expression::FeatureRef(s) if s == "engine"));
    }

    #[test]
    fn path_expression_multi_segment_becomes_feature_chain_ref() {
        let input = span_input("engine.fuelCmdPort.flowRate");
        let (_, node) = path_expression(input).expect("path_expression");
        match &node.value {
            Expression::FeatureChainRef(chain) => {
                assert_eq!(
                    chain.segments,
                    vec![
                        "engine".to_string(),
                        "fuelCmdPort".to_string(),
                        "flowRate".to_string(),
                    ]
                );
            }
            other => panic!("expected FeatureChainRef, got {other:?}"),
        }
    }

    #[test]
    fn path_expression_leading_qualified_name_then_dot_chain() {
        let input = span_input("Foo::bar.baz");
        let (_, node) = path_expression(input).expect("path_expression");
        match &node.value {
            Expression::FeatureChainRef(chain) => {
                assert_eq!(
                    chain.segments,
                    vec!["Foo::bar".to_string(), "baz".to_string()]
                );
            }
            other => panic!("expected FeatureChainRef, got {other:?}"),
        }
    }

    #[test]
    fn metadata_access_expression_parses() {
        let input = span_input("x.metadata");
        let (_, node) = expression(input).expect("expression");
        match &node.value {
            Expression::MetadataAccess(base) => {
                assert!(matches!(&base.value, Expression::FeatureRef(s) if s == "x"));
            }
            other => panic!("expected MetadataAccess, got {other:?}"),
        }
    }

    #[test]
    fn parenthesized_expression_preserves_explicit_parens_marker() {
        let input = span_input("(a + b)");
        let (_, node) = expression(input).expect("expression");
        match &node.value {
            Expression::Parenthesized(inner) => {
                assert!(matches!(&inner.value, Expression::BinaryOp { .. }));
            }
            other => panic!("expected Parenthesized, got {other:?}"),
        }
    }

    #[test]
    fn non_parenthesized_binary_expression_has_no_parenthesized_wrapper() {
        let input = span_input("a + b");
        let (_, node) = expression(input).expect("expression");
        assert!(matches!(&node.value, Expression::BinaryOp { .. }));
    }
}
