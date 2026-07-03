use super::body::exhibit_state;
use super::prelude::*;

/// Value part for usages: `= expr` | `:= expr` | `default = expr` | `default := expr` | `default expr`.
fn usage_value_part(input: Input<'_>) -> IResult<Input<'_>, Node<crate::ast::Expression>> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = alt((
        preceded(tag(&b"="[..]), ws_and_comments),
        preceded(tag(&b":="[..]), ws_and_comments),
        preceded(
            preceded(tag(&b"default"[..]), ws1),
            alt((
                preceded(alt((tag(&b"="[..]), tag(&b":="[..]))), ws_and_comments),
                ws_and_comments,
            )),
        ),
    ))
    .parse(input)?;
    expression(input)
}

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
    let (input, multiplicity_opt) = opt(multiplicity).parse(input)?;
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
                name: String::new(),
                type_name: String::new(),
                multiplicity: multiplicity_opt,
                ordered,
                subsets: None,
                redefines: Some(redefines_qname),
                value,
                body,
                name_span: None,
                type_ref_span: None,
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
    let (input, multiplicity_opt) = opt(multiplicity).parse(input)?;
    let (input, ordered_before_type) = usage_ordered_modifier(input)?;
    let (input, type_result) = optional_typings(input)?;
    let (type_ref_span, type_name) = type_result
        .map(|(s, t)| (Some(s), t))
        .unwrap_or((None, String::new()));
    let (input, trailing_multiplicity_opt) = opt(multiplicity).parse(input)?;
    let multiplicity_opt = multiplicity_opt.or(trailing_multiplicity_opt);
    let (input, ordered_after_type) = usage_ordered_modifier(input)?;
    let ordered = ordered_before_type || ordered_after_type;
    let (input, leading_clauses) = specialization_clauses(input)?;
    let (input, post_clause_multiplicity) = opt(multiplicity).parse(input)?;
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
                name: name_str,
                type_name,
                multiplicity: multiplicity_opt,
                ordered,
                subsets,
                redefines,
                value,
                body,
                name_span: Some(name_span),
                type_ref_span,
            },
        ),
    ))
}

/// Part usage: 'part' ( ':>>' qualified_name | (':>>')? name ':' type_name? ... ) multiplicity? ... body
pub(crate) fn part_usage(input: Input<'_>) -> IResult<Input<'_>, Node<PartUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, usage_prefix) = opt(alt((
        map(preceded(tag(&b"abstract"[..]), ws1), |_| {
            DefinitionPrefix::Abstract
        }),
        map(preceded(tag(&b"variation"[..]), ws1), |_| {
            DefinitionPrefix::Variation
        }),
    )))
    .parse(input)?;
    let (input, is_individual) = opt(preceded(tag(&b"individual"[..]), ws1))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
    let (input, _) = tag(&b"part"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (peek, _) = ws_and_comments(input)?;
    if (peek.fragment().starts_with(b":")
        && !peek.fragment().starts_with(b":>")
        && !peek.fragment().starts_with(b":>>"))
        || starts_with_keyword(peek.fragment(), b"defined")
    {
        let (input, mut usage) = anonymous_part_usage(start, input)?;
        usage.value.usage_prefix = usage_prefix;
        usage.value.is_individual = is_individual;
        return Ok((input, usage));
    }
    if let Ok((input, usage)) = part_usage_redefines_only(start, input) {
        let mut usage = usage;
        usage.value.usage_prefix = usage_prefix;
        usage.value.is_individual = is_individual;
        return Ok((input, usage));
    }
    let (input, mut usage) = part_usage_named(start, input)?;
    usage.value.usage_prefix = usage_prefix;
    usage.value.is_individual = is_individual;
    Ok((input, usage))
}

fn anonymous_part_usage<'a>(
    start: Input<'a>,
    input: Input<'a>,
) -> IResult<Input<'a>, Node<PartUsage>> {
    let (input, multiplicity_before) = opt(multiplicity).parse(input)?;
    let (input, ordered_before_type) = usage_ordered_modifier(input)?;
    let (input, (type_ref_span, type_name)) = typings(input)?;
    let (input, multiplicity_after) = opt(multiplicity).parse(input)?;
    let multiplicity_opt = multiplicity_before.or(multiplicity_after);
    let (input, ordered_after_type) = usage_ordered_modifier(input)?;
    let ordered = ordered_before_type || ordered_after_type;
    let (input, clauses) = specialization_clauses(input)?;
    let (input, post_clause_multiplicity) = opt(multiplicity).parse(input)?;
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
                name: String::new(),
                type_name,
                multiplicity: multiplicity_opt,
                ordered,
                subsets: clauses.subsets,
                redefines: clauses.redefines,
                value,
                body,
                name_span: None,
                type_ref_span: Some(type_ref_span),
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

/// Perform body element: doc comment or in/out binding.
fn perform_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PerformBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = alt((
        map(doc_comment, PerformBodyElement::Doc),
        map(perform_in_out_binding, PerformBodyElement::InOut),
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

/// Perform usage: `perform` action_path body (with optional `{ }` body).
pub(crate) fn perform_usage(input: Input<'_>) -> IResult<Input<'_>, Node<Perform>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"perform"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, action_name) = perform_action_path(input)?;
    let (input, body) = perform_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Perform {
                action_name,
                type_name: None,
                body,
            },
        ),
    ))
}

/// Perform action declaration: `perform action` name (`:` type_name)? (`;` or body).
pub(crate) fn perform_action_decl(input: Input<'_>) -> IResult<Input<'_>, Node<Perform>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"perform"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(&b"action"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, action_name) = name(input)?;
    let (input, type_name) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let (input, body) = preceded(
        ws_and_comments,
        alt((
            map(tag(&b";"[..]), |_| PerformBody::Semicolon),
            perform_body,
        )),
    )
    .parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Perform {
                action_name,
                type_name,
                body,
            },
        ),
    ))
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
        map(consume_part_usage_structured_brace, |_| Some(ConnectBody::Brace)),
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
    let (input, from_expr) = path_expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
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
                from: from_expr,
                to: to_expr,
                body,
            },
        ),
    ))
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
        map(consume_part_usage_structured_brace, |_| RefBody::Brace { elements: vec![] }),
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
        opt(multiplicity),
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

/// Ref in part usage body: `ref` (`part`)? name (`:` type)? (`=` value)? body.
pub(crate) fn part_ref_usage(input: Input<'_>) -> IResult<Input<'_>, Node<RefDecl>> {
    let start = input;
    let (input, _) = tag(&b"ref"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = opt(preceded(tag(&b"part"[..]), ws1)).parse(input)?;
    let (input, _) = opt(preceded(
        ws_and_comments,
        preceded(tag(&b":>>"[..]), ws_and_comments),
    ))
    .parse(input)?;
    let (input, name_str) = name(input)?;
    let (input, type_name) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let (input, value) = opt(preceded(
        preceded(ws_and_comments, tag(&b"="[..])),
        preceded(ws_and_comments, expression),
    ))
    .parse(input)?;
    let type_name = type_name.unwrap_or_default();
    let (input, body) = preceded(
        ws_and_comments,
        alt((
            map(tag(&b";"[..]), |_| RefBody::Semicolon),
            map(consume_part_usage_structured_brace, |_| RefBody::Brace { elements: vec![] }),
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
                value,
                body,
                name_span: None,
                type_ref_span: None,
            },
        ),
    ))
}

fn variant_usage(input: Input<'_>) -> IResult<Input<'_>, Node<VariantUsage>> {
    let start = input;
    let (input, _) = tag(&b"variant"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, name) = name(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(start, input, VariantUsage { name }),
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
        map(perform_action_decl, PartUsageBodyElement::Perform),
        map(perform_usage, PartUsageBodyElement::Perform),
        map(allocate_, PartUsageBodyElement::Allocate),
        map(variant_usage, PartUsageBodyElement::VariantUsage),
        map(attribute_usage, PartUsageBodyElement::AttributeUsage),
        map(
            attribute_usage_shorthand,
            PartUsageBodyElement::AttributeUsage,
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
        map(port_usage, PartUsageBodyElement::PortUsage),
        map(part_ref_usage, PartUsageBodyElement::Ref),
        map(bind_, PartUsageBodyElement::Bind),
        map(satisfy, PartUsageBodyElement::Satisfy),
        map(interface_usage, PartUsageBodyElement::InterfaceUsage),
        map(connect_, PartUsageBodyElement::Connect),
        map(
            crate::parser::flow::flow_usage_member,
            PartUsageBodyElement::FlowUsage,
        ),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn exhibit_state_as_state_usage(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::StateUsage>> {
    let (input, exhibit) = exhibit_state(input)?;
    let state = crate::ast::StateUsage {
        name: exhibit.value.name,
        type_name: exhibit.value.type_name,
        body: exhibit.value.body,
    };
    Ok((input, Node::new(exhibit.span, state)))
}
