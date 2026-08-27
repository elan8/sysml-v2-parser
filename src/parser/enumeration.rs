//! Enumeration definition parsing (BNF EnumerationDefinition).

use crate::ast::{
    EnumDef, EnumeratedValue, EnumerationBody, EnumerationBodyElement, EnumerationUsage,
    Membership, Node,
};
use crate::parser::attribute::attribute_body;
use crate::parser::body::parse_structured_brace_members;
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::lex::{identification, name, visibility_prefix, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::usage::{feature_usage_header, multiplicity_node};
use crate::parser::with_span;
use crate::parser::Input;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// `EnumerationUsageMember = MemberPrefix EnumeratedValue`, where the pinned
/// `EnumeratedValue = 'enum'? Usage` (SysML-textual-bnf.kebnf 528-535).
///
/// This deliberately does **not** accept Pilot's leading `UsageExtensionKeyword*`: the pinned
/// grammar has no such slot, so `#Security enum value ...` remains a source-backed recovery node
/// rather than looking like supported SysML.  Once the optional enum token is past, the remaining
/// declaration is the shared full `Usage` shape: Identification, FeatureSpecializationPart,
/// ValuePart, then UsageBody.
fn enumerated_value(input: Input<'_>) -> IResult<Input<'_>, Node<EnumeratedValue>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, _) = ws_and_comments(input)?;
    let mut input = input;
    let mut extension_keywords = Vec::new();
    while input.fragment().starts_with(b"#") {
        let (rest, keyword) = crate::parser::occurrence_prefix::usage_extension_keyword(input)?;
        extension_keywords.push(keyword);
        let (rest, _) = ws_and_comments(rest)?;
        input = rest;
    }
    let (input, enum_keyword_span) = opt(with_span(|input| {
        preceded(tag(&b"enum"[..]), ws1).parse(input)
    }))
    .parse(input)?;
    let (input, (identification_span, ident)) = with_span(identification).parse(input)?;
    let (input, header) = feature_usage_header(input)?;
    let (input, value) = opt(crate::parser::feature_value::feature_value_part).parse(input)?;
    let (input, body) = crate::parser::part::part_usage_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            EnumeratedValue {
                extension_keywords,
                enum_keyword_span: enum_keyword_span.map(|(span, _)| span),
                identification: ident,
                identification_span,
                typing: header.typing,
                multiplicity: header.multiplicity,
                multiplicity_modifiers: header.multiplicity_modifiers,
                subsets: header
                    .subsets
                    .map(|subsets| (subsets, header.subsetting_value)),
                redefines: header.redefines,
                references: header.references,
                crosses: header.crosses,
                intersects: header.intersects,
                value,
                body,
                membership: Membership::variant(visibility, visibility_span),
            },
        ),
    ))
}

/// `EnumerationBody : EnumerationDefinition = ';' | '{' ( ownedRelationship += AnnotatingMember |
/// ownedRelationship += EnumerationUsageMember )* '}'` (SysML 8.2.2.8).
///
/// This body used to run its own brace loop, which recognized `doc` and `comment` only to drop
/// them -- no node, no span, no diagnostic -- accepted neither of the production's other two
/// alternatives, and on any unparseable member ran to the closing brace discarding everything in
/// between. It now goes through the shared brace-member routine like every other scope, so a
/// malformed member becomes an `Error` node at its authored position and the values after it
/// still parse.
fn enumeration_body(input: Input<'_>) -> IResult<Input<'_>, EnumerationBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        return crate::parser::body::semicolon_body(input);
    }
    let (input, members) = parse_structured_brace_members(
        input,
        ENUMERATION_BODY_STARTERS,
        "enumeration body",
        "recovered_enumeration_body_element",
        enumeration_body_element,
        |start, end| {
            let recovery = build_recovery_error_node_from_span(
                start,
                end,
                ENUMERATION_BODY_STARTERS,
                "enumeration body",
                "recovered_enumeration_body_element",
            );
            node_from_to(
                start,
                end,
                EnumerationBodyElement::Error(node_from_to(start, end, recovery)),
            )
        },
    )?;
    Ok((input, members.into_body()))
}

const ENUMERATION_BODY_STARTERS: &[&[u8]] = &[
    b"enum",
    b"private",
    b"protected",
    b"public",
    b"doc",
    b"comment",
    b"rep",
    b"language",
];

fn enumeration_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<EnumerationBodyElement>> {
    let start = input;
    // Member boundary: `ws_and_notes` leaves a bare `/* ... */` for this scope's
    // annotating member, which is the `Comment` production's keyword-less spelling.
    let (input, _) = crate::parser::lex::ws_and_notes(input)?;
    let (input, element) = nom::branch::alt((
        nom::combinator::map(crate::parser::body::annotating_member, |member| {
            EnumerationBodyElement::Annotating(Box::new(member))
        }),
        nom::combinator::map(enumerated_value, |value| {
            EnumerationBodyElement::Value(Box::new(value))
        }),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, element)))
}

/// Enumeration definition: `enum def` Identification EnumerationBody.
pub(crate) fn enum_def(input: Input<'_>) -> IResult<Input<'_>, Node<EnumDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"enum")
            .def_required()
            .no_basic_prefix()
            .with_captured_visibility(),
    )?;
    let (input, body) = enumeration_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            EnumDef {
                identification: prefix.identification,
                specializes: prefix.specializes,
                body,
                membership: Membership::owning(prefix.visibility, prefix.visibility_span),
            },
        ),
    ))
}

/// Enumeration usage in a definition or usage body: `end`? `enum` name multiplicity? (`:` type)?
/// body. `end` is `EndUsagePrefix` (BNF §8.2.2.6.2, `isEnd ?= 'end'`), reached through the same
/// `UsagePrefix 'enum' Usage` production `AttributeUsage.is_end` documents; unrelated to the
/// separate `EndDecl`/`end_decl` named-connector-end construct.
pub(crate) fn enum_usage(input: Input<'_>) -> IResult<Input<'_>, Node<EnumerationUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = visibility_prefix(input)?;
    let (input, is_end) = opt(preceded(tag(&b"end"[..]), ws1)).parse(input)?;
    let is_end = is_end.is_some();
    let (input, _) = tag(&b"enum"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, name) = name(input)?;
    let (input, multiplicity) = opt(multiplicity_node).parse(input)?;
    let (input, header) = feature_usage_header(input)?;
    let (input, body) = attribute_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            EnumerationUsage {
                name,
                type_name: header.type_reference,
                multiplicity,
                body,
                is_end,
                membership: Membership::feature(visibility, visibility_span),
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

    // --- parser work item 4b (final sweep): Membership on EnumDef/EnumerationUsage ---

    #[test]
    fn enum_def_visibility_prefix_is_captured_on_membership() {
        let (rest, node) = enum_def(input("private enum def E1;")).expect("enum def");
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
    fn enum_def_without_visibility_prefix_has_no_membership_visibility() {
        let (rest, node) = enum_def(input("enum def E1;")).expect("enum def");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.membership.visibility, None);
    }

    #[test]
    fn enum_usage_visibility_prefix_is_captured_on_membership() {
        let (_, node) = enum_usage(input("protected enum e1 : E1;")).expect("enum usage");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Protected)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::FeatureMembership
        );
    }

    #[test]
    fn enum_usage_without_visibility_prefix_has_no_membership_visibility() {
        let (_, node) = enum_usage(input("enum e1 : E1;")).expect("enum usage");
        assert_eq!(node.value.membership.visibility, None);
    }
}
