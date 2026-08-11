//! Shared helpers for parser TDD tests.

use std::path::PathBuf;

use sysml_v2_parser::ast::{
    DeclarationName, Node, Package, PackageBody, QualifiedIdentification, RootElement,
    RootNamespace, Span,
};

pub(crate) fn package_id(name: &str) -> QualifiedIdentification {
    QualifiedIdentification {
        short_name: None,
        name: Some(DeclarationName::Simple(name.to_string())),
    }
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
pub(crate) fn expected_package_foo_semicolon() -> RootNamespace {
    RootNamespace {
        elements: vec![n_len(
            12,
            RootElement::Package(n_len(
                12,
                Package {
                    identification: package_id("Foo"),
                    body: PackageBody::Semicolon,
                },
            )),
        )],
    }
}

/// Build expected AST for `package Bar { }` (input len = 15)
pub(crate) fn expected_package_bar_brace() -> RootNamespace {
    RootNamespace {
        elements: vec![n_len(
            15,
            RootElement::Package(n_len(
                15,
                Package {
                    identification: package_id("Bar"),
                    body: PackageBody::Brace { elements: vec![] },
                },
            )),
        )],
    }
}
