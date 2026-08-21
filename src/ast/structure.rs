use super::behavior::{
    ActionDef, ActionUsage, ActionUsageBodyElement, Allocate, InOut, InOutDecl, StateDefBody,
    StateUsage,
};
use super::body::Body;
use super::common::{AnnotatingMember, Identification, ParseErrorNode};
use super::feature_value::FeatureValue;
use super::membership::Membership;
use super::multiplicity_part::MultiplicityModifiers;
use super::requirement::{
    Dependency, EnumerationUsage, ItemUsage, RequirementUsage, SatisfyRequirementUsage,
};
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
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
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
/// `abstract #situation def AbstractFailure;`. Reuses [`crate::ast::PackageBody`] for the body so any ordinary
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
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
    /// Whether the `def` keyword was authored: `true` for the `ExtendedDefinition` form
    /// (`#situation def Failure;`), `false` for the bare extended-usage shorthand
    /// (`#clouddd ArrowheadCore { ... }`, spec42 Gap 39).
    pub has_def_keyword: bool,
    pub identification: Identification,
    /// Supertype after `:>`.
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: super::package::PackageBody,
}

/// BNF BasicDefinitionPrefix: `abstract` | `variation`.
///
/// The same two-alternative group `RefPrefix` spells for a usage; see
/// [`crate::ast::RefPrefix::variance`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DefinitionPrefix {
    Abstract,
    Variation,
}

/// Body of a part definition: `;` or `{` PartDefBodyElement* `}`.
pub type PartDefBody = Body<PartDefBodyElement>;

/// Element inside a part definition body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PartDefBodyElement {
    Error(Node<ParseErrorNode>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    /// Nested `package` definition. `PartDefinition` owns a `DefinitionBody`, whose
    /// `DefinitionMember` admits `DefinitionElement`, including `Package` (SysML textual BNF
    /// 180-207, 234-248; the pinned Pilot SysML grammar agrees).
    Package(Node<crate::ast::Package>),
    /// Nested `library package` / `standard library package` definition. This remains distinct
    /// from [`Self::Package`], matching the grammar's separate `LibraryPackage` alternative and
    /// retaining its standard-library spelling without a mirrored discriminator flag.
    LibraryPackage(Node<crate::ast::LibraryPackage>),
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    /// A dependency owned by this definition (BNF `DefinitionMember`).
    Dependency(Node<Dependency>),
    AttributeDef(Node<AttributeDef>),
    AttributeUsage(Node<AttributeUsage>),
    /// Bare `name : Type;` / `name = expr;` without a kind keyword (SysML DefaultReferenceUsage).
    DefaultReferenceUsage(Node<DefaultReferenceUsage>),
    RequirementUsage(Node<RequirementUsage>),
    ItemDef(Node<ItemDef>),
    ItemUsage(Node<ItemUsage>),
    Ref(Node<RefDecl>),
    PortUsage(Box<Node<PortUsage>>),
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
    /// `SatisfyRequirementUsage` inside a part definition body (previously only reachable
    /// at package level).
    Satisfy(Box<Node<SatisfyRequirementUsage>>),
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
    /// `port def` nested inside a part definition body, using `port_def` (PAR-002:
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
    /// Nested KerML classifier declaration (`struct`, `classifier`, `datatype`, `assoc`,
    /// `behavior`, ...) inside a part definition body (spec42 Gap 38); see
    /// [`crate::ast::KermlClassifierDecl`].
    KermlClassifier(Box<Node<crate::ast::KermlClassifierDecl>>),
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
    /// `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation ?= 'variation'`
    /// (SysML BNF 219; Pilot `SysML.xtext` 490) -- one slot, two alternatives, carrying the
    /// authored keyword's exact span. `AttributeDefinition` (SysML BNF 510) reaches it through
    /// `DefinitionPrefix` (SysML BNF 225).
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
    pub name: String,
    /// Short name from `< ... >` when present (e.g. unit symbol `m`, `EUR`).
    pub short_name: Option<String>,
    /// Type after `:>`, represented by a relationship whose target IDs resolve through the
    /// enclosing [`crate::ast::ParsedDocument`] (PAR-004 item 1). `typing_span` duplicates the
    /// node's own `span` for existing consumers that read the span without the node.
    pub typing: Option<Node<TypingRelationship>>,
    pub multiplicity: Option<Node<Multiplicity>>,
    /// Default or binding after `=` / `:=` / `default =` before the body terminator.
    pub value: Option<Node<FeatureValue>>,
    pub body: AttributeBody,
    /// Span of the defined name (for semantic tokens).
    pub name_span: Option<Span>,
    /// Span of the type after `:>`, if present (for semantic tokens).
    pub typing_span: Option<Span>,
    /// Span of the default/binding expression value, when present.
    pub value_span: Option<Span>,
    /// `MultiplicityPart`'s `isOrdered`/`isUnique` keyword slots, each carrying the authored
    /// spelling and its exact span. See [`MultiplicityModifiers`](crate::ast::MultiplicityModifiers).
    pub multiplicity_modifiers: crate::ast::MultiplicityModifiers,
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
            && self.multiplicity == other.multiplicity
            && self.value == other.value
            && self.body == other.body
            && self.multiplicity_modifiers == other.multiplicity_modifiers
            && self.membership == other.membership
    }
}

/// Body of an attribute (def or usage): `;` or `{` AttributeBodyElement* `}`.
pub type AttributeBody = Body<AttributeBodyElement>;

// See `RequirementDefBodyElement`'s `#[allow(clippy::large_enum_variant)]` doc comment --
// `AttributeUsage`'s size relative to `Doc`/`Error` is an accepted, crate-wide tradeoff, not
// specific to this enum.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeBodyElement {
    /// A spec-valid member of this body that the parser does not model yet, retained with
    /// its authored span and a diagnostic.
    Unsupported(Node<crate::ast::UnsupportedGrammarNode>),
    Error(Node<ParseErrorNode>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    AttributeDef(Node<AttributeDef>),
    AttributeUsage(Node<AttributeUsage>),
    /// Keyword-less `DefaultReferenceUsage` (`RefPrefix Usage`) nested in an attribute/item
    /// body. It is distinct from `AttributeUsage`, whose `attribute` keyword is required.
    DefaultReferenceUsage(Node<DefaultReferenceUsage>),
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
    /// KerML feature member (`feature x : Natural[1];`, `member feature ...`, `composite
    /// feature ...`, `portion feature all ...`, `var x : Integer;`, `step s : S;`, `expr e
    /// { ... }`) nested in a KerML `class`/`struct`/`datatype` body, which shares this body
    /// grammar via `class_def`; see [`crate::ast::KermlFeature`].
    KermlFeature(Box<Node<crate::ast::KermlFeature>>),
    /// KerML invariant member (`inv checkIt { ... }`) nested in a KerML type body; see
    /// [`crate::ast::KermlInvariantMember`].
    Invariant(Box<Node<crate::ast::KermlInvariantMember>>),
    /// KerML connector member (`connector a ::> a.x to b;`) nested in a KerML type body; see
    /// [`crate::ast::KermlConnectorMember`].
    KermlConnector(Box<Node<crate::ast::KermlConnectorMember>>),
    /// Nested KerML classifier declaration for the rest of the keyword family (`struct`,
    /// `classifier`, `datatype`, `assoc`, `behavior`, ...; spec42 Gap 38); see
    /// [`crate::ast::KermlClassifierDecl`].
    KermlClassifier(Box<Node<crate::ast::KermlClassifierDecl>>),
    /// Named/multiplicity-qualified binding member nested in an attribute/item body
    /// (`binding [1] bind [0..*] base.edges = [0..*] be;`, Geometry `ShapeItems.sysml`;
    /// spec42 Gap 49a).
    Bind(Box<Node<Bind>>),
    /// Named/typed connection usage nested in an attribute/item body (`connection : MatesWith
    /// connect [1] tfe to [1] tfe;`, Geometry `ShapeItems.sysml`; spec42 Gap 49a).
    Connection(Box<Node<ConnectionUsageMember>>),
    /// Nested `calc def` (spec42 Gap 49a).
    CalcDef(Box<Node<crate::ast::view::CalcDef>>),
    /// Nested `calc` usage (`private calc getElapsedUtcTime { ... }` inside `attribute def
    /// Clock`, Quantities and Units `Time.sysml`; spec42 Gap 49a).
    CalcUsage(Box<Node<crate::ast::view::CalcUsage>>),
    /// Plain (non-`assert`) constraint usage nested in an attribute/item body (`abstract
    /// constraint checkedConstraints : ConstraintCheck[0..*] :> ... { ... }`, Systems Library
    /// `Items.sysml`; spec42 Gap 49a).
    ConstraintUsage(Box<Node<ConstraintUsage>>),
    /// `variant` member via `DefinitionBodyItem → VariantUsageMember` (SysML textual BNF
    /// 237-252; pinned Pilot `SysML.xtext` 518-531). `AttributeBody` is shared by attribute,
    /// item, and type bodies, so this owns the member once at that grammar boundary.
    VariantUsage(Node<VariantUsage>),
}

/// Item definition: `item def` Identification body (for events, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemDef {
    /// `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation ?= 'variation'`
    /// (SysML BNF 219; Pilot `SysML.xtext` 490) -- one slot, two alternatives, carrying the
    /// authored keyword's exact span. `ItemDefinition` (SysML BNF 611) reaches it through
    /// `OccurrenceDefinitionPrefix` (SysML BNF 541).
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
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
    /// `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation ?= 'variation'`
    /// (SysML BNF 219; Pilot `SysML.xtext` 490) -- one slot, two alternatives, carrying the
    /// authored keyword's exact span. `IndividualDefinition` (SysML BNF 551;
    /// Pilot `SysML.xtext` 817) spells it directly, ahead of the mandatory `individual`.
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: AttributeBody,
    pub membership: Membership,
}

/// Part usage: `part` name `:` type multiplicity? `ordered`? (`redefines`|`:>>`)? value? body.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PartUsage {
    /// The complete `OccurrenceUsagePrefix` this usage was written with
    /// (`PartUsage = OccurrenceUsagePrefix 'part' Usage`, SysML BNF 623).
    ///
    /// The same shared component `OccurrenceUsage`, `ItemUsage` and `SatisfyRequirementUsage`
    /// carry; see `planning/part-usage-prefix-matrix.md` and
    /// `planning/occurrence-usage-prefix-matrix.md`. It replaced six independent fields
    /// (`usage_prefix`, `is_individual`, `is_reference`, `direction`, `is_derived`,
    /// `is_constant`) that between them kept no authored span and represented neither a
    /// `PortionKind` nor a `UsageExtensionKeyword`, so `snapshot part vehicle_1_t0 { ... }`
    /// (`training/28. Individuals/Individuals and Roles-1.sysml:14`) reached recovery and
    /// `#logical part vehicleLogical : Vehicle { ... }` (`Vehicle Example/SysML v2 Spec Annex A
    /// SimpleVehicleModel.sysml:487`) became two sibling members.
    pub prefix: crate::ast::OccurrenceUsagePrefix,
    /// `SourceSuccessionMember : FeatureMembership = 'then' ownedRelatedElement += SourceSuccession`
    /// (SysML BNF 597), as the authored keyword's span.
    ///
    /// `DefinitionBodyItem` and `NonBehaviorBodyItem` both spell
    /// `( SourceSuccessionMember )? …UsageMember`, so a `then` precedes the *membership* -- and
    /// therefore the visibility keyword and the whole prefix -- rather than being a prefix slot.
    /// It is a separate field for that reason, not a member of
    /// [`OccurrenceUsagePrefix`](crate::ast::OccurrenceUsagePrefix).
    /// `SourceSuccession` and `SourceEndMember` below it contribute no further tokens, so the
    /// keyword's span is the whole authored fact.
    pub then_span: Option<Span>,
    pub name: String,
    /// Short name from `< ... >` when present. See `AttributeUsage::short_name`.
    pub short_name: Option<String>,
    /// Structured typing clause mirroring `AttributeUsage.typing`: every comma-separated target
    /// from `:`/`defined by`/`typed by`, not just the first (S42-004).
    pub typing: Option<Node<TypingRelationship>>,
    /// Multiplicity, e.g. `[2]` parsed into structured lower/upper bounds.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `MultiplicityPart`'s `isOrdered`/`isUnique` keyword slots, each carrying the authored
    /// spelling and its exact span. See [`MultiplicityModifiers`](crate::ast::MultiplicityModifiers).
    pub multiplicity_modifiers: crate::ast::MultiplicityModifiers,
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
        self.prefix == other.prefix
            && self.then_span == other.then_span
            && self.name == other.name
            && self.short_name == other.short_name
            && self.typing == other.typing
            && self.multiplicity == other.multiplicity
            && self.multiplicity_modifiers == other.multiplicity_modifiers
            && self.subsets == other.subsets
            && self.redefines == other.redefines
            && self.value == other.value
            && self.body == other.body
            && self.membership == other.membership
    }
}

/// Body of a part usage: `;` or `{` PartUsageBodyElement* `}`.
pub type PartUsageBody = Body<PartUsageBodyElement>;

/// The `@` spelling of a metadata feature -- the `MetadataFeature` alternative of
/// `AnnotatingElement` (KerML 8.2.5.12 `MetadataFeature`, SysML 8.2.2.27 `MetadataUsage`).
///
/// ```text
/// MetadataFeature = ( PrefixMetadataMember )* ( '@' | 'metadata' )
///                   MetadataFeatureDeclaration
///                   ( 'about' Annotation ( ',' Annotation )* )?
///                   MetadataBody
/// MetadataFeatureDeclaration = ( Identification ( ':' | 'typed' 'by' ) )? OwnedFeatureTyping
/// ```
///
/// # Why the *type* is the reference, and the head is not
///
/// The declaration ends at a required `OwnedFeatureTyping`, so the last qualified name is always
/// the annotated metadata type. The `Identification` in front of `:` / `typed by` is a
/// **declared name**: `@Tag;` names nothing and is typed by `Tag`, while `@t : Tag;` declares
/// `t` and is typed by `Tag`. The superseded shape stored whatever followed `@` as a
/// `QualifiedReferenceId` and demoted the type to an `Option`, so `@t : Tag` allocated an arena
/// reference for the declaration label `t` -- a reference synthesized from a declaration.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetadataAnnotation {
    /// Prefix metadata applied to this metadata feature, in authored order.
    pub prefixes: Vec<Node<MetadataKeywordUsage>>,
    /// Authored `MetadataFeature` introducer. KerML and SysML both allow `@` or `metadata`.
    pub introducer: MetadataFeatureIntroducer,
    /// `MetadataFeatureDeclaration`'s optional `Identification ( ':' | 'typed' 'by' )` prefix,
    /// present exactly when a separator keyword was authored.
    pub declared_name: Option<Node<MetadataDeclaredName>>,
    /// `OwnedFeatureTyping` -- the annotated metadata type. Required by the production.
    pub type_reference: QualifiedReferenceId,
    /// Exact span of the `OwnedFeatureTyping` qualified name.
    pub type_span: Span,
    /// `'about' Annotation ( ',' Annotation )*`, where `Annotation = [QualifiedName]`.
    pub about_targets: Vec<QualifiedReferenceId>,
    /// `MetadataBody`.
    pub body: AttributeBody,
}

impl PartialEq for MetadataAnnotation {
    fn eq(&self, other: &Self) -> bool {
        self.prefixes == other.prefixes
            && self.introducer == other.introducer
            && self.declared_name == other.declared_name
            && self.type_reference == other.type_reference
            && self.about_targets == other.about_targets
            && self.body == other.body
    }
}

/// The mutually exclusive introducers of `MetadataFeature`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MetadataFeatureIntroducer {
    At { span: Span },
    Metadata { span: Span },
}

/// `MetadataFeatureDeclaration`'s `Identification ( ':' | 'typed' 'by' )` prefix.
///
/// The two parts are one field because the grammar makes them inseparable: the separator is only
/// written when an identification precedes it, and an identification is only reachable through
/// the separator.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetadataDeclaredName {
    /// The declared name. A declaration label, never a reference.
    pub identification: Identification,
    /// Which separator keyword the author wrote.
    pub typed_by: MetadataTypedBy,
    /// Exact `:` or `typed by` token span.
    pub typed_by_span: Span,
}

/// The authored spelling of `MetadataFeatureDeclaration`'s `( ':' | 'typed' 'by' )` separator.
///
/// Both spell the same relationship, so emission reproduces what was written rather than
/// canonicalizing one into the other. Only these two are legal here, which is why this is its
/// own enum rather than a reuse of the wider [`crate::ast::TypingSpelling`]: `specializes` and
/// `defined by` are not reachable from this production and must not be representable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MetadataTypedBy {
    /// `:`
    Colon,
    /// `typed by`
    TypedBy,
}

/// The `#` spelling of a metadata reference.
///
/// ```text
/// PrefixMetadataMember     : OwningMembership = '#' ownedRelatedElement += PrefixMetadataFeature
/// PrefixMetadataAnnotation : Annotation       = '#' ownedRelatedElement += PrefixMetadataFeature
/// PrefixMetadataFeature    : MetadataFeature  = ownedRelationship += OwnedFeatureTyping
/// ```
///
/// `OwnedFeatureTyping` is `[QualifiedName]`, so what follows `#` is a **reference to a metadata
/// type** -- possibly qualified (`#ISQ::mass`), possibly quoted -- and never a fabricated name.
/// The `#` is syntax and is kept as a span rather than folded into the name.
///
/// Unlike the `@` spelling, `PrefixMetadataFeature` admits no `Identification`, no
/// `:` / `typed by` clause and no `about` clause: those belong to `MetadataFeature`, which `#`
/// does not reach. Fields for them were removed rather than left permanently empty --
/// `#Tag about X;` is not a production in either layer.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MetadataKeywordUsage {
    /// The `#` sigil. Syntax and provenance, never part of the reference.
    pub hash_span: Span,
    /// `OwnedFeatureTyping` -- the metadata type this tag refers to.
    pub reference: QualifiedReferenceId,
    /// Which of the two `#` productions this is.
    ///
    /// `None` is `PrefixMetadataMember` / `PrefixMetadataAnnotation`: the tag prefixes the
    /// declaration that follows it (`#safety part def X;`) and owns no body. `Some` is
    /// `ExtendedUsage` with an empty `UsageDeclaration` (`#safety;`, `#safety { ... }`), a
    /// member of the enclosing body that writes its own `UsageBody`.
    pub body: Option<AttributeBody>,
}

impl PartialEq for MetadataKeywordUsage {
    fn eq(&self, other: &Self) -> bool {
        self.reference == other.reference && self.body == other.body
    }
}

/// Element inside a part usage body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PartUsageBodyElement {
    Error(Node<ParseErrorNode>),
    /// `end name;` / `end ref name;` inside a usage body, e.g. the two ends of the transfer
    /// constraint in `ref :>> outgoingTransfersFromSelf :> ... { end ref source; end ref
    /// target; }` (Systems Library `Ports.sysml:37`). Occurrence bodies already modelled this
    /// member; part usage bodies did not.
    EndDecl(Node<EndDecl>),
    /// Directed parameter declaration with no kind keyword, e.g. `in :>> MessageTransfer::payload,
    /// MessageAction::payload;` inside a `ref` body (Systems Library `Actions.sysml`). Port and
    /// action bodies already modelled this member; part usage bodies did not, so the same line
    /// was an unexpected keyword here.
    InOutDecl(Node<InOutDecl>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    AttributeUsage(Node<AttributeUsage>),
    /// Bare `name : Type;` without a kind keyword (SysML DefaultReferenceUsage).
    DefaultReferenceUsage(Node<DefaultReferenceUsage>),
    EnumerationUsage(Node<EnumerationUsage>),
    PartUsage(Box<Node<PartUsage>>),
    OccurrenceUsage(Box<Node<OccurrenceUsage>>),
    PortUsage(Box<Node<PortUsage>>),
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
    /// `SatisfyRequirementUsage` inside a part usage body.
    Satisfy(Box<Node<SatisfyRequirementUsage>>),
    StateUsage(Node<StateUsage>),
    /// `action` / `ref action` usage inside a part usage body.
    ActionUsage(Box<Node<ActionUsage>>),
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
    /// `port def` nested inside a part usage body, using `port_def`. See `StateDef`.
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
    /// The view family. `RenderingUsage`, `ViewUsage` and `ViewpointUsage` are
    /// `StructureUsageElement`/`BehaviorUsageElement` alternatives and the three definitions are
    /// `DefinitionElement` alternatives, so `UsageBody = DefinitionBody` admits all six here
    /// exactly as `PartDefBodyElement` already did. This scope had none of them.
    ViewDef(Node<crate::ast::view::ViewDef>),
    ViewUsage(Node<crate::ast::view::ViewUsage>),
    ViewpointDef(Node<crate::ast::view::ViewpointDef>),
    ViewpointUsage(Node<crate::ast::view::ViewpointUsage>),
    RenderingDef(Node<crate::ast::view::RenderingDef>),
    RenderingUsage(Node<crate::ast::view::RenderingUsage>),
    /// Nested KerML classifier declaration (`struct Car1_ { ... }` inside a `part` usage body,
    /// KerML `time_varying_car_driver`; spec42 Gap 38); see
    /// [`crate::ast::KermlClassifierDecl`].
    KermlClassifier(Box<Node<crate::ast::KermlClassifierDecl>>),
}

/// Variant member inside a variation part usage/def body: either an untyped reference to a
/// separately-declared usage (`variant name;`), or a typed usage declared inline with a kind
/// keyword (`variant part name : Type { ... }`, `variant attribute name = expr;`, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariantUsage {
    /// Exactly one `VariantUsageElement` alternative. This discriminant prevents invalid states
    /// such as an untyped reference paired with an inline typed usage.
    pub form: VariantUsageForm,
    /// Ownership/visibility/kind wrapper (parser work item 4b final sweep), `kind` always
    /// [`crate::ast::MembershipKind::VariantMembership`] -- confirmed against
    /// `SysML-textual-bnf.kebnf`'s `VariantUsageMember : VariantMembership = MemberPrefix
    /// 'variant' ownedVariantUsage = VariantUsageElement`, which legally carries a visibility
    /// prefix before this increment added support for parsing one.
    pub membership: Membership,
}

/// The two non-overlapping shapes of `VariantUsageElement` retained by this parser.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VariantUsageForm {
    /// `variant path;` or `variant path { ... }`. The optional body belongs only to this
    /// reference form, e.g. `variant q { attribute b : B :>> a; }`.
    Reference {
        reference: QualifiedReferenceId,
        body: Option<PartUsageBody>,
    },
    /// An inline declared usage (`variant part ...`, `variant action ...`, etc.).
    Typed(VariantTypedUsage),
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
    /// `variant action a;` through `VariantUsageElement → BehaviorUsageElement → ActionUsage`
    /// (SysML textual BNF 374-390 and 392-413; pinned Pilot `SysML.xtext` 679-719).
    Action(Box<Node<ActionUsage>>),
    /// `variant perform doX;` inside a `variation perform action ... { ... }` body (§6 G5).
    Perform(Box<Node<Perform>>),
    /// `variant requirement r1;` inside a `variation requirement r { ... }` body (spec42
    /// Gap 44).
    Requirement(Box<Node<crate::ast::RequirementUsage>>),
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
pub type PerformBody = Body<PerformBodyElement>;

/// Element inside a perform body: doc comment, in/out binding, or variant member.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PerformBodyElement {
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
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
    /// `MultiplicityPart`'s `isOrdered`/`isUnique` keyword slots, each carrying the authored
    /// spelling and its exact span. See [`MultiplicityModifiers`](crate::ast::MultiplicityModifiers).
    pub multiplicity_modifiers: crate::ast::MultiplicityModifiers,
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
            && self.multiplicity_modifiers == other.multiplicity_modifiers
            && self.is_derived == other.is_derived
            && self.is_constant == other.is_constant
            && self.is_end == other.is_end
            && self.is_reference == other.is_reference
            && self.usage_prefix == other.usage_prefix
            && self.membership == other.membership
    }
}

/// SysML `DefaultReferenceUsage` (BNF §8.2.2.6.3): `RefPrefix Usage`, a usage without a kind
/// keyword such as `Capacity : Real;`. Distinct from [`AttributeUsage`], which requires the
/// `attribute` keyword. The Pilot-only `end` extension is intentionally not representable.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefaultReferenceUsage {
    /// The source-backed pinned `RefPrefix` (direction, derived, variance, constant).
    pub prefix: crate::ast::RefPrefix,
    pub name: String,
    /// Optional declaration short name from `Identification`.
    pub short_name: Option<String>,
    /// Type after `:` / `defined by` / `typed by`.
    pub typing: Option<Node<TypingRelationship>>,
    /// Optional `:>` subsetting clause (GH-87), e.g. `torquePerCurrent :>
    /// Quantities::scalarQuantities = ISQ::torque / ISQ::electricCurrent;` (State Space
    /// Representation Examples/EVSample.sysml:47).
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// Optional `:>>` redefinition clause (GH-87), same shorthand position as `subsets`.
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// Optional `::>` / `references` relationship.
    pub references: Option<Node<SubsettingRelationship>>,
    /// Optional `=>` / `crosses` relationship.
    pub crosses: Option<Node<SubsettingRelationship>>,
    /// Optional `intersects` relationship.
    pub intersects: Option<Node<SubsettingRelationship>>,
    /// Optional feature value after `=` / `default`.
    pub value: Option<Node<FeatureValue>>,
    /// Multiplicity clause, e.g. `private instantNum: Natural[1] = ...;` (Kernel Semantic
    /// Library `Occurrences.kerml`). Previously unparseable on keyword-less bindings.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `MultiplicityPart` ordering/uniqueness slots, retained independently of the range.
    pub multiplicity_modifiers: MultiplicityModifiers,
    pub name_span: Option<Span>,
    pub typing_span: Option<Span>,
    pub membership: Membership,
    /// `UsageBody = DefinitionBody`, represented by this slice's shared attribute/item body.
    pub body: AttributeBody,
}

impl PartialEq for DefaultReferenceUsage {
    fn eq(&self, other: &Self) -> bool {
        self.prefix == other.prefix
            && self.name == other.name
            && self.short_name == other.short_name
            && self.typing == other.typing
            && self.subsets == other.subsets
            && self.redefines == other.redefines
            && self.references == other.references
            && self.crosses == other.crosses
            && self.intersects == other.intersects
            && self.value == other.value
            && self.multiplicity == other.multiplicity
            && self.multiplicity_modifiers == other.multiplicity_modifiers
            && self.membership == other.membership
            && self.body == other.body
    }
}

// ---------------------------------------------------------------------------
// Port
// ---------------------------------------------------------------------------

/// Port definition: `port def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PortDef {
    /// `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation ?= 'variation'`
    /// (SysML BNF 219; Pilot `SysML.xtext` 490) -- one slot, two alternatives, carrying the
    /// authored keyword's exact span. `PortDefinition` (SysML BNF 628) reaches it through
    /// `DefinitionPrefix` (SysML BNF 225).
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
    pub identification: Identification,
    /// Supertype after `:>`, e.g. Some("ClutchPort") for `port def ManualClutchPort :> ClutchPort`.
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: PortDefBody,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006), `kind` always
    /// [`crate::ast::MembershipKind::OwningMembership`]. Like `PartDef`, `port_def`/
    /// `port_def` did not previously accept a visibility prefix at all (BNF
    /// `DefinitionMember : OwningMembership = MemberPrefix ownedRelatedElement +=
    /// DefinitionElement` legally allows one before any definition) -- confirmed as a genuine
    /// parsing gap the same way as `PartDef::membership`. See `port::parse_port_def`.
    pub membership: Membership,
}

/// Body of a port definition: `;` or `{` PortDefBodyElement* `}`.
pub type PortDefBody = Body<PortDefBodyElement>;

/// Element inside a port definition body (in/out declarations or nested port usages).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PortDefBodyElement {
    /// A spec-valid member of this body that the parser does not model yet, retained with
    /// its authored span and a diagnostic.
    Unsupported(Node<crate::ast::UnsupportedGrammarNode>),
    InOutDecl(Node<InOutDecl>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    Error(Node<ParseErrorNode>),
    AttributeDef(Node<AttributeDef>),
    AttributeUsage(Node<AttributeUsage>),
    /// `item def` nested inside a port definition body, using `item_def_required` (PAR-002:
    /// previously only `ItemUsage` was reachable here).
    ItemDef(Node<ItemDef>),
    ItemUsage(Node<ItemUsage>),
    /// Enumeration usage nested inside a port definition body (PAR-002 widening).
    EnumerationUsage(Node<EnumerationUsage>),
    PortUsage(Box<Node<PortUsage>>),
    /// `#keyword` metadata tag nested inside a port definition body, either the bare form or the
    /// `PrefixMetadataMember`-style form prefixing the next port-body member (e.g. `#idd port
    /// APIS_HTTP { ... }`) -- previously port definition bodies had no `#`/`@` annotation support
    /// at all, unlike part/item/action/etc. bodies. See `PackageBodyElement::MetadataKeywordUsage`.
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    /// `ref`-prefixed feature declaration, e.g. `abstract ref port interfacingPorts : Port[0..*]
    /// nonunique :> ports { ... }` and `ref self: Port :>> Object::self;` (Systems Library
    /// `Ports.sysml`). This scope accepted no `ref` member at all, so every one of them was
    /// captured as unsupported grammar.
    RefDecl(Node<RefDecl>),
    /// `variant` member via this scope's `DefinitionBodyItem` grammar.
    VariantUsage(Node<VariantUsage>),
}

/// Port usage: `port` name `:` type multiplicity? `:>` subsets? `redefines`? body.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PortUsage {
    /// `PortUsage = OccurrenceUsagePrefix 'port' Usage` (SysML BNF 645): the whole shared prefix,
    /// in the one order the production allows, each slot carrying the authored keyword's span.
    ///
    /// Replaced five spanless fields -- `direction`, `is_abstract`, `is_derived`, `is_constant`,
    /// `is_individual` -- that between them represented neither `variation`, `ref`, a
    /// `PortionKind` nor a `UsageExtensionKeyword`, and were parsed and emitted in two different
    /// orders, neither of them the grammar's. See `planning/port-usage-prefix-matrix.md`.
    pub prefix: crate::ast::OccurrenceUsagePrefix,
    pub name: String,
    /// Short name from `< ... >` when present. See `AttributeUsage::short_name`.
    pub short_name: Option<String>,
    /// Structured, multi-target typing clause after `:` / `typed by` / `defined by`.
    pub typing: Option<Node<TypingRelationship>>,
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `MultiplicityPart`'s `isOrdered`/`isUnique` keyword slots, each carrying the authored
    /// spelling and its exact span. See [`MultiplicityModifiers`](crate::ast::MultiplicityModifiers).
    pub multiplicity_modifiers: crate::ast::MultiplicityModifiers,
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
        self.prefix == other.prefix
            && self.name == other.name
            && self.short_name == other.short_name
            && self.typing == other.typing
            && self.multiplicity == other.multiplicity
            && self.multiplicity_modifiers == other.multiplicity_modifiers
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
pub type PortBody = Body<PortBodyElement>;

/// Element inside a port usage body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::large_enum_variant)]
pub enum PortBodyElement {
    Error(Node<ParseErrorNode>),
    InOutDecl(Node<InOutDecl>),
    PortUsage(Box<Node<PortUsage>>),
    /// An occurrence usage nested in a port body. In particular, this owns SysML's
    /// `EventOccurrenceUsage` forms such as `event occurrence received;` and
    /// `event sourceEvent;`.
    OccurrenceUsage(Box<Node<OccurrenceUsage>>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    /// Attribute usage nested inside a port usage body (PAR-002 widening; this enum previously
    /// had no attribute/item coverage at all).
    AttributeUsage(Node<AttributeUsage>),
    /// Item usage nested inside a port usage body. See `AttributeUsage`.
    ItemUsage(Node<ItemUsage>),
    /// `ref`-prefixed feature declaration, e.g. `protected ref thisParticipant :>> self;` and
    /// `protected ref otherParticipants : Port[1..*] nonunique :> interfacingPorts default …;`
    /// (Systems Library `Interfaces.sysml:52-54`, inside `ref port :>> participant : Port [2..*]
    /// nonunique ordered { … }`).
    ///
    /// `UsageBody = DefinitionBody`, so a port usage body owns the same `ref` member a port
    /// *definition* body already models as [`PortDefBodyElement::RefDecl`]; this scope carried no
    /// `ref` member at all, which only became reachable when `port_usage` started claiming the
    /// `ref port …` declarations that own these bodies.
    RefDecl(Node<RefDecl>),
    /// `variant` member via `UsageBody = DefinitionBody`.
    VariantUsage(Node<VariantUsage>),
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
    /// `ConnectionUsage = OccurrenceUsagePrefix ( … | 'connect' ConnectorPart ) UsageBody`, and
    /// `UsageBody = DefinitionBody`, so this body owns the whole usage member set -- not only the
    /// annotating subset a `RelationshipBody` allows.
    ///
    /// This was a `ConnectBody` marker (`Semicolon | Brace`, no delimiter spans) paired with a
    /// separate `body_elements` list: one body fact in two fields, which is what
    /// [`crate::ast::Body`] exists to prevent. The shared container carries the `;` or the two
    /// brace spans with it.
    pub body: PartUsageBody,
}

// ---------------------------------------------------------------------------
// Interface
// ---------------------------------------------------------------------------

/// Interface definition: `interface def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterfaceDef {
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
    pub is_individual: bool,
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: InterfaceDefBody,
    pub membership: Membership,
}

/// Body of an interface definition: `;` or `{` InterfaceDefBodyElement* `}`.
pub type InterfaceDefBody = Body<InterfaceDefBodyElement>;

/// Element inside an interface definition body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InterfaceDefBodyElement {
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    /// `#Tag` metadata reference: `PrefixMetadataMember` prefixing the next member, or the
    /// `ExtendedUsage` member spelling `#Tag;` / `#Tag { ... }`. This scope reaches both --
    /// every member may carry a prefix, and `ExtendedUsage` is a `NonOccurrenceUsageElement` --
    /// but modelled neither, so a `#` member was reported unsupported here.
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
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
    /// `port def`, using `port_def`. See `AttributeDef`.
    PortDef(Node<PortDef>),
    PortUsage(Box<Node<PortUsage>>),
    /// GH-85: bare `flow <a> to <b>;` shorthand connecting two of this interface's own ends, e.g.
    /// `flow p1.torque to p2.torque;` (OMG spec Annex `Vehicle Example/SysML v2 Spec Annex A
    /// SimpleVehicleModel.sysml`). Previously unmodeled -- this body had no flow arm at all.
    FlowUsage(Node<crate::ast::behavior::FlowUsage>),
    /// `constraint` usage through `InterfaceBodyItem -> InterfaceOccurrenceUsageMember ->
    /// InterfaceOccurrenceUsageElement -> BehaviorUsageElement -> ConstraintUsage` (SysML BNF
    /// 727-750, 374-389, 1382-1395). The existing source-backed ConstraintUsage owns its
    /// occurrence prefix, declaration, and CalculationBody.
    ConstraintUsage(Box<Node<ConstraintUsage>>),
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

/// The immediate declaration introducer after an `end` prefix.
///
/// `ref` is the required keyword of the pinned `ReferenceUsage` production after its
/// `EndUsagePrefix`; `feature` is the distinct KerML compatibility spelling already accepted by
/// the shared end parser. They are alternatives, not a boolean layered on top of the end.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EndDeclIntroducer {
    /// The existing bare end-declaration form has no intervening introducer.
    Bare,
    /// Source-backed `ref` from `EndUsagePrefix 'ref' Usage`.
    Reference { keyword_span: Span },
    /// Source-backed KerML compatibility `feature` spelling.
    KerMLFeature { keyword_span: Span },
}

/// End declaration in interface/connection def: `end` name (`:` type | (`::>` | `references`)
/// target | nested `occurrence`/`item` usage, see [`nested_usage`](EndDecl::nested_usage)) `;`.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndDecl {
    /// `Bare`, source-backed `ref`, or source-backed KerML `feature` immediately after `end`.
    pub introducer: EndDeclIntroducer,
    pub short_name: Option<String>,
    /// A normal declared name or a fixed derivation-end role. `#original`/`#derive` are grammar
    /// roles, not declaration labels.
    pub identity: EndIdentity,
    /// Structured typing for the `: Type` form. A reference-only end has no typing and stores its
    /// target in `references`.
    pub typing: Option<Node<TypingRelationship>>,
    /// Structured reference-subsetting relationship for the `::>`/`references` form (GH-19):
    /// `end name ::> target;` / `end name references target;` names a reference, not a type, so
    /// it must not be modeled as typing (`endType`) downstream. Also populated when `::>`
    /// *trails* an explicit `: Type` instead of replacing it, e.g. `end p3: P ::> p.p1;`.
    /// `None` when no reference-subsetting clause was written at all.
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
    /// `crosses` cross-subsetting clause trailing the `: Type` typed form. `None` when absent.
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
        self.introducer == other.introducer
            && self.short_name == other.short_name
            && self.identity == other.identity
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
    /// `derived` from BNF `RefPrefix`, e.g. `derived ref item receiverArgument : Expression[0..1]
    /// subsets Metadata::metadataItems;` (`sysml.library/Systems Library/SysML.sysml:14`).
    pub is_derived: bool,
    /// `abstract` or `variation` from `RefPrefix` -- alternatives there, so one field, matching
    /// [`AttributeUsage::usage_prefix`]. E.g. `abstract ref port outgoingTransfersFromSelf : ...`
    /// (`sysml.library/Systems Library/Ports.sysml`).
    pub usage_prefix: Option<DefinitionPrefix>,
    /// `constant` from `RefPrefix`. See [`AttributeUsage::is_constant`].
    pub is_constant: bool,
    /// Kind keyword after `ref` (`ref item scene : Scene;`, `ref port :>> participant ...`).
    /// Previously parsed by `connector::ref_decl` and discarded, so formatting dropped the
    /// authored keyword.
    pub kind_keyword: Option<RefDeclKind>,
    pub name: String,
    pub short_name: Option<String>,
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
    /// Multiplicity clause (BNF `MultiplicityPart`), from either the pre-specialization
    /// position (`ref originalRequirement[1] :>> ...`) or the post-typing position (`ref
    /// otherParticipants : Port [1..*] nonunique :> ...`, Systems Library `Interfaces.sysml`).
    /// Previously parsed and discarded by `connector::ref_decl`.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `MultiplicityPart`'s `isOrdered`/`isUnique` keyword slots, each carrying the authored
    /// spelling and its exact span. See [`MultiplicityModifiers`](crate::ast::MultiplicityModifiers).
    pub multiplicity_modifiers: crate::ast::MultiplicityModifiers,
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
            && self.short_name == other.short_name
            && self.direction == other.direction
            && self.is_derived == other.is_derived
            && self.usage_prefix == other.usage_prefix
            && self.is_constant == other.is_constant
            && self.kind_keyword == other.kind_keyword
            && self.typing == other.typing
            && self.redefines == other.redefines
            && self.subsets == other.subsets
            && self.multiplicity == other.multiplicity
            && self.multiplicity_modifiers == other.multiplicity_modifiers
            && self.value == other.value
            && self.body == other.body
            && self.membership == other.membership
    }
}

/// The kind keyword after `ref` on a [`RefDecl`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RefDeclKind {
    /// `part`.
    Part,
    /// `port`.
    Port,
    /// `item`.
    Item,
    /// `requirement`.
    Requirement,
    /// `use case` (`ref use case self : UseCase :>> Case::self;`, Systems Library
    /// `UseCases.sysml`; spec42 Gap 34).
    UseCase,
    /// `concern` (`ref concern :>> self: ConcernCheck;`, Systems Library `Requirements.sysml`).
    Concern,
    /// `viewpoint` (`ref viewpoint :>> self : ViewpointCheck;`, Systems Library `Views.sysml`).
    Viewpoint,
    /// `rendering` (`abstract ref rendering subrenderings : Rendering[0..*] :> renderings;`,
    /// Systems Library `Views.sysml`).
    Rendering,
    /// `view` (`abstract ref view subviews : View[0..*] :> views { ... }`, Systems Library
    /// `Views.sysml`).
    View,
    /// `action` (`private ref action thisConnection = self;`, Systems Library `Flows.sysml`).
    Action,
    /// `case` (`ref case self : Case :>> Calculation::self;`, Systems Library `Cases.sysml`).
    Case,
    /// `verification` (`ref verification self : VerificationCase :>> Case::self;`, Systems
    /// Library `VerificationCases.sysml`).
    Verification,
}

impl RefDeclKind {
    /// The authored keyword spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Part => "part",
            Self::Port => "port",
            Self::Item => "item",
            Self::Requirement => "requirement",
            Self::UseCase => "use case",
            Self::Concern => "concern",
            Self::Viewpoint => "viewpoint",
            Self::Rendering => "rendering",
            Self::View => "view",
            Self::Action => "action",
            Self::Case => "case",
            Self::Verification => "verification",
        }
    }
}

/// Body of a ref declaration: `;` or `{` members `}`.
/// A `ref` usage body.
///
/// `UsageBody = DefinitionBody` (SysML 8.2.2.6.2), so a `ref` body holds the same members as any
/// other usage body no matter which declaration owns the `ref`. It previously had its own element
/// enum whose contents depended on which of five parsers ran -- an encoding of parser provenance
/// rather than of grammar.
pub type RefBody = Body<PartUsageBodyElement>;

/// Shared annotation-only body element for KerML `RelationshipBody` contexts -- BNF
/// `RelationshipBody : Relationship = ';' | '{' (ownedRelationship += OwnedAnnotation)* '}'`,
/// used by `AliasMember`/`Import`/`Dependency` -- and other leaf bodies where a full
/// nested-member grammar isn't (yet) modeled (plain `connect` statement bodies): doc/comment/
/// metadata annotations are retained; anything else recovers to `Error`/`Other` instead of being
/// silently discarded.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::large_enum_variant)]
pub enum RelationshipBodyElement {
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    /// Owned feature member (`dependency z to x, y { feature e; }`; BNF `RelationshipBody`'s
    /// `ownedRelatedElement`, spec42 Gap 37); see [`crate::ast::KermlFeature`].
    KermlFeature(Box<Node<crate::ast::KermlFeature>>),
    Error(Node<ParseErrorNode>),
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
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
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
pub type ConnectionDefBody = Body<ConnectionDefBodyElement>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConnectionDefBodyElement {
    EndDecl(Node<EndDecl>),
    RefDecl(Node<RefDecl>),
    ConnectStmt(Node<ConnectStmt>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    /// `#Tag` metadata reference: `PrefixMetadataMember` prefixing the next member, or the
    /// `ExtendedUsage` member spelling `#Tag;` / `#Tag { ... }`. This scope reaches both --
    /// every member may carry a prefix, and `ExtendedUsage` is a `NonOccurrenceUsageElement` --
    /// but modelled neither, so a `#` member was reported unsupported here.
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    Error(Node<ParseErrorNode>),
    /// PAR-002 widening: this enum previously had no attribute/item/port coverage at all.
    AttributeDef(Node<AttributeDef>),
    AttributeUsage(Node<AttributeUsage>),
    /// `item def`, using `item_def_required`. See `AttributeDef`.
    ItemDef(Node<ItemDef>),
    ItemUsage(Node<ItemUsage>),
    /// `port def`, using `port_def`. See `AttributeDef`.
    PortDef(Node<PortDef>),
    PortUsage(Box<Node<PortUsage>>),
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

pub type EnumerationBody = Body<EnumerationBodyElement>;

/// A member of an `enum def { ... }` body.
///
/// `EnumerationBody` is the one production that names the membership directly --
/// `';' | '{' ( ownedRelationship += AnnotatingMember | ownedRelationship += EnumerationUsageMember )* '}'`
/// (SysML 8.2.2.8) -- so this scope's member set is the annotating production plus enumerated
/// values, and nothing else.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EnumerationBodyElement {
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    /// `EnumerationUsageMember`, one enumerated value.
    Value(Node<EnumeratedValue>),
    /// Malformed syntax retained by the structured recovery parser. This body had no recovery
    /// representation at all: an unparseable member sent it to the closing brace, discarding
    /// everything in between with no node and no diagnostic.
    Error(Node<ParseErrorNode>),
}

/// One enumerated value inside an `enum def { ... }` body. `EnumeratedValue` is a full SysML
/// `Usage`, so its identification, optional value part and usage body are all retained.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumeratedValue {
    pub name: String,
    pub short_name: Option<String>,
    pub value: Option<Node<FeatureValue>>,
    pub body: PartUsageBody,
    pub name_span: Option<Span>,
}

// ---------------------------------------------------------------------------
// Occurrence (Phase 2)
// ---------------------------------------------------------------------------

/// Occurrence definition: `occurrence def` Identification body (BNF OccurrenceDefinition).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OccurrenceDef {
    /// `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation ?= 'variation'`
    /// (SysML BNF 219; Pilot `SysML.xtext` 490) -- one slot, two alternatives, carrying the
    /// authored keyword's exact span. `OccurrenceDefinition` (SysML BNF 548) reaches it through
    /// `OccurrenceDefinitionPrefix` (SysML BNF 541).
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
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
    /// The complete `OccurrenceUsagePrefix` this usage was written with.
    ///
    /// One shared component rather than six independent fields: the production is
    /// `BasicUsagePrefix ('individual')? PortionKind? UsageExtensionKeyword*`, every slot keeps
    /// its authored span, and the three mutually exclusive slots cannot hold two alternatives at
    /// once. `IndividualUsage`, `PortionUsage` and `EventOccurrenceUsage` inline or name the same
    /// production, so all four spellings of this family share one value; which of them was
    /// written is the combination of slots plus the kind keyword recorded below.
    pub prefix: super::occurrence_prefix::OccurrenceUsagePrefix,
    /// `SourceSuccessionMember`'s `then`, as the authored keyword's span; see
    /// [`PartUsage::then_span`]. Was a `bool`, which kept no provenance -- one representation per
    /// syntactic fact, and `Some` *is* the flag.
    pub then_span: Option<Span>,
    /// True for `event occurrence <name>;` (BNF `EventOccurrenceUsage`, §6 G7) — an occurrence
    /// that marks a point in time rather than owning a lifetime. A kind keyword after the prefix,
    /// not a prefix slot.
    pub is_event: bool,
    /// True when the literal `occurrence` kind keyword was authored (BNF `OccurrenceUsage`),
    /// distinct from the prefix's `individual` slot -- `individual occurrence o1;` and bare
    /// `individual o1;` both author `individual`, but only the former also authors this (gap #7).
    /// Needed so emission doesn't fabricate or drop the keyword relative to what was authored.
    pub has_occurrence_keyword: bool,
    /// Declaration label for ordinary occurrence usages.
    pub name: String,
    pub short_name: Option<String>,
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

pub type OccurrenceUsageBody = Body<OccurrenceBodyElement>;

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
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    /// `#Tag` metadata reference: `PrefixMetadataMember` prefixing the next member, or the
    /// `ExtendedUsage` member spelling `#Tag;` / `#Tag { ... }`. `OccurrenceDefinition`/
    /// `OccurrenceUsage` reach both through `DefinitionExtensionKeyword`/`UsageExtensionKeyword`
    /// and `DefinitionBodyItem → NonOccurrenceUsageMember → ExtendedUsage`; this scope was the
    /// one body that modelled neither and captured `#` as opaque text instead.
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    AssertConstraint(Node<AssertConstraintMember>),
    FlowUsage(Node<crate::ast::behavior::FlowUsage>),
    /// `bind` connector usage in an occurrence body. `OccurrenceDefinition`/`OccurrenceUsage`
    /// bodies admit `NonOccurrenceUsageMember`, whose `NonOccurrenceUsageElement` includes
    /// `BindingConnectorAsUsage` (SysML textual BNF 237-247, 349-353, 702-707; the pinned
    /// Pilot SysML grammar agrees). Keep the existing structured connector rather than
    /// recovering its text or rediscovering its ends during emission.
    Bind(Node<Bind>),
    AttributeUsage(Node<AttributeUsage>),
    PartUsage(Box<Node<PartUsage>>),
    /// `item x;` inside an occurrence definition/usage body (GH-87), e.g. `occurrence def Occ {
    /// item x; }` (Simple Tests/OccurrenceTest.sysml:6). `item_usage` itself already fully
    /// supports the bare (untyped, no value) form -- it just wasn't dispatched here.
    ItemUsage(Node<ItemUsage>),
    OccurrenceUsage(Box<Node<OccurrenceUsage>>),
    SuccessionUsage(Node<SuccessionUsage>),
    /// `SatisfyRequirementUsage` inside an occurrence definition body (previously only
    /// reachable at package level).
    Satisfy(Box<Node<SatisfyRequirementUsage>>),
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
    /// `ref`-prefixed feature declaration, e.g. `ref self : SuccessionFlow :>> Flow::self,
    /// FlowTransfer::self;` and `private ref action thisConnection = self;` (Systems Library
    /// `Flows.sysml`). Occurrence bodies accepted no `ref` member at all.
    RefDecl(Node<RefDecl>),
    /// Connection usage, e.g. `connection :HappensDuring connect sourceEvent to [1] self;`
    /// (Systems Library `Flows.sysml`). Part and attribute bodies already dispatched this
    /// member; occurrence bodies did not.
    ConnectionUsage(Box<Node<ConnectionUsageMember>>),
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
    /// `UsageBody = DefinitionBody`, so this body owns the whole usage member set. It was a
    /// `ConnectBody` marker (`Semicolon | Brace`, no delimiter spans) whose brace form was parsed
    /// by `advance_to_closing_brace` and kept nothing at all.
    pub body: PartUsageBody,
    pub membership: Membership,
}

// ---------------------------------------------------------------------------
// Library Package (Phase 2)
// ---------------------------------------------------------------------------

/// Generic definition body: `;` or `{` DefinitionBodyElement* `}`.
pub type DefinitionBody = Body<DefinitionBodyElement>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::large_enum_variant)]
pub enum DefinitionBodyElement {
    /// A spec-valid member of this body that the parser does not model yet, retained with
    /// its authored span and a diagnostic.
    Unsupported(Node<crate::ast::UnsupportedGrammarNode>),
    Error(Node<ParseErrorNode>),
    /// Every recognized member of this body, including the annotating ones. This scope shares
    /// the occurrence-body member set rather than restating it, so it has no annotating variant
    /// of its own: a `Doc` variant here had no construction site in the parser at all --
    /// documentation in a flow, allocation or message body has always arrived as
    /// `OccurrenceMember(OccurrenceBodyElement::Annotating(..))`.
    OccurrenceMember(Node<OccurrenceBodyElement>),
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
    /// The body after the bind, delimiters included (`bind x = y;`, `bind x = y { ... }`).
    ///
    /// `BindingConnectorAsUsage`'s body is `UsageBody`, the same general usage-member production
    /// `PartUsageBody` uses. This was the last `ConnectBody` marker in the AST: a
    /// `Semicolon | Brace` flag with no delimiter spans, paired with a separate `body_elements`
    /// list, so one body fact lived in two fields and an empty `{ }` was indistinguishable from
    /// a `{ ... }` whose members had been discarded.
    ///
    /// Not an `Option`: `BindingConnectorAsUsage` ends at `UsageCompletion`, so a bind always
    /// writes `;` or `{ ... }`. The parser only ever produced `Some`.
    pub body: PartUsageBody,
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
        /// The `InterfaceBody`, delimiters included. Was a `ConnectBody` marker beside a separate
        /// element list -- one body fact in two fields, with no span for either brace.
        body: Body<InterfaceUsageBodyElement>,
    },
    /// `interface` from `to` to body.
    Connection {
        subsets: Option<Node<SubsettingRelationship>>,
        redefines: Option<Node<SubsettingRelationship>>,
        from: Node<Expression>,
        to: Node<Expression>,
        /// See [`InterfaceUsage::TypedConnect`]'s body. This variant kept only an element list
        /// that was always empty -- the parser discarded the body outright -- so the `;`/`{}`
        /// distinction and every member were lost.
        body: Body<InterfaceUsageBodyElement>,
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
        /// See [`InterfaceUsage::TypedConnect`]'s body.
        body: Body<InterfaceUsageBodyElement>,
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
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    /// `end` member inside a typed, non-`connect` interface usage's body. Boxed: `EndDecl` is
    /// much larger than `RefRedef`, the other variant here.
    EndDecl(Box<Node<EndDecl>>),
}

/// Connect at part usage level: `connect` from `to` to body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Connect {
    pub from: Node<ConnectionEnd>,
    pub to: Node<ConnectionEnd>,
    /// `UsageBody = DefinitionBody`, so this body owns the whole usage member set. It was a
    /// `ConnectBody` marker (`Semicolon | Brace`, no delimiter spans) whose brace form was parsed
    /// by `advance_to_closing_brace` and kept nothing at all.
    pub body: PartUsageBody,
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
    /// `UsageBody = DefinitionBody`, so this body owns the whole usage member set. It was a
    /// `ConnectBody` marker (`Semicolon | Brace`, no delimiter spans) whose brace form was parsed
    /// by `advance_to_closing_brace` and kept nothing at all.
    pub body: PartUsageBody,
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
pub type AliasBody = Body<RelationshipBodyElement>;
