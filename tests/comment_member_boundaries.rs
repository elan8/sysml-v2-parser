//! A `comment` member must end at its own body.
//!
//! `Comment = ( 'comment' Identification? )? ( 'about' ... )? ( 'locale' STRING )? REGULAR_COMMENT`
//! (KerML 8.2.3.3.2). The identification is optional, and the body that follows is a block comment
//! -- which the lexer would otherwise treat as skippable trivia. Looking for a name *through* the
//! body made a comment absorb the member written after it: `comment /* a */ doc /* b */` parsed as
//! a single comment named `doc`, and the doc member disappeared with no diagnostic on a strict
//! parse. These tests pin the boundary.

use sysml_v2_parser::ast::{PackageBodyElement, RootElement};
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

fn kinds(members: &[PackageBodyElement]) -> Vec<&'static str> {
    members
        .iter()
        .map(|member| match member {
            PackageBodyElement::Doc(_) => "doc",
            PackageBodyElement::Comment(_) => "comment",
            PackageBodyElement::TextualRep(_) => "rep",
            other => panic!("expected an annotating member, got {other:?}"),
        })
        .collect()
}

fn body(members: &str) -> String {
    format!("package P {{\n{members}}}\n")
}

#[test]
fn a_member_after_a_comment_survives() {
    for (following, expected) in [
        ("  doc /* second */\n", "doc"),
        ("  comment /* second */\n", "comment"),
        ("  rep inline language \"text\" /* second */\n", "rep"),
    ] {
        let source = body(&format!("  comment /* first */\n{following}"));
        assert_eq!(
            kinds(&package_members(&source)),
            vec!["comment", expected],
            "the member after a comment was absorbed:\n{source}"
        );
        assert!(
            parse_for_editor(&source).errors.is_empty(),
            "the fixture should parse cleanly:\n{source}"
        );
    }
}

#[test]
fn an_anonymous_comment_keeps_its_own_text() {
    let members = package_members(&body("  comment /* first */\n  comment /* second */\n"));
    let texts: Vec<String> = members
        .iter()
        .map(|member| match member {
            PackageBodyElement::Comment(comment) => comment.value.text.trim().to_owned(),
            other => panic!("expected a comment, got {other:?}"),
        })
        .collect();
    assert_eq!(texts, vec!["first".to_owned(), "second".to_owned()]);
}

/// The optional parts still parse: a name, a locale, and the `about` clause each precede the body.
#[test]
fn the_optional_clauses_still_parse() {
    let named = package_members(&body("  comment named /* text */\n"));
    let PackageBodyElement::Comment(comment) = &named[0] else {
        panic!("expected a comment");
    };
    assert_eq!(
        comment
            .value
            .identification
            .as_ref()
            .and_then(|id| id.name.clone()),
        Some("named".to_owned())
    );

    let localized = package_members(&body("  comment locale \"en_US\" /* text */\n"));
    let PackageBodyElement::Comment(comment) = &localized[0] else {
        panic!("expected a comment");
    };
    assert_eq!(comment.value.locale.as_deref(), Some("en_US"));
    assert!(
        comment.value.identification.is_none(),
        "`locale` is not a declaration name"
    );

    // `about` names annotated elements, not the comment, so it must not become an identification.
    let about = package_members(&body("  comment about a /* text */\n"));
    let PackageBodyElement::Comment(comment) = &about[0] else {
        panic!("expected a comment");
    };
    assert!(
        comment.value.identification.is_none(),
        "`about` is a clause keyword, not the comment's name"
    );
    assert_eq!(comment.value.text.trim(), "text");
}
