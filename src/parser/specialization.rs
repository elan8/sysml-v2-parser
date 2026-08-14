//! Definition subclassification (`:>` / `specializes`) parsing.

use crate::ast::{Node, QualifiedReferenceId, TypingKind, TypingRelationship};
use crate::parser::lex::{
    qualified_reference, specialization_operator, starts_with_keyword, take_until_terminator,
    ws_and_comments,
};
use crate::parser::{span_from_to, Input};
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

/// Wrap subclassification target(s) in a `TypingRelationship` node. Definition-level
/// `specializes` clauses are always subclassification (never `:` typing) and never conjugated —
/// `qualified_name_segments` doesn't accept a leading `~` here, unlike usage-level typing (see
/// `parser::usage::conjugated_qualified_name`).
fn subclassification_node(
    target: Vec<QualifiedReferenceId>,
    span: crate::ast::Span,
    spelling: crate::ast::TypingSpelling,
) -> Node<TypingRelationship> {
    Node::new(
        span.clone(),
        TypingRelationship {
            target,
            kind: TypingKind::Subclassification,
            span,
            is_conjugated: false,
            is_implied: false,
            spelling,
        },
    )
}

/// Parse one `::`-qualified name into the document reference arena.
fn qualified_name_target(input: Input<'_>) -> IResult<Input<'_>, QualifiedReferenceId> {
    qualified_reference(input)
}

/// Optional definition subclassification: `:> Base` or `specializes Base`, with optional `, Base2`.
pub(crate) fn parse_optional_definition_specialization(
    input: Input<'_>,
) -> IResult<Input<'_>, Option<Node<TypingRelationship>>> {
    let before_specializes = input;
    let (input, opt_first) = opt((
        preceded(ws_and_comments, specialization_operator),
        preceded(ws_and_comments, qualified_name_target),
    ))
    .parse(input)?;
    let Some((spelling, first)) = opt_first else {
        return Ok((input, None));
    };
    let (input, rest) = many0(preceded(
        preceded(ws_and_comments, tag(&b","[..])),
        preceded(ws_and_comments, qualified_name_target),
    ))
    .parse(input)?;
    let mut bases = vec![first];
    bases.extend(rest);
    let span = span_from_to(before_specializes, input);
    Ok((input, Some(subclassification_node(bases, span, spelling))))
}

fn starts_with_typing_colon(fragment: &[u8]) -> bool {
    fragment.starts_with(b":") && !fragment.starts_with(b":>")
}

/// Wrap a `:` typing target in a `TypingRelationship` node with `kind: Typing` (as opposed to
/// [`subclassification_node`]'s `Subclassification`).
fn typing_node(
    target: Vec<QualifiedReferenceId>,
    span: crate::ast::Span,
    is_conjugated: bool,
) -> Node<TypingRelationship> {
    Node::new(
        span.clone(),
        TypingRelationship {
            target,
            kind: TypingKind::Typing,
            span,
            is_conjugated,
            is_implied: false,
            spelling: crate::ast::TypingSpelling::Operator,
        },
    )
}

/// Extract the type target from a plain `: Type` header that has no `:>`/`specializes` clause,
/// e.g. `": MyPortType"` -> `Some((target, false))`, `": ~PortConjugate"` ->
/// `Some((target, true))`. Returns `None` if nothing usable follows the colon.
fn typing_target_from_header(input: Input<'_>) -> IResult<Input<'_>, (QualifiedReferenceId, bool)> {
    let (input, _) = tag(&b":"[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, conjugated) = opt(tag(&b"~"[..])).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, target) = qualified_name_target(input)?;
    let is_conjugated = conjugated.is_some();
    Ok((input, (target, is_conjugated)))
}

/// Extract the subclassification target(s) from a `:>`/`specializes` header fragment,
/// e.g. `": Connection[0..*] nonunique :> linkObjects, parts"` -> `Some([linkObjects, parts])`.
/// The original parser input is advanced to the discovered tail before parsing references, so
/// every allocated identity retains document-relative source provenance.
fn specializes_from_header_input(
    header: &str,
    input: Input<'_>,
) -> Option<Vec<QualifiedReferenceId>> {
    let bytes = header.as_bytes();
    let tail_offset = if let Some(pos) = bytes.windows(2).position(|window| window == b":>") {
        Some(pos + 2)
    } else {
        bytes
            .windows(b"specializes".len())
            .position(|window| window.eq_ignore_ascii_case(b"specializes"))
            .map(|pos| pos + b"specializes".len())
    }?;
    let (tail_input, _) =
        nom::bytes::complete::take::<usize, Input<'_>, nom::error::Error<Input<'_>>>(tail_offset)
            .parse(input)
            .ok()?;
    let (tail_input, _) = ws_and_comments(tail_input).ok()?;
    let (_, targets) = (
        qualified_name_target,
        many0(preceded(
            preceded(ws_and_comments, tag(&b","[..])),
            preceded(ws_and_comments, qualified_name_target),
        )),
    )
        .parse(tail_input)
        .ok()?;
    let (first, rest) = targets;
    let mut bases = vec![first];
    bases.extend(rest);
    Some(bases)
}

/// After `identification`, parse optional typed header and/or subclassification, plus the raw
/// text swallowed by the plain `: Type` scanning branch (`None` for every other branch, including
/// when there is no header at all).
///
/// Supports both:
/// - `def Name :> Base` / `specializes Base`
/// - library shorthand `abstract connection name : Type[multiplicity] :> redefines { ... }`
///
/// Callers such as `connection_def`/`interface_def` use the raw-text half of the result to
/// detect a `connect ...` clause that got discarded as unstructured header text instead of being
/// left for a sibling usage parser -- see `DefinitionPrefixOptions::reject_header_keyword`.
pub(crate) fn parse_optional_definition_header_with_raw(
    input: Input<'_>,
) -> IResult<Input<'_>, (Option<Node<TypingRelationship>>, Option<String>)> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b":>") || starts_with_keyword(input.fragment(), b"specializes")
    {
        let (input, specializes) = parse_optional_definition_specialization(input)?;
        return Ok((input, (specializes, None)));
    }
    if starts_with_typing_colon(input.fragment()) {
        let before_header = input;
        let (input, header) = take_until_terminator(input, b";{")?;
        let span = span_from_to(before_header, input);
        if let Some(targets) = specializes_from_header_input(&header, before_header) {
            return Ok((
                input,
                (
                    Some(subclassification_node(
                        targets,
                        span,
                        crate::ast::TypingSpelling::Operator,
                    )),
                    Some(header),
                ),
            ));
        }
        // No `:>`/`specializes` clause -- the whole header is a plain `: Type` typing clause
        // (e.g. `port p1: MyPortType;` at package level). Previously this fell through to
        // `None` here, silently dropping the type reference instead of surfacing it as a
        // `Typing`-kind relationship the way `:>` surfaces a `Subclassification`-kind one.
        if let Ok((_, (target, is_conjugated))) = typing_target_from_header(before_header) {
            return Ok((
                input,
                (
                    Some(typing_node(vec![target], span, is_conjugated)),
                    Some(header),
                ),
            ));
        }
        return Ok((input, (None, Some(header))));
    }
    Ok((input, (None, None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span_input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    #[test]
    fn header_after_ident_skips_typing_and_extracts_specializes() {
        let input = span_input(": Connection[0..*] nonunique :> linkObjects, parts");
        let (rest, (specializes, raw_header)) =
            parse_optional_definition_header_with_raw(input).expect("header");
        assert!(rest.fragment().is_empty());
        assert_eq!(
            specializes.map(|node| {
                crate::parser::usage::reference_list_text(input, &node.value.target)
            }),
            Some("linkObjects, parts".to_string())
        );
        assert_eq!(
            raw_header.as_deref(),
            Some(": Connection[0..*] nonunique :> linkObjects, parts")
        );
    }

    #[test]
    fn header_after_ident_parses_direct_specializes() {
        let input = span_input(":> Base, Other");
        let (rest, (specializes, raw_header)) =
            parse_optional_definition_header_with_raw(input).expect("header");
        assert!(rest.fragment().is_empty());
        assert_eq!(
            specializes.map(|node| {
                crate::parser::usage::reference_list_text(input, &node.value.target)
            }),
            Some("Base, Other".to_string())
        );
        // The `:>`/`specializes` branch never goes through the raw text-scan path.
        assert_eq!(raw_header, None);
    }

    /// Regression: `port p1: MyPortType;` at package level (bare `: Type`, no `:>`) used to
    /// silently drop the type reference entirely -- `specializes_from_header_text` found no
    /// `:>`/`specializes`, so the whole header was discarded. It must now surface as a
    /// `Typing`-kind relationship.
    #[test]
    fn header_after_ident_captures_bare_typing_colon_with_no_specializes() {
        let input = span_input(": MyPortType;");
        let (rest, (typing, raw_header)) =
            parse_optional_definition_header_with_raw(input).expect("header");
        assert_eq!(rest.fragment(), b";");
        let node = typing.expect("type reference must not be dropped");
        assert_eq!(
            crate::parser::usage::reference_list_text(input, &node.value.target),
            "MyPortType"
        );
        assert_eq!(node.value.kind, TypingKind::Typing);
        assert!(!node.value.is_conjugated);
        assert_eq!(raw_header.as_deref(), Some(": MyPortType"));
    }

    #[test]
    fn header_after_ident_captures_bare_conjugated_typing_colon() {
        let input = span_input(": ~PortConjugate {");
        let (_, (typing, _raw_header)) =
            parse_optional_definition_header_with_raw(input).expect("header");
        let node = typing.expect("type reference must not be dropped");
        assert_eq!(
            crate::parser::usage::reference_list_text(input, &node.value.target),
            "PortConjugate"
        );
        assert_eq!(node.value.kind, TypingKind::Typing);
        assert!(node.value.is_conjugated);
    }

    /// The `reject_header_keyword` mechanism (`DefinitionPrefixOptions`) relies on `contains_keyword`
    /// finding `connect` inside the raw header text that would otherwise be silently discarded --
    /// this is the exact shape that previously made `connection_def` misclassify a
    /// `connection link : Link connect a to b;` usage as a definition.
    #[test]
    fn header_after_ident_raw_header_contains_a_swallowed_connect_clause() {
        let input = span_input(": Link connect a to b;");
        let (_, (_typing, raw_header)) =
            parse_optional_definition_header_with_raw(input).expect("header");
        assert_eq!(raw_header.as_deref(), Some(": Link connect a to b"));
        assert!(crate::parser::lex::contains_keyword(
            raw_header.expect("raw header").as_bytes(),
            b"connect"
        ));
    }
}
