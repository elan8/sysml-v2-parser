//! GH-87: keyword-less minimal feature-declaration shorthand gaps triaged from #83's `examples/`
//! roundtrip scan. Each test below uses the exact (trimmed) real source that motivated the fix.

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
    let sysml_v2_parser::ast::DefinitionBody::Brace { elements, .. } = &occ.value.body else {
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
