//! TDD tests: SysML snippets with expected AST.

use sysml_parser::ast::{
    Identification, Node, Package, PackageBody, RootElement, RootNamespace, Span,
};
use sysml_parser::{parse, parse_with_diagnostics};

fn id(name: &str) -> Identification {
    Identification {
        short_name: None,
        name: Some(name.to_string()),
    }
}

/// Wrap value in a node with dummy span for expected AST construction.
fn n<T>(v: T) -> Node<T> {
    Node::new(Span::dummy(), v)
}

/// Node with span matching parser output for full-input parses (offset 0, line 1, column 1).
fn n_len<T>(len: usize, v: T) -> Node<T> {
    Node::new(
        Span {
            offset: 0,
            line: 1,
            column: 1,
            len,
        },
        v,
    )
}

/// Build expected AST for `package Foo;` (input len = 12)
fn expected_package_foo_semicolon() -> RootNamespace {
    RootNamespace {
        elements: vec![n_len(12, RootElement::Package(n_len(12, Package {
            identification: id("Foo"),
            body: PackageBody::Semicolon,
        })))],
    }
}

/// Build expected AST for `package Bar { }` (input len = 15)
fn expected_package_bar_brace() -> RootNamespace {
    RootNamespace {
        elements: vec![n_len(15, RootElement::Package(n_len(15, Package {
            identification: id("Bar"),
            body: PackageBody::Brace { elements: vec![] },
        })))],
    }
}

#[test]
fn test_package_with_semicolon_body() {
    let input = "package Foo;";
    let result = parse(input).expect("parse should succeed");
    let expected = expected_package_foo_semicolon();
    assert_eq!(result, expected, "AST should match expected for package Foo;");
}

#[test]
fn test_package_with_brace_body() {
    let input = "package Bar { }";
    let result = parse(input).expect("parse should succeed");
    let expected = expected_package_bar_brace();
    assert_eq!(
        result, expected,
        "AST should match expected for package Bar {{ }}"
    );
}

#[test]
fn test_parse_with_diagnostics_partial_ast_and_multiple_errors() {
    // One valid element, two invalid lines, then another valid element. Recovery should collect
    // two errors and still produce a partial AST with both valid packages.
    let input = "package Foo;\nnot valid\nalso bad\npackage Bar;";
    let result = parse_with_diagnostics(input);
    assert!(!result.is_ok(), "should have parse errors");
    assert_eq!(result.errors.len(), 2, "should report two parse errors");
    assert_eq!(
        result.root.elements.len(),
        2,
        "partial AST should contain both valid packages"
    );
    // First element is Foo, second is Bar
    let names: Vec<&str> = result
        .root
        .elements
        .iter()
        .filter_map(|n| {
            if let RootElement::Package(p) = &n.value {
                p.value.identification.name.as_deref()
            } else {
                None
            }
        })
        .collect();
    assert_eq!(names, ["Foo", "Bar"]);

    // Error quality: each error should have "found" snippet and expected context
    for err in &result.errors {
        assert!(err.found.is_some(), "error should have 'found' snippet: {}", err.message);
        assert!(
            err.expected.is_some(),
            "error should have 'expected' context: {}",
            err.message
        );
        assert!(
            err.expected.as_deref().map_or(false, |e| e.contains("package") || e.contains("namespace")),
            "expected should mention package or namespace: {:?}",
            err.expected
        );
        assert!(err.code.is_some(), "error should have a code");
    }
    // First error is at "not valid"
    assert!(
        result.errors[0].found.as_deref().map_or(false, |f| f.contains("not")),
        "first error found should mention invalid token: {:?}",
        result.errors[0].found
    );
}

#[test]
fn test_parse_error_expected_end_of_input_has_found() {
    // Trailing text after valid packages: parse succeeds for "package Foo; package Bar;" then rest "garbage" triggers "expected end of input"
    let input = "package Foo; package Bar; garbage";
    let result = parse(input);
    let err = result.unwrap_err();
    assert!(
        err.message.contains("expected end of input"),
        "error should be 'expected end of input': {}",
        err
    );
    assert!(err.found.is_some(), "expected end of input error should have 'found': {}", err);
    assert!(err.found.as_deref().map_or(false, |f| f.contains("garbage")), "found should show trailing text: {:?}", err.found);
    assert_eq!(err.code.as_deref(), Some("expected_end_of_input"));
}

#[test]
fn test_parse_error_display_includes_found_and_location() {
    let input = "package Foo;\nxyz";
    let result = parse_with_diagnostics(input);
    let err = &result.errors[0];
    let display = err.to_string();
    assert!(display.contains("line"), "Display should include line number");
    assert!(
        err.found.as_ref().map_or(false, |f| display.contains(f)),
        "Display should include found snippet: {}",
        display
    );
}
