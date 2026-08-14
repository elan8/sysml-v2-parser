//! Enumeration definition parsing (BNF EnumerationDefinition).

use crate::ast::{EnumDef, EnumeratedValue, EnumerationBody, EnumerationUsage, Membership, Node};
use crate::parser::attribute::attribute_body;
use crate::parser::body::advance_to_closing_brace;
use crate::parser::definition_prefix::{parse_definition_prefix, DefinitionPrefixOptions};
use crate::parser::lex::{name, take_until_terminator, visibility_prefix, ws1, ws_and_comments};
use crate::parser::node_from_to;
use crate::parser::requirement::{comment_annotation, doc_comment};
use crate::parser::usage::{feature_usage_header, multiplicity_node};
use crate::parser::Input;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use nom::Parser;

/// Enumerated value: optional `enum` keyword + name + `;`. The span covers the name only (not
/// the discarded inline body or `= expr` initializer) so it stays stable regardless of which
/// trailing form is present.
fn enumerated_value(input: Input<'_>) -> IResult<Input<'_>, Node<EnumeratedValue>> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = nom::combinator::opt(preceded(tag(&b"enum"[..]), ws1)).parse(input)?;
    let name_start = input;
    let (input, n) = name(input)?;
    let value_node = node_from_to(name_start, input, EnumeratedValue { name: n });
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b"{") {
        let (input, _) = delimited(
            tag(&b"{"[..]),
            advance_to_closing_brace,
            preceded(ws_and_comments, tag(&b"}"[..])),
        )
        .parse(input)?;
        Ok((input, value_node))
    } else {
        let (input, _) = opt(preceded(
            preceded(ws_and_comments, tag(&b"="[..])),
            preceded(ws_and_comments, |i| take_until_terminator(i, b";")),
        ))
        .parse(input)?;
        let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
        Ok((input, value_node))
    }
}

fn enumeration_body(input: Input<'_>) -> IResult<Input<'_>, EnumerationBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, EnumerationBody::Semicolon));
    }
    let (mut input, _) = tag(&b"{"[..]).parse(input)?;
    let mut values = Vec::new();
    loop {
        let (next, _) = ws_and_comments(input)?;
        input = next;
        if input.fragment().starts_with(b"}") {
            let (input, _) = preceded(ws_and_comments, tag(&b"}"[..])).parse(input)?;
            return Ok((input, EnumerationBody::Brace { elements: values }));
        }
        if let Ok((next, _)) = doc_comment(input) {
            input = next;
            continue;
        }
        if let Ok((next, _)) = comment_annotation(input) {
            input = next;
            continue;
        }
        match enumerated_value(input) {
            Ok((next, value)) => {
                values.push(value);
                input = next;
            }
            Err(_) => {
                let (input, _) = advance_to_closing_brace(input)?;
                let (input, _) = preceded(ws_and_comments, tag(&b"}"[..])).parse(input)?;
                return Ok((input, EnumerationBody::Brace { elements: values }));
            }
        }
    }
}

/// Enumeration definition: `enum def` Identification EnumerationBody.
pub(crate) fn enum_def(input: Input<'_>) -> IResult<Input<'_>, Node<EnumDef>> {
    let start = input;
    let (input, prefix) = parse_definition_prefix(
        input,
        DefinitionPrefixOptions::new(b"enum")
            .def_required()
            .no_abstract()
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

#[cfg(test)]
mod enumerated_value_tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    // Regression coverage: `enumerated_value` used to return a bare `String`, so enumerated
    // values had no source range and couldn't become addressable Spec42/Babel42 elements. It now
    // returns a `Node<EnumeratedValue>` whose span covers just the name, regardless of whether a
    // discarded inline body or `= expr` initializer follows.

    #[test]
    fn enumerated_value_bare_semicolon_form_has_name_span() {
        let (rest, node) = enumerated_value(input("active;")).expect("enumerated value");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "active");
        assert_eq!(node.span.offset, 0);
        assert_eq!(node.span.len, "active".len());
    }

    #[test]
    fn enumerated_value_inline_body_form_has_name_only_span() {
        let (rest, node) =
            enumerated_value(input("active { doc /* x */ }")).expect("enumerated value");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "active");
        assert_eq!(node.span.offset, 0);
        assert_eq!(node.span.len, "active".len());
    }

    #[test]
    fn enumerated_value_initializer_form_has_name_only_span() {
        let (rest, node) = enumerated_value(input("active = 1;")).expect("enumerated value");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "active");
        assert_eq!(node.span.offset, 0);
        assert_eq!(node.span.len, "active".len());
    }

    #[test]
    fn enumerated_value_optional_enum_keyword_prefix_offsets_the_name_span() {
        let (rest, node) = enumerated_value(input("enum active;")).expect("enumerated value");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.name, "active");
        assert_eq!(node.span.offset, "enum ".len());
        assert_eq!(node.span.len, "active".len());
    }

    #[test]
    fn enumeration_body_collects_spanned_values_in_order() {
        let (rest, body) =
            enumeration_body(input("{ active; inactive = 1; degraded { } }")).expect("body");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        let EnumerationBody::Brace { elements: values } = body else {
            panic!("expected brace body");
        };
        let names: Vec<_> = values.iter().map(|v| v.value.name.as_str()).collect();
        assert_eq!(names, ["active", "inactive", "degraded"]);
    }
}
