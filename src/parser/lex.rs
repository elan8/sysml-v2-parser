//! Lexer and skip helpers: whitespace, comments, names, qualified names, and body-skip utilities.

use crate::ast::Identification;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while, take_while1};
use nom::combinator::{map, opt, rest, value};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;
use nom::Parser;

pub(crate) const PACKAGE_BODY_STARTERS: &[&[u8]] = &[
    b"#",
    b"action",
    b"actor",
    b"analysis",
    b"alias",
    b"allocate",
    b"allocation",
    b"abstract",
    b"assert",
    b"assume",
    b"attribute",
    b"calc",
    b"comment",
    b"concern",
    b"connection",
    b"constraint",
    b"case",
    b"dependency",
    b"doc",
    b"enum",
    b"expose",
    b"filter",
    b"frame",
    b"flow",
    b"import",
    b"individual",
    b"interface",
    b"item",
    b"library",
    b"metadata",
    b"namespace",
    b"occurrence",
    b"package",
    b"part",
    b"port",
    b"private",
    b"protected",
    b"public",
    b"render",
    b"rendering",
    b"rep",
    b"require",
    b"requirement",
    b"satisfy",
    b"state",
    b"snapshot",
    b"timeslice",
    b"use",
    b"variation",
    b"verification",
    b"view",
    b"viewpoint",
];

pub(crate) const PART_BODY_STARTERS: &[&[u8]] = &[
    b"#",
    b"@",
    b"abstract",
    b"allocate",
    b"assert",
    b"attribute",
    b"bind",
    b"calc",
    b"comment",
    b"connection",
    b"connect",
    b"doc",
    b"enum",
    b"exhibit",
    b"flow",
    b"individual",
    b"message",
    b"interface",
    b"item",
    b"occurrence",
    b"part",
    b"perform",
    b"port",
    b"private",
    b"protected",
    b"public",
    b"ref",
    b"requirement",
    b"satisfy",
    b"succession",
    b"snapshot",
    b"timeslice",
    b"variant",
    b"variation",
];

pub(crate) const PORT_DEF_BODY_STARTERS: &[&[u8]] =
    &[b"doc", b"attribute", b"port", b"in", b"out", b"inout", b"ref", b"abstract"];

pub(crate) const PORT_BODY_STARTERS: &[&[u8]] = &[b"doc", b"port", b"in", b"out", b"inout"];

pub(crate) const REQUIREMENT_BODY_STARTERS: &[&[u8]] = &[
    b"#",
    b"@",
    b"attribute",
    b"doc",
    b"frame",
    b"import",
    b"rep",
    b"require",
    b"requirement",
    b"satisfy",
    b"subject",
    b"actor",
    b"stakeholder",
    b"purpose",
    b"verify",
];

#[allow(dead_code)]
pub(crate) const STATE_BODY_STARTERS: &[&[u8]] = &[
    b"#",
    b"@",
    b"doc",
    b"do",
    b"entry",
    b"exit",
    b"final",
    b"ref",
    b"state",
    b"then",
    b"transition",
];

pub(crate) const USE_CASE_BODY_STARTERS: &[&[u8]] = &[
    b"@",
    b"abstract",
    b"action",
    b"actor",
    b"assign",
    b"attribute",
    b"bind",
    b"case",
    b"calc",
    b"doc",
    b"first",
    b"flow",
    b"for",
    b"include",
    b"in",
    b"objective",
    b"out",
    b"perform",
    b"private",
    b"protected",
    b"ref",
    b"return",
    b"public",
    b"state",
    b"subject",
    b"then",
];

pub(crate) const CONSTRAINT_DEF_BODY_STARTERS: &[&[u8]] = &[b"doc", b"@", b"in", b"out", b"inout"];

pub(crate) const CALC_DEF_BODY_STARTERS: &[&[u8]] = &[b"@", b"doc", b"in", b"out", b"inout", b"return"];

pub(crate) const VIEW_DEF_BODY_STARTERS: &[&[u8]] =
    &[b"@", b"doc", b"filter", b"render", b"ref", b"abstract"];

pub(crate) const VIEW_BODY_STARTERS: &[&[u8]] =
    &[b"doc", b"expose", b"filter", b"render", b"satisfy"];

pub(crate) const CONNECTION_DEF_BODY_STARTERS: &[&[u8]] = &[b"connect", b"end", b"ref", b"doc"];

/// Skip optional whitespace (space, tab, newline).
pub(crate) fn ws(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) =
        take_while(|c: u8| c == b' ' || c == b'\t' || c == b'\n' || c == b'\r').parse(input)?;
    Ok((input, ()))
}

/// Skip whitespace and comments (block, single-line). Use between tokens and at body boundaries.
/// Does NOT consume "doc /* ... */" — that is a body element (PackageBodyElement::Doc etc.) and must
/// be parsed explicitly so it appears in the AST. //* ... */ is tried before line_comment so that
/// "//*" starts a block comment, not a line comment.
pub(crate) fn ws_and_comments(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let mut input = input;
    loop {
        let start = input.location_offset();
        let (next, _) =
            take_while(|c: u8| c == b' ' || c == b'\t' || c == b'\n' || c == b'\r').parse(input)?;
        input = next;
        let (next, _) =
            many0(alt((block_comment, block_comment_slash_star, line_comment))).parse(input)?;
        input = next;
        if input.location_offset() == start {
            return Ok((input, ()));
        }
    }
}

/// Block comment: /* ... */
fn block_comment(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) = tag(&b"/*"[..]).parse(input)?;
    let (input, _) = take_until(&b"*/"[..]).parse(input)?;
    let (input, _) = tag(&b"*/"[..]).parse(input)?;
    let (input, _) = ws(input)?;
    Ok((input, ()))
}

/// Block comment starting with //* ... */ (e.g. in 4a fixture).
fn block_comment_slash_star(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) = tag(&b"//*"[..]).parse(input)?;
    let (input, _) = take_until(&b"*/"[..]).parse(input)?;
    let (input, _) = tag(&b"*/"[..]).parse(input)?;
    let (input, _) = ws(input)?;
    Ok((input, ()))
}

/// Single-line comment: // to EOL (consumes the newline).
fn line_comment(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) = tag(&b"//"[..]).parse(input)?;
    let (input, _) = take_while(|c: u8| c != b'\n' && c != b'\r').parse(input)?;
    let (input, _) = take_while(|c: u8| c == b'\n' || c == b'\r').parse(input)?;
    Ok((input, ()))
}

/// Parse one or more whitespace characters (consumes at least one).
pub(crate) fn ws1(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) =
        take_while1(|c: u8| c == b' ' || c == b'\t' || c == b'\n' || c == b'\r').parse(input)?;
    Ok((input, ()))
}

/// Skip to the next sync point (next line start after newline and ws/comments), or to end of input.
/// Used for error recovery so parsing can continue after a failed top-level element.
pub(crate) fn skip_to_next_sync_point(input: Input<'_>) -> IResult<Input<'_>, ()> {
    alt((
        map(
            (
                take_until(&b"\n"[..]),
                opt(tag(&b"\n"[..])),
                ws_and_comments,
            ),
            |_| (),
        ),
        value((), rest),
    ))
    .parse(input)
}

/// Skip to the next root-level package or namespace (next line starting with "package " or "namespace "
/// after ws/comments), or to end of input. Used when recovery from a failure inside a package body.
/// Skip to the next root-level package or namespace, or to end of input.
/// Used when recovering from a failure inside a package body (avoids reporting errors on every line).
#[allow(dead_code)]
pub(crate) fn skip_to_next_root_element(mut input: Input<'_>) -> IResult<Input<'_>, ()> {
    loop {
        if input.fragment().is_empty() {
            return Ok((input, ()));
        }
        let (after_ws, _) = ws_and_comments(input).unwrap_or((input, ()));
        let frag = after_ws.fragment();
        if frag.len() >= 8 && (frag.starts_with(b"package ") || frag.starts_with(b"namespace ")) {
            return Ok((after_ws, ()));
        }
        match skip_to_next_sync_point(input) {
            Ok((rest, _)) => input = rest,
            Err(_) => return Ok((input, ())),
        }
    }
}

pub(crate) fn starts_with_keyword(fragment: &[u8], keyword: &[u8]) -> bool {
    if keyword
        .iter()
        .any(|b| !b.is_ascii_alphanumeric() && *b != b'_')
    {
        return fragment.starts_with(keyword);
    }
    fragment.starts_with(keyword)
        && fragment
            .get(keyword.len())
            .is_none_or(|b| b.is_ascii_whitespace() || matches!(*b, b'{' | b':' | b';' | b'['))
}

pub(crate) fn starts_with_any_keyword(fragment: &[u8], keywords: &[&[u8]]) -> bool {
    keywords
        .iter()
        .any(|keyword| starts_with_keyword(fragment, keyword))
}

/// Count `{` / `}` outside comments (line and block) for EOF brace balance checks.
pub(crate) fn brace_balance_outside_comments(bytes: &[u8]) -> (usize, usize) {
    let mut opens = 0usize;
    let mut closes = 0usize;
    let mut pos = 0usize;
    while pos < bytes.len() {
        if pos + 2 <= bytes.len() && bytes[pos..].starts_with(b"/*") {
            if let Some(rel) = find_subslice(&bytes[pos..], b"*/") {
                pos += rel + 2;
                continue;
            }
            break;
        }
        if pos + 2 <= bytes.len() && bytes[pos..].starts_with(b"//") {
            while pos < bytes.len() && bytes[pos] != b'\n' && bytes[pos] != b'\r' {
                pos += 1;
            }
            continue;
        }
        match bytes[pos] {
            b'{' => opens += 1,
            b'}' => closes += 1,
            _ => {}
        }
        pos += 1;
    }
    (opens, closes)
}

fn balanced_inline_depth(fragment: &[u8], pos: usize, brace_depth: &mut usize) -> Option<usize> {
    if pos >= fragment.len() {
        return None;
    }
    match fragment[pos] {
        b'{' => {
            *brace_depth += 1;
            Some(pos + 1)
        }
        b'}' => {
            if *brace_depth == 0 {
                None
            } else {
                *brace_depth -= 1;
                Some(pos + 1)
            }
        }
        _ => Some(pos + 1),
    }
}

fn local_recovery_line_boundary<'a>(input: Input<'a>, starters: &[&[u8]]) -> Option<Input<'a>> {
    let (input, _) = ws_and_comments(input).ok()?;
    let fragment = input.fragment();
    if fragment.is_empty() {
        return Some(input);
    }

    let mut pos = 0usize;
    let mut brace_depth = 0usize;
    while pos < fragment.len() {
        if pos + 2 <= fragment.len() && fragment[pos..].starts_with(b"/*") {
            if let Some(rel) = find_subslice(&fragment[pos..], b"*/") {
                pos += rel + 2;
                continue;
            }
            return None;
        }
        if pos + 2 <= fragment.len() && fragment[pos..].starts_with(b"//") {
            while pos < fragment.len() && fragment[pos] != b'\n' && fragment[pos] != b'\r' {
                pos += 1;
            }
            continue;
        }

        if pos < fragment.len()
            && (fragment[pos] == b'\n' || fragment[pos] == b'\r')
            && brace_depth == 0
        {
            let newline_start = pos;
            while pos < fragment.len() && (fragment[pos] == b'\n' || fragment[pos] == b'\r') {
                pos += 1;
            }
            let (candidate, _) =
                nom::bytes::complete::take::<_, _, nom::error::Error<Input<'a>>>(pos)
                    .parse(input)
                    .ok()?;
            let (candidate, _) = ws_and_comments(candidate).unwrap_or((candidate, ()));
            if (candidate.fragment().is_empty()
                || candidate.fragment().starts_with(b"}")
                || starts_with_any_keyword(candidate.fragment(), starters))
                && newline_start > 0
            {
                return Some(candidate);
            }
            continue;
        }

        if let Some(next_pos) = balanced_inline_depth(fragment, pos, &mut brace_depth) {
            pos = next_pos;
        } else {
            break;
        }
    }

    None
}

/// Skip to the next likely body element starter for the current grammar scope, or to the closing `}` / EOF.
pub(crate) fn skip_to_next_body_element_or_end<'a>(
    mut input: Input<'a>,
    starters: &[&[u8]],
) -> IResult<Input<'a>, ()> {
    loop {
        let (after_ws, _) = ws_and_comments(input).unwrap_or((input, ()));
        input = after_ws;
        if input.fragment().is_empty()
            || input.fragment().starts_with(b"}")
            || starts_with_any_keyword(input.fragment(), starters)
        {
            return Ok((input, ()));
        }
        match skip_to_next_sync_point(input) {
            Ok((rest, _)) if rest.location_offset() != input.location_offset() => input = rest,
            _ => return Ok((input, ())),
        }
    }
}

/// Recover from a failed body element parse by first skipping the current statement or block,
/// then syncing to the next likely body element starter or `}`.
pub(crate) fn recover_body_element<'a>(
    input: Input<'a>,
    starters: &[&[u8]],
) -> IResult<Input<'a>, ()> {
    if let Some(next) = local_recovery_line_boundary(input, starters) {
        if next.location_offset() != input.location_offset() {
            return Ok((next, ()));
        }
    }
    let (input, _) = skip_statement_or_block(input)?;
    skip_to_next_body_element_or_end(input, starters)
}

/// Capture an opaque body member as a `String` without producing an error node.
///
/// Checks that the input starts with one of `keywords`, then consumes the entire statement
/// (up to `;`) or block (`{ … }`) and returns the captured text. Use this as a last-resort
/// parser before error recovery to handle syntax forms we recognise but don't fully model.
pub(crate) fn capture_opaque_member<'a>(
    input: Input<'a>,
    keywords: &[&[u8]],
) -> IResult<Input<'a>, String> {
    let (input, _) = ws_and_comments(input)?;
    if !starts_with_any_keyword(input.fragment(), keywords) {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let start = input;
    let (input, _) = skip_statement_or_block(input)?;
    let len = input.location_offset() - start.location_offset();
    let text = String::from_utf8_lossy(&start.fragment()[..len])
        .trim()
        .to_string();
    Ok((input, text))
}

/// NAME: BASIC_NAME (identifier) or UNRESTRICTED_NAME (single-quoted string).
pub(crate) fn name(input: Input<'_>) -> IResult<Input<'_>, String> {
    alt((quoted_name, basic_name)).parse(input)
}

/// Unquoted identifier: letter or underscore, then alphanumeric or underscore.
fn basic_name(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, raw) = take_while1(|c: u8| c.is_ascii_alphanumeric() || c == b'_').parse(input)?;
    let s = String::from_utf8_lossy(raw.fragment()).into_owned();
    Ok((input, s))
}

/// Quoted name: '...' (content between single quotes; \' for escape).
fn quoted_name(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = tag(&b"'"[..]).parse(input)?;
    let frag = input.fragment();
    let mut count = 0usize;
    let mut bytes = Vec::new();
    while count < frag.len() {
        if frag[count] == b'\\' && count + 1 < frag.len() && frag[count + 1] == b'\'' {
            bytes.push(b'\'');
            count += 2;
        } else if frag[count] == b'\'' {
            count += 1;
            break;
        } else {
            bytes.push(frag[count]);
            count += 1;
        }
    }
    let s = String::from_utf8_lossy(&bytes).into_owned();
    let (input, _) = nom::bytes::complete::take(count).parse(input)?;
    Ok((input, s))
}

/// QualifiedName: ( '$' '::' )? ( NAME '::' )* NAME. Returns string like "SI::kg" or "ISQ::mass".
pub(crate) fn qualified_name(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = ws_and_comments(input)?;
    let (input, opt_dollar) = opt(tag(&b"$"[..])).parse(input)?;
    let (input, _) = opt(preceded(tag(&b"::"[..]), ws_and_comments)).parse(input)?;
    let (input, first) = name(input)?;
    let (input, rest_segments) = many0(preceded(
        preceded(ws_and_comments, tag(&b"::"[..])),
        preceded(ws_and_comments, name),
    ))
    .parse(input)?;
    let mut segments = Vec::new();
    if opt_dollar.is_some() {
        segments.push("$".to_string());
    }
    segments.push(first);
    segments.extend(rest_segments);
    let s = segments.join("::");
    Ok((input, s))
}

/// Like [`qualified_name`], but returns the individual `::`-separated segments (with separator
/// markers) instead of joining them into one opaque string. Used to build
/// [`crate::ast::RelationshipTarget`]s for typing/subclassification/subsetting targets, which need
/// to keep `::`-qualification distinguishable from `.`-feature-chaining (see
/// `crate::ast::relationship_target` module docs).
pub(crate) fn qualified_name_segments(
    input: Input<'_>,
) -> IResult<Input<'_>, Vec<crate::ast::RelationshipTargetSegment>> {
    use crate::ast::{RelationshipTargetSegment, SegmentSeparator};

    let (input, _) = ws_and_comments(input)?;
    let (input, opt_dollar) = opt(tag(&b"$"[..])).parse(input)?;
    let (input, _) = opt(preceded(tag(&b"::"[..]), ws_and_comments)).parse(input)?;
    let (input, first) = name(input)?;
    let (input, rest_segments) = many0(preceded(
        preceded(ws_and_comments, tag(&b"::"[..])),
        preceded(ws_and_comments, name),
    ))
    .parse(input)?;
    let mut segments = Vec::new();
    if opt_dollar.is_some() {
        segments.push(RelationshipTargetSegment {
            name: "$".to_string(),
            separator: None,
        });
        segments.push(RelationshipTargetSegment {
            name: first,
            separator: Some(SegmentSeparator::ColonColon),
        });
    } else {
        segments.push(RelationshipTargetSegment {
            name: first,
            separator: None,
        });
    }
    segments.extend(rest_segments.into_iter().map(|name| RelationshipTargetSegment {
        name,
        separator: Some(SegmentSeparator::ColonColon),
    }));
    Ok((input, segments))
}

/// Skip any content until we see '}' at the same brace level (tracks nesting, skips comments).
pub(crate) fn skip_until_brace_end(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let frag = input.fragment();
    let mut depth = 1u32;
    let mut pos = 0usize;
    while depth > 0 && pos < frag.len() {
        if pos + 2 <= frag.len() && frag[pos..].starts_with(b"/*") {
            if let Some(rel) = find_subslice(&frag[pos..], b"*/") {
                pos += rel + 2;
                continue;
            }
            break;
        }
        if pos + 2 <= frag.len() && frag[pos..].starts_with(b"//") {
            let mut j = pos + 2;
            while j < frag.len() && frag[j] != b'\n' && frag[j] != b'\r' {
                j += 1;
            }
            while j < frag.len() && (frag[j] == b'\n' || frag[j] == b'\r') {
                j += 1;
            }
            pos = j;
            continue;
        }
        if frag[pos] == b'{' {
            depth += 1;
        } else if frag[pos] == b'}' {
            depth -= 1;
            if depth == 0 {
                break;
            }
        }
        pos += 1;
    }
    let (input, _) = nom::bytes::complete::take(pos).parse(input)?;
    Ok((input, ()))
}

pub(crate) fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

const DECLARATION_BOUNDARY_STARTERS: &[&[u8]] = &[
    b"package",
    b"namespace",
    b"import",
    b"part",
    b"attribute",
    b"action",
    b"requirement",
    b"state",
    b"transition",
    b"view",
    b"viewpoint",
    b"rendering",
    b"constraint",
    b"calc",
    b"ref",
    b"port",
    b"perform",
    b"bind",
    b"flow",
    b"first",
    b"merge",
    b"then",
    b"in",
    b"out",
    b"inout",
    b"return",
    b"actor",
    b"subject",
    b"objective",
    b"require",
    b"satisfy",
    b"expose",
    b"doc",
];

fn trim_ascii_end_bytes(mut fragment: &[u8]) -> &[u8] {
    while let Some(last) = fragment.last() {
        if last.is_ascii_whitespace() {
            fragment = &fragment[..fragment.len() - 1];
        } else {
            break;
        }
    }
    fragment
}

fn starts_new_declaration_after_newline(fragment: &[u8], newline_end: usize) -> bool {
    let mut pos = newline_end;
    while pos < fragment.len() && matches!(fragment[pos], b' ' | b'\t' | b'\n' | b'\r') {
        pos += 1;
    }
    let candidate = &fragment[pos..];
    candidate.is_empty()
        || candidate.starts_with(b"}")
        || starts_with_any_keyword(candidate, DECLARATION_BOUNDARY_STARTERS)
}

/// Identification: ( '<' ShortName '>' )? ( Name )?
pub(crate) fn identification(input: Input<'_>) -> IResult<Input<'_>, Identification> {
    let (input, short_name) = opt(delimited(
        preceded(ws_and_comments, tag(&b"<"[..])),
        preceded(ws_and_comments, name),
        preceded(ws_and_comments, tag(&b">"[..])),
    ))
    .parse(input)?;
    let (input, decl_name) = opt(preceded(ws_and_comments, name)).parse(input)?;
    Ok((
        input,
        Identification {
            short_name,
            name: decl_name,
        },
    ))
}

/// Optional `private` / `protected` / `public` visibility prefix, shared by every `*Def`/`*Usage`
/// parser that needs to feed a [`crate::ast::Membership`] (parser work item 4b, post-PAR-006).
/// Returns the span of the whole prefix (zero-width, positioned at `input`, when no prefix is
/// written) alongside the matched [`crate::ast::Visibility`], so callers can build a `Membership`
/// without re-deriving the span themselves. This consolidates what several parsers (`attribute_def`,
/// `attribute_usage`, `filter_member`, `import`, ...) previously matched ad hoc with their own
/// inline `alt((tag("private"), ...))`; matching and discarding the value is what left every
/// `*Def`/`*Usage` struct with no visibility field before this item.
pub(crate) fn visibility_prefix(
    input: Input<'_>,
) -> IResult<Input<'_>, (crate::ast::Span, Option<crate::ast::Visibility>)> {
    let start = input;
    let (input, visibility) = opt(alt((
        map(preceded(tag(&b"private"[..]), ws1), |_| {
            crate::ast::Visibility::Private
        }),
        map(preceded(tag(&b"protected"[..]), ws1), |_| {
            crate::ast::Visibility::Protected
        }),
        map(preceded(tag(&b"public"[..]), ws1), |_| {
            crate::ast::Visibility::Public
        }),
    )))
    .parse(input)?;
    let span = crate::parser::span_from_to(start, input);
    Ok((input, (span, visibility)))
}

/// Take input until we hit one of the terminator bytes (e.g. '{' or ';'), return as string (trimmed).
pub(crate) fn take_until_terminator<'a>(
    input: Input<'a>,
    terminators: &'a [u8],
) -> IResult<Input<'a>, String> {
    let frag = input.fragment();
    let mut i = 0;
    while i < frag.len() {
        if terminators.contains(&frag[i]) {
            let s = String::from_utf8_lossy(&frag[..i]).trim().to_string();
            let (input, _) = nom::bytes::complete::take(i).parse(input)?;
            return Ok((input, s));
        }
        if terminators.contains(&b';') && matches!(frag[i], b'\n' | b'\r') {
            let mut newline_end = i;
            while newline_end < frag.len() && matches!(frag[newline_end], b'\n' | b'\r') {
                newline_end += 1;
            }
            let consumed = trim_ascii_end_bytes(&frag[..i]);
            let consumed_ends_incomplete = consumed.last().is_some_and(|b| {
                matches!(
                    *b,
                    b':' | b'=' | b',' | b'.' | b'+' | b'-' | b'*' | b'/' | b'>' | b'<' | b'|'
                )
            });
            if !consumed.is_empty()
                && !consumed_ends_incomplete
                && starts_new_declaration_after_newline(frag, newline_end)
            {
                let s = String::from_utf8_lossy(&frag[..i]).trim().to_string();
                let (input, _) = nom::bytes::complete::take(i).parse(input)?;
                return Ok((input, s));
            }
        }
        if frag[i] == b'/' && i + 1 < frag.len() && (frag[i + 1] == b'*' || frag[i + 1] == b'/') {
            break;
        }
        i += 1;
    }
    let s = String::from_utf8_lossy(&frag[..i]).trim().to_string();
    let (input, _) = nom::bytes::complete::take(i).parse(input)?;
    Ok((input, s))
}

/// Skip one unknown statement or balanced block.
///
/// This is used as a recovery mechanism inside body parsers so we can continue
/// parsing later known elements instead of aborting the entire enclosing body.
pub(crate) fn skip_statement_or_block(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) = ws_and_comments(input)?;
    let frag = input.fragment();
    if frag.is_empty() {
        return Ok((input, ()));
    }
    if frag[0] == b'}' {
        return Ok((input, ()));
    }
    if frag[0] == b'{' {
        let (input, _) = tag(&b"{"[..]).parse(input)?;
        let (input, _) = skip_until_brace_end(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b"}"[..])).parse(input)?;
        return Ok((input, ()));
    }

    let mut depth = 0usize;
    let mut pos = 0usize;
    while pos < frag.len() {
        if pos + 2 <= frag.len() && frag[pos..].starts_with(b"/*") {
            if let Some(rel) = find_subslice(&frag[pos..], b"*/") {
                pos += rel + 2;
                continue;
            }
            pos = frag.len();
            break;
        }
        if pos + 2 <= frag.len() && frag[pos..].starts_with(b"//") {
            while pos < frag.len() && frag[pos] != b'\n' && frag[pos] != b'\r' {
                pos += 1;
            }
            while pos < frag.len() && (frag[pos] == b'\n' || frag[pos] == b'\r') {
                pos += 1;
            }
            if depth == 0 {
                break;
            }
            continue;
        }
        match frag[pos] {
            b'{' => depth += 1,
            b'}' => {
                if depth == 0 {
                    break;
                }
                depth -= 1;
                if depth == 0 {
                    pos += 1;
                    break;
                }
            }
            b';' if depth == 0 => {
                pos += 1;
                break;
            }
            _ => {}
        }
        pos += 1;
    }
    let advance = pos.max(1).min(frag.len());
    let (input, _) = nom::bytes::complete::take(advance).parse(input)?;
    Ok((input, ()))
}

/// Parse specialization marker in SysML concrete syntax:
/// either symbolic `:>` or keyword `specializes`.
pub(crate) fn specialization_operator(input: Input<'_>) -> IResult<Input<'_>, ()> {
    alt((
        value((), tag(&b":>"[..])),
        value((), terminated(tag(&b"specializes"[..]), ws1)),
    ))
    .parse(input)
}

/// Parse subsetting marker in SysML concrete syntax:
/// either symbolic `:>` or keyword `subsets`.
pub(crate) fn subset_operator(input: Input<'_>) -> IResult<Input<'_>, ()> {
    alt((
        value((), tag(&b":>"[..])),
        value((), terminated(tag(&b"subsets"[..]), ws1)),
    ))
    .parse(input)
}

/// Parse redefinition marker in SysML concrete syntax:
/// either symbolic `:>>` or keyword `redefines`.
pub(crate) fn redefine_operator(input: Input<'_>) -> IResult<Input<'_>, ()> {
    alt((
        value((), tag(&b":>>"[..])),
        value((), terminated(tag(&b"redefines"[..]), ws1)),
    ))
    .parse(input)
}

/// Parse typing marker in SysML concrete syntax:
/// symbolic `:`, or keyword pairs `defined by` / `typed by`.
pub(crate) fn typed_by_operator(input: Input<'_>) -> IResult<Input<'_>, ()> {
    alt((
        value((), tag(&b":"[..])),
        value((), (tag(&b"defined"[..]), ws1, tag(&b"by"[..]), ws1)),
        value((), (tag(&b"typed"[..]), ws1, tag(&b"by"[..]), ws1)),
    ))
    .parse(input)
}

/// Reference subsetting: `::>` or keyword `references`.
pub(crate) fn references_operator(input: Input<'_>) -> IResult<Input<'_>, ()> {
    alt((
        value((), tag(&b"::>"[..])),
        value((), (tag(&b"references"[..]), ws1)),
    ))
    .parse(input)
}

/// Cross subsetting: `=>` or keyword `crosses`.
pub(crate) fn crosses_operator(input: Input<'_>) -> IResult<Input<'_>, ()> {
    alt((
        value((), tag(&b"=>"[..])),
        value((), (tag(&b"crosses"[..]), ws1)),
    ))
    .parse(input)
}

/// Conjugation: `~` prefix on types or `conjugates` keyword form.
#[allow(dead_code)]
pub(crate) fn conjugates_operator(input: Input<'_>) -> IResult<Input<'_>, ()> {
    alt((
        value((), tag(&b"~"[..])),
        value((), (tag(&b"conjugates"[..]), ws1)),
    ))
    .parse(input)
}

/// DECIMAL_VALUE: integer or real literal text (for BNF DECIMAL_VALUE / EXPONENTIAL_VALUE).
#[allow(dead_code)] // used by BNF lexical conformance tests in `bnf_surface`
pub(crate) fn decimal_value_text(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = ws_and_comments(input)?;
    let (input, raw) = take_while1(|c: u8| {
        c.is_ascii_digit() || c == b'.' || c == b'e' || c == b'E' || c == b'+' || c == b'-'
    })
    .parse(input)?;
    Ok((input, String::from_utf8_lossy(raw.fragment()).into_owned()))
}

/// STRING_VALUE: single-quoted unrestricted name content.
#[allow(dead_code)] // used by BNF lexical conformance tests in `bnf_surface`
pub(crate) fn string_value(input: Input<'_>) -> IResult<Input<'_>, String> {
    quoted_name(input)
}

#[cfg(test)]
mod lexical_bnf_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn span_input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    #[test]
    fn name_parses_basic_name() {
        let (_, n) = name(span_input("myPart")).expect("NAME");
        assert_eq!(n, "myPart");
    }

    #[test]
    fn name_parses_unrestricted_name() {
        let (_, n) = name(span_input("'a name'")).expect("UNRESTRICTED_NAME");
        assert_eq!(n, "a name");
    }

    #[test]
    fn name_parses_unrestricted_name_with_degree_symbol() {
        let (_, n) = name(span_input("'\u{00b0}F'")).expect("UNRESTRICTED_NAME");
        assert_eq!(n, "\u{00b0}F");
    }

    #[test]
    fn qualified_name_parses_scoped_name() {
        let (_, q) = qualified_name(span_input("SI::kg")).expect("QualifiedName");
        assert_eq!(q, "SI::kg");
    }

    #[test]
    fn string_value_parses_quoted() {
        let (_, s) = string_value(span_input("'x'")).expect("STRING_VALUE");
        assert_eq!(s, "x");
    }

    #[test]
    fn decimal_value_parses_real() {
        let (_, v) = decimal_value_text(span_input("1.5e-3")).expect("DECIMAL_VALUE");
        assert_eq!(v, "1.5e-3");
    }

    #[test]
    fn ws_and_comments_skip_line_and_block() {
        let input = span_input("  // line\n  /* block */  part");
        let (rest, _) = ws_and_comments(input).expect("WHITE_SPACE");
        assert!(rest.fragment().starts_with(b"part"));
    }

    #[test]
    fn references_operator_accepts_symbol_and_keyword() {
        let (_, _) = references_operator(span_input("::>")).expect("REFERENCES");
        let (_, _) = references_operator(span_input("references ")).expect("REFERENCES");
    }

    #[test]
    fn crosses_operator_accepts_symbol_and_keyword() {
        let (_, _) = crosses_operator(span_input("=>")).expect("CROSSES");
        let (_, _) = crosses_operator(span_input("crosses ")).expect("CROSSES");
    }
}
