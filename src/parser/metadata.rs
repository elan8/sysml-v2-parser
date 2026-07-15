//! Metadata definition and usage parsing (BNF MetadataDefinition / MetadataUsage).

use crate::ast::{MetadataDef, MetadataUsage, Node};
use crate::parser::attribute::metadata_body;
use crate::parser::definition_header::parse_feature_usage_header;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::lex::{name, starts_with_keyword, ws1, ws_and_comments};
use crate::parser::metadata_annotation::parse_about_targets;
use crate::parser::node_from_to;
use crate::parser::Input;
use nom::bytes::complete::tag;
use nom::error::{Error, ErrorKind};
use nom::IResult;
use nom::Parser;

/// Metadata definition: `metadata def` Identification body (optional `abstract` prefix).
pub(crate) fn metadata_def(input: Input<'_>) -> IResult<Input<'_>, Node<MetadataDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"metadata").def_required(),
    )?;
    let (input, body) = metadata_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            MetadataDef {
                is_abstract: prefix.is_abstract,
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
            },
        ),
    ))
}

/// Metadata usage: `metadata` name (`:` type)? body.
pub(crate) fn metadata_usage(input: Input<'_>) -> IResult<Input<'_>, Node<MetadataUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"metadata"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    if starts_with_keyword(input.fragment(), b"def") {
        return Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)));
    }
    let (input, name) = name(input)?;
    let (input, header) = parse_feature_usage_header(input)?;
    let (input, about_targets) = parse_about_targets(input)?;
    let (input, body) = metadata_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            MetadataUsage {
                name,
                type_name: header.type_name,
                about_targets,
                body,
            },
        ),
    ))
}
