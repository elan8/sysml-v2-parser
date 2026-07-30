use super::body::{connection_usage_member, exhibit_state};
use super::prelude::*;
use crate::parser::attribute::directed_attribute_usage;
use crate::parser::feature_value_part as usage_value_part;
use crate::parser::item::directed_item_usage;

fn usage_ordered_modifier(input: Input<'_>) -> IResult<Input<'_>, bool> {
    let (input, ordered) = opt(preceded(ws_and_comments, tag(&b"ordered"[..]))).parse(input)?;
    let (input, _) = opt(preceded(ws_and_comments, tag(&b"nonunique"[..]))).parse(input)?;
    Ok((input, ordered.is_some()))
}

/// Part usage redefines-only: (`:>>` | `redefines`) qualified_name multiplicity? ordered? value? body.
pub(crate) fn part_usage_redefines_only<'a>(
    start: Input<'a>,
    input: Input<'a>,
) -> IResult<Input<'a>, Node<PartUsage>> {
    let (input, (_, redefines_qname)) = prefix_redefinition_target(input)?;
    let (input, multiplicity_opt) = opt(multiplicity_node).parse(input)?;
    let (input, ordered) = usage_ordered_modifier(input)?;
    let (input, value) = opt(preceded(ws_and_comments, usage_value_part)).parse(input)?;
    let (input, body) = part_usage_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            PartUsage {
                usage_prefix: None,
                is_individual: false,
                direction: None,
                is_derived: false,
                is_constant: false,
                name: String::new(),
                short_name: None,
                type_name: String::new(),
                typing: None,
                multiplicity: multiplicity_opt,
                ordered,
                subsets: None,
                redefines: Some(redefines_qname),
                value,
                body,
                name_span: None,
                type_ref_span: None,
                membership: Membership::feature(None, crate::ast::Span::dummy()),
            },
        ),
    ))
}

/// Part usage with name (and optional type, redefines, etc.): (':>>')? name ':' type_name? ...
pub(crate) fn part_usage_named<'a>(
    start: Input<'a>,
    input: Input<'a>,
) -> IResult<Input<'a>, Node<PartUsage>> {
    let (input, _) = opt(preceded(ws_and_comments, tag(&b":>>"[..]))).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (name_span, name_str)) = with_span(name).parse(input)?;
    let (input, multiplicity_opt) = opt(multiplicity_node).parse(input)?;
    let (input, ordered_before_type) = usage_ordered_modifier(input)?;
    let (input, type_result) = optional_typings(input)?;
    let typing = type_result
        .clone()
        .map(|(s, is_conjugated, targets)| typing_node(s, is_conjugated, targets));
    let (type_ref_span, type_name) = type_result
        .map(|(s, is_conjugated, targets)| {
            let t = targets_display_string(&targets);
            (Some(s), if is_conjugated { format!("~{t}") } else { t })
        })
        .unwrap_or((None, String::new()));
    let (input, trailing_multiplicity_opt) = opt(multiplicity_node).parse(input)?;
    let multiplicity_opt = multiplicity_opt.or(trailing_multiplicity_opt);
    let (input, ordered_after_type) = usage_ordered_modifier(input)?;
    let ordered = ordered_before_type || ordered_after_type;
    let (input, leading_clauses) = specialization_clauses(input)?;
    let (input, post_clause_multiplicity) = opt(multiplicity_node).parse(input)?;
    let multiplicity_opt = multiplicity_opt.or(post_clause_multiplicity);
    let (input, ordered_after_clauses) = usage_ordered_modifier(input)?;
    let ordered = ordered || ordered_after_clauses;
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
                usage_prefix: None,
                is_individual: false,
                direction: None,
                is_derived: false,
                is_constant: false,
                name: name_str,
                short_name: None,
                type_name,
                typing,
                multiplicity: multiplicity_opt,
                ordered,
                subsets,
                redefines,
                value,
                body,
                name_span: Some(name_span),
                type_ref_span,
                membership: Membership::feature(None, crate::ast::Span::dummy()),
            },
        ),
    ))
}

/// Part usage: 'part' ( ':>>' qualified_name | (':>>')? name ':' type_name? ... ) multiplicity? ... body
///
/// Prefix keywords follow BNF `RefPrefix`/`OccurrenceUsagePrefix` order (§8.2.2.6.2, §8.2.2.9.2,
/// reached via `PartUsage = OccurrenceUsagePrefix 'part' Usage` -> `OccurrenceUsagePrefix :
/// BasicUsagePrefix ...` -> `BasicUsagePrefix : RefPrefix ...`): direction, `derived`,
/// (`abstract`|`variation`), `constant`, then `individual`.
pub(crate) fn part_usage(input: Input<'_>) -> IResult<Input<'_>, Node<PartUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, direction) = opt(crate::parser::attribute::direction_prefix).parse(input)?;
    let (input, is_derived) = opt(preceded(tag(&b"derived"[..]), ws1))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
    let (input, usage_prefix) = opt(alt((
        map(preceded(tag(&b"abstract"[..]), ws1), |_| {
            DefinitionPrefix::Abstract
        }),
        map(preceded(tag(&b"variation"[..]), ws1), |_| {
            DefinitionPrefix::Variation
        }),
    )))
    .parse(input)?;
    let (input, is_constant) = opt(preceded(tag(&b"constant"[..]), ws1))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
    let (input, is_individual) = opt(preceded(tag(&b"individual"[..]), ws1))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
    let (input, _) = tag(&b"part"[..]).parse(input)?;
    // Allow `part: Type` with no whitespace (anonymous UsageDeclaration).
    let (after_kw, _) = ws_and_comments(input)?;
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
    // citation. Parsed once here at the dispatch level (rather than inside each of the three
    // branches below) and threaded through post-hoc, mirroring how `usage_prefix`/`is_individual`/
    // etc. are already applied to whichever branch matches.
    let (input, short_name) = short_name_prefix(input)?;
    // Consume (not just peek) whitespace/comments after the short name's closing `>` -- see
    // `attribute::attribute_usage`'s identical fix for why this can't reuse `ws1`'s earlier
    // consumption (a short name leaves fresh un-consumed whitespace after it).
    let (input, _) = ws_and_comments(input)?;
    let peek = input;
    if (peek.fragment().starts_with(b":")
        && !peek.fragment().starts_with(b":>")
        && !peek.fragment().starts_with(b":>>"))
        || starts_with_keyword(peek.fragment(), b"defined")
    {
        let (input, mut usage) = anonymous_part_usage(start, input)?;
        usage.value.usage_prefix = usage_prefix;
        usage.value.is_individual = is_individual;
        usage.value.direction = direction;
        usage.value.is_derived = is_derived;
        usage.value.is_constant = is_constant;
        usage.value.short_name = short_name;
        usage.value.membership = Membership::feature(visibility, visibility_span);
        return Ok((input, usage));
    }
    if let Ok((input, usage)) = part_usage_redefines_only(start, input) {
        let mut usage = usage;
        usage.value.usage_prefix = usage_prefix;
        usage.value.is_individual = is_individual;
        usage.value.direction = direction;
        usage.value.is_derived = is_derived;
        usage.value.is_constant = is_constant;
        usage.value.short_name = short_name;
        usage.value.membership = Membership::feature(visibility, visibility_span);
        return Ok((input, usage));
    }
    let (input, mut usage) = part_usage_named(start, input)?;
    usage.value.usage_prefix = usage_prefix;
    usage.value.is_individual = is_individual;
    usage.value.direction = direction;
    usage.value.is_derived = is_derived;
    usage.value.is_constant = is_constant;
    usage.value.short_name = short_name;
    usage.value.membership = Membership::feature(visibility, visibility_span);
    Ok((input, usage))
}

fn anonymous_part_usage<'a>(
    start: Input<'a>,
    input: Input<'a>,
) -> IResult<Input<'a>, Node<PartUsage>> {
    let (input, multiplicity_before) = opt(multiplicity_node).parse(input)?;
    let (input, ordered_before_type) = usage_ordered_modifier(input)?;
    let (input, (type_ref_span, is_conjugated, targets)) = typings(input)?;
    let type_name = targets_display_string(&targets);
    let type_name = if is_conjugated {
        format!("~{type_name}")
    } else {
        type_name
    };
    let typing = Some(typing_node(type_ref_span.clone(), is_conjugated, targets));
    let (input, multiplicity_after) = opt(multiplicity_node).parse(input)?;
    let multiplicity_opt = multiplicity_before.or(multiplicity_after);
    let (input, ordered_after_type) = usage_ordered_modifier(input)?;
    let ordered = ordered_before_type || ordered_after_type;
    let (input, clauses) = specialization_clauses(input)?;
    let (input, post_clause_multiplicity) = opt(multiplicity_node).parse(input)?;
    let multiplicity_opt = multiplicity_opt.or(post_clause_multiplicity);
    let (input, ordered_after_clauses) = usage_ordered_modifier(input)?;
    let ordered = ordered || ordered_after_clauses;
    let (input, value) = opt(preceded(ws_and_comments, usage_value_part)).parse(input)?;
    let (input, body) = part_usage_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            PartUsage {
                usage_prefix: None,
                is_individual: false,
                direction: None,
                is_derived: false,
                is_constant: false,
                name: String::new(),
                short_name: None,
                type_name,
                typing,
                multiplicity: multiplicity_opt,
                ordered,
                subsets: clauses.subsets,
                redefines: clauses.redefines,
                value,
                body,
                name_span: None,
                type_ref_span: Some(type_ref_span),
                membership: Membership::feature(None, crate::ast::Span::dummy()),
            },
        ),
    ))
}

/// Part usage body: ';' or '{' PartUsageBodyElement* '}'
fn part_usage_body(input: Input<'_>) -> IResult<Input<'_>, PartUsageBody> {
    let (input, _) = ws_and_comments(input)?;
    let frag = input.fragment();
    log::debug!(
        "part_usage_body: first 40 bytes: {:?}",
        frag.get(..40.min(frag.len())).unwrap_or(frag),
    );
    let result = alt((
        map(tag(&b";"[..]), |_| PartUsageBody::Semicolon),
        part_usage_body_brace,
    ))
    .parse(input);
    if result.is_err() {
        log::debug!(
            "part_usage_body: failed at: {:?}",
            String::from_utf8_lossy(frag.get(..60.min(frag.len())).unwrap_or(frag)),
        );
    }
    result
}

fn part_usage_body_brace(input: Input<'_>) -> IResult<Input<'_>, PartUsageBody> {
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
            log::debug!("part_usage_body: brace ok, {} elements", elements.len());
            return Ok((input, PartUsageBody::Brace { elements }));
        }
        match part_usage_body_element(input) {
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
            Err(_) if starts_with_any_keyword(input.fragment(), PART_BODY_STARTERS) => {
                let (next, _) = recover_body_element(input, PART_BODY_STARTERS)?;
                if next.location_offset() == input.location_offset() {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Many0,
                    )));
                }
                elements.push(node_from_to(
                    input,
                    next,
                    PartUsageBodyElement::Error(Node::new(
                        crate::ast::Span::dummy(),
                        build_recovery_error_node_from_span(
                            input,
                            next,
                            PART_BODY_STARTERS,
                            "part usage body",
                            "recovered_part_usage_body_element",
                        ),
                    )),
                ));
                input = next;
            }
            Err(_) => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Tag,
                )));
            }
        }
    }
}

fn consume_part_usage_structured_brace(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _elements) = parse_structured_brace_members_with_skip(
        input,
        PART_BODY_STARTERS,
        "part usage body",
        "recovered_part_usage_body_element",
        part_usage_body_element,
        |start, end| {
            node_from_to(
                start,
                end,
                PartUsageBodyElement::Error(Node::new(
                    crate::ast::Span::dummy(),
                    build_recovery_error_node_from_span(
                        start,
                        end,
                        PART_BODY_STARTERS,
                        "part usage body",
                        "recovered_part_usage_body_element",
                    ),
                )),
            )
        },
        BraceMemberSkip::BodyElementRecover,
    )?;
    Ok((input, ()))
}

/// Action path for perform: name ( '.' name )* -> joined with ".".
fn perform_action_path(input: Input<'_>) -> IResult<Input<'_>, String> {
    let (input, first) = name(input)?;
    let mut rest_parser = many0(preceded(
        preceded(ws_and_comments, tag(&b"."[..])),
        preceded(ws_and_comments, name),
    ));
    let (input, rest) = rest_parser.parse(input)?;
    let action_name = std::iter::once(first)
        .chain(rest)
        .collect::<Vec<_>>()
        .join(".");
    Ok((input, action_name))
}

/// In/out binding inside a perform body: `in` name `=` expr `;` or `out` name `=` expr `;`.
fn perform_in_out_binding(input: Input<'_>) -> IResult<Input<'_>, Node<PerformInOutBinding>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, direction) = alt((
        value(InOut::In, tag(&b"in"[..])),
        value(InOut::Out, tag(&b"out"[..])),
    ))
    .parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
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
                name: name_str,
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
        map(doc_comment, PerformBodyElement::Doc),
        map(perform_in_out_binding, PerformBodyElement::InOut),
        // §6 G6: parameter-direction usage members (`in part :>> name = value;`, `in item 'n' :
        // Type { }`, …) reuse the same directed/usage parsers as port-def bodies rather than
        // duplicating the grammar here. Placed before `variant`/`action` so simple `in name =`
        // bindings still win via `perform_in_out_binding` above.
        map(part_usage, |p| PerformBodyElement::PartUsage(Box::new(p))),
        map(directed_item_usage, |i| {
            PerformBodyElement::ItemUsage(Box::new(i))
        }),
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
    let (input, elements) = nom::sequence::delimited(
        tag(&b"{"[..]),
        preceded(
            ws_and_comments,
            many0(preceded(ws_and_comments, perform_body_element)),
        ),
        preceded(ws_and_comments, tag(&b"}"[..])),
    )
    .parse(input)?;
    Ok((input, PerformBody::Brace { elements }))
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
        alt((
            map(tag(&b";"[..]), |_| PerformBody::Semicolon),
            perform_body,
        )),
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
    let (input, action_name) = perform_action_path(input)?;
    let (input, redefines) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>>"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
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
                type_name: None,
                redefines,
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
    let (input, redefines) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>>"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let (input, type_name) = opt(preceded(
        preceded(ws_and_comments, typing_colon),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
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
                type_name,
                redefines,
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
    let (input, body) = connect_body(input)?;
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

/// Bind: `bind` path `=` path (`;` or `{ }`)
pub(crate) fn bind_(input: Input<'_>) -> IResult<Input<'_>, Node<Bind>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"bind"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, left) = path_expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"="[..])).parse(input)?;
    let (input, right) = preceded(ws_and_comments, path_expression).parse(input)?;
    let mut body_parser = alt((
        map(preceded(ws_and_comments, tag(&b";"[..])), |_| {
            Some(ConnectBody::Semicolon)
        }),
        map(consume_part_usage_structured_brace, |_| {
            Some(ConnectBody::Brace)
        }),
    ));
    let (input, body) = body_parser.parse(input)?;
    Ok((
        input,
        node_from_to(start, input, Bind { left, right, body }),
    ))
}

/// Connect (part usage level): `connect` path `to` path body
pub(crate) fn connect_(input: Input<'_>) -> IResult<Input<'_>, Node<Connect>> {
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
    let (input, body) = connect_body(input)?;
    let (input, trailing_subsets) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let (input, trailing_redefines) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>>"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let input = if trailing_subsets.is_some() || trailing_redefines.is_some() {
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

/// Interface usage body elements: `ref` `:>>` name `=` value body (RefRedef), or `doc`.
fn interface_usage_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<InterfaceUsageBodyElement>> {
    alt((
        interface_usage_ref_redef,
        map(doc_comment, |doc| {
            let span = doc.span.clone();
            Node::new(span, InterfaceUsageBodyElement::Doc(doc))
        }),
    ))
    .parse(input)
}

fn interface_usage_ref_redef(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<InterfaceUsageBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"ref"[..]).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b":>>"[..])).parse(input)?;
    let (input, ref_name) = preceded(ws_and_comments, name).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"="[..])).parse(input)?;
    let (input, value) = preceded(ws_and_comments, expression).parse(input)?;
    let (input, body) = ref_body_parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            InterfaceUsageBodyElement::RefRedef {
                name: ref_name,
                value,
                body,
            },
        ),
    ))
}

fn ref_body_parse(input: Input<'_>) -> IResult<Input<'_>, RefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b";"[..]), |_| RefBody::Semicolon),
        map(consume_part_usage_structured_brace, |_| RefBody::Brace {
            elements: vec![],
        }),
    ))
    .parse(input)
}

/// Connect body for interface usage (TypedConnect): `;` or `{` body_elements* `}`
fn connect_body_with_elements(
    input: Input<'_>,
) -> IResult<Input<'_>, (ConnectBody, Vec<Node<InterfaceUsageBodyElement>>)> {
    let (input, _) = ws_and_comments(input)?;
    if let Ok((input, _)) = tag::<_, _, nom::error::Error<Input>>(&b";"[..]).parse(input) {
        return Ok((input, (ConnectBody::Semicolon, vec![])));
    }

    let (mut input, _) = tag(&b"{"[..]).parse(input)?;
    let mut elements = Vec::new();
    loop {
        let (next, _) = ws_and_comments(input)?;
        input = next;
        if input.fragment().starts_with(b"}") {
            let (input, _) = tag(&b"}"[..]).parse(input)?;
            return Ok((input, (ConnectBody::Brace, elements)));
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
/// Accepts either `path` or `endName ::> path`; the end name is currently ignored.
fn connector_end_expression(input: Input<'_>) -> IResult<Input<'_>, Node<Expression>> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = opt((name, preceded(ws_and_comments, tag(&b"::>"[..])))).parse(input)?;
    preceded(ws_and_comments, path_expression).parse(input)
}

/// Interface usage: `interface` ( name `:` )? ( `:Type` )? `connect` path `to` path body
/// or `interface` path `to` path body. The optional interface member name is currently ignored.
pub(crate) fn interface_usage(input: Input<'_>) -> IResult<Input<'_>, Node<InterfaceUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"interface"[..]).parse(input)?;
    let (input, _) = if input.fragment().starts_with(b":") {
        (input, ())
    } else {
        ws1(input)?
    };
    let (input, named_interface) = opt((
        name,
        opt(multiplicity_node),
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let (input, interface_type) = if let Some((_, _, _, interface_type)) = named_interface {
        (input, Some(interface_type))
    } else {
        opt(preceded(
            tag(&b":"[..]),
            preceded(ws_and_comments, qualified_name),
        ))
        .parse(input)?
    };
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b"connect") {
        let (input, _) = tag(&b"connect"[..]).parse(input)?;
        let (input, _) = ws1(input)?;
        let (input, from_expr) = connector_end_expression(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
        let (input, to_expr) = preceded(ws_and_comments, connector_end_expression).parse(input)?;
        let (input, (body, body_elements)) = connect_body_with_elements(input)?;
        Ok((
            input,
            node_from_to(
                start,
                input,
                InterfaceUsage::TypedConnect {
                    interface_type,
                    from: from_expr,
                    to: to_expr,
                    body,
                    body_elements,
                },
            ),
        ))
    } else {
        let (input, from_expr) = connector_end_expression(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
        let (input, to_expr) = preceded(ws_and_comments, connector_end_expression).parse(input)?;
        let (input, _) = opt(connect_body).parse(input)?;
        Ok((
            input,
            node_from_to(
                start,
                input,
                InterfaceUsage::Connection {
                    from: from_expr,
                    to: to_expr,
                    body_elements: vec![],
                },
            ),
        ))
    }
}

/// Ref in part usage/def body: `(visibility)? ref` (`part`)? name (`:` type)? (`=` value)? body.
pub(crate) fn part_ref_usage(input: Input<'_>) -> IResult<Input<'_>, Node<RefDecl>> {
    let start = input;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, _) = tag(&b"ref"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    // Reject kinded refs (`ref action` / `ref state` / `ref port` / …) so those forms can be
    // parsed as real ActionUsage/StateUsage/PortUsage instead of a mis-named RefDecl.
    if crate::parser::lex::starts_with_any_keyword(
        input.fragment(),
        &[
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
    let (input, _) = opt(preceded(tag(&b"part"[..]), ws1)).parse(input)?;
    let (input, _) = opt(preceded(
        ws_and_comments,
        preceded(tag(&b":>>"[..]), ws_and_comments),
    ))
    .parse(input)?;
    let (input, name_str) = name(input)?;
    let (input, type_result) = crate::parser::usage::optional_typings(input)?;
    let (type_ref_span, type_name, typing) =
        crate::parser::usage::typing_fields_from_result(type_result);
    let (input, value) = opt(preceded(
        preceded(ws_and_comments, tag(&b"="[..])),
        preceded(ws_and_comments, expression),
    ))
    .parse(input)?;
    let value = value.map(crate::parser::feature_value::wrap_bind_expression);
    let (input, body) = preceded(
        ws_and_comments,
        alt((
            map(tag(&b";"[..]), |_| RefBody::Semicolon),
            map(consume_part_usage_structured_brace, |_| RefBody::Brace {
                elements: vec![],
            }),
        )),
    )
    .parse(input)?;
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
    let start = input;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let membership = Membership::variant(visibility, visibility_span);
    let (input, _) = tag(&b"variant"[..]).parse(input)?;
    let (input, _) = ws1(input)?;

    if let Ok((next, usage)) = part_usage(input) {
        let name = usage.value.name.clone();
        return Ok((
            next,
            node_from_to(
                start,
                next,
                VariantUsage {
                    name,
                    typed: Some(VariantTypedUsage::Part(Box::new(usage))),
                    membership,
                },
            ),
        ));
    }
    if let Ok((next, usage)) = attribute_usage(input) {
        let name = usage.value.name.clone();
        return Ok((
            next,
            node_from_to(
                start,
                next,
                VariantUsage {
                    name,
                    typed: Some(VariantTypedUsage::Attribute(Box::new(usage))),
                    membership,
                },
            ),
        ));
    }
    if let Ok((next, usage)) = item_usage(input) {
        let name = usage.value.name.clone();
        return Ok((
            next,
            node_from_to(
                start,
                next,
                VariantUsage {
                    name,
                    typed: Some(VariantTypedUsage::Item(Box::new(usage))),
                    membership,
                },
            ),
        ));
    }
    if let Ok((next, usage)) = port_usage(input) {
        let name = usage.value.name.clone();
        return Ok((
            next,
            node_from_to(
                start,
                next,
                VariantUsage {
                    name,
                    typed: Some(VariantTypedUsage::Port(Box::new(usage))),
                    membership,
                },
            ),
        ));
    }
    // §6 G5: `variant perform doX;` inside a `variation perform action ... { ... }` body.
    // `perform_action_decl` first, for the same bare-keyword reason as the dispatchers above.
    if let Ok((next, usage)) = alt((perform_action_decl, perform_usage)).parse(input) {
        let name = usage.value.action_name.clone();
        return Ok((
            next,
            node_from_to(
                start,
                next,
                VariantUsage {
                    name,
                    typed: Some(VariantTypedUsage::Perform(Box::new(usage))),
                    membership,
                },
            ),
        ));
    }

    let (input, name) = name(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            VariantUsage {
                name,
                typed: None,
                membership,
            },
        ),
    ))
}

fn part_usage_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PartUsageBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
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
                map(doc_comment, PartUsageBodyElement::Doc),
                map(
                    crate::parser::metadata_annotation::metadata_keyword_usage,
                    PartUsageBodyElement::MetadataKeywordUsage,
                ),
                map(
                    metadata_annotation,
                    PartUsageBodyElement::MetadataAnnotation,
                ),
                map(annotation, PartUsageBodyElement::Annotation),
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
            map(allocate_, PartUsageBodyElement::Allocate),
            map(variant_usage, PartUsageBodyElement::VariantUsage),
            map(attribute_usage, PartUsageBodyElement::AttributeUsage),
            map(
                attribute_usage_shorthand,
                PartUsageBodyElement::DefaultReferenceUsage,
            ),
            alt((
                map(enum_usage, PartUsageBodyElement::EnumerationUsage),
                map(part_usage, |p| PartUsageBodyElement::PartUsage(Box::new(p))),
            )),
            map(individual_usage, |n| {
                PartUsageBodyElement::OccurrenceUsage(Box::new(n))
            }),
            map(snapshot_usage, |n| {
                PartUsageBodyElement::OccurrenceUsage(Box::new(n))
            }),
            map(timeslice_usage, |n| {
                PartUsageBodyElement::OccurrenceUsage(Box::new(n))
            }),
            map(then_timeslice_usage, |n| {
                PartUsageBodyElement::OccurrenceUsage(Box::new(n))
            }),
            map(occurrence_usage, |n| {
                PartUsageBodyElement::OccurrenceUsage(Box::new(n))
            }),
        )),
        // PAR-002: nested `def` kinds -- usage bodies legally contain nested definitions per BNF
        // `UsageBody = DefinitionBody`. `port_def_required`/`calc_def_required`/
        // `connection_def_required` must be tried before `port_usage`/`connection_usage_member`
        // -- both usage-form parsers have no guard against a bare `def` keyword (same bug class
        // fixed for `PartDefBodyElement` in a prior increment), so `port def Foo;`/
        // `connection def Foo;` would otherwise misparse as a usage named "def".
        // `state_def`/`metadata_def`/`requirement_def`/`occurrence_def` are all
        // `def_required()`-guarded internally and have no usage sibling dispatched in this body
        // today, so no ordering risk for them either.
        alt((
            map(state_def, PartUsageBodyElement::StateDef),
            map(
                crate::parser::enumeration::enum_def,
                PartUsageBodyElement::EnumDef,
            ),
            map(metadata_def, PartUsageBodyElement::MetadataDef),
            map(requirement_def, PartUsageBodyElement::RequirementDef),
            // §6 G5: the usage form was reachable from part *definition* bodies only.
            map(requirement_usage, PartUsageBodyElement::RequirementUsage),
            map(occurrence_def, PartUsageBodyElement::OccurrenceDef),
            map(calc_def_required, PartUsageBodyElement::CalcDef),
            // `constraint_def` before `constraint_usage` for the same bare-`def` reason.
            map(constraint_def, PartUsageBodyElement::ConstraintDef),
            map(constraint_usage, PartUsageBodyElement::ConstraintUsage),
            // §6 G16: a part body is a namespace, so it owns imports too.
            map(crate::parser::import::import_, PartUsageBodyElement::Import),
            map(connection_def_required, PartUsageBodyElement::ConnectionDef),
            map(connection_usage_member, PartUsageBodyElement::Connection),
            map(port_def_required, PartUsageBodyElement::PortDef),
        )),
        alt((
            map(port_usage, PartUsageBodyElement::PortUsage),
            map(part_ref_usage, PartUsageBodyElement::Ref),
            map(bind_, PartUsageBodyElement::Bind),
            map(satisfy, PartUsageBodyElement::Satisfy),
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
        is_abstract: false,
        is_reference: false,
        name: exhibit.value.name,
        type_name: exhibit.value.type_name,
        typing: None,
        multiplicity: None,
        subsets: None,
        // §6 G18: previously dropped, which silently lost the redefinition target of
        // `exhibit <name> :>> <target>;`.
        redefines: exhibit.value.redefines,
        body: exhibit.value.body,
        // `ExhibitState` (the struct this adapts from) has no `membership`/visibility field of
        // its own -- out of this item's scope, see `CHANGELOG.md`'s Item 4b entries -- so there
        // is nothing to thread through here; ad hoc site, `visibility: None` per this rollout's
        // established convention (see `AttributeUsage`'s ad hoc sites).
        membership: crate::ast::Membership::feature(None, crate::ast::Span::dummy()),
    };
    Ok((input, Node::new(exhibit.span, state)))
}

#[cfg(test)]
mod par_002_nested_def_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
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
        let crate::ast::PartDefBody::Brace { elements } = &def_node.value.body else {
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
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
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
        assert_eq!(node.value.action_name, "vehicleMassTest");
        assert_eq!(node.value.redefines, None);
        assert!(matches!(node.value.body, PerformBody::Semicolon));
    }

    #[test]
    fn perform_dotted_name_accepts_semicolon_body() {
        let node = perform("perform providePower.generateTorque;");
        assert_eq!(node.value.action_name, "providePower.generateTorque");
        assert!(matches!(node.value.body, PerformBody::Semicolon));
    }

    #[test]
    fn perform_quoted_name_accepts_semicolon_body() {
        let node = perform("perform 'provide power';");
        assert_eq!(node.value.action_name, "provide power");
        assert!(matches!(node.value.body, PerformBody::Semicolon));
    }

    #[test]
    fn perform_accepts_redefine_clause_with_semicolon_body() {
        let node = perform("perform providePower.generateTorque :>> generateTorque;");
        assert_eq!(node.value.action_name, "providePower.generateTorque");
        assert_eq!(node.value.redefines.as_deref(), Some("generateTorque"));
        assert!(matches!(node.value.body, PerformBody::Semicolon));
    }

    #[test]
    fn perform_accepts_redefine_clause_with_brace_body() {
        let node = perform("perform 'provide power' :>> VehicleA::'provide power' { }");
        assert_eq!(node.value.action_name, "provide power");
        assert_eq!(
            node.value.redefines.as_deref(),
            Some("VehicleA::provide power")
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
        assert_eq!(perform.value.redefines, None);
    }

    /// §6 G6: directed `part`/`item` usages inside a perform body.
    #[test]
    fn perform_body_accepts_directed_part_and_item_usages() {
        let node = perform("perform vehicleMassTest { in part :>> testVehicle = vehicleUnderTest; in item 'mass sample' : MassSample { } }");
        let PerformBody::Brace { elements } = node.value.body else {
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
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
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
    use crate::parser::usage::targets_display_string;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    #[test]
    fn part_usage_captures_short_name() {
        let (rest, node) = part_usage(input("part <eng> engine : Engine;")).expect("part usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.short_name.as_deref(), Some("eng"));
        assert_eq!(node.value.name, "engine");
        assert_eq!(node.value.type_name, "Engine");
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
            node.value
                .redefines
                .as_ref()
                .map(|n| targets_display_string(&n.value.target)),
            Some("engines".to_string())
        );
    }

    #[test]
    fn part_usage_without_short_name_has_none() {
        let (_, node) = part_usage(input("part engine : Engine;")).expect("part usage");
        assert_eq!(node.value.short_name, None);
    }
}
