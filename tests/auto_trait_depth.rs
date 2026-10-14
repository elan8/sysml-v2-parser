//! Consumers prove auto traits for AST content without raising `recursion_limit`.
//!
//! This crate keeps rustc's default limit of 128, like a downstream crate that stores parsed
//! documents or expressions. Proving these bounds structurally would walk the AST type graph and
//! overflow the limit; `Node`'s explicit implementations (`src/ast/auto_traits.rs`) stop each
//! proof at the first `Node`. This test fails to compile if that stops holding.

use std::panic::{RefUnwindSafe, UnwindSafe};
use std::sync::Arc;

use sysml_v2_parser::ast::{
    Expression, Node, ParsedDocument, RootNamespace, StateDefBody, SubsettingRelationship,
    TransitionEffect,
};

fn assert_auto_traits<T: Send + Sync + Unpin + UnwindSafe + RefUnwindSafe>() {}

/// A consumer type in the shape of a lowering memo: shared documents plus retained expressions.
#[allow(dead_code)]
struct RetainedFacts {
    documents: Vec<Arc<ParsedDocument>>,
    expressions: Vec<Node<Expression>>,
    pending: Option<Box<Expression>>,
}

/// A consumer type in the shape of a lowering view: borrows into a parsed body.
#[allow(dead_code)]
struct BorrowedView<'a> {
    redefines: Option<&'a Node<SubsettingRelationship>>,
    effect: Option<&'a TransitionEffect>,
    body: &'a StateDefBody,
    root: &'a RootNamespace,
}

#[test]
fn stored_ast_content_has_every_auto_trait_at_the_default_recursion_limit() {
    assert_auto_traits::<ParsedDocument>();
    assert_auto_traits::<Arc<ParsedDocument>>();
    assert_auto_traits::<Expression>();
    assert_auto_traits::<Node<Expression>>();
    assert_auto_traits::<RetainedFacts>();
    assert_auto_traits::<BorrowedView<'static>>();
}
