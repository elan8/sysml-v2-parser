//! Attribute definition and usage parsing.

use crate::ast::{AttributeBody, AttributeBodyElement, AttributeDef, AttributeUsage, InOut, Node};
use crate::parser::body::parse_structured_brace_members;
use crate::parser::build_recovery_error_node_from_span;
use crate::parser::expr::expression;
use crate::parser::lex::{
    capture_opaque_member, identification, name, subset_operator, ws1, ws_and_comments,
};
use crate::parser::node_from_to;
use crate::parser::requirement::doc_comment;
use crate::parser::usage::{
    multiplicity, optional_typings, prefix_redefinition_target, specialization_clauses, typings,
};
use crate::parser::with_span;
use crate::parser::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value};
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

const ATTRIBUTE_BODY_STARTERS: &[&[u8]] = &[
    b"doc",
    b"attribute",
    b"comment",
    b"@",
    b"#",
    b":>>",
    b":>",
    b":",
    b"ref",
    b"item",
    b"assert",
    b"constraint",
    b"private",
    b"derived",
    b"abstract",
    b"part",
    b"binding",
    b"connection",
];

const ATTRIBUTE_OPAQUE_STARTERS: &[&[u8]] = &[
    b"ref",
    b"item",
    b"assert",
    b"constraint",
    b"private",
    b"derived",
    b"abstract",
    b"part",
    b"binding",
    b"connection",
    b":>>",
    b":>",
    b"attribute",
];

const METADATA_BODY_STARTERS: &[&[u8]] = &[
    b"doc",
    b"attribute",
    b"ref",
    b"comment",
    b":>",
    b":>>",
    b":",
    b"derived",
    b"item",
    b"abstract",
];

const METADATA_OPAQUE_STARTERS: &[&[u8]] = &[b"derived", b"item", b"abstract", b"ref"];

fn local_name_from_qualified_name(qname: &str) -> String {
    qname.rsplit("::").next().unwrap_or(qname).to_string()
}

fn is_reserved_shorthand_starter(name: &str) -> bool {
    matches!(
        name,
        "interface"
            | "part"
            | "connect"
            | "bind"
            | "perform"
            | "allocate"
            | "port"
            | "state"
            | "satisfy"
            | "action"
            | "attribute"
            | "ref"
            | "doc"
            | "metadata"
            | "filter"
            | "use"
            | "view"
            | "viewpoint"
            | "render"
            | "rendering"
            | "requirement"
            | "require"
            | "concern"
            | "actor"
            | "item"
            | "individual"
            | "constraint"
            | "calc"
            | "enum"
            | "occurrence"
    )
}

fn ignored_feature_modifiers(input: Input<'_>) -> IResult<Input<'_>, ()> {
    let (input, _) = many0(preceded(
        ws_and_comments,
        alt((
            value((), multiplicity),
            value((), tag(&b"nonunique"[..])),
            value((), tag(&b"unique"[..])),
            value((), tag(&b"ordered"[..])),
            value((), tag(&b"nonordered"[..])),
        )),
    ))
    .parse(input)?;
    Ok((input, ()))
}

/// Value part: `= expr` | `:= expr` | `default = expr` | `default := expr` (BNF FeatureValue).
fn value_part(input: Input<'_>) -> IResult<Input<'_>, Node<crate::ast::Expression>> {
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = alt((
        preceded(tag(&b"="[..]), ws_and_comments),
        preceded(tag(&b":="[..]), ws_and_comments),
        preceded(
            preceded(tag(&b"default"[..]), ws1),
            alt((
                preceded(alt((tag(&b"="[..]), tag(&b":="[..]))), ws_and_comments),
                ws_and_comments,
            )),
        ),
    ))
    .parse(input)?;
    expression(input)
}

enum MetadataBindingPrefix {
    Subsets,
    Redefines,
}

fn attribute_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<AttributeBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = alt((
        map(doc_comment, AttributeBodyElement::Doc),
        map(
            |i| attribute_def(i, true),
            AttributeBodyElement::AttributeDef,
        ),
        map(attribute_usage, AttributeBodyElement::AttributeUsage),
        map(attribute_feature_binding, AttributeBodyElement::AttributeUsage),
        map(
            |i| capture_opaque_member(i, ATTRIBUTE_OPAQUE_STARTERS),
            AttributeBodyElement::Other,
        ),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

/// Attribute body feature binding: (`:>>` | `:>`)? name (`:` type)? (`=` value)? (`;` | `{` body `}`).
///
/// Catalog unit definitions use this shape for `unitConversion` redefinitions, e.g.
/// `:>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; }`.
fn attribute_feature_binding(input: Input<'_>) -> IResult<Input<'_>, Node<AttributeUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, prefix) = nom::combinator::opt(alt((
        map(
            preceded(ws_and_comments, tag(&b":>>"[..])),
            |_| MetadataBindingPrefix::Redefines,
        ),
        map(
            preceded(ws_and_comments, subset_operator),
            |_| MetadataBindingPrefix::Subsets,
        ),
    )))
    .parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (name_span, name_str)) = with_span(name).parse(input)?;
    if is_reserved_shorthand_starter(&name_str) {
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, typing_result) = optional_typings(input)?;
    let (typing_span, typing) = typing_result
        .map(|(span, s)| (Some(span), Some(s)))
        .unwrap_or((None, None));
    let (input, _) = ignored_feature_modifiers(input)?;
    let (input, value) =
        nom::combinator::opt(preceded(ws_and_comments, value_part)).parse(input)?;
    let (input, _) = ignored_feature_modifiers(input)?;
    let (input, body) = attribute_body(input)?;
    let (subsets, redefines) = match prefix {
        Some(MetadataBindingPrefix::Subsets) => (Some(name_str.clone()), None),
        Some(MetadataBindingPrefix::Redefines) => (None, Some(name_str.clone())),
        None => (None, None),
    };
    Ok((
        input,
        node_from_to(
            start,
            input,
            AttributeUsage {
                name: name_str,
                typing,
                subsets,
                redefines,
                references: None,
                crosses: None,
                value,
                body,
                name_span: Some(name_span),
                typing_span,
                redefines_span: None,
                direction: None,
            },
        ),
    ))
}

fn attribute_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<AttributeBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        ATTRIBUTE_BODY_STARTERS,
        "attribute body",
        "recovered_attribute_body_element",
    );
    node_from_to(
        start,
        end,
        AttributeBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

/// Attribute body: `;` or `{` AttributeBodyElement* `}`.
pub(crate) fn attribute_body(input: Input<'_>) -> IResult<Input<'_>, AttributeBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, AttributeBody::Semicolon));
    }
    let (input, elements) = parse_structured_brace_members(
        input,
        ATTRIBUTE_BODY_STARTERS,
        "attribute body",
        "recovered_attribute_body_element",
        attribute_body_element,
        attribute_body_recovery,
    )?;
    Ok((input, AttributeBody::Brace { elements }))
}

/// Attribute definition: 'attribute' 'def' name ( ':>' | ':' )? qualified_name? body
///
/// When `disambiguate_from_usage` is true (definition bodies that also accept usages), any
/// declaration without an explicit `def` keyword is left for [`attribute_usage`] — the `def`
/// keyword is the sole discriminator between an `AttributeDef` and an `AttributeUsage`, never
/// inferred from typing, modifiers, or the presence of a value. Package-level attributes pass
/// false, since only definitions are legal there.
pub(crate) fn attribute_def(
    input: Input<'_>,
    disambiguate_from_usage: bool,
) -> IResult<Input<'_>, Node<AttributeDef>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = nom::combinator::opt(preceded(
        alt((
            tag(&b"private"[..]),
            tag(&b"protected"[..]),
            tag(&b"public"[..]),
        )),
        ws1,
    ))
    .parse(input)?;
    let (input, _) = nom::combinator::opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
    let (input, _) = tag(&b"attribute"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, has_def) = nom::combinator::opt(preceded(tag(&b"def"[..]), ws1)).parse(input)?;
    let has_def = has_def.is_some();
    if disambiguate_from_usage && !has_def {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let ident_start = input;
    let (input, ident) = identification(input)?;
    let name_span = ident
        .name
        .as_ref()
        .map(|_| crate::parser::span_from_to(ident_start, input));
    let name_str = ident
        .name
        .clone()
        .or_else(|| ident.short_name.clone())
        .ok_or_else(|| {
            nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag))
        })?;
    let short_name = ident.short_name.clone();
    let (input, typing_result) = optional_typings(input)?;
    let (typing_span, typing) = typing_result
        .map(|(span, s)| (Some(span), Some(s)))
        .unwrap_or((None, None));
    let (input, _) = ignored_feature_modifiers(input)?;
    let (input, leading_clauses) = specialization_clauses(input)?;
    let leading_subset = leading_clauses.subsets;
    let (typing_span, typing, leading_value) = if typing.is_none() {
        leading_subset
            .map(|(name, value)| (None, Some(name), value))
            .unwrap_or((typing_span, typing, None))
    } else {
        (typing_span, typing, None)
    };
    let (input, value) =
        nom::combinator::opt(preceded(ws_and_comments, value_part)).parse(input)?;
    let value = value.or(leading_value);
    let value_span = value.as_ref().map(|node| node.span.clone());
    let (input, _) = specialization_clauses(input)?;
    let (input, _) = ignored_feature_modifiers(input)?;
    let (input, body) = attribute_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            AttributeDef {
                name: name_str,
                short_name,
                typing,
                value,
                body,
                name_span,
                typing_span,
                value_span,
            },
        ),
    ))
}

pub(crate) fn direction_prefix(input: Input<'_>) -> IResult<Input<'_>, InOut> {
    alt((
        map(preceded(tag(&b"in"[..]), ws1), |_| InOut::In),
        map(preceded(tag(&b"out"[..]), ws1), |_| InOut::Out),
        map(preceded(tag(&b"inout"[..]), ws1), |_| InOut::InOut),
    ))
    .parse(input)
}

/// `in`/`out`/`inout attribute` usage (port def bodies): direction + [`attribute_usage`].
pub(crate) fn directed_attribute_usage(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<AttributeUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, direction) = direction_prefix(input)?;
    let (input, mut usage) = attribute_usage(input)?;
    usage.value.direction = Some(direction);
    Ok((input, node_from_to(start, input, usage.value)))
}

/// Attribute usage:
/// - `attribute` name ( (`:>` | `:`) type )? ( `redefines` qualified_name )? ( '=' value )? body
/// - `attribute :>>` qualified_name ( '=' value )? body
pub(crate) fn attribute_usage(input: Input<'_>) -> IResult<Input<'_>, Node<AttributeUsage>> {
    enum AttributeUsageHead {
        Named {
            name_span: crate::ast::Span,
            name: String,
        },
        PrefixRedefines {
            redefines_span: crate::ast::Span,
            redefines: String,
        },
    }

    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"attribute"[..]).parse(input)?;
    let (input, _) = ws1(input)?;
    let (input, usage_head) = alt((
        map(
            preceded(ws_and_comments, prefix_redefinition_target),
            |(redefines_span, redefines)| AttributeUsageHead::PrefixRedefines {
                redefines_span,
                redefines,
            },
        ),
        map(with_span(name), |(name_span, name)| {
            AttributeUsageHead::Named { name_span, name }
        }),
    ))
    .parse(input)?;
    let (input, name_span, name_str, typing_span, typing, redefines_span, redefines) =
        match usage_head {
            AttributeUsageHead::PrefixRedefines {
                redefines_span,
                redefines,
            } => {
                let (input, typing_result) = optional_typings(input)?;
                let (typing_span, typing) = typing_result
                    .map(|(span, s)| (Some(span), Some(s)))
                    .unwrap_or((None, None));
                let (input, _) = ignored_feature_modifiers(input)?;
                (
                    input,
                    None,
                    local_name_from_qualified_name(&redefines),
                    typing_span,
                    typing,
                    Some(redefines_span),
                    Some(redefines),
                )
            }
            AttributeUsageHead::Named { name_span, name } => {
                let (input, typing_result) = optional_typings(input)?;
                let (typing_span, typing) = typing_result
                    .map(|(span, s)| (Some(span), Some(s)))
                    .unwrap_or((None, None));
                let (input, _) = ignored_feature_modifiers(input)?;
                (
                    input,
                    Some(name_span),
                    name,
                    typing_span,
                    typing,
                    None,
                    None,
                )
            }
        };
    let (input, leading_clauses) = specialization_clauses(input)?;
    let (input, _) = ignored_feature_modifiers(input)?;
    let leading_subsets_value = leading_clauses
        .subsets
        .as_ref()
        .and_then(|(_, value)| value.clone());
    let (input, value) =
        nom::combinator::opt(preceded(ws_and_comments, value_part)).parse(input)?;
    let (input, trailing_clauses) = specialization_clauses(input)?;
    let (input, _) = ignored_feature_modifiers(input)?;
    let redefines = trailing_clauses
        .redefines
        .or(leading_clauses.redefines)
        .or(redefines);
    let subsets = trailing_clauses
        .subsets
        .or(leading_clauses.subsets)
        .map(|(target, _)| target);
    let references = trailing_clauses.references.or(leading_clauses.references);
    let crosses = trailing_clauses.crosses.or(leading_clauses.crosses);
    let value = value.or(leading_subsets_value);
    let (input, body) = attribute_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            AttributeUsage {
                name: name_str,
                typing,
                subsets,
                redefines,
                references,
                crosses,
                value,
                body,
                name_span,
                typing_span,
                redefines_span,
                direction: None,
            },
        ),
    ))
}

/// Metadata usage body binding: `ref`? (`:>` | `:>>`)? name (`:` type)? (`=` value)? `;`
///
/// Covers §7.27.2 forms such as `approved = true;`, `ref :>> approved = true;`,
/// `:> annotatedElement : Type;`, and `:>> baseType = expr meta Type;`.
fn metadata_binding(input: Input<'_>) -> IResult<Input<'_>, Node<AttributeUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) =
        nom::combinator::opt(preceded(ws_and_comments, tag(&b"ref"[..]))).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, prefix) = nom::combinator::opt(alt((
        map(
            preceded(ws_and_comments, tag(&b":>>"[..])),
            |_| MetadataBindingPrefix::Redefines,
        ),
        map(
            preceded(ws_and_comments, subset_operator),
            |_| MetadataBindingPrefix::Subsets,
        ),
    )))
    .parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (name_span, name_str)) = with_span(name).parse(input)?;
    if is_reserved_shorthand_starter(&name_str) {
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, typing_result) = optional_typings(input)?;
    let (typing_span, typing) = typing_result
        .map(|(span, s)| (Some(span), Some(s)))
        .unwrap_or((None, None));
    let (input, _) = ignored_feature_modifiers(input)?;
    let (input, value) =
        nom::combinator::opt(preceded(ws_and_comments, value_part)).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    let (subsets, redefines) = match prefix {
        Some(MetadataBindingPrefix::Subsets) => (Some(name_str.clone()), None),
        Some(MetadataBindingPrefix::Redefines) => (None, Some(name_str.clone())),
        None => (None, None),
    };
    Ok((
        input,
        node_from_to(
            start,
            input,
            AttributeUsage {
                name: name_str,
                typing,
                subsets,
                redefines,
                references: None,
                crosses: None,
                value,
                body: AttributeBody::Semicolon,
                name_span: Some(name_span),
                typing_span,
                redefines_span: None,
                direction: None,
            },
        ),
    ))
}

fn metadata_body_element(input: Input<'_>) -> IResult<Input<'_>, Node<AttributeBodyElement>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, elem) = alt((
        map(doc_comment, AttributeBodyElement::Doc),
        map(
            |i| attribute_def(i, true),
            AttributeBodyElement::AttributeDef,
        ),
        map(attribute_usage, AttributeBodyElement::AttributeUsage),
        map(metadata_binding, AttributeBodyElement::AttributeUsage),
        map(
            |i| capture_opaque_member(i, METADATA_OPAQUE_STARTERS),
            AttributeBodyElement::Other,
        ),
    ))
    .parse(input)?;
    Ok((input, node_from_to(start, input, elem)))
}

fn metadata_body_recovery(start: Input<'_>, end: Input<'_>) -> Node<AttributeBodyElement> {
    let recovery = build_recovery_error_node_from_span(
        start,
        end,
        METADATA_BODY_STARTERS,
        "metadata body",
        "recovered_metadata_body_element",
    );
    node_from_to(
        start,
        end,
        AttributeBodyElement::Error(node_from_to(start, end, recovery)),
    )
}

/// Metadata annotation/usage body: `;` or `{` members `}` (structured attribute bindings).
pub(crate) fn metadata_body(input: Input<'_>) -> IResult<Input<'_>, AttributeBody> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b";") {
        let (input, _) = tag(&b";"[..]).parse(input)?;
        return Ok((input, AttributeBody::Semicolon));
    }
    let (input, elements) = parse_structured_brace_members(
        input,
        METADATA_BODY_STARTERS,
        "metadata body",
        "recovered_metadata_body_element",
        metadata_body_element,
        metadata_body_recovery,
    )?;
    Ok((input, AttributeBody::Brace { elements }))
}

/// Shorthand attribute usage (no `attribute` keyword) commonly used inside part bodies.
///
/// Supports:
/// - `name : Type ;`
/// - `name : Type = expr ;`
/// - `:>> name : Type = expr ;` (leading `:>>` ignored; treated as a usage)
pub(crate) fn attribute_usage_shorthand(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<AttributeUsage>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) =
        nom::combinator::opt(preceded(ws_and_comments, tag(&b":>>"[..]))).parse(input)?;
    let (input, (name_span, name_str)) = with_span(name).parse(input)?;
    if is_reserved_shorthand_starter(&name_str) {
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, _) = typings(input)?;
    // Keep shorthand values on the shared expression path so precedence/parentheses are preserved.
    let (input, value) =
        nom::combinator::opt(preceded(ws_and_comments, value_part)).parse(input)?;
    let (input, _) = preceded(ws_and_comments, tag(&b";"[..])).parse(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            AttributeUsage {
                name: name_str,
                typing: None,
                subsets: None,
                redefines: None,
                references: None,
                crosses: None,
                value,
                body: AttributeBody::Semicolon,
                name_span: Some(name_span),
                typing_span: None,
                redefines_span: None,
                direction: None,
            },
        ),
    ))
}

#[cfg(test)]
mod attribute_body_tests {
    use super::*;
    use crate::parser::Input;
    use nom_locate::LocatedSpan;

    fn input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    #[test]
    fn feature_binding_parses_unit_conversion_prefix_form() {
        let text = ":>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; }";
        let (rest, node) = attribute_feature_binding(input(text)).expect("feature binding");
        assert!(rest.fragment().is_empty(), "rest: {:?}", rest.fragment());
        assert_eq!(node.value.redefines.as_deref(), Some("unitConversion"));
        assert_eq!(node.value.typing.as_deref(), Some("ConversionByPrefix"));
        let AttributeBody::Brace { elements } = &node.value.body else {
            panic!("expected brace body");
        };
        assert_eq!(elements.len(), 2);
    }
}
