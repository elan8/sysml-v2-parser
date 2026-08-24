//! GH-53, revisited under the reference grammar: an end whose target is a kind-keyworded usage
//! is that usage family's own node with an `EndUsagePrefix` head, not an `EndDecl` carrying a
//! nested usage.
//!
//! `OccurrenceUsagePrefix = ( EndUsagePrefix | BasicUsagePrefix … ) UsageExtensionKeyword*`
//! (reference `SysML.xtext:836-843`) and `EndUsagePrefix = 'end' OwnedCrossFeatureMember?`
//! (SysML BNF 285), whose `OwnedCrossFeature = BasicUsagePrefix UsageDeclaration` (293). So in
//! `end theCauses [*] occurrence theCause :> causes :>> source { }` the end *is* the occurrence
//! usage `theCause`, and `theCauses [*]` is its owned cross feature.

use sysml_v2_parser::ast::{
    ConnectionDefBody, ConnectionDefBodyElement, EndIdentity, OccurrenceUsagePrefix, PackageBody,
    PackageBodyElement, RootElement,
};
use sysml_v2_parser::parse_with_diagnostics;

fn connection_def_elements(input: &str) -> Vec<ConnectionDefBodyElement> {
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace package body");
    };
    let connection = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::ConnectionDef(c) => Some(&c.value),
            _ => None,
        })
        .expect("expected connection def");
    let ConnectionDefBody::Brace { elements, .. } = &connection.body else {
        panic!("expected connection def brace body");
    };
    elements.iter().map(|e| e.value.clone()).collect()
}

/// The cross feature's declared name and whether it authored a multiplicity.
fn cross_feature(prefix: &OccurrenceUsagePrefix) -> (Option<String>, bool) {
    let end = prefix.end().expect("an `end` head");
    let cross = end.cross.as_deref().expect("an owned cross feature");
    let declaration = &cross.value.declaration.value;
    (
        declaration.identification.name.clone(),
        declaration.multiplicity.is_some(),
    )
}

#[test]
fn end_occurrence_usage_owns_its_cross_feature() {
    let input = "package P {\nconnection def Causation {\nend theCauses [*] occurrence theCause :> causes :>> source {\n}\nend theEffects [*] occurrence theEffect :> effects :>> target {\n}\n}\n}";
    let elements = connection_def_elements(input);
    assert_eq!(elements.len(), 2);

    let ConnectionDefBodyElement::OccurrenceUsage(the_cause) = &elements[0] else {
        panic!("expected an occurrence usage, got {:?}", elements[0]);
    };
    assert_eq!(the_cause.value.name, "theCause");
    assert_eq!(
        cross_feature(&the_cause.value.prefix),
        (Some("theCauses".to_owned()), true)
    );
    assert!(the_cause.value.prefix.basic().is_none());
}

#[test]
fn end_item_usage_owns_its_cross_feature() {
    let input = "package P {\nconnection def Touches {\nend touchesToo [0..*] item touchedItemToo :>> separateSpaceToo, thisOccurrence;\nend touches [0..*] item touchedItem :>> separateSpace, thatOccurrence;\n}\n}";
    let elements = connection_def_elements(input);
    assert_eq!(elements.len(), 2);

    let ConnectionDefBodyElement::ItemUsage(touched_item_too) = &elements[0] else {
        panic!("expected an item usage, got {:?}", elements[0]);
    };
    assert_eq!(touched_item_too.value.name, "touchedItemToo");
    assert_eq!(
        cross_feature(&touched_item_too.value.prefix),
        (Some("touchesToo".to_owned()), true)
    );
}

#[test]
fn end_part_usage_with_a_bare_cross_multiplicity() {
    let input = "package P {\nconnection def PressureSeat {\nend [1] part bead : TireBead;\nend [1] part mountingRim : TireMountingRim;\n}\n}";
    let elements = connection_def_elements(input);
    assert_eq!(elements.len(), 2);

    let ConnectionDefBodyElement::PartUsage(bead) = &elements[0] else {
        panic!("expected a part usage, got {:?}", elements[0]);
    };
    assert_eq!(bead.value.name, "bead");
    assert_eq!(cross_feature(&bead.value.prefix), (None, true));
}

#[test]
fn keyword_less_ends_stay_end_declarations() {
    let input = "package P {\nconnection def C {\nend hub ::> mainSwitch[1];\nend source: Anything :>> BinaryLinkObject::source;\nend : TireBead[1];\n}\n}";
    let elements = connection_def_elements(input);
    assert_eq!(elements.len(), 3);

    let ConnectionDefBodyElement::EndDecl(hub) = &elements[0] else {
        panic!("expected an end declaration, got {:?}", elements[0]);
    };
    assert!(matches!(&hub.value.identity, EndIdentity::Declaration(name) if name.value == "hub"));
    assert!(hub.value.references.is_some());
    assert!(hub.value.typing.is_none());

    let ConnectionDefBodyElement::EndDecl(source) = &elements[1] else {
        panic!("expected an end declaration, got {:?}", elements[1]);
    };
    assert!(source.value.typing.is_some());
    assert_eq!(
        source
            .value
            .redefines
            .as_ref()
            .map(|n| n.value.target.len()),
        Some(1)
    );

    let ConnectionDefBodyElement::EndDecl(anonymous) = &elements[2] else {
        panic!("expected an end declaration, got {:?}", elements[2]);
    };
    assert!(matches!(anonymous.value.identity, EndIdentity::Anonymous));
    assert!(anonymous.value.typing.is_some());
    assert!(anonymous.value.multiplicity.is_some());
}
