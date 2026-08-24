//! The shared `OccurrenceUsagePrefix` and the productions it nests.
//!
//! ```text
//! OccurrenceUsagePrefix : OccurrenceUsage =
//!     BasicUsagePrefix
//!     ( isIndividual ?= 'individual' )?
//!     ( portionKind = PortionKind { isPortion = true } )?
//!     UsageExtensionKeyword*                              -- SysML BNF 564, clause 8.2.2.9.2
//!
//! BasicUsagePrefix : Usage = RefPrefix ( isReference ?= 'ref' )?              -- 281
//!
//! RefPrefix : Usage =
//!     ( direction = FeatureDirection )?
//!     ( isDerived ?= 'derived' )?
//!     ( isAbstract ?= 'abstract' | isVariation ?= 'variation' )?
//!     ( isConstant ?= 'constant' )?                                          -- 275
//!
//! FeatureDirection : FeatureDirectionKind = 'in' | 'out' | 'inout'           -- 272
//! PortionKind = 'snapshot' | 'timeslice'                                     -- 585
//! UsageExtensionKeyword : Usage = ownedRelationship += PrefixMetadataMember  -- 296
//! PrefixMetadataMember : OwningMembership = '#' ownedRelatedElement = PrefixMetadataUsage -- 1660
//! PrefixMetadataUsage : MetadataUsage = ownedRelationship += OwnedFeatureTyping           -- 1663
//! ```
//!
//! Every production that spells this prefix spells all of it, in this order, so the components
//! are shared rather than re-declared per family. The full audit -- which families name this
//! production, which name the deliberately different `UsagePrefix` or `ControlNodePrefix`, the
//! corpus evidence, and which families this seam migrated -- is
//! `planning/occurrence-usage-prefix-matrix.md`.
//!
//! # Why these shapes
//!
//! The three mutually exclusive slots (`in`/`out`/`inout`, `abstract`/`variation`,
//! `snapshot`/`timeslice`) are each one optional field holding an enum, so no combination the
//! grammar forbids is representable. The four independent modifiers (`derived`, `constant`,
//! `ref`, `individual`) are each an `Option<Span>`: presence *is* the property, so there is no
//! second boolean to drift from the span, and emission writes the keyword because the author did,
//! not because a flag says so.
//!
//! The nesting is the grammar's own. `RefPrefix` and `BasicUsagePrefix` are separately named
//! productions that `UsagePrefix` and `ControlNodePrefix` also reach, so a later migration of
//! either uses the exact sub-component its production names. Flattening them into one struct
//! would make `ref merge m;` -- which `ControlNodePrefix` forbids -- representable.

use super::behavior::InOut;
use super::core::{Node, Span};
use super::qualified_reference::QualifiedReferenceId;
use super::structure::DefinitionPrefix;

/// `PortionKind = 'snapshot' | 'timeslice'` (SysML BNF 585).
///
/// One slot, two alternatives: a portion usage is a snapshot or a timeslice, never both.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OccurrencePortionKind {
    Snapshot,
    Timeslice,
}

impl OccurrencePortionKind {
    /// The authored keyword this alternative is written as.
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
            Self::Timeslice => "timeslice",
        }
    }
}

/// `RefPrefix : Usage = FeatureDirection? 'derived'? ('abstract' | 'variation')? 'constant'?`
/// (SysML BNF 275).
///
/// The first four slots of every usage prefix in the language, in the only order the grammar
/// permits. All four are optional, so an all-`None` value is the ordinary "no prefix authored"
/// state rather than a sentinel.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RefPrefix {
    /// `direction = FeatureDirection`: `in`, `out` or `inout`, with the authored keyword's span.
    ///
    /// One field holding an enum rather than three booleans, because the production gives the
    /// slot one value.
    pub direction: Option<Node<InOut>>,
    /// `isDerived ?= 'derived'`, as the authored keyword's span. `Some` *is* `isDerived`.
    pub derived_span: Option<Span>,
    /// `isAbstract ?= 'abstract' | isVariation ?= 'variation'` -- one slot, two alternatives.
    ///
    /// [`DefinitionPrefix`] is the same two-alternative group `BasicDefinitionPrefix` spells; the
    /// grammar writes it out twice rather than naming it once, and the crate already reuses the
    /// enum here through `AttributeUsage::usage_prefix`.
    pub variance: Option<Node<DefinitionPrefix>>,
    /// `isConstant ?= 'constant'`, as the authored keyword's span.
    pub constant_span: Option<Span>,
}

impl RefPrefix {
    /// Whether the author wrote any part of this prefix.
    pub fn is_authored(&self) -> bool {
        self.direction.is_some()
            || self.derived_span.is_some()
            || self.variance.is_some()
            || self.constant_span.is_some()
    }
}

/// `BasicUsagePrefix : Usage = RefPrefix ( isReference ?= 'ref' )?` (SysML BNF 281).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BasicUsagePrefix {
    pub ref_prefix: RefPrefix,
    /// `isReference ?= 'ref'`, as the authored keyword's span.
    pub reference_span: Option<Span>,
}

impl BasicUsagePrefix {
    /// Whether the author wrote any part of this prefix.
    pub fn is_authored(&self) -> bool {
        self.ref_prefix.is_authored() || self.reference_span.is_some()
    }
}

/// `UsageExtensionKeyword : Usage = ownedRelationship += PrefixMetadataMember` (SysML BNF 296),
/// i.e. `'#' OwnedFeatureTyping`.
///
/// Deliberately not [`MetadataKeywordUsage`](crate::ast::MetadataKeywordUsage): that type also
/// models the `ExtendedUsage` spelling `#Tag { … }`, which owns a `MetadataBody`, so it carries
/// an `Option<MetadataBody>`. `PrefixMetadataMember` owns nothing but the reference, and a
/// two-field type makes "a prefix keyword with a body" unrepresentable rather than merely unused.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsageExtensionKeyword {
    /// Exact `#` sigil. Syntax, not part of the reference behind it, and emission writes it from
    /// here rather than prepending a character to a rendered name.
    pub hash_span: Span,
    /// `PrefixMetadataUsage`'s `OwnedFeatureTyping` -- a `[QualifiedName]`, so a reference with
    /// its own scope, ordered segments and typed separators in the document arena. Never a
    /// declaration label and never copied text.
    pub annotation: QualifiedReferenceId,
}

/// `EndUsagePrefix : Usage = isEnd ?= 'end' ( ownedRelationship += OwnedCrossFeatureMember )?`
/// (SysML BNF 285; reference `SysML.xtext:568-570`).
///
/// The SysML counterpart of [`EndFeaturePrefix`](crate::ast::EndFeaturePrefix): `end` followed by
/// an optional owned cross feature, which is a `ReferenceUsage` here rather than a KerML
/// `Feature`. `end [1] part bead : TireBead;` (`training/09. Connections`) authors the cross
/// feature as a bare multiplicity; `end theCauses [*] occurrence theCause :> causes;`
/// (`CausationConnections.sysml`) names it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EndUsagePrefix {
    /// Exact `end` keyword.
    pub end_span: Span,
    /// `OwnedCrossFeatureMember`, when authored. Boxed because the declaration is far larger
    /// than the keyword, and most ends author none.
    pub cross: Option<Box<Node<OwnedCrossUsage>>>,
}

/// `OwnedCrossFeature : ReferenceUsage = BasicUsagePrefix UsageDeclaration` (SysML BNF 293).
///
/// Named `OwnedCrossUsage` because the KerML production of the same name --
/// [`OwnedCrossFeature`](crate::ast::OwnedCrossFeature) -- owns a `FeatureDeclaration` and a
/// `BasicFeaturePrefix`, neither of which is this type's. The two are distinct grammar
/// productions with distinct slots, not one node with a language flag.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OwnedCrossUsage {
    pub prefix: BasicUsagePrefix,
    /// The cross feature's own `UsageDeclaration`. At least one of its slots was authored: an
    /// empty declaration is no cross feature at all, and the parser leaves
    /// [`EndUsagePrefix::cross`] empty instead of manufacturing one.
    pub declaration: Node<crate::ast::UsageDeclaration>,
}

/// The choice at the head of [`OccurrenceUsagePrefix`].
///
/// ```text
/// OccurrenceUsagePrefix returns SysML::OccurrenceUsage :
///     ( EndUsagePrefix
///     | BasicUsagePrefix ( isIndividual ?= 'individual' )? ( portionKind = PortionKind )?
///     )
///     UsageExtensionKeyword*                                   -- SysML.xtext:836-843
/// ```
///
/// `end` and the basic slots are alternatives, so `end individual part p;` and
/// `ref end port p;` are unrepresentable rather than merely unemitted.
///
/// The published `.kebnf` (SysML BNF 564-570) spells only the second alternative. The reference
/// implementation, the official example corpus (`end port`, `end [1] part`, `end item`) and the
/// normative model library (`Interfaces.sysml:72` `end port source: Port :>> …`,
/// `Flows.sysml:82` `end occurrence source: Occurrence :>> …`) all author the first, so this
/// parser follows the reference grammar; see `planning/spec42-upstream-gap-audit.md`.
// The basic alternative is the common case on every usage in the corpus; boxing it to shrink the
// enum would spend a heap allocation per usage to save a few inline bytes on the rare `end`.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OccurrenceUsagePrefixHead {
    End(EndUsagePrefix),
    Basic {
        basic: BasicUsagePrefix,
        /// `isIndividual ?= 'individual'`, as the authored keyword's span.
        individual_span: Option<Span>,
        /// `portionKind = PortionKind` -- one slot, so `snapshot timeslice` is unrepresentable.
        portion: Option<Node<OccurrencePortionKind>>,
    },
}

impl Default for OccurrenceUsagePrefixHead {
    fn default() -> Self {
        Self::Basic {
            basic: BasicUsagePrefix::default(),
            individual_span: None,
            portion: None,
        }
    }
}

impl OccurrenceUsagePrefixHead {
    /// Whether the author wrote any part of this head.
    pub fn is_authored(&self) -> bool {
        match self {
            Self::End(_) => true,
            Self::Basic {
                basic,
                individual_span,
                portion,
            } => basic.is_authored() || individual_span.is_some() || portion.is_some(),
        }
    }
}

/// `OccurrenceUsagePrefix : OccurrenceUsage = ( EndUsagePrefix | BasicUsagePrefix ('individual')?
/// PortionKind? ) UsageExtensionKeyword*` (SysML BNF 564; reference `SysML.xtext:836`).
///
/// Shared by every occurrence-usage family this parser has migrated; see the module doc and
/// `planning/occurrence-usage-prefix-matrix.md` §9 for which those are.
///
/// `IndividualUsage` (BNF 576) and `PortionUsage` (BNF 580) inline `BasicUsagePrefix` plus the
/// same two slots instead of naming this production, and `EventOccurrenceUsage` (589) names it,
/// so all four spellings of the occurrence-usage family share one prefix value: the difference
/// between them is which slots were authored and which kind keyword follows, not which prefix
/// grammar applies.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OccurrenceUsagePrefix {
    /// `EndUsagePrefix | BasicUsagePrefix ( 'individual' )? ( PortionKind )?`.
    pub head: OccurrenceUsagePrefixHead,
    /// `UsageExtensionKeyword*` in authored order. The grammar's only repeatable slot here, so
    /// this is the only field that is a sequence; an empty one allocates nothing.
    pub extension_keywords: Vec<Node<UsageExtensionKeyword>>,
}

impl OccurrenceUsagePrefix {
    /// Whether the author wrote any part of this prefix.
    ///
    /// Used by emitters to decide nothing, and by tests and the semantic projection to say
    /// "unprefixed" in one place. Emission writes each component from its own span; it never
    /// consults this.
    pub fn is_authored(&self) -> bool {
        self.head.is_authored() || !self.extension_keywords.is_empty()
    }

    /// The `BasicUsagePrefix` alternative, `None` when the head is `end`.
    pub fn basic(&self) -> Option<&BasicUsagePrefix> {
        match &self.head {
            OccurrenceUsagePrefixHead::Basic { basic, .. } => Some(basic),
            OccurrenceUsagePrefixHead::End(_) => None,
        }
    }

    /// The `EndUsagePrefix` alternative, `None` when the head is basic.
    pub fn end(&self) -> Option<&EndUsagePrefix> {
        match &self.head {
            OccurrenceUsagePrefixHead::End(end) => Some(end),
            OccurrenceUsagePrefixHead::Basic { .. } => None,
        }
    }

    /// The authored `individual` keyword's span; only the basic alternative has the slot.
    pub fn individual_span(&self) -> Option<&Span> {
        match &self.head {
            OccurrenceUsagePrefixHead::Basic {
                individual_span, ..
            } => individual_span.as_ref(),
            OccurrenceUsagePrefixHead::End(_) => None,
        }
    }

    /// The authored portion kind; only the basic alternative has the slot.
    pub fn portion(&self) -> Option<&Node<OccurrencePortionKind>> {
        match &self.head {
            OccurrenceUsagePrefixHead::Basic { portion, .. } => portion.as_ref(),
            OccurrenceUsagePrefixHead::End(_) => None,
        }
    }
}
