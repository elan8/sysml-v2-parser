//! Item definition and usage parsing.

use crate::ast::{ItemDef, ItemUsage, Node};
use crate::parser::attribute::{attribute_body, direction_prefix};
use crate::parser::definition_header::parse_feature_usage_header;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::lex::{name, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::usage::multiplicity_node;
use crate::parser::Input;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::IResult;
use nom::Parser;

/// Item definition: `item def` Identification body
pub(crate) fn item_def(input: Input<'_>) -> IResult<Input<'_>, Node<ItemDef>> {
    parse_item_def(input, false)
}

/// Item definition with required `def` keyword (disambiguates from `item` usages in part bodies).
pub(crate) fn item_def_required(input: Input<'_>) -> IResult<Input<'_>, Node<ItemDef>> {
    parse_item_def(input, true)
}

fn parse_item_def(input: Input<'_>, require_def: bool) -> IResult<Input<'_>, Node<ItemDef>> {
    let start = input;
    let mut options = DefinitionPrefixOptions::new(b"item").with_captured_visibility();
    if require_def {
        options = options.def_required();
    }
    let (input, prefix) = parse_definition_prefix(input, options)?;
    let (input, body) = attribute_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ItemDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
                membership: crate::ast::Membership::owning(
                    prefix.visibility,
                    prefix.visibility_span,
                ),
            },
        ),
    ))
}

/// Item usage in a part definition body: `item` name multiplicity? (`:` type)? body.
pub(crate) fn item_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ItemUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, _) = tag(&b"item"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, name) = name(input)?;
    let (input, multiplicity) = opt(multiplicity_node).parse(input)?;
    let (input, header) = parse_feature_usage_header(input)?;
    let (input, body) = attribute_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ItemUsage {
                name,
                type_name: header.type_name,
                multiplicity,
                body,
                direction: None,
                membership: crate::ast::Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

/// `in`/`out`/`inout item` usage (port def bodies): direction + [`item_usage`].
pub(crate) fn directed_item_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ItemUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, direction) = direction_prefix(input)?;
    let (input, mut usage) = item_usage(input)?;
    usage.value.direction = Some(direction);
    Ok((input, node_from_to(start, input, usage.value)))
}

#[cfg(test)]
mod membership_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    // --- parser work item 4b (continuation): Membership on ItemDef/ItemUsage ---

    #[test]
    fn item_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) = item_usage(input("private item i1: MyItem;")).expect("item usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Private)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn item_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = item_usage(input("item i1: MyItem;")).expect("item usage");
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    /// `item_def` previously never parsed a `private`/`protected`/`public` prefix at all (same
    /// genuine gap as `part_def`/`port_def`).
    #[test]
    fn item_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = item_def(input("protected item def MyItem;")).expect("item def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Protected)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }

    #[test]
    fn item_def_public_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = item_def(input("public item def MyItem;")).expect("item def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
    }

    #[test]
    fn item_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = item_def(input("item def MyItem;")).expect("item def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }
}
