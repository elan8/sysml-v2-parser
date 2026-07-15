use super::prelude::*;
use super::usage::{
    allocate_, connect_, interface_usage, part_ref_usage, part_usage, perform_action_decl,
    perform_usage, variant_usage,
};

/// Part def body: ';' or '{' PartDefBodyElement* '}'
pub(crate) fn part_def_body(input: Input<'_>) -> IResult<Input<'_>, PartDefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b";"[..]), |_| PartDefBody::Semicolon),
        part_def_body_brace,
    ))
    .parse(input)
}

fn try_part_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PartDefBodyElement>> {
    match part_def_body_element(input) {
        Err(e)
            if starts_with_any_keyword(input.fragment(), PART_BODY_STARTERS)
                && starts_with_keyword(input.fragment(), b"part") =>
        {
            if let Ok((next, usage)) = part_usage(input) {
                if next.location_offset() > input.location_offset() {
                    return Ok((
                        next,
                        node_from_to(input, next, PartDefBodyElement::PartUsage(Box::new(usage))),
                    ));
                }
            }
            Err(e)
        }
        other => other,
    }
}

fn part_def_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<PartDefBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        PART_BODY_STARTERS,
        "part definition body",
        "recovered_part_def_body_element",
    );
    if starts_with_any_keyword(start.fragment(), PART_BODY_STARTERS) {
        return node_from_to(
            start,
            end,
            PartDefBodyElement::Error(Node::new(crate::ast::Span::dummy(), recovery)),
        );
    }
    if matches!(
        recovery.code.as_str(),
        "missing_member_name"
            | "missing_type_reference"
            | "invalid_bare_identifier_in_action_body"
            | "invalid_bare_identifier_in_state_body"
            | "unexpected_keyword_in_scope"
            | "missing_semicolon"
            | "missing_body_or_semicolon"
            | "bare_feature_declaration_in_part_def"
            | "invalid_requirement_short_name_syntax"
    ) {
        return node_from_to(
            start,
            end,
            PartDefBodyElement::Error(Node::new(crate::ast::Span::dummy(), recovery)),
        );
    }
    let frag = start.fragment();
    let take = frag.len().min(80);
    let preview = String::from_utf8_lossy(&frag[..take]).trim().to_string();
    node_from_to(start, end, PartDefBodyElement::Other(preview))
}

fn part_def_body_brace(input: Input<'_>) -> IResult<Input<'_>, PartDefBody> {
    let (input, elements) = parse_structured_brace_members_with_skip(
        input,
        PART_BODY_STARTERS,
        "part definition body",
        "recovered_part_def_body_element",
        try_part_def_body_element,
        part_def_body_recovery,
        BraceMemberSkip::BodyElementRecover,
    )?;
    Ok((input, PartDefBody::Brace { elements }))
}

/// Build a `SubsettingRelationship` node from a target and the span of the whole clause,
/// mirroring `usage::subsetting_relationship_node` for the ad hoc `:>`/`:>>` trailing-clause
/// shapes parsed directly in this file (`exhibit_state`, `connection_usage_member`) rather than
/// through `usage::specialization_clauses`.
fn subsetting_relationship_node(
    span: crate::ast::Span,
    kind: crate::ast::SubsettingKind,
    target: String,
) -> Node<crate::ast::SubsettingRelationship> {
    Node::new(
        span.clone(),
        crate::ast::SubsettingRelationship {
            target,
            kind,
            span,
            is_implied: false,
        },
    )
}

/// Exhibit state usage: `exhibit state` name (`:` type)? (`;` or body)
pub(crate) fn exhibit_state(input: Input<'_>) -> IResult<Input<'_>, Node<ExhibitState>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"exhibit"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(&b"state"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, type_name) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let (input, body) = crate::parser::state::state_def_body(input)?;
    let before_redefines = input;
    let (input, redefines) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>>"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let redefines = redefines.map(|target| {
        let span = crate::parser::span_from_to(before_redefines, input);
        subsetting_relationship_node(span, crate::ast::SubsettingKind::Redefines, target)
    });
    let input = if redefines.is_some() {
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
            ExhibitState {
                name: name_str,
                type_name,
                redefines,
                body,
            },
        ),
    ))
}

fn part_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PartDefBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, elem) = alt((
        alt((
            map(doc_comment, PartDefBodyElement::Doc),
            map(comment_annotation, PartDefBodyElement::Comment),
            map(
                crate::parser::metadata_annotation::metadata_keyword_usage,
                PartDefBodyElement::MetadataKeywordUsage,
            ),
            map(
                crate::parser::metadata_annotation::metadata_annotation,
                PartDefBodyElement::MetadataAnnotation,
            ),
            map(annotation, PartDefBodyElement::Annotation),
            map(exhibit_state, PartDefBodyElement::ExhibitState),
            map(calc_usage, PartDefBodyElement::CalcUsage),
            map(perform_action_decl, PartDefBodyElement::Perform),
            map(perform_usage, PartDefBodyElement::Perform),
            map(allocate_, PartDefBodyElement::Allocate),
            map(connection_usage_member, PartDefBodyElement::Connection),
            map(connect_, PartDefBodyElement::Connect),
            map(
                crate::parser::flow::flow_usage_member,
                PartDefBodyElement::FlowUsage,
            ),
            map(part_def, PartDefBodyElement::PartDef),
            map(variant_usage, PartDefBodyElement::VariantUsage),
            map(part_usage, |p| PartDefBodyElement::PartUsage(Box::new(p))),
            map(individual_usage, |n| {
                PartDefBodyElement::OccurrenceUsage(Box::new(n))
            }),
            map(snapshot_usage, |n| {
                PartDefBodyElement::OccurrenceUsage(Box::new(n))
            }),
        )),
        alt((
            map(timeslice_usage, |n| {
                PartDefBodyElement::OccurrenceUsage(Box::new(n))
            }),
            map(then_timeslice_usage, |n| {
                PartDefBodyElement::OccurrenceUsage(Box::new(n))
            }),
            map(occurrence_usage, |n| {
                PartDefBodyElement::OccurrenceUsage(Box::new(n))
            }),
            map(interface_usage, PartDefBodyElement::InterfaceUsage),
            map(interface_def_required, PartDefBodyElement::InterfaceDef),
            map(port_usage, PartDefBodyElement::PortUsage),
            map(part_ref_usage, PartDefBodyElement::Ref),
            map(|i| attribute_def(i, true), PartDefBodyElement::AttributeDef),
            map(attribute_usage, PartDefBodyElement::AttributeUsage),
            map(
                attribute_usage_shorthand,
                PartDefBodyElement::AttributeUsage,
            ),
            map(enum_usage, PartDefBodyElement::EnumerationUsage),
            map(requirement_usage, PartDefBodyElement::RequirementUsage),
            map(item_def_required, PartDefBodyElement::ItemDef),
            map(item_usage, PartDefBodyElement::ItemUsage),
            map(
                crate::parser::occurrence_body::assert_constraint_member,
                PartDefBodyElement::AssertConstraint,
            ),
            map(satisfy, PartDefBodyElement::Satisfy),
            map(opaque_part_member_decl, PartDefBodyElement::OpaqueMember),
        )),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn connection_usage_member(input: Input<'_>) -> IResult<Input<'_>, Node<ConnectionUsageMember>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"connection"[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, name) = if input.fragment().starts_with(b":")
        || input.fragment().starts_with(b"{")
        || input.fragment().starts_with(b";")
    {
        (input, None)
    } else {
        let (input, parsed_name) = name(input)?;
        (input, Some(parsed_name))
    };
    let (input, type_name) = {
        let (peek, _) = ws_and_comments(input)?;
        if peek.fragment().starts_with(b":")
            && !peek.fragment().starts_with(b":>")
            && !peek.fragment().starts_with(b":>>")
        {
            let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
            let (input, parsed_type) = preceded(ws_and_comments, qualified_name).parse(input)?;
            (input, Some(parsed_type))
        } else {
            (input, None)
        }
    };
    let (input, body) = connection_member_body(input)?;
    let before_subsets = input;
    let (input, trailing_subsets) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let subsets = trailing_subsets.map(|target| {
        let span = crate::parser::span_from_to(before_subsets, input);
        subsetting_relationship_node(span, crate::ast::SubsettingKind::Subsets, target)
    });
    let before_redefines = input;
    let (input, trailing_redefines) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>>"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let redefines = trailing_redefines.map(|target| {
        let span = crate::parser::span_from_to(before_redefines, input);
        subsetting_relationship_node(span, crate::ast::SubsettingKind::Redefines, target)
    });
    let input = if subsets.is_some() || redefines.is_some() {
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
            ConnectionUsageMember {
                name,
                type_name,
                body,
                subsets,
                redefines,
            },
        ),
    ))
}

/// Permissive parser for library-style part members not yet modeled with dedicated AST nodes.
/// Examples: `abstract ref action ... { ... }`, `state monitor: StateKind { ... }`.
fn opaque_part_member_decl(input: Input<'_>) -> IResult<Input<'_>, Node<OpaqueMemberDecl>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    if !starts_with_any_keyword(
        input.fragment(),
        &[b"ref", b"action", b"state", b"port", b"connection"],
    ) {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, header_text) =
        crate::parser::lex::take_until_terminator(input, MEMBER_HEADER_UNTIL_BODY)?;
    let keyword = if starts_with_any_keyword(input.fragment(), &[b"ref"]) {
        "ref"
    } else if starts_with_any_keyword(input.fragment(), &[b"action"]) {
        "action"
    } else if starts_with_any_keyword(input.fragment(), &[b"state"]) {
        "state"
    } else if starts_with_any_keyword(input.fragment(), &[b"connection"]) {
        "connection"
    } else {
        "port"
    }
    .to_string();
    let name_str = header_text
        .split(|c: char| {
            c.is_whitespace() || c == ':' || c == '[' || c == ',' || c == '(' || c == ')'
        })
        .filter(|s| !s.is_empty())
        .find(|token| {
            !matches!(
                *token,
                "ref"
                    | "action"
                    | "state"
                    | "port"
                    | "connection"
                    | "part"
                    | "def"
                    | "private"
                    | "protected"
                    | "public"
            )
        })
        .unwrap_or("member")
        .to_string();
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = crate::parser::attribute::attribute_body(input)?;
    let (input, trailing_subsets) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let (input, trailing_redefines) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>>"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let input = if trailing_subsets.is_some() || trailing_redefines.is_some() {
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
            OpaqueMemberDecl {
                keyword,
                name: name_str,
                text: header_text.trim().to_string(),
                body,
            },
        ),
    ))
}
