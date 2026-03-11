//! Attribute definition and usage parsing.

use crate::ast::{AttributeBody, AttributeDef, AttributeUsage};
use crate::parser::expr::expression;
use crate::parser::lex::{
    name, qualified_name, skip_until_brace_end, ws1, ws_and_comments,
};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::{delimited, preceded};
use nom::IResult;

/// Attribute body: ';' or '{' ... '}' (skip content inside braces)
pub(crate) fn attribute_body(input: &[u8]) -> IResult<&[u8], AttributeBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(";"), |_| AttributeBody::Semicolon),
        map(
            delimited(tag("{"), skip_until_brace_end, preceded(ws_and_comments, tag("}"))),
            |_| AttributeBody::Brace,
        ),
    ))(input)
}

/// Attribute definition: 'attribute' name ( ':>' | ':' )? qualified_name? body
pub(crate) fn attribute_def(input: &[u8]) -> IResult<&[u8], AttributeDef> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("attribute")(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = nom::combinator::opt(preceded(tag("def"), ws1))(input)?;
    let (input, name_str) = name(input)?;
    let (input, typing) = nom::combinator::opt(alt((
        preceded(preceded(ws_and_comments, tag(":>")), preceded(ws_and_comments, qualified_name)),
        preceded(preceded(ws_and_comments, tag(":")), preceded(ws_and_comments, qualified_name)),
    )))(input)?;
    let (input, body) = attribute_body(input)?;
    Ok((
        input,
        AttributeDef {
            name: name_str,
            typing,
            body,
        },
    ))
}

/// Attribute usage: 'attribute' name ( 'redefines' qualified_name )? ( '=' value )? body
pub(crate) fn attribute_usage(input: &[u8]) -> IResult<&[u8], AttributeUsage> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("attribute")(input)?;
    let (input, _) = super::lex::ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, redefines) = nom::combinator::opt(preceded(
        preceded(ws_and_comments, tag("redefines")),
        preceded(ws1, qualified_name),
    ))(input)?;
    let (input, value) = nom::combinator::opt(preceded(
        preceded(ws_and_comments, tag("=")),
        preceded(ws_and_comments, expression),
    ))(input)?;
    let (input, body) = attribute_body(input)?;
    Ok((
        input,
        AttributeUsage {
            name: name_str,
            redefines,
            value,
            body,
        },
    ))
}
