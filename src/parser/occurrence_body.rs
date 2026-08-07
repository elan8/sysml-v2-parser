//! Shared occurrence-style body parsing for occurrence defs and generic `DefinitionBody` users.

use crate::ast::{
    AssertConstraintMember, ConstraintDefBody, DefinitionBody, DefinitionBodyElement, Membership,
    Node, OccurrenceBodyElement, OccurrenceUsage, OccurrenceUsageBody, ParseErrorNode,
    SuccessionUsage,
};
use crate::parser::attribute::attribute_usage;
use crate::parser::body::parse_structured_brace_members;
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::connector::connect_body;
use crate::parser::constraint::{structured_constraint_body, StructuredConstraintBody};
use crate::parser::expr::path_expression;
use crate::parser::flow::flow_usage_member;
use crate::parser::lex::{
    capture_opaque_member, name, qualified_name, recover_body_element, starts_with_keyword,
    visibility_prefix, ws1, ws_and_comments,
};
use crate::parser::metadata_annotation::annotation;
use crate::parser::node_from_to;
use crate::parser::part::exhibit_state_as_state_usage;
use crate::parser::part::part_usage;
use crate::parser::requirement::{doc_comment, satisfy};
use crate::parser::usage::{
    multiplicity_node as multiplicity_parser, optional_typings, specialization_clauses,
    targets_display_string,
};
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

pub(crate) const OCCURRENCE_BODY_STARTERS: &[&[u8]] = &[
    b"allocate",
    b"doc",
    b"event",
    b"assert",
    b"satisfy",
    b"attribute",
    b"flow",
    b"message",
    b"succession",
    b"part",
    b"individual",
    b"occurrence",
    b"snapshot",
    b"timeslice",
    b"@",
    b"#",
    b"end",
    b"ref",
    b"abstract",
    b"private",
    b"in",
    b"connection",
];

// Note: "succession" is intentionally NOT in this list — `succession_usage()` in
// `occurrence_body_element` now parses the standalone succession usage for real (see below).
// `end` is structured via `OccurrenceBodyElement::EndDecl` (#73 / 12b-Allocation-1).
const DEFINITION_BODY_OPAQUE_STARTERS: &[&[u8]] =
    &[b"ref", b"abstract", b"private", b"in", b"connection"];

/// `;` or brace body with occurrence-style members (`attribute`, `part`, `occurrence`, …).
pub(crate) fn occurrence_definition_body(input: Input<'_>) -> IResult<Input<'_>, DefinitionBody> {
    occurrence_definition_body_with_labels(
        input,
        "definition body",
        "recovered_definition_body_element",
    )
}

pub(crate) fn occurrence_def_definition_body(
    input: Input<'_>,
) -> IResult<Input<'_>, DefinitionBody> {
    occurrence_definition_body_with_labels(
        input,
        "occurrence definition body",
        "recovered_occurrence_def_body_element",
    )
}

fn occurrence_definition_body_with_labels<'a>(
    input: Input<'a>,
    scope_label: &'static str,
    recovery_code: &'static str,
) -> IResult<Input<'a>, DefinitionBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, DefinitionBody::Semicolon));
    }
    let (input, elements) = parse_structured_brace_members(
        input,
        OCCURRENCE_BODY_STARTERS,
        scope_label,
        recovery_code,
        |input| {
            let start = input;
            let (input, element) = nom::branch::alt((
                nom::combinator::map(
                    |i| capture_opaque_member(i, DEFINITION_BODY_OPAQUE_STARTERS),
                    DefinitionBodyElement::Other,
                ),
                nom::combinator::map(
                    occurrence_body_element,
                    DefinitionBodyElement::OccurrenceMember,
                ),
            ))
            .parse(input)?;
            Ok((input, node_from_to(start, input, element)))
        },
        |start, end| {
            let recovery = build_recovery_error_node_from_span(
                start,
                end,
                OCCURRENCE_BODY_STARTERS,
                scope_label,
                recovery_code,
            );
            node_from_to(
                start,
                end,
                DefinitionBodyElement::Error(node_from_to(start, end, recovery)),
            )
        },
    )?;
    Ok((input, DefinitionBody::Brace { elements }))
}

/// Everything an occurrence usage's leading keywords contribute to the node it builds. Grouped
/// into one struct so [`occurrence_usage_tail`] keeps a readable signature as the BNF
/// `OccurrenceUsagePrefix` slots accumulate.
struct OccurrencePrefix {
    is_individual: bool,
    is_then: bool,
    is_event: bool,
    is_reference: bool,
    is_abstract: bool,
    is_constant: bool,
    portion_kind: Option<String>,
    membership: Membership,
}

impl Default for OccurrencePrefix {
    fn default() -> Self {
        Self {
            is_individual: false,
            is_then: false,
            is_event: false,
            is_reference: false,
            is_abstract: false,
            is_constant: false,
            portion_kind: None,
            membership: Membership::feature(None, crate::ast::Span::dummy()),
        }
    }
}

/// Optional `ref` keyword (BNF `RefPrefix`, §6 G29) before an occurrence usage's kind keyword.
fn occurrence_ref_prefix(input: Input<'_>) -> IResult<Input<'_>, bool> {
    let (input, kw) =
        opt(preceded(preceded(ws_and_comments, tag(&b"ref"[..])), ws1)).parse(input)?;
    Ok((input, kw.is_some()))
}

/// GH-51: `abstract`/`constant` prefix keywords (BNF `RefPrefix`, §8.2.2.9.2), ahead of `ref` per
/// that production's order. Real usage: Systems Library `Domain Libraries/Cause and Effect/
/// CausationConnections.sysml`'s `abstract constant ref occurrence causes[1..*] :>> causes :>
/// participant { ... }`.
fn occurrence_abstract_constant_prefix(input: Input<'_>) -> IResult<Input<'_>, (bool, bool)> {
    let (input, is_abstract) = opt(preceded(
        preceded(ws_and_comments, tag(&b"abstract"[..])),
        ws1,
    ))
    .parse(input)?;
    let (input, is_constant) = opt(preceded(
        preceded(ws_and_comments, tag(&b"constant"[..])),
        ws1,
    ))
    .parse(input)?;
    Ok((input, (is_abstract.is_some(), is_constant.is_some())))
}

pub(crate) fn occurrence_usage(input: Input<'_>) -> IResult<Input<'_>, Node<OccurrenceUsage>> {
    let (input, (visibility_span, visibility)) =
        preceded(ws_and_comments, visibility_prefix).parse(input)?;
    let (input, (is_abstract, is_constant)) = occurrence_abstract_constant_prefix(input)?;
    let (input, is_reference) = occurrence_ref_prefix(input)?;
    occurrence_usage_with_modifiers(
        input,
        OccurrencePrefix {
            is_reference,
            is_abstract,
            is_constant,
            membership: Membership::feature(visibility, visibility_span),
            ..Default::default()
        },
    )
}

pub(crate) fn individual_usage(input: Input<'_>) -> IResult<Input<'_>, Node<OccurrenceUsage>> {
    let (input, (visibility_span, visibility)) =
        preceded(ws_and_comments, visibility_prefix).parse(input)?;
    let (input, is_reference) = occurrence_ref_prefix(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"individual"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    occurrence_usage_tail(
        input,
        OccurrencePrefix {
            is_individual: true,
            is_reference,
            membership: Membership::feature(visibility, visibility_span),
            ..Default::default()
        },
    )
}

pub(crate) fn snapshot_usage(input: Input<'_>) -> IResult<Input<'_>, Node<OccurrenceUsage>> {
    let (input, (visibility_span, visibility)) =
        preceded(ws_and_comments, visibility_prefix).parse(input)?;
    let (input, is_reference) = occurrence_ref_prefix(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"snapshot"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    occurrence_usage_tail(
        input,
        OccurrencePrefix {
            is_reference,
            portion_kind: Some("snapshot".to_string()),
            membership: Membership::feature(visibility, visibility_span),
            ..Default::default()
        },
    )
}

pub(crate) fn timeslice_usage(input: Input<'_>) -> IResult<Input<'_>, Node<OccurrenceUsage>> {
    let (input, (visibility_span, visibility)) =
        preceded(ws_and_comments, visibility_prefix).parse(input)?;
    let (input, is_reference) = occurrence_ref_prefix(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"timeslice"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    occurrence_usage_tail(
        input,
        OccurrencePrefix {
            is_reference,
            portion_kind: Some("timeslice".to_string()),
            membership: Membership::feature(visibility, visibility_span),
            ..Default::default()
        },
    )
}

/// `then timeslice ...`: a succession-continuation form, not a distinct BNF production with its
/// own `BasicUsagePrefix`/visibility grammar (unlike `occurrence`/`individual`/`snapshot`/
/// `timeslice`, each `OccurrenceUsagePrefix`-backed per the BNF and captured above) -- ad hoc
/// site, `visibility: None`, matching this rollout's established convention for constructs with
/// no visibility grammar of their own (see `AttributeUsage`'s ad hoc sites).
pub(crate) fn then_timeslice_usage(input: Input<'_>) -> IResult<Input<'_>, Node<OccurrenceUsage>> {
    let (input, _) = preceded(ws_and_comments, tag(&b"then"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, is_reference) = occurrence_ref_prefix(input)?;
    let (input, _) = tag(&b"timeslice"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    occurrence_usage_tail(
        input,
        OccurrencePrefix {
            is_then: true,
            is_reference,
            portion_kind: Some("timeslice".to_string()),
            membership: Membership::feature(None, crate::ast::Span::dummy()),
            ..Default::default()
        },
    )
}

fn occurrence_usage_with_modifiers(
    input: Input<'_>,
    prefix: OccurrencePrefix,
) -> IResult<Input<'_>, Node<OccurrenceUsage>> {
    // §6 G7: `event occurrence <name>;` (BNF `EventOccurrenceUsage`) and its succession form
    // `then event occurrence <name>;`. Real usage: OMG spec Annex `17a-Sequence-Modeling.sysml`.
    // Handled here rather than as a separate parser so every dispatcher that already reaches
    // `occurrence_usage` picks both forms up.
    let (input, then_kw) =
        opt(preceded(preceded(ws_and_comments, tag(&b"then"[..])), ws1)).parse(input)?;
    let (input, event_kw) =
        opt(preceded(preceded(ws_and_comments, tag(&b"event"[..])), ws1)).parse(input)?;
    // `occurrence` is only optional after `event`: the reference form `event <path>;` names an
    // existing occurrence rather than declaring one (OMG spec Annex `17b-Sequence-Modeling.sysml`).
    let (input, occurrence_kw) = opt(preceded(
        preceded(ws_and_comments, tag(&b"occurrence"[..])),
        ws1,
    ))
    .parse(input)?;
    if occurrence_kw.is_none() && event_kw.is_none() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    occurrence_usage_tail(
        input,
        OccurrencePrefix {
            is_then: prefix.is_then || then_kw.is_some(),
            is_event: event_kw.is_some(),
            ..prefix
        },
    )
}

fn occurrence_usage_tail(
    input: Input<'_>,
    prefix: OccurrencePrefix,
) -> IResult<Input<'_>, Node<OccurrenceUsage>> {
    let start = input;
    // §6 G22: `occurrence :>> causes;` redefines an inherited occurrence without renaming it, so
    // the name is optional here (OMG spec Annex `14c-Language Extensions.sysml`). The dotted form
    // (`event publish_message.sourceEvent;`) names a nested feature, so a `.`-joined path is read
    // as the name, matching how `perform` handles `perform providePower.generateTorque`.
    let (input, name_str) = if starts_specialization_or_body(input) {
        (input, String::new())
    } else {
        occurrence_name_path(input)?
    };
    let (input, leading_clauses) = specialization_clauses(input)?;
    let (input, type_name) = optional_typings(input)?;
    let type_name = type_name.map(|(_, is_conjugated, targets)| {
        let name = targets_display_string(&targets);
        if is_conjugated {
            format!("~{name}")
        } else {
            name
        }
    });
    // GH-51: real usage carries a multiplicity here (`causes[1..*]`); see `OccurrenceUsage::
    // multiplicity`'s doc comment.
    let (input, multiplicity) = opt(preceded(ws_and_comments, multiplicity_parser)).parse(input)?;
    // `#73`: `abstract occurrence situations : Situation[*] nonunique;` — feature modifiers after
    // multiplicity; without skipping them the usage fails and becomes `KermlFeatureDecl`.
    let (input, _) = crate::parser::usage::skip_usage_feature_modifiers(input)?;
    let (input, trailing_clauses) = specialization_clauses(input)?;
    let (input, body) = occurrence_usage_body(input)?;
    let (input, post_body_clauses) = specialization_clauses(input)?;
    let subsets = post_body_clauses
        .subsets
        .map(|(name, _filter)| name)
        .or_else(|| trailing_clauses.subsets.map(|(name, _filter)| name))
        .or_else(|| leading_clauses.subsets.map(|(name, _filter)| name));
    let redefines = post_body_clauses
        .redefines
        .or(trailing_clauses.redefines)
        .or(leading_clauses.redefines);
    let references = post_body_clauses
        .references
        .or(trailing_clauses.references)
        .or(leading_clauses.references);
    let crosses = post_body_clauses
        .crosses
        .or(trailing_clauses.crosses)
        .or(leading_clauses.crosses);
    let intersects = post_body_clauses
        .intersects
        .or(trailing_clauses.intersects)
        .or(leading_clauses.intersects);
    let input = if post_body_clauses.had_any {
        let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
        input
    } else {
        input
    };
    Ok((
        input,
        node_from_to(
            start,
            input,
            OccurrenceUsage {
                is_individual: prefix.is_individual,
                is_then: prefix.is_then,
                is_event: prefix.is_event,
                is_reference: prefix.is_reference,
                is_abstract: prefix.is_abstract,
                is_constant: prefix.is_constant,
                portion_kind: prefix.portion_kind,
                name: name_str,
                type_name,
                multiplicity,
                subsets,
                redefines,
                references,
                crosses,
                intersects,
                body,
                membership: prefix.membership,
            },
        ),
    ))
}

/// True when the next token starts a specialization clause, a typing clause, or the body -- i.e.
/// the usage has no name of its own.
fn starts_specialization_or_body(input: Input<'_>) -> bool {
    let Ok((peek, _)) = ws_and_comments(input) else {
        return false;
    };
    let frag = peek.fragment();
    frag.starts_with(b":") || frag.starts_with(b"{") || frag.starts_with(b";")
}

/// `name` or `name.nested.feature` -- the occurrence's own name, or a path to the nested feature
/// being referenced.
fn occurrence_name_path(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, first) = name(input)?;
    let (input, rest) = nom::multi::many0(preceded(
        preceded(ws_and_comments, tag(&b"."[..])),
        preceded(ws_and_comments, name),
    ))
    .parse(input)?;
    Ok((
        input,
        std::iter::once(first)
            .chain(rest)
            .collect::<Vec<_>>()
            .join("."),
    ))
}

fn occurrence_usage_body(input: Input<'_>) -> IResult<Input<'_>, OccurrenceUsageBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b";"[..]), |_| OccurrenceUsageBody::Semicolon),
        occurrence_usage_body_brace,
    ))
    .parse(input)
}

fn occurrence_usage_body_brace(input: Input<'_>) -> IResult<Input<'_>, OccurrenceUsageBody> {
    let (mut input, _) = tag(&b"{"[..]).parse(input)?;
    let mut elements = Vec::new();
    loop {
        let (next, _) = ws_and_comments(input)?;
        input = next;
        if input.fragment().is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Eof,
            )));
        }
        if input.fragment().starts_with(b"}") {
            let (input, _) = preceded(ws_and_comments, tag(&b"}"[..])).parse(input)?;
            return Ok((input, OccurrenceUsageBody::Brace { elements }));
        }
        match occurrence_body_element(input) {
            Ok((next, element)) => {
                if next.location_offset() == input.location_offset() {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Many0,
                    )));
                }
                elements.push(element);
                input = next;
            }
            Err(_) => {
                let start_unknown = input;
                let (next, _) = recover_body_element(input, OCCURRENCE_BODY_STARTERS)?;
                if next.location_offset() == start_unknown.location_offset() {
                    let (input, _) = crate::parser::body::advance_to_closing_brace(input)?;
                    let (input, _) = preceded(ws_and_comments, tag(&b"}"[..])).parse(input)?;
                    return Ok((input, OccurrenceUsageBody::Brace { elements }));
                }
                let recovery = build_recovery_error_node_from_span(
                    start_unknown,
                    next,
                    OCCURRENCE_BODY_STARTERS,
                    "occurrence body",
                    "recovered_occurrence_body_element",
                );
                let node: Node<ParseErrorNode> = node_from_to(start_unknown, next, recovery);
                elements.push(node_from_to(
                    start_unknown,
                    next,
                    OccurrenceBodyElement::Error(node),
                ));
                input = next;
            }
        }
    }
}

pub(crate) fn occurrence_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<OccurrenceBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, elem) = alt((
        map(doc_comment, OccurrenceBodyElement::Doc),
        map(annotation, OccurrenceBodyElement::Annotation),
        map(
            assert_constraint_member,
            OccurrenceBodyElement::AssertConstraint,
        ),
        map(attribute_usage, OccurrenceBodyElement::AttributeUsage),
        // §6 G15: keyword-less `:>> name (= value)? (;|{ ... })` redefinition binding.
        map(
            crate::parser::attribute::redefinition_feature_binding,
            OccurrenceBodyElement::AttributeUsage,
        ),
        map(flow_usage_member, OccurrenceBodyElement::FlowUsage),
        map(succession_usage, OccurrenceBodyElement::SuccessionUsage),
        map(satisfy, OccurrenceBodyElement::Satisfy),
        // Allocation / connection ends in structured definition bodies (`allocation def { end …; }`).
        map(
            |i| crate::parser::connector::end_decl(i, true),
            OccurrenceBodyElement::EndDecl,
        ),
        // §6 G17: a nested `allocate` decomposing the enclosing allocation usage.
        map(
            crate::parser::part::allocate_,
            OccurrenceBodyElement::Allocate,
        ),
        map(part_usage, |p| {
            OccurrenceBodyElement::PartUsage(Box::new(p))
        }),
        // GH-87: `item x;` inside an occurrence body -- `item_usage` already fully supports the
        // bare (untyped, no value) form, it just wasn't dispatched here (`part_usage` above
        // already was).
        map(
            crate::parser::item::item_usage,
            OccurrenceBodyElement::ItemUsage,
        ),
        map(individual_usage, |n| {
            OccurrenceBodyElement::OccurrenceUsage(Box::new(n))
        }),
        map(snapshot_usage, |n| {
            OccurrenceBodyElement::OccurrenceUsage(Box::new(n))
        }),
        map(timeslice_usage, |n| {
            OccurrenceBodyElement::OccurrenceUsage(Box::new(n))
        }),
        map(then_timeslice_usage, |n| {
            OccurrenceBodyElement::OccurrenceUsage(Box::new(n))
        }),
        map(occurrence_usage, |n| {
            OccurrenceBodyElement::OccurrenceUsage(Box::new(n))
        }),
        // §6 G30: `exhibit (state)? <name> ...` inside occurrence/snapshot bodies (found while
        // closing G15/G18 — real usage: `exhibit vehicleStates.on { ... }` in OMG spec Annex
        // `6-Individual and Snapshots.sysml`).
        map(
            exhibit_state_as_state_usage,
            OccurrenceBodyElement::StateUsage,
        ),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

/// Standalone succession usage: `succession` multiplicity? (`first` multiplicity? source)?
/// `then` multiplicity? target `;` or `{ ... }`. Distinct from `succession flow X to Y;`
/// (handled by `flow_usage_member`, which requires the `flow` keyword right after
/// `succession`) and from the action-body `first ... then ...` control node (`FirstStmt`,
/// only valid inside an action body). Real usage from the SysML Systems Library
/// (`Flows.sysml`): `succession [seBeforeNum] first [0..1] sourceEvent then [0..1] self;`.
pub(crate) fn succession_usage(input: Input<'_>) -> IResult<Input<'_>, Node<SuccessionUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, _) = tag(&b"succession"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    // GH-51: optional name for the succession usage itself (BNF `SuccessionAsUsage`'s
    // `('succession' UsageDeclaration)?` prefix), e.g. `succession causalOrdering first a then
    // b;` (real usage: Systems Library `Domain Libraries/Cause and Effect/
    // CausationConnections.sysml`) -- mirrors `action::succession_prefix`'s name-optionality
    // check for the sibling `first`-embedded form. `flow` is excluded so a malformed `succession
    // flow ...` that reaches this parser (normally claimed first by `flow_usage_member`) doesn't
    // misread the keyword as a name.
    let (input, name) = {
        let (peek, _) = ws_and_comments(input)?;
        let frag = peek.fragment();
        // GH-92.3: a bare `:` (type clause with no name) also means "no name" here, e.g.
        // `succession : HappensJustBefore first a then b;` (`Vehicle Example/
        // VehicleIndividuals.sysml:49`) -- previously only "no name, `first`/multiplicity
        // follows directly" was recognized, so a leading `:` fell through to the name parser
        // below and failed outright.
        if starts_with_keyword(frag, b"first")
            || starts_with_keyword(frag, b"flow")
            || frag.starts_with(b"[")
            || (frag.starts_with(b":") && !frag.starts_with(b":>") && !frag.starts_with(b":>>"))
        {
            (input, None)
        } else {
            let (input, parsed_name) = preceded(ws_and_comments, name).parse(input)?;
            (input, Some(parsed_name))
        }
    };
    // GH-92.3: optional `: Type` clause on the succession usage itself, mirroring
    // `action::succession_prefix`'s identical `succession_type` handling for the sibling
    // action-body `first`-embedded form.
    let (input, type_name) = {
        let (peek, _) = ws_and_comments(input)?;
        if peek.fragment().starts_with(b":") && !peek.fragment().starts_with(b":>") {
            let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
            let (input, type_name) = preceded(ws_and_comments, qualified_name).parse(input)?;
            (input, Some(type_name))
        } else {
            (input, None)
        }
    };
    let (input, multiplicity) = opt(preceded(ws_and_comments, multiplicity_parser)).parse(input)?;
    let (input, _) =
        opt(preceded(ws_and_comments, preceded(tag(&b"first"[..]), ws1))).parse(input)?;
    let (input, source_multiplicity) =
        opt(preceded(ws_and_comments, multiplicity_parser)).parse(input)?;
    let (input, source) = preceded(ws_and_comments, path_expression).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"then"[..])).parse(input)?;
    let (input, target_multiplicity) =
        opt(preceded(ws_and_comments, multiplicity_parser)).parse(input)?;
    let (input, target) = preceded(ws_and_comments, path_expression).parse(input)?;
    let (input, body) = connect_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            SuccessionUsage {
                name,
                type_name,
                multiplicity,
                source,
                source_multiplicity,
                target,
                target_multiplicity,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

pub(crate) fn assert_constraint_member(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<AssertConstraintMember>> {
    let start = input;
    let (input, (visibility_span, visibility)) =
        preceded(ws_and_comments, visibility_prefix).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"assert"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, is_negated) = opt(preceded(tag(&b"not"[..]), ws1))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
    // GH-89: the `constraint` keyword itself is optional -- `assert <name> { ... }`, referencing
    // a previously-declared standalone `constraint` by name and rebinding its `in` parameters, is
    // real usage (`assert massAnalysis3 { in totalMass = mass; ... }`, Simple Tests/
    // ConstraintTest.sysml:78), richer than the already-supported `assert constraint ...` form.
    let (input, _) = opt(preceded(tag(&b"constraint"[..]), ws_and_comments)).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, name) = if input.fragment().starts_with(b"{") || input.fragment().starts_with(b";")
    {
        (input, None)
    } else {
        let (input, parsed_name) = name(input)?;
        (input, Some(parsed_name))
    };
    let (input, type_name) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let (input, body) = structured_constraint_body(input)?;
    let body = match body {
        StructuredConstraintBody::Semicolon => ConstraintDefBody::Semicolon,
        StructuredConstraintBody::Brace { elements } => ConstraintDefBody::Brace { elements },
    };
    Ok((
        input,
        node_from_to(
            start,
            input,
            AssertConstraintMember {
                name,
                type_name,
                body,
                is_negated,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

/// PARSER_BACKLOG_ROADMAP.md §6, G3: `assert_constraint_member` never parsed a name at all, so
/// real usage like `assert constraint engineSelectionRational { }` (OMG spec Annex
/// `10b-Trade-off Among Alternative Configurations.sysml`) fell through to opaque recovery --
/// not just missing from `PartUsageBodyElement`, but unmodeled in the grammar entirely.
#[cfg(test)]
mod assert_constraint_name_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    #[test]
    fn assert_constraint_accepts_a_name() {
        let (rest, node) =
            assert_constraint_member(input("assert constraint engineSelectionRational { }"))
                .expect("named assert constraint");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name.as_deref(), Some("engineSelectionRational"));
        assert!(!node.value.is_negated);
    }

    /// §6 G22 (found while fixing G4): the *typed* named form still fell through to opaque
    /// recovery after G3 added plain names. Real usage: OMG spec Annex `15_05-Unification of
    /// Expression and Constraint Definition.sysml`.
    #[test]
    fn assert_constraint_accepts_a_name_and_a_type() {
        let (rest, node) = assert_constraint_member(input(
            "assert constraint discBrakeFitConstraint_Alt: DiscBrakeFitConstraint_Alt { in wheel = WheelAssy::wheel; }",
        ))
        .expect("typed assert constraint");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.name.as_deref(),
            Some("discBrakeFitConstraint_Alt")
        );
        assert_eq!(
            node.value.type_name.as_deref(),
            Some("DiscBrakeFitConstraint_Alt")
        );
    }

    /// §6 G17: an allocation usage body may decompose the outer allocation with nested
    /// `allocate` members. Real usage: OMG spec Annex `12b-Allocation.sysml`.
    #[test]
    fn occurrence_body_accepts_a_nested_allocate() {
        let (rest, node) = occurrence_body_element(input(
            "allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;",
        ))
        .expect("nested allocate");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, OccurrenceBodyElement::Allocate(_)));
    }

    /// §6 G30: exhibit usages inside occurrence/snapshot bodies.
    #[test]
    fn occurrence_body_accepts_exhibit_state_usage() {
        let (rest, node) =
            occurrence_body_element(input("exhibit vehicleStates.on;")).expect("exhibit state");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, OccurrenceBodyElement::StateUsage(_)));
    }

    #[test]
    fn assert_constraint_anonymous_form_still_works() {
        let (rest, node) =
            assert_constraint_member(input("assert constraint { }")).expect("anonymous form");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, None);
    }

    #[test]
    fn assert_constraint_negated_named_form() {
        let (rest, node) = assert_constraint_member(input("assert not constraint c { }"))
            .expect("negated named form");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name.as_deref(), Some("c"));
        assert!(node.value.is_negated);
    }
}

#[cfg(test)]
mod membership_tests {
    use super::*;
    use crate::parser::usage::targets_display_string;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    #[test]
    fn occurrence_usage_captures_intersects() {
        let (rest, node) =
            occurrence_usage(input("occurrence o1 : O1 intersects a;")).expect("occurrence usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value
                .intersects
                .as_ref()
                .map(|n| targets_display_string(&n.value.target)),
            Some("a".to_string())
        );
    }

    // --- parser work item 4b (final sweep): Membership on OccurrenceUsage/SuccessionUsage ---

    #[test]
    fn occurrence_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) =
            occurrence_usage(input("protected occurrence o1 : O1;")).expect("occurrence usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Protected)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn occurrence_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = occurrence_usage(input("occurrence o1 : O1;")).expect("occurrence usage");
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn occurrence_usage_accepts_abstract_nonunique() {
        let (rest, node) = occurrence_usage(input(
            "abstract occurrence situations : Situation[*] nonunique;",
        ))
        .expect("abstract nonunique occurrence");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.is_abstract);
        assert_eq!(node.value.name, "situations");
    }

    #[test]
    fn individual_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) =
            individual_usage(input("private individual o1 : O1;")).expect("individual usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Private)
        );
    }

    #[test]
    fn snapshot_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) = snapshot_usage(input("public snapshot o1 : O1;")).expect("snapshot usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
    }

    #[test]
    fn timeslice_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) =
            timeslice_usage(input("protected timeslice o1 : O1;")).expect("timeslice usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Protected)
        );
    }

    #[test]
    fn then_timeslice_usage_always_has_no_membership_visibility() {
        // Ad hoc site with no visibility grammar of its own -- see the doc comment on
        // `then_timeslice_usage`.
        let (_, node) =
            then_timeslice_usage(input("then timeslice o1 : O1;")).expect("then timeslice usage");
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn succession_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) = succession_usage(input("protected succession first a then b;"))
            .expect("succession usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Protected)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn succession_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) =
            succession_usage(input("succession first a then b;")).expect("succession usage");
        assert_eq!(node.value.membership.visibility, None);
    }
}
