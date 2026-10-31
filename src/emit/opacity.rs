//! Report the recovered and opaque content that emission must refuse to reprint.

use crate::ast::visit::recovery::{scan_recovery_content, RecoveryContent};
use crate::ast::{RootNamespace, Span};

/// Kind of opaque or recovery content found in an AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpacityKind {
    ParseError,
    UnsupportedGrammar,
}

/// One opaque node: what it is and the source it covers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpacityHit {
    pub span: Span,
    pub kind: OpacityKind,
}

/// Aggregated opacity scan result.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OpacityReport {
    pub hits: Vec<OpacityHit>,
}

impl OpacityReport {
    pub fn is_clean(&self) -> bool {
        self.hits.is_empty()
    }
}

/// Scan `root` for opaque / recovery nodes, in traversal order: depth first, source order within
/// each body. A reported node's children are scanned too.
///
/// This is the same scan that turns those nodes into parse diagnostics, so a tree is opaque
/// exactly when it contains content parsing reported as recovered or not structurally modelled.
pub fn opacity_report(root: &RootNamespace) -> OpacityReport {
    let mut report = OpacityReport::default();
    scan_recovery_content(root, |content| {
        let (span, kind) = match content {
            // Retained malformed syntax: emitting it would present the keyword as a valid name.
            RecoveryContent::ReservedDeclarationName(name) => {
                (*name.span(), OpacityKind::ParseError)
            }
            // Reprinting it would present an import the grammar rejects as valid source.
            RecoveryContent::MissingImportVisibility(span) => (span, OpacityKind::ParseError),
            RecoveryContent::ParseError(node) => (node.span, OpacityKind::ParseError),
            RecoveryContent::UnsupportedGrammar(node) => {
                (node.span, OpacityKind::UnsupportedGrammar)
            }
            // A missing or empty `language` is a diagnostic, but the body is typed and emits as
            // authored.
            RecoveryContent::TextualRepresentation(_) => return,
        };
        report.hits.push(OpacityHit { span, kind });
    });
    report
}

#[cfg(test)]
mod tests {
    use super::{opacity_report, OpacityKind};

    #[test]
    fn reports_recovery_nested_inside_interface_definition() {
        let source = "package P { part def PP { interface def I { this is not valid at all; } } }";
        let parsed = crate::parse_for_editor(source);

        assert!(!parsed.errors.is_empty(), "fixture must exercise recovery");
        let hits = opacity_report(&parsed.document.root).hits;
        assert!(
            hits.iter().any(|hit| hit.kind == OpacityKind::ParseError
                && source[hit.span.offset..hit.span.offset + hit.span.len]
                    .starts_with("this is not valid")),
            "nested interface recovery must make the document opaque: {hits:?}"
        );
    }
}
