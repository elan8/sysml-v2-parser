use crate::ast::{
    CommentAnnotation, ConcernUsage, DocComment, FrameMember, Node, PurposeMember,
    RequireConstraint, RequireConstraintBody, RequirementActorDecl, RequirementDef,
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
    identification, name, qualified_name, skip_statement_or_block, ws, ws1, ws_and_comments,
    REQUIREMENT_BODY_STARTERS,
};
use crate::parser::metadata_annotation::annotation;
use crate::parser::node_from_to;
use crate::parser::usage::{
    feature_usage_header, multiplicity, optional_typings, specialization_clauses,
    targets_display_string,
};
use crate::parser::with_span;
use crate::parser::Input;
use crate::parser::{build_recovery_error_node, build_recovery_error_node_from_span};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::preceded;
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
        "missing_type_reference"
            | "unexpected_keyword_in_scope"
            | "unrecognized_declaration_in_scope"
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
        DefinitionPrefixOptions::new(b"requirement")
            .def_required()
            .with_captured_visibility(),
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
        matches!(recovery.code.as_str(), "missing_type_reference")
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
            map(requirement_usage, |usage| {
                RequirementDefBodyElement::RequirementUsage(Box::new(usage))
            }),
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
            map(
                textual_representation,
                RequirementDefBodyElement::TextualRep,
            ),
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
    parse_requirement_usage_payload_with_abstract(input, default_name, false)
}

pub(crate) fn parse_requirement_usage_payload_with_abstract<'a>(
    input: Input<'a>,
    default_name: Option<&str>,
    already_abstract: bool,
) -> IResult<Input<'a>, RequirementUsage> {
    let (input, _) = ws_and_comments(input)?;
    // Support usage extension keywords where this parser already tolerates them.
    let (input, abstract_kws) = many0(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let is_abstract = already_abstract || !abstract_kws.is_empty();
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
            is_abstract,
            // `variation` is only spelled at the member-position `requirement_usage` parser, which
            // sets this after the fact (this shared payload is also reached from
            // `verify requirement ...` and `objective { requirement ... }`, neither of which has a
            // usage-prefix slot).
            is_variation: false,
            body,
            // No visibility grammar at this shared payload's callers (`verify requirement ...`,
            // `objective { requirement ... }`); the member-position `requirement_usage` parser
            // overrides this after calling into the payload. See `RequirementUsage::membership`.
            membership: crate::ast::Membership::feature(None, crate::ast::Span::dummy()),
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
    let ident = ident_parsed.filter(|i| i.short_name.is_some() || i.name.is_some());
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
    let ident = ident_parsed.filter(|i| i.short_name.is_some() || i.name.is_some());
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
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = opt(preceded(tag(&b"assert"[..]), ws1)).parse(input)?;
    let (input, is_negated) = opt(preceded(tag(&b"not"[..]), ws1))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
    let (input, _) = tag(&b"satisfy"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, (source, inline_requirement)) =
        if let Ok((after_kw, _)) = preceded(tag(&b"requirement"[..]), ws1).parse(input) {
            // Fuller `satisfy requirement <name> : <Type>` form (SysML `SatisfyRequirementUsage`),
            // reusing the shared typing fragment from usage.rs rather than hand-rolling it.
            let inline_start = after_kw;
            let (after_name, req_name) = name(after_kw)?;
            let (after_type, type_suffix) = optional_typings(after_name)?;
            let type_name = type_suffix.map(|(_, is_conjugated, targets)| {
                let type_name = targets_display_string(&targets);
                if is_conjugated {
                    format!("~{type_name}")
                } else {
                    type_name
                }
            });
            let source = node_from_to(
                inline_start,
                after_type,
                crate::ast::Expression::FeatureRef(req_name.clone()),
            );
            (
                after_type,
                (
                    source,
                    Some(crate::ast::InlineSatisfyRequirement {
                        name: req_name,
                        type_name,
                    }),
                ),
            )
        } else {
            let (input, source) = expression(input)?;
            (input, (source, None))
        };
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
                is_negated,
                inline_requirement,
            },
        ),
    ))
}

pub(crate) fn concern_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ConcernUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, _) = nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"concern"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, def_kw) = nom::combinator::opt(preceded(tag(&b"def"[..]), ws1)).parse(input)?;
    let (input, ident) = name(input)?;
    let (input, header) = feature_usage_header(input)?;
    let (input, body) = requirement_def_body(input)?;
    let val = ConcernUsage {
        name: ident,
        type_name: header.type_name,
        body,
        is_definition: def_kw.is_some(),
        membership: crate::ast::Membership::feature(visibility, visibility_span),
    };
    Ok((input, node_from_to(start, input, val)))
}

pub(crate) fn requirement_usage(input: Input<'_>) -> IResult<Input<'_>, Node<RequirementUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, abstract_kw) =
        nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    // §6 G5: `variation requirement engineRqtChoice : EnginePerformanceRequirement { ... }` is
    // real usage in the OMG spec Annex `7b-Variant Configurations.sysml`; the BNF
    // `BasicUsagePrefix` slot allows `variation` wherever it allows `abstract`.
    let (input, variation_kw) =
        nom::combinator::opt(preceded(tag(&b"variation"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"requirement"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, mut val) =
        parse_requirement_usage_payload_with_abstract(input, None, abstract_kw.is_some())?;
    val.is_variation = variation_kw.is_some();
    val.membership = crate::ast::Membership::feature(visibility, visibility_span);
    Ok((input, node_from_to(start, input, val)))
}

#[cfg(test)]
mod membership_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    // --- parser work item 4b (continuation): Membership on RequirementDef/RequirementUsage/ConcernUsage ---

    #[test]
    fn requirement_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) =
            requirement_def(input("private requirement def Need;")).expect("requirement def");
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
    fn requirement_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) =
            requirement_def(input("requirement def Need;")).expect("requirement def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }

    #[test]
    fn requirement_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) = requirement_usage(input("protected requirement need1 : Need;"))
            .expect("requirement usage");
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
    fn requirement_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) =
            requirement_usage(input("requirement need1 : Need;")).expect("requirement usage");
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn concern_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) =
            concern_usage(input("public concern c1 : ConcernType;")).expect("concern usage");
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
    fn concern_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = concern_usage(input("concern c1 : ConcernType;")).expect("concern usage");
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn concern_usage_bare_form_is_not_a_definition() {
        let (rest, node) =
            concern_usage(input("concern c1 : ConcernType;")).expect("concern usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(!node.value.is_definition);
    }

    #[test]
    fn concern_usage_def_form_is_a_definition() {
        let (rest, node) = concern_usage(input("concern def ConcernType;")).expect("concern def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.is_definition);
        assert_eq!(node.value.name, "ConcernType");
    }

    #[test]
    fn concern_usage_def_form_with_type_and_body_is_a_definition() {
        let (rest, node) = concern_usage(input(
            "concern def SafetyConcern : BaseConcern { doc /* d */ }",
        ))
        .expect("concern def with type and body");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.is_definition);
        assert_eq!(node.value.type_name.as_deref(), Some("BaseConcern"));
    }

    /// Payload sites with no visibility grammar of their own (`verify requirement ...`,
    /// `objective { requirement ... }`) always build `visibility: None` -- see
    /// `RequirementUsage::membership`'s doc comment.
    #[test]
    fn verify_requirement_inline_usage_has_no_membership_visibility() {
        let (_, node) = verify_requirement(input("verify requirement r1 : ReqType;"))
            .expect("verify requirement");
        let inline = node
            .value
            .requirement
            .expect("inline requirement usage present");
        assert_eq!(inline.value.membership.visibility, None);
    }
}
