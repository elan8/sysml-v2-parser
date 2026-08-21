//! Parser tests for `tests/fixtures/SurveillanceDrone*.sysml`.

use std::path::Path;
use sysml_v2_parser::ast::{
    AnnotatingMember, Package, PackageBody, PackageBodyElement, PartDef, RootElement,
};
use sysml_v2_parser::{parse, parse_with_diagnostics};

/// Path to the SurveillanceDrone fixture (project-local, not sysml-v2-release).
fn surveillance_drone_fixture_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("SurveillanceDrone.sysml")
}

/// Path to the historical SurveillanceDrone-error.sysml probe.
fn surveillance_drone_error_fixture_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("SurveillanceDrone-error.sysml")
}

/// Path to the historical multi-package SurveillanceDrone-errors.sysml probe.
fn surveillance_drone_errors_fixture_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("SurveillanceDrone-errors.sysml")
}

fn assert_typed_package_member_on_line(package: &Package, line: u32) {
    let PackageBody::Brace { elements, .. } = &package.body else {
        panic!("expected a braced package body");
    };
    let member = elements
        .iter()
        .find(|member| member.span.line == line)
        .unwrap_or_else(|| panic!("expected a package member on source line {line}"));
    assert!(
        !matches!(
            member.value,
            PackageBodyElement::Error(_) | PackageBodyElement::Unsupported(_)
        ),
        "source line {line} must remain a typed member, got {:?}",
        member.value
    );
}

#[test]
fn test_parse_surveillance_drone() {
    super::init_log();
    let path = surveillance_drone_fixture_path();
    log::debug!("fixture path: {}", path.display());
    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {}", path.display(), e));
    let input = input.replace("\r\n", "\n").replace('\r', "\n");
    log::debug!("input len: {} bytes", input.len());

    let result = parse(&input);
    let root = match &result {
        Ok(ast) => ast,
        Err(e) => panic!("parse should succeed for SurveillanceDrone.sysml: {:?}", e),
    };

    // The file's leading `/* ... */` header is the keyword-less `Comment` spelling, so it is a
    // root member of its own ahead of the package.
    assert_eq!(
        root.elements.len(),
        2,
        "expected the header comment and the package SurveillanceDrone"
    );
    let first = &root.elements[1];
    let package = match &first.value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected root to be a Package, got {:?}", other),
    };
    assert_eq!(
        package.identification.simple_name(),
        Some("SurveillanceDrone"),
        "root package should be named SurveillanceDrone"
    );

    let body = match &package.body {
        sysml_v2_parser::ast::PackageBody::Brace { elements, .. } => elements,
        _ => panic!("expected package body to be brace form"),
    };

    // Count key top-level constructs present in the fixture (partial parse may not have all)
    let has_part_def = body
        .iter()
        .any(|e| matches!(&e.value, PackageBodyElement::PartDef(_)));
    let has_requirement_def = body
        .iter()
        .any(|e| matches!(&e.value, PackageBodyElement::RequirementDef(_)));
    let has_use_case_def = body
        .iter()
        .any(|e| matches!(&e.value, PackageBodyElement::UseCaseDef(_)));
    let has_state_def = body
        .iter()
        .any(|e| matches!(&e.value, PackageBodyElement::StateDef(_)));
    let has_constraint_def = body
        .iter()
        .any(|e| matches!(&e.value, PackageBodyElement::ConstraintDef(_)));
    let has_calc_def = body
        .iter()
        .any(|e| matches!(&e.value, PackageBodyElement::CalcDef(_)));
    let has_satisfy = body
        .iter()
        .any(|e| matches!(&e.value, PackageBodyElement::Satisfy(_)));
    let has_doc = body.iter().any(|e| {
        matches!(
            &e.value,
            PackageBodyElement::Annotating(AnnotatingMember::Doc(_))
        )
    });

    assert!(
        has_doc,
        "doc comments must be parsed as Doc elements in the AST, not skipped"
    );
    assert!(has_part_def, "fixture should contain part defs");
    assert!(
        has_requirement_def,
        "fixture should contain requirement defs"
    );
    assert!(has_use_case_def, "fixture should contain use case defs");
    assert!(has_state_def, "fixture should contain state defs");
    assert!(has_constraint_def, "fixture should contain constraint defs");
    assert!(has_calc_def, "fixture should contain calc defs");
    assert!(has_satisfy, "fixture should contain satisfy statements");

    // Line 363: part def SurveillanceQuadrotorDroneWithBehavior :> SurveillanceQuadrotorDrone {
    // Assert that specializes_span is set for the ":> SurveillanceQuadrotorDrone" fragment.
    let part_def_specializes_span = body
        .iter()
        .filter_map(|e| {
            if let PackageBodyElement::PartDef(n) = &e.value {
                Some(&n.value)
            } else {
                None
            }
        })
        .find(|p: &&PartDef| {
            p.identification.name.as_deref() == Some("SurveillanceQuadrotorDroneWithBehavior")
                && p.specializes.as_ref().map(|n| n.value.target.len()) == Some(1)
        });
    let part_def = part_def_specializes_span
        .expect("fixture should contain part def SurveillanceQuadrotorDroneWithBehavior :> SurveillanceQuadrotorDrone");
    assert!(
        part_def.specializes.is_some(),
        "specializes_span must be set when parsing ':> SurveillanceQuadrotorDrone' on line 363"
    );
    let span = &part_def.specializes.as_ref().unwrap().value.span;
    assert_eq!(
        span.line, 363,
        "specializes_span should point to line 363 (':> SurveillanceQuadrotorDrone')"
    );
    let fragment = &input[span.offset..span.offset + span.len];
    assert!(
        fragment.contains(":> SurveillanceQuadrotorDrone"),
        "specializes_span should cover ':> SurveillanceQuadrotorDrone', got: {:?}",
        fragment
    );
}

/// The historical `test {}` probe is now accepted as a complete package member. Keep the exact
/// source fixture as a regression case for that widened dispatch rather than continuing to call
/// it invalid.
#[test]
fn test_surveillance_drone_error_fixture_now_parses_cleanly() {
    super::init_log();
    let path = surveillance_drone_error_fixture_path();
    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {}", path.display(), e));
    let input = input.replace("\r\n", "\n").replace('\r', "\n");

    let strict = parse(&input).expect("the complete historical probe should parse strictly");
    assert!(
        strict.elements.len() == 2,
        "fixture should preserve its header comment and package"
    );
    let RootElement::Package(package) = &strict.elements[1].value else {
        panic!("historical probe should retain its root package");
    };
    assert_typed_package_member_on_line(&package.value, 333);
    assert_typed_package_member_on_line(&package.value, 364);
    let result = parse_with_diagnostics(&input);
    assert!(
        result.errors.is_empty(),
        "historical `test {{}}`/`test2 {{}}` members are now typed, not recovery: {:?}",
        result.errors
    );
}

/// The two formerly-error-labelled members must stay observable in the editor/strict equivalence
/// path: no diagnostic-free source may get a partial editor tree.
#[test]
fn test_surveillance_drone_error_fixture_editor_matches_strict_root() {
    super::init_log();
    let path = surveillance_drone_error_fixture_path();
    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {}", path.display(), e));
    let input = input.replace("\r\n", "\n").replace('\r', "\n");

    let strict = parse(&input).expect("historical error fixture should parse strictly");
    let result = parse_with_diagnostics(&input);
    assert!(
        result.errors.is_empty(),
        "editor parse should share strict acceptance: {:?}",
        result.errors
    );
    assert_eq!(result.document.root, strict.root);
}

/// The multi-package historical error probe now parses cleanly. Its package boundaries remain a
/// useful recovery-regression input, so retain it and assert the complete ordered root instead of
/// stale diagnostics for `test`/`xyz`/`badstmt`.
#[test]
fn test_surveillance_drone_errors_fixture_now_parses_cleanly() {
    super::init_log();
    let path = surveillance_drone_errors_fixture_path();
    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {}", path.display(), e));
    let input = input.replace("\r\n", "\n").replace('\r', "\n");

    let strict = parse(&input).expect("historical multi-package probe should parse strictly");
    let result = parse_with_diagnostics(&input);
    assert_eq!(
        result.errors.len(),
        0,
        "historical probe is now fully typed: {:?}",
        result.errors
    );

    // The file's leading `/* ... */` header is a root member of its own, followed by every source
    // package in authored order.
    assert_eq!(
        result.document.root.elements.len(),
        5,
        "partial AST should contain the header comment and all four packages"
    );
    let first = match &result.document.root.elements[1].value {
        RootElement::Package(p) => &p.value,
        other => panic!(
            "expected first root element to be a Package, got {:?}",
            other
        ),
    };
    assert_eq!(
        first.identification.simple_name(),
        Some("SurveillanceDroneFirst"),
        "first package should be SurveillanceDroneFirst"
    );
    let expected_typed_members = [(2, 15), (3, 19), (4, 23)];
    for (root_index, source_line) in expected_typed_members {
        let RootElement::Package(package) = &strict.elements[root_index].value else {
            panic!("expected root element {root_index} to be a package");
        };
        assert_typed_package_member_on_line(&package.value, source_line);
    }

    assert_eq!(result.document.root, strict.root);
}
