//! Lexer and skip helpers: whitespace, comments, names, qualified names, and body-skip utilities.

use crate::ast::Identification;
use nom::branch::alt;
use nom::sequence::delimited;
use nom::bytes::complete::{tag, take_until, take_while, take_while1};
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

/// Skip optional whitespace (space, tab, newline).
pub(crate) fn ws(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = take_while(|c: u8| c == b' ' || c == b'\t' || c == b'\n' || c == b'\r')(input)?;
    Ok((input, ()))
}

/// Skip whitespace and comments (block, single-line, doc+block). Use between tokens and at body boundaries.
pub(crate) fn ws_and_comments(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = take_while(|c: u8| c == b' ' || c == b'\t' || c == b'\n' || c == b'\r')(input)?;
    let (input, _) = many0(alt((
        block_comment,
        line_comment,
        doc_then_block_comment,
    )))(input)?;
    Ok((input, ()))
}

/// Block comment: /* ... */
fn block_comment(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag("/*")(input)?;
    let (input, _) = take_until("*/")(input)?;
    let (input, _) = tag("*/")(input)?;
    let (input, _) = ws(input)?;
    Ok((input, ()))
}

/// Single-line comment: // to EOL (consumes the newline).
fn line_comment(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag("//")(input)?;
    let (input, _) = take_while(|c: u8| c != b'\n' && c != b'\r')(input)?;
    let (input, _) = take_while(|c: u8| c == b'\n' || c == b'\r')(input)?;
    Ok((input, ()))
}

/// Doc keyword followed by optional whitespace and a block comment.
fn doc_then_block_comment(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag("doc")(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag("/*")(input)?;
    let (input, _) = take_until("*/")(input)?;
    let (input, _) = tag("*/")(input)?;
    let (input, _) = ws(input)?;
    Ok((input, ()))
}

/// Parse one or more whitespace characters (consumes at least one).
pub(crate) fn ws1(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = take_while1(|c: u8| c == b' ' || c == b'\t' || c == b'\n' || c == b'\r')(input)?;
    Ok((input, ()))
}

/// NAME: BASIC_NAME (identifier) or UNRESTRICTED_NAME (single-quoted string).
pub(crate) fn name(input: &[u8]) -> IResult<&[u8], String> {
    alt((quoted_name, basic_name))(input)
}

/// Unquoted identifier: letter or underscore, then alphanumeric or underscore.
fn basic_name(input: &[u8]) -> IResult<&[u8], String> {
    let (input, raw) = take_while1(|c: u8| c.is_ascii_alphanumeric() || c == b'_')(input)?;
    let s = String::from_utf8_lossy(raw).into_owned();
    Ok((input, s))
}

/// Quoted name: '...' (content between single quotes; \' for escape).
fn quoted_name(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = tag("'")(input)?;
    let mut s = String::new();
    let mut i = input;
    while !i.is_empty() {
        if i.starts_with(b"\\'") {
            s.push('\'');
            i = &i[2..];
        } else if i[0] == b'\'' {
            i = &i[1..];
            break;
        } else {
            s.push(i[0] as char);
            i = &i[1..];
        }
    }
    Ok((i, s))
}

/// QualifiedName: ( '$' '::' )? ( NAME '::' )* NAME. Returns string like "SI::kg" or "ISQ::mass".
pub(crate) fn qualified_name(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = ws_and_comments(input)?;
    let (input, opt_dollar) = opt(tag("$"))(input)?;
    let (input, _) = opt(preceded(tag("::"), ws_and_comments))(input)?;
    let (input, first) = name(input)?;
    let (input, rest_segments) = many0(preceded(
        preceded(ws_and_comments, tag("::")),
        preceded(ws_and_comments, name),
    ))(input)?;
    let mut segments = Vec::new();
    if opt_dollar.is_some() {
        segments.push("$".to_string());
    }
    segments.push(first);
    segments.extend(rest_segments);
    let s = segments.join("::");
    Ok((input, s))
}

/// Skip any content until we see '}' at the same brace level (tracks nesting, skips comments).
pub(crate) fn skip_until_brace_end(input: &[u8]) -> IResult<&[u8], ()> {
    let mut depth = 1u32;
    let mut i = input;
    while depth > 0 && !i.is_empty() {
        if i.starts_with(b"/*") {
            if let Some(pos) = find_subslice(i, b"*/") {
                i = &i[pos + 2..];
                continue;
            }
            break;
        }
        if i.starts_with(b"//") {
            let mut j = 2;
            while j < i.len() && i[j] != b'\n' && i[j] != b'\r' {
                j += 1;
            }
            while j < i.len() && (i[j] == b'\n' || i[j] == b'\r') {
                j += 1;
            }
            i = &i[j..];
            continue;
        }
        if i[0] == b'{' {
            depth += 1;
        } else if i[0] == b'}' {
            depth -= 1;
            if depth == 0 {
                break;
            }
        }
        i = &i[1..];
    }
    Ok((i, ()))
}

pub(crate) fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

/// Identification: ( '<' ShortName '>' )? ( Name )?
pub(crate) fn identification(input: &[u8]) -> IResult<&[u8], Identification> {
    let (input, short_name) = opt(delimited(
        preceded(ws_and_comments, tag("<")),
        preceded(ws_and_comments, name),
        preceded(ws_and_comments, tag(">")),
    ))(input)?;
    let (input, decl_name) = opt(preceded(ws_and_comments, name))(input)?;
    Ok((
        input,
        Identification {
            short_name,
            name: decl_name,
        },
    ))
}

/// Take input until we hit one of the terminator bytes (e.g. '{' or ';'), return as string (trimmed).
pub(crate) fn take_until_terminator<'a>(input: &'a [u8], terminators: &[u8]) -> IResult<&'a [u8], String> {
    let mut i = 0;
    while i < input.len() {
        if terminators.contains(&input[i]) {
            let s = String::from_utf8_lossy(&input[..i]).trim().to_string();
            return Ok((&input[i..], s));
        }
        if input[i] == b'/' && i + 1 < input.len() && (input[i + 1] == b'*' || input[i + 1] == b'/') {
            break;
        }
        i += 1;
    }
    let s = String::from_utf8_lossy(&input[..i]).trim().to_string();
    Ok((&input[i..], s))
}
