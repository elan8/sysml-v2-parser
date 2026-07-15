//! Standalone feature-chain AST type (PAR-004 item 6).
//!
//! A feature chain (BNF `FeatureChain`) is a dot-separated sequence of feature names, e.g.
//! `engine.fuelCmdPort.flowRate`, as distinct from a `::`-qualified type/package name such as
//! `ISQ::mass`. Today the parser folds both shapes into plain strings or into
//! [`crate::ast::Expression::FeatureRef`] / [`crate::ast::Expression::MemberAccess`] chains with
//! no dedicated node. This type exists so relationship targets (typing, subsetting, redefinition,
//! etc.) can eventually carry a real feature chain instead of a raw string.
//!
//! **Not yet wired into `Expression`/`src/parser/expr.rs`.** PAR-005 (complete expression AST)
//! is expected to adopt this type for `path_expression` parsing once it lands; until then this is
//! a standalone, reusable building block that relationship parsing can call directly.

use crate::ast::core::Span;

/// A dot-separated feature chain, e.g. `engine.fuelCmdPort.flowRate` -> `["engine",
/// "fuelCmdPort", "flowRate"]`. Distinct from a `::`-qualified name (see
/// [`crate::parser::lex::qualified_name`]), which separates namespace/type segments rather than
/// feature-access segments.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeatureChain {
    /// Ordered feature names, e.g. `["engine", "fuelCmdPort", "flowRate"]`. Always has at least
    /// one element (a chain of length 1 is just a bare feature reference).
    pub segments: Vec<String>,
    /// Span of the whole chain, from the first segment to the last.
    pub span: Span,
}

impl FeatureChain {
    /// Whether this chain is a single, unchained feature reference (no `.` segments).
    pub fn is_single(&self) -> bool {
        self.segments.len() == 1
    }
}
