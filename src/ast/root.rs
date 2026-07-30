use super::common::{Identification, Import};
use super::package::{LibraryPackage, Package, PackageBody, PackageBodyElement};
use crate::ast::core::Node;

/// KerML top-level element (BNF `RootNamespace = PackageBodyElement*`).
///
/// Package / library package / namespace / import are modeled as dedicated variants for
/// consumers that walk the package tree. Any other legal package-body member at file root
/// (definitions and usages) is [`RootElement::Member`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RootElement {
    Package(Node<Package>),
    LibraryPackage(Node<LibraryPackage>),
    Namespace(Node<NamespaceDecl>),
    Import(Node<Import>),
    /// Definition or usage (or other package-body member) at root, per SysML `RootNamespace`.
    Member(Box<Node<PackageBodyElement>>),
}

/// KerML NamespaceDeclaration: `namespace` Identification NamespaceBody (same body structure as Package).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamespaceDecl {
    pub identification: Identification,
    pub body: PackageBody,
}

/// Root of a SysML/KerML document: a sequence of top-level package or namespace elements.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RootNamespace {
    pub elements: Vec<Node<RootElement>>,
}
