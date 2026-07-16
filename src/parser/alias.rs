//! Alias definition parsing.

use crate::ast::{AliasBody, AliasDef, Node, RelationshipTarget};
use crate::parser::body::advance_to_closing_brace;
use crate::parser::lex::{identification, qualified_name_segments, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::{span_from_to, Input};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// Alias body: `;` or `{` ... `}`
fn alias_body(input: Input<'_>) -> IResult<Input<'_>, AliasBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b";"[..]), |_| AliasBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag(&b"{"[..]),
                advance_to_closing_brace,
                preceded(ws_and_comments, tag(&b"}"[..])),
            ),
            |_| AliasBody::Brace,
        ),
    ))
    .parse(input)
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
        let (_, node) =
            alias_def(input("private alias m for ISQ::mass;")).expect("alias def");
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
