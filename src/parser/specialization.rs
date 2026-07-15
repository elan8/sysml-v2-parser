//! Definition subclassification (`:>` / `specializes`) parsing.

use crate::ast::{Node, TypingKind, TypingRelationship};
use crate::parser::lex::{
    qualified_name, specialization_operator, starts_with_keyword, take_until_terminator,
    ws_and_comments,
};
use crate::parser::{span_from_to, Input};
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;
use nom_locate::LocatedSpan;

/// Wrap a subclassification target string in a `TypingRelationship` node. Definition-level
/// `specializes` clauses are always subclassification (never `:` typing) and never conjugated —
/// `qualified_name` doesn't accept a leading `~` here, unlike usage-level typing (see
/// `parser::usage::conjugated_qualified_name`).
fn subclassification_node(target: String, span: crate::ast::Span) -> Node<TypingRelationship> {
    Node::new(
        span.clone(),
        TypingRelationship {
            target,
            kind: TypingKind::Subclassification,
            span,
            is_conjugated: false,
            is_implied: false,
        },
    )
}

/// Optional definition subclassification: `:> Base` or `specializes Base`, with optional `, Base2`.
pub(crate) fn parse_optional_definition_specialization(
    input: Input<'_>,
) -> IResult<Input<'_>, Option<Node<TypingRelationship>>> {
    let before_specializes = input;
    let (input, opt_first) = opt((
        preceded(ws_and_comments, specialization_operator),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let Some((_, first)) = opt_first else {
        return Ok((input, None));
    };
    let (input, rest) = many0(preceded(
        preceded(ws_and_comments, tag(&b","[..])),
        preceded(ws_and_comments, qualified_name),
    ))
    .parse(input)?;
    let specializes = if rest.is_empty() {
        first
    } else {
        let mut bases = vec![first];
        bases.extend(rest);
        bases.join(", ")
    };
    let span = span_from_to(before_specializes, input);
    Ok((input, Some(subclassification_node(specializes, span))))
}

fn starts_with_typing_colon(fragment: &[u8]) -> bool {
    fragment.starts_with(b":") && !fragment.starts_with(b":>")
}

/// Wrap a `:` typing target string in a `TypingRelationship` node with `kind: Typing` (as
/// opposed to [`subclassification_node`]'s `Subclassification`).
fn typing_node(target: String, span: crate::ast::Span, is_conjugated: bool) -> Node<TypingRelationship> {
    Node::new(
        span.clone(),
        TypingRelationship {
            target,
            kind: TypingKind::Typing,
            span,
            is_conjugated,
            is_implied: false,
        },
    )
}

/// Extract the type name from a plain `: Type` header that has no `:>`/`specializes` clause,
/// e.g. `": MyPortType"` -> `Some(("MyPortType", false))`, `": ~PortConjugate"` ->
/// `Some(("PortConjugate", true))`. Returns `None` if nothing usable follows the colon.
fn typing_target_from_header(header: &str) -> Option<(String, bool)> {
    let after_colon = header.strip_prefix(':')?.trim_start();
    let (rest, is_conjugated) = match after_colon.strip_prefix('~') {
        Some(rest) => (rest, true),
        None => (after_colon, false),
    };
    let span_input: Input<'_> = LocatedSpan::new(rest.as_bytes());
    let (_, target) = qualified_name(span_input).ok()?;
    Some((target, is_conjugated))
}

fn specializes_from_header_text(header: &str) -> Option<String> {
    let trimmed = header.trim();
    if let Some(pos) = trimmed.find(":>") {
        let tail = trimmed[pos + 2..].trim();
        if !tail.is_empty() {
            return Some(tail.to_string());
        }
    }
    if let Some(pos) = trimmed
        .as_bytes()
        .windows(b"specializes".len())
        .position(|window| window.eq_ignore_ascii_case(b"specializes"))
    {
        let tail = trimmed[pos + b"specializes".len()..].trim();
        if !tail.is_empty() {
            return Some(tail.to_string());
        }
    }
    None
}

/// After `identification`, parse optional typed header and/or subclassification.
///
/// Supports both:
/// - `def Name :> Base` / `specializes Base`
/// - library shorthand `abstract connection name : Type[multiplicity] :> redefines { ... }`
pub(crate) fn parse_optional_definition_header_after_identification(
    input: Input<'_>,
) -> IResult<Input<'_>, Option<Node<TypingRelationship>>> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b":>") || starts_with_keyword(input.fragment(), b"specializes")
    {
        return parse_optional_definition_specialization(input);
    }
    if starts_with_typing_colon(input.fragment()) {
        let before_header = input;
        let (input, header) = take_until_terminator(input, b";{")?;
        let span = span_from_to(before_header, input);
        if let Some(target) = specializes_from_header_text(&header) {
            return Ok((input, Some(subclassification_node(target, span))));
        }
        // No `:>`/`specializes` clause -- the whole header is a plain `: Type` typing clause
        // (e.g. `port p1: MyPortType;` at package level). Previously this fell through to
        // `None` here, silently dropping the type reference instead of surfacing it as a
        // `Typing`-kind relationship the way `:>` surfaces a `Subclassification`-kind one.
        if let Some((target, is_conjugated)) = typing_target_from_header(&header) {
            return Ok((input, Some(typing_node(target, span, is_conjugated))));
        }
        return Ok((input, None));
    }
    Ok((input, None))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn span_input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    #[test]
    fn header_after_ident_skips_typing_and_extracts_specializes() {
        let input = span_input(": Connection[0..*] nonunique :> linkObjects, parts");
        let (rest, specializes) =
            parse_optional_definition_header_after_identification(input).expect("header");
        assert!(rest.fragment().is_empty());
        assert_eq!(
            specializes.map(|n| n.value.target),
            Some("linkObjects, parts".to_string())
        );
    }

    #[test]
    fn header_after_ident_parses_direct_specializes() {
        let input = span_input(":> Base, Other");
        let (rest, specializes) =
            parse_optional_definition_header_after_identification(input).expect("header");
        assert!(rest.fragment().is_empty());
        assert_eq!(
            specializes.map(|n| n.value.target),
            Some("Base, Other".to_string())
        );
    }

    /// Regression: `port p1: MyPortType;` at package level (bare `: Type`, no `:>`) used to
    /// silently drop the type reference entirely -- `specializes_from_header_text` found no
    /// `:>`/`specializes`, so the whole header was discarded. It must now surface as a
    /// `Typing`-kind relationship.
    #[test]
    fn header_after_ident_captures_bare_typing_colon_with_no_specializes() {
        let input = span_input(": MyPortType;");
        let (rest, typing) =
            parse_optional_definition_header_after_identification(input).expect("header");
        assert_eq!(rest.fragment(), b";");
        let node = typing.expect("type reference must not be dropped");
        assert_eq!(node.value.target, "MyPortType");
        assert_eq!(node.value.kind, TypingKind::Typing);
        assert!(!node.value.is_conjugated);
    }

    #[test]
    fn header_after_ident_captures_bare_conjugated_typing_colon() {
        let input = span_input(": ~PortConjugate {");
        let (_, typing) =
            parse_optional_definition_header_after_identification(input).expect("header");
        let node = typing.expect("type reference must not be dropped");
        assert_eq!(node.value.target, "PortConjugate");
        assert_eq!(node.value.kind, TypingKind::Typing);
        assert!(node.value.is_conjugated);
    }
}
