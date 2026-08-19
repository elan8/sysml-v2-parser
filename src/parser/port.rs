//! Port definition and port usage parsing.

use crate::ast::{
    Node, PortBody, PortBodyElement, PortDef, PortDefBody, PortDefBodyElement, PortUsage,
};
use crate::parser::action::in_out_decl;
use crate::parser::attribute::{
    attribute_def, attribute_feature_binding, attribute_usage, directed_attribute_usage,
};
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::enumeration::enum_usage;
use crate::parser::item::{item_def_required, item_usage};
use crate::parser::lex::{
    name, short_name_prefix, starts_with_keyword, ws1, ws_and_comments, PORT_BODY_STARTERS,
    PORT_DEF_BODY_STARTERS,
};
use crate::parser::node_from_to;
use crate::parser::usage::{
    multiplicity_node, optional_typings, prefix_redefinition_target, specialization_clauses,
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
    alt((crate::parser::body::semicolon_body, port_body_brace)).parse(input)
}

fn port_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PortBodyElement>> {
    let start = input;
    // Member boundary: `ws_and_notes` leaves a bare `/* ... */` for this scope's
    // annotating member, which is the `Comment` production's keyword-less spelling.
    let (input, _) = crate::parser::lex::ws_and_notes(input)?;
    // `port_usage` is the first alternative, so this scope needs no contended pre-dispatch: it
    // already sees `ref port q;` (`Simple Tests/PartTest.sysml:21`) and `#Tag port t;` before
    // `connector::ref_decl` and the annotating members below can claim their first token.
    let (input, elem) = alt((
        map(port_usage, |p| PortBodyElement::PortUsage(Box::new(p))),
        map(in_out_decl, PortBodyElement::InOutDecl),
        map(
            crate::parser::body::annotating_member,
            PortBodyElement::Annotating,
        ),
        // PAR-002 widening: this body previously had no attribute/item coverage at all.
        map(attribute_usage, PortBodyElement::AttributeUsage),
        // A port body may redefine an inherited feature without repeating its kind keyword, e.g.
        // `port pwr : DevicePower { :>> maxCurrent = 0.02 [A]; }`. Attribute and item bodies
        // already accept this prefix-redefinition form; port bodies rejected it.
        map(attribute_feature_binding, PortBodyElement::AttributeUsage),
        map(item_usage, PortBodyElement::ItemUsage),
        // After `port_usage` so `ref port …` reaches the kind-keyword form, exactly as in
        // `port_def_body_element`. `ref_decl` owns the keyword-less `ref` members the Systems
        // Library writes inside a `ref port … { … }` body.
        map(crate::parser::connector::ref_decl, PortBodyElement::RefDecl),
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
    let (input, members) = crate::parser::body::parse_structured_brace_members_with_skip(
        input,
        PORT_BODY_STARTERS,
        "port body",
        "recovered_port_body_element",
        port_body_element,
        port_body_recovery,
        crate::parser::body::BraceMemberSkip::BodyElementRecover,
    )?;
    Ok((input, members.into_body()))
}

/// `PortUsage = OccurrenceUsagePrefix 'port' Usage` (SysML BNF 645).
///
/// The prefix is the shared component `parser::occurrence_prefix` owns, so every slot the
/// production allows -- direction, `derived`, `abstract`/`variation`, `constant`, `ref`,
/// `individual`, `snapshot`/`timeslice` and an ordered run of `UsageExtensionKeyword`s -- is read
/// here in the grammar's order and nowhere else. Before this seam five of the thirteen slots were
/// respelled inline in a *different* order and emitted in a third, so `individual abstract in
/// derived constant port x;` was accepted and silently reordered while the only legal spelling
/// `in derived abstract constant port y;` was refused; see
/// `planning/port-usage-prefix-matrix.md` §9.4.
///
/// Wrapped in a reference transaction because the prefix's `UsageExtensionKeyword*` allocates an
/// arena entry per `#tag` before the production is known to apply. A prefix followed by anything
/// other than `port` fails the whole production, so the member reaches recovery as one node
/// rather than being reinterpreted as an unprefixed usage.
pub(crate) fn port_usage(input: Input<'_>) -> IResult<Input<'_>, Node<PortUsage>> {
    crate::parser::span::reference_transaction(input, port_usage_inner)
}

fn port_usage_inner(input: Input<'_>) -> IResult<Input<'_>, Node<PortUsage>> {
    enum PortUsageHead {
        PrefixRedefines {
            redefines: Node<crate::ast::SubsettingRelationship>,
        },
        Named {
            name_span: Option<crate::ast::Span>,
            name: String,
        },
    }

    let start = input;
    let (input, _) = ws_and_comments(input)?;
    // `OccurrenceUsageMember = MemberPrefix ownedRelatedElement += OccurrenceUsageElement`, so
    // the visibility keyword precedes the usage's own prefix.
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, prefix) = crate::parser::occurrence_prefix::occurrence_usage_prefix(input)?;
    let (input, _) = tag(&b"port"[..]).parse(input)?;
    // SysML allows anonymous port usages: `port: PowerPort;` (Identification may be empty).
    let (after_kw, _) = ws_and_comments(input)?;
    // `port def …` is a `PortDefinition`, which names `DefinitionPrefix` rather than this
    // production. Especially important for `ref port def …`, which no definition parser claims
    // (a definition prefix has no `ref`) and which would otherwise misparse as a usage named
    // `def`.
    if starts_with_keyword(after_kw.fragment(), b"def") {
        return Err(nom::Err::Error(nom::error::Error::new(
            after_kw,
            nom::error::ErrorKind::Tag,
        )));
    }
    let input = if (after_kw.fragment().starts_with(b":")
        && !after_kw.fragment().starts_with(b":>")
        && !after_kw.fragment().starts_with(b":>>"))
        || starts_with_keyword(after_kw.fragment(), b"defined")
    {
        after_kw
    } else {
        let (input, _) = ws1(input)?;
        input
    };
    // `Identification`'s `( '<' ShortName '>' )?` half (BNF §8.2.2.2) -- see
    // `attribute::attribute_usage`'s identical short-name handling for the confirmed real-usage
    // citation.
    let (input, short_name) = short_name_prefix(input)?;
    // Consume (not just peek) whitespace/comments after the short name's closing `>` -- see
    // `attribute::attribute_usage`'s identical fix for why this can't reuse `ws1`'s earlier
    // consumption (a short name leaves fresh un-consumed whitespace after it).
    let (input, _) = ws_and_comments(input)?;
    let peek = input;
    let (input, usage_head) = if (peek.fragment().starts_with(b":")
        && !peek.fragment().starts_with(b":>")
        && !peek.fragment().starts_with(b":>>"))
        || starts_with_keyword(peek.fragment(), b"defined")
    {
        (
            input,
            PortUsageHead::Named {
                name_span: None,
                name: String::new(),
            },
        )
    } else {
        alt((
            map(
                preceded(ws_and_comments, prefix_redefinition_target),
                |(_, redefines)| PortUsageHead::PrefixRedefines { redefines },
            ),
            map(with_span(name), |(name_span, name)| PortUsageHead::Named {
                name_span: Some(name_span),
                name,
            }),
        ))
        .parse(input)?
    };
    let (input, name_str, name_span, prefix_redefines) = match usage_head {
        PortUsageHead::PrefixRedefines { redefines } => {
            (input, String::new(), None, Some(redefines))
        }
        PortUsageHead::Named { name_span, name } => (input, name, name_span, None),
    };
    let (input, type_result) = optional_typings(input)?;
    let (type_ref_span, _, typing) =
        crate::parser::usage::typing_reference_fields_from_result(type_result);
    let (input, multiplicity) = opt(multiplicity_node).parse(input)?;
    // `MultiplicityPart`'s `ordered`/`nonunique`, which may sit either side of a specialization
    // clause: `port ports : Port[0..*] nonunique :> objects;` (`Systems Library/Ports.sysml:48`).
    let (input, modifiers_before) = crate::parser::usage::multiplicity_modifier_slots(input)?;
    let (input, clauses) = specialization_clauses(input)?;
    let (input, modifiers_after) = crate::parser::usage::multiplicity_modifier_slots(input)?;
    let redefines = clauses.redefines.or(prefix_redefines);
    // §6 G11: `port :>> pe = c1.pb;` -- a port usage may carry a feature value, which binds it to
    // another port rather than declaring a fresh one.
    let (input, value) = opt(preceded(
        ws_and_comments,
        crate::parser::feature_value::feature_value_part,
    ))
    .parse(input)?;
    let (input, body) = port_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            PortUsage {
                prefix,
                name: name_str,
                short_name,
                typing,
                multiplicity,
                multiplicity_modifiers: modifiers_before.merge(modifiers_after),
                subsets: clauses.subsets,
                redefines,
                references: clauses.references,
                crosses: clauses.crosses,
                intersects: clauses.intersects,
                value,
                body,
                name_span,
                type_ref_span,
                membership: crate::ast::Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

const PORT_DEF_OPAQUE_STARTERS: &[&[u8]] = &[b"ref", b"abstract"];

fn port_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PortDefBodyElement>> {
    let start = input;
    // Member boundary: `ws_and_notes` leaves a bare `/* ... */` for this scope's
    // annotating member, which is the `Comment` production's keyword-less spelling.
    let (input, _) = crate::parser::lex::ws_and_notes(input)?;
    // A `#tag` run and a leading `ref` are both `OccurrenceUsagePrefix` slots that a sibling
    // production in this scope would otherwise claim first -- the two `#` arms immediately below
    // and `connector::ref_decl` further down; see `occurrence_prefix::starts_contended_prefix`.
    // `#idd port APIS_HTTP { … }` (`Arrowhead Framework Example/AHFNorwayTopics.sysml:22`) became
    // two sibling members, and `ref port c2 : C;` (`Simple Tests/PartTest.sysml:46`) a `RefDecl`.
    if crate::parser::occurrence_prefix::starts_contended_prefix(input) {
        if let Ok((next, usage)) = port_usage(input) {
            let elem = PortDefBodyElement::PortUsage(Box::new(usage));
            return Ok((next, node_from_to(start, next, elem)));
        }
    }
    // `#keyword` metadata tag -- tried first so a stacked/prefixing `#idd port APIS_HTTP { ... }`
    // (bare form, then `PrefixMetadataMember`-style form prefixing the next port-body member)
    // dispatches here instead of falling through to the opaque-capture fallback below. Mirrors
    // `package_body_element`'s identical two-arm `#`-handling.
    if let Ok((input, elem)) = crate::parser::span::reference_transaction(input, |input| {
        map(
            crate::parser::metadata_annotation::metadata_keyword_usage,
            PortDefBodyElement::MetadataKeywordUsage,
        )
        .parse(input)
    }) {
        return Ok((input, node_from_to(start, input, elem)));
    }
    if let Ok((input, elem)) = crate::parser::span::reference_transaction(input, |input| {
        map(
            crate::parser::metadata_annotation::metadata_keyword_prefix,
            PortDefBodyElement::MetadataKeywordUsage,
        )
        .parse(input)
    }) {
        return Ok((input, node_from_to(start, input, elem)));
    }
    let (input, elem) = alt((
        map(item_usage, PortDefBodyElement::ItemUsage),
        map(directed_attribute_usage, PortDefBodyElement::AttributeUsage),
        map(in_out_decl, PortDefBodyElement::InOutDecl),
        map(
            crate::parser::body::annotating_member,
            PortDefBodyElement::Annotating,
        ),
        map(|i| attribute_def(i, true), PortDefBodyElement::AttributeDef),
        map(attribute_usage, PortDefBodyElement::AttributeUsage),
        map(
            attribute_feature_binding,
            PortDefBodyElement::AttributeUsage,
        ),
        // `item_def_required` must be tried before the existing bare `item_usage` arms above -- same def-before-usage discipline as the other body enums
        // wired in prior increments.
        map(item_def_required, PortDefBodyElement::ItemDef),
        map(enum_usage, PortDefBodyElement::EnumerationUsage),
        map(port_usage, |p| PortDefBodyElement::PortUsage(Box::new(p))),
        // After `port_usage` so `ref port ...` reaches the kind-keyword form rather than being
        // read as an anonymous `ref` followed by stray text.
        map(
            crate::parser::connector::ref_decl,
            PortDefBodyElement::RefDecl,
        ),
        map(
            |i| {
                crate::parser::recovery::unsupported_member(
                    i,
                    PORT_DEF_OPAQUE_STARTERS,
                    "port definition body",
                )
            },
            PortDefBodyElement::Unsupported,
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
    alt((crate::parser::body::semicolon_body, port_def_body_brace)).parse(input)
}

fn port_def_body_brace(input: Input<'_>) -> IResult<Input<'_>, PortDefBody> {
    let (input, members) = crate::parser::body::parse_structured_brace_members_with_skip(
        input,
        PORT_DEF_BODY_STARTERS,
        "port definition body",
        "recovered_port_def_body_element",
        port_def_body_element,
        port_def_body_recovery,
        crate::parser::body::BraceMemberSkip::BodyElementRecover,
    )?;
    Ok((input, members.into_body()))
}

/// `PortDefinition = DefinitionPrefix 'port' 'def' Definition …` (SysML BNF 628).
///
/// `def` is a required literal in the pin, so a keyword-less `port p : T;` is a `PortUsage` in
/// every scope and reaches [`port_usage`]. This parser used to make `def` optional at package
/// scope, which folded that legal usage form into a definition: `abstract port ports :
/// Port[0..*] nonunique :> objects;` (`Systems Library/Ports.sysml:48`) lost `abstract`, the
/// multiplicity and `nonunique`, and came back out as `port def ports :> objects;`. See
/// `planning/port-usage-prefix-matrix.md` §7.1.
///
/// `DefinitionPrefix` is `('abstract' | 'variation')?` -- not `OccurrenceDefinitionPrefix` -- so a
/// port definition carries no `individual`, no direction, no `derived`, no `constant` and no
/// `ref`. Those all belong to `PortUsage`'s `OccurrenceUsagePrefix`.
pub(crate) fn port_def(input: Input<'_>) -> IResult<Input<'_>, Node<PortDef>> {
    let start = input;
    let options = DefinitionPrefixOptions::new(b"port")
        .with_captured_visibility()
        .def_required();
    let (input, prefix) = parse_definition_prefix(input, options)?;
    let (input, body) = port_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            PortDef {
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
    fn port_usage_captures_intersects() {
        let (rest, node) =
            port_usage(input("port p : PortType intersects a;")).expect("port usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.intersects.as_ref().map(|n| n.value.target.len()),
            Some(1)
        );
    }

    #[test]
    fn port_body_accepts_nested_attribute_usage() {
        let (rest, node) =
            port_body_element(input("attribute mass: Real;")).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PortBodyElement::AttributeUsage(_)));
    }

    #[test]
    fn port_body_accepts_nested_item_usage() {
        let (rest, node) = port_body_element(input("item i1: MyItem;")).expect("item usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PortBodyElement::ItemUsage(_)));
    }

    #[test]
    fn port_def_body_accepts_nested_item_def_not_misparsed_as_usage() {
        let (rest, node) = port_def_body_element(input("item def MyItem;")).expect("item def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PortDefBodyElement::ItemDef(_)));
    }

    /// `PortDefinition` requires `def`, so a keyword-less `port p1 : MyPortType;` is not one.
    ///
    /// It used to be: `port_def` made `def` optional and claimed the usage form, which is how a
    /// package-scope `port p1 : MyPortType;` came back out as `port def p1 : MyPortType;`.
    #[test]
    fn port_def_refuses_a_declaration_with_no_def_keyword() {
        assert!(
            port_def(input("port p1: MyPortType;")).is_err(),
            "`port p1: MyPortType;` is a PortUsage, not a PortDefinition"
        );
        let (rest, node) = port_usage(input("port p1: MyPortType;")).expect("port usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "p1");
        let typing = node
            .value
            .typing
            .expect("type reference must not be dropped");
        assert_eq!(typing.value.target.len(), 1);
        assert_eq!(typing.value.kind, crate::ast::TypingKind::Typing);
    }

    /// `MultiplicityPart`'s `ordered`/`nonunique` on the keyword-less form the definition parser
    /// used to claim: `abstract port ports : Port[0..*] nonunique :> objects;`
    /// (`Systems Library/Ports.sysml:48`) lost all three facts to that fold.
    #[test]
    fn port_usage_keeps_the_multiplicity_modifiers_of_the_library_form() {
        let (rest, node) = port_usage(input(
            "abstract port ports : Port[0..*] nonunique :> objects;",
        ))
        .expect("port usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.prefix.basic.ref_prefix.variance.is_some());
        assert!(node.value.multiplicity.is_some());
        assert!(!node.value.multiplicity_modifiers.is_unique());
        assert!(!node.value.multiplicity_modifiers.is_ordered());
        assert!(node.value.subsets.is_some());
    }

    #[test]
    fn port_def_body_accepts_nested_enum_usage() {
        let (rest, node) = port_def_body_element(input("enum e1: MyEnum;")).expect("enum usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            PortDefBodyElement::EnumerationUsage(_)
        ));
    }

    /// PAR-002 acceptance criterion: the same `item def` declaration yields the same AST variant
    /// kind nested in a port definition body as it already does nested in a part definition body
    /// (proven in a prior increment via `PartDefBodyElement::ItemDef`).
    #[test]
    fn item_def_is_same_variant_kind_in_port_def_body_as_item_def_required_parser() {
        let text = "item def MyItem;";
        let (_, port_node) = port_def_body_element(input(text)).expect("nested in port def body");
        assert!(matches!(port_node.value, PortDefBodyElement::ItemDef(_)));
        // Directly confirms `item_def_required` (the same parser reused across every body enum
        // wired in this backlog) accepts the identical snippet.
        let result = item_def_required(input(text));
        assert!(
            result.is_ok(),
            "item_def_required should also accept {text:?}"
        );
    }

    // --- parser work item 4b (continuation): Membership on PortDef/PortUsage ---

    #[test]
    fn port_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) = port_usage(input("private port p1: MyPort;")).expect("port usage");
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
    fn port_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = port_usage(input("port p1: MyPort;")).expect("port usage");
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    /// `port_def` previously never parsed a `private`/`protected`/`public` prefix at all (same
    /// genuine gap as `part_def` -- BNF `DefinitionMember : OwningMembership = MemberPrefix
    /// ownedRelatedElement += DefinitionElement` legally allows one before any definition).
    #[test]
    fn port_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = port_def(input("protected port def MyPort;")).expect("port def");
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
    fn port_def_public_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = port_def(input("public port def MyPort;")).expect("port def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
    }

    #[test]
    fn port_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = port_def(input("port def MyPort;")).expect("port def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }

    // --- short-name (`<shortName>`) support on `port_usage`, mirroring `attribute_usage`'s
    // identical gap (shared `Identification` BNF production, §8.2.2.2) -- see
    // `attribute.rs::attribute_body_tests`'s citation of the confirmed real-usage gap in the OMG
    // Geometry domain library's `VehicleGeometryAndCoordinateFrames.sysml`.

    #[test]
    fn port_usage_captures_short_name() {
        let (rest, node) =
            port_usage(input("port <pp> powerPort: PowerPort;")).expect("port usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.short_name.as_deref(), Some("pp"));
        assert_eq!(node.value.name, "powerPort");
    }

    #[test]
    fn port_usage_captures_short_name_with_redefines() {
        let (rest, node) =
            port_usage(input("port <pp> :>> powerPort: PowerPort;")).expect("port usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.short_name.as_deref(), Some("pp"));
        assert_eq!(
            node.value.redefines.as_ref().map(|n| n.value.target.len()),
            Some(1)
        );
    }

    #[test]
    fn port_usage_without_short_name_has_none() {
        let (_, node) = port_usage(input("port p1: MyPort;")).expect("port usage");
        assert_eq!(node.value.short_name, None);
    }
}
