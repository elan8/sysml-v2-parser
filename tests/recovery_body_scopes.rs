use sysml_v2_parser::ast::{
    PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement, RequirementDefBody,
    RequirementDefBodyElement, RootElement, StateDefBody, StateDefBodyElement, UseCaseDefBody,
    UseCaseDefBodyElement,
};
use sysml_v2_parser::parse_with_diagnostics;

#[test]
fn requirement_recovery_keeps_later_members() {
    let input = "package P {\nrequirement def R {\nsubject laptop: ;\nrequire constraint { }\n}\n}";
    let result = parse_with_diagnostics(input);
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let req = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::RequirementDef(r) => Some(&r.value),
            _ => None,
        })
        .expect("requirement def should be present");
    let RequirementDefBody::Brace { elements, .. } = &req.body else {
        panic!("expected requirement body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, RequirementDefBodyElement::Error(_))),
        "malformed requirement member should be preserved as an error node"
    );
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, RequirementDefBodyElement::RequireConstraint(_))),
        "later requirement members should still parse"
    );
}

#[test]
fn use_case_recovery_keeps_later_members() {
    let input = "package P {\nuse case def U {\nactor user: ;\nobjective { }\n}\n}";
    let result = parse_with_diagnostics(input);
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let use_case = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::UseCaseDef(u) => Some(&u.value),
            _ => None,
        })
        .expect("use case def should be present");
    let UseCaseDefBody::Brace { elements, .. } = &use_case.body else {
        panic!("expected use case body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, UseCaseDefBodyElement::Error(_))),
        "malformed use case member should be preserved as an error node"
    );
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, UseCaseDefBodyElement::Objective(_))),
        "later use case members should still parse"
    );
}

#[test]
fn state_recovery_keeps_later_members() {
    // Named usage with typing started but type missing (anonymous `state: Mode;` is legal).
    let input = "package P {\nstate def Machine {\nstate ready: ;\ntransition t then Ready;\n}\n}";
    let result = parse_with_diagnostics(input);
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let state_def = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::StateDef(s) => Some(&s.value),
            _ => None,
        })
        .expect("state def should be present");
    let StateDefBody::Brace { elements, .. } = &state_def.body else {
        panic!("expected state body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, StateDefBodyElement::Error(_))),
        "malformed state member should be preserved as an error node; got {:?}",
        elements
            .iter()
            .map(|e| format!("{:?}", e.value))
            .collect::<Vec<_>>()
    );
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, StateDefBodyElement::Transition(_))),
        "later state members should still parse"
    );
}

#[test]
fn state_body_bare_identifier_reports_targeted_diagnostic_and_keeps_transition() {
    let input = "package P {\nstate def Machine {\nReady;\ntransition t then Ready;\n}\n}";
    let result = parse_with_diagnostics(input);
    let err = result
        .errors
        .iter()
        .find(|e| e.code.as_deref() == Some("invalid_bare_identifier_in_state_body"))
        .expect("expected targeted state bare identifier diagnostic");
    assert_eq!(err.line, Some(3));
    assert_eq!(
        err.expected.as_deref(),
        Some("state body member such as `entry`, `transition`, `then`, `state`, or `ref`")
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let state_def = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::StateDef(s) => Some(&s.value),
            _ => None,
        })
        .expect("state def should be present");
    let StateDefBody::Brace { elements, .. } = &state_def.body else {
        panic!("expected state body");
    };
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, StateDefBodyElement::Error(_))));
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, StateDefBodyElement::Transition(_))));
}

#[test]
fn part_def_recovery_keeps_later_members() {
    let input = "package P {\npart def Vehicle {\nattribute mass: ;\nport p : Port;\n}\n}";
    let result = parse_with_diagnostics(input);
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let part = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p) => Some(&p.value),
            _ => None,
        })
        .expect("part def should be present");
    let PartDefBody::Brace { elements, .. } = &part.body else {
        panic!("expected part def body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, PartDefBodyElement::Error(_))),
        "malformed part def member should be preserved as an error node"
    );
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, PartDefBodyElement::PortUsage(_))),
        "later part def members should still parse"
    );
}
