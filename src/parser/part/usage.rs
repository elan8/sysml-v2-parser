use super::body::{connection_usage_member, exhibit_state};
use super::prelude::*;
use crate::parser::attribute::directed_attribute_usage;
use crate::parser::feature_value_part as usage_value_part;
use crate::parser::item::item_usage;

fn usage_ordered_modifier(input: Input<'_>) -> IResult<Input<'_>, (bool, bool)> {
    let (input, ordered) = opt(preceded(ws_and_comments, tag(&b"ordered"[..]))).parse(input)?;
    let (input, nonunique) = opt(preceded(ws_and_comments, tag(&b"nonunique"[..]))).parse(input)?;
    Ok((input, (ordered.is_some(), nonunique.is_some())))
}

/// Everything a `PartUsage` head recognizes before the declaration.
///
/// `PartUsage = OccurrenceUsagePrefix 'part' Usage` (SysML BNF 623), and `Usage` opens with
/// `Identification`, whose `( '<' ShortName '>' )?` half is read once at the head rather than in
/// each of the three declaration tails. `MemberPrefix`'s visibility belongs to the
/// `OccurrenceUsageMember`/`PackageMember` around the usage, not to the prefix, so it travels
/// here as a [`Membership`] and is stored as one.
///
/// The tails take this by reference and clone it into the node they build. Before this seam each
/// tail defaulted six prefix fields and each head assigned over them one by one, which is how
/// `part_def_or_usage` came to accept a different set of slots than `part_usage`.
pub(crate) struct PartUsageHead {
    prefix: crate::ast::OccurrenceUsagePrefix,
    /// `SourceSuccessionMember`'s `then`, which precedes the membership and therefore the prefix.
    then_span: Option<crate::ast::Span>,
    short_name: Option<String>,
    membership: Membership,
}

/// Part usage redefines-only: (`:>>` | `redefines`) qualified_name multiplicity? ordered? value? body.
fn part_usage_redefines_only<'a>(
    start: Input<'a>,
    input: Input<'a>,
    head: &PartUsageHead,
) -> IResult<Input<'a>, Node<PartUsage>> {
    let (input, (_, redefines_qname)) = prefix_redefinition_target(input)?;
    // GH-92.2: an explicit `: Type` clause may follow the redefines target, e.g. `part redefines
    // rb : LightRollBar[0..1];` (`v1 Spec Examples/8.4.5 Constraining Decomposition/Vehicle
    // Decomposition - Updated.sysml:43`) -- previously only the type-less bare (`part redefines
    // lb;`) and braced-body (`part redefines engine { ... }`) forms were accepted.
    let (input, type_result) = optional_typings(input)?;
    let (type_ref_span, _, typing) =
        crate::parser::usage::typing_reference_fields_from_result(type_result);
    let (input, multiplicity_opt) = opt(multiplicity_node).parse(input)?;
    let (input, (ordered, nonunique)) = usage_ordered_modifier(input)?;
    let (input, value) = opt(preceded(ws_and_comments, usage_value_part)).parse(input)?;
    let (input, body) = part_usage_body(input)?;
    // This form has no declaration name; the target spelling lives only in `redefines`.
    Ok((
        input,
        node_from_to(
            start,
            input,
            PartUsage {
                prefix: head.prefix.clone(),
                then_span: head.then_span.clone(),
                name: String::new(),
                short_name: head.short_name.clone(),
                typing,
                multiplicity: multiplicity_opt,
                ordered,
                nonunique,
                subsets: None,
                redefines: Some(redefines_qname),
                value,
                body,
                name_span: None,
                type_ref_span,
                membership: head.membership.clone(),
            },
        ),
    ))
}

/// Part usage with name (and optional type, redefines, etc.): (':>>')? name ':' type_name? ...
fn part_usage_named<'a>(
    start: Input<'a>,
    input: Input<'a>,
    head: &PartUsageHead,
) -> IResult<Input<'a>, Node<PartUsage>> {
    let (input, _) = opt(preceded(ws_and_comments, tag(&b":>>"[..]))).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (name_span, name_str)) = with_span(name).parse(input)?;
    let (input, multiplicity_opt) = opt(multiplicity_node).parse(input)?;
    let (input, (ordered_before_type, nonunique_before_type)) = usage_ordered_modifier(input)?;
    let (input, early_typing) = optional_typings(input)?;
    let (input, trailing_multiplicity_opt) = opt(multiplicity_node).parse(input)?;
    let multiplicity_opt = multiplicity_opt.or(trailing_multiplicity_opt);
    let (input, (ordered_after_type, nonunique_after_type)) = usage_ordered_modifier(input)?;
    let ordered = ordered_before_type || ordered_after_type;
    let nonunique = nonunique_before_type || nonunique_after_type;
    let (input, leading_clauses) = specialization_clauses(input)?;
    // Typing may follow redefinition: `in part anEngine :>> alternative : Engine;` (validation `10b`).
    let (input, type_result) = if early_typing.is_some() {
        (input, early_typing)
    } else {
        optional_typings(input)?
    };
    let type_ref_span = type_result.as_ref().map(|(span, _, _, _)| span.clone());
    let typing = type_result.map(|(span, is_conjugated, targets, spelling)| {
        typing_node(span, is_conjugated, targets, spelling)
    });
    let (input, post_clause_multiplicity) = opt(multiplicity_node).parse(input)?;
    let multiplicity_opt = multiplicity_opt.or(post_clause_multiplicity);
    let (input, (ordered_after_clauses, nonunique_after_clauses)) = usage_ordered_modifier(input)?;
    let ordered = ordered || ordered_after_clauses;
    let nonunique = nonunique || nonunique_after_clauses;
    let (input, value) = opt(preceded(ws_and_comments, usage_value_part)).parse(input)?;
    let (input, body) = part_usage_body(input)?;
    let (input, trailing_clauses) = specialization_clauses(input)?;
    let subsets = trailing_clauses
        .subsets
        .clone()
        .or(leading_clauses.subsets.clone());
    let redefines = trailing_clauses
        .redefines
        .clone()
        .or(leading_clauses.redefines.clone());
    let input = if trailing_clauses.had_any {
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
            PartUsage {
                prefix: head.prefix.clone(),
                then_span: head.then_span.clone(),
                name: name_str,
                short_name: head.short_name.clone(),
                typing,
                multiplicity: multiplicity_opt,
                ordered,
                nonunique,
                subsets,
                redefines,
                value,
                body,
                name_span: Some(name_span),
                type_ref_span,
                membership: head.membership.clone(),
            },
        ),
    ))
}

/// `PartUsage = OccurrenceUsagePrefix 'part' Usage` (SysML BNF 623).
///
/// One parser for every legal spelling and every scope of §3 in
/// `planning/part-usage-prefix-matrix.md`. The prefix is the shared component
/// [`occurrence_usage_prefix`](crate::parser::occurrence_prefix::occurrence_usage_prefix), not a
/// hand-rolled subset: this function previously accepted `RefPrefix` plus `ref` plus
/// `individual` and nothing else, so `snapshot part vehicle_1_t0 { ... }` (`training/28.
/// Individuals/Individuals and Roles-1.sysml:14`) reached recovery and `#logical part
/// vehicleLogical : Vehicle { ... }` (`Vehicle Example/SysML v2 Spec Annex A
/// SimpleVehicleModel.sysml:487`) became two sibling members.
///
/// Wrapped in a reference transaction because the prefix's `UsageExtensionKeyword*` allocates an
/// arena entry per `#tag` before the production is known to apply. A prefix followed by anything
/// other than `part` fails the whole production, so the member reaches recovery as one node
/// rather than being reinterpreted as an unprefixed usage.
pub(crate) fn part_usage(input: Input<'_>) -> IResult<Input<'_>, Node<PartUsage>> {
    crate::parser::span::reference_transaction(input, part_usage_inner)
}

fn part_usage_inner(input: Input<'_>) -> IResult<Input<'_>, Node<PartUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    // `DefinitionBodyItem`/`NonBehaviorBodyItem` = `( SourceSuccessionMember )? …UsageMember`, so
    // `then` precedes the membership, and `OccurrenceUsageMember = MemberPrefix …`, so the
    // visibility keyword precedes the usage's own prefix. All three in that order.
    let (input, then_span) =
        crate::parser::occurrence_prefix::optional_keyword_token(input, b"then")?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, prefix) = crate::parser::occurrence_prefix::occurrence_usage_prefix(input)?;
    let (input, _) = tag(&b"part"[..]).parse(input)?;
    // Allow `part: Type` with no whitespace (anonymous UsageDeclaration).
    let (after_kw, _) = ws_and_comments(input)?;
    // `part def …` is a definition. Especially important for `ref part def …`, which would
    // otherwise not be claimed by `part_def` (no leading `ref`) and misparse as a usage named
    // `def`.
    if starts_with_keyword(after_kw.fragment(), b"def") {
        return Err(nom::Err::Error(nom::error::Error::new(
            after_kw,
            nom::error::ErrorKind::Tag,
        )));
    }
    let input = if (after_kw.fragment().starts_with(b":")
        && !after_kw.fragment().starts_with(b":>")
        && !after_kw.fragment().starts_with(b":>>"))
        || starts_with_keyword(after_kw.fragment(), b"defined")
    {
        after_kw
    } else {
        let (input, _) = ws1(input)?;
        input
    };
    // `Identification`'s `( '<' ShortName '>' )?` half (BNF §8.2.2.2) -- see
    // `attribute::attribute_usage`'s identical short-name handling for the confirmed real-usage
    // citation. Parsed once here at the dispatch level rather than inside each of the three
    // declaration tails below, which is why it travels in [`PartUsageHead`].
    let (input, short_name) = short_name_prefix(input)?;
    // Consume (not just peek) whitespace/comments after the short name's closing `>` -- see
    // `attribute::attribute_usage`'s identical fix for why this can't reuse `ws1`'s earlier
    // consumption (a short name leaves fresh un-consumed whitespace after it).
    let (input, _) = ws_and_comments(input)?;
    let head = PartUsageHead {
        prefix,
        then_span,
        short_name,
        membership: Membership::feature(visibility, visibility_span),
    };
    let peek = input;
    if (peek.fragment().starts_with(b":")
        && !peek.fragment().starts_with(b":>")
        && !peek.fragment().starts_with(b":>>"))
        || starts_with_keyword(peek.fragment(), b"defined")
    {
        return anonymous_part_usage(start, input, &head);
    }
    if let Ok((input, usage)) = part_usage_redefines_only(start, input, &head) {
        return Ok((input, usage));
    }
    part_usage_named(start, input, &head)
}

fn anonymous_part_usage<'a>(
    start: Input<'a>,
    input: Input<'a>,
    head: &PartUsageHead,
) -> IResult<Input<'a>, Node<PartUsage>> {
    let (input, multiplicity_before) = opt(multiplicity_node).parse(input)?;
    let (input, (ordered_before_type, nonunique_before_type)) = usage_ordered_modifier(input)?;
    let (input, (type_ref_span, is_conjugated, targets, spelling)) = typings(input)?;
    let typing = Some(typing_node(
        type_ref_span.clone(),
        is_conjugated,
        targets,
        spelling,
    ));
    let (input, multiplicity_after) = opt(multiplicity_node).parse(input)?;
    let multiplicity_opt = multiplicity_before.or(multiplicity_after);
    let (input, (ordered_after_type, nonunique_after_type)) = usage_ordered_modifier(input)?;
    let ordered = ordered_before_type || ordered_after_type;
    let nonunique = nonunique_before_type || nonunique_after_type;
    let (input, clauses) = specialization_clauses(input)?;
    let (input, post_clause_multiplicity) = opt(multiplicity_node).parse(input)?;
    let multiplicity_opt = multiplicity_opt.or(post_clause_multiplicity);
    let (input, (ordered_after_clauses, nonunique_after_clauses)) = usage_ordered_modifier(input)?;
    let ordered = ordered || ordered_after_clauses;
    let nonunique = nonunique || nonunique_after_clauses;
    let (input, value) = opt(preceded(ws_and_comments, usage_value_part)).parse(input)?;
    let (input, body) = part_usage_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            PartUsage {
                prefix: head.prefix.clone(),
                then_span: head.then_span.clone(),
                name: String::new(),
                short_name: head.short_name.clone(),
                typing,
                multiplicity: multiplicity_opt,
                ordered,
                nonunique,
                subsets: clauses.subsets,
                redefines: clauses.redefines,
                value,
                body,
                name_span: None,
                type_ref_span: Some(type_ref_span),
                membership: head.membership.clone(),
            },
        ),
    ))
}

/// Part usage body: ';' or '{' PartUsageBodyElement* '}'
pub(crate) fn part_usage_body(input: Input<'_>) -> IResult<Input<'_>, PartUsageBody> {
    let (input, _) = ws_and_comments(input)?;
    let frag = input.fragment();
    log::debug!(
        "part_usage_body: first 40 bytes: {:?}",
        frag.get(..40.min(frag.len())).unwrap_or(frag),
    );
    let result = alt((crate::parser::body::semicolon_body, part_usage_body_brace)).parse(input);
    if result.is_err() {
        log::debug!(
            "part_usage_body: failed at: {:?}",
            String::from_utf8_lossy(frag.get(..60.min(frag.len())).unwrap_or(frag)),
        );
    }
    result
}

fn part_usage_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<PartUsageBodyElement> {
    // Always emit Error for unrecognized tokens (including non-starters). Hard-failing non-starters
    // previously aborted the usage body so the package path swallowed the whole decl as
    // ExtendedLibraryDecl with no diagnostic (GH-12).
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        PART_BODY_STARTERS,
        "part usage body",
        "recovered_part_usage_body_element",
    );
    node_from_to(
        start,
        end,
        PartUsageBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

fn part_usage_body_brace(input: Input<'_>) -> IResult<Input<'_>, PartUsageBody> {
    let (input, members) = parse_structured_brace_members_with_skip(
        input,
        PART_BODY_STARTERS,
        "part usage body",
        "recovered_part_usage_body_element",
        part_usage_body_element,
        part_usage_body_recovery,
        BraceMemberSkip::BodyElementRecover,
    )?;
    log::debug!(
        "part_usage_body: brace ok, {} elements",
        members.elements.len()
    );
    Ok((input, members.into_body()))
}

/// The one `ref` usage body parser.
///
/// `ReferenceUsage` completes with a `UsageBody`, and `UsageBody = DefinitionBody`
/// (SysML 8.2.2.6.2, 8.2.2.5.2), so a `ref` body holds the general usage-member set regardless of
/// which declaration owns the `ref`. Connection, interface, part, action, and state owners all
/// call this rather than parsing the same body through their own member grammar and recording
/// which parser ran in the AST.
pub(crate) fn ref_body(input: Input<'_>) -> IResult<Input<'_>, RefBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let semicolon_start = input;
        let (input, _) = tag(&b";"[..]).parse(semicolon_start)?;
        return Ok((
            input,
            RefBody::Semicolon {
                semicolon_span: crate::parser::span::span_from_to(semicolon_start, input),
            },
        ));
    }
    // Same member grammar as any other usage body, reported under this scope's own name so a
    // diagnostic still tells the author which body they were writing.
    let (input, members) = parse_structured_brace_members_with_skip(
        input,
        PART_BODY_STARTERS,
        "ref usage body",
        "recovered_ref_body_element",
        part_usage_body_element,
        |start, end| {
            let recovery = build_recovery_error_node_from_span(
                start,
                end,
                PART_BODY_STARTERS,
                "ref usage body",
                "recovered_ref_body_element",
            );
            node_from_to(
                start,
                end,
                PartUsageBodyElement::Error(node_from_to(start, end, recovery)),
            )
        },
        BraceMemberSkip::BodyElementRecover,
    )?;
    Ok((input, members.into_body()))
}

fn consume_part_usage_structured_brace(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::parser::body::ParsedBraceMembers<PartUsageBodyElement>> {
    parse_structured_brace_members_with_skip(
        input,
        PART_BODY_STARTERS,
        "part usage body",
        "recovered_part_usage_body_element",
        part_usage_body_element,
        part_usage_body_recovery,
        BraceMemberSkip::BodyElementRecover,
    )
}

/// Arena-backed action path for `perform`, preserving both qualification and dotted segments.
fn perform_action_path(input: Input<'_>) -> IResult<Input<'_>, crate::ast::QualifiedReferenceId> {
    crate::parser::lex::reference_path(input)
}

/// In/out binding inside a perform body: `in` target `=` expr `;` or `out` target `=` expr `;`.
fn perform_in_out_binding(input: Input<'_>) -> IResult<Input<'_>, Node<PerformInOutBinding>> {
    crate::parser::span::reference_transaction(input, perform_in_out_binding_inner)
}

fn perform_in_out_binding_inner(input: Input<'_>) -> IResult<Input<'_>, Node<PerformInOutBinding>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, direction) = alt((
        value(InOut::In, tag(&b"in"[..])),
        value(InOut::Out, tag(&b"out"[..])),
    ))
    .parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, target) = reference_path(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"="[..])).parse(input)?;
    let (input, value_expr) = preceded(ws_and_comments, path_expression).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            PerformInOutBinding {
                direction,
                target,
                value: value_expr,
            },
        ),
    ))
}

/// Perform body element: doc comment, in/out binding, or `variant` member.
fn perform_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PerformBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = alt((
        map(
            crate::parser::body::annotating_member,
            PerformBodyElement::Annotating,
        ),
        map(perform_in_out_binding, PerformBodyElement::InOut),
        // §6 G6: parameter-direction usage members (`in part :>> name = value;`, `in item 'n' :
        // Type { }`, …) reuse the same directed/usage parsers as port-def bodies rather than
        // duplicating the grammar here. Placed before `variant`/`action` so simple `in name =`
        // bindings still win via `perform_in_out_binding` above.
        map(part_usage, |p| PerformBodyElement::PartUsage(Box::new(p))),
        map(item_usage, |i| PerformBodyElement::ItemUsage(Box::new(i))),
        map(directed_attribute_usage, |a| {
            PerformBodyElement::AttributeUsage(Box::new(a))
        }),
        map(variant_usage, PerformBodyElement::Variant),
        map(crate::parser::action::action_usage_body_element, |a| {
            PerformBodyElement::Action(Box::new(a))
        }),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

/// Perform body: `{` PerformBodyElement* `}`.
fn perform_body(input: Input<'_>) -> IResult<Input<'_>, PerformBody> {
    let (input, _) = ws_and_comments(input)?;
    let open_start = input;
    let (input, _) = tag(&b"{"[..]).parse(open_start)?;
    let open_span = crate::parser::span::span_from_to(open_start, input);
    let (input, elements) = preceded(
        ws_and_comments,
        many0(preceded(ws_and_comments, perform_body_element)),
    )
    .parse(input)?;
    let (close_start, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"}"[..]).parse(close_start)?;
    Ok((
        input,
        PerformBody::Brace {
            open_span,
            elements,
            close_span: crate::parser::span::span_from_to(close_start, input),
        },
    ))
}

/// Optional `abstract` / `variation` prefix before `perform` (§6 G5). Mirrors `part_usage`'s
/// handling of the same BNF `BasicUsagePrefix` slot.
fn perform_usage_prefix(input: Input<'_>) -> IResult<Input<'_>, Option<DefinitionPrefix>> {
    opt(alt((
        map(preceded(tag(&b"abstract"[..]), ws1), |_| {
            DefinitionPrefix::Abstract
        }),
        map(preceded(tag(&b"variation"[..]), ws1), |_| {
            DefinitionPrefix::Variation
        }),
    )))
    .parse(input)
}

/// Trailing `= expr` binding on a perform member, e.g. `perform action :>> doXorY = doX;`.
fn perform_value(input: Input<'_>) -> IResult<Input<'_>, Option<Node<crate::ast::FeatureValue>>> {
    opt(preceded(ws_and_comments, usage_value_part)).parse(input)
}

fn perform_body_or_semicolon(input: Input<'_>) -> IResult<Input<'_>, PerformBody> {
    preceded(
        ws_and_comments,
        alt((crate::parser::body::semicolon_body, perform_body)),
    )
    .parse(input)
}

/// Perform usage: (`abstract`|`variation`)? `perform` action_path (`:>>` target)? (`=` value)?
/// (`;` or `{ }` body).
pub(crate) fn perform_usage(input: Input<'_>) -> IResult<Input<'_>, Node<Perform>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, usage_prefix) = perform_usage_prefix(input)?;
    let (input, _) = tag(&b"perform"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, action_reference) = perform_action_path(input)?;
    let (input, redefines) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>>"[..])),
        preceded(ws_and_comments, with_span(qualified_reference)),
    ))
    .parse(input)?;
    let redefines = redefines.map(|(span, target)| {
        single_target_subsetting(span, crate::ast::SubsettingKind::Redefines, target)
    });
    let (input, value) = perform_value(input)?;
    let (input, body) = perform_body_or_semicolon(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Perform {
                usage_prefix,
                action_name: String::new(),
                action_reference: Some(action_reference),
                typing: None,
                multiplicity: None,
                redefines,
                subsets: None,
                value,
                body,
            },
        ),
    ))
}

/// Perform action declaration: (`abstract`|`variation`)? `perform action` name? (`:>>` target)?
/// (`:` type_name)? (`=` value)? (`;` or body).
///
/// §6 G20: the name is optional -- `perform action { ... }` and `perform action :>> doXorY = doX;`
/// are both real OMG spec Annex usage (`3c-Function-based Behavior-structure mod-2.sysml`,
/// `7a1-Variant Configuration - General Concept-a.sysml`) that previously fell through to opaque
/// recovery because `name(input)` was mandatory here.
pub(crate) fn perform_action_decl(input: Input<'_>) -> IResult<Input<'_>, Node<Perform>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, usage_prefix) = perform_usage_prefix(input)?;
    let (input, _) = tag(&b"perform"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(&b"action"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, action_name) = if is_anonymous_perform_action(input) {
        (input, String::new())
    } else {
        name(input)?
    };
    // GH-89: multiplicity after the name, e.g. `takePicture[*] :> PictureTaking::takePicture;`
    // (Camera Example/Camera.sysml:4).
    let (input, multiplicity) = opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    let (input, redefines) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>>"[..])),
        preceded(ws_and_comments, with_span(qualified_reference)),
    ))
    .parse(input)?;
    let redefines = redefines.map(|(span, target)| {
        single_target_subsetting(span, crate::ast::SubsettingKind::Redefines, target)
    });
    // GH-89: `:>` subsets clause, tried only when `:>>` redefines didn't match -- the two are
    // mutually exclusive specialization keywords at this position.
    let (input, subsets) = if redefines.is_none() {
        opt(preceded(
            preceded(ws_and_comments, tag(&b":>"[..])),
            preceded(ws_and_comments, with_span(qualified_reference)),
        ))
        .parse(input)?
    } else {
        (input, None)
    };
    let subsets = subsets.map(|(span, target)| {
        single_target_subsetting(span, crate::ast::SubsettingKind::Subsets, target)
    });
    let (input, type_reference) = opt(preceded(
        preceded(ws_and_comments, typing_colon),
        preceded(ws_and_comments, with_span(qualified_reference)),
    ))
    .parse(input)?;
    let typing = type_reference
        .map(|(span, target)| crate::parser::usage::single_target_typing(span, target));
    let (input, value) = perform_value(input)?;
    let (input, body) = perform_body_or_semicolon(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Perform {
                usage_prefix,
                action_name,
                action_reference: None,
                typing,
                multiplicity,
                redefines,
                subsets,
                value,
                body,
            },
        ),
    ))
}

/// True when a `perform action` declaration has no name of its own -- the next token already
/// begins the redefinition clause, the typing clause, the body, or the terminator.
fn is_anonymous_perform_action(input: Input<'_>) -> bool {
    let Ok((peek, _)) = ws_and_comments(input) else {
        return false;
    };
    let frag = peek.fragment();
    frag.starts_with(b"{")
        || frag.starts_with(b";")
        || frag.starts_with(b":>>")
        || frag.starts_with(b"=")
        || (frag.starts_with(b":") && !frag.starts_with(b":>"))
}

/// `:` that is not the start of `:>` / `:>>`.
fn typing_colon(input: Input<'_>) -> IResult<Input<'_>, Input<'_>> {
    if input.fragment().starts_with(b":>") {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    tag(&b":"[..]).parse(input)
}

/// Allocate: `allocate` source `to` target body.
pub(crate) fn allocate_(input: Input<'_>) -> IResult<Input<'_>, Node<Allocate>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"allocate"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, source) = path_expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
    let (input, target) = preceded(ws_and_comments, path_expression).parse(input)?;
    let (input, body) = ref_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Allocate {
                source,
                target,
                body,
            },
        ),
    ))
}

/// `(name)? (: Type)? (multiplicity)?` for the optional `binding` prefix (BNF
/// `BindingConnectorAsUsage = UsagePrefix ('binding' UsageDeclaration)? 'bind' ...`,
/// §8.2.2.13.2). Mirrors `action::succession_prefix`'s exact structure for the sibling
/// `SuccessionAsUsage = UsagePrefix ('succession' UsageDeclaration)? 'first' ...` production --
/// same anonymous-prefix shape, confirmed by matching real usage (`binding [1] bind ...` in
/// Systems Library `Domain Libraries/Geometry/ShapeItems.sysml` mirrors `succession [seBeforeNum]
/// first ...` in `Flows.sysml`).
type BindingPrefix = (
    Option<String>,
    Option<crate::ast::QualifiedReferenceId>,
    Option<Node<crate::ast::Multiplicity>>,
);

fn binding_prefix(input: Input<'_>) -> IResult<Input<'_>, BindingPrefix> {
    let (input, _) = tag(&b"binding"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (peek, _) = ws_and_comments(input)?;
    let frag = peek.fragment();
    let (input, binding_name) = if starts_with_keyword(frag, b"bind") || frag.starts_with(b"[") {
        (input, None)
    } else {
        let (input, parsed_name) = preceded(ws_and_comments, name).parse(input)?;
        (input, Some(parsed_name))
    };
    let (peek, _) = ws_and_comments(input)?;
    let (input, binding_type) =
        if peek.fragment().starts_with(b":") && !peek.fragment().starts_with(b":>") {
            let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
            let (input, type_name) = preceded(ws_and_comments, qualified_reference).parse(input)?;
            (input, Some(type_name))
        } else {
            (input, None)
        };
    let (input, binding_multiplicity) =
        opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    Ok((input, (binding_name, binding_type, binding_multiplicity)))
}

/// Bind: (`binding` name? (`: Type`)? multiplicity?)? `bind` multiplicity? path `=` multiplicity?
/// path (`;` or `{ }`). The per-endpoint multiplicity mirrors `connect_`'s `from_multiplicity`/
/// `to_multiplicity` (§6 G24) -- confirmed by real usage: `binding [1] bind [0..*] base.edges =
/// [0..*] be;` in Systems Library `Domain Libraries/Geometry/ShapeItems.sysml` (13 occurrences).
pub(crate) fn bind_(input: Input<'_>) -> IResult<Input<'_>, Node<Bind>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, prefix) = opt(binding_prefix).parse(input)?;
    let (binding_name, binding_type, binding_multiplicity) = prefix.unwrap_or((None, None, None));
    let (input, _) = preceded(ws_and_comments, tag(&b"bind"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, left_multiplicity) = opt(multiplicity_node).parse(input)?;
    let (input, left) = preceded(ws_and_comments, path_expression).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"="[..])).parse(input)?;
    let (input, right_multiplicity) =
        opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    let (input, right) = preceded(ws_and_comments, path_expression).parse(input)?;
    let mut body_parser = alt((
        crate::parser::body::semicolon_body,
        map(consume_part_usage_structured_brace, |members| {
            members.into_body()
        }),
    ));
    let (input, body) = body_parser.parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Bind {
                binding_name,
                binding_type,
                binding_multiplicity,
                left,
                left_multiplicity,
                right,
                right_multiplicity,
                body,
            },
        ),
    ))
}

/// Connect (part usage level): `connect` path `to` path body
pub(crate) fn connect_(input: Input<'_>) -> IResult<Input<'_>, Node<Connect>> {
    crate::parser::span::reference_transaction(input, connect_inner)
}

fn connect_inner(input: Input<'_>) -> IResult<Input<'_>, Node<Connect>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"connect"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    // §6 G24: each endpoint may carry its own multiplicity -- `connect [0..1] a.p1 to [1] b.p2;`.
    let (input, from_multiplicity) =
        opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    let (input, from_expr) = path_expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
    let (input, to_multiplicity) =
        opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    let (input, to_expr) = preceded(ws_and_comments, path_expression).parse(input)?;
    let (input, body) = ref_body(input)?;
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
            Connect {
                from: connection_end_with_multiplicity(from_multiplicity, from_expr),
                to: connection_end_with_multiplicity(to_multiplicity, to_expr),
                body,
                subsets,
                redefines,
            },
        ),
    ))
}

/// Wrap a parsed endpoint expression and its optional §6 G24 multiplicity in a `ConnectionEnd`
/// node, reusing the expression's own span (see `ast::core::ConnectionEnd`'s doc comment).
fn connection_end_with_multiplicity(
    multiplicity: Option<Node<crate::ast::Multiplicity>>,
    expr: Node<Expression>,
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

/// Interface usage body elements: `ref` `:>>` name `=` value body (RefRedef), `end` member
/// (GH-85), or `doc`.
fn interface_usage_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<InterfaceUsageBodyElement>> {
    alt((
        interface_usage_ref_redef,
        map(interface_usage_end_decl, |end| {
            let span = end.span.clone();
            Node::new(span, InterfaceUsageBodyElement::EndDecl(Box::new(end)))
        }),
        |input| {
            let start = input;
            let (input, member) = crate::parser::body::annotating_member(input)?;
            Ok((
                input,
                crate::parser::node_from_to(
                    start,
                    input,
                    InterfaceUsageBodyElement::Annotating(member),
                ),
            ))
        },
    ))
    .parse(input)
}

// GH-85: interfaces don't allow the `#name` derived-end-name form (same as
// `interface_def_body_element`'s `end_decl` call -- see `connector::end_decl`'s doc comment).
fn interface_usage_end_decl(input: Input<'_>) -> IResult<Input<'_>, Node<crate::ast::EndDecl>> {
    crate::parser::connector::end_decl(input, false)
}

fn interface_usage_ref_redef(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<InterfaceUsageBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"ref"[..]).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b":>>"[..])).parse(input)?;
    let (input, target) = preceded(ws_and_comments, qualified_reference).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"="[..])).parse(input)?;
    let (input, value) = preceded(ws_and_comments, expression).parse(input)?;
    let (input, body) = ref_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            InterfaceUsageBodyElement::RefRedef {
                target,
                value,
                body,
            },
        ),
    ))
}

/// Connect body for interface usage (TypedConnect): `;` or `{` body_elements* `}`
/// `InterfaceUsage = OccurrenceUsagePrefix 'interface' InterfaceUsageDeclaration InterfaceBody`.
///
/// Returns one shared [`crate::ast::Body`] carrying its own delimiter spans. The two callers used
/// to receive a `ConnectBody` marker beside the element list, so the `;`/`{}` fact lived in two
/// fields and neither `{` nor `}` had a span.
fn interface_usage_body(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::ast::Body<InterfaceUsageBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        return crate::parser::body::semicolon_body(input);
    }
    let (open_start, _) = ws_and_comments(input)?;
    let (mut input, _) = tag(&b"{"[..]).parse(open_start)?;
    let open_span = crate::parser::span::span_from_to(open_start, input);
    let mut elements = Vec::new();
    loop {
        let (next, _) = ws_and_comments(input)?;
        input = next;
        if input.fragment().starts_with(b"}") {
            let close_start = input;
            let (input, _) = tag(&b"}"[..]).parse(close_start)?;
            return Ok((
                input,
                crate::ast::Body::Brace {
                    open_span,
                    elements,
                    close_span: crate::parser::span::span_from_to(close_start, input),
                },
            ));
        }
        let (next, element) = interface_usage_body_element(input)?;
        if next.location_offset() == input.location_offset() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Many0,
            )));
        }
        elements.push(element);
        input = next;
    }
}

/// Connector end reference used in interface/connect syntax.
/// Accepts an optional leading cross multiplicity (`[1]`, discarded -- `InterfaceUsage::from`/
/// `to` don't model per-end multiplicity yet, same as the end name below; GH-16: real Systems
/// Library / Annex fixtures write `connect [1] a to [1] b;`, which failed to parse at all before
/// this), then either `path` or `endName ::> path`; the end name is currently ignored.
fn connector_end_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = opt(preceded(ws_and_comments, multiplicity_node)).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = opt((name, preceded(ws_and_comments, tag(&b"::>"[..])))).parse(input)?;
    preceded(ws_and_comments, path_expression).parse(input)
}

/// Interface usage: `interface` ( name (multiplicity)? `:` Type )? `connect` path `to` path body,
/// or `interface path `to` path body (only legal when no name/type precedes it), or -- GH-16 --
/// `interface` ( name (multiplicity)? )? ( `:` Type )? body with no inline `connect` clause at
/// all (BNF `InterfaceUsageDeclaration`'s `('connect' InterfacePart)?` is optional: ends may be
/// declared inside the body instead, or omitted entirely). The optional interface member name is
/// captured only for the declaration-only form; the connect forms still ignore it.
pub(crate) fn interface_usage(input: Input<'_>) -> IResult<Input<'_>, Node<InterfaceUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"interface"[..]).parse(input)?;
    let (input, _) = if input.fragment().starts_with(b":")
        || input.fragment().starts_with(b";")
        || input.fragment().starts_with(b"{")
    {
        (input, ())
    } else {
        ws1(input)?
    };
    let (input, named_interface) = opt((
        name,
        opt(multiplicity_node),
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, qualified_reference),
    ))
    .parse(input)?;
    let (input, iface_name, interface_type) =
        if let Some((iface_name, _, _, interface_type)) = named_interface {
            (input, Some(iface_name), Some(interface_type))
        } else {
            // GH-85: a bare name with no `: Type` at all, immediately followed by `connect`, e.g.
            // `interface userToFlashlight connect user.onOffCmdPort to
            // flashlight.onOffCmdPort { ... }` (OMG spec Annex `Flashlight Example/Flashlight
            // Example.sysml`). Only *peeks* the `connect` keyword (doesn't consume it) so the
            // `starts_with(b"connect")` dispatch below still finds it; guarded on the literal
            // keyword following so a genuinely anonymous `interface connect a to b;` can't have
            // `connect` itself misread as this name (that would require a second `connect`
            // immediately after it, which never occurs in real source).
            let (input, bare_named) = opt((
                name,
                opt(multiplicity_node),
                preceded(ws_and_comments, nom::combinator::peek(tag(&b"connect"[..]))),
            ))
            .parse(input)?;
            if let Some((iface_name, _, _)) = bare_named {
                (input, Some(iface_name), None)
            } else {
                // A declared interface usage with a name but no typing and no `connect` clause:
                // `interface i;`, `interface i { ... }`, `interface i :> J;`. `UsageDeclaration`
                // makes the `: Type` optional, so the name above was reachable only through the
                // typed or the `connect` spelling and this form fell through to the body parser
                // with the name still unconsumed -- which then failed on it, sending the whole
                // member to recovery.
                //
                // The lookahead is what keeps `interface a to b;` anonymous: `to` is not a
                // declaration terminator, so `a` there stays the first connector end.
                let (input, declared_name) = opt((
                    name,
                    opt(multiplicity_node),
                    preceded(
                        ws_and_comments,
                        nom::combinator::peek(alt((
                            tag(&b";"[..]),
                            tag(&b"{"[..]),
                            tag(&b":>>"[..]),
                            tag(&b":>"[..]),
                            tag(&b"subsets"[..]),
                            tag(&b"redefines"[..]),
                        ))),
                    ),
                ))
                .parse(input)?;
                if let Some((iface_name, _, _)) = declared_name {
                    (input, Some(iface_name), None)
                } else {
                    let (input, interface_type) = opt(preceded(
                        tag(&b":"[..]),
                        preceded(ws_and_comments, qualified_reference),
                    ))
                    .parse(input)?;
                    (input, None, interface_type)
                }
            }
        };
    let (input, spec) = crate::parser::usage::specialization_clauses(input)?;
    let subsets = spec.subsets.map(|(target, _)| target);
    let redefines = spec.redefines;
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b"connect") {
        let (input, _) = tag(&b"connect"[..]).parse(input)?;
        let (input, _) = ws1(input)?;
        let (input, from_expr) = connector_end_expression(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
        let (input, to_expr) = preceded(ws_and_comments, connector_end_expression).parse(input)?;
        let (input, body) = interface_usage_body(input)?;
        return Ok((
            input,
            node_from_to(
                start,
                input,
                InterfaceUsage::TypedConnect {
                    name: iface_name,
                    interface_type,
                    subsets,
                    redefines,
                    from: from_expr,
                    to: to_expr,
                    body,
                },
            ),
        ));
    }
    // BNF: the bare `InterfacePart` alternative (no `connect` keyword) is only legal when there
    // is no preceding `UsageDeclaration` at all -- i.e. no name and no type were captured above.
    if iface_name.is_none() && interface_type.is_none() {
        if let Ok((after_to, (from_expr, to_expr))) = (|| {
            let (input, from_expr) = connector_end_expression(input)?;
            let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
            let (input, to_expr) =
                preceded(ws_and_comments, connector_end_expression).parse(input)?;
            Ok::<_, nom::Err<nom::error::Error<Input<'_>>>>((input, (from_expr, to_expr)))
        })() {
            let (input, body) = interface_usage_body(after_to)?;
            return Ok((
                input,
                node_from_to(
                    start,
                    input,
                    InterfaceUsage::Connection {
                        subsets: subsets.clone(),
                        redefines: redefines.clone(),
                        from: from_expr,
                        to: to_expr,
                        body,
                    },
                ),
            ));
        }
    }
    // GH-16: no `connect` clause (and, if unnamed/untyped, no bare `from to to` form either) --
    // a plain declared interface usage. Ends, if any, are declared inside the body instead.
    let (input, body) = interface_usage_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            InterfaceUsage::Declaration {
                name: iface_name,
                interface_type,
                subsets,
                redefines,
                body,
            },
        ),
    ))
}

/// Bare reference usage: `(visibility)? ref` name (`:` type)? (`=` value)? body.
///
/// Kinded forms (`ref part` / `ref action` / …) are rejected so dedicated usage parsers own them
/// (`part_usage` with `is_reference`, `action_usage`, …). This path is BNF `ReferenceUsage`
/// (`'ref' Usage`), not `PartUsage`.
pub(crate) fn part_ref_usage(input: Input<'_>) -> IResult<Input<'_>, Node<RefDecl>> {
    let start = input;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    // `BasicUsagePrefix = RefPrefix ('ref')?` -- the direction (GH-88.4, e.g. `private in ref y:
    // A, B;`, Simple Tests/ItemTest.sysml:15) and the modifiers after it are all slots of
    // `RefPrefix`, parsed there in the order the production gives them.
    let (input, prefix) = crate::parser::usage::ref_prefix(input)?;
    let direction = prefix.direction;
    let (input, _) = tag(&b"ref"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    // Reject kinded refs so those forms parse as real PartUsage/ActionUsage/StateUsage/…
    // instead of a mis-named RefDecl. Includes `part` (GH-10): `ref part … :> …` belongs on
    // `part_usage` with `is_reference`, which already accepts `FeatureSpecializationPart`.
    if crate::parser::lex::starts_with_any_keyword(
        input.fragment(),
        &[
            b"part",
            b"action",
            b"state",
            b"port",
            b"connection",
            b"item",
            b"attribute",
            b"calc",
            b"flow",
            b"occurrence",
        ],
    ) {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, _) = opt(preceded(
        ws_and_comments,
        preceded(tag(&b":>>"[..]), ws_and_comments),
    ))
    .parse(input)?;
    let (input, name_str) = name(input)?;
    let (input, type_result) = crate::parser::usage::optional_typings(input)?;
    let (type_ref_span, _, typing) =
        crate::parser::usage::typing_reference_fields_from_result(type_result);
    // Trailing `:>>` redefinition after the typing, e.g. `ref self: Part :>> Item::self;`
    // (Systems Library `Parts.sysml`). The typing may equally follow the redefinition -- the
    // canonical emitted order and a legal `FeatureSpecializationPart` ordering -- so retry the
    // typing after the redefinition when it wasn't already written before it.
    let (input, redefines) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::redefinition,
    ))
    .parse(input)?;
    let (input, type_ref_span, typing) = if typing.is_none() && redefines.is_some() {
        let (input, type_result) = crate::parser::usage::optional_typings(input)?;
        let (type_ref_span, _, typing) =
            crate::parser::usage::typing_reference_fields_from_result(type_result);
        (input, type_ref_span, typing)
    } else {
        (input, type_ref_span, typing)
    };
    // `:>` subsets, independent of and in addition to `:>>` redefines (mirrors
    // `connector::ref_decl`).
    let (input, subsets) =
        opt(preceded(ws_and_comments, crate::parser::usage::subsetting)).parse(input)?;
    let subsets = subsets.map(|(target, _value)| target);
    let (input, value) = opt(preceded(
        preceded(ws_and_comments, tag(&b"="[..])),
        preceded(ws_and_comments, expression),
    ))
    .parse(input)?;
    let value = value.map(crate::parser::feature_value::wrap_bind_expression);
    let (input, body) = ref_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            RefDecl {
                short_name: None,
                is_derived: prefix.is_derived,
                usage_prefix: prefix.usage_prefix,
                is_constant: prefix.is_constant,
                direction,
                kind_keyword: None,
                name: name_str,
                typing,
                redefines,
                subsets,
                multiplicity: None,
                ordered: false,
                nonunique: false,
                value,
                body,
                name_span: None,
                type_ref_span,
                membership: crate::ast::Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

/// `variant` member: either a typed usage declared inline with a kind keyword
/// (`variant part name : Type { ... }`, `variant attribute name = expr;`, `variant item ...`,
/// `variant port ...`), or an untyped reference to a separately-declared usage
/// (`variant name;`).
pub(crate) fn variant_usage(input: Input<'_>) -> IResult<Input<'_>, Node<VariantUsage>> {
    crate::parser::span::reference_transaction(input, variant_usage_inner)
}

fn variant_usage_inner(input: Input<'_>) -> IResult<Input<'_>, Node<VariantUsage>> {
    let start = input;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let membership = Membership::variant(visibility, visibility_span);
    let (input, _) = tag(&b"variant"[..]).parse(input)?;
    let (input, _) = ws1(input)?;

    if let Ok((next, usage)) = part_usage(input) {
        return Ok((
            next,
            node_from_to(
                start,
                next,
                VariantUsage {
                    reference: None,
                    typed: Some(VariantTypedUsage::Part(Box::new(usage))),
                    body: None,
                    membership,
                },
            ),
        ));
    }
    if let Ok((next, usage)) = attribute_usage(input) {
        return Ok((
            next,
            node_from_to(
                start,
                next,
                VariantUsage {
                    reference: None,
                    typed: Some(VariantTypedUsage::Attribute(Box::new(usage))),
                    body: None,
                    membership,
                },
            ),
        ));
    }
    if let Ok((next, usage)) = item_usage(input) {
        return Ok((
            next,
            node_from_to(
                start,
                next,
                VariantUsage {
                    reference: None,
                    typed: Some(VariantTypedUsage::Item(Box::new(usage))),
                    body: None,
                    membership,
                },
            ),
        ));
    }
    if let Ok((next, usage)) = port_usage(input) {
        return Ok((
            next,
            node_from_to(
                start,
                next,
                VariantUsage {
                    reference: None,
                    typed: Some(VariantTypedUsage::Port(Box::new(usage))),
                    body: None,
                    membership,
                },
            ),
        ));
    }
    // `variant requirement r1;` inside a `variation requirement r { ... }` body (spec42
    // Gap 44), mirroring the five kind keywords above.
    if let Ok((next, usage)) = requirement_usage(input) {
        return Ok((
            next,
            node_from_to(
                start,
                next,
                VariantUsage {
                    reference: None,
                    typed: Some(VariantTypedUsage::Requirement(Box::new(usage))),
                    body: None,
                    membership,
                },
            ),
        ));
    }
    // §6 G5: `variant perform doX;` inside a `variation perform action ... { ... }` body.
    // `perform_action_decl` first, for the same bare-keyword reason as the dispatchers above.
    if let Ok((next, usage)) = alt((perform_action_decl, perform_usage)).parse(input) {
        return Ok((
            next,
            node_from_to(
                start,
                next,
                VariantUsage {
                    reference: None,
                    typed: Some(VariantTypedUsage::Perform(Box::new(usage))),
                    body: None,
                    membership,
                },
            ),
        ));
    }

    // Untyped reference form: `variant name;` or `variant name { ... }` / `variant 'quoted' {
    // ... }`, referencing an already-declared feature by name rather than declaring a fresh typed
    // usage. Real usage: `Simple Tests/VariabilityTest.sysml:16` (`variant q { attribute b : B
    // :>> a; }`, `q` referring to the sibling `part q : Q;`) and
    // `Variability Examples/VehicleVariabilityModel.sysml:78` (`variant '6cylEngine' { ... }`).
    let (input, reference) = reference_path(input)?;
    let (input, body) = preceded(ws_and_comments, part_usage_body).parse(input)?;
    let body = match body {
        PartUsageBody::Semicolon { .. } => None,
        brace @ PartUsageBody::Brace { .. } => Some(brace),
    };
    Ok((
        input,
        node_from_to(
            start,
            input,
            VariantUsage {
                reference: Some(reference),
                typed: None,
                body,
                membership,
            },
        ),
    ))
}

fn part_usage_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PartUsageBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    // In SysML part bodies, `metadata name ...` is the dedicated MetadataUsage production.
    // It shares its prefix with KerML MetadataFeature, so give the scope-specific production
    // first refusal before the shared AnnotatingElement parser.
    if crate::parser::lex::starts_with_keyword(start.fragment(), b"metadata") {
        if let Ok((next, usage)) = metadata_usage(start) {
            return Ok((
                next,
                node_from_to(start, next, PartUsageBodyElement::MetadataUsage(usage)),
            ));
        }
    }
    // A `#tag` run and a leading `ref` are both `OccurrenceUsagePrefix` slots that a sibling
    // production in this scope would otherwise claim first; see
    // `occurrence_prefix::starts_contended_prefix`.
    if crate::parser::occurrence_prefix::starts_contended_prefix(start) {
        if let Ok((next, usage)) = occurrence_usage(start) {
            let elem = PartUsageBodyElement::OccurrenceUsage(Box::new(usage));
            return Ok((next, node_from_to(start, next, elem)));
        }
        if let Ok((next, usage)) = satisfy(start) {
            let elem = PartUsageBodyElement::Satisfy(Box::new(usage));
            return Ok((next, node_from_to(start, next, elem)));
        }
        if let Ok((next, usage)) = item_usage(start) {
            let elem = PartUsageBodyElement::ItemUsage(usage);
            return Ok((next, node_from_to(start, next, elem)));
        }
        if let Ok((next, usage)) = part_usage(start) {
            let elem = PartUsageBodyElement::PartUsage(Box::new(usage));
            return Ok((next, node_from_to(start, next, elem)));
        }
    }
    let frag = start.fragment();
    let first_30 = frag.get(..30.min(frag.len())).unwrap_or(frag);
    log::debug!(
        "part_usage_body_element: first 30 bytes: {:?} (str: {:?})",
        first_30,
        String::from_utf8_lossy(first_30),
    );
    let (input, elem) = alt((
        alt((
            alt((
                map(
                    crate::parser::body::annotating_member,
                    PartUsageBodyElement::Annotating,
                ),
                // `in :>> target;` and friends. `in_out_decl` rejects the kinded forms
                // (`in item`, `in part`, `in occurrence`, ...) itself, so the arms below still
                // see them.
                map(
                    crate::parser::action::in_out_decl,
                    PartUsageBodyElement::InOutDecl,
                ),
                map(
                    |i| crate::parser::connector::end_decl(i, true),
                    PartUsageBodyElement::EndDecl,
                ),
                map(
                    crate::parser::metadata_annotation::metadata_keyword_usage,
                    PartUsageBodyElement::MetadataKeywordUsage,
                ),
                map(
                    crate::parser::metadata_annotation::metadata_keyword_prefix,
                    PartUsageBodyElement::MetadataKeywordUsage,
                ),
            )),
            map(
                exhibit_state_as_state_usage,
                PartUsageBodyElement::StateUsage,
            ),
            map(action_usage, |a| {
                PartUsageBodyElement::ActionUsage(Box::new(a))
            }),
            map(state_usage, PartUsageBodyElement::StateUsage),
            map(perform_action_decl, PartUsageBodyElement::Perform),
            map(perform_usage, PartUsageBodyElement::Perform),
            // GH-92.3: `succession ... first ... then ...;`, already modeled/parsed for
            // `ConnectionDefBodyElement`/`OccurrenceBodyElement`; just not dispatched here.
            map(
                crate::parser::occurrence_body::succession_usage,
                PartUsageBodyElement::SuccessionUsage,
            ),
            map(allocate_, PartUsageBodyElement::Allocate),
            map(variant_usage, PartUsageBodyElement::VariantUsage),
            map(attribute_usage, PartUsageBodyElement::AttributeUsage),
            // Keyword-less `:>> name …` redefinition (GH-12 fallout: previously swallowed via
            // ExtendedLibraryDecl). Same arm as occurrence bodies (§6 G15).
            map(
                redefinition_feature_binding,
                PartUsageBodyElement::AttributeUsage,
            ),
            map(
                attribute_usage_shorthand,
                PartUsageBodyElement::DefaultReferenceUsage,
            ),
            alt((
                map(enum_usage, PartUsageBodyElement::EnumerationUsage),
                map(part_usage, |p| PartUsageBodyElement::PartUsage(Box::new(p))),
            )),
            map(occurrence_usage, |n| {
                PartUsageBodyElement::OccurrenceUsage(Box::new(n))
            }),
        )),
        // PAR-002: nested `def` kinds -- usage bodies legally contain nested definitions per BNF
        // `UsageBody = DefinitionBody`. `port_def`/`calc_def_required`/
        // `connection_def_required` must be tried before `port_usage`/`connection_usage_member`
        // -- both usage-form parsers have no guard against a bare `def` keyword (same bug class
        // fixed for `PartDefBodyElement` in a prior increment), so `port def Foo;`/
        // `connection def Foo;` would otherwise misparse as a usage named "def".
        // `state_def`/`requirement_def`/`occurrence_def` are all `def_required()`-guarded
        // internally. `metadata_usage` is tried before `metadata_def` (same pairing as part def).
        alt((
            map(state_def, PartUsageBodyElement::StateDef),
            map(
                crate::parser::enumeration::enum_def,
                PartUsageBodyElement::EnumDef,
            ),
            // `metadata_usage` before `metadata_def` so bare `metadata Name { … }` does not misfire
            // (metadata_def is def_required-guarded, but keep def/usage pairing explicit).
            // Nested KerML classifier declarations (`struct Car1_ { ... }` inside a `part`
            // usage body, KerML `time_varying_car_driver`; spec42 Gap 38), keyword-gated so no
            // other member shape is affected.
            map(crate::parser::package::kerml_classifier_structured, |n| {
                PartUsageBodyElement::KermlClassifier(Box::new(n))
            }),
            map(metadata_usage, PartUsageBodyElement::MetadataUsage),
            map(metadata_def, PartUsageBodyElement::MetadataDef),
            map(requirement_def, PartUsageBodyElement::RequirementDef),
            // §6 G5: the usage form was reachable from part *definition* bodies only.
            map(requirement_usage, PartUsageBodyElement::RequirementUsage),
            map(occurrence_def, PartUsageBodyElement::OccurrenceDef),
            // `calc_def_required` before `calc_usage` for the same bare-`def` reason (GH-91.2).
            map(calc_def_required, PartUsageBodyElement::CalcDef),
            map(calc_usage, PartUsageBodyElement::CalcUsage),
            // `constraint_def` before `constraint_usage` for the same bare-`def` reason.
            map(constraint_def, PartUsageBodyElement::ConstraintDef),
            map(constraint_usage, PartUsageBodyElement::ConstraintUsage),
            // §6 G16: a part body is a namespace, so it owns imports too.
            map(crate::parser::import::import_, PartUsageBodyElement::Import),
            map(connection_def_required, PartUsageBodyElement::ConnectionDef),
            map(connection_usage_member, PartUsageBodyElement::Connection),
            map(port_def, PartUsageBodyElement::PortDef),
        )),
        alt((
            map(port_usage, |p| PartUsageBodyElement::PortUsage(Box::new(p))),
            map(part_ref_usage, PartUsageBodyElement::Ref),
            // Kinded `ref item :>> a, b, c;` (Domain Libraries `SpatialItems.sysml`), which
            // `part_ref_usage` deliberately rejects; `connector::ref_decl` owns that shape.
            map(
                crate::parser::connector::ref_decl,
                PartUsageBodyElement::Ref,
            ),
            map(bind_, PartUsageBodyElement::Bind),
            map(satisfy, |n| PartUsageBodyElement::Satisfy(Box::new(n))),
            map(
                crate::parser::occurrence_body::assert_constraint_member,
                PartUsageBodyElement::AssertConstraint,
            ),
            map(interface_usage, PartUsageBodyElement::InterfaceUsage),
            map(connect_, PartUsageBodyElement::Connect),
            // `flow_def` must be tried before `flow_usage_member`: the latter has no guard
            // against a bare `def` keyword either (see comment above).
            map(flow_def, PartUsageBodyElement::FlowDef),
            map(
                crate::parser::flow::flow_usage_member,
                PartUsageBodyElement::FlowUsage,
            ),
            // §6 G25: `item` members were reachable from part *definition* bodies only.
            // `item_def_required` first, for the same bare-`def` reason as `constraint_def`.
            map(item_def_required, PartUsageBodyElement::ItemDef),
            map(item_usage, PartUsageBodyElement::ItemUsage),
            // Analysis case members were reachable from part *definition* bodies only; GH-12
            // recovery surfaced them as diagnostics in OMG Annex `10c-Fuel Economy Analysis.sysml`.
            map(analysis_case_def, PartUsageBodyElement::AnalysisCaseDef),
            map(analysis_case_usage, PartUsageBodyElement::AnalysisCaseUsage),
            // GH-89: `alias <name> for <target>;` nested inside a part usage body, previously
            // only reachable at package-body scope even though `Simple Tests/AliasTest.sysml:16`
            // uses it directly inside a `part <name> : Type { ... }` usage.
            map(
                crate::parser::alias::alias_def,
                PartUsageBodyElement::AliasDef,
            ),
            // GH-89: `include <usecase>;` and `use case <name> : Type { ... }` nested inside a
            // part usage body, previously only reachable inside a use case definition body (or,
            // for `use case` usages, a part *definition* body) even though `Simple Tests/
            // UseCaseTest.sysml:33-35` uses both directly inside a `part <name> : Type { ... }`
            // usage.
            map(
                crate::parser::usecase::include_use_case,
                PartUsageBodyElement::IncludeUseCase,
            ),
            map(
                crate::parser::usecase::use_case_usage,
                PartUsageBodyElement::UseCaseUsage,
            ),
            // GH-89: `verification <name> : Type { ... }` usage nested inside a plain part usage
            // body, previously only reachable from part *definition* bodies (Simple Tests/
            // VerificationTest.sysml:35).
            map(
                crate::parser::case::verification_case_usage,
                PartUsageBodyElement::VerificationCaseUsage,
            ),
            // The view family. `UsageBody = DefinitionBody`, so a part *usage* body admits the
            // same six a part *definition* body already dispatched; this scope had none of them,
            // so `rendering r { ... }` or `view v { ... }` inside `part p { ... }` reached
            // recovery. `def` before `usage` throughout, for the bare-keyword reason the part
            // definition dispatcher documents: `view_usage`, `viewpoint_usage` and
            // `rendering_usage` each read a bare name straight after their keyword.
            // Nested in a sub-alt to stay under nom's 21-branch limit, as the sibling groups are.
            alt((
                map(crate::parser::view::view_def, PartUsageBodyElement::ViewDef),
                map(
                    crate::parser::view::view_usage,
                    PartUsageBodyElement::ViewUsage,
                ),
                map(
                    crate::parser::view::viewpoint_def,
                    PartUsageBodyElement::ViewpointDef,
                ),
                map(
                    crate::parser::view::viewpoint_usage,
                    PartUsageBodyElement::ViewpointUsage,
                ),
                map(
                    crate::parser::view::rendering_def,
                    PartUsageBodyElement::RenderingDef,
                ),
                map(
                    crate::parser::view::rendering_usage,
                    PartUsageBodyElement::RenderingUsage,
                ),
            )),
        )),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

pub(crate) fn exhibit_state_as_state_usage(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::StateUsage>> {
    let (input, exhibit) = exhibit_state(input)?;
    let state = crate::ast::StateUsage {
        direction: exhibit.value.direction,
        is_derived: exhibit.value.is_derived,
        is_abstract: exhibit.value.is_abstract,
        is_reference: exhibit.value.is_reference,
        is_individual: exhibit.value.is_individual,
        name: exhibit.value.name,
        state_reference: exhibit.value.state_reference,
        type_name: exhibit
            .value
            .typing
            .as_ref()
            .and_then(|typing| typing.value.target.first().copied()),
        typing: exhibit.value.typing,
        multiplicity: exhibit.value.multiplicity,
        subsets: exhibit.value.subsets,
        // §6 G18: previously dropped, which silently lost the redefinition target of
        // `exhibit <name> :>> <target>;`.
        redefines: exhibit.value.redefines,
        body: exhibit.value.body,
        // GH-27: `ExhibitState` now carries its own `membership` (visibility prefix), so thread
        // it through instead of the previous ad hoc `visibility: None`.
        membership: exhibit.value.membership,
    };
    Ok((input, Node::new(exhibit.span, state)))
}

#[cfg(test)]
mod par_002_nested_def_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    /// Spec42 Gap 38: KerML classifier-keyword declarations nested inside a part usage body
    /// dispatch to the typed `KermlClassifierDecl` production (`struct Car1_ { ... }` inside
    /// `part c { ... }`, KerML `time_varying_car_driver`).
    #[test]
    fn part_usage_body_dispatches_nested_kerml_classifiers() {
        let (rest, node) = part_usage_body_element(input("struct Car1_ { feature wheels; }"))
            .expect("nested struct");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let PartUsageBodyElement::KermlClassifier(decl) = node.value else {
            panic!("expected KermlClassifier");
        };
        assert_eq!(
            decl.value.keyword,
            crate::ast::KermlClassifierKeyword::Struct
        );
        assert_eq!(decl.value.identification.name.as_deref(), Some("Car1_"));
    }

    #[test]
    fn connect_trailing_relationships_retain_arena_targets() {
        let source = input("connect a to b; :> Network::links :>> Legacy::links;");
        let (rest, node) = connect_(source).expect("connect with trailing relationships");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let subsets = node.value.subsets.expect("subsets relationship");
        let redefines = node.value.redefines.expect("redefines relationship");
        assert_eq!(subsets.value.target.len(), 1);
        assert_eq!(redefines.value.target.len(), 1);
        assert_eq!(
            crate::parser::usage::reference_text(source, subsets.value.target[0]).as_deref(),
            Some("Network::links")
        );
        assert_eq!(
            crate::parser::usage::reference_text(source, redefines.value.target[0]).as_deref(),
            Some("Legacy::links")
        );
    }

    #[test]
    fn part_usage_body_accepts_nested_state_def() {
        let (rest, node) =
            part_usage_body_element(input("state def Modes { state on; state off; }"))
                .expect("state def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartUsageBodyElement::StateDef(_)));
    }

    #[test]
    fn part_usage_body_accepts_nested_enum_def() {
        let (rest, node) = part_usage_body_element(input("enum def MyEnum;")).expect("enum def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartUsageBodyElement::EnumDef(_)));
    }

    #[test]
    fn part_usage_body_accepts_nested_metadata_def() {
        let (rest, node) =
            part_usage_body_element(input("metadata def MyMeta;")).expect("metadata def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartUsageBodyElement::MetadataDef(_)));
    }

    #[test]
    fn part_usage_body_accepts_metadata_usage() {
        let (rest, node) = part_usage_body_element(input(
            "metadata Classified { classificationLevel = ClassificationLevel::conf; }",
        ))
        .expect("metadata usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartUsageBodyElement::MetadataUsage(_)));
    }

    #[test]
    fn part_usage_body_accepts_redefinition_feature_binding() {
        let text = ":>> mass : MassValue = sum((a.mass, b.mass));";
        let (rest, node) =
            part_usage_body_element(input(text)).expect("redefinition feature binding");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            PartUsageBodyElement::AttributeUsage(_)
        ));
    }

    #[test]
    fn part_usage_body_accepts_analysis_case_usage() {
        let (rest, node) = part_usage_body_element(input(
            "analysis cityFuelEconomyAnalysis : FuelEconomyAnalysis;",
        ))
        .expect("analysis case usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            PartUsageBodyElement::AnalysisCaseUsage(_)
        ));
    }

    #[test]
    fn part_usage_body_accepts_nested_flow_def_not_misparsed_as_usage() {
        let (rest, node) = part_usage_body_element(input("flow def DataFlow;")).expect("flow def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartUsageBodyElement::FlowDef(_)));
    }

    #[test]
    fn part_usage_body_accepts_nested_requirement_def() {
        let (rest, node) =
            part_usage_body_element(input("requirement def SafetyReq;")).expect("requirement def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            PartUsageBodyElement::RequirementDef(_)
        ));
    }

    #[test]
    fn part_usage_body_accepts_nested_occurrence_def() {
        let (rest, node) =
            part_usage_body_element(input("occurrence def Failure;")).expect("occurrence def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartUsageBodyElement::OccurrenceDef(_)));
    }

    #[test]
    fn part_usage_body_accepts_nested_port_def_not_misparsed_as_usage() {
        let (rest, node) = part_usage_body_element(input("port def MyPort;")).expect("port def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartUsageBodyElement::PortDef(_)));
    }

    #[test]
    fn part_usage_body_accepts_nested_port_usage() {
        let (rest, node) = part_usage_body_element(input("port p1: MyPort;")).expect("port usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartUsageBodyElement::PortUsage(_)));
    }

    #[test]
    fn part_usage_body_accepts_nested_calc_def() {
        let (rest, node) = part_usage_body_element(input("calc def MyCalc;")).expect("calc def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartUsageBodyElement::CalcDef(_)));
    }

    #[test]
    fn part_usage_body_accepts_nested_connection_def() {
        let (rest, node) =
            part_usage_body_element(input("connection def MyConn;")).expect("connection def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartUsageBodyElement::ConnectionDef(_)));
    }

    /// PARSER_BACKLOG_ROADMAP.md §6, G2: `connection <name> : Type[mult];` usage was wired for
    /// `PartDefBodyElement` but not `PartUsageBodyElement`.
    #[test]
    fn part_usage_body_accepts_connection_usage_member() {
        let (rest, node) =
            part_usage_body_element(input("connection trailerHitch : TrailerHitch[0..1];"))
                .expect("connection usage member");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartUsageBodyElement::Connection(_)));
    }

    /// `connection def` must still win when both are dispatched in the same body -- same
    /// ordering risk already documented for `port`/`flow`/`calc`.
    #[test]
    fn part_usage_body_connection_def_is_not_shadowed_by_connection_usage_member() {
        let (rest, node) =
            part_usage_body_element(input("connection def MyConn;")).expect("connection def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartUsageBodyElement::ConnectionDef(_)));
    }

    /// PARSER_BACKLOG_ROADMAP.md §6, G3: `assert constraint { }` was wired for
    /// `PartDefBodyElement` but not `PartUsageBodyElement`.
    #[test]
    fn part_usage_body_accepts_assert_constraint() {
        let (rest, node) =
            part_usage_body_element(input("assert constraint c { }")).expect("assert constraint");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            PartUsageBodyElement::AssertConstraint(_)
        ));
    }

    /// PARSER_BACKLOG_ROADMAP.md §6, G4: `constraint` usage/definition were wired at package
    /// level only. Part *usage* bodies legally contain the same members as part definition
    /// bodies (BNF `UsageBody = DefinitionBody`), so both are wired here too.
    #[test]
    fn part_usage_body_accepts_constraint_usage() {
        let (rest, node) = part_usage_body_element(input("constraint c : DiscBrakeConstraint { }"))
            .expect("constraint usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            PartUsageBodyElement::ConstraintUsage(_)
        ));
    }

    #[test]
    fn part_usage_body_accepts_nested_constraint_def_not_misparsed_as_usage() {
        let (rest, node) =
            part_usage_body_element(input("constraint def MyConstraint;")).expect("constraint def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PartUsageBodyElement::ConstraintDef(_)));
    }

    /// PAR-002 acceptance criterion, increment 4: the same `state def` declaration yields the
    /// same AST variant kind nested in a part *usage* body as it already does nested in a part
    /// *definition* body (proven in a prior increment) and at package level.
    #[test]
    fn state_def_is_same_variant_kind_in_part_usage_body_as_in_part_def_body() {
        let text = "state def Modes { state on; state off; }";
        let (_, usage_node) =
            part_usage_body_element(input(text)).expect("nested in part usage body");
        assert!(matches!(
            usage_node.value,
            PartUsageBodyElement::StateDef(_)
        ));
        let (_, def_node) =
            crate::parser::part::part_def_or_usage(input(&format!("part def X {{ {text} }}")))
                .expect("part def parses");
        let crate::parser::part::PartDefOrUsage::Def(def_node) = def_node else {
            panic!("expected a part def");
        };
        let crate::ast::PartDefBody::Brace { elements, .. } = &def_node.value.body else {
            panic!("expected brace body");
        };
        assert_eq!(elements.len(), 1);
        assert!(matches!(
            elements[0].value,
            crate::ast::PartDefBodyElement::StateDef(_)
        ));
    }
}

/// PARSER_BACKLOG_ROADMAP.md §6, G1: `perform <path>` (no `action` keyword) only accepted a
/// brace body and had no `:>>` redefinition clause, so real Systems Library usage like
/// `perform 'provide power';` or `perform a.b :>> c.d { }` fell through to opaque recovery.
/// Confirmed against real usage in the OMG spec Annex examples (`08-Requirements.sysml`,
/// `12b-Allocation.sysml`, `12b-Allocation-1.sysml`, `05-State-based Behavior-2.sysml`).
#[cfg(test)]
mod perform_semicolon_and_redefine_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    fn perform(text: &str) -> Node<Perform> {
        let (rest, node) = part_usage_body_element(input(text)).expect("perform parses");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let PartUsageBodyElement::Perform(perform) = node.value else {
            panic!("expected Perform, got {:?}", node.value);
        };
        perform
    }

    #[test]
    fn perform_plain_name_accepts_semicolon_body() {
        let node = perform("perform vehicleMassTest;");
        assert!(node.value.action_name.is_empty());
        assert!(node.value.action_reference.is_some());
        assert_eq!(node.value.redefines, None);
        assert!(matches!(node.value.body, PerformBody::Semicolon { .. }));
    }

    #[test]
    fn perform_dotted_name_accepts_semicolon_body() {
        let node = perform("perform providePower.generateTorque;");
        assert!(node.value.action_name.is_empty());
        assert!(node.value.action_reference.is_some());
        assert!(matches!(node.value.body, PerformBody::Semicolon { .. }));
    }

    #[test]
    fn perform_quoted_name_accepts_semicolon_body() {
        let node = perform("perform 'provide power';");
        assert!(node.value.action_name.is_empty());
        assert!(node.value.action_reference.is_some());
        assert!(matches!(node.value.body, PerformBody::Semicolon { .. }));
    }

    #[test]
    fn perform_accepts_redefine_clause_with_semicolon_body() {
        let node = perform("perform providePower.generateTorque :>> generateTorque;");
        assert!(node.value.action_reference.is_some());
        assert_eq!(
            node.value
                .redefines
                .as_ref()
                .map(|relationship| relationship.value.target.len()),
            Some(1)
        );
        assert!(matches!(node.value.body, PerformBody::Semicolon { .. }));
    }

    #[test]
    fn perform_accepts_redefine_clause_with_brace_body() {
        let node = perform("perform 'provide power' :>> VehicleA::'provide power' { }");
        assert_eq!(
            node.value
                .redefines
                .as_ref()
                .map(|relationship| relationship.value.target.len()),
            Some(1)
        );
        assert!(matches!(node.value.body, PerformBody::Brace { .. }));
    }

    #[test]
    fn perform_plain_brace_body_still_works() {
        let node = perform("perform vehicleMassTest { }");
        assert!(matches!(node.value.body, PerformBody::Brace { .. }));
    }

    #[test]
    fn perform_action_declaration_form_is_unaffected() {
        let (rest, node) = part_usage_body_element(input("perform action 'assemble vehicle' { }"))
            .expect("perform action parses");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let PartUsageBodyElement::Perform(perform) = node.value else {
            panic!("expected Perform, got {:?}", node.value);
        };
        assert_eq!(perform.value.action_name, "assemble vehicle");
        assert!(perform.value.action_reference.is_none());
        assert_eq!(perform.value.redefines, None);
    }

    /// §6 G6: directed `part`/`item` usages inside a perform body.
    #[test]
    fn perform_body_accepts_directed_part_and_item_usages() {
        let node = perform("perform vehicleMassTest { in part :>> testVehicle = vehicleUnderTest; in item 'mass sample' : MassSample { } }");
        let PerformBody::Brace { elements, .. } = node.value.body else {
            panic!("expected brace body");
        };
        assert!(matches!(
            elements[0].value,
            PerformBodyElement::PartUsage(_)
        ));
        assert!(matches!(
            elements[1].value,
            PerformBodyElement::ItemUsage(_)
        ));
    }
}

#[cfg(test)]
mod variant_membership_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    // --- parser work item 4b (final sweep): VariantMembership on VariantUsage, confirmed
    // against the BNF's `VariantUsageMember : VariantMembership = MemberPrefix 'variant'
    // ownedVariantUsage = VariantUsageElement`.

    #[test]
    fn variant_usage_visibility_prefix_is_captured_on_membership_untyped_form() {
        let (rest, node) = variant_usage(input("private variant v1;")).expect("variant usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Private)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::VariantMembership
        );
    }

    #[test]
    fn variant_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = variant_usage(input("variant v1;")).expect("variant usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn variant_usage_visibility_prefix_is_captured_on_membership_typed_form() {
        let (_, node) =
            variant_usage(input("protected variant part p1 : P1;")).expect("variant usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Protected)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::VariantMembership
        );
    }
}

// --- short-name (`<shortName>`) support on `part_usage`, mirroring `attribute_usage`'s identical
// gap (shared `Identification` BNF production, §8.2.2.2) -- see
// `attribute.rs::attribute_body_tests`'s citation of the confirmed real-usage gap in the OMG
// Geometry domain library's `VehicleGeometryAndCoordinateFrames.sysml`.
#[cfg(test)]
mod short_name_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    #[test]
    fn part_usage_captures_short_name() {
        let (rest, node) = part_usage(input("part <eng> engine : Engine;")).expect("part usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.short_name.as_deref(), Some("eng"));
        assert_eq!(node.value.name, "engine");
        assert_eq!(
            node.value
                .typing
                .as_ref()
                .map(|typing| typing.value.target.len()),
            Some(1)
        );
    }

    // Mirrors the anonymous-redefinition shape confirmed in `tests/apollo_regressions.rs`
    // (`part :>> engines[5] = (...);`) -- no own type, so this goes through
    // `part_usage_redefines_only` rather than `part_usage_named`'s permissive (and unrelated)
    // leading-`:>>`-discard shape.
    #[test]
    fn part_usage_captures_short_name_with_redefines() {
        let (rest, node) = part_usage(input("part <e> :>> engines[5];")).expect("part usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.short_name.as_deref(), Some("e"));
        assert_eq!(
            node.value.redefines.as_ref().map(|n| n.value.target.len()),
            Some(1)
        );
    }

    #[test]
    fn part_usage_without_short_name_has_none() {
        let (_, node) = part_usage(input("part engine : Engine;")).expect("part usage");
        assert_eq!(node.value.short_name, None);
    }

    /// GH-10: `ref part` is `PartUsage` with `is_reference` and full specialization.
    #[test]
    fn part_usage_accepts_ref_prefix_with_typing_and_subsetting() {
        let (rest, node) =
            part_usage(input("ref part origin : Remote :> remotes;")).expect("ref part usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.prefix.basic.reference_span.is_some());
        assert_eq!(node.value.name, "origin");
        assert_eq!(
            node.value
                .typing
                .as_ref()
                .map(|typing| typing.value.target.len()),
            Some(1)
        );
        assert_eq!(
            node.value
                .subsets
                .as_ref()
                .map(|(n, _)| n.value.target.len()),
            Some(1)
        );
    }

    #[test]
    fn part_usage_accepts_ref_prefix_with_subsetting_only() {
        let (rest, node) = part_usage(input("ref part origin :> mesolab;")).expect("ref part");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.prefix.basic.reference_span.is_some());
        assert!(node.value.subsets.is_some());
    }

    #[test]
    fn part_usage_without_ref_prefix_is_not_reference() {
        let (_, node) = part_usage(input("part origin : Remote :> remotes;")).expect("part");
        assert!(node.value.prefix.basic.reference_span.is_none());
        assert!(node.value.subsets.is_some());
    }

    #[test]
    fn part_usage_accepts_ref_prefix_leading_redefines_typed_form() {
        // Release validation `15_11-Variable Length Collection Types.sysml`.
        // Previously accepted by `part_ref_usage`; now owned by `part_usage` with `is_reference`.
        let (rest, node) =
            part_usage(input("ref part :>> elements: SparePart;")).expect("ref part :>>");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.prefix.basic.reference_span.is_some());
        assert!(node.value.name.is_empty());
        assert!(node.value.redefines.is_some());
        assert_eq!(
            node.value
                .typing
                .as_ref()
                .map(|typing| typing.value.target.len()),
            Some(1)
        );
    }
}

#[cfg(test)]
mod gh16_interface_usage_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    /// GH-16: a typed interface usage with no inline `connect` clause at all previously failed
    /// to parse entirely (BNF `InterfaceUsageDeclaration`'s `('connect' InterfacePart)?` is
    /// optional, so this is legal -- ends would be declared inside the body instead, or omitted).
    #[test]
    fn interface_usage_accepts_typed_declaration_with_no_connect_clause() {
        let (rest, node) =
            interface_usage(input("interface hubToRim : SpokeInterface;")).expect("declaration");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        match node.value {
            InterfaceUsage::Declaration {
                name,
                interface_type,
                ..
            } => {
                assert_eq!(name.as_deref(), Some("hubToRim"));
                assert!(interface_type.is_some());
            }
            other => panic!("expected Declaration, got {other:?}"),
        }
    }

    /// A bare `interface;` (no name, no type, no connect) is likewise a plain declaration.
    #[test]
    fn interface_usage_accepts_bare_declaration_with_no_name_or_type() {
        let (rest, node) = interface_usage(input("interface;")).expect("bare declaration");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, InterfaceUsage::Declaration { .. }));
    }

    /// GH-16: `connect [1] a to [1] b;` -- a leading cross multiplicity on each connect end
    /// (real shape from the Systems Library / Vehicle Example Annex fixtures) previously failed
    /// to parse; `connector_end_expression` had no support for it at all.
    #[test]
    fn interface_usage_accepts_bracketed_multiplicity_on_connect_ends() {
        let (rest, node) = interface_usage(input(
            "interface hubToRim : SpokeInterface connect [1] hub.p to [1] rim.p;",
        ))
        .expect("typed connect with bracketed multiplicity ends");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, InterfaceUsage::TypedConnect { .. }));
    }

    /// Multi-line form (the `connect` clause on its own line after the typed declaration) must
    /// keep parsing -- this is the common real-world layout in the Annex fixtures.
    #[test]
    fn interface_usage_accepts_multiline_connect_with_bracketed_multiplicity() {
        let (rest, node) = interface_usage(input(
            "interface hubToRim : SpokeInterface\n    connect [1] hub.p to [1] rim.p;",
        ))
        .expect("multiline typed connect");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, InterfaceUsage::TypedConnect { .. }));
    }

    /// Existing named + multiplicity + `::>` end-reference form (Vehicle Example Annex) must
    /// still parse once bracketed multiplicity is layered on top of it.
    #[test]
    fn interface_usage_accepts_end_name_with_bracketed_multiplicity() {
        let (rest, node) = interface_usage(input(
            "interface wheelHubInterface:WheelHubInterface connect [1] lugNutCompositePort ::> wheel1.lugNutCompositePort to [1] shankCompositePort ::> hub1.shankCompositePort;",
        ))
        .expect("named end with bracketed multiplicity");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, InterfaceUsage::TypedConnect { .. }));
    }

    /// Untyped/unnamed bare `interface a to b;` shorthand must still work (regression guard: the
    /// declaration-only fallback must not shadow this legal BNF `InterfacePart` alternative).
    #[test]
    fn interface_usage_still_accepts_bare_connection_shorthand() {
        let (rest, node) =
            interface_usage(input("interface hub.p to rim.p;")).expect("bare connection");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, InterfaceUsage::Connection { .. }));
    }

    /// GH-16 issue samples: https://github.com/elan8/sysml-v2-parser/issues/16
    #[test]
    fn gh16_issue_samples_parse_cleanly() {
        let samples = [
            r#"package Shop {
                port def HubPort;
                interface def SpokeInterface { end p1 : HubPort; end p2 : HubPort; }
                part def Hub { port p : HubPort; }
                part def Rim { port p : HubPort; }
                part def Wheel {
                    part hub : Hub;
                    part rim : Rim;
                    interface hubToRim : SpokeInterface;
                }
            }"#,
            r#"package Shop {
                port def HubPort;
                interface def SpokeInterface { end p1 : HubPort; end p2 : HubPort; }
                part def Hub { port p : HubPort; }
                part def Rim { port p : HubPort; }
                part def Wheel {
                    part hub : Hub;
                    part rim : Rim;
                    interface hubToRim : SpokeInterface
                        connect [1] hub.p to [1] rim.p;
                }
            }"#,
        ];
        for sample in samples {
            crate::parse(sample)
                .unwrap_or_else(|e| panic!("parse failed for sample:\n{sample}\n{e}"));
        }
    }
}

#[cfg(test)]
mod gh48_bind_prefix_and_multiplicity_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    /// GH-48 Gap 2a: bare `bind a = b;` (no `binding` prefix) must keep working exactly as
    /// before -- no regression from adding the optional prefix.
    #[test]
    fn bind_still_accepts_bare_form() {
        let (rest, node) = bind_(input("bind a = b;")).expect("bind a = b;");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.binding_name.is_none());
        assert!(node.value.binding_type.is_none());
        assert!(node.value.binding_multiplicity.is_none());
    }

    /// GH-48 Gap 2a: `binding <name> bind a = b;` -- named binding connector prefix, no type.
    /// Real usage: `sysml-v2-release/sysml/src/examples/Simple Tests/ConnectionTest.sysml`
    /// line 23.
    #[test]
    fn bind_accepts_named_binding_prefix() {
        let (rest, node) = bind_(input("binding ab bind a = b;")).expect("binding ab bind a = b;");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.binding_name.as_deref(), Some("ab"));
        assert!(node.value.binding_type.is_none());
    }

    /// GH-48 Gap 2a: `binding <name> : <Type> bind a = b;` -- named and typed binding connector
    /// prefix. Real usage: `ConnectionTest.sysml` line 24.
    #[test]
    fn bind_accepts_named_typed_binding_prefix() {
        let (rest, node) =
            bind_(input("binding ab1 : AB bind a = b;")).expect("binding ab1 : AB bind a = b;");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.binding_name.as_deref(), Some("ab1"));
        assert!(node.value.binding_type.is_some());
    }

    /// GH-48 Gap 2b: per-endpoint multiplicity on `bind`'s own two operands, plus an anonymous
    /// (name-less) `binding` prefix carrying its own multiplicity. Real usage (13 occurrences):
    /// `sysml-v2-release/sysml.library/Domain Libraries/Geometry/ShapeItems.sysml` lines 431-433,
    /// e.g. `binding [1] bind [0..*] base.edges = [0..*] be;`.
    #[test]
    fn bind_accepts_anonymous_binding_prefix_and_per_endpoint_multiplicity() {
        let (rest, node) = bind_(input("binding [1] bind [0..*] base.edges = [0..*] be;"))
            .expect("binding [1] bind [0..*] base.edges = [0..*] be;");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(node.value.binding_name.is_none());
        assert!(node.value.binding_multiplicity.is_some());
        assert!(node.value.left_multiplicity.is_some());
        assert!(node.value.right_multiplicity.is_some());
    }

    /// Guard against `bind_` swallowing an identifier that merely starts with the `bind`/
    /// `binding` keywords (e.g. a hypothetical `bindFoo`/`bindingFoo` name) -- `ws1` after each
    /// keyword requires actual whitespace, mirroring the same guard already used for `connect`/
    /// `succession`/etc. elsewhere in this crate.
    #[test]
    fn bind_rejects_bind_prefixed_identifier_without_separator() {
        assert!(bind_(input("bindFoo = b;")).is_err());
    }
}
