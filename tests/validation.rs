//! Integration tests that parse SysML validation fixture files.
//!
//! Each validation .sysml file has a corresponding test module under `validation/`
//! for easier maintenance.
//!
//! Logging defaults to WARN so test output stays small. Use `RUST_LOG=debug` (or
//! `RUST_LOG=sysml_v2_parser=debug`) and `--nocapture` when debugging parser behavior.

use std::path::PathBuf;

#[path = "validation/parts_tree_1a.rs"]
mod parts_tree_1a;

/// Root of the SysML v2 Release tree (`SYSML_V2_RELEASE_DIR` or `./sysml-v2-release`).
pub(crate) fn release_root() -> PathBuf {
    std::env::var_os("SYSML_V2_RELEASE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml-v2-release"))
}

/// Path to a fixture under `sysml/src/validation/`.
pub(crate) fn validation_fixture_path(relative: &str) -> PathBuf {
    release_root()
        .join("sysml")
        .join("src")
        .join("validation")
        .join(relative)
}

/// Initialize the logger. Default level is WARN so failures don't flood with DEBUG.
/// Set `RUST_LOG=debug` (or `RUST_LOG=sysml_v2_parser=debug`) when debugging.
pub(crate) fn init_log() {
    let mut builder = env_logger::Builder::from_default_env();
    if std::env::var("RUST_LOG").is_err() {
        builder.filter_level(log::LevelFilter::Warn);
    }
    let _ = builder.try_init();
}

fn diff_debug_strings(parsed: &str, expected: &str) -> (usize, String) {
    let pos = parsed
        .chars()
        .zip(expected.chars())
        .position(|(a, b)| a != b)
        .unwrap_or(parsed.len().min(expected.len()));
    let snippet: String = parsed
        .chars()
        .skip(pos.saturating_sub(80))
        .take(160)
        .collect();
    (pos, snippet)
}

/// Asserts that parsed and expected ASTs are equal. Normalizes parsed (strips optional
/// spans) so comparison matches hand-built expected AST. On failure, panics with a short
/// message (first difference position and snippet) instead of dumping full ASTs.
pub(crate) fn assert_ast_eq(
    parsed: &sysml_v2_parser::ast::RootNamespace,
    expected: &sysml_v2_parser::ast::RootNamespace,
    msg: &str,
) {
    let normalized = parsed.normalize_for_test_comparison();
    if normalized == *expected {
        return;
    }
    let pa = format!("{normalized:?}");
    let pe = format!("{expected:?}");
    let (pos, snippet) = diff_debug_strings(&pa, &pe);
    panic!(
        "{msg}: AST mismatch at char {pos} (parsed {} chars, expected {} chars). Snippet: ...{snippet}... \
         Set RUST_LOG=debug and run with --nocapture for full parser trace.",
        pa.len(),
        pe.len(),
    );
}

/// Compare parsed AST against a checked-in snapshot under `tests/validation/snapshots/`.
/// Regenerate with `UPDATE_VALIDATION_AST=1 cargo test --test validation -- --include-ignored`.
pub(crate) fn assert_ast_snapshot(
    parsed: &sysml_v2_parser::ast::RootNamespace,
    snapshot_name: &str,
    msg: &str,
) {
    let normalized = format!("{:?}", parsed.normalize_for_test_comparison());
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("validation")
        .join("snapshots")
        .join(format!("{snapshot_name}.txt"));

    if std::env::var("UPDATE_VALIDATION_AST").as_deref() == Ok("1") {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create snapshot dir");
        }
        std::fs::write(&path, &normalized).expect("write snapshot");
        return;
    }

    let expected = std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "read snapshot {}: {err}. Run fetch script for sysml-v2-release, or regenerate with UPDATE_VALIDATION_AST=1",
            path.display()
        )
    });
    if normalized == expected {
        return;
    }
    let (pos, snippet) = diff_debug_strings(&normalized, &expected);
    panic!(
        "{msg}: AST mismatch at char {pos} (parsed {} chars, expected {} chars). Snippet: ...{snippet}... \
         Regenerate with UPDATE_VALIDATION_AST=1 cargo test --test validation -- --include-ignored",
        normalized.len(),
        expected.len(),
    );
}

#[path = "validation/parts_interconnection_2a.rs"]
mod parts_interconnection_2a;

#[path = "validation/function_based_behavior_3a.rs"]
mod function_based_behavior_3a;

#[path = "validation/functional_allocation_4a.rs"]
mod functional_allocation_4a;

#[path = "validation/full_validation_suite.rs"]
mod full_validation_suite;

#[path = "validation/full_library_suite.rs"]
mod full_library_suite;

#[path = "validation/surveillance_drone.rs"]
mod surveillance_drone;

#[path = "validation/surveillance_drone_minimal.rs"]
mod surveillance_drone_minimal;

#[path = "validation/traffic_light_intersection.rs"]
mod traffic_light_intersection;

#[path = "validation/kitchen_timer.rs"]
mod kitchen_timer;

#[path = "validation/use_case_ast_shapes.rs"]
mod use_case_ast_shapes;

#[path = "validation/action_ast_shapes.rs"]
mod action_ast_shapes;

#[path = "validation/vehicle_annex_a_example.rs"]
mod vehicle_annex_a_example;

#[path = "validation/parse_entry_point_equivalence.rs"]
mod parse_entry_point_equivalence;
