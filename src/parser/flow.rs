use crate::ast::{
    FlowDeclaration, FlowDef, FlowEndpoints, FlowUsage, FlowUsageKind, Membership, Node,
    PayloadFeature,
};

type ParsedFlowEndpoints<'a> = nom::IResult<
    Input<'a>,
    (
        Option<Node<crate::ast::KermlConnectorEnd>>,
        Option<Node<crate::ast::KermlConnectorEnd>>,
    ),
>;
use crate::parser::body::semicolon_or_structured_definition_body;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::lex::{name, starts_with_keyword, visibility_prefix, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::usage::{
    conjugated_qualified_name, multiplicity_node, optional_typings, usage_declaration,
    usage_declaration_without_identification,
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
            .individual_allowed()
            .with_captured_visibility(),
    )?;
    let (input, body) = semicolon_or_structured_definition_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            FlowDef {
                definition_prefix: prefix.basic_prefix,
                is_individual: prefix.is_individual,
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
        if let Ok((after_typing, Some((_, is_conjugated, targets, _)))) =
            optional_typings(after_name)
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

fn flow_endpoints(input: Input<'_>) -> ParsedFlowEndpoints<'_> {
    let (peek, _) = ws_and_comments(input)?;
    let fragment = peek.fragment();
    if fragment.starts_with(b";") || fragment.starts_with(b"{") {
        return Ok((input, (None, None)));
    }
    if starts_with_keyword(peek.fragment(), b"from") {
        let (input, _) = preceded(ws_and_comments, tag(&b"from"[..])).parse(input)?;
        let (input, from) =
            preceded(ws1, crate::parser::constraint::kerml_connector_end).parse(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
        let (input, to) =
            preceded(ws1, crate::parser::constraint::kerml_connector_end).parse(input)?;
        return Ok((input, (Some(from), Some(to))));
    }
    // Shorthand: end `to` end (no `from` keyword).
    let start = input;
    let (input, from) = crate::parser::constraint::kerml_connector_end(input)?;
    let (peek, _) = ws_and_comments(input)?;
    if starts_with_keyword(peek.fragment(), b"to") {
        let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
        let (input, to) =
            preceded(ws1, crate::parser::constraint::kerml_connector_end).parse(input)?;
        Ok((input, (Some(from), Some(to))))
    } else {
        // No `to`: this member has no endpoints; leave the input untouched.
        Ok((start, (None, None)))
    }
}

fn flow_usage_with_declaration(input: Input<'_>) -> IResult<Input<'_>, FlowUsage> {
    // `Identification` is optional in `UsageDeclaration`. The `of` keyword starts the following
    // payload clause, so it must not be claimed as an invented declaration name. This remains the
    // declaration-led grammar alternative; the endpoint-only alternative is selected separately.
    let (input, declaration) = if starts_with_keyword(input.fragment(), b"of") {
        usage_declaration_without_identification(input)?
    } else {
        usage_declaration(input)?
    };
    let (input, value) = opt(preceded(
        ws_and_comments,
        crate::parser::feature_value::feature_value_part,
    ))
    .parse(input)?;
    let (input, payload) = optional_payload(input)?;
    let (input, (from, to)) = flow_endpoints(input)?;
    let endpoints = match (from, to) {
        (None, None) => None,
        (Some(from), Some(to)) => Some(FlowEndpoints { from, to }),
        // `flow_endpoints` recognizes the pair atomically, so a one-sided result would be a
        // parser defect rather than a valid AST state.
        (Some(_), None) | (None, Some(_)) => unreachable!("flow endpoints are coupled"),
    };
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = semicolon_or_structured_definition_body(input)?;
    Ok((
        input,
        FlowUsage {
            kind: FlowUsageKind::Flow, // overwritten by caller
            declaration: FlowDeclaration::Declared {
                declaration,
                value,
                payload,
                endpoints,
            },
            body,
            membership: Membership::feature(None, crate::ast::Span::dummy()), // overwritten by caller
        },
    ))
}

fn flow_usage_endpoint_only(input: Input<'_>) -> IResult<Input<'_>, FlowUsage> {
    let (input, (from, to)) = flow_endpoints(input)?;
    let (Some(from), Some(to)) = (from, to) else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Verify,
        )));
    };
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = semicolon_or_structured_definition_body(input)?;
    Ok((
        input,
        FlowUsage {
            kind: FlowUsageKind::Flow,
            declaration: FlowDeclaration::EndpointOnly {
                endpoints: FlowEndpoints { from, to },
            },
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

    let (input, mut usage) =
        crate::parser::span::reference_transaction(input, flow_usage_endpoint_only).or_else(
            |_| crate::parser::span::reference_transaction(input, flow_usage_with_declaration),
        )?;
    usage.kind = kind;
    usage.membership = Membership::feature(visibility, visibility_span);
    Ok((input, node_from_to(start, input, usage)))
}

/// Package-level flow usage (alias for `flow_usage_member`).
pub(crate) fn flow_usage(input: Input<'_>) -> IResult<Input<'_>, Node<FlowUsage>> {
    flow_usage_member(input)
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
