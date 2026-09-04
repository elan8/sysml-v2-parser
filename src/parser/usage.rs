//! Shared usage grammar fragments from `UsageDeclaration` / `FeatureSpecializationPart`.

use crate::ast::{
    Expression, Multiplicity, Node, QualifiedReferenceId, Span, SubsettingKind,
    SubsettingRelationship, TypingKind, TypingRelationship, UsageDeclaration,
};
use crate::parser::expr::expression;
use crate::parser::lex::{
    identification, qualified_reference, reference_path, starts_with_keyword, typed_by_operator,
    ws1, ws_and_comments,
};
use crate::parser::{node_from_to, span_from_to, Input};
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// Parse the grammar-owned `UsageDeclaration = Identification FeatureSpecializationPart?`.
///
/// Owners decide whether an empty identification is legal before calling this parser; the typed
/// declaration itself has no owner-specific flags or mirrors.
fn usage_declaration_with_identification<'a>(
    start: Input<'a>,
    input: Input<'a>,
    identification: crate::ast::Identification,
) -> IResult<Input<'a>, Node<UsageDeclaration>> {
    let identification_span = span_from_to(start, input);
    let (input, header) = crate::parser::definition_header::parse_feature_usage_header(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            UsageDeclaration {
                identification,
                identification_span,
                typing: header.typing,
                multiplicity: header.multiplicity,
                multiplicity_modifiers: header.multiplicity_modifiers,
                subsets: header
                    .subsets
                    .map(|relationship| (relationship, header.subsetting_value)),
                redefines: header.redefines,
                references: header.references,
                crosses: header.crosses,
                intersects: header.intersects,
            },
        ),
    ))
}

pub(crate) fn usage_declaration(input: Input<'_>) -> IResult<Input<'_>, Node<UsageDeclaration>> {
    let start = input;
    let (input, identification) = identification(input)?;
    usage_declaration_with_identification(start, input, identification)
}

/// Parse the `UsageDeclaration` form whose optional `Identification` is absent.
///
/// This is a real, source-positioned declaration (with a zero-width identification span), not a
/// placeholder for another grammar alternative. Only parents whose pinned production admits an
/// omitted identification may call it; `FlowDeclaration` uses it before a leading `of` payload
/// clause, whereas its endpoint-only alternative has no declaration at all.
pub(crate) fn usage_declaration_without_identification(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<UsageDeclaration>> {
    usage_declaration_with_identification(
        input,
        input,
        crate::ast::Identification {
            short_name: None,
            name: None,
        },
    )
}

/// BNF `RefPrefix = FeatureDirection? 'derived'? ('abstract' | 'variation')? 'constant'?`
/// (§8.2.2.6.2), the modifier chain every usage may carry ahead of its keyword.
///
/// Owned here rather than re-spelled per usage parser: each parser that hand-rolled a subset of
/// the chain accepted only the combinations that happened to be needed at the time, so a legal
/// prefix was a parse gap in whichever scopes had not adopted it yet.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct RefPrefix {
    /// `in` / `out` / `inout` -- the production's first slot, so it precedes every keyword below.
    pub direction: Option<crate::ast::InOut>,
    pub is_derived: bool,
    /// `abstract` or `variation` -- one slot, since the BNF makes them alternatives.
    pub usage_prefix: Option<crate::ast::DefinitionPrefix>,
    pub is_constant: bool,
}

/// Parse [`RefPrefix`]. Every part is optional, so this never fails; it consumes nothing when the
/// next token is the usage keyword itself.
pub(crate) fn ref_prefix(input: Input<'_>) -> IResult<Input<'_>, RefPrefix> {
    // Each slot is refused on its first byte before the `tag` trial: this prefix is speculated at
    // nearly every member start, and a member rarely carries any of these keywords.
    let first = |input: Input<'_>, byte: u8| input.fragment().first() == Some(&byte);
    let (input, direction) = opt(crate::parser::attribute::direction_prefix).parse(input)?;
    let (input, is_derived) = if first(input, b'd') {
        opt(preceded(tag(&b"derived"[..]), ws1)).parse(input)?
    } else {
        (input, None)
    };
    let (input, usage_prefix) = if first(input, b'a') || first(input, b'v') {
        opt(nom::branch::alt((
            nom::combinator::map(preceded(tag(&b"abstract"[..]), ws1), |_| {
                crate::ast::DefinitionPrefix::Abstract
            }),
            nom::combinator::map(preceded(tag(&b"variation"[..]), ws1), |_| {
                crate::ast::DefinitionPrefix::Variation
            }),
        )))
        .parse(input)?
    } else {
        (input, None)
    };
    let (input, is_constant) = if first(input, b'c') {
        opt(preceded(tag(&b"constant"[..]), ws1)).parse(input)?
    } else {
        (input, None)
    };
    Ok((
        input,
        RefPrefix {
            direction,
            is_derived: is_derived.is_some(),
            usage_prefix,
            is_constant: is_constant.is_some(),
        },
    ))
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct SpecializationClauses {
    pub subsets: Option<(Node<SubsettingRelationship>, Option<Node<Expression>>)>,
    pub redefines: Option<Node<SubsettingRelationship>>,
    pub references: Option<Node<SubsettingRelationship>>,
    pub crosses: Option<Node<SubsettingRelationship>>,
    pub intersects: Option<Node<SubsettingRelationship>>,
    pub had_any: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct UsageHeader {
    pub type_reference: Option<QualifiedReferenceId>,
    pub type_is_conjugated: bool,
    /// The complete `Typings` clause as one relationship node: its authored spelling (`:`,
    /// `typed by`, `defined by`), conjugation, span, and every target -- not just the first.
    /// Callers that keep only a single `type_reference` lose the spelling and the extra targets,
    /// so scopes whose emitter must reproduce what was written read this instead.
    pub typing: Option<Node<TypingRelationship>>,
    pub subsets: Option<Node<SubsettingRelationship>>,
    /// Optional `= expression` written as part of a `Subsettings` clause. The shared grammar
    /// consumes this before the ordinary `FeatureValue` position, so callers that own a typed
    /// value must carry it forward rather than silently losing the expression.
    pub subsetting_value: Option<Node<Expression>>,
    pub redefines: Option<Node<SubsettingRelationship>>,
    pub references: Option<Node<SubsettingRelationship>>,
    pub crosses: Option<Node<SubsettingRelationship>>,
    pub intersects: Option<Node<SubsettingRelationship>>,
    pub had_specialization: bool,
    /// Multiplicity clause captured by [`feature_usage_header`], whether it was written before
    /// or after a specialization group. `None` for [`usage_header`], which has no multiplicity
    /// grammar.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `MultiplicityPart`'s ordering and uniqueness keyword slots (BNF §8.2.2.6.6), with the
    /// authored spellings and their exact spans.
    pub multiplicity_modifiers: crate::ast::MultiplicityModifiers,
}

/// Byte offset of the `]` that closes multiplicity content starting at `frag` (the `[` is
/// already consumed), tracking nested `[...]` depth so a bound expression like `a#(0)` — which
/// itself contains no brackets, but future bound expressions might — doesn't confuse an inner
/// `]` for the closing one. Returns `None` if unterminated.
fn find_multiplicity_close(frag: &[u8]) -> Option<usize> {
    let mut depth = 0i32;
    for (i, &b) in frag.iter().enumerate() {
        match b {
            b'[' => depth += 1,
            b']' => {
                if depth == 0 {
                    return Some(i);
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    None
}

/// Byte offset of the first top-level `..` within `frag[..limit]` (not inside a nested
/// `[...]`), or `None` if this is a bare bound with no range.
fn find_top_level_range_dots(frag: &[u8], limit: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut i = 0usize;
    while i + 1 < limit {
        match frag[i] {
            b'[' => depth += 1,
            b']' => depth -= 1,
            _ => {}
        }
        if depth == 0 && frag[i] == b'.' && frag[i + 1] == b'.' {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Parse one already-isolated multiplicity bound slice: `*` (unbounded, renders as `None`) or a
/// bound expression via [`expression`]. The slice is exactly the bound's text (whitespace
/// aside), so any trailing remainder after a successful expression parse is ignored rather than
/// enforced with `all_consuming` — there isn't anything else it could legally contain.
fn parse_multiplicity_bound_text(
    slice: Input<'_>,
) -> Result<Option<Box<Node<Expression>>>, nom::Err<nom::error::Error<Input<'_>>>> {
    let (rest, _) = ws_and_comments(slice)?;
    if rest.fragment().first() == Some(&b'*') {
        return Ok(None);
    }
    let (_, expr) = expression(rest)?;
    Ok(Some(Box::new(expr)))
}

/// Multiplicity part, parsed into structured bounds: `'[' ('*' | bound ('..' ('*' | bound))?) ']'`.
/// A bare `[3]` yields `lower == upper == Some(3)`; `[1..*]` yields `upper == None`. Bound text is
/// isolated by scanning for the closing `]` and an optional top-level `..` first (rather than
/// handing the whole bracket content to [`expression`] in one call), because `expression`'s
/// binary-operator chain commits once it matches `..` as a range operator and does not backtrack
/// if the right-hand side (`*`) fails to parse as a primary expression (PAR-004/PAR-003 item 5).
pub(crate) fn multiplicity_node(input: Input<'_>) -> IResult<Input<'_>, Node<Multiplicity>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"["[..]).parse(input)?;
    let frag = input.fragment();
    let close_rel = find_multiplicity_close(frag).ok_or_else(|| {
        nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeUntil,
        ))
    })?;
    let dots_rel = find_top_level_range_dots(frag, close_rel);
    let (lower, upper, input) = if let Some(d) = dots_rel {
        let (rest, left_slice) = nom::bytes::complete::take(d).parse(input)?;
        let (rest, _) = tag(&b".."[..]).parse(rest)?;
        let right_len = close_rel - d - 2;
        let (rest, right_slice) = nom::bytes::complete::take(right_len).parse(rest)?;
        let lower = parse_multiplicity_bound_text(left_slice)?;
        let upper = parse_multiplicity_bound_text(right_slice)?;
        (lower, upper, rest)
    } else {
        let (rest, content_slice) = nom::bytes::complete::take(close_rel).parse(input)?;
        let bound = parse_multiplicity_bound_text(content_slice)?;
        (bound.clone(), bound, rest)
    };
    let (input, _) = tag(&b"]"[..]).parse(input)?;
    let span = span_from_to(start, input);
    Ok((input, Node::new(span, Multiplicity { lower, upper, span })))
}

/// `(span, is_conjugated, targets)` for a parsed `typings`/`optional_typings` clause -- see
/// [`typings`] for what each field means.
pub(crate) type TypingsResult = (
    Span,
    bool,
    Vec<QualifiedReferenceId>,
    crate::ast::TypingSpelling,
);

#[cfg(test)]
fn slice_reference_span(input: Input<'_>, span: Span) -> Option<String> {
    let relative = span.offset.checked_sub(input.location_offset())?;
    let end = relative.checked_add(span.len)?;
    input
        .fragment()
        .get(relative..end)
        .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
}

/// Resolve a parsed reference's exact authored spelling while the originating input is live.
#[cfg(test)]
pub(crate) fn reference_text(input: Input<'_>, id: QualifiedReferenceId) -> Option<String> {
    slice_reference_span(input, input.extra.reference_span(id)?)
}

/// Preserve legacy display-only fields while semantic relationships store arena IDs.
#[cfg(test)]
pub(crate) fn reference_list_text(input: Input<'_>, ids: &[QualifiedReferenceId]) -> String {
    ids.iter()
        .filter_map(|id| reference_text(input, *id))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Typings: `:` / `defined by` one or more qualified names, with optional conjugated `~`.
///
/// Returns `(span, is_conjugated, targets)`: `is_conjugated` reflects the first (and, per
/// SysML v2, realistically only) target's leading `~`; each arena identity excludes that prefix
/// (callers that populate a `TypingRelationship` should use `is_conjugated` directly). `targets`
/// almost always has exactly one element, but a legal comma-separated clause retains one identity
/// per target.
pub(crate) fn typings(input: Input<'_>) -> IResult<Input<'_>, TypingsResult> {
    let before = input;
    let (input, spelling) = preceded(ws_and_comments, typed_by_operator).parse(input)?;
    let (input, (first_conjugated, first)) =
        preceded(ws_and_comments, conjugated_qualified_name).parse(input)?;
    let (input, rest) = many0(preceded(
        preceded(ws_and_comments, tag(&b","[..])),
        preceded(ws_and_comments, conjugated_qualified_name),
    ))
    .parse(input)?;
    let mut targets = vec![first];
    for (_, target) in rest {
        targets.push(target);
    }
    Ok((
        input,
        (
            span_from_to(before, input),
            first_conjugated,
            targets,
            spelling,
        ),
    ))
}

/// Optional typings that remain strict once a typing starter is present.
pub(crate) fn optional_typings(input: Input<'_>) -> IResult<Input<'_>, Option<TypingsResult>> {
    let (peek, _) = ws_and_comments(input)?;
    let fragment = peek.fragment();
    if (fragment.starts_with(b":")
        && !fragment.starts_with(b":>")
        && !fragment.starts_with(b"::>")
        && !fragment.starts_with(b":>>")
        && !fragment.starts_with(b":="))
        || starts_with_keyword(fragment, b"defined")
        || starts_with_keyword(fragment, b"typed")
    {
        let (input, typing) = typings(input)?;
        return Ok((input, Some(typing)));
    }
    Ok((input, None))
}

pub(crate) fn typing_reference_fields_from_result(
    result: Option<TypingsResult>,
) -> (
    Option<Span>,
    Option<QualifiedReferenceId>,
    Option<Node<TypingRelationship>>,
) {
    let type_ref_span = result.as_ref().map(|(span, _, _, _)| *span);
    let type_reference = result
        .as_ref()
        .and_then(|(_, _, targets, _)| targets.first().copied());
    let typing = result.map(|(span, is_conjugated, targets, spelling)| {
        typing_node(span, is_conjugated, targets, spelling)
    });
    (type_ref_span, type_reference, typing)
}

/// Parses an optional leading `~` and a qualified name; returns `(was_conjugated, target)` with
/// the `~` stripped from the target's segments rather than folded into them.
pub(crate) fn conjugated_qualified_name(
    input: Input<'_>,
) -> IResult<Input<'_>, (bool, QualifiedReferenceId)> {
    let (input, conjugated) = opt(tag(&b"~"[..])).parse(input)?;
    let (input, target) = qualified_reference(input)?;
    Ok((input, (conjugated.is_some(), target)))
}

/// A single subsetting-family target: a `::`-qualified name optionally continued with
/// `.`-separated feature-chain segments, e.g. `Vehicle::mass.value`.
fn specialization_target(input: Input<'_>) -> IResult<Input<'_>, QualifiedReferenceId> {
    reference_path(input)
}

/// One or more comma-separated [`specialization_target`]s, e.g. the `Base, Other` in
/// `:> Base, Other`. Each target receives its own document-local identity.
pub(crate) fn specialization_targets(
    input: Input<'_>,
) -> IResult<Input<'_>, Vec<QualifiedReferenceId>> {
    let (input, first) = specialization_target(input)?;
    let (input, rest) = many0(preceded(
        preceded(ws_and_comments, tag(&b","[..])),
        preceded(ws_and_comments, specialization_target),
    ))
    .parse(input)?;
    let mut targets = vec![first];
    targets.extend(rest);
    Ok((input, targets))
}

/// Build a `TypingRelationship` node from a `typings`/`optional_typings` result, mirroring
/// `subsetting_relationship_node` below. Shared by every usage-kind parser that wraps a `:`/
/// `defined by`/`typed by` clause -- moved here (from `attribute.rs`, its original single caller)
/// so `PartUsage`/`RefDecl` parsers can reuse the exact same multi-target-capable construction
/// so every caller preserves the same arena identities and multi-target structure.
pub(crate) fn typing_relationship_node(
    span: Span,
    kind: TypingKind,
    is_conjugated: bool,
    target: Vec<QualifiedReferenceId>,
    spelling: crate::ast::TypingSpelling,
) -> Node<TypingRelationship> {
    Node::new(
        span,
        TypingRelationship {
            target,
            kind,
            span,
            is_conjugated,
            is_implied: false,
            spelling,
        },
    )
}

/// Shorthand for the common `:` / `defined by` / `typed by` case (`TypingKind::Typing`).
pub(crate) fn typing_node(
    span: Span,
    is_conjugated: bool,
    target: Vec<QualifiedReferenceId>,
    spelling: crate::ast::TypingSpelling,
) -> Node<TypingRelationship> {
    typing_relationship_node(span, TypingKind::Typing, is_conjugated, target, spelling)
}

/// Build a single-target `TypingRelationship` from an already-parsed arena identity and the
/// surrounding relationship span. Used by ad hoc `ref`-declaration call sites that parse one
/// symbolic `:` target rather than the comma-aware `typings`/`optional_typings` production.
pub(crate) fn single_target_typing(
    span: Span,
    target: QualifiedReferenceId,
) -> Node<TypingRelationship> {
    typing_node(
        span,
        false,
        vec![target],
        crate::ast::TypingSpelling::Operator,
    )
}

/// Single-target convenience over [`subsetting_relationship_node`] for ad hoc `:>`/`:>>`-family
/// shapes parsed directly outside `specialization_targets` (a bare, unqualified feature name --
/// these ad hoc shapes never parse a qualified `::`/`.`-segmented target). Shared by
/// `attribute.rs` (`attribute_feature_binding`, `metadata_binding`) and `part/body.rs`
/// (`exhibit_state`, `connection_usage_member`), which previously each redefined this themselves.
pub(crate) fn single_target_subsetting(
    span: Span,
    kind: SubsettingKind,
    target: QualifiedReferenceId,
) -> Node<SubsettingRelationship> {
    subsetting_relationship_node(vec![target], kind, span)
}

/// A `:>` / `:>>` specialization trailing after a feature's own body, attached to that same
/// feature rather than starting a new member: `item concern1 : Concern { doc /* ... */ } :>
/// concerns;` (Apollo 11 `Purpose/StakeholderPackage.sysml`). Predates before-body specialization
/// support for at least one usage kind -- `exhibit_state` (`part/body.rs`) has carried the
/// `:>>`-only case for a while -- so this generalizes that precedent to `:>` and to any caller of
/// [`feature_usage_header`]/[`usage_header`]; when present it wins over a before-body clause of
/// the same kind, the same rule `exhibit_state` already applies.
///
/// The caller owns consuming the trailing `;` this clause requires (a bodyless member's `;` and a
/// bodied member's closing `}` are not this parser's business); see `item_usage_inner` for the
/// shape.
pub(crate) fn post_body_specialization(
    input: Input<'_>,
) -> IResult<Input<'_>, Option<(SubsettingKind, Node<SubsettingRelationship>)>> {
    let before = input;
    let (input, _) = ws_and_comments(input)?;
    // `:>>` must be tried before `:>`: `tag(":>")` on `:>>concerns` would match its first two
    // bytes and strand a leading `>` in front of the target reference.
    let (input, redefines_tag) = opt(tag(&b":>>"[..])).parse(input)?;
    let (input, kind) = if redefines_tag.is_some() {
        (input, Some(SubsettingKind::Redefines))
    } else {
        let (input, subsets_tag) = opt(tag(&b":>"[..])).parse(input)?;
        (input, subsets_tag.map(|_| SubsettingKind::Subsets))
    };
    let Some(kind) = kind else {
        return Ok((before, None));
    };
    let (input, target) = preceded(ws_and_comments, qualified_reference).parse(input)?;
    let span = span_from_to(before, input);
    Ok((
        input,
        Some((kind, single_target_subsetting(span, kind, target))),
    ))
}

/// [`subsetting_relationship_node`] for a clause whose authored spelling is known.
pub(crate) fn spelled_subsetting_relationship_node(
    target: Vec<QualifiedReferenceId>,
    kind: SubsettingKind,
    spelling: crate::ast::SubsettingSpelling,
    span: Span,
) -> Node<SubsettingRelationship> {
    Node::new(
        span,
        SubsettingRelationship {
            target,
            kind,
            spelling,
            span,
            is_implied: false,
        },
    )
}

/// Build a `SubsettingRelationship` node from target(s) and the span of the whole clause
/// (operator/keyword through target).
pub(crate) fn subsetting_relationship_node(
    target: Vec<QualifiedReferenceId>,
    kind: SubsettingKind,
    span: Span,
) -> Node<SubsettingRelationship> {
    Node::new(
        span,
        SubsettingRelationship {
            target,
            kind,
            // Callers that know the authored spelling use
            // [`spelled_subsetting_relationship_node`]; the operator is the default because every
            // *synthesised* relationship (a redefinition implied by a shorthand, say) is emitted
            // in operator form.
            spelling: crate::ast::SubsettingSpelling::Operator,
            span,
            is_implied: false,
        },
    )
}

/// Subsettings: `:>` / `subsets` target, with optional `= expression` value.
pub(crate) fn subsetting(
    input: Input<'_>,
) -> IResult<Input<'_>, (Node<SubsettingRelationship>, Option<Node<Expression>>)> {
    let before = input;
    let (input, spelling) =
        preceded(ws_and_comments, crate::parser::lex::spelled_subset_operator).parse(input)?;
    let (input, (target, value)) = preceded(
        ws_and_comments,
        (
            specialization_targets,
            opt(preceded(
                preceded(ws_and_comments, tag(&b"="[..])),
                preceded(ws_and_comments, expression),
            )),
        ),
    )
    .parse(input)?;
    let span = span_from_to(before, input);
    let node =
        spelled_subsetting_relationship_node(target, SubsettingKind::Subsets, spelling, span);
    Ok((input, (node, value)))
}

/// Redefinitions: `:>>` / `redefines` target.
pub(crate) fn redefinition(input: Input<'_>) -> IResult<Input<'_>, Node<SubsettingRelationship>> {
    let before = input;
    let (input, (spelling, target)) = (
        preceded(
            ws_and_comments,
            crate::parser::lex::spelled_redefine_operator,
        ),
        preceded(ws_and_comments, specialization_targets),
    )
        .parse(input)?;
    let span = span_from_to(before, input);
    Ok((
        input,
        spelled_subsetting_relationship_node(target, SubsettingKind::Redefines, spelling, span),
    ))
}

/// Parse an optional `Redefinitions` clause without hiding a malformed one.
///
/// `nom::combinator::opt(redefinition)` would turn `:>> ;` into an absent relationship and let
/// the caller's permissive tail skip discard the authored operator. Once the concrete starter is
/// present, the clause is mandatory; callers therefore recover the invalid declaration at their
/// owning body boundary instead.
pub(crate) fn optional_redefinition(
    input: Input<'_>,
) -> IResult<Input<'_>, Option<Node<SubsettingRelationship>>> {
    let (peek, _) = ws_and_comments(input)?;
    if peek.fragment().starts_with(b":>>") || starts_with_keyword(peek.fragment(), b"redefines") {
        let (input, relationship) = redefinition(input)?;
        Ok((input, Some(relationship)))
    } else {
        Ok((input, None))
    }
}

/// Prefix redefinition: `:>>` / `redefines` qualified_name (for usage heads).
pub(crate) fn prefix_redefinition_target(
    input: Input<'_>,
) -> IResult<Input<'_>, (Span, Node<SubsettingRelationship>)> {
    let before = input;
    let (input, target) = redefinition(input)?;
    Ok((input, (span_from_to(before, input), target)))
}

/// Reference subsetting: `::>` / `references` target.
pub(crate) fn reference_subsetting(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<SubsettingRelationship>> {
    let before = input;
    let (input, (spelling, target)) = (
        preceded(
            ws_and_comments,
            crate::parser::lex::spelled_references_operator,
        ),
        preceded(ws_and_comments, specialization_targets),
    )
        .parse(input)?;
    let span = span_from_to(before, input);
    Ok((
        input,
        spelled_subsetting_relationship_node(target, SubsettingKind::References, spelling, span),
    ))
}

/// Cross subsetting: `=>` / `crosses` target.
pub(crate) fn cross_subsetting(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<SubsettingRelationship>> {
    let before = input;
    let (input, (spelling, target)) = (
        preceded(
            ws_and_comments,
            crate::parser::lex::spelled_crosses_operator,
        ),
        preceded(ws_and_comments, specialization_targets),
    )
        .parse(input)?;
    let span = span_from_to(before, input);
    Ok((
        input,
        spelled_subsetting_relationship_node(target, SubsettingKind::Crosses, spelling, span),
    ))
}

/// Intersecting: `intersects` target(s), e.g. `intersects a, b`. Previously tokenized and
/// discarded entirely (`skip_intersects_clause`); now kept structured like the other three
/// subsetting-family clauses.
pub(crate) fn intersecting(input: Input<'_>) -> IResult<Input<'_>, Node<SubsettingRelationship>> {
    let before = input;
    let (input, target) = preceded(
        preceded(ws_and_comments, tag(&b"intersects"[..])),
        preceded(ws_and_comments, specialization_targets),
    )
    .parse(input)?;
    let span = span_from_to(before, input);
    Ok((
        input,
        subsetting_relationship_node(target, SubsettingKind::Intersects, span),
    ))
}

/// Fold `next` into an already-seen clause of the same kind.
///
/// `SubsettingRelationship::target` is a list because one clause may name several comma-separated
/// targets; a clause kind written twice in one header names further targets of the same
/// relationship, so the two are the same fact and belong in the same list.
fn merge_clause(existing: &mut Node<SubsettingRelationship>, next: Node<SubsettingRelationship>) {
    let Node {
        value: next_value,
        span: next_span,
    } = next;
    existing.value.target.extend(next_value.target);
    existing.value.span = existing.value.span.covering(&next_value.span);
    existing.span = existing.span.covering(&next_span);
}

/// [`merge_clause`] over two optional clauses, keeping `leading`'s source order first.
fn merge_groups(
    leading: Option<Node<SubsettingRelationship>>,
    trailing: Option<Node<SubsettingRelationship>>,
) -> Option<Node<SubsettingRelationship>> {
    match (leading, trailing) {
        (Some(mut leading), Some(trailing)) => {
            merge_clause(&mut leading, trailing);
            Some(leading)
        }
        (leading, trailing) => leading.or(trailing),
    }
}

/// [`merge_clause`] against a slot that may not hold a clause yet.
pub(crate) fn merge_into(
    slot: &mut Option<Node<SubsettingRelationship>>,
    next: Node<SubsettingRelationship>,
) {
    match slot {
        Some(existing) => merge_clause(existing, next),
        None => *slot = Some(next),
    }
}

/// Parse zero or more subsetting/redefinition clauses in any order.
///
/// Repeating a clause kind is legal and means what writing its targets in one clause would mean:
/// `subsets step, usage subsets Metadata::metadataItems` (`sysml.library/Systems
/// Library/SysML.sysml:20`) subsets all three. Clauses of one kind therefore accumulate targets
/// rather than overwrite -- overwriting dropped every target but the last with no diagnostic, so
/// that header emitted as `:> Metadata::metadataItems`.
pub(crate) fn specialization_clauses(
    input: Input<'_>,
) -> IResult<Input<'_>, SpecializationClauses> {
    // Clauses accumulate directly: collecting them first would allocate on a path the grammar
    // re-enters speculatively for every usage and definition header.
    let mut out = SpecializationClauses::default();
    let mut input = input;
    loop {
        let (after_ws, _) = ws_and_comments(input)?;
        if let Ok((rest, (relationship, value))) = subsetting(after_ws) {
            match &mut out.subsets {
                Some((existing, existing_value)) => {
                    merge_clause(existing, relationship);
                    // A `subsets x = expr` value belongs to the clause that wrote it; keep the
                    // first, since a second value would be a redefinition of the same feature.
                    if existing_value.is_none() {
                        *existing_value = value;
                    }
                }
                slot @ None => *slot = Some((relationship, value)),
            }
            input = rest;
        } else if let Ok((rest, value)) = redefinition(after_ws) {
            merge_into(&mut out.redefines, value);
            input = rest;
        } else if let Ok((rest, value)) = reference_subsetting(after_ws) {
            merge_into(&mut out.references, value);
            input = rest;
        } else if let Ok((rest, value)) = cross_subsetting(after_ws) {
            merge_into(&mut out.crosses, value);
            input = rest;
        } else if let Ok((rest, value)) = intersecting(after_ws) {
            merge_into(&mut out.intersects, value);
            input = rest;
        } else {
            // Leave the whitespace before a non-clause unconsumed, as `preceded` did.
            return Ok((input, out));
        }
        out.had_any = true;
    }
}

/// Parse the ordered alternatives of KerML `FeatureSpecialization+` without merging repeated
/// clauses. This is the typed component used by migrated owners whose consumers must distinguish
/// clause count and authored order (spec42 Gap 66).
pub(crate) fn feature_specializations(
    input: Input<'_>,
) -> IResult<Input<'_>, Vec<crate::ast::FeatureSpecialization>> {
    let mut input = input;
    let mut out = Vec::new();
    loop {
        let (after_ws, _) = ws_and_comments(input)?;
        let (after_typing, typing) = optional_typings(after_ws)?;
        if let Some((span, is_conjugated, targets, spelling)) = typing {
            let relationship = Node::new(
                span,
                crate::ast::TypingRelationship {
                    target: targets,
                    kind: crate::ast::TypingKind::Typing,
                    span,
                    is_conjugated,
                    is_implied: false,
                    spelling,
                },
            );
            out.push(crate::ast::FeatureSpecialization::Typing(relationship));
            input = after_typing;
        } else if let Ok((rest, (relationship, value))) = subsetting(after_ws) {
            out.push(crate::ast::FeatureSpecialization::Subsetting {
                relationship,
                value,
            });
            input = rest;
        } else if let Ok((rest, relationship)) = reference_subsetting(after_ws) {
            out.push(crate::ast::FeatureSpecialization::ReferenceSubsetting(
                relationship,
            ));
            input = rest;
        } else if let Ok((rest, relationship)) = cross_subsetting(after_ws) {
            out.push(crate::ast::FeatureSpecialization::CrossSubsetting(
                relationship,
            ));
            input = rest;
        } else if let Ok((rest, relationship)) = redefinition(after_ws) {
            out.push(crate::ast::FeatureSpecialization::Redefinition(
                relationship,
            ));
            input = rest;
        } else {
            return Ok((input, out));
        }
    }
}

pub(crate) fn skip_usage_feature_modifiers(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) = multiplicity_modifier_slots(input)?;
    Ok((input, ()))
}

/// One authored `MultiplicityPart` keyword, tagged with the slot it fills.
///
/// The production gives each slot its own alternation, so a keyword cannot fill both and the
/// caller never has to ask which of two booleans a spelling meant.
#[derive(Clone, Copy)]
enum MultiplicitySlot {
    Ordering(crate::ast::MultiplicityOrdering),
    Uniqueness(crate::ast::MultiplicityUniqueness),
}

/// Parse `MultiplicityPart`'s ordering and uniqueness keyword slots (SysML BNF 495-496, KerML BNF
/// 639-640), returning the authored spellings and their exact spans.
///
/// ```text
/// ( isOrdered ?= 'ordered' ( { isUnique = false } 'nonunique' )?
/// | { isUnique = false } 'nonunique' ( isOrdered ?= 'ordered' )? )
/// ```
///
/// The production is a two-way alternation over *distinct* slots, so it admits at most one
/// ordering keyword and at most one uniqueness keyword, in either order. This reads exactly that:
/// one keyword, then at most one more filling the *other* slot. A third keyword, a repeat of a
/// slot already filled, or a contradiction (`ordered nonordered`) is left unconsumed rather than
/// folded into the slot that already has a span. The enclosing declaration then fails at that
/// token and the enclosing scope's member recovery captures the whole member by source span, so
/// the excess becomes an explicit malformed node instead of a keyword the parser quietly drops.
/// The OMG Pilot's generated parser behaves the same way: `MultiplicityPart` is an Xtext fragment
/// matched once, so the token after it is a syntax error (`KerML.xtext` 578-584,
/// `SysML.xtext` 370-376).
///
/// `unique` and `nonordered` -- the explicit spellings of the two metamodel defaults, which
/// neither pinned production lists and which the Pilot grammar does not spell at all -- are
/// recognized here rather than discarded: consuming authored syntax and recording nothing is the
/// one outcome the parser must not produce, and a consumer that cannot tell "the author stated
/// the default" from "the author said nothing" has lost a fact the source contains. They occupy
/// the same two slots, so they obey the same cardinality.
///
/// Keywords match on a token boundary, so a declaration continuing `orderedBy` is not read as an
/// `ordered` modifier followed by a stray `By`.
///
/// Leaves the whitespace before a non-modifier token unconsumed, as `preceded` did.
pub(crate) fn multiplicity_modifier_slots(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::ast::MultiplicityModifiers> {
    multiplicity_modifier_slots_after(crate::ast::MultiplicityModifiers::default(), input)
}

/// [`multiplicity_modifier_slots`] resuming from what an earlier position of the same declaration
/// already read.
///
/// `FeatureSpecializationPart` (SysML BNF 424-426, KerML BNF 632-634) puts the `MultiplicityPart`
/// either before the specializations or after them, so declarations that accept both positions
/// parse the slots twice. Passing the first group in as the accumulator makes a slot filled at the
/// first position stop consumption at the second exactly as a repeat within one position does --
/// otherwise `ordered ordered` is read as one slot per position and the second spelling is folded
/// away with no diagnostic, which is the silent drop this parser must not produce.
pub(crate) fn multiplicity_modifier_slots_after(
    mut modifiers: crate::ast::MultiplicityModifiers,
    mut input: Input<'_>,
) -> IResult<Input<'_>, crate::ast::MultiplicityModifiers> {
    // Terminates: every iteration that continues fills one of two slots that is not yet filled.
    loop {
        let (after_ws, _) = ws_and_comments(input)?;
        let Some((rest, slot, span)) = multiplicity_slot_keyword(after_ws) else {
            // Leave the whitespace before a non-modifier token unconsumed, as `preceded` did.
            break;
        };
        match slot {
            MultiplicitySlot::Ordering(ordering) if modifiers.ordering.is_none() => {
                modifiers.ordering = Some(crate::ast::Node::new(span, ordering));
            }
            MultiplicitySlot::Uniqueness(uniqueness) if modifiers.uniqueness.is_none() => {
                modifiers.uniqueness = Some(crate::ast::Node::new(span, uniqueness));
            }
            // The slot this keyword fills already has an authored spelling and its span. Leave it
            // for the enclosing scope's recovery rather than consuming it into nothing.
            MultiplicitySlot::Ordering(_) | MultiplicitySlot::Uniqueness(_) => break,
        }
        input = rest;
    }
    Ok((input, modifiers))
}

/// Consume one `MultiplicityPart` keyword, returning the slot it fills and its exact span.
fn multiplicity_slot_keyword(input: Input<'_>) -> Option<(Input<'_>, MultiplicitySlot, Span)> {
    use crate::ast::{MultiplicityOrdering, MultiplicityUniqueness};

    // `nonunique` before `unique` and `nonordered` before `ordered`: the shorter spelling is not a
    // prefix of the longer one, but keeping the longer alternative first documents that ordering
    // is deliberate rather than incidental.
    const KEYWORDS: [(&[u8], MultiplicitySlot); 4] = [
        (
            b"nonunique",
            MultiplicitySlot::Uniqueness(MultiplicityUniqueness::Nonunique),
        ),
        (
            b"unique",
            MultiplicitySlot::Uniqueness(MultiplicityUniqueness::Unique),
        ),
        (
            b"nonordered",
            MultiplicitySlot::Ordering(MultiplicityOrdering::Nonordered),
        ),
        (
            b"ordered",
            MultiplicitySlot::Ordering(MultiplicityOrdering::Ordered),
        ),
    ];

    KEYWORDS.iter().find_map(|&(keyword, slot)| {
        let (rest, span) = modifier_keyword(input, keyword)?;
        Some((rest, slot, span))
    })
}

/// Consume one modifier keyword on a token boundary, returning its exact span.
fn modifier_keyword<'a>(input: Input<'a>, keyword: &'static [u8]) -> Option<(Input<'a>, Span)> {
    if !starts_with_keyword(input.fragment(), keyword) {
        return None;
    }
    let (rest, (span, _)) =
        crate::parser::span::with_span(tag::<_, _, nom::error::Error<Input<'a>>>(keyword))
            .parse(input)
            .ok()?;
    Some((rest, span))
}

fn merge_usage_header(
    leading: SpecializationClauses,
    trailing: SpecializationClauses,
    type_result: Option<TypingsResult>,
) -> UsageHeader {
    // Clauses may be written both before and after the typing (`:> a : T :> b`). The two groups
    // are the same relationship for the same feature, so they merge on the same grounds as a
    // repeat within one group -- see `specialization_clauses`.
    let (leading_subsets, leading_subsetting_value) = match leading.subsets {
        Some((relationship, value)) => (Some(relationship), value),
        None => (None, None),
    };
    let (trailing_subsets, trailing_subsetting_value) = match trailing.subsets {
        Some((relationship, value)) => (Some(relationship), value),
        None => (None, None),
    };
    let subsets = merge_groups(leading_subsets, trailing_subsets);
    let redefines = merge_groups(leading.redefines, trailing.redefines);
    let references = merge_groups(leading.references, trailing.references);
    let crosses = merge_groups(leading.crosses, trailing.crosses);
    let intersects = merge_groups(leading.intersects, trailing.intersects);
    let (_, type_reference, typing) = typing_reference_fields_from_result(type_result);
    UsageHeader {
        type_is_conjugated: typing.as_ref().is_some_and(|node| node.value.is_conjugated),
        type_reference,
        typing,
        subsets,
        subsetting_value: leading_subsetting_value.or(trailing_subsetting_value),
        redefines,
        references,
        crosses,
        intersects,
        had_specialization: leading.had_any || trailing.had_any,
        multiplicity: None,
        multiplicity_modifiers: crate::ast::MultiplicityModifiers::default(),
    }
}

/// Usage header for library-style feature usages: optional leading multiplicity,
/// typing, multiplicity, and a second typing/specialization group before the body. This follows
/// `FeatureSpecializationPart`'s `FeatureSpecialization+ MultiplicityPart?
/// FeatureSpecialization* | MultiplicityPart FeatureSpecialization*` ordering (SysML BNF
/// 424-426): a typing may appear on either side of the multiplicity just like every other
/// specialization alternative.
pub(crate) fn feature_usage_header(input: Input<'_>) -> IResult<Input<'_>, UsageHeader> {
    let (input, leading_multiplicity) = opt(multiplicity_node).parse(input)?;
    let (input, leading) = specialization_clauses(input)?;
    let (input, type_result) = optional_typings(input)?;
    let (input, trailing_multiplicity) = opt(multiplicity_node).parse(input)?;
    let (input, modifiers) = multiplicity_modifier_slots(input)?;
    let (input, trailing_type_result) = optional_typings(input)?;
    let (input, trailing) = specialization_clauses(input)?;
    let mut header = merge_usage_header(leading, trailing, type_result.or(trailing_type_result));
    header.multiplicity = trailing_multiplicity.or(leading_multiplicity);
    header.multiplicity_modifiers = modifiers;
    Ok((input, header))
}

/// Parse optional usage typing and specialization in either order:
/// - `<typing> <specialization>*`
/// - `<specialization>* <typing> <specialization>*`
pub(crate) fn usage_header(input: Input<'_>) -> IResult<Input<'_>, UsageHeader> {
    let (input, leading) = specialization_clauses(input)?;
    let (input, type_result) = optional_typings(input)?;
    let (input, trailing) = specialization_clauses(input)?;

    Ok((input, merge_usage_header(leading, trailing, type_result)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span_input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    #[test]
    fn typings_accepts_defined_by_and_multiple_targets() {
        let input = span_input("defined by ~Ports::Fuel, Ports::Command ;");
        let (rest, (_, is_conjugated, targets, _)) = typings(input).expect("typings");
        assert!(
            is_conjugated,
            "first target's leading `~` should be captured"
        );
        assert_eq!(
            reference_list_text(input, &targets),
            "Ports::Fuel, Ports::Command"
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn typings_accepts_typed_by_keyword_alias() {
        let input = span_input("typed by ~Ports::Fuel, Ports::Command ;");
        let (rest, (_, is_conjugated, targets, _)) = typings(input).expect("typings");
        assert!(is_conjugated);
        assert_eq!(
            reference_list_text(input, &targets),
            "Ports::Fuel, Ports::Command"
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    /// Helper for asserting a `Node<SubsettingRelationship>`'s target/kind in one line.
    fn rel_target_kind(
        input: Input<'_>,
        rel: &Node<SubsettingRelationship>,
    ) -> (String, SubsettingKind) {
        (
            reference_list_text(input, &rel.value.target),
            rel.value.kind,
        )
    }

    #[test]
    fn subsetting_accepts_keyword_alias_with_value() {
        let input = span_input("subsets wheel = rearWheel[1];");
        let (_, (target, value)) = subsetting(input).expect("subsetting");
        assert_eq!(
            rel_target_kind(input, &target),
            ("wheel".to_string(), SubsettingKind::Subsets)
        );
        assert!(!target.value.is_implied);
        assert!(value.is_some());
    }

    /// A clause kind written twice names further targets of the same relationship; both survive,
    /// in source order. Overwriting instead (what this asserted before) dropped `base` and `old`
    /// silently, which is what made `sysml.library`'s `subsets step, usage subsets
    /// Metadata::metadataItems` emit as a single-target `:> Metadata::metadataItems`.
    #[test]
    fn specialization_clauses_accepts_multiple_mixed_clauses() {
        let input = span_input("subsets base redefines old :> latest :>> newest ;");
        let (rest, clauses) = specialization_clauses(input).expect("specialization clauses");
        assert_eq!(
            clauses
                .subsets
                .as_ref()
                .map(|(rel, _)| rel_target_kind(input, rel)),
            Some(("base, latest".to_string(), SubsettingKind::Subsets))
        );
        assert_eq!(
            clauses
                .redefines
                .as_ref()
                .map(|rel| rel_target_kind(input, rel)),
            Some(("old, newest".to_string(), SubsettingKind::Redefines))
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn specialization_clauses_accept_dotted_feature_chain_targets() {
        let input = span_input(":> electricGrid.outlets :>> Vehicle::mass.value ;");
        let (rest, clauses) = specialization_clauses(input).expect("specialization clauses");
        assert_eq!(
            clauses
                .subsets
                .as_ref()
                .map(|(rel, _)| rel_target_kind(input, rel)),
            Some(("electricGrid.outlets".to_string(), SubsettingKind::Subsets))
        );
        assert_eq!(
            clauses
                .redefines
                .as_ref()
                .map(|rel| rel_target_kind(input, rel)),
            Some(("Vehicle::mass.value".to_string(), SubsettingKind::Redefines))
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn specialization_clauses_accept_multiple_targets() {
        let input = span_input(":> CoordinateTransformation, List {");
        let (rest, clauses) = specialization_clauses(input).expect("specialization clauses");
        assert_eq!(
            clauses
                .subsets
                .as_ref()
                .map(|(rel, _)| rel_target_kind(input, rel)),
            Some((
                "CoordinateTransformation, List".to_string(),
                SubsettingKind::Subsets
            ))
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b"{"));
    }

    /// A comma-separated `:>` clause retains two independently resolvable arena identities.
    #[test]
    fn specialization_clauses_multi_target_stays_structured_not_joined() {
        let source = ":> CoordinateTransformation, List {";
        let context = crate::parser::span::ParseContext::new();
        let input = context.input(source.as_bytes());
        let (_, clauses) = specialization_clauses(input).expect("specialization clauses");
        let (rel, _value) = clauses.subsets.expect("subsets clause");
        assert_eq!(
            rel.value.target.len(),
            2,
            "expected two distinct targets, not one joined string"
        );
        let first = rel.value.target[0];
        let second = rel.value.target[1];
        let arena = context.finish();
        let source = crate::ast::SourceStorage::from(source);
        let first = arena.get(&source, first).expect("first target view");
        let second = arena.get(&source, second).expect("second target view");
        assert_eq!(first.authored_text(), "CoordinateTransformation");
        assert_eq!(second.authored_text(), "List");
        assert_eq!(first.segments.len(), 1);
        assert_eq!(second.segments.len(), 1);
    }

    /// Regression (parser work item 2): `Vehicle::mass.value`'s `::` and `.` joins must stay
    /// distinguishable in the segment list, not collapse into one opaque string.
    #[test]
    fn specialization_target_distinguishes_colon_colon_from_dot_segments() {
        let source = ":>> Vehicle::mass.value ;";
        let context = crate::parser::span::ParseContext::new();
        let input = context.input(source.as_bytes());
        let (_, target) = redefinition(input).expect("redefinition");
        let first = target.value.first_target().expect("one target");
        let arena = context.finish();
        let source = crate::ast::SourceStorage::from(source);
        let view = arena.get(&source, first).expect("reference view");
        assert_eq!(view.authored_text(), "Vehicle::mass.value");
        assert_eq!(view.segments[0].separator_before, None);
        assert_eq!(
            view.segments[1].separator_before,
            Some(crate::ast::ReferenceSeparator::ColonColon)
        );
        assert_eq!(
            view.segments[2].separator_before,
            Some(crate::ast::ReferenceSeparator::Dot)
        );
    }

    #[test]
    fn usage_header_accepts_typing_then_specialization() {
        let input = span_input(": Engine :> BasePart :>> oldPart ;");
        let (rest, header) = usage_header(input).expect("usage header");
        assert_eq!(
            header
                .type_reference
                .and_then(|id| reference_text(input, id))
                .as_deref(),
            Some("Engine")
        );
        assert_eq!(
            header
                .subsets
                .as_ref()
                .map(|rel| rel_target_kind(input, rel)),
            Some(("BasePart".to_string(), SubsettingKind::Subsets))
        );
        assert_eq!(
            header
                .redefines
                .as_ref()
                .map(|rel| rel_target_kind(input, rel)),
            Some(("oldPart".to_string(), SubsettingKind::Redefines))
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn usage_header_accepts_specialization_then_typing() {
        let input = span_input("subsets base : Engine ;");
        let (rest, header) = usage_header(input).expect("usage header");
        assert_eq!(
            header
                .type_reference
                .and_then(|id| reference_text(input, id))
                .as_deref(),
            Some("Engine")
        );
        assert_eq!(
            header
                .subsets
                .as_ref()
                .map(|rel| rel_target_kind(input, rel)),
            Some(("base".to_string(), SubsettingKind::Subsets))
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn reference_subsetting_accepts_keyword() {
        let input = span_input("references portA ;");
        let (rest, target) = reference_subsetting(input).expect("references");
        assert_eq!(
            rel_target_kind(input, &target),
            ("portA".to_string(), SubsettingKind::References)
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn cross_subsetting_accepts_symbol() {
        let input = span_input("=> other ;");
        let (rest, target) = cross_subsetting(input).expect("crosses");
        assert_eq!(
            rel_target_kind(input, &target),
            ("other".to_string(), SubsettingKind::Crosses)
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn usage_header_preserves_references_and_crosses() {
        let input = span_input(": T references a crosses b ;");
        let (rest, header) = usage_header(input).expect("usage header");
        assert_eq!(
            header
                .type_reference
                .and_then(|id| reference_text(input, id))
                .as_deref(),
            Some("T")
        );
        assert_eq!(
            header
                .references
                .as_ref()
                .map(|rel| rel_target_kind(input, rel)),
            Some(("a".to_string(), SubsettingKind::References))
        );
        assert_eq!(
            header
                .crosses
                .as_ref()
                .map(|rel| rel_target_kind(input, rel)),
            Some(("b".to_string(), SubsettingKind::Crosses))
        );
        assert!(header.subsets.is_none());
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn specialization_clauses_multi_target_references() {
        let input = span_input("references a, b crosses c, d ;");
        let (rest, clauses) = specialization_clauses(input).expect("clauses");
        assert_eq!(
            clauses
                .references
                .as_ref()
                .map(|rel| rel_target_kind(input, rel)),
            Some(("a, b".to_string(), SubsettingKind::References))
        );
        assert_eq!(
            clauses
                .crosses
                .as_ref()
                .map(|rel| rel_target_kind(input, rel)),
            Some(("c, d".to_string(), SubsettingKind::Crosses))
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn intersecting_accepts_keyword() {
        let input = span_input("intersects other ;");
        let (rest, target) = intersecting(input).expect("intersects");
        assert_eq!(
            rel_target_kind(input, &target),
            ("other".to_string(), SubsettingKind::Intersects)
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn intersecting_accepts_multi_target() {
        let input = span_input("intersects a, b ;");
        let (rest, target) = intersecting(input).expect("intersects");
        assert_eq!(
            rel_target_kind(input, &target),
            ("a, b".to_string(), SubsettingKind::Intersects)
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn specialization_clauses_accepts_mixed_subsets_crosses_intersects() {
        let input = span_input("subsets a crosses b intersects c, d ;");
        let (rest, clauses) = specialization_clauses(input).expect("clauses");
        assert_eq!(
            clauses
                .subsets
                .as_ref()
                .map(|(target, _)| rel_target_kind(input, target)),
            Some(("a".to_string(), SubsettingKind::Subsets))
        );
        assert_eq!(
            clauses
                .crosses
                .as_ref()
                .map(|rel| rel_target_kind(input, rel)),
            Some(("b".to_string(), SubsettingKind::Crosses))
        );
        assert_eq!(
            clauses
                .intersects
                .as_ref()
                .map(|rel| rel_target_kind(input, rel)),
            Some(("c, d".to_string(), SubsettingKind::Intersects))
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn usage_header_preserves_intersects() {
        let input = span_input(": T intersects a ;");
        let (rest, header) = usage_header(input).expect("usage header");
        assert_eq!(
            header
                .type_reference
                .and_then(|id| reference_text(input, id))
                .as_deref(),
            Some("T")
        );
        assert_eq!(
            header
                .intersects
                .as_ref()
                .map(|rel| rel_target_kind(input, rel)),
            Some(("a".to_string(), SubsettingKind::Intersects))
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }
}

#[cfg(test)]
mod multiplicity_node_tests {
    use super::*;

    fn literal_bound(bound: &Option<Box<Node<Expression>>>) -> Option<i64> {
        match bound.as_deref().map(|node| &node.value) {
            Some(Expression::LiteralInteger(value)) => Some(*value),
            _ => None,
        }
    }

    fn parse_ok(src: &str) -> (String, Multiplicity) {
        let input = crate::parser::span::test_input(src);
        let (rest, node) = multiplicity_node(input).expect("multiplicity_node should parse");
        (
            String::from_utf8_lossy(rest.fragment()).into_owned(),
            node.value,
        )
    }

    #[test]
    fn bare_number_sets_lower_and_upper_equal() {
        let (_, m) = parse_ok("[3]");
        assert_eq!(literal_bound(&m.lower), Some(3));
        assert_eq!(m.lower, m.upper);
    }

    #[test]
    fn range_with_upper_bound() {
        let (_, m) = parse_ok("[0..1]");
        assert_eq!(literal_bound(&m.lower), Some(0));
        assert_eq!(literal_bound(&m.upper), Some(1));
    }

    #[test]
    fn range_with_unbounded_star_upper_does_not_hard_fail() {
        // Regression: expression()'s binary-operator chain commits once it matches `..` as a
        // range comparison operator and does not backtrack when the right-hand side (`*`) fails
        // to parse as a primary expression — multiplicity_node must not hand the whole bracket
        // content to expression() in one call, or this panics/hard-errors instead of parsing.
        let (rest, m) = parse_ok("[1..*] ordered : RocketEngine;");
        assert_eq!(literal_bound(&m.lower), Some(1));
        assert!(m.upper.is_none());
        assert_eq!(rest.trim_start(), "ordered : RocketEngine;");
    }

    #[test]
    fn bare_unbounded_star() {
        let (_, m) = parse_ok("[*]");
        assert!(m.lower.is_none() && m.upper.is_none());
    }

    #[test]
    fn bare_feature_ref_bound() {
        let input = crate::parser::span::test_input("[seBeforeNum]");
        let (_, node) = multiplicity_node(input).expect("multiplicity_node should parse");
        let Some(lower) = node.value.lower else {
            panic!("feature lower bound");
        };
        let Expression::FeatureRef(reference) = lower.value else {
            panic!("feature-reference lower bound");
        };
        assert_eq!(
            reference_text(input, reference).as_deref(),
            Some("seBeforeNum")
        );
        assert_eq!(node.value.upper.as_deref(), Some(lower.as_ref()));
    }
}
