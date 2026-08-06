//! Alias definition parsing.

use crate::ast::{AliasBody, AliasDef, Node, RelationshipTarget};
use crate::parser::body::relationship_body_annotations;
use crate::parser::lex::{identification, qualified_name_segments, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::{span_from_to, Input};
use nom::bytes::complete::tag;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// Alias body: `;` or `{` doc/comment/rep/metadata* `}` (BNF `RelationshipBody`).
fn alias_body(input: Input<'_>) -> IResult<Input<'_>, AliasBody> {
    let (input, elements) = relationship_body_annotations(input)?;
    let body = match elements {
        None => AliasBody::Semicolon,
        Some(elements) => AliasBody::Brace { elements },
    };
    Ok((input, body))
}

/// Alias definition: `alias` Identification `for` qualified_name body
pub(crate) fn alias_def(input: Input<'_>) -> IResult<Input<'_>, Node<AliasDef>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, _) = tag(&b"alias"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, identification) = identification(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"for"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let target_start = input;
    let (input, target_segments) = qualified_name_segments(input)?;
    let target = RelationshipTarget {
        segments: target_segments,
        span: span_from_to(target_start, input),
    };
    let (input, body) = alias_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            AliasDef {
                identification,
                target,
                body,
                membership: crate::ast::Membership::new(
                    crate::ast::MembershipKind::Alias,
                    visibility,
                    visibility_span,
                ),
            },
        ),
    ))
}

#[cfg(test)]
mod membership_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    // --- parser work item 4b (continuation): Membership on AliasDef ---
    // BNF `AliasMember : Membership = MemberPrefix 'alias' ...` legally permits a visibility
    // prefix, but `alias_def` never parsed one at all before this increment -- same gap class
    // found repeatedly in this rollout.

    #[test]
    fn alias_def_visibility_prefix_is_captured_on_membership() {
        let (_, node) = alias_def(input("private alias m for ISQ::mass;")).expect("alias def");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Private)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::Alias
        );
    }

    #[test]
    fn alias_def_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = alias_def(input("alias m for ISQ::mass;")).expect("alias def");
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::Alias
        );
    }
}
