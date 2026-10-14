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
use crate::ast::visit::recovery::{scan_recovery_content, RecoveryContent};
use crate::ast::{ExtendedLibraryDecl, Node, RootNamespace, Span, TextualRepresentation};
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

pub(crate) fn collect_recovery_errors(source: &str, root: &RootNamespace) -> Vec<ParseError> {
    let mut errors = Vec::new();
    scan_recovery_content(root, |content| match content {
        RecoveryContent::ReservedDeclarationName(name) => {
            let span = name.span();
            let diagnostic =
                ParseError::new("a reserved keyword cannot be an unquoted declaration name")
                    .with_location(span.offset, span.line, span.column)
                    .with_length(span.len)
                    .with_code("reserved_declaration_name")
                    .with_severity(DiagnosticSeverity::Error)
                    .with_category(DiagnosticCategory::ParseError)
                    .with_suggestion("Quote the name using single quotes.");
            errors.push(match source.get(span.offset..span.offset + span.len) {
                Some(authored) => diagnostic.with_found(authored),
                None => diagnostic,
            });
        }
        RecoveryContent::ParseError(node) => {
            errors.push(parse_error_from_recovery_node(&node.span, &node.value));
        }
        // It carries the diagnostic the parser already classified.
        RecoveryContent::UnsupportedGrammar(node) => {
            errors.push(parse_error_from_recovery_node(
                &node.span,
                &node.value.diagnostic,
            ));
        }
        // A `rep` body must name its language; both the missing and the empty spelling are
        // errors.
        RecoveryContent::TextualRepresentation(node) => {
            errors.extend(textual_rep_language_diagnostic(
                source,
                &node.span,
                &node.value,
            ));
        }
        RecoveryContent::ExtendedLibraryDecl(node) => {
            errors.push(extended_library_decl_diagnostic(source, node));
        }
        RecoveryContent::FeatureDecl(node) => errors.push(unsupported_fallback_diagnostic(
            &node.span,
            "KerML feature declaration",
        )),
        RecoveryContent::ClassifierDecl(node) => errors.push(unsupported_fallback_diagnostic(
            &node.span,
            "KerML classifier declaration",
        )),
        RecoveryContent::KermlSemanticDecl(node) => errors.push(unsupported_fallback_diagnostic(
            &node.span,
            "KerML semantic declaration",
        )),
        RecoveryContent::KermlFeatureDecl(node) => errors.push(unsupported_fallback_diagnostic(
            &node.span,
            "KerML feature form",
        )),
    });
    errors
}

/// An extended-library fallback keeps the declaration the grammar could not model. Some of those
/// declarations are recognizable dialect forms with specific guidance to offer, so classify the
/// node's own text here: the alternative -- searching the whole document for the dialect's
/// spelling -- cannot tell a declaration from the same words inside a comment or a string literal.
fn extended_library_decl_diagnostic(source: &str, node: &Node<ExtendedLibraryDecl>) -> ParseError {
    let text = node.value.text.span();
    match source
        .get(text.offset..text.offset + text.len)
        .and_then(|text| {
            crate::parser::diagnostics::invalid_requirement_short_name_syntax_diagnostic(
                text.as_bytes(),
            )
        }) {
        Some((code, message, expected, suggestion)) => ParseError::new(message)
            .with_location(node.span.offset, node.span.line, node.span.column)
            .with_length(node.span.len)
            .with_code(code)
            .with_expected(expected)
            .with_suggestion(suggestion)
            .with_category(DiagnosticCategory::ParseError),
        None => unsupported_fallback_diagnostic(&node.span, "extended-library declaration"),
    }
}
