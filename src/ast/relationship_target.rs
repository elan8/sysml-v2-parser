//! Structured relationship-target AST (parser work item 2, post-PAR-006).
//!
//! [`TypingRelationship::target`](crate::ast::TypingRelationship::target) and
//! [`SubsettingRelationship::target`](crate::ast::SubsettingRelationship::target) used to be a
//! plain `String` built by joining `.`-dotted feature-chain segments and `::`-qualified
//! namespace/type segments into a single string with no way to tell them apart afterward, and by
//! joining comma-separated multi-target clauses (e.g. `:> Base, Other`) with `", "`, losing the
//! fact there were multiple distinct targets.
//!
//! This is a new, narrow type reserved for relationship/alias endpoints -- deliberately not a
//! reuse or widening of [`crate::ast::FeatureChain`], which is wired specifically into
//! `Expression::FeatureChainRef` for expression-level dot-chain parsing and intentionally excludes
//! `::`-qualification and other expression-postfix concerns.

use crate::ast::core::Span;

/// How a [`RelationshipTargetSegment`] joins to the previous segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SegmentSeparator {
    /// `::`, e.g. the join between `ISQ` and `mass` in `ISQ::mass`.
    ColonColon,
    /// `.`, e.g. the join between `engine` and `fuelCmdPort` in `engine.fuelCmdPort`.
    Dot,
}

/// A single segment of a [`RelationshipTarget`], e.g. `mass` in `ISQ::mass`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RelationshipTargetSegment {
    pub name: String,
    /// How this segment joins to the previous one; `None` for the first segment.
    pub separator: Option<SegmentSeparator>,
}

impl PartialEq for RelationshipTargetSegment {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.separator == other.separator
    }
}

impl Eq for RelationshipTargetSegment {}

impl RelationshipTargetSegment {
    /// A single, unqualified segment with no leading separator (used as the sole segment of a
    /// bare-name target such as a subsetting/redefinition target parsed from a plain identifier).
    pub fn simple(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            separator: None,
        }
    }
}

/// A single relationship endpoint: an ordered sequence of `::`- and/or `.`-joined segments, e.g.
/// `ISQ::mass` or `engine.fuelCmdPort.flowRate` or `Vehicle::mass.value`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RelationshipTarget {
    /// Always has at least one element.
    pub segments: Vec<RelationshipTargetSegment>,
    /// Span of this one target (not the whole relationship clause, which may hold several
    /// comma-separated targets).
    pub span: Span,
}

/// Equality ignores `span`, matching [`crate::ast::FeatureChain`]'s and this crate's other
/// span-bearing types' convention: hand-built expected ASTs in tests don't need real source spans.
impl PartialEq for RelationshipTarget {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments
    }
}

impl Eq for RelationshipTarget {}

impl RelationshipTarget {
    /// Build a single-segment target (no `::`/`.` joins) at the given span, e.g. for a bare
    /// feature name used directly as a subsetting/redefinition target.
    pub fn single(name: impl Into<String>, span: Span) -> Self {
        Self {
            segments: vec![RelationshipTargetSegment::simple(name)],
            span,
        }
    }

    /// The dotted/`::`-joined textual form of this target, e.g. `"Vehicle::mass.value"`. Rebuilds
    /// the original separators from `segments` rather than storing a redundant joined string.
    pub fn to_display_string(&self) -> String {
        let mut out = String::new();
        for segment in &self.segments {
            match segment.separator {
                Some(SegmentSeparator::ColonColon) => out.push_str("::"),
                Some(SegmentSeparator::Dot) => out.push('.'),
                None => {}
            }
            out.push_str(&segment.name);
        }
        out
    }

    /// The last segment's name, e.g. `"mass"` for `ISQ::mass` or `"value"` for
    /// `Vehicle::mass.value`. `segments` always has at least one element, so this never returns
    /// `None` for a well-formed target.
    pub fn local_name(&self) -> Option<&str> {
        self.segments.last().map(|s| s.name.as_str())
    }
}
