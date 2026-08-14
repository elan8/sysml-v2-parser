//! The shared body container.

use super::core::Node;

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
/// # What it does not yet model
///
/// The container records no delimiter provenance: neither brace span is retained, and neither is
/// the semicolon's. Consumers that need exact delimiter positions cannot get them from this type
/// today. A recovered or missing closing brace is likewise not represented here -- the parser
/// reports it as a diagnostic and keeps the members it recognized. Adding delimiter provenance
/// means deciding how an incomplete close is represented, which is a grammar decision rather than
/// a mechanical one, so it is deliberately not encoded as an optional span with an undocumented
/// meaning.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Body<E> {
    /// `;` -- the declaration owns no members.
    Semicolon,
    /// `{ ... }` -- an ordered, possibly empty list of members in source order.
    Brace {
        /// Members in authored order, including malformed and unsupported ones at the position
        /// they were written.
        elements: Vec<Node<E>>,
    },
}

impl<E> Body<E> {
    /// Whether the declaration was written with the semicolon form.
    ///
    /// This is the only way to distinguish `;` from `{}`; both have no members.
    pub const fn is_semicolon(&self) -> bool {
        matches!(self, Self::Semicolon)
    }

    /// The members of a brace body, or `None` when the body is the semicolon form.
    ///
    /// Use this when an absent body and an empty one lead to different behavior.
    pub fn braced_elements(&self) -> Option<&[Node<E>]> {
        match self {
            Self::Semicolon => None,
            Self::Brace { elements } => Some(elements),
        }
    }

    /// Iterates the members in source order; a semicolon body yields nothing.
    ///
    /// This flattens the `;`/`{}` distinction on purpose, for consumers that only care about the
    /// members themselves. Use [`braced_elements`](Self::braced_elements) when it matters.
    pub fn members(&self) -> std::slice::Iter<'_, Node<E>> {
        match self {
            Self::Semicolon => [].iter(),
            Self::Brace { elements } => elements.iter(),
        }
    }
}
