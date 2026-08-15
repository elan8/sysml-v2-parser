//! Item definition and usage parsing.

use crate::ast::{ItemDef, ItemUsage, Node};
use crate::parser::attribute::{attribute_body, direction_prefix};
use crate::parser::definition_header::parse_feature_usage_header;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::lex::{name, short_name_prefix, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::usage::multiplicity_node;
use crate::parser::Input;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// Item definition: `item def` Identification body. `def` is mandatory here (unlike some sibling
/// `*_def` parsers) so a bodyless `individual item i1;` short usage form is never misclassified
/// as an `ItemDef` with `i1` as the definition's identification name -- see the package-level
/// dispatch site in `package.rs` (gap #7) and `port_def_required`/`connection_def`'s analogous
/// `_required` naming.
pub(crate) fn item_def_required(input: Input<'_>) -> IResult<Input<'_>, Node<ItemDef>> {
    let start = input;
    let options = DefinitionPrefixOptions::new(b"item")
        .individual_allowed()
        .def_required()
        .with_captured_visibility();
    let (input, prefix) = parse_definition_prefix(input, options)?;
    let (input, body) = attribute_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ItemDef {
                is_individual: prefix.is_individual,
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

/// Item usage in a part definition body: `item` (name | `:>>` redefines)? multiplicity? (`:`
/// type)? (`=` value)? body.
pub(crate) fn item_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ItemUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    // BNF `RefPrefix = 'derived'? ('abstract' | 'variation')? 'constant'?`, e.g. the package-level
    // `abstract item items : Item[0..*] nonunique :> objects { ... }` (Systems Library
    // `Items.sysml`) and `derived item ownedActorParameter : PartUsage[1..1] redefines
    // ownedMemberParameter subsets Metadata::metadataItems;` (`sysml.library/Systems
    // Library/SysML.sysml:28`). Only `abstract` was accepted before, so the `derived` form fell
    // through to unsupported-grammar capture. `ref item` stays with `connector::ref_decl`, which
    // owns the `ref` kind-keyword forms.
    let (input, prefix) = crate::parser::usage::ref_prefix(input)?;
    // BNF `OccurrenceUsagePrefix`: `(isIndividual ?= 'individual')?` (GH-90.1), e.g. `individual
    // item ii : II1;` (Simple Tests/IndividualTest.sysml:4).
    let (input, is_individual) = opt(preceded(tag(&b"individual"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"item"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    // `Identification`'s `( '<' ShortName '>' )?` half (BNF §8.2.2.2) -- see
    // `attribute::attribute_usage`'s identical short-name handling for the confirmed real-usage
    // citation (the same `VehicleGeometryAndCoordinateFrames.sysml` example this function's
    // existing comment below already cites for the name-optional shape).
    let (input, short_name) = short_name_prefix(input)?;
    // Name is optional: `ItemUsage`'s BNF `Identification` legally omits the name in favor of a
    // leading `:>>` redefinition (`item :>> shape : Cylinder { ... }`), the same shape
    // `PartUsage`/`AttributeUsage` already support. `name` simply fails to match `:>>` (not a
    // valid identifier start), so `opt` naturally falls through to `feature_usage_header` below,
    // which already recognizes a leading redefines/typing clause on its own via
    // `specialization_clauses` -- no separate `prefix_redefinition_target` branch needed, unlike
    // `part_usage`/`view_usage`'s hand-rolled dispatch. Confirmed real usage (not speculative) in
    // the OMG Geometry domain library's `VehicleGeometryAndCoordinateFrames.sysml` example.
    let (input, name) = opt(preceded(ws_and_comments, name)).parse(input)?;
    let name = name.unwrap_or_default();
    let (input, multiplicity) = opt(multiplicity_node).parse(input)?;
    let (input, header) = parse_feature_usage_header(input)?;
    let (input, value) = opt(nom::sequence::preceded(
        ws_and_comments,
        crate::parser::feature_value_part,
    ))
    .parse(input)?;
    let (input, body) = attribute_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ItemUsage {
                is_derived: prefix.is_derived,
                usage_prefix: prefix.usage_prefix,
                is_constant: prefix.is_constant,
                name,
                short_name,
                type_name: header.type_reference,
                redefines: header.redefines,
                subsets: header.subsets,
                multiplicity: multiplicity.or(header.multiplicity),
                ordered: header.ordered,
                nonunique: header.nonunique,
                value,
                body,
                direction: prefix.direction,
                is_individual: is_individual.is_some(),
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

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
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
        let (rest, node) =
            item_def_required(input("protected item def MyItem;")).expect("item def");
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
        let (rest, node) = item_def_required(input("public item def MyItem;")).expect("item def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
    }

    #[test]
    fn item_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = item_def_required(input("item def MyItem;")).expect("item def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }
}

#[cfg(test)]
mod redefines_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    // Real usage confirmed in the OMG Geometry domain library's
    // `sysml-v2-release/sysml/src/examples/Geometry Examples/VehicleGeometryAndCoordinateFrames.sysml`.

    #[test]
    fn item_usage_accepts_anonymous_redefinition_with_value() {
        let (rest, node) = item_usage(input(
            "item :>> shape = new Box(4800 [mm], 1840 [mm], 1350 [mm]);",
        ))
        .expect("item usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "");
        assert!(node.value.redefines.is_some());
        assert!(node.value.value.is_some());
        assert!(node.value.type_name.is_none());
    }

    #[test]
    fn item_usage_accepts_anonymous_redefinition_with_type_and_body() {
        let (rest, node) = item_usage(input(
            "item :>> shape : Cylinder { :>> radius = 14 [mm]; :>> height = 40 [mm]; }",
        ))
        .expect("item usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "");
        assert!(node.value.redefines.is_some());
        assert!(node.value.type_name.is_some());
        assert!(node.value.value.is_none());
    }

    #[test]
    fn item_usage_named_form_still_parses() {
        let (rest, node) = item_usage(input("item wheelShape : Circle;")).expect("item usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "wheelShape");
        assert!(node.value.type_name.is_some());
        assert!(node.value.redefines.is_none());
    }

    #[test]
    fn item_usage_named_form_accepts_a_value() {
        let (rest, node) =
            item_usage(input("item shape = new Box(1 [mm], 2 [mm], 3 [mm]);")).expect("item usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "shape");
        assert!(node.value.value.is_some());
    }

    // --- short-name (`<shortName>`) support, mirroring `attribute_usage`'s identical gap (shared
    // `Identification` BNF production, §8.2.2.2) -- see `attribute.rs::attribute_body_tests`'s
    // citation of the confirmed real-usage gap in the OMG Geometry domain library's
    // `VehicleGeometryAndCoordinateFrames.sysml`.

    #[test]
    fn item_usage_captures_short_name() {
        let (rest, node) = item_usage(input("item <ws> wheelShape : Circle;")).expect("item usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.short_name.as_deref(), Some("ws"));
        assert_eq!(node.value.name, "wheelShape");
    }

    #[test]
    fn item_usage_without_short_name_has_none() {
        let (_, node) = item_usage(input("item wheelShape : Circle;")).expect("item usage");
        assert_eq!(node.value.short_name, None);
    }
}
