use crate::ast::{Expression, FlowDef, FlowUsage, FlowUsageKind, Membership, Node, PayloadFeature};

type FlowEndpoints<'a> =
    nom::IResult<Input<'a>, (Option<Node<Expression>>, Option<Node<Expression>>)>;
use crate::parser::body::semicolon_or_structured_definition_body;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::expr::expression;
use crate::parser::lex::{name, starts_with_keyword, visibility_prefix, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::usage::{
    conjugated_qualified_name, feature_usage_header, multiplicity_node, optional_typings,
};
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

pub(crate) fn flow_def(input: Input<'_>) -> IResult<Input<'_>, Node<FlowDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"flow")
            .def_required()
            .with_captured_visibility(),
    )?;
    let (input, body) = semicolon_or_structured_definition_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            FlowDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
                membership: Membership::owning(prefix.visibility, prefix.visibility_span),
            },
        ),
    ))
}

fn flow_usage_keyword(input: Input<'_>) -> IResult<Input<'_>, FlowUsageKind> {
    alt((
        map(
            preceded(tag(&b"succession"[..]), preceded(ws1, tag(&b"flow"[..]))),
            |_| FlowUsageKind::SuccessionFlow,
        ),
        map(tag(&b"message"[..]), |_| FlowUsageKind::Message),
        map(tag(&b"flow"[..]), |_| FlowUsageKind::Flow),
    ))
    .parse(input)
}

fn optional_payload(input: Input<'_>) -> IResult<Input<'_>, Option<Node<PayloadFeature>>> {
    let (peek, _) = ws_and_comments(input)?;
    if starts_with_keyword(peek.fragment(), b"of") {
        let (input, _) = preceded(ws_and_comments, tag(&b"of"[..])).parse(input)?;
        let (input, payload) = preceded(ws1, payload_feature).parse(input)?;
        Ok((input, Some(payload)))
    } else {
        Ok((input, None))
    }
}

/// SysML v2 §8.2.2.16 `PayloadFeature`: an optionally-named feature typed by (and/or given a
/// multiplicity by) the `of` clause -- `of Payload`, `of qty : Payload`, `of qty : Payload[1..3]`.
///
/// Disambiguates "name : Type" from a bare type reference by trying the named form first: an
/// identifier is only treated as the feature's own name if a typing clause (`:`/`typed`/
/// `defined by`) genuinely follows it; otherwise that identifier (or qualified name) is the type
/// reference itself, matching the grammar's second/third `PayloadFeature` alternative (a bare
/// `OwnedFeatureTyping`, no `Identification`). Multiplicity is accepted after either form.
///
/// Scope limit (documented, not silently dropped): does not accept a *leading* multiplicity
/// before the name/type (the grammar's third alternative, `OwnedMultiplicity
/// OwnedFeatureTyping`) -- an extreme edge case with no observed real-world usage, unlike
/// trailing `[mult]` which is common. `feature_usage_header` (used elsewhere) accepts a leading
/// multiplicity too, but discards its value; this function is not built on that combinator.
fn payload_feature(input: Input<'_>) -> IResult<Input<'_>, Node<PayloadFeature>> {
    let start = input;
    if let Ok((after_name, feature_name)) = preceded(ws_and_comments, name).parse(input) {
        if let Ok((after_typing, Some((_, is_conjugated, targets)))) = optional_typings(after_name)
        {
            let type_name = targets.first().copied();
            let (rest, multiplicity) =
                nom::combinator::opt(preceded(ws_and_comments, multiplicity_node))
                    .parse(after_typing)?;
            return Ok((
                rest,
                node_from_to(
                    start,
                    rest,
                    PayloadFeature {
                        name: Some(feature_name),
                        type_name,
                        type_is_conjugated: is_conjugated,
                        multiplicity,
                    },
                ),
            ));
        }
    }
    // Bare type reference: no explicit feature name.
    let (input, (is_conjugated, target)) =
        preceded(ws_and_comments, conjugated_qualified_name).parse(input)?;
    let type_name = Some(target);
    let (input, multiplicity) =
        nom::combinator::opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            PayloadFeature {
                name: None,
                type_name,
                type_is_conjugated: is_conjugated,
                multiplicity,
            },
        ),
    ))
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
            type_name: header.type_reference,
            type_is_conjugated: header.type_is_conjugated,
            payload,
            from,
            to,
            body,
            membership: Membership::feature(None, crate::ast::Span::dummy()), // overwritten by caller
        },
    ))
}

/// Payload-first flow usage (§6 G12): `flow of <payload> (from <a> to <b>)? (`;` | `{ }`)`.
fn flow_usage_payload_first(input: Input<'_>) -> IResult<Input<'_>, FlowUsage> {
    let (input, payload) = optional_payload(input)?;
    let (input, (from, to)) = flow_endpoints(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = semicolon_or_structured_definition_body(input)?;
    Ok((
        input,
        FlowUsage {
            kind: FlowUsageKind::Flow, // overwritten by caller
            name: None,
            type_name: None,
            type_is_conjugated: false,
            payload,
            from,
            to,
            body,
            membership: Membership::feature(None, crate::ast::Span::dummy()), // overwritten by caller
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
            type_is_conjugated: false,
            payload: None,
            from: Some(from),
            to: Some(to),
            body,
            membership: Membership::feature(None, crate::ast::Span::dummy()), // overwritten by caller
        },
    ))
}

/// Unified FlowUsage parser for all structure-usage body contexts.
pub(crate) fn flow_usage_member(input: Input<'_>) -> IResult<Input<'_>, Node<FlowUsage>> {
    let start = input;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, kind) = flow_usage_keyword(input)?;
    let (input, _) = ws1(input)?;

    let (input, mut usage) = {
        let peek = input;
        // §6 G12: `flow of fuel : Fuel from a to b;` puts the payload clause before the endpoints
        // and has no name of its own (OMG spec Annex `3d-Function-based Behavior-item.sysml`).
        // Checked before the name dispatch below, which would otherwise take `of` as the name.
        if starts_with_keyword(peek.fragment(), b"of") {
            flow_usage_payload_first(peek)?
        } else {
            match name(peek) {
                Ok((after_name, _name_str)) => {
                    let (after_name, _) = ws_and_comments(after_name)?;
                    let fragment = after_name.fragment();
                    let is_anonymous =
                        fragment.starts_with(b".") || starts_with_keyword(fragment, b"to");
                    if is_anonymous {
                        flow_usage_anonymous(peek)?
                    } else {
                        flow_usage_named(peek)?
                    }
                }
                Err(_) => flow_usage_anonymous(input)?,
            }
        }
    };
    usage.kind = kind;
    usage.membership = Membership::feature(visibility, visibility_span);
    Ok((input, node_from_to(start, input, usage)))
}

/// Package-level flow usage (alias for `flow_usage_member`).
pub(crate) fn flow_usage(input: Input<'_>) -> IResult<Input<'_>, Node<FlowUsage>> {
    flow_usage_member(input)
}

#[cfg(test)]
mod payload_first_gap_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    /// PARSER_BACKLOG_ROADMAP.md §6, G12: the payload clause may precede the endpoints, with no
    /// name on the flow itself. `of` was previously consumed as the flow's name. Real usage: OMG
    /// spec Annex `3d-Function-based Behavior-item.sysml`.
    #[test]
    fn flow_usage_accepts_a_payload_before_the_endpoints() {
        let source = input(
            "flow of fuel : $::Payloads::Fuel from storageTank.fuelOutPort.fuel to pump.fuelInPort.fuel;",
        );
        let (rest, node) = flow_usage_member(source).expect("payload-first flow");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, None);
        let payload = node.value.payload.expect("payload");
        assert_eq!(payload.value.name.as_deref(), Some("fuel"));
        assert_eq!(
            payload
                .value
                .type_name
                .and_then(|id| crate::parser::usage::reference_text(source, id))
                .as_deref(),
            Some("$::Payloads::Fuel")
        );
        assert!(node.value.from.is_some() && node.value.to.is_some());
    }

    #[test]
    fn flow_usage_accepts_a_payload_before_a_brace_body() {
        let (rest, node) =
            flow_usage_member(input("flow of fuel : Fuel from a.b to c.d { /* note */ }"))
                .expect("payload-first flow");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.payload.is_some());
    }

    /// A flow whose name genuinely is `of`-prefixed (e.g. `offset`) must not take the G12 path.
    #[test]
    fn flow_usage_still_names_identifiers_that_merely_start_with_of() {
        let (rest, node) =
            flow_usage_member(input("flow offset from a to b;")).expect("named flow");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name.as_deref(), Some("offset"));
    }
}

#[cfg(test)]
mod membership_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    // --- parser work item 4b (final sweep): Membership on FlowDef/FlowUsage ---

    #[test]
    fn flow_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = flow_def(input("private flow def F1;")).expect("flow def");
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
    fn flow_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = flow_def(input("flow def F1;")).expect("flow def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn flow_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) =
            flow_usage_member(input("protected flow f1 from a to b;")).expect("flow usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Protected)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn flow_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = flow_usage_member(input("flow f1 from a to b;")).expect("flow usage");
        assert_eq!(node.value.membership.visibility, None);
    }
}
