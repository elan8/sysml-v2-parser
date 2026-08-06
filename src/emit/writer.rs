//! Indenting SysML text writer.

use super::{EmitError, EmitOptions};

pub(crate) struct EmitWriter<'a> {
    buf: String,
    depth: usize,
    opts: &'a EmitOptions,
    at_line_start: bool,
}

impl<'a> EmitWriter<'a> {
    pub(crate) fn new(opts: &'a EmitOptions) -> Self {
        Self {
            buf: String::new(),
            depth: 0,
            opts,
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

    pub(crate) fn push_char(&mut self, c: char) {
        let mut tmp = [0u8; 4];
        self.push_str(c.encode_utf8(&mut tmp));
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
        format!("'{name}'")
    } else {
        name.to_string()
    }
}

fn needs_quotes(name: &str) -> bool {
    if name.is_empty() {
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
pub(crate) fn emit_visibility(
    w: &mut EmitWriter<'_>,
    visibility: Option<crate::ast::Visibility>,
) {
    use crate::ast::Visibility;
    match visibility {
        Some(Visibility::Private) => w.push_str("private "),
        Some(Visibility::Protected) => w.push_str("protected "),
        Some(Visibility::Public) => w.push_str("public "),
        None => {}
    }
}
