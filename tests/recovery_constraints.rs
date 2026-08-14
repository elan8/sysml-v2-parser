use sysml_v2_parser::ast::{
    CalcDefBody, CalcDefBodyElement, PackageBody, PackageBodyElement, RootElement,
};
use sysml_v2_parser::parse_with_diagnostics;

#[test]
fn calc_reports_unmodeled_members_as_explicit_recovery() {
    // `objective` has no calc-body production; it must surface as an explicit recovery node
    // with a diagnostic (previously a diagnostic-silent opaque `Other(...)` capture), while the
    // valid `return` sibling after it still parses.
    let input = "package P { calc def K { objective fuelEconomy { } return result: Real; } }";
    let result = parse_with_diagnostics(input);
    assert_eq!(
        result.errors.len(),
        1,
        "the unmodeled member must produce exactly one diagnostic: {:?}",
        result.errors
    );
    assert_eq!(
        result.errors[0].code.as_deref(),
        Some("recovered_calc_body_element")
    );
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace body");
    };
    let calc = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::CalcDef(c) => Some(&c.value),
            _ => None,
        })
        .expect("calc def should be present");
    let CalcDefBody::Brace { elements } = &calc.body else {
        panic!("expected calc body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, CalcDefBodyElement::Error(_))),
        "the unmodeled member must be an explicit recovery element"
    );
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, CalcDefBodyElement::ReturnDecl(_))),
        "the valid return sibling after the recovered member must still parse"
    );
}
