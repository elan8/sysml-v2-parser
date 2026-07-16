//! Shared usage grammar fragments from `UsageDeclaration` / `FeatureSpecializationPart`.

use crate::ast::{
    Expression, Multiplicity, Node, RelationshipTarget, RelationshipTargetSegment,
    SegmentSeparator, Span, SubsettingKind, SubsettingRelationship,
};
use crate::parser::expr::expression;
use crate::parser::lex::{
    crosses_operator, name, qualified_name_segments, redefine_operator, references_operator,
    starts_with_keyword, subset_operator, typed_by_operator, ws_and_comments,
};
use crate::parser::{span_from_to, Input};
use nom::bytes::complete::{tag, take_until};
use nom::combinator::{map, opt};
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
    pub had_any: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct UsageHeader {
    pub type_name: Option<String>,
    pub subsets: Option<Node<SubsettingRelationship>>,
    pub redefines: Option<Node<SubsettingRelationship>>,
    pub references: Option<Node<SubsettingRelationship>>,
    pub crosses: Option<Node<SubsettingRelationship>>,
    pub had_specialization: bool,
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
pub(crate) type TypingsResult = (Span, bool, Vec<Node<RelationshipTarget>>);

/// Typings: `:` / `defined by` one or more qualified names, with optional conjugated `~`.
///
/// Returns `(span, is_conjugated, targets)`: `is_conjugated` reflects the first (and, per
/// SysML v2, realistically only) target's leading `~`; each target in `targets` has any `~`
/// stripped (callers that have a real `TypingRelationship` node to populate should use
/// `is_conjugated` directly instead of re-embedding `~` into a target). `targets` almost always
/// has exactly one element -- a comma-separated multi-target `:`/`defined by` clause is rare but
/// legal and is no longer collapsed into a single joined string (parser work item 2).
pub(crate) fn typings(input: Input<'_>) -> IResult<Input<'_>, TypingsResult> {
    let before = input;
    let (input, _) = preceded(ws_and_comments, typed_by_operator).parse(input)?;
    let (input, (first_conjugated, first)) =
        preceded(ws_and_comments, conjugated_qualified_name).parse(input)?;
    let (input, rest) = many0(preceded(
        preceded(ws_and_comments, tag(&b","[..])),
        preceded(ws_and_comments, conjugated_qualified_name),
    ))
    .parse(input)?;
    let mut targets = vec![first];
    targets.extend(rest.into_iter().map(|(_, target)| target));
    Ok((
        input,
        (span_from_to(before, input), first_conjugated, targets),
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

/// Join relationship targets into a single `", "`-separated display string, for the plain
/// `type_name: String` fields several usage structs carry (distinct from
/// `TypingRelationship`/`SubsettingRelationship.target`, which keep the structured
/// `Vec<Node<RelationshipTarget>>` -- see `crate::ast::relationship_target` module docs). Matches
/// this crate's historical joined-string behavior for those display-only fields.
pub(crate) fn targets_display_string(targets: &[Node<RelationshipTarget>]) -> String {
    targets
        .iter()
        .map(|t| t.value.to_display_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Build a `Node<RelationshipTarget>` from segments and the span they cover.
fn relationship_target_node(
    segments: Vec<RelationshipTargetSegment>,
    span: Span,
) -> Node<RelationshipTarget> {
    Node::new(span.clone(), RelationshipTarget { segments, span })
}

/// Parses an optional leading `~` and a qualified name; returns `(was_conjugated, target)` with
/// the `~` stripped from the target's segments rather than folded into them.
fn conjugated_qualified_name(
    input: Input<'_>,
) -> IResult<Input<'_>, (bool, Node<RelationshipTarget>)> {
    let before = input;
    let (input, conjugated) = opt(tag(&b"~"[..])).parse(input)?;
    let (input, segments) = qualified_name_segments(input)?;
    let span = span_from_to(before, input);
    Ok((
        input,
        (
            conjugated.is_some(),
            relationship_target_node(segments, span),
        ),
    ))
}

/// A single subsetting-family target: a `::`-qualified name optionally continued with
/// `.`-separated feature-chain segments, e.g. `Vehicle::mass.value`.
fn specialization_target(input: Input<'_>) -> IResult<Input<'_>, Node<RelationshipTarget>> {
    let before = input;
    let (input, mut segments) = qualified_name_segments(input)?;
    let (input, dotted) = many0(preceded(
        preceded(ws_and_comments, tag(&b"."[..])),
        preceded(ws_and_comments, name),
    ))
    .parse(input)?;
    segments.extend(dotted.into_iter().map(|name| RelationshipTargetSegment {
        name,
        separator: Some(SegmentSeparator::Dot),
    }));
    let span = span_from_to(before, input);
    Ok((input, relationship_target_node(segments, span)))
}

/// One or more comma-separated [`specialization_target`]s, e.g. the `Base, Other` in
/// `:> Base, Other`. Previously joined into a single `", "`-separated string, losing the fact
/// there were multiple distinct targets (parser work item 2).
fn specialization_targets(input: Input<'_>) -> IResult<Input<'_>, Vec<Node<RelationshipTarget>>> {
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

/// Build a `SubsettingRelationship` node from target(s) and the span of the whole clause
/// (operator/keyword through target).
fn subsetting_relationship_node(
    target: Vec<Node<RelationshipTarget>>,
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

enum SpecializationClause {
    Subsets((Node<SubsettingRelationship>, Option<Node<Expression>>)),
    Redefines(Node<SubsettingRelationship>),
    References(Node<SubsettingRelationship>),
    Crosses(Node<SubsettingRelationship>),
}

/// Parse zero or more subsetting/redefinition clauses in any order.
///
/// When multiple clauses of the same kind are present, the last one wins.
pub(crate) fn specialization_clauses(
    input: Input<'_>,
) -> IResult<Input<'_>, SpecializationClauses> {
    let (input, clauses) = many0(preceded(
        ws_and_comments,
        nom::branch::alt((
            nom::combinator::map(subsetting, SpecializationClause::Subsets),
            nom::combinator::map(redefinition, SpecializationClause::Redefines),
            nom::combinator::map(reference_subsetting, SpecializationClause::References),
            nom::combinator::map(cross_subsetting, SpecializationClause::Crosses),
        )),
    ))
    .parse(input)?;
    let mut out = SpecializationClauses::default();
    let had_any = !clauses.is_empty();
    for clause in clauses {
        match clause {
            SpecializationClause::Subsets(value) => out.subsets = Some(value),
            SpecializationClause::Redefines(value) => out.redefines = Some(value),
            SpecializationClause::References(value) => out.references = Some(value),
            SpecializationClause::Crosses(value) => out.crosses = Some(value),
        }
    }
    out.had_any = had_any;
    Ok((input, out))
}

fn skip_usage_feature_modifiers(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) = many0(preceded(
        ws_and_comments,
        nom::branch::alt((
            map(tag(&b"ordered"[..]), |_| ()),
            map(tag(&b"nonunique"[..]), |_| ()),
        )),
    ))
    .parse(input)?;
    Ok((input, ()))
}

fn skip_intersects_clause(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) = opt(preceded(
        preceded(ws_and_comments, tag(&b"intersects"[..])),
        preceded(ws_and_comments, specialization_targets),
    ))
    .parse(input)?;
    Ok((input, ()))
}

fn merge_usage_header(
    leading: SpecializationClauses,
    trailing: SpecializationClauses,
    type_result: Option<(Span, bool, Vec<Node<RelationshipTarget>>)>,
) -> UsageHeader {
    let subsets = trailing
        .subsets
        .or(leading.subsets)
        .map(|(target, _value)| target);
    let redefines = trailing.redefines.or(leading.redefines);
    let references = trailing.references.or(leading.references);
    let crosses = trailing.crosses.or(leading.crosses);
    UsageHeader {
        type_name: type_result.map(|(_, is_conjugated, targets)| {
            let name = targets_display_string(&targets);
            if is_conjugated {
                format!("~{name}")
            } else {
                name
            }
        }),
        subsets,
        redefines,
        references,
        crosses,
        had_specialization: leading.had_any || trailing.had_any,
    }
}

/// Usage header for library-style feature usages: optional leading multiplicity,
/// typing, trailing multiplicity, `ordered` / `nonunique`, subsetting/redefinition,
/// and optional `intersects` before the body.
pub(crate) fn feature_usage_header(input: Input<'_>) -> IResult<Input<'_>, UsageHeader> {
    let (input, _) = opt(multiplicity).parse(input)?;
    let (input, leading) = specialization_clauses(input)?;
    let (input, type_result) = optional_typings(input)?;
    let (input, _) = opt(multiplicity).parse(input)?;
    let (input, _) = skip_usage_feature_modifiers(input)?;
    let (input, trailing) = specialization_clauses(input)?;
    let (input, _) = skip_intersects_clause(input)?;
    Ok((input, merge_usage_header(leading, trailing, type_result)))
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
    use nom_locate::LocatedSpan;

    fn span_input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    #[test]
    fn typings_accepts_defined_by_and_multiple_targets() {
        let input = span_input("defined by ~Ports::Fuel, Ports::Command ;");
        let (rest, (_, is_conjugated, targets)) = typings(input).expect("typings");
        assert!(
            is_conjugated,
            "first target's leading `~` should be captured"
        );
        assert_eq!(
            targets_display_string(&targets),
            "Ports::Fuel, Ports::Command"
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn typings_accepts_typed_by_keyword_alias() {
        let input = span_input("typed by ~Ports::Fuel, Ports::Command ;");
        let (rest, (_, is_conjugated, targets)) = typings(input).expect("typings");
        assert!(is_conjugated);
        assert_eq!(
            targets_display_string(&targets),
            "Ports::Fuel, Ports::Command"
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    /// Helper for asserting a `Node<SubsettingRelationship>`'s target/kind in one line.
    fn rel_target_kind(rel: &Node<SubsettingRelationship>) -> (String, SubsettingKind) {
        (targets_display_string(&rel.value.target), rel.value.kind)
    }

    #[test]
    fn subsetting_accepts_keyword_alias_with_value() {
        let input = span_input("subsets wheel = rearWheel[1];");
        let (_, (target, value)) = subsetting(input).expect("subsetting");
        assert_eq!(
            rel_target_kind(&target),
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
                .map(|(rel, _)| rel_target_kind(rel)),
            Some(("latest".to_string(), SubsettingKind::Subsets))
        );
        assert_eq!(
            clauses.redefines.as_ref().map(rel_target_kind),
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
                .map(|(rel, _)| rel_target_kind(rel)),
            Some(("electricGrid.outlets".to_string(), SubsettingKind::Subsets))
        );
        assert_eq!(
            clauses.redefines.as_ref().map(rel_target_kind),
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
                .map(|(rel, _)| rel_target_kind(rel)),
            Some((
                "CoordinateTransformation, List".to_string(),
                SubsettingKind::Subsets
            ))
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b"{"));
    }

    /// Regression (parser work item 2): a comma-separated multi-target `:>` clause used to be
    /// collapsed into a single `", "`-joined `String`, losing the fact there were two distinct
    /// targets. `SubsettingRelationship::target` must now hold one `RelationshipTarget` per
    /// comma-separated name.
    #[test]
    fn specialization_clauses_multi_target_stays_structured_not_joined() {
        let input = span_input(":> CoordinateTransformation, List {");
        let (_, clauses) = specialization_clauses(input).expect("specialization clauses");
        let (rel, _value) = clauses.subsets.expect("subsets clause");
        assert_eq!(
            rel.value.target.len(),
            2,
            "expected two distinct targets, not one joined string"
        );
        assert_eq!(
            rel.value.target[0].value.segments,
            vec![RelationshipTargetSegment {
                name: "CoordinateTransformation".to_string(),
                separator: None,
            }]
        );
        assert_eq!(
            rel.value.target[1].value.segments,
            vec![RelationshipTargetSegment {
                name: "List".to_string(),
                separator: None,
            }]
        );
        assert_eq!(
            rel.value
                .first_target()
                .map(RelationshipTarget::to_display_string),
            Some("CoordinateTransformation".to_string())
        );
    }

    /// Regression (parser work item 2): `Vehicle::mass.value`'s `::` and `.` joins must stay
    /// distinguishable in the segment list, not collapse into one opaque string.
    #[test]
    fn specialization_target_distinguishes_colon_colon_from_dot_segments() {
        let input = span_input(":>> Vehicle::mass.value ;");
        let (_, target) = redefinition(input).expect("redefinition");
        let first = target.value.first_target().expect("one target");
        assert_eq!(
            first.segments,
            vec![
                RelationshipTargetSegment {
                    name: "Vehicle".to_string(),
                    separator: None
                },
                RelationshipTargetSegment {
                    name: "mass".to_string(),
                    separator: Some(SegmentSeparator::ColonColon),
                },
                RelationshipTargetSegment {
                    name: "value".to_string(),
                    separator: Some(SegmentSeparator::Dot),
                },
            ]
        );
        assert_eq!(first.to_display_string(), "Vehicle::mass.value");
    }

    #[test]
    fn usage_header_accepts_typing_then_specialization() {
        let input = span_input(": Engine :> BasePart :>> oldPart ;");
        let (rest, header) = usage_header(input).expect("usage header");
        assert_eq!(header.type_name.as_deref(), Some("Engine"));
        assert_eq!(
            header.subsets.as_ref().map(rel_target_kind),
            Some(("BasePart".to_string(), SubsettingKind::Subsets))
        );
        assert_eq!(
            header.redefines.as_ref().map(rel_target_kind),
            Some(("oldPart".to_string(), SubsettingKind::Redefines))
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn usage_header_accepts_specialization_then_typing() {
        let input = span_input("subsets base : Engine ;");
        let (rest, header) = usage_header(input).expect("usage header");
        assert_eq!(header.type_name.as_deref(), Some("Engine"));
        assert_eq!(
            header.subsets.as_ref().map(rel_target_kind),
            Some(("base".to_string(), SubsettingKind::Subsets))
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn reference_subsetting_accepts_keyword() {
        let input = span_input("references portA ;");
        let (rest, target) = reference_subsetting(input).expect("references");
        assert_eq!(
            rel_target_kind(&target),
            ("portA".to_string(), SubsettingKind::References)
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn cross_subsetting_accepts_symbol() {
        let input = span_input("=> other ;");
        let (rest, target) = cross_subsetting(input).expect("crosses");
        assert_eq!(
            rel_target_kind(&target),
            ("other".to_string(), SubsettingKind::Crosses)
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn usage_header_preserves_references_and_crosses() {
        let input = span_input(": T references a crosses b ;");
        let (rest, header) = usage_header(input).expect("usage header");
        assert_eq!(header.type_name.as_deref(), Some("T"));
        assert_eq!(
            header.references.as_ref().map(rel_target_kind),
            Some(("a".to_string(), SubsettingKind::References))
        );
        assert_eq!(
            header.crosses.as_ref().map(rel_target_kind),
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
            clauses.references.as_ref().map(rel_target_kind),
            Some(("a, b".to_string(), SubsettingKind::References))
        );
        assert_eq!(
            clauses.crosses.as_ref().map(rel_target_kind),
            Some(("c, d".to_string(), SubsettingKind::Crosses))
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }
}

#[cfg(test)]
mod multiplicity_node_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn parse_ok(src: &str) -> (String, Multiplicity) {
        let input = LocatedSpan::new(src.as_bytes());
        let (rest, node) = multiplicity_node(input).expect("multiplicity_node should parse");
        (
            String::from_utf8_lossy(rest.fragment()).into_owned(),
            node.value,
        )
    }

    #[test]
    fn bare_number_sets_lower_and_upper_equal() {
        let (_, m) = parse_ok("[3]");
        assert_eq!(m.to_bracket_string(), "[3]");
        assert_eq!(m.lower, m.upper);
    }

    #[test]
    fn range_with_upper_bound() {
        let (_, m) = parse_ok("[0..1]");
        assert_eq!(m.to_bracket_string(), "[0..1]");
    }

    #[test]
    fn range_with_unbounded_star_upper_does_not_hard_fail() {
        // Regression: expression()'s binary-operator chain commits once it matches `..` as a
        // range comparison operator and does not backtrack when the right-hand side (`*`) fails
        // to parse as a primary expression — multiplicity_node must not hand the whole bracket
        // content to expression() in one call, or this panics/hard-errors instead of parsing.
        let (rest, m) = parse_ok("[1..*] ordered : RocketEngine;");
        assert_eq!(m.to_bracket_string(), "[1..*]");
        assert_eq!(rest.trim_start(), "ordered : RocketEngine;");
    }

    #[test]
    fn bare_unbounded_star() {
        let (_, m) = parse_ok("[*]");
        assert_eq!(m.to_bracket_string(), "[*]");
        assert!(m.lower.is_none() && m.upper.is_none());
    }

    #[test]
    fn bare_feature_ref_bound() {
        let (_, m) = parse_ok("[seBeforeNum]");
        assert_eq!(m.to_bracket_string(), "[seBeforeNum]");
    }
}
