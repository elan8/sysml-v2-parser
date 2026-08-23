//! Grammar-owned `MetadataBody` and its keyword-less reference members.

use crate::ast::{
    Body, MetadataBody, MetadataBodyElement, MetadataBodyRedefinitionOperator, MetadataBodyUsage,
    Node,
};
use crate::parser::body::parse_structured_brace_members;
use crate::parser::feature_value::feature_value_part;
use crate::parser::lex::{qualified_reference, starts_with_keyword, ws1, ws_and_comments};
use crate::parser::{build_recovery_error_node_from_span, node_from_to, with_span, Input};
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

const METADATA_BODY_STARTERS: &[&[u8]] = &[
    b"@",
    b"metadata",
    b"ref",
    b"redefines",
    b":>>",
    b"alias",
    b"import",
];

/// `MetadataBodyUsage = 'ref'? ( ':>>' | 'redefines' )? OwnedRedefinition ValuePart?
/// MetadataBody` (SysML textual BNF 1690-1693).
pub(crate) fn metadata_body_usage(input: Input<'_>) -> IResult<Input<'_>, Node<MetadataBodyUsage>> {
    crate::parser::span::reference_transaction(input, metadata_body_usage_inner)
}

fn metadata_body_usage_inner(input: Input<'_>) -> IResult<Input<'_>, Node<MetadataBodyUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, ref_span) = if starts_with_keyword(input.fragment(), b"ref") {
        let (input, (span, _)) = with_span(tag(&b"ref"[..])).parse(input)?;
        let (input, _) = ws1(input)?;
        (input, Some(span))
    } else {
        (input, None)
    };
    let (input, _) = ws_and_comments(input)?;
    let (input, operator) = if input.fragment().starts_with(b":>>") {
        let (input, (span, _)) = with_span(tag(&b":>>"[..])).parse(input)?;
        let (input, _) = ws_and_comments(input)?;
        (
            input,
            Some(MetadataBodyRedefinitionOperator::ColonGreaterGreater { span }),
        )
    } else if starts_with_keyword(input.fragment(), b"redefines") {
        let (input, (span, _)) = with_span(tag(&b"redefines"[..])).parse(input)?;
        let (input, _) = ws1(input)?;
        (
            input,
            Some(MetadataBodyRedefinitionOperator::Redefines { span }),
        )
    } else {
        (input, None)
    };
    let (input, target) = qualified_reference(input)?;
    let (input, value) = opt(preceded(ws_and_comments, feature_value_part)).parse(input)?;
    let (input, body) = metadata_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            MetadataBodyUsage {
                ref_span,
                operator,
                target,
                value,
                body,
            },
        ),
    ))
}

fn metadata_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<MetadataBodyElement>> {
    let start = input;
    let (input, _) = crate::parser::lex::ws_and_notes(input)?;
    if crate::parser::body::starts_bare_comment(input) {
        let (next, member) = crate::parser::body::attribute_annotating_member(input)?;
        return Ok((
            next,
            node_from_to(start, next, MetadataBodyElement::Annotating(member)),
        ));
    }
    // `MetadataBody = ';' | '{' ( DefinitionMember | MetadataBodyUsageMember | AliasMember
    // | Import )* '}'` (SysML BNF 1677). The keyword-less usage member is tried before the
    // declaration dispatcher because `OwnedRedefinition` is a bare qualified name, which the
    // declaration parsers would otherwise read as the start of a keyword-less usage.
    let (input, element) = nom::branch::alt((
        map(
            crate::parser::body::attribute_annotating_member,
            MetadataBodyElement::Annotating,
        ),
        map(crate::parser::alias::alias_def, MetadataBodyElement::Alias),
        map(crate::parser::import::import_, |import| {
            MetadataBodyElement::Import(Box::new(import))
        }),
        map(metadata_body_usage, MetadataBodyElement::Usage),
        map(crate::parser::attribute::attribute_body_element, |member| {
            MetadataBodyElement::Definition(Box::new(member))
        }),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, element)))
}

fn metadata_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<MetadataBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        METADATA_BODY_STARTERS,
        "metadata body",
        "recovered_metadata_body_element",
    );
    node_from_to(
        start,
        end,
        MetadataBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

/// `MetadataBody = ';' | '{' MetadataBodyElement* '}'`.
pub(crate) fn metadata_body(input: Input<'_>) -> IResult<Input<'_>, MetadataBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let semicolon_start = input;
        let (input, _) = tag(&b";"[..]).parse(semicolon_start)?;
        return Ok((
            input,
            Body::Semicolon {
                semicolon_span: crate::parser::span::span_from_to(semicolon_start, input),
            },
        ));
    }
    let (input, members) = parse_structured_brace_members(
        input,
        METADATA_BODY_STARTERS,
        "metadata body",
        "recovered_metadata_body_element",
        metadata_body_element,
        metadata_body_recovery,
    )?;
    Ok((input, members.into_body()))
}
