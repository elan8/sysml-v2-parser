//! "No body was written" is a state of two specific declarations, not of every body.
//!
//! `DefinitionBody` requires `;` or `{ ... }`, so [`Body`](sysml_v2_parser::ast::Body) has no
//! variant for a declaration that has neither. Two places accept one anyway -- a `#Name` metadata
//! keyword used as a prefix, which annotates the declaration after it, and an action usage whose
//! terminator the parser infers from the statement that follows. Those two hold `Option<Body<_>>`,
//! so no other scope can represent, deserialize, or be constructed with a missing body.

use sysml_v2_parser::ast::{
    ActionUsageBody, AttributeBody, PackageBodyElement, PartDefBodyElement, RootElement,
};
use sysml_v2_parser::{parse, parse_for_editor};

fn package_members(source: &str) -> Vec<PackageBodyElement> {
    let root = parse(source).unwrap_or_else(|error| panic!("parse: {error}\n{source}"));
    let RootElement::Package(package) = &root.elements[0].value else {
        panic!("expected a package");
    };
    package
        .value
        .body
        .members()
        .map(|member| member.value.clone())
        .collect()
}

/// The prefix spelling annotates the declaration that follows and has no body of its own; the
/// member spelling always writes one. The same holds for a prefix on an extended definition.
#[test]
fn a_metadata_keyword_has_a_body_only_when_used_as_a_member() {
    let members = package_members("package P {\n  #safety part def X;\n  #safety;\n}\n");

    let PackageBodyElement::MetadataKeywordUsage(prefix) = &members[0] else {
        panic!("expected the prefix keyword, got {:?}", members[0]);
    };
    assert!(
        prefix.value.body.is_none(),
        "a keyword written as a prefix never had a body to write"
    );

    let PackageBodyElement::MetadataKeywordUsage(member) = &members[2] else {
        panic!("expected the member spelling, got {:?}", members[2]);
    };
    assert!(
        matches!(member.value.body, Some(AttributeBody::Semicolon { .. })),
        "the member spelling wrote its `;`, got {:?}",
        member.value.body
    );
    let attached = package_members("package P {\n  #situation def Failure;\n}\n");
    let PackageBodyElement::ExtendedDefinition(definition) = &attached[0] else {
        panic!("expected an extended definition, got {:?}", attached[0]);
    };
    assert!(
        definition.value.prefix_keywords[0].value.body.is_none(),
        "a prefix attached to a definition has no body either"
    );
}

/// An action usage whose terminator is implied records that none was written, rather than
/// claiming a semicolon.
#[test]
fn an_inferred_action_terminator_is_not_a_semicolon() {
    let members =
        package_members("package P {\n  part def Q {\n    action a\n    action b;\n  }\n}\n");
    let PackageBodyElement::PartDef(part_def) = &members[0] else {
        panic!("expected a part def");
    };
    let usages: Vec<_> = part_def
        .value
        .body
        .members()
        .filter_map(|member| match &member.value {
            PartDefBodyElement::ActionUsage(usage) => Some(usage.value.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(usages.len(), 2, "expected both action usages");
    assert!(
        usages[0].body.is_none(),
        "the first usage wrote no terminator at all"
    );
    assert!(
        matches!(usages[1].body, Some(ActionUsageBody::Semicolon { .. })),
        "the second usage wrote its `;`, got {:?}",
        usages[1].body
    );
}

/// The state is not reachable from any other scope: a package body, a part definition body, and
/// every other alias have only the two authored alternatives. This is a type-level property --
/// there is no `Body` variant to construct or deserialize -- asserted here by exhausting the type.
#[test]
fn other_scopes_have_no_absent_state() {
    let document = parse_for_editor("package P {\n  part def A;\n}\n").document;
    let RootElement::Package(package) = &document.root.elements[0].value else {
        panic!("expected a package");
    };
    // Two alternatives, both requiring an authored token. A third would fail to compile here.
    match &package.value.body {
        sysml_v2_parser::ast::PackageBody::Semicolon { semicolon_span } => {
            panic!("expected a brace body, got a `;` at {semicolon_span:?}")
        }
        sysml_v2_parser::ast::PackageBody::Brace {
            open_span,
            elements,
            close_span,
        } => {
            assert_eq!(elements.len(), 1);
            assert!(open_span.offset < close_span.offset);
        }
    }
}
