//! Shared parsing for `FeatureValue` (BNF): `= expr` | `:= expr` | `default = expr` |
//! `default := expr` | `default expr`. Used by both `attribute.rs` (def/usage attribute value
//! clauses) and `part::usage` (part-usage and ref-decl value clauses), which previously had
//! near-duplicate copies (`value_part` / `usage_value_part`) that syntactically distinguished the
//! five forms via `alt()` but discarded which one matched before calling `expression()`.

use crate::ast::{Expression, FeatureValue, FeatureValueKind, Node};
use crate::parser::expr::expression;
use crate::parser::lex::{ws1, ws_and_comments};
use crate::parser::{node_from_to, Input};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// Which operator (if any) introduced the clause, and whether `default` prefixed it.
#[derive(Clone, Copy)]
struct FeatureValuePrefix {
    kind: FeatureValueKind,
    is_default: bool,
    has_operator: bool,
}

/// Parses the `default`? (`=` | `:=`)? prefix and returns which form matched, without consuming
/// the value expression.
fn feature_value_prefix(input: Input<'_>) -> IResult<Input<'_>, FeatureValuePrefix> {
    alt((
        // `= expr`
        preceded(
            tag(&b"="[..]),
            nom::combinator::value(
                FeatureValuePrefix {
                    kind: FeatureValueKind::Bind,
                    is_default: false,
                    has_operator: true,
                },
                ws_and_comments,
            ),
        ),
        // `:= expr`
        preceded(
            tag(&b":="[..]),
            nom::combinator::value(
                FeatureValuePrefix {
                    kind: FeatureValueKind::Assign,
                    is_default: false,
                    has_operator: true,
                },
                ws_and_comments,
            ),
        ),
        // `default` (`=` | `:=`)? expr
        preceded(
            preceded(tag(&b"default"[..]), ws1),
            alt((
                preceded(
                    tag(&b"="[..]),
                    nom::combinator::value(
                        FeatureValuePrefix {
                            kind: FeatureValueKind::Bind,
                            is_default: true,
                            has_operator: true,
                        },
                        ws_and_comments,
                    ),
                ),
                preceded(
                    tag(&b":="[..]),
                    nom::combinator::value(
                        FeatureValuePrefix {
                            kind: FeatureValueKind::Assign,
                            is_default: true,
                            has_operator: true,
                        },
                        ws_and_comments,
                    ),
                ),
                nom::combinator::value(
                    FeatureValuePrefix {
                        kind: FeatureValueKind::Bind,
                        is_default: true,
                        has_operator: false,
                    },
                    ws_and_comments,
                ),
            )),
        ),
    ))
    .parse(input)
}

/// Wraps a bare expression value that came from a grammar production *other* than `FeatureValue`
/// where only a plain `=` is recognized (never `:=` or `default`) -- e.g. the
/// `subsets target = expr` shorthand's optional value, or the handful of ad hoc `ref` value
/// parses (action/state/part-usage `ref` bodies) that predate this shared type and only ever
/// matched bare `=`. Produces a [`FeatureValueKind::Bind`], non-`default` [`FeatureValue`] so
/// those call sites can still populate a unified `value: Option<Node<FeatureValue>>` field. Uses
/// the expression node's own span, since these productions only have the expression to point at.
pub(crate) fn wrap_bind_expression(expr: Node<Expression>) -> Node<FeatureValue> {
    let span = expr.span.clone();
    Node::new(
        span.clone(),
        FeatureValue {
            kind: FeatureValueKind::Bind,
            is_default: false,
            has_operator: true,
            expression: expr,
            span,
        },
    )
}

/// Value part (BNF `FeatureValue`): `= expr` | `:= expr` | `default = expr` | `default := expr` |
/// `default expr`. Shared by attribute def/usage, part usage, and ref decl value clauses.
pub(crate) fn feature_value_part(input: Input<'_>) -> IResult<Input<'_>, Node<FeatureValue>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, prefix) = feature_value_prefix(input)?;
    // A `{ ... }` after the prefix is a standalone KerML `BodyExpression` value, e.g. the pin
    // initializers `default {true}` / `default {false}` (Systems Library `Actions.sysml`) --
    // previously consumed as an opaque brace and discarded by `in_out_decl`.
    let (peek, _) = ws_and_comments(input)?;
    let (input, expression): (Input<'_>, Node<Expression>) = if peek.fragment().starts_with(b"{") {
        let (input, body) = crate::parser::expr::body_expression(input)?;
        let span = body.span.clone();
        (input, Node::new(span, Expression::BodyExpr(Box::new(body))))
    } else {
        expression(input)?
    };
    Ok((
        input,
        node_from_to(
            start,
            input,
            FeatureValue {
                kind: prefix.kind,
                is_default: prefix.is_default,
                has_operator: prefix.has_operator,
                expression,
                span: crate::parser::span_from_to(start, input),
            },
        ),
    ))
}
