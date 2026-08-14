//! Occurrence definition parsing.

use crate::ast::{Membership, Node, OccurrenceDef};
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::node_from_to;
use crate::parser::occurrence_body::occurrence_def_definition_body;
use crate::parser::Input;
use nom::IResult;

pub(crate) use crate::parser::occurrence_body::{
    individual_usage, occurrence_usage, snapshot_usage, then_timeslice_usage, timeslice_usage,
};

pub(crate) fn occurrence_def(input: Input<'_>) -> IResult<Input<'_>, Node<OccurrenceDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"occurrence")
            .def_required()
            .individual_allowed()
            .with_captured_visibility(),
    )?;
    let (input, body) = occurrence_def_definition_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            OccurrenceDef {
                is_abstract: prefix.is_abstract,
                is_individual: prefix.is_individual,
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

    // --- parser work item 4b (final sweep): Membership on OccurrenceDef ---

    #[test]
    fn occurrence_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) =
            occurrence_def(input("private occurrence def O1;")).expect("occurrence def");
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
    fn occurrence_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = occurrence_def(input("occurrence def O1;")).expect("occurrence def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }
}
