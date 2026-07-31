//! Shared parsers for accept/send payload clauses and transition accept triggers.

use crate::ast::{ActionUsage, Expression, Node, PayloadClause, TransitionAccept, TriggerKind};
use crate::parser::action::action_usage_body;
use crate::parser::expr::expression;
use crate::parser::lex::{name, qualified_name, starts_with_keyword, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::with_span;
use crate::parser::Input;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// Required typed payload: `name : qualified_name`.
pub(crate) fn typed_payload_clause(input: Input<'_>) -> IResult<Input<'_>, PayloadClause> {
    let (input, (name_span, name)) = with_span(name).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
    let (input, (type_span, type_name)) =
        preceded(ws_and_comments, with_span(qualified_name)).parse(input)?;
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
    let (input, expr_node) = expression(input)?;
    let (input, type_suffix) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, with_span(qualified_name)),
    ))
    .parse(input)?;
    let (input, via) = opt(preceded(
        preceded(ws_and_comments, tag(&b"via"[..])),
        preceded(ws1, expression),
    ))
    .parse(input)?;
    if let (Expression::FeatureRef(name), Some((type_span, type_name))) =
        (&expr_node.value, type_suffix)
    {
        return Ok((
            input,
            TransitionAccept::Payload(
                PayloadClause {
                    name: name.clone(),
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
fn control_node_payload_stmt<'a>(
    input: Input<'a>,
    keyword: &'a [u8],
    control_name: &'static str,
) -> IResult<Input<'a>, Node<ActionUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(keyword).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, payload) = typed_payload_clause(input)?;
    let name_span = payload.name_span.clone();
    let type_ref_span = payload.type_span.clone();
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = action_usage_body(input)?;
    let (input, _) =
        nom::combinator::opt(preceded(ws_and_comments, tag(&b";"[..]))).parse(input)?;
    let (accept, send) = if keyword == b"accept" {
        (Some(payload), None)
    } else {
        (None, Some(payload))
    };
    Ok((
        input,
        node_from_to(
            start,
            input,
            ActionUsage {
                is_abstract: false,
                is_variation: false,
                is_reference: false,
                name: control_name.to_string(),
                type_name: String::new(),
                typing: None,
                multiplicity: None,
                subsets: None,
                redefines: None,
                accept,
                send,
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
