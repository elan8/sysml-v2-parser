//! Interface definition and usage parsing.
#![allow(dead_code, unused_imports)]

use crate::ast::{
    ConnectBody, ConnectStmt, ConnectionEnd, EndDecl, InterfaceDef, InterfaceDefBody,
    InterfaceDefBodyElement, Node, RefBody, RefDecl,
};
use crate::parser::attribute::{attribute_def, attribute_usage};
use crate::parser::body::advance_to_closing_brace;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::expr::path_expression;
use crate::parser::item::{item_def_required, item_usage};
use crate::parser::lex::{
    identification, name, qualified_name, take_until_terminator, ws1, ws_and_comments,
};
use crate::parser::node_from_to;
use crate::parser::port::{port_def_required, port_usage};
use crate::parser::requirement::doc_comment;
use crate::parser::with_span;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// End declaration: `end` name `:` type `;`
fn end_decl(input: Input<'_>) -> IResult<Input<'_>, Node<EndDecl>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"end"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) =
        nom::combinator::opt(preceded(ws_and_comments, tag(&b"port"[..]))).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (name_span, name_str)) = with_span(name).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
    let (input, (tilde, (type_ref_span, type_name))) = preceded(
        ws_and_comments,
        (
            nom::combinator::opt(tag(&b"~"[..])),
            with_span(qualified_name),
        ),
    )
    .parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            EndDecl {
                name: name_str,
                type_name: if tilde.is_some() {
                    format!("~{}", type_name)
                } else {
                    type_name
                },
                uses_derived_syntax: false,
                name_span: Some(name_span),
                type_ref_span: Some(type_ref_span),
            },
        ),
    ))
}

/// Ref body: `;` or `{` ... `}`
fn ref_body(input: Input<'_>) -> IResult<Input<'_>, RefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b";"[..]), |_| RefBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag(&b"{"[..]),
                advance_to_closing_brace,
                preceded(ws_and_comments, tag(&b"}"[..])),
            ),
            |_| RefBody::Brace { elements: vec![] },
        ),
    ))
    .parse(input)
}

/// Ref declaration: `ref` name `:` type body
fn ref_decl(input: Input<'_>) -> IResult<Input<'_>, Node<RefDecl>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"ref"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, (name_span, name_str)) = with_span(name).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
    let (input, (type_ref_span, type_name)) =
        preceded(ws_and_comments, with_span(qualified_name)).parse(input)?;
    let (input, body) = ref_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            RefDecl {
                name: name_str,
                type_name,
                value: None,
                body,
                name_span: Some(name_span),
                type_ref_span: Some(type_ref_span),
            },
        ),
    ))
}

/// Connect body: `;` or `{` ... `}`
pub(crate) fn connect_body(input: Input<'_>) -> IResult<Input<'_>, ConnectBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b";"[..]), |_| ConnectBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag(&b"{"[..]),
                advance_to_closing_brace,
                preceded(ws_and_comments, tag(&b"}"[..])),
            ),
            |_| ConnectBody::Brace,
        ),
    ))
    .parse(input)
}

/// Wrap a parsed endpoint expression in a `ConnectionEnd` node, reusing the expression's own
/// span (see `ast::core::ConnectionEnd`'s doc comment). This file's ends are semantically
/// "interface ends" (`InterfaceEnd` is a type alias for `ConnectionEnd` -- checked that an
/// interface end carries nothing beyond a generic connection end, see that alias's doc comment).
fn connect_end(expr: Node<crate::ast::Expression>) -> Node<ConnectionEnd> {
    let span = expr.span.clone();
    Node::new(
        span.clone(),
        ConnectionEnd {
            expression: expr,
            span,
        },
    )
}

/// Connect ends: the n-ary `'(' end (',' end)+ ')'` form (`NaryInterfacePart`), or the ordinary
/// binary `from ... to ...` form. Returns `(from, to, extra_ends)`.
fn connect_ends(
    input: Input<'_>,
) -> IResult<Input<'_>, (Node<ConnectionEnd>, Node<ConnectionEnd>, Vec<Node<ConnectionEnd>>)> {
    alt((
        map(
            (
                preceded(ws_and_comments, tag(&b"("[..])),
                preceded(ws_and_comments, path_expression),
                nom::multi::many1(preceded(
                    preceded(ws_and_comments, tag(&b","[..])),
                    preceded(ws_and_comments, path_expression),
                )),
                preceded(ws_and_comments, tag(&b")"[..])),
            ),
            |(_, first, mut rest, _)| {
                let to = rest.remove(0);
                (
                    connect_end(first),
                    connect_end(to),
                    rest.into_iter().map(connect_end).collect(),
                )
            },
        ),
        map(
            (
                path_expression,
                preceded(ws_and_comments, tag(&b"to"[..])),
                preceded(ws_and_comments, path_expression),
            ),
            |(from, _, to)| (connect_end(from), connect_end(to), Vec::new()),
        ),
    ))
    .parse(input)
}

/// Connect statement: `connect` from `to` to body, or `connect` `(` a `,` b (`,` c)* `)` body
fn connect_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<ConnectStmt>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"connect"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, (from_expr, to_expr, extra_ends)) = connect_ends(input)?;
    let (input, body) = connect_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ConnectStmt {
                from: from_expr,
                to: to_expr,
                extra_ends,
                body,
            },
        ),
    ))
}

fn interface_def_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<InterfaceDefBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, elem) = alt((
        map(doc_comment, InterfaceDefBodyElement::Doc),
        map(end_decl, InterfaceDefBodyElement::EndDecl),
        map(ref_decl, InterfaceDefBodyElement::RefDecl),
        map(connect_stmt, InterfaceDefBodyElement::ConnectStmt),
        // PAR-002 widening: this body previously had no attribute/item/port coverage at all.
        // `item_def_required`/`port_def_required` tried before their usage siblings, same
        // def-before-usage discipline as the other body enums wired in prior increments (their
        // usage parsers have no guard against a bare `def` token).
        map(
            |i| attribute_def(i, true),
            InterfaceDefBodyElement::AttributeDef,
        ),
        map(attribute_usage, InterfaceDefBodyElement::AttributeUsage),
        map(item_def_required, InterfaceDefBodyElement::ItemDef),
        map(item_usage, InterfaceDefBodyElement::ItemUsage),
        map(port_def_required, InterfaceDefBodyElement::PortDef),
        map(port_usage, InterfaceDefBodyElement::PortUsage),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

/// Interface def body: `;` or `{` InterfaceDefBodyElement* `}`
fn interface_def_body(input: Input<'_>) -> IResult<Input<'_>, InterfaceDefBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, InterfaceDefBody::Semicolon));
    }
    let (input, _) = tag(&b"{"[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, elements) =
        many0(preceded(ws_and_comments, interface_def_body_element)).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = if input.fragment().starts_with(b"}") {
        (input, ())
    } else {
        advance_to_closing_brace(input)?
    };
    let (input, _) = preceded(ws_and_comments, tag(&b"}"[..])).parse(input)?;
    Ok((input, InterfaceDefBody::Brace { elements }))
}

/// Interface definition: `interface` `def` Identification body
///
/// `def` is optional here: the standard library uses bare, `def`-less `interface` usages at
/// namespace level (e.g. `abstract interface interfaces: Interface[0..*] nonunique :>
/// connections { ... }` in `Systems Library/Interfaces.sysml`), and there is no dedicated
/// package-level `interface_usage` dispatch to catch them instead — this parser currently folds
/// that legal form into `InterfaceDef`. Use [`interface_def_required`] in any body context that
/// also dispatches `interface_usage`.
pub(crate) fn interface_def(input: Input<'_>) -> IResult<Input<'_>, Node<InterfaceDef>> {
    parse_interface_def(input, false)
}

/// Interface definition with a mandatory `def` keyword: for body contexts (e.g. part-def bodies)
/// that also dispatch `interface_usage`. `interface_usage` only recognizes connector forms
/// (`connect ... to ...`), so an optional `def` would let a non-connector interface usage (e.g.
/// `interface foo : IfaceType;`) be silently misclassified as a definition — the same bug class
/// as PAR-001 in `attribute_def`.
pub(crate) fn interface_def_required(input: Input<'_>) -> IResult<Input<'_>, Node<InterfaceDef>> {
    parse_interface_def(input, true)
}

fn parse_interface_def(
    input: Input<'_>,
    require_def: bool,
) -> IResult<Input<'_>, Node<InterfaceDef>> {
    let start = input;
    let mut options = DefinitionPrefixOptions::new(b"interface");
    if require_def {
        options = options.def_required();
    }
    let (input, prefix) = parse_definition_prefix(input, options)?;
    let (input, body) = interface_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            InterfaceDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
            },
        ),
    ))
}

#[cfg(test)]
mod par_002_widening_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    #[test]
    fn interface_def_body_accepts_nested_attribute_usage() {
        let (rest, node) =
            interface_def_body_element(input("attribute mass: Real;")).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            InterfaceDefBodyElement::AttributeUsage(_)
        ));
    }

    #[test]
    fn interface_def_body_accepts_nested_item_def_not_misparsed_as_usage() {
        let (rest, node) =
            interface_def_body_element(input("item def MyItem;")).expect("item def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, InterfaceDefBodyElement::ItemDef(_)));
    }

    #[test]
    fn interface_def_body_accepts_nested_item_usage() {
        let (rest, node) =
            interface_def_body_element(input("item i1: MyItem;")).expect("item usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, InterfaceDefBodyElement::ItemUsage(_)));
    }

    #[test]
    fn interface_def_body_accepts_nested_port_def_not_misparsed_as_usage() {
        let (rest, node) =
            interface_def_body_element(input("port def MyPort;")).expect("port def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, InterfaceDefBodyElement::PortDef(_)));
    }

    #[test]
    fn interface_def_body_accepts_nested_port_usage() {
        let (rest, node) =
            interface_def_body_element(input("port p1: MyPort;")).expect("port usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, InterfaceDefBodyElement::PortUsage(_)));
    }

    /// PAR-002 acceptance criterion: `port def` yields the same underlying parse (via the shared
    /// `port_def_required` parser) whether reached through `InterfaceDefBodyElement::PortDef` or
    /// any other body enum wired to the same parser in prior increments (e.g.
    /// `PartDefBodyElement::PortDef`).
    #[test]
    fn port_def_is_same_variant_kind_in_interface_def_body_as_port_def_required_parser() {
        let text = "port def MyPort;";
        let (_, iface_node) =
            interface_def_body_element(input(text)).expect("nested in interface def body");
        assert!(matches!(
            iface_node.value,
            InterfaceDefBodyElement::PortDef(_)
        ));
        let result = port_def_required(input(text));
        assert!(result.is_ok(), "port_def_required should also accept {text:?}");
    }
}
