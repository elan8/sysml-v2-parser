//! Import and relationship body parsing.

use crate::ast::{
    FilterPackageMember, Import, Membership, MembershipKind, Node, RelationshipBodyElement,
};
use crate::parser::body::relationship_body_annotations;
use crate::parser::expr::expression;
use crate::parser::lex::{qualified_name, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::span::with_span;
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

/// Import: visibility? 'import' isImportAll? (QualifiedName | QualifiedName '::' '*') RelationshipBody
pub(crate) fn import_(input: Input<'_>) -> IResult<Input<'_>, Node<Import>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_prefix(input)?;
    let (input, _) = tag(&b"import"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = opt(preceded(tag(&b"all"[..]), ws1)).parse(input)?;
    let (input, (qname_span, qname)) = with_span(qualified_name).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    // KerML: NamespaceImport = QualifiedName '::' '*' (::**)? | FilterPackage; MembershipImport = QualifiedName (::**)?
    let (input, target, is_import_all, is_recursive, filter_members) =
        if input.fragment().starts_with(b"::") {
            let (input, _) = preceded(ws_and_comments, tag(&b"::"[..])).parse(input)?;
            let (input, _) = ws_and_comments(input)?;
            if input.fragment().starts_with(b"*")
                && input.fragment().get(1).is_none_or(|c| *c != b'*')
            {
                let (input, _) = preceded(ws_and_comments, tag(&b"*"[..])).parse(input)?;
                let (input, rec_opt) = opt((
                    preceded(ws_and_comments, tag(&b"::"[..])),
                    preceded(ws_and_comments, tag(&b"**"[..])),
                ))
                .parse(input)?;
                (
                    input,
                    format!("{}::*", qname),
                    true,
                    rec_opt.is_some(),
                    None,
                )
            } else if input.fragment().starts_with(b"**") {
                let (input, _) = preceded(ws_and_comments, tag(&b"**"[..])).parse(input)?;
                let (input, filter_opt) = opt(many1(delimited(
                    preceded(ws_and_comments, tag(&b"["[..])),
                    preceded(ws_and_comments, expression),
                    preceded(ws_and_comments, tag(&b"]"[..])),
                )))
                .parse(input)?;
                let filter_members = filter_opt.map(|members| {
                    members
                        .into_iter()
                        .map(|e| Node::new(e.span.clone(), FilterPackageMember { expression: e }))
                        .collect()
                });
                (input, qname, false, true, filter_members)
            } else {
                return Err(nom::Err::Error(nom::error::make_error(
                    input,
                    nom::error::ErrorKind::Tag,
                )));
            }
        } else if input.fragment().starts_with(b"[") {
            // FilterPackage form: QualifiedName [ expr ] [ expr ]+
            let (input, members) = many1(delimited(
                preceded(ws_and_comments, tag(&b"["[..])),
                preceded(ws_and_comments, expression),
                preceded(ws_and_comments, tag(&b"]"[..])),
            ))
            .parse(input)?;
            let filter_members: Vec<Node<FilterPackageMember>> = members
                .into_iter()
                .map(|e| Node::new(e.span.clone(), FilterPackageMember { expression: e }))
                .collect();
            (input, qname, true, false, Some(filter_members))
        } else {
            let (input, rec_opt) = opt((
                preceded(ws_and_comments, tag(&b"::"[..])),
                preceded(ws_and_comments, tag(&b"**"[..])),
            ))
            .parse(input)?;
            (input, qname, false, rec_opt.is_some(), None)
        };
    let (input, body_elements) = relationship_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Import {
                membership: Membership::new(MembershipKind::Import, visibility, visibility_span),
                is_import_all,
                target,
                target_span: qname_span,
                is_recursive,
                filter_members,
                body_elements,
            },
        ),
    ))
}

#[cfg(test)]
mod membership_tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
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
}
