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

/// Quote each `::`-separated segment of a qualified name when required.
///
/// Import targets store unquoted segment text (e.g. `2a-Parts Interconnection::*`);
/// wildcards (`*` / `**`) and the KerML root marker (`$`) are left as-is.
pub(crate) fn format_qualified_name(qname: &str) -> String {
    qname
        .split("::")
        .map(|seg| match seg {
            "*" | "**" | "$" => seg.to_string(),
            other => format_name(other),
        })
        .collect::<Vec<_>>()
        .join("::")
}

/// Quote each `.`-separated segment of a feature path (e.g. `vehicleStates.on`).
pub(crate) fn format_feature_path(path: &str) -> String {
    path.split('.')
        .map(format_name)
        .collect::<Vec<_>>()
        .join(".")
}

/// Quote each segment of a structured relationship target for emission.
pub(crate) fn format_relationship_target(target: &crate::ast::RelationshipTarget) -> String {
    use crate::ast::SegmentSeparator;
    let mut out = String::new();
    for segment in &target.segments {
        match segment.separator {
            Some(SegmentSeparator::ColonColon) => out.push_str("::"),
            Some(SegmentSeparator::Dot) => out.push('.'),
            None => {}
        }
        if segment.name == "$" {
            out.push_str(&segment.name);
        } else {
            out.push_str(&format_name(&segment.name));
        }
    }
    out
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

    #[test]
    fn format_qualified_name_quotes_segments_preserves_wildcards() {
        assert_eq!(format_qualified_name("SI::kg"), "SI::kg");
        assert_eq!(
            format_qualified_name("2a-Parts Interconnection::*"),
            "'2a-Parts Interconnection'::*"
        );
        assert_eq!(
            format_qualified_name("Safety Features::*"),
            "'Safety Features'::*"
        );
        assert_eq!(format_qualified_name("$::ISQ::*"), "$::ISQ::*");
        assert_eq!(format_qualified_name("Pkg::**"), "Pkg::**");
    }

    #[test]
    fn format_feature_path_quotes_segments() {
        assert_eq!(format_feature_path("vehicleStates.on"), "vehicleStates.on");
        assert_eq!(
            format_feature_path("vehicle states.on"),
            "'vehicle states'.on"
        );
    }

    #[test]
    fn format_relationship_target_quotes_segments() {
        use crate::ast::{RelationshipTarget, RelationshipTargetSegment, SegmentSeparator, Span};

        let target = RelationshipTarget {
            segments: vec![RelationshipTargetSegment {
                name: "Temporal-Spatial Reference".into(),
                separator: None,
            }],
            span: Span::dummy(),
        };
        assert_eq!(
            format_relationship_target(&target),
            "'Temporal-Spatial Reference'"
        );

        let chained = RelationshipTarget {
            segments: vec![
                RelationshipTargetSegment {
                    name: "ISQ".into(),
                    separator: None,
                },
                RelationshipTargetSegment {
                    name: "mass".into(),
                    separator: Some(SegmentSeparator::ColonColon),
                },
            ],
            span: Span::dummy(),
        };
        assert_eq!(format_relationship_target(&chained), "ISQ::mass");
    }
}
