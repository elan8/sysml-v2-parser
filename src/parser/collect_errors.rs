//! Collect `ParseError` diagnostics from recovery and fallback nodes embedded in the AST.
//!
//! Recovery diagnostics are a property of the node kind, not of the scope that happens to own
//! it: a malformed member reports the same way inside a package body, a part definition body, or
//! an action body. This consumer therefore states each rule once against the owning traversal
//! boundary ([`crate::ast::visit`]) instead of re-deriving the tree shape, so a new body scope
//! or a new place an existing member can appear reports diagnostics without further edits here.
//!
//! Diagnostics come out in traversal order -- depth first, source order within a body -- which is
//! the order [`crate::parser::parse`] expects before it deduplicates and suppresses cascades.

use super::recovery::parse_error_from_recovery_node;
use crate::ast::visit::{
    walk_classifier_decl, walk_extended_library_decl, walk_feature_decl, walk_kerml_feature_decl,
    walk_kerml_semantic_decl, walk_parse_error_node, walk_textual_representation,
    walk_unsupported_grammar_node, Visitor,
};
use crate::ast::{
    ClassifierDecl, ExtendedLibraryDecl, FeatureDecl, KermlFeatureDecl, KermlSemanticDecl, Node,
    ParseErrorNode, RootNamespace, Span, TextualRepresentation, UnsupportedGrammarNode,
};
use crate::error::{DiagnosticCategory, DiagnosticSeverity, ParseError};

fn textual_rep_language_diagnostic(
    source: &str,
    node_span: &Span,
    rep: &TextualRepresentation,
) -> Option<ParseError> {
    use crate::parser::diagnostic_catalog;
    let Some(language) = rep.language else {
        return Some(
            ParseError::new("rep body is missing the required 'language' keyword and string value")
                .with_location(node_span.offset, node_span.line, node_span.column)
                .with_length(node_span.len.max(1))
                .with_code(diagnostic_catalog::MISSING_REP_LANGUAGE)
                .with_severity(DiagnosticSeverity::Error)
                .with_category(DiagnosticCategory::ParseError),
        );
    };
    let authored =
        source.get(language.span().offset..language.span().offset + language.span().len)?;
    let spelling = crate::ast::decode_string_literal(authored)?;
    if spelling.trim().is_empty() {
        let ls = language.span();
        return Some(
            ParseError::new("rep language value must be a non-empty string")
                .with_location(ls.offset, ls.line, ls.column)
                .with_length(ls.len.max(1))
                .with_code(diagnostic_catalog::INVALID_REP_LANGUAGE)
                .with_severity(DiagnosticSeverity::Error)
                .with_category(DiagnosticCategory::ParseError),
        );
    }
    None
}

fn unsupported_fallback_diagnostic(span: &Span, production: &str) -> ParseError {
    ParseError::new(format!(
        "the spec-valid {production} production is retained but not structurally implemented"
    ))
    .with_location(span.offset, span.line, span.column)
    .with_length(span.len.max(1))
    .with_code("unsupported_grammar_form")
    .with_severity(DiagnosticSeverity::Warning)
    .with_category(DiagnosticCategory::UnsupportedGrammarForm)
}

struct RecoveryErrorCollector<'source> {
    /// The document text the tree's spans index; the tree does not carry it, and the document
    /// envelope that will is not built until after diagnostics are collected.
    source: &'source str,
    errors: Vec<ParseError>,
}

impl Visitor for RecoveryErrorCollector<'_> {
    /// Text the parser could not recognize, kept in place with its authored span.
    fn visit_parse_error_node(&mut self, node: &Node<ParseErrorNode>) {
        self.errors
            .push(parse_error_from_recovery_node(&node.span, &node.value));
        walk_parse_error_node(self, node);
    }

    /// A spec production that parses but has no structural representation yet; it carries the
    /// diagnostic the parser already classified.
    fn visit_unsupported_grammar_node(&mut self, node: &Node<UnsupportedGrammarNode>) {
        self.errors.push(parse_error_from_recovery_node(
            &node.span,
            &node.value.diagnostic,
        ));
        walk_unsupported_grammar_node(self, node);
    }

    /// A `rep` body must name its language; both the missing and the empty spelling are errors.
    fn visit_textual_representation(&mut self, node: &Node<TextualRepresentation>) {
        if let Some(diagnostic) =
            textual_rep_language_diagnostic(self.source, &node.span, &node.value)
        {
            self.errors.push(diagnostic);
        }
        walk_textual_representation(self, node);
    }

    /// An extended-library fallback keeps the declaration the grammar could not model. Some of
    /// those declarations are recognizable dialect forms with specific guidance to offer, so
    /// classify the node's own text here: the alternative -- searching the whole document for the
    /// dialect's spelling -- cannot tell a declaration from the same words inside a comment or a
    /// string literal.
    fn visit_extended_library_decl(&mut self, node: &Node<ExtendedLibraryDecl>) {
        if let Some((code, message, expected, suggestion)) =
            crate::parser::diagnostics::invalid_requirement_short_name_syntax_diagnostic(
                node.value.text.as_bytes(),
            )
        {
            self.errors.push(
                ParseError::new(message)
                    .with_location(node.span.offset, node.span.line, node.span.column)
                    .with_length(node.span.len)
                    .with_code(code)
                    .with_expected(expected)
                    .with_suggestion(suggestion)
                    .with_category(DiagnosticCategory::ParseError),
            );
        } else {
            self.errors.push(unsupported_fallback_diagnostic(
                &node.span,
                "extended-library declaration",
            ));
        }
        walk_extended_library_decl(self, node);
    }

    fn visit_feature_decl(&mut self, node: &Node<FeatureDecl>) {
        self.errors.push(unsupported_fallback_diagnostic(
            &node.span,
            "KerML feature declaration",
        ));
        walk_feature_decl(self, node);
    }

    fn visit_classifier_decl(&mut self, node: &Node<ClassifierDecl>) {
        self.errors.push(unsupported_fallback_diagnostic(
            &node.span,
            "KerML classifier declaration",
        ));
        walk_classifier_decl(self, node);
    }

    fn visit_kerml_semantic_decl(&mut self, node: &Node<KermlSemanticDecl>) {
        self.errors.push(unsupported_fallback_diagnostic(
            &node.span,
            "KerML semantic declaration",
        ));
        walk_kerml_semantic_decl(self, node);
    }

    fn visit_kerml_feature_decl(&mut self, node: &Node<KermlFeatureDecl>) {
        self.errors.push(unsupported_fallback_diagnostic(
            &node.span,
            "KerML feature form",
        ));
        walk_kerml_feature_decl(self, node);
    }
}

pub(crate) fn collect_recovery_errors(source: &str, root: &RootNamespace) -> Vec<ParseError> {
    let mut collector = RecoveryErrorCollector {
        source,
        errors: Vec::new(),
    };
    collector.visit_root_namespace(root);
    collector.errors
}
