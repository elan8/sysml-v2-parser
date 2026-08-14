//! Regression test for a stack overflow (`STATUS_STACK_OVERFLOW`) previously hit while parsing
//! the SysML v2 spec's own canonical example, `sysml/src/examples/Vehicle Example/SysML v2 Spec
//! Annex A SimpleVehicleModel.sysml`.
//!
//! The file's legitimate (bounded) nesting was deep enough that debug builds -- which spend far
//! more stack per recursive-descent call frame than release builds -- could exhaust a caller's
//! default thread stack and abort the whole process instead of returning a [`ParseError`]. See
//! `with_parse_stack` in `src/parser/parse.rs`. This test intentionally runs on whatever thread
//! `cargo test` gives it (no custom stack size) so it actually exercises that fix.

use sysml_v2_parser::ast::RootElement;
use sysml_v2_parser::parse_with_diagnostics;

fn fixture_path() -> std::path::PathBuf {
    super::release_root()
        .join("sysml")
        .join("src")
        .join("examples")
        .join("Vehicle Example")
        .join("SysML v2 Spec Annex A SimpleVehicleModel.sysml")
}

#[test]
#[ignore = "requires SysML v2 release fixtures; run with: cargo test --test validation -- --include-ignored"]
fn test_parse_vehicle_annex_a_example_does_not_crash() {
    super::init_log();

    let path = fixture_path();
    if !path.exists() {
        log::debug!("Fixture not found: {:?}", path);
        log::debug!("Skipping. Run `scripts/fetch-sysml-v2-release.*` or set SYSML_V2_RELEASE_DIR");
        return;
    }

    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {}", path.display(), e));

    // The crash regression: parsing this file must return normally (with a partial AST plus
    // diagnostics for any unsupported constructs), never abort the process.
    let result = parse_with_diagnostics(&input);

    for error in &result.errors {
        eprintln!(
            "[line={:?}, col={:?}, code={:?}] {}",
            error.line, error.column, error.code, error.message
        );
    }

    assert_eq!(
        result.document.root.elements.len(),
        1,
        "expected exactly one top-level package"
    );
    assert!(
        matches!(
            &result.document.root.elements[0].value,
            RootElement::Package(p) if p.value.identification.simple_name() == Some("SimpleVehicleModel")
        ),
        "expected the single top-level element to be the `SimpleVehicleModel` package"
    );

    // Diagnostics, including explicit UnsupportedGrammarForm nodes, are covered by the semantic
    // snapshot corpus. This regression guards normal return and recovered top-level shape; a
    // numeric diagnostic ceiling would conflate newly visible unsupported syntax with breakage.
}
