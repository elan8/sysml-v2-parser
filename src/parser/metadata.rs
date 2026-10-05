//! Metadata definition and usage parsing (BNF MetadataDefinition / MetadataUsage).

use crate::ast::{Membership, MetadataDef, MetadataUsage, Node};
use crate::parser::attribute::metadata_body as attribute_metadata_body;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::lex::qualified_reference;
use crate::parser::lex::{starts_with_keyword, visibility_prefix, ws1, ws_and_comments};
use crate::parser::metadata_annotation::{metadata_declared_name, parse_about_targets};
use crate::parser::metadata_body::metadata_body;
use crate::parser::node_from_to;
use crate::parser::Input;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::error::{Error, ErrorKind};
use nom::IResult;
use nom::Parser;

/// Metadata definition: `metadata def` Identification body (optional `abstract` prefix).
///
/// `MetadataDefinition = ( isAbstract ?= 'abstract' )? DefinitionExtensionKeyword* 'metadata' 'def'
/// Definition` (SysML BNF 1652; Pilot `SysML.xtext` 121) is one of only two definition productions
/// in the pin that does *not* reach `BasicDefinitionPrefix`: it inlines the `abstract` flag and has
/// no `variation` alternative, so this is the one definition kind whose prefix is genuinely a
/// boolean rather than a one-of-two slot. See [`crate::parser::definition_prefix::BasicPrefixSlot`].
pub(crate) fn metadata_def(input: Input<'_>) -> IResult<Input<'_>, Node<MetadataDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"metadata")
            .def_required()
            .abstract_only_prefix()
            .with_captured_visibility(),
    )?;
    let (input, body) = attribute_metadata_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            MetadataDef {
                is_abstract: prefix.basic_prefix.is_some(),
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
                membership: Membership::owning(prefix.visibility, prefix.visibility_span),
            },
        ),
    ))
}

/// Metadata usage: `metadata` MetadataUsageDeclaration (`about` targets)? body.
///
/// `MetadataUsageDeclaration = ( Identification ( ':' | 'typed' 'by' ) )? OwnedFeatureTyping`:
/// the `Identification` is a declared name only when a separator follows it, and the typing is
/// required. So `metadata Tag about x;` is typed by `Tag` and declares no name; only
/// `metadata t : Tag about x;` declares `t`. The declaration is parsed exactly as the `@` /
/// `metadata` annotation member parses it ([`metadata_declared_name`]).
pub(crate) fn metadata_usage(input: Input<'_>) -> IResult<Input<'_>, Node<MetadataUsage>> {
    crate::parser::span::reference_transaction(input, metadata_usage_inner)
}

fn metadata_usage_inner(input: Input<'_>) -> IResult<Input<'_>, Node<MetadataUsage>> {
    let start = input;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"metadata"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    if starts_with_keyword(input.fragment(), b"def") {
        return Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)));
    }
    let (input, declared_name) = opt(metadata_declared_name).parse(input)?;
    let (short_name, name) = match declared_name {
        Some(declared) => (
            declared.value.identification.short_name,
            declared.value.identification.name,
        ),
        None => (None, None),
    };
    let (input, _) = ws_and_comments(input)?;
    let (input, type_reference) = qualified_reference(input)?;
    let (input, about_targets) = parse_about_targets(input)?;
    let (input, body) = metadata_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            MetadataUsage {
                name,
                short_name,
                type_reference,
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

    fn parse_usage(text: &str) -> (crate::ParsedDocument, crate::ast::MetadataUsage) {
        let document = crate::parse(&format!("package P {{ {text} }}")).expect("parse");
        let crate::ast::RootElement::Package(package) = &document.root.elements[0].value else {
            panic!("expected package");
        };
        let crate::ast::PackageBody::Brace { elements, .. } = &package.value.body else {
            panic!("expected package body");
        };
        let crate::ast::PackageBodyElement::MetadataUsage(usage) = &elements[0].value else {
            panic!("expected MetadataUsage, got {:?}", elements[0].value);
        };
        let usage = usage.value.clone();
        (document, usage)
    }

    fn typing_text(document: &crate::ParsedDocument, usage: &crate::ast::MetadataUsage) -> String {
        document
            .qualified_reference(usage.type_reference)
            .expect("typing reference")
            .authored_text()
            .to_owned()
    }

    #[test]
    fn a_lone_name_is_the_metadata_typing_not_a_declared_name() {
        let (document, usage) = parse_usage("metadata Tag about x;");
        assert!(usage.name.is_none() && usage.short_name.is_none());
        assert_eq!(typing_text(&document, &usage), "Tag");
        assert_eq!(usage.about_targets.len(), 1);
    }

    #[test]
    fn a_qualified_lone_name_is_the_metadata_typing() {
        let (document, usage) = parse_usage("metadata Lib::Tag;");
        assert!(usage.name.is_none());
        assert_eq!(typing_text(&document, &usage), "Lib::Tag");
    }

    #[test]
    fn a_name_before_a_separator_is_declared_and_the_typing_follows() {
        for text in [
            "metadata t : Tag about x;",
            "metadata t typed by Tag about x;",
        ] {
            let (document, usage) = parse_usage(text);
            let name = usage.name.expect("declared name");
            assert_eq!(document.declaration_name(name), Some("t"), "{text}");
            assert_eq!(typing_text(&document, &usage), "Tag", "{text}");
        }
    }

    #[test]
    fn a_short_name_only_declaration_is_kept() {
        let (document, usage) = parse_usage("metadata <s> : Tag;");
        assert!(usage.name.is_none());
        let short_name = usage.short_name.expect("short name");
        assert_eq!(document.declaration_name(short_name), Some("s"));
        assert_eq!(typing_text(&document, &usage), "Tag");
    }

    #[test]
    fn metadata_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = metadata_usage(input("metadata m1 : M1;")).expect("metadata usage");
        assert_eq!(node.value.membership.visibility, None);
    }
}
