//! Packed, source-backed qualified references.
//!
//! References in the AST use [`QualifiedReferenceId`] instead of owning a joined string or a
//! vector of segments. The document owns one [`SourceStorage`] and one
//! [`QualifiedReferenceArena`]; resolving an ID returns a borrowed [`QualifiedReferenceView`]
//! without allocating.

use std::borrow::Cow;
use std::sync::OnceLock;

use super::Span;

/// Source text retained by a parsed document.
///
/// A leading UTF-8 byte-order mark is removed to match the parser's existing offset convention:
/// all AST spans are relative to the first byte after the BOM.
#[derive(Debug, Default)]
pub struct SourceStorage {
    text: String,
    /// Byte offset of the first byte of every line. Built once, on first positional query.
    line_starts: OnceLock<Vec<usize>>,
}

#[cfg(feature = "serde")]
impl serde::Serialize for SourceStorage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.text)
    }
}

/// A canonical source position. `offset` is a UTF-8 byte offset; line and column are 1-based and
/// columns count bytes, matching parser spans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePosition {
    pub offset: usize,
    pub line: u32,
    pub column: usize,
}

/// Half-open source range (`start..end`) derived from a parser span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceRange {
    pub start: SourcePosition,
    pub end: SourcePosition,
}

impl SourceStorage {
    pub fn new(mut source: String) -> Self {
        if source.starts_with('\u{FEFF}') {
            source.drain(..'\u{FEFF}'.len_utf8());
        }
        Self {
            text: source,
            line_starts: OnceLock::new(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Borrow the exact source covered by `span`, returning `None` for overflow, out-of-bounds,
    /// or non-UTF-8-boundary offsets.
    pub fn slice(&self, span: &Span) -> Option<&str> {
        let end = span.offset.checked_add(span.len)?;
        self.text.get(span.offset..end)
    }

    /// Convert a UTF-8 byte offset into the parser's canonical 1-based line/byte-column position.
    ///
    /// The document owns a lazily initialized line index, so repeated consumers do not rescan the
    /// source or maintain a competing newline interpretation. Returns `None` for an out-of-bounds
    /// offset or an offset inside a UTF-8 code point. The one-past-the-end offset is valid.
    pub fn position_at(&self, offset: usize) -> Option<SourcePosition> {
        if offset > self.text.len() || !self.text.is_char_boundary(offset) {
            return None;
        }
        let starts = self.line_starts.get_or_init(|| {
            let mut starts = Vec::new();
            starts.push(0);
            starts.extend(
                self.text
                    .bytes()
                    .enumerate()
                    .filter_map(|(index, byte)| (byte == b'\n').then_some(index + 1)),
            );
            starts
        });
        let line_index = starts
            .partition_point(|start| *start <= offset)
            .checked_sub(1)?;
        Some(SourcePosition {
            offset,
            line: u32::try_from(line_index.checked_add(1)?).ok()?,
            column: offset
                .checked_sub(*starts.get(line_index)?)?
                .checked_add(1)?,
        })
    }

    /// Resolve a parser span to a canonical half-open source range.
    pub fn range_of(&self, span: &Span) -> Option<SourceRange> {
        let end = span.offset.checked_add(span.len)?;
        // Requiring a valid slice also rejects either endpoint inside a UTF-8 code point.
        self.text.get(span.offset..end)?;
        Some(SourceRange {
            start: self.position_at(span.offset)?,
            end: self.position_at(end)?,
        })
    }

    #[cfg(feature = "serde")]
    pub(crate) fn validates_span(&self, span: &Span) -> bool {
        self.slice(span).is_some()
            && self.location_at(span.offset) == Some((span.line, span.column))
    }

    #[cfg(feature = "serde")]
    pub(crate) fn trivia_between(&self, start: usize, end: usize) -> bool {
        start <= end && self.text.get(start..end).is_some_and(trivia_only)
    }

    fn location_at(&self, offset: usize) -> Option<(u32, usize)> {
        let position = self.position_at(offset)?;
        Some((position.line, position.column))
    }
}

impl Clone for SourceStorage {
    fn clone(&self) -> Self {
        Self::new(self.text.clone())
    }
}

impl PartialEq for SourceStorage {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Eq for SourceStorage {}

impl From<String> for SourceStorage {
    fn from(source: String) -> Self {
        Self::new(source)
    }
}

impl From<&str> for SourceStorage {
    fn from(source: &str) -> Self {
        Self::new(source.to_owned())
    }
}

impl AsRef<str> for SourceStorage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SourceStorage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let source = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(Self::new(source))
    }
}

/// Opaque identity of a reference in a document's [`QualifiedReferenceArena`].
///
/// IDs are local to one parsed document and must not be compared across documents.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QualifiedReferenceId(u32);

impl QualifiedReferenceId {
    pub(crate) fn from_index(index: u32) -> Self {
        Self(index)
    }

    pub(crate) fn index(self) -> usize {
        self.0 as usize
    }
}

/// How a reference segment is joined to the previous segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReferenceSeparator {
    ColonColon,
    Dot,
}

/// A segment stored in the packed reference arena.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReferenceSegment {
    /// Exact span of the authored name token, including quotes for an unrestricted name.
    pub source_span: Span,
    /// Separator joining this segment to its predecessor; `None` only for the first segment.
    pub separator_before: Option<ReferenceSeparator>,
}

/// Checked range into the arena's packed segment storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SegmentRange {
    start: u32,
    len: u32,
}

impl SegmentRange {
    fn new(start: u32, len: u32) -> Option<Self> {
        start.checked_add(len)?;
        Some(Self { start, len })
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn indices(self, segment_count: usize) -> Option<std::ops::Range<usize>> {
        let start = self.start as usize;
        let len = self.len as usize;
        let end = start.checked_add(len)?;
        (end <= segment_count).then_some(start..end)
    }
}

/// Per-reference metadata stored separately from the packed segments.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QualifiedReferenceMetadata {
    pub segments: SegmentRange,
    pub is_absolute: bool,
    /// Aggregate span from `$::` (when absolute) or the first segment through the final segment.
    pub span: Span,
}

/// Document-owned packed reference storage.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QualifiedReferenceArena {
    references: Vec<QualifiedReferenceMetadata>,
    segments: Vec<ReferenceSegment>,
}

impl QualifiedReferenceArena {
    pub fn len(&self) -> usize {
        self.references.len()
    }

    pub fn is_empty(&self) -> bool {
        self.references.is_empty()
    }

    #[cfg(feature = "serde")]
    pub(crate) fn contains(&self, id: QualifiedReferenceId) -> bool {
        id.index() < self.references.len()
    }

    /// Resolve a document-local ID to a borrowed, allocation-free view.
    pub fn get<'a>(
        &'a self,
        source: &'a SourceStorage,
        id: QualifiedReferenceId,
    ) -> Option<QualifiedReferenceView<'a>> {
        let metadata = self.references.get(id.index())?;
        let range = metadata.segments.indices(self.segments.len())?;
        let segments = self.segments.get(range)?;
        let authored_text = source.slice(&metadata.span)?;
        if segments
            .iter()
            .any(|segment| source.slice(&segment.source_span).is_none())
        {
            return None;
        }
        Some(QualifiedReferenceView {
            metadata,
            segments,
            source: source.as_str(),
            authored_text,
        })
    }

    /// Validate source bounds, range integrity, locations, and segment ordering for the arena.
    /// AST-to-arena ID validation is performed by the parsed-document envelope because the arena
    /// does not own the AST.
    pub fn validate(
        &self,
        source: &SourceStorage,
    ) -> Result<(), QualifiedReferenceValidationError> {
        let mut expected_segment_start = 0usize;
        for (reference_index, metadata) in self.references.iter().enumerate() {
            let id = u32::try_from(reference_index)
                .map(QualifiedReferenceId::from_index)
                .map_err(|_| QualifiedReferenceValidationError::TooManyReferences)?;
            let range = metadata
                .segments
                .indices(self.segments.len())
                .ok_or(QualifiedReferenceValidationError::SegmentRangeOutOfBounds { id })?;
            if range.start != expected_segment_start {
                return Err(QualifiedReferenceValidationError::SegmentRangeNotPacked { id });
            }
            expected_segment_start = range.end;
            if range.is_empty() {
                return Err(QualifiedReferenceValidationError::EmptyReference { id });
            }
            validate_span(source, id, None, &metadata.span)?;
            let reference_end = metadata
                .span
                .offset
                .checked_add(metadata.span.len)
                .ok_or(QualifiedReferenceValidationError::ReferenceSpanOutOfBounds { id })?;
            let authored = source
                .slice(&metadata.span)
                .ok_or(QualifiedReferenceValidationError::ReferenceSpanOutOfBounds { id })?;
            if metadata.is_absolute != authored.starts_with("$::") {
                return Err(QualifiedReferenceValidationError::AbsolutePrefixMismatch { id });
            }

            let segments = &self.segments[range];
            let first_segment = &segments[0];
            let leading = source
                .as_str()
                .get(metadata.span.offset..first_segment.source_span.offset)
                .ok_or(QualifiedReferenceValidationError::AggregateSpanMismatch { id })?;
            let valid_leading = if metadata.is_absolute {
                leading.strip_prefix("$::").is_some_and(trivia_only)
            } else {
                leading.is_empty()
            };
            if !valid_leading {
                return Err(QualifiedReferenceValidationError::AggregateSpanMismatch { id });
            }
            let mut previous_end = metadata.span.offset;
            for (segment_index, segment) in segments.iter().enumerate() {
                validate_span(source, id, Some(segment_index), &segment.source_span)?;
                let segment_end = segment
                    .source_span
                    .offset
                    .checked_add(segment.source_span.len)
                    .ok_or(QualifiedReferenceValidationError::SegmentSpanOutOfBounds {
                        id,
                        segment: segment_index,
                    })?;
                if segment.source_span.offset < metadata.span.offset
                    || segment_end > reference_end
                    || segment.source_span.offset < previous_end
                {
                    return Err(QualifiedReferenceValidationError::SegmentOutsideReference {
                        id,
                        segment: segment_index,
                    });
                }
                let separator_is_valid = if segment_index == 0 {
                    segment.separator_before.is_none()
                } else {
                    segment.separator_before.is_some()
                };
                if !separator_is_valid {
                    return Err(QualifiedReferenceValidationError::InvalidSeparator {
                        id,
                        segment: segment_index,
                    });
                }
                let authored_segment = source.slice(&segment.source_span).ok_or(
                    QualifiedReferenceValidationError::SegmentSpanOutOfBounds {
                        id,
                        segment: segment_index,
                    },
                )?;
                if !is_valid_authored_name(authored_segment) {
                    return Err(QualifiedReferenceValidationError::InvalidSegmentSpelling {
                        id,
                        segment: segment_index,
                    });
                }
                if authored_segment == "$" {
                    return Err(
                        QualifiedReferenceValidationError::SyntheticAbsoluteSegment {
                            id,
                            segment: segment_index,
                        },
                    );
                }
                if segment_index > 0 {
                    let gap = source
                        .as_str()
                        .get(previous_end..segment.source_span.offset)
                        .ok_or(QualifiedReferenceValidationError::SeparatorMismatch {
                            id,
                            segment: segment_index,
                        })?;
                    let Some(separator) = segment.separator_before else {
                        return Err(QualifiedReferenceValidationError::InvalidSeparator {
                            id,
                            segment: segment_index,
                        });
                    };
                    if !gap_matches_separator(gap, separator) {
                        return Err(QualifiedReferenceValidationError::SeparatorMismatch {
                            id,
                            segment: segment_index,
                        });
                    }
                }
                previous_end = segment_end;
            }
            if previous_end != reference_end {
                return Err(QualifiedReferenceValidationError::AggregateSpanMismatch { id });
            }
        }
        if expected_segment_start != self.segments.len() {
            return Err(QualifiedReferenceValidationError::UnreferencedSegments);
        }
        Ok(())
    }
}

fn trivia_only(text: &str) -> bool {
    let mut index = 0;
    skip_trivia(text.as_bytes(), &mut index) && index == text.len()
}

fn is_valid_authored_name(authored: &str) -> bool {
    let bytes = authored.as_bytes();
    let Some(first) = bytes.first() else {
        return false;
    };
    if *first != b'\'' {
        return (first.is_ascii_alphabetic() || *first == b'_')
            && bytes[1..]
                .iter()
                .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'_');
    }
    if bytes.len() < 2 || bytes.last() != Some(&b'\'') {
        return false;
    }
    let mut index = 1;
    while index + 1 < bytes.len() {
        if bytes[index] == b'\\' && bytes.get(index + 1) == Some(&b'\'') {
            index += 2;
        } else if bytes[index] == b'\'' {
            return false;
        } else {
            index += 1;
        }
    }
    index == bytes.len() - 1
}

fn gap_matches_separator(gap: &str, separator: ReferenceSeparator) -> bool {
    let bytes = gap.as_bytes();
    let mut index = 0;
    if !skip_trivia(bytes, &mut index) {
        return false;
    }
    let token = match separator {
        ReferenceSeparator::ColonColon => &b"::"[..],
        ReferenceSeparator::Dot => &b"."[..],
    };
    if !bytes[index..].starts_with(token) {
        return false;
    }
    index += token.len();
    skip_trivia(bytes, &mut index) && index == bytes.len()
}

fn skip_trivia(bytes: &[u8], index: &mut usize) -> bool {
    loop {
        while bytes
            .get(*index)
            .is_some_and(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r'))
        {
            *index += 1;
        }
        if bytes
            .get(*index..)
            .is_some_and(|tail| tail.starts_with(b"//*"))
        {
            let Some(close) = bytes[*index + 3..]
                .windows(2)
                .position(|window| window == b"*/")
            else {
                return false;
            };
            *index += close + 5;
            continue;
        }
        if bytes
            .get(*index..)
            .is_some_and(|tail| tail.starts_with(b"/*"))
        {
            let Some(close) = bytes[*index + 2..]
                .windows(2)
                .position(|window| window == b"*/")
            else {
                return false;
            };
            *index += close + 4;
            continue;
        }
        if bytes
            .get(*index..)
            .is_some_and(|tail| tail.starts_with(b"//"))
        {
            *index += 2;
            while bytes
                .get(*index)
                .is_some_and(|byte| *byte != b'\n' && *byte != b'\r')
            {
                *index += 1;
            }
            continue;
        }
        return true;
    }
}

fn validate_span(
    source: &SourceStorage,
    id: QualifiedReferenceId,
    segment: Option<usize>,
    span: &Span,
) -> Result<(), QualifiedReferenceValidationError> {
    if source.slice(span).is_none() {
        return Err(match segment {
            Some(segment) => {
                QualifiedReferenceValidationError::SegmentSpanOutOfBounds { id, segment }
            }
            None => QualifiedReferenceValidationError::ReferenceSpanOutOfBounds { id },
        });
    }
    if source.location_at(span.offset) != Some((span.line, span.column)) {
        return Err(QualifiedReferenceValidationError::SpanLocationMismatch { id, segment });
    }
    Ok(())
}

/// Borrowed view of one reference and its packed segments.
#[derive(Clone, Copy)]
pub struct QualifiedReferenceView<'a> {
    pub metadata: &'a QualifiedReferenceMetadata,
    pub segments: &'a [ReferenceSegment],
    source: &'a str,
    authored_text: &'a str,
}

impl std::fmt::Debug for QualifiedReferenceView<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("QualifiedReferenceView")
            .field("metadata", &self.metadata)
            .field("segments", &self.segments)
            .field("authored_text", &self.authored_text)
            .finish()
    }
}

impl<'a> QualifiedReferenceView<'a> {
    pub fn authored_text(&self) -> &'a str {
        self.authored_text
    }

    pub fn segment_authored_text(&self, index: usize) -> Option<&'a str> {
        let segment = self.segments.get(index)?;
        let end = segment
            .source_span
            .offset
            .checked_add(segment.source_span.len)?;
        self.source.get(segment.source_span.offset..end)
    }

    /// Return the semantic spelling of a segment.
    ///
    /// Basic names and quoted names without escaped quotes borrow the document source. An
    /// unrestricted name containing `\'` is decoded into an owned string only for that access.
    pub fn segment_decoded_text(&self, index: usize) -> Option<Cow<'a, str>> {
        decode_authored_name(self.segment_authored_text(index)?)
    }
}

pub(crate) fn decode_authored_name(authored: &str) -> Option<Cow<'_, str>> {
    if !authored.starts_with('\'') {
        return Some(Cow::Borrowed(authored));
    }
    if authored.len() < 2 || !authored.ends_with('\'') {
        return None;
    }
    let inner = authored.get(1..authored.len() - 1)?;
    if !inner.contains("\\'") {
        return Some(Cow::Borrowed(inner));
    }
    Some(Cow::Owned(inner.replace("\\'", "'")))
}

/// Capacity or shape failure while appending a parsed reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum QualifiedReferenceBuildError {
    EmptyReference,
    TooManyReferences,
    TooManySegments,
}

/// Mutable parse-time builder for [`QualifiedReferenceArena`].
#[derive(Debug, Default)]
pub(crate) struct QualifiedReferenceArenaBuilder {
    arena: QualifiedReferenceArena,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct QualifiedReferenceArenaCheckpoint {
    references: usize,
    segments: usize,
}

impl QualifiedReferenceArenaBuilder {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Mark the current append position so a speculative nom branch can be rolled back.
    pub(crate) fn checkpoint(&self) -> QualifiedReferenceArenaCheckpoint {
        QualifiedReferenceArenaCheckpoint {
            references: self.arena.references.len(),
            segments: self.arena.segments.len(),
        }
    }

    /// Discard references appended after `checkpoint`.
    pub(crate) fn rollback(&mut self, checkpoint: QualifiedReferenceArenaCheckpoint) {
        self.arena.references.truncate(checkpoint.references);
        self.arena.segments.truncate(checkpoint.segments);
    }

    #[cfg(test)]
    pub(crate) fn reference_span(&self, id: QualifiedReferenceId) -> Option<Span> {
        self.arena
            .references
            .get(id.index())
            .map(|metadata| metadata.span)
    }

    pub(crate) fn add_reference(
        &mut self,
        is_absolute: bool,
        span: Span,
        segments: impl IntoIterator<Item = ReferenceSegment>,
    ) -> Result<QualifiedReferenceId, QualifiedReferenceBuildError> {
        let reference_index = u32::try_from(self.arena.references.len())
            .map_err(|_| QualifiedReferenceBuildError::TooManyReferences)?;
        let segment_start = u32::try_from(self.arena.segments.len())
            .map_err(|_| QualifiedReferenceBuildError::TooManySegments)?;
        let old_segment_len = self.arena.segments.len();
        self.arena.segments.extend(segments);
        let added = self.arena.segments.len() - old_segment_len;
        let segment_len = match u32::try_from(added) {
            Ok(0) => {
                return Err(QualifiedReferenceBuildError::EmptyReference);
            }
            Ok(len) => len,
            Err(_) => {
                self.arena.segments.truncate(old_segment_len);
                return Err(QualifiedReferenceBuildError::TooManySegments);
            }
        };
        let Some(segment_range) = SegmentRange::new(segment_start, segment_len) else {
            self.arena.segments.truncate(old_segment_len);
            return Err(QualifiedReferenceBuildError::TooManySegments);
        };
        self.arena.references.push(QualifiedReferenceMetadata {
            segments: segment_range,
            is_absolute,
            span,
        });
        Ok(QualifiedReferenceId::from_index(reference_index))
    }

    pub(crate) fn finish(self) -> QualifiedReferenceArena {
        self.arena
    }
}

/// Invalid packed arena data, primarily used when validating a serialized document envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualifiedReferenceValidationError {
    TooManyReferences,
    DanglingReference {
        id: QualifiedReferenceId,
    },
    SegmentRangeOutOfBounds {
        id: QualifiedReferenceId,
    },
    SegmentRangeNotPacked {
        id: QualifiedReferenceId,
    },
    UnreferencedSegments,
    EmptyReference {
        id: QualifiedReferenceId,
    },
    ReferenceSpanOutOfBounds {
        id: QualifiedReferenceId,
    },
    SegmentSpanOutOfBounds {
        id: QualifiedReferenceId,
        segment: usize,
    },
    SpanLocationMismatch {
        id: QualifiedReferenceId,
        segment: Option<usize>,
    },
    SegmentOutsideReference {
        id: QualifiedReferenceId,
        segment: usize,
    },
    AggregateSpanMismatch {
        id: QualifiedReferenceId,
    },
    InvalidSeparator {
        id: QualifiedReferenceId,
        segment: usize,
    },
    SeparatorMismatch {
        id: QualifiedReferenceId,
        segment: usize,
    },
    InvalidSegmentSpelling {
        id: QualifiedReferenceId,
        segment: usize,
    },
    AbsolutePrefixMismatch {
        id: QualifiedReferenceId,
    },
    SyntheticAbsoluteSegment {
        id: QualifiedReferenceId,
        segment: usize,
    },
}

impl std::fmt::Display for QualifiedReferenceValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "invalid qualified-reference arena: {self:?}")
    }
}

impl std::error::Error for QualifiedReferenceValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn span(offset: usize, column: usize, len: usize) -> Span {
        Span {
            offset,
            line: 1,
            column,
            len,
        }
    }

    #[test]
    fn source_positions_use_one_document_owned_newline_index() {
        let source = SourceStorage::from("α\r\nbeta\n");

        assert_eq!(
            source.position_at(0),
            Some(SourcePosition {
                offset: 0,
                line: 1,
                column: 1,
            })
        );
        assert_eq!(source.position_at(1), None, "inside UTF-8 code point");
        assert_eq!(
            source.position_at(4),
            Some(SourcePosition {
                offset: 4,
                line: 2,
                column: 1,
            })
        );
        assert_eq!(
            source.position_at(source.len()),
            Some(SourcePosition {
                offset: 9,
                line: 3,
                column: 1,
            })
        );
        assert!(source.line_starts.get().is_some());
    }

    #[test]
    fn source_range_resolves_multiline_span_and_rejects_invalid_boundaries() {
        let source = SourceStorage::from("one\ntwö\nthree");
        let range = source
            .range_of(&Span {
                offset: 2,
                line: 1,
                column: 3,
                len: 7,
            })
            .expect("valid multiline range");
        assert_eq!(
            range,
            SourceRange {
                start: SourcePosition {
                    offset: 2,
                    line: 1,
                    column: 3,
                },
                end: SourcePosition {
                    offset: 9,
                    line: 3,
                    column: 1,
                },
            }
        );
        assert!(source
            .range_of(&Span {
                offset: 6,
                line: 2,
                column: 3,
                len: 1,
            })
            .is_none());
    }

    #[test]
    fn packed_reference_resolves_borrowed_segments() {
        let source = SourceStorage::from("Vehicle::mass.value");
        let mut builder = QualifiedReferenceArenaBuilder::new();
        let id = builder
            .add_reference(
                false,
                span(0, 1, 19),
                [
                    ReferenceSegment {
                        source_span: span(0, 1, 7),
                        separator_before: None,
                    },
                    ReferenceSegment {
                        source_span: span(9, 10, 4),
                        separator_before: Some(ReferenceSeparator::ColonColon),
                    },
                    ReferenceSegment {
                        source_span: span(14, 15, 5),
                        separator_before: Some(ReferenceSeparator::Dot),
                    },
                ],
            )
            .expect("reference should fit");
        let arena = builder.finish();

        arena.validate(&source).expect("arena should validate");
        let view = arena.get(&source, id).expect("reference should resolve");
        assert_eq!(view.authored_text(), "Vehicle::mass.value");
        assert_eq!(view.segment_authored_text(1), Some("mass"));
        assert_eq!(view.segment_decoded_text(2).as_deref(), Some("value"));
        assert_eq!(
            view.segments[2].separator_before,
            Some(ReferenceSeparator::Dot)
        );
    }

    #[test]
    fn absolute_prefix_is_metadata_not_a_segment() {
        let source = SourceStorage::from("$::Library::'mass value'");
        let mut builder = QualifiedReferenceArenaBuilder::new();
        let id = builder
            .add_reference(
                true,
                span(0, 1, 24),
                [
                    ReferenceSegment {
                        source_span: span(3, 4, 7),
                        separator_before: None,
                    },
                    ReferenceSegment {
                        source_span: span(12, 13, 12),
                        separator_before: Some(ReferenceSeparator::ColonColon),
                    },
                ],
            )
            .expect("reference should fit");
        let arena = builder.finish();

        arena.validate(&source).expect("arena should validate");
        let view = arena.get(&source, id).expect("reference should resolve");
        assert!(view.metadata.is_absolute);
        assert_eq!(view.segments.len(), 2);
        let decoded = view.segment_decoded_text(1).expect("decoded name");
        assert!(matches!(decoded, Cow::Borrowed("mass value")));
    }

    #[test]
    fn escaped_quoted_name_decodes_only_on_access() {
        let source = SourceStorage::from("'owner\\'s part'");
        let mut builder = QualifiedReferenceArenaBuilder::new();
        let id = builder
            .add_reference(
                false,
                span(0, 1, 15),
                [ReferenceSegment {
                    source_span: span(0, 1, 15),
                    separator_before: None,
                }],
            )
            .expect("reference should fit");
        let arena = builder.finish();
        let view = arena.get(&source, id).expect("reference should resolve");

        assert_eq!(view.segment_authored_text(0), Some("'owner\\'s part'"));
        let decoded = view.segment_decoded_text(0).expect("decoded name");
        assert!(matches!(&decoded, Cow::Owned(_)));
        assert_eq!(decoded, "owner's part");
    }

    #[test]
    fn source_storage_strips_bom_to_match_parser_offsets() {
        let source = SourceStorage::from("\u{FEFF}A::B");
        assert_eq!(source.as_str(), "A::B");
        assert_eq!(source.slice(&span(3, 4, 1)), Some("B"));
    }

    #[test]
    fn validation_rejects_missing_noninitial_separator() {
        let source = SourceStorage::from("A::B");
        let mut builder = QualifiedReferenceArenaBuilder::new();
        let _ = builder
            .add_reference(
                false,
                span(0, 1, 4),
                [
                    ReferenceSegment {
                        source_span: span(0, 1, 1),
                        separator_before: None,
                    },
                    ReferenceSegment {
                        source_span: span(3, 4, 1),
                        separator_before: None,
                    },
                ],
            )
            .expect("reference should fit");
        let arena = builder.finish();

        assert!(matches!(
            arena.validate(&source),
            Err(QualifiedReferenceValidationError::InvalidSeparator { segment: 1, .. })
        ));
    }

    #[test]
    fn validation_rejects_separator_that_disagrees_with_source() {
        let source = SourceStorage::from("A.B");
        let mut builder = QualifiedReferenceArenaBuilder::new();
        let _ = builder
            .add_reference(
                false,
                span(0, 1, 3),
                [
                    ReferenceSegment {
                        source_span: span(0, 1, 1),
                        separator_before: None,
                    },
                    ReferenceSegment {
                        source_span: span(2, 3, 1),
                        separator_before: Some(ReferenceSeparator::ColonColon),
                    },
                ],
            )
            .expect("reference should fit");
        let arena = builder.finish();

        assert!(matches!(
            arena.validate(&source),
            Err(QualifiedReferenceValidationError::SeparatorMismatch { segment: 1, .. })
        ));
    }

    #[test]
    fn validation_rejects_an_inexact_aggregate_span() {
        let source = SourceStorage::from("A::B;");
        let mut builder = QualifiedReferenceArenaBuilder::new();
        let _ = builder
            .add_reference(
                false,
                span(0, 1, 5),
                [
                    ReferenceSegment {
                        source_span: span(0, 1, 1),
                        separator_before: None,
                    },
                    ReferenceSegment {
                        source_span: span(3, 4, 1),
                        separator_before: Some(ReferenceSeparator::ColonColon),
                    },
                ],
            )
            .expect("reference should fit");

        assert!(matches!(
            builder.finish().validate(&source),
            Err(QualifiedReferenceValidationError::AggregateSpanMismatch { .. })
        ));
    }

    #[test]
    fn validation_accepts_trivia_after_an_absolute_prefix() {
        let source = SourceStorage::from("$:: /* scope */ A");
        let mut builder = QualifiedReferenceArenaBuilder::new();
        let _ = builder
            .add_reference(
                true,
                span(0, 1, 17),
                [ReferenceSegment {
                    source_span: span(16, 17, 1),
                    separator_before: None,
                }],
            )
            .expect("reference should fit");

        builder
            .finish()
            .validate(&source)
            .expect("absolute reference with trivia should validate");
    }

    #[test]
    fn builder_rejects_empty_reference_without_growing_arena() {
        let mut builder = QualifiedReferenceArenaBuilder::new();
        assert_eq!(
            builder.add_reference(false, Span::dummy(), []),
            Err(QualifiedReferenceBuildError::EmptyReference)
        );
        assert!(builder.finish().is_empty());
    }
}
