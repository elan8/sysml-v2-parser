//! Interface definition and usage parsing.

use crate::ast::{
    ConnectBody, ConnectStmt, EndDecl, InterfaceDef, InterfaceDefBody, InterfaceDefBodyElement,
    Node, RefBody, RefDecl,
};
use crate::parser::expr::path_expression;
use crate::parser::lex::{identification, name, qualified_name, skip_until_brace_end, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

/// End declaration: `end` name `:` type `;`
fn end_decl(input: Input<'_>) -> IResult<Input<'_>, Node<EndDecl>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(b"end")(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, _) = preceded(ws_and_comments, tag(b":"))(input)?;
    let (input, type_name) = preceded(ws_and_comments, qualified_name)(input)?;
    let (input, _) = preceded(ws_and_comments, tag(b";"))(input)?;
    Ok((
        input,
        node_from_to(start, input, EndDecl {
            name: name_str,
            type_name,
        }),
    ))
}

/// Ref body: `;` or `{` ... `}`
fn ref_body(input: Input<'_>) -> IResult<Input<'_>, RefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(b";"), |_| RefBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag(b"{"),
                skip_until_brace_end,
                preceded(ws_and_comments, tag(b"}")),
            ),
            |_| RefBody::Brace,
        ),
    ))(input)
}

/// Ref declaration: `ref` name `:` type body
fn ref_decl(input: Input<'_>) -> IResult<Input<'_>, Node<RefDecl>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(b"ref")(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, _) = preceded(ws_and_comments, tag(b":"))(input)?;
    let (input, type_name) = preceded(ws_and_comments, qualified_name)(input)?;
    let (input, body) = ref_body(input)?;
    Ok((
        input,
        node_from_to(start, input, RefDecl {
            name: name_str,
            type_name,
            body,
        }),
    ))
}

/// Connect body: `;` or `{` ... `}`
pub(crate) fn connect_body(input: Input<'_>) -> IResult<Input<'_>, ConnectBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(b";"), |_| ConnectBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag(b"{"),
                skip_until_brace_end,
                preceded(ws_and_comments, tag(b"}")),
            ),
            |_| ConnectBody::Brace,
        ),
    ))(input)
}

/// Connect statement: `connect` from `to` to body
fn connect_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<ConnectStmt>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(b"connect")(input)?;
    let (input, _) = ws1(input)?;
    let (input, from_expr) = path_expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag(b"to"))(input)?;
    let (input, to_expr) = preceded(ws_and_comments, path_expression)(input)?;
    let (input, body) = connect_body(input)?;
    Ok((
        input,
        node_from_to(start, input, ConnectStmt {
            from: from_expr,
            to: to_expr,
            body,
        }),
    ))
}

fn interface_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<InterfaceDefBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, elem) = alt((
        map(end_decl, InterfaceDefBodyElement::EndDecl),
        map(ref_decl, InterfaceDefBodyElement::RefDecl),
        map(connect_stmt, InterfaceDefBodyElement::ConnectStmt),
    ))(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

/// Interface def body: `;` or `{` InterfaceDefBodyElement* `}`
fn interface_def_body(input: Input<'_>) -> IResult<Input<'_>, InterfaceDefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(b";"), |_| InterfaceDefBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag(b"{"),
                preceded(
                    ws_and_comments,
                    many0(preceded(ws_and_comments, interface_def_body_element)),
                ),
                preceded(ws_and_comments, tag(b"}")),
            ),
            |elements| InterfaceDefBody::Brace { elements },
        ),
    ))(input)
}

/// Interface definition: `interface` `def` Identification body
pub(crate) fn interface_def(input: Input<'_>) -> IResult<Input<'_>, Node<InterfaceDef>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(b"interface")(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(b"def")(input)?;
    let (input, _) = ws1(input)?;
    let (input, identification) = identification(input)?;
    let (input, body) = interface_def_body(input)?;
    Ok((
        input,
        node_from_to(start, input, InterfaceDef {
            identification,
            body,
        }),
    ))
}
