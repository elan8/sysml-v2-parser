use super::behavior::{
    ActionDef, ActionDefBodyElement, ActionUsage, ActionUsageBodyElement, Allocate, InOut,
    InOutDecl, StateDefBody, StateUsage,
};
use super::common::{CommentAnnotation, ConnectBody, DocComment, Identification, ParseErrorNode};
use super::feature_value::FeatureValue;
use super::membership::Membership;
use super::relationship_target::RelationshipTarget;
use super::requirement::{Dependency, EnumerationUsage, ItemUsage, RequirementUsage, Satisfy};
use super::view::{CalcUsage, ConstraintDef, ConstraintDefBody, ConstraintUsage};
use crate::ast::core::{
    ConnectionEnd, Expression, Multiplicity, Node, Span, SubsettingRelationship, TypingRelationship,
};

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
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: PartDefBody,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006), `kind` always
    /// [`crate::ast::MembershipKind::OwningMembership`]. Unlike `AttributeDef`, `part_def` did not
    /// previously parse a `private`/`protected`/`public` prefix at all (BNF `DefinitionMember :
    /// OwningMembership = MemberPrefix ownedRelatedElement += DefinitionElement` legally allows
    /// one before any definition, including `PartDefinition`) -- confirmed as a genuine parsing
    /// gap (not just discarded data) by probing `package P { private part def Foo; }`, which fell
    /// through to `ExtendedLibraryDecl` before this item. See `part::def::part_def`.
    pub membership: Membership,
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
    /// A dependency owned by this definition (BNF `DefinitionMember`).
    Dependency(Node<Dependency>),
    Other(String),
    AttributeDef(Node<AttributeDef>),
    AttributeUsage(Node<AttributeUsage>),
    /// Bare `name : Type;` / `name = expr;` without a kind keyword (SysML DefaultReferenceUsage).
    DefaultReferenceUsage(Node<DefaultReferenceUsage>),
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
    /// `constraint def` nested inside a part definition body (§6 G4: `constraint` was wired at
    /// package level only, so both the definition and the usage form fell through to opaque
    /// recovery here).
    ConstraintDef(Node<ConstraintDef>),
    /// `constraint <name>[: Type] { ... }` usage inside a part definition body. See
    /// [`PartDefBodyElement::ConstraintDef`].
    ConstraintUsage(Node<ConstraintUsage>),
    /// `(private|public|protected)? import <qualified-name>;` inside a part definition body
    /// (§6 G16). See [`PartUsageBodyElement::Import`].
    Import(Node<crate::ast::Import>),
    /// `action` / `ref action` usage inside a part definition body (Systems Library
    /// `Parts::performedActions` and similar). Previously fell through to [`OpaqueMemberDecl`].
    ActionUsage(Box<Node<ActionUsage>>),
    /// `action def` nested inside a part definition body (GH-14: previously only `ActionUsage`
    /// was reachable here, so a nested definition fell through to opaque recovery with a
    /// misleading "expected ';' or '{' after action definition header" diagnostic).
    ActionDef(Node<ActionDef>),
    /// `state` / `ref state` usage inside a part definition body. Previously fell through to
    /// [`OpaqueMemberDecl`].
    StateUsage(Node<StateUsage>),
    /// Enumeration usage (`enum` keyword) inside a part definition body.
    EnumerationUsage(Node<EnumerationUsage>),
    /// `assert (not)? constraint { ... }` inside a part definition body (previously only
    /// reachable from occurrence definition bodies).
    AssertConstraint(Node<AssertConstraintMember>),
    /// `satisfy <ref> (by <expr>)?;` inside a part definition body (previously only reachable
    /// at package level).
    Satisfy(Node<Satisfy>),
    /// `variant` name `;` — a variant member inside a `variation part def` body.
    VariantUsage(Node<VariantUsage>),
    /// `state def` nested inside a part definition body (PAR-002: previously only reachable
    /// via the `exhibit state` usage wrapper, never as a standalone nested definition).
    StateDef(Node<crate::ast::behavior::StateDef>),
    /// `metadata def` nested inside a part definition body (PAR-002: previously only
    /// `MetadataAnnotation`/`MetadataKeywordUsage` were reachable here).
    MetadataDef(Node<MetadataDef>),
    /// `metadata` usage (no `def`) nested inside a part definition body. See `MetadataDef`.
    MetadataUsage(Node<MetadataUsage>),
    /// `flow def` nested inside a part definition body (PAR-002: previously only `FlowUsage`
    /// was reachable here).
    FlowDef(Node<crate::ast::behavior::FlowDef>),
    /// `requirement def` nested inside a part definition body (PAR-002: previously only
    /// `RequirementUsage` was reachable here).
    RequirementDef(Node<crate::ast::requirement::RequirementDef>),
    /// `occurrence def` nested inside a part definition body (PAR-002: previously only
    /// `OccurrenceUsage` was reachable here).
    OccurrenceDef(Node<OccurrenceDef>),
    /// `connection def` nested inside a part definition body (PAR-002: previously only
    /// `ConnectionUsageMember`, a usage shape, was reachable here).
    ConnectionDef(Node<ConnectionDef>),
    /// `port def` nested inside a part definition body, using `port_def_required` (PAR-002:
    /// previously only `PortUsage` was reachable here).
    PortDef(Node<PortDef>),
    /// `calc def` nested inside a part definition body, using `calc_def_required` (PAR-002:
    /// previously only `CalcUsage` was reachable here).
    CalcDef(Node<crate::ast::view::CalcDef>),
    /// `enum def` nested inside a part definition body (PAR-002: previously only
    /// `EnumerationUsage` was reachable here; `enum_def` is `def_required()`-guarded internally
    /// so no ordering risk stacking it ahead of `enum_usage`, already dispatched above).
    EnumDef(Node<EnumDef>),
    AllocationDef(Node<crate::ast::behavior::AllocationDef>),
    AllocationUsage(Node<crate::ast::behavior::AllocationUsage>),
    ViewDef(Node<crate::ast::view::ViewDef>),
    ViewUsage(Node<crate::ast::view::ViewUsage>),
    ViewpointDef(Node<crate::ast::view::ViewpointDef>),
    ViewpointUsage(Node<crate::ast::view::ViewpointUsage>),
    RenderingDef(Node<crate::ast::view::RenderingDef>),
    RenderingUsage(Node<crate::ast::view::RenderingUsage>),
    CaseDef(Node<crate::ast::requirement::CaseDef>),
    CaseUsage(Node<crate::ast::requirement::CaseUsage>),
    UseCaseDef(Node<crate::ast::requirement::UseCaseDef>),
    UseCaseUsage(Node<crate::ast::requirement::UseCaseUsage>),
    AnalysisCaseDef(Node<crate::ast::requirement::AnalysisCaseDef>),
    AnalysisCaseUsage(Node<crate::ast::requirement::AnalysisCaseUsage>),
    VerificationCaseDef(Node<crate::ast::requirement::VerificationCaseDef>),
    VerificationCaseUsage(Node<crate::ast::requirement::VerificationCaseUsage>),
    /// `first ... then ...` succession (BNF `SuccessionAsUsage`, §8.2.2.13.3) nested directly
    /// inside a part definition body (GH-40: previously only reachable inside action bodies,
    /// even though `ConnectionTest.sysml` uses the bare and `succession`-prefixed forms directly
    /// inside a `part def`). `then` is mandatory here -- unlike inside an action body, the
    /// `then`-less `first target;` initial-node marker (BNF `InitialNodeMember`) is not part of
    /// the generic `DefinitionBodyItem` grammar a part def body uses, so it is rejected by the
    /// parser rather than represented by this variant.
    FirstStmt(Node<crate::ast::behavior::FirstStmt>),
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
    /// Multiplicity after the type, e.g. `[0..1]` in `connection trailerHitch :
    /// TrailerHitch[0..1];` (OMG spec Annex `3c-Function-based Behavior-structure mod.sysml`).
    pub multiplicity: Option<Node<Multiplicity>>,
    /// Optional inline `connect from to to (, extra)* ` clause (PAR-007 widening): a package- or
    /// part-body-level `connection name : Type connect a to b;` usage. `None` for a plain
    /// `connection name : Type;` declaration with no explicit binding. See `connect_to`/
    /// `connect_extra_ends` for the rest of the ends and `connection_def`'s doc comment for why
    /// this shape previously reached `ConnectionDef` instead of here.
    pub connect_from: Option<Node<ConnectionEnd>>,
    pub connect_to: Option<Node<ConnectionEnd>>,
    /// Additional ends beyond `connect_from`/`connect_to`, from the n-ary
    /// `connect (a, b, c, ...)` form. Always empty when `connect_from` is `None`.
    pub connect_extra_ends: Vec<Node<ConnectionEnd>>,
    pub body: ConnectionDefBody,
    pub subsets: Option<Node<SubsettingRelationship>>,
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006), `kind` always
    /// [`crate::ast::MembershipKind::FeatureMembership`]. See [`PortDef::membership`] for the
    /// same "genuine new grammar coverage, not just discarded data" rationale --
    /// `connection_usage_member` did not previously accept a visibility prefix either.
    pub membership: Membership,
}

/// Exhibit state usage: `OccurrenceUsagePrefix` subset `exhibit` (`state`)? name (`:` type)?
/// (`:>`/`:>>` …)? body. GH-27: rebuilt on the same shared prefix/specialization helpers
/// `state_usage` composes (see `crate::parser::part::body::exhibit_state`), so it carries the
/// same header fields as [`StateUsage`], including the `direction`/`is_derived`/`is_individual`
/// slots added for GH-45.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExhibitState {
    /// Leading direction (`in`/`out`/`inout`).
    pub direction: Option<InOut>,
    /// Leading `derived` keyword.
    pub is_derived: bool,
    /// Leading `abstract` keyword.
    pub is_abstract: bool,
    /// Leading `ref` keyword — reference feature usage (`ref exhibit …`).
    pub is_reference: bool,
    /// Leading `individual` keyword (after `ref`, per `OccurrenceUsagePrefix` order).
    pub is_individual: bool,
    pub name: String,
    pub type_name: Option<String>,
    /// Structured typing clause when a `:` target was written.
    pub typing: Option<Node<TypingRelationship>>,
    /// Multiplicity after the type, when present.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// Optional `subsets` / `:>` clause.
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// Optional `redefines` / `:>>` clause, from before or after the body.
    pub redefines: Option<Node<SubsettingRelationship>>,
    pub body: StateDefBody,
    pub membership: Membership,
}

/// Attribute definition: `attribute` `def` name (`:>` | `:` type)? (`=` value)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeDef {
    pub name: String,
    /// Short name from `< ... >` when present (e.g. unit symbol `m`, `EUR`).
    pub short_name: Option<String>,
    /// Type after `:>`, e.g. `Some(TypingRelationship { target: "ISQ::mass", .. })` (PAR-004
    /// item 1). `typing_span` duplicates the node's own `span` for existing consumers that read
    /// the span without the node.
    pub typing: Option<Node<TypingRelationship>>,
    /// Default or binding after `=` / `:=` / `default =` before the body terminator.
    pub value: Option<Node<FeatureValue>>,
    pub body: AttributeBody,
    /// Span of the defined name (for semantic tokens).
    pub name_span: Option<Span>,
    /// Span of the type after `:>`, if present (for semantic tokens).
    pub typing_span: Option<Span>,
    /// Span of the default/binding expression value, when present.
    pub value_span: Option<Span>,
    /// `ordered` keyword from `MultiplicityPart` (BNF §8.2.2.6.6). Legal on a feature
    /// declaration generally; previously consumed and discarded by `ignored_feature_modifiers`.
    pub ordered: bool,
    /// `nonunique` keyword from `MultiplicityPart`. See `ordered`.
    pub nonunique: bool,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006). `kind` is always
    /// [`crate::ast::MembershipKind::OwningMembership`] for a `*Def` -- a nested attribute definition becomes
    /// a new named member of its owning namespace, not a feature of it. `visibility` captures an
    /// explicit `private`/`protected`/`public` prefix when written (previously matched and
    /// discarded by `attribute_def`'s parser); `None` means no explicit prefix was written (the
    /// owning namespace's default visibility applies, not resolved here). Non-optional --
    /// `attribute_def` unconditionally builds one, using `visibility: None` when no prefix was
    /// written.
    pub membership: Membership,
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

// See `RequirementDefBodyElement`'s `#[allow(clippy::large_enum_variant)]` doc comment --
// `AttributeUsage`'s size relative to `Doc`/`Error` is an accepted, crate-wide tradeoff, not
// specific to this enum.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    AttributeDef(Node<AttributeDef>),
    AttributeUsage(Node<AttributeUsage>),
    /// `occurrence ...` usage (§6 G27). `AttributeBody` is shared with `item def` / `item` usage
    /// bodies, and an item *is* an occurrence, so `occurrence :>> causes;` is a legal member --
    /// see the OMG spec Annex `14c-Language Extensions.sysml`.
    OccurrenceUsage(Box<Node<OccurrenceUsage>>),
    /// `connect a to b;` connector usage. `AttributeBody` is shared with `item def`/`item` usage
    /// bodies, which legally own connector members (e.g. the FMEA library's `#prevention connect
    /// 'battery depleted' to req1;` in `14c-Language Extensions.sysml`).
    Connect(Node<Connect>),
    /// `#keyword` metadata tag, bare or `PrefixMetadataMember`-style prefixing the next member
    /// (see `PackageBodyElement::MetadataKeywordUsage`). `14c-Language Extensions.sysml`'s FMEA
    /// library example prefixes almost every member in these shared bodies with one.
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    Other(String),
}

/// Item definition: `item def` Identification body (for events, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: AttributeBody,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006), `kind` always
    /// [`crate::ast::MembershipKind::OwningMembership`]. Like `PartDef`/`PortDef`, `item_def`/
    /// `item_def_required` did not previously accept a visibility prefix at all -- confirmed as
    /// a genuine parsing gap the same way. See `item::parse_item_def`.
    pub membership: Membership,
}

/// Individual definition: `individual def` Identification `:>` type body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndividualDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: AttributeBody,
    pub membership: Membership,
}

/// Part usage: `part` name `:` type multiplicity? `ordered`? (`redefines`|`:>>`)? value? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PartUsage {
    /// Optional `abstract` or `variation` prefix on a part usage.
    pub usage_prefix: Option<DefinitionPrefix>,
    pub is_individual: bool,
    /// Leading `ref` from BNF `BasicUsagePrefix` (`isReference ?= 'ref'`), reached via
    /// `PartUsage = OccurrenceUsagePrefix 'part' Usage` → `OccurrenceUsagePrefix :
    /// BasicUsagePrefix ...`. Distinguishes `ref part origin : T :> x;` from composite
    /// `part origin : T :> x;`. Parallel to `ActionUsage::is_reference` / `StateUsage::is_reference`.
    pub is_reference: bool,
    /// Direction prefix when parsed as `in`/`out`/`inout part ...` (BNF `RefPrefix`, reachable
    /// through `OccurrenceUsagePrefix` -> `BasicUsagePrefix` -> `RefPrefix`, same production
    /// chain `AttributeUsage.direction` uses).
    pub direction: Option<InOut>,
    /// `derived` keyword from `RefPrefix` -- see `AttributeUsage::is_derived` for the BNF
    /// citation; the same `OccurrenceUsagePrefix` chain applies to part usages.
    pub is_derived: bool,
    /// `constant` keyword from `RefPrefix` -- see `AttributeUsage::is_constant`.
    pub is_constant: bool,
    pub name: String,
    /// Short name from `< ... >` when present. See `AttributeUsage::short_name`.
    pub short_name: Option<String>,
    /// Type after `:`, e.g. "Vehicle", "AxleAssembly". A comma-separated multi-target clause
    /// (`part vehicle : Vehicle, SpatialItem;`) joins into one display string here; see `typing`
    /// for the structured, multi-target-capable form (S42-004).
    pub type_name: String,
    /// Structured typing clause mirroring `AttributeUsage.typing`: every comma-separated target
    /// from `:`/`defined by`/`typed by`, not just the first (S42-004). `None` when no typing
    /// clause was written (`type_name` is then empty).
    pub typing: Option<Node<TypingRelationship>>,
    /// Multiplicity, e.g. `[2]` parsed into structured lower/upper bounds.
    pub multiplicity: Option<Node<Multiplicity>>,
    pub ordered: bool,
    /// Optional `subsets` feature and value expression.
    pub subsets: Option<(Node<SubsettingRelationship>, Option<Node<Expression>>)>,
    /// Redefines target, e.g. `frontAxleAssembly` or `vehicle1::mass`.
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// Value expression (= expr, default = expr, := expr).
    pub value: Option<Node<FeatureValue>>,
    pub body: PartUsageBody,
    /// Span of the usage name (for semantic tokens).
    pub name_span: Option<Span>,
    /// Span of the type reference after `:` (for semantic tokens).
    pub type_ref_span: Option<Span>,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006), `kind` always
    /// [`crate::ast::MembershipKind::FeatureMembership`]. See [`PartDef::membership`] for the
    /// same "genuine new grammar coverage, not just discarded data" rationale -- `part_usage` did
    /// not previously accept a visibility prefix either.
    pub membership: Membership,
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
    /// Bare `name : Type;` without a kind keyword (SysML DefaultReferenceUsage).
    DefaultReferenceUsage(Node<DefaultReferenceUsage>),
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
    /// `action` / `ref action` usage inside a part usage body.
    ActionUsage(Box<Node<ActionUsage>>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    /// `variant` name `;` inside a variation part usage body.
    VariantUsage(Node<VariantUsage>),
    /// `state def` nested inside a part usage body (PAR-002: usage bodies legally contain
    /// nested definitions per BNF `UsageBody = DefinitionBody`, previously zero Def-kind
    /// variants existed on this enum at all).
    StateDef(Node<crate::ast::behavior::StateDef>),
    /// `metadata def` nested inside a part usage body. See `StateDef`.
    MetadataDef(Node<MetadataDef>),
    /// `flow def` nested inside a part usage body. See `StateDef`.
    FlowDef(Node<crate::ast::behavior::FlowDef>),
    /// `requirement def` nested inside a part usage body. See `StateDef`.
    RequirementDef(Node<crate::ast::requirement::RequirementDef>),
    /// `occurrence def` nested inside a part usage body. See `StateDef`.
    OccurrenceDef(Node<OccurrenceDef>),
    /// `port def` nested inside a part usage body, using `port_def_required`. See `StateDef`.
    PortDef(Node<PortDef>),
    /// `calc def` nested inside a part usage body, using `calc_def_required`. See `StateDef`.
    CalcDef(Node<crate::ast::view::CalcDef>),
    /// `connection def` nested inside a part usage body, using `connection_def_required`. See
    /// `StateDef`.
    ConnectionDef(Node<ConnectionDef>),
    /// `enum def` nested inside a part usage body. See `StateDef`.
    EnumDef(Node<EnumDef>),
    /// `connection` usage member inside a part usage body (previously only reachable from part
    /// definition bodies; see `PartDefBodyElement::Connection`).
    Connection(Node<ConnectionUsageMember>),
    /// `assert (not)? constraint { ... }` inside a part usage body (previously only reachable
    /// from part definition and occurrence definition bodies; see
    /// `PartDefBodyElement::AssertConstraint`).
    AssertConstraint(Node<AssertConstraintMember>),
    /// `constraint def` nested inside a part usage body (§6 G4). See
    /// `PartDefBodyElement::ConstraintDef`.
    ConstraintDef(Node<ConstraintDef>),
    /// `constraint <name>[: Type] { ... }` usage inside a part usage body (§6 G4). See
    /// `PartDefBodyElement::ConstraintUsage`.
    ConstraintUsage(Node<ConstraintUsage>),
    /// `(private|public|protected)? import <qualified-name>;` inside a part usage body (§6 G16).
    /// Imports are namespace members, and a part usage body is a namespace; real usage is
    /// OMG spec Annex `8-Requirements.sysml`.
    Import(Node<crate::ast::Import>),
    /// `requirement <name> : Type { ... }` usage inside a part usage body (§6 G5: previously
    /// reachable from part *definition* bodies only).
    RequirementUsage(Node<RequirementUsage>),
    /// `item def` nested inside a part usage body (§6 G25). See `StateDef`.
    ItemDef(Node<ItemDef>),
    /// `item <name> : Type (;|{ ... })` usage inside a part usage body (§6 G25: previously
    /// reachable from part *definition* bodies only, so every `item` member of a nested part
    /// usage -- e.g. `part fuelTank : FuelTank { item fuel : Fuel; }` in the OMG spec Annex
    /// `3d-Function-based Behavior-item.sysml` -- fell into error recovery).
    ItemUsage(Node<ItemUsage>),
    /// `metadata <name> { ... }` usage inside a part usage body (previously only reachable from
    /// part *definition* bodies; GH-12 recovery made the silent `ExtendedLibraryDecl` fallthrough
    /// visible). Real usage: OMG Annex `14a-Language Extensions.sysml`.
    MetadataUsage(Node<MetadataUsage>),
    /// `analysis def` nested inside a part usage body. See `MetadataUsage`.
    AnalysisCaseDef(Node<crate::ast::requirement::AnalysisCaseDef>),
    /// `analysis <name> : Type { ... }` usage inside a part usage body. Real usage: OMG Annex
    /// `10c-Fuel Economy Analysis.sysml`.
    AnalysisCaseUsage(Node<crate::ast::requirement::AnalysisCaseUsage>),
}

/// Variant member inside a variation part usage/def body: either an untyped reference to a
/// separately-declared usage (`variant name;`), or a typed usage declared inline with a kind
/// keyword (`variant part name : Type { ... }`, `variant attribute name = expr;`, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariantUsage {
    /// The variant's own name — the referenced usage's name for the untyped form, or the
    /// nested usage's own name for the typed form.
    pub name: String,
    /// Present when declared with a kind keyword (`variant part ...;`); `None` for the untyped
    /// reference form (`variant name;`).
    pub typed: Option<VariantTypedUsage>,
    /// Ownership/visibility/kind wrapper (parser work item 4b final sweep), `kind` always
    /// [`crate::ast::MembershipKind::VariantMembership`] -- confirmed against
    /// `SysML-textual-bnf.kebnf`'s `VariantUsageMember : VariantMembership = MemberPrefix
    /// 'variant' ownedVariantUsage = VariantUsageElement`, which legally carries a visibility
    /// prefix before this increment added support for parsing one.
    pub membership: Membership,
}

/// The nested usage of a typed `variant` member (BNF `VariantUsageElement`, restricted here to
/// the kinds most commonly used for variability modeling).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VariantTypedUsage {
    Part(Box<Node<PartUsage>>),
    Attribute(Box<Node<AttributeUsage>>),
    Item(Box<Node<ItemUsage>>),
    Port(Box<Node<PortUsage>>),
    /// `variant perform doX;` inside a `variation perform action ... { ... }` body (§6 G5).
    Perform(Box<Node<Perform>>),
}

/// Enacted performance: `perform` action_path `{` body `}` inside a part usage.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Perform {
    /// Optional `abstract` / `variation` prefix (§6 G5). `variation perform action doXorY { ... }`
    /// is real usage in the OMG spec Annex `7a1-Variant Configuration - General Concept-a.sysml`.
    pub usage_prefix: Option<DefinitionPrefix>,
    /// Qualified action name (e.g. "provide power" or "provide power.generate torque"). Empty for
    /// the anonymous forms (`perform action { ... }`, `perform action :>> target = value;`),
    /// matching [`PartUsage::name`]'s convention.
    pub action_name: String,
    /// Type after `:` in "perform action name : Type" form.
    pub type_name: Option<String>,
    /// Redefinition target after `:>>`, e.g. `doXorY` in `perform action :>> doXorY = doX;`.
    pub redefines: Option<String>,
    /// Bound value after `=`, e.g. `doX` in `perform action :>> doXorY = doX;`.
    pub value: Option<Node<FeatureValue>>,
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

/// Element inside a perform body: doc comment, in/out binding, or variant member.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PerformBodyElement {
    Doc(Node<DocComment>),
    InOut(Node<PerformInOutBinding>),
    /// `variant perform doX;` inside a `variation perform action ... { ... }` body (§6 G5).
    Variant(Node<VariantUsage>),
    /// Any action-body member inside an anonymous `perform action { ... }` (§6 G20), which owns a
    /// real action body rather than just parameter bindings. Delegates to the same dispatcher
    /// `ActionUsageBody` uses instead of duplicating the control-node grammar here.
    Action(Box<Node<ActionUsageBodyElement>>),
    /// Directed or undirected `part` usage inside a perform body (§6 G6), e.g.
    /// `in part :>> testVehicle = vehicleUnderTest;`.
    PartUsage(Box<Node<PartUsage>>),
    /// `in`/`out`/`inout item` usage inside a perform body (§6 G6).
    ItemUsage(Box<Node<ItemUsage>>),
    /// `in`/`out`/`inout attribute` usage inside a perform body (§6 G6).
    AttributeUsage(Box<Node<AttributeUsage>>),
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
    /// Short name from `< ... >` when present, e.g. `attribute <wcf> wheelCoordinateFrame : ...`
    /// (confirmed real usage in the OMG Geometry domain library's
    /// `VehicleGeometryAndCoordinateFrames.sysml`). See `AttributeDef::short_name`.
    pub short_name: Option<String>,
    /// Type after `:` or `:>`, e.g. `Some(TypingRelationship { target: "MassValue", .. })`
    /// (PAR-004 item 1). `typing_span` duplicates the node's own `span`.
    pub typing: Option<Node<TypingRelationship>>,
    /// Subsets target after `:>` / `subsets`.
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// Redefines target, e.g. `Vehicle::mass`.
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// References target after `::>` / `references`.
    pub references: Option<Node<SubsettingRelationship>>,
    /// Crosses target after `=>` / `crosses`.
    pub crosses: Option<Node<SubsettingRelationship>>,
    /// Intersects target(s) after `intersects`.
    pub intersects: Option<Node<SubsettingRelationship>>,
    /// Value expression.
    pub value: Option<Node<FeatureValue>>,
    pub body: AttributeBody,
    /// Span of the usage name (for semantic tokens).
    pub name_span: Option<Span>,
    /// Span of the type after `:` / `:>`, if present (for semantic tokens).
    pub typing_span: Option<Span>,
    /// Span of the redefines target after `redefines`, if present (for semantic tokens).
    pub redefines_span: Option<Span>,
    /// Direction prefix when parsed as `in`/`out`/`inout attribute ...` (e.g. in port def bodies).
    pub direction: Option<InOut>,
    /// Structured multiplicity range from `MultiplicityPart` (e.g. `[0..1]`), when present.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `ordered` keyword from `MultiplicityPart` (BNF §8.2.2.6.6).
    pub ordered: bool,
    /// `nonunique` keyword from `MultiplicityPart`.
    pub nonunique: bool,
    /// `derived` keyword from `RefPrefix` (BNF §8.2.2.6.2) -- usage-only, no `Definition`
    /// equivalent (`AttributeDefinition` uses `DefinitionPrefix`, which has no `derived`).
    pub is_derived: bool,
    /// `constant` keyword from `RefPrefix` -- usage-only, same rationale as `is_derived`.
    pub is_constant: bool,
    /// `end` keyword from `EndUsagePrefix` (BNF §8.2.2.6.2, `isEnd ?= 'end'`) -- an alternative
    /// to `RefPrefix` reached through the same `UsagePrefix` production `AttributeUsage` uses
    /// (`UsagePrefix 'attribute' Usage`). Distinct from the unrelated `EndDecl`/`end_decl`
    /// construct (`connection.rs`/`interface.rs`), which is a separate named-connector-end
    /// declaration (`end name : Type;`), not this boolean prefix modifier.
    pub is_end: bool,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006). `kind` is always
    /// [`crate::ast::MembershipKind::FeatureMembership`] for a `*Usage` -- a nested attribute usage
    /// contributes a feature to its owning type, as opposed to a `*Def`'s
    /// [`crate::ast::MembershipKind::OwningMembership`]. See [`AttributeDef::membership`] for the
    /// `visibility` capture rationale; the same "matched and previously discarded" prefix applies
    /// here.
    pub membership: Membership,
}

/// SysML `DefaultReferenceUsage` (BNF §8.2.2.6 / Spec §7.6.4): a usage without a kind keyword,
/// e.g. `Capacity : Real;`. Distinct from [`AttributeUsage`], which requires the `attribute`
/// keyword. Historically parsed via `attribute_usage_shorthand` into `AttributeUsage`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefaultReferenceUsage {
    pub name: String,
    /// Type after `:` / `defined by` / `typed by`.
    pub typing: Option<Node<TypingRelationship>>,
    /// Optional feature value after `=` / `default`.
    pub value: Option<Node<FeatureValue>>,
    pub name_span: Option<Span>,
    pub typing_span: Option<Span>,
    pub membership: Membership,
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
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: PortDefBody,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006), `kind` always
    /// [`crate::ast::MembershipKind::OwningMembership`]. Like `PartDef`, `port_def`/
    /// `port_def_required` did not previously accept a visibility prefix at all (BNF
    /// `DefinitionMember : OwningMembership = MemberPrefix ownedRelatedElement +=
    /// DefinitionElement` legally allows one before any definition) -- confirmed as a genuine
    /// parsing gap the same way as `PartDef::membership`. See `port::parse_port_def`.
    pub membership: Membership,
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
    /// `item def` nested inside a port definition body, using `item_def_required` (PAR-002:
    /// previously only `ItemUsage` was reachable here).
    ItemDef(Node<ItemDef>),
    ItemUsage(Node<ItemUsage>),
    /// Enumeration usage nested inside a port definition body (PAR-002 widening).
    EnumerationUsage(Node<EnumerationUsage>),
    PortUsage(Node<PortUsage>),
    Other(String),
}

/// Port usage: `port` name `:` type multiplicity? `:>` subsets? `redefines`? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PortUsage {
    /// Direction prefix when parsed as `in`/`out`/`inout port ...` (BNF `RefPrefix`, reachable
    /// through `OccurrenceUsagePrefix` -> `BasicUsagePrefix` -> `RefPrefix`).
    pub direction: Option<InOut>,
    /// Leading `abstract` keyword (Systems Library e.g. `abstract port ownedPorts`).
    pub is_abstract: bool,
    /// `derived` keyword from `RefPrefix`. See `AttributeUsage::is_derived`.
    pub is_derived: bool,
    /// `constant` keyword from `RefPrefix`. See `AttributeUsage::is_constant`.
    pub is_constant: bool,
    pub name: String,
    /// Short name from `< ... >` when present. See `AttributeUsage::short_name`.
    pub short_name: Option<String>,
    pub type_name: Option<String>,
    pub multiplicity: Option<Node<Multiplicity>>,
    /// Subsets feature and optional value expression.
    pub subsets: Option<(Node<SubsettingRelationship>, Option<Node<Expression>>)>,
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// References target after `::>` / `references`.
    pub references: Option<Node<SubsettingRelationship>>,
    /// Crosses target after `=>` / `crosses`.
    pub crosses: Option<Node<SubsettingRelationship>>,
    /// Intersects target(s) after `intersects`.
    pub intersects: Option<Node<SubsettingRelationship>>,
    /// `= expr` / `:= expr` feature value (§6 G11). A port usage carrying a value is the
    /// binding-connector shorthand -- `port :>> pe = c1.pb;` in the OMG spec Annex
    /// `2c-Parts Interconnection-Multiple Decompositions.sysml` binds the redefined port to
    /// another port instead of declaring a fresh one.
    pub value: Option<Node<crate::ast::FeatureValue>>,
    pub body: PortBody,
    /// Span of the usage name (for semantic tokens).
    pub name_span: Option<Span>,
    /// Span of the type reference after `:`, if present (for semantic tokens).
    pub type_ref_span: Option<Span>,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006), `kind` always
    /// [`crate::ast::MembershipKind::FeatureMembership`]. See [`PortDef::membership`] for the
    /// same "genuine new grammar coverage, not just discarded data" rationale -- `port_usage` did
    /// not previously accept a visibility prefix either.
    pub membership: Membership,
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
    Doc(Node<DocComment>),
    /// Attribute usage nested inside a port usage body (PAR-002 widening; this enum previously
    /// had no attribute/item coverage at all).
    AttributeUsage(Node<AttributeUsage>),
    /// Item usage nested inside a port usage body. See `AttributeUsage`.
    ItemUsage(Node<ItemUsage>),
}

/// Connect statement in interface def or usage: `connect` from `to` to body, or the SysML v2
/// n-ary form `connect (a, b, c, ...) body` (`NaryConnectorPart`/`NaryInterfacePart`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConnectStmt {
    pub from: Node<ConnectionEnd>,
    pub to: Node<ConnectionEnd>,
    /// Additional ends beyond `from`/`to` from the parenthesized n-ary form; empty for the
    /// ordinary binary `from ... to ...` form.
    pub extra_ends: Vec<Node<ConnectionEnd>>,
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
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: InterfaceDefBody,
    pub membership: Membership,
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
    /// PAR-002 widening: this enum previously had no attribute/item/port coverage at all.
    AttributeDef(Node<AttributeDef>),
    AttributeUsage(Node<AttributeUsage>),
    /// `item def`, using `item_def_required`. See `AttributeDef`.
    ItemDef(Node<ItemDef>),
    ItemUsage(Node<ItemUsage>),
    /// `port def`, using `port_def_required`. See `AttributeDef`.
    PortDef(Node<PortDef>),
    PortUsage(Node<PortUsage>),
}

/// End declaration in interface/connection def: `end` name (`:` type | (`::>` | `references`)
/// target) `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndDecl {
    pub name: String,
    /// Display string for the type (`:` form) or reference target (`::>`/`references` form).
    /// Kept for backward compatibility and simple display; see `references` for the structured
    /// reference-subsetting form (GH-19).
    pub type_name: String,
    /// True when this end used `::>`/`references` reference subsetting instead of `:` typing.
    pub uses_derived_syntax: bool,
    /// Structured reference-subsetting relationship for the `::>`/`references` form (GH-19):
    /// `end name ::> target;` / `end name references target;` names a reference, not a type, so
    /// it must not be modeled as typing (`endType`) downstream. `None` for the `end name : Type;`
    /// typing form.
    pub references: Option<Node<SubsettingRelationship>>,
    /// Optional multiplicity after the type/reference target, e.g. `[1]` in `end hub ::>
    /// mainSwitch[1];` (BNF `DefaultInterfaceEnd`'s `Usage` production carries the same optional
    /// multiplicity every other usage declaration does). `None` when absent.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// Span of the name (for semantic tokens).
    pub name_span: Option<Span>,
    /// Span of the type/reference target after `:`/`::>`/`references` (for semantic tokens).
    pub type_ref_span: Option<Span>,
}

/// Ref declaration in interface def: `ref` name `:` type body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RefDecl {
    pub name: String,
    /// Type after `:`, e.g. "Vehicle". A comma-separated multi-target clause joins into one
    /// display string here; see `typing` for the structured, multi-target-capable form.
    pub type_name: String,
    /// Structured typing clause mirroring `PartUsage.typing`/`AttributeUsage.typing`: every
    /// comma-separated target from a `:` clause, not just the first (S42-004). `None` when no
    /// typing clause was written (`type_name` is then empty).
    pub typing: Option<Node<TypingRelationship>>,
    /// Redefines target from an optional leading `:>>` clause, e.g. `ref sentMessage :>>
    /// sentTransfer: MessageTransfer, MessageAction { ... }` (Systems Library `Actions.sysml`).
    /// Previously this whole `:>> target : type` combination silently discarded the redefines
    /// target and the entire typing clause as unparsed text once `:>>` was seen.
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// Optional binding value: `= expr` (SysML shorthand binding for references).
    pub value: Option<Node<FeatureValue>>,
    pub body: RefBody,
    /// Span of the name (for semantic tokens).
    pub name_span: Option<Span>,
    /// Span of the type after `:` (for semantic tokens).
    pub type_ref_span: Option<Span>,
    /// Ownership/visibility wrapper (`FeatureMembership`). Populated when a visibility prefix
    /// precedes `ref` (e.g. `private ref mass : MassValue;`).
    pub membership: Membership,
}

/// Body of a ref declaration: `;` or `{` members `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RefBody {
    Semicolon,
    /// Braced body. `elements` is populated for action-context ref bodies;
    /// other contexts (state, part, interface) produce an empty vec.
    Brace {
        elements: Vec<Node<ActionDefBodyElement>>,
    },
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
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: ConnectionDefBody,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006), `kind` always
    /// [`crate::ast::MembershipKind::OwningMembership`]. Like `PartDef`/`PortDef`/`ItemDef`,
    /// `connection_def`/`connection_def_required` did not previously accept a visibility prefix
    /// at all -- confirmed as a genuine parsing gap the same way. See
    /// `connection::parse_connection_def`.
    pub membership: Membership,
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
    Doc(Node<DocComment>),
    Error(Node<ParseErrorNode>),
    /// PAR-002 widening: this enum previously had no attribute/item/port coverage at all.
    AttributeDef(Node<AttributeDef>),
    AttributeUsage(Node<AttributeUsage>),
    /// `item def`, using `item_def_required`. See `AttributeDef`.
    ItemDef(Node<ItemDef>),
    ItemUsage(Node<ItemUsage>),
    /// `port def`, using `port_def_required`. See `AttributeDef`.
    PortDef(Node<PortDef>),
    PortUsage(Node<PortUsage>),
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
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: AttributeBody,
    pub membership: Membership,
}

/// Metadata usage: `metadata` name (`:` type)? (`about` targets)? body (BNF MetadataUsage).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetadataUsage {
    pub name: String,
    pub type_name: Option<String>,
    pub about_targets: Vec<String>,
    pub body: AttributeBody,
    pub membership: Membership,
}

// ---------------------------------------------------------------------------
// Enumeration (Phase 2)
// ---------------------------------------------------------------------------

/// Enumeration definition: `enum def` Identification EnumerationBody (BNF EnumerationDefinition).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: EnumerationBody,
    pub membership: Membership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EnumerationBody {
    Semicolon,
    Brace { values: Vec<Node<EnumeratedValue>> },
}

/// One enumerated value inside an `enum def { ... }` body: optional `enum` keyword + name, with
/// an optional inline body or `= expr` initializer that the parser discards (BNF
/// EnumeratedValue). Only the name and its span are retained.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumeratedValue {
    pub name: String,
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
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: DefinitionBody,
    pub membership: Membership,
}

/// Occurrence usage: `occurrence` name (`:` type)? body, with optional individual/portion modifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OccurrenceUsage {
    pub is_individual: bool,
    pub is_then: bool,
    /// True for `event occurrence <name>;` (BNF `EventOccurrenceUsage`, §6 G7) — an occurrence
    /// that marks a point in time rather than owning a lifetime.
    pub is_event: bool,
    /// Leading `ref` keyword (BNF `RefPrefix`, §6 G29), as in `ref individual :>> vehicleUnderTest
    /// : TestVehicle1 { ... }` from the OMG spec Annex `9-Verification-simplified.sysml`.
    pub is_reference: bool,
    pub portion_kind: Option<String>,
    pub name: String,
    pub type_name: Option<String>,
    pub subsets: Option<Node<SubsettingRelationship>>,
    pub redefines: Option<Node<SubsettingRelationship>>,
    pub references: Option<Node<SubsettingRelationship>>,
    pub crosses: Option<Node<SubsettingRelationship>>,
    pub intersects: Option<Node<SubsettingRelationship>>,
    pub body: OccurrenceUsageBody,
    pub membership: Membership,
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
    /// Optional name after `constraint`, e.g. `engineSelectionRational` in
    /// `assert constraint engineSelectionRational { ... }`. `None` for the anonymous form
    /// (`assert constraint { ... }`).
    pub name: Option<String>,
    /// Optional type after `:`, e.g. `DiscBrakeFitConstraint_Alt` in `assert constraint
    /// discBrakeFitConstraint_Alt : DiscBrakeFitConstraint_Alt { ... }` (§6 G22 — the named form
    /// was closed in G3 but the typed one still fell through to opaque recovery).
    pub type_name: Option<String>,
    pub body: ConstraintDefBody,
    /// `true` for a negated assert: `assert not constraint ...`.
    pub is_negated: bool,
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
    SuccessionUsage(Node<SuccessionUsage>),
    /// `satisfy <ref> (by <expr>)?;` inside an occurrence definition body (previously only
    /// reachable at package level).
    Satisfy(Node<Satisfy>),
    /// `allocate <source> to <target>;` nested inside an allocation usage body (§6 G17), which
    /// decomposes the outer allocation. Real usage: OMG spec Annex `12b-Allocation.sysml`.
    Allocate(Node<Allocate>),
    /// `exhibit (state)? <name> ...` inside an occurrence body (§6 G30). An individual/snapshot
    /// exhibits states just as a part usage does -- real usage: `exhibit vehicleStates.on { ... }`
    /// in the OMG spec Annex `6-Individual and Snapshots.sysml`.
    StateUsage(Node<StateUsage>),
}

/// Standalone succession usage directly in a definition/occurrence body (distinct from the
/// action-body `first ... then ...` control node and from `succession flow X to Y;`):
/// `succession` multiplicity? (`first` multiplicity? source)? `then` multiplicity? target
/// `;` or `{` ... `}`. E.g. `succession [seBeforeNum] first [0..1] sourceEvent then [0..1] self;`
/// (SysML Systems Library `Flows.sysml`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SuccessionUsage {
    /// Multiplicity of the succession feature itself, e.g. `[seBeforeNum]`.
    pub multiplicity: Option<Node<Multiplicity>>,
    pub source: Node<Expression>,
    /// Multiplicity on the `first` end, e.g. `[0..1]`.
    pub source_multiplicity: Option<Node<Multiplicity>>,
    pub target: Node<Expression>,
    /// Multiplicity on the `then` end, e.g. `[0..1]`.
    pub target_multiplicity: Option<Node<Multiplicity>>,
    pub body: ConnectBody,
    pub membership: Membership,
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
    /// `interface` (name (multiplicity)?)? (`:` Type)? body -- a declared interface usage with
    /// no inline `connect` clause (GH-16). Per BNF `InterfaceUsageDeclaration = UsageDeclaration
    /// ValuePart? ('connect' InterfacePart)? | InterfacePart`, the `connect` clause is optional:
    /// ends may be declared inside the body instead, or omitted entirely (e.g. as a placeholder
    /// to be redefined later). Previously this shape fell through to opaque recovery because
    /// `interface_usage` unconditionally required either a `connect` clause or a bare `from to
    /// to` form.
    Declaration {
        name: Option<String>,
        interface_type: Option<String>,
        body: ConnectBody,
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
    Doc(Node<DocComment>),
}

/// Connect at part usage level: `connect` from `to` to body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Connect {
    pub from: Node<ConnectionEnd>,
    pub to: Node<ConnectionEnd>,
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
    /// The aliased element's qualified name, e.g. `ISQ::mass` in `alias m for ISQ::mass;`.
    ///
    /// Structured the same way as [`crate::ast::TypingRelationship::target`]/
    /// [`crate::ast::SubsettingRelationship::target`] (parser work item 2) rather than a plain
    /// joined `String`, so `::`-qualified segments stay distinguishable and the target carries
    /// its own span. Unlike those fields this is a single [`crate::ast::RelationshipTarget`], not
    /// a `Vec` -- an alias target (`[QualifiedName]` per the grammar) is always exactly one
    /// qualified name, with no comma-separated multi-target concept (parser work item 4a).
    pub target: RelationshipTarget,
    pub body: AliasBody,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006 continuation), `kind`
    /// always [`crate::ast::MembershipKind::Alias`] -- the variant reserved for this struct since
    /// `Membership`'s introduction, previously unconstructed. Genuine new grammar coverage: BNF
    /// `AliasMember : Membership = MemberPrefix 'alias' ...` (`SysML-textual-bnf.kebnf`) legally
    /// permits a `private`/`protected`/`public` prefix before `alias`, but `alias_def` never parsed
    /// one at all before this increment -- same gap class found repeatedly in this rollout (see
    /// `crate::ast::PortDef::membership`).
    pub membership: Membership,
}

/// Body of an alias definition: `;` or `{` ... `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AliasBody {
    Semicolon,
    Brace,
}
