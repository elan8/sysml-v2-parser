use super::behavior::{AssignStmt, ForLoop, InOut, ThenAction};
use super::common::TextualRepresentation;
use super::common::{ConnectBody, DocComment, Identification, Import, ParseErrorNode, Visibility};
use super::feature_value::FeatureValue;
use super::membership::Membership;
use super::structure::RelationshipBodyElement;
use super::structure::{
    Annotation, AttributeBody, AttributeDef, AttributeUsage, MetadataAnnotation,
    MetadataKeywordUsage, VariantUsage,
};
use super::view::{CalcUsage, ConstraintDefBodyElement, ConstraintUsage};
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RequirementDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<RequirementDefBodyElement>>,
    },
}

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
    /// Unmodeled requirement-body element captured as raw text (used for library parsing).
    Other(String),
    Annotation(Node<Annotation>),
    MetadataAnnotation(Node<MetadataAnnotation>),
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
    TextualRep(Node<TextualRepresentation>),
    Doc(Node<DocComment>),
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
    pub body: RequireConstraintBody,
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

/// Require constraint body: `;` or `{` ConstraintDefBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RequireConstraintBody {
    Semicolon,
    Brace {
        elements: Vec<Node<ConstraintDefBodyElement>>,
    },
}

/// Requirement usage / Satisfy. Example: `satisfy EnduranceReq by droneInstance;`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Satisfy {
    pub source: Node<Expression>,
    pub target: Node<Expression>,
    pub body: ConnectBody,
    /// Structured elements from a braced satisfy body, e.g. constraint members.
    /// `None` when the body is a semicolon terminator.
    pub body_elements: Option<Vec<Node<ConstraintDefBodyElement>>>,
    /// `true` for a negated satisfy usage: `(assert)? not satisfy ...`.
    pub is_negated: bool,
    /// Present for the fuller `satisfy requirement <name> : <Type> by <expr>;` form
    /// (`SatisfyRequirementUsage` in the KerML grammar); `None` for the bare
    /// `satisfy <ref> (by <expr>)?;` shorthand.
    pub inline_requirement: Option<InlineSatisfyRequirement>,
}

/// Inline named/typed requirement usage after `satisfy requirement`, e.g.
/// `satisfy requirement myReq : ReqType by someExpr;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InlineSatisfyRequirement {
    pub name: String,
    pub type_name: Option<QualifiedReferenceId>,
}

/// Bare requirement Usage.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequirementUsage {
    pub name: String,
    /// Short name from `< ... >` when present (e.g. `requirement <'1.1'> vehicleMass1 : …`).
    pub short_name: Option<String>,
    pub type_name: Option<QualifiedReferenceId>,
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
    /// Leading `abstract` keyword (BNF `RefPrefix`), e.g. the package-level `abstract item
    /// items : Item[0..*] nonunique :> objects { ... }` (Systems Library `Items.sysml`).
    pub is_abstract: bool,
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
    /// `ordered` keyword from `MultiplicityPart`. Previously skipped and discarded.
    pub ordered: bool,
    /// `nonunique` keyword from `MultiplicityPart` (`Item[0..*] nonunique`, Systems Library
    /// `Items.sysml`). See `ordered`.
    pub nonunique: bool,
    /// Value expression (`= expr`, `default = expr`, `:= expr`), e.g. `new Box(...)`.
    pub value: Option<Node<FeatureValue>>,
    pub body: AttributeBody,
    /// Set when parsed as `in`/`out`/`inout item` in port def bodies.
    pub direction: Option<InOut>,
    /// Leading `individual` keyword (BNF `OccurrenceUsagePrefix`, GH-90.1), e.g. `individual
    /// item ii : II1;` (`Simple Tests/IndividualTest.sysml:4`).
    pub is_individual: bool,
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
    pub body: ConnectBody,
    /// Braced-body members (BNF `RelationshipBody`: the doc/comment/metadata annotation subset
    /// plus owned feature members, spec42 Gap 37). `None` when the body is a semicolon
    /// terminator.
    pub body_elements: Option<Vec<Node<RelationshipBodyElement>>>,
}

/// Framed concern member in requirement body: `frame` name (`;` or body).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameMember {
    pub name: String,
    pub body: RequirementDefBody,
}

/// Concern usage at package level: `concern` name (`:` type)? RequirementBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConcernUsage {
    pub name: String,
    pub type_name: Option<QualifiedReferenceId>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UseCaseDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<UseCaseDefBodyElement>>,
    },
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReturnRefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<ReturnRefBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReturnRefBodyElement {
    Doc(Node<crate::ast::DocComment>),
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
    /// Unmodeled use-case / analysis-case body element captured as raw text (used for library parsing).
    Other(String),
    Annotation(Node<Annotation>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    AttributeDef(Node<AttributeDef>),
    Doc(Node<DocComment>),
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
    pub type_name: QualifiedReferenceId,
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
