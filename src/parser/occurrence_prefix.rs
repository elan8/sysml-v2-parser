//! The shared `OccurrenceUsagePrefix` parser.
//!
//! ```text
//! OccurrenceUsagePrefix : OccurrenceUsage =
//!     BasicUsagePrefix ( isIndividual ?= 'individual' )?
//!     ( portionKind = PortionKind )? UsageExtensionKeyword*     -- SysML BNF 564
//! BasicUsagePrefix : Usage = RefPrefix ( isReference ?= 'ref' )?
//! RefPrefix : Usage = FeatureDirection? 'derived'? ('abstract'|'variation')? 'constant'?
//! ```
//!
//! One parser for every family that spells this production, so a legal prefix cannot be a parse
//! gap in whichever scope has not hand-rolled that slot yet. Before this module the prefix was
//! respelled five times in `occurrence_body.rs` alone, each accepting a different subset in a
//! different order, and `derived`, `variation` and `UsageExtensionKeyword*` were accepted by none
//! of them. `planning/occurrence-usage-prefix-matrix.md` records which families use it.
//!
//! Slots are consumed strictly in the production's order, so an out-of-order spelling such as
//! `ref abstract occurrence o;` leaves the second keyword unconsumed and the owning production
//! fails, rather than being quietly normalized into the legal order. Nothing here ever consumes
//! input it does not record.

use crate::ast::{
    BasicUsagePrefix, DefinitionPrefix, EndUsagePrefix, InOut, Node, OccurrencePortionKind,
    OccurrenceUsagePrefix, OccurrenceUsagePrefixHead, OwnedCrossUsage, RefPrefix, Span,
    UsageExtensionKeyword,
};
use crate::parser::lex::{starts_with_keyword, ws_and_comments};
use crate::parser::span::with_span;
use crate::parser::{node_from_to, Input};
use nom::bytes::complete::tag;
use nom::IResult;
use nom::Parser;

/// Consume one keyword plus the trivia around it, returning the keyword's exact token span.
///
/// [`starts_with_keyword`] supplies the token boundary -- `in` cannot match the first two bytes of
/// a declaration named `input`, nor `ref` the first three of `reference` -- so the separator
/// itself is [`ws_and_comments`] rather than `ws1`. That distinction is load-bearing: `ws1`
/// consumes whitespace but stops at `/`, so a comment between two of a production's keywords would
/// be left sitting in front of the next one and `in /* why */ derived occurrence o;` would fail to
/// parse even though the same comment is trivia everywhere else.
///
/// A comment *abutting* the keyword (`occurrence/* why */ o`) is still rejected, and not by
/// anything this module owns: [`starts_with_keyword`] requires the byte after an
/// identifier-shaped keyword to be whitespace or one of `{ : ; [`, so every keyword in this
/// crate behaves the same way. Widening that predicate is a lexical change affecting every
/// starter table, dispatch guard and recovery sync in the crate, and is recorded as its own seam
/// in `planning/satisfy-requirement-usage-matrix.md` §7.
pub(crate) fn keyword_token<'a>(
    input: Input<'a>,
    keyword: &'static [u8],
) -> IResult<Input<'a>, Span> {
    let (after_ws, _) = ws_and_comments(input)?;
    if !starts_with_keyword(after_ws.fragment(), keyword) {
        return Err(nom::Err::Error(nom::error::Error::new(
            after_ws,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (rest, (span, _)) = with_span(tag(keyword)).parse(after_ws)?;
    let (rest, _) = ws_and_comments(rest)?;
    Ok((rest, span))
}

/// [`keyword_token`] for a keyword the production makes optional. Consumes nothing when the
/// keyword is absent, including the whitespace in front of it.
pub(crate) fn optional_keyword_token<'a>(
    input: Input<'a>,
    keyword: &'static [u8],
) -> IResult<Input<'a>, Option<Span>> {
    match keyword_token(input, keyword) {
        Ok((rest, span)) => Ok((rest, Some(span))),
        Err(_) => Ok((input, None)),
    }
}

/// [`keyword_token`] for input whose leading trivia the caller has already consumed.
///
/// The prefix has thirteen optional slots and almost every member of every body scope has none of
/// them, so the probe that *fails* is the hot one. Re-running the trivia scan and rebuilding a
/// located span for each failed probe is what made walking the prefix measurable on the corpus
/// benchmark; this reduces a failure to one keyword comparison. Every success re-establishes the
/// invariant by consuming the trivia after its own token, so the slots can be probed in sequence.
pub(crate) fn slot_keyword<'a>(
    input: Input<'a>,
    keyword: &'static [u8],
) -> Option<(Input<'a>, Span)> {
    if !starts_with_keyword(input.fragment(), keyword) {
        return None;
    }
    let (rest, (span, _)) = with_span(tag::<_, _, nom::error::Error<Input<'a>>>(keyword))
        .parse(input)
        .ok()?;
    let (rest, _) = ws_and_comments(rest).ok()?;
    Some((rest, span))
}

/// The first of two alternatives in one slot, as a `Node` carrying the authored keyword's span.
///
/// Takes trivia-free input; see [`slot_keyword`].
pub(crate) fn optional_alternative<'a, T: Copy>(
    input: Input<'a>,
    alternatives: [(&'static [u8], T); 2],
) -> (Input<'a>, Option<Node<T>>) {
    for (keyword, value) in alternatives {
        if let Some((rest, span)) = slot_keyword(input, keyword) {
            return (rest, Some(Node::new(span, value)));
        }
    }
    (input, None)
}

/// `RefPrefix : Usage = FeatureDirection? 'derived'? ('abstract'|'variation')? 'constant'?`.
///
/// Every slot is optional, so this never fails; it consumes nothing when the next token is
/// already the kind keyword. `inout` is tried before `in` for readability only --
/// [`starts_with_keyword`] would reject `in` against `inout x` anyway, because `o` is not a token
/// boundary.
/// Takes trivia-free input; see [`slot_keyword`].
pub(crate) fn ref_prefix(input: Input<'_>) -> (Input<'_>, RefPrefix) {
    let (input, direction) = optional_direction(input);
    let (input, derived_span) = match slot_keyword(input, b"derived") {
        Some((rest, span)) => (rest, Some(span)),
        None => (input, None),
    };
    let (input, variance) = optional_alternative(
        input,
        [
            (&b"abstract"[..], DefinitionPrefix::Abstract),
            (&b"variation"[..], DefinitionPrefix::Variation),
        ],
    );
    let (input, constant_span) = match slot_keyword(input, b"constant") {
        Some((rest, span)) => (rest, Some(span)),
        None => (input, None),
    };
    (
        input,
        RefPrefix {
            direction,
            derived_span,
            variance,
            constant_span,
        },
    )
}

/// `direction = FeatureDirection` where `FeatureDirection = 'in' | 'out' | 'inout'`.
pub(crate) fn optional_direction(input: Input<'_>) -> (Input<'_>, Option<Node<InOut>>) {
    for (keyword, value) in [
        (&b"inout"[..], InOut::InOut),
        (&b"in"[..], InOut::In),
        (&b"out"[..], InOut::Out),
    ] {
        if let Some((rest, span)) = slot_keyword(input, keyword) {
            return (rest, Some(Node::new(span, value)));
        }
    }
    (input, None)
}

/// `BasicUsagePrefix : Usage = RefPrefix ( isReference ?= 'ref' )?`.
///
/// Takes trivia-free input; see [`slot_keyword`].
fn basic_usage_prefix(input: Input<'_>) -> (Input<'_>, BasicUsagePrefix) {
    let (input, ref_prefix) = ref_prefix(input);
    let (input, reference_span) = match slot_keyword(input, b"ref") {
        Some((rest, span)) => (rest, Some(span)),
        None => (input, None),
    };
    (
        input,
        BasicUsagePrefix {
            ref_prefix,
            reference_span,
        },
    )
}

/// One `UsageExtensionKeyword = PrefixMetadataMember = '#' PrefixMetadataUsage`.
///
/// The `'#' OwnedFeatureTyping` head is the crate's one owning parser for the sigil-and-reference
/// pair, so an extension keyword's name is a qualified reference in the document arena with its
/// own scope, segments and typed separators -- absolute (`#$::Lib::Tag`), qualified
/// (`#ISQ::mass`), dotted or quoted (`#'safety critical'`) exactly as authored.
pub(crate) fn usage_extension_keyword(
    input: Input<'_>,
) -> IResult<Input<'_>, Node<UsageExtensionKeyword>> {
    let (after_ws, _) = ws_and_comments(input)?;
    let (rest, (hash_span, annotation)) =
        crate::parser::metadata_annotation::metadata_keyword_head(after_ws)?;
    Ok((
        rest,
        node_from_to(
            after_ws,
            rest,
            UsageExtensionKeyword {
                hash_span,
                annotation,
            },
        ),
    ))
}

/// Whether a member at this position opens with a prefix token a sibling production would
/// otherwise claim first.
///
/// Two of `OccurrenceUsagePrefix`'s FIRST tokens also head a different production in the same
/// scopes: `#` heads `PrefixMetadataMember` as a standalone member, and `ref` heads
/// `ReferenceUsage`. Both of those parsers run before the migrated families in several scopes and
/// neither knows about the kind keyword that follows, so left alone they claim a prefixed
/// occurrence, item or satisfy usage and silently drop the production it really was --
/// `ref individual snapshot satisfy R;` became a `ReferenceUsage` named `individual`, with no
/// diagnostic. The scopes concerned give the migrated families first refusal when this returns
/// true; every attempt is transactional, so a member that really is one of the other two falls
/// through unchanged.
pub(crate) fn starts_contended_prefix(input: Input<'_>) -> bool {
    // The prefix slots that head no competing production, skipped so that `derived ref item x;`
    // is recognized as contended even though its first token is not. `ref` and `#` are the only
    // two heads a sibling production shares, so a run that reaches neither is not contended and
    // the scope's ordinary dispatch order -- which puts each `*_def` parser ahead of its `*_usage`
    // sibling -- is left exactly as it was.
    const UNCONTENDED_SLOTS: &[&[u8]] = &[
        b"inout",
        b"in",
        b"out",
        b"derived",
        b"abstract",
        b"variation",
        b"constant",
        b"individual",
        b"snapshot",
        b"timeslice",
    ];
    let mut cursor = input;
    loop {
        let Ok((after_ws, _)) = ws_and_comments(cursor) else {
            return false;
        };
        let fragment = after_ws.fragment();
        if fragment.starts_with(b"#")
            || starts_with_keyword(fragment, b"ref")
            || starts_with_keyword(fragment, b"end")
        {
            return true;
        }
        let Some(keyword) = UNCONTENDED_SLOTS
            .iter()
            .find(|keyword| starts_with_keyword(fragment, keyword))
        else {
            return false;
        };
        let Ok((rest, _)) = tag::<_, _, nom::error::Error<Input<'_>>>(*keyword).parse(after_ws)
        else {
            return false;
        };
        cursor = rest;
    }
}

/// Whether the member can begin an `OccurrenceUsage`: `then`, a visibility keyword, a prefix
/// slot keyword, a `#` extension, `event`, or `occurrence` leads. Exact word matches; a `true`
/// only admits the real parser.
pub(crate) fn could_start_occurrence_usage(input: Input<'_>) -> bool {
    const STARTERS: &[&[u8]] = &[
        b"then",
        b"public",
        b"private",
        b"protected",
        b"event",
        b"occurrence",
        b"end",
        b"inout",
        b"in",
        b"out",
        b"derived",
        b"abstract",
        b"variation",
        b"constant",
        b"ref",
        b"individual",
        b"snapshot",
        b"timeslice",
    ];
    let Ok((cursor, _)) = ws_and_comments(input) else {
        return false;
    };
    let fragment = cursor.fragment();
    fragment.starts_with(b"#")
        || STARTERS
            .iter()
            .any(|keyword| starts_with_keyword(fragment, keyword))
}

/// Whether `keyword` can introduce this member's kind after its optional prefixes.
///
/// Six usage families spell `MemberPrefix OccurrenceUsagePrefix <kind-keyword> ...`, and each is
/// speculated at member starts it does not own: the whole prefix was parsed (allocating arena
/// entries for `#tag` extensions) only for the kind keyword to refuse. This scans past trivia,
/// `then`, visibility, every prefix slot keyword, and `#`-extension tags *without allocating*,
/// then answers whether `keyword` leads. It is deliberately permissive -- a `true` only admits
/// the real parser, which still decides -- but a `false` must be exact, so the skip set is a
/// superset of every caller's authored prefix vocabulary.
pub(crate) fn kind_keyword_follows(input: Input<'_>, keyword: &[u8]) -> bool {
    scan_prefix_for(input, |fragment| starts_with_keyword(fragment, keyword))
}

/// Whether a `#` extension tag follows the member's optional prefixes; same contract as
/// [`kind_keyword_follows`], with the `#` itself as the target instead of a keyword.
pub(crate) fn hash_extension_follows(input: Input<'_>) -> bool {
    scan_prefix_for(input, |fragment| fragment.starts_with(b"#"))
}

fn scan_prefix_for(input: Input<'_>, is_target: impl Fn(&[u8]) -> bool) -> bool {
    const SKIPPED: &[&[u8]] = &[
        b"then",
        b"assert",
        b"not",
        b"member",
        b"public",
        b"private",
        b"protected",
        b"inout",
        b"in",
        b"out",
        b"end",
        b"derived",
        b"abstract",
        b"variation",
        b"constant",
        b"ref",
        b"individual",
        b"snapshot",
        b"timeslice",
    ];
    let Ok((mut cursor, _)) = ws_and_comments(input) else {
        return false;
    };
    loop {
        let fragment = cursor.fragment();
        if is_target(fragment) {
            return true;
        }
        if fragment.starts_with(b"#") {
            // Skip the `#` and its qualified-name tag: name bytes, separators, and quoted
            // segments (whose closing quote may be escaped).
            let mut index = 1usize;
            while index < fragment.len() {
                match fragment[index] {
                    b'\'' => {
                        index += 1;
                        while index < fragment.len() {
                            if fragment[index] == b'\\' && index + 1 < fragment.len() {
                                index += 2;
                            } else if fragment[index] == b'\'' {
                                index += 1;
                                break;
                            } else {
                                index += 1;
                            }
                        }
                    }
                    byte if byte.is_ascii_alphanumeric() => index += 1,
                    b'_' | b':' | b'.' | b'$' => index += 1,
                    _ => break,
                }
            }
            let Ok((rest, _)) =
                nom::bytes::complete::take::<_, _, nom::error::Error<Input<'_>>>(index)
                    .parse(cursor)
            else {
                return false;
            };
            let Ok((rest, _)) = ws_and_comments(rest) else {
                return false;
            };
            cursor = rest;
            continue;
        }
        if let Some(skipped) = SKIPPED
            .iter()
            .find(|candidate| starts_with_keyword(fragment, candidate))
        {
            let Ok((rest, _)) =
                nom::bytes::complete::take::<_, _, nom::error::Error<Input<'_>>>(skipped.len())
                    .parse(cursor)
            else {
                return false;
            };
            let Ok((rest, _)) = ws_and_comments(rest) else {
                return false;
            };
            cursor = rest;
            continue;
        }
        // A multiplicity (`end [1] port …`) or a declared name (`end touches [0..*] item …`)
        // may also stand before the kind keyword. Skipping them stays safe -- a `true` only
        // admits the real parser -- while refusing here would reject authored syntax.
        let skip_len = match fragment.first() {
            Some(b'[') => {
                let mut depth = 0usize;
                let mut index = 0usize;
                loop {
                    match fragment.get(index) {
                        Some(b'[') => depth += 1,
                        Some(b']') => {
                            depth -= 1;
                            if depth == 0 {
                                break index + 1;
                            }
                        }
                        Some(_) => {}
                        None => return false,
                    }
                    index += 1;
                }
            }
            Some(b'\'') => {
                let mut index = 1usize;
                loop {
                    match fragment.get(index) {
                        Some(b'\\') => index += 2,
                        Some(b'\'') => break index + 1,
                        Some(_) => index += 1,
                        None => return false,
                    }
                }
            }
            Some(byte) if byte.is_ascii_alphabetic() || *byte == b'_' => fragment
                .iter()
                .take_while(|byte| byte.is_ascii_alphanumeric() || **byte == b'_')
                .count(),
            _ => return false,
        };
        let Ok((rest, _)) =
            nom::bytes::complete::take::<_, _, nom::error::Error<Input<'_>>>(skip_len)
                .parse(cursor)
        else {
            return false;
        };
        let Ok((rest, _)) = ws_and_comments(rest) else {
            return false;
        };
        cursor = rest;
    }
}

/// Whether the next unquoted identifier token spells a reserved SysML keyword.
///
/// A reserved keyword can never be an unquoted declaration name, so a production whose next slot
/// is a declaration label uses this to refuse a sibling family's kind keyword instead of reading
/// it as a name. A *quoted* name is never a keyword, so `'individual'` still declares one.
pub(crate) fn next_word_is_reserved(input: Input<'_>) -> bool {
    let Ok((after_ws, _)) = ws_and_comments(input) else {
        return false;
    };
    let fragment = after_ws.fragment();
    let word_len = fragment
        .iter()
        .take_while(|byte| byte.is_ascii_alphanumeric() || **byte == b'_')
        .count();
    word_len > 0 && crate::parser::lex::is_reserved_keyword(&fragment[..word_len])
}

/// Whether the first byte can open any slot of the production.
///
/// Every one of the thirteen slots is optional, so the overwhelmingly common case is a member
/// that authored none of them -- and that case has to pay for eleven failed keyword comparisons
/// before the caller can require its kind keyword. Six families now spell this prefix, so that
/// bill is paid six times per member of a body scope with no starter table.
///
/// The first byte of every slot is one of `# i o d a v c r s t`, so one table lookup answers
/// "could this be a prefix at all" and returns the unauthored prefix without probing a slot.
/// The table is derived from the keyword list rather than written out, so a new slot cannot be
/// added without extending it.
const fn prefix_slot_first_bytes() -> [bool; 256] {
    let mut table = [false; 256];
    table[b'#' as usize] = true;
    let keywords: [&[u8]; 12] = [
        b"end",
        b"in",
        b"out",
        b"inout",
        b"derived",
        b"abstract",
        b"variation",
        b"constant",
        b"ref",
        b"individual",
        b"snapshot",
        b"timeslice",
    ];
    let mut index = 0;
    while index < keywords.len() {
        table[keywords[index][0] as usize] = true;
        index += 1;
    }
    table
}

const PREFIX_SLOT_FIRST_BYTES: [bool; 256] = prefix_slot_first_bytes();

fn starts_a_prefix_slot(fragment: &[u8]) -> bool {
    match fragment.first() {
        Some(byte) => PREFIX_SLOT_FIRST_BYTES[*byte as usize],
        None => false,
    }
}

/// `OccurrenceUsagePrefix`, the whole production.
///
/// Never fails: every slot is optional, and an all-absent result is the ordinary "no prefix
/// authored" state. Callers decide whether what follows is a head they own; a prefix followed by
/// nothing they recognize must make the *owning* production fail, so that the member reaches
/// recovery as one node rather than being reinterpreted as an unprefixed usage.
///
/// Speculative reference allocation: a `UsageExtensionKeyword` allocates an arena entry for its
/// qualified name. Callers therefore run this inside
/// [`reference_transaction`](crate::parser::span::reference_transaction), which rolls the arena
/// back when the owning production is refused.
/// `OwnedCrossFeature : ReferenceUsage = BasicUsagePrefix UsageDeclaration` (SysML BNF 293),
/// the declaration `OwnedCrossFeatureMember` hangs between `end` and the owning usage's kind
/// keyword.
///
/// Fails -- consuming nothing -- when nothing stands between `end` and a reserved keyword, so a
/// bare `end port p : P;` keeps an empty cross slot, and when what follows the declaration is
/// not a reserved keyword, so the keyword-less `end p1 : P;` is left to `DefaultReferenceUsage`
/// untouched. Takes trivia-free input and returns trivia-free input.
fn owned_cross_usage(input: Input<'_>) -> IResult<Input<'_>, Node<OwnedCrossUsage>> {
    let start = input;
    let (input, prefix) = basic_usage_prefix(input);
    // A reserved keyword can never be an unquoted declaration name, so a kind keyword right
    // after the prefix means the cross feature has no identification.
    let (input, declaration) = if next_word_is_reserved(input) {
        crate::parser::usage::usage_declaration_without_identification(input)?
    } else {
        crate::parser::usage::usage_declaration(input)?
    };
    // A cross feature is a declaration: a bare prefix (`end derived part p : T;`) is a prefix
    // keyword in a position no production spells, which the owning scope reports as
    // `end_feature_invalid_prefix`, not a nameless cross feature.
    if !usage_declaration_is_authored(&declaration.value) {
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (rest, _) = ws_and_comments(input)?;
    if !next_word_is_reserved(rest) {
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    Ok((
        rest,
        node_from_to(
            start,
            input,
            OwnedCrossUsage {
                prefix,
                declaration,
            },
        ),
    ))
}

fn usage_declaration_is_authored(declaration: &crate::ast::UsageDeclaration) -> bool {
    declaration.identification.name.is_some()
        || declaration.identification.short_name.is_some()
        || declaration.typing.is_some()
        || declaration.multiplicity.is_some()
        || declaration.multiplicity_modifiers.is_authored()
        || declaration.subsets.is_some()
        || declaration.redefines.is_some()
        || declaration.references.is_some()
        || declaration.crosses.is_some()
        || declaration.intersects.is_some()
}

/// `EndUsagePrefix : Usage = isEnd ?= 'end' ( ownedRelationship += OwnedCrossFeatureMember )?`
/// (SysML BNF 285). Returns `None`, consuming nothing, when `end` does not lead. Takes
/// trivia-free input.
fn end_usage_prefix(input: Input<'_>) -> (Input<'_>, Option<EndUsagePrefix>) {
    let Some((rest, end_span)) = slot_keyword(input, b"end") else {
        return (input, None);
    };
    // The cross feature owns qualified references, so a failed attempt rolls its arena work
    // back rather than leaking entries the accepted syntax never named.
    let (rest, cross) = match crate::parser::span::reference_transaction(rest, owned_cross_usage) {
        Ok((rest, cross)) => (rest, Some(Box::new(cross))),
        Err(_) => (rest, None),
    };
    (rest, Some(EndUsagePrefix { end_span, cross }))
}

pub(crate) fn occurrence_usage_prefix(
    input: Input<'_>,
) -> IResult<Input<'_>, OccurrenceUsagePrefix> {
    // Consume leading trivia once; every slot below then probes trivia-free input and
    // re-establishes that invariant after its own token. See [`slot_keyword`].
    let (input, _) = ws_and_comments(input)?;
    if !starts_a_prefix_slot(input.fragment()) {
        return Ok((input, OccurrenceUsagePrefix::default()));
    }
    // `EndUsagePrefix | BasicUsagePrefix ( 'individual' )? ( PortionKind )?` -- a choice, so the
    // basic slots are never probed once `end` has led (reference `SysML.xtext:836-843`).
    let (input, head) = match end_usage_prefix(input) {
        (rest, Some(end)) => (rest, OccurrenceUsagePrefixHead::End(end)),
        (input, None) => {
            let (input, basic) = basic_usage_prefix(input);
            let (input, individual_span) = match slot_keyword(input, b"individual") {
                Some((rest, span)) => (rest, Some(span)),
                None => (input, None),
            };
            let (input, portion) = optional_alternative(
                input,
                [
                    (&b"snapshot"[..], OccurrencePortionKind::Snapshot),
                    (&b"timeslice"[..], OccurrencePortionKind::Timeslice),
                ],
            );
            (
                input,
                OccurrenceUsagePrefixHead::Basic {
                    basic,
                    individual_span,
                    portion,
                },
            )
        }
    };
    let mut input = input;
    let mut extension_keywords = Vec::new();
    while input.fragment().starts_with(b"#") {
        let Ok((rest, keyword)) = usage_extension_keyword(input) else {
            break;
        };
        extension_keywords.push(keyword);
        let Ok((rest, _)) = ws_and_comments(rest) else {
            break;
        };
        input = rest;
    }
    Ok((
        input,
        OccurrenceUsagePrefix {
            head,
            extension_keywords,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(text: &str) -> Input<'_> {
        crate::parser::span::test_input(text)
    }

    #[test]
    fn an_absent_prefix_consumes_nothing() {
        let source = input("occurrence o1;");
        let (rest, prefix) = occurrence_usage_prefix(source).expect("prefix");
        assert!(!prefix.is_authored());
        assert_eq!(rest.fragment(), source.fragment());
    }

    #[test]
    fn an_end_head_keeps_its_cross_feature_and_excludes_the_basic_slots() {
        let text = "end theCauses [*] occurrence theCause;";
        let (rest, prefix) = occurrence_usage_prefix(input(text)).expect("prefix");
        let end = prefix.end().expect("an `end` head");
        assert_eq!(
            &text[end.end_span.offset..end.end_span.offset + end.end_span.len],
            "end"
        );
        let cross = end.cross.as_deref().expect("a cross feature");
        assert_eq!(
            &text[cross.span.offset..cross.span.offset + cross.span.len],
            "theCauses [*]"
        );
        assert_eq!(
            cross.value.declaration.value.identification.name.map(|n| {
                let span = n.span();
                &text[span.offset..span.offset + span.len]
            }),
            Some("theCauses")
        );
        assert!(cross.value.declaration.value.multiplicity.is_some());
        assert!(prefix.basic().is_none());
        assert!(prefix.individual_span().is_none());
        assert!(rest.fragment().starts_with(b"occurrence"));
    }

    #[test]
    fn an_end_head_with_nothing_before_the_keyword_owns_no_cross_feature() {
        let text = "end port p : P;";
        let (rest, prefix) = occurrence_usage_prefix(input(text)).expect("prefix");
        assert!(prefix.end().expect("an `end` head").cross.is_none());
        assert!(rest.fragment().starts_with(b"port"));
    }

    #[test]
    fn a_keyword_less_end_leaves_its_declaration_for_the_default_reference_usage() {
        let text = "end p1 : P;";
        let (rest, prefix) = occurrence_usage_prefix(input(text)).expect("prefix");
        assert!(prefix.end().expect("an `end` head").cross.is_none());
        assert!(rest.fragment().starts_with(b"p1"));
    }

    #[test]
    fn a_bare_prefix_after_end_is_not_a_cross_feature() {
        let text = "end derived part p : T;";
        let (rest, prefix) = occurrence_usage_prefix(input(text)).expect("prefix");
        assert!(prefix.end().expect("an `end` head").cross.is_none());
        assert!(rest.fragment().starts_with(b"derived"));
    }

    #[test]
    fn every_slot_keeps_its_authored_span() {
        let text = "in derived variation constant ref individual timeslice occurrence o1;";
        let (_, prefix) = occurrence_usage_prefix(input(text)).expect("prefix");
        let basic = prefix.basic().expect("basic head");
        let ref_prefix = &basic.ref_prefix;
        let slice = |span: &Span| &text[span.offset..span.offset + span.len];
        assert_eq!(
            ref_prefix
                .direction
                .as_ref()
                .map(|n| (n.value, slice(&n.span))),
            Some((InOut::In, "in"))
        );
        assert_eq!(ref_prefix.derived_span.as_ref().map(slice), Some("derived"));
        assert_eq!(
            ref_prefix
                .variance
                .as_ref()
                .map(|n| (n.value, slice(&n.span))),
            Some((DefinitionPrefix::Variation, "variation"))
        );
        assert_eq!(
            ref_prefix.constant_span.as_ref().map(slice),
            Some("constant")
        );
        assert_eq!(basic.reference_span.as_ref().map(slice), Some("ref"));
        assert_eq!(prefix.individual_span().map(slice), Some("individual"));
        assert_eq!(
            prefix.portion().map(|n| (n.value, slice(&n.span))),
            Some((OccurrencePortionKind::Timeslice, "timeslice"))
        );
    }

    #[test]
    fn an_out_of_order_slot_is_left_unconsumed() {
        // `ref` precedes `individual`, so the second `ref` here is not a prefix token and the
        // owning production has to fail on it rather than accept a reordered prefix.
        let (rest, prefix) = occurrence_usage_prefix(input("individual ref occurrence o1;"))
            .expect("prefix never fails");
        assert!(prefix.individual_span().is_some());
        assert!(prefix.basic().expect("basic head").reference_span.is_none());
        assert!(rest.fragment().starts_with(b"ref occurrence"));
    }

    #[test]
    fn a_second_exclusive_alternative_is_left_unconsumed() {
        let (rest, prefix) =
            occurrence_usage_prefix(input("snapshot timeslice t;")).expect("prefix");
        assert_eq!(
            prefix.portion().map(|n| n.value),
            Some(OccurrencePortionKind::Snapshot)
        );
        assert!(rest.fragment().starts_with(b"timeslice"));
    }

    #[test]
    fn extension_keywords_keep_authored_order() {
        let (rest, prefix) =
            occurrence_usage_prefix(input("#First #Second occurrence o1;")).expect("prefix");
        assert_eq!(prefix.extension_keywords.len(), 2);
        assert!(rest
            .fragment()
            .trim_ascii_start()
            .starts_with(b"occurrence"));
        let first = prefix.extension_keywords[0].value.annotation;
        let second = prefix.extension_keywords[1].value.annotation;
        assert_ne!(first, second);
    }

    #[test]
    fn a_comment_between_two_slots_is_trivia() {
        let (rest, prefix) =
            occurrence_usage_prefix(input("in /* why */ derived occurrence o1;")).expect("prefix");
        assert!(prefix
            .basic()
            .expect("basic head")
            .ref_prefix
            .direction
            .is_some());
        assert!(prefix
            .basic()
            .expect("basic head")
            .ref_prefix
            .derived_span
            .is_some());
        assert!(rest.fragment().starts_with(b"occurrence"));
    }

    /// Every slot alone, so a missing entry in the first-byte admission table shows up as a
    /// prefix the parser silently stops seeing rather than as a compile error.
    #[test]
    fn each_slot_is_admitted_on_its_own() {
        for (text, authored) in [
            ("in occurrence o;", true),
            ("out occurrence o;", true),
            ("inout occurrence o;", true),
            ("derived occurrence o;", true),
            ("abstract occurrence o;", true),
            ("variation occurrence o;", true),
            ("constant occurrence o;", true),
            ("ref occurrence o;", true),
            ("individual occurrence o;", true),
            ("snapshot occurrence o;", true),
            ("timeslice occurrence o;", true),
            ("#Tag occurrence o;", true),
            ("occurrence o;", false),
        ] {
            let (_, prefix) = occurrence_usage_prefix(input(text)).expect("prefix");
            assert_eq!(prefix.is_authored(), authored, "for {text:?}");
        }
    }

    #[test]
    fn a_prefix_keyword_that_is_only_a_name_prefix_is_not_consumed() {
        let (rest, prefix) = occurrence_usage_prefix(input("individualCount;")).expect("prefix");
        assert!(!prefix.is_authored());
        assert!(rest.fragment().starts_with(b"individualCount"));
    }
}
