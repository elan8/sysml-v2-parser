//! Typed `Membership` AST (parser work item 4b, post-PAR-006).
//!
//! Every def/usage element previously lived as a bare, undifferentiated child in its owning
//! body's `Vec<Node<XBodyElement>>` -- ownership, visibility, and membership *kind* were not
//! represented independently of which enum variant the element happened to parse into (e.g.
//! `PartDefBodyElement::AttributeUsage(..)` vs `PartDefBodyElement::AttributeDef(..)` implicitly
//! encoded "feature membership" vs "owning membership" through the variant choice alone, with no
//! way to attach visibility to either).
//!
//! [`Membership`] is a small wrapper carrying that information explicitly, attached directly to
//! the highest-traffic member-bearing structs (see each struct's own `membership` field doc for
//! which ones) rather than by wrapping every body-enum element in a generic `Member<T>` container
//! -- the latter would touch every body enum across the whole crate (the largest possible blast
//! radius) for uniformity that most consumers don't need; direct fields let each struct family be
//! migrated independently, matching this backlog's established "start narrow, extend as needed"
//! discipline (see `RelationshipTarget`'s doc comment for the precedent).
//!
//! This item explicitly does **not** attempt a full sweep of every def/usage struct in one pass;
//! see `CHANGELOG.md`'s "Item 4b" entry for the exact list of struct families that got a
//! `membership` field in this increment vs. what's left as a documented, scoped-out follow-up.

use crate::ast::common::Visibility;
use crate::ast::core::Span;

/// What kind of member a def/usage element is, independent of visibility.
///
/// Deliberately starts narrow (matching the `TypingRelationship`/`SubsettingRelationship` kind
/// enums' own history): only the two forms actually distinguishable from where `Membership` is
/// wired in this increment are present. Extend with further variants (e.g. a dedicated
/// `ParameterMembership`, `ReturnParameterMembership`, `VariantMembership`, ...) only once a real
/// parser site needs to distinguish them, per this repo's "don't invent fields/variants with no
/// concrete grammar backing" precedent (see the PAR-003b `unique`/`readonly`/`variable`
/// out-of-scope notes in `CHANGELOG.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MembershipKind {
    /// A member owned by its containing namespace/definition -- the common case for a nested
    /// `*Def` (e.g. a nested `attribute def`/`part def`): the definition itself becomes a new
    /// named member of the owning namespace, not a feature of it.
    OwningMembership,
    /// A member that is itself a feature of the owning type -- the common case for a `*Usage`
    /// (e.g. a nested `attribute`/`part` usage): as opposed to a nested definition, a usage
    /// contributes a feature to its owning type.
    FeatureMembership,
    /// An imported member visible in this namespace but not owned by it (`import` clause).
    Import,
    /// An aliased member (`alias ... for ...`).
    Alias,
    /// A `variant` member inside a `variation part`/`variation` body (BNF `VariantUsageMember :
    /// VariantMembership = MemberPrefix 'variant' ownedVariantUsage = VariantUsageElement`),
    /// confirmed against `SysML-textual-bnf.kebnf` during the Item 4b final sweep -- named
    /// `VariantMembership` in the grammar itself, and its `MemberPrefix` legally carries a
    /// visibility prefix, so this is a distinct kind rather than reusing `FeatureMembership`.
    VariantMembership,
    /// An `actor` member inside a use-case-family body (BNF `ActorMember : ActorMembership =
    /// MemberPrefix ownedRelatedElement += ActorUsage`), confirmed against
    /// `SysML-textual-bnf.kebnf` during the Item 4b final sweep -- named `ActorMembership` in the
    /// grammar itself, distinct from `RequirementActorDecl`/`ActorDecl`'s own membership form.
    ActorMembership,
}

/// Ownership/visibility/kind wrapper for a def/usage member, kept alongside (not replacing) the
/// member's existing typed element. See the module doc for why this is a direct field on select
/// structs rather than a generic wrapper around every body-enum element.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Membership {
    pub kind: MembershipKind,
    /// `private` / `protected` / `public` prefix, when explicitly written. `None` means the
    /// member uses its owning namespace's default visibility (public at package level, private
    /// for most nested definition/usage members per the KerML default-visibility rules) --
    /// `Membership` records only what was explicitly written, it does not resolve defaults.
    pub visibility: Option<Visibility>,
    /// Span of the visibility prefix keyword itself when present, otherwise a zero-width span at
    /// the start of the member. Kept separate from the member's own span so semantic-token
    /// highlighting of just the `private`/`protected`/`public` keyword doesn't need to re-derive
    /// it from source text.
    pub span: Span,
}

/// Equality ignores `span`, matching this crate's other span-bearing types' convention: hand-built
/// expected ASTs in tests don't need real source spans.
impl PartialEq for Membership {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.visibility == other.visibility
    }
}

impl Eq for Membership {}

impl Membership {
    pub fn new(kind: MembershipKind, visibility: Option<Visibility>, span: Span) -> Self {
        Self {
            kind,
            visibility,
            span,
        }
    }

    /// Convenience constructor for the common `*Def` case (see [`MembershipKind::OwningMembership`]).
    pub fn owning(visibility: Option<Visibility>, span: Span) -> Self {
        Self::new(MembershipKind::OwningMembership, visibility, span)
    }

    /// Convenience constructor for the common `*Usage` case (see [`MembershipKind::FeatureMembership`]).
    pub fn feature(visibility: Option<Visibility>, span: Span) -> Self {
        Self::new(MembershipKind::FeatureMembership, visibility, span)
    }

    /// Convenience constructor for `VariantUsage` (see [`MembershipKind::VariantMembership`]).
    pub fn variant(visibility: Option<Visibility>, span: Span) -> Self {
        Self::new(MembershipKind::VariantMembership, visibility, span)
    }

    /// Convenience constructor for `ActorUsage` (see [`MembershipKind::ActorMembership`]).
    pub fn actor(visibility: Option<Visibility>, span: Span) -> Self {
        Self::new(MembershipKind::ActorMembership, visibility, span)
    }
}
