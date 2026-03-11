//! Package and root namespace parsing.

use crate::ast::{
    Package, PackageBody, PackageBodyElement, RootNamespace,
};
use crate::parser::import::import_;
use crate::parser::lex::{identification, ws1, ws_and_comments};
use crate::parser::part::{part_def, part_usage};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

/// Keyword "package" with following whitespace.
fn keyword_package(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag("package")(input)?;
    let (input, _) = ws1(input)?;
    Ok((input, ()))
}

/// package Identification PackageBody
fn package_(input: &[u8]) -> IResult<&[u8], Package> {
    let (input, _) = keyword_package(input)?;
    let (input, identification) = identification(input)?;
    let (input, body) = package_body(input)?;
    Ok((
        input,
        Package {
            identification,
            body,
        },
    ))
}

/// PackageBody: ';' | '{' PackageBodyElement* '}'
pub(crate) fn package_body(input: &[u8]) -> IResult<&[u8], PackageBody> {
    alt((
        map(preceded(ws_and_comments, tag(";")), |_| PackageBody::Semicolon),
        map(
            nom::sequence::delimited(
                preceded(ws_and_comments, tag("{")),
                preceded(ws_and_comments, many0(preceded(ws_and_comments, package_body_element))),
                preceded(ws_and_comments, tag("}")),
            ),
            |elements| PackageBody::Brace { elements },
        ),
    ))(input)
}

/// PackageBodyElement: Package | Import | PartDef | PartUsage
pub(crate) fn package_body_element(input: &[u8]) -> IResult<&[u8], PackageBodyElement> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(package_, PackageBodyElement::Package),
        map(import_, PackageBodyElement::Import),
        map(part_def, PackageBodyElement::PartDef),
        map(part_usage, PackageBodyElement::PartUsage),
    ))(input)
}

/// Root: PackageBodyElement*
pub(crate) fn root_namespace(input: &[u8]) -> IResult<&[u8], RootNamespace> {
    let (input, _) = ws_and_comments(input)?;
    let (input, elements) = many0(preceded(ws_and_comments, package_body_element))(input)?;
    let (input, _) = ws_and_comments(input)?;
    Ok((input, RootNamespace { elements }))
}
