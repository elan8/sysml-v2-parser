//! GH-88: attribute/reference usage modifier gaps (`::>`, `derived constant ref`, `abstract`,
//! directed `in ref` with comma-separated types, unnamed `attribute :>`).
//! Each test below uses the exact (trimmed) real source that motivated the fix.

use sysml_v2_parser::ast::{PackageBody, PackageBodyElement, RootElement};
use sysml_v2_parser::parse_with_diagnostics;

fn package_elements(input: &str) -> Vec<PackageBodyElement> {
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
    elements.iter().map(|e| e.value.clone()).collect()
}

/// Real usage: `Simple Tests/CalculationTest.sysml:14`:
/// ```text
/// part vehicle : Vehicle {
///     part eng : VehiclePart;
///     part trans : VehiclePart;
///     attribute ::> m = ms.totalMass;
/// }
/// ```
/// Previously: `attribute_usage`'s head dispatch had no `::>` (reference-subsetting) case --
/// only `:>>` (redefines) was recognized as a name-standing-in prefix.
#[test]
fn gh88_1_reference_subsetting_operator_stands_in_for_name() {
    let elements = package_elements(
        r#"package P {
            part def VehiclePart { attribute m; }
            part def Vehicle;
            part vehicle : Vehicle {
                part ms : VehiclePart;
                attribute ::> m = ms.m;
            }
        }"#,
    );
    let PackageBodyElement::PartUsage(vehicle) = &elements[2] else {
        panic!("expected PartUsage, got {:?}", elements[2]);
    };
    let sysml_v2_parser::ast::PartUsageBody::Brace { elements, .. } = &vehicle.value.body else {
        panic!("expected brace part usage body");
    };
    let attr = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartUsageBodyElement::AttributeUsage(a) => Some(&a.value),
        _ => None,
    });
    let attr = attr.expect("expected an AttributeUsage element");
    assert!(attr.name.is_empty());
    assert!(attr.references.is_some());
}

/// Real usage: `Simple Tests/PartTest.sysml:8-9`:
/// ```text
/// part def A {
///     constant attribute x[0..2];
///     derived constant ref attribute y :> x;
/// }
/// ```
/// Previously: `attribute_usage`'s `RefPrefix` handling only recognized `derived`/`constant`;
/// `abstract`/`variation`/`ref` were assumed illegal on an attribute usage (incorrectly, per
/// BNF `BasicUsagePrefix = RefPrefix ('ref')?`, `RefPrefix = 'derived'? ('abstract'|'variation')?
/// 'constant'?`).
#[test]
fn gh88_2_derived_constant_ref_attribute_modifier_stack() {
    let elements = package_elements(
        r#"package P {
            part def A {
                constant attribute x[0..2];
                derived constant ref attribute y :> x;
            }
        }"#,
    );
    let PackageBodyElement::PartDef(a) = &elements[0] else {
        panic!("expected PartDef, got {:?}", elements[0]);
    };
    let sysml_v2_parser::ast::PartDefBody::Brace { elements, .. } = &a.value.body else {
        panic!("expected brace part def body");
    };
    let y = elements
        .iter()
        .filter_map(|e| match &e.value {
            sysml_v2_parser::ast::PartDefBodyElement::AttributeUsage(a) => Some(&a.value),
            _ => None,
        })
        .find(|a| a.name == "y")
        .expect("expected attribute y");
    assert!(y.is_derived);
    assert!(y.is_constant);
    assert!(y.is_reference);
    assert!(y.subsets.is_some());
}

/// Real usage: `Mass Roll-up Example/MassRollup.sysml:20-21`:
/// ```text
/// part filteredMassThing :> compositeThing {
///     abstract attribute minMass :> ISQ::mass;
/// }
/// ```
/// Previously: `abstract` was not recognized as a valid attribute-usage prefix at all.
#[test]
fn gh88_3_abstract_prefix_on_plain_attribute_usage() {
    let elements = package_elements(
        r#"package P {
            part def CompositeThing;
            part filteredMassThing : CompositeThing {
                abstract attribute minMass :> ISQ::mass;
            }
        }"#,
    );
    let PackageBodyElement::PartUsage(p) = &elements[1] else {
        panic!("expected PartUsage, got {:?}", elements[1]);
    };
    let sysml_v2_parser::ast::PartUsageBody::Brace { elements, .. } = &p.value.body else {
        panic!("expected brace part usage body");
    };
    let attr = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartUsageBodyElement::AttributeUsage(a) => Some(&a.value),
        _ => None,
    });
    let attr = attr.expect("expected an AttributeUsage element");
    assert_eq!(attr.name, "minMass");
    assert_eq!(
        attr.usage_prefix,
        Some(sysml_v2_parser::ast::DefinitionPrefix::Abstract)
    );
}

/// Real usage: `Simple Tests/ItemTest.sysml:15`:
/// ```text
/// private part def C {
///     private in ref y: A, B;
/// }
/// ```
/// Previously: `part_ref_usage` had no `in`/`out` direction prefix handling at all (the
/// comma-separated multi-target type list already worked via `optional_typings`/`typings`).
#[test]
fn gh88_4_directed_ref_with_comma_separated_types() {
    let elements = package_elements(
        r#"package P {
            part def A;
            part def B;
            private part def C {
                private in ref y: A, B;
            }
        }"#,
    );
    let PackageBodyElement::PartDef(c) = &elements[2] else {
        panic!("expected PartDef, got {:?}", elements[2]);
    };
    let sysml_v2_parser::ast::PartDefBody::Brace { elements, .. } = &c.value.body else {
        panic!("expected brace part def body");
    };
    let y = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartDefBodyElement::Ref(r) => Some(&r.value),
        _ => None,
    });
    let y = y.expect("expected a Ref element");
    assert_eq!(y.name, "y");
    assert_eq!(y.direction, Some(sysml_v2_parser::ast::InOut::In));
    assert_eq!(
        y.typing.as_ref().map(|typing| typing.value.target.len()),
        Some(2)
    );
}

/// Real usage: `Geometry Examples/CarWithShapeAndCSG.sysml:84`:
/// ```text
/// attribute :> differencesOf[1] {
///     item :>> elements = (rawEngineBlock, cylinder1, cylinder2);
/// }
/// ```
/// Previously: `attribute_usage`'s head dispatch had no bare `:>` (subsets, no name) case --
/// only `:>>` (redefines) was recognized as a name-standing-in prefix; a bare `:>` fell through
/// to opaque recovery.
#[test]
fn gh88_5_unnamed_attribute_subsets_usage() {
    let elements = package_elements(
        r#"package P {
            part def Q {
                attribute :> differencesOf[1];
            }
        }"#,
    );
    let PackageBodyElement::PartDef(q) = &elements[0] else {
        panic!("expected PartDef, got {:?}", elements[0]);
    };
    let sysml_v2_parser::ast::PartDefBody::Brace { elements, .. } = &q.value.body else {
        panic!("expected brace part def body");
    };
    let attr = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartDefBodyElement::AttributeUsage(a) => Some(&a.value),
        _ => None,
    });
    let attr = attr.expect("expected an AttributeUsage element");
    assert!(attr.name.is_empty());
    assert!(attr.subsets.is_some());
    assert!(attr.multiplicity.is_some());
}
