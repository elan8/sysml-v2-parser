//! Upstream gap 70: members a `MetadataBody` admits but the parser rejected.
//!
//! ```text
//! MetadataBody : Type = ';' | '{' ( DefinitionMember | MetadataBodyUsageMember
//!                                 | AliasMember | Import )* '}'      -- SysML BNF 1677
//! ```
//!
//! Only two of the four alternatives were dispatched, so a nested declaration, an alias and an
//! import in a metadata body all reached recovery -- the declaration and its span were lost
//! outright, with the scope, not the member, blamed for it.

use sysml_v2_parser::ast::{
    AttributeBodyElement, MetadataBody, MetadataBodyElement, PackageBody, PackageBodyElement,
    RootElement,
};
use sysml_v2_parser::parse_with_diagnostics;

fn metadata_body_members(
    body_source: &str,
) -> (sysml_v2_parser::ParsedDocument, Vec<MetadataBodyElement>) {
    let source = format!("package P {{ part x {{ @M {{ {body_source} }} }} }}");
    let result = parse_with_diagnostics(&source);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics for `{body_source}`: {:?}",
        result.errors
    );
    let RootElement::Package(pkg) = &result.document.root.elements[0].value else {
        panic!("expected package");
    };
    let PackageBody::Brace { elements, .. } = &pkg.value.body else {
        panic!("expected brace package body");
    };
    let PackageBodyElement::PartUsage(part) = &elements[0].value else {
        panic!("expected part usage, got {:?}", elements[0].value);
    };
    let found = format!("{part:?}");
    assert!(
        !found.contains("ParseErrorNode"),
        "no member may reach recovery: {found}"
    );
    let members =
        collect_metadata_body(part).expect("the part body must own one metadata annotation");
    (result.document, members)
}

fn collect_metadata_body(
    part: &sysml_v2_parser::ast::Node<sysml_v2_parser::ast::PartUsage>,
) -> Option<Vec<MetadataBodyElement>> {
    // The annotation is the sole member of the part body; find it structurally rather than by
    // reaching through a fixed index chain that would break on unrelated body changes.
    let sysml_v2_parser::ast::PartUsageBody::Brace { elements, .. } = &part.value.body else {
        return None;
    };
    for element in elements {
        if let sysml_v2_parser::ast::PartUsageBodyElement::Annotating(
            sysml_v2_parser::ast::AnnotatingMember::MetadataAnnotation(annotation),
        ) = &element.value
        {
            if let MetadataBody::Brace { elements, .. } = &annotation.value.body {
                return Some(elements.iter().map(|e| e.value.clone()).collect());
            }
        }
    }
    None
}

#[test]
fn a_nested_declaration_reaches_the_ast_with_its_declaration() {
    let (doc, members) = metadata_body_members("attribute def X;");
    assert_eq!(members.len(), 1);
    let MetadataBodyElement::Definition(member) = &members[0] else {
        panic!("expected a DefinitionMember, got {:?}", members[0]);
    };
    let AttributeBodyElement::AttributeDef(def) = &member.value else {
        panic!("expected an attribute def, got {:?}", member.value);
    };
    assert_eq!(
        def.value.name.and_then(|n| doc.declaration_name(n)),
        Some("X")
    );
    assert!(member.span.len > 0, "the member keeps its own source span");
}

#[test]
fn an_alias_member_reaches_the_ast() {
    let (doc, members) = metadata_body_members("alias a for b;");
    assert!(
        matches!(&members[0], MetadataBodyElement::Alias(alias)
            if alias.value.identification.name.and_then(|n| doc.declaration_name(n)) == Some("a")),
        "got {:?}",
        members[0]
    );
}

#[test]
fn an_import_member_reaches_the_ast() {
    let (_, members) = metadata_body_members("import X::*;");
    assert!(
        matches!(&members[0], MetadataBodyElement::Import(_)),
        "got {:?}",
        members[0]
    );
}

#[test]
fn the_keyword_less_redefinition_member_still_wins_its_spelling() {
    // `MetadataBodyUsage`'s `OwnedRedefinition` is a bare qualified name, so it must be tried
    // before the declaration dispatcher, which would otherwise read it as a keyword-less usage.
    for source in ["q = 5;", "redefines q = 5;", ":>> q = 5;"] {
        let (_, members) = metadata_body_members(source);
        assert!(
            matches!(&members[0], MetadataBodyElement::Usage(_)),
            "`{source}` must stay a MetadataBodyUsage, got {:?}",
            members[0]
        );
    }
}

#[test]
fn a_redefinition_and_a_declaration_coexist_in_authored_order() {
    let (_, members) = metadata_body_members(":>> q = 5; attribute def X;");
    assert_eq!(members.len(), 2);
    assert!(matches!(&members[0], MetadataBodyElement::Usage(_)));
    assert!(matches!(&members[1], MetadataBodyElement::Definition(_)));
}
