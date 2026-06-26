//! KerML fallback and modeled declaration nodes.

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
