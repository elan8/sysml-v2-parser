use crate::ast::{AllocationDef, AllocationUsage, Membership, Node};
use crate::parser::body::semicolon_or_structured_definition_body;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::lex::{name, visibility_prefix, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::usage::feature_usage_header;
use crate::parser::Input;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// Allocate end: optional `endName ::>` then an expression (`logical ::> torqueGenerator`).
fn allocation_end_expr(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlConnectorEnd>> {
    crate::parser::constraint::kerml_connector_end(input)
}

pub(crate) fn allocation_def(input: Input<'_>) -> IResult<Input<'_>, Node<AllocationDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"allocation")
            .def_required()
            .no_abstract()
            .with_captured_visibility(),
    )?;
    let (input, body) = semicolon_or_structured_definition_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            AllocationDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
                membership: Membership::owning(prefix.visibility, prefix.visibility_span),
            },
        ),
    ))
}

pub(crate) fn allocation_usage(input: Input<'_>) -> IResult<Input<'_>, Node<AllocationUsage>> {
    let start = input;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"allocation"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, header) = feature_usage_header(input)?;
    let type_name = header.type_reference;
    // `#73`: `allocate logical ::> torqueGenerator to physical ::> powerTrain` — optional
    // end-name + `::>` before each allocate expression (same shape as connect ends).
    let (input, source) = opt(preceded(
        preceded(ws_and_comments, tag(&b"allocate"[..])),
        preceded(ws1, allocation_end_expr),
    ))
    .parse(input)?;
    let (input, target) = match source {
        Some(_) => {
            let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
            let (input, target) = preceded(ws1, allocation_end_expr).parse(input)?;
            (input, Some(target))
        }
        None => (input, None),
    };
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = semicolon_or_structured_definition_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            AllocationUsage {
                name: name_str,
                type_name,
                type_is_conjugated: header.type_is_conjugated,
                subsets: header.subsets,
                redefines: header.redefines,
                source,
                target,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

pub(crate) fn allocate_usage(input: Input<'_>) -> IResult<Input<'_>, Node<AllocationUsage>> {
    let start = input;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"allocate"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, source) = allocation_end_expr(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
    let (input, target) = preceded(ws1, allocation_end_expr).parse(input)?;
    let (input, body) = semicolon_or_structured_definition_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            AllocationUsage {
                name: String::new(),
                type_name: None,
                type_is_conjugated: false,
                subsets: None,
                redefines: None,
                source: Some(source),
                target: Some(target),
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

#[cfg(test)]
mod membership_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    // --- parser work item 4b (final sweep): Membership on AllocationDef/AllocationUsage ---

    #[test]
    fn allocation_def_parses_end_members() {
        let (rest, node) = allocation_def(input(
            "allocation def LogicalToPhysical {\n\tend logical : LogicalElement;\n\tend physical : PhysicalElement;\n}",
        ))
        .expect("allocation def with ends");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let crate::ast::DefinitionBody::Brace { elements, .. } = &node.value.body else {
            panic!("expected brace body");
        };
        assert_eq!(elements.len(), 2);
        for el in elements {
            let crate::ast::DefinitionBodyElement::OccurrenceMember(m) = &el.value else {
                panic!("expected OccurrenceMember, got {:?}", el.value);
            };
            assert!(
                matches!(m.value, crate::ast::OccurrenceBodyElement::EndDecl(_)),
                "expected EndDecl, got {:?}",
                m.value
            );
        }
    }

    #[test]
    fn allocation_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) =
            allocation_def(input("private allocation def A1;")).expect("allocation def");
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
    fn allocation_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = allocation_def(input("allocation def A1;")).expect("allocation def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn allocation_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) =
            allocation_usage(input("protected allocation a1;")).expect("allocation usage");
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
    fn allocation_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = allocation_usage(input("allocation a1;")).expect("allocation usage");
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn allocation_usage_accepts_named_reference_ends() {
        let source =
            input("allocation a : $::Allocations::T allocate logical ::> src to physical ::> dst;");
        let (rest, node) = allocation_usage(source).expect("allocation with ::> ends");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .type_name
                .and_then(|id| crate::parser::usage::reference_text(source, id))
                .as_deref(),
            Some("$::Allocations::T")
        );
        assert!(node.value.source.is_some());
        assert!(node.value.target.is_some());
    }

    #[test]
    fn allocate_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) = allocate_usage(input("public allocate a to b;")).expect("allocate usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn allocate_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = allocate_usage(input("allocate a to b;")).expect("allocate usage");
        assert_eq!(node.value.membership.visibility, None);
    }
}
