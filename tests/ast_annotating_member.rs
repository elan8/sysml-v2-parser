//! The shared annotating-member family (`ast::AnnotatingMember`).
//!
//! `AnnotatingElement = Comment | Documentation | TextualRepresentation | MetadataFeature` is one
//! production in both the KerML and SysML grammars, so a scope that owns annotating members owns
//! the whole production. These tests pin that: the members parse the same way in every scope that
//! accepts the family, and they no longer depend on which emitter path a document happens to take.

use sysml_v2_parser::ast::{
    AnnotatingMember, ConnectionDefBodyElement, PackageBodyElement, PartUsageBodyElement, RefBody,
    RelationshipBodyElement, RootElement,
};
use sysml_v2_parser::{emit_sysml, parse};

fn package_elements(source: &str) -> Vec<PackageBodyElement> {
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

fn dependency_body(source: &str) -> Vec<RelationshipBodyElement> {
    for element in package_elements(source) {
        if let PackageBodyElement::Dependency(dependency) = element {
            return dependency
                .value
                .body_elements
                .as_ref()
                .expect("dependency body")
                .iter()
                .map(|member| member.value.clone())
                .collect();
        }
    }
    panic!("expected a dependency member");
}

/// Each alternative of the production reaches the same family in a relationship body.
#[test]
fn a_relationship_body_owns_the_whole_annotating_production() {
    let members = dependency_body(concat!(
        "package P {\n",
        "  dependency d from a to b {\n",
        "    doc /* why */\n",
        "    comment /* aside */\n",
        "    rep inline language \"text\" /* hello */\n",
        "  }\n",
        "}\n",
    ));

    let kinds: Vec<&str> = members
        .iter()
        .map(|member| match member {
            RelationshipBodyElement::Annotating(AnnotatingMember::Doc(_)) => "doc",
            RelationshipBodyElement::Annotating(AnnotatingMember::Comment(_)) => "comment",
            RelationshipBodyElement::Annotating(AnnotatingMember::TextualRep(_)) => "rep",
            RelationshipBodyElement::Annotating(AnnotatingMember::MetadataAnnotation(_)) => {
                "metadata"
            }
            other => panic!("expected an annotating member, got {other:?}"),
        })
        .collect();
    assert_eq!(kinds, vec!["doc", "comment", "rep"]);
}

/// A `ref` body accepts the same production through the same parser, rather than translating one
/// scope's members into another's -- that translation used to be a six-arm `match` in the ref-body
/// parser.
#[test]
fn a_ref_body_owns_the_same_production() {
    let elements = package_elements(concat!(
        "package P {\n",
        "  connection def C {\n",
        "    port a;\n",
        "    ref b {\n",
        "      doc /* r */\n",
        "    }\n",
        "  }\n",
        "}\n",
    ));
    let PackageBodyElement::ConnectionDef(connection) = &elements[0] else {
        panic!("expected a connection def");
    };
    let ref_decl = connection
        .value
        .body
        .members()
        .find_map(|member| match &member.value {
            ConnectionDefBodyElement::RefDecl(ref_decl) => Some(&ref_decl.value),
            _ => None,
        })
        .expect("expected a ref declaration");
    let RefBody::Brace { elements } = &ref_decl.body else {
        panic!("expected a brace ref body");
    };
    assert!(matches!(
        &elements[0].value,
        PartUsageBodyElement::Annotating(AnnotatingMember::Doc(_))
    ));
}

/// Before the family, three emitter copies handled these members and disagreed: a `rep` member
/// emitted from one path and failed as unsupported from the others, so whether a document could
/// be formatted depended on which construct owned the body. One production now means one
/// emitter.
#[test]
fn every_scope_that_accepts_a_rep_member_can_emit_it() {
    for source in [
        // A dependency body, an alias body, and a connect body inside a connection definition all
        // used to fail here; an import body, reaching a different copy of the same match, did not.
        "package P {\n  dependency d from a to b {\n    rep inline language \"text\" /* hello */\n  }\n}\n",
        "package P {\n  part def A;\n  alias B for A {\n    rep inline language \"text\" /* hello */\n  }\n}\n",
        "package P {\n  import ISQ::* {\n    rep inline language \"text\" /* hello */\n  }\n}\n",
        "package P {\n  connection def C {\n    port a;\n    port b;\n    connect a to b {\n      rep inline language \"text\" /* hello */\n    }\n  }\n}\n",
    ] {
        let parsed = parse(source).unwrap_or_else(|error| panic!("parse: {error}\n{source}"));
        let emitted = emit_sysml(&parsed)
            .unwrap_or_else(|error| panic!("emit failed for a rep member: {error}\n{source}"));
        assert!(
            emitted.contains("rep inline language \"text\""),
            "emitted output lost the rep member:\n{emitted}"
        );
        let reparsed = parse(&emitted).unwrap_or_else(|error| panic!("reparse: {error}\n{emitted}"));
        assert_eq!(
            parsed.normalize_for_test_comparison(),
            reparsed.normalize_for_test_comparison(),
            "rep member changed across format/reparse; emitted:\n{emitted}"
        );
    }
}
