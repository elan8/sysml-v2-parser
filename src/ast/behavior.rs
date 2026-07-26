use super::common::{ConnectBody, DocComment, Identification, ParseErrorNode};
use super::membership::Membership;
use super::requirement::RequirementUsage;
use super::structure::{
    Annotation, Bind, DefinitionBody, MetadataAnnotation, MetadataKeywordUsage, Perform, RefDecl,
};
use crate::ast::core::{
    Expression, Multiplicity, Node, Span, SubsettingRelationship, TypingRelationship,
};

/// Action definition: `action def` Identification body (in/out params).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionDef {
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActionDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<ActionDefBodyElement>>,
    },
}

/// Element inside an action definition body.
// Body-element variants intentionally preserve their direct `Node<T>` AST shape. Boxing only
// `ThenAction` would make equivalent action constructs differ by containing context and introduce
// indirection throughout parser consumers solely to satisfy a size heuristic.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActionDefBodyElement {
    Error(Node<ParseErrorNode>),
    InOutDecl(Node<InOutDecl>),
    Doc(Node<DocComment>),
    Annotation(Node<Annotation>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
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
    IfStmt(Node<IfStmt>),
    StateUsage(Node<StateUsage>),
    ActionUsage(Box<Node<ActionUsage>>),
    Assign(Node<AssignStmt>),
    ForLoop(Node<ForLoop>),
    ThenAction(Node<ThenAction>),
    Decl(Node<ActionBodyDecl>),
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

/// Succession to an action usage: `then action ...`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThenAction {
    pub action: Node<ActionUsage>,
}

/// In/out parameter in action def: `in` name `:` type `;` or `out` name `:` type `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InOutDecl {
    pub direction: InOut,
    pub name: String,
    pub type_name: String,
    /// Optional default value: `= expr` initializer on in/out parameters.
    pub value: Option<Node<Expression>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InOut {
    In,
    Out,
    InOut,
}

/// Typed payload on accept/send control nodes: `accept name : Type` or `send name : Type`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PayloadClause {
    pub name: String,
    pub type_name: Option<String>,
    pub name_span: Span,
    pub type_span: Option<Span>,
}

/// Transition accept trigger: typed payload or shorthand expression (`accept StartPressed`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransitionAccept {
    /// `(payload, via)` — `via` is the optional `via <port>` clause after the trigger.
    Payload(PayloadClause, Option<Node<Expression>>),
    /// `(expr, via)` — `via` is the optional `via <port>` clause after the trigger.
    Shorthand(Node<Expression>, Option<Node<Expression>>),
}

/// Transition `do` effect: a structured action-usage form (SysML v2 `EffectBehaviorUsage`)
/// or a bare expression shorthand.
///
/// Examples: `do action powerUp : PowerUp;`, `do send new TimeoutSignal() via commPort`,
/// `do accept Ack via commPort`, `do assign x := y`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransitionEffect {
    /// `action` name (`:` type)? — perform an owned/named action.
    Perform {
        name: Option<String>,
        type_name: Option<String>,
    },
    /// `accept` payload (`:` type)? (`via` expr)?
    Accept {
        payload: Node<Expression>,
        type_name: Option<String>,
        via: Option<Node<Expression>>,
    },
    /// `send` payload (`:` type)? ((`via` expr)? (`to` expr)? | `to` expr)
    Send {
        payload: Node<Expression>,
        type_name: Option<String>,
        via: Option<Node<Expression>>,
        to: Option<Node<Expression>>,
    },
    /// `assign` lhs `:=` rhs
    Assign {
        lhs: Node<Expression>,
        rhs: Node<Expression>,
    },
    /// Bare expression shorthand (e.g. a reference to an existing action usage).
    Expression(Node<Expression>),
}

/// Action usage: `(abstract)? (ref)? action` name (`:` type)? (`[mult]`)? (`:>`/` :>>` …)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionUsage {
    /// Leading `abstract` keyword (Systems Library e.g. `abstract ref action performedActions`).
    pub is_abstract: bool,
    /// Leading `ref` keyword — reference feature usage rather than composite
    /// (`ref action …` inside a part body).
    pub is_reference: bool,
    pub name: String,
    pub type_name: String,
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
    /// For standalone `send param : Type` control-node statements.
    pub send: Option<PayloadClause>,
    pub body: ActionUsageBody,
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

/// Body of an action usage: `;` or `{` ActionUsageBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActionUsageBody {
    Semicolon,
    Brace {
        elements: Vec<Node<ActionUsageBodyElement>>,
    },
}

/// Element inside an action usage body.
// Keep the same direct-node representation as `ActionDefBodyElement`; see its size rationale.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActionUsageBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    Annotation(Node<Annotation>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
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
    IfStmt(Node<IfStmt>),
    StateUsage(Node<StateUsage>),
    ActionUsage(Box<Node<ActionUsage>>),
    Assign(Node<AssignStmt>),
    ForLoop(Node<ForLoop>),
    ThenAction(Node<ThenAction>),
    Decl(Node<ActionBodyDecl>),
}

/// A minimally-modeled declaration inside an action/behavior body (e.g. `attribute ...;`, `calc ...;`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionBodyDecl {
    pub keyword: String,
    pub text: String,
}

/// Flow definition: `flow def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlowDef {
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
    pub type_name: Option<String>,
    pub multiplicity: Option<Node<Multiplicity>>,
}

/// Flow usage: `flow` | `message` | `succession flow` with optional name, payload, and endpoints.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlowUsage {
    pub kind: FlowUsageKind,
    pub name: Option<String>,
    pub type_name: Option<String>,
    pub payload: Option<Node<PayloadFeature>>,
    pub from: Option<Node<Expression>>,
    pub to: Option<Node<Expression>>,
    pub body: DefinitionBody,
    pub membership: Membership,
}

/// First/then control flow: `first` expr `then` expr body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FirstStmt {
    pub first: Node<Expression>,
    pub then: Node<Expression>,
    pub body: FirstMergeBody,
}

/// Merge: `merge` expr body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MergeStmt {
    pub merge: Node<Expression>,
    pub body: FirstMergeBody,
}

/// Decision node: `decide` expr body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecisionStmt {
    pub decide: Node<Expression>,
    pub body: FirstMergeBody,
}

/// Join node: `join` expr body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JoinStmt {
    pub join: Node<Expression>,
    pub body: FirstMergeBody,
}

/// Fork node: `fork` expr body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForkStmt {
    pub fork: Node<Expression>,
    pub body: FirstMergeBody,
}

/// Body of first/merge: `;` or `{` ... `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FirstMergeBody {
    Semicolon,
    Brace,
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

/// If control node: `if` condition `{` thenBody `}` (`else` `{` elseBody `}`)?.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfStmt {
    pub condition: Node<Expression>,
    pub then_body: ActionDefBody,
    pub else_body: Option<ActionDefBody>,
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
    pub body: ConnectBody,
}

/// Allocation definition: `allocation def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AllocationDef {
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
    pub type_name: Option<String>,
    pub source: Option<Node<Expression>>,
    pub target: Option<Node<Expression>>,
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
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub body: StateDefBody,
    pub membership: Membership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StateDefBody {
    Semicolon,
    Brace {
        elements: Vec<Node<StateDefBodyElement>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StateDefBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    Annotation(Node<Annotation>),
    MetadataAnnotation(Node<MetadataAnnotation>),
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    Other(String),
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
    /// `ref` name `:` type body ÔÇô reference binding in state.
    Ref(Node<RefDecl>),
    RequirementUsage(Node<RequirementUsage>),
    StateUsage(Node<StateUsage>),
    Transition(Box<Node<Transition>>),
}

/// Entry action: `entry` (`;` or body).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntryAction {
    /// For `entry action name body` form; None for plain `entry` body.
    pub action_name: Option<String>,
    pub body: StateDefBody,
}

/// Do action: `do` (`;` or body).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DoAction {
    /// For `do action name body` form; None for plain `do` body.
    pub action_name: Option<String>,
    pub body: StateDefBody,
}

/// Exit action: `exit` (`;` or body).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExitAction {
    /// For `exit action name body` form; None for plain `exit` body.
    pub action_name: Option<String>,
    pub body: StateDefBody,
}

/// Then (initial state): `then` name `;`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThenStmt {
    pub state_name: String,
    pub name_span: Option<Span>,
}

/// Final state: `final` name `;` or `final state` name `;`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FinalState {
    pub state_name: String,
    pub name_span: Span,
}

/// State usage: `(abstract)? (ref)? state` name (`:` type)? (`:>`/` :>>` …)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StateUsage {
    /// Leading `abstract` keyword.
    pub is_abstract: bool,
    /// Leading `ref` keyword — reference feature usage (`ref state …`).
    pub is_reference: bool,
    pub name: String,
    pub type_name: Option<String>,
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
    pub body: ConnectBody,
}

// ---------------------------------------------------------------------------
// Constraints & Calculations
// ---------------------------------------------------------------------------
