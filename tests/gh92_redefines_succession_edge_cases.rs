//! GH-92: literal `redefines` keyword edge cases and unnamed typed succession statement.
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

/// Real usage: `Mass Roll-up Example/Vehicles.sysml:26`:
/// ```text
/// part c :> car {
///     redefines mass = 1000 [kg];
/// }
/// ```
/// Previously: `redefinition_feature_binding`/`attribute_feature_binding` only recognized the
/// symbolic `:>>` operator as a redefinition prefix, not the literal `redefines` keyword (a
/// documented synonym via `redefine_operator`, already used elsewhere e.g. `part_usage`).
#[test]
fn gh92_1_bare_redefines_keyword_standalone_body_member() {
    let elements = package_elements(
        r#"package P {
            part def Car { attribute mass; }
            part c : Car {
                redefines mass = 1000 [kg];
            }
        }"#,
    );
    let PackageBodyElement::PartUsage(c) = &elements[1] else {
        panic!("expected PartUsage, got {:?}", elements[1]);
    };
    let sysml_v2_parser::ast::PartUsageBody::Brace { elements, .. } = &c.value.body else {
        panic!("expected brace part usage body");
    };
    let attr = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartUsageBodyElement::AttributeUsage(a) => Some(&a.value),
        _ => None,
    });
    let attr = attr.expect("expected an AttributeUsage element");
    assert!(attr.name.is_empty());
    assert!(attr.redefines.is_some());
    assert!(attr.value.is_some());
}

/// Real usage: `v1 Spec Examples/8.4.5 Constraining Decomposition/
/// Vehicle Decomposition - Updated.sysml:43`:
/// ```text
/// part redefines chs {
///     part redefines rb : LightRollBar[0..1];
/// }
/// ```
/// Previously: `part_usage_redefines_only` had no `: Type` clause parsing at all -- the
/// type-less bare (`part redefines lb;`) and braced-body (`part redefines engine { ... }`)
/// forms already worked, but adding an explicit type broke parsing entirely.
#[test]
fn gh92_2_part_redefines_with_explicit_type_clause() {
    let elements = package_elements(
        r#"package P {
            part def Chassis { part rb; }
            part def LightRollBar;
            part redefines chs {
                part redefines rb : LightRollBar[0..1];
            }
        }"#,
    );
    let PackageBodyElement::PartUsage(chs) = &elements[2] else {
        panic!("expected PartUsage, got {:?}", elements[2]);
    };
    let sysml_v2_parser::ast::PartUsageBody::Brace { elements, .. } = &chs.value.body else {
        panic!("expected brace part usage body");
    };
    let rb = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartUsageBodyElement::PartUsage(p) => Some(&p.value),
        _ => None,
    });
    let rb = rb.expect("expected a nested PartUsage element");
    // The target is semantic, not a declaration name, even when an explicit type follows.
    assert!(rb.name.is_empty());
    assert!(rb.typing.is_some());
    assert!(rb.redefines.is_some());
    assert!(rb.multiplicity.is_some());
}

/// Real usage: `Vehicle Example/VehicleIndividuals.sysml:49`:
/// ```text
/// individual part vehicle1 : Vehicle1 {
///     succession : HappensJustBefore first vehicle1_t0 then vehicle1_t0_t1;
/// }
/// ```
/// Previously: (a) `succession_usage`'s name-optionality check didn't recognize a bare `:` (type
/// with no name) as "no name", so it tried and failed to parse `HappensJustBefore` as a name-less
/// succession's own name; (b) `succession_usage` had no `: Type` parsing at all (unlike its
/// action-body sibling `first_stmt`/`succession_prefix`); (c) `succession_usage` was never
/// dispatched inside `part_usage_body_element` at all, only `ConnectionDefBodyElement`/
/// `OccurrenceBodyElement`.
#[test]
fn gh92_3_unnamed_typed_succession_in_part_usage_body() {
    let elements = package_elements(
        r#"package P {
            part def V;
            part def HappensJustBefore;
            part vehicle1 : V {
                part vehicle1_t0 : V;
                part vehicle1_t0_t1 : V;
                succession : HappensJustBefore first vehicle1_t0 then vehicle1_t0_t1;
            }
        }"#,
    );
    let PackageBodyElement::PartUsage(vehicle1) = &elements[2] else {
        panic!("expected PartUsage, got {:?}", elements[2]);
    };
    let sysml_v2_parser::ast::PartUsageBody::Brace { elements, .. } = &vehicle1.value.body else {
        panic!("expected brace part usage body");
    };
    let succ = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartUsageBodyElement::SuccessionUsage(s) => Some(&s.value),
        _ => None,
    });
    let succ = succ.expect("expected a SuccessionUsage element");
    assert!(succ.name.is_none());
    assert!(succ.type_name.is_some());
}
