//! Recovery error nodes and classification for structured body parsing.

use super::diagnostics::{
    bare_comma_sequence_diagnostic, category_from_code, invalid_bare_identifier_in_body_diagnostic,
    invalid_bracket_expression_diagnostic, invalid_end_feature_prefix_diagnostic,
    invalid_expose_separator_diagnostic, invalid_typing_operator_diagnostic,
    missing_expression_after_operator_diagnostic, missing_semicolon_or_body_diagnostic,
    missing_type_diagnostic, trim_ascii_end, trim_ascii_start,
    unexpected_keyword_in_scope_diagnostic, verify_requirement_bare_reference_diagnostic,
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
    InvalidBracketExpression {
        code: String,
        message: String,
        expected: String,
        suggestion: String,
    },
    BareCommaSequence {
        code: String,
        message: String,
        expected: String,
        suggestion: String,
    },
    /// `end` spelled with a slot only `BasicFeaturePrefix` owns (KerML BNF 573/577/584).
    InvalidEndFeaturePrefix {
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
    /// `verify requirement <feature-chain>;` -- the `requirement` keyword needs a declaration,
    /// not a bare reference (SysML textual BNF `RequirementVerificationUsage`).
    VerifyRequirementExpectsDeclaration {
        code: String,
        message: String,
        expected: String,
        suggestion: String,
    },
    MissingSemicolon,
    /// A `#` or `@` head this scope does not model, but that the pinned grammar admits.
    UnsupportedAnnotation,
    /// A `#` or `@` sigil not followed by the `[QualifiedName]` its production requires.
    ///
    /// Distinct from [`RecoveryClassification::UnsupportedAnnotation`]: that one is valid syntax
    /// the parser has not reached yet, this one is not a metadata reference at all, and the two
    /// must stay separable in both the recovery node and its diagnostic.
    MalformedAnnotationHead,
    Unexpected,
}

/// Whether a `#`/`@` sigil is followed by the `[QualifiedName]` its production requires.
///
/// `PrefixMetadataFeature` and `MetadataFeatureDeclaration` both end at an `OwnedFeatureTyping`,
/// so a sigil followed by anything that cannot begin a name -- `#;`, `@ {`, `#::x` -- is
/// malformed rather than merely unsupported here.
fn annotation_head_is_well_formed(trimmed: &[u8]) -> bool {
    let after_sigil = trim_ascii_start(&trimmed[1..]);
    after_sigil
        .first()
        .is_some_and(|b| b.is_ascii_alphabetic() || *b == b'_' || *b == b'\'' || *b == b'$')
}

fn classify_recovery(
    input: Input<'_>,
    recovery_end: Input<'_>,
    starters: &[&[u8]],
    scope_label: &str,
) -> RecoveryClassification {
    let trimmed = trim_ascii_start(input.fragment());

    if let Some((code, message, expected, suggestion)) =
        verify_requirement_bare_reference_diagnostic(trimmed)
    {
        return RecoveryClassification::VerifyRequirementExpectsDeclaration {
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

    // Ahead of the keyword-in-scope classifications: those would report the *scope* as the
    // problem, when the authored prefix is what has no derivation.
    if let Some((code, message, expected, suggestion)) =
        invalid_end_feature_prefix_diagnostic(trimmed)
    {
        return RecoveryClassification::InvalidEndFeaturePrefix {
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

    if let Some((code, message, expected, suggestion)) =
        invalid_bracket_expression_diagnostic(trimmed)
    {
        return RecoveryClassification::InvalidBracketExpression {
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

    if let Some((code, message, expected, suggestion)) = bare_comma_sequence_diagnostic(trimmed) {
        return RecoveryClassification::BareCommaSequence {
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

    if trimmed.starts_with(b"#") || trimmed.starts_with(b"@") {
        return if annotation_head_is_well_formed(trimmed) {
            RecoveryClassification::UnsupportedAnnotation
        } else {
            RecoveryClassification::MalformedAnnotationHead
        };
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
        RecoveryClassification::MissingTypeReference {
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
        | RecoveryClassification::InvalidBracketExpression {
            code,
            message,
            expected,
            suggestion,
        }
        | RecoveryClassification::BareCommaSequence {
            code,
            message,
            expected,
            suggestion,
        }
        | RecoveryClassification::InvalidEndFeaturePrefix {
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
        }
        | RecoveryClassification::VerifyRequirementExpectsDeclaration {
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
            message: format!(
                "incomplete parser support for metadata syntax in {scope_label}"
            ),
            code: "unsupported_annotation_syntax".to_string(),
            expected: Some(format!("supported {scope_label} element or metadata form")),
            found: recovery_found_snippet_from_span(input, recovery_end),
            suggestion: Some(
                "This `#`/`@` metadata form is legal SysML but not fully parsed yet; rewrite using a supported metadata form or simplify the annotated declaration."
                    .to_string(),
            ),
            category: Some(DiagnosticCategory::UnsupportedGrammarForm),
        },
        RecoveryClassification::MalformedAnnotationHead => ParseErrorNode {
            message: format!("malformed metadata reference in {scope_label}"),
            code: "malformed_annotation_head".to_string(),
            expected: Some("qualified name after `#` or `@`".to_string()),
            found: recovery_found_snippet_from_span(input, recovery_end),
            suggestion: Some(
                "`#` and `@` are followed by the qualified name of a metadata type, as in `#safety` or `@Safety`."
                    .to_string(),
            ),
            category: Some(DiagnosticCategory::ParseError),
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

/// The member form of [`unsupported_body_member`]: recognizes one of the scope's spec-valid
/// starter keywords, consumes the statement or block it introduces, and retains it as an
/// unsupported node rather than as untyped text.
pub(crate) fn unsupported_member<'a>(
    input: crate::parser::Input<'a>,
    starters: &[&[u8]],
    scope_label: &str,
) -> nom::IResult<crate::parser::Input<'a>, crate::ast::Node<crate::ast::UnsupportedGrammarNode>> {
    use crate::parser::lex::{skip_statement_or_block, starts_with_any_keyword, ws_and_comments};
    let (input, _) = ws_and_comments(input)?;
    if !starts_with_any_keyword(input.fragment(), starters) {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let start = input;
    let (input, _) = skip_statement_or_block(input)?;
    Ok((input, unsupported_body_member(start, input, scope_label)))
}

/// A spec-valid member that the containing scope does not model.
///
/// The member's text is retained by span, not copied, and it carries a diagnostic: keeping it
/// silently -- as an opaque string with no report -- hid real gaps from every consumer.
pub(crate) fn unsupported_body_member(
    start: crate::parser::Input<'_>,
    end: crate::parser::Input<'_>,
    scope_label: &str,
) -> crate::ast::Node<crate::ast::UnsupportedGrammarNode> {
    let span = crate::parser::span::span_from_to(start, end);
    let found = String::from_utf8_lossy(&start.fragment()[..span.len.min(start.fragment().len())])
        .trim()
        .to_string();
    let diagnostic = ParseErrorNode {
        message: format!(
            "this {scope_label} member is spec-valid but not structurally implemented"
        ),
        code: "unsupported_grammar_form".to_owned(),
        expected: None,
        found: (!found.is_empty()).then_some(found),
        suggestion: Some(
            "The member is retained with its source span; its structure is not modelled yet."
                .to_owned(),
        ),
        category: Some(DiagnosticCategory::UnsupportedGrammarForm),
    };
    crate::ast::Node::new(
        span,
        crate::ast::UnsupportedGrammarNode {
            production: crate::ast::UnsupportedProduction::UnmodelledBodyMember,
            diagnostic,
        },
    )
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
    let severity = if node.code == "unsupported_annotation_syntax"
        || node.code == "unsupported_grammar_form"
    {
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
