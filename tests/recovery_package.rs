use sysml_v2_parser::ast::{PackageBody, PackageBodyElement, RootElement};
use sysml_v2_parser::{parse, parse_with_diagnostics};

#[test]
fn package_recovery_inserts_error_node_and_keeps_later_sibling() {
    // parse_root now rejects any document with an embedded recovery placeholder (GH-2), so the
    // local-recovery guarantee this test checks belongs to parse_with_diagnostics, not parse.
    let input = "package P {\n#fmeaspec requirement req1 { }\npart def Good;\n}";
    assert!(
        parse(input).is_err(),
        "strict should reject the unsupported annotation"
    );
    let root = parse_with_diagnostics(input).root;
    let pkg = match &root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, PackageBodyElement::Error(_))),
        "expected package body error node"
    );
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, PackageBodyElement::PartDef(_))),
        "later valid sibling should still parse"
    );
}

#[test]
fn package_recovery_diagnostic_is_specific() {
    let input = "package P {\n#fmeaspec requirement req1 { }\npart def Good;\n}";
    let result = parse_with_diagnostics(input);
    let err = result
        .errors
        .iter()
        .find(|e| e.code.as_deref() == Some("unsupported_annotation_syntax"))
        .expect("expected package recovery diagnostic");
    assert!(
        err.message.contains("annotation"),
        "annotation recovery should explain the failure"
    );
    assert!(
        err.found
            .as_deref()
            .is_some_and(|f| f.contains("#fmeaspec")),
        "diagnostic should preserve recovered snippet"
    );
}
