use sysml_v2_parser::ast::{
    PackageBodyElement, RootElement, ViewBody, ViewBodyElement, WriteSemanticAst,
};
use sysml_v2_parser::parse_with_diagnostics;

fn resolved(
    document: &sysml_v2_parser::ast::ParsedDocument,
    id: sysml_v2_parser::ast::QualifiedReferenceId,
) -> &str {
    document
        .qualified_reference(id)
        .expect("reachable qualified-reference ID")
        .authored_text()
}

#[test]
fn malformed_import_branch_does_not_publish_its_allocated_target() {
    let source = "import Ghost::Target unexpected;\nimport Live::Target;";
    let result = parse_with_diagnostics(source);
    assert!(!result.errors.is_empty(), "malformed import should recover");
    let import = result
        .document
        .root
        .elements
        .iter()
        .find_map(|element| match &element.value {
            RootElement::Import(import) => Some(&import.value),
            _ => None,
        })
        .expect("valid import after recovery");
    assert_eq!(
        resolved(&result.document, import.target.reference),
        "Live::Target"
    );
    assert_eq!(result.document.qualified_references.len(), 1);

    // Compare the owning semantic projection, not opaque IDs from different documents.
    let repeated = parse_with_diagnostics(source);
    let mut first_projection = Vec::new();
    let mut repeated_projection = Vec::new();
    result
        .document
        .write_semantic_ast(&mut first_projection)
        .expect("first semantic projection");
    repeated
        .document
        .write_semantic_ast(&mut repeated_projection)
        .expect("repeated semantic projection");
    assert_eq!(first_projection, repeated_projection);
}

#[test]
fn malformed_alias_branch_does_not_shift_the_following_target_identity() {
    let result = parse_with_diagnostics(
        "alias broken for Ghost::Target unexpected;\nalias live for Live::Target;",
    );
    assert!(!result.errors.is_empty(), "malformed alias should recover");
    let alias = result
        .document
        .root
        .elements
        .iter()
        .find_map(|element| match &element.value {
            RootElement::Member(member) => match &member.value {
                PackageBodyElement::AliasDef(alias) => Some(&alias.value),
                _ => None,
            },
            _ => None,
        })
        .expect("valid alias after recovery");
    assert_eq!(resolved(&result.document, alias.target), "Live::Target");
    assert_eq!(result.document.qualified_references.len(), 1);
}

#[test]
fn malformed_expose_branch_rolls_back_before_the_next_view_member() {
    let result =
        parse_with_diagnostics("view v { expose Ghost::Target unexpected; expose Live::Target; }");
    assert!(!result.errors.is_empty(), "malformed expose should recover");
    let view = result
        .document
        .root
        .elements
        .iter()
        .find_map(|element| match &element.value {
            RootElement::Member(member) => match &member.value {
                PackageBodyElement::ViewUsage(view) => Some(&view.value),
                _ => None,
            },
            _ => None,
        })
        .expect("view usage");
    let ViewBody::Brace { elements, .. } = &view.body else {
        panic!("view body");
    };
    let expose = elements
        .iter()
        .find_map(|element| match &element.value {
            ViewBodyElement::Expose(expose) => Some(&expose.value),
            _ => None,
        })
        .expect("valid expose after recovery");
    assert_eq!(
        resolved(&result.document, expose.target.reference),
        "Live::Target"
    );
    assert_eq!(result.document.qualified_references.len(), 1);
}

#[test]
fn failed_default_reference_usage_does_not_publish_an_orphan_reference() {
    let result = parse_with_diagnostics("candidate :> Live::Target unexpected;");
    assert!(!result.errors.is_empty());
    assert_eq!(result.document.qualified_references.len(), 0);
    let mut projection = Vec::new();
    result
        .document
        .write_semantic_ast(&mut projection)
        .expect("semantic projection");
    let projection = String::from_utf8(projection).expect("UTF-8 projection");
    assert!(!projection.contains("(token \"Live\")"));
    assert!(!projection.contains("(token \"Target\")"));
}

#[test]
fn malformed_for_range_rolls_back_references_and_keeps_later_action() {
    let source = r#"package P {
action def Iterate {
    for item in Domain::fleet.activeMembers { action visit; }
    for orphan in Ghost::leaked + { action swallowed; }
    action later : Later::Type;
}
}"#;
    let result = parse_with_diagnostics(source);
    assert!(
        !result.errors.is_empty(),
        "malformed loop range should recover"
    );
    let package = match &result.document.root.elements[0].value {
        RootElement::Package(package) => &package.value,
        other => panic!("expected package, got {other:?}"),
    };
    let action = match &package.body {
        sysml_v2_parser::ast::PackageBody::Brace { elements, .. } => match &elements[0].value {
            PackageBodyElement::ActionDef(action) => &action.value,
            other => panic!("expected action def, got {other:?}"),
        },
        other => panic!("expected package body, got {other:?}"),
    };
    let elements = match &action.body {
        sysml_v2_parser::ast::ActionDefBody::Brace { elements, .. } => elements,
        other => panic!("expected action body, got {other:?}"),
    };
    assert_eq!(elements.len(), 3, "recovery must keep both valid siblings");
    let range = match &elements[0].value {
        sysml_v2_parser::ast::ActionDefBodyElement::ForLoop(for_loop) => {
            &for_loop.value.in_parameter.expression
        }
        other => panic!("expected valid for-loop, got {other:?}"),
    };
    let (base, member) = match &range.value {
        sysml_v2_parser::ast::Expression::MemberAccess { base, member, .. } => {
            let sysml_v2_parser::ast::Expression::FeatureRef(base) = base.value else {
                panic!("expected feature-ref base")
            };
            (base, member)
        }
        other => panic!("expected member-access range, got {other:?}"),
    };
    assert_eq!(resolved(&result.document, base), "Domain::fleet");
    assert_eq!(resolved(&result.document, *member), "activeMembers");
    assert!(matches!(
        elements[1].value,
        sysml_v2_parser::ast::ActionDefBodyElement::Error(_)
    ));
    let later_type = match &elements[2].value {
        sysml_v2_parser::ast::ActionDefBodyElement::ActionUsage(action) => {
            action.value.type_name.expect("later action type")
        }
        other => panic!("expected later action sibling, got {other:?}"),
    };
    assert_eq!(resolved(&result.document, later_type), "Later::Type");
    assert_eq!(
        result.document.qualified_references.len(),
        3,
        "the failed Ghost::leaked range must publish no arena identities"
    );
}
