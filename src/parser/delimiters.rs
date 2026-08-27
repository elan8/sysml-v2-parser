//! One owning lexical prescan of a document's brace structure.
//!
//! Brace balance and structural nesting depth are facts about the whole document that several
//! diagnostics need (`nesting_too_deep`, `missing_closing_brace`, `unexpected_closing_brace`).
//! They are derived here exactly once per parse, by a scanner that recognizes comments and quoted
//! text the same way [`super::lex`] does, so no consumer re-derives them with its own copy of the
//! lexical rules.

use super::parse::MAX_SYNTAX_NESTING;

/// Bytes that can begin a delimiter, comment, or quoted text: everything the scanner must inspect.
const INTERESTING: [bool; 256] = {
    let mut table = [false; 256];
    table[b'/' as usize] = true;
    table[b'\'' as usize] = true;
    table[b'"' as usize] = true;
    table[b'{' as usize] = true;
    table[b'}' as usize] = true;
    table
};

/// Whole-document brace structure outside comments and quoted text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DelimiterScan {
    opens: usize,
    closes: usize,
    last_close: Option<usize>,
    nesting_overflow: Option<usize>,
}

impl DelimiterScan {
    /// Scan `bytes` once, recording brace balance and the first structurally too-deep `{`.
    ///
    /// Comments and quoted text follow the lexer's rules: `//` runs to the next `\n` or `\r`,
    /// `/*` and `//*` run to the next `*/` without nesting, and `'`/`"` open quoted text in which
    /// `\` escapes the following byte.
    pub(crate) fn new(bytes: &[u8]) -> Self {
        let mut opens = 0usize;
        let mut closes = 0usize;
        let mut last_close = None;
        let mut depth = 0usize;
        let mut nesting_overflow = None;
        let mut pos = 0usize;

        while pos < bytes.len() {
            // Most of a document is neither a delimiter nor the start of a comment or quote;
            // skip those bytes with a table lookup rather than the match below.
            while let Some(&byte) = bytes.get(pos) {
                if INTERESTING[byte as usize] {
                    break;
                }
                pos += 1;
            }
            if pos >= bytes.len() {
                break;
            }
            let byte = bytes[pos];
            let next = bytes.get(pos + 1).copied();
            match (byte, next) {
                (b'/', Some(b'/')) if bytes.get(pos + 2).copied() == Some(b'*') => {
                    pos = skip_block_comment(bytes, pos + 3);
                }
                (b'/', Some(b'/')) => {
                    pos = skip_line_comment(bytes, pos + 2);
                }
                (b'/', Some(b'*')) => {
                    pos = skip_block_comment(bytes, pos + 2);
                }
                (b'\'' | b'"', _) => {
                    pos = skip_quoted(bytes, pos + 1, byte);
                }
                (b'{', _) => {
                    opens += 1;
                    depth += 1;
                    if depth > MAX_SYNTAX_NESTING && nesting_overflow.is_none() {
                        nesting_overflow = Some(pos);
                    }
                    pos += 1;
                }
                (b'}', _) => {
                    closes += 1;
                    last_close = Some(pos);
                    depth = depth.saturating_sub(1);
                    pos += 1;
                }
                _ => pos += 1,
            }
        }

        Self {
            opens,
            closes,
            last_close,
            nesting_overflow,
        }
    }

    /// Count of structural `{` and `}` in the document.
    pub(crate) fn balance(&self) -> (usize, usize) {
        (self.opens, self.closes)
    }

    /// True when the document opens more bodies than it closes.
    pub(crate) fn has_unclosed_brace(&self) -> bool {
        self.opens > self.closes
    }

    /// Byte offset of the last structural `}` in the document, if any.
    pub(crate) fn last_close(&self) -> Option<usize> {
        self.last_close
    }

    /// Byte offset of the first `{` that exceeds [`MAX_SYNTAX_NESTING`], if any.
    pub(crate) fn nesting_overflow(&self) -> Option<usize> {
        self.nesting_overflow
    }
}

/// Offset just past the line comment body starting at `pos` (the terminator is left unconsumed;
/// it is ordinary whitespace to every caller).
fn skip_line_comment(bytes: &[u8], pos: usize) -> usize {
    match memchr::memchr2(b'\n', b'\r', &bytes[pos..]) {
        Some(offset) => pos + offset,
        None => bytes.len(),
    }
}

/// Offset just past the closing `*/` of a block comment whose body starts at `pos`, or end of
/// input when the comment is unterminated.
fn skip_block_comment(bytes: &[u8], pos: usize) -> usize {
    let mut pos = pos;
    while let Some(offset) = memchr::memchr(b'*', &bytes[pos..]) {
        pos += offset + 1;
        if bytes.get(pos) == Some(&b'/') {
            return pos + 1;
        }
    }
    bytes.len()
}

/// Offset just past the closing `delimiter` of quoted text whose body starts at `pos`, or end of
/// input when the text is unterminated.
fn skip_quoted(bytes: &[u8], pos: usize, delimiter: u8) -> usize {
    let mut pos = pos;
    while let Some(offset) = bytes
        .get(pos..)
        .and_then(|rest| memchr::memchr2(b'\\', delimiter, rest))
    {
        let byte = bytes[pos + offset];
        pos += offset + 1;
        if byte == b'\\' {
            pos += 1;
            continue;
        }
        return pos;
    }
    bytes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_structural_braces_only() {
        let source = br#"package P { attribute a = "{{{"; /* } */ // }
        }"#;
        let scan = DelimiterScan::new(source);
        assert_eq!(scan.balance(), (1, 1));
        assert!(!scan.has_unclosed_brace());
        assert_eq!(scan.nesting_overflow(), None);
    }

    #[test]
    fn slash_slash_star_opens_a_block_comment() {
        // `//*` is a block comment to the lexer, so the `}` inside it is not structural and the
        // `{` after the terminator is.
        let scan = DelimiterScan::new(b"//* }\n } */ {");
        assert_eq!(scan.balance(), (1, 0));
    }

    #[test]
    fn block_comments_do_not_nest() {
        // The lexer closes a block comment at the first `*/`, so the trailing `{` is structural.
        let scan = DelimiterScan::new(b"/* /* */ {");
        assert_eq!(scan.balance(), (1, 0));
    }

    #[test]
    fn line_comment_ends_at_carriage_return() {
        let scan = DelimiterScan::new(b"// comment\r{");
        assert_eq!(scan.balance(), (1, 0));
    }

    #[test]
    fn unclosed_brace_is_reported() {
        let scan = DelimiterScan::new(b"package P { part p;");
        assert!(scan.has_unclosed_brace());
    }

    #[test]
    fn nesting_overflow_reports_first_offending_brace() {
        let source = "{".repeat(MAX_SYNTAX_NESTING + 1);
        let scan = DelimiterScan::new(source.as_bytes());
        assert_eq!(scan.nesting_overflow(), Some(MAX_SYNTAX_NESTING));

        let source = "{".repeat(MAX_SYNTAX_NESTING);
        assert_eq!(
            DelimiterScan::new(source.as_bytes()).nesting_overflow(),
            None
        );
    }

    #[test]
    fn escaped_quote_does_not_end_quoted_text() {
        let scan = DelimiterScan::new(br#""a\"}" {"#);
        assert_eq!(scan.balance(), (1, 0));
    }
}
