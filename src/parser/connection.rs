//! Connection definition parsing (BNF ConnectionDefinition).

use crate::ast::{ConnectionDef, ConnectionDefBody, ConnectionDefBodyElement, Node};
use crate::parser::attribute::{attribute_def, attribute_usage};
use crate::parser::body::parse_structured_brace_members;
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::connector::{connect_stmt, end_decl, ref_decl};
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::item::{item_def_required, item_usage};
use crate::parser::lex::{ws_and_comments, CONNECTION_DEF_BODY_STARTERS};
use crate::parser::node_from_to;
use crate::parser::occurrence_body::{
    assert_constraint_member, occurrence_usage, succession_usage,
};
use crate::parser::part::part_usage;
use crate::parser::port::{port_def_required, port_usage};
use crate::parser::requirement::doc_comment;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::Parser;

fn connection_def_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<ConnectionDefBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, elem) = alt((
        // GH-33: connections allow the fixed `#original`/`#derive` end-role form (tested real usage; see
        // `connector::end_decl`'s doc comment); interfaces don't.
        map(|i| end_decl(i, true), ConnectionDefBodyElement::EndDecl),
        map(ref_decl, ConnectionDefBodyElement::RefDecl),
        map(connect_stmt, ConnectionDefBodyElement::ConnectStmt),
        map(doc_comment, ConnectionDefBodyElement::Doc),
        // PAR-002 widening: this body previously had no attribute/item/port coverage at all.
        // Same def-before-usage discipline as `InterfaceDefBodyElement`/other body enums.
        map(
            |i| attribute_def(i, true),
            ConnectionDefBodyElement::AttributeDef,
        ),
        map(attribute_usage, ConnectionDefBodyElement::AttributeUsage),
        map(item_def_required, ConnectionDefBodyElement::ItemDef),
        map(item_usage, ConnectionDefBodyElement::ItemUsage),
        map(port_def_required, ConnectionDefBodyElement::PortDef),
        map(port_usage, ConnectionDefBodyElement::PortUsage),
        // GH-51: real Systems/Domain Library connection defs use these member kinds too --
        // see `ConnectionDefBodyElement`'s doc comment for the exact real-usage citations.
        map(
            assert_constraint_member,
            ConnectionDefBodyElement::AssertConstraint,
        ),
        map(occurrence_usage, |n| {
            ConnectionDefBodyElement::OccurrenceUsage(Box::new(n))
        }),
        map(succession_usage, ConnectionDefBodyElement::SuccessionUsage),
        // GH-89: bare `part p;` member, e.g. `abstract connection def C { part p; end end1; }`
        // (Simple Tests/ConnectionTest.sysml:31).
        map(part_usage, |p| {
            ConnectionDefBodyElement::PartUsage(Box::new(p))
        }),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn connection_def_body_recovery(
    start: Input<'_>,
    end: Input<'_>,
) -> Node<ConnectionDefBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        CONNECTION_DEF_BODY_STARTERS,
        "connection definition body",
        "recovered_connection_def_body_element",
    );
    node_from_to(
        start,
        end,
        ConnectionDefBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

pub(crate) fn connection_member_body(input: Input<'_>) -> IResult<Input<'_>, ConnectionDefBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let semicolon_start = input;
        let (input, _) = tag(&b";"[..]).parse(semicolon_start)?;
        return Ok((
            input,
            ConnectionDefBody::Semicolon {
                semicolon_span: crate::parser::span::span_from_to(semicolon_start, input),
            },
        ));
    }
    let (input, members) = parse_structured_brace_members(
        input,
        CONNECTION_DEF_BODY_STARTERS,
        "connection definition body",
        "recovered_connection_def_body_element",
        connection_def_body_element,
        connection_def_body_recovery,
    )?;
    Ok((input, members.into_body()))
}

/// Connection definition: `connection def` Identification body.
///
/// `def` is intentionally optional, same rationale (and same real-library evidence) as
/// `port_def`/`calc_def`/`constraint_def`: the Systems Library uses bare, `def`-less `connection`
/// declarations at namespace level with `abstract`, multiplicity, `nonunique`, and `:>` subsets
/// before the body (e.g. `abstract connection connections: Connection[0..*] nonunique :>
/// linkObjects, parts { ... }` in `Systems Library/Connections.sysml`), a shape
/// `connection_usage_member` (`src/parser/part/body.rs`) does not parse (no `abstract`/
/// multiplicity/`nonunique` support, and its `:>`/`:>>` handling is trailing-after-body only).
/// `parse_definition_prefix`'s header parsing (`specialization::
/// parse_optional_definition_header_after_identification`, a generic text-scan for `: Type[mult]
/// nonunique :> target`) already accepts this whole shape, and identification's `name` is
/// optional, so `connection_def` is effectively a grammar superset of `connection_usage_member`
/// for every practical package-level input.
///
/// **PAR-006b audit note**: `connection_usage_member` is also dispatched at package level
/// (`package.rs::try_package_body_structure`, right after `connection_def`, added by PAR-002).
/// This was investigated as a possible PAR-001-class def/usage ambiguity (an earlier draft of
/// this comment claimed "nothing else shares the `connection` keyword" at package level, which
/// PAR-002 made stale). Making `def` conditionally required here (mirroring the `connection`
/// keyword inside part bodies, where `connection_def_required` is safe because
/// `connection_usage_member`'s narrower grammar is the *only* usage form dispatched there) was
/// tried and broke `test_systems_library_node_types_no_extended`/
/// `test_full_library_node_types_no_extended` in the `SYSML_V2_RELEASE_DIR` gate: the real bare
/// `abstract connection ... nonunique :> ...` forms above stopped parsing as `ConnectionDef` and,
/// since `connection_usage_member` can't parse them either, fell all the way through to
/// `ExtendedLibraryDecl`. That is a worse outcome than the status quo, not a fix. Do not add
/// `.def_required()` here without first widening `connection_usage_member` to cover
/// `abstract`/multiplicity/`nonunique`/leading `:>` subsets, matching the port/calc/constraint
/// precedent from CHANGELOG 0.33.0.
///
/// **PAR-007 update**: the PAR-006b claim above that "there is no live misclassification bug
/// here" was correct for the shapes it checked, but missed one: `connection link : Link connect
/// a to b;` (a typed connector usage with an inline `connect ... to ...` clause) *was*
/// misclassified. The plain `: Type` header scan (`specialization::
/// parse_optional_definition_header_after_identification`) greedily consumes everything up to
/// `;`/`{` and silently discards it once a leading type name is extracted, so `connection_def`
/// matched this input too -- with an empty body, having swallowed and dropped the `connect`
/// clause entirely -- rather than correctly leaving it for `connection_usage_member`. This is
/// narrower and safer than a `.def_required()` guard: `.reject_header_keyword(b"connect")` only
/// fails the parse when the discarded header text contains a top-level `connect` keyword, which
/// the bare Systems-Library shape above never does, so that regression cannot recur (see
/// `par_006b_audit_tests` below, still green).
pub(crate) fn connection_def(input: Input<'_>) -> IResult<Input<'_>, Node<ConnectionDef>> {
    parse_connection_def(
        input,
        DefinitionPrefixOptions::new(b"connection")
            .with_derivation_role()
            .individual_allowed()
            .with_captured_visibility()
            .reject_header_keyword(b"connect")
            // GH-20: a `def`-less, non-`abstract` `connection name : Type { ... }` with no
            // `:>`/`specializes` clause in its header (SysML v2 §7.13.2's plain named typed
            // connection usage) is a `ConnectionUsageMember`, not a definition -- see
            // `reject_plain_typed_header_without_def`'s doc comment. The bare Systems-Library
            // definition shape this parser must keep accepting (PAR-006b) always carries
            // `abstract` and/or a `:>` subclassification clause, so it's unaffected.
            .reject_plain_typed_header_without_def(),
    )
}

/// Connection definition with required `def` keyword, for contexts (e.g. nested inside a part
/// definition body) where a bare `connection` usage form (`connection_usage_member`) is already
/// dispatched separately -- requiring `def` here prevents a `def`-less connection usage from
/// being misclassified as a definition, the same bug class as PAR-001 in `attribute_def`. Does
/// does not support the `#derivation` def-less form ([`connection_def`] does); nothing in the
/// nested-part-body grammar currently needs that combination.
pub(crate) fn connection_def_required(input: Input<'_>) -> IResult<Input<'_>, Node<ConnectionDef>> {
    parse_connection_def(
        input,
        DefinitionPrefixOptions::new(b"connection")
            .def_required()
            .individual_allowed()
            .with_captured_visibility()
            .reject_header_keyword(b"connect"),
    )
}

fn parse_connection_def(
    input: Input<'_>,
    options: DefinitionPrefixOptions,
) -> IResult<Input<'_>, Node<ConnectionDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(input, options)?;
    let (input, body) = connection_member_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ConnectionDef {
                is_individual: prefix.is_individual,
                derivation_role: prefix.derivation_role,
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
                membership: crate::ast::Membership::owning(
                    prefix.visibility,
                    prefix.visibility_span,
                ),
            },
        ),
    ))
}

#[cfg(test)]
mod par_002_widening_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    #[test]
    fn connection_def_body_accepts_nested_attribute_usage() {
        let (rest, node) =
            connection_def_body_element(input("attribute mass: Real;")).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            ConnectionDefBodyElement::AttributeUsage(_)
        ));
    }

    #[test]
    fn connection_def_body_accepts_nested_item_def_not_misparsed_as_usage() {
        let (rest, node) =
            connection_def_body_element(input("item def MyItem;")).expect("item def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, ConnectionDefBodyElement::ItemDef(_)));
    }

    #[test]
    fn connection_def_body_accepts_nested_port_def_not_misparsed_as_usage() {
        let (rest, node) =
            connection_def_body_element(input("port def MyPort;")).expect("port def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, ConnectionDefBodyElement::PortDef(_)));
    }

    #[test]
    fn connection_def_body_accepts_nested_port_usage() {
        let (rest, node) =
            connection_def_body_element(input("port p1: MyPort;")).expect("port usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, ConnectionDefBodyElement::PortUsage(_)));
    }

    /// PAR-002 acceptance criterion: `attribute` usage yields the same underlying parse (via the
    /// shared `attribute_usage` parser, also wired into `PartDefBodyElement`/`PackageBodyElement`
    /// in prior increments) whether reached through `ConnectionDefBodyElement::AttributeUsage` or
    /// any other body enum.
    #[test]
    fn attribute_usage_is_same_variant_kind_in_connection_def_body_as_shared_parser() {
        let text = "attribute mass: Real;";
        let (_, conn_node) =
            connection_def_body_element(input(text)).expect("nested in connection def body");
        assert!(matches!(
            conn_node.value,
            ConnectionDefBodyElement::AttributeUsage(_)
        ));
        let result = attribute_usage(input(text));
        assert!(
            result.is_ok(),
            "attribute_usage should also accept {text:?}"
        );
    }
}

#[cfg(test)]
mod par_006b_audit_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    /// PAR-006b audit: `connection_def` must keep accepting this exact real-Systems-Library shape
    /// (`Systems Library/Connections.sysml`) -- a bare, `def`-less, un-annotated `connection`
    /// usage with `abstract`, multiplicity, `nonunique`, and leading `:>` subsets before the
    /// body. This is the shape that made a `def_required_unless_annotated()` guard on
    /// `connection_def` unsafe (see the doc comment on `connection_def` above): tightening `def`
    /// requirements here without first widening `connection_usage_member` to cover this shape
    /// sends it to `ExtendedLibraryDecl` instead, which is what broke the
    /// `SYSML_V2_RELEASE_DIR` gate (`test_systems_library_node_types_no_extended`) during this
    /// audit. This test exists so any future attempt to tighten `connection_def` fails fast,
    /// locally, and points back at this note instead of only failing the much slower full-library
    /// gate.
    #[test]
    fn connection_def_accepts_the_bare_abstract_multiplicity_nonunique_subsets_form_that_makes_def_required_unsafe(
    ) {
        let text =
            "abstract connection connections: Connection[0..*] nonunique :> linkObjects, parts { }";
        let result = connection_def(input(text));
        assert!(
            result.is_ok(),
            "connection_def should still accept the bare Systems-Library connection form, got {result:?}"
        );
    }
}

#[cfg(test)]
mod membership_tests {
    use super::*;
    use crate::parser::part::connection_usage_member;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    // --- parser work item 4b (continuation): Membership on ConnectionDef/ConnectionUsageMember ---

    /// `connection_usage_member` previously never parsed a `private`/`protected`/`public` prefix
    /// at all (same genuine gap as `part_def`/`port_def`/`port_usage`/`item_def`/`item_usage`).
    #[test]
    fn connection_usage_member_visibility_prefix_is_captured_on_membership() {
        let (_, node) = connection_usage_member(input("private connection c1: MyConnection;"))
            .expect("connection usage member");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Private)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn connection_usage_member_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = connection_usage_member(input("connection c1: MyConnection;"))
            .expect("connection usage member");
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    /// PARSER_BACKLOG_ROADMAP.md §6, G2: `connection_usage_member` had no multiplicity support
    /// at all, so real usage like `connection trailerHitch : TrailerHitch[0..1];` (OMG spec
    /// Annex `3c-Function-based Behavior-structure mod.sysml`) fell through to opaque recovery.
    #[test]
    fn connection_usage_member_accepts_multiplicity() {
        let (rest, node) =
            connection_usage_member(input("connection trailerHitch : TrailerHitch[0..1];"))
                .expect("connection usage member with multiplicity");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name.as_deref(), Some("trailerHitch"));
        assert!(node.value.type_reference.is_some());
        let multiplicity = node.value.multiplicity.expect("multiplicity present");
        assert!(multiplicity.value.lower.is_some());
        assert!(multiplicity.value.upper.is_some());
    }

    #[test]
    fn connection_usage_member_without_multiplicity_still_works() {
        let (rest, node) = connection_usage_member(input("connection c1: MyConnection;"))
            .expect("connection usage member without multiplicity");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.multiplicity, None);
    }

    /// `connection_def`/`connection_def_required` previously never parsed a visibility prefix
    /// either (same genuine gap as `part_def`/`port_def`/`item_def`).
    #[test]
    fn connection_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = connection_def(input("protected connection def MyConnection;"))
            .expect("connection def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Protected)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }

    #[test]
    fn connection_def_public_visibility_prefix_is_captured_on_membership() {
        let (rest, node) =
            connection_def(input("public connection def MyConnection;")).expect("connection def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
    }

    #[test]
    fn connection_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) =
            connection_def(input("connection def MyConnection;")).expect("connection def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }
}
