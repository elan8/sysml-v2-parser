use super::behavior::{AssignStmt, ForLoop, InOut, ThenAction};
use super::common::{ConnectBody, DocComment, Identification, Import, ParseErrorNode, Visibility};
use super::common::TextualRepresentation;
use super::structure::{
    Annotation, AttributeBody, AttributeDef, AttributeUsage, MetadataAnnotation,
    MetadataKeywordUsage,
};
use super::membership::Membership;
use super::view::ConstraintDefBodyElement;
use crate::ast::core::{Expression, Multiplicity, Node, Span, SubsettingRelationship, TypingRelationship};

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
    RequirementActorDecl(Node<RequirementActorDecl>),
    Stakeholder(Node<StakeholderMember>),
    Purpose(Node<PurposeMember>),
    AttributeDef(Node<AttributeDef>),
    AttributeUsage(Node<AttributeUsage>),
    VerifyRequirement(Node<VerifyRequirementMember>),
    RequireConstraint(Node<RequireConstraint>),
    Frame(Node<FrameMember>),
    TextualRep(Node<TextualRepresentation>),
    Doc(Node<DocComment>),
}

/// Viewpoint stakeholder: typed parameter or shorthand concern reference.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakeholderMember {
    pub name: String,
    pub type_name: Option<String>,
    pub name_span: Span,
    pub type_span: Option<Span>,
}

/// Viewpoint purpose concern reference.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PurposeMember {
    pub target: String,
    pub target_span: Span,
}

/// Subject declaration: `subject` name `:` type `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubjectDecl {
    pub name: String,
    pub type_name: String,
}

/// Actor parameter in a requirement body: `actor` name? `:` type `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequirementActorDecl {
    pub name: String,
    pub type_name: String,
}

/// Require constraint: `require constraint { ... }`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequireConstraint {
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
    pub target: Option<String>,
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
    pub type_name: Option<String>,
}

/// Bare requirement Usage.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequirementUsage {
    pub name: String,
    pub type_name: Option<String>,
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// True for `abstract requirement ...`.
    pub is_abstract: bool,
    pub body: RequirementDefBody,
}

/// Item usage inside a part definition body: `item` name multiplicity? (`:` type)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemUsage {
    pub name: String,
    pub type_name: Option<String>,
    pub multiplicity: Option<Node<Multiplicity>>,
    pub body: AttributeBody,
    /// Set when parsed as `in`/`out`/`inout item` in port def bodies.
    pub direction: Option<InOut>,
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
    pub type_name: Option<String>,
    pub multiplicity: Option<Node<Multiplicity>>,
    pub body: AttributeBody,
    /// `end` keyword from `EndUsagePrefix` (BNF §8.2.2.6.2, `isEnd ?= 'end'`), reached through
    /// the same `UsagePrefix` production `AttributeUsage.is_end` uses (`UsagePrefix 'enum'
    /// Usage`). See `structure::AttributeUsage::is_end` for the full BNF citation.
    pub is_end: bool,
}

/// Dependency: `dependency` (Identification `from`)? client(s) `to` supplier(s) RelationshipBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Dependency {
    pub identification: Option<Identification>,
    pub clients: Vec<String>,
    pub suppliers: Vec<String>,
    pub body: ConnectBody,
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
    pub type_name: Option<String>,
    pub body: RequirementDefBody,
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
}

/// Case usage: `case` name (`:` type)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaseUsage {
    pub name: String,
    pub type_name: Option<String>,
    /// True for `abstract case ...`.
    pub is_abstract: bool,
    pub body: UseCaseDefBody,
}

/// Analysis case definition: `analysis def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnalysisCaseDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    /// True for `abstract analysis def ...`.
    pub is_abstract: bool,
    pub body: UseCaseDefBody,
}

/// Analysis case usage: `analysis` name (`:` type)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnalysisCaseUsage {
    pub name: String,
    pub type_name: Option<String>,
    /// True for `abstract analysis ...`.
    pub is_abstract: bool,
    pub body: UseCaseDefBody,
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
}

/// Verification case usage: `verification` name (`:` type)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerificationCaseUsage {
    pub name: String,
    pub type_name: Option<String>,
    /// True for `abstract verification ...`.
    pub is_abstract: bool,
    pub body: UseCaseDefBody,
}

/// Use case usage at package level: `use case` name (`:` type)? CaseBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UseCaseUsage {
    pub name: String,
    pub type_name: Option<String>,
    /// True for `abstract use case ...`.
    pub is_abstract: bool,
    pub body: UseCaseDefBody,
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
    pub target: String,
}

/// `then done;` inside a case/use-case body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThenDone {}

/// `include <usecase> ...` inside a case/use-case body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IncludeUseCase {
    pub name: String,
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
    pub name: String,
    /// Raw RHS expression text up to `;` (we don't model the expression grammar here yet).
    pub rhs: String,
}

/// `ref :>> <name> { ... }` redefinition used in SysML v2 release fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RefRedefinition {
    pub name: String,
    /// Raw body text for now (balanced `{ ... }` including nested braces).
    pub body: String,
}

/// `return [attribute] <name> [: <type>] [= <expr>] ;`
/// or `return :>> <name> [= <expr>] ;`
/// — return parameter declaration in analysis/verification case bodies.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaseReturnDecl {
    pub name: String,
    pub name_span: Option<Span>,
    pub type_name: Option<String>,
    /// True for `return :>> name` redefine form.
    pub is_redefine: bool,
}

/// `return ref <name><multiplicity?> { ... }` used in SysML v2 release libraries.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReturnRef {
    pub name: String,
    pub multiplicity: Option<Node<Multiplicity>>,
    /// Raw body text (balanced `{ ... }` including nested braces).
    pub body: String,
    /// Structured `return <expr>;` inside the body when parsed.
    pub return_expression: Option<crate::ast::Node<crate::ast::Expression>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    ReturnRef(Node<ReturnRef>),
    CaseReturnDecl(Node<CaseReturnDecl>),
    Assign(Node<AssignStmt>),
    ForLoop(Node<ForLoop>),
    ThenAction(Node<ThenAction>),
    FlowUsage(Node<crate::ast::behavior::FlowUsage>),
}

/// actor usage `actor pilot : Operator;`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActorUsage {
    pub name: String,
    pub type_name: String,
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
