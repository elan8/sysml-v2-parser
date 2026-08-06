//! Canonical SysML textual emitter for AST → source roundtrips.
//!
//! Emits structured AST fields only. Opaque / recovery nodes fail closed via
//! [`EmitError::Opaque`] or are reported by [`opacity_report`].

mod behavior;
mod expr;
mod opacity;
mod requirement;
mod root;
mod structure;
mod view;
mod writer;

pub use opacity::{opacity_report, OpacityHit, OpacityKind, OpacityReport};

use crate::ast::RootNamespace;
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
    Opaque {
        path: String,
        kind: OpacityKind,
    },
    /// A structured construct is not yet implemented by the emitter.
    Unsupported {
        path: String,
        construct: String,
    },
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
        }
    }
}

impl std::error::Error for EmitError {}

/// Emit canonical SysML text for `root` using default [`EmitOptions`].
pub fn emit_sysml(root: &RootNamespace) -> Result<String, EmitError> {
    emit_sysml_with_options(root, &EmitOptions::default())
}

/// Emit canonical SysML text for `root` using `opts`.
pub fn emit_sysml_with_options(
    root: &RootNamespace,
    opts: &EmitOptions,
) -> Result<String, EmitError> {
    let report = opacity_report(root);
    if !report.is_clean() {
        let hit = &report.hits[0];
        return Err(EmitError::Opaque {
            path: hit.path.clone(),
            kind: hit.kind,
        });
    }
    let mut w = EmitWriter::new(opts);
    root::emit_root(&mut w, root)?;
    Ok(w.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        Identification, Membership, MembershipKind, Node, Package, PackageBody,
        PackageBodyElement, PartDef, PartDefBody, PartDefBodyElement, RootElement, RootNamespace,
        Span, Visibility,
    };

    fn owning() -> Membership {
        Membership::owning(None, Span::dummy())
    }

    #[test]
    fn emit_minimal_part_def_package() {
        let root = RootNamespace {
            elements: vec![Node::new(
                Span::dummy(),
                RootElement::Package(Node::new(
                    Span::dummy(),
                    Package {
                        identification: Identification {
                            short_name: None,
                            name: Some("P".into()),
                        },
                        body: PackageBody::Brace {
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
                                        body: PartDefBody::Semicolon,
                                        membership: owning(),
                                    },
                                )),
                            )],
                        },
                    },
                )),
            )],
        };
        let out = emit_sysml(&root).expect("emit");
        assert_eq!(out.trim(), "package P {\n    part def Vehicle;\n}");
    }

    #[test]
    fn emit_rejects_opaque_other() {
        let root = RootNamespace {
            elements: vec![Node::new(
                Span::dummy(),
                RootElement::Package(Node::new(
                    Span::dummy(),
                    Package {
                        identification: Identification {
                            short_name: None,
                            name: Some("P".into()),
                        },
                        body: PackageBody::Brace {
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
                                        body: PartDefBody::Brace {
                                            elements: vec![Node::new(
                                                Span::dummy(),
                                                PartDefBodyElement::Other("mystery;".into()),
                                            )],
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
        let err = emit_sysml(&root).expect_err("opaque");
        assert!(matches!(err, EmitError::Opaque { .. }));
    }

    #[test]
    fn opacity_report_finds_other() {
        let root = RootNamespace {
            elements: vec![Node::new(
                Span::dummy(),
                RootElement::Package(Node::new(
                    Span::dummy(),
                    Package {
                        identification: Identification {
                            short_name: None,
                            name: Some("P".into()),
                        },
                        body: PackageBody::Brace {
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
                                        body: PartDefBody::Brace {
                                            elements: vec![Node::new(
                                                Span::dummy(),
                                                PartDefBodyElement::Other("x".into()),
                                            )],
                                        },
                                        membership: Membership::new(
                                            MembershipKind::OwningMembership,
                                            Some(Visibility::Private),
                                            Span::dummy(),
                                        ),
                                    },
                                )),
                            )],
                        },
                    },
                )),
            )],
        };
        let report = opacity_report(&root);
        assert!(!report.is_clean());
        assert_eq!(report.hits[0].kind, OpacityKind::Other);
    }
}
