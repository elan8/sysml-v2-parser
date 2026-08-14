//! Whether a `comment` member wrote its keyword is a grammatical fact, not a formatting choice.
//!
//! `Comment = ( 'comment' Identification? )? ( 'locale' STRING )? REGULAR_COMMENT` (KerML
//! 8.2.3.3.2) makes the keyword optional, and the two spellings are not interchangeable on the way
//! out: a member emitted without its keyword is a bare block comment, which reparses as trivia and
//! disappears. `CommentAnnotation::keyword_span` records what was authored so emission reproduces
//! it.

use sysml_v2_parser::ast::{PackageBodyElement, RootElement};
use sysml_v2_parser::{emit_sysml, parse};

fn first_comment(source: &str) -> sysml_v2_parser::ast::CommentAnnotation {
    let root = parse(source).unwrap_or_else(|error| panic!("parse: {error}\n{source}"));
    let RootElement::Package(package) = &root.elements[0].value else {
        panic!("expected a package");
    };
    for member in package.value.body.members() {
        if let PackageBodyElement::Comment(comment) = &member.value {
            return comment.value.clone();
        }
    }
    panic!("expected a comment member in:\n{source}");
}

#[test]
fn the_authored_keyword_is_recorded_with_its_span() {
    let with_keyword = first_comment("package P {\n  comment /* text */\n}\n");
    let span = with_keyword
        .keyword_span
        .as_ref()
        .expect("the `comment` keyword was authored");
    assert_eq!(span.len, "comment".len());
    assert_eq!(span.line, 2);

    // The keyword-less spelling: `locale "..." /* ... */` is a legal comment without it.
    let without_keyword = first_comment("package P {\n  locale \"en_US\" /* text */\n}\n");
    assert!(without_keyword.keyword_span.is_none());
    assert_eq!(without_keyword.locale.as_deref(), Some("en_US"));
}

#[test]
fn an_anonymous_comment_member_survives_format_and_reparse() {
    for source in [
        "package P {\n  comment /* text */\n}\n",
        "package P {\n  comment named /* text */\n}\n",
        "package P {\n  locale \"en_US\" /* text */\n}\n",
        "package P {\n  comment locale \"en_US\" /* text */\n}\n",
    ] {
        let parsed = parse(source).unwrap_or_else(|error| panic!("parse: {error}\n{source}"));
        let emitted = emit_sysml(&parsed).unwrap_or_else(|error| panic!("emit: {error}\n{source}"));
        let reparsed =
            parse(&emitted).unwrap_or_else(|error| panic!("reparse: {error}\n{emitted}"));
        assert_eq!(
            parsed.normalize_for_test_comparison(),
            reparsed.normalize_for_test_comparison(),
            "comment member changed across format/reparse\nsource:\n{source}emitted:\n{emitted}"
        );
    }
}

/// Emission must not invent a keyword that was not authored, either.
#[test]
fn the_keyword_less_spelling_is_not_given_a_keyword() {
    let parsed = parse("package P {\n  locale \"en_US\" /* text */\n}\n").expect("parse");
    let emitted = emit_sysml(&parsed).expect("emit");
    assert!(
        !emitted.contains("comment"),
        "emission invented a `comment` keyword:\n{emitted}"
    );
    assert!(emitted.contains("locale \"en_US\""), "{emitted}");
}
