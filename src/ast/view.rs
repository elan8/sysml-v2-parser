use super::behavior::InOutDecl;
use super::common::FilterMember;
use super::common::{ConnectBody, DocComment, Identification, ParseErrorNode};
use super::membership::Membership;
use super::requirement::RequirementDefBody;
use super::structure::MetadataAnnotation;
use crate::ast::core::{
    Expression, Multiplicity, Node, SubsettingRelationship, TypingRelationship,
};

/// Constraint definition: `constraint def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConstraintDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: ConstraintDefBody,
    pub membership: Membership,
}

/// Constraint usage: `constraint` name (feature usage header: typing/subsetting/multiplicity/
/// `ordered`/`nonunique`)? body. Package-level only -- mirrors [`ConstraintDef`]'s existing
/// `abstract`/`def`-less real-library forms (`Systems Library/Constraints.sysml`'s
/// `constraintChecks`/`assertedConstraintChecks`/`negatedConstraintChecks`), which previously had
/// no distinct usage-side AST node and folded into `ConstraintDef` (see
/// `parser::constraint::constraint_def`'s doc comment history).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConstraintUsage {
    pub name: String,
    pub type_name: Option<String>,
    pub body: ConstraintDefBody,
    pub membership: Membership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConstraintDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<ConstraintDefBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConstraintDefBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    InOutDecl(Node<InOutDecl>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    Expression(Node<Expression>), // e.g. totalThrust >= totalWeight * margin
    /// A `constraint` member nested inside a `constraint def { ... }` body (e.g. the Systems
    /// Library's `RequirementConstraintCheck::assumptions`/`::constraints`, redefined/subset
    /// from another constraint). Not boxed: `ConstraintUsage` is a small struct and the
    /// recursion into its own `body: ConstraintDefBody` already passes through a `Vec`.
    Constraint(Node<ConstraintUsage>),
    /// Keyword-less `:>> name = …` binding inside `require name { … }` (validation `10c`).
    AttributeUsage(Box<Node<crate::ast::AttributeUsage>>),
    /// Unmodeled constraint-body element captured as raw text (used for library parsing).
    Other(String),
}

/// constraint body {}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConstraintBody {
    Semicolon,
    Brace, // Often contains docs or block of expressions
}

/// Calc definition: `calc def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CalcDef {
    pub identification: Identification,
    pub body: CalcDefBody,
    pub membership: Membership,
}

/// Calculation usage: `calc` Identification (`:` type)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CalcUsage {
    pub identification: Identification,
    pub type_name: Option<String>,
    /// Redefinition target for `calc :>> name { … }` (validation `10b`).
    pub redefines: Option<String>,
    /// `= expr` / `:= expr` binding (`in calc scenario = cityScenario;`, validation `10c`).
    pub value: Option<Node<crate::ast::FeatureValue>>,
    /// Set when parsed as `in`/`out`/`inout calc` (validation `10c`).
    pub direction: Option<crate::ast::InOut>,
    pub body: CalcDefBody,
    pub membership: Membership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CalcDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<CalcDefBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CalcDefBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    InOutDecl(Node<InOutDecl>),
    ReturnDecl(Node<ReturnDecl>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    Expression(Node<Expression>), // formula
    /// Nested `calc` usage inside a calc body (validation `10b` rollups).
    CalcUsage(Box<Node<CalcUsage>>),
    /// Directed `in part …` parameter (validation `10b`).
    PartUsage(Box<Node<crate::ast::PartUsage>>),
    /// Unmodeled calc-body element captured as raw text (used for library parsing).
    Other(String),
}

/// Return declaration: `return` (`:>>`)? name? (`:`|`:>`) type (`=` expr)? `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReturnDecl {
    /// Empty for anonymous `return : Type [= expr];` (validation `10c`, `10d`).
    pub name: String,
    pub type_name: String,
    /// True for `return :>> name : Type = …` (validation `10b`).
    pub is_redefine: bool,
    /// True when the type is introduced with `:>` rather than `:` (validation `10b` rollups).
    pub is_subsetting: bool,
    pub value: Option<Node<crate::ast::Expression>>,
}

// ---------------------------------------------------------------------------
// Views and Viewpoints (SysML v2 Clause 8.2.2.26)
// ---------------------------------------------------------------------------

/// View definition: `view def` Identification ViewDefinitionBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: ViewDefBody,
    pub membership: Membership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ViewDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<ViewDefBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ViewDefBodyElement {
    Error(Node<ParseErrorNode>),
    /// Unmodeled view-definition body element captured as raw text (used for library parsing).
    Other(String),
    Doc(Node<DocComment>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    Filter(Node<FilterMember>),
    ViewRendering(Node<ViewRenderingUsage>),
}

/// View rendering usage: `render` name `:` type (`;` or body).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewRenderingUsage {
    pub name: String,
    pub type_name: Option<String>,
    pub body: RenderingUsageBody,
    pub membership: Membership,
}

/// Body of a `render`/`rendering` usage: `;` or `{` RenderingUsageBodyElement* `}`. Per BNF
/// Clause 8.2.2.26.1 (`ViewRenderingUsage : RenderingUsage = ownedRelationship +=
/// OwnedReferenceSubsetting FeatureSpecializationPart? UsageBody | ...`), the body is a generic
/// `UsageBody` -- most notably it can own a redefined `columnView` feature of `asElementTable`
/// (`view :>> columnView[N] { render ...; }`, confirmed against real usage in
/// `sysml-v2-release/sysml/src/training/42. Views/Views Example.sysml` and
/// `.../validation/11-View and Viewpoint/11a-View-Viewpoint.sysml`) -- not just a `;`/opaque
/// `{...}` the way the previous `ConnectBody` field type treated it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenderingUsageBody {
    Semicolon,
    Brace {
        elements: Vec<Node<RenderingUsageBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenderingUsageBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    /// Nested `view` usage member, e.g. a `columnView` redefinition (`view :>> columnView[1] {
    /// render asTextualNotation; }`).
    ViewUsage(Node<ViewUsage>),
}

/// Viewpoint definition: `viewpoint def` Identification RequirementBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewpointDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: RequirementDefBody,
    pub membership: Membership,
}

/// Rendering definition: `rendering def` Definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderingDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: RenderingDefBody,
    pub membership: Membership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenderingDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<RenderingDefBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenderingDefBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    Filter(Node<FilterMember>),
    ViewRendering(Node<ViewRenderingUsage>),
    Other(String),
}

/// View usage: `view` name `:` type? ViewBody, or the anonymous redefinition form `view :>>
/// name[multiplicity]? ViewBody` (BNF `ViewUsage = OccurrenceUsagePrefix 'view' UsageDeclaration?
/// ValuePart? ViewBody`, where `UsageDeclaration` legally omits the name in favor of a leading
/// `:>>` redefinition target -- the same shape `PartUsage`'s `redefines`/`multiplicity` fields
/// already cover). `name` is empty for the anonymous-redefinition form, matching
/// `PartUsage::name`'s existing convention.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewUsage {
    pub name: String,
    pub type_name: Option<String>,
    /// Redefines target, e.g. `columnView` in `view :>> columnView[1] { ... }`. `None` for the
    /// ordinary named form.
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// Multiplicity, e.g. `[1]` in `view :>> columnView[1] { ... }`.
    pub multiplicity: Option<Node<Multiplicity>>,
    pub body: ViewBody,
    pub membership: Membership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ViewBody {
    Semicolon,
    Brace {
        elements: Vec<Node<ViewBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ViewBodyElement {
    Error(Node<ParseErrorNode>),
    /// Unmodeled view body element captured as raw text (used for library parsing).
    Other(String),
    Doc(Node<DocComment>),
    Filter(Node<FilterMember>),
    ViewRendering(Node<ViewRenderingUsage>),
    Expose(Node<ExposeMember>),
    Satisfy(Node<SatisfyViewMember>),
}

/// Expose in view body: `expose` (MembershipImport | NamespaceImport) RelationshipBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExposeMember {
    /// Full target path (e.g. vehicle, vehicle::*, vehicle::*::**, SystemModel::vehicle::**).
    pub target: String,
    /// Whether this is a namespace import (`X::*`) rather than a membership import (`X`) --
    /// mirrors [`crate::ast::Import::is_import_all`]. `expose` is normatively an Import per this
    /// struct's own BNF doc comment; the parser already distinguished the four suffix forms
    /// (`::*::**` / `::**` / `::*` / plain) to build `target`, but previously discarded which one
    /// matched into the concatenated string instead of retaining it structurally.
    pub is_import_all: bool,
    /// Whether the import is recursive (`::**` suffix) -- mirrors
    /// [`crate::ast::Import::is_recursive`].
    pub is_recursive: bool,
    pub body: ConnectBody,
}

/// Satisfy in view body: `satisfy` QualifiedName RelationshipBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SatisfyViewMember {
    pub viewpoint_ref: String,
    pub body: ConnectBody,
}

/// Viewpoint usage: `viewpoint` ConstraintUsageDeclaration RequirementBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewpointUsage {
    pub name: String,
    pub type_name: String,
    pub body: RequirementDefBody,
    pub membership: Membership,
}

/// Rendering usage: `rendering` Usage.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderingUsage {
    pub name: String,
    pub type_name: Option<String>,
    pub body: RenderingUsageBody,
    pub membership: Membership,
}
