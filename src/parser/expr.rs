//! Expression and path parsing for values and bind/connect.

use crate::ast::{
    Argument, BinaryOperator, CollectionOperator, CollectionOperatorBody,
    CollectionOperatorParameter, CollectionOperatorParameterTyping, Expression, InOut, Node,
    ReferenceSeparator, Span, TypeCheckKind, UnaryOperator,
};
use crate::parser::lex::{
    classified_reference_path, name, qualified_reference, reference_path, starts_with_keyword,
    ws_and_comments, ReferencePathKind,
};
use crate::parser::Input;
use crate::parser::{node_from_to, with_span};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value};
use nom::sequence::preceded;
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
    // A `.` only continues the literal when NOT part of a `..` range operator (`1..4`) and when
    // followed by a digit -- `1.` alone previously lexed as a real, silently turning
    // `(1..size(seq))` into `1.` + member access instead of a Range expression.
    if i + 1 < frag.len() && frag[i] == b'.' && frag[i + 1].is_ascii_digit() {
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
        map(|i| literal_keyword_token(i, b"true"), |_| true),
        map(|i| literal_keyword_token(i, b"false"), |_| false),
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
    let (input, n) = qualified_reference(input)?;
    Ok((input, node_from_to(start, input, Expression::FeatureRef(n))))
}

/// Metadata reference: @ qualified_name (e.g. @Safety, @Security for filter expressions).
fn metadata_ref_primary(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"@"[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, n) = qualified_reference(input)?;
    Ok((
        input,
        node_from_to(start, input, Expression::Classification { metaclass: n }),
    ))
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
            Expression::Unit(unit_name),
        ))),
    );
    let expr = Expression::LiteralWithUnit {
        value: Box::new(value_node),
        unit: Box::new(unit),
    };
    Ok((input, node_from_to(start, input, expr)))
}

/// KerML null value: the `null` keyword. Empty parens `()` are a *separate* production
/// (`Expression::Null` too, but spelled `(` `)`) handled directly by the iterative expression
/// engine below, alongside every other `(`-led construct, since -- like any parenthesized group --
/// it must not recurse through this function.
fn null_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = literal_keyword_token(input, b"null")?;
    Ok((input, node_from_to(start, input, Expression::Null)))
}

/// SelectExpression: base `.?` selector
fn select_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    crate::parser::span::reference_transaction(input, select_expression_inner)
}

fn select_expression_inner(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, base) = feature_ref_primary(input)?;
    let (input, _) = tag(&b".?"[..]).parse(input)?;
    let (input, selector) = preceded(ws_and_comments, qualified_reference).parse(input)?;
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
    crate::parser::span::reference_transaction(input, collect_expression_inner)
}

fn collect_expression_inner(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, base) = feature_ref_primary(input)?;
    let (input, _) = tag(&b".**"[..]).parse(input)?;
    let (input, selector) = preceded(ws_and_comments, qualified_reference).parse(input)?;
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

/// Match a bare alphabetic keyword, requiring a word boundary immediately after it (matching
/// [`starts_with_keyword`]'s existing use for the `meta` postfix operator below). Without this, a
/// plain `tag()` on e.g. `not`/`and`/`or`/`istype`/`as`/`new` would also match as a prefix of any
/// unrelated identifier that merely starts with the same letters -- and does, on real, in-use
/// identifiers in the official SysML v2 Systems Library: `notEmpty` (Kernel Semantic Library,
/// silently misparsed as `not Empty`), `newSeq` (Kernel Function Library, silently misparsed as
/// `new Seq`), and more generally any `order*`/`as*` identifier if it ever appears where a `or`/`as`
/// token check runs. These were previously silent -- no parse error, just a wrong AST -- which is
/// why none of the existing diagnostic-count-based tests caught them.
fn keyword_token<'a>(input: Input<'a>, keyword: &'static [u8]) -> IResult<Input<'a>, Input<'a>> {
    if !starts_with_keyword(input.fragment(), keyword) {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    tag(keyword).parse(input)
}

/// Match a literal keyword (`true`/`false`/`null`), requiring a word boundary immediately after
/// it. Deliberately distinct from [`keyword_token`]: that helper's boundary check
/// ([`starts_with_keyword`]) only accepts whitespace or `{`/`:`/`;`/`[` as a follower, which is
/// correct for operator keywords (`not`/`and`/`or`/...) and declaration keywords (always
/// followed by an operand or a body-opening token) but wrong for *literal* keywords, which can be
/// legally followed by any non-identifier byte a value can be followed by -- `)`, `,`, `==`, `!=`,
/// end of input, etc. (e.g. `f(true)`, `x == null`). Reusing `keyword_token` verbatim here would
/// trade the mis-lexing bug this fixes for a new one: valid literals immediately followed by
/// punctuation outside that narrow allowlist would stop parsing as literals at all.
///
/// Without any boundary check at all (the bug this fixes, GH-58), a bare `tag()` on `true`/
/// `false`/`null` also matches as a prefix of any longer identifier that merely starts with the
/// same letters -- e.g. `nullPoint`, `trueValue`, `falseAlarm` -- silently misparsing as the
/// literal followed by a stray, unparseable identifier fragment.
fn literal_keyword_token<'a>(
    input: Input<'a>,
    keyword: &'static [u8],
) -> IResult<Input<'a>, Input<'a>> {
    let is_ident_boundary = input
        .fragment()
        .get(keyword.len())
        .is_none_or(|b| !b.is_ascii_alphanumeric() && *b != b'_');
    if !input.fragment().starts_with(keyword) || !is_ident_boundary {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    tag(keyword).parse(input)
}

/// KerML type test suffix: `istype Type`, `hastype Type`, or `as Type`.
fn type_check_kind_token(input: Input<'_>) -> IResult<Input<'_>, TypeCheckKind> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(|i| keyword_token(i, b"istype"), |_| TypeCheckKind::Istype),
        map(|i| keyword_token(i, b"hastype"), |_| TypeCheckKind::Hastype),
        map(|i| keyword_token(i, b"as"), |_| TypeCheckKind::As),
    ))
    .parse(input)
}

fn type_check_primary(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, kind) = type_check_kind_token(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, type_name) = qualified_reference(input)?;
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

/// Primary alternatives that never recurse into [`expression`]. `(`-led groups/tuples/empty-parens
/// and `new`-led constructors are handled directly by the iterative engine below instead, because
/// (unlike every alternative here) they can nest arbitrarily deep.
fn primary_atom(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        conditional_expression,
        extent_expression,
        literal_with_unit,
        literal_only,
        null_expression,
        metadata_ref_primary,
        type_check_primary,
        collect_expression,
        select_expression,
        feature_ref_primary,
    ))
    .parse(input)
}

/// `if <test> ? <then> else <else>`
fn conditional_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, _) = keyword_token(input, b"if")?;
    let (input, test) = expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"?"[..])).parse(input)?;
    let (input, then_expr) = expression(input)?;
    let (input, _) = preceded(ws_and_comments, |i| keyword_token(i, b"else")).parse(input)?;
    let (input, else_expr) = expression(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Expression::Conditional {
                test: Box::new(test),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            },
        ),
    ))
}

/// `all QualifiedName`
fn extent_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, _) = keyword_token(input, b"all")?;
    let (input, target) = preceded(ws_and_comments, qualified_reference).parse(input)?;
    Ok((
        input,
        node_from_to(start, input, Expression::Extent { target }),
    ))
}

fn collection_operator_parameter(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<CollectionOperatorParameter>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, (direction_span, direction)) = with_span(|input| {
        alt((
            map(|input| keyword_token(input, b"inout"), |_| InOut::InOut),
            map(|input| keyword_token(input, b"in"), |_| InOut::In),
            map(|input| keyword_token(input, b"out"), |_| InOut::Out),
        ))
        .parse(input)
    })(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, reference_keyword_span) = if starts_with_keyword(input.fragment(), b"ref") {
        let (input, (span, _)) = with_span(|input| keyword_token(input, b"ref"))(input)?;
        let (input, _) = ws_and_comments(input)?;
        (input, Some(span))
    } else {
        (input, None)
    };
    let (input, (name_span, parameter_name)) = with_span(name)(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, typing) = if input.fragment().starts_with(b":") {
        let (input, (separator_span, _)) = with_span(tag(&b":"[..]))(input)?;
        let (input, _) = ws_and_comments(input)?;
        let (input, target) = qualified_reference(input)?;
        (
            input,
            Some(CollectionOperatorParameterTyping {
                separator_span,
                target,
            }),
        )
    } else {
        (input, None)
    };
    let (input, _) = ws_and_comments(input)?;
    let (input, (semicolon_span, _)) = with_span(tag(&b";"[..]))(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            CollectionOperatorParameter {
                direction: Node::new(direction_span, direction),
                reference_keyword_span,
                name: parameter_name,
                name_span,
                typing,
                semicolon_span,
            },
        ),
    ))
}

/// Standalone KerML `BodyExpression` (`{ parameters* result? }`) -- the same shape a collection
/// operator's brace body uses; see [`Expression::BodyExpr`].
pub(crate) fn body_expression(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<CollectionOperatorBody>> {
    collection_operator_body(input)
}

fn collection_operator_body(input: Input<'_>) -> IResult<Input<'_>, Node<CollectionOperatorBody>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (mut input, (open_brace_span, _)) = with_span(tag(&b"{"[..]))(input)?;
    let mut parameters = Vec::new();
    loop {
        let (next, _) = ws_and_comments(input)?;
        if starts_with_keyword(next.fragment(), b"in")
            || starts_with_keyword(next.fragment(), b"out")
            || starts_with_keyword(next.fragment(), b"inout")
        {
            let (next, parameter) = collection_operator_parameter(next)?;
            parameters.push(parameter);
            input = next;
        } else {
            input = next;
            break;
        }
    }
    let (next, _) = ws_and_comments(input)?;
    let (input, result) = if next.fragment().starts_with(b"}") {
        (next, None)
    } else {
        let (input, result) = expression(next)?;
        (input, Some(Box::new(result)))
    };
    let (input, _) = ws_and_comments(input)?;
    let (input, (close_brace_span, _)) = with_span(tag(&b"}"[..]))(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            CollectionOperatorBody {
                open_brace_span,
                parameters,
                result,
                close_brace_span,
            },
        ),
    ))
}

/// Operator tokens yield the typed operator directly. Spelling a 1-3 byte token as an owned
/// `String` only to convert it into this enum allocated once per token, on every speculative
/// attempt as well as every accepted one, and left two representations of the same fact.
fn logical_op_token(input: Input<'_>) -> IResult<Input<'_>, BinaryOperator> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        // Symbolic forms first: `&&`/`||` must win over `additive_op_token`'s bare `&`/`|`, which
        // would otherwise greedily match just the first character and misparse `a && b` as `a`
        // followed by a stray, unparseable `& b`.
        value(BinaryOperator::And, tag(&b"&&"[..])),
        value(BinaryOperator::Or, tag(&b"||"[..])),
        value(BinaryOperator::And, |i| keyword_token(i, b"and")),
        value(BinaryOperator::Or, |i| keyword_token(i, b"or")),
        value(BinaryOperator::Xor, |i| keyword_token(i, b"xor")),
    ))
    .parse(input)
}

/// Implication tier: `implies` and the KerML null-coalescing `??` (BNF groups `'??' | 'or' |
/// 'and' | 'implies'`; `??` binds loosest alongside `implies`), e.g. `collection->reduce '+'
/// ?? zero` (Kernel Function Library `DataFunctions.kerml`).
fn implies_op_token(input: Input<'_>) -> IResult<Input<'_>, BinaryOperator> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        value(BinaryOperator::Implies, |i| keyword_token(i, b"implies")),
        value(BinaryOperator::NullCoalesce, tag(&b"??"[..])),
    ))
    .parse(input)
}

fn equality_op_token(input: Input<'_>) -> IResult<Input<'_>, BinaryOperator> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        value(BinaryOperator::StrictEq, tag(&b"==="[..])),
        value(BinaryOperator::StrictNe, tag(&b"!=="[..])),
        value(BinaryOperator::Eq, tag(&b"=="[..])),
        value(BinaryOperator::Ne, tag(&b"!="[..])),
    ))
    .parse(input)
}

fn comparison_op_token(input: Input<'_>) -> IResult<Input<'_>, BinaryOperator> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        value(BinaryOperator::Ge, tag(&b">="[..])),
        value(BinaryOperator::Le, tag(&b"<="[..])),
        value(BinaryOperator::Gt, tag(&b">"[..])),
        value(BinaryOperator::Lt, tag(&b"<"[..])),
        value(BinaryOperator::Range, tag(&b".."[..])),
    ))
    .parse(input)
}

fn additive_op_token(input: Input<'_>) -> IResult<Input<'_>, BinaryOperator> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        value(BinaryOperator::Add, tag(&b"+"[..])),
        value(BinaryOperator::Sub, tag(&b"-"[..])),
        value(BinaryOperator::BitOr, tag(&b"|"[..])),
        value(BinaryOperator::BitAnd, tag(&b"&"[..])),
    ))
    .parse(input)
}

fn multiplicative_op_token(input: Input<'_>) -> IResult<Input<'_>, BinaryOperator> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        value(BinaryOperator::Exp, tag(&b"**"[..])),
        value(BinaryOperator::Mul, tag(&b"*"[..])),
        value(BinaryOperator::Div, tag(&b"/"[..])),
        value(BinaryOperator::Mod, tag(&b"%"[..])),
        value(BinaryOperator::Pow, tag(&b"^"[..])),
    ))
    .parse(input)
}

/// Unary operator token: + - ~ not (KerML UnaryOperator).
fn unary_op_token(input: Input<'_>) -> IResult<Input<'_>, UnaryOperator> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        value(UnaryOperator::Not, |i| keyword_token(i, b"not")),
        value(UnaryOperator::BitNot, tag(&b"~"[..])),
        value(UnaryOperator::Plus, tag(&b"+"[..])),
        value(UnaryOperator::Minus, tag(&b"-"[..])),
    ))
    .parse(input)
}

// ---------------------------------------------------------------------------------------------
// Iterative (Pratt / precedence-climbing) expression engine.
//
// Recursive descent naturally encodes "how deeply is this expression nested" as native call-stack
// depth. Every `(`-group, function/constructor argument list, and `#( )` index used to re-enter the
// grammar through a real recursive call -- one native stack frame per nesting level, with no limit,
// so pathological input like `((((...))))` or `f(g(h(i(...))))` could overflow the stack. Binary
// operator *chains* were already safe (a `loop`, not recursion) but mixing that with recursive
// grouping still left the two genuinely unbounded vectors above.
//
// This module instead keeps one explicit, heap-allocated stack of suspended parse contexts
// (`Frame` + `ItemState` pairs in `expression`'s `stack: Vec<_>`): entering `(` pushes a frame
// instead of recursing, and the matching `)` pops it and resumes. Nesting depth becomes `Vec`
// growth, not call-stack growth -- the native call stack used by `expression` itself stays O(1)
// regardless of how deeply the input nests.
// ---------------------------------------------------------------------------------------------

const PREC_IMPLIES: u8 = 0;
const PREC_LOGICAL: u8 = 1;
const PREC_EQUALITY: u8 = 2;
const PREC_COMPARISON: u8 = 3;
const PREC_ADDITIVE: u8 = 4;
const PREC_MULTIPLICATIVE: u8 = 5;

/// Try each precedence tier's operator token, tightest-binding first -- the same order in which the
/// original recursive ladder effectively tried them (each level only got a chance once every
/// tighter-binding level nested inside it had already failed to match). Preserving that order
/// preserves any token-overlap edge cases (e.g. `&` vs `&&`) exactly as they behaved before.
fn any_binary_op_token(input: Input<'_>) -> IResult<Input<'_>, (BinaryOperator, u8)> {
    alt((
        map(multiplicative_op_token, |op| (op, PREC_MULTIPLICATIVE)),
        // `logical_op_token` before `additive_op_token`: its symbolic `&&`/`||` forms must win over
        // `additive_op_token`'s bare `&`/`|` (see the comment in `logical_op_token`).
        map(logical_op_token, |op| (op, PREC_LOGICAL)),
        map(additive_op_token, |op| (op, PREC_ADDITIVE)),
        map(comparison_op_token, |op| (op, PREC_COMPARISON)),
        map(equality_op_token, |op| (op, PREC_EQUALITY)),
        map(implies_op_token, |op| (op, PREC_IMPLIES)),
    ))
    .parse(input)
}

struct PendingOp {
    op: BinaryOperator,
    prec: u8,
}

/// Precedence-climbing state for ONE expression (an argument value, a parenthesized group's item,
/// or a full top-level expression): a `Vec`-backed operand stack and operator stack implementing
/// the classic iterative operator-precedence algorithm. All operators here are left-associative --
/// the original recursive ladder never treated any operator, including `**`, as right-associative,
/// so this preserves that exactly (e.g. `a ** b ** c` still parses as `(a ** b) ** c`).
#[derive(Default)]
struct Climb {
    operands: Vec<Node<Expression>>,
    ops: Vec<PendingOp>,
}

impl Climb {
    /// Push a new operator, first reducing any already-pending operator whose precedence is at
    /// least as tight (this is what gives left-associativity for same-precedence chains).
    fn push_op(&mut self, op: BinaryOperator, prec: u8) {
        while let Some(top) = self.ops.last() {
            if top.prec < prec {
                break;
            }
            self.reduce_one();
        }
        self.ops.push(PendingOp { op, prec });
    }

    /// Pop one operator and its two operands, combine them into a `BinaryOp`, and push the result
    /// back. A no-op if the stacks are shorter than expected -- this is only ever called with both
    /// present by construction (`push_op`/`finish` never call it otherwise), and this crate never
    /// panics on any input, so an unreachable state degrades silently rather than crashing.
    fn reduce_one(&mut self) {
        let Some(pending) = self.ops.pop() else {
            return;
        };
        let Some(right) = self.operands.pop() else {
            return;
        };
        let Some(left) = self.operands.pop() else {
            self.operands.push(right);
            return;
        };
        let span = Span {
            offset: left.span.offset,
            line: left.span.line,
            column: left.span.column,
            len: (right.span.offset + right.span.len).saturating_sub(left.span.offset),
        };
        self.operands.push(Node::new(
            span,
            Expression::BinaryOp {
                op: pending.op,
                left: Box::new(left),
                right: Box::new(right),
            },
        ));
    }

    /// Fold everything down to the single resulting value. Only called once at least one atom has
    /// been pushed (`expression` always feeds an atom in before checking whether the item is
    /// done), so `operands` is never actually empty here; the fallback exists only so this can
    /// never panic even if that invariant were somehow violated.
    fn finish(mut self) -> Node<Expression> {
        while !self.ops.is_empty() {
            self.reduce_one();
        }
        self.operands
            .pop()
            .unwrap_or_else(|| Node::new(Span::dummy(), Expression::Null))
    }
}

/// Per-item parse state, suspended on `expression`'s explicit stack whenever a `(` is entered and
/// resumed when its matching `)` is found: the climb in progress, any unary prefixes already
/// consumed and awaiting their operand (and where they started, for the `UnaryOp` span), and --
/// inside a call-style argument list -- the `name =` prefix already consumed for the item
/// currently being parsed, if any.
struct ItemState<'a> {
    climb: Climb,
    pending_unary: Vec<UnaryOperator>,
    prefix_start: Input<'a>,
    arg_parameter: Option<crate::ast::QualifiedReferenceId>,
}

impl<'a> ItemState<'a> {
    fn fresh(at: Input<'a>) -> Self {
        ItemState {
            climb: Climb::default(),
            pending_unary: Vec::new(),
            prefix_start: at,
            arg_parameter: None,
        }
    }
}

/// What a suspended `(`-delimited item list will build once its closing `)` is reached.
enum FrameKind {
    /// `(` expr (`,` expr)* `)` in primary position: one item -> `Parenthesized`, 2+ -> `Tuple`.
    Group,
    /// Invocation argument list: postfix `(` args `)` applied to `base`.
    Invocation { base: Node<Expression> },
    /// Index argument: postfix `#(` expr `)` applied to `base` -- exactly one item, no comma.
    Index { base: Node<Expression> },
    /// Arrow-invocation argument list: postfix `->` name `(` args `)` applied to `base`.
    ArrowInvocation {
        base: Node<Expression>,
        member: String,
    },
    /// Constructor argument list: `new` type_name `(` args `)`.
    Constructor {
        type_name: crate::ast::QualifiedReferenceId,
    },
}

/// A suspended `(`-delimited list, collecting comma-separated items until its closing `)`.
struct Frame<'a> {
    kind: FrameKind,
    /// Span anchor for the eventual built node: the start of `base`/`new`/the opening `(` itself
    /// for a bare group -- exactly where the original recursive parser captured its `start`.
    open_at: Input<'a>,
    items: Vec<Argument>,
}

impl<'a> Frame<'a> {
    /// Call-style frames (as opposed to `Group`/`Index`) support `NAME '=' value` items and allow
    /// zero items (`f()`).
    fn is_call_style(&self) -> bool {
        matches!(
            self.kind,
            FrameKind::Invocation { .. }
                | FrameKind::ArrowInvocation { .. }
                | FrameKind::Constructor { .. }
        )
    }

    /// `Index` never allows a comma -- its argument list is exactly one bare expression.
    fn allows_comma(&self) -> bool {
        !matches!(self.kind, FrameKind::Index { .. })
    }
}

/// Look ahead for a KerML named-argument prefix (`reference '=' ...`), used only when starting a fresh
/// item inside a call-style argument list. Only a lone `=` counts -- `==`/`===` must stay ordinary
/// equality expressions, so e.g. `f(a == b)` is one positional boolean argument, not `a`-named-`=
/// b`. On no match, returns the input completely unchanged so normal atom parsing proceeds (`a ==
/// b` as a positional value, or `a` as a positional value followed by `,`/`)`).
fn named_arg_prefix(input: Input<'_>) -> (Input<'_>, Option<crate::ast::QualifiedReferenceId>) {
    let checkpoint = input.extra.reference_checkpoint();
    let Ok((ws_input, _)) = ws_and_comments(input) else {
        return (input, None);
    };
    let Ok((after_name, parameter)) = reference_path(ws_input) else {
        return (input, None);
    };
    let Ok((after_ws, _)) = ws_and_comments(after_name) else {
        input.extra.rollback_references(checkpoint);
        return (input, None);
    };
    let frag = after_ws.fragment();
    if frag.first() == Some(&b'=') && frag.get(1) != Some(&b'=') {
        let eq_result: IResult<Input<'_>, Input<'_>> = tag(&b"="[..]).parse(after_ws);
        let Ok((after_eq, _)) = eq_result else {
            input.extra.rollback_references(checkpoint);
            return (input, None);
        };
        let Ok((after_eq, _)) = ws_and_comments(after_eq) else {
            input.extra.rollback_references(checkpoint);
            return (input, None);
        };
        (after_eq, Some(parameter))
    } else {
        input.extra.rollback_references(checkpoint);
        (input, None)
    }
}

/// Look ahead for a `new` constructor's type name (`new` QualifiedName). `keyword_token` already
/// rejects identifiers that merely start with `new` (e.g. `newSeq`, real usage in the Kernel
/// Function Library). This also consumes nothing and returns `None` if a genuine `new` keyword
/// isn't followed by a valid qualified name, letting the caller fall back to parsing it as a plain
/// identifier via [`primary_atom`] (`new` is not a reserved word in
/// [`crate::parser::lex::basic_name`]) -- matching the original recursive parser's `alt` fallthrough
/// from `constructor_expression` to `feature_ref_primary`.
fn try_constructor_prefix(
    after_ws: Input<'_>,
) -> Option<(Input<'_>, crate::ast::QualifiedReferenceId)> {
    let kw_result: IResult<Input<'_>, Input<'_>> = keyword_token(after_ws, b"new");
    let (after_kw, _) = kw_result.ok()?;
    let (after_kw, _) = ws_and_comments(after_kw).ok()?;
    qualified_reference(after_kw).ok()
}

/// Build the final node for a frame once its closing `)` has been consumed.
fn build_frame_node<'a>(frame: Frame<'a>, end: Input<'a>) -> Node<Expression> {
    let Frame {
        kind,
        open_at,
        items,
    } = frame;
    match kind {
        FrameKind::Group => {
            let mut values: Vec<Node<Expression>> =
                items.into_iter().map(|arg| arg.value).collect();
            if values.len() == 1 {
                let value = values
                    .pop()
                    .unwrap_or_else(|| Node::new(Span::dummy(), Expression::Null));
                node_from_to(open_at, end, Expression::Parenthesized(Box::new(value)))
            } else {
                node_from_to(open_at, end, Expression::Tuple(values))
            }
        }
        FrameKind::Invocation { base } => node_from_to(
            open_at,
            end,
            Expression::Invocation {
                callee: Box::new(base),
                args: items,
            },
        ),
        FrameKind::Index { base } => {
            let index = items
                .into_iter()
                .next()
                .map(|arg| arg.value)
                .unwrap_or_else(|| Node::new(Span::dummy(), Expression::Null));
            node_from_to(
                open_at,
                end,
                Expression::Index {
                    base: Box::new(base),
                    index: Box::new(index),
                },
            )
        }
        FrameKind::ArrowInvocation { base, member } => node_from_to(
            open_at,
            end,
            Expression::CollectionOp {
                op: CollectionOperator::from_name(&member),
                base: Box::new(base),
                args: items,
                brace_body: None,
                dot_shorthand: false,
            },
        ),
        FrameKind::Constructor { type_name } => node_from_to(
            open_at,
            end,
            Expression::Constructor {
                type_name,
                args: items,
            },
        ),
    }
}

/// Full expression with precedence-aware binary parsing. See the module-level comment above for
/// why this is an explicit-stack loop rather than recursive descent.
pub(crate) fn expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    crate::parser::span::reference_transaction(input, expression_inner)
}

fn expression_inner(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let mut stack: Vec<(Frame<'_>, ItemState<'_>)> = Vec::new();
    let mut state = ItemState::fresh(input);
    let mut input = input;
    // Set when a frame has just been popped: the built node (still needs postfix applied) and the
    // `primary_start` anchor to resume postfix-span-building with, skipping the atom-fetch step.
    let mut pending_atom: Option<(Node<Expression>, Input<'_>)> = None;

    'outer: loop {
        // === Step 1: obtain one atom, with unary prefixes not yet applied ===
        let (mut atom, primary_start) = match pending_atom.take() {
            Some(pair) => pair,
            None => {
                state.prefix_start = input;
                while let Ok((next, tok)) = unary_op_token(input) {
                    state.pending_unary.push(tok);
                    input = next;
                }
                let (after_ws, _) = ws_and_comments(input)?;
                if after_ws.fragment().starts_with(b"(") {
                    let (after_paren, _) = tag(&b"("[..]).parse(after_ws)?;
                    let (peek, _) = ws_and_comments(after_paren)?;
                    if peek.fragment().starts_with(b")") {
                        let (after_close, _) = tag(&b")"[..]).parse(peek)?;
                        input = after_close;
                        (
                            node_from_to(after_ws, after_close, Expression::Null),
                            after_ws,
                        )
                    } else {
                        stack.push((
                            Frame {
                                kind: FrameKind::Group,
                                open_at: after_ws,
                                items: Vec::new(),
                            },
                            std::mem::replace(&mut state, ItemState::fresh(after_paren)),
                        ));
                        input = after_paren;
                        continue 'outer;
                    }
                } else if let Some((after_type, type_name)) = try_constructor_prefix(after_ws) {
                    let (peek, _) = ws_and_comments(after_type)?;
                    if peek.fragment().starts_with(b"(") {
                        let (after_paren, _) = tag(&b"("[..]).parse(peek)?;
                        let (empty_peek, _) = ws_and_comments(after_paren)?;
                        if empty_peek.fragment().starts_with(b")") {
                            let (after_close, _) = tag(&b")"[..]).parse(empty_peek)?;
                            input = after_close;
                            (
                                node_from_to(
                                    after_ws,
                                    after_close,
                                    Expression::Constructor {
                                        type_name,
                                        args: Vec::new(),
                                    },
                                ),
                                after_ws,
                            )
                        } else {
                            stack.push((
                                Frame {
                                    kind: FrameKind::Constructor { type_name },
                                    open_at: after_ws,
                                    items: Vec::new(),
                                },
                                std::mem::replace(&mut state, ItemState::fresh(after_paren)),
                            ));
                            input = after_paren;
                            let (after_lookahead, maybe_name) = named_arg_prefix(input);
                            if let Some(parameter) = maybe_name {
                                state.arg_parameter = Some(parameter);
                                input = after_lookahead;
                            }
                            continue 'outer;
                        }
                    } else {
                        input = after_type;
                        (
                            node_from_to(
                                after_ws,
                                after_type,
                                Expression::Constructor {
                                    type_name,
                                    args: Vec::new(),
                                },
                            ),
                            after_ws,
                        )
                    }
                } else {
                    let (next, node) = primary_atom(after_ws)?;
                    input = next;
                    (node, after_ws)
                }
            }
        };

        // === Step 2: apply postfix operators, which may themselves open a new `(`-delimited list ===
        loop {
            let (next, _) = ws_and_comments(input)?;
            if next.fragment().starts_with(b"(") {
                let (after_paren, _) = tag(&b"("[..]).parse(next)?;
                let (empty_peek, _) = ws_and_comments(after_paren)?;
                if empty_peek.fragment().starts_with(b")") {
                    let (after_close, _) = tag(&b")"[..]).parse(empty_peek)?;
                    let expr = Expression::Invocation {
                        callee: Box::new(atom),
                        args: Vec::new(),
                    };
                    atom = node_from_to(primary_start, after_close, expr);
                    input = after_close;
                    continue;
                }
                stack.push((
                    Frame {
                        kind: FrameKind::Invocation { base: atom },
                        open_at: primary_start,
                        items: Vec::new(),
                    },
                    std::mem::replace(&mut state, ItemState::fresh(after_paren)),
                ));
                input = after_paren;
                let (after_lookahead, maybe_name) = named_arg_prefix(input);
                if let Some(parameter) = maybe_name {
                    state.arg_parameter = Some(parameter);
                    input = after_lookahead;
                }
                continue 'outer;
            }
            if next.fragment().starts_with(b"#") {
                let (after_hash, _) = tag(&b"#"[..]).parse(next)?;
                let (after_paren, _) =
                    preceded(ws_and_comments, tag(&b"("[..])).parse(after_hash)?;
                stack.push((
                    Frame {
                        kind: FrameKind::Index { base: atom },
                        open_at: primary_start,
                        items: Vec::new(),
                    },
                    std::mem::replace(&mut state, ItemState::fresh(after_paren)),
                ));
                input = after_paren;
                continue 'outer;
            }
            if next.fragment().starts_with(b"::") {
                let (next, _) = tag(&b"::"[..]).parse(next)?;
                let (next, _) = ws_and_comments(next)?;
                let (next, member) = qualified_reference(next)?;
                let expr = Expression::MemberAccess {
                    base: Box::new(atom),
                    member,
                    separator: ReferenceSeparator::ColonColon,
                };
                atom = node_from_to(primary_start, next, expr);
                input = next;
                continue;
            }
            // `[unit]` measurement/coordinate-frame annotation after a value-shaped atom:
            // `(0, shape.width/2, 0)[source]`, `new Rotation(...)[frame]`, `angle[deg]` in
            // expression position (Domain Geometry libraries; spec42 Gap 49c). Numeric literals
            // keep their dedicated `literal_with_unit` path at atom level. Speculative: only
            // commits when a unit-shaped token closes with `]`, so declaration-level
            // multiplicities (`[1]`, `[0..*]`) after a typing are unaffected -- those never pass
            // through this engine.
            if next.fragment().starts_with(b"[")
                && matches!(
                    atom.value,
                    Expression::Parenthesized(_)
                        | Expression::Tuple(_)
                        | Expression::Invocation { .. }
                        | Expression::Constructor { .. }
                        | Expression::FeatureRef(_)
                        | Expression::MemberAccess { .. }
                )
            {
                let bracket_attempt = (|| -> IResult<Input<'_>, (Span, String)> {
                    let (after_open, _) = tag(&b"["[..]).parse(next)?;
                    let (after_open, _) = ws_and_comments(after_open)?;
                    let unit_start = after_open;
                    let (after_unit, unit_name) = unit_name_in_brackets(after_open)?;
                    let unit_span = crate::parser::span_from_to(unit_start, after_unit);
                    let (after_unit, _) = ws_and_comments(after_unit)?;
                    let (after_close, _) = tag(&b"]"[..]).parse(after_unit)?;
                    Ok((after_close, (unit_span, unit_name)))
                })();
                if let Ok((after_close, (unit_span, unit_name))) = bracket_attempt {
                    let unit = Node::new(
                        unit_span.clone(),
                        Expression::Bracket(Box::new(Node::new(
                            unit_span,
                            Expression::Unit(unit_name),
                        ))),
                    );
                    let expr = Expression::LiteralWithUnit {
                        value: Box::new(atom),
                        unit: Box::new(unit),
                    };
                    atom = node_from_to(primary_start, after_close, expr);
                    input = after_close;
                    continue;
                }
            }
            // KerML dot shorthands for body-expression operators: `x.{in xx; xx + 1}` is the
            // `collect` sugar and `x.?{in xx; cond}` the `select` sugar (spec42
            // `kerml/expressions.md`). Checked before plain member access, whose `.` + name
            // parse would otherwise fail on the `{`.
            if next.fragment().starts_with(b".{") || next.fragment().starts_with(b".?{") {
                let select = next.fragment().starts_with(b".?{");
                let (next, _) = if select {
                    tag(&b".?"[..]).parse(next)?
                } else {
                    tag(&b"."[..]).parse(next)?
                };
                let (after_brace, body) = collection_operator_body(next)?;
                let expr = Expression::CollectionOp {
                    op: CollectionOperator::from_name(if select { "select" } else { "collect" }),
                    base: Box::new(atom),
                    args: Vec::new(),
                    brace_body: Some(Box::new(body)),
                    dot_shorthand: true,
                };
                atom = node_from_to(primary_start, after_brace, expr);
                input = after_brace;
                continue;
            }
            if next.fragment().starts_with(b".") && !next.fragment().starts_with(b"..") {
                let (next, _) = tag(&b"."[..]).parse(next)?;
                let (next, _) = ws_and_comments(next)?;
                let member_input = next;
                let (next, member_text) = name(next)?;
                // `expr.metadata` is a dedicated KerML production (MetadataAccessExpression, BNF
                // 8.2.5.8.3: `ElementReferenceMember '.' 'metadata'`), distinct from ordinary
                // member access.
                let expr = if member_text == "metadata" {
                    Expression::MetadataAccess(Box::new(atom))
                } else {
                    let (_, member) = qualified_reference(member_input)?;
                    Expression::MemberAccess {
                        base: Box::new(atom),
                        member,
                        separator: ReferenceSeparator::Dot,
                    }
                };
                atom = node_from_to(primary_start, next, expr);
                input = next;
                continue;
            }
            if next.fragment().starts_with(b"->") {
                let (next, _) = tag(&b"->"[..]).parse(next)?;
                let (next, _) = ws_and_comments(next)?;
                let member_input = next;
                let (next, member) = name(next)?;
                let (after_name, _) = ws_and_comments(next)?;
                // Brace-body form: `collection->forAll { in ref w; expr }`
                if after_name.fragment().starts_with(b"{") {
                    let (after_brace, body) = collection_operator_body(after_name)?;
                    let expr = Expression::CollectionOp {
                        op: CollectionOperator::from_name(&member),
                        base: Box::new(atom),
                        args: Vec::new(),
                        brace_body: Some(Box::new(body)),
                        dot_shorthand: false,
                    };
                    atom = node_from_to(primary_start, after_brace, expr);
                    input = after_brace;
                    continue;
                }
                // KerML arrow-invocation, e.g. `collection->size()`, `xs->select(p)`.
                if after_name.fragment().starts_with(b"(") {
                    let (after_paren, _) = tag(&b"("[..]).parse(after_name)?;
                    let (empty_peek, _) = ws_and_comments(after_paren)?;
                    if empty_peek.fragment().starts_with(b")") {
                        let (after_close, _) = tag(&b")"[..]).parse(empty_peek)?;
                        let expr = Expression::CollectionOp {
                            op: CollectionOperator::from_name(&member),
                            base: Box::new(atom),
                            args: Vec::new(),
                            brace_body: None,
                            dot_shorthand: false,
                        };
                        atom = node_from_to(primary_start, after_close, expr);
                        input = after_close;
                        continue;
                    }
                    stack.push((
                        Frame {
                            kind: FrameKind::ArrowInvocation { base: atom, member },
                            open_at: primary_start,
                            items: Vec::new(),
                        },
                        std::mem::replace(&mut state, ItemState::fresh(after_paren)),
                    ));
                    input = after_paren;
                    let (after_lookahead, maybe_name) = named_arg_prefix(input);
                    if let Some(parameter) = maybe_name {
                        state.arg_parameter = Some(parameter);
                        input = after_lookahead;
                    }
                    continue 'outer;
                }
                // Function-reference argument with no parentheses: `->reduce
                // RealFunctions::'+'`, `->reduce min`, `->reduce '*' ?? 1` (Kernel Function
                // Library). The argument is a single (possibly qualified or quoted) function
                // name; reserved keywords (`and`, `then`, ...) stay operators/keywords.
                let leading_word_len = after_name
                    .fragment()
                    .iter()
                    .take_while(|b| b.is_ascii_alphanumeric() || **b == b'_')
                    .count();
                let takes_function_ref = after_name
                    .fragment()
                    .first()
                    .is_some_and(|&b| b == b'\'' || b.is_ascii_alphabetic() || b == b'_')
                    && !crate::parser::lex::is_reserved_keyword(
                        &after_name.fragment()[..leading_word_len],
                    );
                if takes_function_ref {
                    if let Ok((after_ref, func_ref)) = qualified_reference(after_name) {
                        let span = crate::parser::span_from_to(after_name, after_ref);
                        let expr = Expression::CollectionOp {
                            op: CollectionOperator::from_name(&member),
                            base: Box::new(atom),
                            args: vec![Argument {
                                parameter: None,
                                value: Node::new(span, Expression::FeatureRef(func_ref)),
                            }],
                            brace_body: None,
                            dot_shorthand: false,
                        };
                        atom = node_from_to(primary_start, after_ref, expr);
                        input = after_ref;
                        continue;
                    }
                }
                // Bare arrow access with no call (rare) -- fall back to plain member access.
                let (_, member_ref) = qualified_reference(member_input)?;
                let expr = Expression::MemberAccess {
                    base: Box::new(atom),
                    member: member_ref,
                    separator: ReferenceSeparator::Dot,
                };
                atom = node_from_to(primary_start, next, expr);
                input = next;
                continue;
            }
            if let Ok((after_kind, kind)) = type_check_kind_token(next) {
                if let Ok((after_type, type_name)) = qualified_reference(after_kind) {
                    let expr = Expression::TypeCheck {
                        kind,
                        operand: Some(Box::new(atom)),
                        type_name,
                    };
                    atom = node_from_to(primary_start, after_type, expr);
                    input = after_type;
                    continue;
                }
            }
            if starts_with_keyword(next.fragment(), b"meta") {
                let (next, _) = tag(&b"meta"[..]).parse(next)?;
                let (next, _) = ws_and_comments(next)?;
                let (next, metaclass) = qualified_reference(next)?;
                let expr = Expression::MetaCast {
                    base: Box::new(atom),
                    metaclass,
                };
                atom = node_from_to(primary_start, next, expr);
                input = next;
                continue;
            }
            // No postfix operator matched: leave `input` where it was before this iteration's
            // whitespace peek (`next` is intentionally discarded). The original recursive
            // `postfix()` unconditionally committed that peek even on a non-match, so every atom's
            // span used to bleed into one trailing run of whitespace/comments past its last real
            // token -- e.g. `1750 [kg]` before ` {` used to span through the trailing space. That
            // was never a deliberate design choice, just an artifact of `postfix()`'s
            // strip-then-check structure, and PARSE_AST_VERSION is bumped alongside this change
            // (see CHANGELOG) precisely so spans now end exactly at each expression's own text.
            break;
        }

        // === Step 3: apply any pending unary prefixes now that postfix is fully resolved ===
        for op in state.pending_unary.drain(..).rev() {
            atom = node_from_to(
                state.prefix_start,
                input,
                Expression::UnaryOp {
                    op,
                    operand: Box::new(atom),
                },
            );
        }

        // === Step 4: feed the atom into the climb, then look for a following binary operator ===
        state.climb.operands.push(atom);
        if let Ok((next, (op, prec))) = any_binary_op_token(input) {
            state.climb.push_op(op, prec);
            input = next;
            continue 'outer;
        }

        // No operator continues this item -- it's complete.
        let value = std::mem::take(&mut state.climb).finish();
        let Some((frame, _)) = stack.last_mut() else {
            return Ok((input, value));
        };
        let arg_parameter = state.arg_parameter.take();
        frame.items.push(Argument {
            parameter: arg_parameter,
            value,
        });
        let allows_comma = frame.allows_comma();
        let is_call_style = frame.is_call_style();
        let (peek, _) = ws_and_comments(input)?;
        if allows_comma && peek.fragment().starts_with(b",") {
            let (next, _) = tag(&b","[..]).parse(peek)?;
            input = next;
            state = ItemState::fresh(next);
            if is_call_style {
                let (after_lookahead, maybe_name) = named_arg_prefix(input);
                if let Some(parameter) = maybe_name {
                    state.arg_parameter = Some(parameter);
                    input = after_lookahead;
                }
            }
            continue 'outer;
        }
        if peek.fragment().starts_with(b")") {
            let (next, _) = tag(&b")"[..]).parse(peek)?;
            // `stack` is known non-empty here (the `let-else` above already confirmed it, and
            // nothing between there and here can pop it) -- but rather than assert that with a
            // panicking `expect`, treat the (unreachable) alternative as an ordinary parse error,
            // since this crate never panics on any input.
            let Some((frame, outer_state)) = stack.pop() else {
                return Err(nom::Err::Error(nom::error::Error::new(
                    next,
                    nom::error::ErrorKind::Fail,
                )));
            };
            let open_at = frame.open_at;
            let built = build_frame_node(frame, next);
            state = outer_state;
            input = next;
            pending_atom = Some((built, open_at));
            continue 'outer;
        }
        return Err(nom::Err::Error(nom::error::Error::new(
            peek,
            nom::error::ErrorKind::Tag,
        )));
    }
}

/// Path expression: qualified name and/or member access (for bind/connect).
/// Supports `A`, `A::B::C`, `A.B.C`, and combinations like `A::B.C`.
///
/// A single segment (or a purely `::`-qualified name) stays [`Expression::FeatureRef`]. A path
/// containing `.` becomes [`Expression::FeatureChainRef`]. Both variants carry the same shared
/// arena identity; its borrowed view preserves the authored segments and separator kinds.
pub(crate) fn path_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    // `reference_path` preserves both `::` and `.` separators in the document arena.
    let (rest, (reference, path_kind)) = classified_reference_path(input)?;
    let expr = match path_kind {
        ReferencePathKind::Qualified => Expression::FeatureRef(reference),
        ReferencePathKind::Dotted => Expression::FeatureChainRef(reference),
    };
    Ok((rest, node_from_to(start, rest, expr)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span_input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    macro_rules! reference_is {
        ($input:expr, $id:expr, $expected:expr) => {
            crate::parser::usage::reference_text($input, *$id).as_deref() == Some($expected)
        };
    }

    #[test]
    fn keyword_prefixed_identifiers_stay_plain_identifiers() {
        for text in [
            "newSeq",
            "newValue",
            "notEmpty",
            "order",
            "ordered",
            "assert",
            "assoc",
            "originalReq",
            "asOf",
            "istypeOf",
            "hastypeName",
        ] {
            let input = span_input(text);
            let (rest, node) = expression(input).unwrap_or_else(|e| {
                panic!("expected {text:?} to parse as a plain identifier, got error: {e:?}")
            });
            assert!(rest.fragment().is_empty(), "did not fully consume {text:?}");
            assert!(
                matches!(&node.value, Expression::FeatureRef(s) if reference_is!(input, s, text)),
                "expected FeatureRef({text:?}), got {:?}",
                node.value
            );
        }
    }

    #[test]
    fn literal_prefixed_identifiers_stay_plain_identifiers() {
        for text in ["nullPoint", "trueValue", "falseAlarm", "zeroPoint"] {
            let input = span_input(text);
            let (rest, node) = expression(input).unwrap_or_else(|e| {
                panic!("expected {text:?} to parse as a plain identifier, got error: {e:?}")
            });
            assert!(rest.fragment().is_empty(), "did not fully consume {text:?}");
            assert!(
                matches!(&node.value, Expression::FeatureRef(s) if reference_is!(input, s, text)),
                "expected FeatureRef({text:?}), got {:?}",
                node.value
            );
        }
    }

    #[test]
    fn literal_keywords_still_work_immediately_before_punctuation() {
        // `true`/`false`/`null` are values, not operators: unlike `keyword_token`'s narrow
        // allowlist (whitespace or `{`/`:`/`;`/`[`), they can be legally followed by any
        // non-identifier byte with no space -- a closing paren, comma, or comparison operator.
        // Tested directly against the leaf parsers (not `expression()`) so a comparison operator
        // like `==` can't be misread as evidence the literal itself was rejected: `expression()`
        // would still produce *a* result for `null == x` even if `null_expression` failed here and
        // some other alternative absorbed the input differently.
        let (rest, node) = literal_boolean(span_input("true)")).expect("literal_boolean");
        assert_eq!(rest.fragment(), b")");
        assert!(matches!(&node.value, Expression::LiteralBoolean(true)));

        let (rest, node) = literal_boolean(span_input("false,")).expect("literal_boolean");
        assert_eq!(rest.fragment(), b",");
        assert!(matches!(&node.value, Expression::LiteralBoolean(false)));

        let (rest, node) = null_expression(span_input("null==x")).expect("null_expression");
        assert_eq!(rest.fragment(), b"==x");
        assert!(matches!(&node.value, Expression::Null));
    }

    #[test]
    fn keyword_operators_still_work_with_a_trailing_word_boundary() {
        let input = span_input("notEmpty(x)");
        let (_, node) = expression(input).expect("expression");
        assert!(matches!(&node.value, Expression::Invocation { .. }));

        let input = span_input("not x");
        let (_, node) = expression(input).expect("expression");
        assert!(matches!(
            &node.value,
            Expression::UnaryOp {
                op: UnaryOperator::Not,
                ..
            }
        ));

        let input = span_input("a and b");
        let (_, node) = expression(input).expect("expression");
        assert!(matches!(
            &node.value,
            Expression::BinaryOp {
                op: BinaryOperator::And,
                ..
            }
        ));

        let input = span_input("a && b");
        let (_, node) = expression(input).expect("expression");
        assert!(matches!(
            &node.value,
            Expression::BinaryOp {
                op: BinaryOperator::And,
                ..
            }
        ));

        let input = span_input("a || b");
        let (_, node) = expression(input).expect("expression");
        assert!(matches!(
            &node.value,
            Expression::BinaryOp {
                op: BinaryOperator::Or,
                ..
            }
        ));

        let input = span_input("new A(x)");
        let (_, node) = expression(input).expect("expression");
        assert!(matches!(&node.value, Expression::Constructor { .. }));

        let input = span_input("x istype T");
        let (_, node) = expression(input).expect("expression");
        assert!(matches!(
            &node.value,
            Expression::TypeCheck {
                operand: Some(_),
                ..
            }
        ));
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
                assert!(
                    matches!(&right.value, Expression::FeatureRef(s) if reference_is!(input, s, "c"))
                );
            }
            other => panic!("expected implies, got {other:?}"),
        }
    }

    #[test]
    fn expression_parses_arrow_invocation_as_collection_op() {
        let input = span_input("powerProfile->size()");
        let (_, node) = expression(input).expect("expression");
        match &node.value {
            Expression::CollectionOp { op, base, args, .. } => {
                assert_eq!(op, &CollectionOperator::Size);
                assert!(args.is_empty());
                assert!(
                    matches!(&base.value, Expression::FeatureRef(s) if reference_is!(input, s, "powerProfile"))
                );
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
                    Expression::CollectionOp { op, base, args, .. } => {
                        assert_eq!(op, &CollectionOperator::Other("c".to_string()));
                        assert!(args.is_empty());
                        match &base.value {
                            Expression::MemberAccess {
                                base: inner_base,
                                member: inner_member,
                                ..
                            } => {
                                assert!(reference_is!(input, inner_member, "b"));
                                assert!(
                                    matches!(&inner_base.value, Expression::FeatureRef(s) if reference_is!(input, s, "a"))
                                );
                            }
                            other => panic!(
                                "expected MemberAccess base (bare arrow access), got {other:?}"
                            ),
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
                assert!(reference_is!(input, type_name, "A"));
                assert_eq!(args.len(), 2);
                assert!(args.iter().all(|a| a.parameter.is_none()));
                assert!(
                    matches!(&args[0].value.value, Expression::FeatureRef(s) if reference_is!(input, s, "x"))
                );
                assert!(
                    matches!(&args[1].value.value, Expression::FeatureRef(s) if reference_is!(input, s, "y"))
                );
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
                assert!(reference_is!(input, type_name, "RiskLevel"));
                assert_eq!(args.len(), 1);
                assert!(
                    matches!(args[0].parameter, Some(parameter) if reference_is!(input, &parameter, "probability"))
                );
                assert!(
                    matches!(&args[0].value.value, Expression::FeatureRef(s) if reference_is!(input, s, "LevelEnum::low"))
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
                assert!(reference_is!(input, type_name, "A"));
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
                assert!(
                    matches!(args[0].parameter, Some(parameter) if reference_is!(input, &parameter, "q"))
                );
                assert!(matches!(
                    &args[0].value.value,
                    Expression::LiteralInteger(1)
                ));
                assert!(
                    matches!(args[1].parameter, Some(parameter) if reference_is!(input, &parameter, "p"))
                );
                assert!(
                    matches!(&args[1].value.value, Expression::FeatureRef(s) if reference_is!(input, s, "a"))
                );
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
                assert!(args[0].parameter.is_none());
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
            Expression::CollectionOp { op, base, args, .. } => {
                assert_eq!(op, &CollectionOperator::Collect);
                assert_eq!(args.len(), 1);
                assert!(
                    matches!(&base.value, Expression::FeatureRef(s) if reference_is!(input, s, "items"))
                );
            }
            other => panic!("expected CollectionOp, got {other:?}"),
        }
    }

    #[test]
    fn collection_op_brace_body_retains_parameters_result_and_provenance() {
        let source_text =
            "items->forAll { in ref item : Domain::Item; out accepted; item == selected.item }";
        let source = crate::ast::SourceStorage::from(source_text);
        let context = crate::parser::span::ParseContext::new();
        let (rest, node) = expression(context.input(source_text.as_bytes())).expect("expression");
        assert!(rest.fragment().is_empty());
        let Expression::CollectionOp {
            op,
            args,
            brace_body: Some(body),
            ..
        } = &node.value
        else {
            panic!("expected collection operator with body");
        };
        assert_eq!(op, &CollectionOperator::ForAll);
        assert!(args.is_empty());
        assert_eq!(
            source.slice(&body.span),
            Some("{ in ref item : Domain::Item; out accepted; item == selected.item }")
        );
        assert_eq!(source.slice(&body.value.open_brace_span), Some("{"));
        assert_eq!(source.slice(&body.value.close_brace_span), Some("}"));
        assert_eq!(body.value.parameters.len(), 2);
        let item = &body.value.parameters[0].value;
        assert_eq!(item.direction.value, InOut::In);
        assert_eq!(source.slice(&item.direction.span), Some("in"));
        assert_eq!(
            item.reference_keyword_span
                .as_ref()
                .and_then(|span| source.slice(span)),
            Some("ref")
        );
        assert_eq!(item.name, "item");
        assert_eq!(source.slice(&item.name_span), Some("item"));
        assert_eq!(
            source.slice(&item.typing.as_ref().expect("typing").separator_span),
            Some(":")
        );
        assert_eq!(source.slice(&item.semicolon_span), Some(";"));
        assert!(matches!(
            body.value.result.as_deref().map(|result| &result.value),
            Some(Expression::BinaryOp {
                op: BinaryOperator::Eq,
                ..
            })
        ));
        let arena = context.finish();
        assert_eq!(
            arena
                .get(&source, item.typing.as_ref().expect("typing").target)
                .expect("type reference")
                .authored_text(),
            "Domain::Item"
        );
    }

    #[test]
    fn malformed_collection_op_body_rolls_back_references() {
        let context = crate::parser::span::ParseContext::new();
        assert!(expression(context.input(b"items->forAll { in x : Domain::T x == y }")).is_err());
        assert!(context.finish().is_empty());
    }

    #[test]
    fn path_expression_single_segment_stays_feature_ref() {
        let input = span_input("engine");
        let (_, node) = path_expression(input).expect("path_expression");
        assert!(
            matches!(&node.value, Expression::FeatureRef(s) if reference_is!(input, s, "engine"))
        );
    }

    #[test]
    fn path_expression_multi_segment_becomes_feature_chain_ref() {
        let input = span_input("engine.fuelCmdPort.flowRate");
        let (_, node) = path_expression(input).expect("path_expression");
        match &node.value {
            Expression::FeatureChainRef(reference) => assert!(reference_is!(
                input,
                reference,
                "engine.fuelCmdPort.flowRate"
            )),
            other => panic!("expected FeatureChainRef, got {other:?}"),
        }
    }

    #[test]
    fn path_expression_leading_qualified_name_then_dot_chain() {
        let input = span_input("Foo::bar.baz");
        let (_, node) = path_expression(input).expect("path_expression");
        match &node.value {
            Expression::FeatureChainRef(reference) => {
                assert!(reference_is!(input, reference, "Foo::bar.baz"))
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
                assert!(
                    matches!(&base.value, Expression::FeatureRef(s) if reference_is!(input, s, "x"))
                );
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

    #[test]
    fn tuple_expression_parses_multiple_elements() {
        let input = span_input("(a, b, c)");
        let (_, node) = expression(input).expect("expression");
        match &node.value {
            Expression::Tuple(elements) => {
                assert_eq!(elements.len(), 3);
            }
            other => panic!("expected Tuple, got {other:?}"),
        }
    }

    #[test]
    fn empty_parens_parse_as_null() {
        let input = span_input("()");
        let (_, node) = expression(input).expect("expression");
        assert!(matches!(&node.value, Expression::Null));
    }

    #[test]
    fn index_expression_parses_single_bracketed_expression() {
        let input = span_input("items#(0)");
        let (_, node) = expression(input).expect("expression");
        match &node.value {
            Expression::Index { base, index } => {
                assert!(
                    matches!(&base.value, Expression::FeatureRef(s) if reference_is!(input, s, "items"))
                );
                assert!(matches!(&index.value, Expression::LiteralInteger(0)));
            }
            other => panic!("expected Index, got {other:?}"),
        }
    }

    #[test]
    fn deeply_nested_parentheses_do_not_overflow_the_stack() {
        const DEPTH: usize = 200_000;
        let mut text = String::with_capacity(DEPTH * 2 + 1);
        for _ in 0..DEPTH {
            text.push('(');
        }
        text.push('1');
        for _ in 0..DEPTH {
            text.push(')');
        }
        let input = span_input(&text);
        let (rest, node) = expression(input).expect("expression");
        assert!(rest.fragment().is_empty());
        let mut depth = 0usize;
        let mut current = &node;
        loop {
            match &current.value {
                Expression::Parenthesized(inner) => {
                    depth += 1;
                    current = inner;
                }
                Expression::LiteralInteger(1) => break,
                other => panic!("unexpected node at depth {depth}: {other:?}"),
            }
        }
        assert_eq!(depth, DEPTH);
    }

    /// Spec42 Gap 49c: the `[unit]` annotation applies to tuple/invocation/reference bases in
    /// expression position (`(0, w/2, 0)[source]`, Domain Geometry coordinate-frame idiom),
    /// not just scalar literals.
    #[test]
    fn unit_annotation_applies_to_non_literal_bases() {
        for (source, expect_tuple) in [
            ("(0, w/2, 0)[source]", true),
            ("new Translation((0, w, 0)[source])", false),
            ("angle[deg]", false),
        ] {
            let (rest, node) = expression(crate::parser::span::test_input(source)).expect(source);
            assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
            let dump = format!("{:?}", node.value);
            assert!(
                dump.contains("LiteralWithUnit"),
                "no unit in {source}: {dump}"
            );
            if expect_tuple {
                let Expression::LiteralWithUnit { value, .. } = &node.value else {
                    panic!("expected LiteralWithUnit for {source}");
                };
                assert!(matches!(value.value, Expression::Tuple(_)));
            }
        }
    }

    #[test]
    fn long_postfix_chain_does_not_overflow_the_stack() {
        const DEPTH: usize = 200_000;
        let mut text = String::from("a");
        for _ in 0..DEPTH {
            text.push_str(".b");
        }
        let input = span_input(&text);
        let (rest, node) = expression(input).expect("expression");
        assert!(rest.fragment().is_empty());
        let mut depth = 0usize;
        let mut current = &node;
        loop {
            match &current.value {
                Expression::MemberAccess {
                    base: inner,
                    member,
                    ..
                } => {
                    assert!(reference_is!(input, member, "b"));
                    depth += 1;
                    current = inner;
                }
                Expression::FeatureRef(s) if reference_is!(input, s, "a") => break,
                other => panic!("unexpected node at depth {depth}: {other:?}"),
            }
        }
        assert_eq!(depth, DEPTH);
    }

    #[test]
    fn long_nested_invocation_chain_does_not_overflow_the_stack() {
        const DEPTH: usize = 50_000;
        let mut text = String::new();
        for _ in 0..DEPTH {
            text.push_str("f(");
        }
        text.push('1');
        for _ in 0..DEPTH {
            text.push(')');
        }
        let input = span_input(&text);
        let (rest, node) = expression(input).expect("expression");
        assert!(rest.fragment().is_empty());
        let mut depth = 0usize;
        let mut current = &node;
        loop {
            match &current.value {
                Expression::Invocation { callee, args } => {
                    assert!(
                        matches!(&callee.value, Expression::FeatureRef(s) if reference_is!(input, s, "f"))
                    );
                    assert_eq!(args.len(), 1);
                    depth += 1;
                    current = &args[0].value;
                }
                Expression::LiteralInteger(1) => break,
                other => panic!("unexpected node at depth {depth}: {other:?}"),
            }
        }
        assert_eq!(depth, DEPTH);
    }
}
