use sysml_v2_parser::ast::{
    ConnectionDefBody, ConnectionDefBodyElement, DerivationConnectionRole, DerivationEndRole,
    EndIdentity, PackageBody, PackageBodyElement, RootElement,
};
use sysml_v2_parser::{emit_sysml, parse_with_diagnostics};

#[test]
fn derivation_connection_parses_without_recovery_diagnostics() {
    let input = "package P {\nrequirement def OriginalReq;\nrequirement def DerivedReq;\n#derivation connection {\nend #original ::> OriginalReq;\nend #derive ::> DerivedReq;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace body");
    };
    let connection = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::ConnectionDef(conn) => Some(&conn.value),
            _ => None,
        })
        .expect("expected derivation connection");
    let derivation_role = connection
        .derivation_role
        .as_ref()
        .expect("typed derivation role");
    assert_eq!(derivation_role.value, DerivationConnectionRole::Derivation);
    assert_eq!(
        result.document.source.slice(&derivation_role.span),
        Some("#derivation")
    );

    let ConnectionDefBody::Brace { elements } = &connection.body else {
        panic!("expected connection body");
    };
    for (expected_role, expected_marker, expected_target) in [
        (DerivationEndRole::Original, "#original", "OriginalReq"),
        (DerivationEndRole::Derive, "#derive", "DerivedReq"),
    ] {
        let end = elements
            .iter()
            .find_map(|element| match &element.value {
                ConnectionDefBodyElement::EndDecl(end)
                    if matches!(
                        &end.value.identity,
                        EndIdentity::Derivation(role) if role.value == expected_role
                    ) =>
                {
                    Some(&end.value)
                }
                ConnectionDefBodyElement::EndDecl(_)
                | ConnectionDefBodyElement::RefDecl(_)
                | ConnectionDefBodyElement::ConnectStmt(_)
                | ConnectionDefBodyElement::Doc(_)
                | ConnectionDefBodyElement::Error(_)
                | ConnectionDefBodyElement::AttributeDef(_)
                | ConnectionDefBodyElement::AttributeUsage(_)
                | ConnectionDefBodyElement::ItemDef(_)
                | ConnectionDefBodyElement::ItemUsage(_)
                | ConnectionDefBodyElement::PortDef(_)
                | ConnectionDefBodyElement::PortUsage(_)
                | ConnectionDefBodyElement::AssertConstraint(_)
                | ConnectionDefBodyElement::OccurrenceUsage(_)
                | ConnectionDefBodyElement::SuccessionUsage(_)
                | ConnectionDefBodyElement::PartUsage(_) => None,
            })
            .expect("derivation end role");
        let EndIdentity::Derivation(role) = &end.identity else {
            unreachable!("filtered to derivation role")
        };
        assert_eq!(
            result.document.source.slice(&role.span),
            Some(expected_marker)
        );
        assert!(end.typing.is_none());
        let target = end
            .references
            .as_ref()
            .and_then(|references| references.value.target.first())
            .and_then(|target| result.document.qualified_reference(*target))
            .expect("source-backed derivation target");
        assert_eq!(
            target.segment_decoded_text(0).as_deref(),
            Some(expected_target)
        );
    }

    let emitted = emit_sysml(&result.document).expect("emit derivation connection");
    assert!(emitted.contains("#derivation connection def"));
    assert!(emitted.contains("end #original ::> OriginalReq;"));
    assert!(emitted.contains("end #derive ::> DerivedReq;"));
}

#[test]
fn unknown_derivation_end_role_recovers_and_retains_following_end() {
    let input = "package P {\n#derivation connection {\nend #mystery ::> Missing;\nend #derive ::> Kept;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        !result.errors.is_empty(),
        "unknown derivation role must produce a diagnostic"
    );

    let package = match &result.document.root.elements[0].value {
        RootElement::Package(package) => &package.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &package.body else {
        panic!("expected package body");
    };
    let connection = elements
        .iter()
        .find_map(|element| match &element.value {
            PackageBodyElement::ConnectionDef(connection) => Some(&connection.value),
            _ => None,
        })
        .expect("recovered connection");
    let ConnectionDefBody::Brace { elements } = &connection.body else {
        panic!("expected connection body");
    };

    let mut saw_recovery = false;
    let mut saw_derive = false;
    for element in elements {
        match &element.value {
            ConnectionDefBodyElement::Error(_) => saw_recovery = true,
            ConnectionDefBodyElement::EndDecl(end) => match &end.value.identity {
                EndIdentity::Derivation(role) => match role.value {
                    DerivationEndRole::Original => {}
                    DerivationEndRole::Derive => saw_derive = true,
                },
                EndIdentity::Declaration(_) => {}
            },
            ConnectionDefBodyElement::RefDecl(_)
            | ConnectionDefBodyElement::ConnectStmt(_)
            | ConnectionDefBodyElement::Doc(_)
            | ConnectionDefBodyElement::AttributeDef(_)
            | ConnectionDefBodyElement::AttributeUsage(_)
            | ConnectionDefBodyElement::ItemDef(_)
            | ConnectionDefBodyElement::ItemUsage(_)
            | ConnectionDefBodyElement::PortDef(_)
            | ConnectionDefBodyElement::PortUsage(_)
            | ConnectionDefBodyElement::AssertConstraint(_)
            | ConnectionDefBodyElement::OccurrenceUsage(_)
            | ConnectionDefBodyElement::SuccessionUsage(_)
            | ConnectionDefBodyElement::PartUsage(_) => {}
        }
    }
    assert!(
        saw_recovery,
        "unknown marker must remain as recovery syntax"
    );
    assert!(saw_derive, "recovery must retain the following valid end");
}
