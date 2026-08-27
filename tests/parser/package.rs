//! Parser tests: package

use super::common::*;
use sysml_v2_parser::ast::Span;
use sysml_v2_parser::ast::*;
use sysml_v2_parser::{parse, parse_with_diagnostics};

#[test]
fn test_package_with_semicolon_body() {
    let input = "package Foo;";
    let result = parse(input).expect("parse should succeed");
    let expected = expected_package_foo_semicolon(&result);
    assert_eq!(
        result.root.normalize_for_test_comparison(),
        expected.normalize_for_test_comparison(),
        "AST should match expected for package Foo;"
    );
}

#[test]
fn test_package_with_brace_body() {
    let input = "package Bar { }";
    let result = parse(input).expect("parse should succeed");
    let expected = expected_package_bar_brace(&result);
    assert_eq!(
        result.root.normalize_for_test_comparison(),
        expected.normalize_for_test_comparison(),
        "AST should match expected for package Bar {{ }}"
    );
}

#[test]
fn test_standard_library_package_header_parses() {
    let input = "standard library package SysML { }";
    let result = parse(input).expect("parse should succeed");
    assert_eq!(result.elements.len(), 1);
    match &result.elements[0].value {
        RootElement::LibraryPackage(lp) => {
            assert!(lp.value.is_standard);
            assert_eq!(
                lp.value
                    .identification
                    .simple_name()
                    .and_then(|n| result.declaration_name(n)),
                Some("SysML")
            );
            assert!(
                matches!(lp.value.body, PackageBody::Brace { ref elements, .. } if elements.is_empty())
            );
        }
        other => panic!("expected library package, got {:?}", other),
    }
}

#[test]
fn test_legacy_library_standard_package_header_still_parses() {
    let input = "library standard package LegacyStd;";
    let result = parse(input).expect("parse should succeed");
    assert_eq!(
        result.root.normalize_for_test_comparison(),
        RootNamespace {
            elements: vec![n_len(
                input.len(),
                RootElement::LibraryPackage(n_len(
                    input.len(),
                    LibraryPackage {
                        is_standard: true,
                        identification: package_id(&result, "LegacyStd"),
                        body: PackageBody::Semicolon {
                            semicolon_span: Span::dummy(),
                        },
                    }
                ))
            )]
        }
        .normalize_for_test_comparison()
    );
}

#[test]
fn test_library_abstract_action_feature_decl_parses_without_diagnostics() {
    // Representative Systems Library syntax (Actions.sysml): abstract action feature with typing,
    // multiplicity, modifier, and specialization, with a doc-only body.
    let input = r#"package P {
abstract action sendActions: SendAction[0..*] nonunique :> actions, sendPerformances {
  doc /* sendActions is the base feature for SendActionUsages. */
}
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "expected no diagnostics; got: {:?}",
        result.errors
    );
}

#[test]
fn test_library_return_assignment_form_parses_without_diagnostics() {
    // Representative Domain library syntax: `return name = expr;`
    let input = r#"package P {
calc def C {
  return result = integrate.result;
}
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "expected no diagnostics; got: {:?}",
        result.errors
    );
}

// --- Top-level import (Phase 0: BNF RootNamespace = PackageBodyElement*) ---

#[test]
fn test_root_level_import_then_package() {
    let input = "private import Views::*;\npackage P { }";
    let result = parse(input).expect("parse should succeed");
    assert_eq!(result.elements.len(), 2);
    match &result.elements[0].value {
        sysml_v2_parser::ast::RootElement::Import(_) => {}
        _ => panic!("expected first element to be Import"),
    }
    match &result.elements[1].value {
        sysml_v2_parser::ast::RootElement::Package(p) => {
            assert_eq!(
                p.identification
                    .simple_name()
                    .and_then(|n| result.declaration_name(n)),
                Some("P")
            );
        }
        _ => panic!("expected second element to be Package"),
    }
}

// --- View/Viewpoint/Rendering (spec-1: Clause 8.2.2.26) ---

#[test]
fn test_stdlib_requirement_usecase_enum_map_to_dedicated_nodes() {
    let input = "package P {
        abstract requirement def RequirementCheck :> BaseType { }
        use case def UseCase :> Case { }
        enum def VerdictKind { pass; fail; }
    }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements, .. } => elements,
        _ => panic!("expected brace body"),
    };
    assert!(matches!(
        elements[0].value,
        PackageBodyElement::RequirementDef(_)
    ));
    let PackageBodyElement::RequirementDef(req) = &elements[0].value else {
        panic!("expected requirement def");
    };
    assert_eq!(
        req.value.specializes.as_ref().map(|n| n.value.target.len()),
        Some(1)
    );
    assert!(matches!(
        elements[1].value,
        PackageBodyElement::UseCaseDef(_)
    ));
    let PackageBodyElement::UseCaseDef(uc) = &elements[1].value else {
        panic!("expected use case def");
    };
    assert_eq!(
        uc.value.specializes.as_ref().map(|n| n.value.target.len()),
        Some(1)
    );
    assert!(matches!(elements[2].value, PackageBodyElement::EnumDef(_)));
}

#[test]
fn test_stdlib_part_port_viewpoint_map_to_dedicated_nodes() {
    let input = "package P {
        abstract part def Part :> Item { }
        abstract port def Port :> Object { }
        abstract viewpoint def ViewpointCheck :> RequirementCheck { }
    }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements, .. } => elements,
        _ => panic!("expected brace body"),
    };
    assert!(matches!(elements[0].value, PackageBodyElement::PartDef(_)));
    let PackageBodyElement::PartDef(part) = &elements[0].value else {
        panic!("expected part def");
    };
    assert_eq!(
        part.value
            .specializes
            .as_ref()
            .map(|n| n.value.target.len()),
        Some(1)
    );
    assert!(matches!(elements[1].value, PackageBodyElement::PortDef(_)));
    let PackageBodyElement::PortDef(port) = &elements[1].value else {
        panic!("expected port def");
    };
    assert_eq!(
        port.value
            .specializes
            .as_ref()
            .map(|n| n.value.target.len()),
        Some(1)
    );
    assert!(matches!(
        elements[2].value,
        PackageBodyElement::ViewpointDef(_)
    ));
    let PackageBodyElement::ViewpointDef(vp) = &elements[2].value else {
        panic!("expected viewpoint def");
    };
    assert_eq!(
        vp.value.specializes.as_ref().map(|n| n.value.target.len()),
        Some(1)
    );
    assert!(
        !elements
            .iter()
            .any(|e| matches!(e.value, PackageBodyElement::ExtendedLibraryDecl(_))),
        "sample should not fall back to ExtendedLibraryDecl"
    );
}

#[test]
fn test_quantities_abstract_attribute_def_maps_dedicated() {
    let input = "package P { abstract attribute def TensorQuantityValue :> Array { attribute num: Number[1..*]; } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements, .. } => elements,
        _ => panic!("expected brace body"),
    };
    assert!(matches!(
        elements[0].value,
        PackageBodyElement::AttributeDef(_)
    ));
}

#[test]
fn test_comment_about_member_does_not_consume_next_package() {
    let input = r#"package P {
part def BMS {
}
comment about BMS
/* BMS = Battery Management System */
}
package Next {
  part def BatteryLevelComputer;
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "comment about should parse without package-boundary recovery: {:?}",
        result.errors
    );
    assert_eq!(result.document.root.elements.len(), 2);
}

#[test]
fn test_parse_package_with_quoted_name() {
    let input = "package '15.10-Primitive Data Types' { }";
    let result = parse(input).expect("quoted package names should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    assert_eq!(
        pkg.identification
            .simple_name()
            .and_then(|n| result.declaration_name(n)),
        Some("'15.10-Primitive Data Types'")
    );
    assert_eq!(
        pkg.identification
            .simple_name()
            .and_then(|n| result.decoded_declaration_name(n))
            .as_deref(),
        Some("15.10-Primitive Data Types")
    );
}
