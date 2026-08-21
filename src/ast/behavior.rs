use super::body::Body;
use super::common::{AnnotatingMember, Identification, ParseErrorNode};
use super::membership::Membership;
use super::requirement::RequirementUsage;
use super::structure::{
    Bind, DefinitionBody, MetadataKeywordUsage, MetadataUsage, Perform, RefDecl,
};
use crate::ast::core::{
    Expression, Multiplicity, Node, Span, SubsettingRelationship, TypingRelationship,
};
use crate::ast::feature_value::FeatureValue;
use crate::ast::DefinitionPrefix;
use crate::ast::QualifiedReferenceId;

/// Action definition: `action def` Identification body (in/out params).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionDef {
    /// `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation ?= 'variation'`
    /// (SysML BNF 219; Pilot `SysML.xtext` 490) -- one slot, two alternatives, carrying the
    /// authored keyword's exact span. `ActionDefinition` (SysML BNF 894) reaches it through
    /// `OccurrenceDefinitionPrefix` (SysML BNF 541).
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
    /// `individual action def FuelConsumption_1 :> FuelConsumption;` (GH-90.1, `Individuals
    /// Examples/AnalysisIndividualExample.sysml:77`).
    pub is_individual: bool,
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: ActionDefBody,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006), `kind` always
    /// [`crate::ast::MembershipKind::OwningMembership`]. Genuine new grammar coverage: `action_def`
    /// did not previously accept a `private`/`protected`/`public` prefix -- same gap class found
    /// repeatedly in this rollout (see `crate::ast::PortDef::membership`).
    pub membership: Membership,
}

/// Body of an action definition: `;` or `{` ActionDefBodyElement* `}`.
pub type ActionDefBody = Body<ActionDefBodyElement>;

/// Element inside an action definition body.
// Body-element variants intentionally preserve their direct `Node<T>` AST shape. Boxing only
// `ThenAction` would make equivalent action constructs differ by containing context and introduce
// indirection throughout parser consumers solely to satisfy a size heuristic.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActionDefBodyElement {
    Error(Node<ParseErrorNode>),
    /// An import owned directly by this action body.
    ///
    /// `ActionBodyItem → NonBehaviorBodyItem → Import` (SysML textual BNF 901-917; pinned
    /// Pilot `SysML.xtext` 1368-1381) applies to both action definitions and usages.
    Import(Node<crate::ast::Import>),
    InOutDecl(Node<InOutDecl>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    /// A dependency owned by this action definition.
    ///
    /// `ActionBody → ActionBodyItem → NonBehaviorBodyItem → DefinitionMember →
    /// DefinitionElement → Dependency`, so `dependency X to Y;` -- and the
    /// `PrefixMetadataAnnotation`-tagged `#refinement dependency X to Y;` the Apollo model
    /// writes here -- are ordinary members of this scope. Neither was dispatched, so the whole
    /// statement was swallowed by the opaque `#` fallback that owned `AnnotationHead::Opaque`.
    Dependency(Node<crate::ast::Dependency>),
    /// Literal `metadata` keyword form (BNF `MetadataUsage`'s `('@' | 'metadata')` alternative,
    /// GH-86), e.g. `metadata ToolExecution { toolName = "ModelCenter"; }` (Analysis Examples/
    /// AnalysisAnnotation.sysml). Previously only dispatched at package-body scope.
    MetadataUsage(Node<MetadataUsage>),
    RefDecl(Node<RefDecl>),
    Perform(Node<Perform>),
    Bind(Node<Bind>),
    FlowUsage(Node<FlowUsage>),
    FirstStmt(Node<FirstStmt>),
    MergeStmt(Node<MergeStmt>),
    DecisionStmt(Node<DecisionStmt>),
    JoinStmt(Node<JoinStmt>),
    ForkStmt(Node<ForkStmt>),
    TerminateStmt(Node<TerminateStmt>),
    WhileStmt(Node<WhileStmt>),
    /// `loop { ... }` (§6 G14).
    LoopStmt(Node<LoopStmt>),
    IfStmt(Node<IfStmt>),
    StateUsage(Node<StateUsage>),
    ActionUsage(Box<Node<ActionUsage>>),
    /// `part` usage via `StructureUsageMember` in `ActionBodyItem` (GH-13).
    PartUsage(Box<Node<crate::ast::PartUsage>>),
    /// `item` usage via `StructureUsageMember` in `ActionBodyItem` (GH-13).
    ItemUsage(Node<crate::ast::ItemUsage>),
    /// `assert constraint` via `BehaviorUsageMember` / `AssertConstraintUsage` (GH-13).
    AssertConstraint(Node<crate::ast::AssertConstraintMember>),
    /// `snapshot` / portion usage via `StructureUsageMember` → `PortionUsage` (GH-13).
    OccurrenceUsage(Box<Node<crate::ast::OccurrenceUsage>>),
    Assign(Node<AssignStmt>),
    ForLoop(Node<ForLoop>),
    ThenAction(Node<ThenAction>),
    /// `attribute` usage declared inside an action body (BNF `ActionBodyItem` →
    /// `NonBehaviorBodyItem` → `StructureUsageMember`), e.g. `attribute mass = 5;` as a sibling
    /// of ordinary action statements (spec42 Gap 33; formerly the opaque `ActionBodyDecl`).
    AttributeUsage(Box<Node<crate::ast::AttributeUsage>>),
    /// `calc` usage declared inside an action body (spec42 Gap 33; formerly the opaque
    /// `ActionBodyDecl`).
    CalcUsage(Box<Node<crate::ast::CalcUsage>>),
    /// Nested `action def` declaration (spec42 Gap 33; formerly flattened to the opaque
    /// `ActionBodyDecl` keeping only the definition's name).
    ActionDef(Box<Node<ActionDef>>),
    /// Keyword-less `name = expr;` feature binding (§6 G26), e.g. `measurement =
    /// testVehicle.mass;` in the OMG spec Annex `9-Verification-simplified.sysml`.
    DefaultReferenceUsage(Node<crate::ast::DefaultReferenceUsage>),
}

/// Assignment statement (SysML v2 AssignmentNode/AssignmentActionUsage).
///
/// Examples:
/// - `assign x := y;`
/// - `then assign position := dynamics.x_out;`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssignStmt {
    pub is_then: bool,
    pub lhs: Node<Expression>,
    pub rhs: Node<Expression>,
}

/// For-loop node (SysML v2 ForLoopNode) - modeled minimally.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForLoop {
    pub var: String,
    pub range: Node<Expression>,
    pub body: ActionDefBody,
}

/// Succession to a following node: `then action ...`, `then perform ...`, a control node, or
/// `then <name>;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThenAction {
    pub target: ThenTarget,
}

/// What a `then` succession connects to. Control-node alternatives own their declaration and
/// mandatory `ActionBody`, rather than degrading an anonymous control keyword into a feature
/// reference.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ThenTarget {
    /// `then action <name> accept ...;` — an inline action usage declaration.
    Action(Box<Node<ActionUsage>>),
    /// `then perform body;` — succession to a perform usage (Systems Library `Actions.sysml`).
    Perform(Box<Node<crate::ast::Perform>>),
    /// `then merge continue;` or `then merge;` — an inline merge node.
    Merge(Node<MergeStmt>),
    /// `then fork F { in a; out b1; out b2; }` — an inline fork node (GH-86, Simple Tests/
    /// ControlNodeTest.sysml).
    Fork(Node<ForkStmt>),
    /// `then decide D;` — an inline decision node (GH-86, Simple Tests/DecisionTest.sysml).
    Decide(Node<DecisionStmt>),
    /// `then join J;` or `then join;` — an inline join node.
    Join(Node<JoinStmt>),
    /// `then accept S;` — an inline accept trigger, reusing the same shorthand/payload/
    /// time-trigger forms already supported after a state `transition` (GH-86, Simple Tests/
    /// ActionTest.sysml).
    Accept(Node<TransitionAccept>),
    /// `then send new S() to b;` — an inline send action (spec42 gap 30, Simple Tests/
    /// ActionTest.sysml). Carries the same `ActionUsage` shape (with `send`/`via`/`to`
    /// clauses) the standalone `send ...;` statement produces.
    Send(Box<Node<ActionUsage>>),
    /// `then continue;` — a reference to an already-declared node.
    Feature(Node<Expression>),
}

/// Direction-prefixed parameter declaration (BNF `FeatureDirection` + keyword-less `Usage`),
/// e.g. `in` name `:` type `;`, `inout replacementValues : Anything[0..*] nonunique;`, or
/// `in transitionLinkSource : Action :>> TransitionPerformance::transitionLinkSource;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InOutDecl {
    pub direction: InOut,
    /// Whether the declaration used the `ref` feature prefix (`in ref name : Type`).
    pub is_reference: bool,
    /// Whether the declaration used the KerML `var` time-varying prefix (`out var y1;`).
    pub is_var: bool,
    /// Declared parameter name. Empty for the leading `:>> target` redefinition form.
    pub name: String,
    /// `:>` subsets clause (`out voltage :> ISQ::electricPotential = ...;`, spec42 evsample;
    /// Gap 45 fallout). Previously the `:>` spelling was silently folded into `type_name`.
    pub subsets: Option<Node<crate::ast::SubsettingRelationship>>,
    pub type_name: Option<QualifiedReferenceId>,
    /// Multiplicity clause (BNF `MultiplicityPart`). May precede the typing (`in
    /// transitionLinkSource[1]: StateAction :>> ...`, Systems Library `States.sysml`) or follow
    /// it (`inout replacementValues : Anything[0..*] nonunique;`, `Actions.sysml`); the parser
    /// accepts one clause in either position.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `MultiplicityPart`'s `isOrdered`/`isUnique` keyword slots, each carrying the authored
    /// spelling and its exact span. See [`MultiplicityModifiers`](crate::ast::MultiplicityModifiers).
    pub multiplicity_modifiers: crate::ast::MultiplicityModifiers,
    /// Arena-backed redefinition targets, from either position the grammar allows: the leading
    /// unnamed form (`in :>> target = expr;`, validation `08`; `name` is empty) or trailing a
    /// named declaration (`in transitionLinkSource : Action :>> A::x, B::y;`, Systems Library
    /// `Actions.sysml`/`States.sysml`; `name` is non-empty). Comma-separated multi-target
    /// clauses keep every target.
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// Optional value clause: `= expr`, `:= expr`, or `default (=|:=)? expr` (BNF
    /// `FeatureValue`), e.g. `in target : Occurrence[1] default that as Occurrence { ... }`
    /// (Systems Library `Actions.sysml`).
    pub value: Option<Node<FeatureValue>>,
    /// Retained `{ ... }` terminator body elements (`in occ ... { doc ... }`); `None` for the
    /// `;`-terminated form. Parameter bodies share the action-body member grammar, matching the
    /// parser, which always dispatched brace terminators through the action-body machinery.
    pub body: Option<Vec<Node<ActionDefBodyElement>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InOut {
    In,
    Out,
    InOut,
}

/// Typed payload on accept/send control nodes: `accept name : Type` or `send name : Type`.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PayloadClause {
    pub name: String,
    pub type_name: Option<QualifiedReferenceId>,
    pub name_span: Span,
    pub type_span: Option<Span>,
}

impl PartialEq for PayloadClause {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.type_name == other.type_name
    }
}

/// Payload on a standalone `send` control-node statement (GH-86): either the typed-name form
/// (`send name : Type;`, e.g. Systems Library `SendAction`) or a general expression (`send new
/// Publish(someTopic, somePublication);`, Interaction Sequencing Examples/
/// ServerSequenceOutsideRealization-2.sysml). BNF `SendNode`'s payload is `NodeParameterMember`
/// (`FeatureBinding = OwnedExpression`) -- a general expression -- unlike `accept`'s
/// `PayloadParameter`, which stays typed-name-only (kept as [`PayloadClause`] on
/// [`ActionUsage::accept`]).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SendPayload {
    Typed(PayloadClause),
    Expression(Node<Expression>),
}

/// Transition accept trigger: typed payload or shorthand expression (`accept StartPressed`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransitionAccept {
    /// `(payload, via)` — `via` is the optional `via <port>` clause after the trigger.
    Payload(PayloadClause, Option<Node<Expression>>),
    /// `(expr, via)` — `via` is the optional `via <port>` clause after the trigger.
    Shorthand(Node<Expression>, Option<Node<Expression>>),
    /// A time trigger: `accept at <expr>`, `accept when <expr>`, or `accept after <expr>`
    /// (BNF `TriggerKind`, §6 G8). Real usage: `accept at vehicle1_c1.maintenanceTime` in the
    /// OMG spec Annex `5-State-based Behavior-1.sysml`. Without this the trigger keyword was
    /// swallowed as the payload expression itself.
    TimeTrigger(TriggerKind, Node<Expression>),
}

/// The three time-trigger keywords of BNF `TriggerKind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TriggerKind {
    /// `accept at <time-expr>` — fires at an absolute instant.
    At,
    /// `accept when <bool-expr>` — fires on a change to the condition.
    When,
    /// `accept after <duration-expr>` — fires a duration after the source became active.
    After,
}

/// Transition `do` effect: a structured action-usage form (SysML v2 `EffectBehaviorUsage`)
/// or a bare expression shorthand.
///
/// Examples: `do action powerUp : PowerUp;`, `do action { in x; }`,
/// `do send new TimeoutSignal() via commPort`, `do accept Ack via commPort`,
/// `do assign x := y`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransitionEffect {
    /// `action` UsageDeclaration? ActionBody? — perform an owned action. The declaration and
    /// brace body are independently optional (SysML BNF 1324-1325; Pilot `EffectBehaviorUsage`
    /// 1917-1919), so `do action { ... }` retains a body without inventing a declaration name.
    Perform {
        name: Option<String>,
        type_name: Option<QualifiedReferenceId>,
        body: Option<ActionDefBody>,
    },
    /// `accept` payload (`:` type)? (`via` expr)?
    Accept {
        payload: Node<Expression>,
        type_name: Option<QualifiedReferenceId>,
        via: Option<Node<Expression>>,
        body: Option<ActionDefBody>,
    },
    /// `send` payload (`:` type)? ((`via` expr)? (`to` expr)? | `to` expr)
    Send {
        payload: Node<Expression>,
        type_name: Option<QualifiedReferenceId>,
        via: Option<Node<Expression>>,
        to: Option<Node<Expression>>,
        body: Option<ActionDefBody>,
    },
    /// `assign` lhs `:=` rhs
    Assign {
        lhs: Node<Expression>,
        rhs: Node<Expression>,
        body: Option<ActionDefBody>,
    },
    /// Bare expression shorthand (e.g. a reference to an existing action usage).
    Expression(Node<Expression>),
}

/// Action usage: `(abstract|variation)? (ref)? action` name (`:` type)? (`[mult]`)? (`:>`/` :>>` …)? body.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionUsage {
    /// Leading `abstract` keyword (Systems Library e.g. `abstract ref action performedActions`).
    pub is_abstract: bool,
    /// Leading `variation` keyword (`RefPrefix.isVariation`, GH-13). Mutually exclusive with
    /// [`Self::is_abstract`] per BNF `RefPrefix`.
    pub is_variation: bool,
    /// Leading `ref` keyword — reference feature usage rather than composite
    /// (`ref action …` inside a part body).
    pub is_reference: bool,
    /// Leading `individual` keyword (BNF `OccurrenceUsagePrefix`, GH-90.1), e.g. `individual
    /// action a : AP1;` (`Simple Tests/IndividualTest.sysml:30`).
    pub is_individual: bool,
    pub name: String,
    pub short_name: Option<String>,
    pub type_name: Option<QualifiedReferenceId>,
    /// Structured typing clause (multi-target capable), mirroring `PartUsage.typing`.
    pub typing: Option<Node<TypingRelationship>>,
    /// Multiplicity after the type, e.g. `[0..*]`.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// Optional `subsets` / `:>` clause.
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// Optional `redefines` / `:>>` clause.
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// For `action ... accept param : Type` form.
    pub accept: Option<PayloadClause>,
    /// For standalone `send` control-node statements: `send param : Type` or a general
    /// expression payload (`send new Publish(x, y)`, GH-86).
    pub send: Option<SendPayload>,
    /// Optional `via <expr>` targeting clause on a standalone `accept`/`send` control-node
    /// statement (BNF `AcceptParameterPart`/`SenderReceiverPart`, GH-86), e.g. `send new
    /// Publish(someTopic, somePublication) via publicationPort;`.
    pub via: Option<Node<Expression>>,
    /// Optional trailing `to <expr>` clause on a standalone `send` statement (BNF
    /// `SenderReceiverPart`, GH-86), e.g. `then send new S() to b;` (Simple Tests/ActionTest.sysml).
    pub to: Option<Node<Expression>>,
    /// `None` when no terminator was authored and the following statement implies the end of
    /// this usage (Systems Library `LoopAction` style). The grammar requires `;` or `{ ... }`,
    /// so this records that neither was written rather than pretending a `;` was.
    pub body: Option<ActionUsageBody>,
    /// Span of the usage name (for semantic tokens).
    pub name_span: Option<Span>,
    /// Span of the type reference after `:` (for semantic tokens).
    pub type_ref_span: Option<Span>,
    /// Ownership/visibility/kind wrapper (parser work item 4b, post-PAR-006), `kind` always
    /// [`crate::ast::MembershipKind::FeatureMembership`]. Captured with real visibility only for
    /// the primary `action_usage` parser; the `accept`/`send` standalone control-node-statement
    /// constructor (`control_node_payload_stmt`) has no visibility grammar of its own and always
    /// sets `visibility: None`, matching this rollout's ad hoc-site convention.
    pub membership: Membership,
}

impl PartialEq for ActionUsage {
    fn eq(&self, other: &Self) -> bool {
        self.is_abstract == other.is_abstract
            && self.is_variation == other.is_variation
            && self.is_reference == other.is_reference
            && self.is_individual == other.is_individual
            && self.name == other.name
            && self.short_name == other.short_name
            && self.type_name == other.type_name
            && self.typing == other.typing
            && self.multiplicity == other.multiplicity
            && self.subsets == other.subsets
            && self.redefines == other.redefines
            && self.accept == other.accept
            && self.send == other.send
            && self.via == other.via
            && self.to == other.to
            && self.body == other.body
            && self.membership == other.membership
    }
}

/// Body of an action usage: `;` or `{` ActionUsageBodyElement* `}`.
pub type ActionUsageBody = Body<ActionUsageBodyElement>;

/// Element inside an action usage body.
// Keep the same direct-node representation as `ActionDefBodyElement`; see its size rationale.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActionUsageBodyElement {
    Error(Node<ParseErrorNode>),
    /// An import owned directly by this action body; see [`ActionDefBodyElement::Import`].
    Import(Node<crate::ast::Import>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    /// A dependency owned by this action usage; see [`ActionDefBodyElement::Dependency`].
    Dependency(Node<crate::ast::Dependency>),
    /// Literal `metadata` keyword form (BNF `MetadataUsage`'s `('@' | 'metadata')` alternative,
    /// GH-86), e.g. `metadata ToolExecution { toolName = "ModelCenter"; }` (Analysis Examples/
    /// AnalysisAnnotation.sysml). Previously only dispatched at package-body scope.
    MetadataUsage(Node<MetadataUsage>),
    InOutDecl(Node<InOutDecl>),
    RefDecl(Node<RefDecl>),
    Bind(Node<Bind>),
    FlowUsage(Node<FlowUsage>),
    FirstStmt(Node<FirstStmt>),
    MergeStmt(Node<MergeStmt>),
    DecisionStmt(Node<DecisionStmt>),
    JoinStmt(Node<JoinStmt>),
    ForkStmt(Node<ForkStmt>),
    TerminateStmt(Node<TerminateStmt>),
    WhileStmt(Node<WhileStmt>),
    /// `loop { ... }` (§6 G14).
    LoopStmt(Node<LoopStmt>),
    IfStmt(Node<IfStmt>),
    StateUsage(Node<StateUsage>),
    ActionUsage(Box<Node<ActionUsage>>),
    /// `part` usage via `StructureUsageMember` in `ActionBodyItem` (GH-13).
    PartUsage(Box<Node<crate::ast::PartUsage>>),
    /// `item` usage via `StructureUsageMember` in `ActionBodyItem` (GH-13).
    ItemUsage(Node<crate::ast::ItemUsage>),
    /// `assert constraint` via `BehaviorUsageMember` / `AssertConstraintUsage` (GH-13).
    AssertConstraint(Node<crate::ast::AssertConstraintMember>),
    /// `snapshot` / portion usage via `StructureUsageMember` → `PortionUsage` (GH-13).
    OccurrenceUsage(Box<Node<crate::ast::OccurrenceUsage>>),
    Assign(Node<AssignStmt>),
    ForLoop(Node<ForLoop>),
    ThenAction(Node<ThenAction>),
    /// `attribute` usage declared inside an action body (spec42 Gap 33); see
    /// [`ActionDefBodyElement::AttributeUsage`].
    AttributeUsage(Box<Node<crate::ast::AttributeUsage>>),
    /// `calc` usage declared inside an action body (spec42 Gap 33); see
    /// [`ActionDefBodyElement::CalcUsage`].
    CalcUsage(Box<Node<crate::ast::CalcUsage>>),
    /// Nested `action def` declaration (spec42 Gap 33); see
    /// [`ActionDefBodyElement::ActionDef`].
    ActionDef(Box<Node<ActionDef>>),
    /// Keyword-less `name = expr;` feature binding (§6 G26), e.g. `measurement =
    /// testVehicle.mass;` in the OMG spec Annex `9-Verification-simplified.sysml`.
    DefaultReferenceUsage(Node<crate::ast::DefaultReferenceUsage>),
    /// `variant name;` referencing a sibling variation action's variant (GH-89.7), e.g. `variant
    /// generateTorque4Cyl;` inside `action providePowerFamily : ProvidePower { variation action
    /// generateTorque : GenerateTorque { variant generateTorque4Cyl; ... } ... }`
    /// (`Variability Examples/VehicleVariabilityModel.sysml:128`).
    VariantUsage(Node<crate::ast::VariantUsage>),
}

/// Flow definition: `flow def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlowDef {
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
    pub is_individual: bool,
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: DefinitionBody,
    pub membership: Membership,
}

/// Kind of flow usage statement per SysML v2 §8.2.2.16.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FlowUsageKind {
    Flow,
    Message,
    SuccessionFlow,
}

/// `of` clause payload feature on a flow usage: SysML v2 §8.2.2.16 `PayloadFeature` — an
/// optionally-named `Feature` typed by (and/or given a multiplicity by) the `of` clause, e.g.
/// `of Payload`, `of qty : Payload`, `of qty : Payload[1..3]`. Distinct from a plain expression:
/// the flow owns this as a real `FeatureMembership`, not a value reference.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PayloadFeature {
    pub name: Option<String>,
    pub type_name: Option<QualifiedReferenceId>,
    pub type_is_conjugated: bool,
    pub multiplicity: Option<Node<Multiplicity>>,
}

/// Flow usage: `flow` | `message` | `succession flow` with optional name, payload, and endpoints.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlowUsage {
    pub kind: FlowUsageKind,
    pub name: Option<String>,
    pub type_name: Option<QualifiedReferenceId>,
    pub type_is_conjugated: bool,
    /// `:>` subsets clause (spec42 gap 28), previously parsed by the shared usage header and
    /// discarded.
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// `:>>` redefines clause. See `subsets`.
    pub redefines: Option<Node<SubsettingRelationship>>,
    pub payload: Option<Node<PayloadFeature>>,
    /// Typed `from <end> to <end>` ends (spec42 gap 28), the same connector-end shape
    /// `AllocationUsage` and the KerML connector members use -- previously opaque `Expression`
    /// nodes.
    pub from: Option<Node<crate::ast::KermlConnectorEnd>>,
    pub to: Option<Node<crate::ast::KermlConnectorEnd>>,
    pub body: DefinitionBody,
    pub membership: Membership,
}

/// First/then control flow: `first` expr `then` expr body, with an optional leading `succession`
/// keyword prefix (SysML v2 §8.2.2.13.3 `SuccessionAsUsage`, GH-38) that may itself carry a
/// name, a type, and/or a multiplicity, e.g. `succession first a then b;`,
/// `succession s1 : AB first a then b;`, `succession [seBeforeNum] first [0..1] a then [0..1]
/// b;` (Systems Library `Flows.sysml`/`States.sysml`, `ConnectionTest.sysml`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FirstStmt {
    /// Name of the succession itself, e.g. `succession s first a then b;`. `None` when the
    /// bare `first ... then ...` form is used (no `succession` keyword) or the `succession`
    /// prefix is unnamed.
    pub succession_name: Option<String>,
    /// Type of the succession itself, e.g. `succession s1 : AB first a then b;`.
    pub succession_type: Option<QualifiedReferenceId>,
    /// Multiplicity of the succession feature itself, e.g. `succession [seBeforeNum] first ...`.
    pub succession_multiplicity: Option<Node<Multiplicity>>,
    pub first: Node<Expression>,
    /// Multiplicity on the `first` end, e.g. `first [0..1] sourceEvent`.
    pub first_multiplicity: Option<Node<Multiplicity>>,
    /// `None` for the standalone initial-node marker `first start;` (§6 G13), which names a
    /// starting node without declaring a succession to a target.
    pub then: Option<Node<Expression>>,
    /// Multiplicity on the `then` end, e.g. `then [0..1] self`.
    pub then_multiplicity: Option<Node<Multiplicity>>,
    pub body: FirstMergeBody,
}

/// The optional declaration following a control-node keyword.
///
/// `UsageDeclaration` is syntactically present in the pinned SysML BNF but can be empty because
/// its `Identification` slots are optional (BNF 42-44, 308-310); the Pilot grammar writes the
/// same fact as `UsageDeclaration?` (SysML.xtext 1664-1685). Model the two authored states
/// explicitly rather than encoding an absent declaration as an optional expression.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ControlNodeDeclaration {
    /// `merge;`, `decide;`, `join;`, or `fork;`.
    Anonymous,
    /// The currently supported named declaration surface, e.g. `merge continue;`.
    Named(Node<Expression>),
}

/// Merge: `merge` declaration action-body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MergeStmt {
    pub declaration: ControlNodeDeclaration,
    pub body: FirstMergeBody,
}

/// Decision node: `decide` declaration action-body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecisionStmt {
    pub declaration: ControlNodeDeclaration,
    pub body: FirstMergeBody,
}

/// Join node: `join` declaration action-body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JoinStmt {
    pub declaration: ControlNodeDeclaration,
    pub body: FirstMergeBody,
}

/// Fork node: `fork` declaration action-body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForkStmt {
    pub declaration: ControlNodeDeclaration,
    pub body: FirstMergeBody,
}

/// Body of a `first`/`merge`/`decide`/`join`/`fork` control node: `;` or `{ ... }`.
///
/// `MergeNode`/`DecisionNode`/`JoinNode`/`ForkNode` (SysML BNF §8.2.2.17.3) all end in the same
/// body production every other declaration ends in, so this is the shared [`Body`] with this
/// scope's member set, not a shape of its own.
///
/// It was its own `Semicolon | Brace(Node<FirstMergeBraceBody>)` enum, which cost two things. The
/// semicolon alternative carried no span, so `first x;` was the one body in the AST whose
/// terminator could not be located without re-scanning the source. And the brace alternative
/// wrapped its delimiters in a second [`Node`], making the body extent two facts that had to be
/// cross-checked against each other -- a check its provenance validation actually performed --
/// where the delimiter spans alone already say it once.
pub type FirstMergeBody = Body<FirstMergeBodyElement>;

/// Provenance rules for a [`FirstMergeBody`] beyond those every body already gets.
///
/// `ProvenanceVisitor::visit_body_braces` validates the delimiters themselves and that members
/// fall between them, for every `Body<E>` in the AST. The rules below are stricter, and were
/// written for this scope when it had a body type of its own: members are ordered and non-empty,
/// nothing but trivia sits between them, and each element's span matches the span of the member it
/// retains. Generalizing the first three to every body family is worthwhile and is deliberately
/// not attempted here -- it would change the provenance contract of fourteen scopes at once.
#[cfg(feature = "serde")]
pub(crate) fn validate_first_merge_body_provenance(
    body: &FirstMergeBody,
    source: &crate::ast::SourceStorage,
) -> Result<(), String> {
    let Body::Brace {
        open_span,
        elements,
        close_span,
    } = body
    else {
        return Ok(());
    };

    fn span_end(span: &Span, role: &str) -> Result<usize, String> {
        span.offset
            .checked_add(span.len)
            .ok_or_else(|| format!("{role} span overflows"))
    }

    fn validate_span(
        source: &crate::ast::SourceStorage,
        span: &Span,
        role: &str,
    ) -> Result<(), String> {
        if source.validates_span(span) {
            Ok(())
        } else {
            Err(format!("invalid first/merge {role} span"))
        }
    }

    validate_span(source, open_span, "open brace")?;
    validate_span(source, close_span, "close brace")?;
    if source.slice(open_span) != Some("{") {
        return Err("first/merge open brace span does not contain \"{\"".to_owned());
    }
    if source.slice(close_span) != Some("}") {
        return Err("first/merge close brace span does not contain \"}\"".to_owned());
    }

    let open_end = span_end(open_span, "first/merge open brace")?;
    if open_end > close_span.offset {
        return Err("first/merge brace delimiter spans overlap".to_owned());
    }

    let mut previous_end = open_end;
    for element in elements {
        validate_span(source, &element.span, "body element")?;
        if element.span.len == 0 {
            return Err("first/merge body elements must have non-empty spans".to_owned());
        }
        let element_end = span_end(&element.span, "first/merge body element")?;
        if previous_end > element.span.offset || element_end > close_span.offset {
            return Err(
                "first/merge body elements must be ordered within the brace delimiters".to_owned(),
            );
        }
        if !source.trivia_between(previous_end, element.span.offset) {
            return Err(
                "unmodeled non-trivia source occurs before a first/merge body element".to_owned(),
            );
        }
        let retained_span = match &element.value {
            FirstMergeBodyElement::Member(member) => &member.span,
            FirstMergeBodyElement::Unsupported(unsupported) => &unsupported.span,
            FirstMergeBodyElement::Error(error) => &error.span,
        };
        if retained_span != &element.span {
            return Err("first/merge body element and retained member spans must match".to_owned());
        }
        previous_end = element_end;
    }
    if !source.trivia_between(previous_end, close_span.offset) {
        return Err(
            "unmodeled non-trivia source occurs after the last first/merge body element".to_owned(),
        );
    }
    Ok(())
}

/// Ordered semantic members retained inside a braced [`FirstMergeBody`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FirstMergeBodyElement {
    /// A member recognized by the shared action-body grammar.
    Member(Box<Node<ActionDefBodyElement>>),
    /// Spec-valid action-body syntax that this AST does not model yet.
    Unsupported(Node<crate::ast::UnsupportedGrammarNode>),
    /// Malformed syntax retained by the structured recovery parser.
    Error(Node<ParseErrorNode>),
}

/// Terminate control node: `terminate;` or `terminate target;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TerminateStmt {
    /// Optional target being terminated; bare `terminate;` terminates the enclosing action.
    pub target: Option<Node<Expression>>,
}

/// While-loop control node: `while` condition `{` ActionDefBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhileStmt {
    pub condition: Node<Expression>,
    pub body: ActionDefBody,
}

/// Loop control node: `loop` `{` body `}` (§6 G14) — a `while` with no condition. Closed alongside
/// `decide`/`join`/`fork`/`if`/`while` in the §5 audit's family, which missed `loop`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LoopStmt {
    pub body: ActionDefBody,
}

/// If control node: `if` condition `{` thenBody `}` (`else` `{` elseBody `}`)?.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfStmt {
    pub condition: Node<Expression>,
    pub then_body: ActionBranchBody,
    pub else_body: Option<ActionBranchBody>,
}

/// The `then` or `else` branch of an `if` control node.
///
/// The grammar offers two spellings (BNF `IfNode`, SysML 8.2.2.17.3): a braced action body, or a
/// single member written without braces (`if x then y;`, `else if ...`). They are different
/// authored syntax, so they are different states here -- a shorthand branch has no delimiters to
/// record, and treating it as a one-member brace body meant re-emitting it with braces the author
/// never wrote.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActionBranchBody {
    /// `{ ... }` exactly as authored, including its delimiters.
    Braced(ActionDefBody),
    /// A single member written without braces.
    Shorthand(Box<Node<ActionDefBodyElement>>),
}

// ---------------------------------------------------------------------------
// Allocation
// ---------------------------------------------------------------------------

/// Allocate statement at part usage level: `allocate` from `to` to body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Allocate {
    pub source: Node<Expression>,
    pub target: Node<Expression>,
    /// `UsageBody = DefinitionBody`, so this body owns the whole usage member set. It was a
    /// `ConnectBody` marker (`Semicolon | Brace`, no delimiter spans) whose brace form was parsed
    /// by `advance_to_closing_brace` and kept nothing at all.
    pub body: crate::ast::PartUsageBody,
}

/// Allocation definition: `allocation def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AllocationDef {
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
    pub is_individual: bool,
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: DefinitionBody,
    pub membership: Membership,
}

/// Allocation usage: `allocation` name (`:` type)? [`allocate` source `to` target]? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AllocationUsage {
    pub name: String,
    pub type_name: Option<QualifiedReferenceId>,
    pub type_is_conjugated: bool,
    /// `:>` subsets clause (spec42 gap 27), previously parsed by the shared usage header and
    /// discarded.
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// `:>>` redefines clause. See `subsets`.
    pub redefines: Option<Node<SubsettingRelationship>>,
    /// Typed `allocate <end> to <end>` ends (spec42 gap 27): each end is an optional
    /// multiplicity plus an arena-backed feature chain with an optional `::>`/`references`
    /// end-name split, the same shape connector members use -- previously opaque
    /// `Expression` nodes with the authored end name discarded.
    pub source: Option<Node<crate::ast::KermlConnectorEnd>>,
    pub target: Option<Node<crate::ast::KermlConnectorEnd>>,
    pub body: DefinitionBody,
    pub membership: Membership,
}

// ---------------------------------------------------------------------------
// Requirements
// ---------------------------------------------------------------------------

/// State definition: `state def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StateDef {
    /// `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation ?= 'variation'`
    /// (SysML BNF 219; Pilot `SysML.xtext` 490) -- one slot, two alternatives, carrying the
    /// authored keyword's exact span. `StateDefinition` (SysML BNF 1191) reaches it through
    /// `OccurrenceDefinitionPrefix` (SysML BNF 541).
    pub definition_prefix: Option<Node<DefinitionPrefix>>,
    /// `individual state def ...` (BNF `OccurrenceUsagePrefix`/definition-prefix `isIndividual`,
    /// GH-90.1), mirroring `ActionDef::is_individual`.
    pub is_individual: bool,
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: StateDefBody,
    pub membership: Membership,
}

pub type StateDefBody = Body<StateDefBodyElement>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StateDefBodyElement {
    Error(Node<ParseErrorNode>),
    /// The complete `AnnotatingElement` production; see [`crate::ast::AnnotatingMember`].
    Annotating(AnnotatingMember),
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    /// `in`/`out`/`inout` parameter redecl inside `entry`/`do`/`exit` action bodies
    /// (e.g. `do 'sense temperature' { out temp; }`, validation `05`).
    InOutDecl(Node<InOutDecl>),
    /// `entry` (`;` or body) - entry action.
    Entry(Node<EntryAction>),
    /// `do` (`;` or body) - do action.
    Do(Node<DoAction>),
    /// `exit` (`;` or body) - exit action.
    Exit(Node<ExitAction>),
    /// `then` name `;` - initial state.
    Then(Node<ThenStmt>),
    /// `final` / `final state` name `;` - explicit final state.
    FinalState(Node<FinalState>),
    /// `ref` name `:` type body – reference binding in state.
    Ref(Box<Node<RefDecl>>),
    RequirementUsage(Node<RequirementUsage>),
    StateUsage(Node<StateUsage>),
    Transition(Box<Node<Transition>>),
    /// `attribute` usage member (`attribute :>> isTriggerDuring;`, Systems Library
    /// `States.sysml`; spec42 Gap 42).
    AttributeUsage(Box<Node<crate::ast::AttributeUsage>>),
    /// `action` usage member (`action :>> subactions :> middle { ... }`, `action substates:
    /// StateAction[0..*] :> stateActions;`; spec42 Gap 42).
    ActionUsage(Box<Node<ActionUsage>>),
    /// Standalone succession usage (`succession stateSequencing first [0..1] exclusiveStates
    /// then [0..1] exclusiveStates { ... }`; spec42 Gap 42).
    SuccessionUsage(Node<crate::ast::SuccessionUsage>),
    /// `assert constraint { ... }` member (spec42 Gap 42).
    AssertConstraint(Node<crate::ast::AssertConstraintMember>),
    /// `part` usage through `StateBodyItem -> NonBehaviorBodyItem ->
    /// StructureUsageMember -> StructureUsageElement -> PartUsage` (SysML BNF 1200-1205,
    /// 910-917, 262-264, 356-363, 623).  It keeps the existing source-backed PartUsage shape
    /// rather than treating a state body as a special spelling of a part body.
    PartUsage(Box<Node<crate::ast::PartUsage>>),
    /// `constraint` usage through the state body's `BehaviorUsageMember` branch (SysML BNF
    /// 1200-1205, 266-268, 374-389, 1382-1395).  ConstraintUsage already owns its complete
    /// occurrence prefix, specialization clauses, and CalculationBody.
    ConstraintUsage(Box<Node<crate::ast::ConstraintUsage>>),
}

/// Entry action: `entry` (`;` or body).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntryAction {
    /// Referenced action for `entry action path body` / `entry path body`; absent for a plain
    /// `entry` body. This is a semantic target, not a declaration label.
    pub action_reference: Option<QualifiedReferenceId>,
    /// True when the `action` keyword was written (`entry action path` vs `entry path`).
    pub has_action_keyword: bool,
    /// Declared name of a *new* nested action (`entry action entryAction :>> 'entry';`,
    /// Systems Library `States.sysml`; spec42 Gap 43). Mutually exclusive with
    /// `action_reference`, which the reference form keeps.
    pub declared_name: Option<String>,
    /// `: Action` typing on the declaration form (`do action doAction : Action :>> 'do';`).
    pub type_name: Option<QualifiedReferenceId>,
    /// `:>> target` redefinition on the declaration form.
    pub redefines: Option<Node<crate::ast::SubsettingRelationship>>,
    /// `assign`/`send`/`accept` effect written directly under the keyword (`entry assign
    /// counter.count := 0;`, spec42 `assignment_test`), mirroring [`Transition::effect`]
    /// (spec42 Gap 43).
    pub effect: Option<TransitionEffect>,
    pub body: StateDefBody,
}

/// Do action: `do` (`;` or body).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DoAction {
    /// Referenced action for `do action path body` / `do path body`; absent for a plain `do` body.
    /// This is a semantic target, not a declaration label.
    pub action_reference: Option<QualifiedReferenceId>,
    /// True when the `action` keyword was written (`do action path` vs `do path`).
    pub has_action_keyword: bool,
    /// Declared name of a *new* nested action (`entry action entryAction :>> 'entry';`,
    /// Systems Library `States.sysml`; spec42 Gap 43). Mutually exclusive with
    /// `action_reference`, which the reference form keeps.
    pub declared_name: Option<String>,
    /// `: Action` typing on the declaration form (`do action doAction : Action :>> 'do';`).
    pub type_name: Option<QualifiedReferenceId>,
    /// `:>> target` redefinition on the declaration form.
    pub redefines: Option<Node<crate::ast::SubsettingRelationship>>,
    /// `assign`/`send`/`accept` effect written directly under the keyword (`entry assign
    /// counter.count := 0;`, spec42 `assignment_test`), mirroring [`Transition::effect`]
    /// (spec42 Gap 43).
    pub effect: Option<TransitionEffect>,
    pub body: StateDefBody,
}

/// Exit action: `exit` (`;` or body).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExitAction {
    /// Referenced action for `exit action path body` / `exit path body`; absent for a plain `exit`
    /// body. This is a semantic target, not a declaration label.
    pub action_reference: Option<QualifiedReferenceId>,
    /// True when the `action` keyword was written (`exit action path` vs `exit path`).
    pub has_action_keyword: bool,
    /// Declared name of a *new* nested action (`entry action entryAction :>> 'entry';`,
    /// Systems Library `States.sysml`; spec42 Gap 43). Mutually exclusive with
    /// `action_reference`, which the reference form keeps.
    pub declared_name: Option<String>,
    /// `: Action` typing on the declaration form (`do action doAction : Action :>> 'do';`).
    pub type_name: Option<QualifiedReferenceId>,
    /// `:>> target` redefinition on the declaration form.
    pub redefines: Option<Node<crate::ast::SubsettingRelationship>>,
    /// `assign`/`send`/`accept` effect written directly under the keyword (`entry assign
    /// counter.count := 0;`, spec42 `assignment_test`), mirroring [`Transition::effect`]
    /// (spec42 Gap 43).
    pub effect: Option<TransitionEffect>,
    pub body: StateDefBody,
}

/// Then (initial state): `then` state-path `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThenStmt {
    pub state_reference: QualifiedReferenceId,
}

/// Final state: `final` name `;` or `final state` name `;`
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FinalState {
    pub state_name: String,
    pub name_span: Span,
}

impl PartialEq for FinalState {
    fn eq(&self, other: &Self) -> bool {
        self.state_name == other.state_name
    }
}

/// State usage: `OccurrenceUsagePrefix` subset `state` name (`:` type)? (`:>`/` :>>` …)? body.
/// GH-45: `direction`/`is_derived`/`is_individual` cover the rest of the BNF `RefPrefix`/
/// `OccurrenceUsagePrefix` slots that real usage justified (see
/// `crate::parser::state::state_usage`'s doc comment) — `constant` and portion kind
/// (`snapshot`/`timeslice`) are still not covered, with zero real-usage evidence found on state
/// usages to justify them.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StateUsage {
    /// Leading direction (`in`/`out`/`inout`).
    pub direction: Option<InOut>,
    /// Leading `derived` keyword.
    pub is_derived: bool,
    /// Leading `abstract` keyword.
    pub is_abstract: bool,
    /// Leading `ref` keyword — reference feature usage (`ref state …`).
    pub is_reference: bool,
    /// Leading `individual` keyword (after `ref`, per `OccurrenceUsagePrefix` order).
    pub is_individual: bool,
    pub name: String,
    /// Referenced state path when an `exhibit path` is represented in an occurrence body.
    pub state_reference: Option<QualifiedReferenceId>,
    pub type_name: Option<QualifiedReferenceId>,
    /// Structured typing clause when a `:` target was written.
    pub typing: Option<Node<TypingRelationship>>,
    /// Multiplicity after the type, when present.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// Optional `subsets` / `:>` clause.
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// Optional `redefines` / `:>>` clause.
    pub redefines: Option<Node<SubsettingRelationship>>,
    pub body: StateDefBody,
    pub membership: Membership,
}

/// Transition: `transition` name [`first` source [`accept` trigger]] [`if` guard] [`do` effect] `then` target body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transition {
    pub name: Option<String>,
    /// If omitted, form is `transition name then target;`.
    pub source: Option<Node<Expression>>,
    /// When `first` is present on a transition, the source state is also an initial state.
    pub is_initial: bool,
    /// Structured or shorthand accept trigger after `first` source.
    pub accept: Option<TransitionAccept>,
    pub guard: Option<Node<Expression>>,
    pub effect: Option<TransitionEffect>,
    pub target: Node<Expression>,
    /// `TransitionUsage = 'transition' … ActionBody`. Was a `ConnectBody` marker whose brace form
    /// was parsed by `advance_to_closing_brace` and kept nothing.
    pub body: ActionDefBody,
}

// ---------------------------------------------------------------------------
// Constraints & Calculations
// ---------------------------------------------------------------------------
