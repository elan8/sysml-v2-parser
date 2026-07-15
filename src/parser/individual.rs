//! Individual definition parsing.

use crate::ast::{IndividualDef, Node};
use crate::parser::attribute::attribute_body;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::node_from_to;
use crate::parser::Input;
use nom::IResult;

/// Individual definition: `individual def` Identification ( (`:>` | `specializes`) qualified_name )? body
pub(crate) fn individual_def(input: Input<'_>) -> IResult<Input<'_>, Node<IndividualDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"individual")
            .def_required()
            .no_abstract(),
    )?;
    let (input, body) = attribute_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            IndividualDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
            },
        ),
    ))
}
