//! Span, Node, Expression, and shared AST traits.

use super::{QualifiedReferenceId, ReferenceSeparator};

/// Source location: byte offset, line, column, and length in the source file.
/// Line and column are **1-based**. Use [`crate::ast::ParsedDocument::range`] when deriving
/// navigation or diagnostic ranges; a span alone cannot correctly calculate a multiline end.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Span {
    pub offset: usize,
    pub line: u32,
    pub column: usize,
    pub len: usize,
}

impl Span {
    /// Dummy span for tests or synthetic nodes (offset 0, line 1, column 1, len 0).
    pub fn dummy() -> Self {
        Self {
            offset: 0,
            line: 1,
            column: 1,
            len: 0,
        }
    }

    /// The smallest span containing both `self` and `other`, including any source between them.
    ///
    /// Used where one AST fact is written as several separate source fragments -- a
    /// subsetting-family clause repeated in one usage header, for instance -- so the merged fact
    /// still points at every byte the author wrote for it rather than at the first fragment only.
    pub fn covering(&self, other: &Self) -> Self {
        let (first, second) = if self.offset <= other.offset {
            (self, other)
        } else {
            (other, self)
        };
        let end = second.offset.saturating_add(second.len);
        Self {
            offset: first.offset,
            line: first.line,
            column: first.column,
            len: end.saturating_sub(first.offset),
        }
    }

    /// Legacy single-line projection. Prefer [`crate::ast::ParsedDocument::range`], which uses the
    /// canonical document-owned source index and handles multiline spans.
    #[deprecated(note = "use ParsedDocument::range so multiline ranges use canonical source data")]
    pub fn to_lsp_range(&self) -> (u32, u32, u32, u32) {
        let start_line = self.line.saturating_sub(1);
        let start_char = self.column.saturating_sub(1);
        let end_char = start_char.saturating_add(self.len);
        (start_line, start_char as u32, start_line, end_char as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::Span;

    #[test]
    fn span_dummy() {
        let s = Span::dummy();
        assert_eq!(s.offset, 0);
        assert_eq!(s.line, 1);
        assert_eq!(s.column, 1);
        assert_eq!(s.len, 0);
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Node<T> {
    pub span: Span,
    pub value: T,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> Eq for Node<T> {}

impl<T> Node<T> {
    pub fn new(span: Span, value: T) -> Self {
        Self { span, value }
    }
}

impl<T> std::ops::Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

/// Trait for generic access to node source span (e.g. visitors).
pub trait AstNode {
    fn span(&self) -> Span;
}

impl<T> AstNode for Node<T> {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

/// Classified binary operator for semantic diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BinaryOperator {
    Eq,
    Ne,
    StrictEq,
    StrictNe,
    Lt,
    Le,
    Gt,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    Pow,
    And,
    Or,
    Xor,
    Implies,
    Range,
    BitOr,
    BitAnd,
    /// KerML null-coalescing operator `??` (BNF `NullCoalescingExpression`), e.g.
    /// `collection->reduce '+' ?? zero` (Kernel Function Library `DataFunctions.kerml`).
    NullCoalesce,
    /// Unclassified or extension operator; retains source token.
    Other(String),
}

impl BinaryOperator {
    pub fn from_token(token: &str) -> Self {
        match token {
            "==" => Self::Eq,
            "!=" => Self::Ne,
            "===" => Self::StrictEq,
            "!==" => Self::StrictNe,
            "<" => Self::Lt,
            "<=" => Self::Le,
            ">" => Self::Gt,
            ">=" => Self::Ge,
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            "%" => Self::Mod,
            "^" => Self::Pow,
            "**" => Self::Exp,
            "&&" | "and" => Self::And,
            "||" | "or" => Self::Or,
            "xor" => Self::Xor,
            "implies" => Self::Implies,
            "??" => Self::NullCoalesce,
            ".." => Self::Range,
            "|" => Self::BitOr,
            "&" => Self::BitAnd,
            other => Self::Other(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Eq => "==",
            Self::Ne => "!=",
            Self::StrictEq => "===",
            Self::StrictNe => "!==",
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Mod => "%",
            Self::Pow => "^",
            Self::Exp => "**",
            Self::And => "&&",
            Self::Or => "||",
            Self::Xor => "xor",
            Self::Implies => "implies",
            Self::NullCoalesce => "??",
            Self::Range => "..",
            Self::BitOr => "|",
            Self::BitAnd => "&",
            Self::Other(s) => s.as_str(),
        }
    }
}

/// KerML type-check operator (`istype`, `hastype`, `as`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypeCheckKind {
    Istype,
    Hastype,
    As,
}

/// Classified unary operator.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
    BitNot,
    Other(String),
}

impl UnaryOperator {
    pub fn from_token(token: &str) -> Self {
        match token {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "not" => Self::Not,
            "~" => Self::BitNot,
            other => Self::Other(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Not => "not",
            Self::BitNot => "~",
            Self::Other(s) => s.as_str(),
        }
    }
}

/// A grammar-owned `SequenceExpressionList`: the comma-separated operand list shared by
/// `SequenceExpression`, `BracketExpression`, and `IndexExpression` (KerML textual BNF
/// 8.2.5.8.2). `comma_before` is absent only on the first element; an optional final comma is
/// retained separately because the grammar admits `OwnedExpression ','?`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SequenceExpressionList {
    pub elements: Vec<SequenceExpressionElement>,
    pub trailing_comma_span: Option<Span>,
}

/// One expression in a [`SequenceExpressionList`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SequenceExpressionElement {
    pub comma_before: Option<Span>,
    pub expression: Node<Expression>,
}

/// Expression: literals, feature refs, member access, index, bracket/unit, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    LiteralInteger(i64),
    LiteralReal(String),
    LiteralString(String),
    LiteralBoolean(bool),
    /// Single name or qualified name.
    FeatureRef(QualifiedReferenceId),
    /// base.member (e.g. engine.fuelCmdPort).
    MemberAccess {
        base: Box<Node<Expression>>,
        member: QualifiedReferenceId,
        separator: ReferenceSeparator,
    },
    /// `base#(operands)`, KerML `IndexExpression`.
    Index {
        base: Box<Node<Expression>>,
        hash_span: Span,
        open_paren_span: Span,
        operands: Box<Node<SequenceExpressionList>>,
        close_paren_span: Span,
    },
    /// `base[operands]`, KerML `BracketExpression`. The operands are ordinary typed
    /// expressions: a unit-looking `SI::mm` remains a source-backed qualified reference and
    /// `N * m` remains a binary expression, rather than being copied into a unit string.
    Bracket {
        base: Box<Node<Expression>>,
        open_bracket_span: Span,
        operands: Box<Node<SequenceExpressionList>>,
        close_bracket_span: Span,
    },
    /// Binary infix operation e.g. `a >= b * c`, `x / y`.
    BinaryOp {
        op: BinaryOperator,
        left: Box<Node<Expression>>,
        right: Box<Node<Expression>>,
    },
    /// Unary prefix: + - ~ not
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Node<Expression>>,
    },
    /// Function-like invocation, e.g. `ComputeMargin(a, b)`.
    Invocation {
        callee: Box<Node<Expression>>,
        args: Vec<Argument>,
    },
    /// Metadata classification: `@Metaclass` (e.g. `@SysML::PartUsage`).
    Classification {
        metaclass: QualifiedReferenceId,
    },
    /// Reflective meta cast: `expr meta Metaclass` (e.g. `userActions meta SysML::Usage`).
    MetaCast {
        base: Box<Node<Expression>>,
        metaclass: QualifiedReferenceId,
    },
    /// Type test: `expr istype Type`, `expr hastype Type`, or `expr as Type`.
    TypeCheck {
        kind: TypeCheckKind,
        operand: Option<Box<Node<Expression>>>,
        type_name: QualifiedReferenceId,
    },
    /// Select expression: `base.?selector`.
    Select {
        base: Box<Node<Expression>>,
        selector: QualifiedReferenceId,
    },
    /// Collect expression: `base.**selector`.
    Collect {
        base: Box<Node<Expression>>,
        selector: QualifiedReferenceId,
    },
    /// KerML null or empty sequence ().
    Null,
    /// `(operands)`, KerML `SequenceExpression`. A singleton is still a sequence production;
    /// grouping and comma-list spelling therefore share one source-backed representation.
    Sequence {
        open_paren_span: Span,
        operands: Box<Node<SequenceExpressionList>>,
        close_paren_span: Span,
    },
    /// Constructor/instantiation expression: `new Type(...)` (KerML `ConstructorExpression`,
    /// BNF 8.2.5.8.3). `type_name` is an arena-backed qualified type reference, consistent with
    /// [`Expression::TypeCheck`], [`Expression::MetaCast`], and
    /// [`Expression::Classification`]. `new Foo::Bar(...)` names a type, not a dotted feature
    /// access path.
    Constructor {
        type_name: QualifiedReferenceId,
        args: Vec<Argument>,
    },
    /// Multi-segment dotted feature-chain reference, e.g. `engine.fuelCmdPort.flowRate`.
    /// The arena view preserves each `.`/`::` separator and segment span; a single, unchained
    /// name stays [`Expression::FeatureRef`].
    FeatureChainRef(QualifiedReferenceId),
    /// KerML collection-operator invocation via `->`, e.g. `xs->collect(f)`, `xs->select(p)`,
    /// `xs->size()`, `xs->includes(x)` (PAR-005 item 2). Distinguished from a generic
    /// [`Expression::Invocation`] so the specific operator is recoverable from the AST without
    /// string-matching the callee name.
    CollectionOp {
        op: CollectionOperator,
        base: Box<Node<Expression>>,
        args: Vec<Argument>,
        /// Structured BodyExpression form `->forAll { in ref item : T; predicate }`.
        /// Parenthesized invocation leaves this `None`.
        brace_body: Option<Box<Node<CollectionOperatorBody>>>,
        /// True for the KerML dot shorthand spellings `x.{...}` (collect) and `x.?{...}`
        /// (select), so emission renders the authored form instead of `->collect`/`->select`.
        dot_shorthand: bool,
    },
    /// Metadata-access expression: `expr.metadata` (KerML `MetadataAccessExpression`, BNF
    /// 8.2.5.8.3: `ElementReferenceMember '.' 'metadata'`). Distinct from [`Expression::Classification`]
    /// (`@Metaclass`, tests whether an element is classified by a metaclass) and
    /// [`Expression::MetaCast`] (`expr meta Metaclass`, reflective cast) -- this accesses the
    /// metadata Feature/Element object itself for the referenced element.
    MetadataAccess(Box<Node<Expression>>),
    /// Conditional: `if <test> ? <then> else <else>` (KerML ConditionalExpression; validation
    /// `15_04`, `7b`).
    Conditional {
        test: Box<Node<Expression>>,
        then_expr: Box<Node<Expression>>,
        else_expr: Box<Node<Expression>>,
    },
    /// Extent expression: `all QualifiedName` (KerML; validation `10b`).
    Extent {
        target: QualifiedReferenceId,
    },
    /// Standalone KerML `BodyExpression` used as a primary expression, e.g. the pin
    /// initializers `in whileTest default {true}` / `in untilTest default {false}` (Systems
    /// Library `Actions.sysml`). Shares [`CollectionOperatorBody`]'s `{ parameters* result? }`
    /// shape -- both render the same KerML `ExpressionBody` production -- but stands on its
    /// own instead of following a `->op` collection operator.
    BodyExpr(Box<Node<CollectionOperatorBody>>),
}

/// Structured KerML `BodyExpression` supplied to a collection operator.
///
/// The enclosing [`Node`] spans the complete `{ ... }` form. Exact delimiter and declaration
/// token spans are retained for navigation and diagnostics without scanning the source.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectionOperatorBody {
    pub open_brace_span: Span,
    /// Documentation written before the parameters, e.g. `alternatives->minimize { doc /* For a
    /// MinimizeObjective, the best value is the minimum one. */ ... }`
    /// (`sysml.library/Domain Libraries/Analysis/TradeStudies.sysml:88`). A body expression is a
    /// feature body, so it may be documented like any other.
    pub doc: Option<Box<Node<super::DocComment>>>,
    pub parameters: Vec<Node<CollectionOperatorParameter>>,
    pub result: Option<Box<Node<Expression>>>,
    pub close_brace_span: Span,
}

/// One `UsageDeclaration` parameter in a collection-operator body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectionOperatorParameter {
    /// Direction keyword and its exact authored span. `None` for the undirected form, e.g. `p2`
    /// in `vertices->exists{p2 : Point; ...}` (`sysml.library/Domain Libraries/Geometry/
    /// ShapeItems.sysml:72`) -- a body-expression parameter declares a feature, and a feature
    /// need not be directed.
    pub direction: Option<Node<super::InOut>>,
    /// Exact `ref` keyword span when the parameter is a reference feature.
    pub reference_keyword_span: Option<Span>,
    /// The grammar-owned identification and complete feature-specialization header.
    pub declaration: Node<super::UsageDeclaration>,
    /// How the parameter is terminated: `;` or its own brace body. `in ref a { doc /* ... */ }`
    /// (`TradeStudies.sysml:162`) documents the parameter instead of ending it with `;`.
    pub terminator: CollectionOperatorParameterTerminator,
}

/// What closes a [`CollectionOperatorParameter`] declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CollectionOperatorParameterTerminator {
    /// `p2 : Point;`
    Semicolon { span: Span },
    /// `in ref a { doc /* ... */ }` -- the parameter's own body, holding its documentation.
    Body {
        open_brace_span: Span,
        doc: Option<Box<Node<super::DocComment>>>,
        close_brace_span: Span,
    },
}

/// Move every direct `Node<Expression>` child out of `expr`, replacing each with a cheap
/// non-recursive placeholder, and push it onto `out`. Used by [`Drop for Expression`](Expression)
/// to unwind arbitrarily deep trees (e.g. `((((...))))` or `f(g(h(i(...))))`, which the iterative
/// expression parser now happily accepts instead of stack-overflowing) with an explicit heap `Vec`
/// instead of the native call stack, which Rust's default derived drop glue would otherwise use one
/// frame of per nesting level.
fn take_expression_children(expr: &mut Expression, out: &mut Vec<Node<Expression>>) {
    fn take_box(slot: &mut Box<Node<Expression>>) -> Node<Expression> {
        *std::mem::replace(slot, Box::new(Node::new(Span::dummy(), Expression::Null)))
    }

    match expr {
        Expression::MemberAccess { base, .. }
        | Expression::MetadataAccess(base)
        | Expression::MetaCast { base, .. }
        | Expression::Select { base, .. }
        | Expression::Collect { base, .. } => {
            out.push(take_box(base));
        }
        Expression::Index { base, operands, .. } | Expression::Bracket { base, operands, .. } => {
            out.push(take_box(base));
            out.extend(
                std::mem::take(&mut operands.value.elements)
                    .into_iter()
                    .map(|element| element.expression),
            );
        }
        Expression::BinaryOp { left, right, .. } => {
            out.push(take_box(left));
            out.push(take_box(right));
        }
        Expression::UnaryOp { operand, .. } => {
            out.push(take_box(operand));
        }
        Expression::Invocation { callee, args } => {
            out.push(take_box(callee));
            out.extend(std::mem::take(args).into_iter().map(|arg| arg.value));
        }
        Expression::BodyExpr(body) => {
            if let Some(result) = body.value.result.take() {
                out.push(*result);
            }
        }
        Expression::CollectionOp {
            base,
            args,
            brace_body,
            ..
        } => {
            out.push(take_box(base));
            out.extend(std::mem::take(args).into_iter().map(|arg| arg.value));
            if let Some(body) = brace_body {
                if let Some(result) = body.value.result.take() {
                    out.push(*result);
                }
            }
        }
        Expression::Conditional {
            test,
            then_expr,
            else_expr,
        } => {
            out.push(take_box(test));
            out.push(take_box(then_expr));
            out.push(take_box(else_expr));
        }
        Expression::Constructor { args, .. } => {
            out.extend(std::mem::take(args).into_iter().map(|arg| arg.value));
        }
        Expression::Sequence { operands, .. } => {
            out.extend(
                std::mem::take(&mut operands.value.elements)
                    .into_iter()
                    .map(|element| element.expression),
            );
        }
        Expression::TypeCheck { operand, .. } => {
            if let Some(boxed) = operand.take() {
                out.push(*boxed);
            }
        }
        Expression::LiteralInteger(_)
        | Expression::LiteralReal(_)
        | Expression::LiteralString(_)
        | Expression::LiteralBoolean(_)
        | Expression::FeatureRef(_)
        | Expression::Classification { .. }
        | Expression::Null
        | Expression::FeatureChainRef(_)
        | Expression::Extent { .. } => {}
    }
}

impl Drop for Expression {
    fn drop(&mut self) {
        let mut pending: Vec<Node<Expression>> = Vec::new();
        take_expression_children(self, &mut pending);
        while let Some(mut node) = pending.pop() {
            take_expression_children(&mut node.value, &mut pending);
            // `node` drops here: its children were already moved out above, so this is a shallow,
            // non-recursive drop no matter how deep the original tree was.
        }
    }
}

/// A single invocation/constructor argument: positional or named (KerML `ArgumentList`,
/// BNF 8.2.5.8.3: `PositionalArgumentList | NamedArgumentList`), e.g. `F(a, b)` (positional) vs.
/// `F(x = a, y = b)` (named, via `NamedArgument: ParameterRedefinition '=' ArgumentValue`).
/// PAR-005 item 5: named arguments are real, used syntax in the SysML textual notation (e.g.
/// `new RiskLevel(probability = LevelEnum::low)` in the Systems Library) and previously caused
/// the whole enclosing declaration to fall into parser recovery because only positional
/// expressions were accepted inside `(...)`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Argument {
    /// `Some(parameter)` for a named argument (the redefined parameter reference to the left of
    /// `=`); `None` for a positional argument.
    pub parameter: Option<QualifiedReferenceId>,
    pub value: Node<Expression>,
}

/// Known KerML/SysML collection-operator names reachable via `base->op(...)` (PAR-005 item 2).
/// Not exhaustive of every Kernel/Systems Library function -- `Other` preserves any arrow-invoked
/// name this list doesn't special-case, so no arrow invocation is ever lost or misclassified as a
/// plain member call.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CollectionOperator {
    Collect,
    Select,
    SelectOne,
    Size,
    IsEmpty,
    NotEmpty,
    Includes,
    Including,
    Excludes,
    Excluding,
    ExcludingAt,
    ExcludingOnce,
    Equals,
    ForAll,
    Exists,
    Sum,
    Sort,
    Filter,
    Reduce,
    /// Any other arrow-invoked name not special-cased above, e.g. `->minimize`, `->maximize`.
    Other(String),
}

impl CollectionOperator {
    pub fn from_name(name: &str) -> Self {
        match name {
            "collect" => Self::Collect,
            "select" => Self::Select,
            "selectOne" => Self::SelectOne,
            "size" => Self::Size,
            "isEmpty" => Self::IsEmpty,
            "notEmpty" => Self::NotEmpty,
            "includes" => Self::Includes,
            "including" => Self::Including,
            "excludes" => Self::Excludes,
            "excluding" => Self::Excluding,
            "excludingAt" => Self::ExcludingAt,
            "excludingOnce" => Self::ExcludingOnce,
            "equals" => Self::Equals,
            "forAll" => Self::ForAll,
            "exists" => Self::Exists,
            "sum" => Self::Sum,
            "sort" => Self::Sort,
            "filter" => Self::Filter,
            "reduce" => Self::Reduce,
            other => Self::Other(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Collect => "collect",
            Self::Select => "select",
            Self::SelectOne => "selectOne",
            Self::Size => "size",
            Self::IsEmpty => "isEmpty",
            Self::NotEmpty => "notEmpty",
            Self::Includes => "includes",
            Self::Including => "including",
            Self::Excludes => "excludes",
            Self::Excluding => "excluding",
            Self::ExcludingAt => "excludingAt",
            Self::ExcludingOnce => "excludingOnce",
            Self::Equals => "equals",
            Self::ForAll => "forAll",
            Self::Exists => "exists",
            Self::Sum => "sum",
            Self::Sort => "sort",
            Self::Filter => "filter",
            Self::Reduce => "reduce",
            Self::Other(s) => s.as_str(),
        }
    }
}

/// Multiplicity bounds, e.g. `[1..*]`, `[0..1]`, `[3]` (PAR-004/PAR-003 item 5).
///
/// A bare bound like `[3]` means `lower == upper == Some(3)`. An unbounded `*` (as in `[1..*]` or
/// bare `[*]`) is represented as `None` for that side.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Multiplicity {
    /// Lower bound expression, e.g. `1` in `[1..*]`. `None` when the lower bound is unbounded
    /// (bare `[*]` with no explicit lower bound).
    pub lower: Option<Box<Node<Expression>>>,
    /// Upper bound expression, e.g. `*` renders as `None` (unbounded); `10` in `[1..10]` is
    /// `Some(10)`.
    pub upper: Option<Box<Node<Expression>>>,
    /// Span of the whole `[...]` fragment, including the brackets.
    pub span: Span,
}

/// Equality ignores `span`, matching `Node<T>`'s convention elsewhere in this crate: hand-built
/// expected ASTs in tests don't need to reproduce real source spans to compare equal.
impl PartialEq for Multiplicity {
    fn eq(&self, other: &Self) -> bool {
        self.lower == other.lower && self.upper == other.upper
    }
}

impl Eq for Multiplicity {}

/// Whether a [`TypingRelationship`] is a `:` typing/definition relationship or a `:>`
/// subclassification/specialization relationship (PAR-004 item 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypingKind {
    /// `:` / `defined by` / `typed by` — this feature is typed by the target.
    Typing,
    /// `:>` / `specializes` — this definition/usage specializes (subclassifies) the target.
    Subclassification,
}

/// A typing or subclassification relationship target, e.g. the `ISQ::mass` in `attribute mass :
/// ISQ::mass;` (typing) or the `Vehicle` in `part def Car :> Vehicle;` (subclassification)
/// (PAR-004 item 1, folding in PAR-003's conjugation concept from item 4).
///
/// The target is a list of document-local [`QualifiedReferenceId`]s rather than a joined string:
/// a `:` / `:>` clause can name more than one comma-separated target (e.g. `:> Base, Other`), and
/// the shared arena keeps every target's `::`- vs. `.`-separated segments distinct. Almost always
/// has exactly one element -- use [`TypingRelationship::first_target`] for that common case.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypingRelationship {
    /// The type/supertype target(s), e.g. `[ISQ::mass]`. Always has at least one element.
    pub target: Vec<QualifiedReferenceId>,
    pub kind: TypingKind,
    /// Span of the whole relationship fragment (operator/keyword through target), when known.
    pub span: Span,
    /// True when the target was written with a leading `~` (conjugated typing/definition
    /// identity), e.g. `~PortConjugate` in `port p : ~PortConjugate;`. The `~` is stripped from
    /// `target` once captured here.
    pub is_conjugated: bool,
    /// True for a relationship the parser infers rather than one explicitly written in source.
    /// Always `false` today — nothing in this parser currently produces implied relationships —
    /// but the field exists so a future implied-relationship producer doesn't need another AST
    /// migration.
    pub is_implied: bool,
    /// Which spelling the author wrote for the relationship operator. Emission renders this
    /// spelling instead of canonicalizing `specializes` to `:>` / `defined by` to `:`.
    pub spelling: TypingSpelling,
}

/// The authored spelling of a [`TypingRelationship`]'s operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypingSpelling {
    /// The symbolic operator: `:` for typing, `:>` for subclassification.
    Operator,
    /// The `specializes` keyword (subclassification).
    Specializes,
    /// The `defined by` keyword pair (typing).
    DefinedBy,
    /// The `typed by` keyword pair (typing).
    TypedBy,
}

/// Equality ignores `span`, matching `Node<T>`'s span-ignoring convention: where a relationship
/// was authored is provenance.
///
/// `spelling` is compared. `specializes B` and `:> B` do name the same relationship, but the
/// spelling is what the author wrote and what emission reproduces
/// ([`crate::emit`] matches on it), so excluding it from equality meant a formatter that swapped
/// one keyword for the other would pass every whole-AST comparison in the suite.
impl PartialEq for TypingRelationship {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target
            && self.kind == other.kind
            && self.is_conjugated == other.is_conjugated
            && self.is_implied == other.is_implied
            && self.spelling == other.spelling
    }
}

impl Eq for TypingRelationship {}

impl TypingRelationship {
    /// The single target for the overwhelmingly common case of a `:`/`:>` clause naming exactly
    /// one type. Returns the first target when a rare comma-separated multi-target clause is
    /// present, and `None` only if `target` is unexpectedly empty.
    pub fn first_target(&self) -> Option<QualifiedReferenceId> {
        self.target.first().copied()
    }
}

/// Which subsetting-family clause a [`SubsettingRelationship`] came from (PAR-004 item 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SubsettingKind {
    /// `:>` / `subsets`.
    Subsets,
    /// `::>` / `references` (reference subsetting).
    References,
    /// `:>>` / `redefines`.
    Redefines,
    /// `=>` / `crosses` (cross subsetting).
    Crosses,
    /// `intersects` (KerML `Intersecting`).
    Intersects,
}

/// A subsetting-family relationship target: subsetting, reference subsetting, redefinition, or
/// cross subsetting, e.g. the `rearWheel` in `subsets wheel = rearWheel[1]` (PAR-004 item 2).
///
/// Kept as a separate `Option<Node<SubsettingRelationship>>` field per clause kind on each owning
/// struct (not collapsed into one field) because not every struct supports all four clause kinds
/// (e.g. `ExhibitState` only has `redefines`), and a struct can have more than one of these
/// clauses present at once (e.g. both `subsets` and `redefines`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubsettingRelationship {
    /// The subsetted/redefined/crossed target(s), e.g. `[rearWheel]`. A clause can name more than
    /// one comma-separated target; almost always has exactly one element -- use
    /// [`SubsettingRelationship::first_target`] for that common case.
    pub target: Vec<QualifiedReferenceId>,
    pub kind: SubsettingKind,
    /// Span of the whole relationship fragment (operator/keyword through target).
    pub span: Span,
    /// True for a relationship the parser infers rather than one explicitly written in source.
    /// Always `false` today, matching [`TypingRelationship::is_implied`]'s rationale.
    pub is_implied: bool,
}

/// Equality ignores `span`, matching [`TypingRelationship`]'s convention.
impl PartialEq for SubsettingRelationship {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target
            && self.kind == other.kind
            && self.is_implied == other.is_implied
    }
}

impl SubsettingRelationship {
    /// The single target for the overwhelmingly common case of a subsetting-family clause naming
    /// exactly one target. Returns the first target when a rare comma-separated multi-target
    /// clause is present, and `None` only if `target` is unexpectedly empty.
    pub fn first_target(&self) -> Option<QualifiedReferenceId> {
        self.target.first().copied()
    }
}

impl Eq for SubsettingRelationship {}

/// A connector/interface endpoint, wrapping the endpoint's path expression with its own span
/// (PAR-004 item 3). Distinguishes an endpoint from an arbitrary standalone
/// [`Expression`]/[`Node<Expression>`] appearing elsewhere in the AST, even though today the
/// endpoint's own `span` and the inner `Node<Expression>`'s span are identical (the expression
/// itself already carries a real span from `path_expression` parsing).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConnectionEnd {
    pub expression: Node<Expression>,
    /// Endpoint multiplicity (§6 G24), as in `connect [0..1] a.p1 to [1] b.p2;` from the OMG spec
    /// Annex `7a1-Variant Configuration - General Concept-a.sysml`. `None` for the (far more
    /// common) unadorned endpoint.
    pub multiplicity: Option<Node<Multiplicity>>,
    pub span: Span,
}

/// Equality ignores `span`, matching [`TypingRelationship`]'s convention.
impl PartialEq for ConnectionEnd {
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression && self.multiplicity == other.multiplicity
    }
}

impl Eq for ConnectionEnd {}

impl Expression {
    /// Whether this expression node is a literal Boolean.
    pub fn is_boolean_literal(&self) -> bool {
        matches!(self, Self::LiteralBoolean(_))
    }

    /// Whether this expression is a metadata `@Metaclass` classification.
    pub fn is_classification(&self) -> bool {
        matches!(self, Self::Classification { .. })
    }

    /// Whether this expression is a KerML type test (`istype` / `hastype` / `as`).
    pub fn is_type_check(&self) -> bool {
        matches!(self, Self::TypeCheck { .. })
    }

    /// Whether a binary operator is a comparison.
    pub fn binary_op_is_comparison(op: &BinaryOperator) -> bool {
        matches!(
            op,
            BinaryOperator::Eq
                | BinaryOperator::Ne
                | BinaryOperator::StrictEq
                | BinaryOperator::StrictNe
                | BinaryOperator::Lt
                | BinaryOperator::Le
                | BinaryOperator::Gt
                | BinaryOperator::Ge
        )
    }

    /// Whether a binary operator is logical (`and` / `or` / `xor` / `implies`).
    pub fn binary_op_is_logical(op: &BinaryOperator) -> bool {
        matches!(
            op,
            BinaryOperator::And
                | BinaryOperator::Or
                | BinaryOperator::Xor
                | BinaryOperator::Implies
        )
    }
}
