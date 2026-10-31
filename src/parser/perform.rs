//! The shared SysML `PerformActionUsageDeclaration` production (8.2.2.17.2).
use crate::ast::{FeatureValue, Node, PerformActionTarget};
use crate::parser::lex::{reference_path, ws_and_notes};
use crate::parser::Input;
use nom::{IResult, Parser};

pub(crate) fn perform_action_declaration(
    input: Input<'_>,
) -> IResult<Input<'_>, (PerformActionTarget, Option<Node<FeatureValue>>)> {
    crate::parser::span::reference_transaction(input, |input| {
        let (input, keyword) =
            crate::parser::occurrence_prefix::optional_keyword_token(input, b"action")?;
        let (input, target) = if keyword.is_some() {
            let (input, declaration) = crate::parser::usage::usage_declaration(input)?;
            (input, PerformActionTarget::Action(Box::new(declaration)))
        } else {
            let (input, action) = reference_path(input)?;
            let (after, declaration) =
                crate::parser::usage::usage_declaration_without_identification(input)?;
            let authored = {
                let d = &declaration.value;
                d.typing.is_some()
                    || d.multiplicity.is_some()
                    || d.multiplicity_modifiers != Default::default()
                    || d.subsets.is_some()
                    || d.redefines.is_some()
                    || d.references.is_some()
                    || d.crosses.is_some()
                    || d.intersects.is_some()
            };
            let (input, specialization) = if authored {
                (after, Some(Box::new(declaration)))
            } else {
                (input, None)
            };
            (
                input,
                PerformActionTarget::Reference {
                    action,
                    specialization,
                },
            )
        };
        let (input, value) = nom::combinator::opt(nom::sequence::preceded(
            ws_and_notes,
            crate::parser::feature_value::feature_value_part,
        ))
        .parse(input)?;
        Ok((input, (target, value)))
    })
}
