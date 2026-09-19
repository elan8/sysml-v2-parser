//! Regression test for a stack overflow (`SIGABRT`, "thread has overflowed its stack")
//! previously hit while parsing the SysML v2 spec's own `Vehicle Example/VehicleIndividuals.sysml`
//! example on a 2 MiB thread stack in a debug build.
//!
//! Unlike `vehicle_annex_a_example.rs`'s crash (fixed by routing `parse_root`/
//! `parse_with_diagnostics` through `stacker::maybe_grow`), this file's nesting is shallow (6
//! levels, well under `MAX_SYNTAX_NESTING`) -- what made it costly was the combination of an
//! `individual` declaration with a redefinition/multi-specialization header and a body whose sole
//! member is a `doc` comment (its own multi-step `identification`/`locale`/`comment_body` parse
//! chain), which together cost enough per level that the existing 1 MiB `NESTED_BODY_RED_ZONE`
//! could be exhausted *within* a single level's own work, before the next level's own headroom
//! check ran. See `NESTED_BODY_RED_ZONE` in `src/parser/stack.rs`. This test intentionally runs
//! on whatever thread `cargo test` gives it (no custom stack size) so it actually exercises the
//! fix.

use sysml_v2_parser::ast::RootElement;
use sysml_v2_parser::parse;

fn fixture_path() -> std::path::PathBuf {
    super::release_root()
        .join("sysml")
        .join("src")
        .join("examples")
        .join("Vehicle Example")
        .join("VehicleIndividuals.sysml")
}

#[test]
#[ignore = "requires SysML v2 release fixtures; run with: cargo test --test validation -- --include-ignored"]
fn test_parse_vehicle_individuals_example_does_not_crash() {
    super::init_log();

    let path = fixture_path();
    if !path.exists() {
        log::debug!("Fixture not found: {:?}", path);
        log::debug!("Skipping. Run `scripts/fetch-sysml-v2-release.*` or set SYSML_V2_RELEASE_DIR");
        return;
    }

    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {}", path.display(), e));

    // The crash regression: parsing this file must return normally, never abort the process.
    // Unlike the Annex A fixture, this file is clean (no recovery-worthy constructs), so the
    // strict `parse()` entry point -- the one kr0ki (a downstream consumer) actually calls --
    // is the more faithful regression guard here.
    let document = parse(&input).unwrap_or_else(|e| panic!("parse {}: {}", path.display(), e));

    assert_eq!(
        document.root.elements.len(),
        1,
        "expected exactly one top-level package"
    );
    assert!(
        matches!(
            &document.root.elements[0].value,
            RootElement::Package(p) if p.value.identification.simple_name().and_then(|n| document.declaration_name(n)) == Some("VehicleIndividuals")
        ),
        "expected the single top-level element to be the `VehicleIndividuals` package"
    );
}
