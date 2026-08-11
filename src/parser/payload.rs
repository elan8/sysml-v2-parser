//! Shared parsers for accept/send payload clauses and transition accept triggers.

use crate::ast::{
    ActionUsage, Expression, Node, PayloadClause, SendPayload, Span, TransitionAccept, TriggerKind,
};
use crate::parser::action::action_usage_body;
use crate::parser::expr::expression;
use crate::parser::lex::{
    name, qualified_reference, starts_with_any_keyword, starts_with_keyword, ws1, ws_and_comments,
};
use crate::parser::node_from_to;
use crate::parser::with_span;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// Required typed payload: `name : qualified_name`.
pub(crate) fn typed_payload_clause(input: Input<'_>) -> IResult<Input<'_>, PayloadClause> {
    let (input, (name_span, name)) = with_span(name).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
    let (input, (type_span, type_name)) =
        preceded(ws_and_comments, with_span(qualified_reference)).parse(input)?;
    Ok((
        input,
        PayloadClause {
            name,
            type_name: Some(type_name),
            name_span,
            type_span: Some(type_span),
        },
    ))
}

/// BNF `TriggerKind` (§6 G8): `at` | `when` | `after`. The trailing `ws1` keeps identifiers that
/// merely start with one of these keywords (`atomicMass`, `afterburner`) out.
fn trigger_kind(input: Input<'_>) -> IResult<Input<'_>, TriggerKind> {
    nom::branch::alt((
        nom::combinator::map(preceded(tag(&b"at"[..]), ws1), |_| TriggerKind::At),
        nom::combinator::map(preceded(tag(&b"when"[..]), ws1), |_| TriggerKind::When),
        nom::combinator::map(preceded(tag(&b"after"[..]), ws1), |_| TriggerKind::After),
    ))
    .parse(input)
}

/// After `accept` keyword: `name : Type` or shorthand expression, with an optional trailing
/// `via <port>` clause (e.g. `accept TurnOn via commPort`).
pub(crate) fn transition_accept(input: Input<'_>) -> IResult<Input<'_>, TransitionAccept> {
    let (input, _) = preceded(ws_and_comments, tag(&b"accept"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    // §6 G8: a `TriggerKind` keyword replaces the payload entirely. Checked before `expression`,
    // which would otherwise take the keyword itself as a feature reference and leave the real
    // trigger expression to be misread as the transition's effect.
    if let Ok((input, kind)) = trigger_kind(input) {
        let (input, expr_node) = expression(input)?;
        return Ok((input, TransitionAccept::TimeTrigger(kind, expr_node)));
    }
    let payload_name = name(input).ok().map(|(_, value)| value);
    let (input, expr_node) = expression(input)?;
    let (input, type_suffix) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, with_span(qualified_reference)),
    ))
    .parse(input)?;
    let (input, via) = opt(preceded(
        preceded(ws_and_comments, tag(&b"via"[..])),
        preceded(ws1, expression),
    ))
    .parse(input)?;
    if let (Expression::FeatureRef(_), Some(name), Some((type_span, type_name))) =
        (&expr_node.value, payload_name, type_suffix)
    {
        return Ok((
            input,
            TransitionAccept::Payload(
                PayloadClause {
                    name,
                    type_name: Some(type_name),
                    name_span: expr_node.span.clone(),
                    type_span: Some(type_span),
                },
                via,
            ),
        ));
    }
    Ok((input, TransitionAccept::Shorthand(expr_node, via)))
}

/// Standalone control-node statement: `accept|send` payload (`;` or body).
///
/// BNF `AcceptNode`'s payload (`PayloadParameter`) is mandatory and typed-name-only;
/// `SendNode`'s payload (`NodeParameterMember` = `FeatureBinding` = a general `OwnedExpression`)
/// is optional and accepts either the same typed-name shorthand or a general expression, e.g.
/// `send new Publish(someTopic, somePublication) via publicationPort;` (GH-86, Interaction
/// Sequencing Examples/ServerSequenceOutsideRealization-2.sysml). Both accept an optional
/// trailing `via <expr>` (BNF `AcceptParameterPart`/`SenderReceiverPart`); send additionally
/// accepts a trailing `to <expr>` (BNF `SenderReceiverPart`), e.g. `then send new S() to b;`
/// (Simple Tests/ActionTest.sysml).
fn control_node_payload_stmt<'a>(
    input: Input<'a>,
    keyword: &'a [u8],
    control_name: &'static str,
) -> IResult<Input<'a>, Node<ActionUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(keyword).parse(input)?;
    let (input, _) = ws1(input)?;
    let is_send = keyword == b"send";

    let (input, name_span, type_ref_span, accept, send) = if is_send {
        // Peek for `via`/`to` first: neither is reserved in `name`/`expression`, so an empty
        // payload (BNF `EmptyParameterMember`) would otherwise be greedily consumed as a bare
        // feature reference instead of being left for the `via`/`to` clauses below.
        let (peek, _) = ws_and_comments(input)?;
        let payload_follows = !starts_with_any_keyword(peek.fragment(), &[b"via", b"to"]);
        let (input, payload) = if payload_follows {
            opt(alt((
                map(typed_payload_clause, SendPayload::Typed),
                map(expression, SendPayload::Expression),
            )))
            .parse(input)?
        } else {
            (input, None)
        };
        let name_span = match &payload {
            Some(SendPayload::Typed(p)) => p.name_span.clone(),
            Some(SendPayload::Expression(e)) => e.span.clone(),
            None => Span::dummy(),
        };
        let type_ref_span = match &payload {
            Some(SendPayload::Typed(p)) => p.type_span.clone(),
            _ => None,
        };
        (input, name_span, type_ref_span, None, payload)
    } else {
        let (input, payload) = typed_payload_clause(input)?;
        let name_span = payload.name_span.clone();
        let type_ref_span = payload.type_span.clone();
        (input, name_span, type_ref_span, Some(payload), None)
    };

    let (input, via) = opt(preceded(
        preceded(ws_and_comments, tag(&b"via"[..])),
        preceded(ws1, expression),
    ))
    .parse(input)?;
    let (input, to) = if is_send {
        opt(preceded(
            preceded(ws_and_comments, tag(&b"to"[..])),
            preceded(ws1, expression),
        ))
        .parse(input)?
    } else {
        (input, None)
    };

    let (input, _) = ws_and_comments(input)?;
    let (input, body) = action_usage_body(input)?;
    let (input, _) =
        nom::combinator::opt(preceded(ws_and_comments, tag(&b";"[..]))).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ActionUsage {
                is_abstract: false,
                is_variation: false,
                is_reference: false,
                is_individual: false,
                name: control_name.to_string(),
                type_name: None,
                typing: None,
                multiplicity: None,
                subsets: None,
                redefines: None,
                accept,
                send,
                via,
                to,
                body,
                name_span: Some(name_span),
                type_ref_span,
                // No visibility grammar at this standalone `accept`/`send` control-node-statement
                // position. See `ActionUsage::membership`.
                membership: crate::ast::Membership::feature(None, crate::ast::Span::dummy()),
            },
        ),
    ))
}

pub(crate) fn control_node_action_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ActionUsage>> {
    let (peek, _) = ws_and_comments(input)?;
    let frag = peek.fragment();
    if starts_with_keyword(frag, b"accept") {
        return control_node_payload_stmt(input, b"accept", "accept");
    }
    if starts_with_keyword(frag, b"send") {
        return control_node_payload_stmt(input, b"send", "send");
    }
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Tag,
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_payload_uses_arena_backed_qualified_reference() {
        let source = crate::parser::span::test_input("signal : $::Signals::Command");
        let (rest, payload) = typed_payload_clause(source).expect("typed payload");
        assert!(rest.fragment().is_empty());
        assert_eq!(
            payload
                .type_name
                .and_then(|id| crate::parser::usage::reference_text(source, id))
                .as_deref(),
            Some("$::Signals::Command")
        );
    }
}
