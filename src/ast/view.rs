use super::behavior::InOutDecl;
use super::common::{ConnectBody, DocComment, Identification, ParseErrorNode};
use super::common::{FilterMember, ImportTarget};
use super::feature_value::FeatureValue;
use super::membership::Membership;
use super::requirement::RequirementDefBody;
use super::structure::MetadataAnnotation;
use crate::ast::core::{
    Expression, Multiplicity, Node, SubsettingRelationship, TypingRelationship,
};
use crate::ast::QualifiedReferenceId;

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
    pub type_name: Option<QualifiedReferenceId>,
    /// Usage-level `:>` subsetting, e.g. `constraint c :> Base;`. Mirrors
    /// `ConnectionUsageMember::subsets`.
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// Usage-level `:>>` redefinition, e.g. `constraint c :>> Base;`. Mirrors
    /// `ConnectionUsageMember::redefines`.
    pub redefines: Option<Node<SubsettingRelationship>>,
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
    InOutDecl(Box<Node<InOutDecl>>),
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
    /// Supertype(s) after `:>`, e.g. `Some(..)` for `calc def X :> Y { }`. Mirrors
    /// `PartDef::specializes`/`ActionDef::specializes`.
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: CalcDefBody,
    pub membership: Membership,
}

/// Calculation usage: `calc` Identification (`:` type)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CalcUsage {
    pub identification: Identification,
    pub type_name: Option<QualifiedReferenceId>,
    /// Redefinition targets for `calc :>> name { … }` and multi-target trailing clauses.
    pub redefines: Option<Vec<QualifiedReferenceId>>,
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
    InOutDecl(Box<Node<InOutDecl>>),
    /// KerML kinded parameter member: `in expr fn[0..*] { ... }`, `in bool test = expr;`,
    /// `in feature clock : Clock[1] default localClock { ... }` (Kernel Function/Semantic
    /// Libraries).
    TypedParameter(Box<Node<crate::ast::TypedParameterMember>>),
    /// KerML feature member (`derived var feature x : T[mult] redefines y;`, `feature all
    /// s: Occurrence subsets a inverse of b { ... }`); see
    /// [`crate::ast::KermlFeatureMember`].
    KermlFeature(Box<Node<crate::ast::KermlFeatureMember>>),
    /// KerML invariant member (`inv name? { expr }`); see
    /// [`crate::ast::KermlInvariantMember`].
    Invariant(Box<Node<crate::ast::KermlInvariantMember>>),
    /// KerML connector member; see [`crate::ast::KermlConnectorMember`].
    Connector(Box<Node<crate::ast::KermlConnectorMember>>),
    /// KerML binding connector member; see [`crate::ast::KermlBindingMember`].
    Binding(Box<Node<crate::ast::KermlBindingMember>>),
    /// KerML succession member; see [`crate::ast::KermlSuccessionMember`].
    Succession(Box<Node<crate::ast::KermlSuccessionMember>>),
    /// KerML end member with an owned cross feature; see [`crate::ast::KermlEndMember`].
    EndMember(Box<Node<crate::ast::KermlEndMember>>),
    /// `import` member inside a type body (`private import SequenceFunctions::*;`, Kernel
    /// Function Library `VectorFunctions.kerml`).
    Import(Box<Node<crate::ast::Import>>),
    /// `comment about a, b ...` annotation member (`Occurrences.kerml`).
    Comment(Node<crate::ast::CommentAnnotation>),
    /// Nested `attribute` usage member (`private attribute position : Natural[1] = ...;`,
    /// Systems Library `Interfaces.sysml`; previously captured opaquely).
    AttributeUsage(Box<Node<crate::ast::AttributeUsage>>),
    /// `assert constraint { ... }` member inside a type body (`ScalarValues.kerml`).
    AssertConstraint(Box<Node<crate::ast::AssertConstraintMember>>),
    /// Nested KerML classifier declaration inside a type body (`struct StructuredSurface
    /// specializes StructuredSpaceObject, Surface { ... }` inside a struct body,
    /// Kernel Semantic Library `Objects.kerml`).
    KermlClassifier(Box<Node<crate::ast::KermlClassifierDecl>>),
    /// Keyword-less feature binding, named (`private instantNum: Natural[1] = ...;`) or the
    /// anonymous leading-redefinition form (`:>> dimension = size(components);`,
    /// `VectorValues.kerml`).
    DefaultReferenceUsage(Box<Node<crate::ast::DefaultReferenceUsage>>),
    ReturnDecl(Box<Node<ReturnDecl>>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    Expression(Node<Expression>), // formula
    /// Nested `calc` usage inside a calc body (validation `10b` rollups).
    CalcUsage(Box<Node<CalcUsage>>),
    /// Nested `calc def` inside a calc body (Domain Libraries `SampledFunctions.sysml`'s
    /// private `Linear` rollup helper).
    CalcDef(Box<Node<CalcDef>>),
    /// Directed `in part …` parameter (validation `10b`).
    PartUsage(Box<Node<crate::ast::PartUsage>>),
}

/// Return declaration: `return` (`:>>`)? name? (`:`|`:>`) type (`=` expr)? `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReturnDecl {
    /// Kind keyword between `return` and the declaration (`return attribute verdict :
    /// VerdictKind = ...;`, `return feature timeSignal : TimeSignal[1] = ...`, Kernel Semantic
    /// Library `Observation.kerml`/`Triggers.kerml`).
    pub kind_keyword: Option<ReturnKindKeyword>,
    /// Empty for anonymous `return : Type [= expr];` (validation `10c`, `10d`).
    pub name: String,
    /// `None` for the untyped named forms `return result [1..1];` / `return sampling = ...;`
    /// (Domain Libraries `SampledFunctions.sysml`).
    pub type_name: Option<QualifiedReferenceId>,
    /// True for `return :>> name : Type = …` (validation `10b`).
    pub is_redefine: bool,
    /// True when the type is introduced with `:>` rather than `:` (validation `10b` rollups).
    pub is_subsetting: bool,
    /// Multiplicity clause after the type, e.g. `return : Real[1] = x;` (Kernel Function
    /// Library). Previously unparseable.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `ordered` keyword from `MultiplicityPart` (`return : Anything[0..*] ordered nonunique;`,
    /// Kernel Function Library `BaseFunctions.kerml`).
    pub ordered: bool,
    /// `nonunique` keyword from `MultiplicityPart`. See `ordered`.
    pub nonunique: bool,
    /// Trailing redefinition targets; repeated `redefines` clauses merge their targets
    /// (`return resultValues : Anything [*] nonunique redefines result redefines values;`,
    /// Kernel Semantic Library `FeatureReferencingPerformances.kerml`).
    pub redefines: Option<Node<crate::ast::SubsettingRelationship>>,
    /// Value clause: `= expr` or `default (=|:=)? expr` (`return : Real default
    /// NumericalFunctions::sum0(collection, 0.0);`, Kernel Function Library
    /// `RealFunctions.kerml`).
    pub value: Option<Node<crate::ast::FeatureValue>>,
    /// Result body: `;` (`CalcDefBody::Semicolon`) or `{ ... }` following the calc-body member
    /// grammar (`return positionVector : Position3dVector[1] { attribute :>> mRef = ...; }`,
    /// Domain Libraries `SpatialItems.sysml`).
    pub body: CalcDefBody,
}

// ---------------------------------------------------------------------------
// Views and Viewpoints (SysML v2 Clause 8.2.2.26)
// ---------------------------------------------------------------------------

/// The kind keyword after `return` on a [`ReturnDecl`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReturnKindKeyword {
    /// `attribute`.
    Attribute,
    /// `feature`.
    Feature,
}

impl ReturnKindKeyword {
    /// The authored keyword spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Attribute => "attribute",
            Self::Feature => "feature",
        }
    }
}

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
    pub type_name: Option<QualifiedReferenceId>,
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
    ViewUsage(Box<Node<ViewUsage>>),
    /// Nested `rendering` usage member, e.g. the anonymous `rendering :>> subrenderings[0..*] =
    /// columnView.viewRendering;` inside `asElementTable` (Systems Library `Views.sysml`).
    Rendering(Box<Node<RenderingUsage>>),
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
    pub type_name: Option<QualifiedReferenceId>,
    /// Subsets target, e.g. `baseView` in `view v :> baseView { ... }`.
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// Redefines target, e.g. `columnView` in `view :>> columnView[1] { ... }`. `None` for the
    /// ordinary named form.
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// Multiplicity, e.g. `[1]` in `view :>> columnView[1] { ... }` or `[0..*]` in `view
    /// columnView[0..*] ordered { ... }` (Systems Library `Views.sysml`). Previously captured
    /// only by the anonymous redefinition form and discarded on the named path.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `ordered` keyword from `MultiplicityPart` (`view columnView[0..*] ordered { ... }`).
    /// Previously skipped and discarded.
    pub ordered: bool,
    /// `nonunique` keyword from `MultiplicityPart`. See `ordered`.
    pub nonunique: bool,
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
    pub target: ImportTarget,
    pub body: ConnectBody,
}

/// Satisfy in view body: `satisfy` QualifiedName RelationshipBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SatisfyViewMember {
    pub viewpoint_ref: QualifiedReferenceId,
    pub body: ConnectBody,
}

/// Viewpoint usage: `viewpoint` ConstraintUsageDeclaration RequirementBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ViewpointUsage {
    pub name: String,
    pub type_name: Option<QualifiedReferenceId>,
    /// `:>` subsets clause (spec42 gap 25), mirroring [`ViewUsage::subsets`]. Previously parsed
    /// by the shared usage header and discarded.
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// `:>>` redefines clause, mirroring [`ViewUsage::redefines`].
    pub redefines: Option<Node<SubsettingRelationship>>,
    pub body: RequirementDefBody,
    pub membership: Membership,
}

/// Rendering usage: `rendering` Usage.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderingUsage {
    /// Leading `abstract` keyword (BNF `RefPrefix`). Previously parsed and discarded.
    pub is_abstract: bool,
    /// Declared name. Empty for the anonymous redefinition form (`rendering :>>
    /// subrenderings[0..*] = columnView.viewRendering;`, Systems Library `Views.sysml`).
    pub name: String,
    pub type_name: Option<QualifiedReferenceId>,
    /// Multiplicity clause (BNF `MultiplicityPart`), e.g. `asTreeDiagram :
    /// GraphicalRendering[1]` (Systems Library `Views.sysml`). Previously parsed and discarded
    /// inside the shared usage header.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `ordered` keyword from `MultiplicityPart` (`abstract rendering renderings :
    /// Rendering[0..*] nonunique :> parts`, Systems Library `Views.sysml`). Previously skipped
    /// inside the shared usage header.
    pub ordered: bool,
    /// `nonunique` keyword from `MultiplicityPart`. See `ordered`.
    pub nonunique: bool,
    /// `:>` subsets clause, e.g. `: GraphicalRendering[1] :> renderings`. Previously parsed and
    /// discarded.
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// `:>>` redefinition clause, e.g. the anonymous `rendering :>> subrenderings[0..*] = ...`
    /// (Systems Library `Views.sysml`).
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// Optional value clause (BNF `ValuePart`), e.g. `= columnView.viewRendering`.
    pub value: Option<Node<FeatureValue>>,
    pub body: RenderingUsageBody,
    pub membership: Membership,
}
