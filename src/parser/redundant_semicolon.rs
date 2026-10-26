//! Report a `;` written directly after a member's closing `}`.
//!
//! `part def A { ... };` is grammatical: `DefaultReferenceUsage = RefPrefix Usage` (SysML
//! 8.2.2.6.3) has every slot optional and `UsageBody` may be `;`, so the trailing `;` parses as a
//! second, empty, anonymous member. It is almost always a statement terminator written by habit
//! rather than an intended feature, so it is reported as a warning with its exact `;` span.
//! The empty usage remains valid syntax and does not cause strict parsing to fail.
//!
//! The rule is stated over typed spans only. A brace body reports its `}` through
//! [`Visitor::visit_body_braces`]; an empty member is a [`DefaultReferenceUsage`] with no authored
//! slot and a semicolon body. The member is redundant when the nearest token ending at or before
//! its `;` is such a `}`: nothing but trivia separates them, and the `;` is not the first member
//! of a body (whose nearest token is that body's `{`).

use std::collections::BTreeSet;

use crate::ast::visit::{walk_default_reference_usage, Visitor};
use crate::ast::{AttributeBody, DefaultReferenceUsage, Node, RootNamespace, Span};
use crate::error::{DiagnosticCategory, DiagnosticSeverity, ParseError};
use crate::parser::diagnostic_catalog;

pub(crate) fn collect_redundant_semicolon_warnings(root: &RootNamespace) -> Vec<ParseError> {
    let mut scan = Scan::default();
    scan.visit_root_namespace(root);
    scan.empty_members
        .iter()
        .filter(|semicolon| {
            scan.token_ends
                .range(..=semicolon.offset)
                .next_back()
                .is_some_and(|end| scan.close_brace_ends.contains(end))
        })
        .map(|semicolon| {
            ParseError::new(
                "possible redundant ';' after a closing '}': parsed as an empty anonymous usage",
            )
            .with_location(semicolon.offset, semicolon.line, semicolon.column)
            .with_length(semicolon.len.max(1))
            .with_code(diagnostic_catalog::REDUNDANT_SEMICOLON_AFTER_BODY)
            .with_severity(DiagnosticSeverity::Warning)
            .with_category(DiagnosticCategory::ParseError)
            .with_found(";")
            .with_suggestion("Remove the ';' if the empty usage was unintended.")
        })
        .collect()
}

#[derive(Default)]
struct Scan {
    /// End offset of every non-empty span in the tree. Zero-width spans (an absent visibility
    /// keyword, for example) mark a position rather than a token, so they are left out.
    token_ends: BTreeSet<usize>,
    /// End offset of every brace body's `}`.
    close_brace_ends: BTreeSet<usize>,
    /// The `;` of every member that authored nothing but that `;`, in traversal order.
    empty_members: Vec<Span>,
}

impl Visitor for Scan {
    fn visit_span(&mut self, span: &Span) {
        if span.len > 0 {
            self.token_ends.insert(span.offset + span.len);
        }
    }

    fn visit_body_braces<E>(&mut self, _open: &Span, _elements: &[Node<E>], close: &Span) {
        self.close_brace_ends.insert(close.offset + close.len);
    }

    fn visit_default_reference_usage(&mut self, node: &Node<DefaultReferenceUsage>) {
        if let Some(semicolon) = empty_member_semicolon(&node.value) {
            self.empty_members.push(*semicolon);
        }
        walk_default_reference_usage(self, node);
    }
}

/// The `;` of a keyword-less usage that authored no slot at all, or `None` when it authored any.
fn empty_member_semicolon(usage: &DefaultReferenceUsage) -> Option<&Span> {
    let DefaultReferenceUsage {
        prefix,
        name,
        short_name,
        typing,
        subsets,
        redefines,
        references,
        crosses,
        intersects,
        value,
        multiplicity,
        multiplicity_modifiers,
        typing_span,
        membership,
        body,
    } = usage;
    let authored = prefix.is_authored()
        || name.is_some()
        || short_name.is_some()
        || typing.is_some()
        || subsets.is_some()
        || redefines.is_some()
        || references.is_some()
        || crosses.is_some()
        || intersects.is_some()
        || value.is_some()
        || multiplicity.is_some()
        || multiplicity_modifiers.is_authored()
        || typing_span.is_some()
        || membership.visibility.is_some();
    match body {
        AttributeBody::Semicolon { semicolon_span } if !authored => Some(semicolon_span),
        AttributeBody::Semicolon { .. } | AttributeBody::Brace { .. } => None,
    }
}
