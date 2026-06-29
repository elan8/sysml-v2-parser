use crate::ast::{Expression, FlowDef, FlowUsage, FlowUsageKind, Node};

type FlowEndpoints<'a> = nom::IResult<Input<'a>, (Option<Node<Expression>>, Option<Node<Expression>>)>;
use crate::parser::body::semicolon_or_structured_definition_body;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::expr::expression;
use crate::parser::lex::{name, starts_with_keyword, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::usage::feature_usage_header;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

pub(crate) fn flow_def(input: Input<'_>) -> IResult<Input<'_>, Node<FlowDef>> {
    let start = input;
    let (input, prefix) =
        parse_definition_prefix(input, DefinitionPrefixOptions::new(b"flow").def_required())?;
    let (input, body) = semicolon_or_structured_definition_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            FlowDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                specializes_span: prefix.specializes_span,
                body,
            },
        ),
    ))
}

fn flow_usage_keyword(input: Input<'_>) -> IResult<Input<'_>, FlowUsageKind> {
    alt((
        map(
            preceded(
                tag(&b"succession"[..]),
                preceded(ws1, tag(&b"flow"[..])),
            ),
            |_| FlowUsageKind::SuccessionFlow,
        ),
        map(tag(&b"message"[..]), |_| FlowUsageKind::Message),
        map(tag(&b"flow"[..]), |_| FlowUsageKind::Flow),
    ))
    .parse(input)
}

fn optional_payload(input: Input<'_>) -> IResult<Input<'_>, Option<Node<crate::ast::Expression>>> {
    let (peek, _) = ws_and_comments(input)?;
    if starts_with_keyword(peek.fragment(), b"of") {
        let (input, _) = preceded(ws_and_comments, tag(&b"of"[..])).parse(input)?;
        let (input, payload) = preceded(ws1, expression).parse(input)?;
        Ok((input, Some(payload)))
    } else {
        Ok((input, None))
    }
}

fn flow_endpoints(input: Input<'_>) -> FlowEndpoints<'_> {
    let (peek, _) = ws_and_comments(input)?;
    let fragment = peek.fragment();
    if fragment.starts_with(b";") || fragment.starts_with(b"{") {
        return Ok((input, (None, None)));
    }
    if starts_with_keyword(peek.fragment(), b"from") {
        let (input, _) = preceded(ws_and_comments, tag(&b"from"[..])).parse(input)?;
        let (input, from) = preceded(ws1, expression).parse(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
        let (input, to) = preceded(ws1, expression).parse(input)?;
        return Ok((input, (Some(from), Some(to))));
    }
    // Shorthand: expr `to` expr (no `from` keyword).
    let (input, from) = expression(input)?;
    let (peek, _) = ws_and_comments(input)?;
    if starts_with_keyword(peek.fragment(), b"to") {
        let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
        let (input, to) = preceded(ws1, expression).parse(input)?;
        Ok((input, (Some(from), Some(to))))
    } else {
        Ok((input, (None, None)))
    }
}

fn flow_usage_named(input: Input<'_>) -> IResult<Input<'_>, FlowUsage> {
    let (input, name_str) = name(input)?;
    let (input, header) = feature_usage_header(input)?;
    let (input, payload) = optional_payload(input)?;
    let (input, (from, to)) = flow_endpoints(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = semicolon_or_structured_definition_body(input)?;
    Ok((
        input,
        FlowUsage {
            kind: FlowUsageKind::Flow, // overwritten by caller
            name: Some(name_str),
            type_name: header.type_name,
            payload,
            from,
            to,
            body,
        },
    ))
}

fn flow_usage_anonymous(input: Input<'_>) -> IResult<Input<'_>, FlowUsage> {
    let (input, from) = expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
    let (input, to) = preceded(ws1, expression).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = semicolon_or_structured_definition_body(input)?;
    Ok((
        input,
        FlowUsage {
            kind: FlowUsageKind::Flow,
            name: None,
            type_name: None,
            payload: None,
            from: Some(from),
            to: Some(to),
            body,
        },
    ))
}

/// Unified FlowUsage parser for all structure-usage body contexts.
pub(crate) fn flow_usage_member(input: Input<'_>) -> IResult<Input<'_>, Node<FlowUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, kind) = flow_usage_keyword(input)?;
    let (input, _) = ws1(input)?;

    let (input, mut usage) = {
        let peek = input;
        match name(peek) {
            Ok((after_name, _name_str)) => {
                let (after_name, _) = ws_and_comments(after_name)?;
                let fragment = after_name.fragment();
                let is_anonymous = fragment.starts_with(b".")
                    || starts_with_keyword(fragment, b"to");
                if is_anonymous {
                    flow_usage_anonymous(peek)?
                } else {
                    flow_usage_named(peek)?
                }
            }
            Err(_) => flow_usage_anonymous(input)?,
        }
    };
    usage.kind = kind;
    Ok((input, node_from_to(start, input, usage)))
}

/// Package-level flow usage (alias for `flow_usage_member`).
pub(crate) fn flow_usage(input: Input<'_>) -> IResult<Input<'_>, Node<FlowUsage>> {
    flow_usage_member(input)
}
