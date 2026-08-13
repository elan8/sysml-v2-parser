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
    /// The authored leading keyword. An exhaustive enum rather than a `String`: the parser only
    /// ever matches one of a finite set of KerML classifier/feature starters, and distinct
    /// variants (`Assoc`/`Association`) preserve authored synonym spelling without copying text.
    pub keyword: KermlBareDeclarationKeyword,
    /// Span of the declared name, when present. The name text itself lives in the document
    /// source and is resolved through it rather than copied into this node.
    pub name_span: Option<Span>,
    /// The `[...]` multiplicity clause, when present.
    pub multiplicity: Option<Node<Multiplicity>>,
}

/// The finite set of KerML classifier/feature keywords that introduce a bare (bodyless)
/// [`KermlBareDeclaration`]. Synonymous spellings (`assoc`/`association`) get distinct variants
/// so authored spelling round-trips without storing owned text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KermlBareDeclarationKeyword {
    Behavior,
    Bool,
    Function,
    Interaction,
    Datatype,
    Inv,
    Invariant,
    Multiplicity,
    Assoc,
    Association,
    Metaclass,
    Step,
    Occurrence,
    Expr,
    Predicate,
    Succession,
    Classifier,
}

impl KermlBareDeclarationKeyword {
    /// The exact authored spelling for this variant, e.g. `"assoc"` vs. `"association"`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Behavior => "behavior",
            Self::Bool => "bool",
            Self::Function => "function",
            Self::Interaction => "interaction",
            Self::Datatype => "datatype",
            Self::Inv => "inv",
            Self::Invariant => "invariant",
            Self::Multiplicity => "multiplicity",
            Self::Assoc => "assoc",
            Self::Association => "association",
            Self::Metaclass => "metaclass",
            Self::Step => "step",
            Self::Occurrence => "occurrence",
            Self::Expr => "expr",
            Self::Predicate => "predicate",
            Self::Succession => "succession",
            Self::Classifier => "classifier",
        }
    }

    /// Match the leading bytes of `fragment` against every keyword variant, longest-first where
    /// spellings could otherwise prefix-collide (none currently do, but this keeps the match
    /// order intentional rather than incidental).
    pub fn starters() -> &'static [(&'static [u8], Self)] {
        &[
            (b"behavior", Self::Behavior),
            (b"bool", Self::Bool),
            (b"function", Self::Function),
            (b"interaction", Self::Interaction),
            (b"datatype", Self::Datatype),
            (b"inv", Self::Inv),
            (b"invariant", Self::Invariant),
            (b"multiplicity", Self::Multiplicity),
            (b"assoc", Self::Assoc),
            (b"association", Self::Association),
            (b"metaclass", Self::Metaclass),
            (b"step", Self::Step),
            (b"occurrence", Self::Occurrence),
            (b"expr", Self::Expr),
            (b"predicate", Self::Predicate),
            (b"succession", Self::Succession),
            (b"classifier", Self::Classifier),
        ]
    }
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
