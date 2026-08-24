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
    /// Byte offset at which each line starts, `line_starts[line - 1]` for the 1-indexed line
    /// `LocatedSpan` tracks. Built once per document by [`ParseContext::input`], so a column is
    /// one subtraction rather than `nom_locate`'s backward scan to the previous newline -- which
    /// is O(line length) per span and made every node on a long line cost the whole line.
    line_starts: RefCell<Vec<usize>>,
}

impl ParseContext {
    pub(crate) fn new() -> Self {
        Self {
            qualified_references: RefCell::new(QualifiedReferenceArenaBuilder::new()),
            line_starts: RefCell::new(Vec::new()),
        }
    }

    /// Create a location-aware parser input backed by this parse context.
    pub(crate) fn input<'a>(&'a self, source: &'a [u8]) -> Input<'a> {
        let mut line_starts = vec![0];
        line_starts.extend(memchr::memchr_iter(b'\n', source).map(|index| index + 1));
        *self.line_starts.borrow_mut() = line_starts;
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

/// The 1-indexed byte column of `input`'s position, as `LocatedSpan::get_column` defines it,
/// in O(1) from the document's line-start table.
pub(crate) fn column_of(input: &Input<'_>) -> usize {
    let line_starts = input.extra.owner.line_starts.borrow();
    match line_starts.get(input.location_line() as usize - 1) {
        Some(line_start) => input.location_offset() - line_start + 1,
        // Only reachable for an input not created by `ParseContext::input`; scan rather than
        // report a wrong column.
        None => input.get_column(),
    }
}

/// Run one complete parser production as an arena transaction.
///
/// Reference lexers allocate only after accepting a complete path, but a containing production
/// can still fail after that point (for example, an import target followed by a malformed body).
/// `nom` may then try another alternative or editor recovery. Rolling back here prevents IDs from
/// failed productions from becoming observable in the finished document arena.
pub(crate) fn reference_transaction<'a, O, E, F>(
    input: Input<'a>,
    parser: F,
) -> nom::IResult<Input<'a>, O, E>
where
    F: FnOnce(Input<'a>) -> nom::IResult<Input<'a>, O, E>,
{
    let checkpoint = input.extra.reference_checkpoint();
    let result = parser(input);
    if result.is_err() {
        input.extra.rollback_references(checkpoint);
    }
    result
}

/// Run a lookahead that only inspects the input, discarding whatever it allocated.
///
/// [`reference_transaction`] rolls back on failure, which is what a production wants. A dispatch
/// guard is different: it parses ahead purely to decide which arm to take, then the chosen arm
/// parses the same bytes again. Keeping the probe's entries would leave every reference in the
/// lookahead region allocated twice.
pub(crate) fn reference_probe<'a, T, F>(input: Input<'a>, probe: F) -> T
where
    F: FnOnce(Input<'a>) -> T,
{
    let checkpoint = input.extra.reference_checkpoint();
    let result = probe(input);
    input.extra.rollback_references(checkpoint);
    result
}

/// Test-only convenience for the many focused parser unit tests that need an arena context.
#[cfg(test)]
pub(crate) fn test_input(text: &str) -> Input<'_> {
    let context: &'static ParseContext = Box::leak(Box::new(ParseContext::new()));
    context.input(text.as_bytes())
}

/// `input` advanced by `skipped` bytes, which must lie within its fragment.
///
/// The lexer's trivia scanners already know how many bytes they consumed; this rebuilds the
/// location directly rather than going through `nom::Input::take_from`, whose generic slicing
/// path recomputes the consumed length and pays a call for every zero-length skip.
#[inline]
pub(crate) fn advance(input: Input<'_>, skipped: usize) -> Input<'_> {
    if skipped == 0 {
        return input;
    }
    let fragment = input.fragment();
    let (consumed, rest) = fragment.split_at(skipped);
    let newlines = memchr::memchr_iter(b'\n', consumed).count() as u32;
    // SAFETY: `rest` is the tail of `input`'s own fragment, so the byte offset and line count
    // carried alongside it describe exactly the position `nom_locate` would compute by slicing.
    unsafe {
        LocatedSpan::new_from_raw_offset(
            input.location_offset() + skipped,
            input.location_line() + newlines,
            rest,
            input.extra,
        )
    }
}

/// Build a Span from the start and rest inputs (the consumed region).
pub fn span_from_to(start: Input<'_>, rest: Input<'_>) -> Span {
    let len = start.fragment().len().saturating_sub(rest.fragment().len());
    Span {
        offset: start.location_offset(),
        line: start.location_line(),
        column: column_of(&start),
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
