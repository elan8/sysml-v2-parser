//! The shared SysML `PerformActionUsageDeclaration` production (8.2.2.17.2).
use crate::ast::{FeatureValue, Node, PerformActionTarget};
use crate::parser::lex::{reference_path, ws_and_comments};
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
            let (input, redefines) = crate::parser::usage::optional_redefinition(input)?;
            (input, PerformActionTarget::Reference { action, redefines })
        };
        let (input, value) = nom::combinator::opt(nom::sequence::preceded(
            ws_and_comments,
            crate::parser::feature_value::feature_value_part,
        ))
        .parse(input)?;
        Ok((input, (target, value)))
    })
}
