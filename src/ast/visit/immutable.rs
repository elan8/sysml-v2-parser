//! Read-only expansion of the shared traversal inventory.

use crate::ast::*;

ast_traversal!(
    /// Borrowing visitor over every AST node reachable from
    /// [`RootNamespace`](crate::ast::RootNamespace).
    ///
    /// Implement the methods for the nodes a consumer cares about; every default implementation
    /// forwards to the matching `walk_*` function, which visits the node's span, its fields in
    /// declaration order, and its children in source order. Call the `walk_*` function from an
    /// override to keep descending.
    ///
    /// See the [module documentation](self) for the exhaustiveness contract.
    Visitor,
);
