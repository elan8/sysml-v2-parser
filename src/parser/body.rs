//! Shared definition body terminators (semicolon or structured brace).

use crate::ast::{AnnotatingMember, DefinitionBody, Node, ParseErrorNode, RelationshipBodyElement};
use crate::parser::lex::{
    recover_body_element, skip_statement_or_block, skip_until_brace_end, ws_and_comments,
    RELATIONSHIP_BODY_STARTERS,
};
use crate::parser::metadata_annotation::metadata_annotation;
use crate::parser::occurrence_body::occurrence_definition_body;
use crate::parser::requirement::{comment_annotation, doc_comment, textual_representation};
use crate::parser::{build_recovery_error_node_from_span, node_from_to, Input};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::Parser;

/// Parse a KerML `RelationshipBody`-shaped brace body (`;` or `{`
/// doc/comment/rep/metadata* `}`), with recovery-to-`Error` for anything else instead of
/// silently discarding it (BNF `RelationshipBody : Relationship = ';' | '{' (ownedRelationship
/// += OwnedAnnotation)* '}'`). Shared by alias/import/dependency bodies and other leaf bodies
/// with the same annotation-only shape (plain `connect` statement bodies).
pub(crate) fn relationship_body(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::ast::Body<RelationshipBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        return semicolon_body(input);
    }
    let (input, members) = parse_structured_brace_members(
        input,
        RELATIONSHIP_BODY_STARTERS,
        "relationship body",
        "recovered_relationship_body_element",
        relationship_body_member,
        |start, end| {
            let recovery = build_recovery_error_node_from_span(
                start,
                end,
                RELATIONSHIP_BODY_STARTERS,
                "relationship body",
                "recovered_relationship_body_element",
            );
            let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
            node_from_to(start, end, RelationshipBodyElement::Error(node))
        },
    )?;
    Ok((input, members.into_body()))
}

/// The same body, for the callers whose AST field is still an optional member list rather than a
/// shared [`crate::ast::Body`].
pub(crate) fn relationship_body_annotations(
    input: Input<'_>,
) -> IResult<Input<'_>, Option<Vec<Node<RelationshipBodyElement>>>> {
    let (input, body) = relationship_body(input)?;
    let elements = match body {
        crate::ast::Body::Semicolon { .. } => None,
        crate::ast::Body::Brace { elements, .. } => Some(elements),
    };
    Ok((input, elements))
}

/// A full `RelationshipBody` member: the shared annotation subset plus owned related elements
/// (`dependency z to x, y { feature e; }`; BNF `RelationshipBody`'s `ownedRelatedElement`,
/// spec42 Gap 37). `relationship_body_element` stays annotation-only because ref bodies reuse it
/// for their annotation tail.
fn relationship_body_member(input: Input<'_>) -> IResult<Input<'_>, Node<RelationshipBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    if let Ok((input, elem)) = map(crate::parser::constraint::kerml_feature_member, |n| {
        RelationshipBodyElement::KermlFeature(Box::new(n))
    })
    .parse(input)
    {
        return Ok((input, node_from_to(start, input, elem)));
    }
    relationship_body_element(input)
}

/// The grammar's `AnnotatingElement` production (KerML 8.2.3.3.1, SysML 8.2.2.4.1).
///
/// A scope accepts the whole production or none of it, so every scope that owns annotating
/// members dispatches through here rather than repeating the four alternatives.
pub(crate) fn annotating_member(input: Input<'_>) -> IResult<Input<'_>, AnnotatingMember> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b"@") {
        return map(metadata_annotation, AnnotatingMember::MetadataAnnotation).parse(input);
    }
    if let Ok(parsed) = map(doc_comment, AnnotatingMember::Doc).parse(input) {
        return Ok(parsed);
    }
    if let Ok(parsed) = map(comment_annotation, AnnotatingMember::Comment).parse(input) {
        return Ok(parsed);
    }
    map(textual_representation, AnnotatingMember::TextualRep).parse(input)
}

pub(crate) fn relationship_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<RelationshipBodyElement>> {
    let start = input;
    let (input, member) = annotating_member(input)?;
    let elem = RelationshipBodyElement::Annotating(member);
    Ok((input, node_from_to(start, input, elem)))
}

/// How to advance past a member that failed to parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BraceMemberSkip {
    /// `skip_statement_or_block` (view/action-style bodies).
    StatementOrBlock,
    /// `recover_body_element` with the provided starter list (part def bodies).
    BodyElementRecover,
}

/// The `;` alternative of any body, capturing the exact semicolon token.
///
/// Every scope spells this the same way, so no scope needs its own copy of "consume `;` and
/// remember where it was".
pub(crate) fn semicolon_body<E>(input: Input<'_>) -> IResult<Input<'_>, crate::ast::Body<E>> {
    let (start, _) = ws_and_comments(input)?;
    let (rest, _) = tag(&b";"[..]).parse(start)?;
    Ok((
        rest,
        crate::ast::Body::Semicolon {
            semicolon_span: crate::parser::span::span_from_to(start, rest),
        },
    ))
}

/// A parsed brace body: its delimiters plus the members between them.
///
/// The delimiters are captured where they are consumed, so no caller has to re-find `{` or `}` to
/// record where the body began and ended.
pub(crate) struct ParsedBraceMembers<E> {
    pub(crate) open_span: crate::ast::Span,
    pub(crate) elements: Vec<Node<E>>,
    pub(crate) close_span: crate::ast::Span,
}

impl<E> ParsedBraceMembers<E> {
    /// The brace alternative of a shared [`crate::ast::Body`], delimiters included.
    pub(crate) fn into_body(self) -> crate::ast::Body<E> {
        crate::ast::Body::Brace {
            open_span: self.open_span,
            elements: self.elements,
            close_span: self.close_span,
        }
    }
}

/// Parse `{` element* `}` with recovery for unknown members.
pub(crate) fn parse_structured_brace_members<'a, E, F, G>(
    input: Input<'a>,
    starters: &[&[u8]],
    _scope_label: &str,
    _recovery_code: &str,
    parse_element: F,
    map_recovery: G,
) -> IResult<Input<'a>, ParsedBraceMembers<E>>
where
    F: FnMut(Input<'a>) -> IResult<Input<'a>, Node<E>>,
    G: FnMut(Input<'a>, Input<'a>) -> Node<E>,
{
    parse_structured_brace_members_with_skip(
        input,
        starters,
        _scope_label,
        _recovery_code,
        parse_element,
        map_recovery,
        BraceMemberSkip::StatementOrBlock,
    )
}

/// Parse `{` element* `}` with a configurable recovery skip strategy.
pub(crate) fn parse_structured_brace_members_with_skip<'a, E, F, G>(
    input: Input<'a>,
    starters: &[&[u8]],
    _scope_label: &str,
    _recovery_code: &str,
    parse_element: F,
    map_recovery: G,
    skip_mode: BraceMemberSkip,
) -> IResult<Input<'a>, ParsedBraceMembers<E>>
where
    F: FnMut(Input<'a>) -> IResult<Input<'a>, Node<E>>,
    G: FnMut(Input<'a>, Input<'a>) -> Node<E>,
{
    crate::parser::stack::with_nested_body_stack(move || {
        parse_structured_brace_members_inner(
            input,
            starters,
            _scope_label,
            _recovery_code,
            parse_element,
            map_recovery,
            skip_mode,
        )
    })
}

fn parse_structured_brace_members_inner<'a, E, F, G>(
    input: Input<'a>,
    starters: &[&[u8]],
    _scope_label: &str,
    _recovery_code: &str,
    mut parse_element: F,
    mut map_recovery: G,
    skip_mode: BraceMemberSkip,
) -> IResult<Input<'a>, ParsedBraceMembers<E>>
where
    F: FnMut(Input<'a>) -> IResult<Input<'a>, Node<E>>,
    G: FnMut(Input<'a>, Input<'a>) -> Node<E>,
{
    let (open_start, _) = ws_and_comments(input)?;
    let (mut input, _) = tag(&b"{"[..]).parse(open_start)?;
    let open_span = crate::parser::span::span_from_to(open_start, input);
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
        // A redundant `;` between members terminates nothing: it is separator punctuation, not a
        // member, so it is consumed here rather than reaching the member parser and being reported
        // as unrecognized content.
        if input.fragment().starts_with(b";") {
            let (next, _) = tag(&b";"[..]).parse(input)?;
            input = next;
            continue;
        }
        if input.fragment().starts_with(b"}") {
            let (close_start, _) = ws_and_comments(input)?;
            let (input, _) = tag(&b"}"[..]).parse(close_start)?;
            let close_span = crate::parser::span::span_from_to(close_start, input);
            return Ok((
                input,
                ParsedBraceMembers {
                    open_span,
                    elements,
                    close_span,
                },
            ));
        }
        let reference_checkpoint = input.extra.reference_checkpoint();
        match parse_element(input) {
            Ok((next, element)) => {
                if next.location_offset() == input.location_offset() {
                    input.extra.rollback_references(reference_checkpoint);
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Many0,
                    )));
                }
                elements.push(element);
                input = next;
            }
            Err(_) => {
                input.extra.rollback_references(reference_checkpoint);
                let start_unknown = input;
                let (after_ws, _) = ws_and_comments(input)?;
                if after_ws.fragment().starts_with(b"}") {
                    input = after_ws;
                    continue;
                }
                let next = match skip_mode {
                    BraceMemberSkip::StatementOrBlock => {
                        let Ok((next, _)) = skip_statement_or_block(input) else {
                            let (closing, _) = advance_to_closing_brace(input)?;
                            elements.push(map_recovery(start_unknown, closing));
                            let (close_start, _) = ws_and_comments(closing)?;
                            let (input, _) = tag(&b"}"[..]).parse(close_start)?;
                            let close_span = crate::parser::span::span_from_to(close_start, input);
                            return Ok((
                                input,
                                ParsedBraceMembers {
                                    open_span,
                                    elements,
                                    close_span,
                                },
                            ));
                        };
                        next
                    }
                    BraceMemberSkip::BodyElementRecover => {
                        let Ok((next, _)) = recover_body_element(input, starters) else {
                            let (closing, _) = advance_to_closing_brace(input)?;
                            elements.push(map_recovery(start_unknown, closing));
                            let (close_start, _) = ws_and_comments(closing)?;
                            let (input, _) = tag(&b"}"[..]).parse(close_start)?;
                            let close_span = crate::parser::span::span_from_to(close_start, input);
                            return Ok((
                                input,
                                ParsedBraceMembers {
                                    open_span,
                                    elements,
                                    close_span,
                                },
                            ));
                        };
                        next
                    }
                };
                if next.location_offset() == start_unknown.location_offset() {
                    let (closing, _) = advance_to_closing_brace(input)?;
                    elements.push(map_recovery(start_unknown, closing));
                    let (close_start, _) = ws_and_comments(closing)?;
                    let (input, _) = tag(&b"}"[..]).parse(close_start)?;
                    let close_span = crate::parser::span::span_from_to(close_start, input);
                    return Ok((
                        input,
                        ParsedBraceMembers {
                            open_span,
                            elements,
                            close_span,
                        },
                    ));
                }
                let (next, _) = ws_and_comments(next)?;
                if next.fragment().starts_with(b"}") {
                    elements.push(map_recovery(start_unknown, next));
                    input = next;
                    continue;
                }
                elements.push(map_recovery(start_unknown, next));
                input = next;
            }
        }
    }
}

/// `;` or brace body with occurrence-style members for flow/allocation defs and usages.
pub(crate) fn semicolon_or_structured_definition_body(
    input: Input<'_>,
) -> IResult<Input<'_>, DefinitionBody> {
    occurrence_definition_body(input)
}

/// Advance through statements/blocks until the next token is `}`.
pub(crate) fn advance_to_closing_brace(mut input: Input<'_>) -> IResult<Input<'_>, ()> {
    loop {
        let (next, _) = ws_and_comments(input)?;
        if next.fragment().starts_with(b"}") {
            return Ok((next, ()));
        }
        let (next, _) = skip_statement_or_block(next)?;
        if next.location_offset() == input.location_offset() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Many0,
            )));
        }
        input = next;
    }
}
/// none currently; keep for families that still need broad compatibility.
#[allow(dead_code)]
pub(crate) fn semicolon_or_opaque_brace_body(
    input: Input<'_>,
) -> IResult<Input<'_>, DefinitionBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        return semicolon_body(input);
    }
    let open_start = input;
    let (input, _) = tag(&b"{"[..]).parse(open_start)?;
    let open_span = crate::parser::span::span_from_to(open_start, input);
    let (input, _) = skip_until_brace_end(input)?;
    let (close_start, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"}"[..]).parse(close_start)?;
    Ok((
        input,
        DefinitionBody::Brace {
            open_span,
            elements: vec![],
            close_span: crate::parser::span::span_from_to(close_start, input),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::DefinitionBodyElement;

    fn span_input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    #[test]
    fn semicolon_body() {
        let input = span_input(";");
        let (rest, body) = semicolon_or_opaque_brace_body(input).expect("body");
        assert!(matches!(body, DefinitionBody::Semicolon { .. }));
        assert!(rest.fragment().is_empty());
    }

    #[test]
    fn structured_brace_parses_doc_member() {
        let input = span_input("{ doc /* note */ }");
        let (rest, body) = semicolon_or_structured_definition_body(input).expect("body");
        assert!(matches!(
            body,
            DefinitionBody::Brace { ref elements, .. } if !elements.is_empty()
        ));
        assert!(rest.fragment().is_empty());
    }

    #[test]
    fn statement_brace_body_emits_recovery_for_unknown_statements() {
        let input = span_input("{ doc /* note */ x = y; nested { z = q; } }");
        let (rest, body) = semicolon_or_structured_definition_body(input).expect("body");
        let DefinitionBody::Brace { elements, .. } = body else {
            panic!("expected brace body");
        };
        assert!(
            elements
                .iter()
                .any(|element| matches!(element.value, DefinitionBodyElement::Error(_))),
            "unknown statements should surface as recovery Error nodes"
        );
        assert!(rest.fragment().is_empty());
    }
}
