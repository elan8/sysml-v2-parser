#![allow(dead_code, unused_imports)]

use crate::ast::{
    EntryAction, FinalState, Node, RefBody, RefDecl, StateDef, StateDefBody, StateDefBodyElement,
    StateUsage, ThenStmt, Transition,
};
use crate::parser::body::parse_structured_brace_members;
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::expr::expression;
use crate::parser::lex::{
    identification, name, qualified_name, recover_body_element, skip_statement_or_block,
    starts_with_any_keyword, starts_with_keyword, take_until_terminator, ws1, ws_and_comments,
    STATE_BODY_STARTERS,
};

const UNTIL_BODY: &[u8] = b";{";
use crate::parser::metadata_annotation::{annotation, metadata_annotation, metadata_keyword_usage};
use crate::parser::payload::transition_accept;
use crate::parser::node_from_to;
use crate::parser::requirement::{doc_comment, requirement_usage};
use crate::parser::usage::{feature_usage_header, multiplicity, usage_header};
use crate::parser::with_span;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

pub(crate) fn state_def(input: Input<'_>) -> IResult<Input<'_>, Node<StateDef>> {
    let start = input;
    let (input, prefix) =
        parse_definition_prefix(input, DefinitionPrefixOptions::new(b"state").def_required())?;
    let (input, body) = state_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            StateDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                specializes_span: prefix.specializes_span,
                body,
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
                "missing_member_name"
                    | "missing_type_reference"
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

/// Parse `{` state-body members `}` with recovery, discarding elements (opaque ref bodies).
fn consume_state_structured_brace(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _elements) = parse_structured_brace_members(
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
                "missing_member_name"
                    | "missing_type_reference"
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
    Ok((input, ()))
}

/// Entry action: `entry` (`;` or body)  or  `entry action` name body
fn entry_action(input: Input<'_>) -> IResult<Input<'_>, Node<EntryAction>> {
    let start = input;
    let (input, _) = tag(&b"entry"[..]).parse(input)?;
    let (input, action_name) = opt((
        preceded(ws_and_comments, tag(&b"action"[..])),
        preceded(ws1, name),
    ))
    .parse(input)?;
    let action_name = action_name.map(|(_, n)| n);
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = take_until_terminator(input, UNTIL_BODY)?;
    let (input, body) = state_def_body(input)?;
    Ok((
        input,
        node_from_to(start, input, EntryAction { action_name, body }),
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

    let (input, _) = ws_and_comments(input)?;
    let (mut input, value) = opt(preceded(
        preceded(ws_and_comments, tag(&b"="[..])),
        preceded(ws_and_comments, expression),
    ))
    .parse(input)?;

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
            map(consume_state_structured_brace, |_| RefBody::Brace { elements: vec![] }),
        )),
    )
    .parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            RefDecl {
                name: name_str,
                type_name,
                value,
                body,
                name_span: Some(name_span),
                type_ref_span: Some(type_ref_span),
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
        map(transition, |n| {
            node_from_to(start, input, StateDefBodyElement::Transition(Box::new(n)))
        }),
    ));
    parser.parse(input)
}

pub(crate) fn state_usage(input: Input<'_>) -> IResult<Input<'_>, Node<StateUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"state"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, n) = name(input)?;
    let (input, header) = feature_usage_header(input)?;
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
                name: n,
                type_name: header.type_name,
                body,
            },
        ),
    ))
}

pub(crate) fn transition(input: Input<'_>) -> IResult<Input<'_>, Node<Transition>> {
    let start = input;
    let (input, _) = tag(&b"transition"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, n) = {
        let (peek, _) = ws_and_comments(input)?;
        if starts_with_keyword(peek.fragment(), b"first")
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
    // Optional: `first` source with optional `accept` trigger.
    let (input, first_clause) = opt((
        preceded(ws_and_comments, tag(&b"first"[..])),
        ws1,
        expression,
        opt(transition_accept),
    ))
    .parse(input)?;
    let (source, accept, is_initial) = match first_clause {
        // Named transitions use `first` for the source state; only unnamed transitions are initial.
        Some((_, _, src, acc)) => (Some(src), acc, n.is_none()),
        None => (None, None, false),
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
        preceded(ws1, expression),
    ))
    .parse(input)?;
    let effect = effect.map(|(_, expr)| expr);
    let (input, _) = preceded(ws_and_comments, tag(&b"then"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, target) = expression(input)?;
    let (input, body) = preceded(
        ws_and_comments,
        crate::parser::interface::connect_body,
    )
    .parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Transition {
                name: n,
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
