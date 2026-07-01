//! Shared occurrence-style body parsing for occurrence defs and generic `DefinitionBody` users.

use crate::ast::{
    AssertConstraintMember, ConstraintDefBody, DefinitionBody, DefinitionBodyElement, Node,
    OccurrenceBodyElement, OccurrenceUsage, OccurrenceUsageBody, ParseErrorNode,
};
use crate::parser::attribute::attribute_usage;
use crate::parser::body::parse_structured_brace_members;
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::constraint::{structured_constraint_body, StructuredConstraintBody};
use crate::parser::lex::{capture_opaque_member, name, recover_body_element, ws1, ws_and_comments};
use crate::parser::metadata_annotation::annotation;
use crate::parser::node_from_to;
use crate::parser::flow::flow_usage_member;
use crate::parser::part::part_usage;
use crate::parser::requirement::doc_comment;
use crate::parser::usage::{optional_typings, specialization_clauses};
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

pub(crate) const OCCURRENCE_BODY_STARTERS: &[&[u8]] = &[
    b"doc",
    b"assert",
    b"attribute",
    b"flow",
    b"message",
    b"succession",
    b"part",
    b"individual",
    b"occurrence",
    b"snapshot",
    b"timeslice",
    b"@",
    b"#",
    b"end",
    b"ref",
    b"abstract",
    b"private",
    b"in",
    b"connection",
];

const DEFINITION_BODY_OPAQUE_STARTERS: &[&[u8]] =
    &[b"end", b"ref", b"abstract", b"private", b"in", b"connection", b"succession"];

/// `;` or brace body with occurrence-style members (`attribute`, `part`, `occurrence`, …).
pub(crate) fn occurrence_definition_body(input: Input<'_>) -> IResult<Input<'_>, DefinitionBody> {
    occurrence_definition_body_with_labels(
        input,
        "definition body",
        "recovered_definition_body_element",
    )
}

pub(crate) fn occurrence_def_definition_body(input: Input<'_>) -> IResult<Input<'_>, DefinitionBody> {
    occurrence_definition_body_with_labels(
        input,
        "occurrence definition body",
        "recovered_occurrence_def_body_element",
    )
}

fn occurrence_definition_body_with_labels<'a>(
    input: Input<'a>,
    scope_label: &'static str,
    recovery_code: &'static str,
) -> IResult<Input<'a>, DefinitionBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, DefinitionBody::Semicolon));
    }
    let (input, elements) = parse_structured_brace_members(
        input,
        OCCURRENCE_BODY_STARTERS,
        scope_label,
        recovery_code,
        |input| {
            let start = input;
            let (input, element) = nom::branch::alt((
                nom::combinator::map(
                    |i| capture_opaque_member(i, DEFINITION_BODY_OPAQUE_STARTERS),
                    DefinitionBodyElement::Other,
                ),
                nom::combinator::map(occurrence_body_element, DefinitionBodyElement::OccurrenceMember),
            ))
            .parse(input)?;
            Ok((
                input,
                node_from_to(start, input, element),
            ))
        },
        |start, end| {
            let recovery = build_recovery_error_node_from_span(
                start,
                end,
                OCCURRENCE_BODY_STARTERS,
                scope_label,
                recovery_code,
            );
            node_from_to(
                start,
                end,
                DefinitionBodyElement::Error(node_from_to(start, end, recovery)),
            )
        },
    )?;
    Ok((input, DefinitionBody::Brace { elements }))
}

pub(crate) fn occurrence_usage(input: Input<'_>) -> IResult<Input<'_>, Node<OccurrenceUsage>> {
    occurrence_usage_with_modifiers(input, false, false, None)
}

pub(crate) fn individual_usage(input: Input<'_>) -> IResult<Input<'_>, Node<OccurrenceUsage>> {
    let (input, _) = preceded(ws_and_comments, tag(&b"individual"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    occurrence_usage_tail(input, true, false, None)
}

pub(crate) fn snapshot_usage(input: Input<'_>) -> IResult<Input<'_>, Node<OccurrenceUsage>> {
    let (input, _) = preceded(ws_and_comments, tag(&b"snapshot"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    occurrence_usage_tail(input, false, false, Some("snapshot".to_string()))
}

pub(crate) fn timeslice_usage(input: Input<'_>) -> IResult<Input<'_>, Node<OccurrenceUsage>> {
    let (input, _) = preceded(ws_and_comments, tag(&b"timeslice"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    occurrence_usage_tail(input, false, false, Some("timeslice".to_string()))
}

pub(crate) fn then_timeslice_usage(input: Input<'_>) -> IResult<Input<'_>, Node<OccurrenceUsage>> {
    let (input, _) = preceded(ws_and_comments, tag(&b"then"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(&b"timeslice"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    occurrence_usage_tail(input, false, true, Some("timeslice".to_string()))
}

fn occurrence_usage_with_modifiers(
    input: Input<'_>,
    is_individual: bool,
    is_then: bool,
    portion_kind: Option<String>,
) -> IResult<Input<'_>, Node<OccurrenceUsage>> {
    let (input, _) = preceded(ws_and_comments, tag(&b"occurrence"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    occurrence_usage_tail(input, is_individual, is_then, portion_kind)
}

fn occurrence_usage_tail(
    input: Input<'_>,
    is_individual: bool,
    is_then: bool,
    portion_kind: Option<String>,
) -> IResult<Input<'_>, Node<OccurrenceUsage>> {
    let start = input;
    let (input, name_str) = name(input)?;
    let (input, leading_clauses) = specialization_clauses(input)?;
    let (input, type_name) = optional_typings(input)?;
    let type_name = type_name.map(|(_, name)| name);
    let (input, trailing_clauses) = specialization_clauses(input)?;
    let (input, body) = occurrence_usage_body(input)?;
    let (input, post_body_clauses) = specialization_clauses(input)?;
    let subsets = post_body_clauses
        .subsets
        .map(|(name, _filter)| name)
        .or_else(|| trailing_clauses.subsets.map(|(name, _filter)| name))
        .or_else(|| leading_clauses.subsets.map(|(name, _filter)| name));
    let redefines = post_body_clauses
        .redefines
        .or(trailing_clauses.redefines)
        .or(leading_clauses.redefines);
    let references = post_body_clauses
        .references
        .or(trailing_clauses.references)
        .or(leading_clauses.references);
    let crosses = post_body_clauses
        .crosses
        .or(trailing_clauses.crosses)
        .or(leading_clauses.crosses);
    let input = if post_body_clauses.had_any {
        let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
        input
    } else {
        input
    };
    Ok((
        input,
        node_from_to(
            start,
            input,
            OccurrenceUsage {
                is_individual,
                is_then,
                portion_kind,
                name: name_str,
                type_name,
                subsets,
                redefines,
                references,
                crosses,
                body,
            },
        ),
    ))
}

fn occurrence_usage_body(input: Input<'_>) -> IResult<Input<'_>, OccurrenceUsageBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b";"[..]), |_| OccurrenceUsageBody::Semicolon),
        occurrence_usage_body_brace,
    ))
    .parse(input)
}

fn occurrence_usage_body_brace(input: Input<'_>) -> IResult<Input<'_>, OccurrenceUsageBody> {
    let (mut input, _) = tag(&b"{"[..]).parse(input)?;
    let mut elements = Vec::new();
    loop {
        let (next, _) = ws_and_comments(input)?;
        input = next;
        if input.fragment().is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Eof,
            )));
        }
        if input.fragment().starts_with(b"}") {
            let (input, _) = preceded(ws_and_comments, tag(&b"}"[..])).parse(input)?;
            return Ok((input, OccurrenceUsageBody::Brace { elements }));
        }
        match occurrence_body_element(input) {
            Ok((next, element)) => {
                if next.location_offset() == input.location_offset() {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Many0,
                    )));
                }
                elements.push(element);
                input = next;
            }
            Err(_) => {
                let start_unknown = input;
                let (next, _) = recover_body_element(input, OCCURRENCE_BODY_STARTERS)?;
                if next.location_offset() == start_unknown.location_offset() {
                    let (input, _) = crate::parser::body::advance_to_closing_brace(input)?;
                    let (input, _) = preceded(ws_and_comments, tag(&b"}"[..])).parse(input)?;
                    return Ok((input, OccurrenceUsageBody::Brace { elements }));
                }
                let recovery = build_recovery_error_node_from_span(
                    start_unknown,
                    next,
                    OCCURRENCE_BODY_STARTERS,
                    "occurrence body",
                    "recovered_occurrence_body_element",
                );
                let node: Node<ParseErrorNode> = node_from_to(start_unknown, next, recovery);
                elements.push(node_from_to(
                    start_unknown,
                    next,
                    OccurrenceBodyElement::Error(node),
                ));
                input = next;
            }
        }
    }
}

pub(crate) fn occurrence_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<OccurrenceBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, elem) = alt((
        map(doc_comment, OccurrenceBodyElement::Doc),
        map(annotation, OccurrenceBodyElement::Annotation),
        map(
            assert_constraint_member,
            OccurrenceBodyElement::AssertConstraint,
        ),
        map(attribute_usage, OccurrenceBodyElement::AttributeUsage),
        map(flow_usage_member, OccurrenceBodyElement::FlowUsage),
        map(part_usage, |p| {
            OccurrenceBodyElement::PartUsage(Box::new(p))
        }),
        map(individual_usage, |n| {
            OccurrenceBodyElement::OccurrenceUsage(Box::new(n))
        }),
        map(snapshot_usage, |n| {
            OccurrenceBodyElement::OccurrenceUsage(Box::new(n))
        }),
        map(timeslice_usage, |n| {
            OccurrenceBodyElement::OccurrenceUsage(Box::new(n))
        }),
        map(then_timeslice_usage, |n| {
            OccurrenceBodyElement::OccurrenceUsage(Box::new(n))
        }),
        map(occurrence_usage, |n| {
            OccurrenceBodyElement::OccurrenceUsage(Box::new(n))
        }),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn assert_constraint_member(input: Input<'_>) -> IResult<Input<'_>, Node<AssertConstraintMember>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"assert"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, is_negated) = opt(preceded(tag(&b"not"[..]), ws1))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
    let (input, _) = tag(&b"constraint"[..]).parse(input)?;
    let (input, body) = structured_constraint_body(input)?;
    let body = match body {
        StructuredConstraintBody::Semicolon => ConstraintDefBody::Semicolon,
        StructuredConstraintBody::Brace { elements } => ConstraintDefBody::Brace { elements },
    };
    Ok((
        input,
        node_from_to(start, input, AssertConstraintMember { body, is_negated }),
    ))
}
