//! Metadata definition and usage parsing (BNF MetadataDefinition / MetadataUsage).

use crate::ast::{Membership, MetadataDef, MetadataUsage, Node};
use crate::parser::attribute::metadata_body;
use crate::parser::definition_header::parse_feature_usage_header;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::lex::{name, starts_with_keyword, visibility_prefix, ws1, ws_and_comments};
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
        DefinitionPrefixOptions::new(b"metadata")
            .def_required()
            .with_captured_visibility(),
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
                membership: Membership::owning(prefix.visibility, prefix.visibility_span),
            },
        ),
    ))
}

/// Metadata usage: `metadata` name (`:` type)? body.
pub(crate) fn metadata_usage(input: Input<'_>) -> IResult<Input<'_>, Node<MetadataUsage>> {
    let start = input;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
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
                type_reference: header.type_reference,
                about_targets,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

#[cfg(test)]
mod membership_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    // --- parser work item 4b (final sweep): Membership on MetadataDef/MetadataUsage ---

    #[test]
    fn metadata_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = metadata_def(input("private metadata def M1;")).expect("metadata def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Private)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }

    #[test]
    fn metadata_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = metadata_def(input("metadata def M1;")).expect("metadata def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn metadata_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) =
            metadata_usage(input("protected metadata m1 : M1;")).expect("metadata usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Protected)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn metadata_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = metadata_usage(input("metadata m1 : M1;")).expect("metadata usage");
        assert_eq!(node.value.membership.visibility, None);
    }
}
