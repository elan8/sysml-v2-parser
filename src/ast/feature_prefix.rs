//! The shared KerML `FeaturePrefix` and the productions it nests.
//!
//! ```text
//! Feature =                                                     -- KerML BNF 562, clause 8.2.4.3.1
//!     ( FeaturePrefix
//!       ( 'feature' | ownedRelationship += PrefixMetadataMember )
//!       FeatureDeclaration?
//!     | ( EndFeaturePrefix | BasicFeaturePrefix )
//!       FeatureDeclaration
//!     )
//!     ValuePart? TypeBody
//!
//! FeaturePrefix =                                                                    -- 584
//!     ( EndFeaturePrefix ( ownedRelationship += OwnedCrossFeatureMember )?
//!     | BasicFeaturePrefix
//!     )
//!     ( ownedRelationship += PrefixMetadataMember )*
//!
//! EndFeaturePrefix : Feature =                                                       -- 573
//!     ( isConstant ?= 'const' { isVariable = true } )?
//!     isEnd ?= 'end'
//!
//! BasicFeaturePrefix : Feature =                                                     -- 577
//!     ( direction = FeatureDirection )?
//!     ( isDerived ?= 'derived' )?
//!     ( isAbstract ?= 'abstract' )?
//!     ( isComposite ?= 'composite' | isPortion ?= 'portion' )?
//!     ( isVariable ?= 'var' | isConstant ?= 'const' { isVariable = true } )?
//!
//! OwnedCrossFeatureMember : OwningMembership = ownedRelatedElement += OwnedCrossFeature -- 592
//! OwnedCrossFeature : Feature = BasicFeaturePrefix FeatureDeclaration                 -- 595
//! FeatureDirection : FeatureDirectionKind = 'in' | 'out' | 'inout'                    -- 598
//! PrefixMetadataMember : OwningMembership = '#' ownedRelatedElement = PrefixMetadataUsage -- 1404
//! ```
//!
//! `Feature` (562), `Step` (863), `Expression` (895) and `BooleanExpression` (908) are the same
//! sentence four times -- `FeaturePrefix`, one keyword, `FeatureDeclaration`, `ValuePart?`, a body
//! -- so they share one prefix value and one node rather than one node per keyword. The full
//! audit (which productions spell this prefix, the corpus evidence, and why the three former nodes
//! were one production all along) is `planning/kerml-feature-prefix-matrix.md`.
//!
//! # Why these shapes
//!
//! [`FeaturePrefixHead`] is an enum, not two optional fields, because `FeaturePrefix` is a
//! *choice*: `EndFeaturePrefix` and `BasicFeaturePrefix` are alternatives, and `direction` lives
//! only in the second. That makes `in end feature x;` unrepresentable rather than merely unparsed
//! -- the conclusion `tests/snapshots/spec42/end_prefix_recovery.md` pins. For the same reason
//! `cross` hangs off the [`End`](FeaturePrefixHead::End) variant only, since
//! `OwnedCrossFeatureMember` appears in that alternative and nowhere else.
//!
//! [`EndFeaturePrefix::end_span`] is a bare [`Span`], not an `Option`, because `isEnd ?= 'end'` is
//! the one *required* token in that production: there is no `EndFeaturePrefix` without `end`.
//!
//! The two mutually exclusive slots (`composite`/`portion`, `var`/`const`) are each one optional
//! field holding an enum, so `composite portion` and `var const` are unrepresentable. Before this
//! seam the second was **accepted**, setting two independent booleans at once.
//!
//! Every independent modifier is an `Option<Span>`: presence *is* the property, so there is no
//! second boolean to drift from the span, and emission writes the keyword because the author did.
//!
//! `const` is the last slot of `BasicFeaturePrefix` and the first of `EndFeaturePrefix`. Because
//! the two are alternatives of one choice, `const` before `end` is `EndFeaturePrefix` and `const`
//! without `end` is `BasicFeaturePrefix`'s variability slot; no spelling reaches both, and none
//! puts `const` after `end`. The `{ isVariable = true }` action on both is abstract syntax, not
//! concrete: this component stores the authored alternative and derives `is_variable` from it
//! rather than storing a second flag that could disagree with the span.
//!
//! `member` is *not* modelled here. `TypeFeatureMember = MemberPrefix 'member' FeatureElement`
//! (523) puts it on the membership, ahead of the whole prefix, and the corpus writes it there:
//! `member abstract feature carSpeed : Real;` (`Variable Feature Examples/
//! TimeVaryingCarDriver.kerml:100`).

use super::behavior::InOut;
use super::common::DeclarationName;
use super::core::{Node, Span};
use super::occurrence_prefix::UsageExtensionKeyword;

/// `( isComposite ?= 'composite' | isPortion ?= 'portion' )?` (KerML BNF 581).
///
/// One slot, two alternatives: a feature is composite or a portion, never both.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeaturePortionKind {
    /// `composite`.
    Composite,
    /// `portion`.
    Portion,
}

impl FeaturePortionKind {
    /// The authored keyword this alternative is written as.
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Composite => "composite",
            Self::Portion => "portion",
        }
    }
}

/// `( isVariable ?= 'var' | isConstant ?= 'const' { isVariable = true } )?` (KerML BNF 582).
///
/// One slot, two alternatives, so `var const feature b;` is unrepresentable. Both alternatives
/// set `isVariable` in the abstract syntax; see [`is_variable`](Self::is_variable).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeatureVariability {
    /// `var`.
    Var,
    /// `const`.
    Const,
}

impl FeatureVariability {
    /// The authored keyword this alternative is written as.
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Var => "var",
            Self::Const => "const",
        }
    }

    /// `isVariable`, which the grammar's `{ isVariable = true }` action sets for *both*
    /// alternatives -- derived from the authored keyword rather than stored beside it.
    pub const fn is_variable(self) -> bool {
        true
    }

    /// `isConstant`, set by the `const` alternative only.
    pub const fn is_constant(self) -> bool {
        matches!(self, Self::Const)
    }
}

/// `BasicFeaturePrefix : Feature = FeatureDirection? 'derived'? 'abstract'?
/// ('composite' | 'portion')? ('var' | 'const')?` (KerML BNF 577).
///
/// All five slots are optional, so an all-`None` value is the ordinary "no prefix authored" state
/// rather than a sentinel.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BasicFeaturePrefix {
    /// `direction = FeatureDirection` (598): `in`, `out` or `inout`, with the authored keyword's
    /// span.
    ///
    /// One field holding an enum rather than three booleans, because the production gives the
    /// slot one value. This field existing only here -- and not on [`EndFeaturePrefix`] -- is what
    /// makes `in end feature x;` unrepresentable.
    pub direction: Option<Node<InOut>>,
    /// `isDerived ?= 'derived'`, as the authored keyword's span. `Some` *is* `isDerived`.
    pub derived_span: Option<Span>,
    /// `isAbstract ?= 'abstract'`, as the authored keyword's span.
    pub abstract_span: Option<Span>,
    /// `isComposite ?= 'composite' | isPortion ?= 'portion'` -- one slot, two alternatives.
    pub portioning: Option<Node<FeaturePortionKind>>,
    /// `isVariable ?= 'var' | isConstant ?= 'const'` -- one slot, two alternatives.
    pub variability: Option<Node<FeatureVariability>>,
}

impl BasicFeaturePrefix {
    /// Whether the author wrote any part of this prefix.
    pub fn is_authored(&self) -> bool {
        self.direction.is_some()
            || self.derived_span.is_some()
            || self.abstract_span.is_some()
            || self.portioning.is_some()
            || self.variability.is_some()
    }

    /// `isAbstract`, derived from the authored keyword's presence.
    pub fn is_abstract(&self) -> bool {
        self.abstract_span.is_some()
    }

    /// `isDerived`, derived from the authored keyword's presence.
    pub fn is_derived(&self) -> bool {
        self.derived_span.is_some()
    }

    /// `isComposite`, set by the `composite` alternative of the portioning slot.
    pub fn is_composite(&self) -> bool {
        matches!(
            self.portioning.as_ref().map(|n| n.value),
            Some(FeaturePortionKind::Composite)
        )
    }

    /// `isPortion`, set by the `portion` alternative of the portioning slot.
    pub fn is_portion(&self) -> bool {
        matches!(
            self.portioning.as_ref().map(|n| n.value),
            Some(FeaturePortionKind::Portion)
        )
    }

    /// `isVariable`, which both alternatives of the variability slot set.
    pub fn is_variable(&self) -> bool {
        self.variability.is_some()
    }

    /// `isConstant`, set by the `const` alternative of the variability slot.
    pub fn is_constant(&self) -> bool {
        matches!(
            self.variability.as_ref().map(|n| n.value),
            Some(FeatureVariability::Const)
        )
    }
}

/// `EndFeaturePrefix : Feature = ( isConstant ?= 'const' )? isEnd ?= 'end'` (KerML BNF 573).
///
/// The existence of this value *is* `isEnd`, which is why [`end_span`](Self::end_span) is a bare
/// [`Span`] rather than an `Option`: the production has no spelling without `end`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndFeaturePrefix {
    /// `isConstant ?= 'const'` preceding `end`, as the authored keyword's span
    /// (`const end feature b;`, KerML `associations` fixture; spec42 Gap 36).
    pub constant_span: Option<Span>,
    /// `isEnd ?= 'end'` -- required, so a [`Span`] and not an `Option<Span>`.
    pub end_span: Span,
}

impl EndFeaturePrefix {
    /// `isConstant`, derived from the authored keyword's presence.
    pub fn is_constant(&self) -> bool {
        self.constant_span.is_some()
    }

    /// `isVariable`, which the `const` alternative's `{ isVariable = true }` action sets.
    pub fn is_variable(&self) -> bool {
        self.constant_span.is_some()
    }
}

/// `OwnedCrossFeature : Feature = BasicFeaturePrefix FeatureDeclaration` (KerML BNF 595), carried
/// into the prefix by `OwnedCrossFeatureMember` (592).
///
/// The cross feature named between `end` and the feature keyword:
/// `end guardedLink [0..1] feature constrainedHBLink: HappensBefore;`
/// (`Kernel Semantic Library/TransitionPerformances.kerml:61`) declares `guardedLink [0..1]` here
/// and `constrainedHBLink: HappensBefore` on the feature itself.
///
/// `FeatureDeclaration` (601) is the full shared declaration tail; this models the slots the
/// corpus actually authors in cross position (identification, multiplicity, subsetting). The
/// remainder of that tail is recorded as deferred in `planning/kerml-feature-prefix-matrix.md`
/// §11 rather than represented by fields no parser fills.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OwnedCrossFeature {
    /// `BasicFeaturePrefix` on the cross feature itself.
    pub prefix: BasicFeaturePrefix,
    /// `FeatureIdentification`'s declared name. Empty for the unnamed spelling
    /// `end [1] feature transferSource references source;` (`Transfers.kerml`).
    pub name: Option<DeclarationName>,
    /// Multiplicity clause on the cross feature.
    pub multiplicity: Option<Node<crate::ast::Multiplicity>>,
    /// `MultiplicityPart`'s `isOrdered`/`isUnique` keyword slots, each carrying the authored
    /// spelling and its exact span.
    pub multiplicity_modifiers: crate::ast::MultiplicityModifiers,
    /// `subsets`/`:>` clause on the cross feature.
    pub subsets: Option<Node<crate::ast::SubsettingRelationship>>,
}

/// The `FeaturePrefix` choice (KerML BNF 584): `EndFeaturePrefix ( OwnedCrossFeatureMember )?` or
/// `BasicFeaturePrefix`.
///
/// An enum because the grammar writes an alternation. Modelling it as two optional fields would
/// make `in end feature x;` representable, and the direction-beside-`end` spelling is exactly what
/// `tests/snapshots/spec42/end_prefix_recovery.md` pins as unauthorable.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeaturePrefixHead {
    /// `EndFeaturePrefix ( ownedRelationship += OwnedCrossFeatureMember )?`.
    End {
        /// The `('const')? 'end'` prefix itself.
        prefix: EndFeaturePrefix,
        /// The optional owned cross feature between `end` and the feature keyword.
        ///
        /// Boxed: a cross feature carries a whole declaration, and the corpus authors one on a
        /// small minority of end features, so the common `Basic` alternative should not pay for
        /// it in every `FeaturePrefix`.
        cross: Option<Box<Node<OwnedCrossFeature>>>,
    },
    /// `BasicFeaturePrefix` -- the alternative that owns `direction`.
    Basic(BasicFeaturePrefix),
}

impl Default for FeaturePrefixHead {
    fn default() -> Self {
        Self::Basic(BasicFeaturePrefix::default())
    }
}

impl FeaturePrefixHead {
    /// The `BasicFeaturePrefix` alternative, when that is the one the author took.
    pub fn basic(&self) -> Option<&BasicFeaturePrefix> {
        match self {
            Self::Basic(prefix) => Some(prefix),
            Self::End { .. } => None,
        }
    }

    /// The `EndFeaturePrefix` alternative, when that is the one the author took.
    pub fn end(&self) -> Option<&EndFeaturePrefix> {
        match self {
            Self::End { prefix, .. } => Some(prefix),
            Self::Basic(_) => None,
        }
    }

    /// Whether the author wrote any part of this prefix. The `End` alternative always has, since
    /// `end` is required there.
    pub fn is_authored(&self) -> bool {
        match self {
            Self::End { .. } => true,
            Self::Basic(prefix) => prefix.is_authored(),
        }
    }
}

/// `FeaturePrefix` (KerML BNF 584): the `EndFeaturePrefix`/`BasicFeaturePrefix` choice followed by
/// any number of prefix metadata keywords.
///
/// Shared by every KerML feature-family production that spells it; see the module doc for which.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeaturePrefix {
    /// The `EndFeaturePrefix | BasicFeaturePrefix` choice.
    pub head: FeaturePrefixHead,
    /// `( ownedRelationship += PrefixMetadataMember )*` (1404) in authored order -- the only
    /// repeatable slot in this prefix, so the only field that is a sequence. An empty one
    /// allocates nothing.
    ///
    /// Reuses [`UsageExtensionKeyword`], which already models `'#' OwnedFeatureTyping` for the
    /// occurrence seam: one production, one representation. That type keeps its SysML-derived
    /// name; `planning/kerml-feature-prefix-matrix.md` §11 records the rename as debt rather than
    /// churning the migrated families for it.
    pub metadata_keywords: Vec<Node<UsageExtensionKeyword>>,
}

impl FeaturePrefix {
    /// Whether the author wrote any part of this prefix.
    pub fn is_authored(&self) -> bool {
        self.head.is_authored() || !self.metadata_keywords.is_empty()
    }

    /// `direction`, which only the `BasicFeaturePrefix` alternative can carry.
    pub fn direction(&self) -> Option<&Node<InOut>> {
        self.head.basic().and_then(|p| p.direction.as_ref())
    }

    /// `isEnd`, i.e. whether the author took the `EndFeaturePrefix` alternative.
    pub fn is_end(&self) -> bool {
        matches!(self.head, FeaturePrefixHead::End { .. })
    }

    /// `isAbstract`, which only the `BasicFeaturePrefix` alternative can carry.
    pub fn is_abstract(&self) -> bool {
        self.head
            .basic()
            .is_some_and(BasicFeaturePrefix::is_abstract)
    }

    /// `isDerived`, which only the `BasicFeaturePrefix` alternative can carry.
    pub fn is_derived(&self) -> bool {
        self.head
            .basic()
            .is_some_and(BasicFeaturePrefix::is_derived)
    }

    /// `isComposite`, which only the `BasicFeaturePrefix` alternative can carry.
    pub fn is_composite(&self) -> bool {
        self.head
            .basic()
            .is_some_and(BasicFeaturePrefix::is_composite)
    }

    /// `isPortion`, which only the `BasicFeaturePrefix` alternative can carry.
    pub fn is_portion(&self) -> bool {
        self.head
            .basic()
            .is_some_and(BasicFeaturePrefix::is_portion)
    }

    /// `isConstant`, which both alternatives can carry -- `const end` on the end prefix, a bare
    /// `const` on the basic one.
    pub fn is_constant(&self) -> bool {
        match &self.head {
            FeaturePrefixHead::End { prefix, .. } => prefix.is_constant(),
            FeaturePrefixHead::Basic(prefix) => prefix.is_constant(),
        }
    }

    /// `isVariable`, which both `var` and `const` set in either alternative.
    pub fn is_variable(&self) -> bool {
        match &self.head {
            FeaturePrefixHead::End { prefix, .. } => prefix.is_variable(),
            FeaturePrefixHead::Basic(prefix) => prefix.is_variable(),
        }
    }

    /// The owned cross feature, which only the `EndFeaturePrefix` alternative can carry.
    pub fn cross(&self) -> Option<&Node<OwnedCrossFeature>> {
        match &self.head {
            FeaturePrefixHead::End { cross, .. } => cross.as_deref(),
            FeaturePrefixHead::Basic(_) => None,
        }
    }
}
