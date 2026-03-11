//! Port definition and port usage parsing.

use crate::ast::{PortBody, PortDef, PortDefBody, PortDefBodyElement, PortUsage};
use crate::parser::expr::expression;
use crate::parser::lex::{identification, name, qualified_name, ws1, ws_and_comments};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Port body: ';' or '{' PortUsage* '}' (nested ports) or '{' skip '}' for Brace only.
fn port_body(input: &[u8]) -> IResult<&[u8], PortBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(";"), |_| PortBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag("{"),
                preceded(
                    ws_and_comments,
                    many0(preceded(ws_and_comments, port_usage)),
                ),
                preceded(ws_and_comments, tag("}")),
            ),
            |elements| {
                if elements.is_empty() {
                    PortBody::Brace
                } else {
                    PortBody::BraceWithPorts { elements }
                }
            },
        ),
    ))(input)
}

/// Port usage: 'port' name ( ':' type )? multiplicity? ( ':>' name ( '=' expr )? )? ( 'redefines' qualified_name )? body
pub(crate) fn port_usage(input: &[u8]) -> IResult<&[u8], PortUsage> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("port")(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, type_name) = opt(preceded(
        preceded(ws_and_comments, tag(":")),
        preceded(ws_and_comments, qualified_name),
    ))(input)?;
    let (input, multiplicity) = opt(multiplicity)(input)?;
    let (input, subsets) = opt(preceded(
        preceded(ws_and_comments, tag(":>")),
        preceded(ws_and_comments, tuple((
            name,
            opt(preceded(
                preceded(ws_and_comments, tag("=")),
                preceded(ws_and_comments, expression),
            )),
        ))),
    ))(input)?;
    let (input, redefines) = opt(preceded(
        preceded(ws_and_comments, tag("redefines")),
        preceded(ws1, qualified_name),
    ))(input)?;
    let (input, body) = port_body(input)?;
    Ok((
        input,
        PortUsage {
            name: name_str,
            type_name,
            multiplicity,
            subsets: subsets.map(|(feat, val)| (feat, val)),
            redefines,
            body,
        },
    ))
}

fn multiplicity(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, content) = take_until("]")(input)?;
    let (input, _) = tag("]")(input)?;
    let s = format!("[{}]", String::from_utf8_lossy(content).trim());
    Ok((input, s))
}

/// Port def body: ';' or '{' PortDefBodyElement* '}'
fn port_def_body(input: &[u8]) -> IResult<&[u8], PortDefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(";"), |_| PortDefBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag("{"),
                preceded(
                    ws_and_comments,
                    many0(preceded(ws_and_comments, port_def_body_element)),
                ),
                preceded(ws_and_comments, tag("}")),
            ),
            |elements| PortDefBody::Brace { elements },
        ),
    ))(input)
}

fn port_def_body_element(input: &[u8]) -> IResult<&[u8], PortDefBodyElement> {
    map(port_usage, PortDefBodyElement::PortUsage)(input)
}

/// Port definition: 'port' 'def' Identification body
pub(crate) fn port_def(input: &[u8]) -> IResult<&[u8], PortDef> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("port")(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag("def")(input)?;
    let (input, _) = ws1(input)?;
    let (input, identification) = identification(input)?;
    let (input, body) = port_def_body(input)?;
    Ok((
        input,
        PortDef {
            identification,
            body,
        },
    ))
}
