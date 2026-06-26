use super::behavior::{Allocate, InOut, InOutDecl, StateDefBody, StateUsage};
use super::common::{CommentAnnotation, ConnectBody, DocComment, Identification, ParseErrorNode};
use super::requirement::{EnumerationUsage, ItemUsage, RequirementUsage, Satisfy};
use super::view::{CalcUsage, ConstraintDefBody};
use crate::ast::core::{Expression, Node, Span};

/// Part definition: `part def` Identification (`:>` specializes)? Body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PartDef {
    /// Optional `abstract` or `variation` prefix (BNF BasicDefinitionPrefix).
    pub definition_prefix: Option<DefinitionPrefix>,
    /// Whether this is an `individual part def`.
    pub is_individual: bool,
    pub identification: Identification,
    /// Supertype after `:>`, e.g. Some("Axle") for `part def FrontAxle :> Axle`.
    pub specializes: Option<String>,
    /// Span of the `:> <type>` fragment (for semantic tokens), when present.
    pub specializes_span: Option<Span>,
    pub body: PartDefBody,
}

/// BNF BasicDefinitionPrefix: `abstract` | `variation`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DefinitionPrefix {
    Abstract,
    Variation,
}

/// Body of a part definition: `;` or `{` PartDefBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PartDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<PartDefBodyElement>>,
    },
}

/// Element inside a part definition body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PartDefBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    Comment(Node<CommentAnnotation>),
    Annotation(Node<Annotation>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    Other(String),
    AttributeDef(Node<AttributeDef>),
    AttributeUsage(Node<AttributeUsage>),
    RequirementUsage(Node<RequirementUsage>),
    ItemDef(Node<ItemDef>),
    ItemUsage(Node<ItemUsage>),
    Ref(Node<RefDecl>),
    PortUsage(Node<PortUsage>),
    PartUsage(Box<Node<PartUsage>>),
    PartDef(Node<PartDef>),
    OccurrenceUsage(Box<Node<OccurrenceUsage>>),
    InterfaceDef(Node<InterfaceDef>),
    InterfaceUsage(Node<InterfaceUsage>),
    Connect(Node<Connect>),
    FlowUsage(Node<crate::ast::behavior::FlowUsage>),
    /// `connection` usage member inside a part definition body.
    Connection(Node<ConnectionUsageMember>),
    Perform(Node<Perform>),
    Allocate(Node<Allocate>),
    OpaqueMember(Node<OpaqueMemberDecl>),
    /// `exhibit state` name `:` type (`;` or body).
    ExhibitState(Node<ExhibitState>),
    /// Calculation usage (`calc` keyword) inside a part definition body.
    CalcUsage(Node<CalcUsage>),
    /// Enumeration usage (`enum` keyword) inside a part definition body.
    EnumerationUsage(Node<EnumerationUsage>),
}

/// Library-tolerant part member preserved without forcing it into an unrelated node shape.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpaqueMemberDecl {
    pub keyword: String,
    pub name: String,
    pub text: String,
    pub body: AttributeBody,
}

/// Connection usage member inside part definitions.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConnectionUsageMember {
    pub name: Option<String>,
    pub type_name: Option<String>,
    pub body: ConnectionDefBody,
    pub subsets: Option<String>,
    pub redefines: Option<String>,
}

/// Exhibit state usage: `exhibit state` name `:` type (`;` or body).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExhibitState {
    pub name: String,
    pub type_name: Option<String>,
    pub redefines: Option<String>,
    pub body: StateDefBody,
}

/// Attribute definition: `attribute` [`def`] name (`:>` | `:` type)? (`=` value)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeDef {
    pub name: String,
    /// Short name from `< ... >` when present (e.g. unit symbol `m`, `EUR`).
    pub short_name: Option<String>,
    /// Type after `:>`, e.g. Some("ISQ::mass").
    pub typing: Option<String>,
    /// Default or binding after `=` / `:=` / `default =` before the body terminator.
    pub value: Option<Node<Expression>>,
    pub body: AttributeBody,
    /// Span of the defined name (for semantic tokens).
    pub name_span: Option<Span>,
    /// Span of the type after `:>`, if present (for semantic tokens).
    pub typing_span: Option<Span>,
    /// Span of the default/binding expression value, when present.
    pub value_span: Option<Span>,
}

/// Body of an attribute (def or usage): `;` or `{` AttributeBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeBody {
    Semicolon,
    Brace {
        elements: Vec<Node<AttributeBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    AttributeDef(Node<AttributeDef>),
    AttributeUsage(Node<AttributeUsage>),
    Other(String),
}

/// Item definition: `item def` Identification body (for events, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemDef {
    pub identification: Identification,
    pub specializes: Option<String>,
    pub specializes_span: Option<Span>,
    pub body: AttributeBody,
}

/// Individual definition: `individual def` Identification `:>` type body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndividualDef {
    pub identification: Identification,
    pub specializes: Option<String>,
    pub specializes_span: Option<Span>,
    pub body: AttributeBody,
}

/// Part usage: `part` name `:` type multiplicity? `ordered`? (`redefines`|`:>>`)? value? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PartUsage {
    /// Optional `abstract` or `variation` prefix on a part usage.
    pub usage_prefix: Option<DefinitionPrefix>,
    pub is_individual: bool,
    pub name: String,
    /// Type after `:`, e.g. "Vehicle", "AxleAssembly".
    pub type_name: String,
    /// Multiplicity, e.g. Some("[2]").
    pub multiplicity: Option<String>,
    pub ordered: bool,
    /// Optional `subsets` feature and value expression.
    pub subsets: Option<(String, Option<Node<Expression>>)>,
    /// Redefines target, e.g. Some("frontAxleAssembly") or Some("vehicle1::mass").
    pub redefines: Option<String>,
    /// Value expression (= expr, default = expr, := expr).
    pub value: Option<Node<Expression>>,
    pub body: PartUsageBody,
    /// Span of the usage name (for semantic tokens).
    pub name_span: Option<Span>,
    /// Span of the type reference after `:` (for semantic tokens).
    pub type_ref_span: Option<Span>,
}

/// Body of a part usage: `;` or `{` PartUsageBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PartUsageBody {
    Semicolon,
    Brace {
        elements: Vec<Node<PartUsageBodyElement>>,
    },
}

/// Metadata annotation on usage: `@` Name (`:` Type)? (`about` targets)? MetadataBody.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetadataAnnotation {
    pub name: String,
    pub type_name: Option<String>,
    pub about_targets: Vec<String>,
    pub body: AttributeBody,
    pub head_span: Option<Span>,
    pub type_span: Option<Span>,
}

/// User-defined metadata keyword usage: `#keyword` (`:` Type)? (`about` targets)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetadataKeywordUsage {
    pub keyword: String,
    pub type_name: Option<String>,
    pub about_targets: Vec<String>,
    pub body: AttributeBody,
    pub keyword_span: Span,
    pub type_span: Option<Span>,
}

/// Generic annotation or metadata usage captured in body scopes.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Annotation {
    pub sigil: String,
    pub head: String,
    pub type_name: Option<String>,
    pub body: ConnectBody,
    pub head_span: Option<Span>,
    pub type_span: Option<Span>,
}

/// Element inside a part usage body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PartUsageBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    Annotation(Node<Annotation>),
    AttributeUsage(Node<AttributeUsage>),
    EnumerationUsage(Node<EnumerationUsage>),
    PartUsage(Box<Node<PartUsage>>),
    OccurrenceUsage(Box<Node<OccurrenceUsage>>),
    PortUsage(Node<PortUsage>),
    Bind(Node<Bind>),
    /// `ref` name `:` type body (reference binding in part usage).
    Ref(Node<RefDecl>),
    InterfaceUsage(Node<InterfaceUsage>),
    Connect(Node<Connect>),
    FlowUsage(Node<crate::ast::behavior::FlowUsage>),
    Perform(Node<Perform>),
    Allocate(Node<Allocate>),
    Satisfy(Node<Satisfy>),
    StateUsage(Node<StateUsage>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    /// `variant` name `;` inside a variation part usage body.
    VariantUsage(Node<VariantUsage>),
}

/// Variant reference inside a variation part usage: `variant` name `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariantUsage {
    pub name: String,
}

/// Enacted performance: `perform` action_path `{` body `}` inside a part usage.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Perform {
    /// Qualified action name (e.g. "provide power" or "provide power.generate torque").
    pub action_name: String,
    /// Type after `:` in "perform action name : Type" form.
    pub type_name: Option<String>,
    pub body: PerformBody,
}

/// Body of a perform: `;` or `{` PerformBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PerformBody {
    Semicolon,
    Brace {
        elements: Vec<Node<PerformBodyElement>>,
    },
}

/// Element inside a perform body: doc comment or in/out binding.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PerformBodyElement {
    Doc(Node<DocComment>),
    InOut(Node<PerformInOutBinding>),
}

/// In/out binding inside a perform body: `in` name `=` expr `;` or `out` name `=` expr `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PerformInOutBinding {
    pub direction: InOut,
    pub name: String,
    pub value: Node<Expression>,
}

/// Attribute usage: `attribute` name (`:>` | `:` type)? `redefines`? value? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeUsage {
    pub name: String,
    /// Type after `:` or `:>`, e.g. Some("MassValue").
    pub typing: Option<String>,
    /// Subsets target after `:>` / `subsets`.
    pub subsets: Option<String>,
    /// Redefines target, e.g. Some("Vehicle::mass").
    pub redefines: Option<String>,
    /// References target after `::>` / `references`.
    pub references: Option<String>,
    /// Crosses target after `=>` / `crosses`.
    pub crosses: Option<String>,
    /// Value expression.
    pub value: Option<Node<Expression>>,
    pub body: AttributeBody,
    /// Span of the usage name (for semantic tokens).
    pub name_span: Option<Span>,
    /// Span of the type after `:` / `:>`, if present (for semantic tokens).
    pub typing_span: Option<Span>,
    /// Span of the redefines target after `redefines`, if present (for semantic tokens).
    pub redefines_span: Option<Span>,
    /// Direction prefix when parsed as `in`/`out`/`inout attribute ...` (e.g. in port def bodies).
    pub direction: Option<InOut>,
}

// ---------------------------------------------------------------------------
// Port
// ---------------------------------------------------------------------------

/// Port definition: `port def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PortDef {
    pub identification: Identification,
    /// Supertype after `:>`, e.g. Some("ClutchPort") for `port def ManualClutchPort :> ClutchPort`.
    pub specializes: Option<String>,
    pub specializes_span: Option<Span>,
    pub body: PortDefBody,
}

/// Body of a port definition: `;` or `{` PortDefBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PortDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<PortDefBodyElement>>,
    },
}

/// Element inside a port definition body (in/out declarations or nested port usages).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PortDefBodyElement {
    InOutDecl(Node<InOutDecl>),
    Doc(Node<DocComment>),
    Error(Node<ParseErrorNode>),
    AttributeDef(Node<AttributeDef>),
    AttributeUsage(Node<AttributeUsage>),
    ItemUsage(Node<ItemUsage>),
    PortUsage(Node<PortUsage>),
    Other(String),
}

/// Port usage: `port` name `:` type multiplicity? `:>` subsets? `redefines`? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PortUsage {
    pub name: String,
    pub type_name: Option<String>,
    pub multiplicity: Option<String>,
    /// Subsets feature and optional value expression.
    pub subsets: Option<(String, Option<Node<Expression>>)>,
    pub redefines: Option<String>,
    /// References target after `::>` / `references`.
    pub references: Option<String>,
    /// Crosses target after `=>` / `crosses`.
    pub crosses: Option<String>,
    pub body: PortBody,
    /// Span of the usage name (for semantic tokens).
    pub name_span: Option<Span>,
    /// Span of the type reference after `:`, if present (for semantic tokens).
    pub type_ref_span: Option<Span>,
}

/// Body of a port usage: `;` or `{` PortBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PortBody {
    Semicolon,
    Brace {
        elements: Vec<Node<PortBodyElement>>,
    },
}

/// Element inside a port usage body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::large_enum_variant)]
pub enum PortBodyElement {
    Error(Node<ParseErrorNode>),
    InOutDecl(Node<InOutDecl>),
    PortUsage(Node<PortUsage>),
    Other(String),
}

/// Connect statement in interface def or usage: `connect` from `to` to body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConnectStmt {
    pub from: Node<Expression>,
    pub to: Node<Expression>,
    pub body: ConnectBody,
}

// ---------------------------------------------------------------------------
// Interface
// ---------------------------------------------------------------------------

/// Interface definition: `interface def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterfaceDef {
    pub identification: Identification,
    pub specializes: Option<String>,
    pub specializes_span: Option<Span>,
    pub body: InterfaceDefBody,
}

/// Body of an interface definition: `;` or `{` InterfaceDefBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InterfaceDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<InterfaceDefBodyElement>>,
    },
}

/// Element inside an interface definition body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InterfaceDefBodyElement {
    Doc(Node<DocComment>),
    EndDecl(Node<EndDecl>),
    RefDecl(Node<RefDecl>),
    ConnectStmt(Node<ConnectStmt>),
}

/// End declaration in interface def: `end` name `:` type `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndDecl {
    pub name: String,
    pub type_name: String,
    pub uses_derived_syntax: bool,
    /// Span of the name (for semantic tokens).
    pub name_span: Option<Span>,
    /// Span of the type after `:` (for semantic tokens).
    pub type_ref_span: Option<Span>,
}

/// Ref declaration in interface def: `ref` name `:` type body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RefDecl {
    pub name: String,
    pub type_name: String,
    /// Optional binding value: `= expr` (SysML shorthand binding for references).
    pub value: Option<Node<Expression>>,
    pub body: RefBody,
    /// Span of the name (for semantic tokens).
    pub name_span: Option<Span>,
    /// Span of the type after `:` (for semantic tokens).
    pub type_ref_span: Option<Span>,
}

/// Body of a ref declaration: `;` or `{` ... `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RefBody {
    Semicolon,
    Brace,
}

// ---------------------------------------------------------------------------
// Connection (Phase 2)
// ---------------------------------------------------------------------------

/// Connection definition: `connection def` Identification body (BNF ConnectionDefinition).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConnectionDef {
    pub annotation: Option<String>,
    pub identification: Identification,
    pub specializes: Option<String>,
    pub specializes_span: Option<Span>,
    pub body: ConnectionDefBody,
}

/// Body of a connection definition: `;` or `{` end/ref/connect* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConnectionDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<ConnectionDefBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConnectionDefBodyElement {
    EndDecl(Node<EndDecl>),
    RefDecl(Node<RefDecl>),
    ConnectStmt(Node<ConnectStmt>),
}

// ---------------------------------------------------------------------------
// Metadata (Phase 2)
// ---------------------------------------------------------------------------

/// Metadata definition: `metadata def` Identification body (BNF MetadataDefinition).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetadataDef {
    pub is_abstract: bool,
    pub identification: Identification,
    pub specializes: Option<String>,
    pub specializes_span: Option<Span>,
    pub body: AttributeBody,
}

/// Metadata usage: `metadata` name (`:` type)? (`about` targets)? body (BNF MetadataUsage).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetadataUsage {
    pub name: String,
    pub type_name: Option<String>,
    pub about_targets: Vec<String>,
    pub body: AttributeBody,
}

// ---------------------------------------------------------------------------
// Enumeration (Phase 2)
// ---------------------------------------------------------------------------

/// Enumeration definition: `enum def` Identification EnumerationBody (BNF EnumerationDefinition).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumDef {
    pub identification: Identification,
    pub specializes: Option<String>,
    pub specializes_span: Option<Span>,
    pub body: EnumerationBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EnumerationBody {
    Semicolon,
    Brace { values: Vec<String> },
}

// ---------------------------------------------------------------------------
// Occurrence (Phase 2)
// ---------------------------------------------------------------------------

/// Occurrence definition: `occurrence def` Identification body (BNF OccurrenceDefinition).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OccurrenceDef {
    pub is_abstract: bool,
    pub identification: Identification,
    pub specializes: Option<String>,
    pub specializes_span: Option<Span>,
    pub body: DefinitionBody,
}

/// Occurrence usage: `occurrence` name (`:` type)? body, with optional individual/portion modifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OccurrenceUsage {
    pub is_individual: bool,
    pub is_then: bool,
    pub portion_kind: Option<String>,
    pub name: String,
    pub type_name: Option<String>,
    pub subsets: Option<String>,
    pub redefines: Option<String>,
    pub references: Option<String>,
    pub crosses: Option<String>,
    pub body: OccurrenceUsageBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OccurrenceUsageBody {
    Semicolon,
    Brace {
        elements: Vec<Node<OccurrenceBodyElement>>,
    },
}

/// Occurrence-level assert member: `assert constraint` body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssertConstraintMember {
    pub body: ConstraintDefBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::large_enum_variant)]
pub enum OccurrenceBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    Annotation(Node<Annotation>),
    AssertConstraint(Node<AssertConstraintMember>),
    Other(String),
    FlowUsage(Node<crate::ast::behavior::FlowUsage>),
    AttributeUsage(Node<AttributeUsage>),
    PartUsage(Box<Node<PartUsage>>),
    OccurrenceUsage(Box<Node<OccurrenceUsage>>),
}

// ---------------------------------------------------------------------------
// Library Package (Phase 2)
// ---------------------------------------------------------------------------

/// Generic definition body: `;` or `{` DefinitionBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DefinitionBody {
    Semicolon,
    Brace {
        elements: Vec<Node<DefinitionBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::large_enum_variant)]
pub enum DefinitionBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    OccurrenceMember(Node<OccurrenceBodyElement>),
    Other(String),
}
// ---------------------------------------------------------------------------
// Part usage body: bind, interface usage, connect
// ---------------------------------------------------------------------------

/// Bind: `bind` left `=` right (`;` or `{ }`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bind {
    pub left: Node<Expression>,
    pub right: Node<Expression>,
    /// Optional body after the bind (semicolon or brace); 3a fixture uses `bind x = y { }`.
    pub body: Option<ConnectBody>,
}

/// Interface usage: typed+connect or connection form.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InterfaceUsage {
    /// `interface` `:Type`? `connect` from `to` to body; optional body with ref redefs.
    TypedConnect {
        interface_type: Option<String>,
        from: Node<Expression>,
        to: Node<Expression>,
        body: ConnectBody,
        body_elements: Vec<Node<InterfaceUsageBodyElement>>,
    },
    /// `interface` from `to` to body.
    Connection {
        from: Node<Expression>,
        to: Node<Expression>,
        body_elements: Vec<Node<InterfaceUsageBodyElement>>,
    },
}

/// Element in interface usage body (e.g. ref redefinition).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InterfaceUsageBodyElement {
    /// `ref` `:>>` name `=` value body.
    RefRedef {
        name: String,
        value: Node<Expression>,
        body: RefBody,
    },
}

/// Connect at part usage level: `connect` from `to` to body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Connect {
    pub from: Node<Expression>,
    pub to: Node<Expression>,
    pub body: ConnectBody,
}

// ---------------------------------------------------------------------------
// Alias
// ---------------------------------------------------------------------------

/// Alias definition: `alias` Identification `for` qualified_name body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AliasDef {
    pub identification: Identification,
    pub target: String,
    pub body: AliasBody,
}

/// Body of an alias definition: `;` or `{` ... `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AliasBody {
    Semicolon,
    Brace,
}
