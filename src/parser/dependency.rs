//! Dependency parsing (BNF Dependency, DependencyDeclaration).

use crate::ast::{ConnectBody, Dependency, Identification, Node};
use crate::parser::lex::{qualified_name, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

fn name_list(input: Input<'_>) -> IResult<Input<'_>, Vec<String>> {
    separated_list1(
        preceded(ws_and_comments, tag(&b","[..])),
        preceded(ws_and_comments, qualified_name),
    )
    .parse(input)
}

fn to_suppliers(input: Input<'_>) -> IResult<Input<'_>, Vec<String>> {
    let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    name_list(input)
}

/// dependency DependencyDeclaration RelationshipBody
/// DependencyDeclaration =
///   Identification 'from' QualifiedName (',' QualifiedName)* 'to' …
/// | 'from' QualifiedName (',' QualifiedName)* 'to' …
/// | QualifiedName (',' QualifiedName)* 'to' …
pub(crate) fn dependency(input: Input<'_>) -> IResult<Input<'_>, Node<Dependency>> {
    let start = input;
    let (input, _) = preceded(ws_and_comments, tag(&b"dependency"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, _) = ws_and_comments(input)?;

    let (input, (ident, clients, suppliers)) = alt((
        // `from clients to suppliers` — must beat the bare client-list form so `from` is not
        // stored as a client name (#78 / validation `12a`).
        map(
            (preceded(tag(&b"from"[..]), ws1), name_list, to_suppliers),
            |(_, clients, suppliers)| (None, clients, suppliers),
        ),
        map(
            (
                preceded(ws_and_comments, qualified_name),
                preceded(ws_and_comments, tag(&b"from"[..])),
                ws1,
                name_list,
                to_suppliers,
            ),
            |(name, _, _, clients, suppliers)| {
                (
                    Some(Identification {
                        short_name: None,
                        name: Some(name),
                    }),
                    clients,
                    suppliers,
                )
            },
        ),
        map((name_list, to_suppliers), |(clients, suppliers)| {
            (None, clients, suppliers)
        }),
    ))
    .parse(input)?;

    let (input, (body, body_elements)) = relationship_body_connect(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Dependency {
                identification: ident,
                clients,
                suppliers,
                body,
                body_elements,
            },
        ),
    ))
}

type RelationshipConnectBody = (
    ConnectBody,
    Option<Vec<Node<crate::ast::RelationshipBodyElement>>>,
);

/// `RelationshipBody`: `;` or `{` doc/comment/rep/metadata* `}`.
fn relationship_body_connect(input: Input<'_>) -> IResult<Input<'_>, RelationshipConnectBody> {
    let (input, elements) = crate::parser::body::relationship_body_annotations(input)?;
    match &elements {
        None => Ok((input, (ConnectBody::Semicolon, None))),
        Some(_) => Ok((input, (ConnectBody::Brace, elements))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    #[test]
    fn dependency_from_keyword_is_not_a_client() {
        let (rest, node) = dependency(input("dependency from 'Service Layer' to 'Data Layer';"))
            .expect("dependency from");
        assert!(rest.fragment().is_empty());
        assert!(node.value.identification.is_none());
        assert_eq!(node.value.clients, vec!["Service Layer".to_string()]);
        assert_eq!(node.value.suppliers, vec!["Data Layer".to_string()]);
    }

    #[test]
    fn dependency_named_from_clients() {
        let (rest, node) = dependency(input(
            "dependency Use from 'Application Layer' to 'Service Layer';",
        ))
        .expect("named dependency");
        assert!(rest.fragment().is_empty());
        assert_eq!(
            node.value
                .identification
                .as_ref()
                .and_then(|i| i.name.as_deref()),
            Some("Use")
        );
        assert_eq!(node.value.clients, vec!["Application Layer".to_string()]);
        assert_eq!(node.value.suppliers, vec!["Service Layer".to_string()]);
    }

    #[test]
    fn dependency_clients_without_from_keyword() {
        let (rest, node) = dependency(input("dependency z to x, y;")).expect("dependency z");
        assert!(rest.fragment().is_empty());
        assert_eq!(node.value.clients, vec!["z".to_string()]);
        assert_eq!(node.value.suppliers, vec!["x".to_string(), "y".to_string()]);
    }
}
