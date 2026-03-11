//! Part definition and part usage parsing.

use crate::ast::{
    Bind, Connect, ConnectBody, InterfaceUsage, InterfaceUsageBodyElement, PartDef, PartDefBody,
    PartDefBodyElement, PartUsage, PartUsageBody, PartUsageBodyElement, RefBody,
};
use crate::parser::attribute::{attribute_def, attribute_usage};
use crate::parser::expr::{expression, path_expression};
use crate::parser::interface::connect_body;
use crate::parser::lex::{identification, name, qualified_name, skip_until_brace_end, ws1, ws_and_comments};
use crate::parser::port::port_usage;
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
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(port_usage, PartDefBodyElement::PortUsage),
        map(attribute_def, PartDefBodyElement::AttributeDef),
    ))(input)
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
    let (input, subsets) = opt(alt((
        preceded(
            preceded(ws_and_comments, tag("subsets")),
            preceded(ws1, tuple((
                name,
                opt(preceded(
                    preceded(ws_and_comments, tag("=")),
                    preceded(ws_and_comments, expression),
                )),
            ))),
        ),
        preceded(
            preceded(ws_and_comments, tag(":>")),
            preceded(ws_and_comments, tuple((
                name,
                opt(preceded(
                    preceded(ws_and_comments, tag("=")),
                    preceded(ws_and_comments, expression),
                )),
            ))),
        ),
    )))(input)?;
    let (input, body) = part_usage_body(input)?;
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

/// Bind: `bind` path `=` path (`;` or `{ }`)
pub(crate) fn bind_(input: &[u8]) -> IResult<&[u8], Bind> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("bind")(input)?;
    let (input, _) = ws1(input)?;
    let (input, left) = path_expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag("="))(input)?;
    let (input, right) = preceded(ws_and_comments, path_expression)(input)?;
    let (input, body) = alt((
        map(preceded(ws_and_comments, tag(";")), |_| Some(ConnectBody::Semicolon)),
        map(
            nom::sequence::delimited(
                preceded(ws_and_comments, tag("{")),
                skip_until_brace_end,
                preceded(ws_and_comments, tag("}")),
            ),
            |_| Some(ConnectBody::Brace),
        ),
    ))(input)?;
    Ok((input, Bind { left, right, body }))
}

/// Connect (part usage level): `connect` path `to` path body
fn connect_(input: &[u8]) -> IResult<&[u8], Connect> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("connect")(input)?;
    let (input, _) = ws1(input)?;
    let (input, from_expr) = path_expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag("to"))(input)?;
    let (input, to_expr) = preceded(ws_and_comments, path_expression)(input)?;
    let (input, body) = connect_body(input)?;
    Ok((
        input,
        Connect {
            from: from_expr,
            to: to_expr,
            body,
        },
    ))
}

/// Interface usage body elements: `ref` `:>>` name `=` value body (RefRedef)
fn interface_usage_body_element(input: &[u8]) -> IResult<&[u8], InterfaceUsageBodyElement> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("ref")(input)?;
    let (input, _) = preceded(ws_and_comments, tag(":>>"))(input)?;
    let (input, ref_name) = preceded(ws_and_comments, name)(input)?;
    let (input, _) = preceded(ws_and_comments, tag("="))(input)?;
    let (input, value) = preceded(ws_and_comments, expression)(input)?;
    let (input, body) = ref_body_parse(input)?;
    Ok((
        input,
        InterfaceUsageBodyElement::RefRedef {
            name: ref_name,
            value,
            body,
        },
    ))
}

fn ref_body_parse(input: &[u8]) -> IResult<&[u8], RefBody> {
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

/// Connect body for interface usage (TypedConnect): `;` or `{` body_elements* `}`
fn connect_body_with_elements(input: &[u8]) -> IResult<&[u8], (ConnectBody, Vec<InterfaceUsageBodyElement>)> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(";"), |_| (ConnectBody::Semicolon, vec![])),
        map(
            nom::sequence::delimited(
                tag("{"),
                preceded(
                    ws_and_comments,
                    many0(preceded(ws_and_comments, interface_usage_body_element)),
                ),
                preceded(ws_and_comments, tag("}")),
            ),
            |elements| (ConnectBody::Brace, elements),
        ),
    ))(input)
}

/// Interface usage: `interface` ( `:Type` )? `connect` path `to` path body  OR  `interface` path `to` path body?
fn interface_usage(input: &[u8]) -> IResult<&[u8], InterfaceUsage> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("interface")(input)?;
    let (input, _) = ws1(input)?;
    let (input, interface_type) = opt(preceded(
        tag(":"),
        preceded(ws_and_comments, qualified_name),
    ))(input)?;
    let (input, _) = ws_and_comments(input)?;
    if input.starts_with(b"connect") {
        let (input, _) = tag("connect")(input)?;
        let (input, _) = ws1(input)?;
        let (input, from_expr) = path_expression(input)?;
        let (input, _) = preceded(ws_and_comments, tag("to"))(input)?;
        let (input, to_expr) = preceded(ws_and_comments, path_expression)(input)?;
        let (input, (body, body_elements)) = connect_body_with_elements(input)?;
        Ok((
            input,
            InterfaceUsage::TypedConnect {
                interface_type,
                from: from_expr,
                to: to_expr,
                body,
                body_elements,
            },
        ))
    } else {
        let (input, from_expr) = path_expression(input)?;
        let (input, _) = preceded(ws_and_comments, tag("to"))(input)?;
        let (input, to_expr) = preceded(ws_and_comments, path_expression)(input)?;
        let (input, _) = opt(connect_body)(input)?;
        Ok((
            input,
            InterfaceUsage::Connection {
                from: from_expr,
                to: to_expr,
                body_elements: vec![],
            },
        ))
    }
}

fn part_usage_body_element(input: &[u8]) -> IResult<&[u8], PartUsageBodyElement> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(attribute_usage, PartUsageBodyElement::AttributeUsage),
        map(part_usage, |p| PartUsageBodyElement::PartUsage(Box::new(p))),
        map(port_usage, PartUsageBodyElement::PortUsage),
        map(bind_, PartUsageBodyElement::Bind),
        map(interface_usage, PartUsageBodyElement::InterfaceUsage),
        map(connect_, PartUsageBodyElement::Connect),
    ))(input)
}
