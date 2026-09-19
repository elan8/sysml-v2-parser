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
/// unoptimized builds, where every combinator frame survives. This isn't uniform across member
/// kinds, though: a level whose only member is a `doc` comment -- `identification`, `locale`, and
/// `comment_body` are each their own combinator chain -- costs measurably more than a level with a
/// plain nested declaration, and `individual` bodies with a redefinition/multi-specialization
/// header (`individual x : T :>> f { ... }`) cost more again than a bare `part`. 1 MiB was sized
/// for the general case and, combined with a costly header plus a `doc`-only body six levels deep
/// (`sysml/src/examples/Vehicle Example/VehicleIndividuals.sysml` -- real, spec-legal content,
/// nowhere near `MAX_SYNTAX_NESTING`), still starved on a 2 MiB thread in a debug build before the
/// next level's own check could run: this function is only called once per nesting level, at that
/// level's own entry, so it can't see a level's *remaining* work (an expensive header already
/// parsed, an expensive member still to come) at the moment it checks. 2 MiB restores the same
/// wide margin the original 1 MiB gave the general case, verified against every file in the OMG
/// SysML-v2-Release corpus (`sysml/src/examples`, `sysml/src/validation`, `sysml.library`,
/// `kerml/src/examples`) on a 2 MiB thread. Flat and shallowly nested documents -- the
/// overwhelming majority, and every keystroke in an editor session -- still never allocate at all.
const NESTED_BODY_RED_ZONE: usize = 2 * 1024 * 1024;

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
