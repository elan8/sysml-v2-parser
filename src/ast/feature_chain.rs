//! Standalone feature-chain AST type (PAR-004 item 6).
//!
//! A feature chain (BNF `FeatureChain`) is a dot-separated sequence of feature names, e.g.
//! `engine.fuelCmdPort.flowRate`, as distinct from a `::`-qualified type/package name such as
//! `ISQ::mass`.
//!
//! **Wired into `Expression` by PAR-005 item 3**: `path_expression` (`src/parser/expr.rs`, used
//! for `bind`/`connect`/`allocate` endpoints) now produces
//! [`crate::ast::Expression::FeatureChainRef`] for genuine multi-segment dotted chains instead of
//! folding them into nested [`crate::ast::Expression::MemberAccess`]. A single, unchained segment
//! still stays [`crate::ast::Expression::FeatureRef`]. The general `expression()` grammar's
//! postfix `.` chaining (used for value expressions, e.g. `a.b.c` inside an ordinary calc/
//! constraint body) intentionally still folds into `MemberAccess` -- it's interleaved with other
//! postfix operators (`(...)`, `#(...)`, `::`, `meta`, `->op(...)`) that a pure feature chain
//! doesn't carry, so it stays out of scope for this type.

use crate::ast::core::Span;

/// A dot-separated feature chain, e.g. `engine.fuelCmdPort.flowRate` -> `["engine",
/// "fuelCmdPort", "flowRate"]`. Distinct from a `::`-qualified name (see
/// [`crate::parser::lex::qualified_name`]), which separates namespace/type segments rather than
/// feature-access segments.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeatureChain {
    /// Ordered feature names, e.g. `["engine", "fuelCmdPort", "flowRate"]`. Always has at least
    /// one element (a chain of length 1 is just a bare feature reference).
    pub segments: Vec<String>,
    /// Span of the whole chain, from the first segment to the last.
    pub span: Span,
}

/// Equality ignores `span`, matching `Node<T>`'s convention elsewhere in this crate
/// (`src/ast/core.rs`): hand-built expected ASTs in tests don't need to reproduce real source
/// spans to compare equal. Without this, `FeatureChain` would be the only span-bearing type in
/// the AST whose span is *not* transparently ignored by `PartialEq`, since it's referenced from
/// [`crate::ast::Expression::FeatureChainRef`] as a plain field rather than wrapped in `Node<T>`.
impl PartialEq for FeatureChain {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments
    }
}

impl Eq for FeatureChain {}

impl FeatureChain {
    /// Whether this chain is a single, unchained feature reference (no `.` segments).
    pub fn is_single(&self) -> bool {
        self.segments.len() == 1
    }
}
