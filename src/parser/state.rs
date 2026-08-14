use crate::ast::{
    DoAction, EntryAction, ExitAction, FinalState, Membership, Node, RefDecl, StateDef,
    StateDefBody, StateDefBodyElement, StateUsage, ThenStmt, Transition, TransitionEffect,
};
use crate::parser::body::{advance_to_closing_brace, parse_structured_brace_members};
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::expr::expression;
use crate::parser::lex::{
    name, qualified_reference, reference_path, starts_with_keyword, take_until_terminator,
    visibility_prefix, ws1, ws_and_comments, STATE_BODY_STARTERS,
};

const UNTIL_BODY: &[u8] = b";{";
use crate::parser::metadata_annotation::{annotation, metadata_annotation, metadata_keyword_usage};
use crate::parser::node_from_to;
use crate::parser::payload::transition_accept;
use crate::parser::requirement::{doc_comment, requirement_usage};
use crate::parser::usage::multiplicity;
use crate::parser::with_span;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

pub(crate) fn state_def(input: Input<'_>) -> IResult<Input<'_>, Node<StateDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"state")
            .def_required()
            .individual_allowed()
            .with_captured_visibility(),
    )?;
    let (input, body) = state_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            StateDef {
                is_individual: prefix.is_individual,
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
                membership: Membership::owning(prefix.visibility, prefix.visibility_span),
            },
        ),
    ))
}

pub(crate) fn state_def_body(input: Input<'_>) -> IResult<Input<'_>, StateDefBody> {
    alt((
        map(preceded(ws_and_comments, tag(&b";"[..])), |_| {
            StateDefBody::Semicolon
        }),
        state_def_body_brace,
    ))
    .parse(input)
}

fn state_def_body_brace(input: Input<'_>) -> IResult<Input<'_>, StateDefBody> {
    let (input, elements) = parse_structured_brace_members(
        input,
        STATE_BODY_STARTERS,
        "state body",
        "recovered_state_body_element",
        state_def_body_element,
        |start, end| {
            let recovery = build_recovery_error_node_from_span(
                start,
                end,
                STATE_BODY_STARTERS,
                "state body",
                "recovered_state_body_element",
            );
            if matches!(
                recovery.code.as_str(),
                "missing_type_reference"
                    | "invalid_bare_identifier_in_state_body"
                    | "missing_semicolon"
                    | "missing_body_or_semicolon"
            ) {
                node_from_to(
                    start,
                    end,
                    StateDefBodyElement::Error(node_from_to(start, end, recovery)),
                )
            } else {
                let frag = start.fragment();
                let take = frag.len().min(80);
                let preview = String::from_utf8_lossy(&frag[..take]).trim().to_string();
                node_from_to(start, end, StateDefBodyElement::Other(preview))
            }
        },
    )?;
    Ok((input, StateDefBody::Brace { elements }))
}

/// Shared `entry`/`do`/`exit` header: optional `action` keyword plus an optional source-backed
/// action target.
/// Everything an `entry`/`do`/`exit` keyword may introduce (spec42 Gap 43): nothing (a plain
/// body), an `assign`/`send`/`accept` effect, a new named/typed/redefining nested action
/// declaration, or a reference to an existing action.
struct StateActionHead {
    has_action_keyword: bool,
    action_reference: Option<crate::ast::QualifiedReferenceId>,
    declared_name: Option<String>,
    type_name: Option<crate::ast::QualifiedReferenceId>,
    redefines: Option<Node<crate::ast::SubsettingRelationship>>,
    effect: Option<crate::ast::TransitionEffect>,
}

impl StateActionHead {
    fn empty(has_action_keyword: bool) -> Self {
        StateActionHead {
            has_action_keyword,
            action_reference: None,
            declared_name: None,
            type_name: None,
            redefines: None,
            effect: None,
        }
    }
}

fn state_behavior_action_target(input: Input<'_>) -> IResult<Input<'_>, StateActionHead> {
    // `entry assign counter.count := 0;` / `do send Sig() to port;` -- an effect written
    // directly under the keyword rather than inside a transition's effect clause (spec42
    // `assignment_test`, `25_change_and_time_triggers`; Gap 43).
    {
        let (peek, _) = ws_and_comments(input)?;
        if starts_with_keyword(peek.fragment(), b"send")
            || starts_with_keyword(peek.fragment(), b"accept")
            || starts_with_keyword(peek.fragment(), b"assign")
        {
            // The individual effect parsers, not `transition_effect`: its trailing-`;` leniency
            // would starve the caller's own body terminator.
            let (input, effect) = alt((
                transition_effect_accept,
                transition_effect_send,
                transition_effect_assign,
            ))
            .parse(peek)?;
            return Ok((
                input,
                StateActionHead {
                    effect: Some(effect),
                    ..StateActionHead::empty(false)
                },
            ));
        }
    }
    let (input, has_action_keyword) = opt(preceded(ws_and_comments, tag(&b"action"[..])))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") || input.fragment().starts_with(b"{") {
        return Ok((input, StateActionHead::empty(has_action_keyword)));
    }
    // New nested action declaration: `entry action entryAction :>> 'entry';`, `do action
    // doAction : Action :>> 'do';` (Systems Library `States.sysml`; spec42 Gap 43). The leading
    // token declares a name here, so it must not be interned as a reference.
    let declaration_attempt = (|| -> IResult<Input<'_>, StateActionHead> {
        let (i, declared_name) = preceded(ws_and_comments, name).parse(input)?;
        let (i, type_name) = {
            let (peek, _) = ws_and_comments(i)?;
            if peek.fragment().starts_with(b":") && !peek.fragment().starts_with(b":>") {
                let (i, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(i)?;
                let (i, ty) = preceded(ws_and_comments, qualified_reference).parse(i)?;
                (i, Some(ty))
            } else {
                (i, None)
            }
        };
        let (i, redefines) = opt(preceded(
            ws_and_comments,
            crate::parser::usage::redefinition,
        ))
        .parse(i)?;
        if type_name.is_none() && redefines.is_none() {
            return Err(nom::Err::Error(nom::error::Error::new(
                i,
                nom::error::ErrorKind::Tag,
            )));
        }
        let (peek, _) = ws_and_comments(i)?;
        if !(peek.fragment().starts_with(b";") || peek.fragment().starts_with(b"{")) {
            return Err(nom::Err::Error(nom::error::Error::new(
                i,
                nom::error::ErrorKind::Tag,
            )));
        }
        Ok((
            i,
            StateActionHead {
                declared_name: Some(declared_name),
                type_name,
                redefines,
                ..StateActionHead::empty(has_action_keyword)
            },
        ))
    })();
    if let Ok(result) = declaration_attempt {
        return Ok(result);
    }
    // Bare referenced action usage: `do 'sense temperature' { … }` / `entry initial;`.
    let (input, action_reference) = reference_path(input)?;
    Ok((
        input,
        StateActionHead {
            action_reference: Some(action_reference),
            ..StateActionHead::empty(has_action_keyword)
        },
    ))
}

/// Entry action: `entry` (`;` or body) or `entry action` path body / `entry` path body.
fn entry_action(input: Input<'_>) -> IResult<Input<'_>, Node<EntryAction>> {
    crate::parser::span::reference_transaction(input, entry_action_inner)
}

fn entry_action_inner(input: Input<'_>) -> IResult<Input<'_>, Node<EntryAction>> {
    let start = input;
    let (input, _) = tag(&b"entry"[..]).parse(input)?;
    let (input, head) = state_behavior_action_target(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = state_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            EntryAction {
                action_reference: head.action_reference,
                has_action_keyword: head.has_action_keyword,
                declared_name: head.declared_name,
                type_name: head.type_name,
                redefines: head.redefines,
                effect: head.effect,
                body,
            },
        ),
    ))
}

/// Do action: `do` (`;` or body) or `do action` path body / `do` path body.
fn do_action(input: Input<'_>) -> IResult<Input<'_>, Node<DoAction>> {
    crate::parser::span::reference_transaction(input, do_action_inner)
}

fn do_action_inner(input: Input<'_>) -> IResult<Input<'_>, Node<DoAction>> {
    let start = input;
    let (input, _) = tag(&b"do"[..]).parse(input)?;
    let (input, head) = state_behavior_action_target(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = state_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            DoAction {
                action_reference: head.action_reference,
                has_action_keyword: head.has_action_keyword,
                declared_name: head.declared_name,
                type_name: head.type_name,
                redefines: head.redefines,
                effect: head.effect,
                body,
            },
        ),
    ))
}

/// Exit action: `exit` (`;` or body) or `exit action` path body / `exit` path body.
fn exit_action(input: Input<'_>) -> IResult<Input<'_>, Node<ExitAction>> {
    crate::parser::span::reference_transaction(input, exit_action_inner)
}

fn exit_action_inner(input: Input<'_>) -> IResult<Input<'_>, Node<ExitAction>> {
    let start = input;
    let (input, _) = tag(&b"exit"[..]).parse(input)?;
    let (input, head) = state_behavior_action_target(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = state_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ExitAction {
                action_reference: head.action_reference,
                has_action_keyword: head.has_action_keyword,
                declared_name: head.declared_name,
                type_name: head.type_name,
                redefines: head.redefines,
                effect: head.effect,
                body,
            },
        ),
    ))
}

/// Ref in state body: `ref` (`state`)? name (`:` type)? (`:>>` / `:>` redeclarations)? body
fn state_ref(input: Input<'_>) -> IResult<Input<'_>, Node<RefDecl>> {
    let start = input;
    let (input, _) = tag(&b"ref"[..]).parse(input)?;
    let (input, _) = opt(preceded(ws1, tag(&b"state"[..]))).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, parsed_name) = opt(with_span(name)).parse(input)?;
    let (input, _multiplicity) = opt(multiplicity).parse(input)?;
    let (name_span, name_str) = parsed_name.unwrap_or((crate::ast::Span::dummy(), String::new()));

    let (input, uses_shift) = preceded(
        ws_and_comments,
        alt((
            map(tag(&b":>>"[..]), |_| true),
            map(tag(&b":>"[..]), |_| false),
            map(tag(&b":"[..]), |_| false),
        )),
    )
    .parse(input)?;
    let (input, type_target) = if uses_shift {
        (input, None)
    } else {
        let (input, target) =
            preceded(ws_and_comments, with_span(qualified_reference)).parse(input)?;
        (input, Some(target))
    };
    let type_ref_span = type_target
        .as_ref()
        .map(|(span, _)| span.clone())
        .unwrap_or_else(crate::ast::Span::dummy);
    let typing = type_target.map(|(span, id)| crate::parser::usage::single_target_typing(span, id));

    let (input, _) = ws_and_comments(input)?;
    let (mut input, value) = opt(preceded(
        preceded(ws_and_comments, tag(&b"="[..])),
        preceded(ws_and_comments, expression),
    ))
    .parse(input)?;
    let value = value.map(crate::parser::feature_value::wrap_bind_expression);

    if !input.fragment().is_empty()
        && !input.fragment().starts_with(b";")
        && !input.fragment().starts_with(b"{")
    {
        let (next, _) = take_until_terminator(input, UNTIL_BODY)?;
        input = next;
    }

    let (input, body) = preceded(ws_and_comments, crate::parser::part::ref_body).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            RefDecl {
                direction: None,
                kind_keyword: None,
                name: name_str,
                typing,
                redefines: None,
                subsets: None,
                multiplicity: None,
                ordered: false,
                nonunique: false,
                value,
                body,
                name_span: Some(name_span),
                type_ref_span: Some(type_ref_span),
                membership: Membership::feature(None, crate::ast::Span::dummy()),
            },
        ),
    ))
}

/// Then (initial state): `then` state-path `;`.
fn then_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<ThenStmt>> {
    crate::parser::span::reference_transaction(input, then_stmt_inner)
}

fn then_stmt_inner(input: Input<'_>) -> IResult<Input<'_>, Node<ThenStmt>> {
    let start = input;
    let (input, _) = tag(&b"then"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, state_reference) = crate::parser::lex::reference_path(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(start, input, ThenStmt { state_reference }),
    ))
}

/// Final state: `final` name `;` or `final state` name `;`
fn final_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<FinalState>> {
    let start = input;
    let (input, _) = tag(&b"final"[..]).parse(input)?;
    let (input, _) = opt(preceded(ws1, tag(&b"state"[..]))).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, (name_span, state_name)) = with_span(name).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            FinalState {
                state_name,
                name_span,
            },
        ),
    ))
}

fn state_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<StateDefBodyElement>> {
    let start = input;
    let mut parser = alt((
        map(doc_comment, |n| {
            node_from_to(start, input, StateDefBodyElement::Doc(n))
        }),
        map(metadata_keyword_usage, |n| {
            node_from_to(start, input, StateDefBodyElement::MetadataKeywordUsage(n))
        }),
        map(metadata_annotation, |n| {
            node_from_to(start, input, StateDefBodyElement::MetadataAnnotation(n))
        }),
        map(annotation, |n| {
            node_from_to(start, input, StateDefBodyElement::Annotation(n))
        }),
        map(entry_action, |n| {
            node_from_to(start, input, StateDefBodyElement::Entry(n))
        }),
        map(do_action, |n| {
            node_from_to(start, input, StateDefBodyElement::Do(n))
        }),
        map(exit_action, |n| {
            node_from_to(start, input, StateDefBodyElement::Exit(n))
        }),
        map(then_stmt, |n| {
            node_from_to(start, input, StateDefBodyElement::Then(n))
        }),
        map(final_stmt, |n| {
            node_from_to(start, input, StateDefBodyElement::FinalState(n))
        }),
        map(state_ref, |n| {
            node_from_to(start, input, StateDefBodyElement::Ref(Box::new(n)))
        }),
        map(requirement_usage, |n| {
            node_from_to(start, input, StateDefBodyElement::RequirementUsage(n))
        }),
        map(state_usage, |n| {
            node_from_to(start, input, StateDefBodyElement::StateUsage(n))
        }),
        // spec42 Gap 42: general usage members legal in a state body (Systems Library
        // `States.sysml`): `attribute :>> isTriggerDuring;`, `action :>> subactions :> middle
        // { ... }`, `succession stateSequencing first [0..1] ... then [0..1] ...`, and
        // `assert constraint { ... }` -- dispatched to the same typed productions sibling body
        // enums already use instead of falling to opaque recovery.
        map(crate::parser::attribute::attribute_usage, |n| {
            node_from_to(
                start,
                input,
                StateDefBodyElement::AttributeUsage(Box::new(n)),
            )
        }),
        map(crate::parser::action::action_usage, |n| {
            node_from_to(start, input, StateDefBodyElement::ActionUsage(Box::new(n)))
        }),
        map(crate::parser::occurrence_body::succession_usage, |n| {
            node_from_to(start, input, StateDefBodyElement::SuccessionUsage(n))
        }),
        map(
            crate::parser::occurrence_body::assert_constraint_member,
            |n| node_from_to(start, input, StateDefBodyElement::AssertConstraint(n)),
        ),
        map(crate::parser::action::in_out_decl, |n| {
            node_from_to(start, input, StateDefBodyElement::InOutDecl(n))
        }),
        map(transition, |n| {
            node_from_to(start, input, StateDefBodyElement::Transition(Box::new(n)))
        }),
        map(transition_shorthand, |n| {
            node_from_to(start, input, StateDefBodyElement::Transition(Box::new(n)))
        }),
    ));
    parser.parse(input)
}

/// State usage: `OccurrenceUsagePrefix` subset `state` name (`:` type)? (`:>`/`:>>` …)? body.
/// GH-45: `direction`/`derived`/`individual` prefix keywords follow BNF `RefPrefix`/
/// `OccurrenceUsagePrefix` order (§8.2.2.9.2, §8.2.2.18.2, reached via `StateUsage =
/// OccurrenceUsagePrefix 'state' ...` -> `OccurrenceUsagePrefix : BasicUsagePrefix ...` ->
/// `BasicUsagePrefix : RefPrefix ...`): direction, `derived`, `abstract`, `ref`, then
/// `individual` -- mirrors `part::usage::part_usage`'s identical composition. `constant` and
/// portion kind (`snapshot`/`timeslice`) are the remaining `OccurrenceUsagePrefix` slots; left
/// out for now, no real usage evidence found on state usages to justify them (checked against the
/// vendored SysML v2 Systems Library / spec Annex examples).
pub(crate) fn state_usage(input: Input<'_>) -> IResult<Input<'_>, Node<StateUsage>> {
    let start = input;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, direction) =
        nom::combinator::opt(crate::parser::attribute::direction_prefix).parse(input)?;
    let (input, is_derived) =
        nom::combinator::opt(preceded(tag(&b"derived"[..]), ws1)).parse(input)?;
    let (input, is_abstract) =
        nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, is_reference) =
        nom::combinator::opt(preceded(tag(&b"ref"[..]), ws1)).parse(input)?;
    let (input, is_individual) =
        nom::combinator::opt(preceded(tag(&b"individual"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"state"[..]).parse(input)?;
    // SysML allows anonymous state usages: `state: Mode;` (Identification may be empty).
    let (after_gap, _) = ws_and_comments(input)?;
    // `state def …` is a definition, not a usage named `def`.
    if starts_with_keyword(after_gap.fragment(), b"def") {
        return Err(nom::Err::Error(nom::error::Error::new(
            after_gap,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, n) = if (after_gap.fragment().starts_with(b":")
        && !after_gap.fragment().starts_with(b":>")
        && !after_gap.fragment().starts_with(b":>>"))
        || starts_with_keyword(after_gap.fragment(), b"defined")
    {
        (after_gap, String::new())
    } else {
        let (input, _) = ws1(input)?;
        let (input, n) = name(input)?;
        (input, n)
    };
    let (input, leading) = crate::parser::usage::specialization_clauses(input)?;
    let (input, type_result) = crate::parser::usage::optional_typings(input)?;
    let (input, multiplicity) =
        nom::combinator::opt(crate::parser::usage::multiplicity_node).parse(input)?;
    let (input, _) = crate::parser::usage::skip_usage_feature_modifiers(input)?;
    let (input, trailing) = crate::parser::usage::specialization_clauses(input)?;
    let (_type_ref_span, type_name, typing) =
        crate::parser::usage::typing_reference_fields_from_result(type_result);
    let subsets = trailing
        .subsets
        .clone()
        .or(leading.subsets.clone())
        .map(|(target, _)| target);
    let redefines = trailing.redefines.clone().or(leading.redefines.clone());
    // Optional modifier before body: `parallel` or `initial` (SysML state usage)
    let (input, _) = opt(alt((
        preceded(preceded(ws_and_comments, tag(&b"parallel"[..])), ws1),
        preceded(preceded(ws_and_comments, tag(&b"initial"[..])), ws1),
    )))
    .parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = state_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            StateUsage {
                direction,
                is_derived: is_derived.is_some(),
                is_abstract: is_abstract.is_some(),
                is_reference: is_reference.is_some(),
                is_individual: is_individual.is_some(),
                name: n,
                state_reference: None,
                type_name,
                typing,
                multiplicity,
                subsets,
                redefines,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

/// Optional trailing `{ ActionBodyItem* }` on a transition effect action usage; contents are
/// not retained (mirrors how nested action-usage bodies are treated elsewhere in this module).
fn transition_effect_brace(input: Input<'_>) -> IResult<Input<'_>, ()> {
    map(
        delimited(
            tag(&b"{"[..]),
            advance_to_closing_brace,
            preceded(ws_and_comments, tag(&b"}"[..])),
        ),
        |_| (),
    )
    .parse(input)
}

/// Optional `: Type` suffix on an effect payload/declaration.
fn transition_effect_type_suffix(
    input: Input<'_>,
) -> IResult<Input<'_>, Option<crate::ast::QualifiedReferenceId>> {
    opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, qualified_reference),
    ))
    .parse(input)
}

/// `do action` effect: `action` name (`:` type)? — SysML v2 `PerformActionUsageDeclaration`'s
/// `'action' UsageDeclaration` form, e.g. `do action powerUp : PowerUp;`.
fn transition_effect_perform(input: Input<'_>) -> IResult<Input<'_>, TransitionEffect> {
    let (input, _) = preceded(tag(&b"action"[..]), ws1).parse(input)?;
    let (input, action_name) = name(input)?;
    let (input, type_name) = transition_effect_type_suffix(input)?;
    Ok((
        input,
        TransitionEffect::Perform {
            name: Some(action_name),
            type_name,
        },
    ))
}

/// `do accept` effect: `accept` payload (`:` type)? (`via` expr)?, e.g.
/// `do accept Ack via commPort`.
fn transition_effect_accept(input: Input<'_>) -> IResult<Input<'_>, TransitionEffect> {
    let (input, _) = preceded(tag(&b"accept"[..]), ws1).parse(input)?;
    let (input, payload) = expression(input)?;
    let (input, type_name) = transition_effect_type_suffix(input)?;
    let (input, via) = opt(preceded(
        preceded(ws_and_comments, tag(&b"via"[..])),
        preceded(ws1, expression),
    ))
    .parse(input)?;
    Ok((
        input,
        TransitionEffect::Accept {
            payload,
            type_name,
            via,
        },
    ))
}

/// `do send` effect: `send` payload (`:` type)? (`via` expr)? (`to` expr)? — SysML v2
/// `SenderReceiverPart`, e.g. `do send new TimeoutSignal() via commPort`.
fn transition_effect_send(input: Input<'_>) -> IResult<Input<'_>, TransitionEffect> {
    let (input, _) = preceded(tag(&b"send"[..]), ws1).parse(input)?;
    let (input, payload) = expression(input)?;
    let (input, type_name) = transition_effect_type_suffix(input)?;
    let (input, via) = opt(preceded(
        preceded(ws_and_comments, tag(&b"via"[..])),
        preceded(ws1, expression),
    ))
    .parse(input)?;
    let (input, to) = opt(preceded(
        preceded(ws_and_comments, tag(&b"to"[..])),
        preceded(ws1, expression),
    ))
    .parse(input)?;
    Ok((
        input,
        TransitionEffect::Send {
            payload,
            type_name,
            via,
            to,
        },
    ))
}

/// `do assign` effect: `assign` lhs `:=` rhs.
fn transition_effect_assign(input: Input<'_>) -> IResult<Input<'_>, TransitionEffect> {
    let (input, _) = preceded(tag(&b"assign"[..]), ws1).parse(input)?;
    let (input, lhs) = expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b":="[..])).parse(input)?;
    let (input, rhs) = preceded(ws_and_comments, expression).parse(input)?;
    Ok((input, TransitionEffect::Assign { lhs, rhs }))
}

/// Transition `do` effect: structured `action`/`accept`/`send`/`assign` action usage, or a bare
/// expression shorthand (e.g. a reference to an existing action usage).
fn transition_effect(input: Input<'_>) -> IResult<Input<'_>, TransitionEffect> {
    let (input, _) = ws_and_comments(input)?;
    let (input, effect) = alt((
        transition_effect_perform,
        transition_effect_accept,
        transition_effect_send,
        transition_effect_assign,
        map(expression, TransitionEffect::Expression),
    ))
    .parse(input)?;
    let (input, _) = opt(preceded(ws_and_comments, transition_effect_brace)).parse(input)?;
    // Lenient: some models write a trailing `;` after the effect action usage even though
    // the grammar's TransitionUsage has no separator before `then` (matches spec examples,
    // e.g. `do action powerUp : PowerUp;\nthen on;`).
    let (input, _) = opt(preceded(ws_and_comments, tag(&b";"[..]))).parse(input)?;
    Ok((input, effect))
}

pub(crate) fn transition(input: Input<'_>) -> IResult<Input<'_>, Node<Transition>> {
    let start = input;
    let (input, _) = tag(&b"transition"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, n) = {
        let (peek, _) = ws_and_comments(input)?;
        if starts_with_keyword(peek.fragment(), b"first")
            || starts_with_keyword(peek.fragment(), b"accept")
            || starts_with_keyword(peek.fragment(), b"if")
            || starts_with_keyword(peek.fragment(), b"do")
            || starts_with_keyword(peek.fragment(), b"then")
        {
            (input, None)
        } else {
            let (input, n) = name(input)?;
            (input, Some(n))
        }
    };
    transition_tail(start, input, n)
}

/// Shorthand transition without the `transition` keyword (validation `05-2`):
/// `accept … [if …] [do …] then target;` or `first … then …;` / `if … then …;` / `then …;`.
fn transition_shorthand(input: Input<'_>) -> IResult<Input<'_>, Node<Transition>> {
    let start = input;
    let (peek, _) = ws_and_comments(input)?;
    if !(starts_with_keyword(peek.fragment(), b"first")
        || starts_with_keyword(peek.fragment(), b"accept")
        || starts_with_keyword(peek.fragment(), b"if")
        || starts_with_keyword(peek.fragment(), b"then"))
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            peek,
            nom::error::ErrorKind::Tag,
        )));
    }
    // Bare `do … then …` is uncommon and would fight `do_action`; omit that starter here.
    transition_tail(start, input, None)
}

fn transition_tail<'a>(
    start: Input<'a>,
    input: Input<'a>,
    name: Option<String>,
) -> IResult<Input<'a>, Node<Transition>> {
    // Optional: `first` source with optional `accept` trigger.
    let (input, first_clause) = opt((
        preceded(ws_and_comments, tag(&b"first"[..])),
        ws1,
        expression,
        opt(transition_accept),
    ))
    .parse(input)?;
    let (source, accept_from_first, is_initial) = match first_clause {
        // Named transitions use `first` for the source state; only unnamed transitions are initial.
        Some((_, _, src, acc)) => (Some(src), acc, name.is_none()),
        None => (None, None, false),
    };
    // Shorthand may start with a top-level `accept` (no `first` source).
    let (input, accept) = if accept_from_first.is_some() {
        (input, accept_from_first)
    } else {
        let (input, acc) = opt(transition_accept).parse(input)?;
        (input, acc)
    };
    // Optional: `if` guard and `do` effect before `then`
    let (input, guard) = opt((
        preceded(ws_and_comments, tag(&b"if"[..])),
        preceded(ws1, expression),
    ))
    .parse(input)?;
    let guard = guard.map(|(_, expr)| expr);
    let (input, effect) = opt((
        preceded(ws_and_comments, tag(&b"do"[..])),
        preceded(ws1, transition_effect),
    ))
    .parse(input)?;
    let effect = effect.map(|(_, eff)| eff);
    let (input, _) = preceded(ws_and_comments, tag(&b"then"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, target) = expression(input)?;
    let (input, body) =
        preceded(ws_and_comments, crate::parser::connector::connect_body).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Transition {
                name,
                source,
                is_initial,
                accept,
                guard,
                effect,
                target,
                body,
            },
        ),
    ))
}

#[cfg(test)]
mod state_behavior_action_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    /// Spec42 Gap 43: `assign`/`send`/`accept` effects directly under `entry`/`do`/`exit`.
    #[test]
    fn entry_accepts_a_direct_assign_effect() {
        let (rest, node) =
            entry_action(input("entry assign counter.count := 0;")).expect("entry assign");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value.effect,
            Some(crate::ast::TransitionEffect::Assign { .. })
        ));
        assert!(node.value.action_reference.is_none());
    }

    /// Spec42 Gap 43: named/typed/redefining nested action declarations.
    #[test]
    fn do_accepts_a_named_redefining_action_declaration() {
        let (rest, node) = do_action(input("do action doAction : Action :>> 'do';"))
            .expect("do action declaration");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.declared_name.as_deref(), Some("doAction"));
        assert!(node.value.type_name.is_some());
        assert!(node.value.redefines.is_some());
        assert!(node.value.action_reference.is_none());
    }

    #[test]
    fn entry_accepts_a_bare_redefining_action_declaration() {
        let (rest, node) =
            entry_action(input("entry action entryAction :>> 'entry';")).expect("entry decl");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.declared_name.as_deref(), Some("entryAction"));
        assert!(node.value.redefines.is_some());
    }
}

#[cfg(test)]
mod membership_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    // --- parser work item 4b (final sweep): Membership on StateDef/StateUsage ---

    #[test]
    fn state_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = state_def(input("private state def S1;")).expect("state def");
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
    fn state_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = state_def(input("state def S1;")).expect("state def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn state_usage_visibility_prefix_is_captured_on_membership() {
        let source = input("protected state s1 : $::Modes::S1;");
        let (_, node) = state_usage(source).expect("state usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Protected)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
        assert_eq!(
            node.value
                .type_name
                .and_then(|id| crate::parser::usage::reference_text(source, id))
                .as_deref(),
            Some("$::Modes::S1")
        );
    }

    #[test]
    fn state_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = state_usage(input("state s1 : S1;")).expect("state usage");
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn then_state_target_is_an_arena_backed_reference() {
        let source = input("then Modes::ready;");
        let (rest, node) = super::then_stmt(source).expect("then state");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            crate::parser::usage::reference_text(source, node.value.state_reference).as_deref(),
            Some("Modes::ready")
        );
    }

    #[test]
    fn do_action_keeps_bare_name_and_out_param() {
        let source = input("do 'sense temperature' { out temp; }");
        let (rest, node) = super::do_action(source).expect("do with bare name and out");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .action_reference
                .and_then(|id| crate::parser::usage::reference_text(source, id))
                .as_deref(),
            Some("'sense temperature'")
        );
        assert!(!node.value.has_action_keyword);
        match &node.value.body {
            crate::ast::StateDefBody::Brace { elements } => {
                assert!(matches!(
                    elements[0].value,
                    crate::ast::StateDefBodyElement::InOutDecl(_)
                ));
            }
            other => panic!("expected brace body, got {other:?}"),
        }
    }

    #[test]
    fn shorthand_accept_transition_parses_without_transition_keyword() {
        let (rest, node) = super::transition_shorthand(input("accept 'Start Signal' then on;"))
            .expect("shorthand accept transition");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.accept.is_some());
        assert!(node.value.name.is_none());
        assert!(node.value.source.is_none());
    }
}

#[cfg(test)]
mod state_body_member_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    /// Spec42 Gap 42: `attribute :>> isTriggerDuring;` (Systems Library `States.sysml`) is a
    /// typed attribute-usage member of a state body, not opaque recovery text.
    #[test]
    fn state_body_dispatches_an_attribute_redefinition_member() {
        let (rest, node) = state_def_body_element(input("attribute :>> isTriggerDuring;"))
            .expect("attribute member");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let StateDefBodyElement::AttributeUsage(usage) = node.value else {
            panic!("expected AttributeUsage, got {:?}", node.value);
        };
        assert!(usage.value.redefines.is_some());
        assert!(usage.value.name.is_empty());
    }

    /// Spec42 Gap 42: `action :>> subactions :> middle { }` declares an anonymous action usage
    /// whose leading specialization clauses stand in for the name.
    #[test]
    fn state_body_dispatches_an_anonymous_action_redefinition_member() {
        let (rest, node) = state_def_body_element(input("action :>> subactions :> middle { }"))
            .expect("action member");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let StateDefBodyElement::ActionUsage(usage) = node.value else {
            panic!("expected ActionUsage, got {:?}", node.value);
        };
        assert!(usage.value.redefines.is_some());
        assert!(usage.value.subsets.is_some());
        assert!(usage.value.name.is_empty());
    }

    /// Spec42 Gap 42: the Systems Library's per-end-multiplicity succession spelling.
    #[test]
    fn state_body_dispatches_a_succession_member() {
        let (rest, node) = state_def_body_element(input(
            "succession stateSequencing first [0..1] exclusiveStates then [0..1] exclusiveStates;",
        ))
        .expect("succession member");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let StateDefBodyElement::SuccessionUsage(usage) = node.value else {
            panic!("expected SuccessionUsage, got {:?}", node.value);
        };
        assert_eq!(usage.value.name.as_deref(), Some("stateSequencing"));
        assert!(usage.value.source_multiplicity.is_some());
        assert!(usage.value.target_multiplicity.is_some());
    }

    /// Spec42 Gap 42: `assert constraint { ... }` is a typed member of a state body.
    #[test]
    fn state_body_dispatches_an_assert_constraint_member() {
        let (rest, node) =
            state_def_body_element(input("assert constraint { notEmpty(exclusiveStates) }"))
                .expect("assert constraint member");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            StateDefBodyElement::AssertConstraint(_)
        ));
    }
}
