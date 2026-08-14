//! Unrecognized body content is a diagnosed node, never untyped text.
//!
//! Body scopes used to carry an `Other(String)` member: a copy of the source with no span, no
//! diagnostic, and no structure. It fired for content the scope could not parse, so the two states
//! the parser is supposed to distinguish -- malformed input, and spec-valid syntax it does not
//! model -- both arrived as the same opaque string, and neither was reported. These tests pin the
//! replacement: a recovery node with an exact span and a diagnostic, or an explicit unsupported
//! node, depending on which state it is.

use sysml_v2_parser::ast::{
    AttributeBody, AttributeBodyElement, PackageBodyElement, PartDefBodyElement, RootElement,
    StateDefBody, StateDefBodyElement,
};
use sysml_v2_parser::{parse_for_editor, ParseResult};

fn document(source: &str) -> ParseResult {
    parse_for_editor(source)
}

fn package_members(result: &ParseResult) -> Vec<PackageBodyElement> {
    let RootElement::Package(package) = &result.document.root.elements[0].value else {
        panic!("expected a package");
    };
    package
        .value
        .body
        .members()
        .map(|member| member.value.clone())
        .collect()
}

/// Content the scope cannot parse is malformed: it keeps its authored span and is reported.
#[test]
fn unparseable_content_becomes_a_diagnosed_recovery_node() {
    let source =
        "package P {\n  state def S {\n    unknown stuff;\n    transition t then Ready;\n  }\n}\n";
    let result = document(source);

    let PackageBodyElement::StateDef(state) = &package_members(&result)[0] else {
        panic!("expected a state def");
    };
    let StateDefBody::Brace { elements, .. } = &state.value.body else {
        panic!("expected a brace state body");
    };
    let malformed = elements
        .iter()
        .find_map(|member| match &member.value {
            StateDefBodyElement::Error(error) => Some(error),
            _ => None,
        })
        .expect("the unparseable member is kept as a recovery node");
    // A recovery span covers the region that was skipped, up to the next member, so it carries
    // the trailing trivia with it.
    assert_eq!(
        source[malformed.span.offset..malformed.span.offset + malformed.span.len].trim_end(),
        "unknown stuff;",
        "the recovery node spans the text it could not parse"
    );
    assert!(
        result
            .errors
            .iter()
            .any(|error| error.found.as_deref() == Some("unknown stuff;")),
        "and it is reported: {:?}",
        result.errors
    );

    // The valid sibling after it still parses -- recovery did not consume the rest of the body.
    assert!(
        elements
            .iter()
            .any(|member| matches!(member.value, StateDefBodyElement::Transition(_))),
        "the member after the malformed one is still structured"
    );
}

/// A spec-valid member the scope does not model is a different state, and says so.
#[test]
fn an_unmodelled_member_becomes_an_explicit_unsupported_node() {
    let source = "package P {\n  attribute def A {\n    binding b = c;\n  }\n}\n";
    let result = document(source);

    let PackageBodyElement::AttributeDef(definition) = &package_members(&result)[0] else {
        panic!("expected an attribute def");
    };
    let AttributeBody::Brace { elements, .. } = &definition.value.body else {
        panic!("expected a brace attribute body");
    };
    let unsupported = elements
        .iter()
        .find_map(|member| match &member.value {
            AttributeBodyElement::Unsupported(node) => Some(node),
            _ => None,
        })
        .expect("the unmodelled member is kept as an unsupported node");
    assert_eq!(
        &source[unsupported.span.offset..unsupported.span.offset + unsupported.span.len],
        "binding b = c;",
        "the unsupported node spans exactly the member it could not model"
    );
    assert_eq!(
        unsupported.value.diagnostic.code, "unsupported_grammar_form",
        "and it carries a diagnostic rather than being silent"
    );
    assert!(
        result.errors.iter().any(|error| {
            error.code.as_deref() == Some("unsupported_grammar_form")
                && error.severity == Some(sysml_v2_parser::DiagnosticSeverity::Warning)
        }),
        "reported as a warning, not an error: {:?}",
        result.errors
    );
}

/// The two states stay distinct: an unmodelled member is not reported as malformed input.
#[test]
fn the_two_states_are_not_interchangeable() {
    let unsupported = document("package P {\n  attribute def A {\n    binding b = c;\n  }\n}\n");
    assert!(
        unsupported
            .errors
            .iter()
            .all(|error| error.severity != Some(sysml_v2_parser::DiagnosticSeverity::Error)),
        "an unmodelled but valid member is not an error: {:?}",
        unsupported.errors
    );

    let malformed = document("package P {\n  part def Q {\n    ??? junk ???\n  }\n}\n");
    assert!(
        malformed
            .errors
            .iter()
            .any(|error| error.severity == Some(sysml_v2_parser::DiagnosticSeverity::Error)),
        "unparseable text is an error: {:?}",
        malformed.errors
    );
    let PackageBodyElement::PartDef(part_def) = &package_members(&malformed)[0] else {
        panic!("expected a part def");
    };
    assert!(
        part_def
            .value
            .body
            .members()
            .any(|member| matches!(member.value, PartDefBodyElement::Error(_))),
        "and it is kept as a recovery node"
    );
}
