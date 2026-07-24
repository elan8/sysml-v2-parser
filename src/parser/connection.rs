//! Connection definition parsing (BNF ConnectionDefinition).

use crate::ast::{
    ConnectStmt, ConnectionDef, ConnectionDefBody, ConnectionDefBodyElement, ConnectionEnd,
    EndDecl, Node, RefBody, RefDecl,
};
use crate::parser::attribute::{attribute_def, attribute_usage};
use crate::parser::body::{advance_to_closing_brace, parse_structured_brace_members};
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::expr::path_expression;
use crate::parser::item::{item_def_required, item_usage};
use crate::parser::lex::{
    name, qualified_name, ws1, ws_and_comments, CONNECTION_DEF_BODY_STARTERS,
};
use crate::parser::node_from_to;
use crate::parser::port::{port_def_required, port_usage};
use crate::parser::requirement::doc_comment;
use crate::parser::with_span;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::map;
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
    let typing = Some(crate::parser::usage::single_target_typing(
        type_ref_span.clone(),
        type_name.clone(),
    ));
    let (input, body) = ref_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            RefDecl {
                name: name_str,
                type_name,
                typing,
                redefines: None,
                value: None,
                body,
                name_span: Some(name_span),
                type_ref_span: Some(type_ref_span),
                membership: crate::ast::Membership::feature(None, crate::ast::Span::dummy()),
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

/// `(from, to, extra_ends)` for a parsed connect statement.
pub(crate) type ConnectEnds = (
    Node<ConnectionEnd>,
    Node<ConnectionEnd>,
    Vec<Node<ConnectionEnd>>,
);

/// Connect ends: the n-ary `'(' end (',' end)+ ')'` form (`NaryConnectorPart`), or the ordinary
/// binary `from ... to ...` form. Returns `(from, to, extra_ends)`.
///
/// `pub(crate)` so `part::body::connection_usage_member` can parse the same inline
/// `connect ... to ...` clause a package-level `connection name : Type connect a to b;` usage
/// needs (PAR-007 widening -- see `connection_def`'s doc comment for why this shape used to be
/// misclassified as a definition instead of reaching this parser).
pub(crate) fn connect_ends(input: Input<'_>) -> IResult<Input<'_>, ConnectEnds> {
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
        // PAR-002 widening: this body previously had no attribute/item/port coverage at all.
        // Same def-before-usage discipline as `InterfaceDefBodyElement`/other body enums.
        map(
            |i| attribute_def(i, true),
            ConnectionDefBodyElement::AttributeDef,
        ),
        map(attribute_usage, ConnectionDefBodyElement::AttributeUsage),
        map(item_def_required, ConnectionDefBodyElement::ItemDef),
        map(item_usage, ConnectionDefBodyElement::ItemUsage),
        map(port_def_required, ConnectionDefBodyElement::PortDef),
        map(port_usage, ConnectionDefBodyElement::PortUsage),
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
/// `def` is intentionally optional, same rationale (and same real-library evidence) as
/// `port_def`/`calc_def`/`constraint_def`: the Systems Library uses bare, `def`-less `connection`
/// declarations at namespace level with `abstract`, multiplicity, `nonunique`, and `:>` subsets
/// before the body (e.g. `abstract connection connections: Connection[0..*] nonunique :>
/// linkObjects, parts { ... }` in `Systems Library/Connections.sysml`), a shape
/// `connection_usage_member` (`src/parser/part/body.rs`) does not parse (no `abstract`/
/// multiplicity/`nonunique` support, and its `:>`/`:>>` handling is trailing-after-body only).
/// `parse_definition_prefix`'s header parsing (`specialization::
/// parse_optional_definition_header_after_identification`, a generic text-scan for `: Type[mult]
/// nonunique :> target`) already accepts this whole shape, and identification's `name` is
/// optional, so `connection_def` is effectively a grammar superset of `connection_usage_member`
/// for every practical package-level input.
///
/// **PAR-006b audit note**: `connection_usage_member` is also dispatched at package level
/// (`package.rs::try_package_body_structure`, right after `connection_def`, added by PAR-002).
/// This was investigated as a possible PAR-001-class def/usage ambiguity (an earlier draft of
/// this comment claimed "nothing else shares the `connection` keyword" at package level, which
/// PAR-002 made stale). Making `def` conditionally required here (mirroring the `connection`
/// keyword inside part bodies, where `connection_def_required` is safe because
/// `connection_usage_member`'s narrower grammar is the *only* usage form dispatched there) was
/// tried and broke `test_systems_library_node_types_no_extended`/
/// `test_full_library_node_types_no_extended` in the `SYSML_V2_RELEASE_DIR` gate: the real bare
/// `abstract connection ... nonunique :> ...` forms above stopped parsing as `ConnectionDef` and,
/// since `connection_usage_member` can't parse them either, fell all the way through to
/// `ExtendedLibraryDecl`. That is a worse outcome than the status quo, not a fix. Do not add
/// `.def_required()` here without first widening `connection_usage_member` to cover
/// `abstract`/multiplicity/`nonunique`/leading `:>` subsets, matching the port/calc/constraint
/// precedent from CHANGELOG 0.33.0.
///
/// **PAR-007 update**: the PAR-006b claim above that "there is no live misclassification bug
/// here" was correct for the shapes it checked, but missed one: `connection link : Link connect
/// a to b;` (a typed connector usage with an inline `connect ... to ...` clause) *was*
/// misclassified. The plain `: Type` header scan (`specialization::
/// parse_optional_definition_header_after_identification`) greedily consumes everything up to
/// `;`/`{` and silently discards it once a leading type name is extracted, so `connection_def`
/// matched this input too -- with an empty body, having swallowed and dropped the `connect`
/// clause entirely -- rather than correctly leaving it for `connection_usage_member`. This is
/// narrower and safer than a `.def_required()` guard: `.reject_header_keyword(b"connect")` only
/// fails the parse when the discarded header text contains a top-level `connect` keyword, which
/// the bare Systems-Library shape above never does, so that regression cannot recur (see
/// `par_006b_audit_tests` below, still green).
pub(crate) fn connection_def(input: Input<'_>) -> IResult<Input<'_>, Node<ConnectionDef>> {
    parse_connection_def(
        input,
        DefinitionPrefixOptions::new(b"connection")
            .with_hash_annotation()
            .with_captured_visibility()
            .reject_header_keyword(b"connect"),
    )
}

/// Connection definition with required `def` keyword, for contexts (e.g. nested inside a part
/// definition body) where a bare `connection` usage form (`connection_usage_member`) is already
/// dispatched separately -- requiring `def` here prevents a `def`-less connection usage from
/// being misclassified as a definition, the same bug class as PAR-001 in `attribute_def`. Does
/// not support the hash-annotation def-less form ([`connection_def`] does); nothing in the
/// nested-part-body grammar currently needs that combination.
pub(crate) fn connection_def_required(input: Input<'_>) -> IResult<Input<'_>, Node<ConnectionDef>> {
    parse_connection_def(
        input,
        DefinitionPrefixOptions::new(b"connection")
            .def_required()
            .with_captured_visibility()
            .reject_header_keyword(b"connect"),
    )
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
                membership: crate::ast::Membership::owning(
                    prefix.visibility,
                    prefix.visibility_span,
                ),
            },
        ),
    ))
}

#[cfg(test)]
mod par_002_widening_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    #[test]
    fn connection_def_body_accepts_nested_attribute_usage() {
        let (rest, node) =
            connection_def_body_element(input("attribute mass: Real;")).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            ConnectionDefBodyElement::AttributeUsage(_)
        ));
    }

    #[test]
    fn connection_def_body_accepts_nested_item_def_not_misparsed_as_usage() {
        let (rest, node) =
            connection_def_body_element(input("item def MyItem;")).expect("item def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, ConnectionDefBodyElement::ItemDef(_)));
    }

    #[test]
    fn connection_def_body_accepts_nested_port_def_not_misparsed_as_usage() {
        let (rest, node) =
            connection_def_body_element(input("port def MyPort;")).expect("port def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, ConnectionDefBodyElement::PortDef(_)));
    }

    #[test]
    fn connection_def_body_accepts_nested_port_usage() {
        let (rest, node) =
            connection_def_body_element(input("port p1: MyPort;")).expect("port usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, ConnectionDefBodyElement::PortUsage(_)));
    }

    /// PAR-002 acceptance criterion: `attribute` usage yields the same underlying parse (via the
    /// shared `attribute_usage` parser, also wired into `PartDefBodyElement`/`PackageBodyElement`
    /// in prior increments) whether reached through `ConnectionDefBodyElement::AttributeUsage` or
    /// any other body enum.
    #[test]
    fn attribute_usage_is_same_variant_kind_in_connection_def_body_as_shared_parser() {
        let text = "attribute mass: Real;";
        let (_, conn_node) =
            connection_def_body_element(input(text)).expect("nested in connection def body");
        assert!(matches!(
            conn_node.value,
            ConnectionDefBodyElement::AttributeUsage(_)
        ));
        let result = attribute_usage(input(text));
        assert!(
            result.is_ok(),
            "attribute_usage should also accept {text:?}"
        );
    }
}

#[cfg(test)]
mod par_006b_audit_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    /// PAR-006b audit: `connection_def` must keep accepting this exact real-Systems-Library shape
    /// (`Systems Library/Connections.sysml`) -- a bare, `def`-less, un-annotated `connection`
    /// usage with `abstract`, multiplicity, `nonunique`, and leading `:>` subsets before the
    /// body. This is the shape that made a `def_required_unless_annotated()` guard on
    /// `connection_def` unsafe (see the doc comment on `connection_def` above): tightening `def`
    /// requirements here without first widening `connection_usage_member` to cover this shape
    /// sends it to `ExtendedLibraryDecl` instead, which is what broke the
    /// `SYSML_V2_RELEASE_DIR` gate (`test_systems_library_node_types_no_extended`) during this
    /// audit. This test exists so any future attempt to tighten `connection_def` fails fast,
    /// locally, and points back at this note instead of only failing the much slower full-library
    /// gate.
    #[test]
    fn connection_def_accepts_the_bare_abstract_multiplicity_nonunique_subsets_form_that_makes_def_required_unsafe(
    ) {
        let text =
            "abstract connection connections: Connection[0..*] nonunique :> linkObjects, parts { }";
        let result = connection_def(input(text));
        assert!(
            result.is_ok(),
            "connection_def should still accept the bare Systems-Library connection form, got {result:?}"
        );
    }
}

#[cfg(test)]
mod membership_tests {
    use super::*;
    use crate::parser::part::connection_usage_member;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    // --- parser work item 4b (continuation): Membership on ConnectionDef/ConnectionUsageMember ---

    /// `connection_usage_member` previously never parsed a `private`/`protected`/`public` prefix
    /// at all (same genuine gap as `part_def`/`port_def`/`port_usage`/`item_def`/`item_usage`).
    #[test]
    fn connection_usage_member_visibility_prefix_is_captured_on_membership() {
        let (_, node) = connection_usage_member(input("private connection c1: MyConnection;"))
            .expect("connection usage member");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Private)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn connection_usage_member_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = connection_usage_member(input("connection c1: MyConnection;"))
            .expect("connection usage member");
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    /// `connection_def`/`connection_def_required` previously never parsed a visibility prefix
    /// either (same genuine gap as `part_def`/`port_def`/`item_def`).
    #[test]
    fn connection_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = connection_def(input("protected connection def MyConnection;"))
            .expect("connection def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Protected)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }

    #[test]
    fn connection_def_public_visibility_prefix_is_captured_on_membership() {
        let (rest, node) =
            connection_def(input("public connection def MyConnection;")).expect("connection def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
    }

    #[test]
    fn connection_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) =
            connection_def(input("connection def MyConnection;")).expect("connection def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }
}
