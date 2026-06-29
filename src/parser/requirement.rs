#![allow(dead_code, unused_imports)]

use crate::ast::{
    CommentAnnotation, ConcernUsage, ConstraintBody, DocComment, FrameMember, Node, ParseErrorNode,
    PurposeMember, RequireConstraint, RequireConstraintBody, RequirementActorDecl, RequirementDef,
    RequirementDefBody, RequirementDefBodyElement, RequirementUsage, Satisfy, StakeholderMember,
    SubjectDecl, TextualRepresentation, VerifyRequirementMember,
};
use crate::parser::attribute::{attribute_def, attribute_usage};
use crate::parser::body::parse_structured_brace_members;
use crate::parser::constraint::{structured_constraint_body, StructuredConstraintBody};
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::expr::expression;
use crate::parser::import::import_;
use crate::parser::lex::{
    identification, name, qualified_name, skip_statement_or_block, specialization_operator,
    starts_with_any_keyword, subset_operator, ws, ws1, ws_and_comments, REQUIREMENT_BODY_STARTERS,
};
use crate::parser::metadata_annotation::annotation;
use crate::parser::node_from_to;
use crate::parser::with_span;
use crate::parser::usage::{
    feature_usage_header, multiplicity, specialization_clauses, usage_header,
};
use crate::parser::Input;
use crate::parser::{build_recovery_error_node, build_recovery_error_node_from_span, span_from_to};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

fn other_requirement_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, RequirementDefBodyElement> {
    let (input, _) = ws_and_comments(input)?;
    let start_after_ws = input;

    // If this looks like a genuine syntax error we have a targeted diagnostic for, let the
    // enclosing body recovery path generate an `Error` element so diagnostics are surfaced.
    let trimmed = start_after_ws.fragment();
    let is_redefinition = trimmed.windows(3).any(|w| w == b":>>");
    let diag = build_recovery_error_node(
        start_after_ws,
        REQUIREMENT_BODY_STARTERS,
        "requirement body",
        "recovered_requirement_body_element",
    );
    if matches!(
        diag.code.as_str(),
        "missing_member_name"
            | "missing_type_reference"
            | "unexpected_keyword_in_scope"
            | "missing_expression_after_operator"
            | "unsupported_annotation_syntax"
    ) && !is_redefinition
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            start_after_ws,
            nom::error::ErrorKind::Tag,
        )));
    }

    let (input, _) = skip_statement_or_block(input)?;
    if input.location_offset() == start_after_ws.location_offset() {
        return Err(nom::Err::Error(nom::error::Error::new(
            start_after_ws,
            nom::error::ErrorKind::Many0,
        )));
    }
    let frag = start_after_ws.fragment();
    let take = frag.len().min(80);
    let preview = String::from_utf8_lossy(&frag[..take]).trim().to_string();
    Ok((input, RequirementDefBodyElement::Other(preview)))
}

pub(crate) fn requirement_def(input: Input<'_>) -> IResult<Input<'_>, Node<RequirementDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"requirement").def_required(),
    )?;
    let (input, body) = requirement_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            RequirementDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                specializes_span: prefix.specializes_span,
                body,
            },
        ),
    ))
}

pub(crate) fn requirement_def_body(input: Input<'_>) -> IResult<Input<'_>, RequirementDefBody> {
    alt((
        map(preceded(ws_and_comments, tag(&b";"[..])), |_| {
            RequirementDefBody::Semicolon
        }),
        requirement_def_body_brace,
    ))
    .parse(input)
}

fn requirement_def_body_brace(input: Input<'_>) -> IResult<Input<'_>, RequirementDefBody> {
    let (input, elements) = parse_structured_brace_members(
        input,
        REQUIREMENT_BODY_STARTERS,
        "requirement body",
        "recovered_requirement_body_element",
        requirement_def_body_element,
        requirement_body_recovery_element,
    )?;
    Ok((input, RequirementDefBody::Brace { elements }))
}

fn requirement_body_recovery_element(
    start: Input<'_>,
    end: Input<'_>,
) -> Node<RequirementDefBodyElement> {
    let trimmed = start.fragment();
    let is_libraryish = trimmed.windows(3).any(|w| w == b":>>")
        || trimmed.starts_with(b"ref ")
        || trimmed.starts_with(b"abstract ")
        || trimmed.starts_with(b"return ")
        || trimmed.starts_with(b"objective ");
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        REQUIREMENT_BODY_STARTERS,
        "requirement body",
        "recovered_requirement_body_element",
    );
    let should_error = if is_libraryish {
        matches!(
            recovery.code.as_str(),
            "missing_member_name" | "missing_type_reference"
        )
    } else {
        true
    };
    if should_error {
        node_from_to(
            start,
            end,
            RequirementDefBodyElement::Error(Node::new(crate::ast::Span::dummy(), recovery)),
        )
    } else {
        let frag = start.fragment();
        let take = frag.len().min(80);
        let preview = String::from_utf8_lossy(&frag[..take]).trim().to_string();
        node_from_to(start, end, RequirementDefBodyElement::Other(preview))
    }
}

fn requirement_def_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<RequirementDefBodyElement>> {
    let start = input;
    let (rest, elem) = alt((
        alt((
            map(
                crate::parser::metadata_annotation::metadata_annotation,
                RequirementDefBodyElement::MetadataAnnotation,
            ),
            map(
                crate::parser::metadata_annotation::metadata_keyword_usage,
                RequirementDefBodyElement::MetadataKeywordUsage,
            ),
            map(annotation, RequirementDefBodyElement::Annotation),
            map(import_, RequirementDefBodyElement::Import),
            map(subject_decl, RequirementDefBodyElement::SubjectDecl),
            map(actor_decl, RequirementDefBodyElement::RequirementActorDecl),
            map(
                |i| attribute_def(i, true),
                RequirementDefBodyElement::AttributeDef,
            ),
            map(attribute_usage, RequirementDefBodyElement::AttributeUsage),
            map(
                verify_requirement,
                RequirementDefBodyElement::VerifyRequirement,
            ),
            map(
                require_constraint,
                RequirementDefBodyElement::RequireConstraint,
            ),
            map(frame_member, RequirementDefBodyElement::Frame),
            map(stakeholder_member, RequirementDefBodyElement::Stakeholder),
            map(purpose_member, RequirementDefBodyElement::Purpose),
            map(textual_representation, RequirementDefBodyElement::TextualRep),
            map(doc_comment, RequirementDefBodyElement::Doc),
        )),
        other_requirement_body_element,
    ))
    .parse(input)?;
    Ok((rest, node_from_to(start, rest, elem)))
}

pub(crate) fn parse_requirement_usage_payload<'a>(
    input: Input<'a>,
    default_name: Option<&str>,
) -> IResult<Input<'a>, RequirementUsage> {
    let (input, _) = ws_and_comments(input)?;
    // Support usage extension keywords where this parser already tolerates them.
    let (input, _) = many0(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, name) = {
        let (peek, _) = ws_and_comments(input)?;
        if let Some(default) = default_name {
            if peek.fragment().starts_with(b":")
                || peek.fragment().starts_with(b";")
                || peek.fragment().starts_with(b"{")
            {
                (input, default.to_string())
            } else {
                name(input)?
            }
        } else {
            name(input)?
        }
    };
    let (input, _multiplicity) = opt(multiplicity).parse(input)?;
    let (input, header) = feature_usage_header(input)?;
    let (input, body) = requirement_def_body(input)?;
    let (input, post_body_specialization) = specialization_clauses(input)?;
    let input = if post_body_specialization.had_any {
        let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
        input
    } else {
        input
    };
    Ok((
        input,
        RequirementUsage {
            name,
            type_name: header.type_name,
            subsets: post_body_specialization
                .subsets
                .map(|(target, _)| target)
                .or(header.subsets),
            body,
        },
    ))
}

fn verify_requirement(input: Input<'_>) -> IResult<Input<'_>, Node<VerifyRequirementMember>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"verify"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, member) = if let Ok((input, _)) =
        tag::<_, _, nom::error::Error<Input>>(&b"requirement"[..]).parse(input)
    {
        let (input, requirement) = parse_requirement_usage_payload(input, None)?;
        (
            input,
            VerifyRequirementMember {
                explicit_requirement_keyword: true,
                requirement: Some(node_from_to(start, input, requirement)),
                target: None,
            },
        )
    } else {
        let (input, target) = qualified_name(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
        (
            input,
            VerifyRequirementMember {
                explicit_requirement_keyword: false,
                requirement: None,
                target: Some(target),
            },
        )
    };
    Ok((input, node_from_to(start, input, member)))
}

fn concern_reference_member<'a>(
    input: Input<'a>,
    keyword: &'static [u8],
) -> IResult<Input<'a>, (String, crate::ast::Span)> {
    let (input, _) = preceded(ws_and_comments, tag(keyword)).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, (target_span, target)) =
        preceded(ws_and_comments, with_span(qualified_name)).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((input, (target, target_span)))
}

fn stakeholder_typed_member(input: Input<'_>) -> IResult<Input<'_>, Node<StakeholderMember>> {
    let start = input;
    let (input, decl) = requirement_parameter_decl(input, b"stakeholder", "stakeholder")?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            StakeholderMember {
                name: decl.value.name,
                type_name: Some(decl.value.type_name),
                name_span: decl.span.clone(),
                type_span: Some(decl.span.clone()),
            },
        ),
    ))
}

fn stakeholder_shorthand_member(input: Input<'_>) -> IResult<Input<'_>, Node<StakeholderMember>> {
    let start = input;
    let (input, (name, name_span)) = concern_reference_member(input, b"stakeholder")?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            StakeholderMember {
                name,
                type_name: None,
                name_span,
                type_span: None,
            },
        ),
    ))
}

fn stakeholder_member(input: Input<'_>) -> IResult<Input<'_>, Node<StakeholderMember>> {
    alt((stakeholder_typed_member, stakeholder_shorthand_member)).parse(input)
}

fn purpose_member(input: Input<'_>) -> IResult<Input<'_>, Node<PurposeMember>> {
    let start = input;
    let (input, (target, target_span)) = concern_reference_member(input, b"purpose")?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            PurposeMember {
                target,
                target_span,
            },
        ),
    ))
}

fn frame_member(input: Input<'_>) -> IResult<Input<'_>, Node<FrameMember>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"frame"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, n) = name(input)?;
    let (input, body) = requirement_def_body(input)?;
    Ok((
        input,
        node_from_to(start, input, FrameMember { name: n, body }),
    ))
}

pub(crate) fn subject_decl(input: Input<'_>) -> IResult<Input<'_>, Node<SubjectDecl>> {
    requirement_parameter_decl(input, b"subject", "subject")
}

pub(crate) fn actor_decl(input: Input<'_>) -> IResult<Input<'_>, Node<RequirementActorDecl>> {
    let (input, decl) = requirement_parameter_decl(input, b"actor", "actor")?;
    Ok((
        input,
        Node::new(
            decl.span,
            RequirementActorDecl {
                name: decl.value.name,
                type_name: decl.value.type_name,
            },
        ),
    ))
}

fn requirement_parameter_decl<'a>(
    input: Input<'a>,
    keyword: &'a [u8],
    default_name: &'a str,
) -> IResult<Input<'a>, Node<SubjectDecl>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(keyword)).parse(input)?;
    let (input, n) = {
        let (after_gap, _) = ws_and_comments(input)?;
        if after_gap.fragment().starts_with(b":") {
            (after_gap, default_name.to_string())
        } else {
            let (input, _) = ws1(input)?;
            let (input, n) = name(input)?;
            (input, n)
        }
    };
    let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
    let (input, type_name) = preceded(ws_and_comments, qualified_name).parse(input)?;
    let (input, _) = alt((
        map(preceded(ws_and_comments, tag(&b";"[..])), |_| ()),
        map(structured_constraint_body, |_| ()),
    ))
    .parse(input)?;
    Ok((
        input,
        node_from_to(start, input, SubjectDecl { name: n, type_name }),
    ))
}

pub(crate) fn require_constraint(input: Input<'_>) -> IResult<Input<'_>, Node<RequireConstraint>> {
    let start = input;
    let (input, _) = preceded(
        ws_and_comments,
        alt((tag(&b"require"[..]), tag(&b"assume"[..]))),
    )
    .parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(&b"constraint"[..]).parse(input)?;
    let (input, body) = require_constraint_body(input)?;
    Ok((
        input,
        node_from_to(start, input, RequireConstraint { body }),
    ))
}

pub(crate) fn require_constraint_body(
    input: Input<'_>,
) -> IResult<Input<'_>, RequireConstraintBody> {
    let (input, body) = structured_constraint_body(input)?;
    let body = match body {
        StructuredConstraintBody::Semicolon => RequireConstraintBody::Semicolon,
        StructuredConstraintBody::Brace { elements } => RequireConstraintBody::Brace { elements },
    };
    Ok((input, body))
}

pub(crate) fn constraint_body(input: Input<'_>) -> IResult<Input<'_>, ConstraintBody> {
    let (input, body) = structured_constraint_body(input)?;
    Ok((
        input,
        match body {
            StructuredConstraintBody::Semicolon => ConstraintBody::Semicolon,
            StructuredConstraintBody::Brace { .. } => ConstraintBody::Brace,
        },
    ))
}

/// KerML STRING_VALUE: double-quoted string, returns the inner string.
fn string_value(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"\""[..]).parse(input)?;
    let frag = input.fragment();
    let mut i = 0usize;
    while i < frag.len() {
        if frag[i] == b'\\' && i + 1 < frag.len() {
            i += 2;
            continue;
        }
        if frag[i] == b'"' {
            let s = String::from_utf8_lossy(&frag[..i]).replace("\\\"", "\"");
            let (input, _) = nom::bytes::complete::take(i + 1).parse(input)?;
            return Ok((input, s));
        }
        i += 1;
    }
    let s = String::from_utf8_lossy(frag).replace("\\\"", "\"");
    let (input, _) = nom::bytes::complete::take(frag.len()).parse(input)?;
    Ok((input, s))
}

/// KerML Documentation: 'doc' Identification? ( 'locale' STRING_VALUE )? body = REGULAR_COMMENT.
/// We only parse optional Identification and locale when the next token is not "/*", so that
/// ws_and_comments inside identification does not consume the doc body.
pub(crate) fn doc_comment(input: Input<'_>) -> IResult<Input<'_>, Node<DocComment>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"doc"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, ident_parsed, locale) = if input.fragment().starts_with(b"/*") {
        (input, None, None)
    } else {
        let (input, ident_parsed) = opt(identification).parse(input)?;
        let (input, locale) = opt(preceded(
            preceded(ws_and_comments, tag(&b"locale"[..])),
            preceded(ws1, string_value),
        ))
        .parse(input)?;
        (input, ident_parsed, locale)
    };
    // Use ws (not ws_and_comments) so we don't consume the doc body as a block comment.
    let (input, _) = preceded(ws, tag(&b"/*"[..])).parse(input)?;
    let (input, text_bytes) = nom::bytes::complete::take_until("*/").parse(input)?;
    let (input, _) = tag(&b"*/"[..]).parse(input)?;
    let text = String::from_utf8_lossy(text_bytes.fragment()).to_string();
    let ident = ident_parsed.and_then(|i| {
        if i.short_name.is_some() || i.name.is_some() {
            Some(i)
        } else {
            None
        }
    });
    Ok((
        input,
        node_from_to(
            start,
            input,
            DocComment {
                identification: ident,
                locale,
                text,
            },
        ),
    ))
}

/// KerML Comment: ( 'comment' Identification? )? ( 'locale' STRING_VALUE )? body = REGULAR_COMMENT.
pub(crate) fn comment_annotation(input: Input<'_>) -> IResult<Input<'_>, Node<CommentAnnotation>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"comment"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, ident_parsed) = opt(identification).parse(input)?;
    let (input, locale) = opt(preceded(
        preceded(ws_and_comments, tag(&b"locale"[..])),
        preceded(ws1, string_value),
    ))
    .parse(input)?;
    let (input, _) = nom::bytes::complete::take_until::<_, _, nom::error::Error<Input>>(&b"/*"[..])
        .parse(input)?;
    // Use ws so we don't consume the comment body as a block comment.
    let (input, _) = preceded(ws, tag(&b"/*"[..])).parse(input)?;
    let (input, text_bytes) = nom::bytes::complete::take_until("*/").parse(input)?;
    let (input, _) = tag(&b"*/"[..]).parse(input)?;
    let text = String::from_utf8_lossy(text_bytes.fragment()).to_string();
    let ident = ident_parsed.and_then(|i| {
        if i.short_name.is_some() || i.name.is_some() {
            Some(i)
        } else {
            None
        }
    });
    Ok((
        input,
        node_from_to(
            start,
            input,
            CommentAnnotation {
                identification: ident,
                locale,
                text,
            },
        ),
    ))
}

/// KerML TextualRepresentation: ( 'rep' Identification )? 'language' STRING_VALUE body = REGULAR_COMMENT.
pub(crate) fn textual_representation(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<TextualRepresentation>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"rep"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, rep_identification) = {
        let (peek, _) = ws_and_comments(input)?;
        if crate::parser::lex::starts_with_keyword(peek.fragment(), b"language") {
            (input, None)
        } else {
            let (input, id) = identification(input)?;
            (
                input,
                if id.short_name.is_some() || id.name.is_some() {
                    Some(id)
                } else {
                    None
                },
            )
        }
    };
    // `language STRING_VALUE` is required by the grammar but we parse it
    // resiliently so that a missing or empty language tag produces a node
    // with language_span = None (triggering MISSING_REP_LANGUAGE in the
    // error collector) rather than a hard parse failure.
    let (input, (language_span, language)) = {
        let peek = input;
        let (peek, _) = ws_and_comments(peek)?;
        if crate::parser::lex::starts_with_keyword(peek.fragment(), b"language") {
            let (input, _) = preceded(ws_and_comments, tag(&b"language"[..])).parse(input)?;
            let (input, _) = ws1(input)?;
            let (input, (ls, lang)) = with_span(string_value).parse(input)?;
            (input, (Some(ls), lang))
        } else {
            (input, (None, String::new()))
        }
    };
    // Use ws so we don't consume the body as a block comment.
    let (input, _) = preceded(ws, tag(&b"/*"[..])).parse(input)?;
    let (input, text_bytes) = nom::bytes::complete::take_until("*/").parse(input)?;
    let (input, _) = tag(&b"*/"[..]).parse(input)?;
    let text = String::from_utf8_lossy(text_bytes.fragment()).to_string();
    Ok((
        input,
        node_from_to(
            start,
            input,
            TextualRepresentation {
                rep_identification,
                language,
                language_span,
                text,
            },
        ),
    ))
}

pub(crate) fn satisfy(input: Input<'_>) -> IResult<Input<'_>, Node<Satisfy>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"satisfy"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, source) = expression(input)?;
    let (input, target) = if let Ok((input, _)) = preceded(
        ws_and_comments,
        tag::<_, _, nom::error::Error<Input>>(&b"by"[..]),
    )
    .parse(input)
    {
        let (input, _) = ws1(input)?;
        let (input, target) = expression(input)?;
        (input, target)
    } else {
        // Support shorthand `satisfy RequirementRef;` used in part bodies.
        // We preserve AST shape by mirroring source/target.
        (input, source.clone())
    };
    let (input, (body, body_elements)) = alt((
        map(preceded(ws_and_comments, tag(&b";"[..])), |_| {
            (crate::ast::ConnectBody::Semicolon, None)
        }),
        map(structured_constraint_body, |structured| match structured {
            StructuredConstraintBody::Semicolon => (crate::ast::ConnectBody::Semicolon, None),
            StructuredConstraintBody::Brace { elements } => {
                (crate::ast::ConnectBody::Brace, Some(elements))
            }
        }),
    ))
    .parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Satisfy {
                source,
                target,
                body,
                body_elements,
            },
        ),
    ))
}

pub(crate) fn concern_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ConcernUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"concern"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = nom::combinator::opt(preceded(tag(&b"def"[..]), ws1)).parse(input)?;
    let (input, ident) = name(input)?;
    let (input, header) = feature_usage_header(input)?;
    let (input, body) = requirement_def_body(input)?;
    let val = ConcernUsage {
        name: ident,
        type_name: header.type_name,
        body,
    };
    Ok((input, node_from_to(start, input, val)))
}

pub(crate) fn requirement_usage(input: Input<'_>) -> IResult<Input<'_>, Node<RequirementUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"requirement"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, val) = parse_requirement_usage_payload(input, None)?;
    Ok((input, node_from_to(start, input, val)))
}
