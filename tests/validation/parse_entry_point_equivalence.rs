//! GH-70: on clean input (no diagnostics), the two public parse entry points must agree.
//! `parse()` (strict, used by conformance/roundtrip gates) and `parse_for_editor()` (partial
//! AST plus diagnostics, for IDE/LSP use) should report the same "is this valid" verdict and,
//! when both say yes, build the same AST. See `docs/CONFORMANCE.md`'s "Entry points" section
//! for guidance on which one callers should use.
//!
//! Both currently loop over the same per-element parser (`package::root_element`) under the
//! hood -- `parse_root` via `many0(root_element)`, `parse_with_diagnostics` via a manual
//! recovery loop -- so they should already agree on clean input. This is the regression test
//! GH-70 asks for: a future change to either loop (or to root-level recovery/collection) that
//! makes them diverge on some clean input should fail here instead of surfacing later as a
//! roundtrip harness false positive (the original GH-66/GH-69 bug class).

use sysml_v2_parser::{parse, parse_for_editor};

/// Asserts `parse(src)` succeeds, `parse_for_editor(src)` reports zero diagnostics, and the two
/// resulting ASTs are equal once spans are normalized out.
fn assert_entry_points_agree(src: &str, label: &str) {
    let strict = parse(src).unwrap_or_else(|e| panic!("{label}: parse() failed: {e}"));
    let editor = parse_for_editor(src);
    assert!(
        editor.errors.is_empty(),
        "{label}: parse_for_editor() reported diagnostics on input parse() accepted: {:?}",
        editor.errors
    );
    assert_eq!(
        strict.normalize_for_test_comparison(),
        editor.root.normalize_for_test_comparison(),
        "{label}: parse() and parse_for_editor() built different ASTs for the same clean input"
    );
}

#[test]
fn package_with_part_def_and_attribute() {
    assert_entry_points_agree(
        "package Demo {\n\tpart def Wheel;\n\tattribute value : Real;\n}\n",
        "package_with_part_def_and_attribute",
    );
}

#[test]
fn import_and_doc_comment() {
    assert_entry_points_agree(
        "package Demo {\n\tdoc /* top-level doc */\n\tprivate import Base::*;\n\tpart def Thing;\n}\n",
        "import_and_doc_comment",
    );
}

#[test]
fn action_def_with_control_nodes() {
    assert_entry_points_agree(
        "package Demo {\n\taction def Run {\n\t\tfirst start;\n\t\tthen action step;\n\t\tif true {\n\t\t\tthen step;\n\t\t} else {\n\t\t\tthen start;\n\t\t}\n\t}\n}\n",
        "action_def_with_control_nodes",
    );
}

#[test]
fn requirement_def_with_nested_state() {
    assert_entry_points_agree(
        "package Demo {\n\trequirement def Goal;\n\tstate def Phase;\n\tpart def Mission {\n\t\trequirement goals : Goal;\n\t\texhibit state phases : Phase {\n\t\t\tstate launch;\n\t\t}\n\t}\n}\n",
        "requirement_def_with_nested_state",
    );
}

#[test]
fn nested_calc_def_inside_calc_body() {
    assert_entry_points_agree(
        "package Demo {\n\tcalc def Outer {\n\t\tin x : Real;\n\t\tcalc def Inner {\n\t\t\tin y : Real;\n\t\t\treturn y;\n\t\t}\n\t\treturn x;\n\t}\n}\n",
        "nested_calc_def_inside_calc_body",
    );
}

#[test]
fn interface_with_connect_and_ports() {
    assert_entry_points_agree(
        "package Demo {\n\tpart def A { port p; }\n\tpart def B { port q; }\n\tpart a1 : A;\n\tpart b1 : B;\n\tconnect a1.p to b1.q;\n}\n",
        "interface_with_connect_and_ports",
    );
}

#[test]
#[ignore = "requires SysML v2 release fixtures; run with: cargo test --test validation -- --include-ignored"]
fn validation_fixture_1a_parts_tree() {
    super::init_log();
    let path = super::validation_fixture_path("01-Parts Tree").join("1a-Parts Tree.sysml");
    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {}", path.display(), e));
    assert_entry_points_agree(&input, "1a-Parts Tree.sysml");
}

#[test]
#[ignore = "requires SysML v2 release fixtures; run with: cargo test --test validation -- --include-ignored"]
fn validation_fixture_2a_parts_interconnection() {
    super::init_log();
    let path = super::validation_fixture_path("02-Parts Interconnection")
        .join("2a-Parts Interconnection.sysml");
    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {}", path.display(), e));
    assert_entry_points_agree(&input, "2a-Parts Interconnection.sysml");
}
