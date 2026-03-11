//! Parser input type and span extraction for source locations.

use crate::ast::{Node, Span};
use nom::InputLength;
use nom_locate::LocatedSpan;

/// Parser input: bytes with location tracking (offset, line, column).
pub type Input<'a> = LocatedSpan<&'a [u8]>;

/// Build a Span from the start and rest inputs (the consumed region).
pub fn span_from_to(start: Input<'_>, rest: Input<'_>) -> Span {
    let len = start.input_len().saturating_sub(rest.input_len());
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
