use super::behavior::{
    ActionDef, ActionDefBodyElement, ActionUsage, ActionUsageBodyElement, Allocate, InOut,
    InOutDecl, StateDefBody, StateDefBodyElement, StateUsage,
};
use super::common::{
    CommentAnnotation, ConnectBody, DocComment, Identification, ParseErrorNode,
    TextualRepresentation,
};
use super::feature_value::FeatureValue;
use super::membership::Membership;
use super::requirement::{Dependency, EnumerationUsage, ItemUsage, RequirementUsage, Satisfy};
use super::view::ReturnDecl;
use super::view::{CalcUsage, ConstraintDef, ConstraintDefBody, ConstraintUsage};
use crate::ast::core::{
    ConnectionEnd, Expression, Multiplicity, Node, Span, SubsettingRelationship, TypingRelationship,
};
use crate::ast::QualifiedReferenceId;

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

/// `ExtendedDefinition` (SysML §8.2.2.27): `DefinitionExtensionKeyword+ 'def' DefinitionDeclaration
/// DefinitionBody` -- one or more bare `#<name>` metadata-keyword tags standing *in place of* the
/// usual classifier keyword (`part`/`attribute`/`action`/...) that would otherwise introduce a
/// `Definition`, e.g. `#situation def Failure;`, `#SecurityRelated #situation def Vulnerability;`,
/// `abstract #situation def AbstractFailure;`. Reuses [`PackageBody`] for the body so any ordinary
/// package/definition member (`part p;`, nested definitions, ...) parses inside it exactly as at
/// package scope.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedDefinition {
    /// One-or-more `#<name>` prefix keyword tags (BNF `DefinitionExtensionKeyword+`), in source
    /// order. Reuses the exact `#name` tag representation `metadata_keyword_prefix` already
    /// builds for a bare `#name` reference.
    pub prefix_keywords: Vec<Node<MetadataKeywordUsage>>,
    /// Optional `abstract` or `variation` prefix (BNF BasicDefinitionPrefix), which may precede
    /// the `#`-prefix keywords (`abstract #situation def AbstractFailure;`).
    pub definition_prefix: Option<DefinitionPrefix>,
    pub identification: Identification,
    /// Supertype after `:>`.
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: super::package::PackageBody,
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
    UnsupportedMember(Node<crate::ast::UnsupportedGrammarNode>),
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
    /// `Parts::performedActions` and similar).
    ActionUsage(Box<Node<ActionUsage>>),
    /// `action def` nested inside a part definition body (GH-14: previously only `ActionUsage`
    /// was reachable here, so a nested definition fell through to opaque recovery with a
    /// misleading "expected ';' or '{' after action definition header" diagnostic).
    ActionDef(Node<ActionDef>),
    /// `state` / `ref state` usage inside a part definition body.
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
    /// Bare `bind a = b;` (BNF `BindingConnectorAsUsage`, §8.2.2.13.2) nested directly inside a
    /// part definition body (GH-42 Gap 1: previously only reachable inside a part *usage* body,
    /// even though `ConnectionTest.sysml` uses it directly inside a `part def`, same dispatch-gap
    /// class as `FirstStmt`/GH-40 above).
    Bind(Node<Bind>),
    /// `alias <name> for <target>;` nested inside a part definition body (GH-89), e.g. `part def
    /// P1 { port porig1; alias po1 for porig1; }` (Simple Tests/AliasTest.sysml:7). Previously
    /// only reachable at package-body scope.
    AliasDef(Node<AliasDef>),
}

/// Connection usage member inside part definitions.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConnectionUsageMember {
    pub name: Option<String>,
    pub type_reference: Option<QualifiedReferenceId>,
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
    /// `true` when the member was declared as `ref connection ...` (a reference connection
    /// usage) rather than a plain `connection ...` usage. See `unsupported_part_member`'s former
    /// `ReferenceConnectionUsage` short-circuit in `src/parser/part/body.rs`, now folded into
    /// `connection_usage_member_inner`.
    pub by_reference: bool,
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
    /// Declaration label in the explicit `exhibit state name` form.
    pub name: String,
    /// Referenced state path in the shorthand `exhibit path` form.
    pub state_reference: Option<QualifiedReferenceId>,
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
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeDef {
    pub name: String,
    /// Short name from `< ... >` when present (e.g. unit symbol `m`, `EUR`).
    pub short_name: Option<String>,
    /// Type after `:>`, represented by a relationship whose target IDs resolve through the
    /// enclosing [`crate::ast::ParsedDocument`] (PAR-004 item 1). `typing_span` duplicates the
    /// node's own `span` for existing consumers that read the span without the node.
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

impl PartialEq for AttributeDef {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.short_name == other.short_name
            && self.typing == other.typing
            && self.value == other.value
            && self.body == other.body
            && self.ordered == other.ordered
            && self.nonunique == other.nonunique
            && self.membership == other.membership
    }
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
    /// `assert constraint …` in attribute / item bodies (#72 / validation `15_01`, `15_08`).
    AssertConstraint(Node<AssertConstraintMember>),
    /// `ref` / `ref part` / `ref :>> …` members in attribute / item bodies (validation `15_11`,
    /// `15_19`, `17a`, `17b`).
    RefDecl(Node<RefDecl>),
    /// Nested `part` usage inside an item / attribute body (validation `3e`, `14c`).
    PartUsage(Box<Node<PartUsage>>),
    /// Nested `item` usage inside an item / attribute body, e.g. `item picture : Picture;`
    /// inside `attribute def Show { ... }` (`tests/snapshots/sysml/training/
    /// 21_messaging_with_ports.md`). Reuses the same `item_usage` parser `PartDefBodyElement`/
    /// `PartUsageBodyElement` already dispatch to.
    ItemUsage(Box<Node<ItemUsage>>),
    Other(String),
}

/// Item definition: `item def` Identification body (for events, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemDef {
    /// `individual item def John :> Person { ... }` (GH-90.1, `Individuals Examples/
    /// JohnIndividualExample.sysml:19`).
    pub is_individual: bool,
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

/// KerML `class` classifier definition: `class` Identification (`:>` | `specializes`) type? body,
/// e.g. `class B :> A { }`. Mirrors `IndividualDef` (same `def`-optional, `:>`-specialized,
/// `AttributeBody`-bodied shape) -- previously only reachable through the opaque
/// `classifier_decl` KerML fallback alongside `classifier`/`struct`/`structure`/`subclassifier`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassDef {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: AttributeBody,
    pub membership: Membership,
}

/// Part usage: `part` name `:` type multiplicity? `ordered`? (`redefines`|`:>>`)? value? body.
#[derive(Debug, Clone, Eq)]
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
    /// Structured typing clause mirroring `AttributeUsage.typing`: every comma-separated target
    /// from `:`/`defined by`/`typed by`, not just the first (S42-004).
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

impl PartialEq for PartUsage {
    fn eq(&self, other: &Self) -> bool {
        self.usage_prefix == other.usage_prefix
            && self.is_individual == other.is_individual
            && self.is_reference == other.is_reference
            && self.direction == other.direction
            && self.is_derived == other.is_derived
            && self.is_constant == other.is_constant
            && self.name == other.name
            && self.short_name == other.short_name
            && self.typing == other.typing
            && self.multiplicity == other.multiplicity
            && self.ordered == other.ordered
            && self.subsets == other.subsets
            && self.redefines == other.redefines
            && self.value == other.value
            && self.body == other.body
            && self.membership == other.membership
    }
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
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetadataAnnotation {
    pub reference: QualifiedReferenceId,
    pub type_reference: Option<QualifiedReferenceId>,
    pub about_targets: Vec<QualifiedReferenceId>,
    pub body: AttributeBody,
    pub head_span: Option<Span>,
    pub type_span: Option<Span>,
}

impl PartialEq for MetadataAnnotation {
    fn eq(&self, other: &Self) -> bool {
        self.reference == other.reference
            && self.type_reference == other.type_reference
            && self.about_targets == other.about_targets
            && self.body == other.body
    }
}

/// User-defined metadata keyword usage: `#keyword` (`:` Type)? (`about` targets)? body.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetadataKeywordUsage {
    pub keyword: String,
    pub type_reference: Option<QualifiedReferenceId>,
    pub about_targets: Vec<QualifiedReferenceId>,
    pub body: AttributeBody,
    pub keyword_span: Span,
    pub type_span: Option<Span>,
}

impl PartialEq for MetadataKeywordUsage {
    fn eq(&self, other: &Self) -> bool {
        self.keyword == other.keyword
            && self.type_reference == other.type_reference
            && self.about_targets == other.about_targets
            && self.body == other.body
    }
}

/// Generic annotation or metadata usage captured in body scopes.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Annotation {
    pub sigil: String,
    pub head: AnnotationHead,
    pub type_reference: Option<QualifiedReferenceId>,
    pub body: ConnectBody,
    pub head_span: Option<Span>,
    pub type_span: Option<Span>,
}

impl PartialEq for Annotation {
    fn eq(&self, other: &Self) -> bool {
        self.sigil == other.sigil
            && self.head == other.head
            && self.type_reference == other.type_reference
            && self.body == other.body
    }
}

/// The semantic target of an `@` annotation, or opaque text for an extension-style `#` form.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AnnotationHead {
    Reference(QualifiedReferenceId),
    Opaque(String),
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
    /// `succession` (name)? (`: Type`)? multiplicity? `first` ... `then` ...`;` (GH-92.3, BNF
    /// `SuccessionAsUsage`), e.g. `succession : HappensJustBefore first vehicle1_t0 then
    /// vehicle1_t0_t1;` (`Vehicle Example/VehicleIndividuals.sysml:49`). Already modeled/parsed
    /// for `ConnectionDefBodyElement`/`OccurrenceBodyElement`; just not dispatched here.
    SuccessionUsage(Node<SuccessionUsage>),
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
    /// `calc <name>[: Type] { ... }` usage inside a part usage body (GH-91.2), e.g. `calc 'Solve
    /// for Pressure1' : 'Ideal Gas Law';` (`Analysis Examples/Turbojet Stage
    /// Analysis.sysml:88`). `calc_usage` itself already fully supports quoted names -- only
    /// `calc_def_required` (`CalcDef` above) was dispatched here.
    CalcUsage(Node<crate::ast::view::CalcUsage>),
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
    /// `alias <name> for <target>;` nested inside a part usage body (GH-89), e.g. `part p2 : P1 {
    /// port pdest; alias pd1 for pdest; }` (Simple Tests/AliasTest.sysml:16). Previously only
    /// reachable at package-body scope.
    AliasDef(Node<AliasDef>),
    /// `include <usecase>;` member inside a part usage body (GH-89), e.g. `part system : System {
    /// include uc2; }` (Simple Tests/UseCaseTest.sysml:33). Previously only reachable inside a
    /// use case definition body.
    IncludeUseCase(Node<crate::ast::requirement::IncludeUseCase>),
    /// `use case <name> : Type { ... }` usage nested inside a part usage body (found alongside
    /// `IncludeUseCase` above, same real fixture -- already dispatched in `PartDefBodyElement`,
    /// just not here).
    UseCaseUsage(Node<crate::ast::requirement::UseCaseUsage>),
    /// `verification <name> : Type { ... }` usage nested inside a plain part usage body (GH-89),
    /// e.g. `part verificationContext { verification verificationPlan : VerificationPlan { ... }
    /// }` (Simple Tests/VerificationTest.sysml:35). Already dispatched in `PartDefBodyElement`,
    /// just not here.
    VerificationCaseUsage(Node<crate::ast::requirement::VerificationCaseUsage>),
}

/// Variant member inside a variation part usage/def body: either an untyped reference to a
/// separately-declared usage (`variant name;`), or a typed usage declared inline with a kind
/// keyword (`variant part name : Type { ... }`, `variant attribute name = expr;`, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariantUsage {
    /// Referenced usage in the untyped `variant path;` form. Typed variants keep their declared
    /// name solely on the nested usage instead of duplicating it here.
    pub reference: Option<QualifiedReferenceId>,
    /// Present when declared with a kind keyword (`variant part ...;`); `None` for the untyped
    /// reference form (`variant name;` / `variant name { ... }`).
    pub typed: Option<VariantTypedUsage>,
    /// Optional nested body on the untyped reference form, e.g. `variant q { attribute b : B
    /// :>> a; }` (`Simple Tests/VariabilityTest.sysml:16`) or a quoted name `variant '6cylEngine'
    /// { ... }` (`Variability Examples/VehicleVariabilityModel.sysml:78`). Always `None` when
    /// `typed` is `Some` (the nested typed usage owns its own body).
    pub body: Option<PartUsageBody>,
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
    /// Declaration label in the explicit `perform action name ...` form.
    pub action_name: String,
    /// Referenced action path in the shorthand `perform path` form.
    pub action_reference: Option<QualifiedReferenceId>,
    /// Structured type after `:` in `perform action name : Type`.
    pub typing: Option<Node<TypingRelationship>>,
    /// Multiplicity after the name (GH-89), e.g. `[*]` in `perform action takePicture[*] :>
    /// PictureTaking::takePicture;` (Camera Example/Camera.sysml:4).
    pub multiplicity: Option<Node<Multiplicity>>,
    /// Redefinition target after `:>>`, e.g. `doXorY` in `perform action :>> doXorY = doX;`.
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// Subsetting target after `:>` (GH-89), e.g. `PictureTaking::takePicture` in `perform action
    /// takePicture[*] :> PictureTaking::takePicture;` (Camera Example/Camera.sysml:4). Mutually
    /// exclusive with `redefines` (whichever specialization keyword is present).
    pub subsets: Option<Node<SubsettingRelationship>>,
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

/// In/out binding inside a perform body: `in` target `=` expr `;` or `out` target `=` expr `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PerformInOutBinding {
    pub direction: InOut,
    pub target: QualifiedReferenceId,
    pub value: Node<Expression>,
}

/// Attribute usage: `attribute` name (`:>` | `:` type)? `redefines`? value? body.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeUsage {
    pub name: String,
    /// Short name from `< ... >` when present, e.g. `attribute <wcf> wheelCoordinateFrame : ...`
    /// (confirmed real usage in the OMG Geometry domain library's
    /// `VehicleGeometryAndCoordinateFrames.sysml`). See `AttributeDef::short_name`.
    pub short_name: Option<String>,
    /// Type after `:` or `:>`, represented by a relationship whose target IDs resolve through the
    /// enclosing [`crate::ast::ParsedDocument`] (PAR-004 item 1). `typing_span` duplicates the
    /// node's own `span`.
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
    /// `abstract`/`variation` keyword from `RefPrefix` (GH-88.3), e.g. `abstract attribute
    /// minMass :> ISQ::mass;` (`Mass Roll-up Example/MassRollup.sysml:21`). Named `usage_prefix`
    /// to match `PartUsage::usage_prefix`'s identical `DefinitionPrefix` reuse.
    pub usage_prefix: Option<DefinitionPrefix>,
    /// `constant` keyword from `RefPrefix` -- usage-only, same rationale as `is_derived`.
    pub is_constant: bool,
    /// `ref` keyword from `BasicUsagePrefix` (GH-88.2), e.g. `derived constant ref attribute y
    /// :> x;` (`Simple Tests/PartTest.sysml:9`).
    pub is_reference: bool,
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

impl PartialEq for AttributeUsage {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.short_name == other.short_name
            && self.typing == other.typing
            && self.subsets == other.subsets
            && self.redefines == other.redefines
            && self.references == other.references
            && self.crosses == other.crosses
            && self.intersects == other.intersects
            && self.value == other.value
            && self.body == other.body
            && self.direction == other.direction
            && self.multiplicity == other.multiplicity
            && self.ordered == other.ordered
            && self.nonunique == other.nonunique
            && self.is_derived == other.is_derived
            && self.is_constant == other.is_constant
            && self.is_end == other.is_end
            && self.membership == other.membership
    }
}

/// SysML `DefaultReferenceUsage` (BNF §8.2.2.6 / Spec §7.6.4): a usage without a kind keyword,
/// e.g. `Capacity : Real;`. Distinct from [`AttributeUsage`], which requires the `attribute`
/// keyword. Historically parsed via `attribute_usage_shorthand` into `AttributeUsage`.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefaultReferenceUsage {
    pub name: String,
    /// Type after `:` / `defined by` / `typed by`.
    pub typing: Option<Node<TypingRelationship>>,
    /// Optional `:>` subsetting clause (GH-87), e.g. `torquePerCurrent :>
    /// Quantities::scalarQuantities = ISQ::torque / ISQ::electricCurrent;` (State Space
    /// Representation Examples/EVSample.sysml:47).
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// Optional `:>>` redefinition clause (GH-87), same shorthand position as `subsets`.
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// Optional feature value after `=` / `default`.
    pub value: Option<Node<FeatureValue>>,
    pub name_span: Option<Span>,
    pub typing_span: Option<Span>,
    pub membership: Membership,
    /// `true` for the KerML bare `feature x;` / `feature x : Type;` form (explicit `feature`
    /// keyword, `feature_usage_member` in `package.rs`), `false` for the keyword-less
    /// `name;`/`name = expr;` form this struct is otherwise documented for. Tracked so
    /// `emit_default_reference_usage` can round-trip the keyword rather than always omitting it.
    pub has_feature_keyword: bool,
    /// Optional `{ ... }` body, e.g. KerML `feature f { expr s { in x; return : Boolean; } }`
    /// (spec42 `tests/snapshots/spec42/kerml/expressions.md`). `None` means the usage was
    /// terminated with `;` instead (the only form previously supported). Only reachable for the
    /// explicit-`feature`-keyword form (`has_feature_keyword == true`); the keyword-less
    /// `name;`/`name = expr;` forms never populate this.
    pub body: Option<Vec<Node<FeatureBodyElement>>>,
}

impl PartialEq for DefaultReferenceUsage {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.typing == other.typing
            && self.subsets == other.subsets
            && self.redefines == other.redefines
            && self.value == other.value
            && self.membership == other.membership
            && self.has_feature_keyword == other.has_feature_keyword
            && self.body == other.body
    }
}

/// A member nested inside a `feature NAME { ... }` block body (KerML `Feature`'s
/// `FeatureBodyElement` production alternatives). Narrowly scoped to the shape actually observed
/// in the pinned KerML fixtures -- a nested owned `expr` feature -- rather than the full KerML
/// expression sublanguage; see `DefaultReferenceUsage::body`'s doc comment.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeatureBodyElement {
    /// A nested owned expression feature, e.g. `expr s { in x; return : Boolean; }`.
    Expr(Node<ExprMember>),
}

/// Nested `expr NAME { ... }` member inside a [`FeatureBodyElement::Expr`]. Its own body reuses
/// the same `in`/`out`/`return` parameter-list machinery already shared by `calc`/`constraint`
/// bodies (`crate::parser::action::in_out_decl`, `crate::parser::constraint::return_decl`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExprMember {
    pub name: String,
    pub name_span: Option<Span>,
    pub body: Vec<Node<ExprMemberElement>>,
}

/// A single member of an [`ExprMember`]'s `{ ... }` body: a parameter (`in`/`out`/`inout`) or a
/// `return` declaration -- the only shapes the pinned fixture (`in x; return : Boolean;`) needs.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExprMemberElement {
    InOutDecl(Box<Node<InOutDecl>>),
    ReturnDecl(Node<ReturnDecl>),
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
    /// `#keyword` metadata tag nested inside a port definition body, either the bare form or the
    /// `PrefixMetadataMember`-style form prefixing the next port-body member (e.g. `#idd port
    /// APIS_HTTP { ... }`) -- previously port definition bodies had no `#`/`@` annotation support
    /// at all, unlike part/item/action/etc. bodies. See `PackageBodyElement::MetadataKeywordUsage`.
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    Other(String),
}

/// Port usage: `port` name `:` type multiplicity? `:>` subsets? `redefines`? body.
#[derive(Debug, Clone, Eq)]
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
    /// `individual` keyword (BNF `OccurrenceUsagePrefix`, GH-90.1), e.g. `individual port po1;`
    /// (gap #7). Mirrors `ItemUsage::is_individual`/`ActionUsage::is_individual`.
    pub is_individual: bool,
    pub name: String,
    /// Short name from `< ... >` when present. See `AttributeUsage::short_name`.
    pub short_name: Option<String>,
    /// Structured, multi-target typing clause after `:` / `typed by` / `defined by`.
    pub typing: Option<Node<TypingRelationship>>,
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

impl PartialEq for PortUsage {
    fn eq(&self, other: &Self) -> bool {
        self.direction == other.direction
            && self.is_abstract == other.is_abstract
            && self.is_derived == other.is_derived
            && self.is_constant == other.is_constant
            && self.is_individual == other.is_individual
            && self.name == other.name
            && self.short_name == other.short_name
            && self.typing == other.typing
            && self.multiplicity == other.multiplicity
            && self.subsets == other.subsets
            && self.redefines == other.redefines
            && self.references == other.references
            && self.crosses == other.crosses
            && self.intersects == other.intersects
            && self.value == other.value
            && self.body == other.body
            && self.membership == other.membership
    }
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
    /// Real annotation content from a braced body (`body: ConnectBody::Brace`), tracked
    /// separately from `body` since `ConnectBody` itself is shared as a bare semicolon/brace
    /// marker across several very differently-shaped contexts (bind, TypedConnect, this plain
    /// connect statement). Empty for `ConnectBody::Semicolon`.
    pub body_elements: Vec<Node<RelationshipBodyElement>>,
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
    /// GH-51: `interface_def_body` previously had no recovery-error representation at all --
    /// unparseable content was silently discarded by a hand-rolled `advance_to_closing_brace`
    /// fallback with no diagnostic. See [`ConnectionDefBodyElement::Error`] for the sibling this
    /// mirrors.
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
    /// GH-85: bare `flow <a> to <b>;` shorthand connecting two of this interface's own ends, e.g.
    /// `flow p1.torque to p2.torque;` (OMG spec Annex `Vehicle Example/SysML v2 Spec Annex A
    /// SimpleVehicleModel.sysml`). Previously unmodeled -- this body had no flow arm at all.
    FlowUsage(Node<crate::ast::behavior::FlowUsage>),
}

/// GH-53: the nested-usage kinds confirmed by real usage as an [`EndDecl`]'s target (see
/// `EndDecl::nested_usage`'s doc comment). Only `occurrence`/`item` are evidenced; extend this
/// only alongside a matching real-usage citation, not speculatively.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EndNestedUsage {
    Occurrence(Box<Node<OccurrenceUsage>>),
    Item(Box<Node<ItemUsage>>),
}

/// End declaration in interface/connection def: `end` name (`:` type | (`::>` | `references`)
/// target | nested `occurrence`/`item` usage, see [`nested_usage`](EndDecl::nested_usage)) `;`.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndDecl {
    /// A normal declared name or a fixed derivation-end role. `#original`/`#derive` are grammar
    /// roles, not declaration labels.
    pub identity: EndIdentity,
    /// Structured typing for the `: Type` form. A reference-only end has no typing and stores its
    /// target in `references`.
    pub typing: Option<Node<TypingRelationship>>,
    /// Structured reference-subsetting relationship for the `::>`/`references` form (GH-19):
    /// `end name ::> target;` / `end name references target;` names a reference, not a type, so
    /// it must not be modeled as typing (`endType`) downstream. Also populated when `::>`
    /// *trails* an explicit `: Type` instead of replacing it, e.g. `end port p3: P ::> p.p1;`
    /// (GH-85, `Simple Tests/
    /// ConjugationTest.sysml`). `None` when no reference-subsetting clause was written at all.
    pub references: Option<Node<SubsettingRelationship>>,
    /// Optional multiplicity after the type/reference target, e.g. `[1]` in `end hub ::>
    /// mainSwitch[1];` (BNF `DefaultInterfaceEnd`'s `Usage` production carries the same optional
    /// multiplicity every other usage declaration does). `None` when absent.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// GH-51: `:>>` redefines clause trailing the `: Type` typed form (distinct from
    /// `references`, which models the `::>`/`references` form used *instead* of `:` typing), e.g.
    /// `end source: Anything :>> BinaryLinkObject::source;` (Systems Library `Connections.sysml`).
    /// `None` when absent or when this end used the `::>`/`references` form instead.
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// GH-85: `crosses` cross-subsetting clause trailing the `: Type` typed form, e.g. `end item
    /// cart: ShoppingCart[1] crosses selectedProduct.inCart;` (OMG spec Annex `Association
    /// Examples/ProductSelection_UnownedEnds.sysml`). `None` when absent.
    pub crosses: Option<Node<SubsettingRelationship>>,
    /// GH-53: an alternative end-declaration form where the target is itself a complete, nested
    /// kind-prefixed usage rather than a bare type/reference, e.g. `end theCauses [*] occurrence
    /// theCause :> causes :>> source { ... }` (Systems Library `Domain Libraries/Cause and
    /// Effect/CausationConnections.sysml`) / `end touchesToo [0..*] item touchedItemToo :>>
    /// separateSpaceToo, thisOccurrence;` (`Items.sysml`). `theCauses`/`touchesToo` is still this
    /// `EndDecl`'s declaration identity; the nested usage supplies this end's typing/structure instead of a
    /// `:`/`::>` clause. `None` for the ordinary forms above.
    pub nested_usage: Option<Box<EndNestedUsage>>,
    /// Span of the type/reference target after `:`/`::>`/`references` (for semantic tokens).
    pub type_ref_span: Option<Span>,
}

/// Identity position of a connector end.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EndIdentity {
    /// Ordinary declaration label with its authored token span.
    Declaration(Node<String>),
    /// Fixed derivation role with its authored `#...` token span.
    Derivation(Node<DerivationEndRole>),
}

/// Fixed roles inside a `#derivation connection` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DerivationEndRole {
    /// The `#original` end.
    Original,
    /// The `#derive` end.
    Derive,
}

impl PartialEq for EndDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity
            && self.typing == other.typing
            && self.references == other.references
            && self.multiplicity == other.multiplicity
            && self.redefines == other.redefines
            && self.crosses == other.crosses
            && self.nested_usage == other.nested_usage
    }
}

/// Ref declaration in interface def: `ref` name `:` type body.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RefDecl {
    /// Leading `in`/`out`/`inout` direction (GH-88.4), e.g. `private in ref y: A, B;` inside a
    /// `part def` body (`Simple Tests/ItemTest.sysml:15`). Only parsed by `part_ref_usage`;
    /// `connector::ref_decl`'s call sites have no confirmed real usage for a leading direction.
    pub direction: Option<InOut>,
    pub name: String,
    /// Structured typing clause mirroring `PartUsage.typing`/`AttributeUsage.typing`: every
    /// comma-separated target from a `:` clause, not just the first (S42-004).
    pub typing: Option<Node<TypingRelationship>>,
    /// Redefines target from an optional leading `:>>` clause, e.g. `ref sentMessage :>>
    /// sentTransfer: MessageTransfer, MessageAction { ... }` (Systems Library `Actions.sysml`).
    /// Previously this whole `:>> target : type` combination silently discarded the redefines
    /// target and the entire typing clause as unparsed text once `:>>` was seen.
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// GH-51: `:>` subsets clause, distinct from and independent of `redefines`, e.g. `ref
    /// requirement originalRequirement[1] :>> originalRequirements :> participant { ... }`
    /// (Systems Library `Domain Libraries/Requirement Derivation/DerivationConnections.sysml`).
    pub subsets: Option<Node<SubsettingRelationship>>,
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

impl PartialEq for RefDecl {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.typing == other.typing
            && self.redefines == other.redefines
            && self.subsets == other.subsets
            && self.value == other.value
            && self.body == other.body
            && self.membership == other.membership
    }
}

/// Body of a ref declaration: `;` or `{` members `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RefBody {
    Semicolon,
    Brace { elements: Vec<Node<RefBodyElement>> },
}

/// Element of a ref declaration's braced body (`RefBody::Brace`), wrapping whichever member
/// shape is real for the owning context. BNF `ReferenceUsage` resolves `ref`'s body to a generic
/// `Usage` body, so its real content follows whatever the owning context allows: full nested
/// action members inside an action body, part-usage members inside a part usage body, state
/// members inside a state body. Connection/interface `ref` bodies don't yet have a dedicated
/// member grammar, so they get the same doc/comment/metadata + recovery baseline as
/// [`RelationshipBodyElement`].
///
/// `PartUsageBodyElement`/`ActionDefBodyElement` are inherently larger than the annotation-only
/// variants (same size-difference tradeoff already accepted for `AttributeUsage` in several
/// other body-element enums crate-wide); not boxing keeps this variant shape consistent with
/// those siblings rather than partially addressing the size difference.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RefBodyElement {
    Action(Node<ActionDefBodyElement>),
    PartUsage(Node<PartUsageBodyElement>),
    State(Node<StateDefBodyElement>),
    Doc(Node<DocComment>),
    Comment(Node<CommentAnnotation>),
    TextualRep(Node<TextualRepresentation>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    Error(Node<ParseErrorNode>),
    /// Unmodeled body content captured as raw text (used for library parsing).
    Other(String),
}

/// Shared annotation-only body element for KerML `RelationshipBody` contexts -- BNF
/// `RelationshipBody : Relationship = ';' | '{' (ownedRelationship += OwnedAnnotation)* '}'`,
/// used by `AliasMember`/`Import`/`Dependency` -- and other leaf bodies where a full
/// nested-member grammar isn't (yet) modeled (plain `connect` statement bodies): doc/comment/
/// metadata annotations are retained; anything else recovers to `Error`/`Other` instead of being
/// silently discarded.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RelationshipBodyElement {
    Doc(Node<DocComment>),
    Comment(Node<CommentAnnotation>),
    TextualRep(Node<TextualRepresentation>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    Error(Node<ParseErrorNode>),
    /// Unmodeled body content captured as raw text (used for library parsing).
    Other(String),
}

// ---------------------------------------------------------------------------
// Connection (Phase 2)
// ---------------------------------------------------------------------------

/// Connection definition: `connection def` Identification body (BNF ConnectionDefinition).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DerivationConnectionRole {
    /// The fixed `#derivation` grammar marker.
    Derivation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConnectionDef {
    /// `individual connection def ...` (BNF `OccurrenceUsagePrefix`/definition-prefix
    /// `isIndividual`, GH-90.1), mirroring `ActionDef::is_individual`.
    pub is_individual: bool,
    /// Fixed derivation role and exact marker span. Ordinary connections have no role.
    pub derivation_role: Option<Node<DerivationConnectionRole>>,
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
    /// GH-51: real Systems/Domain Library connection defs own `assert constraint`
    /// (`Cause and Effect/CausationConnections.sysml`, `Requirement Derivation/
    /// DerivationConnections.sysml`) and `occurrence` usages with `abstract`/`constant`/`ref`
    /// prefixes (`CausationConnections.sysml`) as body members -- neither was dispatched here at
    /// all. (`ref requirement ...` in `DerivationConnections.sysml` goes through `RefDecl` above,
    /// not a separate requirement-usage path -- `requirement` is just another `ref_decl` kind
    /// keyword there, same as `part`/`port`/`item`.)
    AssertConstraint(Node<AssertConstraintMember>),
    OccurrenceUsage(Box<Node<OccurrenceUsage>>),
    /// `succession` usage (real usage: `CausationConnections.sysml`'s `private succession
    /// causalOrdering first [nCauses] causes.startShot then [nEffects] effects { ... }`).
    SuccessionUsage(Node<SuccessionUsage>),
    /// Bare `part <name>;` member inside a connection def/usage body (GH-89), e.g. `abstract
    /// connection def C { part p; end end1; ... }` (Simple Tests/ConnectionTest.sysml:31).
    PartUsage(Box<Node<PartUsage>>),
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
    pub type_reference: Option<QualifiedReferenceId>,
    pub about_targets: Vec<QualifiedReferenceId>,
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
    /// `individual occurrence def IO2 { ... }` (GH-90.1, `Simple Tests/IndividualTest.sysml:3`).
    pub is_individual: bool,
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: DefinitionBody,
    pub membership: Membership,
}

/// Occurrence usage: `occurrence` name (`:` type)? body, with optional individual/portion modifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OccurrenceUsage {
    /// Leading `in`/`out`/`inout` direction (BNF `RefPrefix`), e.g. `in occurrence
    /// terminatedOccurrence[1] { ... }` inside an action definition body (Systems Library
    /// `Actions.sysml`'s `TerminateAction`). Mirrors [`ItemUsage`]'s `direction`.
    pub direction: Option<InOut>,
    pub is_individual: bool,
    pub is_then: bool,
    /// True for `event occurrence <name>;` (BNF `EventOccurrenceUsage`, §6 G7) — an occurrence
    /// that marks a point in time rather than owning a lifetime.
    pub is_event: bool,
    /// Leading `ref` keyword (BNF `RefPrefix`, §6 G29), as in `ref individual :>> vehicleUnderTest
    /// : TestVehicle1 { ... }` from the OMG spec Annex `9-Verification-simplified.sysml`.
    pub is_reference: bool,
    /// Leading `abstract` keyword (BNF `RefPrefix`, §8.2.2.9.2). GH-51: real usage in Systems
    /// Library `Domain Libraries/Cause and Effect/CausationConnections.sysml`.
    pub is_abstract: bool,
    /// Leading `constant` keyword (BNF `RefPrefix`). See `is_abstract`.
    pub is_constant: bool,
    /// True when the literal `occurrence` kind keyword was authored (BNF
    /// `OccurrenceUsagePrefix`/`OccurrenceUsageKeyword`), distinct from `is_individual` --
    /// `individual occurrence o1;` and bare `individual o1;` both set `is_individual`, but only
    /// the former also sets this (gap #7). Needed so emission doesn't fabricate or drop the
    /// keyword relative to what was authored.
    pub has_occurrence_keyword: bool,
    pub portion_kind: Option<OccurrencePortionKind>,
    /// Declaration label for ordinary occurrence usages.
    pub name: String,
    /// Existing occurrence referenced by the shorthand `event path` form.
    pub occurrence_reference: Option<QualifiedReferenceId>,
    pub type_name: Option<QualifiedReferenceId>,
    pub type_is_conjugated: bool,
    /// GH-51: `occurrence_usage` previously had no multiplicity support at all, so real usage
    /// like `abstract constant ref occurrence causes[1..*] :>> causes :> participant { ... }`
    /// (Systems Library `Domain Libraries/Cause and Effect/CausationConnections.sysml`) fell
    /// through to recovery even without the `abstract`/`constant`/`ref` prefixes.
    pub multiplicity: Option<Node<Multiplicity>>,
    pub subsets: Option<Node<SubsettingRelationship>>,
    pub redefines: Option<Node<SubsettingRelationship>>,
    pub references: Option<Node<SubsettingRelationship>>,
    pub crosses: Option<Node<SubsettingRelationship>>,
    pub intersects: Option<Node<SubsettingRelationship>>,
    /// Optional value clause (BNF `ValuePart`), e.g. `in occurrence terminatedOccurrence
    /// default that as Occurrence { ... }` (Systems Library `Actions.sysml`).
    pub value: Option<Node<FeatureValue>>,
    pub body: OccurrenceUsageBody,
    pub membership: Membership,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OccurrencePortionKind {
    Snapshot,
    Timeslice,
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
    /// Optional declared name after `constraint`, e.g. `engineSelectionRational` in
    /// `assert constraint engineSelectionRational { ... }`. `None` for the anonymous form
    /// (`assert constraint { ... }`).
    pub declaration_name: Option<String>,
    /// Referenced constraint in the shorthand `assert path { ... }` form.
    pub target: Option<QualifiedReferenceId>,
    /// Optional type after `:`, e.g. `DiscBrakeFitConstraint_Alt` in `assert constraint
    /// discBrakeFitConstraint_Alt : DiscBrakeFitConstraint_Alt { ... }` (§6 G22 — the named form
    /// was closed in G3 but the typed one still fell through to opaque recovery).
    pub type_name: Option<QualifiedReferenceId>,
    pub body: ConstraintDefBody,
    /// `true` for a negated assert: `assert not constraint ...`.
    pub is_negated: bool,
    /// GH-51: `assert_constraint_member` previously never parsed a visibility prefix at all, so
    /// real usage like `private assert constraint disjointCauseEffect { ... }` (Systems Library
    /// `Domain Libraries/Cause and Effect/CausationConnections.sysml`) fell through to recovery.
    pub membership: Membership,
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
    /// `item x;` inside an occurrence definition/usage body (GH-87), e.g. `occurrence def Occ {
    /// item x; }` (Simple Tests/OccurrenceTest.sysml:6). `item_usage` itself already fully
    /// supports the bare (untyped, no value) form -- it just wasn't dispatched here.
    ItemUsage(Node<ItemUsage>),
    OccurrenceUsage(Box<Node<OccurrenceUsage>>),
    SuccessionUsage(Node<SuccessionUsage>),
    /// `satisfy <ref> (by <expr>)?;` inside an occurrence definition body (previously only
    /// reachable at package level).
    Satisfy(Node<Satisfy>),
    /// `allocate <source> to <target>;` nested inside an allocation usage body (§6 G17), which
    /// decomposes the outer allocation. Real usage: OMG spec Annex `12b-Allocation.sysml`.
    Allocate(Node<Allocate>),
    /// `end name : Type;` (or `::>` / nested forms) inside allocation / connection-like
    /// definition bodies. Real usage: OMG Annex `12b-Allocation-1.sysml` (`end logical : …`).
    EndDecl(Node<EndDecl>),
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
    /// Name of the succession usage itself, e.g. `causalOrdering` in `succession causalOrdering
    /// first a then b;` (BNF `SuccessionAsUsage`'s optional `'succession' UsageDeclaration`
    /// prefix, mirrored by `action::succession_prefix` for the `first`-embedded form). GH-51:
    /// real usage in Systems Library `Domain Libraries/Cause and Effect/
    /// CausationConnections.sysml`.
    pub name: Option<String>,
    /// Type of the succession usage itself (BNF `UsageDeclaration`'s `FeatureSpecializationPart`),
    /// e.g. `HappensJustBefore` in the unnamed `succession : HappensJustBefore first a then b;`
    /// (GH-92.3, `Vehicle Example/VehicleIndividuals.sysml:49`). Mirrors
    /// `FirstStmt::succession_type`'s identical field for the sibling action-body `first`-embedded
    /// form.
    pub type_name: Option<QualifiedReferenceId>,
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
    /// Name of the binding connector itself, e.g. `binding ab bind a = b;` (BNF
    /// `BindingConnectorAsUsage`'s optional `'binding' UsageDeclaration` prefix, §8.2.2.13.2).
    /// `None` for the bare `bind a = b;` form (no `binding` keyword) or an unnamed `binding`
    /// prefix (e.g. `binding [1] bind ...`).
    pub binding_name: Option<String>,
    /// Type of the binding connector itself, e.g. `binding ab1 : AB bind a = b;`.
    pub binding_type: Option<QualifiedReferenceId>,
    /// Multiplicity of the binding connector feature itself, e.g. `binding [1] bind ...`
    /// (Systems Library `Domain Libraries/Geometry/ShapeItems.sysml`).
    pub binding_multiplicity: Option<Node<Multiplicity>>,
    pub left: Node<Expression>,
    /// Multiplicity on the left (`bind`) end, e.g. `bind [0..*] base.edges = ...` -- same
    /// per-endpoint-multiplicity shape as `Connect`'s `ConnectionEnd`/§6 G24 and `FirstStmt`'s
    /// `first_multiplicity`/`then_multiplicity`.
    pub left_multiplicity: Option<Node<Multiplicity>>,
    pub right: Node<Expression>,
    /// Multiplicity on the right (`=`) end, e.g. `... = [0..*] be;`.
    pub right_multiplicity: Option<Node<Multiplicity>>,
    /// Optional body after the bind (semicolon or brace); 3a fixture uses `bind x = y { }`.
    pub body: Option<ConnectBody>,
    /// Real content from a braced body (`body: Some(ConnectBody::Brace)`), the same part-usage
    /// member set `PartUsageBody` uses (BNF `BindingConnectorAsUsage`'s body is `UsageBody`, the
    /// same general usage-member production). Empty otherwise.
    pub body_elements: Vec<Node<PartUsageBodyElement>>,
}

/// Interface usage: typed+connect or connection form.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InterfaceUsage {
    /// `interface` name? `:Type`? `connect` from `to` to body; optional body with ref redefs.
    /// GH-85: `name` was added for the named-but-untyped form (`interface userToFlashlight
    /// connect a to b { ... }`, OMG spec Annex `Flashlight Example/Flashlight Example.sysml`) --
    /// previously only the typed (`interface name: Type connect ...`) and fully anonymous
    /// (`Connection` variant) forms were reachable.
    TypedConnect {
        name: Option<String>,
        interface_type: Option<QualifiedReferenceId>,
        subsets: Option<Node<SubsettingRelationship>>,
        redefines: Option<Node<SubsettingRelationship>>,
        from: Node<Expression>,
        to: Node<Expression>,
        body: ConnectBody,
        body_elements: Vec<Node<InterfaceUsageBodyElement>>,
    },
    /// `interface` from `to` to body.
    Connection {
        subsets: Option<Node<SubsettingRelationship>>,
        redefines: Option<Node<SubsettingRelationship>>,
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
        interface_type: Option<QualifiedReferenceId>,
        subsets: Option<Node<SubsettingRelationship>>,
        redefines: Option<Node<SubsettingRelationship>>,
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
        target: QualifiedReferenceId,
        value: Node<Expression>,
        body: RefBody,
    },
    Doc(Node<DocComment>),
    /// GH-85: `end` member inside a typed, non-`connect` interface usage's body, e.g. `interface
    /// i: I { end port p3: P ::> p.p1; end port p4: ~P ::> p.p2; }` (`Simple Tests/
    /// ConjugationTest.sysml`), parallel to the already-supported `connection a: A { end port
    /// p3: ...; }` form. Boxed: `EndDecl` is much larger than `RefRedef`, the other variant here.
    EndDecl(Box<Node<EndDecl>>),
}

/// Connect at part usage level: `connect` from `to` to body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Connect {
    pub from: Node<ConnectionEnd>,
    pub to: Node<ConnectionEnd>,
    pub body: ConnectBody,
    pub subsets: Option<Node<SubsettingRelationship>>,
    pub redefines: Option<Node<SubsettingRelationship>>,
}

/// Package-level `BindingConnectorAsUsage` (BNF §8.2.2.13.2), the keyword-less sibling of
/// [`Bind`]: `binding` (`all`)? name? multiplicity? (`of` | `bind`)? left `=` right body.
/// Distinct from `Bind`/`bind_` (used inside part-def/part-usage bodies), which requires the
/// literal `bind` keyword between the optional `binding` prefix and the `left = right` pair --
/// here that keyword (or the alternative `of`) is itself optional decoration around the same
/// `left = right` pair, confirmed by real usage: `binding instant[instantNum] of startShot =
/// endShot;`, `binding all startShot = endShot;`, `binding x bind a = b;`, `binding [0..1] a =
/// b;` all bind the same `left = right` shape regardless of which (if any) keyword separates the
/// prefix from it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BindingConnectorUsage {
    /// `true` for the `binding all ...` form.
    pub all: bool,
    /// Span of the binding connector's own name, e.g. `instant` in `binding
    /// instant[instantNum] of startShot = endShot;`. `None` when no name is given (e.g. `binding
    /// all ...`, `binding [0..1] ...`). The name text lives in the document source and is
    /// resolved through it rather than copied into this node.
    pub name_span: Option<Span>,
    /// Multiplicity on the binding connector itself, e.g. `[instantNum]` / `[0..1]`.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `true` when the `of` keyword introduced `left` (e.g. `of startShot`); `false` when `left`
    /// was introduced by `bind` or appeared bare. Retained for exact re-emission.
    pub uses_of_keyword: bool,
    /// `true` when the `bind` keyword introduced `left` (e.g. `binding x bind a = b;`). Retained
    /// for exact re-emission.
    pub uses_bind_keyword: bool,
    pub left: QualifiedReferenceId,
    pub right: QualifiedReferenceId,
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
    /// This is one document-local arena identity rather than a `Vec`: an alias target
    /// (`[QualifiedName]` per the grammar) is always exactly one qualified name, with no
    /// comma-separated multi-target concept.
    pub target: crate::ast::QualifiedReferenceId,
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
    Brace {
        elements: Vec<Node<RelationshipBodyElement>>,
    },
}
