use super::behavior::InOutDecl;
use super::body::Body;
use super::common::{AnnotatingMember, Identification, ParseErrorNode};
use super::common::{FilterMember, ImportTarget};
use super::feature_value::FeatureValue;
use super::membership::Membership;
use super::requirement::RequirementDefBody;
use super::structure::{DefinitionPrefix, MetadataKeywordUsage};
use crate::ast::core::{
    Expression, Multiplicity, Node, SubsettingRelationship, TypingRelationship,
};
use crate::ast::QualifiedReferenceId;

/// Constraint definition: `constraint def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConstraintDef {
    /// `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation ?= 'variation'`
    /// (SysML BNF 219; Pilot `SysML.xtext` 490) -- one slot, two alternatives, carrying the
    /// authored keyword's exact span. `ConstraintDefinition` (SysML BNF 1378) reaches it through
    /// `OccurrenceDefinitionPrefix` (SysML BNF 541).
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
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
    /// The complete `OccurrenceUsagePrefix` this usage was written with
    /// (`ConstraintUsage = OccurrenceUsagePrefix 'constraint' ConstraintUsageDeclaration
    /// CalculationBody`, SysML BNF 1382).
    ///
    /// The same shared component `OccurrenceUsage`, `ItemUsage`, `SatisfyRequirementUsage` and
    /// `PartUsage` carry. Nothing was deleted to make room, because nothing was here: the parser
    /// consumed a leading `abstract` and discarded it, and every other slot -- `ref` above all --
    /// was not recognized, so `ref constraint self : ConstraintCheck :>> BooleanEvaluation::self;`
    /// (Systems Library `Constraints.sysml:20`) was shredded into a bare `'ref';` expression plus
    /// a separate constraint usage, with no diagnostic.
    pub prefix: crate::ast::OccurrenceUsagePrefix,
    pub name: String,
    pub short_name: Option<String>,
    pub type_name: Option<QualifiedReferenceId>,
    pub multiplicity: Option<Node<Multiplicity>>,
    /// Usage-level `:>` subsetting, e.g. `constraint c :> Base;`. Mirrors
    /// `ConnectionUsageMember::subsets`.
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// Usage-level `:>>` redefinition, e.g. `constraint c :>> Base;`. Mirrors
    /// `ConnectionUsageMember::redefines`.
    pub redefines: Option<Node<SubsettingRelationship>>,
    pub body: ConstraintDefBody,
    pub membership: Membership,
}

pub type ConstraintDefBody = Body<ConstraintDefBodyElement>;

// `AnnotatingMember` is the whole `AnnotatingElement` production, making it inherently larger
// than sibling variants like `Error`; boxing just this one variant in just this one enum would be
// inconsistent with the ~27 other body-element enums sharing the same `Annotating(AnnotatingMember)`
// shape crate-wide, so the size difference is accepted here rather than partially addressed --
// exactly as `RequirementDefBodyElement` records for `AttributeUsage`. The lint only became
// audible once `Constraint` was boxed, which made this enum smaller, not larger.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConstraintDefBodyElement {
    Error(Node<ParseErrorNode>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    /// `#Tag` metadata reference: `PrefixMetadataMember` prefixing the next member, or the
    /// `ExtendedUsage` member spelling `#Tag;` / `#Tag { ... }`. This scope reaches both --
    /// every member may carry a prefix, and `ExtendedUsage` is a `NonOccurrenceUsageElement` --
    /// but modelled neither, so a `#` member was reported unsupported here.
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    InOutDecl(Box<Node<InOutDecl>>),
    Expression(Node<Expression>), // e.g. totalThrust >= totalWeight * margin
    /// A `constraint` member nested inside a `constraint def { ... }` body (e.g. the Systems
    /// Library's `RequirementConstraintCheck::assumptions`/`::constraints`, redefined/subset
    /// from another constraint). Not boxed: `ConstraintUsage` is a small struct and the
    /// recursion into its own `body: ConstraintDefBody` already passes through a `Vec`.
    /// Boxed because the shared `OccurrenceUsagePrefix` makes `ConstraintUsage` much the largest
    /// member of this scope; the indirection keeps the enum the size of its other variants.
    Constraint(Box<Node<ConstraintUsage>>),
    /// Keyword-less `:>> name = …` binding inside `require name { … }` (validation `10c`).
    AttributeUsage(Box<Node<crate::ast::AttributeUsage>>),
    /// Keyword-less feature declaration (`mass : Real;`): a constraint definition body is a
    /// `DefinitionBody`, so it owns usages as well as the constraint expression.
    FeatureDecl(Box<Node<crate::ast::DefaultReferenceUsage>>),
    /// `require` / `assume` member, e.g. `require viewpointSatisfactions { ... }` inside a
    /// `satisfy requirement ... by that { ... }` body (Systems Library `Views.sysml:43`). The
    /// brace-bodied form previously fell through to the terminal expression arm, which consumed
    /// the name and then could not account for the `{`.
    RequireConstraint(Box<Node<crate::ast::RequireConstraint>>),
    /// `ConstraintDefinition`/`ConstraintUsage` own a `CalculationBody`, which reaches
    /// `ActionBodyItem -> NonBehaviorBodyItem -> StructureUsageMember -> PartUsage` (SysML BNF
    /// 1366, 901, 910, 623). The scope had no variant for one, so `part p : T;` fell through to
    /// the terminal expression arm and was shredded into `'part';` and `p : T;` with no
    /// diagnostic -- input a round trip then wrote back out as two members.
    PartUsage(Box<Node<crate::ast::PartUsage>>),
    /// `ReturnParameterMember`, the second alternative of `CalculationBodyItem = ActionBodyItem
    /// | ReturnParameterMember` (SysML BNF 1366, 1370). `ConstraintDefinition` and
    /// `ConstraintUsage` both end in `CalculationBody` (SysML BNF 1378, 1382, 1359), so a
    /// constraint body owns a `return` member exactly as a calculation body does; the scope
    /// modelled none, so `return result = allTrue(assumptions()) implies
    /// allTrue(constraints()) { doc /* … */ }` (Systems Library `Requirements.sysml:41`) reached
    /// the terminal expression arm, which read the keyword as a name, emitted `'return';` and
    /// `result;` as two invented members, and then reported the remainder of the real member as
    /// `recovered_constraint_body_element`.
    ///
    /// Deliberate duplication, recorded rather than hidden: this enum and
    /// [`CalcDefBodyElement`] model the one `CalculationBody` production, and this variant is
    /// the same node [`CalcDefBodyElement::ReturnDecl`] carries. They are not yet one enum
    /// because this scope also carries `Constraint`, `RequireConstraint` and `FeatureDecl`,
    /// whose membership in a calculation body is its own question. Unifying them is follow-up
    /// work; this variant closes the member gap without deciding it.
    ReturnDecl(Box<Node<ReturnDecl>>),
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
    /// `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation ?= 'variation'`
    /// (SysML BNF 219; Pilot `SysML.xtext` 490) -- one slot, two alternatives, carrying the
    /// authored keyword's exact span. `CalculationDefinition` (SysML BNF 1351) reaches it through
    /// `OccurrenceDefinitionPrefix` (SysML BNF 541).
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
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
    /// `abstract` keyword, e.g. `abstract calc subcalculations : Calculation :> calculations,
    /// subactions { ... }` (Systems Library `Calculations.sysml`). Parsed and discarded until
    /// this field existed, so emission dropped it.
    pub is_abstract: bool,
    pub type_name: Option<QualifiedReferenceId>,
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `:>` subsets clause, which may name several comma-separated targets. Also previously
    /// parsed and discarded, with a comment claiming `CalcUsage` "doesn't model subsetting
    /// separately from redefines" -- it does now, because they are different relationships.
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// Redefinition targets for `calc :>> name { … }` and multi-target trailing clauses.
    pub redefines: Option<Vec<QualifiedReferenceId>>,
    /// `= expr` / `:= expr` binding (`in calc scenario = cityScenario;`, validation `10c`).
    pub value: Option<Node<crate::ast::FeatureValue>>,
    /// Set when parsed as `in`/`out`/`inout calc` (validation `10c`).
    pub direction: Option<crate::ast::InOut>,
    /// The `ref` of `BasicUsagePrefix = RefPrefix ( isReference ?= 'ref' )?`, e.g. `ref calc
    /// self : Calculation :>> Action::self;` (Systems Library `Calculations.sysml`). The keyword
    /// was not accepted at all, so a calculation body fell through to its expression parser and
    /// kept the bare word `ref` as a standalone expression member -- emission then wrote
    /// `'ref';` on its own line and the declaration lost its prefix. Parallel to
    /// `ActionUsage::is_reference` and `PartUsage::is_reference`.
    pub is_reference: bool,
    pub body: CalcDefBody,
    pub membership: Membership,
}

pub type CalcDefBody = Body<CalcDefBodyElement>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::large_enum_variant)]
pub enum CalcDefBodyElement {
    Error(Node<ParseErrorNode>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    /// `#Tag` metadata reference: `PrefixMetadataMember` prefixing the next member, or the
    /// `ExtendedUsage` member spelling `#Tag;` / `#Tag { ... }`. This scope reaches both --
    /// every member may carry a prefix, and `ExtendedUsage` is a `NonOccurrenceUsageElement` --
    /// but modelled neither, so a `#` member was reported unsupported here.
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    /// `CalculationBodyItem = ActionBodyItem | ReturnParameterMember` (SysML 8.2.2.19), so a
    /// calculation body owns every action-body member as well as its own `return`. They arrive
    /// through the action dispatcher rather than as fifteen restated variants, the way
    /// [`crate::ast::DefinitionBodyElement::OccurrenceMember`] delegates its member set.
    ///
    /// Only a SysML calculation body produces this. `calc_def_body` also serves KerML type
    /// bodies, whose `TypeBodyElement` has no action-node alternative, so the calculation
    /// entry point is the only one that dispatches it. Splitting the two scopes into their own
    /// element enums is Phase-4 work; this variant does not make that debt worse.
    ActionMember(Box<Node<crate::ast::ActionDefBodyElement>>),
    InOutDecl(Box<Node<InOutDecl>>),
    /// KerML feature member (`derived var feature x : T[mult] redefines y;`, `feature all
    /// s: Occurrence subsets a inverse of b { ... }`); see
    /// [`crate::ast::KermlFeature`].
    KermlFeature(Box<Node<crate::ast::KermlFeature>>),
    /// KerML invariant member (`inv name? { expr }`); see
    /// [`crate::ast::KermlInvariantMember`].
    Invariant(Box<Node<crate::ast::KermlInvariantMember>>),
    /// KerML connector member; see [`crate::ast::KermlConnectorMember`].
    Connector(Box<Node<crate::ast::KermlConnectorMember>>),
    /// KerML binding connector member; see [`crate::ast::KermlBindingMember`].
    Binding(Box<Node<crate::ast::KermlBindingMember>>),
    /// KerML succession member; see [`crate::ast::KermlSuccessionMember`].
    Succession(Box<Node<crate::ast::KermlSuccessionMember>>),
    /// `import` member inside a type body (`private import SequenceFunctions::*;`, Kernel
    /// Function Library `VectorFunctions.kerml`).
    Import(Box<Node<crate::ast::Import>>),
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
    pub short_name: Option<String>,
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
    /// `MultiplicityPart`'s `isOrdered`/`isUnique` keyword slots, each carrying the authored
    /// spelling and its exact span. See [`MultiplicityModifiers`](crate::ast::MultiplicityModifiers).
    pub multiplicity_modifiers: crate::ast::MultiplicityModifiers,
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
    /// `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation ?= 'variation'`
    /// (SysML BNF 219; Pilot `SysML.xtext` 490) -- one slot, two alternatives, carrying the
    /// authored keyword's exact span. `ViewDefinition` (SysML BNF 1580) reaches it through
    /// `OccurrenceDefinitionPrefix` (SysML BNF 541).
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: ViewDefBody,
    pub membership: Membership,
}

pub type ViewDefBody = Body<ViewDefBodyElement>;

// A `RefDecl` carries a typing, several relationship nodes and a body, making it inherently
// larger than the `Doc`/`Error` variants beside it. Boxing just this one variant in just this one
// enum would be inconsistent with the other body-element enums sharing the same
// `RefDecl(Node<RefDecl>)` shape crate-wide -- see `RequirementDefBodyElement`.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ViewDefBodyElement {
    /// A spec-valid member of this body that the parser does not model yet, retained with
    /// its authored span and a diagnostic.
    Unsupported(Node<crate::ast::UnsupportedGrammarNode>),
    Error(Node<ParseErrorNode>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    /// `#Tag` metadata reference: `PrefixMetadataMember` prefixing the next member, or the
    /// `ExtendedUsage` member spelling `#Tag;` / `#Tag { ... }`. This scope reaches both --
    /// every member may carry a prefix, and `ExtendedUsage` is a `NonOccurrenceUsageElement` --
    /// but modelled neither, so a `#` member was reported unsupported here.
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    Filter(Node<FilterMember>),
    ViewRendering(Node<ViewRenderingUsage>),
    /// `ref`-prefixed feature declaration, e.g. `ref viewpoint :>> self : ViewpointCheck;` and
    /// `abstract ref rendering subrenderings : Rendering[0..*] :> renderings;` (Systems Library
    /// `Views.sysml`). This scope accepted no `ref` member at all.
    RefDecl(Node<crate::ast::RefDecl>),
    /// Nested viewpoint usage, e.g. `viewpoint viewpointSatisfactions : ViewpointCheck[0..*] :>
    /// ...;` (Systems Library `Views.sysml`). Parsed by the same `viewpoint_usage` that package
    /// and part bodies already dispatch.
    ViewpointUsage(Node<ViewpointUsage>),
    /// `satisfy requirement X by Y;`, e.g. `satisfy requirement viewpointConformance by
    /// this;` (`Views.sysml`). The same `SatisfyRequirementUsage` node part bodies already accept.
    Satisfy(Box<Node<crate::ast::SatisfyRequirementUsage>>),
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
pub type RenderingUsageBody = Body<RenderingUsageBodyElement>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::large_enum_variant)]
pub enum RenderingUsageBodyElement {
    Error(Node<ParseErrorNode>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
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
    /// `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation ?= 'variation'`
    /// (SysML BNF 219; Pilot `SysML.xtext` 490) -- one slot, two alternatives, carrying the
    /// authored keyword's exact span. `ViewpointDefinition` (SysML BNF 1632) reaches it through
    /// `OccurrenceDefinitionPrefix` (SysML BNF 541).
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: RequirementDefBody,
    pub membership: Membership,
}

/// Rendering definition: `rendering def` Definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderingDef {
    pub identification: Identification,
    /// `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation ?= 'variation'`
    /// (SysML BNF 219; Pilot `SysML.xtext` 490) -- one slot, two alternatives, carrying the
    /// authored keyword's exact span. `RenderingDefinition` (SysML BNF 1642) reaches it through
    /// `OccurrenceDefinitionPrefix` (SysML BNF 541).
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: RenderingDefBody,
    pub membership: Membership,
}

pub type RenderingDefBody = Body<RenderingDefBodyElement>;

// A `RefDecl` carries a typing, several relationship nodes and a body, making it inherently
// larger than the `Doc`/`Error` variants beside it. Boxing just this one variant in just this one
// enum would be inconsistent with the other body-element enums sharing the same
// `RefDecl(Node<RefDecl>)` shape crate-wide -- see `RequirementDefBodyElement`.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenderingDefBodyElement {
    /// A spec-valid member of this body that the parser does not model yet, retained with
    /// its authored span and a diagnostic.
    Unsupported(Node<crate::ast::UnsupportedGrammarNode>),
    Error(Node<ParseErrorNode>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    Filter(Node<FilterMember>),
    ViewRendering(Node<ViewRenderingUsage>),
    /// `ref`-prefixed feature declaration, e.g. `ref rendering :>> self : Rendering;` and
    /// `abstract ref rendering subrenderings : Rendering[0..*] :> renderings { ... }` (Systems
    /// Library `Views.sysml`).
    RefDecl(Node<crate::ast::RefDecl>),
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
    pub short_name: Option<String>,
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
    /// `MultiplicityPart`'s `isOrdered`/`isUnique` keyword slots, each carrying the authored
    /// spelling and its exact span. See [`MultiplicityModifiers`](crate::ast::MultiplicityModifiers).
    pub multiplicity_modifiers: crate::ast::MultiplicityModifiers,
    pub body: ViewBody,
    pub membership: Membership,
}

pub type ViewBody = Body<ViewBodyElement>;

// A `RefDecl` carries a typing, several relationship nodes and a body, making it inherently
// larger than the `Doc`/`Error` variants beside it. Boxing just this one variant in just this one
// enum would be inconsistent with the other body-element enums sharing the same
// `RefDecl(Node<RefDecl>)` shape crate-wide -- see `RequirementDefBodyElement`.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ViewBodyElement {
    Error(Node<ParseErrorNode>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    Filter(Node<FilterMember>),
    ViewRendering(Node<ViewRenderingUsage>),
    Expose(Node<ExposeMember>),
    /// `SatisfyRequirementUsage` in a view usage body.
    ///
    /// `ViewBodyItem → DefinitionBodyItem → … → BehaviorUsageElement →
    /// SatisfyRequirementUsage`: there is one satisfy production, and a view body reaches it the
    /// same way every other definition or usage body does. This scope used to own a separate
    /// `SatisfyViewMember` node carrying a bare viewpoint reference and a `RelationshipBody`,
    /// which could represent neither the `assert`/`not` prefixes, the `by` clause, the inline
    /// `requirement` declaration, nor a `RequirementBody` member.
    Satisfy(Box<Node<crate::ast::SatisfyRequirementUsage>>),
    /// `ref`-prefixed feature declaration, e.g. `abstract ref rendering :>> viewRendering[0..1];`
    /// inside `view columnView[0..*] ordered { ... }` (Systems Library `Views.sysml`).
    RefDecl(Node<crate::ast::RefDecl>),
}

/// Expose in view body: `expose` (MembershipImport | NamespaceImport) RelationshipBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExposeMember {
    pub target: ImportTarget,
    /// `Expose = 'expose' ( MembershipExpose | NamespaceExpose ) RelationshipBody`. The brace
    /// form used to be skipped wholesale, so its members and both delimiters were discarded.
    pub body: crate::ast::Body<crate::ast::RelationshipBodyElement>,
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
    /// `MultiplicityPart`'s `isOrdered`/`isUnique` keyword slots, each carrying the authored
    /// spelling and its exact span. See [`MultiplicityModifiers`](crate::ast::MultiplicityModifiers).
    pub multiplicity_modifiers: crate::ast::MultiplicityModifiers,
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
