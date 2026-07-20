//! Port definition and port usage parsing.

use crate::ast::{
    Node, PortBody, PortBodyElement, PortDef, PortDefBody, PortDefBodyElement, PortUsage,
};
use crate::parser::action::in_out_decl;
use crate::parser::attribute::{attribute_def, attribute_usage, directed_attribute_usage};
use crate::parser::body::parse_structured_brace_members;
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::enumeration::enum_usage;
use crate::parser::item::{directed_item_usage, item_def_required, item_usage};
use crate::parser::lex::{
    capture_opaque_member, name, ws1, ws_and_comments, PORT_BODY_STARTERS, PORT_DEF_BODY_STARTERS,
};
use crate::parser::node_from_to;
use crate::parser::requirement::doc_comment;
use crate::parser::usage::{
    multiplicity_node, optional_typings, prefix_redefinition_target, specialization_clauses,
    targets_display_string,
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
        // PAR-002 widening: this body previously had no attribute/item coverage at all.
        map(attribute_usage, PortBodyElement::AttributeUsage),
        map(item_usage, PortBodyElement::ItemUsage),
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

/// Port usage: (direction)? (`derived`)? (`constant`)? 'port' ( (`:>>`|`redefines`) target | name )
/// ( ':' type )? multiplicity? clauses? body
///
/// The optional direction/`derived`/`constant` prefixes are BNF `RefPrefix` (§8.2.2.6.2), reached
/// via `PortUsage = OccurrenceUsagePrefix 'port' Usage` -> `BasicUsagePrefix` -> `RefPrefix` --
/// same production chain `PartUsage`'s equivalent prefixes use.
pub(crate) fn port_usage(input: Input<'_>) -> IResult<Input<'_>, Node<PortUsage>> {
    enum PortUsageHead {
        PrefixRedefines {
            name_span: crate::ast::Span,
            redefines: Node<crate::ast::SubsettingRelationship>,
        },
        Named {
            name_span: crate::ast::Span,
            name: String,
        },
    }

    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, direction) = opt(crate::parser::attribute::direction_prefix).parse(input)?;
    let (input, is_derived) = opt(preceded(tag(&b"derived"[..]), ws1))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
    let (input, is_constant) = opt(preceded(tag(&b"constant"[..]), ws1))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
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
        map(with_span(name), |(name_span, name)| PortUsageHead::Named {
            name_span,
            name,
        }),
    ))
    .parse(input)?;
    let (input, name_str, name_span, prefix_redefines) = match usage_head {
        PortUsageHead::PrefixRedefines {
            name_span,
            redefines,
        } => (
            input,
            redefines
                .value
                .first_target()
                .and_then(|t| t.local_name())
                .unwrap_or_default()
                .to_string(),
            name_span,
            Some(redefines),
        ),
        PortUsageHead::Named { name_span, name } => (input, name, name_span, None),
    };
    let (input, type_result) = optional_typings(input)?;
    let (type_ref_span, type_name) = type_result
        .map(|(span, is_conjugated, targets)| {
            let name = targets_display_string(&targets);
            (
                Some(span),
                Some(if is_conjugated {
                    format!("~{name}")
                } else {
                    name
                }),
            )
        })
        .unwrap_or((None, None));
    let (input, multiplicity) = opt(multiplicity_node).parse(input)?;
    let (input, clauses) = specialization_clauses(input)?;
    let redefines = clauses.redefines.or(prefix_redefines);
    let (input, body) = port_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            PortUsage {
                direction,
                is_derived,
                is_constant,
                name: name_str,
                type_name,
                multiplicity,
                subsets: clauses.subsets,
                redefines,
                references: clauses.references,
                crosses: clauses.crosses,
                intersects: clauses.intersects,
                body,
                name_span: Some(name_span),
                type_ref_span,
                membership: crate::ast::Membership::feature(visibility, visibility_span),
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
        // `item_def_required` must be tried before the existing bare `directed_item_usage`/
        // `item_usage` arms above -- same def-before-usage discipline as the other body enums
        // wired in prior increments.
        map(item_def_required, PortDefBodyElement::ItemDef),
        map(enum_usage, PortDefBodyElement::EnumerationUsage),
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
/// `def` is intentionally optional: the standard library uses bare, `def`-less `port` usages at
/// package/namespace level (e.g. `abstract port ports : Port[0..*] nonunique :> objects { ... }`
/// in `Systems Library/Ports.sysml`), and there is no dedicated package-level `port_usage`
/// dispatch to catch them instead — this parser currently folds that legal form into `PortDef`.
/// Do not add `.def_required()` here without first adding real package-level port-usage support.
pub(crate) fn port_def(input: Input<'_>) -> IResult<Input<'_>, Node<PortDef>> {
    parse_port_def(input, false)
}

/// Port definition with required `def` keyword, for contexts (e.g. nested inside a part
/// definition body) where a bare `port` usage form is already dispatched separately via
/// [`port_usage`] -- requiring `def` here prevents a `def`-less port usage from being
/// misclassified as a definition, the same bug class as PAR-001 in `attribute_def`. Unlike
/// [`port_def`] (kept `def`-optional for the package-level bare form documented on that
/// function), this variant is safe to stack ahead of `port_usage` in an `alt(...)` dispatch.
pub(crate) fn port_def_required(input: Input<'_>) -> IResult<Input<'_>, Node<PortDef>> {
    parse_port_def(input, true)
}

fn parse_port_def(input: Input<'_>, require_def: bool) -> IResult<Input<'_>, Node<PortDef>> {
    let start = input;
    let mut options = DefinitionPrefixOptions::new(b"port").with_captured_visibility();
    if require_def {
        options = options.def_required();
    }
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
    use crate::parser::usage::targets_display_string;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    #[test]
    fn port_usage_captures_intersects() {
        let (rest, node) = port_usage(input("port p : PortType intersects a;")).expect("port usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .intersects
                .as_ref()
                .map(|n| targets_display_string(&n.value.target)),
            Some("a".to_string())
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

    /// Regression: the package-level, `def`-less `port p1: MyPortType;` shape used to lose its
    /// type reference entirely -- `parse_definition_prefix`'s shared header parsing swallowed the
    /// `: MyPortType` clause before it reached `PortDef.specializes`, leaving it `None`. It must
    /// now come through as a `Typing`-kind relationship (see `specialization.rs`).
    #[test]
    fn port_def_captures_bare_type_reference_at_package_level() {
        let (rest, node) = port_def(input("port p1: MyPortType;")).expect("port def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.identification.name.as_deref(), Some("p1"));
        let typing = node
            .value
            .specializes
            .expect("type reference must not be dropped");
        assert_eq!(targets_display_string(&typing.value.target), "MyPortType");
        assert_eq!(typing.value.kind, crate::ast::TypingKind::Typing);
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
}
