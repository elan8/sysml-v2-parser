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

pub(super) fn validate_ast_provenance(document: &ParsedDocument) -> Result<(), String> {
    let mut validator = ProvenanceValidator {
        document,
        error: None,
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
}

impl ProvenanceValidator<'_> {
    fn check(&mut self, result: Result<(), String>) {
        if self.error.is_none() {
            self.error = result.err();
        }
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

    /// A brace body claims two delimiter tokens. A deserialized document is data, so check that
    /// the spans still slice to the tokens they claim and that the body is not inside out.
    fn visit_body_braces(&mut self, open: &Span, close: &Span) {
        if self.error.is_some() {
            return;
        }
        self.check(self.delimiter(open, "{", "body open brace"));
        self.check(self.delimiter(close, "}", "body close brace"));
        if self.error.is_none() && open.offset >= close.offset {
            self.error = Some(format!(
                "body open brace at {} does not precede its close brace at {}",
                open.offset, close.offset
            ));
        }
    }

    /// The semicolon form claims one token; the same reasoning applies.
    fn visit_body_semicolon(&mut self, semicolon: &Span) {
        if self.error.is_some() {
            return;
        }
        self.check(self.delimiter(semicolon, ";", "body semicolon"));
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
