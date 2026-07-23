//! Public parse entry points.

use super::collect_errors::{
    collect_implicit_attribute_in_part_def_warnings, collect_recovery_errors,
    collect_requirement_id_dialect_diagnostics,
};
use super::diagnostics::{
    dedup_errors, extra_closing_brace_at_eof, fragment_to_found_snippet, has_unclosed_brace,
    is_illegal_top_level_definition, missing_closing_brace_error,
    missing_closing_brace_error_at_eof, nom_err_to_parse_error, root_body_recovery_error,
    root_body_scope, suppress_diagnostic_cascades, suppress_redundant_closing_brace_errors,
    trim_ascii_start, unexpected_closing_brace_parse_error,
};
use super::lex;
use super::package;
use crate::ast::RootNamespace;
use crate::error::{DiagnosticCategory, DiagnosticSeverity, ParseError};
use nom_locate::LocatedSpan;

/// Maximum structural brace nesting accepted by the recursive-descent parser.
///
/// Flat model size does not consume recursive stack, but each nested package/body does. Rejecting
/// pathological nesting before descent keeps parser stack usage bounded on every host.
pub const MAX_SYNTAX_NESTING: usize = 32;

fn nesting_limit_error(input: &[u8]) -> Option<ParseError> {
    let mut brace_depth = 0usize;
    let mut block_comment_depth = 0usize;
    let mut in_line_comment = false;
    let mut quote = None;
    let mut escaped = false;
    let mut index = 0usize;

    while index < input.len() {
        let byte = input[index];
        let next = input.get(index + 1).copied();

        if in_line_comment {
            if byte == b'\n' {
                in_line_comment = false;
            }
            index += 1;
            continue;
        }
        if block_comment_depth > 0 {
            if byte == b'/' && next == Some(b'*') {
                block_comment_depth += 1;
                index += 2;
            } else if byte == b'*' && next == Some(b'/') {
                block_comment_depth -= 1;
                index += 2;
            } else {
                index += 1;
            }
            continue;
        }
        if let Some(delimiter) = quote {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == delimiter {
                quote = None;
            }
            index += 1;
            continue;
        }

        match (byte, next) {
            (b'/', Some(b'/')) => {
                in_line_comment = true;
                index += 2;
            }
            (b'/', Some(b'*')) => {
                block_comment_depth = 1;
                index += 2;
            }
            (b'"' | b'\'', _) => {
                quote = Some(byte);
                index += 1;
            }
            (b'{', _) => {
                brace_depth += 1;
                if brace_depth > MAX_SYNTAX_NESTING {
                    let prefix = &input[..index];
                    let line = 1 + prefix.iter().filter(|&&b| b == b'\n').count();
                    let column = prefix
                        .iter()
                        .rposition(|&b| b == b'\n')
                        .map_or(index + 1, |newline| index - newline);
                    return Some(
                        ParseError::new(format!(
                            "model nesting exceeds the supported limit of {MAX_SYNTAX_NESTING}"
                        ))
                        .with_location(index, line as u32, column)
                        .with_length(1)
                        .with_code("nesting_too_deep")
                        .with_category(DiagnosticCategory::ParseError)
                        .with_suggestion(
                            "Split deeply nested declarations into packages or definitions referenced by name.",
                        ),
                    );
                }
                index += 1;
            }
            (b'}', _) => {
                brace_depth = brace_depth.saturating_sub(1);
                index += 1;
            }
            _ => index += 1,
        }
    }
    None
}
/// Result of parsing with error recovery: a (possibly partial) AST and zero or more diagnostics.
#[derive(Debug, Clone)]
pub struct ParseResult {
    /// Root namespace; contains all successfully parsed top-level elements (partial when errors occurred).
    pub root: RootNamespace,
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
#[allow(clippy::result_large_err)]
pub fn parse_root(input: &str) -> Result<RootNamespace, ParseError> {
    let bytes = input
        .strip_prefix('\u{FEFF}')
        .map(str::as_bytes)
        .unwrap_or_else(|| input.as_bytes());
    let located = LocatedSpan::new(bytes);
    if let Some(error) = nesting_limit_error(bytes) {
        return Err(error);
    }
    match package::root_namespace(located) {
        Ok((rest, root)) => {
            if !rest.fragment().is_empty() && has_unclosed_brace(bytes) {
                return Err(missing_closing_brace_error_at_eof(bytes));
            }
            if rest.fragment().is_empty() {
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
                    .with_location(offset, rest.location_line(), rest.get_column())
                    .with_length(found_len.max(1))
                    .with_code("expected_end_of_input")
                    .with_category(DiagnosticCategory::ParseError);
                if !found_snippet.is_empty() {
                    pe = pe.with_found(found_snippet);
                }
                if root.elements.is_empty() && is_illegal_top_level_definition(rest.fragment()) {
                    pe = pe
                        .with_code("illegal_top_level_definition")
                        .with_expected("'package', 'namespace', or 'import'")
                        .with_suggestion(
                            "Wrap this declaration in `package ... { ... }` or `namespace ... { ... }`.",
                        );
                    pe.message = "illegal top-level definition".to_string();
                }
                Err(pe)
            }
        }
        Err(nom::Err::Error(e)) => Err(missing_closing_brace_error(bytes, e.input).unwrap_or_else(|| {
            nom_err_to_parse_error(
                &e,
                None,
                Some("'package', 'namespace', or 'import' at top level; or valid element in package body"),
            )
        })),
        Err(nom::Err::Failure(e)) => Err(missing_closing_brace_error(bytes, e.input).unwrap_or_else(|| {
            nom_err_to_parse_error(
                &e,
                None,
                Some("'package', 'namespace', or 'import' at top level; or valid element in package body"),
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
    let bytes = input
        .strip_prefix('\u{FEFF}')
        .map(str::as_bytes)
        .unwrap_or_else(|| input.as_bytes());
    let located = LocatedSpan::new(bytes);
    if let Some(error) = nesting_limit_error(bytes) {
        return ParseResult {
            root: RootNamespace { elements: vec![] },
            errors: vec![error],
        };
    }

    let mut elements = Vec::new();
    let mut errors = Vec::new();

    let (mut input, _) = match lex::ws_and_comments(located) {
        Ok(x) => x,
        Err(_) => {
            return ParseResult {
                root: RootNamespace { elements: vec![] },
                errors: vec![ParseError::new("invalid input")
                    .with_code("invalid_input")
                    .with_category(DiagnosticCategory::ParseError)],
            };
        }
    };

    while errors.len() < MAX_RECOVERY_ERRORS {
        // Skip leading ws/comments; if nothing left, we're done (avoids parsing "" as root_element).
        let (rest, _) = lex::ws_and_comments(input).unwrap_or((input, ()));
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
                    && has_unclosed_brace(bytes)
                    && (lex::starts_with_keyword(trimmed.fragment(), b"package")
                        || lex::starts_with_keyword(trimmed.fragment(), b"namespace")
                        || lex::starts_with_keyword(trimmed.fragment(), b"library")
                        || lex::starts_with_keyword(trimmed.fragment(), b"standard"))
                {
                    errors.push(missing_closing_brace_error_at_eof(bytes));
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
                let pe = missing_closing_brace_error(bytes, e.input).unwrap_or_else(|| {
                    nom_err_to_parse_error(&e, None, Some("'package', 'namespace', or 'import'"))
                });
                errors.push(pe);
                let skip_result = lex::skip_to_next_sync_point(e.input);
                match skip_result {
                    Ok((rest, _)) => input = rest,
                    Err(_) => break,
                }
            }
            Err(nom::Err::Incomplete(_)) => {
                errors.push(
                    ParseError::new("unexpected end of input")
                        .with_location(
                            input.location_offset(),
                            input.location_line(),
                            input.get_column(),
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
        if let Some(err) = extra_closing_brace_at_eof(bytes) {
            errors.push(err);
        } else if has_unclosed_brace(bytes) {
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
                    input.get_column(),
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

    errors.extend(collect_recovery_errors(&RootNamespace {
        elements: elements.clone(),
    }));
    errors.extend(collect_implicit_attribute_in_part_def_warnings(bytes));
    errors.extend(collect_requirement_id_dialect_diagnostics(bytes));
    errors = suppress_redundant_closing_brace_errors(errors);
    errors = dedup_errors(errors);
    errors = suppress_diagnostic_cascades(errors);

    ParseResult {
        root: RootNamespace { elements },
        errors,
    }
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
        assert_eq!(parsed.elements.len(), 1);
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
        let error = nesting_limit_error(source.as_bytes());
        assert!(error.is_none(), "comments and strings are not model scopes");
    }
}
