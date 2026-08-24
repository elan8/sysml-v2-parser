//! An annotating member survives a format-and-reparse round trip in every scope that owns one.
//!
//! Which members each scope holds, and that every scope can emit them, is pinned by
//! `tests/snapshots/sysml/annotating_member_family.md`: its AST section names the production's
//! alternatives in order (`(body (doc) (comment ...) (textual-rep))`) and its format section shows
//! the emitted result for each scope.
//!
//! What a snapshot does not check is that the emitted text parses back to the same tree. The
//! snapshot tool compares the strict and editor parses of the *source* (`actual_snapshot` in
//! `tools/snapshot_tool/support.rs`), not a reparse of its own output, so that assertion stays
//! here. It is the one that catches an emitter which produces something plausible but different --
//! a `rep` member emitted without its `language` clause, say.

use sysml_v2_parser::{emit_sysml, parse};

/// Formatting a document and parsing the result must yield the same tree.
#[track_caller]
fn assert_reparses_identically(source: &str) {
    let parsed = parse(source).unwrap_or_else(|error| panic!("parse: {error}\n{source}"));
    let emitted = emit_sysml(&parsed).unwrap_or_else(|error| panic!("emit: {error}\n{source}"));
    let reparsed = parse(&emitted).unwrap_or_else(|error| panic!("reparse: {error}\n{emitted}"));
    assert_eq!(
        parsed.normalize_for_test_comparison(),
        reparsed.normalize_for_test_comparison(),
        "the annotating member changed across format/reparse\nsource:\n{source}emitted:\n{emitted}"
    );
}

/// Before the shared family, three emitter copies handled these members and disagreed: a `rep`
/// emitted from an import body but failed as unsupported from a dependency, alias or connect body,
/// so whether a document could be formatted depended on which construct owned the body.
#[test]
fn a_rep_member_round_trips_from_every_scope_that_owns_one() {
    for source in [
        "package P {\n  dependency d from a to b {\n    rep inline language \"text\" /* hello */\n  }\n}\n",
        "package P {\n  part def A;\n  alias B for A {\n    rep inline language \"text\" /* hello */\n  }\n}\n",
        "package P {\n  import ISQ::* {\n    rep inline language \"text\" /* hello */\n  }\n}\n",
        "package P {\n  connection def C {\n    port a;\n    port b;\n    connect a to b {\n      rep inline language \"text\" /* hello */\n    }\n  }\n}\n",
    ] {
        assert_reparses_identically(source);
    }
}

/// The other alternatives of the same production, in the scope that owns all of them.
#[test]
fn the_whole_production_round_trips_from_a_relationship_body() {
    assert_reparses_identically(concat!(
        "package P {\n",
        "  dependency d from a to b {\n",
        "    doc /* why */\n",
        "    comment /* aside */\n",
        "    rep inline language \"text\" /* hello */\n",
        "  }\n",
        "}\n",
    ));
}

/// The keyword span is the only fact separating `comment /* x */` from a bare `/* x */`, which
/// reparses as trivia rather than as a member. Emission reads it, so a wire document that
/// redirects it changes what the document says while still spelling a real `comment` keyword.
/// This has no document projection -- the corruption is only reachable through deserialization.
#[cfg(feature = "serde")]
#[test]
fn a_comment_keyword_span_redirected_to_another_comment_is_rejected() {
    let source = "package P {\n  part def A {\n    comment /* first */\n  }\n  part def B {\n    comment /* second */\n  }\n}\n";
    let document = sysml_v2_parser::parse_for_editor(source).document;
    let encoded = serde_json::to_value(&document).expect("the parsed document serializes");

    let body = |index: usize| {
        format!(
            "/root/elements/0/value/Package/value/body/Brace/elements/{index}\
             /value/PartDef/value/body/Brace/elements/0/value/Annotating/Comment"
        )
    };
    let mut tampered = encoded.clone();
    let sibling = tampered
        .pointer(&format!("{}/value/keyword_span", body(1)))
        .cloned()
        .expect("the second comment's keyword span");
    *tampered
        .pointer_mut(&format!("{}/value/keyword_span", body(0)))
        .expect("the first comment's keyword span") = sibling;

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a keyword belonging to another comment must be rejected");
    let message = error.to_string();
    assert!(
        message.contains("comment keyword") && message.contains("outside"),
        "expected a containment failure naming the keyword, got: {message}"
    );
}

/// The same span pointed at text that is not the keyword at all.
#[cfg(feature = "serde")]
#[test]
fn a_comment_keyword_span_covering_other_text_is_rejected() {
    let source = "package P {\n  part def A {\n    comment /* first */\n  }\n}\n";
    let document = sysml_v2_parser::parse_for_editor(source).document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let keyword = "/root/elements/0/value/Package/value/body/Brace/elements/0\
                   /value/PartDef/value/body/Brace/elements/0/value/Annotating/Comment/value/keyword_span";
    *tampered
        .pointer_mut(&format!("{keyword}/offset"))
        .expect("the comment's keyword offset") = serde_json::json!(0);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a keyword span that does not cover `comment` must be rejected");
    assert!(
        error.to_string().contains("comment keyword"),
        "expected the keyword check to name itself, got: {error}"
    );
}

/// Strict parsing rejects a document whose annotating syntax is malformed; editor parsing returns
/// the whole document, with the malformed span as a recovery node and every later sibling intact.
///
/// This pair has no document projection: the snapshot driver runs the strict parser only when the
/// editor parse produced no diagnostics (`actual_snapshot` in `tools/snapshot_tool/support.rs`),
/// so a recovery fixture pins the editor side and can never pin the strict side. The source is the
/// one `tests/snapshots/sysml/annotating_member_recovery.md` uses, so the two move together.
#[test]
fn strict_rejects_malformed_annotating_syntax_that_editor_parsing_recovers() {
    const SOURCE: &str = concat!(
        "package AnnotatingMemberRecovery {\n",
        "    part def Between {\n",
        "        doc /* before */\n",
        "        rep language\n",
        "        comment /* after */\n",
        "    }\n",
        "    part def Later {\n",
        "        rep laterRep language \"text\" /* the declaration after every recovery */\n",
        "    }\n",
        "}\n",
    );

    parse(SOURCE).expect_err("strict parsing must reject the malformed `rep language`");

    let editor = sysml_v2_parser::parse_for_editor(SOURCE);
    assert!(
        !editor.errors.is_empty(),
        "editor parsing must report the malformed member rather than accepting it"
    );

    use sysml_v2_parser::ast::{AnnotatingMember, PartDefBody, PartDefBodyElement, RootElement};
    let RootElement::Package(package) = &editor.document.root.elements[0].value else {
        panic!("editor parsing must still return the package, not a whole-document recovery node");
    };
    let members = package
        .value
        .body
        .braced_elements()
        .expect("a brace package body");

    // The malformed member is a recovery node at its authored position, between the two valid
    // annotating members of the same body -- not a successful-looking annotating node, and not a
    // hole where the siblings used to be.
    let between = members
        .iter()
        .find_map(|member| match &member.value {
            sysml_v2_parser::ast::PackageBodyElement::PartDef(definition)
                if definition
                    .value
                    .identification
                    .name
                    .and_then(|n| editor.document.declaration_name(n))
                    == Some("Between") =>
            {
                Some(&definition.value.body)
            }
            _ => None,
        })
        .expect("the part def whose body holds the malformed member");
    let PartDefBody::Brace { elements, .. } = between else {
        panic!("expected a brace part def body");
    };
    assert!(
        matches!(
            elements.as_slice(),
            [
                sysml_v2_parser::Node {
                    value: PartDefBodyElement::Annotating(AnnotatingMember::Doc(_)),
                    ..
                },
                sysml_v2_parser::Node {
                    value: PartDefBodyElement::Error(_),
                    ..
                },
                sysml_v2_parser::Node {
                    value: PartDefBodyElement::Annotating(AnnotatingMember::Comment(_)),
                    ..
                },
            ]
        ),
        "recovery must sit between the doc before it and the comment after it, got {elements:?}"
    );

    // And the declaration written after the recovered one still parses.
    assert!(
        members.iter().any(|member| matches!(
            &member.value,
            sysml_v2_parser::ast::PackageBodyElement::PartDef(definition)
                if definition.value.identification.name.and_then(|n| editor.document.declaration_name(n)) == Some("Later")
        )),
        "the declaration after the recovered member must survive"
    );
}
