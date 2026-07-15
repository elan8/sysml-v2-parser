use super::prelude::*;
use super::usage::{
    allocate_, connect_, interface_usage, part_ref_usage, part_usage, perform_action_decl,
    perform_usage, variant_usage,
};

/// Part def body: ';' or '{' PartDefBodyElement* '}'
pub(crate) fn part_def_body(input: Input<'_>) -> IResult<Input<'_>, PartDefBody> {
    let (input, _) = ws_and_comments(input)?;
    alt((
        map(tag(&b";"[..]), |_| PartDefBody::Semicolon),
        part_def_body_brace,
    ))
    .parse(input)
}

fn try_part_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PartDefBodyElement>> {
    match part_def_body_element(input) {
        Err(e)
            if starts_with_any_keyword(input.fragment(), PART_BODY_STARTERS)
                && starts_with_keyword(input.fragment(), b"part") =>
        {
            if let Ok((next, usage)) = part_usage(input) {
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
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        PART_BODY_STARTERS,
        "part definition body",
        "recovered_part_def_body_element",
    );
    if starts_with_any_keyword(start.fragment(), PART_BODY_STARTERS) {
        return node_from_to(
            start,
            end,
            PartDefBodyElement::Error(Node::new(crate::ast::Span::dummy(), recovery)),
        );
    }
    if matches!(
        recovery.code.as_str(),
        "missing_member_name"
            | "missing_type_reference"
            | "invalid_bare_identifier_in_action_body"
            | "invalid_bare_identifier_in_state_body"
            | "unexpected_keyword_in_scope"
            | "missing_semicolon"
            | "missing_body_or_semicolon"
            | "bare_feature_declaration_in_part_def"
            | "invalid_requirement_short_name_syntax"
    ) {
        return node_from_to(
            start,
            end,
            PartDefBodyElement::Error(Node::new(crate::ast::Span::dummy(), recovery)),
        );
    }
    let frag = start.fragment();
    let take = frag.len().min(80);
    let preview = String::from_utf8_lossy(&frag[..take]).trim().to_string();
    node_from_to(start, end, PartDefBodyElement::Other(preview))
}

fn part_def_body_brace(input: Input<'_>) -> IResult<Input<'_>, PartDefBody> {
    let (input, elements) = parse_structured_brace_members_with_skip(
        input,
        PART_BODY_STARTERS,
        "part definition body",
        "recovered_part_def_body_element",
        try_part_def_body_element,
        part_def_body_recovery,
        BraceMemberSkip::BodyElementRecover,
    )?;
    Ok((input, PartDefBody::Brace { elements }))
}

/// Build a `SubsettingRelationship` node from a target and the span of the whole clause,
/// mirroring `usage::subsetting_relationship_node` for the ad hoc `:>`/`:>>` trailing-clause
/// shapes parsed directly in this file (`exhibit_state`, `connection_usage_member`) rather than
/// through `usage::specialization_clauses`.
fn subsetting_relationship_node(
    span: crate::ast::Span,
    kind: crate::ast::SubsettingKind,
    target: String,
) -> Node<crate::ast::SubsettingRelationship> {
    Node::new(
        span.clone(),
        crate::ast::SubsettingRelationship {
            target,
            kind,
            span,
            is_implied: false,
        },
    )
}

/// Exhibit state usage: `exhibit state` name (`:` type)? (`;` or body)
pub(crate) fn exhibit_state(input: Input<'_>) -> IResult<Input<'_>, Node<ExhibitState>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"exhibit"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = tag(&b"state"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, type_name) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let (input, body) = crate::parser::state::state_def_body(input)?;
    let before_redefines = input;
    let (input, redefines) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>>"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let redefines = redefines.map(|target| {
        let span = crate::parser::span_from_to(before_redefines, input);
        subsetting_relationship_node(span, crate::ast::SubsettingKind::Redefines, target)
    });
    let input = if redefines.is_some() {
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
            ExhibitState {
                name: name_str,
                type_name,
                redefines,
                body,
            },
        ),
    ))
}

fn part_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<PartDefBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, elem) = alt((
        alt((
            map(doc_comment, PartDefBodyElement::Doc),
            map(comment_annotation, PartDefBodyElement::Comment),
            map(
                crate::parser::metadata_annotation::metadata_keyword_usage,
                PartDefBodyElement::MetadataKeywordUsage,
            ),
            map(
                crate::parser::metadata_annotation::metadata_annotation,
                PartDefBodyElement::MetadataAnnotation,
            ),
            map(annotation, PartDefBodyElement::Annotation),
            map(exhibit_state, PartDefBodyElement::ExhibitState),
            // `calc_def_required` must be tried before `calc_usage`: `calc_usage` has no guard
            // against a bare `def` keyword (same bug class as `flow_usage_member`/`port_usage`
            // above), so `calc def Foo {}` would otherwise misparse as `CalcUsage` named "def".
            map(calc_def_required, PartDefBodyElement::CalcDef),
            map(calc_usage, PartDefBodyElement::CalcUsage),
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
            map(part_ref_usage, PartDefBodyElement::Ref),
            map(|i| attribute_def(i, true), PartDefBodyElement::AttributeDef),
            map(attribute_usage, PartDefBodyElement::AttributeUsage),
            map(
                attribute_usage_shorthand,
                PartDefBodyElement::AttributeUsage,
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
            map(requirement_def, PartDefBodyElement::RequirementDef),
            map(occurrence_def, PartDefBodyElement::OccurrenceDef),
            map(metadata_usage, PartDefBodyElement::MetadataUsage),
            map(metadata_def, PartDefBodyElement::MetadataDef),
            map(item_def_required, PartDefBodyElement::ItemDef),
            map(item_usage, PartDefBodyElement::ItemUsage),
            map(
                crate::parser::occurrence_body::assert_constraint_member,
                PartDefBodyElement::AssertConstraint,
            ),
            map(satisfy, PartDefBodyElement::Satisfy),
            map(opaque_part_member_decl, PartDefBodyElement::OpaqueMember),
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
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn connection_usage_member(input: Input<'_>) -> IResult<Input<'_>, Node<ConnectionUsageMember>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
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
    let (input, type_name) = {
        let (peek, _) = ws_and_comments(input)?;
        if peek.fragment().starts_with(b":")
            && !peek.fragment().starts_with(b":>")
            && !peek.fragment().starts_with(b":>>")
        {
            let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
            let (input, parsed_type) = preceded(ws_and_comments, qualified_name).parse(input)?;
            (input, Some(parsed_type))
        } else {
            (input, None)
        }
    };
    let (input, body) = connection_member_body(input)?;
    let before_subsets = input;
    let (input, trailing_subsets) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let subsets = trailing_subsets.map(|target| {
        let span = crate::parser::span_from_to(before_subsets, input);
        subsetting_relationship_node(span, crate::ast::SubsettingKind::Subsets, target)
    });
    let before_redefines = input;
    let (input, trailing_redefines) = opt(preceded(
        preceded(ws_and_comments, tag(&b":>>"[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let redefines = trailing_redefines.map(|target| {
        let span = crate::parser::span_from_to(before_redefines, input);
        subsetting_relationship_node(span, crate::ast::SubsettingKind::Redefines, target)
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
                type_name,
                body,
                subsets,
                redefines,
            },
        ),
    ))
}

/// Permissive parser for library-style part members not yet modeled with dedicated AST nodes.
/// Examples: `abstract ref action ... { ... }`, `state monitor: StateKind { ... }`.
fn opaque_part_member_decl(input: Input<'_>) -> IResult<Input<'_>, Node<OpaqueMemberDecl>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    if !starts_with_any_keyword(
        input.fragment(),
        &[b"ref", b"action", b"state", b"port", b"connection"],
    ) {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, header_text) =
        crate::parser::lex::take_until_terminator(input, MEMBER_HEADER_UNTIL_BODY)?;
    let keyword = if starts_with_any_keyword(input.fragment(), &[b"ref"]) {
        "ref"
    } else if starts_with_any_keyword(input.fragment(), &[b"action"]) {
        "action"
    } else if starts_with_any_keyword(input.fragment(), &[b"state"]) {
        "state"
    } else if starts_with_any_keyword(input.fragment(), &[b"connection"]) {
        "connection"
    } else {
        "port"
    }
    .to_string();
    let name_str = header_text
        .split(|c: char| {
            c.is_whitespace() || c == ':' || c == '[' || c == ',' || c == '(' || c == ')'
        })
        .filter(|s| !s.is_empty())
        .find(|token| {
            !matches!(
                *token,
                "ref"
                    | "action"
                    | "state"
                    | "port"
                    | "connection"
                    | "part"
                    | "def"
                    | "private"
                    | "protected"
                    | "public"
            )
        })
        .unwrap_or("member")
        .to_string();
    let (input, _) = ws_and_comments(input)?;
    let (input, body) = crate::parser::attribute::attribute_body(input)?;
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
            OpaqueMemberDecl {
                keyword,
                name: name_str,
                text: header_text.trim().to_string(),
                body,
            },
        ),
    ))
}

#[cfg(test)]
mod par_002_nested_def_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
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
        let (_, package_node) =
            package_body_element(input(text)).expect("package-level state def");
        let (_, part_node) = part_def_body_element(input(text)).expect("nested state def");
        assert!(matches!(
            package_node.value,
            crate::ast::PackageBodyElement::StateDef(_)
        ));
        assert!(matches!(part_node.value, PartDefBodyElement::StateDef(_)));
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
        let (rest, node) = part_def_body_element(input("analysis def MyAnalysis;"))
            .expect("analysis case def");
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
}
