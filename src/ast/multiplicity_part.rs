//! The ordering and uniqueness slots of `MultiplicityPart`.
//!
//! ```text
//! MultiplicityPart : Feature =
//!       ownedRelationship += OwnedMultiplicity
//!     | ( ownedRelationship += OwnedMultiplicity )?
//!       ( isOrdered ?= 'ordered' ( { isUnique = false } 'nonunique' )?
//!       | { isUnique = false } 'nonunique' ( isOrdered ?= 'ordered' )? )
//!                                              -- SysML BNF 495, KerML BNF 639
//! ```
//!
//! The multiplicity range itself stays where it already is, as each declaration's own
//! `multiplicity: Option<Node<Multiplicity>>`; this component owns the two keyword slots that
//! follow it, which every declaration carrying a multiplicity also spells.
//!
//! # Why two optional enums rather than two booleans
//!
//! `isOrdered` and `isUnique` are *derived* metamodel values with defaults (`false` and `true`),
//! and the notation writes a keyword only to state something. Two booleans conflated the derived
//! value with the authored one: they could say "not ordered" but not "the author wrote nothing",
//! and they had no room at all for the two spellings that state a default explicitly. So
//! `attribute a : Real[0..*] unique;` re-emitted as `attribute a : Real[0..*];` -- the keyword was
//! recognized, consumed, and thrown away.
//!
//! Each slot is therefore one `Option<Node<_>>` over the alternatives that slot admits. `None` is
//! "unauthored, so the metamodel default applies", and a `Some` carries both which keyword was
//! written and exactly where. One slot per pair also makes `ordered nonordered` and
//! `unique nonunique` unrepresentable rather than merely unusual.
//!
//! `nonordered` and `unique` are the explicit spellings of the two defaults. The pinned
//! productions above do not list them -- they spell only `ordered` and `nonunique` -- and neither
//! does the OMG Pilot implementation, whose grammars contain no `unique` or `nonordered` token at
//! all (`KerML.xtext` 578-586, `SysML.xtext` 370-378; `nonunique` is their only spelling of the
//! uniqueness slot, and it is not even a reserved word in the pin -- see the keyword lists at
//! SysML BNF 21 and KerML BNF 121, which list `nonunique` and `ordered` and neither of the other
//! two). This parser has nevertheless always accepted them, and silently discarding recognized
//! syntax is the one thing it must not do. They are retained here as authored facts, distinct
//! from omission, so a consumer can tell "the author stated the default" from "the author said
//! nothing" and a formatter reproduces what was written. They fill the same two slots, so they
//! obey the same cardinality as the spellings the production does list.
//!
//! # One slot each, and what happens to the excess
//!
//! The production is an alternation over two *distinct* slots, so it admits at most one ordering
//! keyword and at most one uniqueness keyword, and the range that may precede them at most once.
//! Two `Option` fields say exactly that: a repetition is unrepresentable, as is the contradiction
//! `ordered nonordered`.
//!
//! That leaves the question of what the parser does when an author writes one anyway. It does not
//! keep the first spelling and swallow the rest -- that is the permissive fallback `AGENTS.md`
//! forbids, and it made `Real[0..*] ordered ordered` re-emit as `Real[0..*] ordered` with no
//! diagnostic at all. Instead the parser stops at the excess and leaves it unconsumed, so the
//! enclosing declaration fails at that token and the enclosing scope's member recovery captures
//! the whole member by source span as a malformed node with a stable code. Nothing is invented,
//! nothing is dropped, and the valid sibling after it still parses. This also matches the
//! reference tool: `MultiplicityPart` is an Xtext fragment matched once, so the Pilot's generated
//! parser reports a syntax error on the token after it.
//!
//! The cardinality is per *declaration*, not per parse position. `FeatureSpecializationPart`
//! (SysML BNF 424-426, KerML BNF 632-634) lets the part sit either side of the specializations, so
//! the parsers offer the position twice and thread this value through as an accumulator rather
//! than parsing two groups and folding them. Folding was how `ordered ordered` used to survive the
//! narrowing above: one slot filled per position, second spelling discarded. There is therefore no
//! `merge` here; a filled field is what stops the next position from consuming.

use super::core::Node;

/// The `isOrdered` slot of `MultiplicityPart`: one keyword, two spellings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MultiplicityOrdering {
    /// `ordered`, i.e. the production's `isOrdered ?= 'ordered'`.
    Ordered,
    /// `nonordered`, the explicit spelling of the `isOrdered = false` default.
    Nonordered,
}

impl MultiplicityOrdering {
    /// The authored keyword this alternative is written as.
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Ordered => "ordered",
            Self::Nonordered => "nonordered",
        }
    }

    /// The `isOrdered` value this spelling states.
    pub const fn is_ordered(self) -> bool {
        matches!(self, Self::Ordered)
    }
}

/// The `isUnique` slot of `MultiplicityPart`: one keyword, two spellings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MultiplicityUniqueness {
    /// `unique`, the explicit spelling of the `isUnique = true` default.
    Unique,
    /// `nonunique`, i.e. the production's `{ isUnique = false } 'nonunique'`.
    Nonunique,
}

impl MultiplicityUniqueness {
    /// The authored keyword this alternative is written as.
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Unique => "unique",
            Self::Nonunique => "nonunique",
        }
    }

    /// The `isUnique` value this spelling states.
    pub const fn is_unique(self) -> bool {
        matches!(self, Self::Unique)
    }
}

/// The keyword slots of `MultiplicityPart`, shared by every declaration that spells them.
///
/// Both fields default to `None`, so an unmodified declaration is the ordinary empty value rather
/// than a sentinel, and neither slot allocates.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MultiplicityModifiers {
    /// `isOrdered ?= 'ordered'`, or its explicit negation, with the authored keyword's span.
    pub ordering: Option<Node<MultiplicityOrdering>>,
    /// `{ isUnique = false } 'nonunique'`, or its explicit affirmation, with the authored
    /// keyword's span.
    pub uniqueness: Option<Node<MultiplicityUniqueness>>,
}

impl MultiplicityModifiers {
    /// Whether the author wrote either keyword.
    pub fn is_authored(&self) -> bool {
        self.ordering.is_some() || self.uniqueness.is_some()
    }

    /// The effective `isOrdered` value: what the author wrote, else the metamodel default.
    pub fn is_ordered(&self) -> bool {
        self.ordering
            .as_ref()
            .is_some_and(|ordering| ordering.value.is_ordered())
    }

    /// The effective `isUnique` value: what the author wrote, else the metamodel default.
    pub fn is_unique(&self) -> bool {
        self.uniqueness
            .as_ref()
            .is_none_or(|uniqueness| uniqueness.value.is_unique())
    }
}
