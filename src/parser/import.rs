//! Import and relationship body parsing.

use crate::ast::{
    FilterPackageMember, Import, ImportShape, ImportSuffixSpans, ImportTarget, Membership,
    MembershipKind, Node, RelationshipBodyElement,
};
use crate::parser::body::relationship_body_annotations;
use crate::parser::expr::expression;
use crate::parser::lex::{qualified_reference, ws1, ws_and_comments};
use crate::parser::span::reference_transaction;
use crate::parser::Input;
use crate::parser::{node_from_to, span_from_to, with_span};
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::{preceded, terminated};
use nom::IResult;
use nom::Parser;

/// RelationshipBody: `;` or `{` doc/comment/rep/metadata* `}`.
pub(crate) fn relationship_body(
    input: Input<'_>,
) -> IResult<Input<'_>, Option<Vec<Node<RelationshipBodyElement>>>> {
    relationship_body_annotations(input)
}

fn filter_package_member(input: Input<'_>) -> IResult<Input<'_>, Node<FilterPackageMember>> {
    let (input, _) = ws_and_comments(input)?;
    let start = input;
    let (input, (open_bracket_span, _)) = with_span(tag(&b"["[..]))(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, expression) = expression(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (close_bracket_span, _)) = with_span(tag(&b"]"[..]))(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            FilterPackageMember {
                open_bracket_span,
                expression,
                close_bracket_span,
            },
        ),
    ))
}

fn filter_package_members(input: Input<'_>) -> IResult<Input<'_>, Vec<Node<FilterPackageMember>>> {
    many1(filter_package_member).parse(input)
}

fn import_suffix<'a>(
    input: Input<'a>,
    marker: &'static [u8],
) -> IResult<Input<'a>, ImportSuffixSpans> {
    let start = input;
    let (input, (separator_span, _)) = with_span(tag(&b"::"[..]))(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, (marker_span, _)) = with_span(tag(marker))(input)?;
    Ok((
        input,
        ImportSuffixSpans {
            span: span_from_to(start, input),
            separator_span,
            marker_span,
        },
    ))
}

/// Parse the typed suffix shared by import and expose targets.
pub(crate) fn import_shape(input: Input<'_>) -> IResult<Input<'_>, ImportShape> {
    let (input, _) = ws_and_comments(input)?;
    if input.fragment().starts_with(b"::") {
        let (marker_input, _) = tag(&b"::"[..]).parse(input)?;
        let (marker_input, _) = ws_and_comments(marker_input)?;
        if marker_input.fragment().starts_with(b"**") {
            let (input, recursive_suffix) = import_suffix(input, b"**")?;
            let (input, members) = opt(filter_package_members).parse(input)?;
            return Ok((
                input,
                match members {
                    Some(members) => ImportShape::Filter {
                        recursive_suffix: Some(recursive_suffix),
                        members,
                    },
                    None => ImportShape::Membership {
                        recursive_suffix: Some(recursive_suffix),
                    },
                },
            ));
        }
        if marker_input.fragment().starts_with(b"*") {
            let suffix_start = input;
            let (input, wildcard_suffix) = import_suffix(input, b"*")?;
            let (input, recursive_suffix) = opt(preceded(ws_and_comments, |input| {
                import_suffix(input, b"**")
            }))
            .parse(input)?;
            let combined_recursive_suffix_span = recursive_suffix
                .as_ref()
                .map(|_| span_from_to(suffix_start, input));
            return Ok((
                input,
                ImportShape::Namespace {
                    wildcard_suffix,
                    recursive_suffix,
                    combined_recursive_suffix_span,
                },
            ));
        }
        return Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    if input.fragment().starts_with(b"[") {
        let (input, members) = filter_package_members(input)?;
        return Ok((
            input,
            ImportShape::Filter {
                recursive_suffix: None,
                members,
            },
        ));
    }
    Ok((
        input,
        ImportShape::Membership {
            recursive_suffix: None,
        },
    ))
}

/// Import: visibility 'import' isImportAll? (QualifiedName | QualifiedName '::' '*') RelationshipBody
pub(crate) fn import_(input: Input<'_>) -> IResult<Input<'_>, Node<Import>> {
    // Speculated at many member starts; refuse unless `import` follows the optional
    // visibility keyword, before entering an arena transaction. This lookahead also routes
    // a bare import to its owning grammar, which rejects the missing prefix.
    {
        let (cursor, _) = crate::parser::lex::ws_and_comments(input)?;
        let (cursor, _) = opt(crate::parser::lex::visibility_indicator).parse(cursor)?;
        let (cursor, _) = crate::parser::lex::ws_and_comments(cursor)?;
        if !crate::parser::lex::starts_with_keyword(cursor.fragment(), b"import") {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
    }
    reference_transaction(input, import_inner)
}

/// The Import production requires VisibilityIndicator in both pinned grammars.
/// Recovery owns this token diagnosis; it never publishes a successful import with a default.
pub(crate) fn missing_visibility_error(input: Input<'_>) -> Option<crate::ast::ParseErrorNode> {
    let (keyword_start, _) = ws_and_comments(input).ok()?;
    if !crate::parser::lex::starts_with_keyword(keyword_start.fragment(), b"import") {
        return None;
    }
    let (end, _) = tag::<_, _, nom::error::Error<_>>(&b"import"[..])
        .parse(keyword_start)
        .ok()?;
    Some(crate::ast::ParseErrorNode {
        diagnostic_span: Some(span_from_to(keyword_start, end)),
        message: "an import requires an explicit visibility indicator".to_owned(),
        code: "missing_import_visibility".to_owned(),
        expected: Some("'public', 'private', or 'protected' before 'import'".to_owned()),
        found: Some("import".to_owned()),
        suggestion: Some("Specify the import visibility explicitly.".to_owned()),
        category: Some(crate::error::DiagnosticCategory::ParseError),
    })
}

/// `import A as B;` is not SysML: `Import = VisibilityIndicator 'import' 'all'? ImportDeclaration
/// RelationshipBody` (SysML BNF 8.2.2.4) has no renaming clause. Recovery owns this diagnosis so
/// the reader learns that an alias membership (`alias B for A;`) is the construct that names an
/// element differently, rather than seeing a generic unexpected-token error. The span is the `as`
/// clause itself.
///
/// The import target is read with the import grammar's own productions, so comments, quoted
/// names and filter members (whose expressions may contain `as` casts) are handled exactly as the
/// real import parser handles them; only a complete target followed by `as` is diagnosed.
pub(crate) fn alias_clause_error(input: Input<'_>) -> Option<crate::ast::ParseErrorNode> {
    crate::parser::span::reference_probe(input, alias_clause_probe)
}

fn alias_clause_probe(input: Input<'_>) -> Option<crate::ast::ParseErrorNode> {
    let (cursor, _) = ws_and_comments(input).ok()?;
    let (cursor, _) = opt(crate::parser::lex::visibility_indicator)
        .parse(cursor)
        .ok()?;
    let (cursor, _) = ws_and_comments(cursor).ok()?;
    if !crate::parser::lex::starts_with_keyword(cursor.fragment(), b"import") {
        return None;
    }
    let (cursor, _) = tag::<_, _, nom::error::Error<_>>(&b"import"[..])
        .parse(cursor)
        .ok()?;
    let (cursor, _) = ws_and_comments(cursor).ok()?;
    let (target_start, _) = opt(terminated(tag(&b"all"[..]), ws1)).parse(cursor).ok()?;
    let (after_reference, _) = qualified_reference(target_start).ok()?;
    let (after_shape, shape) = import_shape(after_reference).ok()?;
    let target_end = import_target_end(&shape, after_reference.location_offset());
    let (as_start, _) = ws_and_comments(after_shape).ok()?;
    if !crate::parser::lex::starts_with_keyword(as_start.fragment(), b"as") {
        return None;
    }
    let (after_as, _) = tag::<_, _, nom::error::Error<_>>(&b"as"[..])
        .parse(as_start)
        .ok()?;
    let (after_as, _) = ws_and_comments(after_as).ok()?;
    let renamed = crate::parser::lex::name(after_as).ok();
    let clause_end = renamed.map_or(after_as, |(rest, _)| rest);
    let target = {
        let start = target_start.location_offset();
        let relative = target_end.checked_sub(start)?;
        String::from_utf8_lossy(target_start.fragment().get(..relative)?).into_owned()
    };
    let suggestion = match renamed {
        Some((_, name)) => {
            let name = String::from_utf8_lossy(crate::parser::lex::name_bytes(after_as, name));
            format!(
                "Remove 'as {name}' and declare an alias membership instead: 'alias {name} for {target};'                  (with the import's visibility to re-export the name)."
            )
        }
        None => "Remove the 'as' clause, and declare an alias membership to name the element differently:                  'alias <name> for <target>;'."
            .to_owned(),
    };
    // The clause ends at its last authored token, not at the trivia the name parser may skip.
    let mut clause_span = span_from_to(as_start, clause_end);
    if renamed.is_none() {
        clause_span.len = b"as".len();
    }
    Some(crate::ast::ParseErrorNode {
        diagnostic_span: Some(clause_span),
        message:
            "a SysML import has no 'as' clause; an alias membership names an element differently"
                .to_owned(),
        code: crate::parser::diagnostic_catalog::IMPORT_ALIAS_CLAUSE.to_owned(),
        expected: Some("';' or a relationship body after the import target".to_owned()),
        found: Some("as".to_owned()),
        suggestion: Some(suggestion),
        category: Some(crate::error::DiagnosticCategory::ParseError),
    })
}

/// The offset just past an import target's last authored token: its final suffix or filter member
/// if it has one, otherwise the qualified reference itself.
pub(crate) fn import_target_end(shape: &crate::ast::ImportShape, reference_end: usize) -> usize {
    use crate::ast::ImportShape;
    let span_end = |span: &crate::ast::Span| span.offset.saturating_add(span.len);
    match shape {
        ImportShape::Membership { recursive_suffix } => recursive_suffix
            .as_ref()
            .map_or(reference_end, |suffix| span_end(&suffix.span)),
        ImportShape::Namespace {
            wildcard_suffix,
            recursive_suffix,
            combined_recursive_suffix_span,
        } => combined_recursive_suffix_span
            .as_ref()
            .map(span_end)
            .or_else(|| {
                recursive_suffix
                    .as_ref()
                    .map(|suffix| span_end(&suffix.span))
            })
            .unwrap_or_else(|| span_end(&wildcard_suffix.span)),
        ImportShape::Filter {
            recursive_suffix,
            members,
        } => members
            .last()
            .map(|member| span_end(&member.span))
            .or_else(|| {
                recursive_suffix
                    .as_ref()
                    .map(|suffix| span_end(&suffix.span))
            })
            .unwrap_or(reference_end),
    }
}

fn import_inner(input: Input<'_>) -> IResult<Input<'_>, Node<Import>> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, (visibility_span, visibility)) = crate::parser::lex::visibility_indicator(input)?;
    let (input, _) = ws_and_comments(input)?;
    let (input, _) = tag(&b"import"[..]).parse(input)?;
    let (input, _) = ws_and_comments(input)?;
    let target_start = input;
    let (input, all) = opt(terminated(with_span(tag(&b"all"[..])), ws1)).parse(input)?;
    let all_span = all.map(|(span, _)| span);
    let (after_reference, reference) = qualified_reference(input)?;
    let (input, shape) = import_shape(after_reference)?;
    // The target ends at its last authored token, not wherever shape parsing stopped: looking for
    // an absent `::*` consumes the trivia before a braced body, and a span that swallowed it would
    // no longer describe the target it claims.
    let mut target_span = span_from_to(target_start, input);
    target_span.len = import_target_end(&shape, after_reference.location_offset())
        .saturating_sub(target_span.offset);
    let (input, body_elements) = relationship_body(input)?;
    Ok((
        input,
        node_from_to(
            start,
            input,
            Import {
                membership: Membership::new(
                    MembershipKind::Import,
                    Some(visibility),
                    visibility_span,
                ),
                target: ImportTarget {
                    span: target_span,
                    all_span,
                    reference,
                    shape,
                },
                body_elements,
            },
        ),
    ))
}

#[cfg(test)]
mod membership_tests {
    use super::*;
    use crate::parser::span::ParseContext;

    fn input(text: &str) -> Input<'_> {
        let context = Box::leak(Box::new(ParseContext::new()));
        context.input(text.as_bytes())
    }

    // --- parser work item 4b (continuation): `Import.visibility` replaced by `Import.membership` ---

    #[test]
    fn import_visibility_prefix_is_captured_on_membership() {
        let (_, node) = import_(input("public import SI::kg;")).expect("import");
        assert_eq!(
            node.value.membership.visibility,
            Some(crate::ast::Visibility::Public)
        );
        assert_eq!(
            node.value.membership.kind,
            crate::ast::MembershipKind::Import
        );
    }

    #[test]
    fn import_without_visibility_is_rejected_without_publishing_references() {
        let context = ParseContext::new();
        assert!(import_(context.input(b"import SI::kg;")).is_err());
        assert!(context.finish().is_empty());
    }

    #[test]
    fn import_suffixes_are_typed_and_reference_excludes_wildcards() {
        let context = ParseContext::new();
        let source = crate::ast::SourceStorage::from(
            "private import $::A::B:: /* wildcard */ * /* recurse */ :: /* marker */ **;",
        );
        let (rest, node) = import_(context.input(source.as_str().as_bytes())).expect("import");
        assert!(rest.fragment().is_empty());
        let ImportShape::Namespace {
            wildcard_suffix,
            recursive_suffix: Some(recursive_suffix),
            combined_recursive_suffix_span: Some(combined_span),
        } = &node.value.target.shape
        else {
            panic!("expected recursive namespace import");
        };
        assert_eq!(
            source.slice(&wildcard_suffix.span),
            Some(":: /* wildcard */ *")
        );
        assert_eq!(source.slice(&wildcard_suffix.separator_span), Some("::"));
        assert_eq!(source.slice(&wildcard_suffix.marker_span), Some("*"));
        assert_eq!(
            source.slice(&recursive_suffix.span),
            Some(":: /* marker */ **")
        );
        assert_eq!(source.slice(&recursive_suffix.separator_span), Some("::"));
        assert_eq!(source.slice(&recursive_suffix.marker_span), Some("**"));
        assert_eq!(
            source.slice(combined_span),
            Some(":: /* wildcard */ * /* recurse */ :: /* marker */ **")
        );
        let arena = context.finish();
        let view = arena
            .get(&source, node.value.target.reference)
            .expect("reference");
        assert_eq!(view.authored_text(), "$::A::B");
        assert!(view.metadata.is_absolute);
    }

    #[test]
    fn filter_import_retains_typed_expression_members() {
        let context = ParseContext::new();
        let source = crate::ast::SourceStorage::from(
            "private import A /* before */ [ /* left */ x /* right */ ] [y];",
        );
        let (rest, node) = import_(context.input(source.as_str().as_bytes())).expect("import");
        assert!(rest.fragment().is_empty());
        match node.value.target.shape {
            ImportShape::Filter {
                recursive_suffix,
                members,
            } => {
                assert!(recursive_suffix.is_none());
                assert_eq!(members.len(), 2);
                assert_eq!(
                    source.slice(&members[0].span),
                    Some("[ /* left */ x /* right */ ]")
                );
                assert_eq!(source.slice(&members[0].value.open_bracket_span), Some("["));
                assert_eq!(
                    source.slice(&members[0].value.close_bracket_span),
                    Some("]")
                );
                assert_eq!(source.slice(&members[1].span), Some("[y]"));
            }
            other => panic!("expected filter shape, got {other:?}"),
        }
    }
}
