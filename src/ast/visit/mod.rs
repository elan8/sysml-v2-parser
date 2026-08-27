//! The owning boundary for *structural* traversal of the typed AST.
//!
//! The contract this module owns, stated precisely:
//!
//! - generic structural traversal -- walking the tree to reach nodes of some kind -- uses this
//!   inventory, and nothing hand-writes its own recursive descent for that purpose;
//! - policy-complete consumers, which must produce something for every node (emitters, the
//!   semantic snapshot projection), keep their own exhaustive matches, because a compile error
//!   is a stronger contract for them than a default walk;
//! - a consumer must not rely on default recursion where a newly added node would require a
//!   decision. Default recursion is correct when the rule genuinely applies per node kind, and
//!   wrong when the rule has to be complete. Semantic lowering is in the second category.
//!
//! Within the first category there is exactly one inventory of AST types, shared by both
//! traversal directions:
//!
//! - [`Visitor`] borrows the tree and is used by read-only consumers;
//! - [`mutable::VisitorMut`] borrows it mutably for in-place transformations that must not
//!   rebuild nodes (for example erasing spans before a structural comparison).
//!
//! Both traits are expanded from the same `ast_traversal!` macro body in
//! `src/ast/visit/inventory.rs`, so a new variant or field cannot be handled in one direction
//! and silently forgotten in the other.
//!
//! # Exhaustiveness contract
//!
//! The inventory destructures every struct without `..` and matches every enum without `_`.
//! Adding a field or a variant to a type reachable from [`RootNamespace`](crate::ast::RootNamespace)
//! therefore fails to compile until the traversal makes a deliberate decision about it. That
//! compile failure is the point: it is the single place where "does this new syntax carry
//! children, spans, references, or authored text?" has to be answered, and every consumer
//! built on the visitor inherits the answer.
//!
//! Malformed ([`ParseErrorNode`](crate::ast::ParseErrorNode)) and unsupported
//! ([`UnsupportedGrammarNode`](crate::ast::UnsupportedGrammarNode)) members are ordinary
//! members of their scope enums, so they are visited in their original tree position like any
//! other element rather than being skipped or hoisted.
//!
//! # When a consumer belongs here
//!
//! Use the visitor when the rule is stated per node *kind* and the tree shape is incidental to
//! it: "a malformed member reports a diagnostic", "an import target validates its delimiter
//! provenance", "every span is erased". Those rules should not have to be restated for each
//! scope that can own the member, and the default walk means a scope gaining that member later
//! is covered for free.
//!
//! Emitters and the semantic snapshot projection are deliberately *not* built this way. Every
//! node needs output there, so their own exhaustive `match` is the stronger contract: a new
//! variant must fail to compile rather than fall through to a default walk that would quietly
//! render nothing. The visitor's opt-in overrides suit consumers whose rules cover specific
//! node kinds, not consumers that must have an answer for all of them.
//!
//! The same caution applies to downstream consumers. Semantic lowering -- building a resolved
//! model, where a new node implies an ownership, scope, or diagnostic decision -- should keep
//! its own exhaustive matches over the scope enums for exactly this reason. What the visitor is
//! good for downstream is context-free work over the whole tree: collecting ranges and
//! references, provenance and source-fidelity audits, and checking that every reachable node is
//! accounted for.
//!
//! # Traversal order
//!
//! Traversal is pre-order and follows declaration order: a node's own span is visited first,
//! then its fields in the order they are declared, then each element of an ordered body in
//! source order. Consumers that produce ordered output (diagnostics, for example) can rely on
//! this.
//!
//! # Using the visitor
//!
//! Implement only the methods you care about; the defaults walk children.
//!
//! ```
//! use sysml_v2_parser::ast::visit::{walk_part_def, Visitor};
//! use sysml_v2_parser::ast::{Node, PartDef};
//! use sysml_v2_parser::parse;
//!
//! #[derive(Default)]
//! struct CountPartDefs {
//!     seen: usize,
//! }
//!
//! impl Visitor for CountPartDefs {
//!     fn visit_part_def(&mut self, node: &Node<PartDef>) {
//!         self.seen += 1;
//!         walk_part_def(self, node);
//!     }
//! }
//!
//! let root = parse("package P { part def Outer { part def Inner; } }").unwrap();
//! let mut counter = CountPartDefs::default();
//! counter.visit_root_namespace(&root);
//! assert_eq!(counter.seen, 2);
//! ```

#[macro_use]
mod inventory;

mod immutable;
pub mod mutable;

pub use immutable::*;
