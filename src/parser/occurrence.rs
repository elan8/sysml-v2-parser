//! Occurrence definition parsing.

use crate::ast::{Node, OccurrenceDef};
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::node_from_to;
use crate::parser::occurrence_body::occurrence_def_definition_body;
use crate::parser::Input;
use nom::IResult;

pub(crate) use crate::parser::occurrence_body::{
    individual_usage, occurrence_usage, snapshot_usage, then_timeslice_usage, timeslice_usage,
};

pub(crate) fn occurrence_def(input: Input<'_>) -> IResult<Input<'_>, Node<OccurrenceDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"occurrence").def_required(),
    )?;
    let (input, body) = occurrence_def_definition_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            OccurrenceDef {
                is_abstract: prefix.is_abstract,
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
            },
        ),
    ))
}
