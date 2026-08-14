//! Authored syntax that differs must compare as different.
//!
//! `Node<T>` compares its value and not its span, because where a construct was written is
//! provenance. Several hand-written `PartialEq` impls extended that to fields that are *not*
//! provenance -- the `ref` and `abstract` prefixes on an attribute usage, the direction on a `ref`
//! declaration, the keyword spelling of a typing relationship. Each of those is authored syntax
//! that emission reproduces, so excluding it meant a formatter that dropped or swapped one would
//! pass every whole-AST comparison in the suite, including the round-trip tests whose whole job is
//! to catch that.

use sysml_v2_parser::parse;

/// Two sources that differ only in the construct under test must not produce equal ASTs once
/// spans are normalized away.
#[track_caller]
fn assert_distinguished(left: &str, right: &str, what: &str) {
    let left_ast = parse(left).unwrap_or_else(|error| panic!("parse: {error}\n{left}"));
    let right_ast = parse(right).unwrap_or_else(|error| panic!("parse: {error}\n{right}"));
    assert_ne!(
        left_ast.normalize_for_test_comparison(),
        right_ast.normalize_for_test_comparison(),
        "{what} does not survive AST comparison:\n  {left}\n  {right}"
    );
}

#[test]
fn a_reference_usage_is_not_a_composite_one() {
    assert_distinguished(
        "package P { part def Q { attribute x : Real; } }",
        "package P { part def Q { ref attribute x : Real; } }",
        "the `ref` keyword on an attribute usage",
    );
}

#[test]
fn an_abstract_usage_is_not_a_concrete_one() {
    assert_distinguished(
        "package P { part def Q { attribute x : Real; } }",
        "package P { part def Q { abstract attribute x : Real; } }",
        "the `abstract` prefix on an attribute usage",
    );
}

#[test]
fn a_ref_declarations_direction_is_compared() {
    assert_distinguished(
        "package P { part def Q { in ref y : A; } }",
        "package P { part def Q { out ref y : A; } }",
        "the direction on a `ref` declaration",
    );
}

/// `specializes B` and `:> B` name the same relationship, but they are not the same source, and
/// the emitter chooses between them on this field alone (`emit::structure` matches `spelling`).
#[test]
fn a_typing_relationships_keyword_spelling_is_compared() {
    assert_distinguished(
        "package P { part def A :> B; }",
        "package P { part def A specializes B; }",
        "the `:>` versus `specializes` spelling",
    );
    assert_distinguished(
        "package P { part def Q { attribute x : Real; } }",
        "package P { part def Q { attribute x typed by Real; } }",
        "the `:` versus `typed by` spelling",
    );
}

/// The other half of the contract: position still is not identity. The same document authored at a
/// different offset stays equal after normalization.
#[test]
fn position_is_still_not_part_of_identity() {
    let compact = "package P { part def Q { ref attribute x : Real; } }";
    let spaced =
        "\n\n   package P {\n    part def Q {\n        ref attribute x : Real;\n    }\n}\n";
    let compact_ast = parse(compact).expect("parse");
    let spaced_ast = parse(spaced).expect("parse");
    assert_ne!(compact_ast, spaced_ast, "raw equality compares positions");
    assert_eq!(
        compact_ast.normalize_for_test_comparison(),
        spaced_ast.normalize_for_test_comparison(),
        "normalization must still erase position"
    );
}
