//! Import and relationship body parsing.

use crate::ast::{
    FilterPackageMember, Import, ImportShape, ImportSuffixSpans, ImportTarget, Membership,
    MembershipKind, Node, RelationshipBodyElement,
};
use crate::parser::body::relationship_body_annotations;
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

/// RelationshipBody: `;` or `{` doc/comment/rep/metadata* `}`.
pub(crate) fn relationship_body(
    input: Input<'_>,
) -> IResult<Input<'_>, Option<Vec<Node<RelationshipBodyElement>>>> {
    relationship_body_annotations(input)
}

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

/// Import: visibility? 'import' isImportAll? (QualifiedName | QualifiedName '::' '*') RelationshipBody
pub(crate) fn import_(input: Input<'_>) -> IResult<Input<'_>, Node<Import>> {
    reference_transaction(input, import_inner)
}

fn import_inner(input: Input<'_>) -> IResult<Input<'_>, Node<Import>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, _) = tag(&b"import"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let target_start = input;
    let (input, all) = opt(terminated(with_span(tag(&b"all"[..])), ws1)).parse(input)?;
    let all_span = all.map(|(span, _)| span);
    let (input, reference) = qualified_reference(input)?;
    let (input, shape) = import_shape(input)?;
    let target_span = span_from_to(target_start, input);
    let (input, body_elements) = relationship_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Import {
                membership: Membership::new(MembershipKind::Import, visibility, visibility_span),
                target: ImportTarget {
                    span: target_span,
                    all_span,
                    reference,
                    shape,
                },
                body_elements,
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

    // --- parser work item 4b (continuation): `Import.visibility` replaced by `Import.membership` ---

    #[test]
    fn import_visibility_prefix_is_captured_on_membership() {
        let (_, node) = import_(input("public import SI::kg;")).expect("import");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::Import
        );
    }

    #[test]
    fn import_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = import_(input("import SI::kg;")).expect("import");
        assert_eq!(node.value.membership.visibility, None);
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::Import
        );
    }

    #[test]
    fn import_suffixes_are_typed_and_reference_excludes_wildcards() {
        let context = ParseContext::new();
        let source = crate::ast::SourceStorage::from(
            "import $::A::B:: /* wildcard */ * /* recurse */ :: /* marker */ **;",
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
            "import A /* before */ [ /* left */ x /* right */ ] [y];",
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
