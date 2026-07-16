//! Definition subclassification (`:>` / `specializes`) parsing.

use crate::ast::{Node, RelationshipTarget, TypingKind, TypingRelationship};
use crate::parser::lex::{
    qualified_name_segments, specialization_operator, starts_with_keyword, take_until_terminator,
    ws_and_comments,
};
use crate::parser::{span_from_to, with_span, Input};
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;
use nom_locate::LocatedSpan;

/// Wrap subclassification target(s) in a `TypingRelationship` node. Definition-level
/// `specializes` clauses are always subclassification (never `:` typing) and never conjugated —
/// `qualified_name_segments` doesn't accept a leading `~` here, unlike usage-level typing (see
/// `parser::usage::conjugated_qualified_name`).
fn subclassification_node(
    target: Vec<Node<RelationshipTarget>>,
    span: crate::ast::Span,
) -> Node<TypingRelationship> {
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

/// Parse one `::`-qualified name into a single `Node<RelationshipTarget>`, with the span covering
/// just that target (not the whole surrounding clause).
fn qualified_name_target(input: Input<'_>) -> IResult<Input<'_>, Node<RelationshipTarget>> {
    let (input, (span, segments)) = with_span(qualified_name_segments).parse(input)?;
    Ok((
        input,
        Node::new(span.clone(), RelationshipTarget { segments, span }),
    ))
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
    let Some((_, first)) = opt_first else {
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
    Ok((input, Some(subclassification_node(bases, span))))
}

fn starts_with_typing_colon(fragment: &[u8]) -> bool {
    fragment.starts_with(b":") && !fragment.starts_with(b":>")
}

/// Wrap a `:` typing target in a `TypingRelationship` node with `kind: Typing` (as opposed to
/// [`subclassification_node`]'s `Subclassification`).
fn typing_node(
    target: Vec<Node<RelationshipTarget>>,
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
        },
    )
}

/// Extract the type target from a plain `: Type` header that has no `:>`/`specializes` clause,
/// e.g. `": MyPortType"` -> `Some((target, false))`, `": ~PortConjugate"` ->
/// `Some((target, true))`. Returns `None` if nothing usable follows the colon.
fn typing_target_from_header(header: &str) -> Option<(Node<RelationshipTarget>, bool)> {
    let after_colon = header.strip_prefix(':')?.trim_start();
    let (rest, is_conjugated) = match after_colon.strip_prefix('~') {
        Some(rest) => (rest, true),
        None => (after_colon, false),
    };
    let span_input: Input<'_> = LocatedSpan::new(rest.as_bytes());
    let (_, target) = qualified_name_target(span_input).ok()?;
    Some((target, is_conjugated))
}

/// Extract the subclassification target(s) from a `:>`/`specializes` header fragment as raw text,
/// e.g. `": Connection[0..*] nonunique :> linkObjects, parts"` -> `Some([linkObjects, parts])`.
/// Reparses the extracted tail text with [`qualified_name_target`]/comma-splitting rather than
/// keeping it as an opaque joined string, so multi-target clauses stay structured
/// (parser work item 2).
fn specializes_from_header_text(header: &str) -> Option<Vec<Node<RelationshipTarget>>> {
    let trimmed = header.trim();
    let tail = if let Some(pos) = trimmed.find(":>") {
        let tail = trimmed[pos + 2..].trim();
        (!tail.is_empty()).then_some(tail)
    } else {
        trimmed
            .as_bytes()
            .windows(b"specializes".len())
            .position(|window| window.eq_ignore_ascii_case(b"specializes"))
            .and_then(|pos| {
                let tail = trimmed[pos + b"specializes".len()..].trim();
                (!tail.is_empty()).then_some(tail)
            })
    }?;
    let span_input: Input<'_> = LocatedSpan::new(tail.as_bytes());
    let (_, targets) = (
        qualified_name_target,
        many0(preceded(
            preceded(ws_and_comments, tag(&b","[..])),
            preceded(ws_and_comments, qualified_name_target),
        )),
    )
        .parse(span_input)
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
        if let Some(targets) = specializes_from_header_text(&header) {
            return Ok((
                input,
                (Some(subclassification_node(targets, span)), Some(header)),
            ));
        }
        // No `:>`/`specializes` clause -- the whole header is a plain `: Type` typing clause
        // (e.g. `port p1: MyPortType;` at package level). Previously this fell through to
        // `None` here, silently dropping the type reference instead of surfacing it as a
        // `Typing`-kind relationship the way `:>` surfaces a `Subclassification`-kind one.
        if let Some((target, is_conjugated)) = typing_target_from_header(&header) {
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
    use crate::parser::usage::targets_display_string;
    use nom_locate::LocatedSpan;

    fn span_input(text: &str) -> Input<'_> {
        LocatedSpan::new(text.as_bytes())
    }

    #[test]
    fn header_after_ident_skips_typing_and_extracts_specializes() {
        let input = span_input(": Connection[0..*] nonunique :> linkObjects, parts");
        let (rest, (specializes, raw_header)) =
            parse_optional_definition_header_with_raw(input).expect("header");
        assert!(rest.fragment().is_empty());
        assert_eq!(
            specializes.map(|n| targets_display_string(&n.value.target)),
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
            specializes.map(|n| targets_display_string(&n.value.target)),
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
        assert_eq!(targets_display_string(&node.value.target), "MyPortType");
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
        assert_eq!(targets_display_string(&node.value.target), "PortConjugate");
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
