use super::body::part_def_body;
use super::prelude::*;
use super::usage::{part_usage_named, part_usage_redefines_only};

/// Part definition: ( 'abstract' | 'variation' )? 'part' 'def' Identification ( (':>' | 'specializes') qualified_name )? body
pub(crate) fn part_def(input: Input<'_>) -> IResult<Input<'_>, Node<PartDef>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, definition_prefix) = opt(alt((
        map(preceded(tag(&b"abstract"[..]), ws1), |_| {
            DefinitionPrefix::Abstract
        }),
        map(preceded(tag(&b"variation"[..]), ws1), |_| {
            DefinitionPrefix::Variation
        }),
    )))
    .parse(input)?;
    let (input, is_individual) = opt(preceded(tag(&b"individual"[..]), ws1))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
    let (input, _) = tag(&b"part"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(&b"def"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, identification) = identification(input)?;
    let (input, specializes) = parse_optional_definition_specialization(input)?;
    let (input, body) = part_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            PartDef {
                definition_prefix,
                is_individual,
                identification,
                specializes,
                body,
            },
        ),
    ))
}

/// Parses "part" then dispatches: if "def" follows, part_def; else part_usage. Used in package body so "part name" is not consumed by part_def.
pub(crate) fn part_def_or_usage(input: Input<'_>) -> IResult<Input<'_>, PartDefOrUsage> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, definition_prefix) = opt(alt((
        map(preceded(tag(&b"abstract"[..]), ws1), |_| {
            DefinitionPrefix::Abstract
        }),
        map(preceded(tag(&b"variation"[..]), ws1), |_| {
            DefinitionPrefix::Variation
        }),
    )))
    .parse(input)?;
    let (input, is_individual) = opt(preceded(tag(&b"individual"[..]), ws1))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
    let (input, _) = tag(&b"part"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    if let Ok((input, _)) = tag::<_, _, nom::error::Error<Input>>(&b"def"[..]).parse(input) {
        let (input, _) = ws1(input)?;
        let (input, identification) = identification(input)?;
        let (input, specializes) =
            parse_optional_definition_specialization(input)?;
        let (input, body) = part_def_body(input)?;
        return Ok((
            input,
            PartDefOrUsage::Def(node_from_to(
                start,
                input,
                PartDef {
                    definition_prefix,
                    is_individual,
                    identification,
                    specializes,
                    body,
                },
            )),
        ));
    }
    if let Ok((input, usage)) = part_usage_redefines_only(start, input) {
        let mut usage = usage;
        usage.value.usage_prefix = definition_prefix;
        usage.value.is_individual = is_individual;
        return Ok((input, PartDefOrUsage::Usage(usage)));
    }
    let (input, mut usage) = part_usage_named(start, input)?;
    usage.value.usage_prefix = definition_prefix;
    usage.value.is_individual = is_individual;
    Ok((input, PartDefOrUsage::Usage(usage)))
}
