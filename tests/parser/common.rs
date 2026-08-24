//! Shared helpers for parser TDD tests.

use std::path::PathBuf;

use sysml_v2_parser::ast::{
    Node, Package, PackageBody, ParsedDocument, QualifiedIdentification, RootElement,
    RootNamespace, Span,
};

/// Identification of the first root-level (library) package in `doc`, asserting that its simple
/// name is spelled `name`.
///
/// Declaration names are source spans, so an expected AST cannot synthesise one; the
/// identification is taken from the parsed document and its spelling is checked here.
pub(crate) fn package_id(doc: &ParsedDocument, name: &str) -> QualifiedIdentification {
    let identification = doc
        .root
        .elements
        .iter()
        .find_map(|element| match &element.value {
            RootElement::Package(p) => Some(&p.value.identification),
            RootElement::LibraryPackage(p) => Some(&p.value.identification),
            _ => None,
        })
        .expect("document should contain a package");
    assert!(identification.short_name.is_none());
    assert_eq!(
        identification
            .simple_name()
            .and_then(|n| doc.declaration_name(n)),
        Some(name)
    );
    identification.clone()
}

pub(crate) fn sysml_v2_release_root() -> PathBuf {
    std::env::var_os("SYSML_V2_RELEASE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml-v2-release"))
}

pub(crate) fn primitive_data_types_fixture() -> Option<String> {
    let path = sysml_v2_release_root()
        .join("sysml")
        .join("src")
        .join("validation")
        .join("15-Properties-Values-Expressions")
        .join("15_10-Primitive Data Types.sysml");
    std::fs::read_to_string(path).ok()
}

/// Node with span matching parser output for full-input parses (offset 0, line 1, column 1).
pub(crate) fn n_len<T>(len: usize, v: T) -> Node<T> {
    Node::new(
        Span {
            offset: 0,
            line: 1,
            column: 1,
            len,
        },
        v,
    )
}

/// Build expected AST for `package Foo;` (input len = 12)
pub(crate) fn expected_package_foo_semicolon(doc: &ParsedDocument) -> RootNamespace {
    RootNamespace {
        elements: vec![n_len(
            12,
            RootElement::Package(n_len(
                12,
                Package {
                    identification: package_id(doc, "Foo"),
                    body: PackageBody::Semicolon {
                        semicolon_span: Span::dummy(),
                    },
                },
            )),
        )],
    }
}

/// Build expected AST for `package Bar { }` (input len = 15)
pub(crate) fn expected_package_bar_brace(doc: &ParsedDocument) -> RootNamespace {
    RootNamespace {
        elements: vec![n_len(
            15,
            RootElement::Package(n_len(
                15,
                Package {
                    identification: package_id(doc, "Bar"),
                    body: PackageBody::Brace {
                        open_span: Span::dummy(),
                        close_span: Span::dummy(),
                        elements: vec![],
                    },
                },
            )),
        )],
    }
}
