#![allow(dead_code, unused_imports)]

use crate::ast::{
    CalcDef, CalcDefBody, CalcDefBodyElement, CalcUsage, ConstraintDef, ConstraintDefBody,
    ConstraintDefBodyElement, Expression, Node, ParseErrorNode, ReturnDecl,
};
use crate::parser::action::in_out_decl;
use crate::parser::body::parse_structured_brace_members;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::expr::expression;
use crate::parser::lex::{
    identification, name, qualified_name, recover_body_element, skip_statement_or_block,
    starts_with_any_keyword, starts_with_keyword, take_until_terminator, ws1, ws_and_comments,
    CALC_DEF_BODY_STARTERS, CONSTRAINT_DEF_BODY_STARTERS,
};
use crate::parser::Input;
use crate::parser::{build_recovery_error_node_from_span, node_from_to};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

/// `def` is intentionally optional: the standard library uses bare, `def`-less `constraint`
/// usages at namespace level (e.g. `abstract constraint constraintChecks: ConstraintCheck[0..*]
/// nonunique :> booleanEvaluations { ... }` in `Systems Library/Constraints.sysml`), and there is
/// no dedicated `constraint_usage` parser to catch them instead — this parser currently folds
/// that legal form into `ConstraintDef`. Do not add `.def_required()` here without first adding
/// real constraint-usage support.
pub(crate) fn constraint_def(input: Input<'_>) -> IResult<Input<'_>, Node<ConstraintDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"constraint").with_private(),
    )?;
    let (input, body) = constraint_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ConstraintDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
            },
        ),
    ))
}

fn constraint_def_body(input: Input<'_>) -> IResult<Input<'_>, ConstraintDefBody> {
    let (input, body) = structured_constraint_body(input)?;
    let body = match body {
        StructuredConstraintBody::Semicolon => ConstraintDefBody::Semicolon,
        StructuredConstraintBody::Brace { elements } => ConstraintDefBody::Brace { elements },
    };
    Ok((input, body))
}

pub(crate) enum StructuredConstraintBody {
    Semicolon,
    Brace {
        elements: Vec<Node<ConstraintDefBodyElement>>,
    },
}

fn constraint_body_recovery_element(
    start: Input<'_>,
    end: Input<'_>,
) -> Node<ConstraintDefBodyElement> {
    if starts_with_any_keyword(start.fragment(), CONSTRAINT_DEF_BODY_STARTERS) {
        let recovery = build_recovery_error_node_from_span(
            start,
            end,
            CONSTRAINT_DEF_BODY_STARTERS,
            "constraint body",
            "recovered_constraint_body_element",
        );
        let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
        return node_from_to(start, end, ConstraintDefBodyElement::Error(node));
    }
    let preview = String::from_utf8_lossy(&start.fragment()[..start.fragment().len().min(120)])
        .trim()
        .to_string();
    node_from_to(start, end, ConstraintDefBodyElement::Other(preview))
}

pub(crate) fn structured_constraint_body(
    input: Input<'_>,
) -> IResult<Input<'_>, StructuredConstraintBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, StructuredConstraintBody::Semicolon));
    }
    let (input, elements) = parse_structured_brace_members(
        input,
        CONSTRAINT_DEF_BODY_STARTERS,
        "constraint body",
        "recovered_constraint_body_element",
        constraint_def_body_element,
        constraint_body_recovery_element,
    )?;
    Ok((input, StructuredConstraintBody::Brace { elements }))
}

pub(crate) fn constraint_def_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<ConstraintDefBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = if starts_with_keyword(input.fragment(), b"doc") {
        map(
            crate::parser::requirement::doc_comment,
            ConstraintDefBodyElement::Doc,
        )
        .parse(input)?
    } else if input.fragment().starts_with(b"@") {
        map(
            crate::parser::metadata_annotation::metadata_annotation,
            ConstraintDefBodyElement::MetadataAnnotation,
        )
        .parse(input)?
    } else if starts_with_keyword(input.fragment(), b"in")
        || starts_with_keyword(input.fragment(), b"out")
        || starts_with_keyword(input.fragment(), b"inout")
    {
        if named_in_out_missing_type(input) {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
        map(in_out_decl, ConstraintDefBodyElement::InOutDecl).parse(input)?
    } else {
        map(expression, ConstraintDefBodyElement::Expression).parse(input)?
    };
    let (input, _) = opt(preceded(ws_and_comments, tag(&b";"[..]))).parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn safe_constraint_def_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<ConstraintDefBodyElement>> {
    let start = input;
    let mut parser = alt((
        map(in_out_decl, |n| {
            node_from_to(start, input, ConstraintDefBodyElement::InOutDecl(n))
        }),
        map(expression, |n| {
            node_from_to(start, input, ConstraintDefBodyElement::Expression(n))
        }),
    ));
    parser.parse(input)
}

/// Calculation usage: `calc` Identification (`:` type)? body (SysML §7.19.2).
pub(crate) fn calc_usage(input: Input<'_>) -> IResult<Input<'_>, Node<CalcUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"calc"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, identification) = identification(input)?;
    let (input, type_name) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let (input, body) = calc_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            CalcUsage {
                identification,
                type_name,
                body,
            },
        ),
    ))
}

/// `def` is intentionally optional: the standard library uses bare, `def`-less `calc` usages at
/// namespace level (e.g. `abstract calc calculations: Calculation[0..*] nonunique :> actions,
/// evaluations { ... }` in `Systems Library/Calculations.sysml`). `calc_def` and `calc_usage` are
/// never dispatched together in the same alt today (`calc_usage` is only used standalone in part
/// bodies), so this is not the PAR-001 bug class, but do not add `.def_required()` here without
/// checking package-level content first.
pub(crate) fn calc_def(input: Input<'_>) -> IResult<Input<'_>, Node<CalcDef>> {
    let start = input;
    let (input, prefix) =
        parse_definition_prefix(input, DefinitionPrefixOptions::new(b"calc").with_private())?;
    let (input, body) = calc_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            CalcDef {
                identification: prefix.identification,
                body,
            },
        ),
    ))
}

fn calc_body_recovery_element(start: Input<'_>, end: Input<'_>) -> Node<CalcDefBodyElement> {
    if starts_with_any_keyword(start.fragment(), CALC_DEF_BODY_STARTERS) {
        let recovery = build_recovery_error_node_from_span(
            start,
            end,
            CALC_DEF_BODY_STARTERS,
            "calc body",
            "recovered_calc_body_element",
        );
        let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
        return node_from_to(start, end, CalcDefBodyElement::Error(node));
    }
    let frag = start.fragment();
    let take = frag.len().min(120);
    let preview = String::from_utf8_lossy(&frag[..take]).trim().to_string();
    node_from_to(start, end, CalcDefBodyElement::Other(preview))
}

fn calc_def_body(input: Input<'_>) -> IResult<Input<'_>, CalcDefBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, CalcDefBody::Semicolon));
    }
    let (input, elements) = parse_structured_brace_members(
        input,
        CALC_DEF_BODY_STARTERS,
        "calc body",
        "recovered_calc_body_element",
        calc_def_body_element,
        calc_body_recovery_element,
    )?;
    Ok((input, CalcDefBody::Brace { elements }))
}

fn calc_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<CalcDefBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = if starts_with_keyword(input.fragment(), b"doc") {
        map(
            crate::parser::requirement::doc_comment,
            CalcDefBodyElement::Doc,
        )
        .parse(input)?
    } else if input.fragment().starts_with(b"@") {
        map(
            crate::parser::metadata_annotation::metadata_annotation,
            CalcDefBodyElement::MetadataAnnotation,
        )
        .parse(input)?
    } else if starts_with_keyword(input.fragment(), b"in")
        || starts_with_keyword(input.fragment(), b"out")
        || starts_with_keyword(input.fragment(), b"inout")
    {
        if named_in_out_missing_type(input) {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
        map(in_out_decl, CalcDefBodyElement::InOutDecl).parse(input)?
    } else if starts_with_keyword(input.fragment(), b"return") {
        if named_return_missing_type(input) {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
        if let Ok((input, decl)) = return_decl(input) {
            (input, CalcDefBodyElement::ReturnDecl(decl))
        } else if let Ok((input, expr)) = return_expression_stmt(input) {
            (input, CalcDefBodyElement::Expression(expr))
        } else {
            other_calc_return(input)?
        }
    } else {
        map(expression, CalcDefBodyElement::Expression).parse(input)?
    };
    let (input, _) = opt(preceded(ws_and_comments, tag(&b";"[..]))).parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn named_in_out_missing_type(input: Input<'_>) -> bool {
    let Ok((after_ws, _)) = ws_and_comments(input) else {
        return false;
    };
    let direction_len = if starts_with_keyword(after_ws.fragment(), b"inout") {
        5
    } else if starts_with_keyword(after_ws.fragment(), b"out") {
        3
    } else {
        2
    };
    let mut rest = &after_ws.fragment()[direction_len..];
    while let Some(first) = rest.first() {
        if first.is_ascii_whitespace() {
            rest = &rest[1..];
        } else {
            break;
        }
    }
    if rest.starts_with(b":") {
        return false;
    }
    let mut name_len = 0usize;
    while name_len < rest.len()
        && (rest[name_len].is_ascii_alphanumeric() || rest[name_len] == b'_')
    {
        name_len += 1;
    }
    if name_len == 0 {
        return false;
    }
    rest = &rest[name_len..];
    while let Some(first) = rest.first() {
        if first.is_ascii_whitespace() {
            rest = &rest[1..];
        } else {
            break;
        }
    }
    if !rest.starts_with(b":") {
        return false;
    }
    rest = &rest[1..];
    while let Some(first) = rest.first() {
        if first.is_ascii_whitespace() {
            rest = &rest[1..];
        } else {
            break;
        }
    }
    rest.is_empty() || rest.starts_with(b";") || rest.starts_with(b"{") || rest.starts_with(b"}")
}

fn named_return_missing_type(input: Input<'_>) -> bool {
    let Ok((after_ws, _)) = ws_and_comments(input) else {
        return false;
    };
    let mut rest = *after_ws.fragment();
    if !starts_with_keyword(rest, b"return") {
        return false;
    }
    rest = &rest[6..];
    while let Some(first) = rest.first() {
        if first.is_ascii_whitespace() {
            rest = &rest[1..];
        } else {
            break;
        }
    }
    if rest.starts_with(b":") {
        return false;
    }
    let mut name_len = 0usize;
    while name_len < rest.len()
        && (rest[name_len].is_ascii_alphanumeric() || rest[name_len] == b'_')
    {
        name_len += 1;
    }
    if name_len == 0 {
        return false;
    }
    rest = &rest[name_len..];
    while let Some(first) = rest.first() {
        if first.is_ascii_whitespace() {
            rest = &rest[1..];
        } else {
            break;
        }
    }
    if rest.starts_with(b":") {
        rest = &rest[1..];
        while let Some(first) = rest.first() {
            if first.is_ascii_whitespace() {
                rest = &rest[1..];
            } else {
                break;
            }
        }
        return rest.is_empty()
            || rest.starts_with(b";")
            || rest.starts_with(b"{")
            || rest.starts_with(b"}");
    }
    false
}

/// `return sum(parts.massKg);` — expression return (SysML calc / return-ref body), not `return name : Type;`.
pub(crate) fn return_expression_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let (input, _) = preceded(ws_and_comments, tag(&b"return"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, expr) = expression(input)?;
    let (input, _) = opt(preceded(ws_and_comments, tag(&b";"[..]))).parse(input)?;
    Ok((input, expr))
}

fn other_calc_return(input: Input<'_>) -> IResult<Input<'_>, CalcDefBodyElement> {
    let start_unknown = input;
    let (next, _) = skip_statement_or_block(input)?;
    if next.location_offset() == start_unknown.location_offset() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Many0,
        )));
    }
    let preview = String::from_utf8_lossy(
        &start_unknown.fragment()[..start_unknown.fragment().len().min(120)],
    )
    .trim()
    .to_string();
    Ok((next, CalcDefBodyElement::Other(preview)))
}

pub(crate) fn return_decl(input: Input<'_>) -> IResult<Input<'_>, Node<ReturnDecl>> {
    let start = input;
    let (input, _) = tag(&b"return"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, n) = name(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
    let (input, type_name) = preceded(ws_and_comments, qualified_name).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(start, input, ReturnDecl { name: n, type_name }),
    ))
}
