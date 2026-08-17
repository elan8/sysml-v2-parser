//! Metadata/annotation parsing helpers.

use crate::ast::{
    DefinitionPrefix, ExtendedDefinition, MetadataAnnotation, MetadataDeclaredName,
    MetadataFeatureIntroducer, MetadataKeywordUsage, MetadataTypedBy, Node,
};
use crate::parser::attribute::metadata_body;
use crate::parser::lex::{
    identification, qualified_reference, starts_with_keyword, ws1, ws_and_comments,
};
use crate::parser::node_from_to;
use crate::parser::package::package_body;
use crate::parser::specialization::parse_optional_definition_specialization;
use crate::parser::with_span;
use crate::parser::Input;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// Optional `about` qualifiedName (, qualifiedName)* clause (SysML §7.27.2).
pub(crate) fn parse_about_targets(
    input: Input<'_>,
) -> IResult<Input<'_>, Vec<crate::ast::QualifiedReferenceId>> {
    let (input, _) = ws_and_comments(input)?;
    if !starts_with_keyword(input.fragment(), b"about") {
        return Ok((input, Vec::new()));
    }
    let (input, _) = tag(&b"about"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    separated_list1(
        preceded(ws_and_comments, preceded(tag(&b","[..]), ws_and_comments)),
        qualified_reference,
    )
    .parse(input)
}

/// `MetadataFeature` (KerML 8.2.5.12, SysML 8.2.2.27):
///
/// ```text
/// PrefixMetadataMember* ( '@' | 'metadata' ) MetadataFeatureDeclaration
///     ( 'about' Annotation ( ',' Annotation )* )? MetadataBody
/// MetadataFeatureDeclaration = ( Identification ( ':' | 'typed' 'by' ) )? OwnedFeatureTyping
/// ```
pub(crate) fn metadata_annotation(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<MetadataAnnotation>> {
    crate::parser::span::reference_transaction(input, metadata_annotation_inner)
}

/// `MetadataFeatureDeclaration`'s optional `Identification ( ':' | 'typed' 'by' )` prefix.
///
/// Tried before the bare `OwnedFeatureTyping` and required to reach a separator keyword, so
/// `@Tag;` -- whose single qualified name *is* the typing -- never has `Tag` mistaken for a
/// declared name. The whole attempt is inside the caller's reference transaction, so the
/// speculative `Identification` costs no arena entry when it does not pan out.
fn metadata_declared_name(input: Input<'_>) -> IResult<Input<'_>, Node<MetadataDeclaredName>> {
    let (start, _) = ws_and_comments(input)?;
    let (input, ident) = identification(start)?;
    let (input, _) = ws_and_comments(input)?;
    let separator_start = input;
    let (input, typed_by) = if input.fragment().starts_with(b":")
        && !input.fragment().starts_with(b":>")
        && !input.fragment().starts_with(b"::")
    {
        let (input, _) = tag(&b":"[..]).parse(input)?;
        (input, MetadataTypedBy::Colon)
    } else if starts_with_keyword(input.fragment(), b"typed") {
        let (input, _) = tag(&b"typed"[..]).parse(input)?;
        let (input, _) = ws1(input)?;
        let (input, _) = tag(&b"by"[..]).parse(input)?;
        (input, MetadataTypedBy::TypedBy)
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };
    Ok((
        input,
        node_from_to(
            start,
            input,
            MetadataDeclaredName {
                identification: ident,
                typed_by,
                typed_by_span: crate::parser::span::span_from_to(separator_start, input),
            },
        ),
    ))
}

fn metadata_annotation_inner(input: Input<'_>) -> IResult<Input<'_>, Node<MetadataAnnotation>> {
    let start = input;
    let (input, prefixes) =
        nom::multi::many0(preceded(ws_and_comments, extended_definition_prefix_tag))
            .parse(input)?;
    let (introducer_start, _) = ws_and_comments(input)?;
    let (input, introducer) = if introducer_start.fragment().starts_with(b"@") {
        let (input, _) = tag(&b"@"[..]).parse(introducer_start)?;
        (
            input,
            MetadataFeatureIntroducer::At {
                span: crate::parser::span::span_from_to(introducer_start, input),
            },
        )
    } else if starts_with_keyword(introducer_start.fragment(), b"metadata") {
        let (input, _) = tag(&b"metadata"[..]).parse(introducer_start)?;
        (
            input,
            MetadataFeatureIntroducer::Metadata {
                span: crate::parser::span::span_from_to(introducer_start, input),
            },
        )
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            introducer_start,
            nom::error::ErrorKind::Tag,
        )));
    };
    let (input, declared_name) = opt(metadata_declared_name).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (type_span, type_reference)) = with_span(qualified_reference).parse(input)?;
    let (input, about_targets) = parse_about_targets(input)?;
    let (input, body) = metadata_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            MetadataAnnotation {
                prefixes,
                introducer,
                declared_name,
                type_reference,
                type_span,
                about_targets,
                body,
            },
        ),
    ))
}

/// The `'#' OwnedFeatureTyping` head shared by every `#` production, without its continuation.
///
/// `PrefixMetadataFeature : MetadataFeature = ownedRelationship += OwnedFeatureTyping`, and
/// `OwnedFeatureTyping` is `[QualifiedName]`, so this is a reference -- qualified (`#ISQ::mass`)
/// or quoted (`#'safety critical'`) where the author wrote one -- and never a fabricated name.
/// Each caller decides which production it is by what it requires *after* this head.
fn metadata_keyword_head(
    input: Input<'_>,
) -> IResult<Input<'_>, (crate::ast::Span, crate::ast::QualifiedReferenceId)> {
    let (hash_start, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"#"[..]).parse(hash_start)?;
    let hash_span = crate::parser::span::span_from_to(hash_start, input);
    let (input, _) = ws_and_comments(input)?;
    let (input, reference) = qualified_reference(input)?;
    Ok((input, (hash_span, reference)))
}

/// `ExtendedUsage` with an empty `UsageDeclaration` -- the standalone `#Tag;` / `#Tag { ... }`
/// member spelling (`UnextendedUsagePrefix UsageExtensionKeyword+ Usage`, SysML 8.2.2.27, where
/// every part of `Usage` except its `UsageBody` is empty).
///
/// The body is what distinguishes this from [`metadata_keyword_prefix`], so a head not followed
/// by `;` or `{` is refused here rather than accepted with an invented terminator. Neither
/// `: Type` nor `about` is accepted: `PrefixMetadataFeature` has neither, and the `about` clause
/// belongs to `MetadataFeature`, which `#` does not reach in either layer.
pub(crate) fn metadata_keyword_usage(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<MetadataKeywordUsage>> {
    crate::parser::span::reference_transaction(input, metadata_keyword_usage_inner)
}

fn metadata_keyword_usage_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<MetadataKeywordUsage>> {
    let start = input;
    let (input, (hash_span, reference)) = metadata_keyword_head(input)?;
    let (peek, _) = ws_and_comments(input)?;
    if !(peek.fragment().starts_with(b";") || peek.fragment().starts_with(b"{")) {
        return Err(nom::Err::Error(nom::error::Error::new(
            peek,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, body) = metadata_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            MetadataKeywordUsage {
                hash_span,
                reference,
                body: Some(body),
            },
        ),
    ))
}

/// A single bare `#<name>` prefix keyword tag, as used by [`extended_definition`]'s
/// `DefinitionExtensionKeyword+` (SysML §8.2.2.27). Unlike [`metadata_keyword_prefix`], this
/// makes no attempt to guess whether what follows "looks like" another declaration -- the caller
/// (`many1` followed by a required `def`) is what decides whether the whole
/// `ExtendedDefinition` production applies, inside a speculative transaction that rolls back
/// cleanly if it doesn't.
fn extended_definition_prefix_tag(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<MetadataKeywordUsage>> {
    let start = input;
    let (input, (hash_span, reference)) = metadata_keyword_head(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            MetadataKeywordUsage {
                hash_span,
                reference,
                body: None,
            },
        ),
    ))
}

/// `ExtendedDefinition` (SysML §8.2.2.27): `DefinitionExtensionKeyword+ 'def' DefinitionDeclaration
/// DefinitionBody`, e.g. `#situation def Failure;`, `#SecurityRelated #situation def
/// Vulnerability;`, `abstract #situation def AbstractFailure;`, `variation #situation def V;`.
/// Tried in package-body dispatch *before* [`metadata_keyword_prefix`] gets first refusal on a
/// `#<name>` sequence immediately followed by `def` -- `def Failure;` alone is not an
/// independently valid production, unlike the ordinary `PrefixMetadataMember` shape
/// (`#fmeaspec requirement req1 { ... }`) that `metadata_keyword_prefix` exists for.
pub(crate) fn extended_definition(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<ExtendedDefinition>> {
    crate::parser::span::reference_transaction(input, extended_definition_inner)
}

fn extended_definition_inner(input: Input<'_>) -> IResult<Input<'_>, Node<ExtendedDefinition>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, definition_prefix) = opt(nom::branch::alt((
        map(preceded(tag(&b"abstract"[..]), ws1), |_| {
            DefinitionPrefix::Abstract
        }),
        map(preceded(tag(&b"variation"[..]), ws1), |_| {
            DefinitionPrefix::Variation
        }),
    )))
    .parse(input)?;
    let (input, prefix_keywords) =
        many1(preceded(ws_and_comments, extended_definition_prefix_tag)).parse(input)?;
    // `def` is optional: the bare extended-usage shorthand `#clouddd ArrowheadCore { ... }`
    // (spec42 Gap 39) has no declaration keyword at all. The keyword guard below keeps the
    // `#fmeaspec requirement req1 { ... }` `PrefixMetadataMember` shape on
    // `metadata_keyword_prefix`.
    let (input, def_kw) =
        opt(preceded(preceded(ws_and_comments, tag(&b"def"[..])), ws1)).parse(input)?;
    let (input, ident) = identification(input)?;
    if def_kw.is_none() {
        let name_is_usable = ident
            .name
            .as_deref()
            .is_some_and(|n| !crate::parser::lex::is_reserved_keyword(n.as_bytes()));
        if !name_is_usable {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
    }
    let (input, specializes) = parse_optional_definition_specialization(input)?;
    let (input, body) = package_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            ExtendedDefinition {
                prefix_keywords,
                definition_prefix,
                has_def_keyword: def_kw.is_some(),
                identification: ident,
                specializes,
                body,
            },
        ),
    ))
}

/// Bare `#keyword` used as a `PrefixMetadataMember`-style tag on the *following* declaration,
/// rather than owning its own body (SysML §8.2.3.2): `#fmeaspec requirement req1 { ... }`,
/// `#prevention connect a to b;` (OMG spec Annex `14c-Language Extensions.sysml`, FMEA library
/// example). Stops right after the keyword -- the prefixed declaration is left unconsumed for
/// the caller's next body-element iteration to parse as its own element, so `#fmeaspec
/// requirement req1 { ... }` becomes two sibling body elements (a bare `MetadataKeywordUsage`
/// tag, then the real `RequirementUsage`) rather than one combined node.
///
/// Deliberately a separate function from [`metadata_keyword_usage`], not a widening of its
/// guard: the two are different productions. `metadata_keyword_usage` is `ExtendedUsage` with an
/// empty declaration and owns a body; this is `PrefixMetadataMember` and owns nothing but the
/// reference, leaving the prefixed declaration for the caller's next member iteration.
///
/// This intentionally doesn't attempt to resolve whether the keyword is actually a declared
/// `metadata def <keyword> ...` short name (that semantic check, plus a package-local short-name
/// index, is the larger deferred 1.5b item in PARSER_BACKLOG_ROADMAP.md §2.1) -- syntactically,
/// any bare `#<name>` immediately followed by what looks like the start of another declaration
/// (an identifier) is accepted. Stacked prefixes (`#Approval #Classified part def X;`) are
/// accepted the same way -- each `#tag` is a separate `MetadataKeywordUsage` prefix, so a
/// following `#` (the start of the next stacked tag) must be accepted here too, or the whole
/// second `#tag` onward is left for `metadata_keyword_usage`/`metadata_keyword_prefix` to retry
/// on the following body-element dispatch pass.
pub(crate) fn metadata_keyword_prefix(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<MetadataKeywordUsage>> {
    crate::parser::span::reference_transaction(input, metadata_keyword_prefix_inner)
}

fn metadata_keyword_prefix_inner(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<MetadataKeywordUsage>> {
    let start = input;
    let (input, (hash_span, reference)) = metadata_keyword_head(input)?;
    let (peek, _) = ws_and_comments(input)?;
    if !peek
        .fragment()
        .first()
        .is_some_and(|b| b.is_ascii_alphabetic() || *b == b'_' || *b == b'\'' || *b == b'#')
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            peek,
            nom::error::ErrorKind::Tag,
        )));
    }
    Ok((
        input,
        node_from_to(
            start,
            input,
            MetadataKeywordUsage {
                hash_span,
                reference,
                body: None,
            },
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::SourceStorage;
    use crate::parser::span::ParseContext;

    /// Spec42 Gap 39: the bare extended-usage shorthand (`#clouddd ArrowheadCore { ... }`) has
    /// no `def` keyword; the keyword guard keeps `#tag <keyword-led member>` shapes on
    /// `metadata_keyword_prefix`.
    #[test]
    fn extended_definition_accepts_the_def_less_usage_shorthand() {
        let context = ParseContext::new();
        let input = context.input(b"#clouddd ArrowheadCore { part x; }");
        let (rest, node) = extended_definition(input).expect("extended usage shorthand");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(!node.value.has_def_keyword);
        assert_eq!(
            node.value.identification.name.as_deref(),
            Some("ArrowheadCore")
        );

        let context = ParseContext::new();
        let input = context.input(b"#situation def Failure;");
        let (_, node) = extended_definition(input).expect("extended definition");
        assert!(node.value.has_def_keyword);

        let context = ParseContext::new();
        let input = context.input(b"#fmeaspec requirement req1 { }");
        assert!(
            extended_definition(input).is_err(),
            "keyword-led members stay on metadata_keyword_prefix"
        );
    }

    /// `MetadataFeatureDeclaration = ( Identification ( ':' | 'typed' 'by' ) )?
    /// OwnedFeatureTyping`: the qualified name is the *type*, and it is the required half.
    #[test]
    fn a_bare_head_is_the_owned_feature_typing() {
        let source_text = "@$::Profile::Tag about A::B, C;";
        let context = ParseContext::new();
        let input = context.input(source_text.as_bytes());
        let (rest, annotation) = metadata_annotation(input).expect("metadata annotation");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(
            annotation.value.declared_name.is_none(),
            "nothing was written before a separator, so nothing was declared"
        );

        let arena = context.finish();
        let source = SourceStorage::new(source_text.to_owned());
        let typing = arena
            .get(&source, annotation.value.type_reference)
            .expect("owned feature typing");
        assert!(typing.metadata.is_absolute);
        assert_eq!(typing.segments.len(), 2);
        assert_eq!(typing.segment_decoded_text(0).as_deref(), Some("Profile"));
        assert_eq!(typing.segment_decoded_text(1).as_deref(), Some("Tag"));
        assert_eq!(annotation.value.about_targets.len(), 2);
        assert!(annotation
            .value
            .about_targets
            .iter()
            .all(|target| arena.get(&source, *target).is_some()));
    }

    #[test]
    fn metadata_keyword_and_prefixes_are_one_metadata_feature() {
        let source_text = "#Security #Classified metadata Profile::Classified { }";
        let context = ParseContext::new();
        let input = context.input(source_text.as_bytes());
        let (rest, annotation) = metadata_annotation(input).expect("keyword metadata feature");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(annotation.value.prefixes.len(), 2);
        assert!(matches!(
            annotation.value.introducer,
            MetadataFeatureIntroducer::Metadata { .. }
        ));
        let arena = context.finish();
        let source = SourceStorage::new(source_text.to_owned());
        assert_eq!(
            arena
                .get(&source, annotation.value.type_reference)
                .expect("metadata type")
                .segment_decoded_text(1)
                .as_deref(),
            Some("Classified")
        );
    }

    /// With a separator the head is a declaration label, not a reference: it must not reach the
    /// reference arena, and the type behind the separator must.
    #[test]
    fn a_declared_name_is_not_allocated_as_a_reference() {
        for (source_text, spelling) in [
            ("@t : Meta::Type;", MetadataTypedBy::Colon),
            ("@t typed by Meta::Type;", MetadataTypedBy::TypedBy),
        ] {
            let context = ParseContext::new();
            let input = context.input(source_text.as_bytes());
            let (rest, annotation) = metadata_annotation(input).expect(source_text);
            assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
            let declared = annotation
                .value
                .declared_name
                .as_ref()
                .unwrap_or_else(|| panic!("{source_text} declares a name"));
            assert_eq!(declared.value.identification.name.as_deref(), Some("t"));
            assert_eq!(declared.value.typed_by, spelling);

            let arena = context.finish();
            let source = SourceStorage::new(source_text.to_owned());
            let typing = arena
                .get(&source, annotation.value.type_reference)
                .expect("owned feature typing");
            assert_eq!(typing.segments.len(), 2);
            assert_eq!(typing.segment_decoded_text(1).as_deref(), Some("Type"));
            assert_eq!(
                arena.len(),
                1,
                "{source_text}: only the typing is a reference; `t` is a declaration label"
            );
        }
    }

    /// `<s>` is `Identification`'s short-name half and reaches the same declaration slot.
    #[test]
    fn a_short_name_only_declaration_is_kept() {
        let source_text = "@<s> : Tag;";
        let context = ParseContext::new();
        let input = context.input(source_text.as_bytes());
        let (rest, annotation) = metadata_annotation(input).expect("short-name declaration");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let declared = annotation
            .value
            .declared_name
            .as_ref()
            .expect("short name declares");
        assert_eq!(
            declared.value.identification.short_name.as_deref(),
            Some("s")
        );
        assert!(declared.value.identification.name.is_none());
    }

    /// `PrefixMetadataFeature = OwnedFeatureTyping` -- what follows `#` is a reference, so a
    /// qualified or quoted spelling reaches the arena whole rather than being truncated to the
    /// first segment of a copied `String`.
    #[test]
    fn a_hash_head_is_a_qualified_reference() {
        let source_text = "#ISQ::mass;";
        let context = ParseContext::new();
        let input = context.input(source_text.as_bytes());
        let (rest, usage) = metadata_keyword_usage(input).expect("metadata keyword usage");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert!(usage.value.body.is_some(), "the member spelling wrote `;`");
        assert_eq!(
            usage.value.hash_span.len, 1,
            "the `#` is one token of syntax"
        );

        let arena = context.finish();
        let source = SourceStorage::new(source_text.to_owned());
        let reference = arena
            .get(&source, usage.value.reference)
            .expect("prefix metadata feature");
        assert_eq!(reference.segments.len(), 2);
        assert_eq!(reference.segment_decoded_text(0).as_deref(), Some("ISQ"));
        assert_eq!(reference.segment_decoded_text(1).as_deref(), Some("mass"));
    }

    /// Neither `about` nor `: Type` is reachable from `PrefixMetadataFeature`, so the member
    /// spelling refuses them instead of accepting syntax no production spells.
    #[test]
    fn the_hash_spelling_refuses_clauses_that_belong_to_metadata_feature() {
        for source_text in ["#Tag about X;", "#Tag : Other;"] {
            let context = ParseContext::new();
            let input = context.input(source_text.as_bytes());
            assert!(
                metadata_keyword_usage(input).is_err(),
                "{source_text} is not a `#` production"
            );
        }
    }
}
