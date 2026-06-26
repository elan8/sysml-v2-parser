//! Span, Node, Expression, and shared AST traits.

/// Source location: byte offset, line, column, and length in the source file.
/// Line and column are **1-based**. Use [`Span::to_lsp_range`] for 0-based LSP ranges.
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

    /// LSP uses 0-based line and 0-based character. Returns (start_line, start_character, end_line, end_character).
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

/// Expression: literals, feature refs, member access, index, bracket/unit, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    LiteralInteger(i64),
    LiteralReal(String),
    LiteralString(String),
    LiteralBoolean(bool),
    /// Single name or qualified name.
    FeatureRef(String),
    /// base.member (e.g. engine.fuelCmdPort).
    MemberAccess(Box<Node<Expression>>, String),
    /// base#(index) e.g. frontWheel#(1).
    Index {
        base: Box<Node<Expression>>,
        index: Box<Node<Expression>>,
    },
    /// [unit] e.g. [kg].
    Bracket(Box<Node<Expression>>),
    /// value [unit] e.g. 1750 [kg].
    LiteralWithUnit {
        value: Box<Node<Expression>>,
        unit: Box<Node<Expression>>,
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
        args: Vec<Node<Expression>>,
    },
    /// Comma-separated sequence in parentheses, e.g. `(engine1, engine2)` for ordered composition values.
    Tuple(Vec<Node<Expression>>),
    /// Metadata classification: `@Metaclass` (e.g. `@SysML::PartUsage`).
    Classification {
        metaclass: String,
    },
    /// Reflective meta cast: `expr meta Metaclass` (e.g. `userActions meta SysML::Usage`).
    MetaCast {
        base: Box<Node<Expression>>,
        metaclass: String,
    },
    /// Type test: `expr istype Type`, `expr hastype Type`, or `expr as Type`.
    TypeCheck {
        kind: TypeCheckKind,
        operand: Option<Box<Node<Expression>>>,
        type_name: String,
    },
    /// Select expression: `base.?selector`.
    Select {
        base: Box<Node<Expression>>,
        selector: String,
    },
    /// Collect expression: `base.**selector`.
    Collect {
        base: Box<Node<Expression>>,
        selector: String,
    },
    /// KerML null or empty sequence ().
    Null,
}

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
