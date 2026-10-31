//! Import and relationship body parsing.

use crate::ast::{
    FilterPackageMember, Import, ImportShape, ImportSuffixSpans, ImportTarget, ImportVisibility,
    Node,
};
use crate::parser::body::relationship_body;
use crate::parser::expr::expression;
use crate::parser::lex::{qualified_reference, ws1, ws_and_comments};
use crate::parser::span::reference_transaction;
use crate::parser::Input;
use crate::parser::{node_from_to, span_from_to, with_span};
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::{preceded, terminated};
use nom::IResult;
use nom::Parser;

fn filter_package_member(input: Input<'_>) -> IResult<Input<'_>, Node<FilterPackageMember>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, (open_bracket_span, _)) = with_span(tag(&b"["[..]))(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, expression) = expression(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (close_bracket_span, _)) = with_span(tag(&b"]"[..]))(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            FilterPackageMember {
                open_bracket_span,
                expression,
                close_bracket_span,
            },
        ),
    ))
}

fn filter_package_members(input: Input<'_>) -> IResult<Input<'_>, Vec<Node<FilterPackageMember>>> {
    many1(filter_package_member).parse(input)
}

fn import_suffix<'a>(
    input: Input<'a>,
    marker: &'static [u8],
) -> IResult<Input<'a>, ImportSuffixSpans> {
    let start = input;
    let (input, (separator_span, _)) = with_span(tag(&b"::"[..]))(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (marker_span, _)) = with_span(tag(marker))(input)?;
    Ok((
        input,
        ImportSuffixSpans {
            span: span_from_to(start, input),
            separator_span,
            marker_span,
        },
    ))
}

/// Parse the typed suffix shared by import and expose targets.
pub(crate) fn import_shape(input: Input<'_>) -> IResult<Input<'_>, ImportShape> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b"::") {
        let (marker_input, _) = tag(&b"::"[..]).parse(input)?;
        let (marker_input, _) = ws_and_comments(marker_input)?;
        if marker_input.fragment().starts_with(b"**") {
            let (input, recursive_suffix) = import_suffix(input, b"**")?;
            let (input, members) = opt(filter_package_members).parse(input)?;
            return Ok((
                input,
                match members {
                    Some(members) => ImportShape::Filter {
                        recursive_suffix: Some(recursive_suffix),
                        members,
                    },
                    None => ImportShape::Membership {
                        recursive_suffix: Some(recursive_suffix),
                    },
                },
            ));
        }
        if marker_input.fragment().starts_with(b"*") {
            let suffix_start = input;
            let (input, wildcard_suffix) = import_suffix(input, b"*")?;
            let (input, recursive_suffix) = opt(preceded(ws_and_comments, |input| {
                import_suffix(input, b"**")
            }))
            .parse(input)?;
            let combined_recursive_suffix_span = recursive_suffix
                .as_ref()
                .map(|_| span_from_to(suffix_start, input));
            return Ok((
                input,
                ImportShape::Namespace {
                    wildcard_suffix,
                    recursive_suffix,
                    combined_recursive_suffix_span,
                },
            ));
        }
        return Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    if input.fragment().starts_with(b"[") {
        let (input, members) = filter_package_members(input)?;
        return Ok((
            input,
            ImportShape::Filter {
                recursive_suffix: None,
                members,
            },
        ));
    }
    Ok((
        input,
        ImportShape::Membership {
            recursive_suffix: None,
        },
    ))
}

/// Import: visibility 'import' isImportAll? (QualifiedName | QualifiedName '::' '*') RelationshipBody
pub(crate) fn import_(input: Input<'_>) -> IResult<Input<'_>, Node<Import>> {
    // Speculated at many member starts; refuse unless `import` follows the optional
    // visibility keyword, before entering an arena transaction.
    {
        let (cursor, _) = crate::parser::lex::ws_and_comments(input)?;
        let (cursor, _) = opt(crate::parser::lex::visibility_indicator).parse(cursor)?;
        let (cursor, _) = crate::parser::lex::ws_and_comments(cursor)?;
        if !crate::parser::lex::starts_with_keyword(cursor.fragment(), b"import") {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
    }
    reference_transaction(input, import_inner)
}

/// The offset just past an import target's last authored token: its final suffix or filter member
/// if it has one, otherwise the qualified reference itself.
pub(crate) fn import_target_end(shape: &crate::ast::ImportShape, reference_end: usize) -> usize {
    use crate::ast::ImportShape;
    let span_end = |span: &crate::ast::Span| span.offset.saturating_add(span.len);
    match shape {
        ImportShape::Membership { recursive_suffix } => recursive_suffix
            .as_ref()
            .map_or(reference_end, |suffix| span_end(&suffix.span)),
        ImportShape::Namespace {
            wildcard_suffix,
            recursive_suffix,
            combined_recursive_suffix_span,
        } => combined_recursive_suffix_span
            .as_ref()
            .map(span_end)
            .or_else(|| {
                recursive_suffix
                    .as_ref()
                    .map(|suffix| span_end(&suffix.span))
            })
            .unwrap_or_else(|| span_end(&wildcard_suffix.span)),
        ImportShape::Filter {
            recursive_suffix,
            members,
        } => members
            .last()
            .map(|member| span_end(&member.span))
            .or_else(|| {
                recursive_suffix
                    .as_ref()
                    .map(|suffix| span_end(&suffix.span))
            })
            .unwrap_or(reference_end),
    }
}

fn import_inner(input: Input<'_>) -> IResult<Input<'_>, Node<Import>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, authored_visibility) =
        opt(crate::parser::lex::visibility_indicator).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (import_keyword_span, _)) = with_span(tag(&b"import"[..]))(input)?;
    // The grammar requires the prefix. Keeping the typed import lets its target resolve; the
    // recovery scan reports the missing prefix on the keyword.
    let visibility = match authored_visibility {
        Some((span, visibility)) => ImportVisibility::Authored { visibility, span },
        None => ImportVisibility::Missing {
            import_keyword_span,
        },
    };
    let (input, _) = ws_and_comments(input)?;
    let target_start = input;
    let (input, all) = opt(terminated(with_span(tag(&b"all"[..])), ws1)).parse(input)?;
    let all_span = all.map(|(span, _)| span);
    let (after_reference, reference) = qualified_reference(input)?;
    let (input, shape) = import_shape(after_reference)?;
    // The target ends at its last authored token, not wherever shape parsing stopped: looking for
    // an absent `::*` consumes the trivia before a braced body, and a span that swallowed it would
    // no longer describe the target it claims.
    let mut target_span = span_from_to(target_start, input);
    target_span.len = import_target_end(&shape, after_reference.location_offset())
        .saturating_sub(target_span.offset);
    let (input, body) = relationship_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Import {
                visibility,
                target: ImportTarget {
                    span: target_span,
                    all_span,
                    reference,
                    shape,
                },
                body,
            },
        ),
    ))
}

#[cfg(test)]
mod membership_tests {
    use super::*;
    use crate::parser::span::ParseContext;

    fn input(text: &str) -> Input<'_> {
        let context = Box::leak(Box::new(ParseContext::new()));
        context.input(text.as_bytes())
    }

    #[test]
    fn import_visibility_prefix_is_captured_with_its_span() {
        let (_, node) = import_(input("public import SI::kg;")).expect("import");
        let ImportVisibility::Authored { visibility, span } = node.value.visibility else {
            panic!(
                "expected an authored visibility, got {:?}",
                node.value.visibility
            );
        };
        assert_eq!(visibility, crate::ast::Visibility::Public);
        assert_eq!((span.offset, span.len), (0, "public".len()));
    }

    #[test]
    fn import_without_visibility_keeps_its_typed_target() {
        let context = ParseContext::new();
        let source = crate::ast::SourceStorage::from("  import SI::kg;");
        let (rest, node) = import_(context.input(source.as_str().as_bytes())).expect("import");
        assert!(rest.fragment().is_empty());
        let ImportVisibility::Missing {
            import_keyword_span,
        } = node.value.visibility
        else {
            panic!(
                "expected a missing visibility, got {:?}",
                node.value.visibility
            );
        };
        assert_eq!(source.slice(&import_keyword_span), Some("import"));
        let arena = context.finish();
        let view = arena
            .get(&source, node.value.target.reference)
            .expect("reference");
        assert_eq!(view.authored_text(), "SI::kg");
    }

    #[test]
    fn import_suffixes_are_typed_and_reference_excludes_wildcards() {
        let context = ParseContext::new();
        let source = crate::ast::SourceStorage::from(
            "private import $::A::B:: /* wildcard */ * /* recurse */ :: /* marker */ **;",
        );
        let (rest, node) = import_(context.input(source.as_str().as_bytes())).expect("import");
        assert!(rest.fragment().is_empty());
        let ImportShape::Namespace {
            wildcard_suffix,
            recursive_suffix: Some(recursive_suffix),
            combined_recursive_suffix_span: Some(combined_span),
        } = &node.value.target.shape
        else {
            panic!("expected recursive namespace import");
        };
        assert_eq!(
            source.slice(&wildcard_suffix.span),
            Some(":: /* wildcard */ *")
        );
        assert_eq!(source.slice(&wildcard_suffix.separator_span), Some("::"));
        assert_eq!(source.slice(&wildcard_suffix.marker_span), Some("*"));
        assert_eq!(
            source.slice(&recursive_suffix.span),
            Some(":: /* marker */ **")
        );
        assert_eq!(source.slice(&recursive_suffix.separator_span), Some("::"));
        assert_eq!(source.slice(&recursive_suffix.marker_span), Some("**"));
        assert_eq!(
            source.slice(combined_span),
            Some(":: /* wildcard */ * /* recurse */ :: /* marker */ **")
        );
        let arena = context.finish();
        let view = arena
            .get(&source, node.value.target.reference)
            .expect("reference");
        assert_eq!(view.authored_text(), "$::A::B");
        assert!(view.metadata.is_absolute);
    }

    #[test]
    fn filter_import_retains_typed_expression_members() {
        let context = ParseContext::new();
        let source = crate::ast::SourceStorage::from(
            "private import A /* before */ [ /* left */ x /* right */ ] [y];",
        );
        let (rest, node) = import_(context.input(source.as_str().as_bytes())).expect("import");
        assert!(rest.fragment().is_empty());
        match node.value.target.shape {
            ImportShape::Filter {
                recursive_suffix,
                members,
            } => {
                assert!(recursive_suffix.is_none());
                assert_eq!(members.len(), 2);
                assert_eq!(
                    source.slice(&members[0].span),
                    Some("[ /* left */ x /* right */ ]")
                );
                assert_eq!(source.slice(&members[0].value.open_bracket_span), Some("["));
                assert_eq!(
                    source.slice(&members[0].value.close_bracket_span),
                    Some("]")
                );
                assert_eq!(source.slice(&members[1].span), Some("[y]"));
            }
            other => panic!("expected filter shape, got {other:?}"),
        }
    }
}
