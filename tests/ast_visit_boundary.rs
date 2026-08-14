//! The owning AST traversal boundary (`ast::visit`) and the consumers built on it.
//!
//! These cover the properties the boundary exists to guarantee: that every member -- including
//! malformed and unsupported ones -- is visited where it was authored, that the transformation
//! half can rewrite provenance without changing structure, and that the inventory keeps the
//! compile-time exhaustiveness that makes a forgotten variant impossible rather than merely
//! unlikely.

use sysml_v2_parser::ast::visit::{walk_attribute_usage, walk_part_def, Visitor};
use sysml_v2_parser::ast::{AttributeUsage, Node, ParseErrorNode, PartDef, Span};
use sysml_v2_parser::{parse, parse_for_editor};

/// Records the shape of a document as the traversal sees it: declarations and recovery members
/// in one ordered stream.
#[derive(Default)]
struct Trace {
    events: Vec<String>,
}

impl Visitor for Trace {
    fn visit_part_def(&mut self, node: &Node<PartDef>) {
        self.events.push(format!("part-def@{}", node.span.offset));
        walk_part_def(self, node);
    }

    fn visit_parse_error_node(&mut self, node: &Node<ParseErrorNode>) {
        self.events.push(format!(
            "malformed@{}+{} {}",
            node.span.offset, node.span.len, node.value.code
        ));
    }
}

/// Malformed members must be visited in their original tree position -- not hoisted to the end,
/// not skipped, and not at the cost of the valid siblings around them.
#[test]
fn malformed_members_are_visited_between_their_valid_siblings() {
    let source =
        "package P {\n  part def A;\n  ??? junk ???\n  part def B {\n    !!! more !!!\n  }\n}\n";
    let document = parse_for_editor(source);
    let mut trace = Trace::default();
    trace.visit_root_namespace(&document.document.root);

    assert_eq!(
        trace.events,
        vec![
            "part-def@14".to_owned(),
            "malformed@28+15 recovered_package_body_element".to_owned(),
            "part-def@43".to_owned(),
            "malformed@60+15 recovered_part_def_body_element".to_owned(),
        ],
        "recovery members must stay at their authored position between valid siblings"
    );

    // The recorded spans are the authored ones, so the malformed text can still be recovered from
    // the source rather than reconstructed.
    assert_eq!(&source[28..28 + 15], "??? junk ???\n  ");
    assert_eq!(&source[60..60 + 15], "!!! more !!!\n  ");
}

/// Every span the tree owns is reachable from the traversal, which is what lets one consumer
/// state the "spans are provenance, not grammar" comparison policy in one place.
#[derive(Default)]
struct SpanAudit {
    spans: Vec<Span>,
    attribute_name_span_presence: Vec<bool>,
}

impl Visitor for SpanAudit {
    fn visit_span(&mut self, span: &Span) {
        self.spans.push(span.clone());
    }

    fn visit_attribute_usage(&mut self, node: &Node<AttributeUsage>) {
        self.attribute_name_span_presence
            .push(node.value.name_span.is_some());
        walk_attribute_usage(self, node);
    }
}

fn audit(root: &sysml_v2_parser::ast::RootNamespace) -> SpanAudit {
    let mut audit = SpanAudit::default();
    audit.visit_root_namespace(root);
    audit
}

const PREFIX_ATTRIBUTE_SOURCE: &str =
    "package P {\n  part def Q {\n    attribute named : Real;\n    attribute :>> named;\n  }\n}\n";

#[test]
fn span_normalization_erases_locations_but_keeps_optional_span_presence() {
    let root = parse(PREFIX_ATTRIBUTE_SOURCE).expect("parse");
    let before = audit(&root);
    assert!(
        before.spans.iter().any(|span| span.offset != 0),
        "the fixture should carry real authored spans to begin with"
    );
    assert_eq!(
        before.attribute_name_span_presence,
        vec![true, false],
        "the named attribute records a name span; the `:>>` prefix form has no authored name"
    );

    let normalized = root.normalize_for_test_comparison();
    let after = audit(&normalized);

    assert_eq!(
        after.spans.len(),
        before.spans.len(),
        "normalization must not add or drop spans, only erase their locations"
    );
    assert!(
        after.spans.iter().all(|span| *span == Span::dummy()),
        "every reachable span must be erased, including spans recorded on declarations"
    );
    assert_eq!(
        after.attribute_name_span_presence, before.attribute_name_span_presence,
        "whether an optional span was authored at all is grammar, so presence must survive"
    );
}

#[test]
fn structural_comparison_ignores_position_but_not_content() {
    let indented = format!("\n\n   {PREFIX_ATTRIBUTE_SOURCE}");
    let same_shape = parse(&indented).expect("parse indented");
    let original = parse(PREFIX_ATTRIBUTE_SOURCE).expect("parse");
    assert_ne!(
        same_shape, original,
        "raw equality still compares authored positions"
    );
    assert_eq!(
        same_shape.normalize_for_test_comparison(),
        original.normalize_for_test_comparison(),
        "the same document authored at a different offset is structurally the same document"
    );

    let different = parse("package P {\n  part def Q {\n    attribute named : Real;\n  }\n}\n")
        .expect("parse different");
    assert_ne!(
        different.normalize_for_test_comparison(),
        original.normalize_for_test_comparison(),
        "erasing spans must not erase a missing member"
    );
}

/// The inventory's guarantee is a compile failure, and that guarantee rests on two source
/// properties: no wildcard match arm, and no struct pattern that ignores the rest of the fields.
/// Either one would let a new variant or a new child field pass through the traversal -- and
/// through every consumer built on it -- without a decision being made about it.
#[test]
fn traversal_inventory_cannot_absorb_a_new_variant_silently() {
    let inventory = include_str!("../src/ast/visit/inventory.rs");

    let wildcard_arms: Vec<_> = inventory
        .lines()
        .filter(|line| {
            let line = line.trim();
            line.starts_with("_ =>") || line.contains("(..)") || line.ends_with(", .. } =>")
        })
        .collect();
    assert!(
        wildcard_arms.is_empty(),
        "the traversal inventory must match every variant explicitly, found: {wildcard_arms:?}"
    );

    let rest_patterns: Vec<_> = inventory
        .lines()
        .filter(|line| line.contains(".. }") || line.contains(", ..)"))
        .collect();
    assert!(
        rest_patterns.is_empty(),
        "the traversal inventory must destructure every field explicitly, found: {rest_patterns:?}"
    );

    // Both traversal directions come from this one inventory, so neither can drift.
    assert_eq!(
        inventory.matches("macro_rules! ast_traversal").count(),
        1,
        "there must be exactly one traversal inventory"
    );
}
