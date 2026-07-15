//! Package and root namespace parsing.

use crate::ast::{
    ClassifierDecl, ExtendedLibraryDecl, FeatureDecl, FilterMember, KermlFeatureDecl,
    KermlSemanticDecl, LibraryPackage, NamespaceDecl, Node, Package, PackageBody,
    PackageBodyElement, RootElement, RootNamespace, Visibility,
};
use crate::parser::action::{action_def, action_usage};
use crate::parser::alias::alias_def;
use crate::parser::allocation::{allocate_usage, allocation_def, allocation_usage};
use crate::parser::attribute::{attribute_def, attribute_usage};
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::case::{
    analysis_case_def, analysis_case_usage, case_def, case_usage, verification_case_def,
    verification_case_usage,
};
use crate::parser::connection::connection_def;
use crate::parser::constraint::{calc_def, constraint_def};
use crate::parser::dependency::dependency;
use crate::parser::enumeration::{enum_def, enum_usage};
use crate::parser::expr::expression;
use crate::parser::flow::{flow_def, flow_usage};
use crate::parser::import::import_;
use crate::parser::individual::individual_def;
use crate::parser::interface::interface_def;
use crate::parser::item::{item_def, item_usage};
use crate::parser::lex::{
    name, qualified_name, recover_body_element, skip_statement_or_block, starts_with_any_keyword,
    starts_with_keyword, ws1, ws_and_comments, PACKAGE_BODY_STARTERS,
};
use crate::parser::metadata::{metadata_def, metadata_usage};
use crate::parser::node_from_to;
use crate::parser::occurrence::{
    individual_usage, occurrence_def, occurrence_usage, snapshot_usage, timeslice_usage,
};
use crate::parser::part::{part_def_or_usage, PartDefOrUsage};
use crate::parser::port::{port_def, port_usage};
use crate::parser::requirement::{
    comment_annotation, concern_usage, doc_comment, requirement_def, requirement_usage, satisfy,
    textual_representation,
};
use crate::parser::state::state_def;
use crate::parser::state::state_usage;
use crate::parser::usecase::{actor_decl, use_case_def, use_case_usage};
use crate::parser::view::{
    rendering_def, rendering_usage, view_def, view_usage, viewpoint_def, viewpoint_usage,
};
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use nom::Parser;

/// Keyword "package" with following whitespace.
fn keyword_package(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) = tag(&b"package"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    Ok((input, ()))
}

fn required_package_identification(
    input: Input<'_>,
) -> IResult<Input<'_>, crate::ast::Identification> {
    let (input, short_name) = opt(delimited(
        preceded(ws_and_comments, tag(&b"<"[..])),
        preceded(ws_and_comments, name),
        preceded(ws_and_comments, tag(&b">"[..])),
    ))
    .parse(input)?;
    let (input, decl_name) = opt(preceded(ws_and_comments, qualified_name)).parse(input)?;
    if short_name.is_some() || decl_name.is_some() {
        Ok((
            input,
            crate::ast::Identification {
                short_name,
                name: decl_name,
            },
        ))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

/// [standard] library package Identification PackageBody (BNF LibraryPackage)
fn library_package_(input: Input<'_>) -> IResult<Input<'_>, Node<LibraryPackage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    // Accept both `standard library package` (current SysML v2 stdlib)
    // and legacy `library standard package`.
    let (input, is_standard) = if input.fragment().starts_with(b"standard") {
        let (input, _) = tag(&b"standard"[..]).parse(input)?;
        let (input, _) = ws1(input)?;
        let (input, _) = tag(&b"library"[..]).parse(input)?;
        let (input, _) = ws1(input)?;
        (input, true)
    } else {
        let (input, _) = tag(&b"library"[..]).parse(input)?;
        let (input, _) = ws1(input)?;
        let (input, is_standard) = opt(preceded(tag(&b"standard"[..]), ws1))
            .parse(input)
            .map(|(i, o)| (i, o.is_some()))?;
        (input, is_standard)
    };
    let (input, _) = keyword_package(input)?;
    let (input, identification) = required_package_identification(input)?;
    let (input, body) = package_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            LibraryPackage {
                is_standard,
                identification,
                body,
            },
        ),
    ))
}

/// package Identification PackageBody
fn package_(input: Input<'_>) -> IResult<Input<'_>, Node<Package>> {
    let start = input;
    let (input, _) = keyword_package(input)?;
    let (input, identification) = required_package_identification(input)?;
    let (input, body) = package_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Package {
                identification,
                body,
            },
        ),
    ))
}

/// KerML namespace Identification NamespaceBody
fn namespace_decl(input: Input<'_>) -> IResult<Input<'_>, Node<NamespaceDecl>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"namespace"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, identification) = required_package_identification(input)?;
    let (input, body) = package_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            NamespaceDecl {
                identification,
                body,
            },
        ),
    ))
}

/// One root-level element: import, package, or namespace (BNF PackageBodyElement* at root).
pub(crate) fn root_element(input: Input<'_>) -> IResult<Input<'_>, Node<RootElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, elem) = alt((
        map(import_, RootElement::Import),
        map(namespace_decl, RootElement::Namespace),
        map(library_package_, RootElement::LibraryPackage),
        map(package_, RootElement::Package),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

/// PackageBody: ';' | '{' PackageBodyElement* '}'
/// Brace form is tried first so that ws before '{' is not consumed by the semicolon branch.
pub(crate) fn package_body(input: Input<'_>) -> IResult<Input<'_>, PackageBody> {
    alt((
        package_body_brace,
        map(preceded(ws_and_comments, tag(&b";"[..])), |_| {
            PackageBody::Semicolon
        }),
    ))
    .parse(input)
}

fn package_body_element_fallback(input: Input<'_>) -> IResult<Input<'_>, Node<PackageBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let frag = input.fragment();

    if starts_with_keyword(frag, b"part")
        || starts_with_keyword(frag, b"abstract")
        || starts_with_keyword(frag, b"variation")
    {
        let start = input;
        let (input, parsed) = part_def_or_usage(input)?;
        let value = match parsed {
            PartDefOrUsage::Def(n) => PackageBodyElement::PartDef(n),
            PartDefOrUsage::Usage(n) => PackageBodyElement::PartUsage(n),
        };
        return Ok((input, node_from_to(start, input, value)));
    }

    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Tag,
    )))
}

fn modeled_decl_text(start: Input<'_>, end: Input<'_>) -> String {
    let delta = end
        .location_offset()
        .saturating_sub(start.location_offset());
    let bytes = start.fragment();
    let take = delta.min(bytes.len());
    String::from_utf8_lossy(&bytes[..take]).trim().to_string()
}

fn starts_with_visibility_prefix(fragment: &[u8]) -> Option<usize> {
    for prefix in [
        b"public".as_slice(),
        b"private".as_slice(),
        b"protected".as_slice(),
    ] {
        if starts_with_keyword(fragment, prefix) {
            return Some(prefix.len());
        }
    }
    None
}

fn strip_common_decl_prefixes(fragment: &[u8]) -> &[u8] {
    let mut frag = fragment;
    if let Some(len) = starts_with_visibility_prefix(frag) {
        frag = &frag[len..];
        let mut i = 0usize;
        while i < frag.len() && frag[i].is_ascii_whitespace() {
            i += 1;
        }
        frag = &frag[i..];
    }
    if starts_with_keyword(frag, b"abstract") || starts_with_keyword(frag, b"variation") {
        let cut = if starts_with_keyword(frag, b"abstract") {
            8
        } else {
            9
        };
        frag = &frag[cut..];
        let mut i = 0usize;
        while i < frag.len() && frag[i].is_ascii_whitespace() {
            i += 1;
        }
        frag = &frag[i..];
    }
    frag
}

fn is_modeled_decl_start(fragment: &[u8], starters: &[&[u8]]) -> bool {
    if fragment.starts_with(b"#") {
        return false;
    }
    if starts_with_keyword(fragment, b"package")
        || starts_with_keyword(fragment, b"library")
        || starts_with_keyword(fragment, b"namespace")
        || starts_with_keyword(fragment, b"import")
        || starts_with_keyword(fragment, b"doc")
        || starts_with_keyword(fragment, b"comment")
        || starts_with_keyword(fragment, b"filter")
    {
        return false;
    }
    let frag = strip_common_decl_prefixes(fragment);
    starts_with_any_keyword(frag, starters)
}

fn parse_modeled_decl<'a>(
    input: Input<'a>,
    starters: &'a [&'a [u8]],
) -> IResult<Input<'a>, (String, String)> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().is_empty() || input.fragment().starts_with(b"}") {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    if !is_modeled_decl_start(input.fragment(), starters) {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let raw_start = input;
    let stripped = strip_common_decl_prefixes(input.fragment());
    let bnf_production = starters
        .iter()
        .find(|kw| starts_with_keyword(stripped, kw))
        .map(|kw| String::from_utf8_lossy(kw).to_string())
        .unwrap_or_else(|| "declaration".to_string());
    let (input, _) = skip_statement_or_block(input)?;
    Ok((input, (bnf_production, modeled_decl_text(raw_start, input))))
}

fn kerml_semantic_decl(input: Input<'_>) -> IResult<Input<'_>, Node<KermlSemanticDecl>> {
    let start = input;
    let starters: &[&[u8]] = &[
        b"behavior",
        b"bool",
        b"function",
        b"interaction",
        b"datatype",
        b"inv",
        b"invariant",
        b"multiplicity",
        b"assoc",
        b"association",
        b"metaclass",
        b"step",
    ];
    let (input, (bnf_production, text)) = parse_modeled_decl(input, starters)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            KermlSemanticDecl {
                bnf_production,
                text,
            },
        ),
    ))
}

fn kerml_feature_decl(input: Input<'_>) -> IResult<Input<'_>, Node<KermlFeatureDecl>> {
    let start = input;
    let starters: &[&[u8]] = &[b"occurrence", b"expr", b"predicate", b"succession"];
    let (input, (bnf_production, text)) = parse_modeled_decl(input, starters)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            KermlFeatureDecl {
                bnf_production,
                text,
            },
        ),
    ))
}

fn feature_decl(input: Input<'_>) -> IResult<Input<'_>, Node<FeatureDecl>> {
    let start = input;
    let starters: &[&[u8]] = &[b"feature"];
    let (input, (keyword, text)) = parse_modeled_decl(input, starters)?;
    Ok((
        input,
        node_from_to(start, input, FeatureDecl { keyword, text }),
    ))
}

fn classifier_decl(input: Input<'_>) -> IResult<Input<'_>, Node<ClassifierDecl>> {
    let start = input;
    let starters: &[&[u8]] = &[
        b"class",
        b"classifier",
        b"struct",
        b"structure",
        b"subclassifier",
    ];
    let (input, (keyword, text)) = parse_modeled_decl(input, starters)?;
    Ok((
        input,
        node_from_to(start, input, ClassifierDecl { keyword, text }),
    ))
}

fn extended_library_decl(input: Input<'_>) -> IResult<Input<'_>, Node<ExtendedLibraryDecl>> {
    let start = input;
    let starters: &[&[u8]] = &[
        b"action",
        b"allocation",
        b"analysis",
        b"attribute",
        b"case",
        b"calc",
        b"connection",
        b"constraint",
        b"flow",
        b"interface",
        b"item",
        b"metadata",
        b"requirement",
        b"state",
        b"use",
        b"verification",
        b"view",
        b"viewpoint",
        b"rendering",
        b"enum",
        b"message",
        b"concern",
        b"part",
        b"port",
    ];
    let (input, (bnf_production, text)) = parse_modeled_decl(input, starters)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ExtendedLibraryDecl {
                bnf_production,
                text,
            },
        ),
    ))
}

fn package_body_brace(input: Input<'_>) -> IResult<Input<'_>, PackageBody> {
    let (mut input, _) = preceded(ws_and_comments, tag(&b"{"[..])).parse(input)?;
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
            return Ok((input, PackageBody::Brace { elements }));
        }
        match package_body_element(input) {
            Ok((next, element)) => {
                if next.location_offset() == input.location_offset() {
                    return Err(nom::Err::Failure(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Many0,
                    )));
                }
                elements.push(element);
                input = next;
            }
            Err(_)
                if starts_with_any_keyword(input.fragment(), PACKAGE_BODY_STARTERS)
                    || starts_with_any_keyword(
                        strip_common_decl_prefixes(input.fragment()),
                        PACKAGE_BODY_STARTERS,
                    ) =>
            {
                if let Ok((next, element)) = package_body_element_fallback(input) {
                    if next.location_offset() == input.location_offset() {
                        return Err(nom::Err::Failure(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Many0,
                        )));
                    }
                    elements.push(element);
                    input = next;
                    continue;
                }
                let (next, _) = recover_body_element(input, PACKAGE_BODY_STARTERS)?;
                if next.location_offset() == input.location_offset() {
                    return Err(nom::Err::Failure(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Many0,
                    )));
                }
                let recovery = build_recovery_error_node_from_span(
                    input,
                    next,
                    PACKAGE_BODY_STARTERS,
                    "package body",
                    "recovered_package_body_element",
                );
                if matches!(
                    recovery.code.as_str(),
                    "invalid_typing_operator"
                        | "missing_body_or_semicolon"
                        | "missing_expression_after_operator"
                        | "unexpected_keyword_in_scope"
                        | "unsupported_annotation_syntax"
                ) {
                    elements.push(node_from_to(
                        input,
                        next,
                        PackageBodyElement::Error(Node::new(crate::ast::Span::dummy(), recovery)),
                    ));
                    input = next;
                    continue;
                }
                // If we couldn't parse a dedicated node but the line still looks like a modeled
                // library declaration (including `abstract`/visibility prefixes), preserve it as
                // an `ExtendedLibraryDecl` instead of aborting the entire package.
                if let Ok((next, ext)) = map(
                    extended_library_decl,
                    PackageBodyElement::ExtendedLibraryDecl,
                )
                .parse(input)
                {
                    if next.location_offset() == input.location_offset() {
                        return Err(nom::Err::Failure(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Many0,
                        )));
                    }
                    elements.push(node_from_to(input, next, ext));
                    input = next;
                    continue;
                }
                elements.push(node_from_to(
                    input,
                    next,
                    PackageBodyElement::Error(Node::new(crate::ast::Span::dummy(), recovery)),
                ));
                input = next;
            }
            Err(_) => {
                if let Ok((next, element)) = package_body_element_fallback(input) {
                    if next.location_offset() > input.location_offset() {
                        elements.push(element);
                        input = next;
                        continue;
                    }
                }
                let (next, _) = recover_body_element(input, PACKAGE_BODY_STARTERS)?;
                if next.location_offset() == input.location_offset() {
                    return Err(nom::Err::Failure(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Many0,
                    )));
                }
                let recovery = build_recovery_error_node_from_span(
                    input,
                    next,
                    PACKAGE_BODY_STARTERS,
                    "package body",
                    "recovered_package_body_element",
                );
                elements.push(node_from_to(
                    input,
                    next,
                    PackageBodyElement::Error(Node::new(crate::ast::Span::dummy(), recovery)),
                ));
                input = next;
            }
        }
    }
}

/// KerML ElementFilterMember: MemberPrefix? 'filter' condition = OwnedExpression ';'
pub(crate) fn filter_member(input: Input<'_>) -> IResult<Input<'_>, Node<FilterMember>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, visibility) = opt(alt((
        map(preceded(tag(&b"public"[..]), ws1), |_| Visibility::Public),
        map(preceded(tag(&b"private"[..]), ws1), |_| Visibility::Private),
        map(preceded(tag(&b"protected"[..]), ws1), |_| {
            Visibility::Protected
        }),
    )))
    .parse(input)?;
    let (input, _) = tag(&b"filter"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, condition) = expression(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            FilterMember {
                visibility,
                condition,
            },
        ),
    ))
}

macro_rules! try_package_body_dispatch {
    ($input:expr, $start:expr, $parser:expr, $wrap:expr) => {
        if let Ok((input, elem)) = map($parser, $wrap).parse($input) {
            return Ok((input, node_from_to($start, input, elem)));
        }
    };
}

fn try_package_body_annotations<'a>(
    input: Input<'a>,
    start: Input<'a>,
) -> IResult<Input<'a>, Node<PackageBodyElement>> {
    try_package_body_dispatch!(input, start, doc_comment, PackageBodyElement::Doc);
    try_package_body_dispatch!(
        input,
        start,
        comment_annotation,
        PackageBodyElement::Comment
    );
    try_package_body_dispatch!(
        input,
        start,
        textual_representation,
        PackageBodyElement::TextualRep
    );
    try_package_body_dispatch!(input, start, filter_member, PackageBodyElement::Filter);
    try_package_body_dispatch!(
        input,
        start,
        |i| attribute_def(i, false),
        PackageBodyElement::AttributeDef
    );
    // PAR-002: standalone attribute usage at package level (BNF `PackageMember` allows
    // `DefinitionElement | UsageElement`). Tried after `attribute_def(.., false)` above, which
    // is itself `def`-optional and already captures the def-less bare form; this only adds
    // coverage for usage-only shapes (e.g. `subsets`/`references`/`crosses`/`redefines` clauses)
    // that `attribute_def`'s grammar doesn't accept.
    try_package_body_dispatch!(
        input,
        start,
        attribute_usage,
        PackageBodyElement::AttributeUsage
    );
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Alt,
    )))
}

fn try_package_body_packages<'a>(
    input: Input<'a>,
    start: Input<'a>,
) -> IResult<Input<'a>, Node<PackageBodyElement>> {
    try_package_body_dispatch!(
        input,
        start,
        library_package_,
        PackageBodyElement::LibraryPackage
    );
    try_package_body_dispatch!(input, start, package_, PackageBodyElement::Package);
    try_package_body_dispatch!(input, start, import_, PackageBodyElement::Import);
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Alt,
    )))
}

fn try_package_body_structure<'a>(
    input: Input<'a>,
    start: Input<'a>,
) -> IResult<Input<'a>, Node<PackageBodyElement>> {
    try_package_body_dispatch!(input, start, part_def_or_usage, |p| match p {
        PartDefOrUsage::Def(n) => PackageBodyElement::PartDef(n),
        PartDefOrUsage::Usage(n) => PackageBodyElement::PartUsage(n),
    });
    try_package_body_dispatch!(input, start, port_def, PackageBodyElement::PortDef);
    // PAR-002: standalone port usage at package level, tried after `port_def` above (which is
    // `def`-optional per its own doc comment and already captures the bare def-less form) -- see
    // `AttributeUsage` above for the same rationale.
    try_package_body_dispatch!(input, start, port_usage, PackageBodyElement::PortUsage);
    try_package_body_dispatch!(
        input,
        start,
        interface_def,
        PackageBodyElement::InterfaceDef
    );
    try_package_body_dispatch!(
        input,
        start,
        connection_def,
        PackageBodyElement::ConnectionDef
    );
    // PAR-002: standalone connection usage at package level, tried after `connection_def` above
    // for the same reason as `port_usage`.
    try_package_body_dispatch!(
        input,
        start,
        crate::parser::part::connection_usage_member,
        PackageBodyElement::ConnectionUsage
    );
    try_package_body_dispatch!(input, start, dependency, PackageBodyElement::Dependency);
    try_package_body_dispatch!(
        input,
        start,
        metadata_usage,
        PackageBodyElement::MetadataUsage
    );
    try_package_body_dispatch!(input, start, metadata_def, PackageBodyElement::MetadataDef);
    try_package_body_dispatch!(input, start, enum_def, PackageBodyElement::EnumDef);
    // PAR-002: standalone enumeration usage at package level, tried after `enum_def` above for
    // the same reason as `port_usage`.
    try_package_body_dispatch!(
        input,
        start,
        enum_usage,
        PackageBodyElement::EnumerationUsage
    );
    // PAR-002: standalone `ref` declaration at package level (BNF `PackageMember` allows a bare
    // `UsageElement`, and `ref` features are a legal usage shape). Reuses `part_ref_usage`, which
    // despite its name has no part-specific grammar (`ref` (part)? name (: type)? (= value)?).
    try_package_body_dispatch!(
        input,
        start,
        crate::parser::part::part_ref_usage,
        PackageBodyElement::Ref
    );
    try_package_body_dispatch!(
        input,
        start,
        occurrence_def,
        PackageBodyElement::OccurrenceDef
    );
    try_package_body_dispatch!(
        input,
        start,
        occurrence_usage,
        PackageBodyElement::OccurrenceUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        individual_usage,
        PackageBodyElement::OccurrenceUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        snapshot_usage,
        PackageBodyElement::OccurrenceUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        timeslice_usage,
        PackageBodyElement::OccurrenceUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        allocation_def,
        PackageBodyElement::AllocationDef
    );
    try_package_body_dispatch!(
        input,
        start,
        allocation_usage,
        PackageBodyElement::AllocationUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        allocate_usage,
        PackageBodyElement::AllocationUsage
    );
    try_package_body_dispatch!(input, start, flow_def, PackageBodyElement::FlowDef);
    try_package_body_dispatch!(input, start, flow_usage, PackageBodyElement::FlowUsage);
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Alt,
    )))
}

fn try_package_body_behavior<'a>(
    input: Input<'a>,
    start: Input<'a>,
) -> IResult<Input<'a>, Node<PackageBodyElement>> {
    try_package_body_dispatch!(input, start, alias_def, PackageBodyElement::AliasDef);
    try_package_body_dispatch!(input, start, action_def, PackageBodyElement::ActionDef);
    try_package_body_dispatch!(input, start, action_usage, PackageBodyElement::ActionUsage);
    try_package_body_dispatch!(input, start, state_def, PackageBodyElement::StateDef);
    try_package_body_dispatch!(input, start, state_usage, PackageBodyElement::StateUsage);
    try_package_body_dispatch!(input, start, item_def, PackageBodyElement::ItemDef);
    // PAR-002: standalone item usage at package level, tried after `item_def` above (`def`-
    // optional per its own dispatch here) for the same reason as `port_usage`/`AttributeUsage`.
    try_package_body_dispatch!(input, start, item_usage, PackageBodyElement::ItemUsage);
    try_package_body_dispatch!(
        input,
        start,
        individual_def,
        PackageBodyElement::IndividualDef
    );
    try_package_body_dispatch!(
        input,
        start,
        constraint_def,
        PackageBodyElement::ConstraintDef
    );
    try_package_body_dispatch!(input, start, calc_def, PackageBodyElement::CalcDef);
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Alt,
    )))
}

fn try_package_body_requirement<'a>(
    input: Input<'a>,
    start: Input<'a>,
) -> IResult<Input<'a>, Node<PackageBodyElement>> {
    try_package_body_dispatch!(
        input,
        start,
        requirement_def,
        PackageBodyElement::RequirementDef
    );
    try_package_body_dispatch!(
        input,
        start,
        requirement_usage,
        PackageBodyElement::RequirementUsage
    );
    try_package_body_dispatch!(input, start, satisfy, PackageBodyElement::Satisfy);
    try_package_body_dispatch!(input, start, use_case_def, PackageBodyElement::UseCaseDef);
    try_package_body_dispatch!(
        input,
        start,
        use_case_usage,
        PackageBodyElement::UseCaseUsage
    );
    try_package_body_dispatch!(input, start, case_def, PackageBodyElement::CaseDef);
    try_package_body_dispatch!(input, start, case_usage, PackageBodyElement::CaseUsage);
    try_package_body_dispatch!(
        input,
        start,
        analysis_case_def,
        PackageBodyElement::AnalysisCaseDef
    );
    try_package_body_dispatch!(
        input,
        start,
        analysis_case_usage,
        PackageBodyElement::AnalysisCaseUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        verification_case_def,
        PackageBodyElement::VerificationCaseDef
    );
    try_package_body_dispatch!(
        input,
        start,
        verification_case_usage,
        PackageBodyElement::VerificationCaseUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        concern_usage,
        PackageBodyElement::ConcernUsage
    );
    try_package_body_dispatch!(input, start, actor_decl, PackageBodyElement::Actor);
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Alt,
    )))
}

fn try_package_body_view<'a>(
    input: Input<'a>,
    start: Input<'a>,
) -> IResult<Input<'a>, Node<PackageBodyElement>> {
    try_package_body_dispatch!(input, start, view_def, PackageBodyElement::ViewDef);
    try_package_body_dispatch!(
        input,
        start,
        viewpoint_def,
        PackageBodyElement::ViewpointDef
    );
    try_package_body_dispatch!(
        input,
        start,
        rendering_def,
        PackageBodyElement::RenderingDef
    );
    try_package_body_dispatch!(input, start, view_usage, PackageBodyElement::ViewUsage);
    try_package_body_dispatch!(
        input,
        start,
        viewpoint_usage,
        PackageBodyElement::ViewpointUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        rendering_usage,
        PackageBodyElement::RenderingUsage
    );
    try_package_body_dispatch!(input, start, feature_decl, PackageBodyElement::FeatureDecl);
    try_package_body_dispatch!(
        input,
        start,
        classifier_decl,
        PackageBodyElement::ClassifierDecl
    );
    try_package_body_dispatch!(
        input,
        start,
        kerml_semantic_decl,
        PackageBodyElement::KermlSemanticDecl
    );
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Alt,
    )))
}

/// PackageBodyElement: Package | Import | PartDef | PartUsage | PortDef | InterfaceDef | AliasDef | ActionDef | ActionUsage
pub(crate) fn package_body_element(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<PackageBodyElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    if let Ok(r) = try_package_body_annotations(input, start) {
        return Ok(r);
    }
    if let Ok(r) = try_package_body_packages(input, start) {
        return Ok(r);
    }
    if let Ok(r) = try_package_body_structure(input, start) {
        return Ok(r);
    }
    if let Ok(r) = try_package_body_behavior(input, start) {
        return Ok(r);
    }
    if let Ok(r) = try_package_body_requirement(input, start) {
        return Ok(r);
    }
    if let Ok(r) = try_package_body_view(input, start) {
        return Ok(r);
    }
    if starts_with_keyword(input.fragment(), b"occurrence") {
        if let Ok((next, _)) = recover_body_element(input, PACKAGE_BODY_STARTERS) {
            if next.location_offset() != input.location_offset() {
                let recovery = build_recovery_error_node_from_span(
                    input,
                    next,
                    PACKAGE_BODY_STARTERS,
                    "package body",
                    "recovered_package_body_element",
                );
                if matches!(
                    recovery.code.as_str(),
                    "invalid_typing_operator" | "missing_type_reference"
                ) {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Tag,
                    )));
                }
            }
        }
    }
    if let Ok((input, elem)) =
        map(kerml_feature_decl, PackageBodyElement::KermlFeatureDecl).parse(input)
    {
        return Ok((input, node_from_to(start, input, elem)));
    }
    if let Ok((next, _)) = recover_body_element(input, PACKAGE_BODY_STARTERS) {
        if next.location_offset() != input.location_offset() {
            let recovery = build_recovery_error_node_from_span(
                input,
                next,
                PACKAGE_BODY_STARTERS,
                "package body",
                "recovered_package_body_element",
            );
            if matches!(
                recovery.code.as_str(),
                "invalid_typing_operator"
                    | "missing_body_or_semicolon"
                    | "missing_expression_after_operator"
                    | "unexpected_keyword_in_scope"
                    | "unsupported_annotation_syntax"
            ) {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Tag,
                )));
            }
        }
    }
    let (input, elem) = map(
        extended_library_decl,
        PackageBodyElement::ExtendedLibraryDecl,
    )
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

/// Root: (package | namespace)*
pub(crate) fn root_namespace(input: Input<'_>) -> IResult<Input<'_>, RootNamespace> {
    let (input, _) = ws_and_comments(input)?;
    let (input, elements) = many0(preceded(ws_and_comments, root_element)).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    Ok((input, RootNamespace { elements }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom_locate::LocatedSpan;
    use std::path::PathBuf;

    fn sysml_v2_release_root() -> PathBuf {
        std::env::var_os("SYSML_V2_RELEASE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml-v2-release"))
    }

    fn primitive_data_types_fixture() -> Option<String> {
        let path = sysml_v2_release_root()
            .join("sysml")
            .join("src")
            .join("validation")
            .join("15-Properties-Values-Expressions")
            .join("15_10-Primitive Data Types.sysml");
        std::fs::read_to_string(path).ok()
    }

    #[test]
    fn kitchen_timer_display_tail_parses_as_package_body_element() {
        let input = include_str!("../../tests/fixtures/KitchenTimer.sysml")
            .replace("\r\n", "\n")
            .replace('\r', "\n");
        let start = input
            .find("\tpart def Display {")
            .expect("fixture should contain Display part");
        let tail = &input.as_bytes()[start..];
        let located = LocatedSpan::new(tail);

        let result = package_body_element(located);
        assert!(
            result.is_ok(),
            "package_body_element should parse Display tail, got {:?}",
            result
        );
    }

    #[test]
    fn kitchen_timer_display_tail_parses_as_part_directly() {
        let input = include_str!("../../tests/fixtures/KitchenTimer.sysml")
            .replace("\r\n", "\n")
            .replace('\r', "\n");
        let start = input
            .find("\tpart def Display {")
            .expect("fixture should contain Display part");
        let tail = &input.as_bytes()[start..];
        let located = LocatedSpan::new(tail);
        let (located, _) = ws_and_comments(located).expect("leading ws");

        let result = part_def_or_usage(located);
        assert!(
            result.is_ok(),
            "part_def_or_usage should parse Display tail directly, got {:?}",
            result
        );
    }

    #[test]
    fn primitive_data_types_validation_fixture_package_parses_directly() {
        let Some(input) = primitive_data_types_fixture() else {
            return;
        };
        let located = LocatedSpan::new(input.as_bytes());
        let result = package_(located);
        assert!(
            result.is_ok(),
            "package_ should parse fixture, got {:?}",
            result
        );
    }

    #[test]
    fn primitive_data_types_validation_fixture_package_body_parses_directly() {
        let Some(input) = primitive_data_types_fixture() else {
            return;
        };
        let start = input
            .find('{')
            .expect("fixture should contain package body");
        let located = LocatedSpan::new(&input.as_bytes()[start..]);
        let result = package_body_brace(located);
        assert!(
            result.is_ok(),
            "package_body_brace should parse fixture body, got {:?}",
            result
        );
    }

    // --- PAR-002 increment 3: standalone usages at package level ---

    fn parse_input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    #[test]
    fn package_body_accepts_standalone_attribute_usage_with_redefines() {
        // `redefines` is a shape `attribute_def` doesn't accept but `attribute_usage` does --
        // exercises the actual value-add of wiring `AttributeUsage` in behind `AttributeDef`.
        let (rest, node) =
            package_body_element(parse_input("attribute :>> mass = 5;")).expect("attribute usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PackageBodyElement::AttributeUsage(_)));
    }

    #[test]
    fn package_body_accepts_standalone_port_usage() {
        let (rest, node) =
            package_body_element(parse_input("port :>> p1: MyPortType;")).expect("port usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PackageBodyElement::PortUsage(_)));
    }

    #[test]
    fn package_body_accepts_standalone_item_usage() {
        let (rest, node) =
            package_body_element(parse_input("item i1 subsets otherItem;")).expect("item usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PackageBodyElement::ItemUsage(_)));
    }

    #[test]
    fn package_body_accepts_standalone_enumeration_usage() {
        let (rest, node) =
            package_body_element(parse_input("enum e1: MyEnum;")).expect("enum usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            PackageBodyElement::EnumerationUsage(_)
        ));
    }

    #[test]
    fn package_body_accepts_standalone_ref_declaration() {
        let (rest, node) =
            package_body_element(parse_input("ref r1: MyType;")).expect("ref declaration");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PackageBodyElement::Ref(_)));
    }

    #[test]
    fn package_body_accepts_standalone_connection_usage() {
        let (rest, node) =
            package_body_element(parse_input("connection: LinkType;")).expect("connection usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PackageBodyElement::ConnectionUsage(_)));
    }

    /// PAR-002 acceptance criterion, increment 3: the same legal `:>>`-prefixed attribute usage
    /// (a shape `attribute_def` cannot parse at all -- it has no leading-redefines-operator head)
    /// yields the same AST variant kind at package level and nested in a part body.
    #[test]
    fn attribute_usage_with_redefines_is_same_variant_kind_at_package_level_and_nested_in_part() {
        use crate::parser::part::part_def_body;
        let text = "attribute :>> mass = 5;";
        let (_, package_node) =
            package_body_element(parse_input(text)).expect("package-level attribute usage");
        assert!(matches!(
            package_node.value,
            PackageBodyElement::AttributeUsage(_)
        ));
        let part_text = format!("{{ {text} }}");
        let (_, body) = part_def_body(parse_input(&part_text)).expect("nested attribute usage");
        let crate::ast::PartDefBody::Brace { elements } = body else {
            panic!("expected brace body");
        };
        assert_eq!(elements.len(), 1);
        assert!(matches!(
            elements[0].value,
            crate::ast::PartDefBodyElement::AttributeUsage(_)
        ));
    }
}
