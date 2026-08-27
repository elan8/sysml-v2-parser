//! Stack headroom for the recursive-descent body grammar.
//!
//! Only one construct recurses without bound in the parser: a brace body whose members are
//! themselves declarations with brace bodies. Expressions use an explicit heap stack, and
//! [`MAX_SYNTAX_NESTING`](super::parse::MAX_SYNTAX_NESTING) caps how deep body nesting may go
//! before parsing starts. Headroom is therefore acquired where the recursion happens -- once per
//! nesting level, only when the remaining stack is genuinely short -- rather than by mapping a
//! worst-case segment at the parse entry point on behalf of documents that never nest.

/// Stack a single body nesting level may consume before the next level must find new headroom.
///
/// One level costs a few kilobytes in release builds and roughly two orders of magnitude more in
/// unoptimized builds, where every combinator frame survives. 1 MiB covers a debug-build level with
/// wide margin while staying far below a default thread stack, so flat and shallowly nested
/// documents -- the overwhelming majority, and every keystroke in an editor session -- never
/// allocate at all.
const NESTED_BODY_RED_ZONE: usize = 1024 * 1024;

/// Segment size acquired when a nesting level cannot fit in [`NESTED_BODY_RED_ZONE`].
///
/// Sized so a single segment carries the remaining levels of a `MAX_SYNTAX_NESTING`-deep document
/// even in a debug build, making growth a once-per-deep-document event rather than a per-level one.
const NESTED_BODY_GROWTH: usize = 16 * 1024 * 1024;

/// Run one brace-body nesting level, acquiring a new stack segment only if this level would not
/// otherwise fit.
pub(crate) fn with_nested_body_stack<R>(f: impl FnOnce() -> R) -> R {
    stacker::maybe_grow(NESTED_BODY_RED_ZONE, NESTED_BODY_GROWTH, f)
}
