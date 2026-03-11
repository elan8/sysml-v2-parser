//! TDD tests: SysML snippets with expected AST.

use sysml_parser::ast::{
    Identification, Node, Package, PackageBody, PackageBodyElement, RootNamespace, Span,
};
use sysml_parser::parse;

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
        elements: vec![n_len(12, PackageBodyElement::Package(n_len(12, Package {
            identification: id("Foo"),
            body: PackageBody::Semicolon,
        })))],
    }
}

/// Build expected AST for `package Bar { }` (input len = 15)
fn expected_package_bar_brace() -> RootNamespace {
    RootNamespace {
        elements: vec![n_len(15, PackageBodyElement::Package(n_len(15, Package {
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
