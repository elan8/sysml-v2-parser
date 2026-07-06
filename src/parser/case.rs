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
    let (input, prefix) =
        parse_definition_prefix(input, DefinitionPrefixOptions::new(b"case").def_required())?;
    let (input, body) = loose_use_case_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            CaseDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                specializes_span: prefix.specializes_span,
                is_abstract: prefix.is_abstract,
                body,
            },
        ),
    ))
}

pub(crate) fn case_usage(input: Input<'_>) -> IResult<Input<'_>, Node<CaseUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, abstract_kw) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"case"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, usage) = case_like_usage_body(input, abstract_kw.is_some())?;
    Ok((input, node_from_to(start, input, usage)))
}

pub(crate) fn analysis_case_def(input: Input<'_>) -> IResult<Input<'_>, Node<AnalysisCaseDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"analysis").def_required(),
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
                specializes_span: prefix.specializes_span,
                is_abstract: prefix.is_abstract,
                body,
            },
        ),
    ))
}

pub(crate) fn analysis_case_usage(input: Input<'_>) -> IResult<Input<'_>, Node<AnalysisCaseUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, abstract_kw) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"analysis"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, usage) = case_like_usage_body(input, abstract_kw.is_some())?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            AnalysisCaseUsage {
                name: usage.name,
                type_name: usage.type_name,
                is_abstract: usage.is_abstract,
                body: usage.body,
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
        DefinitionPrefixOptions::new(b"verification").def_required(),
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
                specializes_span: prefix.specializes_span,
                is_abstract: prefix.is_abstract,
                body,
            },
        ),
    ))
}

pub(crate) fn verification_case_usage(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<VerificationCaseUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, abstract_kw) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"verification"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, usage) = case_like_usage_body(input, abstract_kw.is_some())?;
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
            },
        ),
    ))
}

fn case_like_usage_body(input: Input<'_>, is_abstract: bool) -> IResult<Input<'_>, CaseUsage> {
    let (input, name) = name(input)?;
    let (input, header) = usage_header(input)?;
    let (input, _) = take_until_terminator(input, b";{")?;
    let (input, body) = loose_use_case_body(input)?;
    Ok((
        input,
        CaseUsage {
            name,
            type_name: header.type_name,
            is_abstract,
            body,
        },
    ))
}

fn loose_use_case_body(input: Input<'_>) -> IResult<Input<'_>, crate::ast::UseCaseDefBody> {
    crate::parser::usecase::use_case_def_body(input)
}
