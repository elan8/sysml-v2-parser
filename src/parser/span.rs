//! Parser input type and span extraction for source locations.

use crate::ast::{
    Node, QualifiedReferenceArena, QualifiedReferenceArenaBuilder,
    QualifiedReferenceArenaCheckpoint, QualifiedReferenceId, ReferenceSegment, Span,
};
use nom_locate::LocatedSpan;
use std::cell::RefCell;

/// Mutable state owned for the duration of one document parse.
///
/// The nom input itself remains cheap to copy: it carries a [`ParseContextRef`] pointing back to
/// this owner, while the arena builder uses interior mutability to collect references from parser
/// combinators. Once parsing is complete, [`finish`](Self::finish) turns the builder into the
/// immutable arena stored in the parsed-document envelope.
#[derive(Debug, Default)]
pub(crate) struct ParseContext {
    qualified_references: RefCell<QualifiedReferenceArenaBuilder>,
}

impl ParseContext {
    pub(crate) fn new() -> Self {
        Self {
            qualified_references: RefCell::new(QualifiedReferenceArenaBuilder::new()),
        }
    }

    /// Create a location-aware parser input backed by this parse context.
    pub(crate) fn input<'a>(&'a self, source: &'a [u8]) -> Input<'a> {
        LocatedSpan::new_extra(source, ParseContextRef { owner: self })
    }

    /// Finish the document-local qualified-reference arena after all parser inputs are dropped.
    pub(crate) fn finish(self) -> QualifiedReferenceArena {
        self.qualified_references.into_inner().finish()
    }
}

/// Copyable handle carried as [`LocatedSpan`]'s extra data.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ParseContextRef<'a> {
    owner: &'a ParseContext,
}

impl ParseContextRef<'_> {
    /// Mark the current arena length before a parser branch performs speculative allocation.
    pub(crate) fn reference_checkpoint(self) -> QualifiedReferenceArenaCheckpoint {
        self.owner.qualified_references.borrow().checkpoint()
    }

    /// Discard reference identities allocated by a parser branch that subsequently backtracked.
    pub(crate) fn rollback_references(self, checkpoint: QualifiedReferenceArenaCheckpoint) {
        self.owner
            .qualified_references
            .borrow_mut()
            .rollback(checkpoint);
    }

    /// Add a fully parsed semantic reference to the document arena.
    ///
    /// Arena exhaustion is reported as `None`; callers translate it into their parser's ordinary
    /// error type so adversarial input cannot panic the parser.
    pub(crate) fn add_reference(
        self,
        is_absolute: bool,
        span: Span,
        segments: impl IntoIterator<Item = ReferenceSegment>,
    ) -> Option<QualifiedReferenceId> {
        self.owner
            .qualified_references
            .borrow_mut()
            .add_reference(is_absolute, span, segments)
            .ok()
    }

    /// Test-only bridge for resolving an ID against the parser input that allocated it.
    #[cfg(test)]
    pub(crate) fn reference_span(self, id: QualifiedReferenceId) -> Option<Span> {
        self.owner.qualified_references.borrow().reference_span(id)
    }
}

/// Parser input: source bytes with location tracking and a document-local arena context.
pub type Input<'a> = LocatedSpan<&'a [u8], ParseContextRef<'a>>;

/// Test-only convenience for the many focused parser unit tests that need an arena context.
#[cfg(test)]
pub(crate) fn test_input(text: &str) -> Input<'_> {
    let context: &'static ParseContext = Box::leak(Box::new(ParseContext::new()));
    context.input(text.as_bytes())
}

/// Build a Span from the start and rest inputs (the consumed region).
pub fn span_from_to(start: Input<'_>, rest: Input<'_>) -> Span {
    let len = start.fragment().len().saturating_sub(rest.fragment().len());
    Span {
        offset: start.location_offset(),
        line: start.location_line(),
        column: start.get_column(),
        len,
    }
}

/// Wrap a parsed value in a Node using the span from start to rest.
pub fn node_from_to<T>(start: Input<'_>, rest: Input<'_>, value: T) -> Node<T> {
    Node::new(span_from_to(start, rest), value)
}

/// Run a parser and return the value together with the span of the consumed input.
/// Use this to capture sub-spans for semantic tokens (e.g. name, type reference).
pub fn with_span<'a, F, O, E>(
    mut f: F,
) -> impl FnMut(Input<'a>) -> nom::IResult<Input<'a>, (Span, O), E>
where
    E: nom::error::ParseError<Input<'a>>,
    F: FnMut(Input<'a>) -> nom::IResult<Input<'a>, O, E>,
{
    move |input: Input<'a>| {
        let start = input;
        let (rest, value) = f(input)?;
        Ok((rest, (span_from_to(start, rest), value)))
    }
}

#[cfg(test)]
mod tests {
    use super::{span_from_to, Input, ParseContext};
    use crate::ast::Span;
    use nom::bytes::complete::tag;
    use nom::error::Error;
    use nom::Parser;

    #[test]
    fn span_from_to_consumed_region() {
        let context = ParseContext::new();
        let bytes = b"package Foo;" as &[u8];
        let start = context.input(bytes);
        let (rest, _) = tag::<_, Input<'_>, Error<Input<'_>>>(&b"package"[..])
            .parse(start)
            .unwrap();
        let span = span_from_to(start, rest);
        assert_eq!(
            span,
            Span {
                offset: 0,
                line: 1,
                column: 1,
                len: 7,
            },
            "span should cover consumed 'package' (7 bytes)"
        );
    }
}
