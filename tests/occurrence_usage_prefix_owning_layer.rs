//! Owning-layer mechanics of the shared `OccurrenceUsagePrefix` seam.
//!
//! What the prefix *parses to* is pinned by the semantic snapshots
//! (`tests/snapshots/sysml/occurrence_usage_prefix_alternatives.md`, `..._owning_scopes.md`,
//! `..._recovery.md`, `..._unterminated.md`): every slot, every mutually exclusive alternative,
//! every scope, and every recovery state with its diagnostic shows up there, so none of that is
//! restated here.
//!
//! What a snapshot cannot show is the layer underneath: whether speculation that fails leaves
//! arena entries behind, whether a corrupted wire document claiming impossible prefix states is
//! rejected, and whether the emitted text is a fixed point. Those are what this file holds.
//!
//! The grammar is `planning/occurrence-usage-prefix-matrix.md`.

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
        "the prefix changed across format/reparse\nsource:\n{source}emitted:\n{emitted}"
    );
    let again = emit_sysml(&reparsed).unwrap_or_else(|error| panic!("re-emit: {error}\n{emitted}"));
    assert_eq!(
        emitted, again,
        "formatting is not idempotent for:\n{source}"
    );
}

/// Every slot of the prefix, on every migrated family, through parse → format → parse → format.
///
/// The rows worth having are the ones that would silently normalize: a slot dropped because the
/// emitter had no field for it, two slots emitted in the wrong order so the reparse assigns them
/// to different productions, or an extension keyword rebuilt by pasting `#` onto a rendered name.
#[test]
fn every_prefix_slot_round_trips_and_formats_idempotently() {
    for source in [
        // No prefix at all: the emitter must not invent one.
        "package P {\n    occurrence o;\n}\n",
        "package P {\n    item i;\n}\n",
        "package P {\n    satisfy r by p;\n}\n",
        // Each `RefPrefix` slot alone.
        "package P {\n    in occurrence o;\n}\n",
        "package P {\n    out occurrence o;\n}\n",
        "package P {\n    inout occurrence o;\n}\n",
        "package P {\n    derived occurrence o;\n}\n",
        "package P {\n    abstract occurrence o;\n}\n",
        "package P {\n    variation occurrence o;\n}\n",
        "package P {\n    constant occurrence o;\n}\n",
        // `BasicUsagePrefix`'s `ref`, then the two `OccurrenceUsagePrefix` slots.
        "package P {\n    ref occurrence o;\n}\n",
        "package P {\n    individual occurrence o;\n}\n",
        "package P {\n    individual o;\n}\n",
        "package P {\n    snapshot s;\n}\n",
        "package P {\n    timeslice t;\n}\n",
        "package P {\n    individual snapshot s;\n}\n",
        "package P {\n    individual timeslice t;\n}\n",
        // The whole prefix, in the only legal order, on each migrated family.
        "package P {\n    in derived abstract constant ref individual snapshot occurrence o;\n}\n",
        "package P {\n    out variation constant ref individual timeslice item i;\n}\n",
        "package P {\n    inout derived abstract constant ref individual snapshot assert not satisfy r by p;\n}\n",
        // Extension keywords: one, several, qualified, absolute, quoted.
        "package P {\n    #Tag occurrence o;\n}\n",
        "package P {\n    #First #Second occurrence o;\n}\n",
        "package P {\n    #Lib::Tag occurrence o;\n}\n",
        "package P {\n    #$::P::Tag occurrence o;\n}\n",
        "package P {\n    #'safety critical' occurrence o;\n}\n",
        "package P {\n    abstract ref #Tag occurrence o;\n}\n",
        "package P {\n    #Tag item i;\n}\n",
        "package P {\n    #Tag satisfy r by p;\n}\n",
        // `MemberPrefix` visibility is the membership's, and precedes the prefix.
        "package P {\n    private ref individual occurrence o;\n}\n",
        "package P {\n    protected satisfy r by p;\n}\n",
        // Both body forms, and a member inside the braced one.
        "package P {\n    ref individual occurrence o {\n        attribute a;\n    }\n}\n",
        "package P {\n    #Tag satisfy r by p {\n        require constraint c;\n    }\n}\n",
    ] {
        assert_round_trips_and_is_idempotent(source);
    }
}

/// Strict and editor entry points agree on diagnostic-free prefixed input.
#[test]
fn strict_and_editor_agree_on_clean_prefixed_input() {
    let source = concat!(
        "package P {\n",
        "    metadata def Tag;\n",
        "    in derived abstract constant ref individual snapshot occurrence o;\n",
        "    out variation ref individual timeslice item i;\n",
        "    #Tag #Lib::Other satisfy r by p;\n",
        "    private individual o2;\n",
        "    then timeslice t;\n",
        "    event occurrence e;\n",
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
        "strict and editor entry points disagree on clean prefixed input"
    );
}

/// A prefix whose usage never arrives must not leave its speculative references behind.
///
/// A `UsageExtensionKeyword` allocates an arena entry for its qualified name before the parser can
/// know whether a head the family owns follows. If a refused parse kept them, the arena would grow
/// entries the tree never names -- invisible in the AST, but dangling identities in the serialized
/// envelope and shifted identities for every reference allocated after them.
#[test]
fn a_refused_prefix_leaves_no_arena_entry() {
    let baseline = parse_for_editor("package P {\n    part def A;\n}\n")
        .document
        .qualified_references
        .len();
    for refused_source in [
        // An extension keyword whose usage never arrives, and which the `PrefixMetadataMember`
        // fallback also refuses -- a digit cannot start the declaration that fallback requires --
        // so nothing in the finished document names the identity the prefix parser allocated.
        "package P {\n    part def A {\n        #Ghost 123;\n    }\n}\n",
        // A complete prefix with no usage after it.
        "package P {\n    part def A {\n        in derived constant;\n    }\n}\n",
        // An out-of-order prefix: the second keyword is never a prefix slot at that position.
        "package P {\n    part def A {\n        ref abstract occurrence o;\n    }\n}\n",
        // Two alternatives of one slot.
        "package P {\n    part def A {\n        snapshot timeslice t;\n    }\n}\n",
    ] {
        let refused = parse_for_editor(refused_source);
        assert!(
            !refused.errors.is_empty(),
            "the fixture is only meaningful if the member is actually refused:\n{refused_source}"
        );
        assert_eq!(
            refused.document.qualified_references.len(),
            baseline,
            "a refused prefix left speculative references in the arena:\n{refused_source}"
        );
    }
}

/// The prefix's own parser never consumes input it does not record.
///
/// An out-of-order or duplicated slot leaves its second keyword unconsumed, which is what makes
/// the owning production fail rather than quietly accepting a reordered prefix. Observable here as
/// the whole member becoming one recovery node with its exact authored span, and the sibling after
/// it surviving.
#[test]
fn an_illegal_prefix_is_refused_rather_than_normalized() {
    for (source, malformed) in [
        (
            "package P {\n    part def A {\n        ref abstract occurrence o;\n        occurrence valid;\n    }\n}\n",
            "ref abstract occurrence o;",
        ),
        (
            "package P {\n    part def A {\n        abstract variation occurrence o;\n        occurrence valid;\n    }\n}\n",
            "abstract variation occurrence o;",
        ),
        (
            "package P {\n    part def A {\n        in out occurrence o;\n        occurrence valid;\n    }\n}\n",
            "in out occurrence o;",
        ),
        (
            "package P {\n    part def A {\n        snapshot timeslice t;\n        occurrence valid;\n    }\n}\n",
            "snapshot timeslice t;",
        ),
    ] {
        let result = parse_for_editor(source);
        assert!(
            !result.errors.is_empty(),
            "an illegal prefix must report:\n{source}"
        );
        let rendered = {
            let mut out = Vec::new();
            sysml_v2_parser::ast::WriteSemanticAst::write_semantic_ast(&result.document, &mut out)
                .expect("semantic projection");
            String::from_utf8(out).expect("utf-8 projection")
        };
        assert!(
            rendered.contains(&format!("(found \"{malformed}\")")),
            "the refused member must keep its exact authored span: {rendered}"
        );
        assert!(
            rendered.contains("(declaration \"valid\")"),
            "the sibling after a refused prefix must survive: {rendered}"
        );
    }
}

/// A wire document may not claim a prefix keyword span over text that is not that keyword.
#[cfg(feature = "serde")]
#[test]
fn a_prefix_keyword_span_covering_other_text_is_rejected() {
    let document = parse_for_editor("package P {\n    derived occurrence o;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let prefix = "/root/elements/0/value/Package/value/body/Brace/elements/0\
                  /value/OccurrenceUsage/value/prefix";
    *tampered
        .pointer_mut(&format!("{prefix}/basic/ref_prefix/derived_span/offset"))
        .expect("the `derived` keyword offset") = serde_json::json!(0);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a `derived` span that does not cover `derived` must be rejected");
    assert!(
        error.to_string().contains("usage prefix `derived` keyword"),
        "expected the `derived` check to name itself, got: {error}"
    );
}

/// A direction slot must cover the alternative its enum claims, not the other one.
#[cfg(feature = "serde")]
#[test]
fn a_direction_span_covering_a_different_keyword_is_rejected() {
    let document = parse_for_editor("package P {\n    inout occurrence o;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let direction = "/root/elements/0/value/Package/value/body/Brace/elements/0\
                     /value/OccurrenceUsage/value/prefix/basic/ref_prefix/direction";
    *tampered
        .pointer_mut(&format!("{direction}/value"))
        .expect("the direction alternative") = serde_json::json!("In");

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a direction claiming `in` over `inout` must be rejected");
    assert!(
        error.to_string().contains("usage prefix direction keyword"),
        "expected the direction check to name itself, got: {error}"
    );
}

/// A portion kind must cover the alternative its enum claims.
#[cfg(feature = "serde")]
#[test]
fn a_portion_span_covering_a_different_keyword_is_rejected() {
    let document = parse_for_editor("package P {\n    snapshot s;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let portion = "/root/elements/0/value/Package/value/body/Brace/elements/0\
                   /value/OccurrenceUsage/value/prefix/portion";
    *tampered
        .pointer_mut(&format!("{portion}/value"))
        .expect("the portion alternative") = serde_json::json!("Timeslice");

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a portion claiming `timeslice` over `snapshot` must be rejected");
    assert!(
        error
            .to_string()
            .contains("occurrence prefix portion keyword"),
        "expected the portion check to name itself, got: {error}"
    );
}

/// Prefix slots written out of the production's order are rejected.
///
/// The type makes the mutually exclusive slots exclusive; what it cannot express is that
/// `individual` follows `ref`, which is why a wire document that swaps their spans has to be
/// caught at the boundary rather than accepted and re-emitted in a different order.
#[cfg(feature = "serde")]
#[test]
fn prefix_slots_written_out_of_order_are_rejected() {
    let document = parse_for_editor("package P {\n    ref individual o;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let prefix = "/root/elements/0/value/Package/value/body/Brace/elements/0\
                  /value/OccurrenceUsage/value/prefix";
    let reference_span = tampered
        .pointer(&format!("{prefix}/basic/reference_span"))
        .cloned()
        .expect("the `ref` keyword span");
    let individual_span = tampered
        .pointer(&format!("{prefix}/individual_span"))
        .cloned()
        .expect("the `individual` keyword span");
    *tampered
        .pointer_mut(&format!("{prefix}/basic/reference_span"))
        .expect("the `ref` keyword span") = individual_span;
    *tampered
        .pointer_mut(&format!("{prefix}/individual_span"))
        .expect("the `individual` keyword span") = reference_span;

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("prefix slots claiming each other's tokens must be rejected");
    assert!(
        error.to_string().contains("usage prefix"),
        "expected a prefix-slot failure, got: {error}"
    );
}

/// The `#` of a `UsageExtensionKeyword` is syntax, and a wire document may not point it elsewhere.
#[cfg(feature = "serde")]
#[test]
fn an_extension_keyword_sigil_covering_other_text_is_rejected() {
    let document = parse_for_editor("package P {\n    #Tag occurrence o;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let sigil = "/root/elements/0/value/Package/value/body/Brace/elements/0\
                 /value/OccurrenceUsage/value/prefix/extension_keywords/0/value/hash_span";
    *tampered
        .pointer_mut(&format!("{sigil}/offset"))
        .expect("the extension keyword sigil offset") = serde_json::json!(0);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("an extension keyword sigil that does not cover `#` must be rejected");
    assert!(
        error.to_string().contains("usage extension keyword sigil"),
        "expected the sigil check to name itself, got: {error}"
    );
}

/// An extension keyword's identity must resolve in the arena that travelled with it.
#[cfg(feature = "serde")]
#[test]
fn a_dangling_extension_keyword_reference_is_rejected() {
    let document = parse_for_editor("package P {\n    #Tag occurrence o;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let annotation = "/root/elements/0/value/Package/value/body/Brace/elements/0\
                      /value/OccurrenceUsage/value/prefix/extension_keywords/0/value/annotation";
    *tampered
        .pointer_mut(annotation)
        .expect("the extension keyword reference") = serde_json::json!(9999);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a reference with no arena entry must be rejected");
    assert!(
        error.to_string().contains("DanglingReference"),
        "expected a dangling-reference failure, got: {error}"
    );
}

/// A prefix span that points outside the declaration owning it is rejected.
#[cfg(feature = "serde")]
#[test]
fn a_prefix_span_outside_its_own_declaration_is_rejected() {
    let document =
        parse_for_editor("package P {\n    individual o;\n    individual o2;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let first = "/root/elements/0/value/Package/value/body/Brace/elements/0\
                 /value/OccurrenceUsage/value/prefix/individual_span/offset";
    let second_offset = tampered
        .pointer(
            "/root/elements/0/value/Package/value/body/Brace/elements/1\
             /value/OccurrenceUsage/value/prefix/individual_span/offset",
        )
        .cloned()
        .expect("the second usage's `individual` keyword offset");
    *tampered
        .pointer_mut(first)
        .expect("the first usage's `individual` keyword offset") = second_offset;

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a prefix keyword belonging to another member must be rejected");
    assert!(
        error
            .to_string()
            .contains("occurrence prefix `individual` keyword"),
        "expected the `individual` check to name itself, got: {error}"
    );
}

/// A prefixed document round-trips through the whole serialized envelope unchanged.
#[cfg(feature = "serde")]
#[test]
fn a_prefixed_document_round_trips_through_the_envelope() {
    let source = concat!(
        "package P {\n",
        "    metadata def Tag;\n",
        "    in derived abstract constant ref individual snapshot #Tag #P::Tag occurrence o {\n",
        "        attribute a;\n",
        "    }\n",
        "    out variation ref individual timeslice #Tag item i;\n",
        "    private inout constant ref #Tag assert not satisfy Requirements::spec by p.engine;\n",
        "}\n",
    );
    let document = parse(source).expect("parse");
    let encoded = serde_json::to_value(&document).expect("serialize");
    let decoded: sysml_v2_parser::ast::ParsedDocument =
        serde_json::from_value(encoded).expect("deserialize");
    assert_eq!(decoded, document);
}
