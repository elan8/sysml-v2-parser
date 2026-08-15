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
