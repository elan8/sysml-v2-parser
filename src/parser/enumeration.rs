//! Enumeration definition parsing (BNF EnumerationDefinition).

use crate::ast::{EnumDef, EnumerationBody, EnumerationUsage, Node};
use crate::parser::attribute::attribute_body;
use crate::parser::body::advance_to_closing_brace;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::lex::{name, take_until_terminator, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::requirement::{comment_annotation, doc_comment};
use crate::parser::usage::{feature_usage_header, multiplicity_node};
use crate::parser::Input;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use nom::Parser;

/// Enumerated value: optional `enum` keyword + name + `;`
fn enumerated_value(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = nom::combinator::opt(preceded(tag(&b"enum"[..]), ws1)).parse(input)?;
    let (input, n) = name(input)?;
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b"{") {
        let (input, _) = delimited(
            tag(&b"{"[..]),
            advance_to_closing_brace,
            preceded(ws_and_comments, tag(&b"}"[..])),
        )
        .parse(input)?;
        Ok((input, n))
    } else {
        let (input, _) = opt(preceded(
            preceded(ws_and_comments, tag(&b"="[..])),
            preceded(ws_and_comments, |i| take_until_terminator(i, b";")),
        ))
        .parse(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
        Ok((input, n))
    }
}

fn enumeration_body(input: Input<'_>) -> IResult<Input<'_>, EnumerationBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, EnumerationBody::Semicolon));
    }
    let (mut input, _) = tag(&b"{"[..]).parse(input)?;
    let mut values = Vec::new();
    loop {
        let (next, _) = ws_and_comments(input)?;
        input = next;
        if input.fragment().starts_with(b"}") {
            let (input, _) = preceded(ws_and_comments, tag(&b"}"[..])).parse(input)?;
            return Ok((input, EnumerationBody::Brace { values }));
        }
        if let Ok((next, _)) = doc_comment(input) {
            input = next;
            continue;
        }
        if let Ok((next, _)) = comment_annotation(input) {
            input = next;
            continue;
        }
        match enumerated_value(input) {
            Ok((next, value)) => {
                values.push(value);
                input = next;
            }
            Err(_) => {
                let (input, _) = advance_to_closing_brace(input)?;
                let (input, _) = preceded(ws_and_comments, tag(&b"}"[..])).parse(input)?;
                return Ok((input, EnumerationBody::Brace { values }));
            }
        }
    }
}

/// Enumeration definition: `enum def` Identification EnumerationBody.
pub(crate) fn enum_def(input: Input<'_>) -> IResult<Input<'_>, Node<EnumDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"enum")
            .def_required()
            .no_abstract(),
    )?;
    let (input, body) = enumeration_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            EnumDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                specializes_span: prefix.specializes_span,
                body,
            },
        ),
    ))
}

/// Enumeration usage in a definition or usage body: `enum` name multiplicity? (`:` type)? body.
pub(crate) fn enum_usage(input: Input<'_>) -> IResult<Input<'_>, Node<EnumerationUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"enum"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, name) = name(input)?;
    let (input, multiplicity) = opt(multiplicity_node).parse(input)?;
    let (input, header) = feature_usage_header(input)?;
    let (input, body) = attribute_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            EnumerationUsage {
                name,
                type_name: header.type_name,
                multiplicity,
                body,
            },
        ),
    ))
}
