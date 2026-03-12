use crate::ast::{
    EntryAction, Node, StateDef, StateDefBody, StateDefBodyElement, StateUsage, ThenStmt, Transition,
};
use crate::parser::requirement::doc_comment;
use crate::parser::expr::expression;
use crate::parser::lex::{identification, name, ws1, ws_and_comments, qualified_name};
use crate::parser::node_from_to;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

fn keyword_state_def(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) = tag(&b"state"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(&b"def"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    Ok((input, ()))
}

pub(crate) fn state_def(input: Input<'_>) -> IResult<Input<'_>, Node<StateDef>> {
    let start = input;
    let (input, _) = keyword_state_def(input)?;
    let (input, ident) = identification(input)?;
    let (input, body) = state_def_body(input)?;
    Ok((input, node_from_to(start, input, StateDef { identification: ident, body })))
}

fn state_def_body(input: Input<'_>) -> IResult<Input<'_>, StateDefBody> {
    alt((
        map(preceded(ws_and_comments, tag(&b";"[..])), |_| StateDefBody::Semicolon),
        map(
            delimited(
                preceded(ws_and_comments, tag(&b"{"[..])),
                many0(preceded(ws_and_comments, state_def_body_element)),
                preceded(ws_and_comments, tag(&b"}"[..])),
            ),
            |elements| StateDefBody::Brace { elements },
        ),
    ))
    .parse(input)
}

/// Entry action: `entry` (`;` or body)
fn entry_action(input: Input<'_>) -> IResult<Input<'_>, Node<EntryAction>> {
    let start = input;
    let (input, _) = tag(&b"entry"[..]).parse(input)?;
    let (input, body) = state_def_body(input)?;
    Ok((input, node_from_to(start, input, EntryAction { body })))
}

/// Then (initial state): `then` name `;`
fn then_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<ThenStmt>> {
    let start = input;
    let (input, _) = tag(&b"then"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, state_name) = name(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((input, node_from_to(start, input, ThenStmt { state_name })))
}

fn state_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<StateDefBodyElement>> {
    let start = input;
    let mut parser = alt((
        map(doc_comment, |n| node_from_to(start, input, StateDefBodyElement::Doc(n))),
        map(entry_action, |n| node_from_to(start, input, StateDefBodyElement::Entry(n))),
        map(then_stmt, |n| node_from_to(start, input, StateDefBodyElement::Then(n))),
        map(state_usage, |n| node_from_to(start, input, StateDefBodyElement::StateUsage(n))),
        map(transition, |n| node_from_to(start, input, StateDefBodyElement::Transition(n))),
    ));
    parser.parse(input)
}

pub(crate) fn state_usage(input: Input<'_>) -> IResult<Input<'_>, Node<StateUsage>> {
    let start = input;
    let (input, _) = tag(&b"state"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, n) = name(input)?;
    let (input, typ) = opt(preceded(preceded(ws_and_comments, tag(&b":"[..])), preceded(ws_and_comments, qualified_name))).parse(input)?;
    let (input, body) = state_def_body(input)?;
    Ok((input, node_from_to(start, input, StateUsage { name: n, type_name: typ, body })))
}

pub(crate) fn transition(input: Input<'_>) -> IResult<Input<'_>, Node<Transition>> {
    let start = input;
    let (input, _) = tag(&b"transition"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, n) = name(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"first"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, source) = expression(input)?;
    // Optional: `accept` trigger expression (e.g. `accept PhaseTimerElapsed`)
    let (input, _) = opt((
        preceded(ws_and_comments, tag(&b"accept"[..])),
        preceded(ws1, expression),
    ))
    .parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"then"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, target) = expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((input, node_from_to(start, input, Transition { name: n, source, target, body: crate::ast::ConnectBody::Semicolon })))
}
