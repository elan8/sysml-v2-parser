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
use crate::parser::item::{item_def_required, item_usage};
use crate::parser::lex::{
    name, qualified_declaration_name, recover_body_element, skip_statement_or_block,
    starts_with_any_keyword, starts_with_keyword, ws1, ws_and_comments,
};
use crate::parser::metadata::{metadata_def, metadata_usage};
use crate::parser::node_from_to;
use crate::parser::occurrence::{occurrence_def, occurrence_usage};
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
pub(crate) fn library_package_(input: Input<'_>) -> IResult<Input<'_>, Node<LibraryPackage>> {
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
pub(crate) fn package_(input: Input<'_>) -> IResult<Input<'_>, Node<Package>> {
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
    let (input, _) = crate::parser::lex::ws_and_notes(input)?;
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
        | PackageBodyElement::Annotating(_)
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
        | PackageBodyElement::CalcUsage(_)
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
        | PackageBodyElement::KermlClassifier(_)
        | PackageBodyElement::KermlConnector(_)
        | PackageBodyElement::KermlRelationship(_)
        | PackageBodyElement::KermlInvariant(_)
        | PackageBodyElement::KermlFeature(_)
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
        | PackageBodyElement::Succession(_)
        | PackageBodyElement::ExhibitState(_)
        | PackageBodyElement::IncludeUseCase(_)
        | PackageBodyElement::ExtendedDefinition(_) => RootElement::Member(boxed),
    };
    Ok((input, node_from_to(start, input, elem)))
}

/// PackageBody: ';' | '{' PackageBodyElement* '}'
/// Brace form is tried first so that ws before '{' is not consumed by the semicolon branch.
pub(crate) fn package_body(input: Input<'_>) -> IResult<Input<'_>, PackageBody> {
    alt((package_body_brace, crate::parser::body::semicolon_body)).parse(input)
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
    let (input, name_span) = if all
        || frag.starts_with(b"[")
        || starts_with_keyword(frag, b"of")
        || starts_with_keyword(frag, b"bind")
    {
        (input, None)
    } else {
        let (input, (span, _text)) = crate::parser::span::with_span(name).parse(input)?;
        (input, Some(span))
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
    let (input, body) = crate::parser::part::ref_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::BindingConnectorUsage {
                all,
                name_span,
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
        | PackageProduction::OccurrenceUsagePrefix
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
/// `interaction DeferredInteraction;`, `predicate deferredPredicate;`, `classifier
/// SpatialFrame;`, and the same shape for every other KerML classifier/feature keyword whose
/// bare (bodyless) form uses it (`class` is handled by [`class_def`] itself, whose body accepts
/// a bare `;` -- see `attribute_body`). Tried before the opaque
/// [`kerml_semantic_decl`]/[`classifier_decl`] fallbacks so this common shape gets a structured
/// node; a keyword followed by a `{` body or a `:>`/`specializes` clause (a different, larger
/// production, e.g. [`class_def`]) falls through to those other productions unchanged.
fn kerml_bare_declaration(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlBareDeclaration>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (keyword_bytes, keyword) = crate::ast::KermlBareDeclarationKeyword::starters()
        .iter()
        .find(|(kw, _)| starts_with_keyword(input.fragment(), kw))
        .ok_or_else(|| {
            nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag))
        })?;
    let (input, _) = nom::bytes::complete::tag(*keyword_bytes).parse(input)?;
    let keyword = *keyword;
    let (input, name_span) =
        opt(preceded(ws1, crate::parser::span::with_span(name))).parse(input)?;
    let name_span = name_span.map(|(span, _text)| span);
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
                name_span,
                multiplicity,
            },
        ),
    ))
}

/// Structured KerML classifier declaration: (visibility)? `abstract`? keyword `all`?
/// Identification multiplicity? (`specializes`/`:>` targets)? type body (Kernel Function/
/// Semantic/Data Type Libraries). Tried before the opaque
/// [`classifier_decl`]/[`kerml_semantic_decl`] fallbacks so these get a typed node; bare
/// `keyword Name;` forward declarations stay on [`kerml_bare_declaration`], which is tried
/// first.
/// KerML explicit relationship declaration (BNF §8.2.4): `specialization S? subtype a
/// specializes b;`, `specialization? subclassifier a specializes b;`, `specialization? typing a
/// typed by b;`, `specialization? subset a subsets b;`, `specialization? redefinition a
/// redefines b;`, `disjoining d? disjoint a from b;`, `inverting i? inverse a of b;`,
/// `featuring (F of)? a by b;` -- each with an annotation-only `RelationshipBody`.
fn kerml_relationship_decl(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlRelationshipDecl>> {
    crate::parser::span::reference_transaction(input, kerml_relationship_decl_inner)
}

fn kerml_relationship_decl_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlRelationshipDecl>> {
    use crate::ast::KermlRelationshipKeyword as Kw;
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;

    // Optional declaration-prefix keyword with its identification.
    let (input, prefixed_identification) =
        if starts_with_keyword(input.fragment(), b"specialization") {
            let (input, _) = tag(&b"specialization"[..]).parse(input)?;
            let (input, _) = ws1(input)?;
            let (input, identification) = crate::parser::lex::identification(input)?;
            (input, Some(identification))
        } else if starts_with_keyword(input.fragment(), b"disjoining") {
            let (input, _) = tag(&b"disjoining"[..]).parse(input)?;
            let (input, _) = ws1(input)?;
            let (input, identification) = crate::parser::lex::identification(input)?;
            (input, Some(identification))
        } else if starts_with_keyword(input.fragment(), b"inverting") {
            let (input, _) = tag(&b"inverting"[..]).parse(input)?;
            let (input, _) = ws1(input)?;
            let (input, identification) = crate::parser::lex::identification(input)?;
            (input, Some(identification))
        } else {
            (input, None)
        };
    let (input, _) = ws_and_comments(input)?;

    // The relationship keyword and its fixed connective.
    let (input, keyword) = if starts_with_keyword(input.fragment(), b"subtype") {
        let (input, _) = tag(&b"subtype"[..]).parse(input)?;
        (input, Kw::Subtype)
    } else if starts_with_keyword(input.fragment(), b"subclassifier") {
        let (input, _) = tag(&b"subclassifier"[..]).parse(input)?;
        (input, Kw::Subclassifier)
    } else if starts_with_keyword(input.fragment(), b"typing") {
        let (input, _) = tag(&b"typing"[..]).parse(input)?;
        (input, Kw::Typing)
    } else if starts_with_keyword(input.fragment(), b"subset") {
        let (input, _) = tag(&b"subset"[..]).parse(input)?;
        (input, Kw::Subset)
    } else if starts_with_keyword(input.fragment(), b"redefinition") {
        let (input, _) = tag(&b"redefinition"[..]).parse(input)?;
        (input, Kw::Redefinition)
    } else if starts_with_keyword(input.fragment(), b"disjoint") {
        let (input, _) = tag(&b"disjoint"[..]).parse(input)?;
        (input, Kw::Disjoint)
    } else if starts_with_keyword(input.fragment(), b"inverse") {
        let (input, _) = tag(&b"inverse"[..]).parse(input)?;
        (input, Kw::Inverse)
    } else if starts_with_keyword(input.fragment(), b"featuring") {
        let (input, _) = tag(&b"featuring"[..]).parse(input)?;
        (input, Kw::Featuring)
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };
    let (input, _) = ws1(input)?;

    // The specialization-family declarations may also spell their identification with the
    // relationship's own keyword doubled: `typing t1 typing f typed by B;`, `subset s1 subset
    // parent subsets f;` (spec42 `kerml/coverage_relationships.md`).
    let (input, doubled_identification) = if prefixed_identification.is_none()
        && matches!(
            keyword,
            Kw::Subtype | Kw::Subclassifier | Kw::Typing | Kw::Subset | Kw::Redefinition
        ) {
        let keyword_bytes: &[u8] = match keyword {
            Kw::Subtype => b"subtype",
            Kw::Subclassifier => b"subclassifier",
            Kw::Typing => b"typing",
            Kw::Subset => b"subset",
            Kw::Redefinition => b"redefinition",
            Kw::Disjoint | Kw::Inverse | Kw::Featuring => unreachable!(),
        };
        let (input, doubled) = opt(map(
            (
                crate::parser::lex::identification,
                ws_and_comments,
                tag(keyword_bytes),
                ws1,
            ),
            |(identification, _, _, _)| identification,
        ))
        .parse(input)?;
        (input, doubled)
    } else {
        (input, None)
    };
    let prefixed_identification = prefixed_identification.or(doubled_identification);
    // `featuring (Identification of)? a by b` names its identification *after* the keyword.
    let (input, identification) = if keyword == Kw::Featuring && prefixed_identification.is_none() {
        let (input, named) = opt(map(
            (
                crate::parser::lex::identification,
                preceded(ws_and_comments, tag(&b"of"[..])),
                ws1,
            ),
            |(identification, _, _)| identification,
        ))
        .parse(input)?;
        (input, named)
    } else {
        (input, prefixed_identification)
    };

    let (input, source) =
        preceded(ws_and_comments, crate::parser::lex::reference_path).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let input = match keyword {
        Kw::Subtype | Kw::Subclassifier => {
            let (input, _) = crate::parser::lex::specialization_operator(input)?;
            input
        }
        Kw::Typing => {
            let (input, _) = crate::parser::lex::typed_by_operator(input)?;
            input
        }
        Kw::Subset => {
            let (input, _) = crate::parser::lex::subset_operator(input)?;
            input
        }
        Kw::Redefinition => {
            let (input, _) = crate::parser::lex::redefine_operator(input)?;
            input
        }
        Kw::Disjoint => {
            let (input, _) = tag(&b"from"[..]).parse(input)?;
            let (input, _) = ws1(input)?;
            input
        }
        Kw::Inverse => {
            let (input, _) = tag(&b"of"[..]).parse(input)?;
            let (input, _) = ws1(input)?;
            input
        }
        Kw::Featuring => {
            let (input, _) = tag(&b"by"[..]).parse(input)?;
            let (input, _) = ws1(input)?;
            input
        }
    };
    let (input, target) =
        preceded(ws_and_comments, crate::parser::lex::reference_path).parse(input)?;
    let (input, body) = crate::parser::body::relationship_body_annotations(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::KermlRelationshipDecl {
                keyword,
                identification,
                source,
                target,
                body,
                membership: crate::ast::Membership::feature(visibility, visibility_span),
            },
        ),
    ))
}

pub(crate) fn kerml_classifier_structured(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlClassifierDecl>> {
    crate::parser::span::reference_transaction(input, kerml_classifier_structured_inner)
}

/// Keywords structurally implemented by [`kerml_classifier_structured`]. `struct` must sort
/// before any longer spelling that would prefix-collide (none currently do;
/// `starts_with_keyword` enforces the word boundary either way).
const KERML_CLASSIFIER_KEYWORDS: &[(&[u8], crate::ast::KermlClassifierKeyword)] = &[
    (b"function", crate::ast::KermlClassifierKeyword::Function),
    (b"datatype", crate::ast::KermlClassifierKeyword::Datatype),
    (b"metaclass", crate::ast::KermlClassifierKeyword::Metaclass),
    (b"struct", crate::ast::KermlClassifierKeyword::Struct),
    (
        b"association",
        crate::ast::KermlClassifierKeyword::Association,
    ),
    (b"assoc", crate::ast::KermlClassifierKeyword::Assoc),
    (b"behavior", crate::ast::KermlClassifierKeyword::Behavior),
    (
        b"interaction",
        crate::ast::KermlClassifierKeyword::Interaction,
    ),
    (b"predicate", crate::ast::KermlClassifierKeyword::Predicate),
    (
        b"multiplicity",
        crate::ast::KermlClassifierKeyword::Multiplicity,
    ),
    (b"type", crate::ast::KermlClassifierKeyword::Type),
    (
        b"classifier",
        crate::ast::KermlClassifierKeyword::Classifier,
    ),
    (b"class", crate::ast::KermlClassifierKeyword::Class),
];

/// Whether this member starts the `abstract`-prefixed spelling of one of this parser's KerML
/// classifier declarations.
///
/// This is a dispatch-only lookahead: it consumes no input and allocates no references. A KerML
/// `BasicFeaturePrefix` also owns `abstract`, so body parsers that route malformed feature-prefix
/// chains to recovery must make this distinction before they try the feature arm. Keep the
/// keyword list here beside the parser that owns the classifier production rather than giving
/// every body scope a second copy of its FIRST set.
pub(crate) fn starts_abstract_kerml_classifier(input: Input<'_>) -> bool {
    let Ok((input, _)) = ws_and_comments(input) else {
        return false;
    };
    let Ok((input, _)) = crate::parser::lex::visibility_prefix(input) else {
        return false;
    };
    let Some((input, _)) = crate::parser::occurrence_prefix::slot_keyword(input, b"abstract")
    else {
        return false;
    };
    KERML_CLASSIFIER_KEYWORDS
        .iter()
        .any(|(keyword, _)| starts_with_keyword(input.fragment(), keyword))
}

/// Zero or more KerML type relationship clauses following a classifier header: `disjoint from
/// A, B`, `unions A, B`, `intersects A, B` (any order, repeatable).
pub(crate) fn kerml_type_relationship_clauses(
    input: Input<'_>,
) -> IResult<Input<'_>, Vec<Node<crate::ast::KermlTypeRelationship>>> {
    let mut out = Vec::new();
    let mut input = input;
    loop {
        let before = input;
        let (after_ws, _) = ws_and_comments(input)?;
        let (rest, keyword) = if starts_with_keyword(after_ws.fragment(), b"disjoint") {
            let (rest, _) = tag(&b"disjoint"[..]).parse(after_ws)?;
            let (rest, _) = ws1(rest)?;
            let (rest, _) = tag(&b"from"[..]).parse(rest)?;
            let (rest, _) = ws1(rest)?;
            (rest, crate::ast::KermlTypeRelationshipKeyword::DisjointFrom)
        } else if starts_with_keyword(after_ws.fragment(), b"unions") {
            let (rest, _) = tag(&b"unions"[..]).parse(after_ws)?;
            let (rest, _) = ws1(rest)?;
            (rest, crate::ast::KermlTypeRelationshipKeyword::Unions)
        } else if starts_with_keyword(after_ws.fragment(), b"intersects") {
            let (rest, _) = tag(&b"intersects"[..]).parse(after_ws)?;
            let (rest, _) = ws1(rest)?;
            (rest, crate::ast::KermlTypeRelationshipKeyword::Intersects)
        } else if starts_with_keyword(after_ws.fragment(), b"differences") {
            let (rest, _) = tag(&b"differences"[..]).parse(after_ws)?;
            let (rest, _) = ws1(rest)?;
            (rest, crate::ast::KermlTypeRelationshipKeyword::Differences)
        } else {
            return Ok((input, out));
        };
        let (rest, first) = crate::parser::lex::qualified_reference(rest)?;
        let (rest, more) = nom::multi::many0(preceded(
            preceded(ws_and_comments, tag(&b","[..])),
            preceded(ws_and_comments, crate::parser::lex::qualified_reference),
        ))
        .parse(rest)?;
        let mut targets = vec![first];
        targets.extend(more);
        let span = crate::parser::span_from_to(before, rest);
        out.push(Node::new(
            span.clone(),
            crate::ast::KermlTypeRelationship {
                keyword,
                targets,
                span,
            },
        ));
        input = rest;
    }
}

fn kerml_classifier_structured_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<crate::ast::KermlClassifierDecl>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, is_abstract) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (keyword_bytes, keyword) = KERML_CLASSIFIER_KEYWORDS
        .iter()
        .find(|(kw, _)| starts_with_keyword(input.fragment(), kw))
        .ok_or_else(|| {
            nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag))
        })?;
    let (input, _) = tag(*keyword_bytes).parse(input)?;
    let mut keyword = *keyword;
    let (input, _) = ws1(input)?;
    // `assoc struct LinkObject specializes Link, Object ...` (Kernel Semantic Library
    // `Objects.kerml`): the compound keyword pair gets its own variant.
    let (input, second) = if keyword == crate::ast::KermlClassifierKeyword::Assoc {
        opt(preceded(tag(&b"struct"[..]), ws1)).parse(input)?
    } else {
        (input, None)
    };
    if second.is_some() {
        keyword = crate::ast::KermlClassifierKeyword::AssocStruct;
    }
    let (input, is_all) = opt(preceded(tag(&b"all"[..]), ws1)).parse(input)?;
    let (input, identification) = crate::parser::lex::identification(input)?;
    let (input, multiplicity) = opt(preceded(
        ws_and_comments,
        crate::parser::usage::multiplicity_node,
    ))
    .parse(input)?;
    let (input, specializes) =
        crate::parser::specialization::parse_optional_definition_specialization(input)?;
    // The `bool`/`expr` feature forms type with `:` instead of specializing:
    // `bool earlierFirstIncomingTransferSort : IncomingTransferSort { ... }`
    // (`Occurrences.kerml`). The relationship kind distinguishes the authored operator.
    let (input, specializes) = if specializes.is_none() {
        let (peek, _) = ws_and_comments(input)?;
        if peek.fragment().starts_with(b":") && !peek.fragment().starts_with(b":>") {
            let before = input;
            let (input, _) = preceded(ws_and_comments, tag(&b":"[..])).parse(input)?;
            let (input, target) =
                preceded(ws_and_comments, crate::parser::lex::qualified_reference).parse(input)?;
            let span = crate::parser::span_from_to(before, input);
            (
                input,
                Some(Node::new(
                    span.clone(),
                    crate::ast::TypingRelationship {
                        target: vec![target],
                        kind: crate::ast::TypingKind::Typing,
                        span,
                        is_conjugated: false,
                        is_implied: false,
                        spelling: crate::ast::TypingSpelling::Operator,
                    },
                )),
            )
        } else {
            (input, None)
        }
    } else {
        (input, specializes)
    };
    // KerML type relationship clauses: `disjoint from A, B`, `unions A, B`, `intersects A, B`
    // (`Occurrences.kerml`, `VectorValues.kerml`). Any order, repeatable.
    let (input, type_relationships) = kerml_type_relationship_clauses(input)?;
    let (input, body) = crate::parser::constraint::calc_def_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            crate::ast::KermlClassifierDecl {
                is_abstract: is_abstract.is_some(),
                keyword,
                is_all: is_all.is_some(),
                identification,
                multiplicity,
                specializes,
                type_relationships,
                body,
                membership: crate::ast::Membership::owning(visibility, visibility_span),
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

// The bare-`feature` package member (`feature x : Integer;`, `feature f { expr s { ... } }`)
// now parses through `kerml_feature` -- one typed representation for every
// `feature`-keyword-led member across package and type-body scopes (spec42 gap 14). The
// previous `DefaultReferenceUsage`-shaped `feature_usage_member` production and its expression
// body machinery were removed with it; `expr s { ... }`
// members now parse as feature members of kind `expr` inside the shared type-body grammar.
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
    let (open_start, _) = ws_and_comments(input)?;
    let (mut input, _) = tag(&b"{"[..]).parse(open_start)?;
    let open_span = crate::parser::span::span_from_to(open_start, input);
    let mut elements = Vec::new();
    loop {
        // See `crate::parser::body::parse_structured_brace_members_inner`: a bare `/* ... */` at
        // a member boundary is the `Comment` production, not trivia.
        let (next, _) = crate::parser::lex::ws_and_notes(input)?;
        input = next;
        if input.fragment().is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Eof,
            )));
        }
        if input.fragment().starts_with(b"}") {
            let (close_start, _) = ws_and_comments(input)?;
            let (input, _) = tag(&b"}"[..]).parse(close_start)?;
            return Ok((
                input,
                PackageBody::Brace {
                    open_span,
                    elements,
                    close_span: crate::parser::span::span_from_to(close_start, input),
                },
            ));
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
                        | "malformed_annotation_head"
                ) {
                    elements.push(node_from_to(
                        input,
                        next,
                        PackageBodyElement::Error(node_from_to(input, next, recovery)),
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
                    PackageBodyElement::Error(node_from_to(input, next, recovery)),
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
                    PackageBodyElement::Error(node_from_to(input, next, recovery)),
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
        |member| PackageBodyElement::Annotating(crate::ast::AnnotatingMember::Doc(member))
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Comment,
        comment_annotation,
        |member| PackageBodyElement::Annotating(crate::ast::AnnotatingMember::Comment(member))
    );
    // GH-91.1: bare `locale "en_US" /* ... */` package member (no `comment` keyword).
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Comment,
        bare_locale_comment,
        |member| PackageBodyElement::Annotating(crate::ast::AnnotatingMember::Comment(member))
    );
    try_package_body_dispatch!(
        input,
        start,
        starter,
        TextualRepresentation,
        textual_representation,
        |member| PackageBodyElement::Annotating(crate::ast::AnnotatingMember::TextualRep(member))
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
        attribute_def,
        PackageBodyElement::AttributeDef
    );
    // PAR-002: standalone attribute usage at package level (BNF `PackageMember` allows
    // `DefinitionElement | UsageElement`). `attribute_def` requires its authored `def` token, so
    // this arm owns every def-less attribute form, including the ordinary typed/value headers.
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
    try_package_body_dispatch!(input, start, starter, Port, port_usage, |p| {
        PackageBodyElement::PortUsage(Box::new(p))
    });
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
        crate::parser::connector::ref_decl,
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
    try_package_body_dispatch!(input, start, starter, Flow, flow_usage, |usage| {
        PackageBodyElement::FlowUsage(Box::new(usage))
    });
    try_package_body_dispatch!(input, start, starter, Message, flow_usage, |usage| {
        PackageBodyElement::FlowUsage(Box::new(usage))
    });
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
    // GH-90.2 / gap #7: package-level `item_def` requires the `def` keyword (mirroring
    // `action_def`/`state_def`) so a bodyless `individual item i1;` short usage form isn't
    // misclassified as an `ItemDef` with `i1` as the definition's identification name. Without
    // `.def_required()`, `item_def`'s `individual_allowed()` option let it match `individual item
    // i1;` before `item_usage` (below) was ever tried.
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Item,
        item_def_required,
        PackageBodyElement::ItemDef
    );
    // PAR-002: standalone item usage at package level, tried after `item_def` above.
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
    // KerML bare `feature` usage declaration (e.g. `feature x;`, `feature x : Type;`, `feature x
    // :> Target;`), previously only reachable through the opaque `kerml_feature_decl` fallback --
    // see `feature_usage_member`'s doc comment.
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Feature,
        crate::parser::constraint::kerml_feature,
        |n| { PackageBodyElement::KermlFeature(Box::new(n)) }
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
    // After `calc_def`, never before it: `calc_def` is deliberately `def`-optional here (see
    // `definition_prefix`'s module doc) because the Systems Library authors bare `calc Name : T;`
    // definitions at namespace level. This arm catches only what that grammar refuses -- a
    // `CalculationUsage`'s multiplicity, `ordered`/`nonunique` and value clauses -- which
    // previously fell through to the unimplemented extended-library declaration.
    try_package_body_dispatch!(
        input,
        start,
        starter,
        Calculation,
        crate::parser::constraint::calc_usage,
        PackageBodyElement::CalcUsage
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
    // `satisfy`, `not satisfy`, `assert satisfy`, and `assert not satisfy` are all one
    // production, so `assert` is a FIRST keyword of two package-body members. The starter table
    // can only name one production per keyword, so the `assert`-led attempt is spelled out here;
    // it fails immediately unless `satisfy` actually follows, leaving `AssertConstraintUsage` to
    // the fallback dispatcher further down. Without it a package-level `assert satisfy r by q;`
    // (`Simple Tests/RequirementTest.sysml:27`) fell through to recovery.
    try_package_body_dispatch!(input, start, starter, AssertConstraintUsage, satisfy, |n| {
        PackageBodyElement::Satisfy(Box::new(n))
    });
    try_package_body_dispatch!(
        input,
        start,
        starter,
        SatisfyRequirementUsage,
        satisfy,
        |n| PackageBodyElement::Satisfy(Box::new(n))
    );
    // A prefixed satisfy usage leads with an `OccurrenceUsagePrefix` token, and the starter table
    // maps each of those to a prefix production (`#`, `abstract`/`variation`, `individual`, the
    // `UsagePrefix` keywords) or -- for `snapshot`/`timeslice` -- to `Occurrence`, never to this
    // one. The two guarded attempts above are therefore skipped for exactly the spellings the
    // prefix seam added, so attempt it again for those starters; the parse fails immediately
    // unless `satisfy` really follows the prefix, and the transaction rolls the arena back when
    // it does not.
    if starter.is_some_and(|production| {
        production.is_prefix()
            || production == crate::parser::grammar_scope::PackageProduction::Occurrence
    }) {
        try_package_body_dispatch!(input, start, satisfy, |n| PackageBodyElement::Satisfy(
            Box::new(n)
        ));
    }
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
    // Bare `classifier`/`class` forward declarations (e.g. `classifier SpatialFrame;`, `class
    // B;`) are tried before the opaque `classifier_decl` fallback so this common shape gets a
    // structured node -- see `kerml_bare_declaration`'s doc comment.
    try_package_body_dispatch!(input, start, kerml_relationship_decl, |n| {
        PackageBodyElement::KermlRelationship(Box::new(n))
    });
    // Classifier declarations (incl. bare `classifier X;`-style forward declarations, which get
    // the same typed node with a `;` body) before the bare-declaration fallback, so every
    // keyword the structured production covers becomes a resolvable named declaration
    // (spec42 gap 13).
    try_package_body_dispatch!(input, start, kerml_classifier_structured, |n| {
        PackageBodyElement::KermlClassifier(Box::new(n))
    });
    try_package_body_dispatch!(
        input,
        start,
        crate::parser::constraint::kerml_feature,
        |n| { PackageBodyElement::KermlFeature(Box::new(n)) }
    );
    // Bare forward declarations for the keywords the structured productions above do not
    // cover (`inv x;`, `occurrence o;`, `succession s;`, ...).
    try_package_body_dispatch!(
        input,
        start,
        kerml_bare_declaration,
        PackageBodyElement::KermlBareDeclaration
    );
    try_package_body_dispatch!(input, start, feature_decl, PackageBodyElement::FeatureDecl);
    try_package_body_dispatch!(
        input,
        start,
        crate::parser::constraint::kerml_connector_member,
        |n| { PackageBodyElement::KermlConnector(Box::new(n)) }
    );
    try_package_body_dispatch!(
        input,
        start,
        crate::parser::constraint::kerml_invariant_member,
        |n| { PackageBodyElement::KermlInvariant(Box::new(n)) }
    );
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
    // Keyword-less implicit-feature shorthand, last so every keyword-led production keeps
    // priority: bare `x;`, `y = expr;`, `z : Type;` package members (spec42 gap 23).
    try_package_body_dispatch!(
        input,
        start,
        crate::parser::attribute::default_reference_usage,
        |n| { PackageBodyElement::DefaultReferenceUsage(n) }
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
    let (input, _) = crate::parser::lex::ws_and_notes(input)?;
    // The keyword-less `Comment` spelling, ahead of the starter lookup: `/*` selects no
    // production keyword, so the lookup would classify the member as unrecognized.
    if input.fragment().starts_with(b"/*") {
        let start = input;
        if let Ok((rest, member)) = crate::parser::requirement::bare_comment(input) {
            return Ok((
                rest,
                Box::new(node_from_to(
                    start,
                    rest,
                    PackageBodyElement::Annotating(crate::ast::AnnotatingMember::Comment(member)),
                )),
            ));
        }
    }
    let start = input;
    if crate::parser::attribute::is_pilot_end_default_reference(input) {
        return Err(nom::Err::Failure(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    // One lookup selects the production this element can be: every alternative belonging to a
    // different production is then skipped rather than parsed and rolled back. A prefix-only or
    // unrecognized keyword yields `None`, which keeps the full sequence.
    let starter = crate::parser::grammar_scope::package_body_starter(input.fragment())
        .map(|starter| starter.production)
        .filter(|production| !production.is_prefix());
    // `ref` is an `OccurrenceUsagePrefix` slot, while `ref case` and `ref verification` remain
    // grammar-owned `RefDecl` spellings. Give only a *complete* AnalysisCaseUsage first refusal:
    // Systems Library `AnalysisCases.sysml:21`'s `ref analysis self ...` otherwise reaches the
    // general reference path before the analysis dispatcher. A failed attempt rolls its arena
    // work back and leaves the existing RefDecl priority untouched for every other `ref` kind.
    if crate::parser::occurrence_prefix::starts_contended_prefix(input) {
        if let Ok((next, usage)) = analysis_case_usage(input) {
            return Ok((
                next,
                Box::new(node_from_to(
                    start,
                    next,
                    PackageBodyElement::AnalysisCaseUsage(usage),
                )),
            ));
        }
        // `ref action` / `ref state` have existing package-body variants. Give their complete
        // typed parsers first refusal over the generic RefDecl paths in structure dispatch;
        // transactions preserve every other `ref` spelling and its arena state on failure.
        if starts_with_keyword(input.fragment(), b"ref") {
            if let Ok((next, usage)) =
                crate::parser::span::reference_transaction(input, action_usage)
            {
                return Ok((
                    next,
                    Box::new(node_from_to(
                        start,
                        next,
                        PackageBodyElement::ActionUsage(usage),
                    )),
                ));
            }
            if let Ok((next, usage)) =
                crate::parser::span::reference_transaction(input, state_usage)
            {
                return Ok((
                    next,
                    Box::new(node_from_to(
                        start,
                        next,
                        PackageBodyElement::StateUsage(usage),
                    )),
                ));
            }
        }
    }
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
    // `ExtendedDefinition` (SysML §8.2.2.27): `#<keyword>+ def <Name> ...`. Must be tried before
    // `metadata_keyword_usage`/`metadata_keyword_prefix` below get first refusal on a `#<name>`
    // sequence immediately followed by `def` -- `def Failure;` alone is not an independently
    // valid production, so leaving it for the next body-element iteration (as
    // `metadata_keyword_prefix` does for ordinary `PrefixMetadataMember` prefixes) would hit raw
    // recovery. Speculative: rolls back cleanly (via `reference_transaction`) to the ordinary
    // `#`-forms below when the `def`/name tail isn't present.
    if let Ok((input, elem)) = crate::parser::span::reference_transaction(input, |input| {
        map(
            crate::parser::metadata_annotation::extended_definition,
            PackageBodyElement::ExtendedDefinition,
        )
        .parse(input)
    }) {
        return Ok((input, Box::new(node_from_to(start, input, elem))));
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
    // `@ Name (: Type)? (about target(, target)*)? body` standalone annotation statement
    // (KerML `Annotation`/`@`-syntax) -- previously only the `#`-keyword forms above were
    // dispatched at package scope, e.g. `@ Classified about Annotated;`.
    if let Ok((input, elem)) = crate::parser::span::reference_transaction(input, |input| {
        map(
            crate::parser::metadata_annotation::metadata_annotation,
            |member| {
                PackageBodyElement::Annotating(crate::ast::AnnotatingMember::MetadataAnnotation(
                    member,
                ))
            },
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
    // `ws_and_notes` at every member boundary, matching the editor entry point's own loop: a bare
    // `/* ... */` between root members is the `Comment` production, and skipping it here would
    // make strict and editor parsing disagree about the document.
    let (input, _) = crate::parser::lex::ws_and_notes(input)?;
    let (input, elements) =
        many0(preceded(crate::parser::lex::ws_and_notes, root_element)).parse(input)?;
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
        let crate::ast::PartDefBody::Brace { elements, .. } = body else {
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
        // Only the tag is consumed -- up to the end of its reference, not past the
        // whitespace after it, so the node span covers `#fmeaspec` and nothing else. The
        // prefixed requirement is left for the next element.
        assert_eq!(rest.fragment(), b" requirement req1 { }");
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
