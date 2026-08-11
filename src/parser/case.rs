use crate::ast::{
    AnalysisCaseDef, AnalysisCaseUsage, CaseDef, CaseUsage, Node, VerificationCaseDef,
    VerificationCaseUsage,
};
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::lex::{name, take_until_terminator, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::usage::usage_header;
use crate::parser::Input;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

pub(crate) fn case_def(input: Input<'_>) -> IResult<Input<'_>, Node<CaseDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"case")
            .def_required()
            .with_captured_visibility(),
    )?;
    let (input, body) = loose_use_case_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            CaseDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                is_abstract: prefix.is_abstract,
                body,
                membership: crate::ast::Membership::owning(
                    prefix.visibility,
                    prefix.visibility_span,
                ),
            },
        ),
    ))
}

pub(crate) fn case_usage(input: Input<'_>) -> IResult<Input<'_>, Node<CaseUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, abstract_kw) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"case"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, usage) = case_like_usage_body(
        input,
        abstract_kw.is_some(),
        crate::ast::Membership::feature(visibility, visibility_span),
    )?;
    Ok((input, node_from_to(start, input, usage)))
}

pub(crate) fn analysis_case_def(input: Input<'_>) -> IResult<Input<'_>, Node<AnalysisCaseDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"analysis")
            .def_required()
            .individual_allowed()
            .with_captured_visibility(),
    )?;
    let (input, body) = loose_use_case_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            AnalysisCaseDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                is_abstract: prefix.is_abstract,
                is_individual: prefix.is_individual,
                body,
                membership: crate::ast::Membership::owning(
                    prefix.visibility,
                    prefix.visibility_span,
                ),
            },
        ),
    ))
}

pub(crate) fn analysis_case_usage(input: Input<'_>) -> IResult<Input<'_>, Node<AnalysisCaseUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, abstract_kw) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    // BNF `OccurrenceUsagePrefix`: `(isIndividual ?= 'individual')?` (GH-90.1), e.g. `individual
    // analysis fuelEconomyAnalysis_1 : FuelEconomyAnalysis_1 { ... }` (Individuals Examples/
    // AnalysisIndividualExample.sysml:79).
    let (input, individual_kw) = opt(preceded(tag(&b"individual"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"analysis"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, usage) = case_like_usage_body(
        input,
        abstract_kw.is_some(),
        crate::ast::Membership::feature(visibility, visibility_span),
    )?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            AnalysisCaseUsage {
                name: usage.name,
                type_name: usage.type_name,
                is_abstract: usage.is_abstract,
                is_individual: individual_kw.is_some(),
                body: usage.body,
                membership: usage.membership,
            },
        ),
    ))
}

pub(crate) fn verification_case_def(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<VerificationCaseDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"verification")
            .def_required()
            .with_captured_visibility(),
    )?;
    let (input, body) = loose_use_case_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            VerificationCaseDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                is_abstract: prefix.is_abstract,
                body,
                membership: crate::ast::Membership::owning(
                    prefix.visibility,
                    prefix.visibility_span,
                ),
            },
        ),
    ))
}

pub(crate) fn verification_case_usage(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<VerificationCaseUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, abstract_kw) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"verification"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, usage) = case_like_usage_body(
        input,
        abstract_kw.is_some(),
        crate::ast::Membership::feature(visibility, visibility_span),
    )?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            VerificationCaseUsage {
                name: usage.name,
                type_name: usage.type_name,
                is_abstract: usage.is_abstract,
                body: usage.body,
                membership: usage.membership,
            },
        ),
    ))
}

fn case_like_usage_body(
    input: Input<'_>,
    is_abstract: bool,
    membership: crate::ast::Membership,
) -> IResult<Input<'_>, CaseUsage> {
    let (input, name) = name(input)?;
    let (input, header) = usage_header(input)?;
    let (input, _) = take_until_terminator(input, b";{")?;
    let (input, body) = loose_use_case_body(input)?;
    Ok((
        input,
        CaseUsage {
            name,
            type_name: header.type_reference,
            is_abstract,
            body,
            membership,
        },
    ))
}

fn loose_use_case_body(input: Input<'_>) -> IResult<Input<'_>, crate::ast::UseCaseDefBody> {
    crate::parser::usecase::use_case_def_body(input)
}

#[cfg(test)]
mod membership_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    // --- parser work item 4b (continuation): Membership on Case/AnalysisCase/VerificationCase families ---

    #[test]
    fn case_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = case_def(input("private case def C1;")).expect("case def");
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
    fn case_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = case_def(input("case def C1;")).expect("case def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn case_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) = case_usage(input("protected case c1 : C1;")).expect("case usage");
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
    fn analysis_case_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) =
            analysis_case_def(input("public analysis def A1;")).expect("analysis case def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }

    #[test]
    fn analysis_case_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) =
            analysis_case_usage(input("private analysis a1 : A1;")).expect("analysis case usage");
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
    fn verification_case_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = verification_case_def(input("private verification def V1;"))
            .expect("verification case def");
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
    fn verification_case_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) = verification_case_usage(input("public verification v1 : V1;"))
            .expect("verification case usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }
}
