//! Internal definition and usage header parsing (P4).
//!
//! Consolidates typed headers and specialization without changing public AST shapes.

use crate::ast::{Node, Span, TypingRelationship};
use crate::parser::specialization::parse_optional_definition_header_with_raw;
use crate::parser::usage::{feature_usage_header, UsageHeader};
use crate::parser::Input;
use nom::IResult;

/// Parsed subclassification / typed header after `identification`.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct DefinitionHeaderParts {
    pub specializes: Option<Node<TypingRelationship>>,
    /// Span of the text swallowed by the plain `: Type` header scan, when that branch was taken.
    /// See [`crate::parser::specialization::parse_optional_definition_header_with_raw`].
    pub raw_header: Option<Span>,
}

/// Parse optional definition header after identification (typing + `:>` / `specializes`).
pub(crate) fn parse_definition_header_after_ident(
    input: Input<'_>,
) -> IResult<Input<'_>, DefinitionHeaderParts> {
    let (input, (specializes, raw_header)) = parse_optional_definition_header_with_raw(input)?;
    Ok((
        input,
        DefinitionHeaderParts {
            specializes,
            raw_header,
        },
    ))
}

/// Feature usage header parts (typing, subsets, redefines, references, crosses).
pub(crate) type FeatureHeaderParts = UsageHeader;

/// Library-style feature usage header (multiplicity, typing, specialization, intersects).
pub(crate) fn parse_feature_usage_header(
    input: Input<'_>,
) -> IResult<Input<'_>, FeatureHeaderParts> {
    feature_usage_header(input)
}
