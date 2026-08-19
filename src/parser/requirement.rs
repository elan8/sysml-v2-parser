use crate::ast::{
    CommentAnnotation, ConcernUsage, DocComment, FrameMember, Node, PurposeMember,
    RequireConstraint, RequirementActorDecl, RequirementDef, RequirementDefBody,
    RequirementDefBodyElement, RequirementUsage, SatisfactionSubject, SatisfiedRequirement,
    SatisfyRequirementUsage, StakeholderMember, SubjectDecl, SubjectRef, TextualRepresentation,
    VerifyRequirementMember,
};
use crate::parser::attribute::{attribute_def, attribute_usage, redefinition_feature_binding};
use crate::parser::body::{parse_structured_brace_members_with_skip, BraceMemberSkip};
use crate::parser::constraint::{constraint_def_body, constraint_usage};
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::import::import_;
use crate::parser::lex::{
    identification, name, qualified_reference, recover_body_element, short_name_prefix,
    starts_with_keyword, ws, ws1, ws_and_comments, REQUIREMENT_BODY_STARTERS,
};
use crate::parser::node_from_to;
use crate::parser::occurrence_prefix::{
    keyword_token, next_word_is_reserved, occurrence_usage_prefix, optional_keyword_token,
};
use crate::parser::usage::{feature_usage_header, multiplicity_node, specialization_clauses};
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
            | "malformed_annotation_head"
    ) && !is_redefinition
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            start_after_ws,
            nom::error::ErrorKind::Tag,
        )));
    }

    // Stop at the next member's own starter keyword rather than at the next `;`: a malformed
    // member with no terminator of its own would otherwise take the following member's, which
    // swallows a valid sibling. This is the same boundary the part/usage body scopes recover on,
    // and it matters here because a `SatisfyRequirementUsage` owns a `RequirementBody`.
    let (input, _) = recover_body_element(input, REQUIREMENT_BODY_STARTERS)?;
    if input.location_offset() == start_after_ws.location_offset() {
        return Err(nom::Err::Error(nom::error::Error::new(
            start_after_ws,
            nom::error::ErrorKind::Many0,
        )));
    }
    let recovery = build_recovery_error_node_from_span(
        start_after_ws,
        input,
        REQUIREMENT_BODY_STARTERS,
        "requirement body",
        "recovered_requirement_body_element",
    );
    Ok((
        input,
        RequirementDefBodyElement::Error(node_from_to(start_after_ws, input, recovery)),
    ))
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
                is_abstract: crate::parser::definition_prefix::slot_is_abstract(
                    prefix.basic_prefix.as_ref(),
                ),
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
        crate::parser::body::semicolon_body,
        requirement_def_body_brace,
    ))
    .parse(input)
}

fn requirement_def_body_brace(input: Input<'_>) -> IResult<Input<'_>, RequirementDefBody> {
    let (input, members) = parse_structured_brace_members_with_skip(
        input,
        REQUIREMENT_BODY_STARTERS,
        "requirement body",
        "recovered_requirement_body_element",
        requirement_def_body_element,
        requirement_body_recovery_element,
        // Synchronize on the next member's starter keyword rather than on the next `;`, which a
        // malformed member with no terminator of its own would otherwise borrow from the valid
        // member after it. The part definition and usage bodies already recover this way; it
        // matters here too because a `SatisfyRequirementUsage` owns a `RequirementBody`.
        BraceMemberSkip::BodyElementRecover,
    )?;
    Ok((input, members.into_body()))
}

fn requirement_body_recovery_element(
    start: Input<'_>,
    end: Input<'_>,
) -> Node<RequirementDefBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        REQUIREMENT_BODY_STARTERS,
        "requirement body",
        "recovered_requirement_body_element",
    );
    node_from_to(
        start,
        end,
        RequirementDefBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

fn requirement_def_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<RequirementDefBodyElement>> {
    let start = input;
    // A `#tag` run and a leading `ref` are both `OccurrenceUsagePrefix` slots that a sibling
    // production in this scope would otherwise claim first; see
    // `occurrence_prefix::starts_contended_prefix`.
    {
        let (after_ws, _) = ws_and_comments(input)?;
        if crate::parser::occurrence_prefix::starts_contended_prefix(after_ws) {
            if let Ok((next, usage)) = satisfy(after_ws) {
                let elem = RequirementDefBodyElement::Satisfy(Box::new(usage));
                return Ok((next, node_from_to(start, next, elem)));
            }
            if let Ok((next, usage)) = crate::parser::port::port_usage(after_ws) {
                let elem = RequirementDefBodyElement::PortUsage(Box::new(usage));
                return Ok((next, node_from_to(start, next, elem)));
            }
        }
    }
    let (rest, elem) = alt((
        alt((
            map(
                crate::parser::body::annotating_member,
                RequirementDefBodyElement::Annotating,
            ),
            map(
                crate::parser::metadata_annotation::metadata_keyword_usage,
                RequirementDefBodyElement::MetadataKeywordUsage,
            ),
            map(
                crate::parser::metadata_annotation::metadata_keyword_prefix,
                RequirementDefBodyElement::MetadataKeywordUsage,
            ),
            map(
                crate::parser::dependency::dependency,
                RequirementDefBodyElement::Dependency,
            ),
            map(import_, RequirementDefBodyElement::Import),
            // `subject;` before typed `subject name : Type;` so the shorthand wins.
            map(subject_ref, RequirementDefBodyElement::SubjectRef),
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
            // Keyword-less `:>> name = …` / `:> name …` bindings (validation `09`, `14c`).
            map(
                redefinition_feature_binding,
                RequirementDefBodyElement::AttributeUsage,
            ),
            map(
                crate::parser::part::variant_usage,
                RequirementDefBodyElement::VariantUsage,
            ),
            map(
                verify_requirement,
                RequirementDefBodyElement::VerifyRequirement,
            ),
            map(
                require_constraint,
                RequirementDefBodyElement::RequireConstraint,
            ),
            map(constraint_usage, RequirementDefBodyElement::Constraint),
            map(frame_member, RequirementDefBodyElement::Frame),
            map(stakeholder_member, RequirementDefBodyElement::Stakeholder),
            map(purpose_member, RequirementDefBodyElement::Purpose),
            // Nested in a sub-alt to stay under nom's 21-branch limit.
            alt((
                map(requirement_def, |definition| {
                    RequirementDefBodyElement::RequirementDef(Box::new(definition))
                }),
                map(concern_usage, RequirementDefBodyElement::ConcernUsage),
                map(crate::parser::constraint::calc_usage, |n| {
                    RequirementDefBodyElement::CalcUsage(Box::new(n))
                }),
                map(crate::parser::port::port_usage, |n| {
                    RequirementDefBodyElement::PortUsage(Box::new(n))
                }),
                map(crate::parser::allocation::allocate_usage, |n| {
                    RequirementDefBodyElement::AllocationUsage(Box::new(n))
                }),
                // `RequirementBodyItem → DefinitionBodyItem → … → SatisfyRequirementUsage`, so a
                // satisfy usage is a member of every requirement body -- including the
                // `RequirementBody` that a satisfy usage owns itself.
                map(satisfy, |n| RequirementDefBodyElement::Satisfy(Box::new(n))),
            )),
            map(
                crate::parser::connector::ref_decl,
                RequirementDefBodyElement::RefDecl,
            ),
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
    // `#73`: `requirement <'1.1'> vehicleMass1 : Type { … }` — short name before the usage name.
    // Without this, the `<…>` form fails `name()` and falls through to `ExtendedLibraryDecl`.
    let (input, short_name) = short_name_prefix(input)?;
    let (input, name) = {
        let (peek, _) = ws_and_comments(input)?;
        let starts_body_or_spec = peek.fragment().starts_with(b":")
            || peek.fragment().starts_with(b";")
            || peek.fragment().starts_with(b"{")
            || peek.fragment().starts_with(b"::>")
            || starts_with_keyword(peek.fragment(), b"references")
            || starts_with_keyword(peek.fragment(), b"redefines")
            || starts_with_keyword(peek.fragment(), b"subsets");
        if let Some(default) = default_name {
            if starts_body_or_spec {
                (input, default.to_string())
            } else {
                preceded(ws_and_comments, name).parse(input)?
            }
        } else if starts_body_or_spec {
            // Anonymous usage: `requirement references X { … }` / `requirement : Type { … }`.
            (input, String::new())
        } else {
            preceded(ws_and_comments, name).parse(input)?
        }
    };
    let (input, multiplicity) = opt(crate::parser::usage::multiplicity_node).parse(input)?;
    let (input, header) = feature_usage_header(input)?;
    let (input, value) = opt(preceded(
        ws_and_comments,
        crate::parser::feature_value::feature_value_part,
    ))
    .parse(input)?;
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
            short_name,
            type_name: header.type_reference,
            multiplicity: multiplicity.or(header.multiplicity),
            subsets: post_body_specialization
                .subsets
                .map(|(target, _)| target)
                .or(header.subsets),
            references: post_body_specialization.references.or(header.references),
            is_abstract,
            // `variation` is only spelled at the member-position `requirement_usage` parser, which
            // sets this after the fact (this shared payload is also reached from
            // `verify requirement ...` and `objective { requirement ... }`, neither of which has a
            // usage-prefix slot).
            is_variation: false,
            value,
            direction: None,
            body,
            // No visibility grammar at this shared payload's callers (`verify requirement ...`,
            // `objective { requirement ... }`); the member-position `requirement_usage` parser
            // overrides this after calling into the payload. See `RequirementUsage::membership`.
            membership: crate::ast::Membership::feature(None, crate::ast::Span::dummy()),
        },
    ))
}

fn verify_requirement(input: Input<'_>) -> IResult<Input<'_>, Node<VerifyRequirementMember>> {
    crate::parser::span::reference_transaction(input, verify_requirement_inner)
}

fn verify_requirement_inner(input: Input<'_>) -> IResult<Input<'_>, Node<VerifyRequirementMember>> {
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
                redefines: None,
            },
        )
    } else {
        let (input, target) = qualified_reference(input)?;
        let (input, _) = ws_and_comments(input)?;
        if input.fragment().starts_with(b":>>") {
            let (input, _) = tag(&b":>>"[..]).parse(input)?;
            let (input, redefines) = preceded(ws_and_comments, qualified_reference).parse(input)?;
            let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
            (
                input,
                VerifyRequirementMember {
                    explicit_requirement_keyword: false,
                    requirement: None,
                    target: Some(target),
                    redefines: Some(redefines),
                },
            )
        } else {
            let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
            (
                input,
                VerifyRequirementMember {
                    explicit_requirement_keyword: false,
                    requirement: None,
                    target: Some(target),
                    redefines: None,
                },
            )
        }
    };
    Ok((input, node_from_to(start, input, member)))
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
                declaration_name: decl.value.name,
                target: None,
                type_name: decl.value.type_name,
                is_redefinition: false,
            },
        ),
    ))
}

fn stakeholder_shorthand_member(input: Input<'_>) -> IResult<Input<'_>, Node<StakeholderMember>> {
    crate::parser::span::reference_transaction(input, stakeholder_shorthand_member_inner)
}

fn stakeholder_shorthand_member_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<StakeholderMember>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"stakeholder"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, target) = preceded(ws_and_comments, qualified_reference).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            StakeholderMember {
                declaration_name: String::new(),
                target: Some(target),
                type_name: None,
                is_redefinition: false,
            },
        ),
    ))
}

fn stakeholder_redefinition_member(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<StakeholderMember>> {
    crate::parser::span::reference_transaction(input, stakeholder_redefinition_member_inner)
}

fn stakeholder_redefinition_member_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<StakeholderMember>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"stakeholder"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(&b":>>"[..]).parse(input)?;
    let (input, target) = preceded(ws_and_comments, qualified_reference).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            StakeholderMember {
                declaration_name: String::new(),
                target: Some(target),
                type_name: None,
                is_redefinition: true,
            },
        ),
    ))
}

fn stakeholder_member(input: Input<'_>) -> IResult<Input<'_>, Node<StakeholderMember>> {
    alt((
        stakeholder_redefinition_member,
        stakeholder_typed_member,
        stakeholder_shorthand_member,
    ))
    .parse(input)
}

fn subject_ref(input: Input<'_>) -> IResult<Input<'_>, Node<SubjectRef>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"subject"[..])).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((input, node_from_to(start, input, SubjectRef {})))
}

fn purpose_member(input: Input<'_>) -> IResult<Input<'_>, Node<PurposeMember>> {
    crate::parser::span::reference_transaction(input, purpose_member_inner)
}

fn purpose_member_inner(input: Input<'_>) -> IResult<Input<'_>, Node<PurposeMember>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"purpose"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, target) = preceded(ws_and_comments, qualified_reference).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((input, node_from_to(start, input, PurposeMember { target })))
}

fn frame_member(input: Input<'_>) -> IResult<Input<'_>, Node<FrameMember>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"frame"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, concern_keyword) = opt(preceded(tag(&b"concern"[..]), ws1)).parse(input)?;
    let (input, ident) = identification(input)?;
    let (input, header) = crate::parser::usage::feature_usage_header(input)?;
    let (input, value) = opt(crate::parser::feature_value::feature_value_part).parse(input)?;
    let (input, body) = requirement_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            FrameMember {
                has_concern_keyword: concern_keyword.is_some(),
                name: ident.name.unwrap_or_default(),
                short_name: ident.short_name,
                type_name: header.type_reference,
                multiplicity: header.multiplicity,
                subsets: header.subsets,
                redefines: header.redefines,
                value,
                body,
            },
        ),
    ))
}

pub(crate) fn subject_decl(input: Input<'_>) -> IResult<Input<'_>, Node<SubjectDecl>> {
    crate::parser::span::reference_transaction(input, subject_decl_inner)
}

fn subject_decl_inner(input: Input<'_>) -> IResult<Input<'_>, Node<SubjectDecl>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"subject"[..])).parse(input)?;
    let (input, short_name) = crate::parser::lex::short_name_prefix(input)?;
    let (input, _) = ws_and_comments(input)?;

    // `subject` name? redefines? (`:` type)? redefines? multiplicity? value? `;`
    // Reject bare `subject;` — that is [`subject_ref`]. The anonymous forms start directly at
    // `:`/`:>>`/`[`/`=`/`default` (`subject = expr;`, `subject :>> vehicle = vehicle_large;`,
    // spec42 Gap 35).
    let (input, n) = {
        if input.fragment().starts_with(b":")
            || input.fragment().starts_with(b"[")
            || input.fragment().starts_with(b"=")
            || input.fragment().starts_with(b";")
            || crate::parser::lex::starts_with_keyword(input.fragment(), b"default")
        {
            (input, String::new())
        } else {
            let (input, n) = name(input)?;
            (input, n)
        }
    };
    // `:>>` may be authored before the typing (`subject subj :>> Case::subj;`, Systems Library
    // `UseCases.sysml`) or after it; capture either spelling on the same field (spec42 Gap 35).
    let (input, leading_redefines) = opt(crate::parser::usage::redefinition).parse(input)?;
    let (input, type_name) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, qualified_reference),
    ))
    .parse(input)?;
    // Multiplicity binds to the type and so is written before a trailing `:>>`:
    // `subject subj : View[1] :>> RequirementCheck::subj;` (Systems Library `Views.sysml`).
    // Parsing the redefinition first left the `[1]` in front of it and the whole member failed.
    let (input, multiplicity) = opt(multiplicity_node).parse(input)?;
    let (input, trailing_redefines) = if leading_redefines.is_none() {
        opt(crate::parser::usage::redefinition).parse(input)?
    } else {
        (input, None)
    };
    let redefines = leading_redefines.or(trailing_redefines);
    // `= expr`, `default expr`, and `default = expr` all land on the shared `FeatureValue`
    // clause (`subject generateTorque default engine1.generateTorque;`, OMG spec Annex A).
    let (input, value) = opt(crate::parser::feature_value::feature_value_part).parse(input)?;
    // `;` or a braced body (docs / nested members discarded for now — validation `08`
    // `subject vehicle : Vehicle { doc … }`).
    let (input, _) = alt((
        map(preceded(ws_and_comments, tag(&b";"[..])), |_| ()),
        map(constraint_def_body, |_| ()),
    ))
    .parse(input)?;
    if n.is_empty() && type_name.is_none() && value.is_none() && redefines.is_none() {
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    Ok((
        input,
        node_from_to(
            start,
            input,
            SubjectDecl {
                name: n,
                short_name,
                type_name,
                redefines,
                multiplicity,
                value,
            },
        ),
    ))
}

pub(crate) fn actor_decl(input: Input<'_>) -> IResult<Input<'_>, Node<RequirementActorDecl>> {
    let (input, decl) = requirement_parameter_decl(input, b"actor", "actor")?;
    let Some(type_name) = decl.value.type_name else {
        return Err(nom::Err::Failure(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Verify,
        )));
    };
    Ok((
        input,
        Node::new(
            decl.span,
            RequirementActorDecl {
                name: decl.value.name,
                short_name: decl.value.short_name,
                type_name,
                multiplicity: decl.value.multiplicity,
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
    let (input, short_name) = crate::parser::lex::short_name_prefix(input)?;
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
    let (input, type_name) = preceded(ws_and_comments, qualified_reference).parse(input)?;
    let (input, multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    let (input, _) = alt((
        map(preceded(ws_and_comments, tag(&b";"[..])), |_| ()),
        map(constraint_def_body, |_| ()),
    ))
    .parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            SubjectDecl {
                name: n,
                short_name,
                type_name: Some(type_name),
                redefines: None,
                multiplicity,
                value: None,
            },
        ),
    ))
}

pub(crate) fn require_constraint(input: Input<'_>) -> IResult<Input<'_>, Node<RequireConstraint>> {
    let start = input;
    let (input, assume_kw) = preceded(
        ws_and_comments,
        alt((
            map(tag(&b"assume"[..]), |_| true),
            map(tag(&b"require"[..]), |_| false),
        )),
    )
    .parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, has_constraint_keyword) =
        opt(preceded(tag(&b"constraint"[..]), ws_and_comments)).parse(input)?;
    let has_constraint_keyword = has_constraint_keyword.is_some();
    let (input, _) = ws_and_comments(input)?;
    // `#73` / validation `08`: `assume constraint fuelConstraint { … }` — optional name before
    // body. The keyword-less `require <name>;` form instead *references* an existing
    // constraint, so it captures an arena-backed qualified reference (spec42 gap 29); the
    // `constraint`-keyword form declares a fresh name.
    let (input, name, target) =
        if input.fragment().starts_with(b"{") || input.fragment().starts_with(b";") {
            (input, None, None)
        } else if has_constraint_keyword {
            let (input, n) = name(input)?;
            (input, Some(n), None)
        } else {
            let (input, reference) = qualified_reference(input)?;
            (input, None, Some(reference))
        };
    let (input, body) = constraint_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            RequireConstraint {
                is_assume: assume_kw,
                has_constraint_keyword,
                name,
                target,
                body,
            },
        ),
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
    } else if starts_with_keyword(input.fragment(), b"locale") {
        // GH-91.1: `doc locale "en_US" /* ... */` (no identification) -- without this guard,
        // `identification` below greedily consumes the bare word `locale` itself as the doc
        // comment's own name, leaving nothing for the subsequent `locale` keyword check to
        // match. Real usage: `Simple Tests/CommentTest.sysml:32`.
        let (input, locale) = preceded(
            preceded(ws_and_comments, tag(&b"locale"[..])),
            preceded(ws1, string_value),
        )
        .parse(input)?;
        (input, None, Some(locale))
    } else {
        let (input, ident_parsed) = opt(identification).parse(input)?;
        // `ws`, not `ws_and_comments`: this member's own `/* ... */` body must terminate the
        // search for an optional `locale`. Skipping comments here walked straight past the body
        // and found the *next* member's `locale`, fusing two members into one and discarding
        // this one's text with no diagnostic -- `comment named /* two */` followed by `locale
        // "en_US" /* three */` became a single comment named `named`, in locale `en_US`, whose
        // text was ` three `. Same hazard the `/*` guard above and the body scan below document.
        let (input, locale) = opt(preceded(
            preceded(ws, tag(&b"locale"[..])),
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

/// Bare `locale STRING_VALUE /* ... */` package member (GH-91.1): KerML `Comment`'s
/// `('comment' Identification?)?` prefix is entirely optional, so a comment can legally omit
/// the `comment` keyword altogether, leaving just `('locale' STRING_VALUE)? RegularComment`.
/// `comment_annotation` below requires the `comment` keyword unconditionally, so this handles
/// the omitted-keyword case as its own small parser rather than widening that one (which is
/// reused across many other body contexts). Real usage: `Simple Tests/CommentTest.sysml:25`.
pub(crate) fn bare_locale_comment(input: Input<'_>) -> IResult<Input<'_>, Node<CommentAnnotation>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"locale"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, locale) = string_value(input)?;
    // Use ws (not ws_and_comments) so we don't consume the comment body as a block comment.
    let (input, _) = preceded(ws, tag(&b"/*"[..])).parse(input)?;
    let (input, text_bytes) = nom::bytes::complete::take_until("*/").parse(input)?;
    let (input, _) = tag(&b"*/"[..]).parse(input)?;
    let text = String::from_utf8_lossy(text_bytes.fragment()).to_string();
    Ok((
        input,
        node_from_to(
            start,
            input,
            CommentAnnotation {
                keyword_span: None,
                identification: None,
                about_targets: Vec::new(),
                locale: Some(locale),
                text,
            },
        ),
    ))
}

/// KerML Comment: `( 'comment' Identification ( 'about' Annotation ( ',' Annotation )* )? )?
/// ( 'locale' STRING_VALUE )? body = REGULAR_COMMENT` (8.2.3.3.2).
///
/// Wrapped in a reference transaction because the `about` clause allocates qualified references
/// and this parser is tried speculatively: a comment that fails after its targets are read must
/// not leave them in the document's arena.
pub(crate) fn comment_annotation(input: Input<'_>) -> IResult<Input<'_>, Node<CommentAnnotation>> {
    crate::parser::span::reference_transaction(input, comment_annotation_inner)
}

fn comment_annotation_inner(input: Input<'_>) -> IResult<Input<'_>, Node<CommentAnnotation>> {
    let start = input;
    let (keyword_start, _) = ws_and_comments(input)?;
    let (input, keyword) = tag(&b"comment"[..]).parse(keyword_start)?;
    let keyword_span = node_from_to(keyword_start, input, ()).span;
    let _ = keyword;
    let (input, _) = ws1(input)?;
    // Each guard keeps `identification` from claiming something that is not a name. The body
    // guard is the important one: `identification` skips block comments as trivia, so without it
    // `comment /* a */ doc /* b */` parses as one comment named `doc` and the doc member
    // disappears with no diagnostic. `about` and `locale` are keywords of this same production
    // (GH-91.1), and a bare `locale` would otherwise be taken as the comment's own name.
    let (input, ident_parsed) = if input.fragment().starts_with(b"/*")
        || starts_with_keyword(input.fragment(), b"about")
        || starts_with_keyword(input.fragment(), b"locale")
    {
        (input, None)
    } else {
        opt(identification).parse(input)?
    };
    // `comment about x, y /* ... */`. The clause used to be skipped with an unbounded
    // `take_until("/*")`, which is a raw substring search across the rest of the document: it ran
    // past this member's own end, past the enclosing `}`, and through however many later
    // declarations it took to find a block comment, discarding all of them with no diagnostic --
    // and it dropped the annotated elements themselves. The targets are parsed as the qualified
    // references they are, and the scan is gone.
    let (peek, _) = ws(input)?;
    let (input, about_targets) = if starts_with_keyword(peek.fragment(), b"about") {
        crate::parser::metadata_annotation::parse_about_targets(peek)?
    } else {
        (input, Vec::new())
    };
    // `ws`, not `ws_and_comments`: this member's own `/* ... */` body must terminate the search
    // for an optional `locale`. Skipping comments walked straight past the body and found the
    // *next* member's `locale`, fusing two members into one and discarding this one's text with
    // no diagnostic -- `comment named /* two */` followed by `locale "en_US" /* three */` became
    // a single comment named `named`, in locale `en_US`, whose text was ` three `.
    let (input, locale) = opt(preceded(
        preceded(ws, tag(&b"locale"[..])),
        preceded(ws1, string_value),
    ))
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
                keyword_span: Some(keyword_span),
                identification: ident,
                about_targets,
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
    // BNF: `('rep' Identification)?` is entirely optional -- a bare `language "alf" /* ... */`
    // with no `rep` prefix is valid on its own (GH-86, e.g. `action def setX { language "alf"
    // /* c.x = newX; */ }`, Simple Tests/TextualRepresentationTest.sysml). Previously `rep` was
    // parsed as an unconditional mandatory tag, so this bare form failed even wherever
    // `textual_representation` itself was already dispatched.
    let (input, rep_identification) = {
        let (peek, _) = ws_and_comments(input)?;
        if crate::parser::lex::starts_with_keyword(peek.fragment(), b"rep") {
            let (input, _) = tag(&b"rep"[..]).parse(input)?;
            let (input, _) = ws1(input)?;
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
        } else {
            (input, None)
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

/// `SatisfyRequirementUsage` (SysML 8.2.2.21.2).
///
/// ```text
/// SatisfyRequirementUsage =
///     OccurrenceUsagePrefix 'assert' ( isNegated ?= 'not' ) 'satisfy'
///     ( ownedRelationship += OwnedReferenceSubsetting FeatureSpecializationPart?
///     | 'requirement' UsageDeclaration )
///     ValuePart?
///     ( 'by' ownedRelationship += SatisfactionSubjectMember )?
///     RequirementBody
/// ```
///
/// See [`crate::ast::SatisfyRequirementUsage`] for the two places where the pinned production text
/// omits an optionality marker its own corpus proves is there.
pub(crate) fn satisfy(input: Input<'_>) -> IResult<Input<'_>, Node<SatisfyRequirementUsage>> {
    crate::parser::span::reference_transaction(input, satisfy_inner)
}

/// `( OwnedReferenceSubsetting | 'requirement' UsageDeclaration )` -- the production's two
/// mutually exclusive requirement clauses. The `requirement` keyword is what selects between
/// them, so the choice is made once here rather than inferred later from which field is set.
fn satisfied_requirement(input: Input<'_>) -> IResult<Input<'_>, SatisfiedRequirement> {
    let (after_ws, _) = ws_and_comments(input)?;
    if let Ok((rest, keyword_span)) = keyword_token(after_ws, b"requirement") {
        let (rest, identification) = usage_declaration_identification(rest)?;
        return Ok((
            rest,
            SatisfiedRequirement::Declaration(node_from_to(
                after_ws,
                rest,
                crate::ast::InlineRequirementDeclaration {
                    keyword_span,
                    identification,
                },
            )),
        ));
    }
    // `OwnedReferenceSubsetting = [QualifiedName] | OwnedFeatureChain`: a reference with typed
    // `::` and `.` separators, not an expression.
    let (rest, reference) = crate::parser::lex::reference_path(input)?;
    Ok((rest, SatisfiedRequirement::Reference { reference }))
}

/// `UsageDeclaration`'s `Identification = ( '<' NAME '>' )? ( NAME )?`.
///
/// Both halves are optional, so `satisfy requirement by x;` and `satisfy requirement;` declare an
/// anonymous requirement. `name` accepts any identifier, so an anonymous declaration would
/// otherwise swallow the `by` that follows it; a reserved keyword therefore ends the
/// identification. A *quoted* name is never a keyword, so `satisfy requirement 'by' …` still
/// declares a requirement called `by`.
fn usage_declaration_identification(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::ast::Identification> {
    let (input, short_name) = short_name_prefix(input)?;
    let (after_ws, _) = ws_and_comments(input)?;
    if next_word_is_reserved(after_ws) {
        return Ok((
            input,
            crate::ast::Identification {
                short_name,
                name: None,
            },
        ));
    }
    let (input, declared_name) = opt(preceded(ws_and_comments, name)).parse(input)?;
    Ok((
        input,
        crate::ast::Identification {
            short_name,
            name: declared_name,
        },
    ))
}

/// `'by' ownedRelationship += SatisfactionSubjectMember`.
///
/// The membership chain bottoms out at `FeatureChainMember = [QualifiedName] |
/// OwnedFeatureChainMember`, so the subject is a source-backed reference path.
fn satisfaction_subject(input: Input<'_>) -> IResult<Input<'_>, Node<SatisfactionSubject>> {
    let (after_ws, _) = ws_and_comments(input)?;
    let (rest, by_span) = keyword_token(after_ws, b"by")?;
    let (rest, reference) = crate::parser::lex::reference_path(rest)?;
    Ok((
        rest,
        node_from_to(after_ws, rest, SatisfactionSubject { by_span, reference }),
    ))
}

fn satisfy_inner(input: Input<'_>) -> IResult<Input<'_>, Node<SatisfyRequirementUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    // `BehaviorUsageMember : FeatureMembership = MemberPrefix ownedRelatedElement +=
    // BehaviorUsageElement`: the visibility keyword belongs to the membership and precedes the
    // usage's own `OccurrenceUsagePrefix`.
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, prefix) = occurrence_usage_prefix(input)?;
    let (input, assert_span) = optional_keyword_token(input, b"assert")?;
    let (input, not_span) = optional_keyword_token(input, b"not")?;
    let (input, satisfy_span) = keyword_token(input, b"satisfy")?;
    let (input, requirement) = satisfied_requirement(input)?;
    // `FeatureSpecializationPart?`, shared by both alternatives of the requirement clause.
    let (input, header) = feature_usage_header(input)?;
    if header.intersects.is_some() {
        // `FeatureSpecialization = Typings | Subsettings | References | Crosses | Redefinitions`.
        // KerML's `Intersecting` is not reachable from this SysML production, so accepting it
        // here would silently discard an authored clause the AST has no role for.
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, value) = opt(crate::parser::feature_value::feature_value_part).parse(input)?;
    let (input, subject) = opt(satisfaction_subject).parse(input)?;
    let (input, body) = requirement_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            SatisfyRequirementUsage {
                prefix,
                membership: crate::ast::Membership::feature(visibility, visibility_span),
                assert_span,
                not_span,
                satisfy_span,
                requirement,
                typing: header.typing,
                multiplicity: header.multiplicity,
                multiplicity_modifiers: header.multiplicity_modifiers.clone(),
                subsets: header.subsets,
                redefines: header.redefines,
                references: header.references,
                crosses: header.crosses,
                value,
                subject,
                body,
            },
        ),
    ))
}

pub(crate) fn concern_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ConcernUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, abstract_kw) =
        nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"concern"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, def_kw) = nom::combinator::opt(preceded(tag(&b"def"[..]), ws1)).parse(input)?;
    let (input, ident) = name(input)?;
    let (input, header) = feature_usage_header(input)?;
    let (input, body) = requirement_def_body(input)?;
    let val = ConcernUsage {
        name: ident,
        is_abstract: abstract_kw.is_some(),
        type_name: header.type_reference,
        multiplicity: header.multiplicity,
        subsets: header.subsets,
        redefines: header.redefines,
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
    let (peek, _) = ws_and_comments(input)?;
    let input = if peek.fragment().starts_with(b";") || peek.fragment().starts_with(b"{") {
        peek
    } else {
        let (input, _) = ws1(input)?;
        input
    };
    let (input, mut val) =
        parse_requirement_usage_payload_with_abstract(input, None, abstract_kw.is_some())?;
    val.is_variation = variation_kw.is_some();
    val.membership = crate::ast::Membership::feature(visibility, visibility_span);
    Ok((input, node_from_to(start, input, val)))
}

#[cfg(test)]
mod typed_reference_tests {
    use super::*;
    use crate::ast::{
        QualifiedReferenceArena, QualifiedReferenceId, ReferenceSeparator, SourceStorage,
    };
    use crate::parser::span::ParseContext;

    fn parse_node<T>(
        text: &str,
        parser: for<'a> fn(Input<'a>) -> IResult<Input<'a>, Node<T>>,
    ) -> (Node<T>, SourceStorage, QualifiedReferenceArena) {
        let source = SourceStorage::new(text.to_owned());
        let context = ParseContext::new();
        let (rest, node) = parser(context.input(source.as_str().as_bytes())).expect("parse node");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        (node, source, context.finish())
    }

    fn assert_absolute_two_segment_reference(
        source: &SourceStorage,
        arena: &QualifiedReferenceArena,
        id: QualifiedReferenceId,
        authored: &str,
    ) {
        let reference = arena.get(source, id).expect("qualified reference");
        assert_eq!(reference.authored_text(), authored);
        assert!(reference.metadata.is_absolute);
        assert_eq!(reference.segments.len(), 2);
        assert_eq!(reference.segments[0].separator_before, None);
        assert_eq!(
            reference.segments[1].separator_before,
            Some(ReferenceSeparator::ColonColon)
        );
    }

    #[test]
    fn subject_type_is_an_absolute_source_backed_reference() {
        let (subject, source, arena) =
            parse_node("subject vehicle : $::Domain::Vehicle;", subject_decl);
        assert_absolute_two_segment_reference(
            &source,
            &arena,
            subject.value.type_name.expect("subject type"),
            "$::Domain::Vehicle",
        );
    }

    #[test]
    fn verify_targets_are_distinct_source_backed_references() {
        let (verify, source, arena) = parse_node(
            "verify $::Requirements::Mass :>> Base::Mass;",
            verify_requirement,
        );
        assert_absolute_two_segment_reference(
            &source,
            &arena,
            verify.value.target.expect("verify target"),
            "$::Requirements::Mass",
        );
        let redefines = arena
            .get(
                &source,
                verify.value.redefines.expect("verify redefinition"),
            )
            .expect("redefinition reference");
        assert_eq!(redefines.authored_text(), "Base::Mass");
        assert!(!redefines.metadata.is_absolute);
    }

    #[test]
    fn stakeholder_reference_is_separate_from_declaration_name() {
        let (stakeholder, source, arena) =
            parse_node("stakeholder $::Concerns::Safety;", stakeholder_member);
        assert!(stakeholder.value.declaration_name.is_empty());
        let target = stakeholder.value.target.expect("stakeholder target");
        assert_absolute_two_segment_reference(&source, &arena, target, "$::Concerns::Safety");
    }
}

#[cfg(test)]
mod membership_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
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
        assert!(node.value.type_name.is_some());
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

    #[test]
    fn requirement_usage_accepts_short_name() {
        let (rest, node) = requirement_usage(input(
            "requirement <'1.1'> vehicleMass1: MassLimitationRequirement { subject vehicle : Vehicle; }",
        ))
        .expect("requirement usage with short name");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.short_name.as_deref(), Some("1.1"));
        assert_eq!(node.value.name, "vehicleMass1");
        assert!(node.value.type_name.is_some());
    }
}
