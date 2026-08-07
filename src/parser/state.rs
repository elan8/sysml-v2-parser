use crate::ast::{
    DoAction, EntryAction, ExitAction, FinalState, Membership, Node, RefBody, RefBodyElement,
    RefDecl, StateDef, StateDefBody, StateDefBodyElement, StateUsage, ThenStmt, Transition,
    TransitionEffect,
};
use crate::parser::body::{advance_to_closing_brace, parse_structured_brace_members};
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::expr::expression;
use crate::parser::lex::{
    name, qualified_name, starts_with_keyword, take_until_terminator, visibility_prefix, ws1,
    ws_and_comments, STATE_BODY_STARTERS,
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
            .with_captured_visibility(),
    )?;
    let (input, body) = state_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            StateDef {
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
                    StateDefBodyElement::Error(Node::new(crate::ast::Span::dummy(), recovery)),
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

/// Parse `{` state-body members `}` with recovery.
fn consume_state_structured_brace(
    input: Input<'_>,
) -> IResult<Input<'_>, Vec<Node<StateDefBodyElement>>> {
    parse_structured_brace_members(
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
                    StateDefBodyElement::Error(Node::new(crate::ast::Span::dummy(), recovery)),
                )
            } else {
                let frag = start.fragment();
                let take = frag.len().min(80);
                let preview = String::from_utf8_lossy(&frag[..take]).trim().to_string();
                node_from_to(start, end, StateDefBodyElement::Other(preview))
            }
        },
    )
}

/// Shared `entry`/`do`/`exit` header: optional `action` keyword + optional referenced name.
fn state_behavior_action_target(input: Input<'_>) -> IResult<Input<'_>, (bool, Option<String>)> {
    let (input, has_action_keyword) = opt(preceded(ws_and_comments, tag(&b"action"[..])))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") || input.fragment().starts_with(b"{") {
        return Ok((input, (has_action_keyword, None)));
    }
    // Bare referenced action usage: `do 'sense temperature' { … }` / `entry initial;`.
    // When `action` was written, the name is required by the grammar; when not, still try a name
    // before the body terminator (do not swallow transition effects like `do send …`).
    if !has_action_keyword
        && (starts_with_keyword(input.fragment(), b"send")
            || starts_with_keyword(input.fragment(), b"accept")
            || starts_with_keyword(input.fragment(), b"assign"))
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, action_name) = name(input)?;
    Ok((input, (has_action_keyword, Some(action_name))))
}

/// Entry action: `entry` (`;` or body)  or  `entry action` name body / `entry` name body
fn entry_action(input: Input<'_>) -> IResult<Input<'_>, Node<EntryAction>> {
    let start = input;
    let (input, _) = tag(&b"entry"[..]).parse(input)?;
    let (input, (has_action_keyword, action_name)) = state_behavior_action_target(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = state_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            EntryAction {
                action_name,
                has_action_keyword,
                body,
            },
        ),
    ))
}

/// Do action: `do` (`;` or body)  or  `do action` name body / `do` name body
fn do_action(input: Input<'_>) -> IResult<Input<'_>, Node<DoAction>> {
    let start = input;
    let (input, _) = tag(&b"do"[..]).parse(input)?;
    let (input, (has_action_keyword, action_name)) = state_behavior_action_target(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = state_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            DoAction {
                action_name,
                has_action_keyword,
                body,
            },
        ),
    ))
}

/// Exit action: `exit` (`;` or body)  or  `exit action` name body / `exit` name body
fn exit_action(input: Input<'_>) -> IResult<Input<'_>, Node<ExitAction>> {
    let start = input;
    let (input, _) = tag(&b"exit"[..]).parse(input)?;
    let (input, (has_action_keyword, action_name)) = state_behavior_action_target(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = state_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ExitAction {
                action_name,
                has_action_keyword,
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
    let (input, (type_ref_span, type_name)) = if uses_shift {
        (input, (crate::ast::Span::dummy(), String::new()))
    } else {
        preceded(ws_and_comments, with_span(qualified_name)).parse(input)?
    };
    let typing = if type_name.is_empty() {
        None
    } else {
        Some(crate::parser::usage::single_target_typing(
            type_ref_span.clone(),
            type_name.clone(),
        ))
    };

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

    let (input, body) = preceded(
        ws_and_comments,
        alt((
            map(tag(&b";"[..]), |_| RefBody::Semicolon),
            map(consume_state_structured_brace, |elements| RefBody::Brace {
                elements: elements
                    .into_iter()
                    .map(|e| {
                        let span = e.span.clone();
                        Node::new(span, RefBodyElement::State(e))
                    })
                    .collect(),
            }),
        )),
    )
    .parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            RefDecl {
                direction: None,
                name: name_str,
                type_name,
                typing,
                redefines: None,
                subsets: None,
                value,
                body,
                name_span: Some(name_span),
                type_ref_span: Some(type_ref_span),
                membership: Membership::feature(None, crate::ast::Span::dummy()),
            },
        ),
    ))
}

/// Then (initial state): `then` name `;`
fn then_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<ThenStmt>> {
    let start = input;
    let (input, _) = tag(&b"then"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, (name_span, state_name)) = with_span(name).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ThenStmt {
                state_name,
                name_span: Some(name_span),
            },
        ),
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
            node_from_to(start, input, StateDefBodyElement::Ref(n))
        }),
        map(requirement_usage, |n| {
            node_from_to(start, input, StateDefBodyElement::RequirementUsage(n))
        }),
        map(state_usage, |n| {
            node_from_to(start, input, StateDefBodyElement::StateUsage(n))
        }),
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
        crate::parser::usage::typing_fields_from_result(type_result);
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
                type_name: if type_name.is_empty() {
                    None
                } else {
                    Some(type_name)
                },
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
fn transition_effect_type_suffix(input: Input<'_>) -> IResult<Input<'_>, Option<String>> {
    opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, qualified_name),
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
mod membership_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
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
        let (_, node) = state_usage(input("protected state s1 : S1;")).expect("state usage");
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
    fn state_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = state_usage(input("state s1 : S1;")).expect("state usage");
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn do_action_keeps_bare_name_and_out_param() {
        let (rest, node) = super::do_action(input("do 'sense temperature' { out temp; }"))
            .expect("do with bare name and out");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.action_name.as_deref(), Some("sense temperature"));
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
