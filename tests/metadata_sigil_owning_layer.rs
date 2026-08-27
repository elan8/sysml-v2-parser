//! Owning-layer mechanics of the `@` / `#` metadata seam that have no document projection.
//!
//! What the two sigil productions *parse to* is pinned by the semantic snapshots
//! (`tests/snapshots/sysml/metadata_sigil_alternatives.md`, `..._owning_scopes.md`,
//! `..._recovery.md`): the alternatives, the scopes, the recovery states and their diagnostics all
//! show up there, so none of that is restated here.
//!
//! What a snapshot cannot show is the layer underneath: whether a corrupted wire document is
//! rejected, whether speculation that fails leaves arena entries behind, and whether formatting the
//! emitted text reparses to the same tree. Those are what this file holds.

use sysml_v2_parser::{emit_sysml, parse, parse_for_editor};

/// Formatting a document, parsing the result, and formatting again: the tree must survive the
/// first round trip and the text must be a fixed point of the second.
#[track_caller]
fn assert_round_trips_and_is_idempotent(source: &str) {
    let parsed = parse(source).unwrap_or_else(|error| panic!("parse: {error}\n{source}"));
    let emitted = emit_sysml(&parsed).unwrap_or_else(|error| panic!("emit: {error}\n{source}"));
    let reparsed = parse(&emitted).unwrap_or_else(|error| panic!("reparse: {error}\n{emitted}"));
    assert_eq!(
        parsed.normalize_for_test_comparison(),
        reparsed.normalize_for_test_comparison(),
        "the metadata member changed across format/reparse\nsource:\n{source}emitted:\n{emitted}"
    );
    let again = emit_sysml(&reparsed).unwrap_or_else(|error| panic!("re-emit: {error}\n{emitted}"));
    assert_eq!(
        emitted, again,
        "formatting is not idempotent for:\n{source}"
    );
}

/// Every alternative of `MetadataFeatureDeclaration` and of the `#` productions, through
/// parse → format → parse → format.
///
/// The `typed by` and short-name rows are the ones that would silently normalize: both spell the
/// same relationship as `:`, so an emitter that canonicalized them would still produce a document
/// that parses -- just not the one the author wrote.
#[test]
fn every_declaration_alternative_round_trips_and_formats_idempotently() {
    for source in [
        // The bare head is the OwnedFeatureTyping, relative and absolute.
        "package P {\n  part def A {\n    @Tag;\n  }\n}\n",
        "package P {\n  part def A {\n    @Profile::Tag;\n  }\n}\n",
        "package P {\n  part def A {\n    @$::P::Tag;\n  }\n}\n",
        "package P {\n  part def A {\n    @Profile::'quoted tag';\n  }\n}\n",
        // The declaration half, in both authored spellings, with and without a short name.
        "package P {\n  part def A {\n    @named : Tag;\n  }\n}\n",
        "package P {\n  part def A {\n    @named typed by Tag;\n  }\n}\n",
        "package P {\n  part def A {\n    @<short> : Tag;\n  }\n}\n",
        "package P {\n  part def A {\n    @<short> named : Tag;\n  }\n}\n",
        // The about clause, and both body forms.
        "package P {\n  part def A {\n    @Tag about A, Profile::Other;\n  }\n}\n",
        "package P {\n  part def A {\n    @Tag {\n      doc /* body */\n    }\n  }\n}\n",
        // `#` as a member (ExtendedUsage) and as a prefix (PrefixMetadataMember).
        "package P {\n  part def A {\n    #Tag;\n  }\n}\n",
        "package P {\n  part def A {\n    #Profile::Tag;\n  }\n}\n",
        "package P {\n  part def A {\n    #Tag {\n      doc /* body */\n    }\n  }\n}\n",
        "package P {\n  #Tag\n  part def A;\n}\n",
        "package P {\n  #Tag\n  #Other\n  part def A;\n}\n",
    ] {
        assert_round_trips_and_is_idempotent(source);
    }
}

/// Strict and editor entry points agree on diagnostic-free sigil input.
#[test]
fn strict_and_editor_agree_on_clean_metadata_input() {
    let source = concat!(
        "package P {\n",
        "  metadata def Tag;\n",
        "  #Tag\n",
        "  part def A {\n",
        "    @Tag;\n",
        "    @named : Tag;\n",
        "    @spelled typed by Tag about A;\n",
        "    #Tag;\n",
        "    #Tag {\n",
        "      doc /* body */\n",
        "    }\n",
        "  }\n",
        "}\n",
    );
    let strict = parse(source).expect("strict parse");
    let editor = parse_for_editor(source);
    assert!(
        editor.errors.is_empty(),
        "editor parse of clean input reported diagnostics: {:?}",
        editor.errors
    );
    assert_eq!(
        strict.normalize_for_test_comparison(),
        editor.document.normalize_for_test_comparison(),
        "strict and editor entry points disagree on clean metadata input"
    );
}

/// A `#` head that no production continues must not leave its speculative reference behind.
///
/// `metadata_keyword_usage` allocates the `OwnedFeatureTyping` before it can know whether a `;` or
/// `{` follows, and `metadata_keyword_prefix` before it can know whether a declaration follows. If
/// either kept its allocation on failure, the arena would grow an entry the tree never names --
/// invisible in the AST, but a dangling identity in the serialized envelope.
#[test]
fn a_refused_metadata_head_leaves_no_arena_entry() {
    let clean = parse_for_editor("package P {\n  part def A;\n}\n").document;
    let refused = parse_for_editor("package P {\n  #tag : Foo weirdstuff;\n  part def A;\n}\n");
    assert!(
        !refused.errors.is_empty(),
        "the fixture is only meaningful if the head is actually refused"
    );
    assert_eq!(
        refused.document.qualified_references.len(),
        clean.qualified_references.len(),
        "a refused `#` head left speculative references in the arena"
    );
}

/// The same, for the `@` spelling's speculative `Identification`.
#[test]
fn a_refused_metadata_annotation_leaves_no_arena_entry() {
    let clean = parse_for_editor("package P {\n  part def A;\n}\n").document;
    let refused = parse_for_editor("package P {\n  @tag : Foo weirdstuff;\n  part def A;\n}\n");
    assert!(
        !refused.errors.is_empty(),
        "the fixture is only meaningful if the head is actually refused"
    );
    assert_eq!(
        refused.document.qualified_references.len(),
        clean.qualified_references.len(),
        "a refused `@` head left speculative references in the arena"
    );
}

/// The `#` sigil span is provenance, not decoration: emission writes the sigil from it. A wire
/// document that redirects it at other text would still resolve every reference it names.
#[cfg(feature = "serde")]
#[test]
fn a_metadata_keyword_sigil_span_covering_other_text_is_rejected() {
    let source = "package P {\n  part def A {\n    #Tag;\n  }\n}\n";
    let document = parse_for_editor(source).document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let sigil = "/root/elements/0/value/Package/value/body/Brace/elements/0\
                 /value/PartDef/value/body/Brace/elements/0/value/MetadataKeywordUsage/value/hash_span";
    *tampered
        .pointer_mut(&format!("{sigil}/offset"))
        .expect("the keyword usage's sigil offset") = serde_json::json!(0);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a sigil span that does not cover `#` must be rejected");
    assert!(
        error.to_string().contains("metadata keyword sigil"),
        "expected the sigil check to name itself, got: {error}"
    );
}

/// The `@` sigil, checked the same way.
#[cfg(feature = "serde")]
#[test]
fn a_metadata_annotation_sigil_span_covering_other_text_is_rejected() {
    let source = "package P {\n  part def A {\n    @Tag;\n  }\n}\n";
    let document = parse_for_editor(source).document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let sigil = "/root/elements/0/value/Package/value/body/Brace/elements/0/value/PartDef\
                 /value/body/Brace/elements/0/value/Annotating/MetadataAnnotation/value/introducer/At/span";
    *tampered
        .pointer_mut(&format!("{sigil}/offset"))
        .expect("the annotation's sigil offset") = serde_json::json!(0);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a sigil span that does not cover `@` must be rejected");
    assert!(
        error.to_string().contains("metadata feature introducer"),
        "expected the sigil check to name itself, got: {error}"
    );
}

/// The separator variant and its span are one fact written twice on the wire; they must agree.
///
/// A document claiming `MetadataTypedBy::TypedBy` over a span covering `:` would emit `typed by`
/// for a document whose source says `:`, which is a formatting change the emitter cannot see.
#[cfg(feature = "serde")]
#[test]
fn a_declaration_separator_disagreeing_with_its_span_is_rejected() {
    let source = "package P {\n  part def A {\n    @named : Tag;\n  }\n}\n";
    let document = parse_for_editor(source).document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let declared = "/root/elements/0/value/Package/value/body/Brace/elements/0/value/PartDef\
                    /value/body/Brace/elements/0/value/Annotating/MetadataAnnotation/value/declared_name";
    *tampered
        .pointer_mut(&format!("{declared}/value/typed_by"))
        .expect("the declaration's separator spelling") = serde_json::json!("TypedBy");

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a separator variant contradicting its own span must be rejected");
    assert!(
        error.to_string().contains("metadata declaration separator"),
        "expected the separator check to name itself, got: {error}"
    );
}

/// A dangling `OwnedFeatureTyping` identity is rejected rather than silently emitted.
#[cfg(feature = "serde")]
#[test]
fn a_dangling_metadata_typing_reference_is_rejected() {
    let source = "package P {\n  part def A {\n    #Tag;\n  }\n}\n";
    let document = parse_for_editor(source).document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let reference = "/root/elements/0/value/Package/value/body/Brace/elements/0/value/PartDef\
                     /value/body/Brace/elements/0/value/MetadataKeywordUsage/value/reference";
    *tampered
        .pointer_mut(reference)
        .expect("the keyword usage's typing reference") = serde_json::json!(9999);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a reference with no arena entry must be rejected");
    assert!(
        error.to_string().contains("DanglingReference"),
        "expected a dangling-reference failure, got: {error}"
    );
}
