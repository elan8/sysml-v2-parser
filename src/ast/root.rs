use super::common::{Identification, Import};
use super::package::{LibraryPackage, Package, PackageBody};
use crate::ast::core::Node;

/// KerML top-level element: package, namespace, import, or library package (BNF RootNamespace = PackageBodyElement*).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RootElement {
    Package(Node<Package>),
    LibraryPackage(Node<LibraryPackage>),
    Namespace(Node<NamespaceDecl>),
    Import(Node<Import>),
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
