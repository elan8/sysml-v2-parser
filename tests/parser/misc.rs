//! Parser tests: misc

use super::common::*;
use sysml_v2_parser::ast::*;
use sysml_v2_parser::{parse, parse_with_diagnostics};

#[test]
fn test_parse_primitive_data_types_validation_fixture() {
    let Some(input) = primitive_data_types_fixture() else {
        return;
    };
    let result = parse(&input);
    assert!(
        result.is_ok(),
        "fixture should parse cleanly; diagnostics: {:?}",
        parse_with_diagnostics(&input).errors
    );
}

#[test]
fn test_case_usage_accepts_typed_by_and_specialization_clauses() {
    let input = r#"package P {
case analyze typed by Mission::CaseType subsets BaseCase;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements, .. } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let case_usage = match &elements[0].value {
        PackageBodyElement::CaseUsage(c) => c,
        other => panic!("expected case usage, got {:?}", other),
    };
    assert!(case_usage.value.type_name.is_some());
}
