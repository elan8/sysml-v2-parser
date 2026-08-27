//! Owning-layer mechanics of `PortUsage`'s `OccurrenceUsagePrefix`.
//!
//! What the prefix *parses to* is pinned by the semantic snapshots
//! (`tests/snapshots/sysml/port_usage_prefix_alternatives.md`, `..._owning_scopes.md`,
//! `..._recovery.md`, `..._unterminated.md`): every slot, every mutually exclusive alternative,
//! every scope, and every recovery state with its diagnostic shows up there, so none of that is
//! restated here.
//!
//! What a snapshot cannot show is the layer underneath: whether speculation that fails leaves
//! arena entries behind, whether the strict and editor entry points agree, whether a corrupted
//! wire document claiming impossible prefix states is rejected, and whether the emitted text is a
//! fixed point. Those are what this file holds.
//!
//! The grammar is `planning/port-usage-prefix-matrix.md`.

#[cfg(feature = "serde")]
#[path = "common/serde_wire.rs"]
mod serde_wire;

#[cfg(feature = "serde")]
use serde_wire::{basic_occurrence_prefix_head_mut, occurrence_prefix_mut};
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

/// Every slot of the prefix, on every declaration shape, through parse → format → parse → format.
///
/// The rows worth having are the ones that would silently normalize: a slot dropped because the
/// emitter had no field for it, two slots emitted in the wrong order so the reparse assigns them
/// to different productions, or an extension keyword rebuilt by pasting `#` onto a rendered name.
#[test]
fn every_prefix_slot_round_trips_and_formats_idempotently() {
    for source in [
        // No prefix at all: the emitter must not invent one -- nor a `def` keyword, which the
        // `def`-optional port-definition parser used to add at package scope.
        "package P {\n    port p;\n}\n",
        // Each `RefPrefix` slot alone.
        "package P {\n    in port p : T;\n}\n",
        "package P {\n    out port p : T;\n}\n",
        "package P {\n    inout port p : T;\n}\n",
        "package P {\n    derived port p : T;\n}\n",
        "package P {\n    abstract port p : T;\n}\n",
        "package P {\n    variation port p : T;\n}\n",
        "package P {\n    constant port p : T;\n}\n",
        // `BasicUsagePrefix`'s `ref`, then the two `OccurrenceUsagePrefix` slots.
        "package P {\n    ref port p : T;\n}\n",
        "package P {\n    individual port p : T;\n}\n",
        "package P {\n    snapshot port p;\n}\n",
        "package P {\n    timeslice port p;\n}\n",
        "package P {\n    individual snapshot port p;\n}\n",
        "package P {\n    individual timeslice port p;\n}\n",
        // The whole prefix, in the only legal order.
        "package P {\n    in derived abstract constant ref individual snapshot port p : T;\n}\n",
        "package P {\n    out variation ref individual timeslice port p : T;\n}\n",
        // Extension keywords: one, several, qualified, absolute, quoted.
        "package P {\n    #Tag port p;\n}\n",
        "package P {\n    #First #Second port p;\n}\n",
        "package P {\n    #Lib::Tag port p;\n}\n",
        "package P {\n    #$::P::Tag port p;\n}\n",
        "package P {\n    #'safety critical' port p;\n}\n",
        "package P {\n    abstract ref #Tag port p : T;\n}\n",
        // `MemberPrefix` visibility is the membership's, and precedes the prefix.
        "package P {\n    private ref individual port p : T;\n}\n",
        "package P {\n    protected #Tag port p : T;\n}\n",
        // Every declaration shape the `Usage` tail reaches, with a prefix in front of it.
        "package P {\n    ref port <sn> p : T;\n}\n",
        "package P {\n    ref port : T;\n}\n",
        "package P {\n    ref port :>> target;\n}\n",
        "package P {\n    ref port :>> target : T;\n}\n",
        "package P {\n    derived port p : T[0..*] ordered nonunique;\n}\n",
        "package P {\n    derived port p : T :> other;\n}\n",
        "package P {\n    derived port p : T :>> other;\n}\n",
        "package P {\n    derived port p : T ::> other;\n}\n",
        "package P {\n    derived port p : T => other;\n}\n",
        "package P {\n    derived port p : T intersects other;\n}\n",
        "package P {\n    constant port :>> target = other;\n}\n",
        // Both body forms, and a member inside the braced one.
        "package P {\n    ref individual port p : T {\n        attribute a;\n    }\n}\n",
        "package P {\n    #Tag port p : T {\n        port inner : T;\n    }\n}\n",
        // The Systems Library shape the `def`-optional definition parser used to claim and strip.
        "package P {\n    abstract port ports : Port[0..*] nonunique :> objects;\n}\n",
        // `port def` beside the usage: the keyword decides, and the prefix does not change that.
        "package P {\n    port def D;\n    ref individual port p : D;\n}\n",
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
        "    port def T;\n",
        "    in derived abstract constant ref individual snapshot port p : T;\n",
        "    out variation ref individual timeslice port q : T;\n",
        "    #Tag #P::Tag port r : T;\n",
        "    private ref port s : T;\n",
        "    part def Owner {\n",
        "        snapshot port inDefinition;\n",
        "        #Tag port tagged : T;\n",
        "    }\n",
        "    port def PortOwner {\n",
        "        ref port nested : T;\n",
        "    }\n",
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

/// A prefix whose `port` never arrives must not leave its speculative references behind.
///
/// A `UsageExtensionKeyword` allocates an arena entry for its qualified name before the parser can
/// know whether `port` follows. If a refused parse kept them, the arena would grow entries the
/// tree never names -- invisible in the AST, but dangling identities in the serialized envelope
/// and shifted identities for every reference allocated after them.
#[test]
fn a_refused_port_prefix_leaves_no_arena_entry() {
    let baseline = parse_for_editor("package P {\n    part def A;\n}\n")
        .document
        .qualified_references
        .len();
    for refused_source in [
        // An extension keyword whose usage never arrives, and which the `PrefixMetadataMember`
        // fallback also refuses -- a digit cannot start the declaration that fallback requires --
        // so nothing in the finished document names the identity the prefix parser allocated.
        "package P {\n    part def A {\n        #Ghost 123;\n    }\n}\n",
        // A complete prefix with no `port` after it.
        "package P {\n    part def A {\n        in derived constant ref;\n    }\n}\n",
        // An out-of-order prefix: the second keyword is never a prefix slot at that position.
        "package P {\n    part def A {\n        ref derived port p;\n    }\n}\n",
        // Two alternatives of one slot.
        "package P {\n    part def A {\n        snapshot timeslice port p;\n    }\n}\n",
        // `port def` after a usage-only prefix is a definition this parser must refuse, not a
        // usage named `def`.
        "package P {\n    part def A {\n        ref port def B;\n    }\n}\n",
    ] {
        let refused = parse_for_editor(refused_source);
        assert!(
            !refused.errors.is_empty(),
            "the fixture is only meaningful if the member is actually refused:\n{refused_source}"
        );
        assert_eq!(
            refused.document.qualified_references.len(),
            baseline,
            "a refused port prefix left speculative references in the arena:\n{refused_source}"
        );
    }
}

/// A wire document may not claim a prefix keyword span over text that is not that keyword.
#[cfg(feature = "serde")]
#[test]
fn a_port_prefix_keyword_span_covering_other_text_is_rejected() {
    let document = parse_for_editor("package P {\n    derived port p;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    *basic_occurrence_prefix_head_mut(&mut tampered, 0, 1)
        .get_mut("basic")
        .and_then(|basic| basic.get_mut("ref_prefix"))
        .and_then(|ref_prefix| ref_prefix.get_mut("derived_span"))
        .and_then(|span| span.get_mut("offset"))
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
fn a_port_direction_span_covering_a_different_keyword_is_rejected() {
    let document = parse_for_editor("package P {\n    inout port p;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    *basic_occurrence_prefix_head_mut(&mut tampered, 0, 1)
        .get_mut("basic")
        .and_then(|basic| basic.get_mut("ref_prefix"))
        .and_then(|ref_prefix| ref_prefix.get_mut("direction"))
        .and_then(|direction| direction.get_mut("value"))
        .expect("the direction alternative") = serde_json::json!("In");

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a direction claiming `in` over `inout` must be rejected");
    assert!(
        error.to_string().contains("usage prefix direction keyword"),
        "expected the direction check to name itself, got: {error}"
    );
}

/// A variance slot must cover the alternative its enum claims.
#[cfg(feature = "serde")]
#[test]
fn a_port_variance_span_covering_a_different_keyword_is_rejected() {
    let document = parse_for_editor("package P {\n    abstract port p;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    *basic_occurrence_prefix_head_mut(&mut tampered, 0, 1)
        .get_mut("basic")
        .and_then(|basic| basic.get_mut("ref_prefix"))
        .and_then(|ref_prefix| ref_prefix.get_mut("variance"))
        .and_then(|variance| variance.get_mut("value"))
        .expect("the variance alternative") = serde_json::json!("Variation");

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a variance claiming `variation` over `abstract` must be rejected");
    assert!(
        error.to_string().contains("usage prefix variance keyword"),
        "expected the variance check to name itself, got: {error}"
    );
}

/// A portion kind must cover the alternative its enum claims.
#[cfg(feature = "serde")]
#[test]
fn a_port_portion_span_covering_a_different_keyword_is_rejected() {
    let document = parse_for_editor("package P {\n    snapshot port p;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    *basic_occurrence_prefix_head_mut(&mut tampered, 0, 1)
        .get_mut("portion")
        .and_then(|portion| portion.get_mut("value"))
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
fn port_prefix_slots_written_out_of_order_are_rejected() {
    let document = parse_for_editor("package P {\n    ref individual port p;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let head = basic_occurrence_prefix_head_mut(&mut tampered, 0, 1);
    let reference_span = head
        .get("basic")
        .and_then(|basic| basic.get("reference_span"))
        .cloned()
        .expect("the `ref` keyword span");
    let individual_span = head
        .get("individual_span")
        .cloned()
        .expect("the `individual` keyword span");
    *head
        .get_mut("basic")
        .and_then(|basic| basic.get_mut("reference_span"))
        .expect("the `ref` keyword span") = individual_span;
    *head
        .get_mut("individual_span")
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
fn a_port_extension_keyword_sigil_covering_other_text_is_rejected() {
    let document = parse_for_editor("package P {\n    #Tag port p;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    *occurrence_prefix_mut(&mut tampered, 0, 1)
        .get_mut("extension_keywords")
        .and_then(|keywords| keywords.get_mut(0))
        .and_then(|keyword| keyword.get_mut("value"))
        .and_then(|value| value.get_mut("hash_span"))
        .and_then(|span| span.get_mut("offset"))
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
fn a_dangling_port_extension_keyword_reference_is_rejected() {
    let document = parse_for_editor("package P {\n    #Tag port p;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    *occurrence_prefix_mut(&mut tampered, 0, 1)
        .get_mut("extension_keywords")
        .and_then(|keywords| keywords.get_mut(0))
        .and_then(|keyword| keyword.get_mut("value"))
        .and_then(|value| value.get_mut("annotation"))
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
fn a_port_prefix_span_outside_its_own_declaration_is_rejected() {
    let document =
        parse_for_editor("package P {\n    individual port p;\n    individual port q;\n}\n")
            .document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let second_offset = basic_occurrence_prefix_head_mut(&mut tampered, 1, 2)
        .get("individual_span")
        .and_then(|span| span.get("offset"))
        .cloned()
        .expect("the second usage's `individual` keyword offset");
    *basic_occurrence_prefix_head_mut(&mut tampered, 0, 2)
        .get_mut("individual_span")
        .and_then(|span| span.get_mut("offset"))
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

/// A document whose schema version is not this build's is rejected outright.
#[cfg(feature = "serde")]
#[test]
fn a_port_prefix_document_from_another_schema_version_is_rejected() {
    let document = parse_for_editor("package P {\n    ref individual port p;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    *tampered
        .pointer_mut("/ast_version")
        .expect("the envelope's schema version") =
        serde_json::json!(sysml_v2_parser::PARSE_AST_VERSION - 1);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a document from another schema version must be rejected");
    assert!(
        error.to_string().contains("version"),
        "expected a version failure, got: {error}"
    );
}

/// A prefixed document round-trips through the whole serialized envelope unchanged.
#[cfg(feature = "serde")]
#[test]
fn a_prefixed_port_document_round_trips_through_the_envelope() {
    let source = concat!(
        "package P {\n",
        "    metadata def Tag;\n",
        "    port def T;\n",
        "    in derived abstract constant ref individual snapshot #Tag #P::Tag port p : T {\n",
        "        attribute a;\n",
        "    }\n",
        "    out variation ref individual timeslice #Tag port q : T;\n",
        "    private port <sn> r : T[0..*] ordered nonunique :> p;\n",
        "    ref port :>> p : T;\n",
        "}\n",
    );
    let document = parse(source).expect("parse");
    let encoded = serde_json::to_value(&document).expect("serialize");
    let decoded: sysml_v2_parser::ast::ParsedDocument =
        serde_json::from_value(encoded).expect("deserialize");
    assert_eq!(decoded, document);
}
