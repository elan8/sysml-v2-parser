//! Deserialization-time validation of the facts a serialized document cannot be trusted to carry.
//!
//! A wire document arrives as data, so accepting it is validation rather than reconstruction: every
//! qualified-reference identity has to resolve in the arena that travelled with it, and every AST
//! position that owns exact delimiter provenance has to still agree with the source it claims to
//! index. Both checks run in one pass over the [owning traversal boundary](crate::ast::visit), so a
//! new AST position that carries an identity or a delimiter span is covered the moment the
//! traversal knows about it -- there is no separate list here to keep in step.

use super::visit::{walk_first_merge_brace_body, walk_import_target, Visitor};
use super::*;

fn span_end(span: &Span) -> usize {
    span.offset.saturating_add(span.len)
}

pub(super) fn validate_ast_provenance(document: &ParsedDocument) -> Result<(), String> {
    let mut validator = ProvenanceValidator {
        document,
        error: None,
        owners: Vec::new(),
    };
    validator.visit_root_namespace(&document.root);
    match validator.error {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

struct ProvenanceValidator<'a> {
    document: &'a ParsedDocument,
    /// The first failure wins; later checks are skipped so validation reports one stable reason.
    error: Option<String>,
    /// Spans of the nodes currently being walked, outermost first. A body's delimiters have to
    /// sit inside the declaration that owns them, and only the traversal knows which that is.
    owners: Vec<Span>,
}

impl ProvenanceValidator<'_> {
    fn check(&mut self, result: Result<(), String>) {
        if self.error.is_none() {
            self.error = result.err();
        }
    }

    /// A delimiter must lie within the declaration that owns it, which is the innermost node the
    /// traversal is inside.
    fn owned(&self, span: &Span, role: &str) -> Result<(), String> {
        let Some(owner) = self.owners.last() else {
            return Ok(());
        };
        if span.offset < owner.offset || span_end(span) > span_end(owner) {
            return Err(format!(
                "{role} at {} lies outside the declaration that owns it, {}..{}",
                span.offset,
                owner.offset,
                span_end(owner)
            ));
        }
        Ok(())
    }

    /// A delimiter span must lie inside the source, land on character boundaries, and contain
    /// exactly the token it claims.
    fn delimiter(&self, span: &Span, token: &str, role: &str) -> Result<(), String> {
        match self.document.source.slice(span) {
            Some(text) if text == token => Ok(()),
            Some(text) => Err(format!("{role} span covers {text:?} rather than {token:?}")),
            None => Err(format!(
                "{role} span at offset {} is not a valid slice of the document source",
                span.offset
            )),
        }
    }
}

impl Visitor for ProvenanceValidator<'_> {
    /// An identity is meaningful only against the arena it was serialized with.
    fn visit_qualified_reference(&mut self, reference: &QualifiedReferenceId) {
        if self.error.is_some() {
            return;
        }
        if !self.document.qualified_references.contains(*reference) {
            self.error = Some(
                qualified_reference::QualifiedReferenceValidationError::DanglingReference {
                    id: *reference,
                }
                .to_string(),
            );
        }
    }

    fn enter_node(&mut self, span: &Span) {
        self.owners.push(span.clone());
    }

    fn leave_node(&mut self, _span: &Span) {
        self.owners.pop();
    }

    /// A brace body claims two delimiter tokens, and claiming them is not enough: a wire document
    /// could point both at any other `{ ... }` pair and still spell them correctly. The pair has
    /// to be the one belonging to this body -- inside the declaration that owns it, in order, and
    /// wrapped around its own members.
    fn visit_body_braces<E>(&mut self, open: &Span, elements: &[Node<E>], close: &Span) {
        if self.error.is_some() {
            return;
        }
        self.check(self.delimiter(open, "{", "body open brace"));
        self.check(self.delimiter(close, "}", "body close brace"));
        self.check(self.owned(open, "body open brace"));
        self.check(self.owned(close, "body close brace"));
        if self.error.is_some() {
            return;
        }
        if span_end(open) > close.offset {
            self.error = Some(format!(
                "body open brace at {} does not precede its close brace at {}",
                open.offset, close.offset
            ));
            return;
        }
        for element in elements {
            if element.span.offset < span_end(open) || span_end(&element.span) > close.offset {
                self.error = Some(format!(
                    "body member at {} lies outside its own delimiters {}..{}",
                    element.span.offset,
                    open.offset,
                    span_end(close)
                ));
                return;
            }
        }
    }

    /// The semicolon form claims one token; the same reasoning applies.
    fn visit_body_semicolon(&mut self, semicolon: &Span) {
        if self.error.is_some() {
            return;
        }
        self.check(self.delimiter(semicolon, ";", "body semicolon"));
        self.check(self.owned(semicolon, "body semicolon"));
    }

    /// Import targets own ordered separator, wildcard, and recursion spans that must still line up
    /// with the source text and with the reference they name.
    fn visit_import_target(&mut self, target: &ImportTarget) {
        if self.error.is_some() {
            return;
        }
        let result = target
            .validate_provenance(&self.document.source, &self.document.qualified_references)
            .map_err(|error| error.to_string());
        self.check(result);
        walk_import_target(self, target);
    }

    /// A `first`/`merge`/`decide`/`join`/`fork` brace body records both delimiters explicitly.
    fn visit_first_merge_brace_body(&mut self, node: &Node<FirstMergeBraceBody>) {
        if self.error.is_some() {
            return;
        }
        let result = node
            .value
            .validate_provenance(&node.span, &self.document.source);
        self.check(result);
        walk_first_merge_brace_body(self, node);
    }
}
