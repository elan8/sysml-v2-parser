//! Canonical SysML textual emitter for AST → source roundtrips.
//!
//! Emits structured AST fields and can reproduce resilient parser recovery nodes from their
//! document source spans. Other opaque constructs fail closed via [`EmitError::Opaque`] or are
//! reported by [`opacity_report`].
//!
//! # Production formatter limitation
//!
//! TODO(comment-note-trivia-provenance): KerML distinguishes semantic `REGULAR_COMMENT` tokens
//! (`Comment`, `Documentation`, and `TextualRepresentation` bodies) from non-model lexical notes
//! (`// ...` and `//* ... */`) and whitespace. The parser must model regular comments and apply the
//! specification's body-normalization rules; a separate source-fidelity syntax/trivia layer must
//! retain lexical-note and whitespace spans without pretending they are semantic AST elements.
//! The emitter currently cannot preserve notes or the placement of all semantic comments. Do not
//! present it as a production source formatter until recovery adjacency is retained and end-to-end
//! fixtures prove comment/note retention and idempotence:
//! `format(format(source)) == format(source)`.

mod behavior;
mod expr;
mod opacity;
mod requirement;
mod root;
mod structure;
mod view;
mod writer;

pub use opacity::{opacity_report, OpacityHit, OpacityKind, OpacityReport};

use crate::ast::{ParsedDocument, QualifiedReferenceId, Span};
use writer::EmitWriter;

/// Options controlling canonical SysML emission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitOptions {
    /// Spaces per nesting level (default: 4).
    pub indent: usize,
    /// Whether to emit `doc` / `comment` / `rep` bodies (default: true).
    pub emit_comments: bool,
}

impl Default for EmitOptions {
    fn default() -> Self {
        Self {
            indent: 4,
            emit_comments: true,
        }
    }
}

/// Failure while emitting SysML from an AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmitError {
    /// An opaque / recovery node was encountered; emission refuses to reprint raw text.
    Opaque { path: String, kind: OpacityKind },
    /// A structured construct is not yet implemented by the emitter.
    Unsupported { path: String, construct: String },
    /// An arena identity could not be resolved or contained invalid source-backed metadata.
    InvalidQualifiedReference {
        path: String,
        id: QualifiedReferenceId,
    },
    /// A source-backed span could not be resolved against the document's source text.
    InvalidSpan { path: String, span: Span },
}

impl std::fmt::Display for EmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opaque { path, kind } => {
                write!(f, "opaque AST at {path}: {kind:?}")
            }
            Self::Unsupported { path, construct } => {
                write!(f, "unsupported emit construct at {path}: {construct}")
            }
            Self::InvalidQualifiedReference { path, id } => {
                write!(f, "invalid qualified reference {id:?} at {path}")
            }
            Self::InvalidSpan { path, span } => {
                write!(f, "invalid source span {span:?} at {path}")
            }
        }
    }
}

impl std::error::Error for EmitError {}

/// Emit canonical SysML text for `document` using default [`EmitOptions`].
pub fn emit_sysml(document: &ParsedDocument) -> Result<String, EmitError> {
    emit_sysml_with_options(document, &EmitOptions::default())
}

/// Emit a resilient editor document, reproducing parser recovery nodes from their captured source
/// spans while canonically emitting structured siblings.
///
/// Unlike [`emit_sysml`], this entry point does not reject a document merely because its opacity
/// report contains parse-recovery nodes. Other unsupported constructs still fail normally.
pub fn emit_recovered_sysml(document: &ParsedDocument) -> Result<String, EmitError> {
    let options = EmitOptions::default();
    let mut writer = EmitWriter::new(document, &options);
    root::emit_root(&mut writer, &document.root)?;
    Ok(writer.finish())
}

/// Emit canonical SysML text for `document` using `opts`.
pub fn emit_sysml_with_options(
    document: &ParsedDocument,
    opts: &EmitOptions,
) -> Result<String, EmitError> {
    let report = opacity_report(&document.root);
    if !report.is_clean() {
        let hit = &report.hits[0];
        return Err(EmitError::Opaque {
            path: hit.path.clone(),
            kind: hit.kind,
        });
    }
    let mut w = EmitWriter::new(document, opts);
    root::emit_root(&mut w, &document.root)?;
    Ok(w.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        DeclarationName, Identification, Membership, Node, Package, PackageBody,
        PackageBodyElement, ParsedDocument, PartDef, PartDefBody, QualifiedIdentification,
        QualifiedReferenceArena, RootElement, RootNamespace, SourceStorage, Span,
    };

    fn owning() -> Membership {
        Membership::owning(None, Span::dummy())
    }

    fn package_identification(name: &str) -> QualifiedIdentification {
        QualifiedIdentification {
            short_name: None,
            name: Some(DeclarationName::Simple(name.to_owned())),
        }
    }

    fn document(root: RootNamespace) -> ParsedDocument {
        ParsedDocument {
            source: SourceStorage::default(),
            qualified_references: QualifiedReferenceArena::default(),
            root,
        }
    }

    #[test]
    fn emit_minimal_part_def_package() {
        let root = RootNamespace {
            elements: vec![Node::new(
                Span::dummy(),
                RootElement::Package(Node::new(
                    Span::dummy(),
                    Package {
                        identification: package_identification("P"),
                        body: PackageBody::Brace {
                            open_span: Span::dummy(),
                            close_span: Span::dummy(),
                            elements: vec![Node::new(
                                Span::dummy(),
                                PackageBodyElement::PartDef(Node::new(
                                    Span::dummy(),
                                    PartDef {
                                        definition_prefix: None,
                                        is_individual: false,
                                        identification: Identification {
                                            short_name: None,
                                            name: Some("Vehicle".into()),
                                        },
                                        specializes: None,
                                        body: PartDefBody::Semicolon {
                                            semicolon_span: Span::dummy(),
                                        },
                                        membership: owning(),
                                    },
                                )),
                            )],
                        },
                    },
                )),
            )],
        };
        let out = emit_sysml(&document(root)).expect("emit");
        assert_eq!(out.trim(), "package P {\n    part def Vehicle;\n}");
    }

    /// The inverse of what this used to assert. A `connect` body inside a part definition was a
    /// `ConnectBody` marker whose members lived nowhere, so emission had to refuse it; it is a
    /// `UsageBody` now, so it emits, and the opacity report has nothing to say about it.
    #[test]
    fn a_connect_body_is_no_longer_opaque() {
        let source = "package P {\n    part def Q {\n        connect a to b {\n            doc /* why */\n        }\n    }\n}\n";
        let document = crate::parse_for_editor(source).document;
        assert!(
            opacity_report(&document.root).is_clean(),
            "a connect body retains its members, so nothing about it is opaque"
        );
        let emitted = emit_sysml(&document).expect("a connect body emits");
        assert!(
            emitted.contains("doc"),
            "the body's member must survive emission, got: {emitted}"
        );
    }

    /// `OpacityKind::OpaqueConnectBrace` is still reachable from the `ConnectBody` owners that
    /// have not been converted yet -- a `transition` body is one -- so the state stays covered
    /// rather than becoming an untested variant while those conversions land.
    #[test]
    fn a_transition_brace_body_is_still_opaque() {
        let source = "package P {\n    state def S {\n        state a;\n        state b;\n        transition first a then b {\n            doc /* why */\n        }\n    }\n}\n";
        let document = crate::parse_for_editor(source).document;
        let report = opacity_report(&document.root);
        assert!(!report.is_clean());
        assert!(
            report
                .hits
                .iter()
                .any(|hit| hit.kind == OpacityKind::OpaqueConnectBrace),
            "expected an opaque connect-brace hit, got: {:?}",
            report.hits
        );
    }

    #[test]
    fn recovered_emit_preserves_braced_error_and_formats_later_sibling() {
        let source = r#"package P {
action def A {
  badstmt {}
  action good { };
}
}"#;
        let parsed = crate::parse_for_editor(source);
        assert!(
            !parsed.errors.is_empty(),
            "fixture must exercise parser recovery"
        );
        assert!(matches!(
            emit_sysml(&parsed.document),
            Err(EmitError::Opaque {
                kind: OpacityKind::ParseError,
                ..
            })
        ));

        let emitted = emit_recovered_sysml(&parsed.document).expect("recovered emit");
        assert_eq!(
            emitted,
            "package P {\n    action def A {\n        badstmt {}\n        action good {\n        }\n    }\n}\n"
        );
    }

    #[test]
    fn first_merge_brace_body_emits_typed_members_and_roundtrips() {
        let source = "package P { action def A { fork choice {\n  in left;\n  out right;\n} } }";
        let document = crate::parse(source).expect("parse typed fork body");
        assert!(opacity_report(&document.root).is_clean());

        let emitted = emit_sysml(&document).expect("emit typed fork body");
        assert!(emitted.contains("fork choice {"));
        assert!(emitted.contains("in left;"));
        assert!(emitted.contains("out right;"));

        let reparsed = crate::parse(&emitted).expect("reparse emitted fork body");
        assert!(opacity_report(&reparsed.root).is_clean());
    }
}
