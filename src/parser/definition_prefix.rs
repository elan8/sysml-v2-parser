//! Shared definition prelude: modifiers, keyword, `def`, identification, header.
//!
//! `DefinitionPrefixOptions::def_required()` is the reusable form of the PAR-001
//! disambiguation fix (see `attribute::attribute_def`'s `disambiguate_from_usage` parameter for
//! the original, still-bespoke instance): any body parser that dispatches a `*_def` parser
//! alongside a matching `*_usage` parser for the same keyword MUST set `.def_required()` so a
//! `def`-less declaration is left for the usage parser rather than silently misclassified as a
//! definition. `action`, `allocation`, `case`/`analysis`/`verification`/`use case`, `enum`,
//! `flow`, `individual`, `interface` (via `interface_def_required`), `item` (via
//! `item_def_required`), `metadata`, `occurrence`, `requirement`, `state`, and `view`/
//! `viewpoint`/`rendering` all already do this. `connection`, `constraint`/`calc`, and `port` are
//! deliberately left `Optional` at their package-level (non-`_required`) entry points — see the
//! "do not add `.def_required()` here" comments in `connection.rs`/`constraint.rs`/`port.rs` —
//! because the real Systems Library uses bare, `def`-less declarations for those keywords at
//! namespace level (with `abstract`/multiplicity/`nonunique`/subsets modifiers a competing usage
//! parser doesn't fully cover) with no dedicated package-level usage parser able to fall back to
//! instead; adding `.def_required()` there breaks the full `SYSML_V2_RELEASE_DIR` validation gate
//! (this was tried and reverted for `port`/`constraint`/`calc`, see CHANGELOG 0.33.0, and tried
//! and reverted again for `connection` specifically during the PAR-006b audit -- see
//! `connection.rs::connection_def`'s doc comment for that investigation). Any new body-enum
//! wiring (PAR-002) that adds a `def`/usage pair sharing a keyword must use this module rather
//! than hand-rolling another bespoke guard, but a package-level `_def`/`_usage` pair sharing a
//! keyword is not automatically the PAR-001 bug class if the `_def` parser's grammar is already a
//! strict superset of the `_usage` parser's -- confirm which shapes only the usage parser accepts
//! before assuming a guard is missing.

use crate::ast::{Identification, Node, TypingRelationship, Visibility};
use crate::parser::definition_header::parse_definition_header_after_ident;
use crate::parser::lex::{contains_keyword, identification, ws1, ws_and_comments};
use crate::parser::Input;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::opt;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefKeywordMode {
    Required,
    Optional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityPrefix {
    None,
    /// Optional `private` / `protected` / `public` before the keyword, captured (not discarded)
    /// via [`crate::parser::lex::visibility_prefix`] into `DefinitionPrefixResult::visibility`/
    /// `visibility_span`, for callers building a [`crate::ast::Membership`] (parser work item 4b
    /// continuation, `PortDef`/`ItemDef`/`ConnectionDef`, and -- as of the Item 4b final sweep --
    /// every remaining `*Def` in this crate, including `constraint`/`calc`, which previously used
    /// a since-removed `OptionalPrivate` mode that matched-and-discarded a bare `private` only;
    /// `Captured` is a strict superset that also accepts `protected`/`public`).
    Captured,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationMode {
    None,
    /// Leading `#identifier` (connection definitions).
    HashIdentifier,
}

#[derive(Debug, Clone, Copy)]
pub struct DefinitionPrefixOptions {
    pub keyword: &'static [u8],
    /// Second keyword after the first (`use` then `case`, etc.).
    pub second_keyword: Option<&'static [u8]>,
    pub def: DefKeywordMode,
    pub abstract_allowed: bool,
    pub visibility: VisibilityPrefix,
    pub annotation: AnnotationMode,
    /// When set, fail this definition parse if the plain `: Type` header scan swallows this
    /// keyword as part of its discarded trailing text. See
    /// [`DefinitionPrefixOptions::reject_header_keyword`].
    pub reject_header_keyword: Option<&'static [u8]>,
}

impl DefinitionPrefixOptions {
    pub const fn new(keyword: &'static [u8]) -> Self {
        Self {
            keyword,
            second_keyword: None,
            def: DefKeywordMode::Optional,
            abstract_allowed: true,
            visibility: VisibilityPrefix::None,
            annotation: AnnotationMode::None,
            reject_header_keyword: None,
        }
    }

    pub const fn with_second_keyword(mut self, second: &'static [u8]) -> Self {
        self.second_keyword = Some(second);
        self
    }

    pub const fn def_required(mut self) -> Self {
        self.def = DefKeywordMode::Required;
        self
    }

    pub const fn no_abstract(mut self) -> Self {
        self.abstract_allowed = false;
        self
    }

    /// Capture (not discard) an optional `private`/`protected`/`public` prefix into
    /// `DefinitionPrefixResult::visibility`/`visibility_span`. See
    /// [`VisibilityPrefix::Captured`].
    pub const fn with_captured_visibility(mut self) -> Self {
        self.visibility = VisibilityPrefix::Captured;
        self
    }

    pub const fn with_hash_annotation(mut self) -> Self {
        self.annotation = AnnotationMode::HashIdentifier;
        self
    }

    /// Reject (fail) this definition parse when the plain `: Type` header scan swallows
    /// `keyword` as part of its discarded trailing text -- e.g. a `connect ... to ...` clause on
    /// a `connection`/`interface` declaration line, which otherwise makes the `def` parser
    /// silently accept and discard usage-only syntax instead of leaving it for the sibling usage
    /// parser to claim. Do not use this for keywords that can legitimately co-occur with a plain
    /// typing/subclassification header (it would wrongly reject those).
    pub const fn reject_header_keyword(mut self, keyword: &'static [u8]) -> Self {
        self.reject_header_keyword = Some(keyword);
        self
    }
}

#[derive(Debug, Clone)]
pub struct DefinitionPrefixResult {
    pub identification: Identification,
    pub specializes: Option<Node<TypingRelationship>>,
    pub annotation: Option<String>,
    pub is_abstract: bool,
    /// `private`/`protected`/`public` prefix, captured only when
    /// [`DefinitionPrefixOptions::with_captured_visibility`] was set; `None` for every other
    /// caller's `VisibilityPrefix` mode (matching this crate's "record only what was explicitly
    /// requested" convention -- see `Membership::visibility`'s own doc comment).
    pub visibility: Option<Visibility>,
    /// Span of the visibility prefix keyword when [`Captured`](VisibilityPrefix::Captured) and
    /// present; a zero-width span at the prefix's position otherwise.
    pub visibility_span: crate::ast::Span,
}

/// Parse from start of input through identification and optional subclassification header.
pub(crate) fn parse_definition_prefix(
    input: Input<'_>,
    options: DefinitionPrefixOptions,
) -> IResult<Input<'_>, DefinitionPrefixResult> {
    let (input, _) = ws_and_comments(input)?;

    let (input, annotation) = match options.annotation {
        AnnotationMode::None => (input, None),
        AnnotationMode::HashIdentifier => {
            let (input, raw) = opt(preceded(
                tag(&b"#"[..]),
                take_while1(|c: u8| c.is_ascii_alphanumeric() || c == b'_'),
            ))
            .parse(input)?;
            let annotation = raw.map(|span| String::from_utf8_lossy(span.fragment()).to_string());
            let (input, _) = ws_and_comments(input)?;
            (input, annotation)
        }
    };

    let (input, visibility, visibility_span) = match options.visibility {
        VisibilityPrefix::None => (input, None, crate::parser::span_from_to(input, input)),
        VisibilityPrefix::Captured => {
            let (input, (span, vis)) = crate::parser::lex::visibility_prefix(input)?;
            (input, vis, span)
        }
    };

    let (input, is_abstract) = if options.abstract_allowed {
        let (input, found) = opt(preceded(tag(&b"abstract"[..]), ws1)).parse(input)?;
        (input, found.is_some())
    } else {
        (input, false)
    };

    let (input, _) = tag(options.keyword).parse(input)?;
    let (input, _) = ws1(input)?;
    let input = if let Some(second) = options.second_keyword {
        let (input, _) = tag(second).parse(input)?;
        let (input, _) = ws1(input)?;
        input
    } else {
        input
    };

    let input = match options.def {
        DefKeywordMode::Required => {
            let (input, _) = tag(&b"def"[..]).parse(input)?;
            let (input, _) = ws1(input)?;
            input
        }
        DefKeywordMode::Optional => opt(preceded(tag(&b"def"[..]), ws1)).parse(input)?.0,
    };

    let (input, identification) = identification(input)?;
    let (input, header) = parse_definition_header_after_ident(input)?;
    if let Some(keyword) = options.reject_header_keyword {
        if header
            .raw_header
            .as_deref()
            .is_some_and(|raw| contains_keyword(raw.as_bytes(), keyword))
        {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }
    }
    let specializes = header.specializes;

    Ok((
        input,
        DefinitionPrefixResult {
            identification,
            specializes,
            annotation,
            is_abstract,
            visibility,
            visibility_span,
        },
    ))
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
    fn prefix_parses_item_def_with_specializes() {
        let input = span_input("abstract item def Foo :> Base { }");
        let (rest, prefix) =
            parse_definition_prefix(input, DefinitionPrefixOptions::new(b"item")).expect("prefix");
        assert!(prefix.is_abstract);
        assert_eq!(prefix.identification.name.as_deref(), Some("Foo"));
        assert_eq!(
            prefix
                .specializes
                .as_ref()
                .map(|n| targets_display_string(&n.value.target)),
            Some("Base".to_string())
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b"{"));
    }

    #[test]
    fn prefix_parses_typed_library_header() {
        let input = span_input("connection connections : Connection[0..*] :> linkObjects, parts {");
        let (rest, prefix) = parse_definition_prefix(
            input,
            DefinitionPrefixOptions::new(b"connection").no_abstract(),
        )
        .expect("prefix");
        assert_eq!(prefix.identification.name.as_deref(), Some("connections"));
        assert_eq!(
            prefix
                .specializes
                .as_ref()
                .map(|n| targets_display_string(&n.value.target)),
            Some("linkObjects, parts".to_string())
        );
        assert!(rest.fragment().starts_with(b"{"));
    }

    #[test]
    fn prefix_parses_hash_annotation_connection() {
        let input = span_input("#MyConn abstract connection conn :> Base ;");
        let (rest, prefix) = parse_definition_prefix(
            input,
            DefinitionPrefixOptions::new(b"connection").with_hash_annotation(),
        )
        .expect("prefix");
        assert_eq!(prefix.annotation.as_deref(), Some("MyConn"));
        assert!(prefix.is_abstract);
        assert_eq!(
            prefix
                .specializes
                .as_ref()
                .map(|n| targets_display_string(&n.value.target)),
            Some("Base".to_string())
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b";"));
    }

    #[test]
    fn prefix_private_before_abstract_constraint() {
        let input = span_input("private abstract constraint def X ;");
        let (_, prefix) = parse_definition_prefix(
            input,
            DefinitionPrefixOptions::new(b"constraint")
                .with_captured_visibility()
                .def_required(),
        )
        .expect("prefix");
        assert!(prefix.is_abstract);
        assert_eq!(prefix.identification.name.as_deref(), Some("X"));
        assert_eq!(prefix.visibility, Some(crate::ast::Visibility::Private));
    }

    #[test]
    fn prefix_required_def_individual() {
        let input = span_input("individual def X :> Y;");
        let (_, prefix) = parse_definition_prefix(
            input,
            DefinitionPrefixOptions::new(b"individual")
                .def_required()
                .no_abstract(),
        )
        .expect("prefix");
        assert!(!prefix.is_abstract);
        assert_eq!(
            prefix
                .specializes
                .as_ref()
                .map(|n| targets_display_string(&n.value.target)),
            Some("Y".to_string())
        );
    }
}
