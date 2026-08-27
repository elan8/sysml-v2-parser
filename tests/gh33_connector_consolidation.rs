//! GH-33: `connection.rs` and `interface.rs` independently implemented the same seven
//! connector-end functions (`end_decl`, `ref_body`, `ref_decl`, `connect_body`, a connection-end
//! wrapper, `connect_ends`, `connect_stmt`), which had already cost real double work once (#19)
//! and had silently drifted into behavior gaps neither file's own tests caught. Both now share one
//! implementation in `src/parser/connector.rs`.
//!
//! These tests assert that a capability added to the shared implementation is visible from both
//! `connection_def` and `interface_def` parsing -- the acceptance criterion this issue asked for,
//! so a future two-file-fix regression like #19 would be caught structurally here instead of
//! relying on remembering to test both files.

use sysml_v2_parser::ast::{
    ConnectionDefBody, ConnectionDefBodyElement, InterfaceDefBody, InterfaceDefBodyElement,
    PackageBody, PackageBodyElement, RootElement,
};
use sysml_v2_parser::parse_with_diagnostics;

fn package_elements(input: &str) -> Vec<sysml_v2_parser::Node<PackageBodyElement>> {
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    match &pkg.body {
        PackageBody::Brace { elements, .. } => elements.clone(),
        _ => panic!("expected brace package body"),
    }
}

/// Both `connection def` and `interface def` `end` declarations accept the `~` conjugated-type
/// prefix (e.g. `end p1 : ~PowerPort;`). Before GH-33, `connection.rs`'s `end_decl` lacked this
/// entirely -- only `interface.rs`'s had it, with no BNF basis for the restriction (`ConnectorEnd`
/// and `InterfaceEnd` are the same production). Both now go through `connector::end_decl`.
#[test]
fn conjugated_end_type_works_in_both_connection_and_interface_defs() {
    let input = "package P {\nport def PowerPort;\nconnection def C { end p1 : ~PowerPort; }\ninterface def I { end p1 : ~PowerPort; }\n}";
    let elements = package_elements(input);

    let connection = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::ConnectionDef(c) => Some(&c.value),
            _ => None,
        })
        .expect("expected connection def");
    let ConnectionDefBody::Brace {
        elements: connection_elements,
        ..
    } = &connection.body
    else {
        panic!("expected connection def brace body");
    };
    let end = connection_elements
        .iter()
        .find_map(|e| match &e.value {
            ConnectionDefBodyElement::EndDecl(end) => Some(&end.value),
            _ => None,
        })
        .expect("expected end decl in connection def");
    assert!(end.typing.as_ref().is_some_and(|typing| {
        typing.value.is_conjugated && typing.value.first_target().is_some()
    }));

    let interface = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::InterfaceDef(i) => Some(&i.value),
            _ => None,
        })
        .expect("expected interface def");
    let InterfaceDefBody::Brace {
        elements: interface_elements,
        ..
    } = &interface.body
    else {
        panic!("expected interface def brace body");
    };
    let end = interface_elements
        .iter()
        .find_map(|e| match &e.value {
            InterfaceDefBodyElement::EndDecl(end) => Some(&end.value),
            _ => None,
        })
        .expect("expected end decl in interface def");
    assert!(end.typing.as_ref().is_some_and(|typing| {
        typing.value.is_conjugated && typing.value.first_target().is_some()
    }));
}

/// Both `connection def` and `interface def` `connect` statements accept per-endpoint
/// multiplicity (§6 G24, `connect [0..1] a to [1] b;`). Before GH-33, `interface.rs`'s
/// `connect_ends` lacked this entirely -- only `connection.rs`'s had it, with no BNF basis for
/// restricting it to connections only (`ConnectorPart`/`InterfacePart` share the same
/// `ConnectorEnd`/`InterfaceEnd` grammar). Both now go through `connector::connect_ends`.
#[test]
fn per_endpoint_multiplicity_on_connect_works_in_both_connection_and_interface_defs() {
    let input = "package P {\npart def A;\npart def B;\nconnection def C { end a : A; end b : B; connect [1] a to [1] b; }\ninterface def I { end a : A; end b : B; connect [1] a to [1] b; }\n}";
    let elements = package_elements(input);

    let connection = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::ConnectionDef(c) => Some(&c.value),
            _ => None,
        })
        .expect("expected connection def");
    let ConnectionDefBody::Brace {
        elements: connection_elements,
        ..
    } = &connection.body
    else {
        panic!("expected connection def brace body");
    };
    let connect_stmt = connection_elements
        .iter()
        .find_map(|e| match &e.value {
            ConnectionDefBodyElement::ConnectStmt(stmt) => Some(&stmt.value),
            _ => None,
        })
        .expect("expected connect stmt in connection def");
    assert!(connect_stmt.from.value.multiplicity.is_some());
    assert!(connect_stmt.to.value.multiplicity.is_some());

    let interface = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::InterfaceDef(i) => Some(&i.value),
            _ => None,
        })
        .expect("expected interface def");
    let InterfaceDefBody::Brace {
        elements: interface_elements,
        ..
    } = &interface.body
    else {
        panic!("expected interface def brace body");
    };
    let connect_stmt = interface_elements
        .iter()
        .find_map(|e| match &e.value {
            InterfaceDefBodyElement::ConnectStmt(stmt) => Some(&stmt.value),
            _ => None,
        })
        .expect("expected connect stmt in interface def");
    assert!(connect_stmt.from.value.multiplicity.is_some());
    assert!(connect_stmt.to.value.multiplicity.is_some());
}

/// Connections keep typed fixed derivation-end roles (real usage: `tests/derivation_connections.rs`);
/// this is the one genuine, evidenced difference between the two contexts, so it stays
/// parameterized (`connector::end_decl`'s `allow_derived_name`) rather than shared unconditionally.
#[test]
fn derivation_end_role_is_connection_only_by_design() {
    let input = "package P {\nrequirement def R1;\nrequirement def R2;\n#derivation connection { end #original ::> R1; end #derive ::> R2; }\n}";
    let elements = package_elements(input);
    let connection = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::ConnectionDef(c) => Some(&c.value),
            _ => None,
        })
        .expect("expected derivation connection def");
    let ConnectionDefBody::Brace { elements, .. } = &connection.body else {
        panic!("expected connection def brace body");
    };
    assert!(elements.iter().any(|e| matches!(
        &e.value,
        ConnectionDefBodyElement::EndDecl(end)
            if matches!(
                &end.value.identity,
                sysml_v2_parser::ast::EndIdentity::Derivation(role)
                    if role.value == sysml_v2_parser::ast::DerivationEndRole::Original
            )
    )));
}
