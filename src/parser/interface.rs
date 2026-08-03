//! Interface definition and usage parsing.

use crate::ast::{InterfaceDef, InterfaceDefBody, InterfaceDefBodyElement, Membership, Node};
use crate::parser::attribute::{attribute_def, attribute_usage};
use crate::parser::body::parse_structured_brace_members;
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::connector::{connect_stmt, end_decl, ref_decl};
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::item::{item_def_required, item_usage};
use crate::parser::lex::{ws_and_comments, INTERFACE_DEF_BODY_STARTERS};
use crate::parser::node_from_to;
use crate::parser::port::{port_def_required, port_usage};
use crate::parser::requirement::doc_comment;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::Parser;

fn interface_def_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<InterfaceDefBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, elem) = alt((
        map(doc_comment, InterfaceDefBodyElement::Doc),
        // GH-33: interfaces don't allow the `#name` derived-end-name form connections do (no
        // matching real-usage evidence found for interfaces) -- see `connector::end_decl`'s doc
        // comment.
        map(|i| end_decl(i, false), InterfaceDefBodyElement::EndDecl),
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

fn interface_def_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<InterfaceDefBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        INTERFACE_DEF_BODY_STARTERS,
        "interface definition body",
        "recovered_interface_def_body_element",
    );
    node_from_to(
        start,
        end,
        InterfaceDefBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

/// Interface def body: `;` or `{` InterfaceDefBodyElement* `}`
///
/// GH-51: previously a hand-rolled `many0` loop that fell back to `advance_to_closing_brace` with
/// no diagnostic at all when an element failed to parse -- unlike `connection_member_body`
/// (`src/parser/connection.rs`), which already used `parse_structured_brace_members` and surfaced
/// a real `Error` element via a recovery callback for the same situation. Now routed through the
/// same shared recovery machinery.
fn interface_def_body(input: Input<'_>) -> IResult<Input<'_>, InterfaceDefBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, InterfaceDefBody::Semicolon));
    }
    let (input, elements) = parse_structured_brace_members(
        input,
        INTERFACE_DEF_BODY_STARTERS,
        "interface definition body",
        "recovered_interface_def_body_element",
        interface_def_body_element,
        interface_def_body_recovery,
    )?;
    Ok((input, InterfaceDefBody::Brace { elements }))
}

/// Interface definition: `interface` `def` Identification body
///
/// `def` is optional here: the standard library uses bare, `def`-less `interface` usages at
/// namespace level (e.g. `abstract interface interfaces: Interface[0..*] nonunique :>
/// connections { ... }` in `Systems Library/Interfaces.sysml`), a shape a `def_required` usage
/// parser can't recover. `parse_interface_def` sets `.reject_header_keyword(b"connect")` (mirror
/// of `connection_def`'s PAR-007 fix, same rationale) so a package-level `interface iface :
/// PowerInterface connect a to b;` usage -- which used to be misclassified as `InterfaceDef` with
/// its `connect` clause silently discarded -- fails this parser and falls through to
/// `interface_usage` (`part::usage::interface_usage`, now also dispatched at package level; see
/// `package.rs`) instead of being swallowed here. The bare abstract/multiplicity Systems Library
/// shape above never contains a `connect` keyword, so it is unaffected.
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
    let mut options = DefinitionPrefixOptions::new(b"interface")
        .with_captured_visibility()
        .reject_header_keyword(b"connect");
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
                membership: Membership::owning(prefix.visibility, prefix.visibility_span),
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
        let (rest, node) = interface_def_body_element(input("item def MyItem;")).expect("item def");
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
        let (rest, node) = interface_def_body_element(input("port def MyPort;")).expect("port def");
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
        assert!(
            result.is_ok(),
            "port_def_required should also accept {text:?}"
        );
    }
}

#[cfg(test)]
mod membership_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    // --- parser work item 4b (final sweep): Membership on InterfaceDef ---

    #[test]
    fn interface_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) =
            interface_def(input("private interface def I1;")).expect("interface def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Private)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }

    #[test]
    fn interface_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = interface_def(input("interface def I1;")).expect("interface def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }
}
