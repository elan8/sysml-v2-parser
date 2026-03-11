//! Interface definition and usage parsing.

use crate::ast::{
    ConnectBody, ConnectStmt, EndDecl, InterfaceDef, InterfaceDefBody, InterfaceDefBodyElement,
    RefBody, RefDecl,
};
use crate::parser::expr::path_expression;
use crate::parser::lex::{identification, name, qualified_name, skip_until_brace_end, ws1, ws_and_comments};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

/// End declaration: `end` name `:` type `;`
fn end_decl(input: &[u8]) -> IResult<&[u8], EndDecl> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("end")(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, _) = preceded(ws_and_comments, tag(":"))(input)?;
    let (input, type_name) = preceded(ws_and_comments, qualified_name)(input)?;
    let (input, _) = preceded(ws_and_comments, tag(";"))(input)?;
    Ok((
        input,
        EndDecl {
            name: name_str,
            type_name,
        },
    ))
}

/// Ref body: `;` or `{` ... `}`
fn ref_body(input: &[u8]) -> IResult<&[u8], RefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(";"), |_| RefBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag("{"),
                skip_until_brace_end,
                preceded(ws_and_comments, tag("}")),
            ),
            |_| RefBody::Brace,
        ),
    ))(input)
}

/// Ref declaration: `ref` name `:` type body
fn ref_decl(input: &[u8]) -> IResult<&[u8], RefDecl> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("ref")(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, _) = preceded(ws_and_comments, tag(":"))(input)?;
    let (input, type_name) = preceded(ws_and_comments, qualified_name)(input)?;
    let (input, body) = ref_body(input)?;
    Ok((
        input,
        RefDecl {
            name: name_str,
            type_name,
            body,
        },
    ))
}

/// Connect body: `;` or `{` ... `}`
pub(crate) fn connect_body(input: &[u8]) -> IResult<&[u8], ConnectBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(";"), |_| ConnectBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag("{"),
                skip_until_brace_end,
                preceded(ws_and_comments, tag("}")),
            ),
            |_| ConnectBody::Brace,
        ),
    ))(input)
}

/// Connect statement: `connect` from `to` to body
fn connect_stmt(input: &[u8]) -> IResult<&[u8], ConnectStmt> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("connect")(input)?;
    let (input, _) = ws1(input)?;
    let (input, from_expr) = path_expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag("to"))(input)?;
    let (input, to_expr) = preceded(ws_and_comments, path_expression)(input)?;
    let (input, body) = connect_body(input)?;
    Ok((
        input,
        ConnectStmt {
            from: from_expr,
            to: to_expr,
            body,
        },
    ))
}

fn interface_def_body_element(input: &[u8]) -> IResult<&[u8], InterfaceDefBodyElement> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(end_decl, InterfaceDefBodyElement::EndDecl),
        map(ref_decl, InterfaceDefBodyElement::RefDecl),
        map(connect_stmt, InterfaceDefBodyElement::ConnectStmt),
    ))(input)
}

/// Interface def body: `;` or `{` InterfaceDefBodyElement* `}`
fn interface_def_body(input: &[u8]) -> IResult<&[u8], InterfaceDefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(";"), |_| InterfaceDefBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag("{"),
                preceded(
                    ws_and_comments,
                    many0(preceded(ws_and_comments, interface_def_body_element)),
                ),
                preceded(ws_and_comments, tag("}")),
            ),
            |elements| InterfaceDefBody::Brace { elements },
        ),
    ))(input)
}

/// Interface definition: `interface` `def` Identification body
pub(crate) fn interface_def(input: &[u8]) -> IResult<&[u8], InterfaceDef> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("interface")(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag("def")(input)?;
    let (input, _) = ws1(input)?;
    let (input, identification) = identification(input)?;
    let (input, body) = interface_def_body(input)?;
    Ok((
        input,
        InterfaceDef {
            identification,
            body,
        },
    ))
}
