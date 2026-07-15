//! Typed feature-value AST (parser work item 1, post-PAR-006).
//!
//! [`AttributeDef::value`](crate::ast::AttributeDef::value),
//! [`AttributeUsage::value`](crate::ast::AttributeUsage::value),
//! [`PartUsage::value`](crate::ast::PartUsage::value), and
//! [`RefDecl::value`](crate::ast::RefDecl::value) used to be a bare `Option<Node<Expression>>`,
//! even though the grammar (BNF `FeatureValue`) distinguishes five legal forms that carry real
//! semantic difference: bare `= expr` (bind), bare `:= expr` (assign), `default = expr`,
//! `default := expr`, and bare `default expr`. The parser already matched all five forms via
//! `alt()` but discarded which branch matched before calling `expression()`, so the operator
//! (`=` vs `:=`) and the presence of the `default` keyword were both lost.
//!
//! This type keeps that information: [`FeatureValueKind`] distinguishes the `=`/`:=` operator,
//! and `is_default` separately records whether the clause was introduced by the `default`
//! keyword (which is legal independently of the operator -- `default expr` with no explicit
//! operator is also a legal form).

use crate::ast::core::{Expression, Node, Span};

/// Which operator introduced a [`FeatureValue`]: `=` (bind) or `:=` (assign). Bare `default expr`
/// with no explicit operator is treated as [`FeatureValueKind::Bind`], matching `=`'s semantics
/// (the default form of binding).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeatureValueKind {
    /// `=`, e.g. `attribute mass = 10 [kg];`. Also used for bare `default expr` with no explicit
    /// operator, since that form's semantics match `=`.
    Bind,
    /// `:=`, e.g. `attribute mass := 10 [kg];`.
    Assign,
}

/// A feature's value clause (BNF `FeatureValue`): `= expr` | `:= expr` | `default = expr` |
/// `default := expr` | `default expr`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeatureValue {
    /// Whether the operator was `=` or `:=` (or, for bare `default expr`, `=`'s implied kind).
    pub kind: FeatureValueKind,
    /// Whether the `default` keyword prefixed the clause. Independent of `kind`: `default =`,
    /// `default :=`, and bare `default expr` all set this `true`.
    pub is_default: bool,
    pub expression: Node<Expression>,
    /// Span of the whole value clause, from the start of the operator/`default` keyword through
    /// the end of the expression.
    pub span: Span,
}

/// Equality ignores `span`, matching this crate's other span-bearing types' convention:
/// hand-built expected ASTs in tests don't need real source spans.
impl PartialEq for FeatureValue {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.is_default == other.is_default && self.expression == other.expression
    }
}

impl Eq for FeatureValue {}
