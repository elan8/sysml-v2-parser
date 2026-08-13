//! KerML fallback and modeled declaration nodes.

use crate::ast::{Multiplicity, Node, Span};

/// A structurally recognized bare KerML declaration: `kind` `name`? (`[` multiplicity `]`)? `;`.
/// Covers the shape shared by declarations such as `datatype DeferredType;`,
/// `multiplicity exactlyOne [1..1];`, `interaction DeferredInteraction;`, and
/// `predicate deferredPredicate;` -- a leading KerML classifier/feature keyword, an optional
/// name, an optional multiplicity, and a terminating `;` with no body. Declarations of the same
/// keyword that instead carry a `{ ... }` body are a different, larger production and remain
/// represented by the opaque [`KermlSemanticDecl`]/[`KermlFeatureDecl`] fallback nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlBareDeclaration {
    /// The authored leading keyword, e.g. `"datatype"`, `"multiplicity"`, `"interaction"`,
    /// `"predicate"`. Preserved verbatim (not normalized to a canonical spelling) since KerML
    /// admits synonymous spellings (e.g. `assoc`/`association`) that are authored facts.
    pub keyword: String,
    /// The declared name, when present.
    pub name: Option<String>,
    /// Span of `name`, when present.
    pub name_span: Option<Span>,
    /// The `[...]` multiplicity clause, when present.
    pub multiplicity: Option<Node<Multiplicity>>,
}

/// Modeled KerML semantic declaration captured as package-level syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlSemanticDecl {
    pub bnf_production: String,
    pub text: String,
}

/// Modeled KerML feature declaration family (occurrence/expr/predicate/succession).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlFeatureDecl {
    pub bnf_production: String,
    pub text: String,
}

/// Package-level KerML feature declaration captured as an explicit dedicated node.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeatureDecl {
    pub keyword: String,
    pub text: String,
}

/// Package-level KerML classifier declaration captured as an explicit dedicated node.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassifierDecl {
    pub keyword: String,
    pub text: String,
}

/// Modeled extended SysML/KerML declaration family not yet represented by
/// dedicated concrete nodes (e.g. concern/message style library declarations).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedLibraryDecl {
    pub bnf_production: String,
    pub text: String,
}
