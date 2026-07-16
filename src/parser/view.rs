//! View, viewpoint, and rendering parsing (SysML v2 Clause 8.2.2.26).

use crate::ast::{
    ExposeMember, FilterMember, Membership, Node, ParseErrorNode, RenderingDef, RenderingDefBody,
    RenderingDefBodyElement, RenderingUsage, SatisfyViewMember, ViewBody, ViewBodyElement, ViewDef,
    ViewDefBody, ViewDefBodyElement, ViewRenderingUsage, ViewUsage, ViewpointDef, ViewpointUsage,
};
use crate::parser::definition_header::parse_feature_usage_header;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::interface::connect_body;
use crate::parser::lex::{
    capture_opaque_member, name, qualified_name, starts_with_any_keyword, visibility_prefix, ws1,
    ws_and_comments, VIEW_BODY_STARTERS, VIEW_DEF_BODY_STARTERS,
};
use crate::parser::requirement::{doc_comment, requirement_def_body};
use crate::parser::Input;
use crate::parser::{build_recovery_error_node_from_span, node_from_to};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, success};
use nom::sequence::preceded;
use nom::{IResult, Parser};

const VIEW_DEF_OPAQUE_STARTERS: &[&[u8]] = &[b"ref", b"abstract"];

fn view_def_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<ViewDefBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = alt((
        map(doc_comment, ViewDefBodyElement::Doc),
        map(
            crate::parser::metadata_annotation::metadata_annotation,
            ViewDefBodyElement::MetadataAnnotation,
        ),
        map(view_filter_member, ViewDefBodyElement::Filter),
        map(view_rendering_usage, ViewDefBodyElement::ViewRendering),
        map(
            |i| capture_opaque_member(i, VIEW_DEF_OPAQUE_STARTERS),
            ViewDefBodyElement::Other,
        ),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn view_filter_member(input: Input<'_>) -> IResult<Input<'_>, Node<FilterMember>> {
    crate::parser::package::filter_member(input)
}

fn view_rendering_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ViewRenderingUsage>> {
    let start = input;
    let (input, (visibility_span, visibility)) =
        preceded(ws_and_comments, visibility_prefix).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"render"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, header) = parse_feature_usage_header(input)?;
    let (input, body) = connect_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ViewRenderingUsage {
                name: name_str,
                type_name: header.type_name,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

fn view_def_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<ViewDefBodyElement> {
    if starts_with_any_keyword(start.fragment(), VIEW_DEF_BODY_STARTERS) {
        let recovery = build_recovery_error_node_from_span(
            start,
            end,
            VIEW_DEF_BODY_STARTERS,
            "view definition body",
            "recovered_view_def_body_element",
        );
        let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
        return node_from_to(start, end, ViewDefBodyElement::Error(node));
    }
    let preview = String::from_utf8_lossy(&start.fragment()[..start.fragment().len().min(60)])
        .trim()
        .to_string();
    node_from_to(start, end, ViewDefBodyElement::Other(preview))
}

fn view_def_body(input: Input<'_>) -> IResult<Input<'_>, ViewDefBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, ViewDefBody::Semicolon));
    }
    let (input, elements) = crate::parser::body::parse_structured_brace_members(
        input,
        VIEW_DEF_BODY_STARTERS,
        "view definition body",
        "recovered_view_def_body_element",
        view_def_body_element,
        view_def_body_recovery,
    )?;
    Ok((input, ViewDefBody::Brace { elements }))
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
        map(doc_comment, RenderingDefBodyElement::Doc),
        map(view_filter_member, RenderingDefBodyElement::Filter),
        map(view_rendering_usage, RenderingDefBodyElement::ViewRendering),
        map(
            |i| capture_opaque_member(i, RENDERING_DEF_OPAQUE_STARTERS),
            RenderingDefBodyElement::Other,
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
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, RenderingDefBody::Semicolon));
    }
    let (input, elements) = crate::parser::body::parse_structured_brace_members(
        input,
        VIEW_DEF_BODY_STARTERS,
        "rendering definition body",
        "recovered_rendering_def_body_element",
        rendering_def_body_element,
        rendering_def_body_recovery,
    )?;
    Ok((input, RenderingDefBody::Brace { elements }))
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
        map(doc_comment, ViewBodyElement::Doc),
        map(view_filter_member, ViewBodyElement::Filter),
        map(view_rendering_usage, ViewBodyElement::ViewRendering),
        map(expose_member, ViewBodyElement::Expose),
        map(satisfy_view_member, ViewBodyElement::Satisfy),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

/// Append zero or more `.` + qualified-name feature-chain segments (SysML §7.6.6).
fn parse_expose_feature_chain_suffix(
    mut input: Input<'_>,
    mut target: String,
) -> IResult<Input<'_>, String> {
    loop {
        let (next, _) = ws_and_comments(input)?;
        if next.fragment().first() != Some(&b'.') {
            return Ok((next, target));
        }
        let (next, _) = tag(&b"."[..]).parse(next)?;
        let (next, _) = ws_and_comments(next)?;
        let (next, segment) = qualified_name.parse(next)?;
        target.push('.');
        target.push_str(&segment);
        input = next;
    }
}

/// expose (MembershipImport | NamespaceImport) RelationshipBody
/// MembershipImport = QualifiedName (::**)?
/// NamespaceImport = QualifiedName :: * (::**)?
fn expose_member(input: Input<'_>) -> IResult<Input<'_>, Node<ExposeMember>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"expose"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, first) = qualified_name.parse(input)?;
    let (input, target) = alt((
        // ::*::** (try before ::* since * would consume first char of **)
        map(
            (
                preceded(ws_and_comments, tag(&b"::"[..])),
                preceded(ws_and_comments, tag(&b"*"[..])),
                preceded(ws_and_comments, tag(&b"::"[..])),
                preceded(ws_and_comments, tag(&b"**"[..])),
            ),
            |_| format!("{}::*::**", first),
        ),
        // ::** (try before ::*)
        map(
            (
                preceded(ws_and_comments, tag(&b"::"[..])),
                preceded(ws_and_comments, tag(&b"**"[..])),
            ),
            |_| format!("{}::**", first),
        ),
        // ::*
        map(
            (
                preceded(ws_and_comments, tag(&b"::"[..])),
                preceded(ws_and_comments, tag(&b"*"[..])),
            ),
            |_| format!("{}::*", first),
        ),
        // plain
        map(success(()), |_| first.clone()),
    ))
    .parse(input)?;
    let (input, target) = parse_expose_feature_chain_suffix(input, target)?;
    // Optional filter [ expr ] - skip content to reach body
    let (input, _) = nom::combinator::opt(nom::sequence::delimited(
        preceded(ws_and_comments, tag(&b"["[..])),
        nom::bytes::complete::take_until(&b"]"[..]),
        preceded(ws_and_comments, tag(&b"]"[..])),
    ))
    .parse(input)?;
    let (input, body) = connect_body(input)?;
    Ok((
        input,
        node_from_to(start, input, ExposeMember { target, body }),
    ))
}

/// satisfy QualifiedName RelationshipBody (simplified form in view body)
fn satisfy_view_member(input: Input<'_>) -> IResult<Input<'_>, Node<SatisfyViewMember>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"satisfy"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, viewpoint_ref) = qualified_name.parse(input)?;
    let (input, body) = connect_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            SatisfyViewMember {
                viewpoint_ref,
                body,
            },
        ),
    ))
}

fn view_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<ViewBodyElement> {
    if starts_with_any_keyword(start.fragment(), VIEW_BODY_STARTERS) {
        let recovery = build_recovery_error_node_from_span(
            start,
            end,
            VIEW_BODY_STARTERS,
            "view body",
            "recovered_view_body_element",
        );
        let node: Node<ParseErrorNode> = node_from_to(start, end, recovery);
        return node_from_to(start, end, ViewBodyElement::Error(node));
    }
    let preview = String::from_utf8_lossy(&start.fragment()[..start.fragment().len().min(60)])
        .trim()
        .to_string();
    node_from_to(start, end, ViewBodyElement::Other(preview))
}

fn view_body(input: Input<'_>) -> IResult<Input<'_>, ViewBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, ViewBody::Semicolon));
    }
    let (input, elements) = crate::parser::body::parse_structured_brace_members_with_skip(
        input,
        VIEW_BODY_STARTERS,
        "view body",
        "recovered_view_body_element",
        view_body_element,
        view_body_recovery,
        crate::parser::body::BraceMemberSkip::BodyElementRecover,
    )?;
    Ok((input, ViewBody::Brace { elements }))
}

pub(crate) fn view_usage(input: Input<'_>) -> IResult<Input<'_>, Node<ViewUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, _) = nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"view"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
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
                type_name: header.type_name,
                body,
                membership: Membership::feature(visibility, visibility_span),
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
                type_name: header.type_name.unwrap_or_default(),
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
    let (input, _) = nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"rendering"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, name_str) = name(input)?;
    let (input, header) = parse_feature_usage_header(input)?;
    let (input, body) = connect_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            RenderingUsage {
                name: name_str,
                type_name: header.type_name,
                body,
                membership: Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

#[cfg(test)]
mod expose_diagnostic_tests {
    use crate::ast::{PackageBody, PackageBodyElement, RootElement, ViewBody, ViewBodyElement};
    use crate::parse_with_diagnostics;

    #[test]
    fn expose_feature_chain_is_parsed_without_separator_diagnostic() {
        let input = "package Views { view structure: GeneralView { expose SurveillanceDrone.SurveillanceQuadrotorDrone; } }";
        let result = parse_with_diagnostics(input);
        assert!(
            result.is_ok(),
            "expected feature-chain expose to parse, got {:?}",
            result.errors
        );
        let root = result.root;
        let pkg = match &root.elements[0].value {
            RootElement::Package(p) => p,
            other => panic!("expected package, got {other:?}"),
        };
        let view_usage = match &pkg.value.body {
            PackageBody::Brace { elements } => match &elements[0].value {
                PackageBodyElement::ViewUsage(v) => v,
                other => panic!("expected view usage, got {other:?}"),
            },
            other => panic!("expected brace body, got {other:?}"),
        };
        let expose = match &view_usage.value.body {
            ViewBody::Brace { elements } => match &elements[0].value {
                ViewBodyElement::Expose(e) => e,
                other => panic!("expected expose member, got {other:?}"),
            },
            other => panic!("expected view body, got {other:?}"),
        };
        assert_eq!(
            expose.value.target, "SurveillanceDrone.SurveillanceQuadrotorDrone",
            "feature-chain segments should be preserved in expose target"
        );
    }
}

#[cfg(test)]
mod membership_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
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
