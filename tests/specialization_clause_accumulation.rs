//! A subsetting-family clause written more than once in one header keeps every target.
//!
//! `SubsettingRelationship::target` is a list because one clause may name several comma-separated
//! targets. Writing the clause kind twice means the same thing, so the parser used to overwrite
//! the earlier clause -- dropping targets with no diagnostic and no way for a caller to notice.
//! The `sysml.library` shape that motivated the fix is
//! `derived ref item 'action' : ActionUsage[0..*] ordered subsets step, usage subsets
//! Metadata::metadataItems;` (`sysml.library/Systems Library/SysML.sysml:20`), which emitted as a
//! single-target `:> Metadata::metadataItems`.

use sysml_v2_parser::{emit_sysml, parse_for_editor};

/// The member as it is emitted back out, with the diagnostics that parsing it produced.
fn round_trip(member: &str) -> (Vec<String>, String) {
    let source = format!("package P {{\n    part def D {{\n        {member}\n    }}\n}}\n");
    let parsed = parse_for_editor(&source);
    let codes = parsed
        .errors
        .iter()
        .map(|error| error.code.clone().unwrap_or_default())
        .collect();
    let emitted = emit_sysml(&parsed.document).expect("emit");
    let member = emitted
        .lines()
        .nth(2)
        .expect("the member line")
        .trim()
        .to_string();
    (codes, member)
}

#[test]
fn a_repeated_subsets_clause_keeps_every_target_in_source_order() {
    let (codes, emitted) = round_trip("attribute x : T[0..*] ordered subsets a, b subsets c;");
    assert!(codes.is_empty(), "unexpected diagnostics: {codes:?}");
    assert_eq!(emitted, "attribute x : T[0..*] ordered :> a, b, c;");
}

#[test]
fn a_repeated_redefines_clause_keeps_every_target() {
    let (codes, emitted) = round_trip("attribute x : T redefines a redefines b;");
    assert!(codes.is_empty(), "unexpected diagnostics: {codes:?}");
    assert_eq!(emitted, "attribute x : T :>> a, b;");
}

/// The keyword and operator spellings are the same relationship, so mixing them merges too.
#[test]
fn the_keyword_and_operator_spellings_merge_into_one_relationship() {
    let (codes, emitted) = round_trip("part p : T subsets base :> latest;");
    assert!(codes.is_empty(), "unexpected diagnostics: {codes:?}");
    assert_eq!(emitted, "part p : T :> base, latest;");
}

/// Clauses on either side of the typing are one relationship as well: the header parses the two
/// positions as separate clause groups, and the groups merge on the same grounds as a repeat
/// within one group.
#[test]
fn clauses_before_and_after_the_typing_merge() {
    let (codes, emitted) = round_trip("item x :> base : T :> latest;");
    assert!(codes.is_empty(), "unexpected diagnostics: {codes:?}");
    assert_eq!(emitted, "item x : T :> base, latest;");
}

/// Different clause kinds stay separate relationships -- merging is per kind, not across kinds.
#[test]
fn different_clause_kinds_do_not_merge_with_each_other() {
    let (codes, emitted) = round_trip("part p : T subsets a redefines b;");
    assert!(codes.is_empty(), "unexpected diagnostics: {codes:?}");
    assert_eq!(emitted, "part p : T :> a :>> b;");
}

/// The merged relationship's span must cover every fragment the author wrote for it, so a caller
/// highlighting the relationship highlights all of it rather than the first clause alone.
#[test]
fn the_merged_span_covers_both_authored_fragments() {
    let source =
        "package P {\n    part def D {\n        part p : T subsets base :> latest;\n    }\n}\n";
    let parsed = parse_for_editor(source);
    assert!(
        parsed.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        parsed.errors
    );

    let span = subsets_span(&parsed.document).expect("the part usage's subsets relationship");
    let covered = &source[span.offset..span.offset + span.len];
    assert_eq!(covered, "subsets base :> latest");
}

/// Reach the one `part p` usage's `subsets` relationship span without depending on the shape of
/// every intervening body enum.
fn subsets_span(
    document: &sysml_v2_parser::ast::ParsedDocument,
) -> Option<sysml_v2_parser::ast::Span> {
    use sysml_v2_parser::ast::visit::Visitor;
    use sysml_v2_parser::ast::{Node, SubsettingKind, SubsettingRelationship};

    #[derive(Default)]
    struct FindSubsets {
        span: Option<sysml_v2_parser::ast::Span>,
    }

    impl Visitor for FindSubsets {
        fn visit_subsetting_relationship(&mut self, node: &Node<SubsettingRelationship>) {
            if node.value.kind == SubsettingKind::Subsets && self.span.is_none() {
                self.span = Some(node.value.span.clone());
            }
        }
    }

    let mut finder = FindSubsets::default();
    finder.visit_root_namespace(&document.root);
    finder.span
}
