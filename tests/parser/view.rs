//! Parser tests: view

use sysml_v2_parser::ast::*;
use sysml_v2_parser::parse;

fn reference_text(
    document: &ParsedDocument,
    reference: Option<QualifiedReferenceId>,
) -> Option<&str> {
    reference
        .and_then(|id| document.qualified_reference(id))
        .map(|view| view.authored_text())
}

#[test]
fn test_view_def_parse() {
    let input = "package P { view def Name { } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements, .. } => elements,
        _ => panic!("expected brace body"),
    };
    assert_eq!(elements.len(), 1);
    match &elements[0].value {
        PackageBodyElement::ViewDef(vd) => {
            assert_eq!(vd.identification.name.as_deref(), Some("Name"));
            assert!(
                matches!(&vd.body, ViewDefBody::Brace { ref elements, .. } if elements.is_empty())
            );
        }
        _ => panic!("expected ViewDef"),
    }
}

#[test]
fn test_viewpoint_def_parse() {
    let input = "package P { viewpoint def Name { } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements, .. } => elements,
        _ => panic!("expected brace body"),
    };
    assert_eq!(elements.len(), 1);
    match &elements[0].value {
        PackageBodyElement::ViewpointDef(vpd) => {
            assert_eq!(vpd.identification.name.as_deref(), Some("Name"));
        }
        _ => panic!("expected ViewpointDef"),
    }
}

#[test]
fn test_rendering_def_parse() {
    let input = "package P { rendering def Name; }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements, .. } => elements,
        _ => panic!("expected brace body"),
    };
    assert_eq!(elements.len(), 1);
    match &elements[0].value {
        PackageBodyElement::RenderingDef(rd) => {
            assert_eq!(rd.identification.name.as_deref(), Some("Name"));
            assert!(matches!(rd.body, RenderingDefBody::Semicolon { .. }));
        }
        _ => panic!("expected RenderingDef"),
    }
}

#[test]
fn test_view_usage_parse() {
    let input = "package P { view name : ViewType { } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements, .. } => elements,
        _ => panic!("expected brace body"),
    };
    assert_eq!(elements.len(), 1);
    match &elements[0].value {
        PackageBodyElement::ViewUsage(vu) => {
            assert_eq!(vu.name, "name");
            assert_eq!(reference_text(&result, vu.type_name), Some("ViewType"));
            assert!(
                matches!(&vu.body, ViewBody::Brace { ref elements, .. } if elements.is_empty())
            );
        }
        _ => panic!("expected ViewUsage"),
    }
}

#[test]
fn test_view_and_connection_bodies_parse_members() {
    let input = "package P { view def V { doc /*d*/ filter x > 0; render r : Renderer; } view v : V { expose Model::*; satisfy VP; } connection def C { end from : A; end to : B; connect from to to; } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements, .. } => elements,
        _ => panic!("expected brace body"),
    };
    let view_def = match &elements[0].value {
        PackageBodyElement::ViewDef(vd) => &vd.value,
        _ => panic!("expected ViewDef"),
    };
    assert!(matches!(&view_def.body, ViewDefBody::Brace { elements, .. } if !elements.is_empty()));
    let view_usage = match &elements[1].value {
        PackageBodyElement::ViewUsage(v) => &v.value,
        _ => panic!("expected ViewUsage"),
    };
    assert!(matches!(&view_usage.body, ViewBody::Brace { elements, .. } if !elements.is_empty()));
    let connection_def = match &elements[2].value {
        PackageBodyElement::ConnectionDef(c) => &c.value,
        _ => panic!("expected ConnectionDef"),
    };
    assert!(matches!(
        &connection_def.body,
        sysml_v2_parser::ast::ConnectionDefBody::Brace { elements, .. } if !elements.is_empty()
    ));
}

#[test]
fn test_viewpoint_usage_accepts_defined_by_typing() {
    let input = r#"package P {
viewpoint safety defined by Mission::SafetyViewpoint { }
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements, .. } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let viewpoint = match &elements[0].value {
        PackageBodyElement::ViewpointUsage(v) => v,
        other => panic!("expected viewpoint usage, got {:?}", other),
    };
    assert_eq!(
        reference_text(&result, viewpoint.value.type_name),
        Some("Mission::SafetyViewpoint")
    );
}

#[test]
fn test_action_state_and_view_usage_accept_typed_by_alias() {
    let input = r#"package P {
action send typed by Mission::SendAction;
state running typed by Mission::Mode;
view dashboard typed by Mission::DashboardView;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements, .. } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };

    let action_usage = match &elements[0].value {
        PackageBodyElement::ActionUsage(a) => a,
        other => panic!("expected action usage, got {:?}", other),
    };
    assert_eq!(
        reference_text(&result, action_usage.value.type_name),
        Some("Mission::SendAction")
    );

    let state_usage = match &elements[1].value {
        PackageBodyElement::StateUsage(s) => s,
        other => panic!("expected state usage, got {:?}", other),
    };
    assert_eq!(
        reference_text(&result, state_usage.value.type_name),
        Some("Mission::Mode")
    );

    let view_usage = match &elements[2].value {
        PackageBodyElement::ViewUsage(v) => v,
        other => panic!("expected view usage, got {:?}", other),
    };
    assert_eq!(
        reference_text(&result, view_usage.value.type_name),
        Some("Mission::DashboardView")
    );
}

#[test]
fn test_rendering_usage_accepts_typed_by_alias() {
    let input = r#"package P {
rendering skin typed by Mission::Renderer;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements, .. } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let rendering_usage = match &elements[0].value {
        PackageBodyElement::RenderingUsage(r) => r,
        other => panic!("expected rendering usage, got {:?}", other),
    };
    assert_eq!(
        reference_text(&result, rendering_usage.value.type_name),
        Some("Mission::Renderer")
    );
}
