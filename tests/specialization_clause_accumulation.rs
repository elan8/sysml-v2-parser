//! The span of a merged subsetting-family relationship covers every fragment the author wrote.
//!
//! The rest of the accumulation behaviour -- which targets survive, in what order, which clause
//! kinds stay separate -- lives in `tests/snapshots/sysml/specialization_clause_accumulation.md`,
//! where the AST section names the relationship and its target list. This file keeps only the one
//! fact that projection cannot show: `write_subsetting` records a relationship's kind, `implied`
//! flag and targets, but not its span, so a merge that pointed only at the first clause would look
//! identical in a snapshot.

use sysml_v2_parser::ast::{Node, ParsedDocument, Span, SubsettingKind, SubsettingRelationship};
use sysml_v2_parser::parse_for_editor;

/// The first `subsets` relationship reachable from the document, found through the shared
/// traversal rather than by walking the body enums by hand.
fn first_subsets_span(document: &ParsedDocument) -> Option<Span> {
    use sysml_v2_parser::ast::visit::Visitor;

    #[derive(Default)]
    struct FindSubsets {
        span: Option<Span>,
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

/// The authored text a relationship's span covers, for a single-member part definition body.
#[track_caller]
fn subsets_span_text(source: &str) -> String {
    let parsed = parse_for_editor(source);
    assert!(
        parsed.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        parsed.errors
    );
    let span = first_subsets_span(&parsed.document).expect("a subsets relationship");
    source[span.offset..span.offset + span.len].to_string()
}

/// Two clauses written apart in the source are one relationship; its span must cover both, so a
/// caller highlighting the relationship highlights all of it rather than the first clause alone.
#[test]
fn a_merged_relationship_spans_both_authored_fragments() {
    assert_eq!(
        subsets_span_text(
            "package P {\n    part def D {\n        part p : T subsets base :> latest;\n    }\n}\n"
        ),
        "subsets base :> latest"
    );
}

/// A single clause still spans exactly itself -- merging widens the span, it does not stretch
/// every relationship to the end of its declaration.
#[test]
fn an_unmerged_relationship_spans_only_its_own_clause() {
    assert_eq!(
        subsets_span_text(
            "package P {\n    part def D {\n        part p : T subsets base;\n    }\n}\n"
        ),
        "subsets base"
    );
}
