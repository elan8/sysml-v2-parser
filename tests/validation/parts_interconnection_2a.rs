//! Parser smoke test for `02-Parts Interconnection/2a-Parts Interconnection.sysml`.
//!
//! Qualified references are document-local identities, so this fixture deliberately validates
//! the parsed document rather than hand-constructing an AST with synthetic reference IDs.

use sysml_v2_parser::ast::{ImportShape, PackageBody, PackageBodyElement, RootElement};
use sysml_v2_parser::parse;

#[test]
#[ignore = "requires SysML v2 release fixtures; run with: cargo test --test validation -- --include-ignored"]
fn test_parse_2a_parts_interconnection() {
    super::init_log();
    let path = super::validation_fixture_path("02-Parts Interconnection")
        .join("2a-Parts Interconnection.sysml");
    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read fixture {}: {error}", path.display()));
    let document = parse(&input).expect("parse should succeed for 2a-Parts Interconnection.sysml");

    let package = match &document.root.elements.as_slice() {
        [root] => match &root.value {
            RootElement::Package(package) => &package.value,
            other => panic!("expected package root, got {other:?}"),
        },
        roots => panic!("expected one package root, got {}", roots.len()),
    };
    assert_eq!(
        package.identification.simple_name(),
        Some("2a-Parts Interconnection")
    );

    let PackageBody::Brace { elements, .. } = &package.body else {
        panic!("expected package brace body");
    };
    let imports: Vec<_> = elements
        .iter()
        .filter_map(|element| match &element.value {
            PackageBodyElement::Import(import) => Some(&import.value),
            _ => None,
        })
        .collect();
    assert_eq!(imports.len(), 2);
    assert!(imports.iter().all(|import| matches!(
        import.target.shape,
        ImportShape::Namespace {
            recursive_suffix: None,
            combined_recursive_suffix_span: None,
            ..
        }
    )));
    assert!(imports.iter().all(|import| document
        .qualified_reference(import.target.reference)
        .is_some()));

    let nested_packages = elements
        .iter()
        .filter(|element| matches!(element.value, PackageBodyElement::Package(_)))
        .count();
    assert_eq!(nested_packages, 2);
}
