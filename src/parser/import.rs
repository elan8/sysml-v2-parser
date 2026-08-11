//! Import and relationship body parsing.

use crate::ast::{
    FilterPackageMember, Import, ImportShape, ImportTarget, Membership, MembershipKind, Node,
    RelationshipBodyElement,
};
use crate::parser::body::relationship_body_annotations;
use crate::parser::expr::expression;
use crate::parser::lex::{qualified_reference, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::Input;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use nom::Parser;

/// RelationshipBody: `;` or `{` doc/comment/rep/metadata* `}`.
pub(crate) fn relationship_body(
    input: Input<'_>,
) -> IResult<Input<'_>, Option<Vec<Node<RelationshipBodyElement>>>> {
    relationship_body_annotations(input)
}

fn filter_package_members(input: Input<'_>) -> IResult<Input<'_>, Vec<Node<FilterPackageMember>>> {
    let (input, members) = many1(delimited(
        preceded(ws_and_comments, tag(&b"["[..])),
        preceded(ws_and_comments, expression),
        preceded(ws_and_comments, tag(&b"]"[..])),
    ))
    .parse(input)?;
    Ok((
        input,
        members
            .into_iter()
            .map(|expression| {
                Node::new(expression.span.clone(), FilterPackageMember { expression })
            })
            .collect(),
    ))
}

/// Parse the typed suffix shared by import and expose targets.
pub(crate) fn import_shape(input: Input<'_>) -> IResult<Input<'_>, ImportShape> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b"::") {
        let (input, _) = tag(&b"::"[..]).parse(input)?;
        let (input, _) = ws_and_comments(input)?;
        if input.fragment().starts_with(b"**") {
            let (input, _) = tag(&b"**"[..]).parse(input)?;
            let (input, members) = opt(filter_package_members).parse(input)?;
            return Ok((
                input,
                match members {
                    Some(members) => ImportShape::Filter {
                        recursive: true,
                        members,
                    },
                    None => ImportShape::Membership { recursive: true },
                },
            ));
        }
        if input.fragment().starts_with(b"*") {
            let (input, _) = tag(&b"*"[..]).parse(input)?;
            let (input, recursive) = opt((
                preceded(ws_and_comments, tag(&b"::"[..])),
                preceded(ws_and_comments, tag(&b"**"[..])),
            ))
            .parse(input)?;
            return Ok((
                input,
                ImportShape::Namespace {
                    recursive: recursive.is_some(),
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
                recursive: false,
                members,
            },
        ));
    }
    Ok((input, ImportShape::Membership { recursive: false }))
}

/// Import: visibility? 'import' isImportAll? (QualifiedName | QualifiedName '::' '*') RelationshipBody
pub(crate) fn import_(input: Input<'_>) -> IResult<Input<'_>, Node<Import>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, _) = tag(&b"import"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = opt(preceded(tag(&b"all"[..]), ws1)).parse(input)?;
    let (input, reference) = qualified_reference(input)?;
    let (input, shape) = import_shape(input)?;
    let (input, body_elements) = relationship_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Import {
                membership: Membership::new(MembershipKind::Import, visibility, visibility_span),
                target: ImportTarget { reference, shape },
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
        let source = crate::ast::SourceStorage::from("import $::A::B::*::**;");
        let (rest, node) = import_(context.input(source.as_str().as_bytes())).expect("import");
        assert!(rest.fragment().is_empty());
        assert_eq!(
            node.value.target.shape,
            ImportShape::Namespace { recursive: true }
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
        let (rest, node) = import_(context.input(b"import A[x][y];")).expect("import");
        assert!(rest.fragment().is_empty());
        match node.value.target.shape {
            ImportShape::Filter { recursive, members } => {
                assert!(!recursive);
                assert_eq!(members.len(), 2);
            }
            other => panic!("expected filter shape, got {other:?}"),
        }
    }
}
