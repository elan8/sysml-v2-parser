//! View, viewpoint, and rendering parsing (SysML v2 Clause 8.2.2.26).

use crate::ast::{
    ExposeMember, FilterMember, ImportTarget, Membership, Node, ParseErrorNode, RenderingDef,
    RenderingDefBody, RenderingDefBodyElement, RenderingUsage, RenderingUsageBody,
    RenderingUsageBodyElement, ViewBody, ViewBodyElement, ViewDef, ViewDefBody, ViewDefBodyElement,
    ViewRenderingUsage, ViewUsage, ViewpointDef, ViewpointUsage,
};
use crate::parser::definition_header::parse_feature_usage_header;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::import::import_shape;
use crate::parser::lex::{
    name, reference_path, visibility_prefix, ws1, ws_and_comments, VIEW_BODY_STARTERS,
    VIEW_DEF_BODY_STARTERS,
};
use crate::parser::requirement::requirement_def_body;
use crate::parser::usage::{multiplicity_node, prefix_redefinition_target};
use crate::parser::Input;
use crate::parser::{build_recovery_error_node_from_span, node_from_to, span_from_to};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::{IResult, Parser};

const VIEW_DEF_OPAQUE_STARTERS: &[&[u8]] = &[b"ref", b"abstract"];

fn view_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<ViewDefBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = alt((
        map(
            crate::parser::body::annotating_member,
            ViewDefBodyElement::Annotating,
        ),
        // Both `#` productions: the `ExtendedUsage` member spelling (which owns a `;`/`{}`
        // body) is tried before the `PrefixMetadataMember` spelling, which owns no body and
        // leaves the prefixed declaration for the next member iteration.
        alt((
            map(
                crate::parser::metadata_annotation::metadata_keyword_usage,
                ViewDefBodyElement::MetadataKeywordUsage,
            ),
            map(
                crate::parser::metadata_annotation::metadata_keyword_prefix,
                ViewDefBodyElement::MetadataKeywordUsage,
            ),
        )),
        map(
            crate::parser::connector::ref_decl,
            ViewDefBodyElement::RefDecl,
        ),
        map(view_filter_member, ViewDefBodyElement::Filter),
        map(view_rendering_usage, ViewDefBodyElement::ViewRendering),
        map(viewpoint_usage, ViewDefBodyElement::ViewpointUsage),
        map(crate::parser::requirement::satisfy, |n| {
            ViewDefBodyElement::Satisfy(Box::new(n))
        }),
        map(
            |i| {
                crate::parser::recovery::unsupported_member(
                    i,
                    VIEW_DEF_OPAQUE_STARTERS,
                    "view definition body",
                )
            },
            ViewDefBodyElement::Unsupported,
        ),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn view_filter_member(input: Input<'_>) -> IResult<Input<'_>, Node<FilterMember>> {
    crate::parser::package::filter_member(input)
}

/// Body of a `render`/`rendering` usage (BNF `UsageBody`, Clause 8.2.2.26.1): a nested `view`
/// usage member -- most notably a `columnView` redefinition of `asElementTable` (`view :>>
/// columnView[N] { render ...; }`, confirmed real usage in `sysml-v2-release/sysml/src/training/
/// 42. Views/Views Example.sysml` and `.../validation/11-View and Viewpoint/11a-View-Viewpoint.
/// sysml`) -- or a doc comment. Any other content falls through to the shared brace-member
/// recovery path (`parse_structured_brace_members`) as an `Error` node, same as every other
/// structured body in this module; scoped to what's confirmed needed rather than guessing at a
/// wider `UsageBody` grammar with no concrete real-usage backing.
fn rendering_usage_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<RenderingUsageBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = alt((
        map(
            crate::parser::body::annotating_member,
            RenderingUsageBodyElement::Annotating,
        ),
        map(view_usage, |n| {
            RenderingUsageBodyElement::ViewUsage(Box::new(n))
        }),
        // Nested `rendering` usage, e.g. the anonymous `rendering :>> subrenderings[0..*] =
        // columnView.viewRendering;` inside `asElementTable` (Systems Library `Views.sysml`).
        map(rendering_usage, |n| {
            RenderingUsageBodyElement::Rendering(Box::new(n))
        }),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn rendering_usage_body_recovery(
    start: Input<'_>,
    end: Input<'_>,
) -> Node<RenderingUsageBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        VIEW_DEF_BODY_STARTERS,
        "rendering usage body",
        "recovered_rendering_usage_body_element",
    );
    node_from_to(
        start,
        end,
        RenderingUsageBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

fn rendering_usage_body(input: Input<'_>) -> IResult<Input<'_>, RenderingUsageBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let semicolon_start = input;
        let (input, _) = tag(&b";"[..]).parse(semicolon_start)?;
        return Ok((
            input,
            RenderingUsageBody::Semicolon {
                semicolon_span: crate::parser::span::span_from_to(semicolon_start, input),
            },
        ));
    }
    let (input, members) = crate::parser::body::parse_structured_brace_members(
        input,
        VIEW_DEF_BODY_STARTERS,
        "rendering usage body",
        "recovered_rendering_usage_body_element",
        rendering_usage_body_element,
        rendering_usage_body_recovery,
    )?;
    Ok((input, members.into_body()))
}

fn view_rendering_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ViewRenderingUsage>> {
    let start = input;
    let (input, (visibility_span, visibility)) =
        preceded(ws_and_comments, visibility_prefix).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"render"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    // BNF `ViewRenderingUsage`'s second alternative: `('rendering' | UsageExtensionKeyword+)
    // Usage` -- explicit typed usage declaration, vs. the first alternative's bare reference-
    // subsetting shorthand (`render r;`). Real usage: `render rendering r1: R[0..1];` (Simple
    // Tests/ViewTest.sysml:32).
    let (input, _) = opt(preceded(tag(&b"rendering"[..]), ws1)).parse(input)?;
    let (input, name_str) = name(input)?;
    let (input, header) = parse_feature_usage_header(input)?;
    let (input, body) = rendering_usage_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ViewRenderingUsage {
                name: name_str,
                type_name: header.type_reference,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

fn view_def_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<ViewDefBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        VIEW_DEF_BODY_STARTERS,
        "view definition body",
        "recovered_view_def_body_element",
    );
    let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
    node_from_to(start, end, ViewDefBodyElement::Error(node))
}

fn view_def_body(input: Input<'_>) -> IResult<Input<'_>, ViewDefBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let semicolon_start = input;
        let (input, _) = tag(&b";"[..]).parse(semicolon_start)?;
        return Ok((
            input,
            ViewDefBody::Semicolon {
                semicolon_span: crate::parser::span::span_from_to(semicolon_start, input),
            },
        ));
    }
    let (input, members) = crate::parser::body::parse_structured_brace_members(
        input,
        VIEW_DEF_BODY_STARTERS,
        "view definition body",
        "recovered_view_def_body_element",
        view_def_body_element,
        view_def_body_recovery,
    )?;
    Ok((input, members.into_body()))
}

pub(crate) fn view_def(input: Input<'_>) -> IResult<Input<'_>, Node<ViewDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"view")
            .def_required()
            .with_captured_visibility(),
    )?;
    let (input, body) = view_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ViewDef {
                identification: prefix.identification,
                is_abstract: prefix.is_abstract,
                specializes: prefix.specializes,
                body,
                membership: Membership::owning(prefix.visibility, prefix.visibility_span),
            },
        ),
    ))
}

pub(crate) fn viewpoint_def(input: Input<'_>) -> IResult<Input<'_>, Node<ViewpointDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"viewpoint")
            .def_required()
            .with_captured_visibility(),
    )?;
    let (input, body) = requirement_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ViewpointDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
                membership: Membership::owning(prefix.visibility, prefix.visibility_span),
            },
        ),
    ))
}

const RENDERING_DEF_OPAQUE_STARTERS: &[&[u8]] = &[b"ref", b"abstract"];

fn rendering_def_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<RenderingDefBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = alt((
        map(
            crate::parser::body::annotating_member,
            RenderingDefBodyElement::Annotating,
        ),
        map(
            crate::parser::connector::ref_decl,
            RenderingDefBodyElement::RefDecl,
        ),
        map(view_filter_member, RenderingDefBodyElement::Filter),
        map(view_rendering_usage, RenderingDefBodyElement::ViewRendering),
        map(
            |i| {
                crate::parser::recovery::unsupported_member(
                    i,
                    RENDERING_DEF_OPAQUE_STARTERS,
                    "rendering definition body",
                )
            },
            RenderingDefBodyElement::Unsupported,
        ),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn rendering_def_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<RenderingDefBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        VIEW_DEF_BODY_STARTERS,
        "rendering definition body",
        "recovered_rendering_def_body_element",
    );
    node_from_to(
        start,
        end,
        RenderingDefBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

fn rendering_def_body(input: Input<'_>) -> IResult<Input<'_>, RenderingDefBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let semicolon_start = input;
        let (input, _) = tag(&b";"[..]).parse(semicolon_start)?;
        return Ok((
            input,
            RenderingDefBody::Semicolon {
                semicolon_span: crate::parser::span::span_from_to(semicolon_start, input),
            },
        ));
    }
    let (input, members) = crate::parser::body::parse_structured_brace_members(
        input,
        VIEW_DEF_BODY_STARTERS,
        "rendering definition body",
        "recovered_rendering_def_body_element",
        rendering_def_body_element,
        rendering_def_body_recovery,
    )?;
    Ok((input, members.into_body()))
}

pub(crate) fn rendering_def(input: Input<'_>) -> IResult<Input<'_>, Node<RenderingDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"rendering")
            .def_required()
            .with_captured_visibility(),
    )?;
    let (input, body) = rendering_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            RenderingDef {
                identification: prefix.identification,
                is_abstract: prefix.is_abstract,
                specializes: prefix.specializes,
                body,
                membership: Membership::owning(prefix.visibility, prefix.visibility_span),
            },
        ),
    ))
}

fn view_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<ViewBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = alt((
        map(
            crate::parser::body::annotating_member,
            ViewBodyElement::Annotating,
        ),
        map(crate::parser::connector::ref_decl, ViewBodyElement::RefDecl),
        map(view_filter_member, ViewBodyElement::Filter),
        map(view_rendering_usage, ViewBodyElement::ViewRendering),
        map(expose_member, ViewBodyElement::Expose),
        // `ViewBodyItem -> DefinitionBodyItem -> ... -> SatisfyRequirementUsage`: one satisfy
        // production, dispatched here through the same parser every other body scope uses.
        map(crate::parser::requirement::satisfy, |n| {
            ViewBodyElement::Satisfy(Box::new(n))
        }),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

/// expose (MembershipImport | NamespaceImport) RelationshipBody
/// MembershipImport = QualifiedName (::**)?
/// NamespaceImport = QualifiedName :: * (::**)?
fn expose_member(input: Input<'_>) -> IResult<Input<'_>, Node<ExposeMember>> {
    crate::parser::span::reference_transaction(input, expose_member_inner)
}

fn expose_member_inner(input: Input<'_>) -> IResult<Input<'_>, Node<ExposeMember>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"expose"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let target_start = input;
    let (after_reference, reference) = reference_path(input)?;
    let (input, shape) = import_shape(after_reference)?;
    // The target ends at its last authored token, not wherever shape parsing stopped -- looking
    // for an absent `::*` consumes the trivia before a braced body. `import_` documents the same
    // hazard; `expose` shared the bug and only stopped hiding it once the body became a real
    // `Body` whose deserialization checks the target span it sits beside.
    let mut target_span = span_from_to(target_start, input);
    target_span.len =
        crate::parser::import::import_target_end(&shape, after_reference.location_offset())
            .saturating_sub(target_span.offset);
    let (input, body) = crate::parser::body::relationship_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ExposeMember {
                target: ImportTarget {
                    span: target_span,
                    all_span: None,
                    reference,
                    shape,
                },
                body,
            },
        ),
    ))
}

fn view_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<ViewBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        VIEW_BODY_STARTERS,
        "view body",
        "recovered_view_body_element",
    );
    let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
    node_from_to(start, end, ViewBodyElement::Error(node))
}

fn view_body(input: Input<'_>) -> IResult<Input<'_>, ViewBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let semicolon_start = input;
        let (input, _) = tag(&b";"[..]).parse(semicolon_start)?;
        return Ok((
            input,
            ViewBody::Semicolon {
                semicolon_span: crate::parser::span::span_from_to(semicolon_start, input),
            },
        ));
    }
    let (input, members) = crate::parser::body::parse_structured_brace_members_with_skip(
        input,
        VIEW_BODY_STARTERS,
        "view body",
        "recovered_view_body_element",
        view_body_element,
        view_body_recovery,
        crate::parser::body::BraceMemberSkip::BodyElementRecover,
    )?;
    Ok((input, members.into_body()))
}

pub(crate) fn view_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ViewUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, _) = nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"view"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    // Anonymous redefinition form (BNF `ViewUsage`'s `UsageDeclaration?` legally omits the name
    // in favor of a leading `:>>` target, same shape `PartUsage`'s `part_usage_redefines_only`
    // already handles) -- e.g. `view :>> columnView[1] { render asTextualNotation; }`, confirmed
    // real usage in `sysml-v2-release/sysml/src/training/42. Views/Views Example.sysml` and
    // `.../validation/11-View and Viewpoint/11a-View-Viewpoint.sysml`. Peek before committing to
    // the named path, mirroring `part_usage`'s own dispatch.
    let (peek, _) = ws_and_comments(input)?;
    if peek.fragment().starts_with(b":>>") {
        let (input, mut usage) = view_usage_redefines_only(start, input)?;
        usage.value.membership = Membership::feature(visibility, visibility_span);
        return Ok((input, usage));
    }
    let (input, name_str) = name(input)?;
    let (input, header) = parse_feature_usage_header(input)?;
    let (input, body) = view_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ViewUsage {
                name: name_str,
                type_name: header.type_reference,
                subsets: header.subsets,
                redefines: header.redefines,
                multiplicity: header.multiplicity,
                ordered: header.ordered,
                nonunique: header.nonunique,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

/// Anonymous `view :>> name[multiplicity]? ViewBody` redefinition form -- see [`view_usage`]'s
/// doc comment. Mirrors `part_usage_redefines_only`'s shape exactly: redefinition target,
/// optional multiplicity, then straight to the body (no `: Type` header -- the type comes from
/// the redefined feature, not a fresh typing clause).
fn view_usage_redefines_only<'a>(
    start: Input<'a>,
    input: Input<'a>,
) -> IResult<Input<'a>, Node<ViewUsage>> {
    let (input, (_, redefines_target)) = prefix_redefinition_target(input)?;
    let (input, multiplicity_opt) = opt(multiplicity_node).parse(input)?;
    let (input, (ordered, nonunique)) = crate::parser::usage::usage_feature_modifier_flags(input)?;
    let (input, body) = view_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ViewUsage {
                name: String::new(),
                type_name: None,
                subsets: None,
                redefines: Some(redefines_target),
                multiplicity: multiplicity_opt,
                ordered,
                nonunique,
                body,
                membership: Membership::feature(None, crate::ast::Span::dummy()),
            },
        ),
    ))
}

pub(crate) fn viewpoint_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ViewpointUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, _) = nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"viewpoint"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, header) = parse_feature_usage_header(input)?;
    let (input, body) = requirement_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ViewpointUsage {
                name: name_str,
                type_name: header.type_reference,
                subsets: header.subsets,
                redefines: header.redefines,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

pub(crate) fn rendering_usage(input: Input<'_>) -> IResult<Input<'_>, Node<RenderingUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, is_abstract) =
        nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"rendering"[..]).parse(input)?;
    // The declaration name is optional: the anonymous redefinition form `rendering :>>
    // subrenderings[0..*] = columnView.viewRendering;` (Systems Library `Views.sysml`) goes
    // straight to its specialization clause.
    let (after_gap, _) = ws_and_comments(input)?;
    let (input, name_str) = if after_gap.fragment().starts_with(b":")
        || after_gap.fragment().starts_with(b"[")
        || after_gap.fragment().starts_with(b"{")
        || after_gap.fragment().starts_with(b";")
    {
        (after_gap, String::new())
    } else {
        let (input, _) = ws1(input)?;
        name(input)?
    };
    // Header clauses, each retained: leading `:>>` redefinition (the anonymous form), typing,
    // multiplicity (before or after the typing), and a `:>` subsets clause -- `asTreeDiagram :
    // GraphicalRendering[1] :> renderings { ... }` (Systems Library `Views.sysml`).
    let (input, leading_redefines) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::redefinition,
    ))
    .parse(input)?;
    let (input, leading_multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    let (input, type_result) = crate::parser::usage::optional_typings(input)?;
    let type_name = type_result.and_then(|(_, _, targets, _)| targets.first().copied());
    let (input, trailing_multiplicity) = if leading_multiplicity.is_none() {
        opt(preceded(
            ws_and_comments,
            crate::parser::usage::multiplicity_node,
        ))
        .parse(input)?
    } else {
        (input, None)
    };
    let (input, (ordered, nonunique)) = crate::parser::usage::usage_feature_modifier_flags(input)?;
    let (input, redefines) = if leading_redefines.is_none() {
        opt(preceded(
            ws_and_comments,
            crate::parser::usage::redefinition,
        ))
        .parse(input)?
    } else {
        (input, leading_redefines)
    };
    let (input, subsets) =
        opt(preceded(ws_and_comments, crate::parser::usage::subsetting)).parse(input)?;
    let subsets = subsets.map(|(target, _value)| target);
    // Optional value clause: `= columnView.viewRendering` (Systems Library `Views.sysml`).
    let (input, value) = opt(crate::parser::feature_value::feature_value_part).parse(input)?;
    let (input, body) = rendering_usage_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            RenderingUsage {
                is_abstract: is_abstract.is_some(),
                name: name_str,
                type_name,
                multiplicity: leading_multiplicity.or(trailing_multiplicity),
                ordered,
                nonunique,
                subsets,
                redefines,
                value,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

#[cfg(test)]
mod expose_diagnostic_tests {
    use crate::ast::{
        ExposeMember, ImportShape, PackageBody, PackageBodyElement, ParsedDocument, RootElement,
        ViewBody, ViewBodyElement,
    };
    use crate::parse_with_diagnostics;

    fn expose_of(document: &ParsedDocument) -> &ExposeMember {
        let root = &document.root;
        let pkg = match &root.elements[0].value {
            RootElement::Package(p) => p,
            other => panic!("expected package, got {other:?}"),
        };
        let view_usage = match &pkg.value.body {
            PackageBody::Brace { elements, .. } => match &elements[0].value {
                PackageBodyElement::ViewUsage(v) => v,
                other => panic!("expected view usage, got {other:?}"),
            },
            other => panic!("expected brace body, got {other:?}"),
        };
        match &view_usage.value.body {
            ViewBody::Brace { elements, .. } => match &elements[0].value {
                ViewBodyElement::Expose(e) => &e.value,
                other => panic!("expected expose member, got {other:?}"),
            },
            other => panic!("expected view body, got {other:?}"),
        }
    }

    #[test]
    fn expose_feature_chain_is_parsed_without_separator_diagnostic() {
        let input = "package Views { view structure: GeneralView { expose SurveillanceDrone.SurveillanceQuadrotorDrone; } }";
        let result = parse_with_diagnostics(input);
        assert!(
            result.is_ok(),
            "expected feature-chain expose to parse, got {:?}",
            result.errors
        );
        let expose = expose_of(&result.document);
        let reference = result
            .document
            .qualified_reference(expose.target.reference)
            .expect("expose reference");
        assert_eq!(
            reference.authored_text(),
            "SurveillanceDrone.SurveillanceQuadrotorDrone",
            "feature-chain segments should be preserved in expose target"
        );
        assert!(matches!(
            expose.target.shape,
            ImportShape::Membership {
                recursive_suffix: None
            }
        ));
    }

    #[test]
    fn expose_plain_target_is_a_membership_import() {
        let result =
            parse_with_diagnostics("package Views { view v : GeneralView { expose vehicle; } }");
        assert!(result.is_ok(), "expected parse, got {:?}", result.errors);
        assert!(matches!(
            expose_of(&result.document).target.shape,
            ImportShape::Membership {
                recursive_suffix: None
            }
        ));
    }

    #[test]
    fn expose_wildcard_target_is_a_namespace_import() {
        let result =
            parse_with_diagnostics("package Views { view v : GeneralView { expose vehicle::*; } }");
        assert!(result.is_ok(), "expected parse, got {:?}", result.errors);
        assert!(matches!(
            expose_of(&result.document).target.shape,
            ImportShape::Namespace {
                recursive_suffix: None,
                combined_recursive_suffix_span: None,
                ..
            }
        ));
    }

    #[test]
    fn expose_recursive_membership_import() {
        let result = parse_with_diagnostics(
            "package Views { view v : GeneralView { expose vehicle::**; } }",
        );
        assert!(result.is_ok(), "expected parse, got {:?}", result.errors);
        assert!(matches!(
            expose_of(&result.document).target.shape,
            ImportShape::Membership {
                recursive_suffix: Some(_)
            }
        ));
    }

    #[test]
    fn expose_recursive_namespace_import() {
        let result = parse_with_diagnostics(
            "package Views { view v : GeneralView { expose vehicle::*::**; } }",
        );
        assert!(result.is_ok(), "expected parse, got {:?}", result.errors);
        assert!(matches!(
            expose_of(&result.document).target.shape,
            ImportShape::Namespace {
                recursive_suffix: Some(_),
                combined_recursive_suffix_span: Some(_),
                ..
            }
        ));
    }

    #[test]
    fn expose_filter_retains_typed_expressions_and_brace_body() {
        let result = parse_with_diagnostics(
            "package Views { view v : GeneralView { expose vehicle[x][y] {} } }",
        );
        assert!(result.is_ok(), "expected parse, got {:?}", result.errors);
        let expose = expose_of(&result.document);
        match &expose.target.shape {
            ImportShape::Filter {
                recursive_suffix,
                members,
            } => {
                assert!(recursive_suffix.is_none());
                assert_eq!(members.len(), 2);
            }
            other => panic!("expected filter shape, got {other:?}"),
        }
        assert!(expose.body.braced_elements().is_some());
    }

    #[test]
    fn view_type_and_satisfy_target_are_source_backed_references() {
        let result = parse_with_diagnostics(
            "package Views { view v : $::Views::General { satisfy Viewpoints::VP; } }",
        );
        assert!(result.is_ok(), "expected parse, got {:?}", result.errors);
        let package = match &result.document.root.elements[0].value {
            RootElement::Package(package) => &package.value,
            other => panic!("expected package, got {other:?}"),
        };
        let view = match &package.body {
            PackageBody::Brace { elements, .. } => match &elements[0].value {
                PackageBodyElement::ViewUsage(view) => &view.value,
                other => panic!("expected view usage, got {other:?}"),
            },
            other => panic!("expected package body, got {other:?}"),
        };
        let view_type = result
            .document
            .qualified_reference(view.type_name.expect("view type"))
            .expect("source-backed view type");
        assert_eq!(view_type.authored_text(), "$::Views::General");
        assert!(view_type.metadata.is_absolute);
        let satisfy = match &view.body {
            ViewBody::Brace { elements, .. } => match &elements[0].value {
                ViewBodyElement::Satisfy(satisfy) => &satisfy.value,
                other => panic!("expected satisfy member, got {other:?}"),
            },
            other => panic!("expected view body, got {other:?}"),
        };
        let reference = match &satisfy.requirement {
            crate::ast::SatisfiedRequirement::Reference { reference } => *reference,
            other => panic!("expected the reference alternative, got {other:?}"),
        };
        let target = result
            .document
            .qualified_reference(reference)
            .expect("source-backed viewpoint target");
        assert_eq!(target.authored_text(), "Viewpoints::VP");
        assert_eq!(target.segments.len(), 2);
        assert_eq!(
            target.segments[1].separator_before,
            Some(crate::ast::ReferenceSeparator::ColonColon)
        );
    }
}

#[cfg(test)]
mod membership_tests {
    use super::*;
    use crate::parser::span::ParseContext;

    fn input(text: &str) -> Input<'_> {
        let context = Box::leak(Box::new(ParseContext::new()));
        context.input(text.as_bytes())
    }

    // --- parser work item 4b (final sweep): Membership on the view family (7 structs) ---

    #[test]
    fn view_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = view_def(input("private view def V1;")).expect("view def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Private)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }

    #[test]
    fn view_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = view_def(input("view def V1;")).expect("view def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn viewpoint_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) =
            viewpoint_def(input("protected viewpoint def VP1;")).expect("viewpoint def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Protected)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }

    #[test]
    fn rendering_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = rendering_def(input("public rendering def R1;")).expect("rendering def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }

    #[test]
    fn view_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) = view_usage(input("private view v1 : V1;")).expect("view usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Private)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn view_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = view_usage(input("view v1 : V1;")).expect("view usage");
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn viewpoint_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) =
            viewpoint_usage(input("protected viewpoint vp1 : VP1;")).expect("viewpoint usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Protected)
        );
    }

    #[test]
    fn rendering_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) =
            rendering_usage(input("public rendering r1 : R1;")).expect("rendering usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
    }

    #[test]
    fn view_rendering_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) =
            view_rendering_usage(input("protected render r1 : R1;")).expect("view rendering usage");
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
    fn view_rendering_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) =
            view_rendering_usage(input("render r1 : R1;")).expect("view rendering usage");
        assert_eq!(node.value.membership.visibility, None);
    }
}

#[cfg(test)]
mod column_view_tests {
    use super::*;
    use crate::parser::span::ParseContext;

    fn input(text: &str) -> Input<'_> {
        let context = Box::leak(Box::new(ParseContext::new()));
        context.input(text.as_bytes())
    }

    // Real usage confirmed in sysml-v2-release/sysml/src/training/42. Views/Views Example.sysml
    // and .../validation/11-View and Viewpoint/11a-View-Viewpoint.sysml:
    //   rendering asTextualNotationTable :> asElementTable {
    //       view :>> columnView[1] {
    //           render asTextualNotation;
    //       }
    //   }

    #[test]
    fn view_usage_accepts_anonymous_redefinition_form() {
        let (rest, node) = view_usage(input(
            "view :>> columnView[1] { render asTextualNotation; }",
        ))
        .expect("view usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "");
        assert!(node.value.redefines.is_some());
        assert!(node.value.multiplicity.is_some());
    }

    #[test]
    fn rendering_usage_captures_nested_column_view_redefinition() {
        let (rest, node) = rendering_usage(input(
            "rendering asTextualNotationTable :> asElementTable { view :>> columnView[1] { render asTextualNotation; } }",
        ))
        .expect("rendering usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let RenderingUsageBody::Brace { elements, .. } = &node.value.body else {
            panic!("expected brace body, got {:?}", node.value.body);
        };
        assert_eq!(elements.len(), 1);
        let RenderingUsageBodyElement::ViewUsage(column_view) = &elements[0].value else {
            panic!("expected a nested view usage, got {:?}", elements[0].value);
        };
        assert_eq!(column_view.value.name, "");
        assert!(column_view.value.redefines.is_some());
        let ViewBody::Brace {
            elements: nested_elements,
            ..
        } = &column_view.value.body
        else {
            panic!("expected brace body on nested columnView");
        };
        let has_nested_render = nested_elements
            .iter()
            .any(|e| matches!(e.value, ViewBodyElement::ViewRendering(_)));
        assert!(has_nested_render, "expected a nested render binding");
    }

    #[test]
    fn view_rendering_usage_body_accepts_column_view_redefinition() {
        // Inline `render asElementTable { view :>> columnView[1] { render asTextualNotation; } }`
        // form (the other real fixture, used directly on a view usage's `render` binding).
        let (rest, node) = view_rendering_usage(input(
            "render asElementTable { view :>> columnView[1] { render asTextualNotation; } }",
        ))
        .expect("view rendering usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let RenderingUsageBody::Brace { elements, .. } = &node.value.body else {
            panic!("expected brace body, got {:?}", node.value.body);
        };
        assert_eq!(elements.len(), 1);
        assert!(matches!(
            elements[0].value,
            RenderingUsageBodyElement::ViewUsage(_)
        ));
    }
}
