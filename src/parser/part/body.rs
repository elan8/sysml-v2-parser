use super::prelude::*;
use super::usage::{
    allocate_, bind_, connect_, interface_usage, part_ref_usage, part_usage, perform_action_decl,
    perform_usage, variant_usage,
};
use crate::parser::action::first_stmt;
use crate::parser::lex::skip_statement_or_block;

/// Part def body: ';' or '{' PartDefBodyElement* '}'
pub(crate) fn part_def_body(input: Input<'_>) -> IResult<Input<'_>, PartDefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((crate::parser::body::semicolon_body, part_def_body_brace)).parse(input)
}

fn try_part_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PartDefBodyElement>> {
    match crate::parser::span::reference_transaction(input, part_def_body_element) {
        Err(e)
            if starts_with_any_keyword(input.fragment(), PART_BODY_STARTERS)
                && starts_with_keyword(input.fragment(), b"part") =>
        {
            if let Ok((next, usage)) = crate::parser::span::reference_transaction(input, part_usage)
            {
                if next.location_offset() > input.location_offset() {
                    return Ok((
                        next,
                        node_from_to(input, next, PartDefBodyElement::PartUsage(Box::new(usage))),
                    ));
                }
            }
            Err(e)
        }
        other => other,
    }
}

fn part_def_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<PartDefBodyElement> {
    // Always emit Error (never Other): unrecognized tokens must surface as diagnostics for both
    // `parse` and `parse_for_editor`, matching package/attribute/port body recovery (GH-12).
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        PART_BODY_STARTERS,
        "part definition body",
        "recovered_part_def_body_element",
    );
    node_from_to(
        start,
        end,
        PartDefBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

/// GH-40 / BNF `SuccessionAsUsage` (§8.2.2.13.3): the generic `DefinitionBodyItem` grammar a
/// `part def` body uses only reaches `first`/`succession` succession syntax through
/// `NonOccurrenceUsageMember` -> `SuccessionAsUsage`, whose `'then' ownedRelationship +=
/// ConnectorEndMember` clause is mandatory. The bare `first target;` marker (no `then`) is a
/// distinct production, `InitialNodeMember`, reachable only from `ActionBodyItem` (action
/// def/usage bodies) -- not from `DefinitionBodyItem`. `first_stmt` is shared with action bodies
/// and makes `then` optional there (§6 G13), so this wrapper rejects the `then`-less form to keep
/// part def body dispatch grammar-accurate; `merge`/`decide`/`join`/`fork` (BNF `ActionNode`, via
/// `ActionNodeMember`) are `ActionBodyItem`-only outright and are intentionally not wired up here.
fn part_def_succession_stmt(input: Input<'_>) -> IResult<Input<'_>, Node<crate::ast::FirstStmt>> {
    let (rest, node) = first_stmt(input)?;
    if node.value.then.is_none() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Verify,
        )));
    }
    Ok((rest, node))
}

fn part_def_body_brace(input: Input<'_>) -> IResult<Input<'_>, PartDefBody> {
    let (input, members) = parse_structured_brace_members_with_skip(
        input,
        PART_BODY_STARTERS,
        "part definition body",
        "recovered_part_def_body_element",
        try_part_def_body_element,
        part_def_body_recovery,
        BraceMemberSkip::BodyElementRecover,
    )?;
    Ok((input, members.into_body()))
}

/// Exhibit state usage: `OccurrenceUsagePrefix` subset `exhibit` (`state`)? name (`:` type)?
/// (`:>`/`:>>` …)? body. GH-27: rebuilt on the same shared prefix/specialization helpers
/// `state_usage` (`crate::parser::state::state_usage`) composes -- `visibility_prefix`,
/// direction/`derived`/`abstract`/`ref`/`individual` prefix handling, `specialization_clauses`,
/// `optional_typings`, `multiplicity_node`, `skip_usage_feature_modifiers` -- so the two stay in
/// sync with the shared `StateUsageBody` tail. GH-45 added direction/`derived`/`individual` to
/// both in lockstep; `constant` and portion kind remain unsupported on both, see
/// `state_usage`'s doc comment for why.
pub(crate) fn exhibit_state(input: Input<'_>) -> IResult<Input<'_>, Node<ExhibitState>> {
    crate::parser::span::reference_transaction(input, exhibit_state_inner)
}

fn exhibit_state_inner(input: Input<'_>) -> IResult<Input<'_>, Node<ExhibitState>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, direction) = opt(crate::parser::attribute::direction_prefix).parse(input)?;
    let (input, is_derived) = opt(preceded(tag(&b"derived"[..]), ws1)).parse(input)?;
    let (input, is_abstract) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, is_reference) = opt(preceded(tag(&b"ref"[..]), ws1)).parse(input)?;
    let (input, is_individual) = opt(preceded(tag(&b"individual"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"exhibit"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    // §6 G18: the `state` keyword is optional -- `exhibit 'vehicle states' :>> VehicleA::'vehicle
    // states';` (OMG spec Annex `5-State-based Behavior-2.sysml`) exhibits an already-declared
    // state usage by redefinition, without redeclaring its kind.
    let (input, state_keyword) = opt(preceded(tag(&b"state"[..]), ws1)).parse(input)?;
    let (input, name, state_reference) = if state_keyword.is_some() {
        let (input, name) = name(input)?;
        (input, name, None)
    } else {
        let (input, reference) = crate::parser::lex::reference_path(input)?;
        (input, String::new(), Some(reference))
    };
    let (input, leading) = specialization_clauses(input)?;
    let (input, type_result) = optional_typings(input)?;
    let (input, multiplicity) = opt(multiplicity_node).parse(input)?;
    let (input, _) = crate::parser::usage::skip_usage_feature_modifiers(input)?;
    let (input, trailing) = specialization_clauses(input)?;
    let (_type_ref_span, _, typing) =
        crate::parser::usage::typing_reference_fields_from_result(type_result);
    let subsets = trailing
        .subsets
        .clone()
        .or(leading.subsets.clone())
        .map(|(target, _)| target);
    let redefines_before_body = trailing.redefines.clone().or(leading.redefines.clone());
    // §8.2.2.18.2: `ExhibitStateUsage` shares `StateUsageBody` with plain `state` usages, so the
    // same `parallel`/`initial` modifier is legal here too (OMG spec Annex `exhibit state
    // vehicleStates parallel { ... }` in `5-State-based Behavior-2.sysml`) -- GH-17.
    let (input, _) = opt(alt((
        preceded(preceded(ws_and_comments, tag(&b"parallel"[..])), ws1),
        preceded(preceded(ws_and_comments, tag(&b"initial"[..])), ws1),
    )))
    .parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = crate::parser::state::state_def_body(input)?;
    // `:>>` may also come *after* the body -- this predates the before-body support above (via
    // `specialization_clauses`) and stays supported; when present it wins over a before-body one.
    let before_post_body_redefines = input;
    let (input, post_body_redefines) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>>"[..])),
        preceded(ws_and_comments, qualified_reference),
    ))
    .parse(input)?;
    let post_body_redefines = post_body_redefines.map(|target| {
        let span = crate::parser::span_from_to(before_post_body_redefines, input);
        single_target_subsetting(span, crate::ast::SubsettingKind::Redefines, target)
    });
    let input = if post_body_redefines.is_some() {
        let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
        input
    } else {
        input
    };
    let redefines = post_body_redefines.or(redefines_before_body);
    Ok((
        input,
        node_from_to(
            start,
            input,
            ExhibitState {
                direction,
                is_derived: is_derived.is_some(),
                is_abstract: is_abstract.is_some(),
                is_reference: is_reference.is_some(),
                is_individual: is_individual.is_some(),
                name,
                state_reference,
                typing,
                multiplicity,
                subsets,
                redefines,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

fn part_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PartDefBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, elem) = alt((
        alt((
            map(
                crate::parser::body::annotating_member,
                PartDefBodyElement::Annotating,
            ),
            map(
                crate::parser::metadata_annotation::metadata_keyword_usage,
                PartDefBodyElement::MetadataKeywordUsage,
            ),
            map(annotation, PartDefBodyElement::Annotation),
            map(exhibit_state, PartDefBodyElement::ExhibitState),
            // Each `_def` parser must be tried before its `_usage` sibling: neither `calc_usage`
            // nor `constraint_usage` guards against a bare `def` keyword (same bug class as
            // `flow_usage_member`/`port_usage` below), so `calc def Foo {}` would otherwise
            // misparse as `CalcUsage` named "def". Grouped into a sub-`alt()` to stay under
            // nom's 21-branch limit on the outer one.
            alt((
                map(calc_def_required, PartDefBodyElement::CalcDef),
                map(calc_usage, PartDefBodyElement::CalcUsage),
                map(constraint_def, PartDefBodyElement::ConstraintDef),
                map(constraint_usage, PartDefBodyElement::ConstraintUsage),
                // §6 G16: a part body is a namespace, so it owns imports too.
                map(crate::parser::import::import_, PartDefBodyElement::Import),
            )),
            map(perform_action_decl, PartDefBodyElement::Perform),
            map(perform_usage, PartDefBodyElement::Perform),
            map(allocate_, PartDefBodyElement::Allocate),
            // `connection_def_required` must be tried before `connection_usage_member`: the
            // latter has no guard against a bare `def` keyword (same bug class as
            // `flow_usage_member`/`port_usage`/`calc_usage` above), so `connection def Foo {}`
            // would otherwise misparse as a connection usage named "def".
            map(connection_def_required, PartDefBodyElement::ConnectionDef),
            map(connection_usage_member, PartDefBodyElement::Connection),
            map(connect_, PartDefBodyElement::Connect),
            // `flow_def` (def_required internally) must be tried before `flow_usage_member`:
            // the latter has no guard against a bare `def` keyword being consumed as a flow
            // usage's name, which misparses `flow def DataFlow;` as `FlowUsage { name: "def" }`.
            map(flow_def, PartDefBodyElement::FlowDef),
            map(
                crate::parser::flow::flow_usage_member,
                PartDefBodyElement::FlowUsage,
            ),
            map(part_def, PartDefBodyElement::PartDef),
            // Nested KerML classifier declarations (`struct`/`classifier`/`datatype`/...,
            // spec42 Gap 38), keyword-gated so no other member shape is affected.
            map(crate::parser::package::kerml_classifier_structured, |n| {
                PartDefBodyElement::KermlClassifier(Box::new(n))
            }),
            map(variant_usage, PartDefBodyElement::VariantUsage),
            map(part_usage, |p| PartDefBodyElement::PartUsage(Box::new(p))),
            map(individual_usage, |n| {
                PartDefBodyElement::OccurrenceUsage(Box::new(n))
            }),
            map(snapshot_usage, |n| {
                PartDefBodyElement::OccurrenceUsage(Box::new(n))
            }),
        )),
        alt((
            map(timeslice_usage, |n| {
                PartDefBodyElement::OccurrenceUsage(Box::new(n))
            }),
            map(then_timeslice_usage, |n| {
                PartDefBodyElement::OccurrenceUsage(Box::new(n))
            }),
            map(occurrence_usage, |n| {
                PartDefBodyElement::OccurrenceUsage(Box::new(n))
            }),
            map(interface_usage, PartDefBodyElement::InterfaceUsage),
            map(interface_def_required, PartDefBodyElement::InterfaceDef),
            // `port_def_required` must be tried before `port_usage`: `port_usage` has no guard
            // against a bare `def` keyword (same bug class caught for `flow_usage_member` above),
            // so `port def Foo {}` would otherwise misparse as `PortUsage { name: "def" }`.
            map(port_def_required, PartDefBodyElement::PortDef),
            map(port_usage, PartDefBodyElement::PortUsage),
            // GH-14: `action_def` must be tried before `action_usage` -- the latter has no guard
            // against a bare `def` keyword being consumed as the usage's name for most kinds (same
            // bug class as `flow_usage_member`/`port_usage`/`calc_usage` above), but here it
            // actively rejects a following `def` (see `action_usage`'s explicit check), so without
            // this arm `action def Foo { }` fell through to opaque recovery entirely.
            map(action_def, PartDefBodyElement::ActionDef),
            // Kinded usages before plain `ref` / opaque catch-all so Systems Library forms like
            // `abstract ref action performedActions: Action[0..*] :> actions, enactedPerformances`
            // become real ActionUsage/StateUsage nodes.
            map(action_usage, |a| {
                PartDefBodyElement::ActionUsage(Box::new(a))
            }),
            map(state_usage, PartDefBodyElement::StateUsage),
            map(part_ref_usage, PartDefBodyElement::Ref),
            // GH-42 Gap 1: bare `bind a = b;` (BNF `BindingConnectorAsUsage`, §8.2.2.13.2) was
            // never dispatched here, even though `bind_` was already wired into part *usage*
            // bodies (`part_usage_body_element` below) -- mirrors that arm's placement.
            map(bind_, PartDefBodyElement::Bind),
            map(|i| attribute_def(i, true), PartDefBodyElement::AttributeDef),
            map(attribute_usage, PartDefBodyElement::AttributeUsage),
            map(
                attribute_usage_shorthand,
                PartDefBodyElement::DefaultReferenceUsage,
            ),
            map(enum_usage, PartDefBodyElement::EnumerationUsage),
            map(requirement_usage, PartDefBodyElement::RequirementUsage),
        )),
        alt((
            // PAR-002: nested `def` kinds that were previously only reachable at package level.
            // Each of these parsers already applies `DefinitionPrefixOptions::def_required()`
            // internally (state, requirement, occurrence, flow -- see their own modules), so
            // there is no PAR-001-class ambiguity risk stacking them ahead of their usage-only
            // siblings here: a bare (`def`-less) declaration always falls through to the usage
            // arm above/below instead.
            map(state_def, PartDefBodyElement::StateDef),
            map(
                crate::parser::enumeration::enum_def,
                PartDefBodyElement::EnumDef,
            ),
            map(requirement_def, PartDefBodyElement::RequirementDef),
            map(occurrence_def, PartDefBodyElement::OccurrenceDef),
            map(metadata_usage, PartDefBodyElement::MetadataUsage),
            map(metadata_def, PartDefBodyElement::MetadataDef),
            map(dependency, PartDefBodyElement::Dependency),
            map(item_def_required, PartDefBodyElement::ItemDef),
            // GH-89.9: directed `in`/`out` item usage, e.g. `out item pwrCmd:PwrCmd;` (Timeslice
            // and Snapshot Examples/TimeVaryingAttribute.sysml:14). Already supported in action
            // bodies via `action_def_body_element`'s equivalent `directed_item_usage` arm; part
            // def bodies had no such path. Tried before the plain `item_usage` arm since that one
            // has no direction-prefix handling of its own.
            map(
                crate::parser::item::directed_item_usage,
                PartDefBodyElement::ItemUsage,
            ),
            map(item_usage, PartDefBodyElement::ItemUsage),
            map(
                crate::parser::occurrence_body::assert_constraint_member,
                PartDefBodyElement::AssertConstraint,
            ),
            map(satisfy, PartDefBodyElement::Satisfy),
            map(
                unsupported_part_member,
                PartDefBodyElement::UnsupportedMember,
            ),
        )),
        alt((
            // PAR-002: remaining nested `def`/usage pairs, previously only reachable at package
            // level. Each `_def` parser here already requires the `def` keyword internally (see
            // each module), so per-pair ordering is `def` before `usage` throughout -- several of
            // these `_usage` parsers (`allocation_usage`, `view_usage`, `viewpoint_usage`,
            // `rendering_usage`) call a bare `name(input)` right after their keyword with no
            // guard against `def`, the same risk pattern already fixed for `flow`/`port`/`calc`/
            // `connection` above, so getting this order right matters here too.
            map(allocation_def, PartDefBodyElement::AllocationDef),
            map(allocation_usage, PartDefBodyElement::AllocationUsage),
            map(view_def, PartDefBodyElement::ViewDef),
            map(view_usage, PartDefBodyElement::ViewUsage),
            map(viewpoint_def, PartDefBodyElement::ViewpointDef),
            map(viewpoint_usage, PartDefBodyElement::ViewpointUsage),
            map(rendering_def, PartDefBodyElement::RenderingDef),
            map(rendering_usage, PartDefBodyElement::RenderingUsage),
            map(analysis_case_def, PartDefBodyElement::AnalysisCaseDef),
            map(analysis_case_usage, PartDefBodyElement::AnalysisCaseUsage),
            map(
                verification_case_def,
                PartDefBodyElement::VerificationCaseDef,
            ),
            map(
                verification_case_usage,
                PartDefBodyElement::VerificationCaseUsage,
            ),
            map(case_def, PartDefBodyElement::CaseDef),
            map(case_usage, PartDefBodyElement::CaseUsage),
            map(use_case_def, PartDefBodyElement::UseCaseDef),
            map(use_case_usage, PartDefBodyElement::UseCaseUsage),
        )),
        // GH-40 / BNF `SuccessionAsUsage` (§8.2.2.13.3, `NonOccurrenceUsageElement`): the bare and
        // `succession`-prefixed `first ... then ...` forms, previously only reachable inside
        // action bodies, even though `ConnectionTest.sysml` uses them directly inside a `part def`
        // body. `merge`/`decide`/`join`/`fork` (BNF `ActionNode`) are NOT included: per the BNF,
        // `ActionNodeMember` is reachable only from `ActionBodyItem`, not the generic
        // `DefinitionBodyItem` a part def body uses -- no real usage exercises them here either.
        map(part_def_succession_stmt, PartDefBodyElement::FirstStmt),
        // GH-89: `alias <name> for <target>;` nested inside a part definition body, previously
        // only reachable at package-body scope even though `Simple Tests/AliasTest.sysml:7` uses
        // it directly inside a `part def`.
        map(
            crate::parser::alias::alias_def,
            PartDefBodyElement::AliasDef,
        ),
        // GH-87: keyword-less `name;` / `name = expr;` feature binding (§6 G26), previously only
        // reachable inside action bodies even though `part def V { m; }` is real usage (Simple
        // Tests/AnalysisTest.sysml:4). Tried absolute last, after `attribute_usage_shorthand`
        // above (which requires `: Type`), so every keyword-led/typed member keeps priority.
        map(
            crate::parser::attribute::bare_or_valued_feature_binding,
            PartDefBodyElement::DefaultReferenceUsage,
        ),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

pub(crate) fn connection_usage_member(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<ConnectionUsageMember>> {
    crate::parser::span::reference_transaction(input, connection_usage_member_inner)
}

fn connection_usage_member_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<ConnectionUsageMember>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, is_reference) = opt(preceded(tag(&b"ref"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"connection"[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, name) = if input.fragment().starts_with(b":")
        || input.fragment().starts_with(b"{")
        || input.fragment().starts_with(b";")
    {
        (input, None)
    } else {
        let (input, parsed_name) = name(input)?;
        (input, Some(parsed_name))
    };
    let (input, leading_multiplicity) =
        opt(crate::parser::usage::multiplicity_node).parse(input)?;
    let (input, type_reference) = {
        let (peek, _) = ws_and_comments(input)?;
        if peek.fragment().starts_with(b":")
            && !peek.fragment().starts_with(b":>")
            && !peek.fragment().starts_with(b":>>")
        {
            let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
            let (input, parsed_type) =
                preceded(ws_and_comments, qualified_reference).parse(input)?;
            (input, Some(parsed_type))
        } else {
            (input, None)
        }
    };
    let (input, trailing_multiplicity) =
        opt(crate::parser::usage::multiplicity_node).parse(input)?;
    let multiplicity = leading_multiplicity.or(trailing_multiplicity);
    // PAR-007 widening: an inline `connect from to to (, extra)*` clause between the type and the
    // body, e.g. `connection link : Link connect sensorA.cmd to sensorB.cmd;`. Optional -- a
    // plain `connection link : Link;` declaration with no explicit binding must keep parsing.
    // This is the fallback `connection_def` now leaves for when its header scan detects a
    // swallowed `connect` keyword (see `connection_def`'s doc comment).
    let (input, connect) = opt(preceded(
        preceded(ws_and_comments, tag(&b"connect"[..])),
        preceded(ws1, connect_ends),
    ))
    .parse(input)?;
    let (connect_from, connect_to, connect_extra_ends) = match connect {
        Some((from, to, extra)) => (Some(from), Some(to), extra),
        None => (None, None, Vec::new()),
    };
    let (input, body) = connection_member_body(input)?;
    let before_subsets = input;
    let (input, trailing_subsets) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>"[..])),
        preceded(ws_and_comments, qualified_reference),
    ))
    .parse(input)?;
    let subsets = trailing_subsets.map(|target| {
        let span = crate::parser::span_from_to(before_subsets, input);
        single_target_subsetting(span, crate::ast::SubsettingKind::Subsets, target)
    });
    let before_redefines = input;
    let (input, trailing_redefines) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>>"[..])),
        preceded(ws_and_comments, qualified_reference),
    ))
    .parse(input)?;
    let redefines = trailing_redefines.map(|target| {
        let span = crate::parser::span_from_to(before_redefines, input);
        single_target_subsetting(span, crate::ast::SubsettingKind::Redefines, target)
    });
    let input = if subsets.is_some() || redefines.is_some() {
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
            ConnectionUsageMember {
                name,
                type_reference,
                multiplicity,
                connect_from,
                connect_to,
                connect_extra_ends,
                body,
                subsets,
                redefines,
                membership: crate::ast::Membership::feature(visibility, visibility_span),
                by_reference: is_reference.is_some(),
            },
        ),
    ))
}

/// Recognize a spec-valid connection-like member whose semantic production is not implemented in
/// this scope. The complete source span is retained, but no header text is scanned or guessed into
/// declaration/reference fields.
fn unsupported_part_member(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::UnsupportedGrammarNode>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    if !starts_with_any_keyword(input.fragment(), &[b"connection"]) {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    // `ref connection ...` is now handled structurally by `connection_usage_member`
    // (`by_reference`), which is tried before this fallback in the same `alt`. Only a bare
    // `connection ...` that `connection_usage_member` itself failed to parse reaches here.
    let (production, member_start) = (
        crate::ast::UnsupportedProduction::ConnectionUsageInPartDefinition,
        input,
    );
    let (mut input, _) = skip_statement_or_block(member_start)?;
    let (after_ws, _) = ws_and_comments(input)?;
    if after_ws.fragment().starts_with(b":>") || after_ws.fragment().starts_with(b":>>") {
        let (after_relationship, _) = skip_statement_or_block(after_ws)?;
        input = after_relationship;
    }
    let diagnostic = crate::ast::ParseErrorNode {
        message: "spec-valid connection-like member is not implemented in part definitions"
            .to_owned(),
        code: "unsupported_grammar_form".to_owned(),
        expected: Some("structured connection/reference usage".to_owned()),
        found: None,
        suggestion: Some(
            "Keep this syntax; parser support is incomplete rather than the model being malformed."
                .to_owned(),
        ),
        category: Some(crate::error::DiagnosticCategory::UnsupportedGrammarForm),
    };
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::UnsupportedGrammarNode {
                production,
                diagnostic,
            },
        ),
    ))
}

#[cfg(test)]
mod par_002_nested_def_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    #[test]
    fn part_def_body_accepts_nested_state_def() {
        let text = "state def Modes { state on; state off; }";
        let (rest, node) = part_def_body_element(input(text)).expect("state def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(
            matches!(node.value, PartDefBodyElement::StateDef(_)),
            "expected StateDef, got {:?}",
            node.value
        );
    }

    #[test]
    fn exhibit_shorthand_keeps_its_dotted_state_reference_in_the_arena() {
        let source = input("exhibit vehicleStates.on;");
        let (rest, node) = exhibit_state(source).expect("exhibit reference");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.name.is_empty());
        assert_eq!(
            node.value
                .state_reference
                .and_then(|id| crate::parser::usage::reference_text(source, id))
                .as_deref(),
            Some("vehicleStates.on")
        );
    }

    #[test]
    fn part_def_body_accepts_nested_enum_def_not_misparsed_as_usage() {
        let (rest, node) = part_def_body_element(input("enum def MyEnum;")).expect("enum def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::EnumDef(_)));
    }

    #[test]
    fn part_def_body_accepts_nested_metadata_def() {
        let text = "metadata def MyMeta;";
        let (rest, node) = part_def_body_element(input(text)).expect("metadata def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(
            matches!(node.value, PartDefBodyElement::MetadataDef(_)),
            "expected MetadataDef, got {:?}",
            node.value
        );
    }

    #[test]
    fn part_def_body_accepts_nested_flow_def() {
        let text = "flow def DataFlow;";
        let (rest, node) = part_def_body_element(input(text)).expect("flow def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(
            matches!(node.value, PartDefBodyElement::FlowDef(_)),
            "expected FlowDef, got {:?}",
            node.value
        );
    }

    #[test]
    fn part_def_body_accepts_nested_requirement_def() {
        let text = "requirement def SafetyReq;";
        let (rest, node) = part_def_body_element(input(text)).expect("requirement def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(
            matches!(node.value, PartDefBodyElement::RequirementDef(_)),
            "expected RequirementDef, got {:?}",
            node.value
        );
    }

    #[test]
    fn part_def_body_accepts_nested_occurrence_def() {
        let text = "occurrence def Failure;";
        let (rest, node) = part_def_body_element(input(text)).expect("occurrence def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(
            matches!(node.value, PartDefBodyElement::OccurrenceDef(_)),
            "expected OccurrenceDef, got {:?}",
            node.value
        );
    }

    #[test]
    fn part_def_body_accepts_nested_metadata_usage_and_still_prefers_def() {
        // Bare `metadata` (no `def`) must still dispatch to MetadataUsage, not misfire into
        // MetadataDef (metadata_def uses def_required() internally, so this also exercises that
        // guard rather than relying purely on dispatch order).
        let (rest, node) =
            part_def_body_element(input("metadata approvedBy;")).expect("metadata usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(
            matches!(node.value, PartDefBodyElement::MetadataUsage(_)),
            "expected MetadataUsage, got {:?}",
            node.value
        );
    }

    /// PAR-002 acceptance criterion: the same legal declaration retains the same semantic AST
    /// variant kind whether it appears at package level or nested in a part definition body.
    #[test]
    fn state_def_is_same_variant_kind_at_package_level_and_nested_in_part() {
        use crate::parser::package::package_body_element;

        let text = "state def Modes { state on; state off; }";
        let (_, package_node) = package_body_element(input(text)).expect("package-level state def");
        let (_, part_node) = part_def_body_element(input(text)).expect("nested state def");
        assert!(matches!(
            package_node.value,
            crate::ast::PackageBodyElement::StateDef(_)
        ));
        assert!(matches!(part_node.value, PartDefBodyElement::StateDef(_)));
    }

    #[test]
    fn enum_def_is_same_variant_kind_at_package_level_and_nested_in_part() {
        use crate::parser::package::package_body_element;

        let text = "enum def MyEnum;";
        let (_, package_node) = package_body_element(input(text)).expect("package-level enum def");
        let (_, part_node) = part_def_body_element(input(text)).expect("nested enum def");
        assert!(matches!(
            package_node.value,
            crate::ast::PackageBodyElement::EnumDef(_)
        ));
        assert!(matches!(part_node.value, PartDefBodyElement::EnumDef(_)));
    }

    #[test]
    fn requirement_def_is_same_variant_kind_at_package_level_and_nested_in_part() {
        use crate::parser::package::package_body_element;

        let text = "requirement def SafetyReq;";
        let (_, package_node) =
            package_body_element(input(text)).expect("package-level requirement def");
        let (_, part_node) = part_def_body_element(input(text)).expect("nested requirement def");
        assert!(matches!(
            package_node.value,
            crate::ast::PackageBodyElement::RequirementDef(_)
        ));
        assert!(matches!(
            part_node.value,
            PartDefBodyElement::RequirementDef(_)
        ));
    }

    // --- PAR-002 increment 2: connection/port/calc/allocation/view/viewpoint/rendering/
    // case/analysis-case/verification-case/use-case defs nested in part bodies ---

    #[test]
    fn part_def_body_accepts_nested_connection_def() {
        let (rest, node) =
            part_def_body_element(input("connection def MyConn;")).expect("connection def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::ConnectionDef(_)));
    }

    #[test]
    fn part_def_body_accepts_nested_connection_usage_not_misparsed_as_def() {
        // Bare (`def`-less) connection usage must still dispatch to the usage shape, not be
        // swallowed by `connection_def_required` (which requires `def`) nor misparse "def" as a
        // usage name via `connection_usage_member`.
        let (rest, node) =
            part_def_body_element(input("connection link: Link;")).expect("connection usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::Connection(_)));
    }

    #[test]
    fn part_def_body_accepts_nested_port_def_not_misparsed_as_usage() {
        let (rest, node) = part_def_body_element(input("port def MyPort;")).expect("port def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::PortDef(_)));
    }

    #[test]
    fn part_def_body_accepts_nested_port_usage() {
        let (rest, node) = part_def_body_element(input("port p1: MyPort;")).expect("port usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::PortUsage(_)));
    }

    #[test]
    fn part_def_body_accepts_nested_calc_def_not_misparsed_as_usage() {
        let (rest, node) = part_def_body_element(input("calc def MyCalc;")).expect("calc def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::CalcDef(_)));
    }

    #[test]
    fn part_def_body_accepts_nested_calc_usage() {
        let (rest, node) = part_def_body_element(input("calc c1: MyCalc;")).expect("calc usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::CalcUsage(_)));
    }

    #[test]
    fn part_def_body_accepts_nested_allocation_def() {
        let (rest, node) =
            part_def_body_element(input("allocation def MyAlloc;")).expect("allocation def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::AllocationDef(_)));
    }

    #[test]
    fn part_def_body_accepts_nested_view_def() {
        let (rest, node) = part_def_body_element(input("view def MyView;")).expect("view def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::ViewDef(_)));
    }

    #[test]
    fn part_def_body_accepts_nested_viewpoint_def() {
        let (rest, node) =
            part_def_body_element(input("viewpoint def MyViewpoint;")).expect("viewpoint def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::ViewpointDef(_)));
    }

    #[test]
    fn part_def_body_accepts_nested_rendering_def() {
        let (rest, node) =
            part_def_body_element(input("rendering def MyRendering;")).expect("rendering def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::RenderingDef(_)));
    }

    #[test]
    fn part_def_body_accepts_nested_case_def() {
        let (rest, node) = part_def_body_element(input("case def MyCase;")).expect("case def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::CaseDef(_)));
    }

    #[test]
    fn part_def_body_accepts_nested_analysis_case_def() {
        let (rest, node) =
            part_def_body_element(input("analysis def MyAnalysis;")).expect("analysis case def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::AnalysisCaseDef(_)));
    }

    #[test]
    fn part_def_body_accepts_nested_verification_case_def() {
        let (rest, node) = part_def_body_element(input("verification def MyVerification;"))
            .expect("verification case def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            PartDefBodyElement::VerificationCaseDef(_)
        ));
    }

    #[test]
    fn part_def_body_accepts_nested_use_case_def() {
        let (rest, node) =
            part_def_body_element(input("use case def MyUseCase;")).expect("use case def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::UseCaseDef(_)));
    }

    /// PAR-002 acceptance criterion, increment 2: same variant kind at package level vs. nested
    /// in a part body, for the def kinds most at risk of the bare-`def`-keyword ambiguity bug
    /// (connection/port/calc all had a real instance of it fixed in this increment).
    #[test]
    fn connection_def_is_same_variant_kind_at_package_level_and_nested_in_part() {
        use crate::parser::package::package_body_element;

        let text = "connection def MyConn;";
        let (_, package_node) =
            package_body_element(input(text)).expect("package-level connection def");
        let (_, part_node) = part_def_body_element(input(text)).expect("nested connection def");
        assert!(matches!(
            package_node.value,
            crate::ast::PackageBodyElement::ConnectionDef(_)
        ));
        assert!(matches!(
            part_node.value,
            PartDefBodyElement::ConnectionDef(_)
        ));
    }

    #[test]
    fn case_def_is_same_variant_kind_at_package_level_and_nested_in_part() {
        use crate::parser::package::package_body_element;

        let text = "case def MyCase;";
        let (_, package_node) = package_body_element(input(text)).expect("package-level case def");
        let (_, part_node) = part_def_body_element(input(text)).expect("nested case def");
        assert!(matches!(
            package_node.value,
            crate::ast::PackageBodyElement::CaseDef(_)
        ));
        assert!(matches!(part_node.value, PartDefBodyElement::CaseDef(_)));
    }

    #[test]
    fn part_def_body_parses_abstract_ref_action_as_action_usage() {
        let src =
            "abstract ref action performedActions: Action[0..*] :> actions, enactedPerformances;";
        let (rest, node) = part_def_body_element(input(src)).expect("part def body element");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            PartDefBodyElement::ActionUsage(action) => {
                assert!(action.value.is_abstract);
                assert!(action.value.is_reference);
                assert_eq!(action.value.name, "performedActions");
                assert!(action.value.type_name.is_some());
                assert!(action.value.subsets.is_some());
            }
            other => panic!("expected ActionUsage, got {other:?}"),
        }
    }

    /// PARSER_BACKLOG_ROADMAP.md §6, G4: the `constraint` keyword was wired at package level
    /// only, so both the definition and the usage form fell through to opaque recovery inside a
    /// part definition body. Real usage: OMG spec Annex `15_03-Value Expression.sysml` and
    /// `15_05-Unification of Expression and Constraint Definition.sysml`.
    #[test]
    fn part_def_body_accepts_untyped_constraint_usage_with_expression_body() {
        let (rest, node) = part_def_body_element(input(
            "constraint hasLegalProfileDepth {profileDepth >= 3.5 [mm]}",
        ))
        .expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            PartDefBodyElement::ConstraintUsage(c) => {
                assert_eq!(c.value.name, "hasLegalProfileDepth");
                assert_eq!(c.value.type_name, None);
            }
            other => panic!("expected ConstraintUsage, got {other:?}"),
        }
    }

    #[test]
    fn part_def_body_accepts_typed_constraint_usage_with_in_binding_body() {
        let (rest, node) = part_def_body_element(input(
            "constraint discBrakeConstraint : DiscBrakeConstraint { in wheelAssy = Vehicle_2::wheelAssy; }",
        ))
        .expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            PartDefBodyElement::ConstraintUsage(c) => {
                assert_eq!(c.value.name, "discBrakeConstraint");
                assert!(c.value.type_name.is_some());
            }
            other => panic!("expected ConstraintUsage, got {other:?}"),
        }
    }

    /// `constraint def` must win over `constraint_usage` in the same `alt()` -- the usage parser
    /// calls a bare `name(input)` right after the keyword and would take `def` as the name.
    #[test]
    fn part_def_body_accepts_nested_constraint_def_not_misparsed_as_usage() {
        let (rest, node) =
            part_def_body_element(input("constraint def MyConstraint;")).expect("constraint def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::ConstraintDef(_)));
    }

    #[test]
    fn part_def_body_parses_ref_state_as_state_usage() {
        let src = "ref state monitor: StateKind;";
        let (rest, node) = part_def_body_element(input(src)).expect("part def body element");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            PartDefBodyElement::StateUsage(state) => {
                assert!(state.value.is_reference);
                assert_eq!(state.value.name, "monitor");
                assert!(state.value.type_name.is_some());
            }
            other => panic!("expected StateUsage, got {other:?}"),
        }
    }

    /// GH-10: `ref part` is BNF `PartUsage` with `isReference`, so `:>` specialization must
    /// parse (same path as plain `part … :> …`), not the narrow `part_ref_usage` / `RefDecl`.
    #[test]
    fn part_def_body_parses_ref_part_with_subsetting_as_part_usage() {
        let src = "ref part origin : Remote :> remotes;";
        let (rest, node) = part_def_body_element(input(src)).expect("part def body element");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            PartDefBodyElement::PartUsage(part) => {
                assert!(part.value.is_reference);
                assert_eq!(part.value.name, "origin");
                assert_eq!(
                    part.value
                        .typing
                        .as_ref()
                        .map(|typing| typing.value.target.len()),
                    Some(1)
                );
                assert!(part.value.subsets.is_some());
            }
            other => panic!("expected PartUsage, got {other:?}"),
        }
    }

    #[test]
    fn part_def_body_keeps_bare_ref_as_ref_decl() {
        let src = "ref sensor: Sensor;";
        let (rest, node) = part_def_body_element(input(src)).expect("part def body element");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::Ref(_)));
    }

    /// GH-14: `action def` nested inside a `part def` body previously fell through to opaque
    /// recovery with a misleading "expected ';' or '{' after action definition header" diagnostic,
    /// even though the usage form (`action getTile;`) and the same definition at package level
    /// both parsed fine.
    #[test]
    fn part_def_body_accepts_nested_action_def_not_misparsed_as_usage() {
        let (rest, node) =
            part_def_body_element(input("action def GetTile { }")).expect("action def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::ActionDef(_)));
    }

    #[test]
    fn part_def_body_accepts_nested_action_usage() {
        let (rest, node) = part_def_body_element(input("action getTile;")).expect("action usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartDefBodyElement::ActionUsage(_)));
    }

    #[test]
    fn action_def_is_same_variant_kind_at_package_level_and_nested_in_part() {
        use crate::parser::package::package_body_element;

        let text = "action def GetTile { }";
        let (_, package_node) =
            package_body_element(input(text)).expect("package-level action def");
        let (_, part_node) = part_def_body_element(input(text)).expect("nested action def");
        assert!(matches!(
            package_node.value,
            crate::ast::PackageBodyElement::ActionDef(_)
        ));
        assert!(matches!(part_node.value, PartDefBodyElement::ActionDef(_)));
    }

    /// GH-14 issue sample: https://github.com/elan8/sysml-v2-parser/issues/14
    #[test]
    fn gh14_issue_sample_parses_cleanly() {
        let sample = r#"package Shop {
            part def TileCache {
                action def GetTile { }
            }
        }"#;
        crate::parse(sample).unwrap_or_else(|e| panic!("parse failed for sample:\n{sample}\n{e}"));
    }

    // --- GH-40: `first`/`succession` succession syntax (BNF `SuccessionAsUsage`) nested directly
    // in a part definition body, previously only reachable inside action bodies. `merge`/`decide`
    // (BNF `ActionNode`) are deliberately NOT covered here -- per the BNF, `ActionNodeMember` is
    // reachable only from `ActionBodyItem`, not the generic `DefinitionBodyItem` a part def body
    // uses. ---

    #[test]
    fn part_def_body_accepts_bare_first_then_stmt() {
        let (rest, node) =
            part_def_body_element(input("first a then b;")).expect("first ... then ...");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            PartDefBodyElement::FirstStmt(stmt) => {
                assert!(stmt.value.succession_name.is_none());
                assert!(stmt.value.then.is_some());
            }
            other => panic!("expected FirstStmt, got {other:?}"),
        }
    }

    #[test]
    fn part_def_body_accepts_named_succession_first_then_stmt() {
        let (rest, node) = part_def_body_element(input("succession s first a then b;"))
            .expect("succession first ... then ...");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            PartDefBodyElement::FirstStmt(stmt) => {
                assert_eq!(stmt.value.succession_name.as_deref(), Some("s"));
                assert!(stmt.value.succession_type.is_none());
            }
            other => panic!("expected FirstStmt, got {other:?}"),
        }
    }

    #[test]
    fn part_def_body_accepts_named_typed_succession_first_then_stmt() {
        let (rest, node) = part_def_body_element(input("succession s1 : AB first a then b;"))
            .expect("succession : Type first ... then ...");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            PartDefBodyElement::FirstStmt(stmt) => {
                assert_eq!(stmt.value.succession_name.as_deref(), Some("s1"));
                assert!(stmt.value.succession_type.is_some());
            }
            other => panic!("expected FirstStmt, got {other:?}"),
        }
    }

    /// BNF `ActionNode` (`merge`/`decide`/`join`/`fork`) is reachable only from `ActionBodyItem`
    /// (action def/usage bodies), not the generic `DefinitionBodyItem` a part def body uses, so
    /// these bare control nodes must still fall through to recovery here.
    #[test]
    fn part_def_body_rejects_bare_merge_and_decide_stmt() {
        assert!(part_def_body_element(input("merge M;")).is_err());
        assert!(part_def_body_element(input("decide D;")).is_err());
    }

    /// GH-42 Gap 1: bare `bind a = b;` (BNF `BindingConnectorAsUsage`, §8.2.2.13.2) was never
    /// dispatched inside a part definition body -- `bind_` was already wired into part *usage*
    /// bodies, same dispatch-gap class as `FirstStmt`/GH-40 above. Real usage:
    /// `sysml-v2-release/sysml/src/examples/Simple Tests/ConnectionTest.sysml` line 22, inside a
    /// `part def P { ... }` body. The `binding <name> (: Type)? bind ...` named-prefix form
    /// (Gap 2, same issue) is tracked and fixed separately.
    #[test]
    fn part_def_body_accepts_bare_bind_stmt() {
        let (rest, node) = part_def_body_element(input("bind a = b;")).expect("bind a = b;");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            PartDefBodyElement::Bind(bind) => {
                let crate::ast::Expression::FeatureRef(left) = &bind.value.left.value else {
                    panic!("expected left feature reference");
                };
                let crate::ast::Expression::FeatureRef(right) = &bind.value.right.value else {
                    panic!("expected right feature reference");
                };
                assert_ne!(left, right);
            }
            other => panic!("expected Bind, got {other:?}"),
        }
    }

    /// BNF `InitialNodeMember` (the `then`-less `first target;` marker) is likewise
    /// `ActionBodyItem`-only; only the full `SuccessionAsUsage` form with a mandatory `then` is
    /// part of `DefinitionBodyItem`.
    #[test]
    fn part_def_body_rejects_first_stmt_without_then() {
        assert!(part_def_body_element(input("first a;")).is_err());
    }

    /// GH-40 issue sample: https://github.com/elan8/sysml-v2-parser/issues/40 -- the
    /// `first`/`succession` lines from `sysml-v2-release/sysml/src/examples/Simple
    /// Tests/ConnectionTest.sysml`'s `part def P { ... }` body.
    #[test]
    fn gh40_issue_sample_parses_cleanly() {
        let sample = r#"package ConnectionTest {
            part def P {
                part a;
                part b;

                first a then b;
                succession s first a then b;
                succession s1 : AB first a then b;
            }
        }"#;
        crate::parse(sample).unwrap_or_else(|e| panic!("parse failed for sample:\n{sample}\n{e}"));
    }
}
