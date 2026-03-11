//! Action definition and action usage parsing (function-based behavior).

use crate::ast::{
    ActionDef, ActionDefBody, ActionUsage, ActionUsageBody, ActionUsageBodyElement, FirstMergeBody,
    FirstStmt, Flow, InOut, InOutDecl, MergeStmt,
};
use crate::parser::expr::path_expression;
use crate::parser::interface::connect_body;
use crate::parser::lex::{identification, name, qualified_name, skip_until_brace_end, ws1, ws_and_comments};
use crate::parser::part::bind_;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

/// First/merge body: `;` or `{` ... `}`
fn first_merge_body(input: &[u8]) -> IResult<&[u8], FirstMergeBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(";"), |_| FirstMergeBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag("{"),
                skip_until_brace_end,
                preceded(ws_and_comments, tag("}")),
            ),
            |_| FirstMergeBody::Brace,
        ),
    ))(input)
}

/// In/out decl: `in` name `:` type `;` or `out` name `:` type `;`
fn in_out_decl(input: &[u8]) -> IResult<&[u8], InOutDecl> {
    let (input, _) = ws_and_comments(input)?;
    let (input, direction) = alt((
        map(preceded(tag("in"), ws1), |_| InOut::In),
        map(preceded(tag("out"), ws1), |_| InOut::Out),
    ))(input)?;
    let (input, param_name) = name(input)?;
    let (input, _) = preceded(ws_and_comments, tag(":"))(input)?;
    let (input, type_name) = preceded(ws_and_comments, qualified_name)(input)?;
    let (input, _) = preceded(ws_and_comments, tag(";"))(input)?;
    Ok((
        input,
        InOutDecl {
            direction,
            name: param_name,
            type_name,
        },
    ))
}

/// Action def body: `;` or `{` InOutDecl* `}`
fn action_def_body(input: &[u8]) -> IResult<&[u8], ActionDefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(";"), |_| ActionDefBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag("{"),
                preceded(
                    ws_and_comments,
                    many0(preceded(ws_and_comments, in_out_decl)),
                ),
                preceded(ws_and_comments, tag("}")),
            ),
            |elements| ActionDefBody::Brace { elements },
        ),
    ))(input)
}

/// Action definition: `action` `def` Identification body
pub(crate) fn action_def(input: &[u8]) -> IResult<&[u8], ActionDef> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("action")(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag("def")(input)?;
    let (input, _) = ws1(input)?;
    let (input, identification) = identification(input)?;
    let (input, body) = action_def_body(input)?;
    Ok((
        input,
        ActionDef {
            identification,
            body,
        },
    ))
}

/// Flow: `flow` path `to` path body
fn flow_(input: &[u8]) -> IResult<&[u8], Flow> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("flow")(input)?;
    let (input, _) = ws1(input)?;
    let (input, from_expr) = path_expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag("to"))(input)?;
    let (input, to_expr) = preceded(ws_and_comments, path_expression)(input)?;
    let (input, body) = connect_body(input)?;
    Ok((
        input,
        Flow {
            from: from_expr,
            to: to_expr,
            body,
        },
    ))
}

/// First stmt: `first` path `then` path body
fn first_stmt(input: &[u8]) -> IResult<&[u8], FirstStmt> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("first")(input)?;
    let (input, _) = ws1(input)?;
    let (input, first_expr) = path_expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag("then"))(input)?;
    let (input, then_expr) = preceded(ws_and_comments, path_expression)(input)?;
    let (input, body) = first_merge_body(input)?;
    Ok((
        input,
        FirstStmt {
            first: first_expr,
            then: then_expr,
            body,
        },
    ))
}

/// Merge stmt: `merge` path body
fn merge_stmt(input: &[u8]) -> IResult<&[u8], MergeStmt> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("merge")(input)?;
    let (input, _) = ws1(input)?;
    let (input, merge_expr) = path_expression(input)?;
    let (input, body) = first_merge_body(input)?;
    Ok((
        input,
        MergeStmt {
            merge: merge_expr,
            body,
        },
    ))
}

/// Action usage body: `;` or `{` ActionUsageBodyElement* `}`
fn action_usage_body(input: &[u8]) -> IResult<&[u8], ActionUsageBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(";"), |_| ActionUsageBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag("{"),
                preceded(
                    ws_and_comments,
                    many0(preceded(ws_and_comments, action_usage_body_element)),
                ),
                preceded(ws_and_comments, tag("}")),
            ),
            |elements| ActionUsageBody::Brace { elements },
        ),
    ))(input)
}

/// Action usage body element: InOutDecl | Bind | Flow | FirstStmt | MergeStmt | ActionUsage
fn action_usage_body_element(input: &[u8]) -> IResult<&[u8], ActionUsageBodyElement> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(in_out_decl, ActionUsageBodyElement::InOutDecl),
        map(bind_, ActionUsageBodyElement::Bind),
        map(flow_, ActionUsageBodyElement::Flow),
        map(first_stmt, ActionUsageBodyElement::FirstStmt),
        map(merge_stmt, ActionUsageBodyElement::MergeStmt),
        map(action_usage, |a| ActionUsageBodyElement::ActionUsage(Box::new(a))),
    ))(input)
}

/// Action usage: `action` name ( `:` type_name ( `accept` param `:` param_type )? | `accept` param_name `:` param_type ) body
pub(crate) fn action_usage(input: &[u8]) -> IResult<&[u8], ActionUsage> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag("action")(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, (type_name, accept)) = nom::branch::alt((
        // `:` type_name then optional `accept` param `:` param_type
        nom::combinator::map(
            nom::sequence::tuple((
                preceded(ws_and_comments, tag(":")),
                preceded(ws_and_comments, name),
                nom::combinator::opt(preceded(
                    preceded(ws_and_comments, tag("accept")),
                    preceded(ws1, nom::sequence::tuple((
                        name,
                        preceded(ws_and_comments, tag(":")),
                        preceded(ws_and_comments, qualified_name),
                    ))),
                )),
            )),
            |(_, type_name, accept)| (type_name, accept.map(|(pn, _, tn)| (pn, tn))),
        ),
        // `accept` param_name `:` param_type (no type_name before; type_name = param_type)
        nom::combinator::map(
            preceded(
                preceded(ws_and_comments, tag("accept")),
                preceded(
                    ws1,
                    nom::sequence::tuple((
                        name,
                        preceded(ws_and_comments, tag(":")),
                        preceded(ws_and_comments, name),
                    )),
                ),
            ),
            |(param_name, _, param_type)| (param_type.clone(), Some((param_name, param_type))),
        ),
    ))(input)?;
    let (input, body) = action_usage_body(input)?;
    Ok((
        input,
        ActionUsage {
            name: name_str,
            type_name,
            accept: accept,
            body,
        },
    ))
}
