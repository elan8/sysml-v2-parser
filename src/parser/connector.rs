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
//! - `connection.rs`'s end identity accepts fixed `#original`/`#derive` derivation roles (tested in
//!   `tests/derivation_connections.rs`, e.g. `end #original ::> OriginalReq;`); `interface.rs`
//!   never did. Unlike the two gaps above, this one has no matching real-usage evidence on the
//!   interface side, so it stays parameterized (`allow_derivation_role`) rather than blindly
//!   widened.
//!
//! Consolidating here fixes the first two gaps for both callers at once and makes the third an
//! explicit, visible choice at each call site instead of an accidental omission.

use crate::ast::{
    ConnectBody, ConnectStmt, ConnectionEnd, DerivationEndRole, EndDecl, EndIdentity,
    EndNestedUsage, Node, RefBody, RefBodyElement, RefDecl,
};
use crate::parser::body::{advance_to_closing_brace, relationship_body_annotations};
use crate::parser::expr::path_expression;
use crate::parser::feature_value::feature_value_part;
use crate::parser::item::item_usage;
use crate::parser::lex::{name, qualified_reference, starts_with_keyword, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::occurrence_body::occurrence_usage;
use crate::parser::usage::{
    cross_subsetting, multiplicity_node, redefinition, reference_subsetting, single_target_typing,
    subsetting, typing_node,
};
use crate::parser::with_span;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// Fixed derivation-end role, with its exact authored `#...` marker span.
fn derivation_end_role(input: Input<'_>) -> IResult<Input<'_>, Node<DerivationEndRole>> {
    let start = input;
    let (input, _) = tag(&b"#"[..]).parse(input)?;
    let (input, role) = if starts_with_keyword(input.fragment(), b"original") {
        let (input, _) = tag(&b"original"[..]).parse(input)?;
        (input, DerivationEndRole::Original)
    } else if starts_with_keyword(input.fragment(), b"derive") {
        let (input, _) = tag(&b"derive"[..]).parse(input)?;
        (input, DerivationEndRole::Derive)
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Verify,
        )));
    };
    Ok((input, node_from_to(start, input, role)))
}

/// End declaration: `end` `#tag`? multiplicity? (`part`|`port`|`ref`|`item`)? name multiplicity?
/// (`:` (`~`)? type (`crosses` target)? | (`::>`|`references`) target | nested `occurrence`/`item`
/// usage) multiplicity? `;`.
///
/// `allow_derivation_role` gates the fixed derivation-role form above -- `true` for
/// `connection.rs` (tested real usage), `false` for `interface.rs` (no matching evidence;
/// preserves existing behavior exactly).
pub(crate) fn end_decl(
    input: Input<'_>,
    allow_derivation_role: bool,
) -> IResult<Input<'_>, Node<EndDecl>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"end"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    // GH-85: a `#tag` metadata-prefix annotation may precede the rest of the end declaration,
    // e.g. `end #cause cause1 : Causer1;` (OMG spec Annex `Cause and Effect Examples/
    // CauseAndEffectExample.sysml`), `end #original r1 : Req1;` (`Requirements Examples/
    // RequirementDerivationExample.sysml`). Distinct from `allow_derivation_role`'s fixed-role form
    // below (`end #original ::> OriginalReq;`, `tests/derivation_connections.rs`): there the
    // fixed marker is a derivation role, immediately followed by an operator
    // (`::>`/`:`/`;`/multiplicity); here `#tag` is a separate prefix and a declaration name still
    // follows. The trailing
    // `peek(name)` requires that a name actually follows before committing to the prefix
    // reading, so `#original ::> ...` still falls through to the derivation-role path intact.
    // Discarded like the kind keywords below -- `EndDecl` doesn't model metadata annotations.
    let (input, _) = opt(preceded(
        ws_and_comments,
        (tag(&b"#"[..]), name, ws1, nom::combinator::peek(name)),
    ))
    .parse(input)?;
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
    // cause: Situation;`, `end [0..1] item cart: ShoppingCart[1];` (OMG spec Annex `Association
    // Examples/ProductSelection_N_ary.sysml`, GH-85)). Also KerML/SysML library forms `end
    // feature source: …` (Transfers.kerml) and `end occurrence source: …` (Flows.sysml). Not
    // retained as a separate field: none of these keywords change this end's own grammar (name +
    // type/reference), and `EndDecl` doesn't model usage-kind distinctions (GH-19/GH-20/GH-51;
    // #73 library regression).
    // Nested `end name [*] occurrence/item nestedName …` (GH-53) is unaffected: that kind keyword
    // appears *after* the end's own name there, so it is not consumed by this leading-position
    // optional kind (this alt only fires before a name has been parsed at all).
    let (input, _) = opt(preceded(
        ws_and_comments,
        alt((
            (tag(&b"part"[..]), ws1),
            (tag(&b"port"[..]), ws1),
            (tag(&b"ref"[..]), ws1),
            (tag(&b"feature"[..]), ws1),
            (tag(&b"occurrence"[..]), ws1),
            (tag(&b"item"[..]), ws1),
        )),
    ))
    .parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, identity) = if allow_derivation_role && input.fragment().starts_with(b"#") {
        let (input, role) = derivation_end_role(input)?;
        (input, EndIdentity::Derivation(role))
    } else {
        let (input, (span, declaration_name)) = with_span(name).parse(input)?;
        (
            input,
            EndIdentity::Declaration(Node::new(span, declaration_name)),
        )
    };
    // GH-53: a multiplicity may also appear right after the name, before the target -- the most
    // common position for most usage kinds (e.g. `end touchesToo [0..*] item ...` in
    // `Items.sysml`, `end theCauses [*] occurrence ...` in `CausationConnections.sysml`).
    let (input, mid_multiplicity) =
        opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    let leading_multiplicity = mid_multiplicity.or(leading_multiplicity);

    // `::>` / `references` reference subsetting (GH-19): the target is a reference, not a type, so
    // it's modeled via the same structured `SubsettingRelationship` every other reference-
    // subsetting clause uses -- not folded into typing like the `:` form below.
    if let Ok((input, references)) = reference_subsetting(input) {
        // Trailing multiplicity on the reference target (e.g. `::> mainSwitch[1]`) is parsed the
        // same way `part_usage_redefines_only`/other subsetting-family callers do: the shared
        // `reference_subsetting` parser only consumes the qualified target, so any multiplicity
        // is a separate, subsequent optional parse here (GH-20 real-world example).
        let (input, trailing_multiplicity) =
            opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
        let type_ref_span = references.value.span.clone();
        return Ok((
            input,
            node_from_to(
                start,
                input,
                EndDecl {
                    identity,
                    typing: None,
                    references: Some(references),
                    multiplicity: trailing_multiplicity.or(leading_multiplicity),
                    redefines: None,
                    crosses: None,
                    nested_usage: None,
                    type_ref_span: Some(type_ref_span),
                },
            ),
        ));
    }

    // GH-53: an alternative form where the target is a complete, nested kind-prefixed usage
    // rather than a bare type/reference -- see `EndDecl::nested_usage`'s doc comment. Only
    // `occurrence`/`item` are evidenced; each of those parsers already handles its own full
    // grammar (specialization clauses, multiplicity, body), so this end's own `name`/multiplicity
    // above are everything this branch itself needs to contribute.
    let (peek, _) = ws_and_comments(input)?;
    let peek_frag = peek.fragment();
    if starts_with_keyword(peek_frag, b"occurrence") {
        let (input, nested) = occurrence_usage(input)?;
        return Ok((
            input,
            node_from_to(
                start,
                input,
                EndDecl {
                    identity,
                    typing: None,
                    references: None,
                    multiplicity: leading_multiplicity,
                    redefines: None,
                    crosses: None,
                    nested_usage: Some(Box::new(EndNestedUsage::Occurrence(Box::new(nested)))),
                    type_ref_span: None,
                },
            ),
        ));
    }
    if starts_with_keyword(peek_frag, b"item") {
        let (input, nested) = item_usage(input)?;
        return Ok((
            input,
            node_from_to(
                start,
                input,
                EndDecl {
                    identity,
                    typing: None,
                    references: None,
                    multiplicity: leading_multiplicity,
                    redefines: None,
                    crosses: None,
                    nested_usage: Some(Box::new(EndNestedUsage::Item(Box::new(nested)))),
                    type_ref_span: None,
                },
            ),
        ));
    }

    let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
    let (input, (tilde, (type_ref_span, type_reference))) = preceded(
        ws_and_comments,
        (opt(tag(&b"~"[..])), with_span(qualified_reference)),
    )
    .parse(input)?;
    let typing = Some(typing_node(
        type_ref_span.clone(),
        tilde.is_some(),
        vec![type_reference],
        crate::ast::TypingSpelling::Operator,
    ));
    let (input, trailing_multiplicity) =
        opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    // GH-51: `:>>` redefines may trail the typed form, e.g. `end source: Anything :>>
    // BinaryLinkObject::source;` (Systems Library `Connections.sysml`) -- distinct from the
    // `::>`/`references` form used *instead* of `:` typing above.
    let (input, redefines) = opt(preceded(ws_and_comments, redefinition)).parse(input)?;
    // GH-85: `::>`/`references` reference-subsetting may *also* trail the typed form, in addition
    // to (not instead of) the `:` type -- e.g. `end port p3: P ::> p.p1;` (`Simple Tests/
    // ConjugationTest.sysml`, mirrored from Systems Library `Interfaces.sysml`'s `ref port :>>
    // participant : Port ...` shape). Distinct from the `::>`-instead-of-typing branch above
    // (the reference-only branch above): here typing *was* written, so this only additionally
    // populates the structured `references` field.
    let (input, trailing_references) =
        opt(preceded(ws_and_comments, reference_subsetting)).parse(input)?;
    // GH-85: `crosses` cross-subsetting may also trail the typed form, e.g. `end item cart:
    // ShoppingCart[1] crosses selectedProduct.inCart;` (OMG spec Annex `Association Examples/
    // ProductSelection_UnownedEnds.sysml`).
    let (input, crosses) = opt(preceded(ws_and_comments, cross_subsetting)).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            EndDecl {
                identity,
                typing,
                references: trailing_references,
                multiplicity: trailing_multiplicity.or(leading_multiplicity),
                redefines,
                crosses,
                nested_usage: None,
                type_ref_span: Some(type_ref_span),
            },
        ),
    ))
}

/// Starters for a connection/interface `ref` declaration body: the annotation members shared
/// with `RelationshipBody` contexts plus nested (optionally visibility-prefixed) `ref`
/// declarations (Systems Library `Interfaces.sysml`).
const REF_USAGE_BODY_STARTERS: &[&[u8]] = &[
    b"doc",
    b"comment",
    b"rep",
    b"@",
    b"ref",
    b"attribute",
    b"private",
    b"protected",
    b"public",
];

/// Ref body for a connection/interface `ref` declaration: `;` or `{` (doc/comment/rep/metadata
/// | nested `ref` declaration)* `}`. A `ref` usage body is a full `UsageBody` per the BNF; the
/// members with real library evidence are annotations and nested visibility-prefixed `ref`
/// declarations (`protected ref thisParticipant :>> self;`, Systems Library `Interfaces.sysml`),
/// with recovery-to-`Error` for anything else.
pub(crate) fn ref_body(input: Input<'_>) -> IResult<Input<'_>, RefBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, RefBody::Semicolon));
    }
    let (input, elements) = crate::parser::body::parse_structured_brace_members(
        input,
        REF_USAGE_BODY_STARTERS,
        "ref usage body",
        "recovered_ref_body_element",
        ref_body_element,
        |start, end| {
            let recovery = crate::parser::build_recovery_error_node_from_span(
                start,
                end,
                REF_USAGE_BODY_STARTERS,
                "ref usage body",
                "recovered_ref_body_element",
            );
            let node: Node<crate::ast::ParseErrorNode> = node_from_to(start, end, recovery);
            node_from_to(start, end, RefBodyElement::Error(node))
        },
    )?;
    Ok((input, RefBody::Brace { elements }))
}

fn ref_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<RefBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    if let Ok((input, nested)) = ref_decl(input) {
        return Ok((
            input,
            node_from_to(start, input, RefBodyElement::Ref(Box::new(nested))),
        ));
    }
    // `attribute :>> Disc::edges::innerSpaceDimension, ...;` (Domain Libraries
    // `ShapeItems.sysml`).
    if starts_with_keyword(input.fragment(), b"attribute") {
        let (input, usage) = crate::parser::attribute::attribute_usage(input)?;
        return Ok((
            input,
            node_from_to(
                start,
                input,
                RefBodyElement::AttributeUsage(Box::new(usage)),
            ),
        ));
    }
    let (input, annotation) = crate::parser::body::relationship_body_element(input)?;
    let span = annotation.span.clone();
    let wrapped = match annotation.value {
        crate::ast::RelationshipBodyElement::Doc(n) => RefBodyElement::Doc(n),
        crate::ast::RelationshipBodyElement::Comment(n) => RefBodyElement::Comment(n),
        crate::ast::RelationshipBodyElement::TextualRep(n) => RefBodyElement::TextualRep(n),
        crate::ast::RelationshipBodyElement::MetadataAnnotation(n) => {
            RefBodyElement::MetadataAnnotation(n)
        }
        crate::ast::RelationshipBodyElement::Error(n) => RefBodyElement::Error(n),
        crate::ast::RelationshipBodyElement::Other(text) => RefBodyElement::Other(text),
        // `relationship_body_element` is the annotation-only subset and never produces owned
        // feature members; those are dispatched by `relationship_body_member` (spec42 Gap 37),
        // which ref bodies do not use.
        crate::ast::RelationshipBodyElement::KermlFeature(_) => {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Alt,
            )))
        }
    };
    Ok((input, Node::new(span, wrapped)))
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
    // Visibility prefix (BNF `MemberPrefix`), e.g. `protected ref thisParticipant :>> self;`
    // (Systems Library `Interfaces.sysml`).
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, _) = tag(&b"ref"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, kind_keyword) = opt(preceded(
        ws_and_comments,
        alt((
            map((tag(&b"part"[..]), ws1), |_| crate::ast::RefDeclKind::Part),
            map((tag(&b"port"[..]), ws1), |_| crate::ast::RefDeclKind::Port),
            map((tag(&b"item"[..]), ws1), |_| crate::ast::RefDeclKind::Item),
            map((tag(&b"requirement"[..]), ws1), |_| {
                crate::ast::RefDeclKind::Requirement
            }),
        )),
    ))
    .parse(input)?;
    // `ref :>> name ...` (redefinition) may omit the name before `:>>`.
    let (input, parsed_name) = opt(preceded(ws_and_comments, with_span(name))).parse(input)?;
    let (input, leading_multiplicity) =
        opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    let (name_span, name_str) = parsed_name.unwrap_or((crate::ast::Span::dummy(), String::new()));

    // `redefinition` (shared with every other `:>>` call site) already handles the comma-separated
    // multi-target form, e.g. `ref port :>> Interface::participant, BinaryConnection::
    // participant[2] nonunique ordered;` (`Interfaces.sysml`).
    let (input, redefines) = opt(preceded(ws_and_comments, redefinition)).parse(input)?;

    let (input, type_ref_span, typing) = {
        let (peek, _) = ws_and_comments(input)?;
        if peek.fragment().starts_with(b":") && !peek.fragment().starts_with(b":>") {
            let (input, (type_ref_span, type_name)) = preceded(
                ws_and_comments,
                preceded(tag(&b":"[..]), with_span(qualified_reference)),
            )
            .parse(input)?;
            let typing = Some(single_target_typing(type_ref_span.clone(), type_name));
            (input, Some(type_ref_span), typing)
        } else {
            (input, None, None)
        }
    };
    let (input, trailing_multiplicity) =
        opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    // `nonunique`/`ordered` may directly follow the multiplicity, before any further
    // specialization clause: `ref otherParticipants : Port [1..*] nonunique :>
    // interfacingPorts default ...;` (Systems Library `Interfaces.sysml`). The later capture
    // below covers the post-clause position (`... :>> participant [2..*] nonunique ordered
    // { ... }`).
    let (input, (ordered_after_multiplicity, nonunique_after_multiplicity)) =
        crate::parser::usage::usage_feature_modifier_flags(input)?;
    // `:>>` redefines may also follow the type instead of preceding it, e.g. `ref self: Item
    // :>> Object::self;` (Systems Library `Items.sysml`) -- only retry if the earlier attempt
    // (before the type) didn't already find one.
    let (input, redefines) = if redefines.is_none() {
        opt(preceded(ws_and_comments, redefinition)).parse(input)?
    } else {
        (input, redefines)
    };
    // `:>` subsets, independent of and in addition to `:>>` redefines, e.g. `ref requirement
    // originalRequirement[1] :>> originalRequirements :> participant { ... }` (Systems Library
    // `Domain Libraries/Requirement Derivation/DerivationConnections.sysml`).
    let (input, subsets) = opt(preceded(ws_and_comments, subsetting)).parse(input)?;
    let subsets = subsets.map(|(target, _value)| target);
    // `nonunique`/`ordered` feature modifiers may also follow the specialization clauses
    // (real usage: `Interfaces.sysml`'s `ref port :>> participant : Port [2..*] nonunique
    // ordered { ... }`).
    let (input, (ordered_after_clauses, nonunique_after_clauses)) =
        crate::parser::usage::usage_feature_modifier_flags(input)?;
    // Optional value/default clause, e.g. `ref item :>> localClock : Clock[1] default
    // Time::universalClock { ... }` (Domain Libraries `SpatialItems.sysml`).
    let (input, value) = opt(preceded(ws_and_comments, feature_value_part)).parse(input)?;
    let (input, body) = ref_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            RefDecl {
                direction: None,
                kind_keyword,
                name: name_str,
                typing,
                subsets,
                redefines,
                multiplicity: trailing_multiplicity.or(leading_multiplicity),
                ordered: ordered_after_multiplicity || ordered_after_clauses,
                nonunique: nonunique_after_multiplicity || nonunique_after_clauses,
                value,
                body,
                name_span: Some(name_span),
                type_ref_span,
                membership: crate::ast::Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

/// Connect body: `;` or `{` ... `}`. Marker-only (no content preserved): shared by many callers
/// beyond `connect_stmt` (`Message`, `Allocate`, view/metadata constructs, ...) with no evidence
/// of real content ever appearing in their braces. See [`connect_stmt_body`] for `connect_stmt`'s
/// own body, which does preserve doc/comment/metadata annotations.
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

/// Connect statement body: `;` or `{` doc/comment/rep/metadata* `}` (BNF `RelationshipBody`).
/// Distinct from the shared marker-only [`connect_body`] above: `ConnectStmt` is the one real,
/// evidenced call site for this (a plain `connect a to b { ... }` statement can carry `doc`/
/// annotations same as any other relationship).
fn connect_stmt_body(
    input: Input<'_>,
) -> IResult<Input<'_>, (ConnectBody, Vec<Node<crate::ast::RelationshipBodyElement>>)> {
    let (input, elements) = relationship_body_annotations(input)?;
    match elements {
        None => Ok((input, (ConnectBody::Semicolon, Vec::new()))),
        Some(elements) => Ok((input, (ConnectBody::Brace, elements))),
    }
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
    let (input, (body, body_elements)) = connect_stmt_body(input)?;
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
                body_elements,
            },
        ),
    ))
}

#[cfg(test)]
mod end_decl_kind_tests {
    use super::end_decl;
    use crate::ast::EndIdentity;

    fn input(text: &str) -> crate::parser::Input<'_> {
        crate::parser::span::test_input(text)
    }

    #[test]
    fn end_decl_accepts_feature_kind_with_redefines() {
        let (rest, node) = end_decl(
            input(
                "end feature source: Occurrence redefines FlowTransfer::source, transfers::source;",
            ),
            true,
        )
        .expect("end feature");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            &node.value.identity,
            EndIdentity::Declaration(name) if name.value == "source"
        ));
        assert_eq!(
            node.value
                .typing
                .as_ref()
                .map(|typing| typing.value.target.len()),
            Some(1)
        );
        assert!(node.value.redefines.is_some());
    }

    #[test]
    fn end_decl_accepts_occurrence_kind_with_redefines() {
        let (rest, node) = end_decl(
            input("end occurrence source: Occurrence :>> Message::source, FlowTransfer::source;"),
            true,
        )
        .expect("end occurrence");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            &node.value.identity,
            EndIdentity::Declaration(name) if name.value == "source"
        ));
        assert_eq!(
            node.value
                .typing
                .as_ref()
                .map(|typing| typing.value.target.len()),
            Some(1)
        );
        assert!(node.value.redefines.is_some());
        assert!(node.value.nested_usage.is_none());
    }
}
