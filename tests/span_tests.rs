//! Tests that AST node spans (offset, line, column, len) are calculated correctly.

use sysml_v2_parser::ast::{AstNode, RootElement};
use sysml_v2_parser::parse;

fn assert_span(
    node: &impl AstNode,
    offset: usize,
    line: u32,
    column: usize,
    len: usize,
    msg: &str,
) {
    let s = node.span();
    assert_eq!(s.offset, offset, "{} (offset)", msg);
    assert_eq!(s.line, line, "{} (line)", msg);
    assert_eq!(s.column, column, "{} (column)", msg);
    assert_eq!(s.len, len, "{} (len)", msg);
}

#[test]
fn test_single_line_package_span() {
    let input = "package Foo;";
    let result = parse(input).expect("parse should succeed");
    assert_eq!(result.elements.len(), 1, "one top-level element");

    let elem = &result.elements[0];
    assert_span(elem, 0, 1, 1, 12, "root element covers full input");

    if let RootElement::Package(pkg) = &elem.value {
        assert_span(pkg, 0, 1, 1, 12, "inner package node span");
    } else {
        panic!("expected Package element");
    }
}

#[test]
fn test_multi_line_second_element_span() {
    let line1 = "package Foo;";
    let newline = "\n";
    let line2 = "package Bar;";
    let input = format!("{}{}{}", line1, newline, line2);
    let result = parse(&input).expect("parse should succeed");
    assert_eq!(result.elements.len(), 2, "two top-level elements");

    // First element span covers "package Foo;" only (newline consumed by ws between elements).
    let len1 = line1.len();
    let len2 = line2.len();
    let offset2 = line1.len() + newline.len();

    assert_span(
        &result.elements[0],
        0,
        1,
        1,
        len1,
        "first element ends after first line (no newline in span)",
    );
    assert_span(
        &result.elements[1],
        offset2,
        2,
        1,
        len2,
        "second element starts at line 2, column 1",
    );
}
