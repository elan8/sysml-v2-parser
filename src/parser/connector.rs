//! Shared connector-end grammar (BNF `ConnectorEnd`/`InterfaceEnd`, §8.2.2.13.2/§8.2.2.14.2 --
//! both productions are structurally identical: optional cross multiplicity, optional declared
//! name, `OwnedReferenceSubsetting`) used by both `connection.rs` (`ConnectionDefinition`) and
//! `interface.rs` (`InterfaceDefinition`).
//!
//! GH-33: `connection.rs` and `interface.rs` independently implemented the same seven functions
//! (`end_decl`, `ref_body`, `ref_decl`, `connect_body`, a connection-end wrapper, `connect_ends`,
//! `connect_stmt`) with only cosmetic naming differences. That duplication had already cost real
//! work once (#19 required the same reference-subsetting fix twice, once per file) and, worse,
//! had silently drifted into three genuine behavior gaps neither file's own tests caught:
//!
//! - `connection.rs`'s typed end form (`end name : Type;`) never accepted the `~` conjugated-type
//!   prefix that `interface.rs`'s did (e.g. `end p1 : ~PowerPort;`, real usage in
//!   `KitchenTimer.sysml`/`SurveillanceDrone.sysml`) -- no BNF basis for the restriction, since
//!   `ConnectorEnd` and `InterfaceEnd` share the same production.
//! - `interface.rs`'s `connect_ends` never accepted the §6 G24 per-endpoint multiplicity
//!   (`connect [0..1] a to [1] b;`) that `connection.rs`'s did -- same reasoning, no BNF basis for
//!   restricting it to connections only.
//! - `connection.rs`'s end name accepts the `#name` derived-end-name form (tested in
//!   `tests/derivation_connections.rs`, e.g. `end #original ::> OriginalReq;`); `interface.rs`
//!   never did. Unlike the two gaps above, this one has no matching real-usage evidence on the
//!   interface side, so it stays parameterized (`allow_derived_name`) rather than blindly widened.
//!
//! Consolidating here fixes the first two gaps for both callers at once and makes the third an
//! explicit, visible choice at each call site instead of an accidental omission.

use crate::ast::{ConnectBody, ConnectStmt, ConnectionEnd, EndDecl, Node, RefBody, RefDecl};
use crate::parser::body::advance_to_closing_brace;
use crate::parser::expr::path_expression;
use crate::parser::lex::{name, qualified_name, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::usage::{
    multiplicity_node, redefinition, reference_subsetting, single_target_typing, subsetting,
};
use crate::parser::with_span;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// `#name` derived-end-name form, e.g. `end #original ::> OriginalReq;` (real usage:
/// `tests/derivation_connections.rs`, KerML `Derivation`-style connections). Only reachable when
/// [`end_decl`] is called with `allow_derived_name: true`.
fn derived_end_name(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = tag(&b"#"[..]).parse(input)?;
    let (input, value) =
        take_while1(|c: u8| c.is_ascii_alphanumeric() || c == b'_').parse(input)?;
    Ok((
        input,
        format!("#{}", String::from_utf8_lossy(value.fragment())),
    ))
}

/// End declaration: `end` multiplicity? (`part`|`port`|`ref`)? name (`:` (`~`)? type |
/// (`::>`|`references`) target) multiplicity? `;`.
///
/// `allow_derived_name` gates the `#name` form above -- `true` for `connection.rs` (tested real
/// usage), `false` for `interface.rs` (no matching evidence; preserves existing behavior exactly).
pub(crate) fn end_decl(
    input: Input<'_>,
    allow_derived_name: bool,
) -> IResult<Input<'_>, Node<EndDecl>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"end"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    // GH-51: a leading multiplicity may precede the kind keyword/name (BNF `ConnectorEnd`'s
    // `OwnedCrossMultiplicityMember` position), e.g. `end [1] part bead : TireBead;` (Systems
    // Library training `09. Connections/Connections Example.sysml`), `end [*] ref cause:
    // Situation;` (OMG spec Annex `14c-Language Extensions.sysml`). Distinct from the trailing
    // multiplicity parsed after the type/reference target below (e.g. `end touchesToo [0..*]
    // item ...` in `Items.sysml` -- both positions occur in real usage).
    let (input, leading_multiplicity) =
        opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    // Optional structural kind keyword (BNF `InterfaceOccurrenceUsageElement`/
    // `StructureUsageElement`, e.g. `end part hub : Hub;`, `end port p1 : P;`, `end [*] ref
    // cause: Situation;`). Not retained as a separate field: none of `part`/`port`/`ref` change
    // this end's own grammar (name + type/reference), and `EndDecl` doesn't model usage-kind
    // distinctions (GH-19/GH-20: real-world `end part hub ::> mainSwitch[1];` examples were
    // rejected before this widened to also cover `part`; GH-51 widened it further to `ref`).
    let (input, _) = opt(preceded(
        ws_and_comments,
        alt((
            (tag(&b"part"[..]), ws1),
            (tag(&b"port"[..]), ws1),
            (tag(&b"ref"[..]), ws1),
        )),
    ))
    .parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (name_span, name_str)) = if allow_derived_name {
        with_span(|input| alt((derived_end_name, name)).parse(input)).parse(input)?
    } else {
        with_span(name).parse(input)?
    };

    // `::>` / `references` reference subsetting (GH-19): the target is a reference, not a type, so
    // it's modeled via the same structured `SubsettingRelationship` every other reference-
    // subsetting clause uses -- not folded into `type_name`/typing like the `:` form below.
    if let Ok((input, references)) = reference_subsetting(input) {
        // Trailing multiplicity on the reference target (e.g. `::> mainSwitch[1]`) is parsed the
        // same way `part_usage_redefines_only`/other subsetting-family callers do: the shared
        // `reference_subsetting` parser only consumes the qualified target, so any multiplicity
        // is a separate, subsequent optional parse here (GH-20 real-world example).
        let (input, trailing_multiplicity) =
            opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
        let type_ref_span = references.value.span.clone();
        let type_name = references.value.target_display();
        return Ok((
            input,
            node_from_to(
                start,
                input,
                EndDecl {
                    name: name_str,
                    type_name,
                    uses_derived_syntax: true,
                    references: Some(references),
                    multiplicity: trailing_multiplicity.or(leading_multiplicity),
                    redefines: None,
                    name_span: Some(name_span),
                    type_ref_span: Some(type_ref_span),
                },
            ),
        ));
    }

    let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
    let (input, (tilde, (type_ref_span, type_name))) = preceded(
        ws_and_comments,
        (opt(tag(&b"~"[..])), with_span(qualified_name)),
    )
    .parse(input)?;
    let type_name = if tilde.is_some() {
        format!("~{type_name}")
    } else {
        type_name
    };
    let (input, trailing_multiplicity) =
        opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    // GH-51: `:>>` redefines may trail the typed form, e.g. `end source: Anything :>>
    // BinaryLinkObject::source;` (Systems Library `Connections.sysml`) -- distinct from the
    // `::>`/`references` form used *instead* of `:` typing above.
    let (input, redefines) = opt(preceded(ws_and_comments, redefinition)).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            EndDecl {
                name: name_str,
                type_name,
                uses_derived_syntax: false,
                references: None,
                multiplicity: trailing_multiplicity.or(leading_multiplicity),
                redefines,
                name_span: Some(name_span),
                type_ref_span: Some(type_ref_span),
            },
        ),
    ))
}

/// Ref body: `;` or `{` ... `}`.
pub(crate) fn ref_body(input: Input<'_>) -> IResult<Input<'_>, RefBody> {
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

/// Ref declaration: `ref` (`part`|`port`|`item`)? name? multiplicity? (`:>>` redefines)? (`:`
/// type)? (`nonunique`|`ordered`)* body.
///
/// GH-51: previously required a name and a `:` type unconditionally, with no redefines or
/// multiplicity support at all -- real usage needs both optional (mirrors
/// `action::action_ref_decl`'s equivalent widening, adapted for the `part`/`port`/`item` kind
/// keywords this context uses instead of `action`): `ref port :>> participant : Port [2..*]
/// nonunique ordered { ... }` (Systems Library `Interfaces.sysml`, no name at all -- the
/// redefinition target stands in for it).
pub(crate) fn ref_decl(input: Input<'_>) -> IResult<Input<'_>, Node<RefDecl>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"ref"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = opt(preceded(
        ws_and_comments,
        alt((
            (tag(&b"part"[..]), ws1),
            (tag(&b"port"[..]), ws1),
            (tag(&b"item"[..]), ws1),
            (tag(&b"requirement"[..]), ws1),
        )),
    ))
    .parse(input)?;
    // `ref :>> name ...` (redefinition) may omit the name before `:>>`.
    let (input, parsed_name) = opt(preceded(ws_and_comments, with_span(name))).parse(input)?;
    let (input, _) = opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    let (name_span, name_str) = parsed_name.unwrap_or((crate::ast::Span::dummy(), String::new()));

    // `redefinition` (shared with every other `:>>` call site) already handles the comma-separated
    // multi-target form, e.g. `ref port :>> Interface::participant, BinaryConnection::
    // participant[2] nonunique ordered;` (`Interfaces.sysml`).
    let (input, redefines) = opt(preceded(ws_and_comments, redefinition)).parse(input)?;

    let (input, type_ref_span, type_name, typing) = {
        let (peek, _) = ws_and_comments(input)?;
        if peek.fragment().starts_with(b":") && !peek.fragment().starts_with(b":>") {
            let (input, (type_ref_span, type_name)) = preceded(
                ws_and_comments,
                preceded(tag(&b":"[..]), with_span(qualified_name)),
            )
            .parse(input)?;
            let typing = Some(single_target_typing(
                type_ref_span.clone(),
                type_name.clone(),
            ));
            (input, Some(type_ref_span), type_name, typing)
        } else {
            (input, None, String::new(), None)
        }
    };
    let (input, _) = opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    // `:>` subsets, independent of and in addition to `:>>` redefines, e.g. `ref requirement
    // originalRequirement[1] :>> originalRequirements :> participant { ... }` (Systems Library
    // `Domain Libraries/Requirement Derivation/DerivationConnections.sysml`).
    let (input, subsets) = opt(preceded(ws_and_comments, subsetting)).parse(input)?;
    let subsets = subsets.map(|(target, _value)| target);
    // `nonunique`/`ordered` feature modifiers (real usage: `Interfaces.sysml`'s `ref port :>>
    // participant : Port [2..*] nonunique ordered { ... }`); accepted and discarded, matching
    // `RefDecl`'s existing "don't model every shorthand" scope (see `value`'s doc comment for the
    // same rationale on the binding form).
    let (input, _) = nom::multi::many0(preceded(
        ws_and_comments,
        alt((tag(&b"nonunique"[..]), tag(&b"ordered"[..]))),
    ))
    .parse(input)?;
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
                subsets,
                redefines,
                value: None,
                body,
                name_span: Some(name_span),
                type_ref_span,
                membership: crate::ast::Membership::feature(None, crate::ast::Span::dummy()),
            },
        ),
    ))
}

/// Connect body: `;` or `{` ... `}`.
pub(crate) fn connect_body(input: Input<'_>) -> IResult<Input<'_>, ConnectBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b";"[..]), |_| ConnectBody::Semicolon),
        map(
            nom::sequence::delimited(
                tag(&b"{"[..]),
                advance_to_closing_brace,
                preceded(ws_and_comments, tag(&b"}"[..])),
            ),
            |_| ConnectBody::Brace,
        ),
    ))
    .parse(input)
}

/// Wrap a parsed endpoint expression in a `ConnectionEnd` node with no multiplicity, reusing the
/// expression's own span (see `ast::core::ConnectionEnd`'s doc comment).
fn connection_end(expr: Node<crate::ast::Expression>) -> Node<ConnectionEnd> {
    connection_end_with_multiplicity(None, expr)
}

/// [`connection_end`] for the §6 G24 `connect [0..1] a to [1] b;` form. The endpoint's span still
/// comes from the expression alone, so a multiplicity-bearing endpoint reports the same range as
/// the bare one.
pub(crate) fn connection_end_with_multiplicity(
    multiplicity: Option<Node<crate::ast::Multiplicity>>,
    expr: Node<crate::ast::Expression>,
) -> Node<ConnectionEnd> {
    let span = expr.span.clone();
    Node::new(
        span.clone(),
        ConnectionEnd {
            expression: expr,
            multiplicity,
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

/// Connect ends: the n-ary `'(' end (',' end)+ ')'` form (`NaryConnectorPart`/`NaryInterfacePart`
/// -- structurally identical), or the ordinary binary `from ... to ...` form, with §6 G24
/// per-endpoint multiplicity on the binary form. Returns `(from, to, extra_ends)`.
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
                // §6 G24: each binary endpoint may carry its own multiplicity.
                opt(preceded(ws_and_comments, multiplicity_node)),
                path_expression,
                preceded(ws_and_comments, tag(&b"to"[..])),
                opt(preceded(ws_and_comments, multiplicity_node)),
                preceded(ws_and_comments, path_expression),
            ),
            |(from_mult, from, _, to_mult, to)| {
                (
                    connection_end_with_multiplicity(from_mult, from),
                    connection_end_with_multiplicity(to_mult, to),
                    Vec::new(),
                )
            },
        ),
    ))
    .parse(input)
}

/// Connect statement: `connect` from `to` to body, or `connect` `(` a `,` b (`,` c)* `)` body.
pub(crate) fn connect_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<ConnectStmt>> {
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
