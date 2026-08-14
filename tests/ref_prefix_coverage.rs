//! BNF `RefPrefix` is accepted wherever the grammar allows it, and survives emission.
//!
//! `RefPrefix = 'derived'? ('abstract' | 'variation')? 'constant'?` (§8.2.2.6.2) may precede any
//! usage keyword. Each parser used to hand-roll whichever part of the chain it happened to need,
//! so a legal prefix was a parse gap in whichever scopes had not adopted it -- `derived ref item
//! receiverArgument : Expression[0..1] subsets Metadata::metadataItems;` (`sysml.library/Systems
//! Library/SysML.sysml:14`) and 190 siblings all fell through to unsupported-grammar capture.
//!
//! Emission is asserted alongside parsing because a prefix the AST cannot hold is indistinguishable
//! from one it silently drops: `abstract ref :>> trailerHitch[1];` parsed cleanly for a long time
//! and formatted as `ref :>> trailerHitch;`.

use sysml_v2_parser::{emit_sysml, parse_for_editor};

/// Parse `member` inside `scope`, and return its diagnostics with the emitted member text.
#[track_caller]
fn member_round_trip(scope: &str, member: &str) -> (Vec<String>, String) {
    let source = format!("package P {{\n    {scope} {{\n        {member}\n    }}\n}}\n");
    let parsed = parse_for_editor(&source);
    let codes = parsed
        .errors
        .iter()
        .map(|error| error.code.clone().unwrap_or_default())
        .collect();
    let emitted = emit_sysml(&parsed.document)
        .unwrap_or_else(|error| panic!("emit {member:?} in {scope:?}: {error:?}"));
    let member = emitted
        .lines()
        .nth(2)
        .expect("the member line")
        .trim()
        .to_string();
    (codes, member)
}

/// The member parses without diagnostics and comes back out exactly as written.
#[track_caller]
fn assert_round_trips(scope: &str, member: &str) {
    let (codes, emitted) = member_round_trip(scope, member);
    assert!(
        codes.is_empty(),
        "{member:?} in {scope:?} produced {codes:?}"
    );
    assert_eq!(emitted, member, "in {scope:?}");
}

#[test]
fn a_derived_ref_item_is_a_structured_metadata_member() {
    assert_round_trips(
        "metadata def M",
        "derived ref item receiverArgument : Expression[0..1] :> Metadata::metadataItems;",
    );
}

/// The exact `sysml.library` shape that dominated the L2 backlog: a `derived ref` prefix, a
/// quoted name, `ordered`, and two `subsets` clauses that together name three targets.
#[test]
fn the_library_metadata_member_shape_round_trips() {
    assert_round_trips(
        "metadata def ActionDefinition",
        "derived ref item 'action' : ActionUsage[0..*] ordered :> step, usage, Metadata::metadataItems;",
    );
}

#[test]
fn a_derived_item_usage_without_ref_is_accepted() {
    assert_round_trips(
        "metadata def M",
        "derived item ownedActorParameter :>> ownedMemberParameter : PartUsage[1];",
    );
}

/// `RefDecl` had no field for the prefix at all, so this parsed and then lost the keyword.
#[test]
fn an_abstract_prefix_on_an_anonymous_ref_survives_emission() {
    assert_round_trips("action def A", "abstract ref :>> trailerHitch;");
}

#[test]
fn a_constant_prefix_is_accepted_on_an_item_usage() {
    assert_round_trips("part def D", "constant item x : T;");
}

/// `is_abstract` could not represent the `variation` alternative; `usage_prefix` can.
#[test]
fn a_variation_item_usage_keeps_its_keyword() {
    assert_round_trips("part def D", "variation item x : T;");
}

/// The chain has one legal order, and every emitter must use it.
#[test]
fn the_prefix_keywords_are_emitted_in_grammar_order() {
    assert_round_trips(
        "metadata def M",
        "derived abstract constant ref item x : T;",
    );
    assert_round_trips(
        "part def D",
        "derived abstract constant ref attribute x : T;",
    );
    assert_round_trips("part def D", "derived abstract constant ref part x : T;");
}
