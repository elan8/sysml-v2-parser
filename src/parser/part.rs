//! Part definition and part usage parsing.

use crate::ast::{
    PartDef, PartDefBody, PartDefBodyElement, PartUsage, PartUsageBody, PartUsageBodyElement,
};
use crate::parser::attribute::{attribute_def, attribute_usage};
use crate::parser::lex::{
    identification, name, qualified_name, take_until_terminator, ws1, ws_and_comments,
};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Part def body: ';' or '{' PartDefBodyElement* '}'
pub(crate) fn part_def_body(input: &[u8]) -> IResult<&[u8], PartDefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(";"), |_| PartDefBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag("{"),
                preceded(
                    ws_and_comments,
                    many0(preceded(ws_and_comments, part_def_body_element)),
                ),
                preceded(ws_and_comments, tag("}")),
            ),
            |elements| PartDefBody::Brace { elements },
        ),
    ))(input)
}

fn part_def_body_element(input: &[u8]) -> IResult<&[u8], PartDefBodyElement> {
    map(attribute_def, PartDefBodyElement::AttributeDef)(input)
}

/// Part definition: 'part' 'def' Identification ( ':>' qualified_name )? body
pub(crate) fn part_def(input: &[u8]) -> IResult<&[u8], PartDef> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("part")(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag("def")(input)?;
    let (input, _) = ws1(input)?;
    let (input, identification) = identification(input)?;
    let (input, specializes) = opt(preceded(
        preceded(ws_and_comments, tag(":>")),
        preceded(ws_and_comments, qualified_name),
    ))(input)?;
    let (input, body) = part_def_body(input)?;
    Ok((
        input,
        PartDef {
            identification,
            specializes,
            body,
        },
    ))
}

/// Multiplicity: '[' ... ']' as string
fn multiplicity(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, content) = take_until("]")(input)?;
    let (input, _) = tag("]")(input)?;
    let s = format!("[{}]", String::from_utf8_lossy(content).trim());
    Ok((input, s))
}

/// Part usage: 'part' name ':' type_name multiplicity? 'ordered'? ( 'subsets' name '=' value )? body
pub(crate) fn part_usage(input: &[u8]) -> IResult<&[u8], PartUsage> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("part")(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, type_name) = opt(preceded(
        preceded(ws_and_comments, tag(":")),
        preceded(ws_and_comments, qualified_name),
    ))(input)?;
    let (input, multiplicity_opt) = opt(multiplicity)(input)?;
    let (input, ordered) = opt(preceded(ws_and_comments, tag("ordered")))(input)?;
    let (input, subsets) = opt(preceded(
        preceded(ws_and_comments, tag("subsets")),
        preceded(ws1, tuple((
            name,
            opt(preceded(
                preceded(ws_and_comments, tag("=")),
                preceded(ws_and_comments, |i| take_until_terminator(i, b"{;")),
            )),
        ))),
    ))(input)?;
    let (input, body) = part_usage_body(input)?;
    let subsets = subsets.map(|(feat, val)| (feat, val.filter(|s| !s.is_empty())));
    Ok((
        input,
        PartUsage {
            name: name_str,
            type_name: type_name.unwrap_or_else(String::new),
            multiplicity: multiplicity_opt,
            ordered: ordered.is_some(),
            subsets,
            body,
        },
    ))
}

/// Part usage body: ';' or '{' PartUsageBodyElement* '}'
fn part_usage_body(input: &[u8]) -> IResult<&[u8], PartUsageBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(";"), |_| PartUsageBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag("{"),
                preceded(
                    ws_and_comments,
                    many0(preceded(ws_and_comments, part_usage_body_element)),
                ),
                preceded(ws_and_comments, tag("}")),
            ),
            |elements| PartUsageBody::Brace { elements },
        ),
    ))(input)
}

fn part_usage_body_element(input: &[u8]) -> IResult<&[u8], PartUsageBodyElement> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(attribute_usage, PartUsageBodyElement::AttributeUsage),
        map(part_usage, |p| PartUsageBodyElement::PartUsage(Box::new(p))),
    ))(input)
}
