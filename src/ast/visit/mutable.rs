//! Mutable expansion of the shared traversal inventory.
//!
//! This is the transformation half of the traversal boundary. It exists for consumers that
//! genuinely have to change nodes in place -- currently span normalization for structural
//! comparison -- and it is expanded from the same inventory as the read-only
//! [`Visitor`](super::Visitor), so neither direction can drift from the grammar independently.
//!
//! A mutable visitor may rewrite provenance and authored text, but it cannot invent or drop
//! tree structure: the walk visits exactly the nodes that are there. Transformations that need
//! to add or remove members belong in the parser, not here.

use crate::ast::*;

ast_traversal!(
    /// Mutably borrowing visitor over every AST node reachable from
    /// [`RootNamespace`](crate::ast::RootNamespace).
    ///
    /// Mirrors [`Visitor`](super::Visitor) method for method; only the borrow is different.
    VisitorMut,
    mut
);
