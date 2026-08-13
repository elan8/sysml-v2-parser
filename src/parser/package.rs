//! Package and root namespace parsing.

use crate::ast::{
    ClassifierDecl, DeclarationName, ExtendedLibraryDecl, FeatureDecl, FilterMember,
    KermlFeatureDecl, KermlSemanticDecl, LibraryPackage, NamespaceDecl, Node, Package, PackageBody,
    PackageBodyElement, QualifiedIdentification, RootElement, RootNamespace, Visibility,
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
use crate::parser::connector::connect_body;
use crate::parser::constraint::{calc_def, constraint_def, constraint_usage};
use crate::parser::dependency::dependency;
use crate::parser::enumeration::{enum_def, enum_usage};
use crate::parser::expr::expression;
use crate::parser::flow::{flow_def, flow_usage};
use crate::parser::grammar_scope::{
    package_body_starter, PackageProduction, PACKAGE_BODY_STARTERS,
};
use crate::parser::import::import_;
use crate::parser::individual::individual_def;
use crate::parser::interface::interface_def;
use crate::parser::item::{item_def, item_usage};
use crate::parser::lex::{
    name, qualified_declaration_name, recover_body_element, skip_statement_or_block,
    starts_with_any_keyword, starts_with_keyword, ws1, ws_and_comments,
};
use crate::parser::metadata::{metadata_def, metadata_usage};
use crate::parser::node_from_to;
use crate::parser::occurrence::{
    individual_usage, occurrence_def, occurrence_usage, snapshot_usage, timeslice_usage,
};
use crate::parser::part::{interface_usage, part_def_or_usage, PartDefOrUsage};
use crate::parser::port::{port_def, port_usage};
use crate::parser::requirement::{
    bare_locale_comment, comment_annotation, concern_usage, doc_comment, requirement_def,
    requirement_usage, satisfy, textual_representation,
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
) -> IResult<Input<'_>, QualifiedIdentification> {
    let (input, short_name) = opt(delimited(
        preceded(ws_and_comments, tag(&b"<"[..])),
        preceded(ws_and_comments, name),
        preceded(ws_and_comments, tag(&b">"[..])),
    ))
    .parse(input)?;
    let (input, decl_name) = opt(preceded(
        ws_and_comments,
        alt((
            map(qualified_declaration_name, DeclarationName::Qualified),
            map(name, DeclarationName::Simple),
        )),
    ))
    .parse(input)?;
    if short_name.is_some() || decl_name.is_some() {
        Ok((
            input,
            QualifiedIdentification {
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
    crate::parser::span::reference_transaction(input, library_package_inner)
}

fn library_package_inner(input: Input<'_>) -> IResult<Input<'_>, Node<LibraryPackage>> {
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
    crate::parser::span::reference_transaction(input, package_inner)
}

fn package_inner(input: Input<'_>) -> IResult<Input<'_>, Node<Package>> {
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
    crate::parser::span::reference_transaction(input, namespace_decl_inner)
}

fn namespace_decl_inner(input: Input<'_>) -> IResult<Input<'_>, Node<NamespaceDecl>> {
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

/// One root-level element (BNF `RootNamespace = PackageBodyElement*`).
///
/// Dedicated variants for package / namespace / import keep existing consumers stable; all other
/// legal package-body members (e.g. `part def` at file root) become [`RootElement::Member`].
pub(crate) fn root_element(input: Input<'_>) -> IResult<Input<'_>, Node<RootElement>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    if let Ok((next, elem)) = crate::parser::span::reference_transaction(input, |input| {
        map(import_, |import| RootElement::Import(Box::new(import))).parse(input)
    }) {
        return Ok((next, node_from_to(start, next, elem)));
    }
    if let Ok((next, elem)) = crate::parser::span::reference_transaction(input, |input| {
        map(namespace_decl, RootElement::Namespace).parse(input)
    }) {
        return Ok((next, node_from_to(start, next, elem)));
    }
    if let Ok((next, elem)) = crate::parser::span::reference_transaction(input, |input| {
        map(library_package_, RootElement::LibraryPackage).parse(input)
    }) {
        return Ok((next, node_from_to(start, next, elem)));
    }
    if let Ok((next, elem)) = crate::parser::span::reference_transaction(input, |input| {
        map(package_, RootElement::Package).parse(input)
    }) {
        return Ok((next, node_from_to(start, next, elem)));
    }
    let (input, boxed) = package_body_element(input)?;
    let elem = match &boxed.value {
        PackageBodyElement::Package(n) => RootElement::Package(n.clone()),
        PackageBodyElement::LibraryPackage(n) => RootElement::LibraryPackage(n.clone()),
        PackageBodyElement::Import(n) => RootElement::Import(Box::new(n.clone())),
        PackageBodyElement::Error(_)
        | PackageBodyElement::Unsupported(_)
        | PackageBodyElement::Doc(_)
        | PackageBodyElement::Comment(_)
        | PackageBodyElement::TextualRep(_)
        | PackageBodyElement::Filter(_)
        | PackageBodyElement::PartDef(_)
        | PackageBodyElement::PartUsage(_)
        | PackageBodyElement::PortDef(_)
        | PackageBodyElement::InterfaceDef(_)
        | PackageBodyElement::AliasDef(_)
        | PackageBodyElement::AttributeDef(_)
        | PackageBodyElement::ActionDef(_)
        | PackageBodyElement::ActionUsage(_)
        | PackageBodyElement::RequirementDef(_)
        | PackageBodyElement::RequirementUsage(_)
        | PackageBodyElement::Satisfy(_)
        | PackageBodyElement::UseCaseDef(_)
        | PackageBodyElement::Actor(_)
        | PackageBodyElement::StateDef(_)
        | PackageBodyElement::StateUsage(_)
        | PackageBodyElement::ItemDef(_)
        | PackageBodyElement::IndividualDef(_)
        | PackageBodyElement::ConstraintDef(_)
        | PackageBodyElement::ConstraintUsage(_)
        | PackageBodyElement::CalcDef(_)
        | PackageBodyElement::ViewDef(_)
        | PackageBodyElement::ViewpointDef(_)
        | PackageBodyElement::RenderingDef(_)
        | PackageBodyElement::ViewUsage(_)
        | PackageBodyElement::ViewpointUsage(_)
        | PackageBodyElement::RenderingUsage(_)
        | PackageBodyElement::ConnectionDef(_)
        | PackageBodyElement::MetadataDef(_)
        | PackageBodyElement::MetadataUsage(_)
        | PackageBodyElement::EnumDef(_)
        | PackageBodyElement::OccurrenceDef(_)
        | PackageBodyElement::OccurrenceUsage(_)
        | PackageBodyElement::Dependency(_)
        | PackageBodyElement::AllocationDef(_)
        | PackageBodyElement::AllocationUsage(_)
        | PackageBodyElement::FlowDef(_)
        | PackageBodyElement::FlowUsage(_)
        | PackageBodyElement::ConcernUsage(_)
        | PackageBodyElement::CaseDef(_)
        | PackageBodyElement::CaseUsage(_)
        | PackageBodyElement::AnalysisCaseDef(_)
        | PackageBodyElement::AnalysisCaseUsage(_)
        | PackageBodyElement::VerificationCaseDef(_)
        | PackageBodyElement::VerificationCaseUsage(_)
        | PackageBodyElement::UseCaseUsage(_)
        | PackageBodyElement::FeatureDecl(_)
        | PackageBodyElement::ClassifierDecl(_)
        | PackageBodyElement::KermlSemanticDecl(_)
        | PackageBodyElement::KermlFeatureDecl(_)
        | PackageBodyElement::KermlBareDeclaration(_)
        | PackageBodyElement::ExtendedLibraryDecl(_)
        | PackageBodyElement::AttributeUsage(_)
        | PackageBodyElement::ItemUsage(_)
        | PackageBodyElement::PortUsage(_)
        | PackageBodyElement::ConnectionUsage(_)
        | PackageBodyElement::InterfaceUsage(_)
        | PackageBodyElement::Ref(_)
        | PackageBodyElement::EnumerationUsage(_)
        | PackageBodyElement::MetadataKeywordUsage(_)
        | PackageBodyElement::Connect(_)
        | PackageBodyElement::DefaultReferenceUsage(_)
        | PackageBodyElement::AssertConstraint(_)
        | PackageBodyElement::PerformUsage(_)
        | PackageBodyElement::BindingConnectorUsage(_)
        | PackageBodyElement::ClassDef(_)
        | PackageBodyElement::Succession(_)
        | PackageBodyElement::ExhibitState(_)
        | PackageBodyElement::IncludeUseCase(_) => RootElement::Member(boxed),
    };
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
    crate::parser::span::reference_transaction(input, package_body_element_fallback_inner)
}

fn package_body_element_fallback_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<PackageBodyElement>> {
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

/// Package-level `BindingConnectorAsUsage` (BNF §8.2.2.13.2): `binding` (`all`)? name?
/// multiplicity? (`of`|`bind`)? left `=` right body. See `crate::ast::BindingConnectorUsage`'s
/// doc comment for the four real shapes this covers.
fn binding_connector_usage(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::BindingConnectorUsage>> {
    crate::parser::span::reference_transaction(input, binding_connector_usage_inner)
}

fn binding_connector_usage_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::BindingConnectorUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"binding"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, all) = opt(preceded(tag(&b"all"[..]), ws1))
        .parse(input)
        .map(|(i, o)| (i, o.is_some()))?;
    let (peek, _) = ws_and_comments(input)?;
    let frag = peek.fragment();
    let (input, name) = if all
        || frag.starts_with(b"[")
        || starts_with_keyword(frag, b"of")
        || starts_with_keyword(frag, b"bind")
    {
        (input, None)
    } else {
        let (input, parsed_name) = name(input)?;
        (input, Some(parsed_name))
    };
    let (input, multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    let (peek, _) = ws_and_comments(input)?;
    let (input, uses_of_keyword) = if starts_with_keyword(peek.fragment(), b"of") {
        let (input, _) = preceded(ws_and_comments, tag(&b"of"[..])).parse(input)?;
        let (input, _) = ws1(input)?;
        (input, true)
    } else {
        (input, false)
    };
    let (peek, _) = ws_and_comments(input)?;
    let (input, uses_bind_keyword) =
        if !uses_of_keyword && starts_with_keyword(peek.fragment(), b"bind") {
            let (input, _) = preceded(ws_and_comments, tag(&b"bind"[..])).parse(input)?;
            let (input, _) = ws1(input)?;
            (input, true)
        } else {
            (input, false)
        };
    let (input, left) =
        preceded(ws_and_comments, crate::parser::lex::qualified_reference).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b"="[..])).parse(input)?;
    let (input, right) =
        preceded(ws_and_comments, crate::parser::lex::qualified_reference).parse(input)?;
    let (input, body) = connect_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::BindingConnectorUsage {
                all,
                name,
                multiplicity,
                uses_of_keyword,
                uses_bind_keyword,
                left,
                right,
                body,
            },
        ),
    ))
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

fn unsupported_package_element(
    input: Input<'_>,
    recovery_end: Input<'_>,
) -> Option<Node<PackageBodyElement>> {
    let (trimmed, _) = ws_and_comments(input).ok()?;
    let stripped = strip_common_decl_prefixes(trimmed.fragment());
    let starter = package_body_starter(stripped)?;
    let unsupported_production = match starter.production {
        PackageProduction::BindingConnectorAsUsage => {
            crate::ast::UnsupportedProduction::BindingConnectorAsUsage
        }
        PackageProduction::PerformActionUsage => {
            crate::ast::UnsupportedProduction::PerformActionUsage
        }
        PackageProduction::PrefixMetadataMember
        | PackageProduction::DefinitionElement
        | PackageProduction::Action
        | PackageProduction::ActorUsage
        | PackageProduction::AliasMember
        | PackageProduction::Allocation
        | PackageProduction::AnalysisCase
        | PackageProduction::AssertConstraintUsage
        | PackageProduction::Constraint
        | PackageProduction::Attribute
        | PackageProduction::Calculation
        | PackageProduction::Case
        | PackageProduction::Comment
        | PackageProduction::Concern
        | PackageProduction::Connection
        | PackageProduction::Dependency
        | PackageProduction::Documentation
        | PackageProduction::Enumeration
        | PackageProduction::Expose
        | PackageProduction::ElementFilterMember
        | PackageProduction::Flow
        | PackageProduction::Message
        | PackageProduction::Succession
        | PackageProduction::ExhibitStateUsage
        | PackageProduction::IncludeUseCaseUsage
        | PackageProduction::RequirementBodyItem
        | PackageProduction::Import
        | PackageProduction::Individual
        | PackageProduction::Interface
        | PackageProduction::Item
        | PackageProduction::LibraryPackage
        | PackageProduction::Metadata
        | PackageProduction::Namespace
        | PackageProduction::Occurrence
        | PackageProduction::Package
        | PackageProduction::Part
        | PackageProduction::Port
        | PackageProduction::MemberPrefix
        | PackageProduction::ViewRenderingUsage
        | PackageProduction::Rendering
        | PackageProduction::TextualRepresentation
        | PackageProduction::Requirement
        | PackageProduction::SatisfyRequirementUsage
        | PackageProduction::UseCase
        | PackageProduction::VerificationCase
        | PackageProduction::View
        | PackageProduction::Viewpoint
        | PackageProduction::FeaturePrefix
        | PackageProduction::UsagePrefix
        | PackageProduction::Connector
        | PackageProduction::TextualRepresentationLanguage
        | PackageProduction::Feature
        | PackageProduction::Class => return None,
    };
    let found = crate::parser::recovery::recovery_found_snippet_from_span(input, recovery_end);
    let recovery = crate::ast::ParseErrorNode {
        message: format!(
            "the spec-valid {} production is not implemented in package bodies",
            starter.production.bnf_name()
        ),
        code: "unsupported_grammar_form".to_owned(),
        expected: Some(format!(
            "implemented package-body form for {}",
            starter.production.bnf_name()
        )),
        found,
        suggestion: Some(
            "Keep this syntax; parser support is incomplete rather than the authored model being malformed."
                .to_owned(),
        ),
        category: Some(crate::error::DiagnosticCategory::UnsupportedGrammarForm),
    };
    Some(node_from_to(
        input,
        recovery_end,
        PackageBodyElement::Unsupported(Node::new(
            crate::ast::Span::dummy(),
            crate::ast::UnsupportedGrammarNode {
                production: unsupported_production,
                diagnostic: recovery,
            },
        )),
    ))
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

/// Structurally recognized bare KerML declaration: `kind` `name`? (`[` multiplicity `]`)? `;`.
/// Covers `datatype DeferredType;`, `multiplicity exactlyOne [1..1];`,
/// `interaction DeferredInteraction;`, `predicate deferredPredicate;`, and the same shape for
/// every other KerML classifier/feature keyword whose bare (bodyless) form uses it. Tried before
/// the opaque [`kerml_semantic_decl`]/[`kerml_feature_decl`] fallbacks so this common shape gets
/// a structured node; a keyword followed by a `{` body (a different, larger production) falls
/// through to those fallbacks unchanged.
fn kerml_bare_declaration(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlBareDeclaration>> {
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
        b"occurrence",
        b"expr",
        b"predicate",
        b"succession",
    ];
    let (input, _) = ws_and_comments(input)?;
    let keyword = starters
        .iter()
        .find(|kw| starts_with_keyword(input.fragment(), kw))
        .ok_or_else(|| {
            nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag))
        })?;
    let (input, _) = nom::bytes::complete::tag(*keyword).parse(input)?;
    let keyword = String::from_utf8_lossy(keyword).to_string();
    let (input, name_opt) =
        opt(preceded(ws1, crate::parser::span::with_span(name))).parse(input)?;
    let (name, name_span) = match name_opt {
        Some((span, text)) => (Some(text), Some(span)),
        None => (None, None),
    };
    let (input, multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    let (input, _) =
        preceded(ws_and_comments, nom::bytes::complete::tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::KermlBareDeclaration {
                keyword,
                name,
                name_span,
                multiplicity,
            },
        ),
    ))
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

/// KerML `class` classifier definition: `class` Identification (`:>`|`specializes`) type? body.
/// Mirrors `individual_def` exactly (same `def`-optional, `no_abstract`, captured-visibility
/// shape) -- see `crate::ast::ClassDef`'s doc comment.
fn class_def(input: Input<'_>) -> IResult<Input<'_>, Node<crate::ast::ClassDef>> {
    let start = input;
    let (input, prefix) = crate::parser::definition_prefix::parse_definition_prefix(
        input,
        crate::parser::definition_prefix::DefinitionPrefixOptions::new(b"class")
            .no_abstract()
            .with_captured_visibility(),
    )?;
    let (input, body) = crate::parser::attribute::attribute_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::ClassDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
                membership: crate::ast::Membership::owning(
                    prefix.visibility,
                    prefix.visibility_span,
                ),
            },
        ),
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

/// KerML bare `feature` usage declaration: `feature` name (`:` type | `:>` target | `:>>`
/// target)? (`=` value)? `;`. Reuses the same `DefaultReferenceUsage` shape the keyword-less
/// `name;` / `name = expr;` form already produces (see `feature_value_binding`/
/// `bare_or_valued_feature_binding` in `attribute.rs`), just with the explicit leading `feature`
/// keyword consumed first and `:` typing additionally supported (those two helpers only handle
/// `:>`/`:>>`, never a plain `:` type since the keyword-less form can't tell a type from an
/// expression there).
fn feature_usage_member(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::DefaultReferenceUsage>> {
    crate::parser::span::reference_transaction(input, feature_usage_member_inner)
}

fn feature_usage_member_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::DefaultReferenceUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"feature"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, (name_span, name_str)) = crate::parser::span::with_span(name).parse(input)?;
    let (input, typing_result) = crate::parser::usage::optional_typings(input)?;
    let (typing_span, typing) = typing_result
        .map(|(span, is_conj, targets)| {
            (
                Some(span.clone()),
                Some(crate::parser::usage::typing_node(span, is_conj, targets)),
            )
        })
        .unwrap_or((None, None));
    let (input, spec) = crate::parser::usage::specialization_clauses(input)?;
    let leading_value = spec.subsets.as_ref().and_then(|(_, v)| v.clone());
    let (input, value) =
        opt(preceded(ws_and_comments, crate::parser::feature_value_part)).parse(input)?;
    let value = value.or(leading_value.map(crate::parser::feature_value::wrap_bind_expression));
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::DefaultReferenceUsage {
                name: name_str,
                typing,
                subsets: spec.subsets.map(|(target, _value)| target),
                redefines: spec.redefines,
                value,
                name_span: Some(name_span),
                typing_span,
                membership: crate::ast::Membership::feature(None, crate::ast::Span::dummy()),
                has_feature_keyword: true,
            },
        ),
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
    crate::parser::stack::with_nested_body_stack(|| package_body_brace_inner(input))
}

fn package_body_brace_inner(input: Input<'_>) -> IResult<Input<'_>, PackageBody> {
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
                elements.push(*element);
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
                if let Some(unsupported) = unsupported_package_element(input, next) {
                    elements.push(unsupported);
                    input = next;
                    continue;
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
                        | "unrecognized_declaration_in_scope"
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
                if let Some(unsupported) = unsupported_package_element(input, next) {
                    elements.push(unsupported);
                    input = next;
                    continue;
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
    // Guarded form: skip the attempt entirely when the element's leading keyword selects a
    // different production. `$starter` is `None` when no keyword discriminates here (an
    // unrecognized or prefix-only starter), in which case every alternative is still tried.
    ($input:expr, $start:expr, $starter:expr, $production:ident, $parser:expr, $wrap:expr) => {{
        if !matches!($starter, Some(p) if p != crate::parser::grammar_scope::PackageProduction::$production)
        {
            try_package_body_dispatch!($input, $start, $parser, $wrap);
        }
    }};
    ($input:expr, $start:expr, $parser:expr, $wrap:expr) => {{
        let transaction_input = $input;
        let checkpoint = transaction_input.extra.reference_checkpoint();
        match map($parser, $wrap).parse(transaction_input) {
            Ok((input, elem)) => {
                return Ok((input, Box::new(node_from_to($start, input, elem))));
            }
            Err(_) => transaction_input.extra.rollback_references(checkpoint),
        }
    }};
}

fn try_package_body_annotations<'a>(
    input: Input<'a>,
    start: Input<'a>,
    starter: Option<crate::parser::grammar_scope::PackageProduction>,
) -> IResult<Input<'a>, Box<Node<PackageBodyElement>>> {
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Documentation,
        doc_comment,
        PackageBodyElement::Doc
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Comment,
        comment_annotation,
        PackageBodyElement::Comment
    );
    // GH-91.1: bare `locale "en_US" /* ... */` package member (no `comment` keyword).
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Comment,
        bare_locale_comment,
        PackageBodyElement::Comment
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        TextualRepresentation,
        textual_representation,
        PackageBodyElement::TextualRep
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        ElementFilterMember,
        filter_member,
        PackageBodyElement::Filter
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Attribute,
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
        starter,
        Attribute,
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
    starter: Option<crate::parser::grammar_scope::PackageProduction>,
) -> IResult<Input<'a>, Box<Node<PackageBodyElement>>> {
    try_package_body_dispatch!(
        input,
        start,
        starter,
        LibraryPackage,
        library_package_,
        PackageBodyElement::LibraryPackage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Package,
        package_,
        PackageBodyElement::Package
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Import,
        import_,
        PackageBodyElement::Import
    );
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Alt,
    )))
}

fn try_package_body_structure<'a>(
    input: Input<'a>,
    start: Input<'a>,
    starter: Option<crate::parser::grammar_scope::PackageProduction>,
) -> IResult<Input<'a>, Box<Node<PackageBodyElement>>> {
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Part,
        part_def_or_usage,
        |p| match p {
            PartDefOrUsage::Def(n) => PackageBodyElement::PartDef(n),
            PartDefOrUsage::Usage(n) => PackageBodyElement::PartUsage(n),
        }
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Port,
        port_def,
        PackageBodyElement::PortDef
    );
    // PAR-002: standalone port usage at package level, tried after `port_def` above (which is
    // `def`-optional per its own doc comment and already captures the bare def-less form) -- see
    // `AttributeUsage` above for the same rationale.
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Port,
        port_usage,
        PackageBodyElement::PortUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Interface,
        interface_def,
        PackageBodyElement::InterfaceDef
    );
    // PAR-007: standalone interface usage at package level, tried after `interface_def` above
    // now that `interface_def` rejects a swallowed `connect` clause instead of silently
    // discarding it -- see `interface_def`'s doc comment and `PackageBodyElement::InterfaceUsage`.
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Interface,
        interface_usage,
        PackageBodyElement::InterfaceUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Connection,
        connection_def,
        PackageBodyElement::ConnectionDef
    );
    // PAR-002: standalone connection usage at package level, tried after `connection_def` above
    // for the same reason as `port_usage`.
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Connection,
        crate::parser::part::connection_usage_member,
        PackageBodyElement::ConnectionUsage
    );
    // Standalone `connect a to b;` connector usage at package level (distinct from the
    // `connection <name> : Type` usage form above) -- see the OMG spec Annex `14c-Language
    // Extensions.sysml` FMEA library example.
    try_package_body_dispatch!(
        input,
        start,
        crate::parser::part::connect_,
        PackageBodyElement::Connect
    );
    // `BindingConnectorAsUsage` at package level, e.g. `binding instant[instantNum] of startShot
    // = endShot;` -- see `binding_connector_usage`'s doc comment for the full grammar this
    // covers.
    try_package_body_dispatch!(
        input,
        start,
        starter,
        BindingConnectorAsUsage,
        binding_connector_usage,
        PackageBodyElement::BindingConnectorUsage
    );
    // Package-level `SuccessionAsUsage` (BNF §8.2.2.13.3), e.g. `succession s1 : AB first a then
    // b;`, `first a then b;` -- reuses `first_stmt`, the identical shape already parsed inside
    // action bodies (GH-38).
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Succession,
        crate::parser::action::first_stmt,
        PackageBodyElement::Succession
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Dependency,
        dependency,
        PackageBodyElement::Dependency
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Metadata,
        metadata_usage,
        PackageBodyElement::MetadataUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Metadata,
        metadata_def,
        PackageBodyElement::MetadataDef
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Enumeration,
        enum_def,
        PackageBodyElement::EnumDef
    );
    // PAR-002: standalone enumeration usage at package level, tried after `enum_def` above for
    // the same reason as `port_usage`.
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Enumeration,
        enum_usage,
        PackageBodyElement::EnumerationUsage
    );
    // PAR-002 / GH-10: standalone `ref` / `ref part` at package level.
    // `part_def_or_usage` (earlier in this dispatch) now accepts `ref part …` as PartUsage with
    // `is_reference`. Bare `ref name …` (BNF `ReferenceUsage`) still uses `part_ref_usage`.
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
        starter,
        Allocation,
        allocation_def,
        PackageBodyElement::AllocationDef
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Allocation,
        allocation_usage,
        PackageBodyElement::AllocationUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Allocation,
        allocate_usage,
        PackageBodyElement::AllocationUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Flow,
        flow_def,
        PackageBodyElement::FlowDef
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Flow,
        flow_usage,
        PackageBodyElement::FlowUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Message,
        flow_usage,
        PackageBodyElement::FlowUsage
    );
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Alt,
    )))
}

fn try_package_body_behavior<'a>(
    input: Input<'a>,
    start: Input<'a>,
    starter: Option<crate::parser::grammar_scope::PackageProduction>,
) -> IResult<Input<'a>, Box<Node<PackageBodyElement>>> {
    try_package_body_dispatch!(
        input,
        start,
        starter,
        AliasMember,
        alias_def,
        PackageBodyElement::AliasDef
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Action,
        action_def,
        PackageBodyElement::ActionDef
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Action,
        action_usage,
        PackageBodyElement::ActionUsage
    );
    try_package_body_dispatch!(input, start, state_def, PackageBodyElement::StateDef);
    try_package_body_dispatch!(input, start, state_usage, PackageBodyElement::StateUsage);
    try_package_body_dispatch!(
        input,
        start,
        starter,
        ExhibitStateUsage,
        crate::parser::part::exhibit_state,
        PackageBodyElement::ExhibitState
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        IncludeUseCaseUsage,
        crate::parser::usecase::include_use_case,
        PackageBodyElement::IncludeUseCase
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Item,
        item_def,
        PackageBodyElement::ItemDef
    );
    // PAR-002: standalone item usage at package level, tried after `item_def` above (`def`-
    // optional per its own dispatch here) for the same reason as `port_usage`/`AttributeUsage`.
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Item,
        item_usage,
        PackageBodyElement::ItemUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Individual,
        individual_def,
        PackageBodyElement::IndividualDef
    );
    // KerML `class` classifier definition (e.g. `class B :> A { }`), previously only reachable
    // through the opaque `classifier_decl` fallback -- see `class_def`'s doc comment.
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Class,
        class_def,
        PackageBodyElement::ClassDef
    );
    // KerML bare `feature` usage declaration (e.g. `feature x;`, `feature x : Type;`, `feature x
    // :> Target;`), previously only reachable through the opaque `kerml_feature_decl` fallback --
    // see `feature_usage_member`'s doc comment.
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Feature,
        feature_usage_member,
        PackageBodyElement::DefaultReferenceUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Constraint,
        constraint_def,
        PackageBodyElement::ConstraintDef
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Constraint,
        constraint_usage,
        PackageBodyElement::ConstraintUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Calculation,
        calc_def,
        PackageBodyElement::CalcDef
    );
    // Standalone `perform <action-path>;` performance usage at package level (e.g. `perform
    // process;`, OMG spec Annex `4a-Fundamental Activities.sysml`-style shorthand form).
    // `perform_usage` (shared with part-def/part-usage bodies) already covers the full
    // `perform` action_path (`:>>` target)? (`=` value)? (`;`|`{ }`) grammar.
    try_package_body_dispatch!(
        input,
        start,
        starter,
        PerformActionUsage,
        crate::parser::part::perform_usage,
        PackageBodyElement::PerformUsage
    );
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Alt,
    )))
}

fn try_package_body_requirement<'a>(
    input: Input<'a>,
    start: Input<'a>,
    starter: Option<crate::parser::grammar_scope::PackageProduction>,
) -> IResult<Input<'a>, Box<Node<PackageBodyElement>>> {
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Requirement,
        requirement_def,
        PackageBodyElement::RequirementDef
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Requirement,
        requirement_usage,
        PackageBodyElement::RequirementUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        SatisfyRequirementUsage,
        satisfy,
        PackageBodyElement::Satisfy
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        UseCase,
        use_case_def,
        PackageBodyElement::UseCaseDef
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        UseCase,
        use_case_usage,
        PackageBodyElement::UseCaseUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Case,
        case_def,
        PackageBodyElement::CaseDef
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Case,
        case_usage,
        PackageBodyElement::CaseUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        AnalysisCase,
        analysis_case_def,
        PackageBodyElement::AnalysisCaseDef
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        AnalysisCase,
        analysis_case_usage,
        PackageBodyElement::AnalysisCaseUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        VerificationCase,
        verification_case_def,
        PackageBodyElement::VerificationCaseDef
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        VerificationCase,
        verification_case_usage,
        PackageBodyElement::VerificationCaseUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Concern,
        concern_usage,
        PackageBodyElement::ConcernUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        ActorUsage,
        actor_decl,
        PackageBodyElement::Actor
    );
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Alt,
    )))
}

fn try_package_body_view<'a>(
    input: Input<'a>,
    start: Input<'a>,
    starter: Option<crate::parser::grammar_scope::PackageProduction>,
) -> IResult<Input<'a>, Box<Node<PackageBodyElement>>> {
    try_package_body_dispatch!(
        input,
        start,
        starter,
        View,
        view_def,
        PackageBodyElement::ViewDef
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Viewpoint,
        viewpoint_def,
        PackageBodyElement::ViewpointDef
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Rendering,
        rendering_def,
        PackageBodyElement::RenderingDef
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        View,
        view_usage,
        PackageBodyElement::ViewUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Viewpoint,
        viewpoint_usage,
        PackageBodyElement::ViewpointUsage
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Rendering,
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
        kerml_bare_declaration,
        PackageBodyElement::KermlBareDeclaration
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
) -> IResult<Input<'_>, Box<Node<PackageBodyElement>>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    // One lookup selects the production this element can be: every alternative belonging to a
    // different production is then skipped rather than parsed and rolled back. A prefix-only or
    // unrecognized keyword yields `None`, which keeps the full sequence.
    let starter = crate::parser::grammar_scope::package_body_starter(input.fragment())
        .map(|starter| starter.production)
        .filter(|production| !production.is_prefix());
    if let Ok(r) = try_package_body_annotations(input, start, starter) {
        return Ok(r);
    }
    if let Ok(r) = try_package_body_packages(input, start, starter) {
        return Ok(r);
    }
    if let Ok(r) = try_package_body_structure(input, start, starter) {
        return Ok(r);
    }
    if let Ok(r) = try_package_body_behavior(input, start, starter) {
        return Ok(r);
    }
    if let Ok(r) = try_package_body_requirement(input, start, starter) {
        return Ok(r);
    }
    if let Ok(r) = try_package_body_view(input, start, starter) {
        return Ok(r);
    }
    // `#keyword` metadata tag -- package bodies previously had no `#`/`@` annotation support at
    // all. Tried only after every other dispatcher above: some definitions (e.g. `connection_def`
    // via `DefinitionPrefixOptions::with_derivation_role()`) already capture `#derivation`
    // as their *own* annotation field, and must get first refusal so `#derivation connection {
    // ... }` still becomes one `ConnectionDef` node, not a stray metadata tag followed by a
    // separate (and here invalid, since it'd be missing its own annotation-aware header)
    // `connection` parse. Bare/typed/`about`/body form tried first, then the
    // `PrefixMetadataMember`-style form that prefixes the next member (e.g. `#fmeaspec
    // requirement req1 { ... }`, OMG spec Annex `14c-Language Extensions.sysml`).
    if let Ok((input, elem)) = crate::parser::span::reference_transaction(input, |input| {
        map(
            crate::parser::metadata_annotation::metadata_keyword_usage,
            PackageBodyElement::MetadataKeywordUsage,
        )
        .parse(input)
    }) {
        return Ok((input, Box::new(node_from_to(start, input, elem))));
    }
    if let Ok((input, elem)) = crate::parser::span::reference_transaction(input, |input| {
        map(
            crate::parser::metadata_annotation::metadata_keyword_prefix,
            PackageBodyElement::MetadataKeywordUsage,
        )
        .parse(input)
    }) {
        return Ok((input, Box::new(node_from_to(start, input, elem))));
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
    // GH-89: `assert (not)? (constraint)? <name>? (: Type)? { ... }` at package scope, previously
    // dispatched in six other body contexts but not here, e.g. `assert not massLimitation { :>>
    // mass = vehicle3.mass; :>> massLimit = vehicle4.mass; }` (Simple Tests/ConstraintTest.sysml:89).
    if let Ok((input, elem)) = map(
        crate::parser::occurrence_body::assert_constraint_member,
        PackageBodyElement::AssertConstraint,
    )
    .parse(input)
    {
        return Ok((input, Box::new(node_from_to(start, input, elem))));
    }
    // GH-87: keyword-less `name = expr;` binding (§6 G26), previously only reachable inside
    // part/attribute/action bodies, even though official OMG spec-derived examples use it at
    // package scope: `pressure = force / length^2;` (v1 Spec Examples/8.4.1 Wheel Hub Assembly/
    // Wheel Package.sysml:9) and `T1 = 10.0 [N * m];` (Vehicle Example/VehicleUsages.sysml:14).
    // Tried after every specific dispatcher above (including the `#`/`@` metadata-tag forms) so
    // real keyword-led/typed members keep priority, but before the KerML-opaque fallback below.
    // Deliberately value-*mandatory* (`feature_value_binding`, not the bare-name-permitting
    // `bare_or_valued_feature_binding` used in part def bodies for #87.1): package bodies have
    // their own existing recovery diagnostics for a bare identifier/misused keyword with no value
    // (`unrecognized_identifier_is_not_reported_as_a_keyword` /
    // `misused_real_keyword_is_still_reported_as_unexpected_keyword`,
    // `tests/recovery_diagnostics_integration.rs`) that a permissive bare-`name;` arm here would
    // silently swallow before those diagnostics ever ran -- same class of regression already
    // avoided for action bodies, see `feature_value_binding`'s doc comment.
    if let Ok((input, elem)) = crate::parser::span::reference_transaction(input, |input| {
        map(
            crate::parser::attribute::feature_value_binding,
            PackageBodyElement::DefaultReferenceUsage,
        )
        .parse(input)
    }) {
        return Ok((input, Box::new(node_from_to(start, input, elem))));
    }
    if let Ok((input, elem)) = crate::parser::span::reference_transaction(input, |input| {
        map(
            kerml_bare_declaration,
            PackageBodyElement::KermlBareDeclaration,
        )
        .parse(input)
    }) {
        return Ok((input, Box::new(node_from_to(start, input, elem))));
    }
    if let Ok((input, elem)) =
        map(kerml_feature_decl, PackageBodyElement::KermlFeatureDecl).parse(input)
    {
        return Ok((input, Box::new(node_from_to(start, input, elem))));
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
                    | "unrecognized_declaration_in_scope"
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
    Ok((input, Box::new(node_from_to(start, input, elem))))
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
        let located = crate::parser::span::test_input(std::str::from_utf8(tail).unwrap());

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
        let located = crate::parser::span::test_input(std::str::from_utf8(tail).unwrap());
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
        let located = crate::parser::span::test_input(&input);
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
        let located = crate::parser::span::test_input(&input[start..]);
        let result = package_body_brace(located);
        assert!(
            result.is_ok(),
            "package_body_brace should parse fixture body, got {:?}",
            result
        );
    }

    // --- PAR-002 increment 3: standalone usages at package level ---

    fn parse_input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
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

    // --- PAR-007: connection/interface usage with an inline `connect ... to ...` clause ---
    //
    // Previously `connection link : Link connect a to b;` / `interface iface : IfaceType connect
    // a to b;` at package level were misclassified as `ConnectionDef`/`InterfaceDef`: the plain
    // `: Type` header scan swallowed and silently discarded the `connect ...` clause, so the
    // `def` parser matched an empty-bodied definition instead of leaving the input for the usage
    // parser. See `connection_def`/`interface_def`'s doc comments for the root cause and fix
    // (`DefinitionPrefixOptions::reject_header_keyword`).

    #[test]
    fn package_body_accepts_connection_usage_with_inline_connect_clause() {
        let (rest, node) =
            package_body_element(parse_input("connection link : Link connect a to b;"))
                .expect("connection usage with connect clause");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let PackageBodyElement::ConnectionUsage(usage) = node.value else {
            panic!("expected ConnectionUsage, got {:?}", node.value);
        };
        assert_eq!(usage.value.name.as_deref(), Some("link"));
        assert!(usage.value.type_reference.is_some());
        assert!(usage.value.connect_from.is_some());
        assert!(usage.value.connect_to.is_some());
        assert!(usage.value.connect_extra_ends.is_empty());
    }

    #[test]
    fn package_body_accepts_connection_usage_with_nary_connect_clause() {
        let (rest, node) =
            package_body_element(parse_input("connection link : Link connect (a, b, c);"))
                .expect("connection usage with n-ary connect clause");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let PackageBodyElement::ConnectionUsage(usage) = node.value else {
            panic!("expected ConnectionUsage, got {:?}", node.value);
        };
        assert_eq!(usage.value.connect_extra_ends.len(), 1);
    }

    #[test]
    fn package_body_accepts_interface_usage_with_inline_connect_clause() {
        let (rest, node) =
            package_body_element(parse_input("interface iface : IfaceType connect a to b;"))
                .expect("interface usage with connect clause");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PackageBodyElement::InterfaceUsage(_)));
    }

    /// PAR-006b guard, exercised through the full package-body dispatch this time (not just
    /// `connection_def` in isolation, as in `connection::par_006b_audit_tests`): the real
    /// Systems-Library shape must still classify as `ConnectionDef`, not fall through to
    /// `ConnectionUsage`, since it contains no `connect` keyword.
    #[test]
    fn package_body_still_accepts_bare_systems_library_connection_def_shape() {
        let (rest, node) = package_body_element(parse_input(
            "abstract connection connections: Connection[0..*] nonunique :> linkObjects, parts { }",
        ))
        .expect("bare Systems Library connection def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PackageBodyElement::ConnectionDef(_)));
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

/// `#`/connector support in package bodies -- previously package bodies had no `#`/`@`
/// annotation or `connect a to b;` support at all. Closes the last remaining gap in
/// PARSER_BACKLOG_ROADMAP.md §6's full-validation-suite audit (`14c-Language Extensions.sysml`'s
/// FMEA library example, which prefixes nearly every member with a `#<tag>`).
#[cfg(test)]
mod package_metadata_and_connect_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    #[test]
    fn package_body_accepts_bare_metadata_tag() {
        let (rest, node) = package_body_element(input("#Tag;")).expect("bare metadata tag");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(
            node.value,
            PackageBodyElement::MetadataKeywordUsage(_)
        ));
    }

    #[test]
    fn package_body_accepts_metadata_tag_prefixing_a_requirement() {
        let (rest, node) = package_body_element(input("#fmeaspec requirement req1 { }"))
            .expect("prefix-form metadata tag");
        // Only the tag is consumed; the prefixed requirement is left for the next element.
        assert_eq!(rest.fragment(), b"requirement req1 { }");
        assert!(matches!(
            node.value,
            PackageBodyElement::MetadataKeywordUsage(_)
        ));
    }

    #[test]
    fn package_body_accepts_standalone_connect() {
        let (rest, node) =
            package_body_element(input("connect a to b;")).expect("standalone connect");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(matches!(node.value, PackageBodyElement::Connect(_)));
    }

    /// Regression guard: `connection_def`'s fixed derivation-role prefix must still win over the
    /// new bare
    /// metadata-tag dispatch, or `#derivation connection { ... }` misparses into a stray tag
    /// followed by an unannotated (and here invalid) `connection` declaration.
    #[test]
    fn package_body_prefers_typed_derivation_connection_role() {
        let (rest, node) = package_body_element(input("#derivation connection { end a; end b; }"))
            .expect("connection def with hash annotation");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let PackageBodyElement::ConnectionDef(conn) = node.value else {
            panic!("expected ConnectionDef, got {:?}", node.value);
        };
        assert!(matches!(
            conn.value.derivation_role.as_ref().map(|role| role.value),
            Some(crate::ast::DerivationConnectionRole::Derivation)
        ));
    }
}
