//! Package and root namespace parsing.

use crate::ast::{
    Node, Package, PackageBody, PackageBodyElement, RootNamespace,
};
use crate::parser::action::{action_def, action_usage};
use crate::parser::alias::alias_def;
use crate::parser::attribute::attribute_def;
use crate::parser::import::import_;
use crate::parser::interface::interface_def;
use crate::parser::lex::{identification, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::part::{part_def, part_usage};
use crate::parser::port::port_def;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

/// Keyword "package" with following whitespace.
fn keyword_package(input: Input<'_>) -> IResult<Input<'_>, ()> {
    log::debug!("keyword_package: input len={}", input.fragment().len());
    let (input, _) = tag(b"package")(input)?;
    log::debug!("keyword_package: after tag, rest len={}", input.fragment().len());
    let (input, _) = ws1(input)?;
    Ok((input, ()))
}

/// package Identification PackageBody
fn package_(input: Input<'_>) -> IResult<Input<'_>, Node<Package>> {
    let start = input;
    let (input, _) = keyword_package(input)?;
    let (input, identification) = identification(input)?;
    let (input, body) = package_body(input)?;
    Ok((
        input,
        node_from_to(start, input, Package {
            identification,
            body,
        }),
    ))
}

/// PackageBody: ';' | '{' PackageBodyElement* '}'
pub(crate) fn package_body(input: Input<'_>) -> IResult<Input<'_>, PackageBody> {
    alt((
        map(preceded(ws_and_comments, tag(b";")), |_| PackageBody::Semicolon),
        map(
            nom::sequence::delimited(
                preceded(ws_and_comments, tag(b"{")),
                preceded(ws_and_comments, many0(preceded(ws_and_comments, package_body_element))),
                preceded(ws_and_comments, tag(b"}")),
            ),
            |elements| PackageBody::Brace { elements },
        ),
    ))(input)
}

/// PackageBodyElement: Package | Import | PartDef | PartUsage | PortDef | InterfaceDef | AliasDef | ActionDef | ActionUsage
pub(crate) fn package_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PackageBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let frag = start.fragment();
    log::debug!(
        "package_body_element: first 20 bytes: {:?}",
        frag.get(..20.min(frag.len())).unwrap_or(frag),
    );
    let (input, elem) = alt((
        map(package_, PackageBodyElement::Package),
        map(import_, PackageBodyElement::Import),
        map(part_def, PackageBodyElement::PartDef),
        map(part_usage, PackageBodyElement::PartUsage),
        map(port_def, PackageBodyElement::PortDef),
        map(interface_def, PackageBodyElement::InterfaceDef),
        map(alias_def, PackageBodyElement::AliasDef),
        map(attribute_def, PackageBodyElement::AttributeDef),
        map(action_def, PackageBodyElement::ActionDef),
        map(action_usage, PackageBodyElement::ActionUsage),
    ))(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

/// Root: PackageBodyElement*
pub(crate) fn root_namespace(input: Input<'_>) -> IResult<Input<'_>, RootNamespace> {
    let (input, _) = ws_and_comments(input)?;
    let (input, elements) = many0(preceded(ws_and_comments, package_body_element))(input)?;
    let (input, _) = ws_and_comments(input)?;
    Ok((input, RootNamespace { elements }))
}
