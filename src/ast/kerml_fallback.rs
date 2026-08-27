//! KerML fallback and modeled declaration nodes.

use crate::ast::{DeclarationName, Multiplicity, Node, Span};

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
    pub name: Option<DeclarationName>,
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

/// Retained source of a declaration the parser recognized but did not model structurally.
///
/// A span into the document, trimmed of surrounding trivia; resolve through
/// [`crate::ast::ParsedDocument::opaque_text`]. It is deliberately not a [`crate::ast::Span`]
/// in the open: opaque text is a narrowly scoped state, and consumers must not treat it as a
/// parsed construct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpaqueText {
    pub(crate) span: Span,
}

impl OpaqueText {
    pub(crate) fn new(span: Span) -> Self {
        Self { span }
    }

    /// The exact source span of the retained text.
    pub fn span(&self) -> &Span {
        &self.span
    }
}

/// Modeled KerML semantic declaration captured as package-level syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlSemanticDecl {
    /// The starter keyword that classified this declaration (its BNF production name).
    pub keyword_span: Span,
    pub text: OpaqueText,
}

/// Modeled KerML feature declaration family (occurrence/expr/predicate/succession).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlFeatureDecl {
    /// The starter keyword that classified this declaration (its BNF production name).
    pub keyword_span: Span,
    pub text: OpaqueText,
}

/// Package-level KerML feature declaration captured as an explicit dedicated node.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeatureDecl {
    /// The `feature` keyword.
    pub keyword_span: Span,
    pub text: OpaqueText,
}

/// Package-level KerML classifier declaration captured as an explicit dedicated node.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassifierDecl {
    /// The classifier keyword (`class`, `classifier`, `struct`, `structure`, `subclassifier`).
    pub keyword_span: Span,
    pub text: OpaqueText,
}

/// Modeled extended SysML/KerML declaration family not yet represented by
/// dedicated concrete nodes (e.g. concern/message style library declarations).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedLibraryDecl {
    /// The starter keyword that classified this declaration (its BNF production name).
    pub keyword_span: Span,
    pub text: OpaqueText,
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
    /// `ConjugationPart = ( 'conjugates' | '~' ) ownedRelationship += OwnedConjugation`
    /// (KerML BNF 462; clause 8.2.4.1.3). `TypeDeclaration` spells `( SpecializationPart |
    /// ConjugationPart )?`, so this and [`Self::specializes`] are alternatives of one choice and
    /// the parser fills at most one of them. Kept apart from the `~T` *typing* flag
    /// ([`crate::ast::TypingRelationship::is_conjugated`]), which is a different production:
    /// that one conjugates the type a feature is typed by, this one conjugates the declared type
    /// itself.
    pub conjugates: Option<Node<Conjugation>>,
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
    /// `association` (KerML `Association`, spelled out).
    Association,
    /// `behavior` (KerML `Behavior`).
    Behavior,
    /// `interaction` (KerML `Interaction`).
    Interaction,
    /// `predicate` (KerML `Predicate`).
    Predicate,
    /// `multiplicity` (KerML `Multiplicity`), the bodied form (`multiplicity zeroOrOne [0..1]
    /// { ... }`); the bare `;` form stays on [`KermlBareDeclaration`].
    Multiplicity,
    /// `type` (KerML `Type`, the general type declaration: `type UnionType unions A, B;`).
    Type,
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
            Self::Association => "association",
            Self::Behavior => "behavior",
            Self::Interaction => "interaction",
            Self::Predicate => "predicate",
            Self::Multiplicity => "multiplicity",
            Self::Type => "type",
            Self::Classifier => "classifier",
            Self::Class => "class",
            Self::AssocStruct => "assoc struct",
        }
    }
}

/// One `ConjugationPart` on a type declaration: `classifier C conjugates A;` or `classifier C ~ A;`.
///
/// `ConjugationPart : Type = ( 'conjugates' | '~' ) ownedRelationship += OwnedConjugation`
/// (KerML BNF 462). The `Conjugation` relationship's *source* is the declared type, so only the
/// target is carried here; the owning declaration is the source.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Conjugation {
    /// The conjugated type, `OwnedConjugation`'s single `[QualifiedName]`.
    pub target: crate::ast::QualifiedReferenceId,
    /// Which of the production's two alternatives was authored.
    pub spelling: ConjugationSpelling,
    /// Span of the whole clause, operator/keyword through target.
    pub span: Span,
}

/// Which spelling of [`Conjugation`] the author wrote.
///
/// The two are interchangeable in the grammar, so this is emission and provenance information,
/// not a semantic distinction -- exactly like [`crate::ast::TypingSpelling`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConjugationSpelling {
    /// The `conjugates` keyword.
    Keyword,
    /// The `~` operator.
    Operator,
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
    /// `differences`.
    Differences,
}

/// One ordered [`FeatureRelationshipPart`](https://www.omg.org/spec/KerML/)
/// on a [`KermlFeature`]'s declaration tail.
///
/// KerML `FeatureDeclaration` ends in `FeatureRelationshipPart*`.  Keeping the
/// alternatives in one ordered list is therefore essential: `featured by B
/// inverse of A::f` is not two independent optional slots, and any alternative
/// may repeat.  References remain identities in the document's packed
/// [`QualifiedReferenceArena`](crate::ast::QualifiedReferenceArena); this enum
/// deliberately owns no copied spelling.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeatureRelationshipPart {
    /// `TypeRelationshipPart` (`unions`, `intersects`, `disjoint from`, or
    /// `differences`).
    TypeRelationship(Node<KermlTypeRelationship>),
    /// `chains` followed by its feature chain.
    Chaining {
        target: crate::ast::QualifiedReferenceId,
    },
    /// `inverse of` followed by its feature chain.
    Inverting {
        target: crate::ast::QualifiedReferenceId,
    },
    /// `featured by` followed by one or more `OwnedTypeFeaturing` targets.
    TypeFeaturing(Node<TypeFeaturingPart>),
}

/// The source-backed target sequence of one `featured by` relationship part.
///
/// `TypeFeaturingPart = 'featured' 'by' OwnedTypeFeaturing ( ','
/// OwnedTypeFeaturing )*` in the pinned KerML grammar.  Each target is a
/// document-local qualified-reference identity; its spelling, separators, and
/// source span live in the document arena rather than in a copied string.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeFeaturingPart {
    /// Nonempty, authored-order `OwnedTypeFeaturing` targets.
    pub targets: Vec<crate::ast::QualifiedReferenceId>,
}

impl KermlTypeRelationshipKeyword {
    /// The authored keyword spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DisjointFrom => "disjoint from",
            Self::Unions => "unions",
            Self::Intersects => "intersects",
            Self::Differences => "differences",
        }
    }
}

/// KerML feature member inside a type body, e.g. `derived var feature annotatedElement :
/// Element[1..*] ordered redefines annotatedElement;` (Kernel Semantic Library `KerML.kerml`)
/// or `feature all spaceShotOf: Occurrence[0..*] subsets spaceSliceOf inverse of spaceShots
/// { ... }` (`Occurrences.kerml`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlFeature {
    /// `member` prefix (KerML `TypeFeatureMember`'s discriminator, BNF 523). On the membership,
    /// ahead of the whole prefix -- `member abstract feature carSpeed : Real;` -- so it is not a
    /// [`FeaturePrefix`](crate::ast::FeaturePrefix) slot.
    pub is_member: bool,
    /// `FeaturePrefix` (KerML BNF 584): the `EndFeaturePrefix | BasicFeaturePrefix` choice and its
    /// trailing prefix metadata keywords, as the grammar's own nesting.
    ///
    /// Replaces the seven independent booleans this node used to carry. Those made `var const
    /// feature b;` representable (and, before this seam, *accepted*) and put end-ness in two
    /// places at once; see `planning/kerml-feature-prefix-matrix.md` §5.
    pub prefix: crate::ast::FeaturePrefix,
    /// The feature-kind keyword: `feature`, `step`, `expr`, or `bool`, with its exact span.
    ///
    /// `None` for the keyword-less prefixed forms (`portion redefines portionOfLife = ...;`,
    /// Kernel Semantic Library `Occurrences.kerml`), where `Feature` is implied. Presence *is*
    /// "the keyword was authored", so there is no separate flag to disagree with the value.
    pub kind: Option<Node<KermlFeatureKind>>,
    /// `all` after the kind keyword (KerML `isSufficient`).
    pub is_all: bool,
    /// Declared name (may be quoted, e.g. `'in'`). Empty for the redefinition-led form
    /// (`portion feature redefines spaceBoundary [1];`).
    pub name: Option<DeclarationName>,
    /// Ordered `FeatureSpecialization+` alternatives. Clause identity is retained independently
    /// from each clause's target list, so `crosses a crosses b` is not collapsed into the same
    /// syntax as `crosses a, b`.
    pub specializations: Vec<FeatureSpecialization>,
    /// Multiplicity clause, accepted before or after the typing (and after a leading
    /// redefinition target).
    pub multiplicity: Option<Node<crate::ast::Multiplicity>>,
    /// `MultiplicityPart`'s `isOrdered`/`isUnique` keyword slots, each carrying the authored
    /// spelling and its exact span. See [`MultiplicityModifiers`](crate::ast::MultiplicityModifiers).
    pub multiplicity_modifiers: crate::ast::MultiplicityModifiers,
    /// Ordered `FeatureRelationshipPart*` declaration tail. This replaces the
    /// superseded fixed `chains`, `inverse_of`, and `type_relationships` slots:
    /// the grammar permits any alternative to repeat and interleave.
    pub relationship_parts: Vec<Node<FeatureRelationshipPart>>,
    /// Value clause: `= expr` / `:= expr` / `default (=|:=)? expr`.
    pub value: Option<Node<crate::ast::FeatureValue>>,
    /// Body following the shared type-body member grammar: `;` or `{ ... }`.
    pub body: crate::ast::CalcDefBody,
    pub membership: crate::ast::Membership,
}

/// One ordered alternative of KerML `FeatureSpecialization` (BNF 643).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeatureSpecialization {
    Typing(Node<crate::ast::TypingRelationship>),
    Subsetting {
        relationship: Node<crate::ast::SubsettingRelationship>,
        value: Option<Node<crate::ast::Expression>>,
    },
    ReferenceSubsetting(Node<crate::ast::SubsettingRelationship>),
    CrossSubsetting(Node<crate::ast::SubsettingRelationship>),
    Redefinition(Node<crate::ast::SubsettingRelationship>),
}

/// The kind keyword of a [`KermlFeature`].
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
    pub name: Option<DeclarationName>,
    /// Body holding the invariant's boolean expression(s) via the shared type-body grammar.
    pub body: crate::ast::CalcDefBody,
    pub membership: crate::ast::Membership,
}

/// One end of a KerML connector/binding/succession member: optional `[multiplicity]` plus a
/// (possibly dotted) feature chain, e.g. `[1] self` or `onOccurrence.startingAt.startShot`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlConnectorEnd {
    pub multiplicity: Option<Node<crate::ast::Multiplicity>>,
    pub target: crate::ast::QualifiedReferenceId,
    /// `references` chain on the end, e.g. `from [0..*] separateOccurrenceToo references
    /// elements.notIntersection to ...` (Kernel Semantic Library `Occurrences.kerml`).
    pub references: Option<crate::ast::QualifiedReferenceId>,
}

/// KerML connector member in a type body: `connector` `all`? name? `:` type multiplicity?
/// (`from` end `to` end)? body, e.g. `connector :HappensDuring from [1] self to [1] this;` or
/// `private connector all during: HappensDuring[0..1] from self to occ;` (Kernel Semantic
/// Library `Occurrences.kerml`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlConnectorMember {
    pub is_all: bool,
    /// Declared name; empty for the anonymous `connector :Type` form.
    pub name: Option<DeclarationName>,
    /// `:` type target.
    pub typing: Option<crate::ast::QualifiedReferenceId>,
    pub multiplicity: Option<Node<crate::ast::Multiplicity>>,
    /// `from`/`to` ends when written.
    pub from: Option<Node<KermlConnectorEnd>>,
    pub to: Option<Node<KermlConnectorEnd>>,
    /// Body following the shared type-body member grammar: `;` or `{ ... }`.
    pub body: crate::ast::CalcDefBody,
    pub membership: crate::ast::Membership,
}

/// The optional inline binary ends of a KerML [`KermlBindingMember`]. Connector ends declared in
/// the binding's type body remain body-owned [`KermlFeature`] nodes instead.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlBindingEndPair {
    /// Authored `of` introducer. Required after a declared binding name and optional for the
    /// anonymous alternative.
    pub of_span: Option<Span>,
    pub left: Node<KermlConnectorEnd>,
    pub equals_span: Span,
    pub right: Node<KermlConnectorEnd>,
}

/// KerML binding connector member: `binding` followed by a feature declaration and optional
/// inline `of` end `=` end pair, or `all` and an optional anonymous pair, then a type body.
/// Besides `binding [1] startShot = [1] endShot;`, this represents declaration-only connectors
/// whose ends are body features: `binding tern { end feature e1; end feature e2; }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlBindingMember {
    /// Exact `all` keyword span on the anonymous sufficient alternative.
    pub all_span: Option<Span>,
    /// Declared name; empty for the anonymous alternative.
    pub name: Option<DeclarationName>,
    /// Multiplicity in the supported feature-declaration head (`binding instant[instantNum] of
    /// ...`, `Triggers.kerml`).
    pub multiplicity: Option<Node<crate::ast::Multiplicity>>,
    /// Optional declaration-level binary connector ends. Absence is distinct from an empty body:
    /// body-owned `end feature` members may supply any number of connector ends.
    pub inline_ends: Option<Node<KermlBindingEndPair>>,
    /// KerML `TypeBody`, represented by the shared typed calc/type-body member grammar.
    pub body: crate::ast::CalcDefBody,
    pub membership: crate::ast::Membership,
}

/// KerML succession member: `succession` end `then` end `;`, e.g. `succession [1] ifTest then
/// [0..1] elseClause;` (Kernel Semantic Library `ControlPerformances.kerml`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlSuccessionMember {
    /// `all` sufficiency (`private succession all [*] trigger then [*] guard;`,
    /// `TransitionPerformances.kerml`).
    pub is_all: bool,
    /// Declared succession name, present only with the `first` keyword form (`succession
    /// triggerAfter [taNum] first [0..1] transitionLinkSource then ...;`); empty otherwise.
    pub name: Option<DeclarationName>,
    /// The succession's own multiplicity in the named `first` form (`[taNum]` above).
    pub multiplicity: Option<Node<crate::ast::Multiplicity>>,
    pub first: Node<KermlConnectorEnd>,
    pub then: Node<KermlConnectorEnd>,
    pub membership: crate::ast::Membership,
}

/// KerML explicit relationship declaration (BNF §8.2.4: `Specialization`,
/// `Subclassification`, `FeatureTyping`, `Subsetting`, `Redefinition`, `Disjoining`,
/// `FeatureInverting`, `TypeFeaturing`), e.g. `specialization S subclassifier A specializes
/// B;`, `typing t typed x by T;`, `subset parent subsets f;`, `disjoining d1 disjoint A from
/// B;`, `featuring F of y by C;` (spec42 gap 22).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KermlRelationshipDecl {
    /// The declaration-introducer keyword pair.
    pub keyword: KermlRelationshipKeyword,
    /// The authored `specialization` / `disjoining` / `inverting` declaration keyword, when
    /// written: `( 'specialization' Identification? )?` (KerML BNF 442) admits the keyword
    /// with no identification (`specialization subtype x :> y;`, `Types.kerml`), so its
    /// presence is a fact of its own rather than an inference from the identification.
    pub declaration_keyword_span: Option<crate::ast::Span>,
    /// Optional declared identification (`specialization S ...`, `disjoining d1 ...`,
    /// `inverting i ...`, `featuring F of ...`).
    pub identification: Option<crate::ast::Identification>,
    /// The specific/typed/subsetting/disjoined/inverted/featured element (a possibly dotted
    /// feature chain).
    pub source: crate::ast::QualifiedReferenceId,
    /// The general/type/subsetted/disjoining/inverting/featuring element.
    pub target: crate::ast::QualifiedReferenceId,
    /// `;` or the annotation-only `RelationshipBody`.
    pub body: Option<Vec<Node<crate::ast::RelationshipBodyElement>>>,
    pub membership: crate::ast::Membership,
}

/// Which explicit relationship a [`KermlRelationshipDecl`] declares. Variants carry the
/// authored introducer keyword; the connective between source and target is fixed per variant
/// (`specializes`/`:>`, `typed by`/`:`, `subsets`/`:>`, `redefines`/`:>>`, `from`, `of`,
/// `by`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KermlRelationshipKeyword {
    /// `subtype a specializes b` (`Specialization`).
    Subtype,
    /// `subclassifier a specializes b` (`Subclassification`).
    Subclassifier,
    /// `typing a typed by b` (`FeatureTyping`).
    Typing,
    /// `subset a subsets b` (`Subsetting`).
    Subset,
    /// `redefinition a redefines b` (`Redefinition`).
    Redefinition,
    /// `disjoint a from b` (`Disjoining`).
    Disjoint,
    /// `inverse a of b` (`FeatureInverting`).
    Inverse,
    /// `conjugate a conjugates b` (`Conjugation`, KerML BNF 463-475): `conjugation c1 conjugate
    /// Conjugate1 conjugates Original;` (`Types.kerml`).
    Conjugate,
    /// `featuring (I of)? a by b` (`TypeFeaturing`).
    Featuring,
}

impl KermlRelationshipKeyword {
    /// The authored introducer keyword spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Subtype => "subtype",
            Self::Subclassifier => "subclassifier",
            Self::Typing => "typing",
            Self::Subset => "subset",
            Self::Redefinition => "redefinition",
            Self::Disjoint => "disjoint",
            Self::Inverse => "inverse",
            Self::Featuring => "featuring",
            Self::Conjugate => "conjugate",
        }
    }
}
