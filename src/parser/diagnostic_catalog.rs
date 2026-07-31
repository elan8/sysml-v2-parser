//! Stable parser diagnostic code registry for Spec42 and LSP consumers.

/// Parser-owned diagnostic codes emitted by recovery and validation paths.
pub const MISSING_TYPE_REFERENCE: &str = "missing_type_reference";
pub const INVALID_TYPING_OPERATOR: &str = "invalid_typing_operator";
pub const INVALID_QUALIFIED_NAME_SEPARATOR: &str = "invalid_qualified_name_separator";
pub const MISSING_EXPRESSION_AFTER_OPERATOR: &str = "missing_expression_after_operator";
pub const INVALID_UNIT_REFERENCE: &str = "invalid_unit_reference";
pub const INVALID_BARE_IDENTIFIER_IN_ACTION_BODY: &str = "invalid_bare_identifier_in_action_body";
pub const INVALID_BARE_IDENTIFIER_IN_STATE_BODY: &str = "invalid_bare_identifier_in_state_body";
pub const UNEXPECTED_KEYWORD_IN_SCOPE: &str = "unexpected_keyword_in_scope";
pub const UNRECOGNIZED_DECLARATION_IN_SCOPE: &str = "unrecognized_declaration_in_scope";
pub const BARE_COMMA_IN_FEATURE_VALUE: &str = "bare_comma_in_feature_value";
pub const INVALID_REQUIREMENT_SHORT_NAME_SYNTAX: &str = "invalid_requirement_short_name_syntax";
pub const UNSUPPORTED_ANNOTATION_SYNTAX: &str = "unsupported_annotation_syntax";
pub const RECOVERY_CASCADE_SUPPRESSED: &str = "recovery_cascade_suppressed";
pub const RECOVERED_ROOT_BODY: &str = "recovered_root_body";
pub const MISSING_CLOSING_BRACE: &str = "missing_closing_brace";
pub const UNEXPECTED_CLOSING_BRACE: &str = "unexpected_closing_brace";
pub const MISSING_SEMICOLON: &str = "missing_semicolon";
pub const MISSING_BODY_OR_SEMICOLON: &str = "missing_body_or_semicolon";
pub const MISSING_REP_LANGUAGE: &str = "missing_rep_language";
pub const INVALID_REP_LANGUAGE: &str = "invalid_rep_language";
pub const NESTING_TOO_DEEP: &str = "nesting_too_deep";
pub const UNEXPECTED_EOF: &str = "unexpected_eof";
pub const EXPECTED_END_OF_INPUT: &str = "expected_end_of_input";

/// All stable codes documented for cross-repo contracts.
///
/// Recovery fallbacks that start with `recovered_` are intentionally omitted here; they are
/// scope-local editor UX codes, not a fixed Spec42 contract surface.
pub const DOCUMENTED_CODES: &[&str] = &[
    MISSING_TYPE_REFERENCE,
    INVALID_TYPING_OPERATOR,
    INVALID_QUALIFIED_NAME_SEPARATOR,
    MISSING_EXPRESSION_AFTER_OPERATOR,
    INVALID_UNIT_REFERENCE,
    INVALID_BARE_IDENTIFIER_IN_ACTION_BODY,
    INVALID_BARE_IDENTIFIER_IN_STATE_BODY,
    UNEXPECTED_KEYWORD_IN_SCOPE,
    UNRECOGNIZED_DECLARATION_IN_SCOPE,
    BARE_COMMA_IN_FEATURE_VALUE,
    INVALID_REQUIREMENT_SHORT_NAME_SYNTAX,
    UNSUPPORTED_ANNOTATION_SYNTAX,
    RECOVERY_CASCADE_SUPPRESSED,
    RECOVERED_ROOT_BODY,
    MISSING_CLOSING_BRACE,
    UNEXPECTED_CLOSING_BRACE,
    MISSING_SEMICOLON,
    MISSING_BODY_OR_SEMICOLON,
    MISSING_REP_LANGUAGE,
    INVALID_REP_LANGUAGE,
    NESTING_TOO_DEEP,
    UNEXPECTED_EOF,
    EXPECTED_END_OF_INPUT,
];
