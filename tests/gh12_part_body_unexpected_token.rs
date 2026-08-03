//! GH-12: Arbitrary non-SysML text inside part bodies must be rejected.
//!
//! Previously part def recovery fell through to `Other` (ignored by diagnostics) and part usage
//! hard-failed so the package path swallowed the decl as `ExtendedLibraryDecl`. Package / action /
//! attribute / port bodies already rejected the same junk.

use sysml_v2_parser::ast::{
    PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement, PartUsageBody,
    PartUsageBodyElement, RootElement,
};
use sysml_v2_parser::{parse, parse_for_editor, parse_with_diagnostics};

const JUNK: &str = "%%% this is not SysML at all %%%";

fn assert_rejects_with_unexpected_token(input: &str, scope_substr: &str) {
    let strict = parse(input);
    assert!(
        strict.is_err(),
        "strict parse should reject junk in {scope_substr}: {input}"
    );
    let err = strict.unwrap_err();
    assert!(
        err.message.contains("unexpected token") && err.message.contains(scope_substr),
        "strict diagnostic should mention unexpected token in {scope_substr}, got: {}",
        err.message
    );

    let editor = parse_for_editor(input);
    assert!(
        !editor.errors.is_empty(),
        "parse_for_editor should report diagnostics for junk in {scope_substr}"
    );
    assert!(
        editor.errors.iter().any(|e| {
            e.message.contains("unexpected token") && e.message.contains(scope_substr)
        }),
        "editor diagnostics should mention unexpected token in {scope_substr}, got: {:?}",
        editor.errors
    );
}

#[test]
fn part_def_body_rejects_arbitrary_non_sysml_text() {
    let input = format!("package Shop {{\n    part def Wheel {{\n        {JUNK}\n    }}\n}}\n");
    assert_rejects_with_unexpected_token(&input, "part definition body");

    let result = parse_with_diagnostics(&input);
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace body");
    };
    let part_def = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p) => Some(&p.value),
            _ => None,
        })
        .expect("part def should remain in the AST under editor parse");
    let PartDefBody::Brace { elements } = &part_def.body else {
        panic!("expected part def brace body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, PartDefBodyElement::Error(_))),
        "junk should be preserved as a part def body Error node"
    );
}

#[test]
fn part_usage_body_rejects_arbitrary_non_sysml_text() {
    let input = format!(
        "package Shop {{\n    part def Wheel;\n    part wheel : Wheel {{\n        {JUNK}\n    }}\n}}\n"
    );
    assert_rejects_with_unexpected_token(&input, "part usage body");

    let result = parse_with_diagnostics(&input);
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace body");
    };
    let part_usage = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartUsage(p) if p.value.name == "wheel" => Some(&p.value),
            _ => None,
        })
        .expect("part usage should remain a real PartUsage, not ExtendedLibraryDecl");
    let PartUsageBody::Brace { elements } = &part_usage.body else {
        panic!("expected part usage brace body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, PartUsageBodyElement::Error(_))),
        "junk should be preserved as a part usage body Error node"
    );
}

#[test]
fn other_body_kinds_still_reject_same_junk() {
    let cases = [
        (format!("package P {{\n    {JUNK}\n}}\n"), "package body"),
        (
            format!("package P {{\n    action def A {{\n        {JUNK}\n    }}\n}}\n"),
            "action",
        ),
        (
            format!("package P {{\n    attribute def A {{\n        {JUNK}\n    }}\n}}\n"),
            "attribute body",
        ),
        (
            format!("package P {{\n    port def A {{\n        {JUNK}\n    }}\n}}\n"),
            "port",
        ),
    ];
    for (input, scope_hint) in cases {
        let strict = parse(&input);
        assert!(
            strict.is_err(),
            "strict parse should still reject junk ({scope_hint}): {input}"
        );
        let editor = parse_for_editor(&input);
        assert!(
            !editor.errors.is_empty(),
            "parse_for_editor should still report diagnostics ({scope_hint})"
        );
        assert!(
            editor
                .errors
                .iter()
                .any(|e| e.message.contains("unexpected token")),
            "expected unexpected-token diagnostic for {scope_hint}, got: {:?}",
            editor.errors
        );
    }
}
