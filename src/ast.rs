//! Abstract syntax tree types for SysML v2 textual notation.

/// Expression: literals, feature refs, member access, index, bracket/unit, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    LiteralInteger(i64),
    LiteralReal(String),
    LiteralString(String),
    LiteralBoolean(bool),
    /// Single name or qualified name.
    FeatureRef(String),
    /// base.member (e.g. engine.fuelCmdPort).
    MemberAccess(Box<Expression>, String),
    /// base#(index) e.g. frontWheel#(1).
    Index {
        base: Box<Expression>,
        index: Box<Expression>,
    },
    /// [unit] e.g. [kg].
    Bracket(Box<Expression>),
    /// value [unit] e.g. 1750 [kg].
    LiteralWithUnit {
        value: Box<Expression>,
        unit: Box<Expression>,
    },
}

/// Root of a SysML document: a sequence of package-level elements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootNamespace {
    pub elements: Vec<PackageBodyElement>,
}

/// Top-level element inside a namespace or package body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageBodyElement {
    Package(Package),
    Import(Import),
    PartDef(PartDef),
    PartUsage(PartUsage),
    PortDef(PortDef),
    InterfaceDef(InterfaceDef),
    AliasDef(AliasDef),
    AttributeDef(AttributeDef),
    ActionDef(ActionDef),
    ActionUsage(ActionUsage),
}

/// A package declaration: `package` Identification PackageBody
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub identification: Identification,
    pub body: PackageBody,
}

/// Identification: optional short name in `< >`, optional name.
/// BNF: ( '<' declaredShortName = NAME '>' )? ( declaredName = NAME )?
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identification {
    /// Short name inside `< ... >`, if present.
    pub short_name: Option<String>,
    /// Main declared name (may be quoted, e.g. '1a-Parts Tree').
    pub name: Option<String>,
}

/// Package body: either `;` or `{` PackageBodyElement* `}`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageBody {
    /// Semicolon form: no body elements.
    Semicolon,
    /// Brace form: list of body elements (may be empty).
    Brace {
        elements: Vec<PackageBodyElement>,
    },
}

/// Visibility for imports and members.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

/// Import: `private`? `import` `all`? QualifiedName (`::` `*`)? etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub visibility: Option<Visibility>,
    /// Whether this is a namespace import (Definitions::*) or membership (SI::kg).
    pub is_import_all: bool,
    /// Import target, e.g. "SI::kg" or "Definitions::*".
    pub target: String,
}

/// Part definition: `part def` Identification (`:>` specializes)? Body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartDef {
    pub identification: Identification,
    /// Supertype after `:>`, e.g. Some("Axle") for `part def FrontAxle :> Axle`.
    pub specializes: Option<String>,
    pub body: PartDefBody,
}

/// Body of a part definition: `;` or `{` PartDefBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartDefBody {
    Semicolon,
    Brace {
        elements: Vec<PartDefBodyElement>,
    },
}

/// Element inside a part definition body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartDefBodyElement {
    AttributeDef(AttributeDef),
    PortUsage(PortUsage),
}

/// Attribute definition: `attribute` name (`:>` type)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeDef {
    pub name: String,
    /// Type after `:>`, e.g. Some("ISQ::mass").
    pub typing: Option<String>,
    pub body: AttributeBody,
}

/// Body of an attribute (def or usage): `;` or `{` ... `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeBody {
    Semicolon,
    Brace,
}

/// Part usage: `part` name `:` type multiplicity? `ordered`? body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartUsage {
    pub name: String,
    /// Type after `:`, e.g. "Vehicle", "AxleAssembly".
    pub type_name: String,
    /// Multiplicity, e.g. Some("[2]").
    pub multiplicity: Option<String>,
    pub ordered: bool,
    /// Optional `subsets` feature and value expression, e.g. ("frontWheel", Some(Index(FeatureRef("frontWheel"), LiteralInteger(1)))).
    pub subsets: Option<(String, Option<Expression>)>,
    pub body: PartUsageBody,
}

/// Body of a part usage: `;` or `{` PartUsageBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartUsageBody {
    Semicolon,
    Brace {
        elements: Vec<PartUsageBodyElement>,
    },
}

/// Element inside a part usage body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartUsageBodyElement {
    AttributeUsage(AttributeUsage),
    PartUsage(Box<PartUsage>),
    PortUsage(PortUsage),
    Bind(Bind),
    InterfaceUsage(InterfaceUsage),
    Connect(Connect),
}

/// Attribute usage: `attribute` name `redefines`? value? body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeUsage {
    pub name: String,
    /// Redefines target, e.g. Some("Vehicle::mass").
    pub redefines: Option<String>,
    /// Value expression, e.g. LiteralWithUnit { value: LiteralInteger(1750), unit: Bracket(FeatureRef("kg")) }.
    pub value: Option<Expression>,
    pub body: AttributeBody,
}

// ---------------------------------------------------------------------------
// Port
// ---------------------------------------------------------------------------

/// Port definition: `port def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortDef {
    pub identification: Identification,
    pub body: PortDefBody,
}

/// Body of a port definition: `;` or `{` PortDefBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortDefBody {
    Semicolon,
    Brace {
        elements: Vec<PortDefBodyElement>,
    },
}

/// Element inside a port definition body (nested port usages).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortDefBodyElement {
    PortUsage(PortUsage),
}

/// Port usage: `port` name `:` type multiplicity? `:>` subsets? `redefines`? body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortUsage {
    pub name: String,
    pub type_name: Option<String>,
    pub multiplicity: Option<String>,
    /// Subsets feature and optional value expression.
    pub subsets: Option<(String, Option<Expression>)>,
    pub redefines: Option<String>,
    pub body: PortBody,
}

/// Body of a port usage: `;` or `{` PortUsage* `}` (nested ports).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortBody {
    Semicolon,
    Brace,
    /// Brace with nested port usages (e.g. port vehicleToRoadPort redefines ... { port left...; port right...; }).
    BraceWithPorts { elements: Vec<PortUsage> },
}

// ---------------------------------------------------------------------------
// Interface
// ---------------------------------------------------------------------------

/// Interface definition: `interface def` Identification body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceDef {
    pub identification: Identification,
    pub body: InterfaceDefBody,
}

/// Body of an interface definition: `;` or `{` InterfaceDefBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceDefBody {
    Semicolon,
    Brace {
        elements: Vec<InterfaceDefBodyElement>,
    },
}

/// Element inside an interface definition body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceDefBodyElement {
    EndDecl(EndDecl),
    RefDecl(RefDecl),
    ConnectStmt(ConnectStmt),
}

/// End declaration in interface def: `end` name `:` type `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndDecl {
    pub name: String,
    pub type_name: String,
}

/// Ref declaration in interface def: `ref` name `:` type body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefDecl {
    pub name: String,
    pub type_name: String,
    pub body: RefBody,
}

/// Body of a ref declaration: `;` or `{` ... `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefBody {
    Semicolon,
    Brace,
}

/// Connect statement in interface def or usage: `connect` from `to` to body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectStmt {
    pub from: Expression,
    pub to: Expression,
    pub body: ConnectBody,
}

/// Body of a connect statement: `;` or `{` ... `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectBody {
    Semicolon,
    Brace,
}

// ---------------------------------------------------------------------------
// Part usage body: bind, interface usage, connect
// ---------------------------------------------------------------------------

/// Bind: `bind` left `=` right (`;` or `{ }`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bind {
    pub left: Expression,
    pub right: Expression,
    /// Optional body after the bind (semicolon or brace); 3a fixture uses `bind x = y { }`.
    pub body: Option<ConnectBody>,
}

/// Interface usage: typed+connect or connection form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceUsage {
    /// `interface` `:Type`? `connect` from `to` to body; optional body with ref redefs.
    TypedConnect {
        interface_type: Option<String>,
        from: Expression,
        to: Expression,
        body: ConnectBody,
        body_elements: Vec<InterfaceUsageBodyElement>,
    },
    /// `interface` from `to` to body.
    Connection {
        from: Expression,
        to: Expression,
        body_elements: Vec<InterfaceUsageBodyElement>,
    },
}

/// Element in interface usage body (e.g. ref redefinition).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceUsageBodyElement {
    /// `ref` `:>>` name `=` value body.
    RefRedef { name: String, value: Expression, body: RefBody },
}

/// Connect at part usage level: `connect` from `to` to body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connect {
    pub from: Expression,
    pub to: Expression,
    pub body: ConnectBody,
}

// ---------------------------------------------------------------------------
// Alias
// ---------------------------------------------------------------------------

/// Alias definition: `alias` Identification `for` qualified_name body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasDef {
    pub identification: Identification,
    pub target: String,
    pub body: AliasBody,
}

/// Body of an alias definition: `;` or `{` ... `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AliasBody {
    Semicolon,
    Brace,
}

// ---------------------------------------------------------------------------
// Action (function-based behavior)
// ---------------------------------------------------------------------------

/// Action definition: `action def` Identification body (in/out params).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionDef {
    pub identification: Identification,
    pub body: ActionDefBody,
}

/// Body of an action definition: `;` or `{` InOutDecl* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionDefBody {
    Semicolon,
    Brace {
        elements: Vec<InOutDecl>,
    },
}

/// In/out parameter in action def: `in` name `:` type `;` or `out` name `:` type `;`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InOutDecl {
    pub direction: InOut,
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InOut {
    In,
    Out,
}

/// Action usage: `action` name `:` type_name (`accept` param_name `:` param_type)? body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionUsage {
    pub name: String,
    pub type_name: String,
    /// For accept form: (param_name, param_type).
    pub accept: Option<(String, String)>,
    pub body: ActionUsageBody,
}

/// Body of an action usage: `;` or `{` ActionUsageBodyElement* `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionUsageBody {
    Semicolon,
    Brace {
        elements: Vec<ActionUsageBodyElement>,
    },
}

/// Element inside an action usage body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionUsageBodyElement {
    InOutDecl(InOutDecl),
    Bind(Bind),
    Flow(Flow),
    FirstStmt(FirstStmt),
    MergeStmt(MergeStmt),
    ActionUsage(Box<ActionUsage>),
}

/// Flow: `flow` from `to` to body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flow {
    pub from: Expression,
    pub to: Expression,
    pub body: ConnectBody,
}

/// First/then control flow: `first` expr `then` expr body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstStmt {
    pub first: Expression,
    pub then: Expression,
    pub body: FirstMergeBody,
}

/// Merge: `merge` expr body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeStmt {
    pub merge: Expression,
    pub body: FirstMergeBody,
}

/// Body of first/merge: `;` or `{` ... `}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirstMergeBody {
    Semicolon,
    Brace,
}
