use std::path::PathBuf;

use sysml_v2_parser::ast::{
    PackageBody, PackageBodyElement, RootElement, ThenTarget, UseCaseDefBody, UseCaseDefBodyElement,
};
use sysml_v2_parser::{emit_sysml, parse_root, parse_with_diagnostics};

fn sysml_v2_release_root() -> PathBuf {
    std::env::var_os("SYSML_V2_RELEASE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml-v2-release"))
}

fn fixture_path() -> PathBuf {
    sysml_v2_release_root()
        .join("sysml")
        .join("src")
        .join("validation")
        .join("18-Use Case")
        .join("18-Use Case.sysml")
}

#[test]
fn test_use_case_validation_fixture_has_typed_succession_nodes() {
    super::init_log();

    let path = fixture_path();
    if !path.exists() {
        // Allow running without the submodule present.
        return;
    }

    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {}", path.display(), e));
    let input = input.replace("\r\n", "\n").replace('\r', "\n");

    let root = parse_root(&input).expect("fixture should parse");
    let pkg = match &root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package root element"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };

    let use_case_body = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::UseCaseUsage(u) => Some(&u.value.body),
            PackageBodyElement::UseCaseDef(u) => Some(&u.value.body),
            _ => None,
        })
        .expect("expected a use case in validation fixture");

    let sysml_v2_parser::ast::UseCaseDefBody::Brace { elements: body, .. } = use_case_body else {
        panic!("expected brace body");
    };

    assert!(
        body.iter()
            .any(|e| matches!(e.value, UseCaseDefBodyElement::FirstStmt(_))),
        "expected `first ...;` to parse as the shared action-body FirstStmt"
    );
    assert!(
        body.iter()
            .any(|e| matches!(e.value, UseCaseDefBodyElement::ThenIncludeUseCase(_))),
        "expected `then include ... {{}}` to parse as ThenIncludeUseCase"
    );
    assert!(
        body.iter()
            .any(|e| matches!(e.value, UseCaseDefBodyElement::ThenUseCaseUsage(_))),
        "expected `then use case ... {{}}` to parse as ThenUseCaseUsage"
    );
    assert!(
        body.iter()
            .any(|e| matches!(e.value, UseCaseDefBodyElement::ThenAction(_))),
        "expected `then done;` to parse as the shared action-body ThenAction"
    );
}

/// A self-contained regression for elan8/spec42#138: a use-case body is a SysML `ActionBody`,
/// so `first <node>;` / `then <target>;` / `then done;` are legal members. They must parse into
/// the shared `FirstStmt` / `ThenAction` nodes (never the removed `FirstSuccession` / `ThenDone`),
/// carry their reference targets, and round-trip through emission.
#[test]
fn use_case_body_first_then_flow_uses_shared_action_nodes() {
    super::init_log();

    let src = "package P {\n\
        \tpart def Robot;\n\
        \taction def Mission;\n\
        \tuse case run {\n\
        \t\tsubject robot : Robot;\n\
        \t\tfirst start;\n\
        \t\tthen action mission : Mission;\n\
        \t\tthen done;\n\
        \t}\n\
        }\n";

    let parsed = parse_with_diagnostics(src);
    assert!(
        parsed.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        parsed.errors
    );

    let RootElement::Package(pkg) = &parsed.document.root.elements[0].value else {
        panic!("expected package");
    };
    let PackageBody::Brace { elements, .. } = &pkg.value.body else {
        panic!("expected brace body");
    };
    let body = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::UseCaseUsage(u) => match &u.value.body {
                UseCaseDefBody::Brace { elements, .. } => Some(elements),
                UseCaseDefBody::Semicolon { .. } => None,
            },
            _ => None,
        })
        .expect("expected the use case usage");

    let first = body
        .iter()
        .find_map(|e| match &e.value {
            UseCaseDefBodyElement::FirstStmt(f) => Some(&f.value),
            _ => None,
        })
        .expect("`first start;` should be a FirstStmt");
    assert!(
        first.then.is_none(),
        "`first start;` is a bare initial-node marker with no `then` end"
    );

    let then_targets: Vec<&ThenTarget> = body
        .iter()
        .filter_map(|e| match &e.value {
            UseCaseDefBodyElement::ThenAction(t) => Some(&t.value.target),
            _ => None,
        })
        .collect();
    assert_eq!(then_targets.len(), 2, "`then action …;` and `then done;`");
    assert!(
        matches!(then_targets[0], ThenTarget::Action(_)),
        "`then action mission : Mission;` is an inline action declaration"
    );
    assert!(
        matches!(then_targets[1], ThenTarget::Feature(_)),
        "`then done;` carries a `done` feature reference, not an empty marker"
    );

    let emitted = emit_sysml(&parsed.document).expect("emit");
    assert!(
        emitted.contains("first start;")
            && emitted.contains("then action mission : Mission;")
            && emitted.contains("then done;"),
        "flow members must round-trip:\n{emitted}"
    );
    assert!(
        parse_root(&emitted).is_ok(),
        "emitted source must reparse cleanly"
    );
}
