//! Shared usage grammar fragments from `UsageDeclaration` / `FeatureSpecializationPart`.

use crate::ast::{
    Expression, Multiplicity, Node, QualifiedReferenceId, Span, SubsettingKind,
    SubsettingRelationship, TypingKind, TypingRelationship,
};
use crate::parser::expr::expression;
use crate::parser::lex::{
    crosses_operator, qualified_reference, redefine_operator, reference_path, references_operator,
    starts_with_keyword, subset_operator, typed_by_operator, ws_and_comments,
};
use crate::parser::{span_from_to, Input};
use nom::bytes::complete::{tag, take_until};
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

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
    pub subsets: Option<Node<SubsettingRelationship>>,
    pub redefines: Option<Node<SubsettingRelationship>>,
    pub references: Option<Node<SubsettingRelationship>>,
    pub crosses: Option<Node<SubsettingRelationship>>,
    pub intersects: Option<Node<SubsettingRelationship>>,
    pub had_specialization: bool,
    /// Post-typing multiplicity clause, captured by [`feature_usage_header`] (the pre-typing
    /// clause position is typically consumed by the caller before the header). `None` for
    /// [`usage_header`], which has no multiplicity grammar.
    pub multiplicity: Option<Node<Multiplicity>>,
    /// `ordered` multiplicity property from `MultiplicityPart` (BNF §8.2.2.6.6), captured by
    /// [`feature_usage_header`]; previously skipped and discarded.
    pub ordered: bool,
    /// `nonunique` multiplicity property. See `ordered`.
    pub nonunique: bool,
}

/// Multiplicity part: '[' ... ']'.
pub(crate) fn multiplicity(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"["[..]).parse(input)?;
    let (input, content) = take_until(&b"]"[..]).parse(input)?;
    let (input, _) = tag(&b"]"[..]).parse(input)?;
    Ok((
        input,
        format!("[{}]", String::from_utf8_lossy(content.fragment()).trim()),
    ))
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
    Ok((
        input,
        Node::new(span.clone(), Multiplicity { lower, upper, span }),
    ))
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
    let type_ref_span = result.as_ref().map(|(span, _, _, _)| span.clone());
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
        span.clone(),
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

/// Build a single-target `redefines`/`:>>` relationship from an already-parsed arena identity,
/// mirroring [`single_target_typing`] for the same ad hoc `ref`-declaration call sites.
pub(crate) fn single_target_redefines(
    span: Span,
    target: QualifiedReferenceId,
) -> Node<SubsettingRelationship> {
    single_target_subsetting(span, SubsettingKind::Redefines, target)
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

/// Build a `SubsettingRelationship` node from target(s) and the span of the whole clause
/// (operator/keyword through target).
pub(crate) fn subsetting_relationship_node(
    target: Vec<QualifiedReferenceId>,
    kind: SubsettingKind,
    span: Span,
) -> Node<SubsettingRelationship> {
    Node::new(
        span.clone(),
        SubsettingRelationship {
            target,
            kind,
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
    let (input, _) = preceded(ws_and_comments, subset_operator).parse(input)?;
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
    let node = subsetting_relationship_node(target, SubsettingKind::Subsets, span);
    Ok((input, (node, value)))
}

/// Redefinitions: `:>>` / `redefines` target.
pub(crate) fn redefinition(input: Input<'_>) -> IResult<Input<'_>, Node<SubsettingRelationship>> {
    let before = input;
    let (input, target) = preceded(
        preceded(ws_and_comments, redefine_operator),
        preceded(ws_and_comments, specialization_targets),
    )
    .parse(input)?;
    let span = span_from_to(before, input);
    Ok((
        input,
        subsetting_relationship_node(target, SubsettingKind::Redefines, span),
    ))
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
    let (input, target) = preceded(
        preceded(ws_and_comments, references_operator),
        preceded(ws_and_comments, specialization_targets),
    )
    .parse(input)?;
    let span = span_from_to(before, input);
    Ok((
        input,
        subsetting_relationship_node(target, SubsettingKind::References, span),
    ))
}

/// Cross subsetting: `=>` / `crosses` target.
pub(crate) fn cross_subsetting(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<SubsettingRelationship>> {
    let before = input;
    let (input, target) = preceded(
        preceded(ws_and_comments, crosses_operator),
        preceded(ws_and_comments, specialization_targets),
    )
    .parse(input)?;
    let span = span_from_to(before, input);
    Ok((
        input,
        subsetting_relationship_node(target, SubsettingKind::Crosses, span),
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

/// Parse zero or more subsetting/redefinition clauses in any order.
///
/// When multiple clauses of the same kind are present, the last one wins.
pub(crate) fn specialization_clauses(
    input: Input<'_>,
) -> IResult<Input<'_>, SpecializationClauses> {
    // Clauses accumulate directly: collecting them first would allocate on a path the grammar
    // re-enters speculatively for every usage and definition header. A later clause of the same
    // kind overwrites an earlier one, as the fold it replaces did.
    let mut out = SpecializationClauses::default();
    let mut input = input;
    loop {
        let (after_ws, _) = ws_and_comments(input)?;
        if let Ok((rest, value)) = subsetting(after_ws) {
            out.subsets = Some(value);
            input = rest;
        } else if let Ok((rest, value)) = redefinition(after_ws) {
            out.redefines = Some(value);
            input = rest;
        } else if let Ok((rest, value)) = reference_subsetting(after_ws) {
            out.references = Some(value);
            input = rest;
        } else if let Ok((rest, value)) = cross_subsetting(after_ws) {
            out.crosses = Some(value);
            input = rest;
        } else if let Ok((rest, value)) = intersecting(after_ws) {
            out.intersects = Some(value);
            input = rest;
        } else {
            // Leave the whitespace before a non-clause unconsumed, as `preceded` did.
            return Ok((input, out));
        }
        out.had_any = true;
    }
}

pub(crate) fn skip_usage_feature_modifiers(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) = usage_feature_modifier_flags(input)?;
    Ok((input, ()))
}

/// Parse zero or more `ordered` / `nonunique` multiplicity-property keywords (BNF
/// `MultiplicityPart`), returning which of the two were written as `(ordered, nonunique)`
/// flags. Like [`skip_usage_feature_modifiers`], leaves the whitespace before a non-modifier
/// token unconsumed.
pub(crate) fn usage_feature_modifier_flags(input: Input<'_>) -> IResult<Input<'_>, (bool, bool)> {
    let mut input = input;
    let mut ordered = false;
    let mut nonunique = false;
    loop {
        let (after_ws, _) = ws_and_comments(input)?;
        if let Some(rest) = consume_literal(after_ws, b"ordered") {
            ordered = true;
            input = rest;
        } else if let Some(rest) = consume_literal(after_ws, b"nonunique") {
            nonunique = true;
            input = rest;
        } else {
            // Leave the whitespace before a non-modifier unconsumed, as `preceded` did.
            return Ok((input, (ordered, nonunique)));
        }
    }
}

/// Consume `literal` if the input starts with it, matching `nom`'s `tag` (no word boundary).
fn consume_literal<'a>(input: Input<'a>, literal: &[u8]) -> Option<Input<'a>> {
    input
        .fragment()
        .starts_with(literal)
        .then(|| nom::Input::take_from(&input, literal.len()))
}

fn merge_usage_header(
    leading: SpecializationClauses,
    trailing: SpecializationClauses,
    type_result: Option<TypingsResult>,
) -> UsageHeader {
    let subsets = trailing
        .subsets
        .or(leading.subsets)
        .map(|(target, _value)| target);
    let redefines = trailing.redefines.or(leading.redefines);
    let references = trailing.references.or(leading.references);
    let crosses = trailing.crosses.or(leading.crosses);
    let intersects = trailing.intersects.or(leading.intersects);
    UsageHeader {
        type_is_conjugated: type_result
            .as_ref()
            .is_some_and(|(_, is_conjugated, _, _)| *is_conjugated),
        type_reference: type_result
            .as_ref()
            .and_then(|(_, _, targets, _)| targets.first().copied()),
        subsets,
        redefines,
        references,
        crosses,
        intersects,
        had_specialization: leading.had_any || trailing.had_any,
        multiplicity: None,
        ordered: false,
        nonunique: false,
    }
}

/// Usage header for library-style feature usages: optional leading multiplicity,
/// typing, trailing multiplicity, `ordered` / `nonunique`, subsetting/redefinition,
/// and `intersects` (folded into `specialization_clauses`, called for both the leading and
/// trailing position) before the body.
pub(crate) fn feature_usage_header(input: Input<'_>) -> IResult<Input<'_>, UsageHeader> {
    let (input, leading_multiplicity) = opt(multiplicity_node).parse(input)?;
    let (input, leading) = specialization_clauses(input)?;
    let (input, type_result) = optional_typings(input)?;
    let (input, trailing_multiplicity) = opt(multiplicity_node).parse(input)?;
    let (input, (ordered, nonunique)) = usage_feature_modifier_flags(input)?;
    let (input, trailing) = specialization_clauses(input)?;
    let mut header = merge_usage_header(leading, trailing, type_result);
    header.multiplicity = trailing_multiplicity.or(leading_multiplicity);
    header.ordered = ordered;
    header.nonunique = nonunique;
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

    #[test]
    fn specialization_clauses_accepts_multiple_mixed_clauses() {
        let input = span_input("subsets base redefines old :> latest :>> newest ;");
        let (rest, clauses) = specialization_clauses(input).expect("specialization clauses");
        assert_eq!(
            clauses
                .subsets
                .as_ref()
                .map(|(rel, _)| rel_target_kind(input, rel)),
            Some(("latest".to_string(), SubsettingKind::Subsets))
        );
        assert_eq!(
            clauses
                .redefines
                .as_ref()
                .map(|rel| rel_target_kind(input, rel)),
            Some(("newest".to_string(), SubsettingKind::Redefines))
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
