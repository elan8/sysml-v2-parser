//! Dependency parsing (BNF Dependency, DependencyDeclaration).

use crate::ast::{Dependency, Identification, Node, QualifiedReferenceId};
use crate::parser::lex::{name, qualified_reference, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

fn reference_list(input: Input<'_>) -> IResult<Input<'_>, Vec<QualifiedReferenceId>> {
    separated_list1(
        preceded(ws_and_comments, tag(&b","[..])),
        preceded(ws_and_comments, qualified_reference),
    )
    .parse(input)
}

fn to_suppliers(input: Input<'_>) -> IResult<Input<'_>, Vec<QualifiedReferenceId>> {
    let (input, _) = preceded(ws_and_comments, tag(&b"to"[..])).parse(input)?;
    let (input, _) = ws1(input)?;
    reference_list(input)
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
            (
                preceded(tag(&b"from"[..]), ws1),
                reference_list,
                to_suppliers,
            ),
            |(_, clients, suppliers)| (None, clients, suppliers),
        ),
        map(
            (
                preceded(ws_and_comments, name),
                preceded(ws_and_comments, tag(&b"from"[..])),
                ws1,
                reference_list,
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
        map((reference_list, to_suppliers), |(clients, suppliers)| {
            (None, clients, suppliers)
        }),
    ))
    .parse(input)?;

    let (input, body) = crate::parser::body::relationship_body(input)?;
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
            },
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{QualifiedReferenceArena, SourceStorage};
    use crate::parser::span::ParseContext;

    fn parsed(text: &str) -> (Dependency, SourceStorage, QualifiedReferenceArena) {
        let source = SourceStorage::new(text.to_owned());
        let context = ParseContext::new();
        let (rest, node) =
            dependency(context.input(source.as_str().as_bytes())).expect("dependency should parse");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        (node.value, source, context.finish())
    }

    fn authored<'a>(
        source: &'a SourceStorage,
        arena: &'a QualifiedReferenceArena,
        id: QualifiedReferenceId,
    ) -> &'a str {
        arena
            .get(source, id)
            .expect("source-backed dependency reference")
            .authored_text()
    }

    #[test]
    fn dependency_from_keyword_is_not_a_client() {
        let (dependency, source, arena) =
            parsed("dependency from 'Service Layer' to 'Data Layer';");
        assert!(dependency.identification.is_none());
        assert_eq!(
            authored(&source, &arena, dependency.clients[0]),
            "'Service Layer'"
        );
        assert_eq!(
            authored(&source, &arena, dependency.suppliers[0]),
            "'Data Layer'"
        );
    }

    /// Spec42 Gap 37: a braced `RelationshipBody` owns feature members alongside the annotation
    /// subset (`dependency z to x, y { feature e; }`, kerml `dependencies` fixture).
    #[test]
    fn dependency_body_owns_feature_members() {
        let (dependency, _source, _arena) = parsed("dependency z to x, y { feature e; }");
        let elements = dependency.body.braced_elements().expect("braced body");
        assert_eq!(elements.len(), 1);
        let crate::ast::RelationshipBodyElement::KermlFeature(feature) = &elements[0].value else {
            panic!("expected KermlFeature member");
        };
        assert_eq!(feature.value.name, "e");
    }

    #[test]
    fn dependency_named_from_clients() {
        let (dependency, source, arena) =
            parsed("dependency Use from 'Application Layer' to 'Service Layer';");
        assert_eq!(
            dependency
                .identification
                .as_ref()
                .and_then(|i| i.name.as_deref()),
            Some("Use")
        );
        assert_eq!(
            authored(&source, &arena, dependency.clients[0]),
            "'Application Layer'"
        );
        assert_eq!(
            authored(&source, &arena, dependency.suppliers[0]),
            "'Service Layer'"
        );
    }

    #[test]
    fn dependency_clients_without_from_keyword() {
        let (dependency, source, arena) = parsed("dependency z to x, y;");
        assert_eq!(authored(&source, &arena, dependency.clients[0]), "z");
        let suppliers: Vec<_> = dependency
            .suppliers
            .iter()
            .copied()
            .map(|id| authored(&source, &arena, id))
            .collect();
        assert_eq!(suppliers, ["x", "y"]);
    }
}
