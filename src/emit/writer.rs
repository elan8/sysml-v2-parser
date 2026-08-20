//! Indenting SysML text writer.

use super::{EmitError, EmitOptions};
use crate::ast::{ParsedDocument, QualifiedReferenceId, ReferenceSeparator, Span};

pub(crate) struct EmitWriter<'a> {
    buf: String,
    depth: usize,
    opts: &'a EmitOptions,
    document: &'a ParsedDocument,
    at_line_start: bool,
}

impl<'a> EmitWriter<'a> {
    pub(crate) fn new(document: &'a ParsedDocument, opts: &'a EmitOptions) -> Self {
        Self {
            buf: String::new(),
            depth: 0,
            opts,
            document,
            at_line_start: true,
        }
    }

    pub(crate) fn emit_comments(&self) -> bool {
        self.opts.emit_comments
    }

    pub(crate) fn finish(self) -> String {
        self.buf
    }

    pub(crate) fn push_str(&mut self, s: &str) {
        if self.at_line_start && !s.is_empty() && s != "\n" {
            for _ in 0..self.depth * self.opts.indent {
                self.buf.push(' ');
            }
            self.at_line_start = false;
        }
        self.buf.push_str(s);
        if s.ends_with('\n') {
            self.at_line_start = true;
        }
    }

    /// Drop one trailing space when the declaration that would have consumed it was not authored.
    ///
    /// Prefix and kind keywords are written space-suffixed, and an anonymous usage
    /// (`ref individual :>> driver : Alice;`) writes no declaration label after them, so without
    /// this the next clause -- which supplies its own leading space -- doubles it. Reparsing is
    /// unaffected either way; what this fixes is emitted text an author would not have written.
    pub(crate) fn trim_trailing_space(&mut self) {
        if self.buf.ends_with(' ') {
            self.buf.pop();
        }
    }

    pub(crate) fn push_char(&mut self, c: char) {
        let mut tmp = [0u8; 4];
        self.push_str(c.encode_utf8(&mut tmp));
    }

    /// Re-emit the exact source captured by a resilient parser recovery node.
    pub(crate) fn push_recovery_span(&mut self, path: &str, span: &Span) -> Result<(), EmitError> {
        let text = self
            .document
            .source
            .slice(span)
            .ok_or_else(|| EmitError::Unsupported {
                path: path.to_owned(),
                construct: "recovery node has an invalid source span".to_owned(),
            })?;
        self.push_str(text.trim_end());
        Ok(())
    }

    /// Emit an authored name from its source span, applying the same quoting rules as an owned
    /// name would (`format_name`). Used by nodes that keep only a span for the declared name
    /// rather than copying its text, per the "authored spelling lives in source" contract.
    pub(crate) fn push_span_name(&mut self, path: &str, span: &Span) -> Result<(), EmitError> {
        let text = self
            .document
            .source
            .slice(span)
            .ok_or_else(|| EmitError::InvalidSpan {
                path: path.to_owned(),
                span: span.clone(),
            })?;
        self.push_str(&format_name(text));
        Ok(())
    }

    /// Emit one arena-backed reference without reconstructing or splitting a display string.
    pub(crate) fn push_qualified_reference(
        &mut self,
        path: &str,
        id: QualifiedReferenceId,
    ) -> Result<(), EmitError> {
        let document = self.document;
        let reference = document.qualified_reference(id).ok_or_else(|| {
            EmitError::InvalidQualifiedReference {
                path: path.to_owned(),
                id,
            }
        })?;
        if reference.metadata.is_absolute {
            self.push_str("$::");
        }
        for (index, segment) in reference.segments.iter().enumerate() {
            match segment.separator_before {
                Some(ReferenceSeparator::ColonColon) => self.push_str("::"),
                Some(ReferenceSeparator::Dot) => self.push_char('.'),
                None if index == 0 => {}
                None => {
                    return Err(EmitError::InvalidQualifiedReference {
                        path: path.to_owned(),
                        id,
                    });
                }
            }
            // The arena retains each segment's exact lexical span. Re-emit that authored token
            // rather than its decoded name: quoting is syntactic provenance, and a quoted name
            // that happens to be a valid bare identifier must not be normalized away.
            let authored = document.source.slice(&segment.source_span).ok_or_else(|| {
                EmitError::InvalidSpan {
                    path: path.to_owned(),
                    span: segment.source_span.clone(),
                }
            })?;
            self.push_str(authored);
        }
        Ok(())
    }

    pub(crate) fn newline(&mut self) {
        self.buf.push('\n');
        self.at_line_start = true;
    }

    pub(crate) fn indent(&mut self) {
        self.depth += 1;
    }

    pub(crate) fn dedent(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }

    pub(crate) fn unsupported(
        &self,
        path: &str,
        construct: impl Into<String>,
    ) -> Result<(), EmitError> {
        Err(EmitError::Unsupported {
            path: path.to_string(),
            construct: construct.into(),
        })
    }
}

/// Quote a SysML name when it is not a bare identifier.
pub(crate) fn format_name(name: &str) -> String {
    if needs_quotes(name) {
        format!("'{}'", name.replace('\'', "\\'"))
    } else {
        name.to_string()
    }
}

fn needs_quotes(name: &str) -> bool {
    if name.is_empty() {
        return true;
    }
    // A declared name that spells a reserved keyword (`'in'`, `'private'`, Kernel Semantic
    // Library `KerML.kerml`) was necessarily authored quoted; emitting it bare would reparse as
    // the keyword.
    if crate::parser::lex::is_reserved_keyword(name.as_bytes()) {
        return true;
    }
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return true;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return true;
    }
    !chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Emit visibility keyword followed by a space when present.
pub(crate) fn emit_visibility(w: &mut EmitWriter<'_>, visibility: Option<crate::ast::Visibility>) {
    use crate::ast::Visibility;
    match visibility {
        Some(Visibility::Private) => w.push_str("private "),
        Some(Visibility::Protected) => w.push_str("protected "),
        Some(Visibility::Public) => w.push_str("public "),
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_name_quotes_when_needed() {
        assert_eq!(format_name("Vehicle"), "Vehicle");
        assert_eq!(
            format_name("2a-Parts Interconnection"),
            "'2a-Parts Interconnection'"
        );
        assert_eq!(format_name("_ok"), "_ok");
    }
}
