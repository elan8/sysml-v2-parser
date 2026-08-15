//! Action definition and action usage parsing (function-based behavior).

use crate::ast::{
    ActionDef, ActionDefBody, ActionDefBodyElement, ActionUsage, ActionUsageBody,
    ActionUsageBodyElement, AssignStmt, DecisionStmt, FirstMergeBody, FirstMergeBodyElement,
    FirstMergeBraceBody, FirstStmt, ForLoop, ForkStmt, IfStmt, InOut, InOutDecl, JoinStmt,
    LoopStmt, MergeStmt, Multiplicity, Node, ParseErrorNode, TerminateStmt, ThenAction, ThenTarget,
    WhileStmt,
};
use crate::parser::body::parse_structured_brace_members;
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::expr::{expression, path_expression};
use crate::parser::feature_value::feature_value_part;
use crate::parser::lex::{
    name, qualified_reference, starts_with_any_keyword, starts_with_keyword, take_until_terminator,
    ws1, ws_and_comments,
};
use crate::parser::metadata_annotation::{annotation, metadata_annotation};
use crate::parser::node_from_to;
use crate::parser::part::bind_;
use crate::parser::usage::{multiplicity_node, redefinition, usage_feature_modifier_flags};
use crate::parser::with_span;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

const ACTION_BODY_STARTERS: &[&[u8]] = &[
    b"in",
    b"out",
    b"ref",
    b"perform",
    b"bind",
    b"flow",
    b"first",
    b"merge",
    b"state",
    b"assign",
    b"then",
    b"for",
    b"succession",
    b"action",
    b"attribute",
    b"calc",
    b"event",
    b"part",
    b"item",
    b"assert",
    b"variation",
    b"snapshot",
    b"accept",
    b"decide",
    b"fork",
    b"join",
    b"send",
    b"terminate",
    b"while",
    b"loop",
    b"if",
    b"@",
    b"#",
];

const CONTROL_NODE_KEYWORDS: &[&[u8]] = &[b"accept", b"send", b"terminate", b"while", b"if"];

const UNTIL_SEMI_OR_BRACE: &[u8] = b";{";

fn doc_comment_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<crate::ast::DocComment>> {
    let (input, doc) = crate::parser::requirement::doc_comment(input)?;
    let (input, _) = opt(preceded(ws_and_comments, tag(&b";"[..]))).parse(input)?;
    Ok((input, doc))
}

/// Ref declaration inside an action body.
///
/// The Systems Library uses `ref name :>> redefinesTarget: Type1, Type2 { ... }` in action
/// definitions (e.g. `Actions.sysml`'s `SendAction`/`AcceptMessageAction`: `ref sentMessage :>>
/// sentTransfer: MessageTransfer, MessageAction { ... }`) -- an optional `:>>` redefines clause
/// optionally followed by a `:` typing clause, which may itself name more than one
/// comma-separated target. Previously, seeing `:>>` swallowed everything up to the body/
/// terminator as unparsed text, silently discarding both the redefines target and the entire
/// typing clause (S42-004). We also accept `= expr` bindings; anything still unrecognized before
/// the terminator is skipped as before.
fn action_ref_decl(input: Input<'_>) -> IResult<Input<'_>, Node<crate::ast::RefDecl>> {
    crate::parser::span::reference_transaction(input, action_ref_decl_inner)
}

fn action_ref_decl_inner(input: Input<'_>) -> IResult<Input<'_>, Node<crate::ast::RefDecl>> {
    use crate::parser::expr::expression;
    use crate::parser::usage::{
        optional_typings, single_target_redefines, single_target_typing,
        typing_reference_fields_from_result,
    };

    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = opt(alt((
        preceded(tag(&b"public"[..]), ws1),
        preceded(tag(&b"private"[..]), ws1),
        preceded(tag(&b"protected"[..]), ws1),
    )))
    .parse(input)?;
    // `BasicUsagePrefix = RefPrefix ('ref')?`, e.g. `abstract ref :>> trailerHitch[1];` (OMG spec
    // Annex `3c-Function-based Behavior-structure mod-2.sysml`). The modifiers used to be accepted
    // and discarded, because `RefDecl` had nowhere to put them.
    let (input, prefix) = crate::parser::usage::ref_prefix(input)?;
    let (input, _) = tag(&b"ref"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    // The kind keyword is retained: `RefDecl::kind_keyword` models it, and dropping it made
    // `derived ref action deferred : ActionUsage;` format back as `derived ref deferred : ...`.
    let (input, kind_keyword) = opt(preceded(tag(&b"action"[..]), ws1))
        .parse(input)
        .map(|(input, kw)| (input, kw.map(|_| crate::ast::RefDeclKind::Action)))?;
    // `ref :>> name ...` (redefinition) may omit the name before `:>>`.
    let (input, parsed_name) = opt(with_span(name)).parse(input)?;
    let (input, multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    let (name_span, name_str) = parsed_name.unwrap_or((crate::ast::Span::dummy(), String::new()));

    // Optional `:>>` redefines clause: `ref NAME :>> TARGET`.
    let (input, redefines_target) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>>"[..])),
        preceded(ws_and_comments, with_span(qualified_reference)),
    ))
    .parse(input)?;
    let redefines = redefines_target.map(|(span, target)| single_target_redefines(span, target));

    let (input, type_ref_span, typing) = if redefines.is_some() {
        // After `:>> target`, an optional `:` typing clause (possibly multi-target) may follow.
        let (input, typing_result) = optional_typings(input)?;
        let (type_ref_span, _, typing) = typing_reference_fields_from_result(typing_result);
        (input, type_ref_span, typing)
    } else {
        // No `:>>` redefines clause seen: bare `:` (multi-target aware) or legacy `:>`
        // (single-target only; kept for backward compatibility -- no confirmed real usage of
        // this spelling in action-def bodies, unlike `:>>`/`:` above).
        let (peek, _) = ws_and_comments(input)?;
        if peek.fragment().starts_with(b":>") && !peek.fragment().starts_with(b":>>") {
            let (input, _) = preceded(ws_and_comments, tag(&b":>"[..])).parse(input)?;
            let (input, (span, target)) =
                preceded(ws_and_comments, with_span(qualified_reference)).parse(input)?;
            let typing = Some(single_target_typing(span.clone(), target));
            (input, Some(span), typing)
        } else {
            let (input, typing_result) = optional_typings(input)?;
            let (type_ref_span, _, typing) = typing_reference_fields_from_result(typing_result);
            (input, type_ref_span, typing)
        }
    };

    // `:>` subsets, independent of the `:>>` redefinition above: `derived ref action deferred :
    // ActionUsage :> Metadata::metadataItems;`. Without this the clause reached the
    // skip-to-terminator below and was swallowed whole.
    let (input, subsets) = opt(preceded(ws_and_comments, crate::parser::usage::subsetting))
        .parse(input)
        .map(|(input, clause)| (input, clause.map(|(relationship, _value)| relationship)))?;
    let (input, _) = ws_and_comments(input)?;
    let (mut input, value) = opt(preceded(
        preceded(ws_and_comments, tag(&b"="[..])),
        preceded(ws_and_comments, expression),
    ))
    .parse(input)?;
    let value = value.map(crate::parser::feature_value::wrap_bind_expression);

    // Accept and skip any remaining unmodeled shorthand before the body/terminator.
    if !input.fragment().is_empty()
        && !input.fragment().starts_with(b";")
        && !input.fragment().starts_with(b"{")
    {
        let (next, _) = take_until_terminator(input, UNTIL_SEMI_OR_BRACE)?;
        input = next;
    }

    let (input, body) = crate::parser::part::ref_body(input)?;

    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::RefDecl {
                is_derived: prefix.is_derived,
                usage_prefix: prefix.usage_prefix,
                is_constant: prefix.is_constant,
                direction: prefix.direction,
                kind_keyword,
                name: name_str,
                typing,
                redefines,
                subsets,
                multiplicity,
                ordered: false,
                nonunique: false,
                value,
                body,
                name_span: Some(name_span),
                type_ref_span,
                membership: crate::ast::Membership::feature(None, crate::ast::Span::dummy()),
            },
        ),
    ))
}

/// First/merge body: `;` or `{` ... `}`
fn first_merge_body(input: Input<'_>) -> IResult<Input<'_>, FirstMergeBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b";"[..]), |_| FirstMergeBody::Semicolon),
        first_merge_brace_body,
    ))
    .parse(input)
}

fn first_merge_brace_body(input: Input<'_>) -> IResult<Input<'_>, FirstMergeBody> {
    let start = input;
    let (input, members) = parse_structured_brace_members(
        input,
        ACTION_BODY_STARTERS,
        "first/merge body",
        "recovered_first_merge_body_element",
        first_merge_body_element,
        |start, end| {
            let recovery = build_recovery_error_node_from_span(
                start,
                end,
                ACTION_BODY_STARTERS,
                "first/merge body",
                "recovered_first_merge_body_element",
            );
            let node = node_from_to(start, end, recovery);
            node_from_to(start, end, FirstMergeBodyElement::Error(node))
        },
    )?;
    Ok((
        input,
        FirstMergeBody::Brace(node_from_to(
            start,
            input,
            FirstMergeBraceBody {
                open_brace_span: members.open_span,
                elements: members.elements,
                close_brace_span: members.close_span,
            },
        )),
    ))
}

fn first_merge_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<FirstMergeBodyElement>> {
    let (input, member) = action_def_body_element(input)?;
    let span = member.span.clone();
    let value = match member.value {
        // BNF `MergeNode`/`DecisionNode`/`JoinNode`/`ForkNode` (SysML-textual-bnf.kebnf §8.2.2.17.3)
        // all use the same `ActionBody` production as a plain action definition body, so a
        // `first`/`merge` brace body legitimately accepts any action-body declaration (e.g. `calc
        // opaque;`), not just the narrower allow-list this used to downgrade to `Unsupported`.
        value @ (ActionDefBodyElement::AttributeUsage(_)
        | ActionDefBodyElement::CalcUsage(_)
        | ActionDefBodyElement::ActionDef(_)
        | ActionDefBodyElement::Error(_)
        | ActionDefBodyElement::InOutDecl(_)
        | ActionDefBodyElement::Doc(_)
        | ActionDefBodyElement::Annotation(_)
        | ActionDefBodyElement::MetadataAnnotation(_)
        | ActionDefBodyElement::MetadataKeywordUsage(_)
        | ActionDefBodyElement::MetadataUsage(_)
        | ActionDefBodyElement::TextualRep(_)
        | ActionDefBodyElement::RefDecl(_)
        | ActionDefBodyElement::Perform(_)
        | ActionDefBodyElement::Bind(_)
        | ActionDefBodyElement::FlowUsage(_)
        | ActionDefBodyElement::FirstStmt(_)
        | ActionDefBodyElement::MergeStmt(_)
        | ActionDefBodyElement::DecisionStmt(_)
        | ActionDefBodyElement::JoinStmt(_)
        | ActionDefBodyElement::ForkStmt(_)
        | ActionDefBodyElement::TerminateStmt(_)
        | ActionDefBodyElement::WhileStmt(_)
        | ActionDefBodyElement::LoopStmt(_)
        | ActionDefBodyElement::IfStmt(_)
        | ActionDefBodyElement::StateUsage(_)
        | ActionDefBodyElement::ActionUsage(_)
        | ActionDefBodyElement::PartUsage(_)
        | ActionDefBodyElement::ItemUsage(_)
        | ActionDefBodyElement::AssertConstraint(_)
        | ActionDefBodyElement::OccurrenceUsage(_)
        | ActionDefBodyElement::Assign(_)
        | ActionDefBodyElement::ForLoop(_)
        | ActionDefBodyElement::ThenAction(_)
        | ActionDefBodyElement::DefaultReferenceUsage(_)) => {
            FirstMergeBodyElement::Member(Box::new(Node::new(span.clone(), value)))
        }
    };
    Ok((input, Node::new(span, value)))
}

/// In/out decl: `in` name `:` type `;` or `out` name `:` type `;`
pub(crate) fn in_out_decl(input: Input<'_>) -> IResult<Input<'_>, Node<InOutDecl>> {
    crate::parser::span::reference_transaction(input, in_out_decl_inner)
}

fn in_out_decl_inner(input: Input<'_>) -> IResult<Input<'_>, Node<InOutDecl>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, direction) = alt((
        map(preceded(tag(&b"in"[..]), ws1), |_| InOut::In),
        map(preceded(tag(&b"out"[..]), ws1), |_| InOut::Out),
        map(preceded(tag(&b"inout"[..]), ws1), |_| InOut::InOut),
    ))
    .parse(input)?;
    // `in item …` / `in part …` are StructureUsageMembers and `in occurrence …` is an
    // OccurrenceUsageMember, not plain InOutDecl parameters. Fail here (before the unstructured
    // fallback) so action-body dispatch can try those arms.
    let (peek, _) = ws_and_comments(input)?;
    if starts_with_keyword(peek.fragment(), b"item")
        || starts_with_keyword(peek.fragment(), b"part")
        || starts_with_keyword(peek.fragment(), b"occurrence")
        || starts_with_keyword(peek.fragment(), b"expr")
        || starts_with_keyword(peek.fragment(), b"bool")
        || starts_with_keyword(peek.fragment(), b"feature")
        || starts_with_keyword(peek.fragment(), b"calc")
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, _) = nom::combinator::opt(preceded(tag(&b"attribute"[..]), ws1)).parse(input)?;
    // `in :>> name = expr;` / `out :>> name;` (validation `08` require bodies). Also covers the
    // multiplicity form (`in :>> payload [0..*];`) and the comma-separated multi-target form
    // (`in :>> MessageTransfer::payload, MessageAction::payload;`), both from Systems Library
    // `Actions.sysml`'s `SendAction`/`TransitionAction`.
    let (peek_redef, _) = ws_and_comments(input)?;
    if peek_redef.fragment().starts_with(b":>>")
        || starts_with_keyword(peek_redef.fragment(), b"redefines")
    {
        let (input, redefines) = redefinition(input)?;
        // Optional trailing `: Type` between the redefinition target and the value, e.g.
        // `out attribute :>> a_out : AccelerationValue = Acceleration(dt, tm, tp);`
        // (Systems Library-adjacent Analysis Examples/Dynamics.sysml:64, GH-86).
        let (input, type_name) = opt(preceded(
            preceded(ws_and_comments, tag(&b":"[..])),
            preceded(ws_and_comments, qualified_reference),
        ))
        .parse(input)?;
        let (input, multiplicity) =
            opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
        let (input, (ordered, nonunique)) = usage_feature_modifier_flags(input)?;
        let (input, value) = opt(feature_value_part).parse(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
        return Ok((
            input,
            node_from_to(
                start,
                input,
                InOutDecl {
                    direction,
                    is_reference: false,
                    is_var: false,
                    name: String::new(),
                    subsets: None,
                    type_name,
                    multiplicity,
                    ordered,
                    nonunique,
                    redefines: Some(redefines),
                    value,
                    body: None,
                },
            ),
        ));
    }
    let parsed = (|| {
        // Library shorthand: `in action body { ... }` (treat as name `body` typed as `action`)
        let (input, action_typed_name) = opt(preceded(tag(&b"action"[..]), ws1)).parse(input)?;
        let (input, is_reference) = opt(preceded(tag(&b"ref"[..]), ws1)).parse(input)?;
        // `out var y1;` (KerML `var` time-varying prefix on a directed parameter).
        let (input, is_var) = opt(preceded(tag(&b"var"[..]), ws1)).parse(input)?;
        // Anonymous typed parameter: `in : TensorQuantityValue[1];` (Domain Libraries
        // `TensorCalculations.sysml`). The name is legally omitted when the typing follows
        // directly.
        let (peek_anon, _) = ws_and_comments(input)?;
        let (input, param_name) =
            if peek_anon.fragment().starts_with(b":") && !peek_anon.fragment().starts_with(b":>") {
                (input, String::new())
            } else {
                name(input)?
            };
        // BNF `FeatureSpecializationPart` allows the `MultiplicityPart` before or after the
        // typing: `in transitionLinkSource[1]: StateAction :>> ...` (Systems Library
        // `States.sysml`) vs `inout replacementValues : Anything[0..*] nonunique;`
        // (`Actions.sysml`). Accept one multiplicity clause in either position.
        let (input, leading_multiplicity) =
            opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
        let (input, (leading_ordered, leading_nonunique)) = usage_feature_modifier_flags(input)?;
        // In action usages, pin declarations may omit the type (e.g. `out videoStream;`)
        // to reference the corresponding typed parameter on the referenced action definition.
        // Action definitions generally include the type (e.g. `out videoStream : String;`),
        // but accepting the shorthand here prevents recovery errors in common models.
        // `:>` is a subsets clause, retained as such (`out voltage :> ISQ::electricPotential =
        // ...;`, spec42 evsample); it was previously folded into `type_name`, erasing the
        // authored relationship kind. Parsed inline (not via `usage::subsetting`) so the `=`
        // value stays on the declaration's own `FeatureValue` clause below.
        let subsets_start = input;
        let subsets_attempt =
            (|| -> IResult<Input<'_>, Node<crate::ast::SubsettingRelationship>> {
                let (i, _) = preceded(ws_and_comments, crate::parser::lex::subset_operator)
                    .parse(subsets_start)?;
                let (i, targets) = preceded(
                    ws_and_comments,
                    crate::parser::usage::specialization_targets,
                )
                .parse(i)?;
                let span = crate::parser::span_from_to(subsets_start, i);
                Ok((
                    i,
                    crate::parser::usage::subsetting_relationship_node(
                        targets,
                        crate::ast::SubsettingKind::Subsets,
                        span,
                    ),
                ))
            })();
        let (input, subsets) = match subsets_attempt {
            Ok((i, node)) => (i, Some(node)),
            Err(_) => (input, None),
        };
        let (input, type_name) = nom::combinator::opt(map(
            (
                preceded(ws_and_comments, tag(&b":"[..])),
                preceded(ws_and_comments, qualified_reference),
            ),
            |(_, tn)| tn,
        ))
        .parse(input)?;
        let (input, trailing_multiplicity) = if leading_multiplicity.is_none() {
            opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?
        } else {
            (input, None)
        };
        let (input, (trailing_ordered, trailing_nonunique)) = usage_feature_modifier_flags(input)?;
        // Trailing `:>>` redefinition after a named declaration, including the comma-separated
        // multi-target form: `in transitionLinkSource[1]: StateAction :>>
        // TransitionAction::transitionLinkSource, StateTransitionPerformance::
        // transitionLinkSource;` (Systems Library `States.sysml`).
        let (input, redefines) = opt(preceded(ws_and_comments, redefinition)).parse(input)?;
        // The typing may trail the redefinition target: `in enclosingItem :>> 'frame' :
        // SpatialItem[1];` (Domain Libraries `SpatialItems.sysml`).
        let (input, type_name) = if type_name.is_none() && redefines.is_some() {
            opt(map(
                (
                    preceded(ws_and_comments, tag(&b":"[..])),
                    preceded(ws_and_comments, qualified_reference),
                ),
                |(_, tn)| tn,
            ))
            .parse(input)?
        } else {
            (input, type_name)
        };
        let (input, trailing_multiplicity) =
            if trailing_multiplicity.is_none() && leading_multiplicity.is_none() {
                opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?
            } else {
                (input, trailing_multiplicity)
            };
        let _ = action_typed_name;

        // Optional value clause: `= expr` / `:= expr` / `default (=|:=)? expr`, e.g.
        // `in a : Real = 0.0;` or `in target : Occurrence[1] default that as Occurrence`
        // (Systems Library `Actions.sysml`). `feature_value_part` also covers the
        // expression-body initializer form `default {true}` as a typed
        // `Expression::BodyExpr`, previously consumed opaquely here and discarded.
        let (input, value) = opt(feature_value_part).parse(input)?;

        // Standard library sometimes uses braced pin bodies without a trailing semicolon.
        // Accept either `;` or a `{ ... }` body as a terminator, retaining the body elements.
        let (input, body) = preceded(
            ws_and_comments,
            alt((
                map(tag(&b";"[..]), |_| None),
                map(consume_action_structured_brace, |members| {
                    Some(members.elements)
                }),
            )),
        )
        .parse(input)?;
        Ok::<_, nom::Err<nom::error::Error<Input<'_>>>>((
            input,
            InOutDecl {
                direction,
                is_reference: is_reference.is_some(),
                is_var: is_var.is_some(),
                name: param_name,
                subsets,
                type_name,
                multiplicity: leading_multiplicity.or(trailing_multiplicity),
                ordered: leading_ordered || trailing_ordered,
                nonunique: leading_nonunique || trailing_nonunique,
                redefines,
                value,
                body,
            },
        ))
    })();
    // Malformed declarations must fall through to the enclosing body's explicit recovery node;
    // do not guess a declaration name from opaque text and silently turn it into valid syntax.
    let (input, decl) = parsed?;
    Ok((input, node_from_to(start, input, decl)))
}

/// Action def body: `;` or `{` ActionDefBodyElement* `}`
fn action_def_body(input: Input<'_>) -> IResult<Input<'_>, ActionDefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((crate::parser::body::semicolon_body, action_def_body_brace)).parse(input)
}

pub(crate) fn action_def_body_brace(input: Input<'_>) -> IResult<Input<'_>, ActionDefBody> {
    let (input, members) = parse_structured_brace_members(
        input,
        ACTION_BODY_STARTERS,
        "action body",
        "recovered_action_body_element",
        action_def_body_element,
        |start, end| {
            let recovery = build_recovery_error_node_from_span(
                start,
                end,
                ACTION_BODY_STARTERS,
                "action body",
                "recovered_action_body_element",
            );
            let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
            node_from_to(start, end, ActionDefBodyElement::Error(node))
        },
    )?;
    Ok((input, members.into_body()))
}

/// Parse a `{` action-body `}`, returning the parsed member elements. The enclosing span stays
/// available at the caller.
fn consume_action_structured_brace(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::parser::body::ParsedBraceMembers<ActionDefBodyElement>> {
    parse_structured_brace_members(
        input,
        ACTION_BODY_STARTERS,
        "action body",
        "recovered_action_body_element",
        action_def_body_element,
        |start, end| {
            let recovery = build_recovery_error_node_from_span(
                start,
                end,
                ACTION_BODY_STARTERS,
                "action body",
                "recovered_action_body_element",
            );
            let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
            node_from_to(start, end, ActionDefBodyElement::Error(node))
        },
    )
}

pub(crate) fn assign_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<AssignStmt>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, is_then) = opt(map(preceded(tag(&b"then"[..]), ws1), |_| true)).parse(input)?;
    let is_then = is_then.unwrap_or(false);
    let (input, _) = tag(&b"assign"[..]).parse(input)?;
    let (input, _) = ws1(input)?;

    // LHS: structured feature-chain target (SysML v2 `AssignmentTargetParameter`).
    let (input, lhs) = path_expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b":="[..])).parse(input)?;

    let (after_rhs, rhs) = preceded(ws_and_comments, expression).parse(input)?;
    let (after_semi, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(after_rhs)?;

    Ok((
        after_semi,
        node_from_to(start, after_semi, AssignStmt { is_then, lhs, rhs }),
    ))
}

pub(crate) fn for_loop(input: Input<'_>) -> IResult<Input<'_>, Node<ForLoop>> {
    crate::parser::span::reference_transaction(input, for_loop_inner)
}

fn for_loop_inner(input: Input<'_>) -> IResult<Input<'_>, Node<ForLoop>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"for"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, var) = name(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"in"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    // A successfully parsed loop always owns a typed expression. If the range is malformed, the
    // complete production fails so the containing editor body inserts an explicit recovery node;
    // `reference_transaction` also discards any references allocated before that failure.
    let (input, range) = expression(input)?;
    let (input, body) = preceded(ws_and_comments, action_def_body_brace).parse(input)?;
    Ok((
        input,
        node_from_to(start, input, ForLoop { var, range, body }),
    ))
}

/// The succession target itself, shared by `then <target>;` and the `else <target>;` shorthand
/// used as an unbraced `if`/`else` branch (GH-86, `if x > 1 then A2; else A3;`,
/// Simple Tests/DecisionTest.sysml:5-7). Keyword consumption is the caller's job.
fn then_or_else_target(input: Input<'_>) -> IResult<Input<'_>, ThenTarget> {
    // §6 G23: `then merge <name>;` and `then <name>;` are the succession shorthand that follows
    // `first <name>;`; only the inline-declaration form (`then action ...`) was accepted before.
    // `merge_stmt` must precede `action_usage`, which would otherwise take `merge` as a name.
    alt((
        map(merge_stmt, ThenTarget::Merge),
        // Bare `fork`/`accept`/`decide` control-node references (GH-86): all already fully
        // parse standalone (`fork_stmt`/`transition_accept`/`decision_stmt`), just weren't tried
        // as a `then` target.
        map(fork_stmt, ThenTarget::Fork),
        map(decision_stmt, ThenTarget::Decide),
        map(
            with_span(|i| {
                nom::sequence::terminated(
                    crate::parser::payload::transition_accept,
                    preceded(ws_and_comments, tag(&b";"[..])),
                )
                .parse(i)
            }),
            |(span, accept)| ThenTarget::Accept(Node::new(span, accept)),
        ),
        // `then send new S() to b;` (spec42 gap 30): the same control-node parse the
        // standalone `send ...;` statement uses, kept as its own variant so the send shape is
        // distinguishable from a bare feature reference.
        map(then_send_target, |a| ThenTarget::Send(Box::new(a))),
        // `perform action …` before bare `perform …`; both before `action_usage`.
        map(crate::parser::part::perform_action_decl, |perform| {
            ThenTarget::Perform(Box::new(perform))
        }),
        map(crate::parser::part::perform_usage, |perform| {
            ThenTarget::Perform(Box::new(perform))
        }),
        // `action_usage` already accepts visibility / abstract / ref / variation prefixes.
        map(action_usage, |a| ThenTarget::Action(Box::new(a))),
        map(
            nom::sequence::terminated(path_expression, preceded(ws_and_comments, tag(&b";"[..]))),
            ThenTarget::Feature,
        ),
    ))
    .parse(input)
}

/// `send ...` as a `then` succession target -- only the `send` control-node form, so the other
/// control keywords keep their own arms.
fn then_send_target(input: Input<'_>) -> IResult<Input<'_>, Node<ActionUsage>> {
    let (peek, _) = ws_and_comments(input)?;
    if !starts_with_keyword(peek.fragment(), b"send") {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    control_node_action_usage(input)
}

pub(crate) fn then_action(input: Input<'_>) -> IResult<Input<'_>, Node<ThenAction>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"then"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, target) = then_or_else_target(input)?;
    Ok((input, node_from_to(start, input, ThenAction { target })))
}

/// The unbraced `else <target>;` shorthand `if`-branch (GH-86 non-brace `if`/`decide` gap), for
/// use *after* the caller has already consumed the `else` keyword itself (unlike
/// [`then_action`]/[`if_stmt`]'s `then` branch, which consumes its own leading keyword since
/// nothing else does so for it). Reuses [`ThenAction`]/[`ThenTarget`] for the target: which
/// keyword introduced it is already captured positionally by `IfStmt.then_body`/`else_body`, so
/// no separate AST type is needed -- `if_stmt` wraps the result as a synthetic one-element
/// `{ then <target>; }` body, identical to what the braced spelling of the same statement already
/// produces, so it re-emits/reparses to the same structure either way.
fn else_target_shorthand(input: Input<'_>) -> IResult<Input<'_>, Node<ThenAction>> {
    let start = input;
    let (input, target) = then_or_else_target(input)?;
    Ok((input, node_from_to(start, input, ThenAction { target })))
}

/// Element inside an action definition body.
///
/// SysML v2 ActionBodyItem includes both declarations and action behavior usages.
/// We support a pragmatic subset used by function-based behavior examples.
/// Control-node action usages (`accept`, `send`, …) map to `ActionUsage` nodes.
fn control_node_action_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ActionUsage>> {
    if let Ok(result) = crate::parser::payload::control_node_action_usage(input) {
        return Ok(result);
    }
    let (peek, _) = ws_and_comments(input)?;
    if starts_with_any_keyword(peek.fragment(), CONTROL_NODE_KEYWORDS) {
        return visibility_action_usage(input);
    }
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Alt,
    )))
}

fn action_def_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::ActionDefBodyElement>> {
    use crate::ast::ActionDefBodyElement;
    use crate::parser::part::perform_action_decl;
    use crate::parser::state::state_usage;

    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, elem) = nom::branch::alt((
        map(assign_stmt, ActionDefBodyElement::Assign),
        map(for_loop, ActionDefBodyElement::ForLoop),
        map(then_action, ActionDefBodyElement::ThenAction),
        // Gap 33: `attribute`/`calc`/`event` declarations dispatch through their typed
        // productions (the opaque `ActionBodyDecl` fallback is retired). Before `in_out_decl`
        // so these keywords never read as parameter names; `occurrence_usage` covers the
        // `event`/`event occurrence` forms.
        nom::branch::alt((
            map(crate::parser::attribute::attribute_usage, |a| {
                ActionDefBodyElement::AttributeUsage(Box::new(a))
            }),
            map(crate::parser::constraint::calc_usage, |c| {
                ActionDefBodyElement::CalcUsage(Box::new(c))
            }),
            map(crate::parser::occurrence_body::occurrence_usage, |n| {
                ActionDefBodyElement::OccurrenceUsage(Box::new(n))
            }),
        )),
        map(in_out_decl, ActionDefBodyElement::InOutDecl),
        map(doc_comment_stmt, ActionDefBodyElement::Doc),
        map(
            metadata_annotation,
            ActionDefBodyElement::MetadataAnnotation,
        ),
        map(
            crate::parser::metadata_annotation::metadata_keyword_usage,
            ActionDefBodyElement::MetadataKeywordUsage,
        ),
        map(annotation, ActionDefBodyElement::Annotation),
        map(action_ref_decl, ActionDefBodyElement::RefDecl),
        map(perform_action_decl, ActionDefBodyElement::Perform),
        map(bind_, ActionDefBodyElement::Bind),
        map(
            crate::parser::flow::flow_usage_member,
            ActionDefBodyElement::FlowUsage,
        ),
        map(first_stmt, ActionDefBodyElement::FirstStmt),
        map(merge_stmt, ActionDefBodyElement::MergeStmt),
        map(decision_stmt, ActionDefBodyElement::DecisionStmt),
        map(join_stmt, ActionDefBodyElement::JoinStmt),
        map(fork_stmt, ActionDefBodyElement::ForkStmt),
        map(state_usage, ActionDefBodyElement::StateUsage),
        // nom's alt() caps out at 21 branches; nest the newer control nodes plus the
        // remaining fallbacks in a sub-alt() to stay under that limit.
        nom::branch::alt((
            map(terminate_stmt, ActionDefBodyElement::TerminateStmt),
            map(while_stmt, ActionDefBodyElement::WhileStmt),
            map(loop_stmt, ActionDefBodyElement::LoopStmt),
            map(if_stmt, ActionDefBodyElement::IfStmt),
            // Literal `metadata` keyword form of `MetadataUsage` (BNF `('@' | 'metadata')`,
            // GH-86), e.g. `metadata ToolExecution { ... }`. Previously only dispatched at
            // package-body scope even though `crate::parser::metadata::metadata_usage` already
            // implements it fully (including rejecting `metadata def ...`).
            map(
                crate::parser::metadata::metadata_usage,
                ActionDefBodyElement::MetadataUsage,
            ),
            // KerML `TextualRepresentation` (GH-86), e.g. `language "alf" /* c.x = newX; */`.
            // Previously not reachable from any action body.
            map(
                crate::parser::requirement::textual_representation,
                ActionDefBodyElement::TextualRep,
            ),
            // Nested `action def` must win over `action_usage` (which would otherwise treat
            // `def` as a usage name).
            map(action_def, |d| ActionDefBodyElement::ActionDef(Box::new(d))),
            map(control_node_action_usage, |a| {
                ActionDefBodyElement::ActionUsage(Box::new(a))
            }),
            map(visibility_action_usage, |a| {
                ActionDefBodyElement::ActionUsage(Box::new(a))
            }),
            // GH-13 / BNF `ActionBodyItem` → `NonBehaviorBodyItem` /
            // `StructureUsageMember` + `BehaviorUsageMember` (`AssertConstraintUsage`).
            // Directed `in item`/`in part` reach here after `in_out_decl` rejects those keywords.
            nom::branch::alt((
                map(crate::parser::part::part_usage, |p| {
                    ActionDefBodyElement::PartUsage(Box::new(p))
                }),
                map(
                    crate::parser::item::directed_item_usage,
                    ActionDefBodyElement::ItemUsage,
                ),
                map(
                    crate::parser::item::item_usage,
                    ActionDefBodyElement::ItemUsage,
                ),
                map(
                    crate::parser::occurrence_body::assert_constraint_member,
                    ActionDefBodyElement::AssertConstraint,
                ),
                // `in occurrence …` reaches here after `in_out_decl` rejects the keyword,
                // same as directed `in item`/`in part` above.
                map(
                    crate::parser::occurrence_body::directed_occurrence_usage,
                    |n| ActionDefBodyElement::OccurrenceUsage(Box::new(n)),
                ),
                map(crate::parser::occurrence_body::snapshot_usage, |n| {
                    ActionDefBodyElement::OccurrenceUsage(Box::new(n))
                }),
            )),
            // §6 G26: last, so every keyword-led member above keeps priority over the
            // keyword-less `name = expr;` binding.
            map(
                crate::parser::attribute::feature_value_binding,
                ActionDefBodyElement::DefaultReferenceUsage,
            ),
        )),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

/// Action definition: `action` `def` Identification body
pub(crate) fn action_def(input: Input<'_>) -> IResult<Input<'_>, Node<ActionDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"action")
            .def_required()
            .individual_allowed()
            .with_captured_visibility(),
    )?;
    let (input, body) = action_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ActionDef {
                is_individual: prefix.is_individual,
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
                membership: crate::ast::Membership::owning(
                    prefix.visibility,
                    prefix.visibility_span,
                ),
            },
        ),
    ))
}

/// First stmt: `first` path `then` path body
/// Optional leading `succession` (name)? (`:` Type)? (`[mult]`)? prefix before `first`/`then`
/// (SysML v2 §8.2.2.13.3 `SuccessionAsUsage`, GH-38). Only called once the `succession` keyword
/// itself has been peeked; consumes it plus whatever of the optional trailing name/type/
/// multiplicity is present. Real usage: Systems Library `Flows.sysml`'s unnamed
/// `succession [seBeforeNum] first ...` (multiplicity, no name), `States.sysml`'s
/// `succession stateSequencing first ...` (name, no type), and
/// `sysml/src/examples/Simple Tests/ConnectionTest.sysml`'s `succession s first a then b;` /
/// `succession s1 : AB first a then b;` (name, and name + type).
/// `(name, type, multiplicity)` of a parsed `succession` prefix, all `None` when absent.
type SuccessionPrefix = (
    Option<String>,
    Option<crate::ast::QualifiedReferenceId>,
    Option<Node<Multiplicity>>,
);

fn succession_prefix(input: Input<'_>) -> IResult<Input<'_>, SuccessionPrefix> {
    let (input, _) = tag(&b"succession"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (peek, _) = ws_and_comments(input)?;
    let frag = peek.fragment();
    // GH-92.3: unnamed `succession : Type first a then b;` (a `:` type clause with no name at
    // all) -- previously only "no name, multiplicity/`first` follows directly" was recognized as
    // the name-less case; a leading `:` (not `:>`/`:>>`) fell through to the name parser below
    // and failed outright. Real usage: `Vehicle Example/VehicleIndividuals.sysml:49`.
    let (input, succession_name) = if starts_with_keyword(frag, b"first")
        || frag.starts_with(b"[")
        || (frag.starts_with(b":") && !frag.starts_with(b":>") && !frag.starts_with(b":>>"))
    {
        (input, None)
    } else {
        let (input, parsed_name) = preceded(ws_and_comments, name).parse(input)?;
        (input, Some(parsed_name))
    };
    let (peek, _) = ws_and_comments(input)?;
    let (input, succession_type) =
        if peek.fragment().starts_with(b":") && !peek.fragment().starts_with(b":>") {
            let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
            let (input, type_name) = preceded(ws_and_comments, qualified_reference).parse(input)?;
            (input, Some(type_name))
        } else {
            (input, None)
        };
    let (input, succession_multiplicity) =
        opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    Ok((
        input,
        (succession_name, succession_type, succession_multiplicity),
    ))
}

/// First stmt: (`succession` prefix)? `first` `[mult]`? path (`then` `[mult]`? path)? body
pub(crate) fn first_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<FirstStmt>> {
    crate::parser::span::reference_transaction(input, first_stmt_inner)
}

fn first_stmt_inner(input: Input<'_>) -> IResult<Input<'_>, Node<FirstStmt>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, succession) = opt(succession_prefix).parse(input)?;
    let (succession_name, succession_type, succession_multiplicity) =
        succession.unwrap_or((None, None, None));
    let (input, _) = preceded(ws_and_comments, tag(&b"first"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, first_multiplicity) =
        opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    let (input, first_expr) = preceded(ws_and_comments, path_expression).parse(input)?;
    // §6 G13: `then` is optional -- `first start;` on its own marks an initial node without
    // declaring a succession (OMG spec Annex `3a-Function-based Behavior-2.sysml`, where the
    // following `then merge continue;` / `then action ...;` lines supply the targets).
    let (input, then_parts) = opt(preceded(
        preceded(ws_and_comments, tag(&b"then"[..])),
        (
            opt(preceded(ws_and_comments, multiplicity_node)),
            preceded(ws_and_comments, path_expression),
        ),
    ))
    .parse(input)?;
    let (then_multiplicity, then_expr) = match then_parts {
        Some((mult, expr)) => (mult, Some(expr)),
        None => (None, None),
    };
    let (input, body) = first_merge_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            FirstStmt {
                succession_name,
                succession_type,
                succession_multiplicity,
                first: first_expr,
                first_multiplicity,
                then: then_expr,
                then_multiplicity,
                body,
            },
        ),
    ))
}

/// Merge stmt: `merge` path body
fn merge_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<MergeStmt>> {
    crate::parser::span::reference_transaction(input, merge_stmt_inner)
}

fn merge_stmt_inner(input: Input<'_>) -> IResult<Input<'_>, Node<MergeStmt>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"merge"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, merge_expr) = path_expression(input)?;
    let (input, body) = first_merge_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            MergeStmt {
                merge: merge_expr,
                body,
            },
        ),
    ))
}

/// Decision node: `decide` path body
fn decision_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<DecisionStmt>> {
    crate::parser::span::reference_transaction(input, decision_stmt_inner)
}

fn decision_stmt_inner(input: Input<'_>) -> IResult<Input<'_>, Node<DecisionStmt>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"decide"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, decide_expr) = path_expression(input)?;
    let (input, body) = first_merge_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            DecisionStmt {
                decide: decide_expr,
                body,
            },
        ),
    ))
}

/// Join node: `join` path body
fn join_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<JoinStmt>> {
    crate::parser::span::reference_transaction(input, join_stmt_inner)
}

fn join_stmt_inner(input: Input<'_>) -> IResult<Input<'_>, Node<JoinStmt>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"join"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, join_expr) = path_expression(input)?;
    let (input, body) = first_merge_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            JoinStmt {
                join: join_expr,
                body,
            },
        ),
    ))
}

/// Fork node: `fork` path body
fn fork_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<ForkStmt>> {
    crate::parser::span::reference_transaction(input, fork_stmt_inner)
}

fn fork_stmt_inner(input: Input<'_>) -> IResult<Input<'_>, Node<ForkStmt>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"fork"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, fork_expr) = path_expression(input)?;
    let (input, body) = first_merge_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ForkStmt {
                fork: fork_expr,
                body,
            },
        ),
    ))
}

/// Terminate control node: `terminate;` or `terminate target;`
fn terminate_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<TerminateStmt>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"terminate"[..]).parse(input)?;
    let (input, target) = opt(preceded(ws1, path_expression)).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((input, node_from_to(start, input, TerminateStmt { target })))
}

/// While-loop control node: `while` condition `{` ... `}` (bare condition, no `decide`/`join`/`fork`-style parens).
fn while_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<WhileStmt>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"while"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, condition) = expression(input)?;
    let (input, body) = preceded(ws_and_comments, action_def_body_brace).parse(input)?;
    Ok((
        input,
        node_from_to(start, input, WhileStmt { condition, body }),
    ))
}

/// Loop control node: `loop` `{` body `}` (§6 G14) — `while_stmt` without a condition.
fn loop_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<LoopStmt>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"loop"[..]).parse(input)?;
    let (input, body) = preceded(ws_and_comments, action_def_body_brace).parse(input)?;
    Ok((input, node_from_to(start, input, LoopStmt { body })))
}

/// A single `then <target>;`/`else <target>;` shorthand statement (GH-86): a branch member with
/// no braces of its own.
fn then_action_branch(node: Node<ThenAction>) -> crate::ast::ActionBranchBody {
    let span = node.span.clone();
    crate::ast::ActionBranchBody::Shorthand(Box::new(Node::new(
        span,
        ActionDefBodyElement::ThenAction(node),
    )))
}

/// A nested `else if ...` (BNF `IfNode`'s `IfNodeParameterMember` else-alternative, GH-86),
/// likewise written without braces.
fn if_stmt_branch(node: Node<IfStmt>) -> crate::ast::ActionBranchBody {
    let span = node.span.clone();
    crate::ast::ActionBranchBody::Shorthand(Box::new(Node::new(
        span,
        ActionDefBodyElement::IfStmt(node),
    )))
}

/// If control node: `if` condition (`{` thenBody `}` | `then` target `;`) (`else` (`{` elseBody
/// `}` | `if` ... | target `;`))?
///
/// The non-brace `then`/`else` shorthand (GH-86) is real SysML v2 usage, not a parser
/// convenience: `if x == 1 then A1;` and `if x > 1 then A2; else A3;` (Simple Tests/
/// DecisionTest.sysml:5-7) are guarded successions written without an enclosing action body.
/// `else if ...` chaining (also GH-86, Simple Tests/StructuredControlTest.sysml:7-13) is BNF
/// `IfNode`'s `('else' (ActionBodyParameterMember | IfNodeParameterMember))?` -- the else branch
/// can itself be a nested `IfNode`, not just a body.
fn if_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<IfStmt>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"if"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, condition) = expression(input)?;
    let (input, then_body) = preceded(
        ws_and_comments,
        alt((
            map(action_def_body_brace, crate::ast::ActionBranchBody::Braced),
            map(then_action, then_action_branch),
        )),
    )
    .parse(input)?;
    let (input, else_body) = opt(preceded(
        preceded(ws_and_comments, tag(&b"else"[..])),
        preceded(
            ws_and_comments,
            alt((
                map(action_def_body_brace, crate::ast::ActionBranchBody::Braced),
                map(if_stmt, if_stmt_branch),
                map(else_target_shorthand, then_action_branch),
            )),
        ),
    ))
    .parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            IfStmt {
                condition,
                then_body,
                else_body,
            },
        ),
    ))
}

/// Action usage body: `;`, `{` … `}`, or no body at all when the next token starts another
/// statement/succession (Systems Library `LoopAction` style without braces).
///
/// `None` is that third case. The grammar requires a terminator, so recording its absence keeps
/// the parser's leniency visible instead of fabricating a `;` nobody wrote.
pub(crate) fn action_usage_body(input: Input<'_>) -> IResult<Input<'_>, Option<ActionUsageBody>> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(crate::parser::body::semicolon_body, Some),
        map(action_usage_body_brace, Some),
        map(peek_implicit_action_usage_body_end, |_| None),
    ))
    .parse(input)
}

/// Succeeds without consuming input when the next token cannot continue this usage header/body
/// and instead begins a sibling action-body statement or closes the enclosing brace.
fn peek_implicit_action_usage_body_end(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (peek, _) = ws_and_comments(input)?;
    let frag = peek.fragment();
    if frag.starts_with(b"}")
        || starts_with_any_keyword(
            frag,
            &[
                b"assign",
                b"then",
                b"while",
                b"if",
                b"for",
                b"accept",
                b"send",
                b"merge",
                b"first",
                b"decide",
                b"join",
                b"fork",
                b"terminate",
                b"private",
                b"public",
                b"protected",
                b"action",
                b"perform",
                b"bind",
                b"flow",
                b"message",
                b"succession",
                b"state",
                b"part",
                b"item",
                b"assert",
                b"variation",
                b"snapshot",
                b"doc",
                b"@",
                b"#",
            ],
        )
    {
        Ok((input, ()))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Alt,
        )))
    }
}

fn action_usage_body_brace(input: Input<'_>) -> IResult<Input<'_>, ActionUsageBody> {
    let (input, members) = parse_structured_brace_members(
        input,
        ACTION_BODY_STARTERS,
        "action body",
        "recovered_action_body_element",
        action_usage_body_element,
        |start, end| {
            let recovery = build_recovery_error_node_from_span(
                start,
                end,
                ACTION_BODY_STARTERS,
                "action body",
                "recovered_action_body_element",
            );
            let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
            node_from_to(start, end, ActionUsageBodyElement::Error(node))
        },
    )?;
    Ok((input, members.into_body()))
}

/// Action usage body element: InOutDecl | Bind | Flow | FirstStmt | MergeStmt | ActionUsage
pub(crate) fn action_usage_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<ActionUsageBodyElement>> {
    use crate::parser::state::state_usage;

    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, elem) = alt((
        map(assign_stmt, ActionUsageBodyElement::Assign),
        map(for_loop, ActionUsageBodyElement::ForLoop),
        map(then_action, ActionUsageBodyElement::ThenAction),
        // Gap 33: typed dispatch replacing the retired opaque `ActionBodyDecl`; see the
        // matching branch in `action_def_body_element`.
        nom::branch::alt((
            map(crate::parser::attribute::attribute_usage, |a| {
                ActionUsageBodyElement::AttributeUsage(Box::new(a))
            }),
            map(crate::parser::constraint::calc_usage, |c| {
                ActionUsageBodyElement::CalcUsage(Box::new(c))
            }),
            map(crate::parser::occurrence_body::occurrence_usage, |n| {
                ActionUsageBodyElement::OccurrenceUsage(Box::new(n))
            }),
        )),
        map(in_out_decl, ActionUsageBodyElement::InOutDecl),
        map(doc_comment_stmt, ActionUsageBodyElement::Doc),
        map(
            metadata_annotation,
            ActionUsageBodyElement::MetadataAnnotation,
        ),
        map(
            crate::parser::metadata_annotation::metadata_keyword_usage,
            ActionUsageBodyElement::MetadataKeywordUsage,
        ),
        map(annotation, ActionUsageBodyElement::Annotation),
        map(action_ref_decl, ActionUsageBodyElement::RefDecl),
        map(bind_, ActionUsageBodyElement::Bind),
        map(
            crate::parser::flow::flow_usage_member,
            ActionUsageBodyElement::FlowUsage,
        ),
        map(first_stmt, ActionUsageBodyElement::FirstStmt),
        map(merge_stmt, ActionUsageBodyElement::MergeStmt),
        map(decision_stmt, ActionUsageBodyElement::DecisionStmt),
        map(join_stmt, ActionUsageBodyElement::JoinStmt),
        map(fork_stmt, ActionUsageBodyElement::ForkStmt),
        map(state_usage, ActionUsageBodyElement::StateUsage),
        // nom's alt() caps out at 21 branches; nest the newer control nodes plus the
        // remaining fallbacks in a sub-alt() to stay under that limit.
        nom::branch::alt((
            map(terminate_stmt, ActionUsageBodyElement::TerminateStmt),
            map(while_stmt, ActionUsageBodyElement::WhileStmt),
            map(loop_stmt, ActionUsageBodyElement::LoopStmt),
            map(if_stmt, ActionUsageBodyElement::IfStmt),
            // Literal `metadata` keyword form of `MetadataUsage` (BNF `('@' | 'metadata')`,
            // GH-86), e.g. `metadata ToolExecution { ... }`. Previously only dispatched at
            // package-body scope even though `crate::parser::metadata::metadata_usage` already
            // implements it fully (including rejecting `metadata def ...`).
            map(
                crate::parser::metadata::metadata_usage,
                ActionUsageBodyElement::MetadataUsage,
            ),
            // KerML `TextualRepresentation` (GH-86), e.g. `language "alf" /* c.x = newX; */`.
            // Previously not reachable from any action body.
            map(
                crate::parser::requirement::textual_representation,
                ActionUsageBodyElement::TextualRep,
            ),
            map(action_def, |d| {
                ActionUsageBodyElement::ActionDef(Box::new(d))
            }),
            map(control_node_action_usage, |a| {
                ActionUsageBodyElement::ActionUsage(Box::new(a))
            }),
            map(visibility_action_usage, |a| {
                ActionUsageBodyElement::ActionUsage(Box::new(a))
            }),
            // GH-13 / BNF `ActionBodyItem` → `NonBehaviorBodyItem` /
            // `StructureUsageMember` + `BehaviorUsageMember` (`AssertConstraintUsage`).
            // Directed `in item`/`in part` reach here after `in_out_decl` rejects those keywords.
            nom::branch::alt((
                map(crate::parser::part::part_usage, |p| {
                    ActionUsageBodyElement::PartUsage(Box::new(p))
                }),
                map(
                    crate::parser::item::directed_item_usage,
                    ActionUsageBodyElement::ItemUsage,
                ),
                map(
                    crate::parser::item::item_usage,
                    ActionUsageBodyElement::ItemUsage,
                ),
                map(
                    crate::parser::occurrence_body::assert_constraint_member,
                    ActionUsageBodyElement::AssertConstraint,
                ),
                // `in occurrence …` reaches here after `in_out_decl` rejects the keyword,
                // same as directed `in item`/`in part` above.
                map(
                    crate::parser::occurrence_body::directed_occurrence_usage,
                    |n| ActionUsageBodyElement::OccurrenceUsage(Box::new(n)),
                ),
                map(crate::parser::occurrence_body::snapshot_usage, |n| {
                    ActionUsageBodyElement::OccurrenceUsage(Box::new(n))
                }),
            )),
            // GH-89.7: `variant name;` referencing a sibling variation action's variant, e.g.
            // `variant generateTorque4Cyl;` (Variability Examples/VehicleVariabilityModel.sysml:128).
            // Before the keyword-less fallback below since `variant` is a real keyword.
            map(
                crate::parser::part::variant_usage,
                ActionUsageBodyElement::VariantUsage,
            ),
            // §6 G26: last, so every keyword-led member above keeps priority over the
            // keyword-less `name = expr;` binding.
            map(
                crate::parser::attribute::feature_value_binding,
                ActionUsageBodyElement::DefaultReferenceUsage,
            ),
        )),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn visibility_action_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ActionUsage>> {
    // `action_usage` already captures visibility / abstract / ref.
    action_usage(input)
}

/// True when `action` is followed by an `accept`/`send` *payload* clause rather than a usage named
/// `accept`/`send` (e.g. `action send typed by T;` keeps `send` as the name).
fn is_anonymous_accept_or_send_payload(fragment: &[u8]) -> bool {
    use crate::parser::diagnostics::trim_ascii_start;

    let keyword = if starts_with_keyword(fragment, b"accept") {
        &b"accept"[..]
    } else if starts_with_keyword(fragment, b"send") {
        &b"send"[..]
    } else {
        return false;
    };
    let after_kw = trim_ascii_start(&fragment[keyword.len()..]);
    // Usage-declaration continuations after a name — not a control-node payload.
    if after_kw.is_empty()
        || after_kw.starts_with(b";")
        || after_kw.starts_with(b"{")
        || after_kw.starts_with(b"[")
        || after_kw.starts_with(b":")
        || starts_with_keyword(after_kw, b"typed")
        || starts_with_keyword(after_kw, b"defined")
        || starts_with_keyword(after_kw, b"subsets")
        || starts_with_keyword(after_kw, b"redefines")
        || starts_with_keyword(after_kw, b"references")
        || starts_with_keyword(after_kw, b"crosses")
        || starts_with_keyword(after_kw, b"intersects")
        || starts_with_keyword(after_kw, b"ordered")
        || starts_with_keyword(after_kw, b"nonunique")
    {
        return false;
    }
    true
}

/// Action usage: `(visibility)? (abstract|variation)? (ref)? action` name header (`accept` …)? body
pub(crate) fn action_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ActionUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    // BNF `RefPrefix`: `( isAbstract ?= 'abstract' | isVariation ?= 'variation' )?`
    let (input, abstract_or_variation) = opt(alt((
        map(preceded(tag(&b"abstract"[..]), ws1), |_| true),
        map(preceded(tag(&b"variation"[..]), ws1), |_| false),
    )))
    .parse(input)?;
    let (is_abstract, is_variation) = match abstract_or_variation {
        Some(true) => (true, false),
        Some(false) => (false, true),
        None => (false, false),
    };
    let (input, is_reference) =
        nom::combinator::opt(preceded(tag(&b"ref"[..]), ws1)).parse(input)?;
    // BNF `OccurrenceUsagePrefix`: `(isIndividual ?= 'individual')?` after the basic usage prefix
    // (GH-90.1), e.g. `individual action a : AP1;` (Simple Tests/IndividualTest.sysml:30).
    let (input, is_individual) =
        nom::combinator::opt(preceded(tag(&b"individual"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"action"[..]).parse(input)?;
    // SysML allows anonymous action usages: `action: Runner;` (Identification may be empty).
    let (after_gap, _) = ws_and_comments(input)?;
    // `action def …` is a definition, not a usage named `def`.
    if starts_with_keyword(after_gap.fragment(), b"def") {
        return Err(nom::Err::Error(nom::error::Error::new(
            after_gap,
            nom::error::ErrorKind::Tag,
        )));
    }
    // §6 G19: a fully anonymous usage (`action { ... }` / `action;`) is legal too -- the OMG spec
    // Annex `3c-Function-based Behavior-structure mod-1.sysml` declares one directly in a part
    // usage body. Previously only the typed anonymous form (`action: Runner;`) was accepted, so
    // the bodied one fell through to opaque recovery.
    // Also anonymous when the next token opens an `accept`/`send` *payload* clause
    // (`then action accept engineOff : EngineOff;` in `3a-Function-based Behavior-3.sysml`).
    // But `action send typed by T;` names the usage `send` — only treat accept/send as payload
    // starters when the following token is not a usage-declaration continuation.
    // spec42 Gap 42: a leading `:>`/`:>>` specialization clause also stands in for the name
    // (`action :>> subactions :> middle { ... }`, Systems Library `States.sysml`) -- the
    // clauses themselves are picked up by `specialization_clauses` below, mirroring
    // `attribute_usage`'s `PrefixRedefines`/`PrefixSubsets` heads.
    let (input, (name_span, name_str)) = if after_gap.fragment().starts_with(b":")
        || after_gap.fragment().starts_with(b"{")
        || after_gap.fragment().starts_with(b";")
        || starts_with_keyword(after_gap.fragment(), b"defined")
        || is_anonymous_accept_or_send_payload(after_gap.fragment())
    {
        (after_gap, (crate::ast::Span::dummy(), String::new()))
    } else {
        let (input, _) = ws1(input)?;
        with_span(name).parse(input)?
    };
    // Feature-style header: typing, multiplicity, ordered/nonunique, subsets/redefines.
    // Plain `usage_header` drops `[0..*]` (Systems Library `performedActions`).
    let (input, leading) = crate::parser::usage::specialization_clauses(input)?;
    let (input, type_result) = crate::parser::usage::optional_typings(input)?;
    let (input, multiplicity) =
        nom::combinator::opt(crate::parser::usage::multiplicity_node).parse(input)?;
    let (input, _) = crate::parser::usage::skip_usage_feature_modifiers(input)?;
    let (input, trailing) = crate::parser::usage::specialization_clauses(input)?;
    let (type_ref_span, type_name, typing) =
        crate::parser::usage::typing_reference_fields_from_result(type_result);
    let subsets = trailing
        .subsets
        .clone()
        .or(leading.subsets.clone())
        .map(|(target, _)| target);
    let redefines = trailing.redefines.clone().or(leading.redefines.clone());
    let (input, accept) = nom::combinator::opt(preceded(
        preceded(ws_and_comments, tag(&b"accept"[..])),
        preceded(ws1, crate::parser::payload::typed_payload_clause),
    ))
    .parse(input)?;
    // `action <name> send ...` (GH-86, e.g. `action publish send new Publish(someTopic,
    // somePublication) via publicationPort;`, Interaction Sequencing Examples/
    // ServerSequenceOutsideRealization-2.sysml): BNF's `SendNodeDeclaration` fuses the naming
    // `ActionNodeUsageDeclaration` prefix and the `send` payload into one node, mirroring the
    // `accept` suffix above. Without this, `<name> send ...` fell through as an anonymous
    // empty-bodied `<name>` usage followed by a *separate* unnamed `send` sibling statement.
    // The payload itself is optional (BNF `EmptyParameterMember` alternative), e.g. `action
    // snd2 send via this to aa.target;` (Simple Tests/ActionTest.sysml) -- so `saw_send_keyword`
    // is tracked separately from whether a payload was actually captured, to gate the `to`
    // clause correctly even when the payload is absent.
    let (input, saw_send_keyword, send) = if accept.is_none() {
        let (input, send_kw) =
            nom::combinator::opt(preceded(ws_and_comments, tag(&b"send"[..]))).parse(input)?;
        if send_kw.is_some() {
            // Peek for the `via`/`to` keywords first: neither is reserved in `name`/`expression`,
            // so without this guard an empty payload (BNF `EmptyParameterMember`) would greedily
            // swallow `via`/`to` itself as a bare feature reference instead of leaving it for the
            // `via`/`to` clauses below (GH-86, `action snd2 send via this to aa.target;`).
            let (peek, _) = ws_and_comments(input)?;
            let payload_follows = !starts_with_any_keyword(peek.fragment(), &[b"via", b"to"]);
            let (input, send) = if payload_follows {
                nom::combinator::opt(preceded(
                    ws1,
                    nom::branch::alt((
                        nom::combinator::map(
                            crate::parser::payload::typed_payload_clause,
                            crate::ast::SendPayload::Typed,
                        ),
                        nom::combinator::map(expression, crate::ast::SendPayload::Expression),
                    )),
                ))
                .parse(input)?
            } else {
                (input, None)
            };
            (input, true, send)
        } else {
            (input, false, None)
        }
    } else {
        (input, false, None)
    };
    let type_ref_span = accept
        .as_ref()
        .and_then(|p| p.type_span.clone())
        .or(type_ref_span);
    let (input, via) = nom::combinator::opt(preceded(
        preceded(ws_and_comments, tag(&b"via"[..])),
        preceded(ws1, expression),
    ))
    .parse(input)?;
    let (input, to) = if saw_send_keyword {
        nom::combinator::opt(preceded(
            preceded(ws_and_comments, tag(&b"to"[..])),
            preceded(ws1, expression),
        ))
        .parse(input)?
    } else {
        (input, None)
    };
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = action_usage_body(input)?;
    // Spec-wise, a braced body does not require a trailing semicolon. However, in practice some
    // sources write `... { ... };` as a statement terminator. We accept an optional `;` here to
    // avoid cascading recovery errors in action bodies.
    let (input, _) =
        nom::combinator::opt(preceded(ws_and_comments, tag(&b";"[..]))).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ActionUsage {
                is_abstract,
                is_variation,
                is_reference: is_reference.is_some(),
                is_individual: is_individual.is_some(),
                name: name_str,
                type_name,
                typing,
                multiplicity,
                subsets,
                redefines,
                accept,
                send,
                via,
                to,
                body,
                name_span: Some(name_span),
                type_ref_span,
                membership: crate::ast::Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

#[cfg(test)]
mod in_out_decl_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    /// Systems Library `Actions.sysml`: `in transitionLinkSource : Action :>>
    /// TransitionPerformance::transitionLinkSource;` -- a `:>>` redefinition trailing a named,
    /// typed declaration.
    #[test]
    fn in_out_decl_accepts_trailing_redefinition_after_typing() {
        let (rest, node) = in_out_decl(input("in transitionLinkSource : Action :>> T::t;"))
            .expect("trailing redefinition");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "transitionLinkSource");
        assert!(node.value.type_name.is_some());
        let redefines = node.value.redefines.as_ref().expect("redefines");
        assert_eq!(redefines.value.target.len(), 1);
    }

    /// Systems Library `States.sysml`: multiplicity before the typing, and a comma-separated
    /// multi-target redefinition. Both targets must be retained.
    #[test]
    fn in_out_decl_accepts_multiplicity_before_typing_and_multi_target_redefines() {
        let (rest, node) = in_out_decl(input(
            "in transitionLinkSource[1]: StateAction :>> A::t, B::t;",
        ))
        .expect("multiplicity before typing");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.multiplicity.is_some());
        assert!(node.value.type_name.is_some());
        let redefines = node.value.redefines.as_ref().expect("redefines");
        assert_eq!(redefines.value.target.len(), 2);
    }

    /// Systems Library `Actions.sysml`/`Interfaces.sysml`: `ordered`/`nonunique` multiplicity
    /// properties, with and without a typing clause.
    #[test]
    fn in_out_decl_retains_ordered_and_nonunique_flags() {
        let (_, node) = in_out_decl(input("inout replacementValues : Anything[0..*] nonunique;"))
            .expect("nonunique after typing");
        assert!(node.value.nonunique);
        assert!(!node.value.ordered);

        let (_, node) =
            in_out_decl(input("in seq[1..*] nonunique ordered;")).expect("untyped modifiers");
        assert!(node.value.nonunique);
        assert!(node.value.ordered);
        assert!(node.value.type_name.is_none());
    }

    /// Systems Library `Actions.sysml`: `default <expr>` value clause plus a retained `{ ... }`
    /// terminator body.
    #[test]
    fn in_out_decl_accepts_default_expression_value_with_body() {
        let (rest, node) = in_out_decl(input(
            "in target : Occurrence[1] default that as Occurrence { doc /* d */ }",
        ))
        .expect("default value with body");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let value = node.value.value.as_ref().expect("feature value");
        assert!(value.value.is_default);
        let body = node.value.body.as_ref().expect("retained body");
        assert_eq!(body.len(), 1);
        assert!(matches!(body[0].value, ActionDefBodyElement::Doc(_)));
    }

    /// A malformed declaration must fail the parser (so the enclosing body produces an explicit
    /// recovery node) instead of guessing a declaration.
    #[test]
    fn in_out_decl_rejects_malformed_declaration() {
        assert!(in_out_decl(input("in : ;")).is_err());
    }

    /// `in occurrence …` must be rejected here so action-body dispatch routes it to the directed
    /// occurrence-usage arm.
    #[test]
    fn in_out_decl_rejects_occurrence_keyword() {
        assert!(in_out_decl(input("in occurrence o[1];")).is_err());
    }
}

#[cfg(test)]
mod control_node_gap_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    /// PARSER_BACKLOG_ROADMAP.md §6, G13: `first <name>;` with no `then` clause marks an initial
    /// node. Real usage: OMG spec Annex `3a-Function-based Behavior-2.sysml`, where the following
    /// `then merge continue;` lines supply the succession targets.
    #[test]
    fn action_body_accepts_first_without_a_then_clause() {
        let (rest, node) = action_def_body_element(input("first start;")).expect("first stmt");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            ActionDefBodyElement::FirstStmt(f) => assert!(f.value.then.is_none()),
            other => panic!("expected FirstStmt, got {other:?}"),
        }
    }

    #[test]
    fn action_body_still_accepts_first_then_succession() {
        let (rest, node) =
            action_def_body_element(input("first engineStarted then 'generate torque';"))
                .expect("first/then stmt");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            ActionDefBodyElement::FirstStmt(f) => assert!(f.value.then.is_some()),
            other => panic!("expected FirstStmt, got {other:?}"),
        }
    }

    /// GH-38: `succession first ... then ...;` -- the bare `succession` keyword prefix, no
    /// name/type/multiplicity.
    #[test]
    fn action_body_accepts_bare_succession_prefix() {
        let (rest, node) =
            action_def_body_element(input("succession first validate then checkRoute;"))
                .expect("succession first/then stmt");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            ActionDefBodyElement::FirstStmt(f) => {
                assert_eq!(f.value.succession_name, None);
                assert_eq!(f.value.succession_type, None);
                assert!(f.value.then.is_some());
            }
            other => panic!("expected FirstStmt, got {other:?}"),
        }
    }

    /// GH-38: `succession s first a then b;` -- named succession (real usage:
    /// `ConnectionTest.sysml`).
    #[test]
    fn action_body_accepts_named_succession_prefix() {
        let (rest, node) = action_def_body_element(input("succession s first a then b;"))
            .expect("named succession stmt");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            ActionDefBodyElement::FirstStmt(f) => {
                assert_eq!(f.value.succession_name.as_deref(), Some("s"));
                assert_eq!(f.value.succession_type, None);
            }
            other => panic!("expected FirstStmt, got {other:?}"),
        }
    }

    /// GH-38: `succession s1 : AB first a then b;` -- named and typed succession (real usage:
    /// `ConnectionTest.sysml`).
    #[test]
    fn action_body_accepts_named_and_typed_succession_prefix() {
        let source = input("succession s1 : AB first a then b;");
        let (rest, node) = action_def_body_element(source).expect("named + typed succession stmt");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            ActionDefBodyElement::FirstStmt(f) => {
                assert_eq!(f.value.succession_name.as_deref(), Some("s1"));
                assert_eq!(
                    f.value
                        .succession_type
                        .and_then(|id| crate::parser::usage::reference_text(source, id))
                        .as_deref(),
                    Some("AB")
                );
            }
            other => panic!("expected FirstStmt, got {other:?}"),
        }
    }

    /// GH-38: `succession [mult] first [mult] a then [mult] b;` -- multiplicity on the
    /// succession itself and on each end (real usage: Systems Library `Flows.sysml`:
    /// `succession [seBeforeNum] first [0..1] sourceEvent then [0..1] self;`).
    #[test]
    fn action_body_accepts_succession_and_end_multiplicities() {
        let (rest, node) = action_def_body_element(input(
            "succession [seBeforeNum] first [0..1] sourceEvent then [0..1] self;",
        ))
        .expect("succession with multiplicities stmt");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            ActionDefBodyElement::FirstStmt(f) => {
                assert_eq!(f.value.succession_name, None);
                assert!(f.value.succession_multiplicity.is_some());
                assert!(f.value.first_multiplicity.is_some());
                assert!(f.value.then_multiplicity.is_some());
            }
            other => panic!("expected FirstStmt, got {other:?}"),
        }
    }

    /// GH-38: `succession name first [mult] a then [mult] b { ... }` -- named succession with
    /// end multiplicities and a brace body (real usage: Systems Library `States.sysml`).
    #[test]
    fn action_body_accepts_named_succession_with_end_multiplicities_and_brace_body() {
        let source = "succession stateSequencing first [0..1] exclusiveStates then [0..1] exclusiveStates { in pin; }";
        let (rest, node) = action_def_body_element(input(source))
            .expect("named succession with multiplicities + brace body");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            ActionDefBodyElement::FirstStmt(f) => {
                assert_eq!(f.value.succession_name.as_deref(), Some("stateSequencing"));
                assert!(f.value.first_multiplicity.is_some());
                assert!(f.value.then_multiplicity.is_some());
                let FirstMergeBody::Brace(body) = f.value.body else {
                    panic!("expected source-backed brace body");
                };
                assert_eq!(
                    &source[body.span.offset..body.span.offset + body.span.len],
                    "{ in pin; }"
                );
                assert_eq!(body.value.open_brace_span.offset, body.span.offset);
                assert_eq!(body.value.open_brace_span.len, 1);
                assert_eq!(body.value.elements.len(), 1);
                assert!(matches!(
                    &body.value.elements[0].value,
                    FirstMergeBodyElement::Member(member)
                        if matches!(member.value, ActionDefBodyElement::InOutDecl(_))
                ));
                assert_eq!(
                    body.value.close_brace_span.offset,
                    body.span.offset + body.span.len - 1
                );
                assert_eq!(body.value.close_brace_span.len, 1);
            }
            other => panic!("expected FirstStmt, got {other:?}"),
        }
    }

    #[test]
    fn first_merge_body_keeps_structured_decl_and_malformed_members_then_resumes() {
        // BNF `MergeNode`/`DecisionNode`/`JoinNode`/`ForkNode` (SysML-textual-bnf.kebnf
        // §8.2.2.17.3) all use the same `ActionBody` production as a plain action definition
        // body, so `calc opaque;` here is a typed `ActionDefBodyElement::CalcUsage` member, not
        // an unsupported one.
        let source = "first start then finish { calc opaque; bogus ???; in resumed; }";
        let (rest, node) = action_def_body_element(input(source)).expect("first/merge body");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let ActionDefBodyElement::FirstStmt(first) = node.value else {
            panic!("expected FirstStmt");
        };
        let FirstMergeBody::Brace(body) = first.value.body else {
            panic!("expected brace body");
        };
        assert_eq!(body.value.elements.len(), 3);
        assert!(matches!(
            &body.value.elements[0].value,
            FirstMergeBodyElement::Member(member)
                if matches!(member.value, ActionDefBodyElement::CalcUsage(_))
        ));
        assert!(matches!(
            body.value.elements[1].value,
            FirstMergeBodyElement::Error(_)
        ));
        assert!(matches!(
            &body.value.elements[2].value,
            FirstMergeBodyElement::Member(member)
                if matches!(member.value, ActionDefBodyElement::InOutDecl(_))
        ));
    }

    /// §6 G14: `loop { ... }` is a `while` with no condition. The §5 audit wired
    /// `decide`/`join`/`fork`/`if`/`while` but missed `loop`, so it fell through to opaque
    /// recovery. Real usage: OMG spec Annex `3a-Function-based Behavior-3.sysml`.
    #[test]
    fn action_body_accepts_loop_control_node() {
        let (rest, node) = action_def_body_element(input("loop { action x; }")).expect("loop stmt");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, ActionDefBodyElement::LoopStmt(_)));
    }

    #[test]
    fn action_usage_body_accepts_loop_control_node() {
        let (rest, node) =
            action_usage_body_element(input("loop { action x; }")).expect("loop stmt");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, ActionUsageBodyElement::LoopStmt(_)));
    }

    /// Spec42 Gap 33: `attribute`/`calc`/`event`/nested `action def` members in action bodies
    /// dispatch through their typed productions instead of the retired opaque `ActionBodyDecl`.
    #[test]
    fn action_def_body_dispatches_declarations_to_typed_productions() {
        let parse = |source| {
            let (rest, node) = action_def_body_element(input(source)).expect(source);
            assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
            node.value
        };
        assert!(matches!(
            parse("attribute mass = 5;"),
            ActionDefBodyElement::AttributeUsage(_)
        ));
        assert!(matches!(
            parse("calc estimate : Estimate;"),
            ActionDefBodyElement::CalcUsage(_)
        ));
        assert!(matches!(
            parse("event occurrence crossings[0..*] : Crossing;"),
            ActionDefBodyElement::OccurrenceUsage(_)
        ));
        assert!(matches!(
            parse("action def Nested { in signal; }"),
            ActionDefBodyElement::ActionDef(_)
        ));
    }

    #[test]
    fn action_usage_body_dispatches_declarations_to_typed_productions() {
        let parse = |source| {
            let (rest, node) = action_usage_body_element(input(source)).expect(source);
            assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
            node.value
        };
        assert!(matches!(
            parse("attribute duration = 10;"),
            ActionUsageBodyElement::AttributeUsage(_)
        ));
        assert!(matches!(
            parse("calc estimate : Estimate;"),
            ActionUsageBodyElement::CalcUsage(_)
        ));
        assert!(matches!(
            parse("event marker;"),
            ActionUsageBodyElement::OccurrenceUsage(_)
        ));
        assert!(matches!(
            parse("action def Nested { in signal; }"),
            ActionUsageBodyElement::ActionDef(_)
        ));
    }

    /// Spec42 Gap 33: the nested `action def`'s full declaration is retained (formerly flattened
    /// to an opaque keyword/text pair).
    #[test]
    fn nested_action_def_retains_its_declaration() {
        let (rest, node) =
            action_def_body_element(input("action def Nested { in signal; }")).expect("action def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let ActionDefBodyElement::ActionDef(def) = node.value else {
            panic!("expected ActionDef");
        };
        assert_eq!(def.value.identification.name.as_deref(), Some("Nested"));
        let ActionDefBody::Brace { elements, .. } = &def.value.body else {
            panic!("expected brace body");
        };
        assert!(matches!(
            elements[0].value,
            ActionDefBodyElement::InOutDecl(_)
        ));
    }

    /// §6 G23 (found while fixing G13): only `then action ...` was accepted, so the two other
    /// succession-shorthand targets that follow `first start;` in
    /// `3a-Function-based Behavior-2.sysml` fell through to opaque recovery.
    #[test]
    fn then_succession_accepts_a_merge_node_target() {
        let (rest, node) = then_action(input("then merge continue;")).expect("then merge");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value.target, ThenTarget::Merge(_)));
    }

    #[test]
    fn then_succession_accepts_a_bare_feature_target() {
        let (rest, node) = then_action(input("then continue;")).expect("then feature");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value.target, ThenTarget::Feature(_)));
    }

    #[test]
    fn then_succession_still_accepts_an_inline_action_declaration() {
        let (rest, node) = then_action(input("then action engineStarted accept e: EngineStart;"))
            .expect("then action");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value.target, ThenTarget::Action(_)));
    }

    #[test]
    fn then_succession_accepts_anonymous_action_with_accept_payload() {
        let (rest, node) = then_action(input("then action accept engineOff : EngineOff;"))
            .expect("then action accept");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value.target {
            ThenTarget::Action(a) => {
                assert!(a.value.name.is_empty(), "expected anonymous action");
                assert!(a.value.accept.is_some());
            }
            other => panic!("expected Action target, got {other:?}"),
        }
    }

    #[test]
    fn then_succession_accepts_perform_target() {
        let source = input("then perform body;");
        let (rest, node) = then_action(source).expect("then perform");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value.target {
            ThenTarget::Perform(p) => {
                assert!(p.value.action_name.is_empty());
                assert_eq!(
                    p.value
                        .action_reference
                        .and_then(|id| crate::parser::usage::reference_text(source, id))
                        .as_deref(),
                    Some("body")
                );
            }
            other => panic!("expected Perform target, got {other:?}"),
        }
    }

    #[test]
    fn action_body_accepts_then_perform() {
        let (rest, node) =
            action_def_body_element(input("then perform body;")).expect("then perform body");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            ActionDefBodyElement::ThenAction(t) => {
                assert!(matches!(t.value.target, ThenTarget::Perform(_)));
            }
            other => panic!("expected ThenAction, got {other:?}"),
        }
    }

    /// GH-13 / BNF `ActionBodyItem` → `StructureUsageMember` → `PartUsage`.
    #[test]
    fn action_body_accepts_part_usage() {
        let (rest, node) = action_def_body_element(input("part rim : Wheel;")).expect("part usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            ActionDefBodyElement::PartUsage(p) => assert_eq!(p.value.name, "rim"),
            other => panic!("expected PartUsage, got {other:?}"),
        }
    }

    /// GH-13: anonymous `part :>> name { }` redefinition in an action body.
    #[test]
    fn action_body_accepts_anonymous_part_redefinition() {
        let (rest, node) =
            action_def_body_element(input("part :>> station { }")).expect("part :>>");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, ActionDefBodyElement::PartUsage(_)));
    }

    /// GH-13 / BNF `ActionBodyItem` → `StructureUsageMember` → `ItemUsage`.
    #[test]
    fn action_body_accepts_item_usage_and_for_loop() {
        let src = "item spokes : Spoke[*];";
        let (rest, node) = action_def_body_element(input(src)).expect("item usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            ActionDefBodyElement::ItemUsage(i) => assert_eq!(i.value.name, "spokes"),
            other => panic!("expected ItemUsage, got {other:?}"),
        }

        // Spec keyword is `for` (ForLoopNode), not `foreach`.
        let (rest, node) =
            action_def_body_element(input("for s in spokes { action tighten; }")).expect("for");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, ActionDefBodyElement::ForLoop(_)));
    }

    /// GH-13 / BNF `AssertConstraintUsage` via `BehaviorUsageMember`.
    #[test]
    fn action_body_accepts_assert_constraint() {
        let (rest, node) =
            action_def_body_element(input("assert constraint { 1 > 0 }")).expect("assert");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            ActionDefBodyElement::AssertConstraint(_)
        ));
    }

    /// GH-13 / BNF `RefPrefix.isVariation` on `ActionUsage`.
    #[test]
    fn action_body_accepts_variation_action_usage() {
        let (rest, node) = action_def_body_element(input(
            "variation action method { action byHand; action byJig; }",
        ))
        .expect("variation action");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            ActionDefBodyElement::ActionUsage(a) => {
                assert!(a.value.is_variation);
                assert!(!a.value.is_abstract);
                assert_eq!(a.value.name, "method");
            }
            other => panic!("expected ActionUsage, got {other:?}"),
        }
    }

    /// GH-13 / BNF `StructureUsageMember` → `PortionUsage` (`snapshot`).
    #[test]
    fn action_body_accepts_snapshot_usage() {
        let (rest, node) = action_def_body_element(input("snapshot trued { }")).expect("snapshot");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            ActionDefBodyElement::OccurrenceUsage(o) => {
                assert_eq!(
                    o.value.portion_kind,
                    Some(crate::ast::OccurrencePortionKind::Snapshot)
                );
                assert_eq!(o.value.name, "trued");
            }
            other => panic!("expected OccurrenceUsage, got {other:?}"),
        }
    }

    #[test]
    fn action_usage_body_accepts_part_and_item_members() {
        let (rest, node) =
            action_usage_body_element(input("part rim : Wheel;")).expect("part in usage body");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, ActionUsageBodyElement::PartUsage(_)));

        let (rest, node) =
            action_usage_body_element(input("item spokes : Spoke[*];")).expect("item");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, ActionUsageBodyElement::ItemUsage(_)));
    }

    /// End-to-end GH-13 samples from https://github.com/elan8/sysml-v2-parser/issues/13
    #[test]
    fn gh13_issue_samples_parse_cleanly() {
        let samples = [
            r#"package Shop {
                part def Wheel;
                action def Truing { part rim : Wheel; }
            }"#,
            r#"package Shop {
                item def Spoke;
                action def Truing {
                    item spokes : Spoke[*];
                    for s in spokes { action tighten; }
                }
            }"#,
            r#"package Shop {
                action def Truing { assert constraint { 1 > 0 } }
            }"#,
            r#"package Shop {
                action def Truing {
                    variation action method {
                        action byHand;
                        action byJig;
                    }
                }
            }"#,
            r#"package Shop {
                action def Truing { snapshot trued { } }
            }"#,
            r#"package Shop {
                action def Assembly { part :>> station { } }
            }"#,
        ];
        for sample in samples {
            crate::parse(sample)
                .unwrap_or_else(|e| panic!("parse failed for sample:\n{sample}\n{e}"));
        }
    }
}

#[cfg(test)]
mod membership_tests {
    use super::*;
    use crate::ast::Expression;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    // --- parser work item 4b (continuation): Membership on ActionDef/ActionUsage ---

    #[test]
    fn action_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = action_def(input("private action def A1;")).expect("action def");
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
    fn action_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = action_def(input("action def A1;")).expect("action def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn action_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) = action_usage(input("protected action a1 : A1;")).expect("action usage");
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
    fn action_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = action_usage(input("action a1 : A1;")).expect("action usage");
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn abstract_ref_action_with_multiplicity_and_subsets_is_structured() {
        let src =
            "abstract ref action performedActions: Action[0..*] :> actions, enactedPerformances;";
        let source = input(src);
        let (rest, node) = action_usage(source).expect("ref action usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.is_abstract);
        assert!(node.value.is_reference);
        assert_eq!(node.value.name, "performedActions");
        assert_eq!(
            node.value
                .type_name
                .and_then(|id| crate::parser::usage::reference_text(source, id))
                .as_deref(),
            Some("Action")
        );
        assert!(node.value.typing.is_some());
        let multiplicity = &node
            .value
            .multiplicity
            .as_ref()
            .expect("multiplicity")
            .value;
        assert!(matches!(
            multiplicity.lower.as_deref().map(|bound| &bound.value),
            Some(Expression::LiteralInteger(0))
        ));
        assert!(multiplicity.upper.is_none());
        let subsets = crate::parser::usage::reference_list_text(
            source,
            &node
                .value
                .subsets
                .as_ref()
                .expect("subsets clause")
                .value
                .target,
        );
        assert_eq!(subsets, "actions, enactedPerformances");
    }
}
