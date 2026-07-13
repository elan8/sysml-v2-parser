//! Port definition and port usage parsing.
#![allow(dead_code, unused_imports)]

use crate::ast::{
    Node, PortBody, PortBodyElement, PortDef, PortDefBody, PortDefBodyElement, PortUsage,
};
use crate::parser::action::in_out_decl;
use crate::parser::attribute::{attribute_def, attribute_usage, directed_attribute_usage};
use crate::parser::item::directed_item_usage;
use crate::parser::body::parse_structured_brace_members;
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::expr::expression;
use crate::parser::lex::{
    capture_opaque_member, name, ws1, ws_and_comments, PORT_BODY_STARTERS, PORT_DEF_BODY_STARTERS,
};
use crate::parser::node_from_to;
use crate::parser::requirement::doc_comment;
use crate::parser::usage::{
    multiplicity, optional_typings, prefix_redefinition_target, specialization_clauses,
};
use crate::parser::with_span;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// Port body: `;` or `{` PortBodyElement* `}`.
fn port_body(input: Input<'_>) -> IResult<Input<'_>, PortBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b";"[..]), |_| PortBody::Semicolon),
        port_body_brace,
    ))
    .parse(input)
}

fn port_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PortBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = alt((
        map(port_usage, PortBodyElement::PortUsage),
        map(in_out_decl, PortBodyElement::InOutDecl),
        map(doc_comment, PortBodyElement::Doc),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn port_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<PortBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        PORT_BODY_STARTERS,
        "port body",
        "recovered_port_body_element",
    );
    node_from_to(
        start,
        end,
        PortBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

fn port_body_brace(input: Input<'_>) -> IResult<Input<'_>, PortBody> {
    let (input, elements) = parse_structured_brace_members(
        input,
        PORT_BODY_STARTERS,
        "port body",
        "recovered_port_body_element",
        port_body_element,
        port_body_recovery,
    )?;
    Ok((input, PortBody::Brace { elements }))
}

fn local_name_from_qualified_name(qname: &str) -> String {
    qname.rsplit("::").next().unwrap_or(qname).to_string()
}

/// Port usage: 'port' ( (`:>>`|`redefines`) target | name ) ( ':' type )? multiplicity? clauses? body
pub(crate) fn port_usage(input: Input<'_>) -> IResult<Input<'_>, Node<PortUsage>> {
    enum PortUsageHead {
        PrefixRedefines {
            name_span: crate::ast::Span,
            redefines: String,
        },
        Named {
            name_span: crate::ast::Span,
            name: String,
        },
    }

    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"port"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, usage_head) = alt((
        map(
            preceded(ws_and_comments, prefix_redefinition_target),
            |(name_span, redefines)| PortUsageHead::PrefixRedefines {
                name_span,
                redefines,
            },
        ),
        map(with_span(name), |(name_span, name)| PortUsageHead::Named { name_span, name }),
    ))
    .parse(input)?;
    let (input, name_str, name_span, prefix_redefines) = match usage_head {
        PortUsageHead::PrefixRedefines {
            name_span,
            redefines,
        } => (
            input,
            local_name_from_qualified_name(&redefines),
            name_span,
            Some(redefines),
        ),
        PortUsageHead::Named { name_span, name } => (input, name, name_span, None),
    };
    let (input, type_result) = optional_typings(input)?;
    let (type_ref_span, type_name) = type_result
        .map(|(span, name)| (Some(span), Some(name)))
        .unwrap_or((None, None));
    let (input, multiplicity) = opt(multiplicity).parse(input)?;
    let (input, clauses) = specialization_clauses(input)?;
    let redefines = clauses.redefines.or(prefix_redefines);
    let (input, body) = port_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            PortUsage {
                name: name_str,
                type_name,
                multiplicity,
                subsets: clauses.subsets,
                redefines,
                references: clauses.references,
                crosses: clauses.crosses,
                body,
                name_span: Some(name_span),
                type_ref_span,
            },
        ),
    ))
}

const PORT_DEF_OPAQUE_STARTERS: &[&[u8]] = &[b"ref", b"abstract"];

fn port_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PortDefBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = alt((
        map(directed_item_usage, PortDefBodyElement::ItemUsage),
        map(directed_attribute_usage, PortDefBodyElement::AttributeUsage),
        map(in_out_decl, PortDefBodyElement::InOutDecl),
        map(doc_comment, PortDefBodyElement::Doc),
        map(|i| attribute_def(i, true), PortDefBodyElement::AttributeDef),
        map(attribute_usage, PortDefBodyElement::AttributeUsage),
        map(port_usage, PortDefBodyElement::PortUsage),
        map(
            |i| capture_opaque_member(i, PORT_DEF_OPAQUE_STARTERS),
            PortDefBodyElement::Other,
        ),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn port_def_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<PortDefBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        PORT_DEF_BODY_STARTERS,
        "port definition body",
        "recovered_port_def_body_element",
    );
    node_from_to(
        start,
        end,
        PortDefBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

/// Port def body: `;` or `{` PortDefBodyElement* `}`.
fn port_def_body(input: Input<'_>) -> IResult<Input<'_>, PortDefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b";"[..]), |_| PortDefBody::Semicolon),
        port_def_body_brace,
    ))
    .parse(input)
}

fn port_def_body_brace(input: Input<'_>) -> IResult<Input<'_>, PortDefBody> {
    let (input, elements) = parse_structured_brace_members(
        input,
        PORT_DEF_BODY_STARTERS,
        "port definition body",
        "recovered_port_def_body_element",
        port_def_body_element,
        port_def_body_recovery,
    )?;
    Ok((input, PortDefBody::Brace { elements }))
}

/// Port definition: 'port' 'def' Identification ( (':>' | 'specializes') qualified_name )? body
///
/// `def` is mandatory. `port_def` is only dispatched at package top level today, where nothing
/// else can be confused with it, but making `def` optional would reintroduce the PAR-001 bug
/// class the moment a package-level `port_usage` dispatch is added alongside it.
pub(crate) fn port_def(input: Input<'_>) -> IResult<Input<'_>, Node<PortDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"port").def_required(),
    )?;
    let (input, body) = port_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            PortDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                specializes_span: prefix.specializes_span,
                body,
            },
        ),
    ))
}
