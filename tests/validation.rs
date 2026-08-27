//! Integration tests that parse SysML validation fixture files.
//!
//! Each validation .sysml file has a corresponding test module under `validation/`
//! for easier maintenance.
//!
//! Logging defaults to WARN so test output stays small. Use `RUST_LOG=debug` (or
//! `RUST_LOG=sysml_v2_parser=debug`) and `--nocapture` when debugging parser behavior.

use std::path::PathBuf;

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

#[path = "validation/parts_interconnection_2a.rs"]
mod parts_interconnection_2a;

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
