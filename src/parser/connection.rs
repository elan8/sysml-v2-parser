//! Connection definition parsing (BNF ConnectionDefinition).
#![allow(dead_code, unused_imports)]

use crate::ast::{
    ConnectStmt, ConnectionDef, ConnectionDefBody, ConnectionDefBodyElement, ConnectionEnd,
    EndDecl, Node, RefBody, RefDecl,
};
use crate::parser::body::{advance_to_closing_brace, parse_structured_brace_members};
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::expr::path_expression;
use crate::parser::lex::{
    identification, name, qualified_name, take_until_terminator, ws1, ws_and_comments,
    CONNECTION_DEF_BODY_STARTERS,
};
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::node_from_to;
use crate::parser::requirement::doc_comment;
use crate::parser::with_span;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

fn derived_end_name(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = tag(&b"#"[..]).parse(input)?;
    let (input, value) =
        take_while1(|c: u8| c.is_ascii_alphanumeric() || c == b'_').parse(input)?;
    Ok((
        input,
        format!("#{}", String::from_utf8_lossy(value.fragment())),
    ))
}

fn end_decl(input: Input<'_>) -> IResult<Input<'_>, Node<EndDecl>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"end"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, (name_span, name_str)) =
        with_span(|input| alt((derived_end_name, name)).parse(input)).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, uses_derived_syntax) = if let Ok((input, _)) =
        tag::<_, _, nom::error::Error<Input<'_>>>(&b"::>"[..]).parse(input)
    {
        (input, true)
    } else {
        let (input, _) = tag(&b":"[..]).parse(input)?;
        (input, false)
    };
    let (input, (type_ref_span, type_name)) = if uses_derived_syntax {
        let (input, _) = ws_and_comments(input)?;
        let start_type = input;
        let (input, value) =
            take_while1(|c: u8| c != b';' && c != b'\n' && c != b'\r').parse(input)?;
        let type_name = String::from_utf8_lossy(value.fragment()).trim().to_string();
        let span = crate::ast::Span {
            offset: start_type.location_offset(),
            line: start_type.location_line(),
            column: start_type.get_column(),
            len: value.fragment().len(),
        };
        (input, (span, type_name))
    } else {
        preceded(ws_and_comments, with_span(qualified_name)).parse(input)?
    };
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            EndDecl {
                name: name_str,
                type_name,
                uses_derived_syntax,
                name_span: Some(name_span),
                type_ref_span: Some(type_ref_span),
            },
        ),
    ))
}

fn ref_body(input: Input<'_>) -> IResult<Input<'_>, RefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b";"[..]), |_| RefBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag(&b"{"[..]),
                advance_to_closing_brace,
                preceded(ws_and_comments, tag(&b"}"[..])),
            ),
            |_| RefBody::Brace { elements: vec![] },
        ),
    ))
    .parse(input)
}

fn ref_decl(input: Input<'_>) -> IResult<Input<'_>, Node<RefDecl>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"ref"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, (name_span, name_str)) = with_span(name).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
    let (input, (type_ref_span, type_name)) =
        preceded(ws_and_comments, with_span(qualified_name)).parse(input)?;
    let (input, body) = ref_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            RefDecl {
                name: name_str,
                type_name,
                value: None,
                body,
                name_span: Some(name_span),
                type_ref_span: Some(type_ref_span),
            },
        ),
    ))
}

fn connect_body(input: Input<'_>) -> IResult<Input<'_>, crate::ast::ConnectBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b";"[..]), |_| crate::ast::ConnectBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag(&b"{"[..]),
                advance_to_closing_brace,
                preceded(ws_and_comments, tag(&b"}"[..])),
            ),
            |_| crate::ast::ConnectBody::Brace,
        ),
    ))
    .parse(input)
}

/// Wrap a parsed endpoint expression in a `ConnectionEnd` node, reusing the expression's own
/// span (`path_expression` already tracks a real span, so the endpoint's span and the inner
/// expression's span are identical today -- see `ast::core::ConnectionEnd`'s doc comment).
fn connection_end(expr: Node<crate::ast::Expression>) -> Node<ConnectionEnd> {
    let span = expr.span.clone();
    Node::new(
        span.clone(),
        ConnectionEnd {
            expression: expr,
            span,
        },
    )
}

/// Connect ends: the n-ary `'(' end (',' end)+ ')'` form (`NaryConnectorPart`), or the ordinary
/// binary `from ... to ...` form. Returns `(from, to, extra_ends)`.
fn connect_ends(
    input: Input<'_>,
) -> IResult<Input<'_>, (Node<ConnectionEnd>, Node<ConnectionEnd>, Vec<Node<ConnectionEnd>>)> {
    alt((
        map(
            (
                preceded(ws_and_comments, tag(&b"("[..])),
                preceded(ws_and_comments, path_expression),
                nom::multi::many1(preceded(
                    preceded(ws_and_comments, tag(&b","[..])),
                    preceded(ws_and_comments, path_expression),
                )),
                preceded(ws_and_comments, tag(&b")"[..])),
            ),
            |(_, first, mut rest, _)| {
                let to = rest.remove(0);
                (
                    connection_end(first),
                    connection_end(to),
                    rest.into_iter().map(connection_end).collect(),
                )
            },
        ),
        map(
            (
                path_expression,
                preceded(ws_and_comments, tag(&b"to"[..])),
                preceded(ws_and_comments, path_expression),
            ),
            |(from, _, to)| (connection_end(from), connection_end(to), Vec::new()),
        ),
    ))
    .parse(input)
}

fn connect_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<ConnectStmt>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"connect"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, (from_expr, to_expr, extra_ends)) = connect_ends(input)?;
    let (input, body) = connect_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ConnectStmt {
                from: from_expr,
                to: to_expr,
                extra_ends,
                body,
            },
        ),
    ))
}

fn connection_def_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<ConnectionDefBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, elem) = alt((
        map(end_decl, ConnectionDefBodyElement::EndDecl),
        map(ref_decl, ConnectionDefBodyElement::RefDecl),
        map(connect_stmt, ConnectionDefBodyElement::ConnectStmt),
        map(doc_comment, ConnectionDefBodyElement::Doc),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn connection_def_body_recovery(
    start: Input<'_>,
    end: Input<'_>,
) -> Node<ConnectionDefBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        CONNECTION_DEF_BODY_STARTERS,
        "connection definition body",
        "recovered_connection_def_body_element",
    );
    node_from_to(
        start,
        end,
        ConnectionDefBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

pub(crate) fn connection_member_body(input: Input<'_>) -> IResult<Input<'_>, ConnectionDefBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, ConnectionDefBody::Semicolon));
    }
    let (input, elements) = parse_structured_brace_members(
        input,
        CONNECTION_DEF_BODY_STARTERS,
        "connection definition body",
        "recovered_connection_def_body_element",
        connection_def_body_element,
        connection_def_body_recovery,
    )?;
    Ok((input, ConnectionDefBody::Brace { elements }))
}

/// Connection definition: `connection def` Identification body.
///
/// `def` is intentionally optional: a leading `#annotation` (e.g. `#derivation connection { ...
/// }`) is itself a valid definitional marker in place of `def`, per the hash-annotation forms
/// used for derivation/satisfy/requirement-style connections. `connection_def` is only dispatched
/// at package top level today, where nothing else shares the `connection` keyword, so this is
/// not the PAR-001 bug class — do not add `.def_required()` here without also accounting for the
/// annotation-prefixed def-less form.
pub(crate) fn connection_def(input: Input<'_>) -> IResult<Input<'_>, Node<ConnectionDef>> {
    parse_connection_def(input, DefinitionPrefixOptions::new(b"connection").with_hash_annotation())
}

/// Connection definition with required `def` keyword, for contexts (e.g. nested inside a part
/// definition body) where a bare `connection` usage form (`connection_usage_member`) is already
/// dispatched separately -- requiring `def` here prevents a `def`-less connection usage from
/// being misclassified as a definition, the same bug class as PAR-001 in `attribute_def`. Does
/// not support the hash-annotation def-less form ([`connection_def`] does); nothing in the
/// nested-part-body grammar currently needs that combination.
pub(crate) fn connection_def_required(input: Input<'_>) -> IResult<Input<'_>, Node<ConnectionDef>> {
    parse_connection_def(input, DefinitionPrefixOptions::new(b"connection").def_required())
}

fn parse_connection_def(
    input: Input<'_>,
    options: DefinitionPrefixOptions,
) -> IResult<Input<'_>, Node<ConnectionDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(input, options)?;
    let (input, body) = connection_member_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ConnectionDef {
                annotation: prefix.annotation,
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
            },
        ),
    ))
}
