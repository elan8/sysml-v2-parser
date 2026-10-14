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
    walk_classifier_decl, walk_extended_library_decl, walk_feature_decl, walk_kerml_feature_decl,
    walk_kerml_semantic_decl, walk_parse_error_node, walk_textual_representation,
    walk_unsupported_grammar_node, Visitor,
};
use crate::ast::{
    ClassifierDecl, DeclarationName, DeclarationNameState, ExtendedLibraryDecl, FeatureDecl,
    KermlFeatureDecl, KermlSemanticDecl, Node, ParseErrorNode, RootNamespace,
    TextualRepresentation, UnsupportedGrammarNode,
};

/// A node [`scan_recovery_content`] reports, in traversal order.
pub(crate) enum RecoveryContent<'a> {
    /// An unquoted declaration name spelled as a reserved keyword, retained as malformed syntax.
    ReservedDeclarationName(DeclarationName),
    /// Text the parser could not recognize, kept in place with its authored span.
    ParseError(&'a Node<ParseErrorNode>),
    /// A spec production that parses but has no structural representation yet.
    UnsupportedGrammar(&'a Node<UnsupportedGrammarNode>),
    /// Fallback nodes that retain a declaration the typed grammar does not model.
    ExtendedLibraryDecl(&'a Node<ExtendedLibraryDecl>),
    FeatureDecl(&'a Node<FeatureDecl>),
    ClassifierDecl(&'a Node<ClassifierDecl>),
    KermlSemanticDecl(&'a Node<KermlSemanticDecl>),
    KermlFeatureDecl(&'a Node<KermlFeatureDecl>),
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

    fn visit_extended_library_decl(&mut self, node: &Node<ExtendedLibraryDecl>) {
        (self.0)(RecoveryContent::ExtendedLibraryDecl(node));
        walk_extended_library_decl(self, node);
    }

    fn visit_feature_decl(&mut self, node: &Node<FeatureDecl>) {
        (self.0)(RecoveryContent::FeatureDecl(node));
        walk_feature_decl(self, node);
    }

    fn visit_classifier_decl(&mut self, node: &Node<ClassifierDecl>) {
        (self.0)(RecoveryContent::ClassifierDecl(node));
        walk_classifier_decl(self, node);
    }

    fn visit_kerml_semantic_decl(&mut self, node: &Node<KermlSemanticDecl>) {
        (self.0)(RecoveryContent::KermlSemanticDecl(node));
        walk_kerml_semantic_decl(self, node);
    }

    fn visit_kerml_feature_decl(&mut self, node: &Node<KermlFeatureDecl>) {
        (self.0)(RecoveryContent::KermlFeatureDecl(node));
        walk_kerml_feature_decl(self, node);
    }
}
