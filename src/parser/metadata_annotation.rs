//! Metadata/annotation parsing helpers.

use crate::ast::{
    Annotation, AnnotationHead, AttributeBody, MetadataAnnotation, MetadataKeywordUsage, Node,
};
use crate::parser::attribute::metadata_body;
use crate::parser::connector::connect_body;
use crate::parser::lex::{
    name, qualified_reference, starts_with_keyword, take_until_terminator, ws1, ws_and_comments,
};
use crate::parser::node_from_to;
use crate::parser::with_span;
use crate::parser::Input;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::separated_list1;
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

/// Metadata usage: @ Identification ( : Type )? ( about targets )? MetadataBody
pub(crate) fn metadata_annotation(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<MetadataAnnotation>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"@"[..])).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (head_span, reference)) = with_span(qualified_reference).parse(input)?;
    let (input, typed) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, with_span(qualified_reference)),
    ))
    .parse(input)?;
    let (type_reference, type_span) = typed
        .map(|(span, ty)| (Some(ty), Some(span)))
        .unwrap_or((None, None));
    let (input, about_targets) = parse_about_targets(input)?;
    let (input, body) = metadata_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            MetadataAnnotation {
                reference,
                type_reference,
                about_targets,
                body,
                head_span: Some(head_span),
                type_span,
            },
        ),
    ))
}

/// User-defined metadata keyword: `#keyword` (`:` Type)? (`about` targets)? body.
pub(crate) fn metadata_keyword_usage(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<MetadataKeywordUsage>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"#"[..])).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (keyword_span, keyword)) = with_span(name).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let peek = input.fragment();
    if !(peek.starts_with(b":")
        || peek.starts_with(b";")
        || peek.starts_with(b"{")
        || starts_with_keyword(peek, b"about"))
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, typed) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, with_span(qualified_reference)),
    ))
    .parse(input)?;
    let (type_reference, type_span) = typed
        .map(|(span, ty)| (Some(ty), Some(span)))
        .unwrap_or((None, None));
    let (input, about_targets) = parse_about_targets(input)?;
    let (input, body) = metadata_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            MetadataKeywordUsage {
                keyword,
                type_reference,
                about_targets,
                body,
                keyword_span,
                type_span,
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
/// guard: `metadata_keyword_usage` is also relied on to correctly *fail* on this same shape in
/// bodies where `annotation`/`hash_annotation` is dispatched afterward as an opaque-capture
/// fallback (e.g. `#refinement dependency X to Y;` in action/requirement bodies, where
/// `dependency ...` is not an independently valid body element and must be captured whole).
/// Only wire this into bodies that don't already dispatch `hash_annotation`.
///
/// This intentionally doesn't attempt to resolve whether the keyword is actually a declared
/// `metadata def <keyword> ...` short name (that semantic check, plus a package-local short-name
/// index, is the larger deferred 1.5b item in PARSER_BACKLOG_ROADMAP.md §2.1) -- syntactically,
/// any bare `#<name>` immediately followed by what looks like the start of another declaration
/// (an identifier) is accepted.
pub(crate) fn metadata_keyword_prefix(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<MetadataKeywordUsage>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"#"[..])).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (keyword_span, keyword)) = with_span(name).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let peek = input.fragment();
    if !peek
        .first()
        .is_some_and(|b| b.is_ascii_alphabetic() || *b == b'_')
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    Ok((
        input,
        node_from_to(
            start,
            input,
            MetadataKeywordUsage {
                keyword,
                type_reference: None,
                about_targets: Vec::new(),
                body: AttributeBody::Semicolon,
                keyword_span,
                type_span: None,
            },
        ),
    ))
}

/// `#` annotation: structured keyword usage or opaque extended form (`#refinement dependency ...`).
pub(crate) fn hash_annotation(input: Input<'_>) -> IResult<Input<'_>, Node<Annotation>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"#"[..])).parse(input)?;
    let (input, head) = take_until_terminator(input, b";{")?;
    let (input, body) = connect_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Annotation {
                sigil: "#".to_string(),
                head: AnnotationHead::Opaque(head.trim().to_string()),
                type_reference: None,
                body,
                head_span: None,
                type_span: None,
            },
        ),
    ))
}

/// Generic `@` annotation usage (non-metadata-typed); `#` uses [`metadata_keyword_usage`].
pub(crate) fn annotation(input: Input<'_>) -> IResult<Input<'_>, Node<Annotation>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b"#") {
        return hash_annotation(input);
    }
    let (input, _) = tag(&b"@"[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (head_span, head)) = with_span(qualified_reference).parse(input)?;
    let (input, typed) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, with_span(qualified_reference)),
    ))
    .parse(input)?;
    let (type_reference, type_span) = typed
        .map(|(span, ty)| (Some(ty), Some(span)))
        .unwrap_or((None, None));
    let (input, body) = connect_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Annotation {
                sigil: "@".to_string(),
                head: AnnotationHead::Reference(head),
                type_reference,
                body,
                head_span: Some(head_span),
                type_span,
            },
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::SourceStorage;
    use crate::parser::span::ParseContext;

    #[test]
    fn metadata_annotation_keeps_source_backed_reference_fields() {
        let source_text = "@$::Profile::Tag : Meta::Type about A::B, C;";
        let context = ParseContext::new();
        let input = context.input(source_text.as_bytes());
        let (rest, annotation) = metadata_annotation(input).expect("metadata annotation");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let _ = rest;

        let arena = context.finish();
        let source = SourceStorage::new(source_text.to_owned());
        let reference = arena
            .get(&source, annotation.value.reference)
            .expect("annotation reference");
        assert!(reference.metadata.is_absolute);
        assert_eq!(reference.segments.len(), 2);
        assert_eq!(
            reference.segment_decoded_text(0).as_deref(),
            Some("Profile")
        );
        assert_eq!(reference.segment_decoded_text(1).as_deref(), Some("Tag"));

        let type_reference = arena
            .get(
                &source,
                annotation.value.type_reference.expect("annotation type"),
            )
            .expect("resolved annotation type");
        assert_eq!(type_reference.segments.len(), 2);
        assert_eq!(annotation.value.about_targets.len(), 2);
        assert!(annotation
            .value
            .about_targets
            .iter()
            .all(|target| arena.get(&source, *target).is_some()));
    }
}
