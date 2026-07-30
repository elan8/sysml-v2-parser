//! Recovery error nodes and classification for structured body parsing.

use super::diagnostics::{
    category_from_code, invalid_bare_identifier_in_body_diagnostic,
    invalid_expose_separator_diagnostic, invalid_typing_operator_diagnostic,
    invalid_unit_reference_diagnostic, missing_expression_after_operator_diagnostic,
    missing_name_diagnostic, missing_semicolon_or_body_diagnostic, missing_type_diagnostic,
    trim_ascii_end, trim_ascii_start, unexpected_keyword_in_scope_diagnostic,
};
use super::lex;
use super::Input;
use crate::ast::ParseErrorNode;
use crate::error::{DiagnosticCategory, DiagnosticSeverity, ParseError};
pub(crate) fn recovery_found_snippet(input: Input<'_>) -> Option<String> {
    let frag = input.fragment();
    let take = frag
        .iter()
        .position(|&b| b == b'\n' || b == b'\r')
        .unwrap_or(frag.len())
        .min(60);
    let snippet = String::from_utf8_lossy(&frag[..take]).trim().to_string();
    if snippet.is_empty() {
        None
    } else {
        Some(snippet)
    }
}

pub(crate) fn recovery_found_snippet_from_span(
    input: Input<'_>,
    recovery_end: Input<'_>,
) -> Option<String> {
    let consumed_len = recovery_end
        .location_offset()
        .saturating_sub(input.location_offset())
        .min(input.fragment().len());
    if consumed_len == 0 {
        return recovery_found_snippet(input);
    }
    let frag = &input.fragment()[..consumed_len];
    let take = frag
        .iter()
        .position(|&b| b == b'\n' || b == b'\r')
        .unwrap_or(frag.len())
        .min(60);
    let snippet = String::from_utf8_lossy(&frag[..take]).trim().to_string();
    if snippet.is_empty() {
        recovery_found_snippet(input)
    } else {
        Some(snippet)
    }
}
pub(crate) fn build_recovery_error_node(
    input: Input<'_>,
    starters: &[&[u8]],
    scope_label: &str,
    generic_code: &str,
) -> ParseErrorNode {
    build_recovery_error_node_from_span(input, input, starters, scope_label, generic_code)
}

enum RecoveryClassification {
    MissingMemberName {
        code: String,
        message: String,
        expected: String,
        suggestion: String,
    },
    MissingTypeReference {
        code: String,
        message: String,
        expected: String,
        suggestion: String,
    },
    InvalidQualifiedNameSeparator {
        code: String,
        message: String,
        expected: String,
        suggestion: String,
    },
    MissingBodyOrSemicolon {
        code: String,
        message: String,
        expected: String,
        suggestion: String,
    },
    MissingExpressionAfterOperator {
        code: String,
        message: String,
        expected: String,
        suggestion: String,
    },
    InvalidUnitReference {
        code: String,
        message: String,
        expected: String,
        suggestion: String,
    },
    InvalidTypingOperator {
        code: String,
        message: String,
        expected: String,
        suggestion: String,
    },
    InvalidBareIdentifierInBody {
        code: String,
        message: String,
        expected: String,
        suggestion: String,
    },
    UnexpectedKeywordInScope {
        code: String,
        message: String,
        expected: String,
        suggestion: String,
    },
    MissingSemicolon,
    UnsupportedAnnotation,
    Unexpected,
}

fn classify_recovery(
    input: Input<'_>,
    recovery_end: Input<'_>,
    starters: &[&[u8]],
    scope_label: &str,
) -> RecoveryClassification {
    let trimmed = trim_ascii_start(input.fragment());

    if let Some((code, message, expected, suggestion)) =
        missing_name_diagnostic(trimmed, scope_label)
    {
        return RecoveryClassification::MissingMemberName {
            code: code.to_string(),
            message,
            expected,
            suggestion,
        };
    }

    if let Some((code, message, expected, suggestion)) = missing_type_diagnostic(trimmed) {
        return RecoveryClassification::MissingTypeReference {
            code: code.to_string(),
            message,
            expected,
            suggestion,
        };
    }

    if let Some((code, message, expected, suggestion)) =
        invalid_expose_separator_diagnostic(trimmed)
    {
        return RecoveryClassification::InvalidQualifiedNameSeparator {
            code: code.to_string(),
            message,
            expected,
            suggestion,
        };
    }

    if let Some((code, message, expected, suggestion)) = invalid_typing_operator_diagnostic(trimmed)
    {
        return RecoveryClassification::InvalidTypingOperator {
            code: code.to_string(),
            message,
            expected,
            suggestion,
        };
    }

    if let Some((code, message, expected, suggestion)) =
        missing_expression_after_operator_diagnostic(trimmed)
    {
        return RecoveryClassification::MissingExpressionAfterOperator {
            code: code.to_string(),
            message,
            expected,
            suggestion,
        };
    }

    if let Some((code, message, expected, suggestion)) = invalid_unit_reference_diagnostic(trimmed)
    {
        return RecoveryClassification::InvalidUnitReference {
            code: code.to_string(),
            message,
            expected,
            suggestion,
        };
    }

    if let Some((code, message, expected, suggestion)) =
        missing_semicolon_or_body_diagnostic(trimmed)
    {
        return RecoveryClassification::MissingBodyOrSemicolon {
            code: code.to_string(),
            message,
            expected,
            suggestion,
        };
    }

    let consumed_len = recovery_end
        .location_offset()
        .saturating_sub(input.location_offset())
        .min(input.fragment().len());
    let raw_consumed = &input.fragment()[..consumed_len];
    let consumed = trim_ascii_end(raw_consumed);
    let recovered_to_boundary = recovery_end.location_offset() > input.location_offset() && {
        let (next, _) = lex::ws_and_comments(recovery_end).unwrap_or((recovery_end, ()));
        next.fragment().is_empty()
            || next.fragment().starts_with(b"}")
            || lex::starts_with_any_keyword(next.fragment(), starters)
    };

    let consumed_has_newline = raw_consumed.contains(&b'\n') || raw_consumed.contains(&b'\r');
    let first_line_end = consumed
        .iter()
        .position(|b| matches!(*b, b'\n' | b'\r'))
        .unwrap_or(consumed.len());
    let first_line = trim_ascii_end(&consumed[..first_line_end]);
    let consumed_has_delimiters = consumed
        .iter()
        .any(|b| matches!(*b, b'{' | b'}' | b'(' | b')' | b'[' | b']'));
    let consumed_ends_incomplete = first_line.last().is_some_and(|b| {
        matches!(
            *b,
            b':' | b'=' | b',' | b'.' | b'+' | b'-' | b'*' | b'/' | b'>' | b'<' | b'|'
        )
    });
    let first_line_has_semicolon = first_line.contains(&b';');
    if recovered_to_boundary
        && lex::starts_with_any_keyword(trimmed, starters)
        && (consumed_has_newline || recovery_end.fragment().starts_with(b"}"))
        && !consumed.is_empty()
        && !consumed_has_delimiters
        && !consumed_ends_incomplete
        && !first_line_has_semicolon
    {
        return RecoveryClassification::MissingSemicolon;
    }

    if lex::starts_with_keyword(trimmed, b"#") || lex::starts_with_keyword(trimmed, b"@") {
        return RecoveryClassification::UnsupportedAnnotation;
    }

    if let Some((code, message, expected, suggestion)) =
        invalid_bare_identifier_in_body_diagnostic(trimmed, scope_label)
    {
        return RecoveryClassification::InvalidBareIdentifierInBody {
            code: code.to_string(),
            message,
            expected,
            suggestion,
        };
    }

    if let Some((code, message, expected, suggestion)) =
        unexpected_keyword_in_scope_diagnostic(trimmed, starters, scope_label)
    {
        return RecoveryClassification::UnexpectedKeywordInScope {
            code: code.to_string(),
            message,
            expected,
            suggestion,
        };
    }

    RecoveryClassification::Unexpected
}

pub(crate) fn build_recovery_error_node_from_span(
    input: Input<'_>,
    recovery_end: Input<'_>,
    starters: &[&[u8]],
    scope_label: &str,
    generic_code: &str,
) -> ParseErrorNode {
    match classify_recovery(input, recovery_end, starters, scope_label) {
        RecoveryClassification::MissingMemberName {
            code,
            message,
            expected,
            suggestion,
        }
        | RecoveryClassification::MissingTypeReference {
            code,
            message,
            expected,
            suggestion,
        }
        | RecoveryClassification::InvalidQualifiedNameSeparator {
            code,
            message,
            expected,
            suggestion,
        }
        | RecoveryClassification::MissingBodyOrSemicolon {
            code,
            message,
            expected,
            suggestion,
        }
        | RecoveryClassification::MissingExpressionAfterOperator {
            code,
            message,
            expected,
            suggestion,
        }
        | RecoveryClassification::InvalidUnitReference {
            code,
            message,
            expected,
            suggestion,
        }
        | RecoveryClassification::InvalidTypingOperator {
            code,
            message,
            expected,
            suggestion,
        }
        | RecoveryClassification::InvalidBareIdentifierInBody {
            code,
            message,
            expected,
            suggestion,
        }
        | RecoveryClassification::UnexpectedKeywordInScope {
            code,
            message,
            expected,
            suggestion,
        } => ParseErrorNode {
            message,
            code,
            expected: Some(expected),
            found: recovery_found_snippet_from_span(input, recovery_end),
            suggestion: Some(suggestion),
            category: Some(DiagnosticCategory::ParseError),
        },
        RecoveryClassification::MissingSemicolon => ParseErrorNode {
            message: "missing semicolon before next declaration".to_string(),
            code: "missing_semicolon".to_string(),
            expected: Some("';'".to_string()),
            found: recovery_found_snippet_from_span(input, recovery_end),
            suggestion: Some("Insert ';' before this declaration.".to_string()),
            category: Some(DiagnosticCategory::ParseError),
        },
        RecoveryClassification::UnsupportedAnnotation => ParseErrorNode {
            message: format!("unsupported annotation syntax in {scope_label}"),
            code: "unsupported_annotation_syntax".to_string(),
            expected: Some(format!("valid {scope_label} element")),
            found: recovery_found_snippet_from_span(input, recovery_end),
            suggestion: Some(
                "Remove this annotation or extend the parser to support annotated declarations."
                    .to_string(),
            ),
            category: Some(DiagnosticCategory::UnsupportedGrammarForm),
        },
        RecoveryClassification::Unexpected => ParseErrorNode {
            message: format!("unexpected token in {scope_label}"),
            code: generic_code.to_string(),
            expected: Some(format!("valid {scope_label} element")),
            found: recovery_found_snippet_from_span(input, recovery_end),
            suggestion: Some(format!("Fix this {scope_label} member and re-run parsing.")),
            category: Some(DiagnosticCategory::ParseError),
        },
    }
}

pub(crate) fn parse_error_from_recovery_node(
    span: &crate::ast::Span,
    node: &ParseErrorNode,
) -> ParseError {
    let mut err = ParseError::new(node.message.clone())
        .with_location(span.offset, span.line, span.column)
        .with_length(span.len.max(1))
        .with_code(node.code.clone())
        .with_category(
            node.category
                .unwrap_or_else(|| category_from_code(node.code.as_str())),
        );
    let severity = if node.code == "unsupported_annotation_syntax" {
        DiagnosticSeverity::Warning
    } else {
        DiagnosticSeverity::Error
    };
    err = err.with_severity(severity);
    if let Some(expected) = &node.expected {
        err = err.with_expected(expected.clone());
    }
    if let Some(found) = &node.found {
        err = err.with_found(found.clone());
    }
    if let Some(suggestion) = &node.suggestion {
        err = err.with_suggestion(suggestion.clone());
    }
    err
}
