//! GH-113: an attribute usage written with only a subsetting-family target reparses to itself.
//!
//! `attribute :>> target;`, `attribute ::> target;` and `attribute :> target;` declare no name of
//! their own. `emit_attribute_usage` used to derive a display name from the target *and* emit the
//! clause, so `attribute :>> differencesOf[1];` came back as `attribute differencesOf :>>
//! differencesOf;` -- a different, self-referential construct.
//!
//! The shapes and the anonymity are pinned by
//! `tests/snapshots/sysml/attribute_prefix_target_forms.md`, whose AST section shows
//! `(declaration-name none)` beside each relationship and whose format section is
//! `(stable-idempotent)` -- the emitted text is byte-identical to the source, which the duplicated
//! name would break outright.
//!
//! This file keeps the check a fixture cannot make: that the emitted text *parses back* to the
//! same tree. The snapshot tool compares the strict and editor parses of a fixture's source, never
//! a reparse of its own output.

use sysml_v2_parser::{emit_sysml, parse};

#[track_caller]
fn assert_reparses_identically(source: &str) {
    let parsed = parse(source).expect("parse original");
    let emitted = emit_sysml(&parsed).expect("emit");
    let reparsed = parse(&emitted)
        .unwrap_or_else(|error| panic!("reparse of emitted text failed: {error}\n{emitted}"));
    assert_eq!(
        parsed.normalize_for_test_comparison(),
        reparsed.normalize_for_test_comparison(),
        "roundtrip AST mismatch; emitted:\n{emitted}"
    );
}

#[test]
fn a_redefines_prefix_attribute_reparses_identically() {
    assert_reparses_identically(
        "package P {\n    part def Q {\n        attribute differencesOf[1];\n        attribute :>> differencesOf[1];\n    }\n}\n",
    );
}

/// Real usage: `Simple Tests/CalculationTest.sysml:14`.
#[test]
fn a_references_prefix_attribute_reparses_identically() {
    assert_reparses_identically(
        "package P {\n    part def VehiclePart {\n        attribute m;\n    }\n    part def Vehicle;\n    part vehicle : Vehicle {\n        part ms : VehiclePart;\n        attribute ::> m = ms.m;\n    }\n}\n",
    );
}

/// Real usage: `Geometry Examples/CarWithShapeAndCSG.sysml:84`.
#[test]
fn a_subsets_prefix_attribute_reparses_identically() {
    assert_reparses_identically(
        "package P {\n    part def Q {\n        attribute differencesOf[1];\n        attribute :> differencesOf[1];\n    }\n}\n",
    );
}
