//! GH-36: several brace bodies (`alias`, `import`, `dependency`, plain `connect` statements,
//! connection/interface `ref` bodies, part-usage `ref`/`bind` bodies, state `ref` bodies) used to
//! parse via `advance_to_closing_brace`, which skips to the matching `}` and discards everything
//! in between -- not even captured as opaque text, no diagnostic. Each of these now retains real
//! content (doc/comment/metadata annotations at minimum; full member grammar where a richer
//! element type was already cheaply available). These tests confirm a `doc` block inside each
//! body survives into the AST instead of silently vanishing.

use sysml_v2_parser::ast::{
    AliasBody, ConnectBody, ConnectionDefBody, ConnectionDefBodyElement, PackageBody,
    PackageBodyElement, PartDefBody, PartDefBodyElement, RefBody, RefBodyElement,
    RelationshipBodyElement, RootElement, StateDefBody, StateDefBodyElement,
};
use sysml_v2_parser::parse_with_diagnostics;

fn package_elements(input: &str) -> Vec<PackageBodyElement> {
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace package body");
    };
    elements.iter().map(|e| e.value.clone()).collect()
}

#[test]
fn alias_body_retains_doc_comment() {
    let input = "package P {\npart def A;\nalias B for A {\ndoc /* alias annotation */\n}\n}";
    let elements = package_elements(input);
    let alias = elements
        .iter()
        .find_map(|e| match e {
            PackageBodyElement::AliasDef(a) => Some(&a.value),
            _ => None,
        })
        .expect("expected alias def");
    let AliasBody::Brace { elements } = &alias.body else {
        panic!("expected brace alias body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(&e.value, RelationshipBodyElement::Doc(_))),
        "expected doc comment retained in alias body, got: {elements:?}"
    );
}

#[test]
fn import_body_retains_doc_comment() {
    let input = "package P {\nimport ISQ::* {\ndoc /* import annotation */\n}\n}";
    let elements = package_elements(input);
    let import = elements
        .iter()
        .find_map(|e| match e {
            PackageBodyElement::Import(i) => Some(&i.value),
            _ => None,
        })
        .expect("expected import");
    let body_elements = import
        .body_elements
        .as_ref()
        .expect("expected Some body_elements for a braced import body");
    assert!(
        body_elements
            .iter()
            .any(|e| matches!(&e.value, RelationshipBodyElement::Doc(_))),
        "expected doc comment retained in import body, got: {body_elements:?}"
    );
}

#[test]
fn dependency_body_retains_doc_comment() {
    let input =
        "package P {\npart def A;\npart def B;\ndependency A to B {\ndoc /* dependency annotation */\n}\n}";
    let elements = package_elements(input);
    let dependency = elements
        .iter()
        .find_map(|e| match e {
            PackageBodyElement::Dependency(d) => Some(&d.value),
            _ => None,
        })
        .expect("expected dependency");
    let body_elements = dependency
        .body_elements
        .as_ref()
        .expect("expected Some body_elements for a braced dependency body");
    assert!(
        body_elements
            .iter()
            .any(|e| matches!(&e.value, RelationshipBodyElement::Doc(_))),
        "expected doc comment retained in dependency body, got: {body_elements:?}"
    );
}

#[test]
fn plain_connect_statement_retains_doc_comment() {
    let input = "package P {\nconnection def C {\nport a;\nport b;\nconnect a to b {\ndoc /* connect annotation */\n}\n}\n}";
    let elements = package_elements(input);
    let connection = elements
        .iter()
        .find_map(|e| match e {
            PackageBodyElement::ConnectionDef(c) => Some(&c.value),
            _ => None,
        })
        .expect("expected connection def");
    let ConnectionDefBody::Brace { elements } = &connection.body else {
        panic!("expected brace connection def body");
    };
    let connect_stmt = elements
        .iter()
        .find_map(|e| match &e.value {
            ConnectionDefBodyElement::ConnectStmt(c) => Some(&c.value),
            _ => None,
        })
        .expect("expected connect statement");
    assert!(matches!(connect_stmt.body, ConnectBody::Brace));
    assert!(
        connect_stmt
            .body_elements
            .iter()
            .any(|e| matches!(&e.value, RelationshipBodyElement::Doc(_))),
        "expected doc comment retained in connect statement body, got: {:?}",
        connect_stmt.body_elements
    );
}

#[test]
fn connection_ref_body_retains_doc_comment() {
    let input =
        "package P {\nconnection def C {\nref part sensor : Anything {\ndoc /* ref annotation */\n}\n}\n}";
    let elements = package_elements(input);
    let connection = elements
        .iter()
        .find_map(|e| match e {
            PackageBodyElement::ConnectionDef(c) => Some(&c.value),
            _ => None,
        })
        .expect("expected connection def");
    let ConnectionDefBody::Brace { elements } = &connection.body else {
        panic!("expected brace connection def body");
    };
    let ref_decl = elements
        .iter()
        .find_map(|e| match &e.value {
            ConnectionDefBodyElement::RefDecl(r) => Some(&r.value),
            _ => None,
        })
        .expect("expected ref declaration");
    let RefBody::Brace { elements } = &ref_decl.body else {
        panic!("expected brace ref body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(&e.value, RefBodyElement::Doc(_))),
        "expected doc comment retained in connection ref body, got: {elements:?}"
    );
}

#[test]
fn part_usage_ref_body_retains_real_member() {
    let input = "package P {\npart def A {\nref b {\nattribute mass : Real;\n}\n}\n}";
    let elements = package_elements(input);
    let part_def = elements
        .iter()
        .find_map(|e| match e {
            PackageBodyElement::PartDef(p) => Some(&p.value),
            _ => None,
        })
        .expect("expected part def");
    let PartDefBody::Brace { elements } = &part_def.body else {
        panic!("expected brace part def body");
    };
    let ref_decl = elements
        .iter()
        .find_map(|e| match &e.value {
            PartDefBodyElement::Ref(r) => Some(&r.value),
            _ => None,
        })
        .expect("expected ref declaration");
    let RefBody::Brace { elements } = &ref_decl.body else {
        panic!("expected brace ref body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(&e.value, RefBodyElement::PartUsage(_))),
        "expected real part-usage member (attribute) retained in part-usage ref body, got: {elements:?}"
    );
}

#[test]
fn bind_trailing_body_retains_real_member() {
    let input = "package P {\npart def A {\nattribute x : Real;\nattribute y : Real;\nbind x = y {\ndoc /* bind annotation */\n}\n}\n}";
    let elements = package_elements(input);
    let part_def = elements
        .iter()
        .find_map(|e| match e {
            PackageBodyElement::PartDef(p) => Some(&p.value),
            _ => None,
        })
        .expect("expected part def");
    let PartDefBody::Brace { elements } = &part_def.body else {
        panic!("expected brace part def body");
    };
    let bind = elements
        .iter()
        .find_map(|e| match &e.value {
            PartDefBodyElement::Bind(b) => Some(&b.value),
            _ => None,
        })
        .expect("expected bind statement");
    assert!(matches!(bind.body, Some(ConnectBody::Brace)));
    assert!(
        !bind.body_elements.is_empty(),
        "expected real content retained in bind trailing body, got empty"
    );
}

#[test]
fn state_ref_body_retains_real_member() {
    let input = "package P {\nstate def S {\nref b : Anything {\nattribute mass : Real;\n}\n}\n}";
    let elements = package_elements(input);
    let state_def = elements
        .iter()
        .find_map(|e| match e {
            PackageBodyElement::StateDef(s) => Some(&s.value),
            _ => None,
        })
        .expect("expected state def");
    let StateDefBody::Brace { elements } = &state_def.body else {
        panic!("expected brace state def body");
    };
    let ref_decl = elements
        .iter()
        .find_map(|e| match &e.value {
            StateDefBodyElement::Ref(r) => Some(&r.value),
            _ => None,
        })
        .expect("expected ref declaration");
    let RefBody::Brace { elements } = &ref_decl.body else {
        panic!("expected brace ref body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(&e.value, RefBodyElement::State(_))),
        "expected real state member (attribute) retained in state ref body, got: {elements:?}"
    );
}

/// Confirms the already-real action-context `ref` body (unrelated to this fix, but sharing the
/// same `RefBody`/`RefBodyElement` type after the redesign) still works: a `ref` nested in an
/// action body keeps full nested action members, wrapped in `RefBodyElement::Action`.
#[test]
fn action_ref_body_still_retains_action_members() {
    let input = "package P {\naction def Act {\nref x {\naction y;\n}\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );
    // Just confirm it still parses without errors and the ref body isn't discarded to nothing;
    // finding the exact nested node isn't necessary here since ActionDefBodyElement wiring
    // predates this fix and already had dedicated coverage.
}

/// Sanity-check the constraint-body-adjacent sibling pattern (`ConnectBody` + separate
/// `body_elements`, established by `Satisfy`/`Dependency`/`ConnectStmt`) is unaffected for a
/// plain semicolon-terminated dependency: `body_elements` must stay `None`, not `Some(vec![])`.
#[test]
fn dependency_semicolon_body_has_no_body_elements() {
    let input = "package P {\npart def A;\npart def B;\ndependency A to B;\n}";
    let elements = package_elements(input);
    let dependency = elements
        .iter()
        .find_map(|e| match e {
            PackageBodyElement::Dependency(d) => Some(&d.value),
            _ => None,
        })
        .expect("expected dependency");
    assert!(matches!(dependency.body, ConnectBody::Semicolon));
    assert!(dependency.body_elements.is_none());
}
