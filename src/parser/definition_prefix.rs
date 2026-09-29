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
//!
//! **`reject_header_keyword`** exists because some definitions (`interface`, ...) are still
//! `def`-optional, which makes their definition grammar a superset of the sibling usage grammar
//! ([#34](https://github.com/elan8/sysml-v2-parser/issues/34)). `connection` no longer is: it
//! requires `def`, because `ConnectionUsage` now owns the whole `OccurrenceUsagePrefix` and
//! `UsageDeclaration` (see `planning/connection-usage-prefix-matrix.md`), which retired the
//! GH-20 `reject_plain_typed_header_without_def` guard the def-optional grammar needed.

use crate::ast::{
    DefinitionPrefix, Identification, Node, TypingRelationship, UsageExtensionKeyword, Visibility,
};
use crate::parser::definition_header::parse_definition_header_after_ident;
use crate::parser::lex::{contains_keyword, identification, ws1, ws_and_comments};
use crate::parser::Input;
use nom::bytes::complete::tag;
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

/// Whether the production spells `DefinitionExtensionKeyword*` after its basic prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionKeywordMode {
    None,
    /// `OccurrenceDefinitionPrefix`'s `DefinitionExtensionKeyword*` (SysML BNF 541--546): a run
    /// of `'#' QualifiedName` prefix metadata after `individual`, before the kind keyword.
    Occurrence,
}

/// Which prefix slot a definition production actually spells, as the pinned grammar writes it.
///
/// Three of the pin's definition productions differ here, so this is a grammar fact rather than a
/// caller preference, and modelling it as one enum keeps the impossible combination
/// ("`variation` but not `abstract`") unrepresentable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicPrefixSlot {
    /// The production spells no prefix at all, so both keywords are refused.
    ///
    /// `EnumerationDefinition = DefinitionExtensionKeyword* 'enum' 'def' DefinitionDeclaration
    /// EnumerationBody` (SysML BNF 518; Pilot `SysML.xtext` 767) is the only definition in the pin
    /// that reaches neither `DefinitionPrefix` nor `OccurrenceDefinitionPrefix`.
    None,
    /// The production inlines `isAbstract ?= 'abstract'` on its own, so `variation` is refused.
    ///
    /// `MetadataDefinition = ( isAbstract ?= 'abstract' )? DefinitionExtensionKeyword* 'metadata'
    /// 'def' Definition` (SysML BNF 1652; Pilot `SysML.xtext` 121) writes the flag out rather than
    /// referencing `BasicDefinitionPrefix`, so there is no second alternative to represent.
    AbstractOnly,
    /// The production reaches `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation ?=
    /// 'variation'` (SysML BNF 219; Pilot `SysML.xtext` 490), through `DefinitionPrefix` (BNF 225),
    /// `OccurrenceDefinitionPrefix` (BNF 541), `IndividualDefinition` (BNF 551) or
    /// `ExtendedDefinition` (BNF 1696). Every other definition kind in the pin is in this group.
    Basic,
}

#[derive(Debug, Clone, Copy)]
pub struct DefinitionPrefixOptions {
    pub keyword: &'static [u8],
    /// Second keyword after the first (`use` then `case`, etc.).
    pub second_keyword: Option<&'static [u8]>,
    pub def: DefKeywordMode,
    /// The prefix slot this definition's own production spells. See [`BasicPrefixSlot`].
    pub basic_prefix_slot: BasicPrefixSlot,
    /// When set, accept an optional `individual` prefix after `abstract` (BNF
    /// `OccurrenceDefinitionPrefix`'s `(isIndividual ?= 'individual' ...)?`), captured into
    /// [`DefinitionPrefixResult::is_individual`]. Every definition kind whose BNF production
    /// derives from `OccurrenceDefinitionPrefix` legally accepts this (occurrence, item, action,
    /// case/analysis/verification, part, connection, interface, allocation, flow, state, calc,
    /// constraint, requirement, concern, ...) -- opt-in per caller rather than defaulted on, since
    /// several definition kinds (`attribute def`, `enum def`, `metadata def`, ...) do not derive
    /// from it and must keep rejecting a leading `individual` as usual.
    pub individual_allowed: bool,
    pub visibility: VisibilityPrefix,
    pub extension_keywords: ExtensionKeywordMode,
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
            basic_prefix_slot: BasicPrefixSlot::Basic,
            individual_allowed: false,
            visibility: VisibilityPrefix::None,
            extension_keywords: ExtensionKeywordMode::None,
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

    /// This production inlines `isAbstract ?= 'abstract'` instead of reaching
    /// `BasicDefinitionPrefix`. See [`BasicPrefixSlot::AbstractOnly`].
    pub const fn abstract_only_prefix(mut self) -> Self {
        self.basic_prefix_slot = BasicPrefixSlot::AbstractOnly;
        self
    }

    /// This production spells no prefix slot at all. See [`BasicPrefixSlot::None`].
    pub const fn no_basic_prefix(mut self) -> Self {
        self.basic_prefix_slot = BasicPrefixSlot::None;
        self
    }

    /// See [`DefinitionPrefixOptions::individual_allowed`].
    pub const fn individual_allowed(mut self) -> Self {
        self.individual_allowed = true;
        self
    }

    /// Capture (not discard) an optional `private`/`protected`/`public` prefix into
    /// `DefinitionPrefixResult::visibility`/`visibility_span`. See
    /// [`VisibilityPrefix::Captured`].
    pub const fn with_captured_visibility(mut self) -> Self {
        self.visibility = VisibilityPrefix::Captured;
        self
    }

    /// Parse `OccurrenceDefinitionPrefix`'s `DefinitionExtensionKeyword*` run.
    pub const fn with_extension_keywords(mut self) -> Self {
        self.extension_keywords = ExtensionKeywordMode::Occurrence;
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
    /// `DefinitionExtensionKeyword*`, in authored order; empty unless
    /// [`DefinitionPrefixOptions::with_extension_keywords`] was set.
    pub extension_keywords: Vec<Node<UsageExtensionKeyword>>,
    /// `BasicDefinitionPrefix` -- one slot, two alternatives -- with the authored keyword's
    /// exact span. `None` is the ordinary "no prefix authored" state.
    ///
    /// Which alternatives can appear here is [`DefinitionPrefixOptions::basic_prefix_slot`], i.e.
    /// what this definition's own production spells, not what its node happens to be able to hold.
    pub basic_prefix: Option<Node<DefinitionPrefix>>,
    /// `individual` prefix, captured only when
    /// [`DefinitionPrefixOptions::individual_allowed`] was set; `false` otherwise.
    pub is_individual: bool,
    /// `private`/`protected`/`public` prefix, captured only when
    /// [`DefinitionPrefixOptions::with_captured_visibility`] was set; `None` for every other
    /// caller's `VisibilityPrefix` mode (matching this crate's "record only what was explicitly
    /// requested" convention -- see `Membership::visibility`'s own doc comment).
    pub visibility: Option<Visibility>,
    /// Span of the visibility prefix keyword when [`Captured`](VisibilityPrefix::Captured) and
    /// present; a zero-width span at the prefix's position otherwise.
    pub visibility_span: crate::ast::Span,
}

/// `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation ?= 'variation'` (SysML BNF
/// 219), as one slot carrying the authored keyword's exact span.
///
/// Both alternatives occupy the same position, so a node holding one `Option<Node<_>>` cannot
/// represent `abstract variation`, which the grammar forbids.
pub(crate) fn parse_basic_definition_prefix(
    input: Input<'_>,
    slot: BasicPrefixSlot,
) -> IResult<Input<'_>, Option<Node<DefinitionPrefix>>> {
    const ALTERNATIVES: [(&[u8], DefinitionPrefix); 2] = [
        (b"abstract", DefinitionPrefix::Abstract),
        (b"variation", DefinitionPrefix::Variation),
    ];
    for (keyword, value) in ALTERNATIVES {
        let spelled = match (slot, value) {
            (BasicPrefixSlot::None, _) => false,
            (BasicPrefixSlot::AbstractOnly, DefinitionPrefix::Variation) => false,
            (BasicPrefixSlot::AbstractOnly, DefinitionPrefix::Abstract)
            | (BasicPrefixSlot::Basic, _) => true,
        };
        if !spelled {
            continue;
        }
        if let Ok((rest, (span, _))) =
            crate::parser::span::with_span(tag::<_, _, nom::error::Error<Input<'_>>>(keyword))
                .parse(input)
        {
            let (rest, _) = ws1(rest)?;
            return Ok((rest, Some(Node::new(span, value))));
        }
    }
    Ok((input, None))
}

/// Parse from start of input through identification and optional subclassification header.
pub(crate) fn parse_definition_prefix(
    input: Input<'_>,
    options: DefinitionPrefixOptions,
) -> IResult<Input<'_>, DefinitionPrefixResult> {
    let (input, _) = ws_and_comments(input)?;

    let (input, visibility, visibility_span) = match options.visibility {
        VisibilityPrefix::None => (input, None, crate::parser::span_from_to(input, input)),
        VisibilityPrefix::Captured => {
            let (input, (span, vis)) = crate::parser::lex::visibility_prefix(input)?;
            (input, vis, span)
        }
    };

    let (input, basic_prefix) = parse_basic_definition_prefix(input, options.basic_prefix_slot)?;
    // BNF `OccurrenceDefinitionPrefix`: `individual` follows `abstract`, before the keyword.
    let (input, is_individual) = if options.individual_allowed {
        let (input, found) = opt(preceded(tag(&b"individual"[..]), ws1)).parse(input)?;
        (input, found.is_some())
    } else {
        (input, false)
    };

    // `DefinitionExtensionKeyword*` follows `individual` and precedes the kind keyword.
    let (input, extension_keywords) = match options.extension_keywords {
        ExtensionKeywordMode::None => (input, Vec::new()),
        ExtensionKeywordMode::Occurrence => {
            let mut input = input;
            let mut keywords = Vec::new();
            while input.fragment().starts_with(b"#") {
                let (rest, keyword) =
                    crate::parser::occurrence_prefix::usage_extension_keyword(input)?;
                keywords.push(keyword);
                let (rest, _) = ws_and_comments(rest)?;
                input = rest;
            }
            (input, keywords)
        }
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
        DefKeywordMode::Optional => {
            let (input, _) = opt(preceded(tag(&b"def"[..]), ws1)).parse(input)?;
            input
        }
    };

    let (input, identification) = identification(input)?;
    let header_start = input;
    let (input, header) = parse_definition_header_after_ident(input)?;
    if let Some(keyword) = options.reject_header_keyword {
        if header.raw_header.is_some_and(|raw| {
            let relative = raw.offset - header_start.location_offset();
            contains_keyword(
                &header_start.fragment()[relative..relative + raw.len],
                keyword,
            )
        }) {
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
            extension_keywords,
            basic_prefix,
            is_individual,
            visibility,
            visibility_span,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::QualifiedReferenceId;

    fn span_input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    fn target_texts(source: Input<'_>, targets: &[QualifiedReferenceId]) -> Vec<String> {
        targets
            .iter()
            .map(|id| crate::parser::usage::reference_text(source, *id).expect("reference text"))
            .collect()
    }

    #[test]
    fn prefix_parses_item_def_with_specializes() {
        let input = span_input("abstract item def Foo :> Base { }");
        let (rest, prefix) =
            parse_definition_prefix(input, DefinitionPrefixOptions::new(b"item")).expect("prefix");
        assert_eq!(
            prefix.basic_prefix.as_ref().map(|node| node.value),
            Some(DefinitionPrefix::Abstract)
        );
        assert_eq!(
            prefix
                .identification
                .name
                .map(|n| crate::parser::lex::name_bytes(input, n)),
            Some(&b"Foo"[..])
        );
        assert_eq!(
            prefix
                .specializes
                .as_ref()
                .map(|n| target_texts(input, &n.value.target)),
            Some(vec!["Base".to_string()])
        );
        assert!(rest.fragment().trim_ascii_start().starts_with(b"{"));
    }

    #[test]
    fn prefix_parses_typed_library_header() {
        let input = span_input("connection connections : Connection[0..*] :> linkObjects, parts {");
        let (rest, prefix) =
            parse_definition_prefix(input, DefinitionPrefixOptions::new(b"connection"))
                .expect("prefix");
        assert_eq!(
            prefix
                .identification
                .name
                .map(|n| crate::parser::lex::name_bytes(input, n)),
            Some(&b"connections"[..])
        );
        assert_eq!(
            prefix
                .specializes
                .as_ref()
                .map(|n| target_texts(input, &n.value.target)),
            Some(vec!["linkObjects".to_string(), "parts".to_string()])
        );
        assert!(rest.fragment().starts_with(b"{"));
    }

    #[test]
    fn prefix_parses_definition_extension_keywords_after_the_basic_prefix() {
        let text = "abstract #derivation connection def Conn :> Base ;";
        let input = span_input(text);
        let (rest, prefix) = parse_definition_prefix(
            input,
            DefinitionPrefixOptions::new(b"connection")
                .def_required()
                .with_extension_keywords(),
        )
        .expect("prefix");
        assert_eq!(prefix.extension_keywords.len(), 1);
        let keyword = &prefix.extension_keywords[0];
        assert_eq!(
            &text[keyword.span.offset..keyword.span.offset + keyword.span.len],
            "#derivation"
        );
        assert_eq!(
            prefix.basic_prefix.as_ref().map(|node| node.value),
            Some(DefinitionPrefix::Abstract)
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
        assert_eq!(
            prefix.basic_prefix.as_ref().map(|node| node.value),
            Some(DefinitionPrefix::Abstract)
        );
        assert_eq!(
            prefix
                .identification
                .name
                .map(|n| crate::parser::lex::name_bytes(input, n)),
            Some(&b"X"[..])
        );
        assert_eq!(prefix.visibility, Some(crate::ast::Visibility::Private));
    }

    #[test]
    fn prefix_required_def_individual() {
        let input = span_input("individual def X :> Y;");
        let (_, prefix) = parse_definition_prefix(
            input,
            DefinitionPrefixOptions::new(b"individual").def_required(),
        )
        .expect("prefix");
        assert_eq!(prefix.basic_prefix.as_ref().map(|node| node.value), None);
        assert_eq!(
            prefix
                .specializes
                .as_ref()
                .map(|n| target_texts(input, &n.value.target)),
            Some(vec!["Y".to_string()])
        );
    }
}
