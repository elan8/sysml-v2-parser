//! Connection definition parsing (BNF ConnectionDefinition).

use crate::ast::{ConnectionDef, ConnectionDefBody, ConnectionDefBodyElement, Node};
use crate::parser::attribute::{attribute_def, attribute_usage};
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
use crate::parser::port::{port_def, port_usage};
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::Parser;

fn connection_def_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<ConnectionDefBodyElement>> {
    // Member boundary: `ws_and_notes` leaves a bare `/* ... */` for this scope's
    // annotating member, which is the `Comment` production's keyword-less spelling.
    let (input, _) = crate::parser::lex::ws_and_notes(input)?;
    let start = input;
    // A `#tag` run and a leading `ref` are both `OccurrenceUsagePrefix` slots that a sibling
    // production in this scope would otherwise claim first; see
    // `occurrence_prefix::starts_contended_prefix`.
    if crate::parser::occurrence_prefix::starts_contended_prefix(start) {
        if let Ok((next, usage)) = occurrence_usage(start) {
            let elem = ConnectionDefBodyElement::OccurrenceUsage(Box::new(usage));
            return Ok((next, node_from_to(start, next, elem)));
        }
        if let Ok((next, usage)) = crate::parser::item::item_usage(start) {
            let elem = ConnectionDefBodyElement::ItemUsage(usage);
            return Ok((next, node_from_to(start, next, elem)));
        }
        if let Ok((next, usage)) = part_usage(start) {
            let elem = ConnectionDefBodyElement::PartUsage(Box::new(usage));
            return Ok((next, node_from_to(start, next, elem)));
        }
        if let Ok((next, usage)) = port_usage(start) {
            let elem = ConnectionDefBodyElement::PortUsage(Box::new(usage));
            return Ok((next, node_from_to(start, next, elem)));
        }
    }
    let (input, elem) = alt((
        map(end_decl, ConnectionDefBodyElement::EndDecl),
        map(ref_decl, ConnectionDefBodyElement::RefDecl),
        map(connect_stmt, ConnectionDefBodyElement::ConnectStmt),
        map(
            crate::parser::body::annotating_member,
            ConnectionDefBodyElement::Annotating,
        ),
        // Both `#` productions: the `ExtendedUsage` member spelling (which owns a `;`/`{}`
        // body) is tried before the `PrefixMetadataMember` spelling, which owns no body and
        // leaves the prefixed declaration for the next member iteration.
        alt((
            map(
                crate::parser::metadata_annotation::metadata_keyword_usage,
                ConnectionDefBodyElement::MetadataKeywordUsage,
            ),
            map(
                crate::parser::metadata_annotation::metadata_keyword_prefix,
                ConnectionDefBodyElement::MetadataKeywordUsage,
            ),
        )),
        // PAR-002 widening: this body previously had no attribute/item/port coverage at all.
        // Same def-before-usage discipline as `InterfaceDefBodyElement`/other body enums.
        map(attribute_def, ConnectionDefBodyElement::AttributeDef),
        map(attribute_usage, ConnectionDefBodyElement::AttributeUsage),
        map(item_def_required, ConnectionDefBodyElement::ItemDef),
        map(item_usage, ConnectionDefBodyElement::ItemUsage),
        map(port_def, ConnectionDefBodyElement::PortDef),
        map(port_usage, |p| {
            ConnectionDefBodyElement::PortUsage(Box::new(p))
        }),
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
    let (input, members) = crate::parser::body::parse_structured_brace_members_with_skip(
        input,
        CONNECTION_DEF_BODY_STARTERS,
        "connection definition body",
        "recovered_connection_def_body_element",
        connection_def_body_element,
        connection_def_body_recovery,
        crate::parser::body::BraceMemberSkip::BodyElementRecover,
    )?;
    Ok((input, members.into_body()))
}

/// `ConnectionDefinition = OccurrenceDefinitionPrefix 'connection' 'def' Definition` (SysML BNF
/// 664, clause 8.2.2.13).
///
/// `def` is required in every scope. A `def`-less `connection …` is always a `ConnectionUsage`
/// (`part::connection_usage_member`), including the Systems Library's
/// `abstract connection connections : Connection[0..*] nonunique :> linkObjects, parts { … }`
/// and the `#derivation connection d { … }` shape: the usage parser owns the whole
/// `OccurrenceUsagePrefix` and `UsageDeclaration`, so no definition has to stand in for them.
/// See `planning/connection-usage-prefix-matrix.md`.
pub(crate) fn connection_def(input: Input<'_>) -> IResult<Input<'_>, Node<ConnectionDef>> {
    parse_connection_def(
        input,
        DefinitionPrefixOptions::new(b"connection")
            .def_required()
            .with_extension_keywords()
            .individual_allowed()
            .with_captured_visibility(),
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
                definition_prefix: prefix.basic_prefix,
                is_individual: prefix.is_individual,
                extension_keywords: prefix.extension_keywords,
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
mod def_less_usage_tests {
    use super::*;
    use crate::parser::part::connection_usage_member;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    /// The Systems Library's `def`-less declaration (`Systems Library/Connections.sysml`) is a
    /// `ConnectionUsage`: `connection_def` refuses it, and the usage parser keeps every slot the
    /// old definition fallback discarded -- `abstract`, the multiplicity, `nonunique` and the
    /// leading `:>` subsetting.
    #[test]
    fn the_bare_systems_library_connection_is_a_usage_with_every_slot() {
        let text =
            "abstract connection connections: Connection[0..*] nonunique :> linkObjects, parts { }";
        assert!(connection_def(input(text)).is_err());
        let (rest, node) = connection_usage_member(input(text)).expect("connection usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let usage = node.value;
        assert!(usage
            .prefix
            .basic()
            .is_some_and(|basic| basic.ref_prefix.variance.is_some()));
        assert!(usage.typing.is_some());
        assert!(usage.multiplicity.is_some());
        assert!(usage.multiplicity_modifiers.uniqueness.is_some());
        assert!(usage.subsets.is_some());
    }

    /// `#derivation connection d { … }` is a usage whose prefix owns the extension keyword.
    #[test]
    fn a_def_less_derivation_connection_is_a_usage_carrying_its_prefix_metadata() {
        let text = "#derivation connection d { end #original ::> r1; end #derive ::> r2; }";
        assert!(connection_def(input(text)).is_err());
        let (rest, node) = connection_usage_member(input(text)).expect("connection usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.prefix.extension_keywords.len(), 1);
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
        let src = input("connection trailerHitch : TrailerHitch[0..1];");
        let (rest, node) =
            connection_usage_member(src).expect("connection usage member with multiplicity");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .name
                .map(|n| crate::parser::lex::name_bytes(src, n)),
            Some(&b"trailerHitch"[..])
        );
        assert!(node.value.typing.is_some());
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
