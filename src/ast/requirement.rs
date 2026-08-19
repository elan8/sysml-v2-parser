use super::behavior::{AssignStmt, ForLoop, ThenAction};
use super::body::Body;
use super::common::{AnnotatingMember, Identification, Import, ParseErrorNode, Visibility};
use super::feature_value::FeatureValue;
use super::membership::Membership;
use super::structure::RelationshipBodyElement;
use super::structure::{
    AttributeBody, AttributeDef, AttributeUsage, MetadataKeywordUsage, VariantUsage,
};
use super::view::{CalcUsage, ConstraintDefBody, ConstraintUsage};
use crate::ast::core::{
    Expression, Multiplicity, Node, Span, SubsettingRelationship, TypingRelationship,
};
use crate::ast::QualifiedReferenceId;

/// Requirement definition: `requirement def` Identification (`:>` specializes)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequirementDef {
    pub identification: Identification,
    /// Supertype after `:>`, e.g. Some("UserRequirement") for `requirement def Need :> UserRequirement`.
    pub specializes: Option<Node<TypingRelationship>>,
    /// True for `abstract requirement def ...`.
    pub is_abstract: bool,
    pub body: RequirementDefBody,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006), `kind` always
    /// [`crate::ast::MembershipKind::OwningMembership`]. Genuine new grammar coverage (not just
    /// discarded data): `requirement_def` did not previously accept a `private`/`protected`/
    /// `public` prefix -- same gap class found repeatedly in this rollout (see
    /// `crate::ast::PortDef::membership`).
    pub membership: Membership,
}

/// Body of an requirement definition: `;` or `{` RequirementDefBodyElement* `}`.
pub type RequirementDefBody = Body<RequirementDefBodyElement>;

// `AttributeUsage` carries a `Membership` plus several relationship nodes, making it inherently
// larger than sibling variants like `Doc`/`Error`; boxing just this one variant in just this one
// enum would be inconsistent with the ~10 other body-element enums sharing the same
// `AttributeUsage(Node<AttributeUsage>)` shape crate-wide, so the size difference is accepted here
// rather than partially addressed.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RequirementDefBodyElement {
    Error(Node<ParseErrorNode>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    /// A dependency owned by this requirement definition.
    ///
    /// `RequirementBody → DefinitionBodyItem → DefinitionMember → DefinitionElement →
    /// Dependency`, so `dependency X to Y;` -- and the `PrefixMetadataAnnotation`-tagged
    /// `#refinement dependency X to Y;` the Apollo model writes here -- are ordinary members of
    /// this scope. Neither was dispatched, so the whole statement was swallowed by the opaque
    /// `#` fallback that owned `AnnotationHead::Opaque`.
    Dependency(Node<Dependency>),
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    Import(Node<Import>),
    SubjectDecl(Node<SubjectDecl>),
    /// `subject;` shorthand (concern / viewpoint bodies; validation `11a`).
    SubjectRef(Node<SubjectRef>),
    RequirementActorDecl(Node<RequirementActorDecl>),
    /// Composite requirement usage nested in a requirement definition or usage.
    ///
    /// Boxed because a `RequirementUsage` owns a `RequirementDefBody`, which may contain
    /// further requirement usages.
    RequirementUsage(Box<Node<RequirementUsage>>),
    /// Nested requirement definition (`requirement def ...`) from DefinitionBodyItem.
    RequirementDef(Box<Node<RequirementDef>>),
    Stakeholder(Node<StakeholderMember>),
    Purpose(Node<PurposeMember>),
    AttributeDef(Node<AttributeDef>),
    AttributeUsage(Node<AttributeUsage>),
    /// `variant name;` / typed variant inside a `variation requirement` body (validation `7b`).
    VariantUsage(Node<VariantUsage>),
    VerifyRequirement(Node<VerifyRequirementMember>),
    RequireConstraint(Node<RequireConstraint>),
    /// A bare `constraint` member nested inside a `requirement def { ... }` body -- distinct
    /// from `RequireConstraint`, which handles the `assume`/`require`-prefixed forms. Real
    /// usage: the Systems Library's `RequirementCheck` redefining `RequirementConstraintCheck`'s
    /// `assumptions`/`constraints` (`constraint assumptions :>> RequirementConstraintCheck::assumptions;`).
    Constraint(Node<ConstraintUsage>),
    Frame(Node<FrameMember>),
    /// `ref`-prefixed feature declaration, e.g. `ref concern :>> self: ConcernCheck;` and `ref
    /// part actors : Part[0..*] { ... }` (Systems Library `Requirements.sysml`). This scope
    /// accepted no `ref` member at all.
    RefDecl(Node<crate::ast::RefDecl>),
    /// Nested concern usage, e.g. `abstract concern concerns[0..*] :> concernChecks { ... }`
    /// (`Requirements.sysml`). Previously only reachable at package level.
    ConcernUsage(Node<ConcernUsage>),
    /// Nested calc usage, e.g. `in calc eval : EvaluationFunction { ... }` and `in calc :>> eval =
    /// evaluationFunction;` (`sysml.library/Domain Libraries/Analysis/TradeStudies.sysml`).
    CalcUsage(Box<Node<CalcUsage>>),
    /// Port usage admitted through DefinitionBodyItem.
    PortUsage(Box<Node<crate::ast::PortUsage>>),
    /// Allocation usage admitted through DefinitionBodyItem.
    AllocationUsage(Box<Node<crate::ast::AllocationUsage>>),
    /// `SatisfyRequirementUsage` as a member of this body.
    ///
    /// `RequirementBodyItem → DefinitionBodyItem → … → BehaviorUsageElement →
    /// SatisfyRequirementUsage`, so a satisfy usage is an ordinary member of every requirement
    /// body -- including the `RequirementBody` a satisfy usage owns itself, which is why this is
    /// boxed.
    Satisfy(Box<Node<SatisfyRequirementUsage>>),
}

/// Viewpoint stakeholder: typed declaration, shorthand concern reference, or `:>>` redefinition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakeholderMember {
    /// Declaration label for `stakeholder name : Type;`; empty for reference forms.
    pub declaration_name: String,
    /// Concern reference for `stakeholder Concern;` and `stakeholder :>> Concern;`.
    pub target: Option<QualifiedReferenceId>,
    pub type_name: Option<QualifiedReferenceId>,
    /// True for `stakeholder :>> name;` (validation `11a`).
    pub is_redefinition: bool,
}

/// Viewpoint purpose concern reference.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PurposeMember {
    pub target: QualifiedReferenceId,
}

impl PartialEq for PurposeMember {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target
    }
}

/// Subject declaration: `subject` name? (`:` type)? multiplicity? (`=` value)? `;`
/// or the bare binding `subject = expr;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubjectDecl {
    pub name: String,
    pub type_name: Option<QualifiedReferenceId>,
    /// `:>>`/`redefines` redefinition clause (`subject subj :>> Case::subj;`, or the type-less
    /// anonymous form `subject :>> vehicle = vehicle_large;`; spec42 Gap 35).
    pub redefines: Option<Node<crate::ast::SubsettingRelationship>>,
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `= expr` / `default expr` value clause (`subject generateTorque default
    /// engine1.generateTorque;`, OMG spec Annex A; spec42 Gap 35 widened this from a bare
    /// `=`-only `Expression`).
    pub value: Option<Node<crate::ast::FeatureValue>>,
}

/// Actor parameter in a requirement body: `actor` name? `:` type `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequirementActorDecl {
    pub name: String,
    pub type_name: QualifiedReferenceId,
    pub multiplicity: Option<Node<Multiplicity>>,
}

/// Require/assume constraint: `(require|assume) constraint` name? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequireConstraint {
    /// True when spelled `assume` rather than `require`.
    pub is_assume: bool,
    /// True when the `constraint` keyword follows `require`/`assume`.
    /// False for `require name;` / `require name { … }` (validation `08`).
    pub has_constraint_keyword: bool,
    /// Optional usage name (`assume constraint fuelConstraint { … }`) -- the *declared* name of
    /// the `constraint`-keyword form.
    pub name: Option<String>,
    /// Arena-backed target of the keyword-less reference shorthand `require <qualified.name>;`
    /// / `assume <name>;` (spec42 gap 29): the referenced constraint, resolvable through the
    /// document's qualified-reference table. `None` for the `constraint`-keyword declaration
    /// form, whose `name` declares rather than references.
    pub target: Option<crate::ast::QualifiedReferenceId>,
    pub body: ConstraintDefBody,
}

/// Requirement verification usage in requirement/objective bodies:
/// `verify requirement <...>` or shorthand `verify <qualified_name>;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerifyRequirementMember {
    /// True for `verify requirement ...`; false for shorthand `verify ...;`.
    pub explicit_requirement_keyword: bool,
    /// Parsed requirement usage when explicit form is used.
    pub requirement: Option<Node<RequirementUsage>>,
    /// Shorthand verified requirement reference (`verify QualifiedName;`).
    pub target: Option<QualifiedReferenceId>,
    /// Redefinition target after `:>>` (`verify vehicleMassRequirement :>> massRequirement;`).
    pub redefines: Option<QualifiedReferenceId>,
}

/// `SatisfyRequirementUsage` (SysML v2 textual notation, clause 8.2.2.21.2).
///
/// ```text
/// SatisfyRequirementUsage =
///     OccurrenceUsagePrefix 'assert' ( isNegated ?= 'not' ) 'satisfy'
///     ( ownedRelationship += OwnedReferenceSubsetting
///       FeatureSpecializationPart?
///     | 'requirement' UsageDeclaration )
///     ValuePart?
///     ( 'by' ownedRelationship += SatisfactionSubjectMember )?
///     RequirementBody
/// ```
///
/// # Two documented departures from the pinned production text
///
/// The pinned `SysML-textual-bnf.kebnf` writes `'assert' ( isNegated ?= 'not' )` without the
/// optionality markers the same file uses everywhere else (compare `AssertConstraintUsage`, which
/// writes `'assert' ( isNegated ?= 'not' )?`, and `LibraryPackage`, which writes
/// `( isStandard ?= 'standard' ) 'library'` for a prefix that is certainly optional). The pinned
/// corpus that ships in the same release settles it: `Simple Tests/RequirementTest.sysml` writes
/// all four prefix combinations -- `satisfy`, `assert satisfy`, `not satisfy`, and
/// `assert not satisfy` -- and the Systems Library's `Views.sysml` writes a bare
/// `satisfy requirement viewpointConformance by that { … }` that the strict library gate requires
/// to parse without a diagnostic. Both keywords are therefore modelled as independently optional,
/// each keeping its authored span rather than collapsing to a flag.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SatisfyRequirementUsage {
    /// The production's own first element: `OccurrenceUsagePrefix`.
    ///
    /// The shared component, not a satisfy-local one -- every `OccurrenceUsageElement` spells the
    /// same prefix, and `planning/occurrence-usage-prefix-matrix.md` records which families now
    /// use it. It precedes `assert`, so its spans all sit before
    /// [`assert_span`](Self::assert_span).
    pub prefix: crate::ast::OccurrenceUsagePrefix,
    /// `MemberPrefix`'s `visibility = VisibilityIndicator`, from
    /// `BehaviorUsageMember : FeatureMembership = MemberPrefix ownedRelatedElement +=
    /// BehaviorUsageElement`.
    ///
    /// The visibility keyword belongs to the *membership*, not to the usage, so it is a separate
    /// field rather than a slot of [`prefix`](Self::prefix) -- and it precedes the prefix in the
    /// source. `kind` is always [`crate::ast::MembershipKind::FeatureMembership`].
    pub membership: Membership,
    /// Exact `assert` keyword token, or `None` when the author omitted it.
    ///
    /// Presence is a grammatical fact emission reproduces, so it is a span rather than a flag:
    /// the source stays authoritative for where the keyword was written.
    pub assert_span: Option<Span>,
    /// Exact `not` keyword token (`isNegated`), or `None` when the author omitted it.
    pub not_span: Option<Span>,
    /// Exact `satisfy` keyword token. Always authored; kept so no consumer has to rescan the
    /// source to find where the usage's keyword sits between its optional prefixes and its
    /// requirement clause.
    pub satisfy_span: Span,
    /// Which of the production's two mutually exclusive requirement clauses was authored.
    pub requirement: SatisfiedRequirement,
    /// `FeatureSpecializationPart?`'s `Typings` clause (`: Type`, `defined by Type`, ...).
    ///
    /// The specialization part is shared by both alternatives of the requirement clause -- the
    /// reference alternative spells it directly, the declaration alternative reaches it through
    /// `UsageDeclaration` -- so it lives beside them rather than being duplicated into each.
    pub typing: Option<Node<TypingRelationship>>,
    /// `MultiplicityPart`'s `OwnedMultiplicity`, e.g. `[1]`.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `MultiplicityPart`'s `isOrdered`/`isUnique` keyword slots, each carrying the authored
    /// spelling and its exact span. See [`MultiplicityModifiers`](crate::ast::MultiplicityModifiers).
    pub multiplicity_modifiers: crate::ast::MultiplicityModifiers,
    /// `FeatureSpecialization`'s `Subsettings` clause (`:>` / `subsets`).
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// `FeatureSpecialization`'s `Redefinitions` clause (`:>>` / `redefines`).
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// `FeatureSpecialization`'s `References` clause (`::>` / `references`).
    pub references: Option<Node<SubsettingRelationship>>,
    /// `FeatureSpecialization`'s `Crosses` clause (`crosses`).
    pub crosses: Option<Node<SubsettingRelationship>>,
    /// `ValuePart?` -- an `=` / `:=` / `default` binding on the satisfied requirement.
    pub value: Option<Node<FeatureValue>>,
    /// `( 'by' ownedRelationship += SatisfactionSubjectMember )?`.
    ///
    /// `None` means the author wrote no `by` clause, and emission writes none. There is no
    /// separate "had a by clause" flag, and the satisfied requirement is never copied here to
    /// fabricate one.
    pub subject: Option<Node<SatisfactionSubject>>,
    /// `RequirementBody` -- the same `';' | '{' RequirementBodyItem* '}'` a `requirement def`
    /// owns, so requirement-specific members such as `require`, `assume`, `frame`, `subject`, and
    /// a nested `requirement` usage are members of this body.
    pub body: RequirementDefBody,
}

/// The two mutually exclusive spellings of `SatisfyRequirementUsage`'s requirement clause.
///
/// They are different grammar alternatives, not one shape with optional parts: the first *refers*
/// to a requirement that exists elsewhere, the second *declares* one inline. A declaration label
/// is never stored as a reference identity, and a reference is never synthesized from a label.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SatisfiedRequirement {
    /// `ownedRelationship += OwnedReferenceSubsetting`, i.e. `[QualifiedName] | OwnedFeatureChain`
    /// -- `satisfy vehicleSpecification …`, `satisfy Requirements::engineSpecification …`.
    Reference {
        /// The referenced requirement, resolvable through the owning document's arena.
        reference: QualifiedReferenceId,
    },
    /// `'requirement' UsageDeclaration` -- `satisfy requirement req1 : Req1 …`.
    Declaration(Node<InlineRequirementDeclaration>),
}

/// `'requirement' UsageDeclaration`'s declaration half.
///
/// `UsageDeclaration = Identification FeatureSpecializationPart?`; the specialization half belongs
/// to the owning [`SatisfyRequirementUsage`] because the reference alternative spells the same
/// clauses. `Identification` itself is `( '<' NAME '>' )? ( NAME )?`, so both halves are optional
/// and `satisfy requirement by x;` declares an anonymous requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InlineRequirementDeclaration {
    /// Exact `requirement` keyword token that selects this alternative.
    pub keyword_span: Span,
    /// The declared name and short name. A declaration label, never a reference.
    pub identification: Identification,
}

/// `SatisfactionSubjectMember`, the `by` clause's payload.
///
/// ```text
/// SatisfactionSubjectMember     : SubjectMembership        = ownedRelatedElement += SatisfactionParameter
/// SatisfactionParameter         : ReferenceUsage           = ownedRelationship  += SatisfactionFeatureValue
/// SatisfactionFeatureValue      : FeatureValue             = ownedRelatedElement += SatisfactionReferenceExpression
/// SatisfactionReferenceExpression : FeatureReferenceExpression = ownedRelationship += FeatureChainMember
/// FeatureChainMember            : Membership               = memberElement = [QualifiedName] | OwnedFeatureChainMember
/// ```
///
/// The chain bottoms out at a qualified name or an owned feature chain, so the subject is a
/// source-backed reference with typed `::`/`.` separators -- not a general expression.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SatisfactionSubject {
    /// Exact `by` keyword token.
    pub by_span: Span,
    /// The subject feature: `[QualifiedName]` or `OwnedFeatureChain`.
    pub reference: QualifiedReferenceId,
}

/// Bare requirement Usage.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequirementUsage {
    pub name: String,
    /// Short name from `< ... >` when present (e.g. `requirement <'1.1'> vehicleMass1 : …`).
    pub short_name: Option<String>,
    pub type_name: Option<QualifiedReferenceId>,
    pub multiplicity: Option<Node<Multiplicity>>,
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// Reference subsetting after `::>` / `references` (validation `08`:
    /// `requirement references vehicleMass1 { … }`).
    pub references: Option<Node<SubsettingRelationship>>,
    /// True for `abstract requirement ...`.
    pub is_abstract: bool,
    /// True for `variation requirement ...` (§6 G5) — a variation point whose body holds
    /// `variant` members.
    pub is_variation: bool,
    /// `= expr` binding (`in requirement r = cityFuelEconomyRequirement;`, validation `10c`).
    pub value: Option<Node<FeatureValue>>,
    /// Set when parsed as `in`/`out`/`inout requirement` (validation `10c`).
    pub direction: Option<crate::ast::InOut>,
    pub body: RequirementDefBody,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006). `kind` is always
    /// [`crate::ast::MembershipKind::FeatureMembership`]. Only captured with real visibility for
    /// the top-level `requirement_usage` member-position parser; the shared
    /// `parse_requirement_usage_payload*` helper used inside `verify requirement ...` and
    /// `objective { requirement ... }` (neither has visibility syntax of its own) always sets
    /// `visibility: None`, matching this rollout's convention for ad hoc, no-visibility-grammar
    /// construction sites (see `AttributeUsage`'s three ad hoc sites in the first `Membership`
    /// increment).
    pub membership: Membership,
}

/// Item usage inside a part definition body: `item` (name | `:>>` redefines)? multiplicity? (`:`
/// type)? (`=` value)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemUsage {
    /// The complete `OccurrenceUsagePrefix` this usage was written with
    /// (`ItemUsage = OccurrenceUsagePrefix 'item' Usage`, SysML BNF 616).
    ///
    /// The same shared component `OccurrenceUsage` and `SatisfyRequirementUsage` carry; see
    /// `planning/occurrence-usage-prefix-matrix.md`. It replaced five independent fields
    /// (`is_derived`, `usage_prefix`, `is_constant`, `direction`, `is_individual`) that between
    /// them kept no authored span and represented neither `ref` nor a `PortionKind` nor a
    /// `UsageExtensionKeyword`, so `ref individual item :>> driver : Alice;` (`training/28.
    /// Individuals/Individuals and Time Slices.sysml:10`) was read as an occurrence usage
    /// *named* `item` and re-emitted as one.
    pub prefix: crate::ast::OccurrenceUsagePrefix,
    /// Empty for the anonymous redefinition form (`item :>> shape : Cylinder { ... }`), matching
    /// `PartUsage::name`'s existing convention.
    pub name: String,
    pub type_name: Option<QualifiedReferenceId>,
    /// Redefines target, e.g. `shape` in `item :>> shape : Cylinder { ... }`. `None` for the
    /// ordinary named form. Confirmed real usage in the OMG Geometry domain library's
    /// `VehicleGeometryAndCoordinateFrames.sysml` example (`item :>> shape = new Box(...);` and
    /// `item :>> shape : Cylinder { ... }`) -- previously unparseable, falling through to opaque
    /// body-element recovery.
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// `:>` subsets clause, e.g. `:> objects` (Systems Library `Items.sysml`). Previously
    /// parsed by the shared usage header and discarded.
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// Short name from `< ... >` when present. See `crate::ast::AttributeUsage::short_name`.
    pub short_name: Option<String>,
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `MultiplicityPart`'s `isOrdered`/`isUnique` keyword slots, each carrying the authored
    /// spelling and its exact span. See [`MultiplicityModifiers`](crate::ast::MultiplicityModifiers).
    pub multiplicity_modifiers: crate::ast::MultiplicityModifiers,
    /// Value expression (`= expr`, `default = expr`, `:= expr`), e.g. `new Box(...)`.
    pub value: Option<Node<FeatureValue>>,
    pub body: AttributeBody,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006), `kind` always
    /// [`crate::ast::MembershipKind::FeatureMembership`]. See
    /// [`crate::ast::PortDef::membership`] for the same "genuine new grammar coverage, not just
    /// discarded data" rationale -- `item_usage` did not previously accept a visibility prefix
    /// either.
    pub membership: Membership,
}

/// Enumeration usage inside a definition body: `enum` name (`:` type)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumerationUsage {
    pub name: String,
    pub type_name: Option<QualifiedReferenceId>,
    pub multiplicity: Option<Node<Multiplicity>>,
    pub body: AttributeBody,
    /// `end` keyword from `EndUsagePrefix` (BNF §8.2.2.6.2, `isEnd ?= 'end'`), reached through
    /// the same `UsagePrefix` production `AttributeUsage.is_end` uses (`UsagePrefix 'enum'
    /// Usage`). See `structure::AttributeUsage::is_end` for the full BNF citation.
    pub is_end: bool,
    pub membership: Membership,
}

/// Dependency: `dependency` (Identification `from`)? client(s) `to` supplier(s) RelationshipBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dependency {
    pub identification: Option<Identification>,
    pub clients: Vec<QualifiedReferenceId>,
    pub suppliers: Vec<QualifiedReferenceId>,
    /// `Dependency = ( PrefixMetadataAnnotation )* 'dependency' DependencyDeclaration
    /// RelationshipBody`, whose members are the annotating production plus owned related
    /// elements. Was a `ConnectBody` marker with no delimiter spans beside a separate
    /// `body_elements` list -- one body fact in two fields.
    pub body: crate::ast::Body<RelationshipBodyElement>,
}

/// Framed concern member in requirement body: `frame` name (`;` or body).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameMember {
    pub has_concern_keyword: bool,
    pub name: String,
    pub short_name: Option<String>,
    pub type_name: Option<QualifiedReferenceId>,
    pub multiplicity: Option<Node<Multiplicity>>,
    pub subsets: Option<Node<SubsettingRelationship>>,
    pub redefines: Option<Node<SubsettingRelationship>>,
    pub value: Option<Node<FeatureValue>>,
    pub body: RequirementDefBody,
}

/// Concern usage at package level: `concern` name (`:` type)? RequirementBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConcernUsage {
    pub name: String,
    /// `abstract` keyword, e.g. `abstract concern concerns[0..*] :> concernChecks { ... }`
    /// (Systems Library `Requirements.sysml`). The parser has always accepted it; this struct
    /// had nowhere to put it, so emission dropped the keyword.
    pub is_abstract: bool,
    pub type_name: Option<QualifiedReferenceId>,
    /// Multiplicity after the name, e.g. `[0..*]` in `abstract concern concerns[0..*]`. Also
    /// previously parsed and discarded.
    pub multiplicity: Option<Node<Multiplicity>>,
    pub subsets: Option<Node<SubsettingRelationship>>,
    pub redefines: Option<Node<SubsettingRelationship>>,
    pub body: RequirementDefBody,
    /// True for `concern def ...`, false for a bare `concern ...` usage. `concern_usage` handles
    /// both the `concern` and `concern def` textual forms itself rather than through a separate
    /// `ConcernDef` struct (the BNF's own `ConcernDefinition` production is not modeled as a
    /// distinct struct); this flag is the sole discriminator between the two, restored so
    /// consumers can classify them distinctly instead of folding both into one semantic kind.
    pub is_definition: bool,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006), `kind` always
    /// [`crate::ast::MembershipKind::FeatureMembership`].
    pub membership: Membership,
}

/// Case definition: `case def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaseDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    /// True for `abstract case def ...`.
    pub is_abstract: bool,
    pub body: UseCaseDefBody,
    /// See [`RequirementDef::membership`]; same gap class found again for `case_def`.
    pub membership: Membership,
}

/// Case usage: `case` name (`:` type)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaseUsage {
    pub name: String,
    pub type_name: Option<QualifiedReferenceId>,
    /// Multiplicity after the type, e.g. `[0..*]` in `abstract case subcases : Case[0..*] :>
    /// cases, subcalculations { ... }` (Systems Library `Cases.sysml:56`). The declaration's tail
    /// used to be skipped wholesale, so this was dropped without a diagnostic.
    pub multiplicity: Option<Node<Multiplicity>>,
    pub subsets: Option<Node<SubsettingRelationship>>,
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// True for `abstract case ...`.
    pub is_abstract: bool,
    pub body: UseCaseDefBody,
    /// See [`RequirementUsage::membership`]; `case_usage` captures real visibility, `kind` always
    /// [`crate::ast::MembershipKind::FeatureMembership`].
    pub membership: Membership,
}

/// Analysis case definition: `analysis def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnalysisCaseDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    /// True for `abstract analysis def ...`.
    pub is_abstract: bool,
    /// `individual analysis def FuelEconomyAnalysis_1 :> FuelEconomyAnalysis;` (GH-90.1,
    /// `Individuals Examples/AnalysisIndividualExample.sysml:76`).
    pub is_individual: bool,
    pub body: UseCaseDefBody,
    /// See [`RequirementDef::membership`]; same gap class found again for `analysis_case_def`.
    pub membership: Membership,
}

/// Analysis case usage: `analysis` name (`:` type)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnalysisCaseUsage {
    pub name: String,
    pub type_name: Option<QualifiedReferenceId>,
    pub subsets: Option<Node<SubsettingRelationship>>,
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// True for `abstract analysis ...`.
    pub is_abstract: bool,
    /// Leading `individual` keyword (BNF `OccurrenceUsagePrefix`, GH-90.1), e.g. `individual
    /// analysis fuelEconomyAnalysis_1 : FuelEconomyAnalysis_1 { ... }` (`Individuals Examples/
    /// AnalysisIndividualExample.sysml:79`).
    pub is_individual: bool,
    pub body: UseCaseDefBody,
    /// See [`RequirementUsage::membership`]; `analysis_case_usage` captures real visibility,
    /// `kind` always [`crate::ast::MembershipKind::FeatureMembership`].
    pub membership: Membership,
}

/// Verification case definition: `verification def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerificationCaseDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    /// True for `abstract verification def ...`.
    pub is_abstract: bool,
    pub body: UseCaseDefBody,
    /// See [`RequirementDef::membership`]; same gap class found again for `verification_case_def`.
    pub membership: Membership,
}

/// Verification case usage: `verification` name (`:` type)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerificationCaseUsage {
    pub name: String,
    pub type_name: Option<QualifiedReferenceId>,
    /// Multiplicity after the type, e.g. `[0..*]` in `abstract verification
    /// subVerificationCases : VerificationCase[0..*] :> verificationCases, subcases { ... }`
    /// (Systems Library `VerificationCases.sysml:42`). See [`CaseUsage::multiplicity`].
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `:>` subsets clause, which may name several comma-separated targets.
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// True for `abstract verification ...`.
    pub is_abstract: bool,
    pub body: UseCaseDefBody,
    /// See [`RequirementUsage::membership`]; `verification_case_usage` captures real visibility,
    /// `kind` always [`crate::ast::MembershipKind::FeatureMembership`].
    pub membership: Membership,
}

/// Use case usage at package level: `use case` name (`:` type)? CaseBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UseCaseUsage {
    pub name: String,
    pub type_name: Option<QualifiedReferenceId>,
    /// True for `abstract use case ...`.
    pub is_abstract: bool,
    /// Multiplicity after the type, e.g. `[0..*]` in `abstract case subcases : Case[0..*] :>
    /// cases, subcalculations { ... }` (Systems Library `Cases.sysml:56`). Previously the
    /// declaration's tail was skipped wholesale, so this and `subsets` were dropped.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `:>` subsets clause, which may name several comma-separated targets.
    pub subsets: Option<Node<SubsettingRelationship>>,
    pub body: UseCaseDefBody,
    /// See [`RequirementUsage::membership`]; captures real visibility at member position
    /// (`use_case_usage`), `visibility: None` at the `then use case ...` control-flow position
    /// (`use_case_usage_in_body`, no visibility grammar there). `kind` always
    /// [`crate::ast::MembershipKind::FeatureMembership`].
    pub membership: Membership,
}

// ---------------------------------------------------------------------------
// Use Cases
// ---------------------------------------------------------------------------

/// Actor declaration: `actor` Identification `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActorDecl {
    pub identification: Identification,
}

/// Use Case definition: `use case def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UseCaseDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    /// True for `abstract use case def ...`.
    pub is_abstract: bool,
    pub body: UseCaseDefBody,
    /// See [`RequirementDef::membership`]; same gap class found again for `use_case_def`.
    pub membership: Membership,
}

pub type UseCaseDefBody = Body<UseCaseDefBodyElement>;

/// `first <name>;` inside a case/use-case body (used in SysML v2 release fixtures).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FirstSuccession {
    pub target: QualifiedReferenceId,
}

/// `then done;` inside a case/use-case body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThenDone {}

/// `include <usecase> ...` inside a case/use-case body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IncludeUseCase {
    pub target: QualifiedReferenceId,
    /// Optional multiplicity suffix like `[0..*]`, parsed into structured lower/upper bounds.
    pub multiplicity: Option<Node<Multiplicity>>,
    pub body: UseCaseDefBody,
}

/// `then include <usecase> ...` inside a case/use-case body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThenIncludeUseCase {
    pub include: Node<IncludeUseCase>,
}

/// `then use case <name> ...` inside a case/use-case body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThenUseCaseUsage {
    pub use_case: Node<UseCaseUsage>,
}

/// `subject;` shorthand used in SysML v2 release fixtures (subject of an enclosing case/use case).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubjectRef {}

/// `actor :>> <name> = <expr>;` redefinition/assignment used in SysML v2 release fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActorRedefinitionAssignment {
    pub target: QualifiedReferenceId,
    /// Structured assignment value with its exact authored expression span.
    pub value: Node<Expression>,
}

/// `ref :>> <name> { ... }` redefinition used in SysML v2 release fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RefRedefinition {
    pub target: QualifiedReferenceId,
    /// Structured nested body with an aggregate span covering its authored terminator or braces.
    pub body: Node<UseCaseDefBody>,
}

/// Optional feature-kind keyword on a case return (`return part …` / `return attribute …`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CaseReturnFeatureKind {
    Part,
    Attribute,
}

/// `return [attribute|part] [:>>] <name> [:|:> <type>] [mult] [=|:= <expr>] ;`
/// — return parameter declaration in analysis/verification case bodies.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaseReturnDecl {
    /// Declaration name for an ordinary return parameter. Empty for the `:>> target` shorthand.
    pub declaration_name: String,
    pub name_span: Option<Span>,
    /// Redefinition target for `return :>> target` / `return part :>> target`.
    pub target: Option<QualifiedReferenceId>,
    pub type_name: Option<QualifiedReferenceId>,
    /// Optional value after `=` / `:=` (validation `10d` uses `:= ()`).
    pub value: Option<Node<FeatureValue>>,
    /// True when the type is introduced with `:>` rather than `:`.
    pub is_subsetting: bool,
    /// Optional `part` / `attribute` keyword after `return`.
    pub feature_kind: Option<CaseReturnFeatureKind>,
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `:>>` clause written *after* the type on a named declaration, e.g. `return verdict :
    /// VerdictKind :>> result;` (Systems Library `VerificationCases.sysml:22`). Distinct from
    /// [`Self::target`], which is the leading anonymous form (`return :>> result;`) where the
    /// redefinition target stands in for the declaration name. The two are the same relationship
    /// written in two positions and are candidates for unification with the rest of the
    /// subsetting family; kept apart here because only `target` substitutes for the name.
    pub redefines: Option<Node<SubsettingRelationship>>,
}

impl PartialEq for CaseReturnDecl {
    fn eq(&self, other: &Self) -> bool {
        self.declaration_name == other.declaration_name
            && self.target == other.target
            && self.type_name == other.type_name
            && self.value == other.value
            && self.is_subsetting == other.is_subsetting
            && self.feature_kind == other.feature_kind
            && self.multiplicity == other.multiplicity
    }
}

/// `return ref <name><multiplicity?> { ... }` used in SysML v2 release libraries.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReturnRef {
    pub name: String,
    pub multiplicity: Option<Node<Multiplicity>>,
    pub body: Node<ReturnRefBody>,
}

pub type ReturnRefBody = Body<ReturnRefBodyElement>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::large_enum_variant)]
pub enum ReturnRefBodyElement {
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    Result(Node<Expression>),
    Error(Node<ParseErrorNode>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// `ThenAction` deliberately has the same direct-node representation here as in action bodies.
// Boxing it only in use cases would make the public AST depend on the containing body kind.
#[allow(clippy::large_enum_variant)]
pub enum UseCaseDefBodyElement {
    Error(Node<ParseErrorNode>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    AttributeDef(Node<AttributeDef>),
    SubjectDecl(Node<SubjectDecl>),
    /// `subject;` shorthand.
    SubjectRef(Node<SubjectRef>),
    ActorUsage(Node<ActorUsage>),
    ActorRedefinitionAssignment(Node<ActorRedefinitionAssignment>),
    Objective(Node<Objective>),
    FirstSuccession(Node<FirstSuccession>),
    ThenIncludeUseCase(Node<ThenIncludeUseCase>),
    ThenUseCaseUsage(Node<ThenUseCaseUsage>),
    ThenDone(Node<ThenDone>),
    IncludeUseCase(Node<IncludeUseCase>),
    RefRedefinition(Node<RefRedefinition>),
    /// Full `ref` declaration (`ref use case self : UseCase :>> Case::self;`, Systems Library
    /// `UseCases.sysml`; spec42 Gap 34). The bare `ref :>> target { ... }` shorthand stays on
    /// [`RefRedefinition`].
    Ref(Box<Node<crate::ast::RefDecl>>),
    /// Directed parameter-member shorthand with no kind keyword (`in scenario = cityScenario;`,
    /// `out voltage :> ISQ::electricPotential = ...;`), mirroring
    /// `ConstraintDefBodyElement`/`CalcDefBodyElement`'s existing wiring (spec42 Gap 45).
    InOutDecl(Box<Node<crate::ast::InOutDecl>>),
    AssertConstraint(Node<crate::ast::AssertConstraintMember>),
    ReturnRef(Node<ReturnRef>),
    CaseReturnDecl(Node<CaseReturnDecl>),
    Assign(Node<AssignStmt>),
    ForLoop(Node<ForLoop>),
    ThenAction(Node<ThenAction>),
    /// Nested `action` usage in analysis/verification case bodies (validation `09`).
    ActionUsage(Box<Node<crate::ast::ActionUsage>>),
    /// Nested `analysis` usage in analysis case bodies (validation `10a`).
    AnalysisCaseUsage(Box<Node<AnalysisCaseUsage>>),
    /// Nested `calc` usage in analysis case bodies (validation `10b`).
    CalcUsage(Box<Node<CalcUsage>>),
    /// Nested case usage: `abstract case subcases : Case[0..*] :> cases, subcalculations { ... }`
    /// (Systems Library `Cases.sysml:56`) and its `use case` / `verification` spellings. The
    /// scope previously accepted only the `then`/`include` control-flow forms, so a plain nested
    /// case member was recovered text.
    UseCaseUsage(Box<Node<UseCaseUsage>>),
    /// Nested `case` usage, e.g. `abstract case subcases : Case[0..*] :> cases, subcalculations
    /// { ... }` (Systems Library `Cases.sysml:56`).
    CaseUsage(Box<Node<CaseUsage>>),
    /// Nested `verification` usage, e.g. `abstract verification subVerificationCases :
    /// VerificationCase[0..*] :> verificationCases, subcases { ... }`
    /// (`VerificationCases.sysml:42`).
    VerificationCaseUsage(Box<Node<VerificationCaseUsage>>),
    /// `attribute` usage / directed `in attribute …` (validation `10c`/`10d`).
    AttributeUsage(Node<AttributeUsage>),
    /// Directed `in requirement …` parameter (validation `10c`).
    RequirementUsage(Box<Node<RequirementUsage>>),
    /// Directed `in part …` / nested part usage in analysis bodies.
    PartUsage(Box<Node<crate::ast::PartUsage>>),
    /// Bare result expression in analysis case bodies (validation `10a`: `vehicle.mass`).
    Expression(Node<Expression>),
    FlowUsage(Node<crate::ast::behavior::FlowUsage>),
}

/// actor usage `actor pilot : Operator;` / `actor passengers : Person[0..4];`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActorUsage {
    pub name: String,
    /// `None` for the bare untyped form `actor environment;` / `actor passenger [0..4];`
    /// (OMG spec Annex A; spec42 Gap 46).
    pub type_name: Option<QualifiedReferenceId>,
    /// Optional multiplicity after the type, e.g. `[0..4]` in `actor passengers : Person[0..4];`
    /// (validation `18-Use Case`).
    pub multiplicity: Option<Node<Multiplicity>>,
    /// Ownership/visibility/kind wrapper (parser work item 4b final sweep), `kind` always
    /// [`crate::ast::MembershipKind::ActorMembership`] -- confirmed against
    /// `SysML-textual-bnf.kebnf`'s `ActorMember : ActorMembership = MemberPrefix
    /// ownedRelatedElement += ActorUsage`, which legally carries a visibility prefix before this
    /// increment added support for parsing one.
    pub membership: Membership,
}

/// Objective `objective { doc ... }`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Objective {
    pub visibility: Option<Visibility>,
    pub requirement: Node<RequirementUsage>,
}

// ---------------------------------------------------------------------------
// State Machine
// ---------------------------------------------------------------------------
