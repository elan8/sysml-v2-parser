use crate::ast::{
    ActorDecl, ActorRedefinitionAssignment, ActorUsage, CalcUsage, CaseReturnDecl, FirstSuccession,
    IncludeUseCase, Membership, Node, Objective, ParseErrorNode, RefRedefinition, ReturnRef,
    ReturnRefBody, ReturnRefBodyElement, SubjectRef, ThenDone, ThenIncludeUseCase,
    ThenUseCaseUsage, UseCaseDef, UseCaseDefBody, UseCaseDefBodyElement, UseCaseUsage, Visibility,
};
use crate::parser::attribute::attribute_def;
use crate::parser::body::parse_structured_brace_members;
use crate::parser::constraint::return_expression_stmt;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::expr::expression;
use crate::parser::lex::{
    identification, name, qualified_reference, skip_statement_or_block, take_until_terminator,
    visibility_prefix, ws1, ws_and_comments, USE_CASE_BODY_STARTERS,
};
use crate::parser::node_from_to;
use crate::parser::requirement::{doc_comment, parse_requirement_usage_payload, subject_decl};
use crate::parser::usage::{multiplicity_node, usage_header};
use crate::parser::with_span;
use crate::parser::Input;
use crate::parser::{build_recovery_error_node, build_recovery_error_node_from_span};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::{IResult, Parser};

fn subject_ref(input: Input<'_>) -> IResult<Input<'_>, Node<SubjectRef>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"subject"[..])).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((input, node_from_to(start, input, SubjectRef {})))
}

fn first_succession(input: Input<'_>) -> IResult<Input<'_>, Node<FirstSuccession>> {
    crate::parser::span::reference_transaction(input, first_succession_inner)
}

fn first_succession_inner(input: Input<'_>) -> IResult<Input<'_>, Node<FirstSuccession>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"first"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, target) = qualified_reference(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(start, input, FirstSuccession { target }),
    ))
}

fn then_done(input: Input<'_>) -> IResult<Input<'_>, Node<ThenDone>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"then"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(&b"done"[..]).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((input, node_from_to(start, input, ThenDone {})))
}

pub(crate) fn include_use_case(input: Input<'_>) -> IResult<Input<'_>, Node<IncludeUseCase>> {
    crate::parser::span::reference_transaction(input, include_use_case_inner)
}

fn include_use_case_inner(input: Input<'_>) -> IResult<Input<'_>, Node<IncludeUseCase>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"include"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, target) = qualified_reference(input)?;
    let (input, mult) = opt(multiplicity_node).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = use_case_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            IncludeUseCase {
                target,
                multiplicity: mult,
                body,
            },
        ),
    ))
}

fn then_include_use_case(input: Input<'_>) -> IResult<Input<'_>, Node<ThenIncludeUseCase>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"then"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, include) = include_use_case(input)?;
    Ok((
        input,
        node_from_to(start, input, ThenIncludeUseCase { include }),
    ))
}

fn use_case_usage_tail(
    input: Input<'_>,
    ident: String,
    is_abstract: bool,
    membership: crate::ast::Membership,
) -> IResult<Input<'_>, UseCaseUsage> {
    let (input, header) = usage_header(input)?;
    let (input, _) = take_until_terminator(input, b";{")?;
    let (input, body) = use_case_def_body(input)?;
    Ok((
        input,
        UseCaseUsage {
            name: ident,
            type_name: header.type_reference,
            is_abstract,
            body,
            membership,
        },
    ))
}

fn use_case_usage_in_body(input: Input<'_>) -> IResult<Input<'_>, Node<UseCaseUsage>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"use"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(&b"case"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, ident) = name(input)?;
    // No visibility grammar at this "then use case ..." control-flow position; only the
    // member-position `use_case_usage` parser below captures real visibility.
    let (input, usage) = use_case_usage_tail(
        input,
        ident,
        false,
        crate::ast::Membership::feature(None, crate::ast::Span::dummy()),
    )?;
    Ok((input, node_from_to(start, input, usage)))
}

fn then_use_case_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ThenUseCaseUsage>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"then"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, use_case) = use_case_usage_in_body(input)?;
    Ok((
        input,
        node_from_to(start, input, ThenUseCaseUsage { use_case }),
    ))
}

fn actor_redefinition_assignment(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<ActorRedefinitionAssignment>> {
    crate::parser::span::reference_transaction(input, actor_redefinition_assignment_inner)
}

fn actor_redefinition_assignment_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<ActorRedefinitionAssignment>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"actor"[..])).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b":>>"[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, target) = qualified_reference(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"="[..])).parse(input)?;
    let (input, value) = preceded(ws_and_comments, expression).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(start, input, ActorRedefinitionAssignment { target, value }),
    ))
}

fn ref_redefinition(input: Input<'_>) -> IResult<Input<'_>, Node<RefRedefinition>> {
    crate::parser::span::reference_transaction(input, ref_redefinition_inner)
}

fn ref_redefinition_inner(input: Input<'_>) -> IResult<Input<'_>, Node<RefRedefinition>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"ref"[..])).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b":>>"[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, target) = qualified_reference(input)?;
    let (body_start, _) = ws_and_comments(input)?;
    let (input, body) = use_case_def_body(body_start)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            RefRedefinition {
                target,
                body: node_from_to(body_start, input, body),
            },
        ),
    ))
}

/// Parses `return [attribute|part]? [:>>]? <name>? [:|:> <type>] [mult]? [=|:= <expr>] ;`
/// or anonymous `return : Type [= expr];`.
///
/// Handles the common verification/analysis case body forms where `return` declares
/// the output parameter (e.g. `return verdict : VerdictKind = ...`).
/// This is tried before `return_ref` so that `return ref` forms still reach `return_ref`.
fn case_return_decl(input: Input<'_>) -> IResult<Input<'_>, Node<CaseReturnDecl>> {
    crate::parser::span::reference_transaction(input, case_return_decl_inner)
}

fn case_return_decl_inner(input: Input<'_>) -> IResult<Input<'_>, Node<CaseReturnDecl>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"return"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    // Reject `return ref ...` — that's handled by return_ref.
    let (after_ws, _) = ws_and_comments(input)?;
    if crate::parser::lex::starts_with_keyword(after_ws.fragment(), b"ref") {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    // Optional `attribute` / `part` keywords (`return part :>> selectedAlternative : Engine;`).
    let (input, _) = ws_and_comments(input)?;
    let (input, feature_kind) = opt(alt((
        map(nom::bytes::complete::tag(&b"attribute"[..]), |_| {
            crate::ast::CaseReturnFeatureKind::Attribute
        }),
        map(nom::bytes::complete::tag(&b"part"[..]), |_| {
            crate::ast::CaseReturnFeatureKind::Part
        }),
    )))
    .parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    // Optional `:>>` redefine — may appear before or after the feature-kind keyword; we already
    // consumed the keyword above, so only the post-keyword form remains here.
    let (input, is_redefine) = if input.fragment().starts_with(b":>>") {
        let (input, _) = tag(&b":>>"[..]).parse(input)?;
        (input, true)
    } else {
        (input, false)
    };
    let (input, _) = ws_and_comments(input)?;
    // A `:>>` form carries a semantic target, not a declaration name. Ordinary and anonymous
    // return declarations keep those grammar alternatives structurally distinct.
    let (input, (declaration_name, name_span, target)) = if is_redefine {
        let (input, target) = qualified_reference(input)?;
        (input, (String::new(), None, Some(target)))
    } else if input.fragment().starts_with(b":") && !input.fragment().starts_with(b":>") {
        (input, (String::new(), None, None))
    } else {
        let (input, (span, declaration_name)) = with_span(name).parse(input)?;
        (input, (declaration_name, Some(span), None))
    };
    // Optional `: type` or `:> type`.
    let (input, _) = ws_and_comments(input)?;
    let (input, (is_subsetting, type_name)) = if input.fragment().starts_with(b":>>") {
        (input, (false, None))
    } else if input.fragment().starts_with(b":>") {
        let (input, _) = tag(&b":>"[..]).parse(input)?;
        let (input, _) = ws_and_comments(input)?;
        let (input, tn) = crate::parser::lex::qualified_reference(input)?;
        (input, (true, Some(tn)))
    } else if input.fragment().starts_with(b":") {
        let (input, _) = tag(&b":"[..]).parse(input)?;
        let (input, _) = ws_and_comments(input)?;
        let (input, tn) = crate::parser::lex::qualified_reference(input)?;
        (input, (false, Some(tn)))
    } else {
        (input, (false, None))
    };
    let (input, multiplicity) = opt(crate::parser::usage::multiplicity_node).parse(input)?;
    let (input, value) = opt(preceded(
        ws_and_comments,
        crate::parser::feature_value::feature_value_part,
    ))
    .parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            CaseReturnDecl {
                declaration_name,
                name_span,
                target,
                type_name,
                value,
                is_subsetting,
                feature_kind,
                multiplicity,
            },
        ),
    ))
}

fn return_ref_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<ReturnRefBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, element) = alt((
        map(doc_comment, ReturnRefBodyElement::Doc),
        map(return_expression_stmt, ReturnRefBodyElement::Result),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, element)))
}

fn return_ref_body(input: Input<'_>) -> IResult<Input<'_>, Node<ReturnRefBody>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, node_from_to(start, input, ReturnRefBody::Semicolon)));
    }
    let starters: &[&[u8]] = &[b"doc", b"return"];
    let (input, elements) = parse_structured_brace_members(
        input,
        starters,
        "return reference body",
        "recovered_return_ref_body_element",
        return_ref_body_element,
        |start, end| {
            let recovery = build_recovery_error_node_from_span(
                start,
                end,
                starters,
                "return reference body",
                "recovered_return_ref_body_element",
            );
            let error = node_from_to(start, end, recovery);
            node_from_to(start, end, ReturnRefBodyElement::Error(error))
        },
    )?;
    Ok((
        input,
        node_from_to(start, input, ReturnRefBody::Brace { elements }),
    ))
}

fn return_ref(input: Input<'_>) -> IResult<Input<'_>, Node<ReturnRef>> {
    crate::parser::span::reference_transaction(input, return_ref_inner)
}

fn return_ref_inner(input: Input<'_>) -> IResult<Input<'_>, Node<ReturnRef>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"return"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(&b"ref"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, n) = name(input)?;
    let (input, mult) = opt(multiplicity_node).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = return_ref_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ReturnRef {
                name: n,
                multiplicity: mult,
                body,
            },
        ),
    ))
}

fn map_use_case_body_recovery(start: Input<'_>, end: Input<'_>) -> UseCaseDefBodyElement {
    let trimmed = start.fragment();
    let is_redefinition = trimmed.windows(3).any(|w| w == b":>>");
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        USE_CASE_BODY_STARTERS,
        "use case body",
        "recovered_use_case_body_element",
    );
    let should_error =
        matches!(recovery.code.as_str(), "missing_type_reference") && !is_redefinition;
    if should_error {
        let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
        UseCaseDefBodyElement::Error(node)
    } else {
        let frag = start.fragment();
        let take = frag.len().min(80);
        let preview = String::from_utf8_lossy(&frag[..take]).trim().to_string();
        UseCaseDefBodyElement::Other(preview)
    }
}

fn other_use_case_body_element(input: Input<'_>) -> IResult<Input<'_>, UseCaseDefBodyElement> {
    let (input, _) = ws_and_comments(input)?;
    let start_after_ws = input;

    // If this looks like a genuine syntax error we have a targeted diagnostic for (e.g. `actor: User;`),
    // let the body recovery path create an `Error` element so `parse_with_diagnostics` surfaces it.
    let trimmed = start_after_ws.fragment();
    let is_redefinition = trimmed.windows(3).any(|w| w == b":>>");
    let diag = build_recovery_error_node(
        start_after_ws,
        USE_CASE_BODY_STARTERS,
        "use case body",
        "recovered_use_case_body_element",
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
    Ok((input, UseCaseDefBodyElement::Other(preview)))
}

pub(crate) fn actor_decl(input: Input<'_>) -> IResult<Input<'_>, Node<ActorDecl>> {
    let start = input;
    let (input, _) = tag(&b"actor"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, ident) = identification(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ActorDecl {
                identification: ident,
            },
        ),
    ))
}

/// use case name ( : type )? CaseBody
pub(crate) fn use_case_usage(input: Input<'_>) -> IResult<Input<'_>, Node<UseCaseUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, abstract_kw) =
        nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"use"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(&b"case"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, ident) = name(input)?;
    let (input, usage) = use_case_usage_tail(
        input,
        ident,
        abstract_kw.is_some(),
        crate::ast::Membership::feature(visibility, visibility_span),
    )?;
    Ok((input, node_from_to(start, input, usage)))
}

pub(crate) fn use_case_def(input: Input<'_>) -> IResult<Input<'_>, Node<UseCaseDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"use")
            .with_second_keyword(b"case")
            .def_required()
            .with_captured_visibility(),
    )?;
    let (input, body) = use_case_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            UseCaseDef {
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

pub(crate) fn use_case_def_body(input: Input<'_>) -> IResult<Input<'_>, UseCaseDefBody> {
    alt((
        map(preceded(ws_and_comments, tag(&b";"[..])), |_| {
            UseCaseDefBody::Semicolon
        }),
        use_case_def_body_brace,
    ))
    .parse(input)
}

fn use_case_def_body_brace(input: Input<'_>) -> IResult<Input<'_>, UseCaseDefBody> {
    let (input, elements) = parse_structured_brace_members(
        input,
        USE_CASE_BODY_STARTERS,
        "use case body",
        "recovered_use_case_body_element",
        use_case_def_body_element,
        |start, end| node_from_to(start, end, map_use_case_body_recovery(start, end)),
    )?;
    Ok((input, UseCaseDefBody::Brace { elements }))
}

pub(crate) fn use_case_def_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<UseCaseDefBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, elem) = alt((
        alt((
            map(doc_comment, UseCaseDefBodyElement::Doc),
            map(
                crate::parser::metadata_annotation::metadata_annotation,
                UseCaseDefBodyElement::MetadataAnnotation,
            ),
            map(
                crate::parser::metadata_annotation::annotation,
                UseCaseDefBodyElement::Annotation,
            ),
            map(
                crate::parser::metadata_annotation::metadata_keyword_usage,
                UseCaseDefBodyElement::MetadataKeywordUsage,
            ),
            map(
                |i| attribute_def(i, true),
                UseCaseDefBodyElement::AttributeDef,
            ),
            map(
                crate::parser::attribute::directed_attribute_usage,
                UseCaseDefBodyElement::AttributeUsage,
            ),
            map(
                crate::parser::attribute::attribute_usage,
                UseCaseDefBodyElement::AttributeUsage,
            ),
            map(subject_decl, UseCaseDefBodyElement::SubjectDecl),
            map(subject_ref, UseCaseDefBodyElement::SubjectRef),
            map(actor_usage, UseCaseDefBodyElement::ActorUsage),
            map(
                actor_redefinition_assignment,
                UseCaseDefBodyElement::ActorRedefinitionAssignment,
            ),
            map(objective, UseCaseDefBodyElement::Objective),
            map(first_succession, UseCaseDefBodyElement::FirstSuccession),
        )),
        alt((
            map(then_done, UseCaseDefBodyElement::ThenDone),
            map(
                then_include_use_case,
                UseCaseDefBodyElement::ThenIncludeUseCase,
            ),
            map(then_use_case_usage, UseCaseDefBodyElement::ThenUseCaseUsage),
            map(include_use_case, UseCaseDefBodyElement::IncludeUseCase),
            map(ref_redefinition, UseCaseDefBodyElement::RefRedefinition),
            map(
                crate::parser::occurrence_body::assert_constraint_member,
                UseCaseDefBodyElement::AssertConstraint,
            ),
            map(case_return_decl, UseCaseDefBodyElement::CaseReturnDecl),
            map(return_ref, UseCaseDefBodyElement::ReturnRef),
            map(
                crate::parser::action::assign_stmt,
                UseCaseDefBodyElement::Assign,
            ),
            map(
                crate::parser::action::for_loop,
                UseCaseDefBodyElement::ForLoop,
            ),
            map(
                crate::parser::action::then_action,
                UseCaseDefBodyElement::ThenAction,
            ),
            map(crate::parser::action::action_usage, |n| {
                UseCaseDefBodyElement::ActionUsage(Box::new(n))
            }),
            map(crate::parser::case::analysis_case_usage, |n| {
                UseCaseDefBodyElement::AnalysisCaseUsage(Box::new(n))
            }),
            map(directed_calc_usage, |n| {
                UseCaseDefBodyElement::CalcUsage(Box::new(n))
            }),
            map(crate::parser::constraint::calc_usage, |n| {
                UseCaseDefBodyElement::CalcUsage(Box::new(n))
            }),
            map(directed_requirement_usage, |n| {
                UseCaseDefBodyElement::RequirementUsage(Box::new(n))
            }),
            map(crate::parser::requirement::requirement_usage, |n| {
                UseCaseDefBodyElement::RequirementUsage(Box::new(n))
            }),
            map(crate::parser::part::part_usage, |n| {
                UseCaseDefBodyElement::PartUsage(Box::new(n))
            }),
            map(
                crate::parser::flow::flow_usage_member,
                UseCaseDefBodyElement::FlowUsage,
            ),
            map(
                |i| {
                    let (i, _) = ws_and_comments(i)?;
                    // Don't steal declaration keywords as FeatureRef expressions.
                    if is_use_case_statement_keyword(i.fragment()) {
                        return Err(nom::Err::Error(nom::error::Error::new(
                            i,
                            nom::error::ErrorKind::Tag,
                        )));
                    }
                    let (i, expr) = expression(i)?;
                    let (i, _) = opt(preceded(ws_and_comments, tag(&b";"[..]))).parse(i)?;
                    Ok((i, expr))
                },
                UseCaseDefBodyElement::Expression,
            ),
            other_use_case_body_element,
        )),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn is_use_case_statement_keyword(frag: &[u8]) -> bool {
    crate::parser::lex::starts_with_keyword(frag, b"in")
        || crate::parser::lex::starts_with_keyword(frag, b"out")
        || crate::parser::lex::starts_with_keyword(frag, b"inout")
        || crate::parser::lex::starts_with_keyword(frag, b"return")
        || crate::parser::lex::starts_with_keyword(frag, b"calc")
        || crate::parser::lex::starts_with_keyword(frag, b"requirement")
        || crate::parser::lex::starts_with_keyword(frag, b"attribute")
        || crate::parser::lex::starts_with_keyword(frag, b"part")
        || crate::parser::lex::starts_with_keyword(frag, b"private")
        || crate::parser::lex::starts_with_keyword(frag, b"protected")
        || crate::parser::lex::starts_with_keyword(frag, b"public")
        || crate::parser::lex::starts_with_keyword(frag, b"action")
        || crate::parser::lex::starts_with_keyword(frag, b"analysis")
        || crate::parser::lex::starts_with_keyword(frag, b"for")
        || crate::parser::lex::starts_with_keyword(frag, b"assign")
        || crate::parser::lex::starts_with_keyword(frag, b"perform")
        || crate::parser::lex::starts_with_keyword(frag, b"subject")
        || crate::parser::lex::starts_with_keyword(frag, b"actor")
        || crate::parser::lex::starts_with_keyword(frag, b"objective")
        || crate::parser::lex::starts_with_keyword(frag, b"flow")
}

fn directed_calc_usage(input: Input<'_>) -> IResult<Input<'_>, Node<CalcUsage>> {
    let (input, direction) = crate::parser::attribute::direction_prefix(input)?;
    let (input, mut usage) = crate::parser::constraint::calc_usage(input)?;
    usage.value.direction = Some(direction);
    Ok((input, usage))
}

fn directed_requirement_usage(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::RequirementUsage>> {
    let (input, direction) = crate::parser::attribute::direction_prefix(input)?;
    let (input, mut usage) = crate::parser::requirement::requirement_usage(input)?;
    usage.value.direction = Some(direction);
    Ok((input, usage))
}

pub(crate) fn actor_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ActorUsage>> {
    crate::parser::span::reference_transaction(input, actor_usage_inner)
}

fn actor_usage_inner(input: Input<'_>) -> IResult<Input<'_>, Node<ActorUsage>> {
    let start = input;
    let (input, (visibility_span, visibility)) =
        preceded(ws_and_comments, visibility_prefix).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"actor"[..])).parse(input)?;
    // SysML allows anonymous actors: `actor : User;` (Identification may be empty).
    let (after_gap, _) = ws_and_comments(input)?;
    let (input, n) = if after_gap.fragment().starts_with(b":")
        && !after_gap.fragment().starts_with(b":>")
        && !after_gap.fragment().starts_with(b":>>")
    {
        (after_gap, String::new())
    } else {
        let (input, _) = ws1(input)?;
        let (input, n) = name(input)?;
        (input, n)
    };
    let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
    let (input, type_name) = preceded(ws_and_comments, qualified_reference).parse(input)?;
    let (input, multiplicity) = opt(multiplicity_node).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ActorUsage {
                name: n,
                type_name,
                multiplicity,
                membership: Membership::actor(visibility, visibility_span),
            },
        ),
    ))
}

pub(crate) fn objective(input: Input<'_>) -> IResult<Input<'_>, Node<Objective>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, visibility) = opt(alt((
        map(preceded(tag(&b"private"[..]), ws1), |_| Visibility::Private),
        map(preceded(tag(&b"protected"[..]), ws1), |_| {
            Visibility::Protected
        }),
        map(preceded(tag(&b"public"[..]), ws1), |_| Visibility::Public),
    )))
    .parse(input)?;
    let (input, _) = tag(&b"objective"[..]).parse(input)?;
    let (input, requirement) = parse_requirement_usage_payload(input, Some("objective"))?;
    let requirement = node_from_to(start, input, requirement);
    Ok((
        input,
        node_from_to(
            start,
            input,
            Objective {
                visibility,
                requirement,
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

    // --- parser work item 4b (continuation): Membership on UseCaseDef/UseCaseUsage ---

    #[test]
    fn use_case_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = use_case_def(input("private use case def U1;")).expect("use case def");
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
    fn use_case_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = use_case_def(input("use case def U1;")).expect("use case def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn use_case_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) = use_case_usage(input("public use case u1 : U1;")).expect("use case usage");
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
    fn use_case_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = use_case_usage(input("use case u1 : U1;")).expect("use case usage");
        assert_eq!(node.value.membership.visibility, None);
    }

    // --- parser work item 4b (final sweep): ActorMembership on ActorUsage, confirmed against
    // the BNF's `ActorMember : ActorMembership = MemberPrefix ownedRelatedElement += ActorUsage`.

    #[test]
    fn actor_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) = actor_usage(input("private actor a1 : A1;")).expect("actor usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Private)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::ActorMembership
        );
    }

    #[test]
    fn actor_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = actor_usage(input("actor a1 : A1;")).expect("actor usage");
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn actor_usage_accepts_multiplicity() {
        let (rest, node) = actor_usage(input("actor passengers : Person[0..4];"))
            .expect("actor with multiplicity");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "passengers");
        let _type_reference = node.value.type_name;
        assert!(node.value.multiplicity.is_some());
    }

    #[test]
    fn failed_return_ref_rolls_back_nested_expression_references() {
        let context = crate::parser::span::ParseContext::new();
        let parsed = return_ref(context.input(b"return ref result { return Ghost::value;"));
        assert!(parsed.is_err());
        assert!(context.finish().is_empty());
    }
}
