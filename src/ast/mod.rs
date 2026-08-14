//! Abstract syntax tree types for SysML v2 textual notation.

mod core;
mod kerml_fallback;

pub use core::*;
pub use kerml_fallback::*;
mod behavior;
mod body;
mod common;
mod feature_value;
mod membership;
mod package;
#[cfg(feature = "serde")]
mod provenance_visit;
mod qualified_reference;
mod requirement;
mod root;
pub mod semantic_format;
mod structure;
mod view;
pub mod visit;

pub use behavior::*;
pub use body::*;
pub use common::*;
pub use feature_value::*;
pub use membership::*;
pub use package::*;
pub use qualified_reference::*;
pub use requirement::*;
pub use root::*;
pub use semantic_format::*;
pub use structure::*;
pub use view::*;

// ---------------------------------------------------------------------------
// Span normalization for structural comparison
// ---------------------------------------------------------------------------

/// Erases source provenance in place so two documents can be compared for structural equality.
///
/// Spans are provenance, not grammar: the same construct authored at a different offset is the
/// same construct. This consumer walks the [owning traversal boundary](crate::ast::visit) and
/// replaces every span -- node spans and the spans recorded directly on declarations alike --
/// with [`Span::dummy`], leaving everything else untouched.
///
/// What it deliberately does *not* erase is the *presence* of an optional span. Whether a
/// construct recorded a `language` clause or a declaration name at all is a grammatical fact,
/// so `Some`/`None` still compares.
pub struct SpanNormalizer;

impl visit::mutable::VisitorMut for SpanNormalizer {
    fn visit_span(&mut self, span: &mut Span) {
        *span = Span::dummy();
    }
}

impl RootNamespace {
    /// Returns a copy with every source span replaced by [`Span::dummy`], so that comparing two
    /// documents compares their grammar rather than where it was authored.
    ///
    /// Use it when the same content is expected from two different sources -- a parse of the
    /// original text versus a parse of its emitted form, or the strict versus editor parse entry
    /// points -- rather than when comparing two parses of the identical input, which already
    /// have identical spans.
    pub fn normalize_for_test_comparison(&self) -> Self {
        use visit::mutable::VisitorMut as _;
        let mut normalized = self.clone();
        SpanNormalizer.visit_root_namespace(&mut normalized);
        normalized
    }
}
