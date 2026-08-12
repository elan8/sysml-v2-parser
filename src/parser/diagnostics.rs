//! Diagnostic classification, nom error mapping, and error post-processing.

use super::delimiters::DelimiterScan;
use super::lex;
use super::Input;
use crate::error::{DiagnosticCategory, DiagnosticSeverity, ParseError};
use nom::error::Error;
const FOUND_SNIPPET_MAX_LEN: usize = 40;

/// Take a short snippet from the input at the error position for "found" display.
/// Uses first line or first FOUND_SNIPPET_MAX_LEN bytes, UTF-8 with replacement char.
pub(crate) fn fragment_to_found_snippet(fragment: &[u8]) -> (String, usize) {
    let take = fragment
        .iter()
        .position(|&b| b == b'\n' || b == b'\r')
        .map(|p| p.min(FOUND_SNIPPET_MAX_LEN))
        .unwrap_or_else(|| fragment.len().min(FOUND_SNIPPET_MAX_LEN));
    let slice = fragment.get(..take).unwrap_or(fragment);
    let s = String::from_utf8_lossy(slice)
        .replace('\n', "\\n")
        .replace('\r', "\\r");
    let len = slice.len();
    (s.trim_end().to_string(), len)
}
/// Map nom error kind to a human-readable message for language server diagnostics.
fn nom_error_kind_to_message(code: &nom::error::ErrorKind) -> &'static str {
    use nom::error::ErrorKind;
    match code {
        ErrorKind::Tag => "expected a specific keyword or punctuation token",
        ErrorKind::Digit => "expected number",
        ErrorKind::Alpha => "expected identifier",
        ErrorKind::AlphaNumeric => "expected identifier",
        ErrorKind::Space => "expected whitespace",
        ErrorKind::MultiSpace => "expected whitespace",
        ErrorKind::Eof => "unexpected end of input",
        ErrorKind::TakeUntil => "expected terminator",
        ErrorKind::TakeWhile1 => "expected token",
        ErrorKind::Alt => {
            "expected a package member (e.g. part, port, attribute, action, requirement, import)"
        }
        ErrorKind::Many0 | ErrorKind::Many1 => {
            "expected a complete body member; check for a missing name, type, or terminator"
        }
        _ => "syntax did not match any known grammar rule here",
    }
}

/// Map nom error kind to a specific code for LSP/quick fixes.
fn nom_error_kind_to_code(code: &nom::error::ErrorKind) -> &'static str {
    use nom::error::ErrorKind;
    match code {
        ErrorKind::Tag => "expected_keyword",
        ErrorKind::Digit => "expected_number",
        ErrorKind::Alpha | ErrorKind::AlphaNumeric => "expected_identifier",
        ErrorKind::Space | ErrorKind::MultiSpace => "expected_whitespace",
        ErrorKind::Eof => "unexpected_eof",
        ErrorKind::TakeUntil => "expected_terminator",
        ErrorKind::TakeWhile1 => "expected_token",
        ErrorKind::Alt => "expected_alt",
        ErrorKind::Many0 | ErrorKind::Many1 => "expected_list",
        _ => "parse_error",
    }
}

pub(crate) fn nom_err_to_parse_error(
    e: &Error<Input<'_>>,
    length_override: Option<usize>,
    expected_context: Option<&'static str>,
) -> ParseError {
    let offset = e.input.location_offset();
    let line = e.input.location_line();
    let column = e.input.get_column();
    let fragment = e.input.fragment();
    let (found_snippet, found_len) = fragment_to_found_snippet(fragment);
    let message = nom_error_kind_to_message(&e.code).to_string();
    let span_len = length_override.unwrap_or(found_len).max(1);
    if trim_ascii_start(fragment).starts_with(b"}") {
        return unexpected_closing_brace_parse_error(e.input);
    }
    let mut pe = ParseError::new(message)
        .with_location(offset, line, column)
        .with_length(span_len)
        .with_code(nom_error_kind_to_code(&e.code))
        .with_severity(DiagnosticSeverity::Error)
        .with_category(DiagnosticCategory::ParseError);
    if !found_snippet.is_empty() {
        pe = pe.with_found(found_snippet);
    }
    if let Some(ctx) = expected_context {
        pe = pe.with_expected(ctx);
    }
    pe
}

pub(crate) fn trim_ascii_start(mut fragment: &[u8]) -> &[u8] {
    while let Some(first) = fragment.first() {
        if first.is_ascii_whitespace() {
            fragment = &fragment[1..];
            continue;
        }
        break;
    }
    fragment
}

pub(crate) fn trim_ascii_end(mut fragment: &[u8]) -> &[u8] {
    while let Some(last) = fragment.last() {
        if last.is_ascii_whitespace() {
            fragment = &fragment[..fragment.len() - 1];
        } else {
            break;
        }
    }
    fragment
}

fn starts_with_missing_type_after_keyword(
    fragment: &[u8],
    keyword: &[u8],
    trailing_keywords: &[&[u8]],
) -> bool {
    let mut fragment = trim_ascii_start(fragment);
    if !lex::starts_with_keyword(fragment, keyword) {
        return false;
    }
    fragment = &fragment[keyword.len()..];
    while let Some(first) = fragment.first() {
        if first.is_ascii_whitespace() {
            fragment = &fragment[1..];
            continue;
        }
        break;
    }
    for trailing in trailing_keywords {
        if lex::starts_with_keyword(fragment, trailing) {
            fragment = &fragment[trailing.len()..];
            while let Some(first) = fragment.first() {
                if first.is_ascii_whitespace() {
                    fragment = &fragment[1..];
                    continue;
                }
                break;
            }
        }
    }

    let mut name_len = 0usize;
    while name_len < fragment.len()
        && (fragment[name_len].is_ascii_alphanumeric() || fragment[name_len] == b'_')
    {
        name_len += 1;
    }
    if name_len == 0 {
        return false;
    }
    fragment = &fragment[name_len..];
    while let Some(first) = fragment.first() {
        if first.is_ascii_whitespace() {
            fragment = &fragment[1..];
            continue;
        }
        break;
    }
    if fragment.starts_with(b":") {
        fragment = &fragment[1..];
    } else if lex::starts_with_keyword(fragment, b"defined") {
        fragment = &fragment[b"defined".len()..];
        fragment = trim_ascii_start(fragment);
        if !lex::starts_with_keyword(fragment, b"by") {
            return false;
        }
        fragment = &fragment[b"by".len()..];
    } else if lex::starts_with_keyword(fragment, b"typed") {
        fragment = &fragment[b"typed".len()..];
        fragment = trim_ascii_start(fragment);
        if !lex::starts_with_keyword(fragment, b"by") {
            return false;
        }
        fragment = &fragment[b"by".len()..];
    } else {
        return false;
    }
    while let Some(first) = fragment.first() {
        if first.is_ascii_whitespace() {
            fragment = &fragment[1..];
            continue;
        }
        break;
    }

    fragment.is_empty()
        || fragment.starts_with(b";")
        || fragment.starts_with(b"{")
        || fragment.starts_with(b"}")
        || lex::starts_with_keyword(fragment, b"then")
        || lex::starts_with_keyword(fragment, b"if")
        || lex::starts_with_keyword(fragment, b"do")
}

pub(crate) fn missing_type_diagnostic(
    fragment: &[u8],
) -> Option<(&'static str, String, String, String)> {
    #[allow(clippy::type_complexity)]
    let cases: &[(&[u8], &[&[u8]], &str)] = &[
        (b"subject", &[], "subject type"),
        (b"actor", &[], "actor type"),
        (b"state", &[], "state type"),
        (b"part", &[], "part type"),
        (b"ref", &[], "reference type"),
        (b"port", &[], "port type"),
        (b"attribute", &[], "attribute type"),
        (b"occurrence", &[], "occurrence type"),
        (b"in", &[], "input type"),
        (b"out", &[], "output type"),
        (b"perform", &[b"action"], "action type"),
        (b"return", &[], "return type"),
    ];

    for &(keyword, trailing, missing_what) in cases {
        if starts_with_missing_type_after_keyword(fragment, keyword, trailing) {
            let keyword_label = String::from_utf8_lossy(keyword);
            let sample_name = if keyword == &b"subject"[..] {
                "laptop"
            } else if keyword == &b"actor"[..] {
                "user"
            } else if keyword == &b"state"[..] {
                "ready"
            } else if keyword == &b"part"[..] {
                "wheel"
            } else if keyword == &b"ref"[..] {
                "sensor"
            } else if keyword == &b"port"[..] {
                "power"
            } else if keyword == &b"attribute"[..] {
                "mass"
            } else if keyword == &b"occurrence"[..] {
                "event"
            } else if keyword == &b"in"[..] {
                "speed"
            } else if keyword == &b"out"[..] {
                "result"
            } else if keyword == &b"perform"[..] {
                "run"
            } else if keyword == &b"return"[..] {
                "result"
            } else {
                "member"
            };
            let sample_type = if keyword == &b"subject"[..] {
                "Laptop"
            } else if keyword == &b"actor"[..] {
                "User"
            } else if keyword == &b"state"[..] {
                "Mode"
            } else if keyword == &b"part"[..] {
                "Wheel"
            } else if keyword == &b"ref"[..] {
                "Sensor"
            } else if keyword == &b"port"[..] {
                "PowerPort"
            } else if keyword == &b"attribute"[..] {
                "MassValue"
            } else if keyword == &b"occurrence"[..] {
                "Event"
            } else if keyword == &b"in"[..] || keyword == &b"out"[..] {
                "Real"
            } else if keyword == &b"perform"[..] {
                "Runner"
            } else if keyword == &b"return"[..] {
                "Real"
            } else {
                "Type"
            };
            let suggestion = if keyword == &b"perform"[..] {
                format!("Use `perform action {sample_name}: {sample_type};`.")
            } else if keyword == &b"return"[..] {
                format!("Use `return {sample_name}: {sample_type};`.")
            } else {
                format!("Use `{keyword_label} {sample_name}: {sample_type};`.")
            };
            return Some((
                "missing_type_reference",
                format!("expected {missing_what} after ':'"),
                format!("{missing_what} after ':'"),
                suggestion,
            ));
        }
    }
    None
}

pub(crate) fn invalid_expose_separator_diagnostic(
    fragment: &[u8],
) -> Option<(&'static str, String, String, String)> {
    let mut fragment = trim_ascii_start(fragment);
    if !lex::starts_with_keyword(fragment, b"expose") {
        return None;
    }
    fragment = &fragment[b"expose".len()..];
    while let Some(first) = fragment.first() {
        if first.is_ascii_whitespace() {
            fragment = &fragment[1..];
            continue;
        }
        break;
    }
    if fragment.is_empty() {
        return None;
    }

    let mut saw_dot = false;
    let mut in_quoted_name = false;
    for &b in fragment {
        if b == b'\'' {
            in_quoted_name = !in_quoted_name;
            continue;
        }
        if in_quoted_name {
            continue;
        }
        if matches!(b, b';' | b'[' | b'{' | b'}' | b'\n' | b'\r') {
            break;
        }
        if b == b'.' {
            saw_dot = true;
            break;
        }
    }
    if !saw_dot {
        return None;
    }

    Some((
        "invalid_qualified_name_separator",
        "invalid qualified name in expose target: use '::' instead of '.'".to_string(),
        "qualified name segments separated by '::'".to_string(),
        "Replace '.' with '::' in the expose target (example: `expose A::B;`).".to_string(),
    ))
}

pub(crate) fn invalid_requirement_short_name_syntax_diagnostic(
    fragment: &[u8],
) -> Option<(&'static str, String, String, String)> {
    let fragment = trim_ascii_start(fragment);
    if fragment.starts_with(b"requirement def") {
        let mut rest = trim_ascii_start(&fragment[b"requirement def".len()..]);
        if rest.starts_with(b"id") {
            rest = trim_ascii_start(&rest[2..]);
            if rest.first() == Some(&b'\'') || rest.first() == Some(&b'"') {
                let quote = rest[0];
                if let Some(close) = rest[1..].iter().position(|&b| b == quote) {
                    let req_id = String::from_utf8_lossy(&rest[1..1 + close]);
                    return Some((
                        "invalid_requirement_short_name_syntax",
                        format!(
                            "requirement definition uses non-standard `id '{req_id}'` syntax; use a short name in angle brackets"
                        ),
                        "short name in angle brackets after `requirement def`".to_string(),
                        format!(
                            "Use `requirement def <'{req_id}'> ...` instead of `requirement def id '{req_id}' ...`."
                        ),
                    ));
                }
            }
        }
    }

    // Header already consumed `id` as a name; recovery starts at the quoted requirement ID.
    if fragment.first() == Some(&b'\'') || fragment.first() == Some(&b'"') {
        let quote = fragment[0];
        if let Some(close) = fragment[1..].iter().position(|&b| b == quote) {
            let req_id = String::from_utf8_lossy(&fragment[1..1 + close]);
            return Some((
                "invalid_requirement_short_name_syntax",
                format!(
                    "requirement ID `'{req_id}'` should use short-name syntax in angle brackets, not a separate `id` keyword"
                ),
                "short name in angle brackets after `requirement def`".to_string(),
                format!("Use `requirement def <'{req_id}'> ...` instead of `requirement def id '{req_id}' ...`."),
            ));
        }
    }
    None
}

fn starts_declaration_header(fragment: &[u8], prefix: &[u8]) -> bool {
    if !fragment.starts_with(prefix) {
        return false;
    }
    let rest = &fragment[prefix.len()..];
    rest.is_empty()
        || rest[0].is_ascii_whitespace()
        || rest[0] == b'<'
        || rest[0] == b';'
        || rest[0] == b'{'
}

pub(crate) fn missing_semicolon_or_body_diagnostic(
    fragment: &[u8],
) -> Option<(&'static str, String, String, String)> {
    if let Some(diag) = invalid_requirement_short_name_syntax_diagnostic(fragment) {
        return Some(diag);
    }
    let fragment = trim_ascii_start(fragment);
    let cases: &[(&[u8], &str, &str)] = &[
        (
            b"action def",
            "action definition",
            "Use `action def Run;` or `action def Run { ... }`.",
        ),
        (
            b"part def",
            "part definition",
            "Use `part def Wheel;` or `part def Wheel { ... }`.",
        ),
        (
            b"requirement def",
            "requirement definition",
            "Use `requirement def R;` or `requirement def R { ... }`.",
        ),
        (
            b"state def",
            "state definition",
            "Use `state def Ready;` or `state def Ready { ... }`.",
        ),
        (
            b"view",
            "view declaration",
            "Use `view structure: GeneralView;` or `view structure: GeneralView { ... }`.",
        ),
        (
            b"rendering def",
            "rendering definition",
            "Use `rendering def Diagram;` or `rendering def Diagram { ... }`.",
        ),
    ];

    for (prefix, label, suggestion) in cases {
        if starts_declaration_header(fragment, prefix) {
            return Some((
                "missing_body_or_semicolon",
                format!("expected ';' or '{{' after {label} header"),
                "';' or '{' after declaration header".to_string(),
                suggestion.to_string(),
            ));
        }
    }
    None
}

/// Declaration header only (stops at `{` or `;`) so body usages with `:` are not misclassified.
fn definition_declaration_header(fragment: &[u8]) -> &[u8] {
    let fragment = trim_ascii_start(fragment);
    let end = fragment
        .iter()
        .position(|&b| b == b'{' || b == b';')
        .unwrap_or(fragment.len());
    trim_ascii_end(&fragment[..end])
}

/// True when a definition header uses `:` for subclassification instead of `:>`.
fn definition_header_has_invalid_specialization_colon(header: &[u8]) -> bool {
    let header = trim_ascii_start(header);
    let prefixes: &[(&[u8], &str)] = &[(b"part def", "part def"), (b"port def", "port def")];
    for (prefix, _) in prefixes {
        if !header.starts_with(prefix) {
            continue;
        }
        let mut rest = trim_ascii_start(&header[prefix.len()..]);
        if rest.starts_with(b"<") {
            if let Some(close) = rest[1..].iter().position(|&b| b == b'>') {
                rest = trim_ascii_start(&rest[close + 2..]);
            } else {
                return false;
            }
        }
        while !rest.is_empty() && !rest[0].is_ascii_whitespace() && rest[0] != b':' {
            rest = &rest[1..];
        }
        rest = trim_ascii_start(rest);
        if rest.starts_with(b":>") || rest.starts_with(b":>>") {
            return false;
        }
        if rest.starts_with(b"specializes") {
            return false;
        }
        if rest.first() == Some(&b':') {
            return true;
        }
    }
    false
}

pub(crate) fn invalid_typing_operator_diagnostic(
    fragment: &[u8],
) -> Option<(&'static str, String, String, String)> {
    let header = definition_declaration_header(fragment);
    if !definition_header_has_invalid_specialization_colon(header) {
        return None;
    }
    let (label, suggestion) = if header.starts_with(b"port def") {
        (
            "port definition specialization",
            "Use `port def PowerPort :> BasePort;` when specializing a definition.",
        )
    } else {
        (
            "part definition specialization",
            "Use `part def Vehicle :> BaseVehicle;` when specializing a definition.",
        )
    };
    Some((
        "invalid_typing_operator",
        format!("invalid typing operator in {label}: use ':>' instead of ':'"),
        "':>' specialization operator".to_string(),
        suggestion.to_string(),
    ))
}

pub(crate) fn missing_expression_after_operator_diagnostic(
    fragment: &[u8],
) -> Option<(&'static str, String, String, String)> {
    let fragment = trim_ascii_start(fragment);
    let cases: &[(&[u8], &str, &str)] = &[
        (
            b"bind",
            "binding expression after '='",
            "Use `bind x = y;`.",
        ),
        (
            b"assign",
            "assignment expression after ':='",
            "Use `assign x := y;`.",
        ),
        (
            b"first",
            "target after 'then'",
            "Use `first start then finish;`.",
        ),
        (
            b"flow",
            "target after 'to'",
            "Use `flow source to target;`.",
        ),
        (
            b"satisfy",
            "target after 'by'",
            "Use `satisfy Req by implementation;`.",
        ),
    ];

    // GH-29: bound the scan to the current statement (mirrors `invalid_unit_reference_diagnostic`/
    // `bare_comma_sequence_diagnostic`, GH-18/#28) -- otherwise a `.contains()` match on unrelated
    // text or a comment further down the file (e.g. a doc comment mentioning `= ;`) can override
    // the true diagnostic for a genuinely malformed statement here.
    let window = local_statement_window(fragment);
    let text = String::from_utf8_lossy(window);
    for (keyword, expected, suggestion) in cases {
        if !lex::starts_with_keyword(fragment, keyword) {
            continue;
        }
        if text.contains("= ;") || text.trim_end().ends_with('=') {
            return Some((
                "missing_expression_after_operator",
                "expected expression after '='".to_string(),
                expected.to_string(),
                suggestion.to_string(),
            ));
        }
        if text.contains(":= ;") || text.trim_end().ends_with(":=") {
            return Some((
                "missing_expression_after_operator",
                "expected expression after ':='".to_string(),
                expected.to_string(),
                suggestion.to_string(),
            ));
        }
        if text.contains(" then ;") || text.trim_end().ends_with(" then") {
            return Some((
                "missing_expression_after_operator",
                "expected target after 'then'".to_string(),
                expected.to_string(),
                suggestion.to_string(),
            ));
        }
        if text.contains(" to ;") || text.trim_end().ends_with(" to") {
            return Some((
                "missing_expression_after_operator",
                "expected target after 'to'".to_string(),
                expected.to_string(),
                suggestion.to_string(),
            ));
        }
        if text.contains(" by ;") || text.trim_end().ends_with(" by") {
            return Some((
                "missing_expression_after_operator",
                "expected target after 'by'".to_string(),
                expected.to_string(),
                suggestion.to_string(),
            ));
        }
    }
    None
}

/// Bounds `fragment` to the current statement/member: scanning stops before entering a `//` or
/// `/* */` comment, at the first depth-0 `;`, or at a depth-0 closing `}`/`)`/`]` that isn't
/// matched by an opener within the window (i.e. the enclosing scope's own delimiter). Local
/// pattern-matching diagnostics (like [`invalid_unit_reference_diagnostic`] and
/// [`bare_comma_sequence_diagnostic`]) must scan within this window rather than the unbounded rest
/// of the file -- otherwise bracket- or comma-like text in unrelated code, or in a doc comment far
/// below the real error site, can override the true diagnostic (GH-18).
fn local_statement_window(fragment: &[u8]) -> &[u8] {
    const MAX_WINDOW: usize = 400;
    let mut depth: i32 = 0;
    let mut end = 0usize;
    while end < fragment.len() && end < MAX_WINDOW {
        if fragment[end..].starts_with(b"/*") || fragment[end..].starts_with(b"//") {
            break;
        }
        match fragment[end] {
            b'{' | b'(' | b'[' => depth += 1,
            b'}' | b')' | b']' => {
                if depth == 0 {
                    break;
                }
                depth -= 1;
            }
            b';' if depth == 0 => {
                end += 1;
                break;
            }
            _ => {}
        }
        end += 1;
    }
    &fragment[..end]
}

pub(crate) fn invalid_unit_reference_diagnostic(
    fragment: &[u8],
) -> Option<(&'static str, String, String, String)> {
    let fragment = trim_ascii_start(fragment);
    let window = local_statement_window(fragment);
    let text = String::from_utf8_lossy(window);
    if !(text.contains('[') && text.contains(']')) {
        return None;
    }

    if text.contains("[]") || text.contains("[ ]") {
        return Some((
            "invalid_unit_reference",
            "expected unit name inside '[ ]'".to_string(),
            "unit name inside '[ ]'".to_string(),
            "Use a concrete unit such as `1750 [kg]`.".to_string(),
        ));
    }

    if text.contains("[;")
        || text.contains("[ ;")
        || text.contains("[)")
        || text.contains("[ ]")
        || text.contains("[,")
    {
        return Some((
            "invalid_unit_reference",
            "invalid unit expression inside '[ ]'".to_string(),
            "unit name inside '[ ]'".to_string(),
            "Use a unit symbol or qualified unit name (example: `[kg]` or `[SI::kg]`).".to_string(),
        ));
    }

    None
}

pub(crate) fn unexpected_keyword_in_scope_diagnostic(
    fragment: &[u8],
    starters: &[&[u8]],
    scope_label: &str,
) -> Option<(&'static str, String, String, String)> {
    let fragment = trim_ascii_start(fragment);
    if fragment.is_empty() || fragment.starts_with(b"#") || fragment.starts_with(b"@") {
        return None;
    }
    let keyword_end = fragment
        .iter()
        .position(|b| !b.is_ascii_alphanumeric() && *b != b'_')
        .unwrap_or(fragment.len());
    if keyword_end == 0 {
        return None;
    }
    let keyword = &fragment[..keyword_end];
    if lex::starts_with_any_keyword(keyword, starters) {
        return None;
    }
    let keyword_text = String::from_utf8_lossy(keyword);
    if lex::is_reserved_keyword(keyword) {
        // A real SysML keyword used somewhere it isn't valid in this scope -- a grammar-context
        // mismatch, so "unexpected keyword" is accurate.
        Some((
            "unexpected_keyword_in_scope",
            format!("unexpected keyword `{keyword_text}` in {scope_label}"),
            format!("valid {scope_label} element"),
            format!("Replace `{keyword_text}` with a valid {scope_label} member or remove it."),
        ))
    } else {
        // Not a SysML keyword at all -- an unrecognized identifier is an input defect, not a
        // parser gap, so don't imply it's unsupported valid syntax (GH-18).
        Some((
            "unrecognized_declaration_in_scope",
            format!("unrecognized declaration `{keyword_text}` in {scope_label}"),
            format!("valid {scope_label} element"),
            format!(
                "`{keyword_text}` is not a SysML keyword; check for a typo or unsupported construct, or remove it."
            ),
        ))
    }
}

/// Detects a comma-separated value list without sequence brackets, e.g.
/// `part :>> readings = a, b;` -- the expression parser stops at the first bare `,`, so recovery
/// otherwise reports a generic "unexpected token" for the whole member instead of pointing at the
/// missing `( ... )` (GH-18).
pub(crate) fn bare_comma_sequence_diagnostic(
    fragment: &[u8],
) -> Option<(&'static str, String, String, String)> {
    let fragment = trim_ascii_start(fragment);
    let window = local_statement_window(fragment);
    let mut depth: i32 = 0;
    let mut has_assign = false;
    for &b in window {
        match b {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            b'=' if depth == 0 => has_assign = true,
            b',' if depth == 0 && has_assign => {
                return Some((
                    "bare_comma_in_feature_value",
                    "unexpected ',' in feature value assignment; use sequence brackets '(a, b)' to assign multiple values"
                        .to_string(),
                    "sequence brackets '(a, b)' around a comma-separated value list".to_string(),
                    "Wrap the comma-separated values in parentheses, for example `= (a, b)`."
                        .to_string(),
                ));
            }
            _ => {}
        }
    }
    None
}

pub(crate) fn invalid_bare_identifier_in_body_diagnostic(
    fragment: &[u8],
    scope_label: &str,
) -> Option<(&'static str, String, String, String)> {
    let is_action = scope_label.contains("action body");
    let is_state = scope_label.contains("state body");
    if !is_action && !is_state {
        return None;
    }

    let fragment = trim_ascii_start(fragment);
    let ident_end = fragment
        .iter()
        .position(|b| !b.is_ascii_alphanumeric() && *b != b'_')
        .unwrap_or(fragment.len());
    if ident_end == 0 || !fragment[0].is_ascii_alphabetic() {
        return None;
    }

    let ident = &fragment[..ident_end];
    let rest = trim_ascii_start(&fragment[ident_end..]);
    if !(rest.starts_with(b";")
        || rest.starts_with(b"}")
        || rest.starts_with(b"\n")
        || rest.starts_with(b"\r"))
    {
        return None;
    }

    let ident_text = String::from_utf8_lossy(ident);
    if is_action {
        Some((
            "invalid_bare_identifier_in_action_body",
            format!("bare identifier `{ident_text}` is not a valid action body member"),
            "action body member such as `perform`, `bind`, `in`, or `out`".to_string(),
            format!(
                "Use an explicit action-body form, for example `perform {ident_text};`, `bind ... = ...;`, or an `in`/`out` parameter declaration."
            ),
        ))
    } else {
        Some((
            "invalid_bare_identifier_in_state_body",
            format!("bare identifier `{ident_text}` is not a valid state body member"),
            "state body member such as `entry`, `transition`, `then`, `state`, or `ref`"
                .to_string(),
            format!(
                "Use an explicit state-body form, for example `then {ident_text};`, `transition ...;`, or a nested `state` member."
            ),
        ))
    }
}

pub(crate) fn unexpected_closing_brace_parse_error(input: Input<'_>) -> ParseError {
    ParseError::new("unexpected closing '}'")
        .with_location(
            input.location_offset(),
            input.location_line(),
            input.get_column(),
        )
        .with_length(1)
        .with_code("unexpected_closing_brace")
        .with_expected("valid declaration or end of current body")
        .with_found("}")
        .with_suggestion("Remove this '}' or add the missing opening '{' before it.")
        .with_severity(DiagnosticSeverity::Error)
        .with_category(DiagnosticCategory::ParseError)
}

/// Report an unterminated body when the parser stopped at end of input with a body still open.
pub(crate) fn missing_closing_brace_error(
    bytes: &[u8],
    input: Input<'_>,
    delimiters: &DelimiterScan,
) -> Option<ParseError> {
    if !input.fragment().is_empty() || !delimiters.has_unclosed_brace() {
        return None;
    }
    Some(missing_closing_brace_error_at_eof(bytes))
}

pub(crate) fn missing_closing_brace_error_at_eof(bytes: &[u8]) -> ParseError {
    let (line, column) = eof_line_column(bytes);
    ParseError::new("missing closing '}'")
        .with_location(bytes.len(), line, column)
        .with_length(1)
        .with_code("missing_closing_brace")
        .with_expected("'}'")
        .with_suggestion("Add '}' to close the open body.")
        .with_category(DiagnosticCategory::ParseError)
}

/// Report the trailing `}` of a document that closes more bodies than it opens.
pub(crate) fn extra_closing_brace_at_eof(
    bytes: &[u8],
    delimiters: &DelimiterScan,
) -> Option<ParseError> {
    let (opens, closes) = delimiters.balance();
    if closes <= opens {
        return None;
    }
    let offset = delimiters.last_close()?;
    let (line, column) = eof_line_column(&bytes[..offset]);
    Some(
        ParseError::new("unexpected closing '}' at end of file")
            .with_location(offset, line, column)
            .with_length(1)
            .with_code("unexpected_closing_brace")
            .with_expected("end of file or valid declaration")
            .with_found("}")
            .with_suggestion(
                "Remove this extra '}' or add the missing opening '{' earlier in the file.",
            )
            .with_category(DiagnosticCategory::ParseError),
    )
}

pub(crate) fn category_from_code(code: &str) -> DiagnosticCategory {
    if code == "unsupported_annotation_syntax" {
        DiagnosticCategory::UnsupportedGrammarForm
    } else {
        DiagnosticCategory::ParseError
    }
}

fn eof_line_column(bytes: &[u8]) -> (u32, usize) {
    let mut line = 1u32;
    let mut column = 1usize;
    for &b in bytes {
        if b == b'\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

fn diagnostic_specificity(err: &ParseError) -> u8 {
    match err.code.as_deref() {
        Some("missing_type_reference")
        | Some("invalid_qualified_name_separator")
        | Some("invalid_typing_operator")
        | Some("missing_expression_after_operator")
        | Some("invalid_unit_reference")
        | Some("missing_body_or_semicolon")
        | Some("invalid_requirement_short_name_syntax")
        | Some("missing_semicolon")
        | Some("unexpected_closing_brace")
        | Some("missing_closing_brace")
        | Some("unsupported_annotation_syntax")
        | Some("invalid_bare_identifier_in_action_body")
        | Some("invalid_bare_identifier_in_state_body")
        | Some("recovery_cascade_suppressed")
        | Some("unexpected_keyword_in_scope")
        | Some("unrecognized_declaration_in_scope")
        | Some("bare_comma_in_feature_value") => 5,
        Some(code) if code.starts_with("recovered_") => 2,
        Some("expected_end_of_input") | Some("expected_keyword") => 1,
        _ => 3,
    }
}

/// Drop `unexpected_closing_brace` on a line that already has a parse error for an
/// invalid statement block (e.g. `badstmt {} }` â€” the second `}` closes the package).
pub(crate) fn suppress_redundant_closing_brace_errors(errors: Vec<ParseError>) -> Vec<ParseError> {
    // Recovery nodes can cover the entire unterminated declaration while the document-level
    // balance check reports the same condition at EOF. Keep the local EOF diagnostic: it gives the
    // editor the actionable insertion point and avoids publishing the recovery node's broad span as
    // a second error for the same missing token.
    let final_missing_closing_brace = errors
        .iter()
        .filter(|error| error.code.as_deref() == Some("missing_closing_brace"))
        .filter_map(|error| error.offset)
        .max();
    let lines_with_block_error: std::collections::HashSet<u32> = errors
        .iter()
        .filter(|e| e.code.as_deref() != Some("unexpected_closing_brace"))
        .filter_map(|e| e.line)
        .filter(|line| {
            errors.iter().any(|other| {
                other.line == Some(*line)
                    && other
                        .found
                        .as_deref()
                        .is_some_and(|f| f.contains('{') && f.contains('}'))
            })
        })
        .collect();

    errors
        .into_iter()
        .filter(|e| {
            if e.code.as_deref() == Some("missing_closing_brace") {
                return e.offset == final_missing_closing_brace;
            }
            if e.code.as_deref() != Some("unexpected_closing_brace") {
                return true;
            }
            e.line
                .map(|line| !lines_with_block_error.contains(&line))
                .unwrap_or(true)
        })
        .collect()
}

pub(crate) fn dedup_errors(mut errors: Vec<ParseError>) -> Vec<ParseError> {
    errors.sort_by_key(|e| {
        (
            e.offset.unwrap_or(usize::MAX),
            e.line.unwrap_or(u32::MAX),
            e.column.unwrap_or(usize::MAX),
            std::cmp::Reverse(diagnostic_specificity(e)),
        )
    });

    let mut deduped = Vec::new();
    for err in errors {
        let duplicate = deduped.iter().any(|existing: &ParseError| {
            let same_start = existing.offset == err.offset
                && existing.line == err.line
                && existing.column == err.column;
            let same_found = existing.found == err.found;
            let existing_specificity = diagnostic_specificity(existing);
            let err_specificity = diagnostic_specificity(&err);
            same_start
                && (same_found || existing.code == err.code)
                && existing_specificity >= err_specificity
        });
        if !duplicate {
            deduped.push(err);
        }
    }

    deduped.sort_by_key(|e| (e.offset.unwrap_or(usize::MAX), e.line.unwrap_or(u32::MAX)));
    deduped
}

fn is_cascade_candidate(err: &ParseError) -> bool {
    matches!(
        err.code.as_deref(),
        Some("missing_semicolon") | Some("missing_body_or_semicolon")
    ) || err
        .code
        .as_deref()
        .is_some_and(|code| code.starts_with("recovered_"))
}

fn cascade_family(err: &ParseError) -> Option<&str> {
    match err.code.as_deref() {
        Some("missing_semicolon") => Some("missing_semicolon"),
        Some("missing_body_or_semicolon") => Some("missing_body_or_semicolon"),
        Some(code) if code.starts_with("recovered_") => Some("recovered"),
        _ => None,
    }
}

const MAX_CASCADE_LINE_DISTANCE: u32 = 50;

fn make_cascade_summary(run: &[ParseError]) -> Option<ParseError> {
    let summary_anchor = run.first()?;
    let suppressed = run.len().saturating_sub(1);
    let family = cascade_family(summary_anchor).unwrap_or("recovery");
    let mut err = ParseError::new(format!(
        "suppressed {suppressed} cascading {family} diagnostic{} after earlier recovery errors",
        if suppressed == 1 { "" } else { "s" }
    ))
    .with_location(
        summary_anchor.offset?,
        summary_anchor.line?,
        summary_anchor.column?,
    )
    .with_length(summary_anchor.length.unwrap_or(1).max(1))
    .with_code("recovery_cascade_suppressed")
    .with_expected("fix the first syntax error in this body")
    .with_suggestion(
        "Fix the earliest diagnostic in this body first; later syntax errors may be cascades.",
    )
    .with_severity(DiagnosticSeverity::Warning)
    .with_category(DiagnosticCategory::ParseError);
    if let Some(found) = &summary_anchor.found {
        err = err.with_found(found.clone());
    }
    Some(err)
}

pub(crate) fn suppress_diagnostic_cascades(errors: Vec<ParseError>) -> Vec<ParseError> {
    const MAX_UNSUMMARIZED_CASCADE: usize = 1;

    let mut output = Vec::new();
    let mut run: Vec<ParseError> = Vec::new();

    let flush_run = |run: &mut Vec<ParseError>, output: &mut Vec<ParseError>| {
        if run.is_empty() {
            return;
        }
        if run.len() <= MAX_UNSUMMARIZED_CASCADE {
            output.append(run);
        } else {
            let primary_offset = run.first().and_then(|e| e.offset);
            if let Some(mut primary) = run.first().cloned() {
                primary.is_cascade = Some(false);
                output.push(primary);
            }
            for suppressed in run.iter().skip(MAX_UNSUMMARIZED_CASCADE) {
                let _ = primary_offset;
                let _ = suppressed;
            }
            if let Some(summary) = make_cascade_summary(run) {
                output.push(summary);
            }
            run.clear();
        }
    };

    for err in errors {
        let continues_run = run.last().is_some_and(|previous| {
            is_cascade_candidate(&err)
                && cascade_family(previous) == cascade_family(&err)
                && previous
                    .line
                    .zip(err.line)
                    .is_some_and(|(a, b)| b <= a.saturating_add(MAX_CASCADE_LINE_DISTANCE))
        });

        if is_cascade_candidate(&err) && (run.is_empty() || continues_run) {
            run.push(err);
        } else {
            flush_run(&mut run, &mut output);
            if is_cascade_candidate(&err) {
                run.push(err);
            } else {
                output.push(err);
            }
        }
    }
    flush_run(&mut run, &mut output);
    output.sort_by_key(|e| (e.offset.unwrap_or(usize::MAX), e.line.unwrap_or(u32::MAX)));
    output
}

pub(crate) fn root_body_recovery_error(input: Input<'_>, scope: &str) -> ParseError {
    let (found, len) = fragment_to_found_snippet(input.fragment());
    let mut err = ParseError::new(format!(
        "could not parse {scope} body; skipped to next root element"
    ))
    .with_location(
        input.location_offset(),
        input.location_line(),
        input.get_column(),
    )
    .with_length(len.max(1))
    .with_code("recovered_root_body")
    .with_expected(format!("valid {scope} body"))
    .with_suggestion(
        "Fix the first syntax error in this body; later root-level diagnostics may be cascades.",
    )
    .with_severity(DiagnosticSeverity::Error)
    .with_category(DiagnosticCategory::ParseError);
    if !found.is_empty() {
        err = err.with_found(found);
    }
    err
}

pub(crate) fn root_body_scope(fragment: &[u8]) -> Option<&'static str> {
    let fragment = trim_ascii_start(fragment);
    if lex::starts_with_keyword(fragment, b"package")
        || lex::starts_with_keyword(fragment, b"library")
        || lex::starts_with_keyword(fragment, b"standard")
    {
        Some("package")
    } else if lex::starts_with_keyword(fragment, b"namespace") {
        Some("namespace")
    } else {
        None
    }
}
