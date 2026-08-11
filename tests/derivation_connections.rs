use sysml_v2_parser::ast::{
    ConnectionDefBody, ConnectionDefBodyElement, PackageBody, PackageBodyElement, RootElement,
};
use sysml_v2_parser::parse_with_diagnostics;

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
    assert_eq!(connection.annotation.as_deref(), Some("derivation"));

    let ConnectionDefBody::Brace { elements } = &connection.body else {
        panic!("expected connection body");
    };
    assert!(elements.iter().any(|element| match &element.value {
        ConnectionDefBodyElement::EndDecl(end) => {
            end.value.name == "#original"
                && end.value.typing.is_none()
                && end
                    .value
                    .references
                    .as_ref()
                    .is_some_and(|references| references.value.target.len() == 1)
        }
        _ => false,
    }));
    assert!(elements.iter().any(|element| match &element.value {
        ConnectionDefBodyElement::EndDecl(end) => {
            end.value.name == "#derive"
                && end.value.typing.is_none()
                && end
                    .value
                    .references
                    .as_ref()
                    .is_some_and(|references| references.value.target.len() == 1)
        }
        _ => false,
    }));
}
