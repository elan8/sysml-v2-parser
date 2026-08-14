//! Individual definition parsing.

use crate::ast::{IndividualDef, Membership, Node};
use crate::parser::attribute::attribute_body;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::node_from_to;
use crate::parser::Input;
use nom::IResult;

/// Individual definition: `individual def` Identification ( (`:>` | `specializes`) qualified_name )? body
pub(crate) fn individual_def(input: Input<'_>) -> IResult<Input<'_>, Node<IndividualDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"individual")
            .def_required()
            .no_abstract()
            .with_captured_visibility(),
    )?;
    let (input, body) = attribute_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            IndividualDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
                membership: Membership::owning(prefix.visibility, prefix.visibility_span),
            },
        ),
    ))
}

#[cfg(test)]
mod membership_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    // --- parser work item 4b (final sweep): Membership on IndividualDef ---

    #[test]
    fn individual_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) =
            individual_def(input("private individual def I1;")).expect("individual def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Private)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::OwningMembership
        );
    }

    #[test]
    fn individual_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = individual_def(input("individual def I1;")).expect("individual def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }
}
