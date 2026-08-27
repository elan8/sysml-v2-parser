//! Owning-layer mechanics of the `SatisfyRequirementUsage` seam that have no document projection.
//!
//! What the production *parses to* is pinned by the semantic snapshots
//! (`tests/snapshots/sysml/satisfy_requirement_usage_alternatives.md`, `..._owning_scopes.md`,
//! `..._recovery.md`): the two requirement alternatives, the `assert`/`not` prefixes, the `by`
//! clause, the requirement-body member set, the scopes, and the recovery states with their
//! diagnostics all show up there, so none of that is restated here.
//!
//! What a snapshot cannot show is the layer underneath: whether a corrupted wire document is
//! rejected, whether speculation that fails leaves arena entries behind, and whether formatting the
//! emitted text reparses to the same tree. Those are what this file holds.

use sysml_v2_parser::ast::WriteSemanticAst;
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
        "the satisfy usage changed across format/reparse\nsource:\n{source}emitted:\n{emitted}"
    );
    let again = emit_sysml(&reparsed).unwrap_or_else(|error| panic!("re-emit: {error}\n{emitted}"));
    assert_eq!(
        emitted, again,
        "formatting is not idempotent for:\n{source}"
    );
}

/// Every alternative of the production, through parse → format → parse → format.
///
/// The rows that would silently normalize are the ones worth having: an omitted `by` that an
/// emitter fills in from the satisfied requirement, an inline declaration label emitted as though
/// it were a qualified reference, and a `.`-separated feature chain rebuilt with the wrong
/// separator all produce text that still parses -- just not the document the author wrote.
#[test]
fn every_alternative_round_trips_and_formats_idempotently() {
    for source in [
        // The reference alternative: relative, absolute, `::`-qualified, `.`-chained, quoted.
        "package P {\n    satisfy r by p;\n}\n",
        "package P {\n    satisfy Requirements::spec by p;\n}\n",
        "package P {\n    satisfy $::P::spec by p;\n}\n",
        "package P {\n    satisfy spec by vehicle.engine;\n}\n",
        "package P {\n    satisfy spec by Model::vehicle.engine;\n}\n",
        "package P {\n    satisfy 'vehicle1-c1 Specification' by 'vehicle 1';\n}\n",
        // An omitted `by` clause stays omitted.
        "package P {\n    satisfy r;\n}\n",
        "package P {\n    satisfy 'system structure perspective';\n}\n",
        // The prefixes, in all four legal combinations.
        "package P {\n    assert satisfy r by p;\n}\n",
        "package P {\n    not satisfy r by p;\n}\n",
        "package P {\n    assert not satisfy r by p;\n}\n",
        // The inline-declaration alternative: named, typed, short-named, and anonymous.
        "package P {\n    satisfy requirement req1 by system;\n}\n",
        "package P {\n    satisfy requirement req1 : Req1 by system;\n}\n",
        "package P {\n    satisfy requirement <'1.1'> req1 : Req1 by system;\n}\n",
        "package P {\n    satisfy requirement req1 : Req1;\n}\n",
        "package P {\n    satisfy requirement by system;\n}\n",
        "package P {\n    satisfy requirement;\n}\n",
        "package P {\n    satisfy requirement 'quoted name' : Req1;\n}\n",
        // `FeatureSpecializationPart` on both alternatives, and `ValuePart`.
        "package P {\n    satisfy r[1] by p;\n}\n",
        "package P {\n    satisfy r :> base by p;\n}\n",
        "package P {\n    satisfy r :>> old by p;\n}\n",
        "package P {\n    satisfy r = 3 by p;\n}\n",
        "package P {\n    satisfy requirement r : Req1 :> base by p;\n}\n",
        // Both body forms, and requirement-specific members a constraint body cannot hold.
        "package P {\n    satisfy r by p {\n    }\n}\n",
        "package P {\n    satisfy r by p {\n        doc\n        /* why */\n    }\n}\n",
        concat!(
            "package P {\n",
            "    satisfy r by p {\n",
            "        subject s : Vehicle;\n",
            "        require constraint c;\n",
            "        assume constraint a;\n",
            "        frame f;\n",
            "        actor operator : Person;\n",
            "        stakeholder concernHolder;\n",
            "        requirement nested :> other;\n",
            "        satisfy inner by q;\n",
            "    }\n",
            "}\n",
        ),
    ] {
        assert_round_trips_and_is_idempotent(source);
    }
}

/// Every materially distinct scope that dispatches the production, round-tripped.
#[test]
fn every_owning_scope_round_trips_and_formats_idempotently() {
    for source in [
        "package P {\n    satisfy r by p;\n}\n",
        "package P {\n    part def A {\n        satisfy r by p;\n    }\n}\n",
        "package P {\n    part a {\n        satisfy r by p;\n    }\n}\n",
        "package P {\n    occurrence o {\n        satisfy r by p;\n    }\n}\n",
        "package P {\n    view def V {\n        satisfy r by p;\n    }\n}\n",
        "package P {\n    view v : V {\n        satisfy r by p;\n    }\n}\n",
        "package P {\n    requirement def R {\n        satisfy r by p;\n    }\n}\n",
    ] {
        assert_round_trips_and_is_idempotent(source);
    }
}

/// Strict and editor entry points agree on diagnostic-free satisfy input.
#[test]
fn strict_and_editor_agree_on_clean_satisfy_input() {
    let source = concat!(
        "package P {\n",
        "    requirement def R;\n",
        "    part p;\n",
        "    satisfy r by p;\n",
        "    assert satisfy r by p;\n",
        "    not satisfy r by p;\n",
        "    assert not satisfy r by p;\n",
        "    satisfy requirement req1 : R by p;\n",
        "    satisfy Requirements::spec by p.engine {\n",
        "        require constraint c;\n",
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
        "strict and editor entry points disagree on clean satisfy input"
    );
}

/// A `satisfy` head that no alternative completes must not leave its speculative references behind.
///
/// The satisfied requirement, the `by` subject, and every specialization target are allocated
/// before the parser can know whether a `RequirementBody` follows. If the refused parse kept them,
/// the arena would grow entries the tree never names -- invisible in the AST, but dangling
/// identities in the serialized envelope.
#[test]
fn a_refused_satisfy_head_leaves_no_arena_entry() {
    let clean = parse_for_editor("package P {\n    part def A;\n}\n").document;
    for refused_source in [
        // No body terminator at all.
        "package P {\n    satisfy spec by target\n    part def A;\n}\n",
        // `by` with nothing to name.
        "package P {\n    satisfy spec by ;\n    part def A;\n}\n",
        // A specialization clause that is not one of this production's `FeatureSpecialization`s.
        "package P {\n    satisfy spec intersects other by target;\n    part def A;\n}\n",
    ] {
        let refused = parse_for_editor(refused_source);
        assert!(
            !refused.errors.is_empty(),
            "the fixture is only meaningful if the head is actually refused:\n{refused_source}"
        );
        assert_eq!(
            refused.document.qualified_references.len(),
            clean.qualified_references.len(),
            "a refused satisfy head left speculative references in the arena:\n{refused_source}"
        );
    }
}

/// A refused satisfy usage must not consume the valid sibling that follows it.
#[test]
fn recovery_preserves_the_sibling_after_a_refused_satisfy() {
    let result = parse_for_editor("package P {\n    satisfy spec by ;\n    part def A;\n}\n");
    assert!(
        !result.errors.is_empty(),
        "the malformed member must report"
    );
    let rendered = {
        let mut out = Vec::new();
        WriteSemanticAst::write_semantic_ast(&result.document, &mut out)
            .expect("semantic projection");
        String::from_utf8(out).expect("utf-8 projection")
    };
    assert!(
        rendered.contains("(malformed"),
        "the malformed satisfy must stay an explicit recovery node: {rendered}"
    );
    assert!(
        rendered.contains("(part-def (name \"A\")"),
        "the sibling after the malformed satisfy must survive: {rendered}"
    );
    assert!(
        !rendered.contains("(satisfy "),
        "a refused satisfy must not be turned into an apparently valid one: {rendered}"
    );
}

/// Recovery must synchronize on every keyword that can *begin* a satisfy usage, not only on
/// `satisfy`.
///
/// `assert`, `not` and `satisfy` are all FIRST tokens of one production. A malformed member with
/// no terminator of its own scans forward to the next recognized starter; if a scope's starter
/// list named only `satisfy`, that scan ran past the `assert`/`not` prefix and took the whole
/// prefixed usage -- including its `;` -- into the recovery node. Every wired scope is checked
/// against every prefix combination here, because two of them (occurrence bodies and view
/// definition bodies) have no member projection in the semantic snapshot yet.
#[test]
fn recovery_stops_at_a_prefixed_satisfy_in_every_owning_scope() {
    for wrapper in [
        "package P {\n  MALFORMED\n  SATISFY\n  part after;\n}\n",
        "package P {\n  part def A {\n    MALFORMED\n    SATISFY\n    part after;\n  }\n}\n",
        "package P {\n  part a {\n    MALFORMED\n    SATISFY\n    part after;\n  }\n}\n",
        "package P {\n  occurrence o {\n    MALFORMED\n    SATISFY\n    part after;\n  }\n}\n",
        "package P {\n  view def V {\n    MALFORMED\n    SATISFY\n    doc\n    /* after */\n  }\n}\n",
        "package P {\n  view v : V {\n    MALFORMED\n    SATISFY\n    doc\n    /* after */\n  }\n}\n",
        "package P {\n  requirement def R {\n    MALFORMED\n    SATISFY\n    subject after;\n  }\n}\n",
    ] {
        for satisfy in [
            "satisfy Spec by target;",
            "not satisfy Spec by target;",
            "assert satisfy Spec by target;",
            "assert not satisfy Spec by target;",
        ] {
            let source = wrapper
                .replace("MALFORMED", "bogus token here")
                .replace("SATISFY", satisfy);
            let result = parse_for_editor(&source);
            assert!(
                !result.errors.is_empty(),
                "the fixture is only meaningful if the malformed member reports:\n{source}"
            );
            let recovered = sysml_v2_parser::emit_recovered_sysml(&result.document)
                .unwrap_or_else(|error| panic!("emit recovered: {error}\n{source}"));
            assert!(
                recovered.contains(satisfy),
                "recovery consumed the prefixed satisfy usage that followed it\nsource:\n{source}recovered:\n{recovered}"
            );
            assert!(
                recovered.contains("bogus token here"),
                "the malformed member lost its captured span\nsource:\n{source}recovered:\n{recovered}"
            );
        }
    }
}

/// A comment between two of the production's keywords is trivia, like a space.
///
/// `assert /* why */ satisfy r by p;` used to fail: the separator after `assert` was `ws1`, which
/// consumes whitespace but stops at `/`, leaving the comment in front of `satisfy`. The keyword
/// token now separates with `ws_and_comments`. The abutting spelling (`satisfy/* c */ r`) is still
/// rejected by the crate-wide keyword-boundary predicate, which behaves identically for `part`,
/// `import` and every other keyword; that is a separate lexical seam, asserted here so the shared
/// behaviour is visible rather than assumed.
#[test]
fn a_comment_may_separate_the_production_keywords() {
    for source in [
        "package P {\n    assert /* why */ satisfy r by p;\n}\n",
        "package P {\n    not /* why */ satisfy r by p;\n}\n",
        "package P {\n    assert /* a */ not /* b */ satisfy r by p;\n}\n",
        "package P {\n    satisfy /* why */ r by p;\n}\n",
        "package P {\n    satisfy r by /* why */ p;\n}\n",
        "package P {\n    satisfy requirement /* why */ x by p;\n}\n",
    ] {
        let result = parse_for_editor(source);
        assert!(
            result.errors.is_empty(),
            "a comment between keywords must be trivia, got {:?} for:\n{source}",
            result.errors
        );
    }
    // The crate-wide boundary rule, stated as the shared fact it is.
    for (satisfy_form, baseline) in [
        (
            "package P {\n    satisfy/* c */ r by p;\n}\n",
            "package P {\n    part/* c */ a;\n}\n",
        ),
        (
            "package P {\n    assert/* c */ satisfy r by p;\n}\n",
            "package P {\n    import/* c */ A::*;\n}\n",
        ),
    ] {
        assert!(
            !parse_for_editor(satisfy_form).errors.is_empty()
                && !parse_for_editor(baseline).errors.is_empty(),
            "a keyword abutting a comment is rejected crate-wide; this asserts satisfy is not \
             special. If the boundary predicate is widened, both of these change together."
        );
    }
}

/// A wire document may not claim the `assert` keyword over text that is not `assert`.
#[cfg(feature = "serde")]
#[test]
fn an_assert_span_covering_other_text_is_rejected() {
    let document = parse_for_editor("package P {\n    assert satisfy r by p;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let usage = "/root/elements/0/value/Package/value/body/Brace/elements/0/value/Satisfy/value";
    *tampered
        .pointer_mut(&format!("{usage}/assert_span/offset"))
        .expect("the usage's assert keyword offset") = serde_json::json!(0);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("an assert span that does not cover `assert` must be rejected");
    assert!(
        error.to_string().contains("satisfy assert keyword"),
        "expected the assert check to name itself, got: {error}"
    );
}

/// The same for `not`, whose presence is the whole of `isNegated`.
#[cfg(feature = "serde")]
#[test]
fn a_negation_span_covering_other_text_is_rejected() {
    let document = parse_for_editor("package P {\n    not satisfy r by p;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let usage = "/root/elements/0/value/Package/value/body/Brace/elements/0/value/Satisfy/value";
    *tampered
        .pointer_mut(&format!("{usage}/not_span/len"))
        .expect("the usage's negation keyword length") = serde_json::json!(2);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a negation span that does not cover `not` must be rejected");
    assert!(
        error.to_string().contains("satisfy negation keyword"),
        "expected the negation check to name itself, got: {error}"
    );
}

/// A `by` clause is only a `by` clause if its keyword span says so.
#[cfg(feature = "serde")]
#[test]
fn a_by_span_covering_other_text_is_rejected() {
    let document = parse_for_editor("package P {\n    satisfy r by p;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let subject = "/root/elements/0/value/Package/value/body/Brace/elements/0\
                   /value/Satisfy/value/subject/value/by_span";
    *tampered
        .pointer_mut(&format!("{subject}/offset"))
        .expect("the subject's by keyword offset") = serde_json::json!(4);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a by span that does not cover `by` must be rejected");
    assert!(
        error.to_string().contains("satisfy by keyword"),
        "expected the by check to name itself, got: {error}"
    );
}

/// The inline-declaration alternative is selected by its `requirement` keyword; a wire document
/// that claims the alternative without the keyword behind it is not a document this parser
/// produced, and emitting it would write `requirement` over source that never said it.
#[cfg(feature = "serde")]
#[test]
fn an_inline_declaration_keyword_span_covering_other_text_is_rejected() {
    let document =
        parse_for_editor("package P {\n    satisfy requirement r : R by p;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let declaration = "/root/elements/0/value/Package/value/body/Brace/elements/0\
                       /value/Satisfy/value/requirement/Declaration/value/keyword_span";
    *tampered
        .pointer_mut(&format!("{declaration}/len"))
        .expect("the declaration's keyword length") = serde_json::json!(7);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a declaration keyword span that does not cover `requirement` is rejected");
    assert!(
        error.to_string().contains("satisfy requirement keyword"),
        "expected the declaration check to name itself, got: {error}"
    );
}

/// `'assert' ( isNegated ?= 'not' )? 'satisfy'` fixes the prefix order.
#[cfg(feature = "serde")]
#[test]
fn prefix_keywords_written_out_of_order_are_rejected() {
    let document = parse_for_editor("package P {\n    assert not satisfy r by p;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let usage = "/root/elements/0/value/Package/value/body/Brace/elements/0/value/Satisfy/value";
    let assert_span = tampered
        .pointer(&format!("{usage}/assert_span"))
        .cloned()
        .expect("the assert keyword span");
    let not_span = tampered
        .pointer(&format!("{usage}/not_span"))
        .cloned()
        .expect("the negation keyword span");
    *tampered
        .pointer_mut(&format!("{usage}/assert_span"))
        .expect("the assert keyword span") = not_span;
    *tampered
        .pointer_mut(&format!("{usage}/not_span"))
        .expect("the negation keyword span") = assert_span;

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("prefixes claiming each other's tokens must be rejected");
    assert!(
        error.to_string().contains("satisfy"),
        "expected a satisfy-prefix failure, got: {error}"
    );
}

/// A dangling satisfied-requirement identity is rejected rather than silently emitted.
#[cfg(feature = "serde")]
#[test]
fn a_dangling_satisfied_requirement_reference_is_rejected() {
    let document = parse_for_editor("package P {\n    satisfy r by p;\n}\n").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let reference = "/root/elements/0/value/Package/value/body/Brace/elements/0\
                     /value/Satisfy/value/requirement/Reference/reference";
    *tampered
        .pointer_mut(reference)
        .expect("the satisfied requirement reference") = serde_json::json!(9999);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("a reference with no arena entry must be rejected");
    assert!(
        error.to_string().contains("DanglingReference"),
        "expected a dangling-reference failure, got: {error}"
    );
}

/// A satisfy usage round-trips through the whole serialized envelope unchanged.
#[cfg(feature = "serde")]
#[test]
fn a_satisfy_document_round_trips_through_the_envelope() {
    let source = concat!(
        "package P {\n",
        "    assert not satisfy Requirements::spec[1] :> base by vehicle.engine {\n",
        "        require constraint c;\n",
        "        satisfy requirement <'1.1'> inner : R by q;\n",
        "    }\n",
        "}\n",
    );
    let document = parse(source).expect("parse");
    let encoded = serde_json::to_value(&document).expect("serialize");
    let decoded: sysml_v2_parser::ast::ParsedDocument =
        serde_json::from_value(encoded).expect("deserialize");
    assert_eq!(decoded, document);
}
