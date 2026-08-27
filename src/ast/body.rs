//! The shared body container.

use super::core::{Node, Span};

/// A declaration body: either the semicolon form or an ordered brace form.
///
/// Almost every SysML and KerML declaration ends the same way -- `;` when the declaration owns no
/// members, or `{ ... }` around an ordered list of them -- and the grammar means the same thing by
/// it each time. The member *set* differs per scope and stays typed: each scope names its own
/// element enum, so `Body<PartDefBodyElement>` and `Body<ActionDefBodyElement>` remain different
/// types and a member cannot migrate to a scope that does not accept it.
///
/// # Distinctions this type keeps
///
/// A semicolon body and an empty brace body are different authored syntax and are represented
/// differently: `Semicolon` versus `Brace { elements: [] }`. Do not treat one as the other, and
/// prefer [`braced_elements`](Self::braced_elements) over [`members`](Self::members) when the
/// difference matters to the caller -- the first makes an absent brace body visible as `None`,
/// the second deliberately flattens it away for consumers that only want the members.
///
/// # A body is always written
///
/// Both alternatives require an authored token, so this type cannot represent a declaration that
/// has no body at all. The two places where the parser accepts one -- a `#Name` metadata keyword
/// used as a prefix on another declaration, and an action usage whose terminator is implied by the
/// statement after it -- hold `Option<Body<_>>` instead, which confines that state to the two
/// grammar contexts that have it rather than offering it to every scope.
///
/// # Delimiter provenance
///
/// Both brace tokens and the semicolon are retained exactly as authored, so a consumer never has
/// to re-scan the source to find where a body began or ended.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Body<E> {
    /// `;` -- the declaration owns no members.
    Semicolon {
        /// Exact `;` token that ended the declaration.
        semicolon_span: Span,
    },
    /// `{ ... }` -- an ordered, possibly empty list of members in source order.
    Brace {
        /// Exact `{` token opening the body.
        open_span: Span,
        /// Members in authored order, including malformed and unsupported ones at the position
        /// they were written.
        elements: Vec<Node<E>>,
        /// Exact `}` token closing the body.
        ///
        /// Every brace body the parser produces has one. An unterminated body does not become a
        /// body with a missing close: the parse of the whole declaration fails and the enclosing
        /// scope keeps the text as a recovery node, so there is no state here for "no close
        /// token". Giving recovery the ability to retain the members it read up to end of input
        /// is a parser change, and the typed close outcome it needs belongs with that change
        /// rather than as an unreachable variant now.
        close_span: Span,
    },
}

impl<E> Body<E> {
    /// Whether the declaration was written with the semicolon form.
    ///
    /// This is the only way to distinguish `;` from `{}`; both have no members.
    pub const fn is_semicolon(&self) -> bool {
        matches!(self, Self::Semicolon { .. })
    }

    /// The members of a brace body, or `None` when the body is the semicolon form.
    ///
    /// Use this when an absent body and an empty one lead to different behavior.
    pub fn braced_elements(&self) -> Option<&[Node<E>]> {
        match self {
            Self::Semicolon { .. } => None,
            Self::Brace { elements, .. } => Some(elements),
        }
    }

    /// Iterates the members in source order; a semicolon body yields nothing.
    ///
    /// This flattens the `;`/`{}` distinction on purpose, for consumers that only care about the
    /// members themselves. Use [`braced_elements`](Self::braced_elements) when it matters.
    pub fn members(&self) -> std::slice::Iter<'_, Node<E>> {
        match self {
            Self::Semicolon { .. } => [].iter(),
            Self::Brace { elements, .. } => elements.iter(),
        }
    }
}
