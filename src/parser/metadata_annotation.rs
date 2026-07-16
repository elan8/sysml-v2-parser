//! Metadata/annotation parsing helpers.

use crate::ast::{Annotation, MetadataAnnotation, MetadataKeywordUsage, Node};
use crate::parser::attribute::metadata_body;
use crate::parser::interface::connect_body;
use crate::parser::lex::{
    name, qualified_name, starts_with_keyword, take_until_terminator, ws1, ws_and_comments,
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
pub(crate) fn parse_about_targets(input: Input<'_>) -> IResult<Input<'_>, Vec<String>> {
    let (input, _) = ws_and_comments(input)?;
    if !starts_with_keyword(input.fragment(), b"about") {
        return Ok((input, Vec::new()));
    }
    let (input, _) = tag(&b"about"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    separated_list1(
        preceded(ws_and_comments, preceded(tag(&b","[..]), ws_and_comments)),
        qualified_name,
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
    let (input, (head_span, name_str)) = with_span(qualified_name).parse(input)?;
    let (input, typed) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, with_span(qualified_name)),
    ))
    .parse(input)?;
    let (type_name, type_span) = typed
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
                name: name_str,
                type_name,
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
        preceded(ws_and_comments, with_span(qualified_name)),
    ))
    .parse(input)?;
    let (type_name, type_span) = typed
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
                type_name,
                about_targets,
                body,
                keyword_span,
                type_span,
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
                head: head.trim().to_string(),
                type_name: None,
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
    let (input, (head_span, head)) = with_span(qualified_name).parse(input)?;
    let (input, typed) = opt(preceded(
        preceded(ws_and_comments, tag(&b":"[..])),
        preceded(ws_and_comments, with_span(qualified_name)),
    ))
    .parse(input)?;
    let (type_name, type_span) = typed
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
                head,
                type_name,
                body,
                head_span: Some(head_span),
                type_span,
            },
        ),
    ))
}
