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
    let ViewBody::Brace { elements } = &view.body else {
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
