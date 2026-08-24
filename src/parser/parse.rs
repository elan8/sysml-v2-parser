//! Public parse entry points.

use super::collect_errors::collect_recovery_errors;
use super::delimiters::DelimiterScan;
use super::diagnostics::{
    dedup_errors, extra_closing_brace_at_eof, fragment_to_found_snippet,
    missing_closing_brace_error, missing_closing_brace_error_at_eof, nom_err_to_parse_error,
    root_body_recovery_error, root_body_scope, suppress_diagnostic_cascades,
    suppress_redundant_closing_brace_errors, trim_ascii_start,
    unexpected_closing_brace_parse_error,
};
use super::lex;
use super::package;
use super::span::{node_from_to, ParseContext};
use crate::ast::{
    PackageBodyElement, ParseErrorNode, ParsedDocument, RootElement, RootNamespace, SourceStorage,
};
use crate::error::{DiagnosticCategory, DiagnosticSeverity, ParseError};

/// Maximum structural brace nesting accepted by the recursive-descent parser.
///
/// Flat model size does not consume recursive stack, but each nested package/body does. Rejecting
/// pathological nesting before descent keeps parser stack usage bounded on every host.
pub const MAX_SYNTAX_NESTING: usize = 32;

/// Build the `nesting_too_deep` diagnostic for the offending `{` recorded by the prescan.
fn nesting_limit_error(input: &[u8], delimiters: &DelimiterScan) -> Option<ParseError> {
    let offset = delimiters.nesting_overflow()?;
    let prefix = &input[..offset];
    let line = 1 + prefix.iter().filter(|&&b| b == b'\n').count();
    let column = prefix
        .iter()
        .rposition(|&b| b == b'\n')
        .map_or(offset + 1, |newline| offset - newline);
    Some(
        ParseError::new(format!(
            "model nesting exceeds the supported limit of {MAX_SYNTAX_NESTING}"
        ))
        .with_location(offset, line as u32, column)
        .with_length(1)
        .with_code("nesting_too_deep")
        .with_category(DiagnosticCategory::ParseError)
        .with_suggestion(
            "Split deeply nested declarations into packages or definitions referenced by name.",
        ),
    )
}

/// Result of parsing with error recovery: a (possibly partial) AST and zero or more diagnostics.
#[derive(Debug, Clone)]
pub struct ParseResult {
    /// Source, reference arena, and root AST. The AST is partial when errors occurred.
    pub document: ParsedDocument,
    /// All parse errors encountered (multiple when recovery is used).
    pub errors: Vec<ParseError>,
}

impl ParseResult {
    /// True if the document parsed fully with no errors.
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}
/// Parse full input; must consume entire input. Strips UTF-8 BOM if present.
///
/// Rejects input that structurally parses but contains a body member the grammar could not
/// match (surfaced as an embedded recovery diagnostic, the same ones [`parse_with_diagnostics`]
/// reports) so both entry points agree on whether a document is valid.
#[allow(clippy::result_large_err)]
pub fn parse_root(input: &str) -> Result<ParsedDocument, ParseError> {
    parse_root_inner(input)
}

/// Parse an owned source buffer without cloning the complete document.
#[allow(clippy::result_large_err)]
pub fn parse_root_owned(input: String) -> Result<ParsedDocument, ParseError> {
    parse_root_source(SourceStorage::new(input))
}

#[allow(clippy::result_large_err)]
fn parse_root_inner(input: &str) -> Result<ParsedDocument, ParseError> {
    parse_root_source(SourceStorage::new(input.to_owned()))
}

#[allow(clippy::result_large_err)]
fn parse_root_source(source: SourceStorage) -> Result<ParsedDocument, ParseError> {
    let context = ParseContext::new();
    let root = parse_root_namespace(source.as_str(), &context)?;
    Ok(ParsedDocument {
        source,
        qualified_references: context.finish(),
        root,
    })
}

#[allow(clippy::result_large_err)]
fn parse_root_namespace(source: &str, context: &ParseContext) -> Result<RootNamespace, ParseError> {
    let bytes = source.as_bytes();
    let located = context.input(bytes);
    let delimiters = DelimiterScan::new(bytes);
    if let Some(error) = nesting_limit_error(bytes, &delimiters) {
        return Err(error);
    }
    match package::root_namespace(located) {
        Ok((rest, root)) => {
            if !rest.fragment().is_empty() && delimiters.has_unclosed_brace() {
                return Err(missing_closing_brace_error_at_eof(bytes));
            }
            if rest.fragment().is_empty() {
                if let Some(error) = collect_recovery_errors(&root).into_iter().next() {
                    log::debug!(
                        "parse_root: rejecting document with embedded recovery diagnostic: {:?}",
                        error.code
                    );
                    return Err(error);
                }
                log::debug!("parse_root: success, {} top-level elements", root.elements.len());
                Ok(root)
            } else if trim_ascii_start(rest.fragment()).starts_with(b"}") {
                Err(unexpected_closing_brace_parse_error(rest))
            } else {
                let offset = located.location_offset() + located.fragment().len() - rest.fragment().len();
                let unconsumed = rest.fragment();
                let first_80 = unconsumed.get(..80.min(unconsumed.len())).unwrap_or(unconsumed);
                log::debug!(
                    "parse_root: expected end of input; parsed {} elements; unconsumed len={}, offset={}, first 80 bytes: {:?}",
                    root.elements.len(),
                    unconsumed.len(),
                    offset,
                    first_80,
                );
                log::debug!(
                    "parse_root: unconsumed as str: {:?}",
                    String::from_utf8_lossy(first_80),
                );
                let (found_snippet, found_len) = fragment_to_found_snippet(rest.fragment());
                let mut pe = ParseError::new("expected end of input")
                    .with_location(offset, rest.location_line(), crate::parser::span::column_of(&rest))
                    .with_length(found_len.max(1))
                    .with_code("expected_end_of_input")
                    .with_category(DiagnosticCategory::ParseError);
                if !found_snippet.is_empty() {
                    pe = pe.with_found(found_snippet);
                }
                Err(pe)
            }
        }
        Err(nom::Err::Error(e)) => Err(missing_closing_brace_error(bytes, e.input, &delimiters).unwrap_or_else(|| {
            nom_err_to_parse_error(
                &e,
                None,
                Some("package-body element at root (package, namespace, import, definition, or usage)"),
            )
        })),
        Err(nom::Err::Failure(e)) => Err(missing_closing_brace_error(bytes, e.input, &delimiters).unwrap_or_else(|| {
            nom_err_to_parse_error(
                &e,
                None,
                Some("package-body element at root (package, namespace, import, definition, or usage)"),
            )
        })),
        Err(nom::Err::Incomplete(_)) => Err(
            ParseError::new("unexpected end of input")
                .with_code("unexpected_eof")
                .with_category(DiagnosticCategory::ParseError),
        ),
    }
}

const MAX_RECOVERY_ERRORS: usize = 100;

/// Parse input with error recovery: collects multiple diagnostics and returns a partial AST when errors occur.
/// Use this for language servers so the user sees all parse errors and features (e.g. hover) can use the partial AST.
pub fn parse_with_diagnostics(input: &str) -> ParseResult {
    parse_with_diagnostics_inner(input)
}

/// Parse an owned source buffer for editor use without cloning the complete document.
pub fn parse_with_diagnostics_owned(input: String) -> ParseResult {
    parse_with_diagnostics_source(SourceStorage::new(input))
}

fn parse_with_diagnostics_inner(input: &str) -> ParseResult {
    parse_with_diagnostics_source(SourceStorage::new(input.to_owned()))
}

fn parse_with_diagnostics_source(source: SourceStorage) -> ParseResult {
    let context = ParseContext::new();
    let (root, errors) = parse_with_diagnostics_document(source.as_str(), &context);
    ParseResult {
        document: ParsedDocument {
            source,
            qualified_references: context.finish(),
            root,
        },
        errors,
    }
}

fn parse_with_diagnostics_document(
    source: &str,
    context: &ParseContext,
) -> (RootNamespace, Vec<ParseError>) {
    let bytes = source.as_bytes();
    let delimiters = DelimiterScan::new(bytes);
    if let Some(error) = nesting_limit_error(bytes, &delimiters) {
        return (RootNamespace { elements: vec![] }, vec![error]);
    }
    let located = context.input(bytes);

    let mut elements = Vec::new();
    let mut errors = Vec::new();

    // `ws_and_notes`, matching the strict entry point: a leading `/* ... */` is the document's
    // first member, not trivia ahead of it.
    let (mut input, _) = match lex::ws_and_notes(located) {
        Ok(x) => x,
        Err(_) => {
            return (
                RootNamespace { elements: vec![] },
                vec![ParseError::new("invalid input")
                    .with_code("invalid_input")
                    .with_category(DiagnosticCategory::ParseError)],
            );
        }
    };

    while errors.len() < MAX_RECOVERY_ERRORS {
        // Skip leading whitespace and notes; if nothing left, we're done (avoids parsing "" as
        // root_element). A bare `/* ... */` is left for `root_element`, because at a member
        // boundary it is the `Comment` production rather than trivia.
        let (rest, _) = lex::ws_and_notes(input).unwrap_or((input, ()));
        input = rest;
        if input.fragment().is_empty() {
            break;
        }
        match package::root_element(input) {
            Ok((rest, elem)) => {
                elements.push(elem);
                input = rest;
            }
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                let (trimmed, _) = lex::ws_and_comments(input).unwrap_or((input, ()));
                if trim_ascii_start(trimmed.fragment()).starts_with(b"}") {
                    errors.push(unexpected_closing_brace_parse_error(trimmed));
                    let skip_result = lex::skip_to_next_sync_point(trimmed);
                    match skip_result {
                        Ok((rest, _)) => input = rest,
                        Err(_) => break,
                    }
                    continue;
                }
                if errors.is_empty()
                    && delimiters.has_unclosed_brace()
                    && (lex::starts_with_keyword(trimmed.fragment(), b"package")
                        || lex::starts_with_keyword(trimmed.fragment(), b"namespace")
                        || lex::starts_with_keyword(trimmed.fragment(), b"library")
                        || lex::starts_with_keyword(trimmed.fragment(), b"standard"))
                {
                    let pe = missing_closing_brace_error_at_eof(bytes);
                    errors.push(pe.clone());
                    // Keep the entire unterminated root declaration as one explicit recovery
                    // node. Returning an empty root here loses the user's source and prevents the
                    // editor emitter from reproducing it while the closing brace is being typed.
                    if let Ok((rest, _)) = lex::skip_to_next_balanced_sync_point(input) {
                        if rest.location_offset() > input.location_offset() {
                            let recovery = ParseErrorNode {
                                message: pe.message,
                                code: pe
                                    .code
                                    .unwrap_or_else(|| "missing_closing_brace".to_owned()),
                                expected: pe.expected,
                                found: pe.found,
                                suggestion: pe.suggestion,
                                category: pe.category,
                            };
                            let error =
                                PackageBodyElement::Error(node_from_to(input, rest, recovery));
                            let member = node_from_to(input, rest, error);
                            elements.push(node_from_to(
                                input,
                                rest,
                                RootElement::Member(Box::new(member)),
                            ));
                            input = rest;
                        }
                    }
                    break;
                }
                if let Some(scope) = root_body_scope(input.fragment()) {
                    let (error_input, _) = lex::ws_and_comments(e.input).unwrap_or((e.input, ()));
                    if error_input.fragment().starts_with(b"{") {
                        errors.push(root_body_recovery_error(error_input, scope));
                        match lex::skip_statement_or_block(error_input) {
                            Ok((rest, _))
                                if rest.location_offset() > error_input.location_offset() =>
                            {
                                input = rest;
                                continue;
                            }
                            _ => {}
                        }
                    }
                }
                let pe = missing_closing_brace_error(bytes, e.input, &delimiters).unwrap_or_else(|| {
                    nom_err_to_parse_error(
                        &e,
                        None,
                        Some("package-body element at root (package, namespace, import, definition, or usage)"),
                    )
                });
                let recovery_start = input;
                errors.push(pe.clone());
                let skip_result = lex::skip_to_next_balanced_sync_point(recovery_start);
                match skip_result {
                    Ok((rest, _)) if rest.location_offset() > recovery_start.location_offset() => {
                        let recovery = ParseErrorNode {
                            message: pe.message,
                            code: pe
                                .code
                                .unwrap_or_else(|| "recovered_root_element".to_owned()),
                            expected: pe.expected,
                            found: pe.found,
                            suggestion: pe.suggestion,
                            category: pe.category,
                        };
                        let error =
                            PackageBodyElement::Error(node_from_to(recovery_start, rest, recovery));
                        let member = node_from_to(recovery_start, rest, error);
                        elements.push(node_from_to(
                            recovery_start,
                            rest,
                            RootElement::Member(Box::new(member)),
                        ));
                        input = rest;
                    }
                    Err(_) => break,
                    _ => break,
                }
            }
            Err(nom::Err::Incomplete(_)) => {
                errors.push(
                    ParseError::new("unexpected end of input")
                        .with_location(
                            input.location_offset(),
                            input.location_line(),
                            crate::parser::span::column_of(&input),
                        )
                        .with_length(1)
                        .with_code("unexpected_eof")
                        .with_category(DiagnosticCategory::ParseError),
                );
                break;
            }
        }
    }

    let (input, _) = lex::ws_and_comments(input).unwrap_or((input, ()));

    if input.fragment().is_empty()
        && !errors.iter().any(|e| {
            matches!(
                e.code.as_deref(),
                Some("missing_closing_brace") | Some("unexpected_closing_brace")
            )
        })
    {
        if let Some(err) = extra_closing_brace_at_eof(bytes, &delimiters) {
            errors.push(err);
        } else if delimiters.has_unclosed_brace() {
            errors.push(missing_closing_brace_error_at_eof(bytes));
        }
    }

    if !input.fragment().is_empty()
        && !errors
            .iter()
            .any(|e| e.code.as_deref() == Some("missing_closing_brace"))
    {
        if trim_ascii_start(input.fragment()).starts_with(b"}") {
            errors.push(unexpected_closing_brace_parse_error(input));
        } else {
            let (found_snippet, found_len) = fragment_to_found_snippet(input.fragment());
            let mut pe = ParseError::new("expected end of input")
                .with_location(
                    input.location_offset(),
                    input.location_line(),
                    crate::parser::span::column_of(&input),
                )
                .with_length(found_len.max(1))
                .with_code("expected_end_of_input")
                .with_severity(DiagnosticSeverity::Error)
                .with_category(DiagnosticCategory::ParseError);
            if !found_snippet.is_empty() {
                pe = pe.with_found(found_snippet);
            }
            errors.push(pe);
        }
    }

    let root = RootNamespace { elements };
    errors.extend(collect_recovery_errors(&root));
    errors = suppress_redundant_closing_brace_errors(errors);
    errors = dedup_errors(errors);
    errors = suppress_diagnostic_cascades(errors);

    (root, errors)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nested_packages(depth: usize) -> String {
        let mut source = String::new();
        for index in 0..depth {
            source.push_str(&format!("package P{index} {{\n"));
        }
        source.push_str("attribute value : Real;\n");
        for _ in 0..depth {
            source.push_str("}\n");
        }
        source
    }

    #[test]
    fn accepts_nesting_at_the_limit() {
        assert!(parse_root(&nested_packages(MAX_SYNTAX_NESTING)).is_ok());
    }

    #[test]
    fn flat_model_size_does_not_consume_nesting_budget() {
        let mut source = String::from("package Stress {\n");
        for index in 0..10_000 {
            source.push_str(&format!("attribute value{index} : Real;\n"));
        }
        source.push_str("}\n");

        let parsed = parse_root(&source).expect("large flat model");
        assert_eq!(parsed.root.elements.len(), 1);
    }

    #[test]
    fn rejects_nesting_beyond_the_limit_before_descent() {
        let error = parse_root(&nested_packages(MAX_SYNTAX_NESTING + 1))
            .expect_err("excessive nesting should be rejected");
        assert_eq!(error.code.as_deref(), Some("nesting_too_deep"));

        let editor = parse_with_diagnostics(&nested_packages(MAX_SYNTAX_NESTING + 1));
        assert_eq!(editor.errors.len(), 1);
        assert_eq!(editor.errors[0].code.as_deref(), Some("nesting_too_deep"));
    }

    #[test]
    fn braces_in_comments_and_quoted_text_do_not_count_as_nesting() {
        let source = format!(
            "package P {{ doc /* {} */ attribute text = \"{}\"; }}",
            "{".repeat(MAX_SYNTAX_NESTING + 1),
            "{".repeat(MAX_SYNTAX_NESTING + 1)
        );
        let bytes = source.as_bytes();
        let error = nesting_limit_error(bytes, &DelimiterScan::new(bytes));
        assert!(error.is_none(), "comments and strings are not model scopes");
    }

    /// GH-2: `parse_root` previously returned `Ok` for input the grammar could not fully match,
    /// because it never inspected the AST for the same embedded recovery diagnostics
    /// `parse_with_diagnostics` surfaces. These cases all structurally parse (balanced braces,
    /// full input consumed) while still containing a body member the grammar dropped.
    #[test]
    fn rejects_misspelled_keyword_that_recovery_would_flag() {
        let source = "package P { attribut def X { } }";
        let error = parse_root(source).expect_err("misspelled keyword should be rejected");
        assert!(error.code.is_some());

        let editor = parse_with_diagnostics(source);
        assert!(!editor.errors.is_empty());
        assert_eq!(error.code, editor.errors[0].code);
    }

    #[test]
    fn rejects_unknown_keyword_that_recovery_would_flag() {
        let source = "package P { frobnicate def Q { } }";
        let error = parse_root(source).expect_err("unknown keyword should be rejected");
        assert!(error.code.is_some());

        let editor = parse_with_diagnostics(source);
        assert!(!editor.errors.is_empty());
        assert_eq!(error.code, editor.errors[0].code);
    }

    #[test]
    fn rejects_typo_that_recovery_would_flag_and_does_not_silently_drop_it() {
        let source = "package P { parts def A { } }";
        let error = parse_root(source).expect_err("typo'd declaration should be rejected");
        assert!(error.code.is_some());
    }

    #[test]
    fn still_accepts_clean_input() {
        let source = "package P { part def A { attribute x : Real; } }";
        let document = parse_root(source).expect("well-formed input should still parse");
        assert_eq!(document.root.elements.len(), 1);
        let editor = parse_with_diagnostics(source);
        assert!(editor.is_ok());
    }

    #[test]
    fn parsed_documents_retain_the_bom_stripped_source() {
        let source = "\u{FEFF}package P;";
        let strict = parse_root(source).expect("well-formed input should parse");
        assert_eq!(strict.source.as_str(), "package P;");

        let editor = parse_with_diagnostics(source);
        assert!(editor.is_ok());
        assert_eq!(editor.document.source.as_str(), "package P;");
    }
}
