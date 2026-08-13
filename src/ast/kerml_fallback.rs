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

/// Structured KerML classifier declaration with a body, e.g. `abstract function isZero
/// specializes DataFunctions::isZero { in x; return : Boolean[1]; }` (Kernel Function Library).
/// Grows one [`KermlClassifierKeyword`] variant per structurally implemented keyword family;
/// keywords not yet listed still fall through to the opaque
/// [`KermlSemanticDecl`]/[`KermlFeatureDecl`] fallbacks.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlClassifierDecl {
    /// Leading `abstract` keyword.
    pub is_abstract: bool,
    /// The classifier kind keyword.
    pub keyword: KermlClassifierKeyword,
    /// `all` after the keyword (KerML `isSufficient`), e.g. `assoc all HappensBefore
    /// specializes HappensLink { ... }` (Kernel Semantic Library `Links.kerml`).
    pub is_all: bool,
    pub identification: crate::ast::Identification,
    /// Multiplicity after the name, e.g. `struct UniversalClockLife[1] :> Clock, Life`
    /// (Kernel Semantic Library `Clocks.kerml`) and the bodied `multiplicity zeroOrOne [0..1]
    /// { ... }` declarations (`Base.kerml`).
    pub multiplicity: Option<Node<crate::ast::Multiplicity>>,
    /// `specializes`/`:>` supertype clause (multi-target), or the `:` typing clause of the
    /// `bool`/`expr` feature forms -- the relationship's kind records which operator was
    /// authored.
    pub specializes: Option<Node<crate::ast::TypingRelationship>>,
    /// KerML type relationship clauses following the header: `disjoint from A, B`,
    /// `unions A, B`, `intersects A, B` (Kernel Semantic Library `Occurrences.kerml`,
    /// Kernel Data Type Library `VectorValues.kerml`). Authored order preserved.
    pub type_relationships: Vec<Node<KermlTypeRelationship>>,
    /// Classifier body. KerML type bodies share the calc-body member grammar (parameters,
    /// `return` results, feature members, invariants, expressions, documentation).
    pub body: crate::ast::CalcDefBody,
    pub membership: crate::ast::Membership,
}

/// The classifier keyword of a structurally implemented [`KermlClassifierDecl`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KermlClassifierKeyword {
    /// `function` (KerML `Function`), e.g. the Kernel Function Library declarations.
    Function,
    /// `datatype` (KerML `DataType`).
    Datatype,
    /// `metaclass` (KerML `Metaclass`).
    Metaclass,
    /// `struct` (KerML `Structure`, short spelling).
    Struct,
    /// `assoc` (KerML `Association`, short spelling).
    Assoc,
    /// `behavior` (KerML `Behavior`).
    Behavior,
    /// `interaction` (KerML `Interaction`).
    Interaction,
    /// `predicate` (KerML `Predicate`).
    Predicate,
    /// `multiplicity` (KerML `Multiplicity`), the bodied form (`multiplicity zeroOrOne [0..1]
    /// { ... }`); the bare `;` form stays on [`KermlBareDeclaration`].
    Multiplicity,
    /// `subclassifier` (KerML dialect shorthand used by `Links.kerml`).
    Subclassifier,
    /// `classifier` (KerML `Classifier`), the bodied form (`abstract classifier Anything
    /// { ... }`); bare forward declarations stay on [`KermlBareDeclaration`].
    Classifier,
    /// `class` (KerML `Class`), reached for the shapes `class_def` rejects (e.g. `abstract
    /// class Occurrence specializes Anything disjoint from DataValue { ... }`).
    Class,
    /// The compound `assoc struct` keyword pair (KerML `AssociationStructure`), e.g. `assoc
    /// struct LinkObject specializes Link, Object intersects Link, Object { ... }`
    /// (`Objects.kerml`).
    AssocStruct,
}

impl KermlClassifierKeyword {
    /// The authored keyword spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::Datatype => "datatype",
            Self::Metaclass => "metaclass",
            Self::Struct => "struct",
            Self::Assoc => "assoc",
            Self::Behavior => "behavior",
            Self::Interaction => "interaction",
            Self::Predicate => "predicate",
            Self::Multiplicity => "multiplicity",
            Self::Subclassifier => "subclassifier",
            Self::Classifier => "classifier",
            Self::Class => "class",
            Self::AssocStruct => "assoc struct",
        }
    }
}

/// One KerML type relationship clause on a [`KermlClassifierDecl`] header: `disjoint from`,
/// `unions`, or `intersects` with one or more comma-separated targets.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlTypeRelationship {
    pub keyword: KermlTypeRelationshipKeyword,
    pub targets: Vec<crate::ast::QualifiedReferenceId>,
    /// Span of the whole clause (keyword through last target).
    pub span: Span,
}

/// The relationship keyword of a [`KermlTypeRelationship`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KermlTypeRelationshipKeyword {
    /// `disjoint from`.
    DisjointFrom,
    /// `unions`.
    Unions,
    /// `intersects`.
    Intersects,
}

impl KermlTypeRelationshipKeyword {
    /// The authored keyword spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DisjointFrom => "disjoint from",
            Self::Unions => "unions",
            Self::Intersects => "intersects",
        }
    }
}

/// KerML feature member inside a type body, e.g. `derived var feature annotatedElement :
/// Element[1..*] ordered redefines annotatedElement;` (Kernel Semantic Library `KerML.kerml`)
/// or `feature all spaceShotOf: Occurrence[0..*] subsets spaceSliceOf inverse of spaceShots
/// { ... }` (`Occurrences.kerml`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlFeatureMember {
    /// `member` prefix (KerML `NonFeatureMember` shorthand used by `KerML.kerml`).
    pub is_member: bool,
    /// `derived` prefix.
    pub is_derived: bool,
    /// `abstract` prefix.
    pub is_abstract: bool,
    /// `composite` prefix.
    pub is_composite: bool,
    /// `portion` prefix.
    pub is_portion: bool,
    /// `var` prefix.
    pub is_var: bool,
    /// `end` prefix.
    pub is_end: bool,
    /// The feature-kind keyword: `feature`, `step`, `expr`, or `bool`.
    pub kind: KermlFeatureKind,
    /// `all` after the kind keyword (KerML `isSufficient`).
    pub is_all: bool,
    /// Declared name (may be quoted, e.g. `'in'`). Empty for the redefinition-led form
    /// (`portion feature redefines spaceBoundary [1];`).
    pub name: String,
    /// `:` typing clause (multi-target).
    pub typing: Option<Node<crate::ast::TypingRelationship>>,
    /// Multiplicity clause, accepted before or after the typing (and after a leading
    /// redefinition target).
    pub multiplicity: Option<Node<crate::ast::Multiplicity>>,
    /// `ordered` keyword from `MultiplicityPart`.
    pub ordered: bool,
    /// `nonunique` keyword from `MultiplicityPart`.
    pub nonunique: bool,
    /// `subsets`/`:>` clause (multi-target, chains allowed).
    pub subsets: Option<Node<crate::ast::SubsettingRelationship>>,
    /// `redefines`/`:>>` clause (multi-target).
    pub redefines: Option<Node<crate::ast::SubsettingRelationship>>,
    /// `references`/`::>` clause.
    pub references: Option<Node<crate::ast::SubsettingRelationship>>,
    /// `inverse of` target chain.
    pub inverse_of: Option<crate::ast::QualifiedReferenceId>,
    /// Value clause: `= expr` / `:= expr` / `default (=|:=)? expr`.
    pub value: Option<Node<crate::ast::FeatureValue>>,
    /// Body following the shared type-body member grammar: `;` or `{ ... }`.
    pub body: crate::ast::CalcDefBody,
    pub membership: crate::ast::Membership,
}

/// The kind keyword of a [`KermlFeatureMember`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KermlFeatureKind {
    /// `feature`.
    Feature,
    /// `step` (`abstract step performances: Performance[0..*] nonunique subsets occurrences
    /// { ... }`, `Performances.kerml`).
    Step,
    /// `expr` (`abstract expr evaluations: Evaluation[0..*] nonunique subsets performances
    /// { ... }`).
    Expr,
    /// `bool` (`bool earlierFirstIncomingTransferSort : IncomingTransferSort { ... }`,
    /// `Occurrences.kerml`).
    Bool,
}

impl KermlFeatureKind {
    /// The authored keyword spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Feature => "feature",
            Self::Step => "step",
            Self::Expr => "expr",
            Self::Bool => "bool",
        }
    }
}

/// KerML invariant member: `inv` name? `{ boolean-expressions }`, e.g. `inv unitBound { -1.0 <=
/// that & that <= 1.0 }` (Kernel Function Library `TrigFunctions.kerml`) or the anonymous
/// `inv { isClosed == true }` (`Occurrences.kerml`). Bare `inv name;` forward declarations stay
/// on [`KermlBareDeclaration`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlInvariantMember {
    /// `inv not` negated form (KerML `isNegated`).
    pub is_negated: bool,
    /// Declared name; empty for the anonymous form.
    pub name: String,
    /// Body holding the invariant's boolean expression(s) via the shared type-body grammar.
    pub body: crate::ast::CalcDefBody,
    pub membership: crate::ast::Membership,
}
