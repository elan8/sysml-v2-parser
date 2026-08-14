//! GH-87: keyword-less minimal feature-declaration shorthand gaps triaged from #83's `examples/`
//! roundtrip scan. Each test below uses the exact (trimmed) real source that motivated the fix.

use sysml_v2_parser::ast::{
    PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement, RootElement,
};
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
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace package body");
    };
    elements.iter().map(|e| e.value.clone()).collect()
}

/// Real usage: `Simple Tests/AnalysisTest.sysml:4`:
/// ```text
/// part def V {
///     m;
/// }
/// ```
/// Previously: `feature_value_binding` required a `feature_value_part` (`=`/`:=`/`default`), and
/// wasn't dispatched inside `part def` bodies at all (only action bodies). A fully bare `name;`
/// with no type and no value is a legal `DefaultReferenceUsage`.
#[test]
fn gh87_1_bare_name_feature_declaration_in_part_def_body() {
    let elements = package_elements(
        r#"package P {
            part def V {
                m;
            }
        }"#,
    );
    let PackageBodyElement::PartDef(part_def) = &elements[0] else {
        panic!("expected PartDef, got {:?}", elements[0]);
    };
    let PartDefBody::Brace { elements } = &part_def.value.body else {
        panic!("expected brace part def body");
    };
    let default_ref = elements.iter().find_map(|e| match &e.value {
        PartDefBodyElement::DefaultReferenceUsage(d) => Some(&d.value),
        _ => None,
    });
    let default_ref = default_ref.expect("expected a DefaultReferenceUsage element");
    assert_eq!(default_ref.name, "m");
    assert!(default_ref.typing.is_none());
    assert!(default_ref.value.is_none());
}

/// Regression: bare identifiers inside *action* bodies must keep triggering the targeted
/// `invalid_bare_identifier_in_action_body` recovery diagnostic (`tests/recovery_actions.rs`)
/// rather than silently parsing as an empty-value `DefaultReferenceUsage` -- `feature_value_binding`
/// (used in action bodies) is deliberately left value-mandatory; only
/// `bare_or_valued_feature_binding` (used in part def bodies) accepts a bare name.
#[test]
fn gh87_1_bare_identifier_in_action_body_still_reports_diagnostic() {
    let result = parse_with_diagnostics(
        r#"package P {
            action def AD {
                batCap;
            }
        }"#,
    );
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code.as_deref() == Some("invalid_bare_identifier_in_action_body")),
        "expected the targeted bare-identifier diagnostic to still fire, got {:?}",
        result.errors
    );
}

/// Real usage: `v1 Spec Examples/8.4.1 Wheel Hub Assembly/Wheel Package.sysml:9` (official OMG
/// spec 1.6 §8.4.1 example, also present in the "- Updated" sibling file):
/// ```text
/// package 'Wheel Package' {
///     pressure = force / length^2;
/// }
/// ```
/// Previously: the keyword-less `name = expr;` binding shorthand was only dispatched inside
/// part/attribute/action bodies (per the existing "§6 G26" `feature_value_binding`), not at
/// package-body scope, even though official OMG spec-derived examples use it there directly.
#[test]
fn gh87_2_package_scope_name_equals_expr_binding() {
    let elements = package_elements(
        r#"package 'Wheel Package' {
            attribute force = 1.0;
            attribute length = 2.0;
            pressure = force / length^2;
        }"#,
    );
    let default_ref = elements.iter().find_map(|e| match e {
        PackageBodyElement::DefaultReferenceUsage(d) => Some(&d.value),
        _ => None,
    });
    let default_ref = default_ref.expect("expected a DefaultReferenceUsage element");
    assert_eq!(default_ref.name, "pressure");
    assert!(default_ref.typing.is_none());
    assert!(default_ref.value.is_some());
}

/// Bare *identifiers* at package scope are the KerML implicit-feature shorthand (`causeA;` in
/// the Cause-and-Effect examples; spec42 gap 23) and parse as a `DefaultReferenceUsage`;
/// misused *reserved keywords* (`then;`) must keep triggering their existing targeted recovery
/// diagnostics rather than silently parsing as an implicit feature.
#[test]
fn gh87_2_bare_identifier_at_package_scope_still_reports_diagnostic() {
    let result = parse_with_diagnostics("package P { test; }");
    assert_eq!(
        result.errors.len(),
        0,
        "a bare identifier is the implicit-feature shorthand: {:?}",
        result.errors
    );

    let result = parse_with_diagnostics("package P { then; }");
    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    assert_eq!(
        result.errors[0].code.as_deref(),
        Some("unexpected_keyword_in_scope")
    );
}

/// Real usage: `State Space Representation Examples/EVSample.sysml:47`:
/// ```text
/// part def Motor {
///     torquePerCurrent :> Quantities::scalarQuantities = ISQ::torque / ISQ::electricCurrent;
/// }
/// ```
/// Previously: `DefaultReferenceUsage` had no `subsets`/`redefines` field at all, so the
/// keyword-less shorthand couldn't carry a leading `:>` specialization clause before the value.
#[test]
fn gh87_3_shorthand_with_subsets_clause_and_value() {
    let elements = package_elements(
        r#"package P {
            part def Quantities { attribute scalarQuantities; }
            part def ISQ { attribute torque; attribute electricCurrent; }
            part def Motor {
                torquePerCurrent :> Quantities::scalarQuantities = ISQ::torque / ISQ::electricCurrent;
            }
        }"#,
    );
    let PackageBodyElement::PartDef(motor) = &elements[2] else {
        panic!("expected PartDef, got {:?}", elements[2]);
    };
    let PartDefBody::Brace { elements } = &motor.value.body else {
        panic!("expected brace part def body");
    };
    let default_ref = elements.iter().find_map(|e| match &e.value {
        PartDefBodyElement::DefaultReferenceUsage(d) => Some(&d.value),
        _ => None,
    });
    let default_ref = default_ref.expect("expected a DefaultReferenceUsage element");
    assert_eq!(default_ref.name, "torquePerCurrent");
    assert!(default_ref.subsets.is_some(), "expected a subsets clause");
    assert!(default_ref.redefines.is_none());
    assert!(default_ref.value.is_some());
}

/// Real usage: `v1 Spec Examples/8.4.1 Wheel Hub Assembly/Wheel Package.sysml:21`:
/// ```text
/// part def WheelAssembly {
///     inflationPressure :> pressure;
/// }
/// ```
/// Same gap as above, but with the `:>` clause and *no* value at all -- the value is inherited
/// from what it subsets.
#[test]
fn gh87_3_shorthand_with_subsets_clause_and_no_value() {
    let elements = package_elements(
        r#"package P {
            attribute pressure = 1.0;
            part def WheelAssembly {
                inflationPressure :> pressure;
            }
        }"#,
    );
    let PackageBodyElement::PartDef(assembly) = &elements[1] else {
        panic!("expected PartDef, got {:?}", elements[1]);
    };
    let PartDefBody::Brace { elements } = &assembly.value.body else {
        panic!("expected brace part def body");
    };
    let default_ref = elements.iter().find_map(|e| match &e.value {
        PartDefBodyElement::DefaultReferenceUsage(d) => Some(&d.value),
        _ => None,
    });
    let default_ref = default_ref.expect("expected a DefaultReferenceUsage element");
    assert_eq!(default_ref.name, "inflationPressure");
    assert!(default_ref.subsets.is_some(), "expected a subsets clause");
    assert!(default_ref.value.is_none());
}

/// Real usage: `Simple Tests/OccurrenceTest.sysml:6`:
/// ```text
/// occurrence def Occ {
///     item x;
/// }
/// ```
/// Previously: `item_usage` already fully supported the bare (untyped, no value) form -- it just
/// wasn't dispatched inside `occurrence_body_element` at all (`part_usage` already was, which is
/// why the sibling `part y;` on the next line of the same fixture already worked).
#[test]
fn gh87_4_bare_item_usage_in_occurrence_def_body() {
    let elements = package_elements(
        r#"package P {
            occurrence def Occ {
                item x;
                part y;
            }
        }"#,
    );
    let PackageBodyElement::OccurrenceDef(occ) = &elements[0] else {
        panic!("expected OccurrenceDef, got {:?}", elements[0]);
    };
    let sysml_v2_parser::ast::DefinitionBody::Brace { elements } = &occ.value.body else {
        panic!("expected brace occurrence def body");
    };
    let elements: Vec<_> = elements
        .iter()
        .filter_map(|e| match &e.value {
            sysml_v2_parser::ast::DefinitionBodyElement::OccurrenceMember(m) => Some(&m.value),
            _ => None,
        })
        .collect();
    let item = elements.iter().find_map(|e| match e {
        sysml_v2_parser::ast::OccurrenceBodyElement::ItemUsage(i) => Some(&i.value),
        _ => None,
    });
    let item = item.expect("expected an ItemUsage element");
    assert_eq!(item.name, "x");
    assert!(item.type_name.is_none());
    assert!(item.value.is_none());

    assert!(
        elements.iter().any(|e| matches!(
            e,
            sysml_v2_parser::ast::OccurrenceBodyElement::PartUsage(p) if p.value.name == "y"
        )),
        "expected the sibling `part y;` to still parse"
    );
}
