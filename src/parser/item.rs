//! Item definition and usage parsing.

use crate::ast::{ItemDef, ItemUsage, Node};
use crate::parser::attribute::attribute_body;
use crate::parser::definition_header::parse_feature_usage_header;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::lex::{name, short_name_prefix, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::occurrence_prefix::next_word_is_reserved;
use crate::parser::usage::multiplicity_node;
use crate::parser::Input;
use nom::combinator::opt;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// Item definition: `item def` Identification body. `def` is mandatory here (unlike some sibling
/// `*_def` parsers) so a bodyless `individual item i1;` short usage form is never misclassified
/// as an `ItemDef` with `i1` as the definition's identification name -- see the package-level
/// dispatch site in `package.rs` (gap #7) and `port_def`/`connection_def`'s analogous
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
                definition_prefix: prefix.basic_prefix,
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
/// `ItemUsage = OccurrenceUsagePrefix 'item' Usage` (SysML BNF 616).
///
/// Wrapped in a reference transaction because the prefix's `UsageExtensionKeyword*` allocates an
/// arena entry per `#tag` before the production is known to apply.
pub(crate) fn item_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ItemUsage>> {
    crate::parser::span::reference_transaction(input, item_usage_inner)
}

fn item_usage_inner(input: Input<'_>) -> IResult<Input<'_>, Node<ItemUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    // `MemberPrefix` precedes the usage's own prefix.
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    // The whole shared `OccurrenceUsagePrefix`, not a hand-rolled subset: this parser previously
    // accepted `RefPrefix` plus `individual` and nothing else, so `ref individual item :>> driver
    // : Alice;` (`training/28. Individuals/Individuals and Time Slices.sysml:10`) was refused
    // here and misread as an occurrence usage named `item`.
    let (input, prefix) = crate::parser::occurrence_prefix::occurrence_usage_prefix(input)?;
    let (input, _) = crate::parser::occurrence_prefix::keyword_token(input, b"item")?;
    // `Identification`'s `( '<' ShortName '>' )?` half (BNF §8.2.2.2) -- see
    // `attribute::attribute_usage`'s identical short-name handling for the confirmed real-usage
    // citation (the same `VehicleGeometryAndCoordinateFrames.sysml` example this function's
    // existing comment below already cites for the name-optional shape).
    let (input, short_name) = short_name_prefix(input)?;
    // Name is optional: `ItemUsage`'s BNF `Identification` legally omits the name in favor of a
    // leading FeatureSpecializationPart (`item :>> shape : Cylinder { ... }` or `item redefines
    // fuelSupply;`). A bare parser `name` accepts reserved keywords, so it used to take
    // `redefines` as the declaration label and strand its target. The shared reserved-name guard
    // leaves every unquoted grammar keyword for the header parser; quoted `'redefines'` remains a
    // valid declaration name. Confirmed direct corpus use: Training 12 Binding Connectors, lines
    // 11--12. This is the same `Identification` boundary `requirement` owns for its anonymous
    // usage declaration.
    let (input, name) = if next_word_is_reserved(input) {
        (input, None)
    } else {
        opt(preceded(ws_and_comments, name)).parse(input)?
    };
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
                prefix,
                name,
                short_name,
                type_name: header.type_reference,
                redefines: header.redefines,
                subsets: header.subsets,
                multiplicity: multiplicity.or(header.multiplicity),
                multiplicity_modifiers: header.multiplicity_modifiers.clone(),
                value,
                body,
                membership: crate::ast::Membership::feature(visibility, visibility_span),
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
        assert!(node.value.name.is_none());
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
        assert!(node.value.name.is_none());
        assert!(node.value.redefines.is_some());
        assert!(node.value.type_name.is_some());
        assert!(node.value.value.is_none());
    }

    #[test]
    fn item_usage_named_form_still_parses() {
        let source = input("item wheelShape : Circle;");
        let (rest, node) = item_usage(source).expect("item usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .name
                .map(|n| crate::parser::lex::name_bytes(source, n)),
            Some(&b"wheelShape"[..])
        );
        assert!(node.value.type_name.is_some());
        assert!(node.value.redefines.is_none());
    }

    #[test]
    fn item_usage_named_form_accepts_a_value() {
        let source = input("item shape = new Box(1 [mm], 2 [mm], 3 [mm]);");
        let (rest, node) = item_usage(source).expect("item usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .name
                .map(|n| crate::parser::lex::name_bytes(source, n)),
            Some(&b"shape"[..])
        );
        assert!(node.value.value.is_some());
    }

    // --- short-name (`<shortName>`) support, mirroring `attribute_usage`'s identical gap (shared
    // `Identification` BNF production, §8.2.2.2) -- see `attribute.rs::attribute_body_tests`'s
    // citation of the confirmed real-usage gap in the OMG Geometry domain library's
    // `VehicleGeometryAndCoordinateFrames.sysml`.

    #[test]
    fn item_usage_captures_short_name() {
        let source = input("item <ws> wheelShape : Circle;");
        let (rest, node) = item_usage(source).expect("item usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .short_name
                .map(|n| crate::parser::lex::name_bytes(source, n)),
            Some(&b"ws"[..])
        );
        assert_eq!(
            node.value
                .name
                .map(|n| crate::parser::lex::name_bytes(source, n)),
            Some(&b"wheelShape"[..])
        );
    }

    #[test]
    fn item_usage_without_short_name_has_none() {
        let (_, node) = item_usage(input("item wheelShape : Circle;")).expect("item usage");
        assert_eq!(node.value.short_name, None);
    }
}
