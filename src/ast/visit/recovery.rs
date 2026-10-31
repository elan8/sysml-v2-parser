//! One traversal that reports every piece of recovered or not-yet-modelled content in a tree.
//!
//! Two consumers ask "what in this tree is not successfully recognized syntax?": parsing, which
//! turns each such node into a diagnostic, and the opacity report, which refuses to emit a tree
//! containing one. They used to answer it separately -- the opacity report with its own
//! hand-written descent over every body scope -- so a scope or node kind added to one could be
//! missed by the other with no compile error. [`scan_recovery_content`] is the single answer;
//! each consumer matches [`RecoveryContent`] exhaustively, so a new kind of recovered content
//! fails to compile until both decide what it means.

use super::{
    walk_parse_error_node, walk_textual_representation, walk_unsupported_grammar_node, Visitor,
};
use crate::ast::{
    BodyClose, DeclarationName, DeclarationNameState, Node, ParseErrorNode, RootNamespace, Span,
    TextualRepresentation, UnsupportedGrammarNode,
};

/// A node [`scan_recovery_content`] reports, in traversal order.
pub(crate) enum RecoveryContent<'a> {
    /// An unquoted declaration name spelled as a reserved keyword, retained as malformed syntax.
    ReservedDeclarationName(DeclarationName),
    /// A brace body still open at end of input; the span is the zero-width end of the source.
    UnterminatedBody(Span),
    /// Text the parser could not recognize, kept in place with its authored span.
    ParseError(&'a Node<ParseErrorNode>),
    /// A spec production that parses but has no structural representation yet.
    UnsupportedGrammar(&'a Node<UnsupportedGrammarNode>),
    /// A `rep` body. It is recognized syntax, but its `language` clause may be missing or empty,
    /// which parsing diagnoses; it is reported here so that check shares this traversal.
    TextualRepresentation(&'a Node<TextualRepresentation>),
}

/// Report every [`RecoveryContent`] node under `root` to `report`, depth first and in source
/// order within each body (see the module-level traversal order of [`super`]). The children of a
/// reported node are scanned too.
pub(crate) fn scan_recovery_content(root: &RootNamespace, report: impl FnMut(RecoveryContent<'_>)) {
    Scan(report).visit_root_namespace(root);
}

struct Scan<F>(F);

impl<F: FnMut(RecoveryContent<'_>)> Visitor for Scan<F> {
    fn visit_declaration_name(&mut self, name: &DeclarationName) {
        match name.state() {
            DeclarationNameState::Parsed => {}
            DeclarationNameState::ReservedKeyword => {
                (self.0)(RecoveryContent::ReservedDeclarationName(*name))
            }
        }
    }

    fn visit_body_braces<E>(&mut self, _open: &Span, _elements: &[Node<E>], close: &BodyClose) {
        match close {
            BodyClose::Authored { .. } => {}
            BodyClose::MissingAtEndOfInput { at } => {
                (self.0)(RecoveryContent::UnterminatedBody(*at))
            }
        }
    }

    fn visit_parse_error_node(&mut self, node: &Node<ParseErrorNode>) {
        (self.0)(RecoveryContent::ParseError(node));
        walk_parse_error_node(self, node);
    }

    fn visit_unsupported_grammar_node(&mut self, node: &Node<UnsupportedGrammarNode>) {
        (self.0)(RecoveryContent::UnsupportedGrammar(node));
        walk_unsupported_grammar_node(self, node);
    }

    fn visit_textual_representation(&mut self, node: &Node<TextualRepresentation>) {
        (self.0)(RecoveryContent::TextualRepresentation(node));
        walk_textual_representation(self, node);
    }
}
