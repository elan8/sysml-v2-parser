//! GH-90: `individual` prefix on definitions/usages, and `timeslice`/`snapshot` usage gaps.
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

/// Real usage: `Individuals Examples/AnalysisIndividualExample.sysml:75-77`:
/// ```text
/// individual analysis def FuelEconomyAnalysis_1 :> FuelEconomyAnalysis;
/// individual action def FuelConsumption_1 :> FuelConsumption;
/// ```
/// Previously: `analysis_case_def`/`action_def` had no `individual` prefix handling at all
/// (`part_def`/`occurrence_def` already supported it via ad hoc parsing).
#[test]
fn gh90_1_individual_analysis_and_action_def_at_package_level() {
    let elements = package_elements(
        r#"package P {
            analysis def FuelEconomyAnalysis;
            action def FuelConsumption;
            individual analysis def FuelEconomyAnalysis_1 :> FuelEconomyAnalysis;
            individual action def FuelConsumption_1 :> FuelConsumption;
        }"#,
    );
    let PackageBodyElement::AnalysisCaseDef(analysis) = &elements[2] else {
        panic!("expected AnalysisCaseDef, got {:?}", elements[2]);
    };
    assert!(analysis.value.is_individual);
    let PackageBodyElement::ActionDef(action) = &elements[3] else {
        panic!("expected ActionDef, got {:?}", elements[3]);
    };
    assert!(action.value.is_individual);
}

/// Real usage: `Simple Tests/IndividualTest.sysml:3`:
/// ```text
/// individual occurrence def IO2 {
///     individual io : IO1;
/// }
/// ```
/// Previously: `occurrence_def` had no `individual` prefix handling.
#[test]
fn gh90_1_individual_occurrence_def_at_package_level() {
    let elements = package_elements(
        r#"package P {
            individual def IO1;
            individual occurrence def IO2 {
                individual io : IO1;
            }
        }"#,
    );
    let PackageBodyElement::OccurrenceDef(occ) = &elements[1] else {
        panic!("expected OccurrenceDef, got {:?}", elements[1]);
    };
    assert!(occ.value.is_individual);
}

/// Real usage: `Individuals Examples/JohnIndividualExample.sysml:19`:
/// ```text
/// individual item def John :> Person { ... }
/// ```
/// Previously: `item_def`/`item_def_required` had no `individual` prefix handling.
#[test]
fn gh90_1_individual_item_def_at_package_level() {
    let elements = package_elements(
        r#"package P {
            item def Person;
            individual item def John :> Person;
        }"#,
    );
    let PackageBodyElement::ItemDef(item) = &elements[1] else {
        panic!("expected ItemDef, got {:?}", elements[1]);
    };
    assert!(item.value.is_individual);
}

/// Real usage: `Simple Tests/IndividualTest.sysml:4,8,30`:
/// ```text
/// individual occurrence def IO2 {
///     individual io : IO1;
/// }
/// individual item def II1 {
///     individual item ii : II1;
/// }
/// individual action def AP1 {
///     individual action a : AP1;
/// }
/// ```
/// `OccurrenceUsagePrefix`'s `individual` also applies to *usages*, not just definitions -- an
/// adjacent gap discovered in the same real fixture once the def-level cascade was cleared.
/// `item`'s named form is only opaquely captured inside attribute/item bodies (matching the
/// existing un-prefixed `item` starter's fidelity there), so only occurrence/action are checked
/// for a structured `is_individual` flag here.
#[test]
fn gh90_1_individual_prefix_on_occurrence_and_action_usages() {
    let elements = package_elements(
        r#"package P {
            individual def IO1;
            individual occurrence def IO2 {
                individual io : IO1;
            }
            action def AP1_T;
            action def AP1 {
                individual action a : AP1_T;
            }
        }"#,
    );
    let PackageBodyElement::OccurrenceDef(occ_def) = &elements[1] else {
        panic!("expected OccurrenceDef, got {:?}", elements[1]);
    };
    let sysml_v2_parser::ast::DefinitionBody::Brace {
        elements: occ_elements,
        ..
    } = &occ_def.value.body
    else {
        panic!("expected brace occurrence def body");
    };
    let occ_usage = occ_elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::DefinitionBodyElement::OccurrenceMember(m) => match &m.value {
            sysml_v2_parser::ast::OccurrenceBodyElement::OccurrenceUsage(o) => Some(&o.value),
            _ => None,
        },
        _ => None,
    });
    let occ_usage = occ_usage.expect("expected an OccurrenceUsage element");
    assert!(occ_usage.prefix.individual_span().is_some());

    let PackageBodyElement::ActionDef(action_def) = &elements[3] else {
        panic!("expected ActionDef, got {:?}", elements[3]);
    };
    let sysml_v2_parser::ast::ActionDefBody::Brace {
        elements: action_elements,
        ..
    } = &action_def.value.body
    else {
        panic!("expected brace action def body");
    };
    let action_usage = action_elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::ActionDefBodyElement::ActionUsage(a) => Some(&a.value),
        _ => None,
    });
    let action_usage = action_usage.expect("expected an ActionUsage element");
    assert!(action_usage.is_individual);
}

/// Real usage: `Individuals Examples/JohnIndividualExample.sysml:9-16`:
/// ```text
/// item def Person {
///     attribute age : ScalarValues::Natural;
///     timeslice asPresident : Person [0..*] { ... }
/// }
/// ```
/// Previously: `attribute_body_element` (shared by `item def`/`item` usage bodies) had no
/// `timeslice_usage`/`snapshot_usage` dispatch at all -- both already fully parse (used
/// elsewhere, e.g. part def bodies), just weren't reachable from here.
#[test]
fn gh90_2_timeslice_usage_inside_item_def_body() {
    let elements = package_elements(
        r#"package P {
            item def Person {
                timeslice asPresident : Person [0..*];
            }
        }"#,
    );
    let PackageBodyElement::ItemDef(person) = &elements[0] else {
        panic!("expected ItemDef, got {:?}", elements[0]);
    };
    let sysml_v2_parser::ast::AttributeBody::Brace { elements, .. } = &person.value.body else {
        panic!("expected brace item def body");
    };
    let timeslice = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::AttributeBodyElement::OccurrenceUsage(o) => Some(&o.value),
        _ => None,
    });
    let timeslice = timeslice.expect("expected an OccurrenceUsage element");
    assert_eq!(timeslice.name, "asPresident");
}
