//! Collect `ParseError` diagnostics from recovery nodes embedded in the AST.
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
use crate::ast::{RootNamespace, Span, TextualRepresentation};
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
    });
    errors
}
