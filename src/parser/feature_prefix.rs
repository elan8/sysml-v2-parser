//! The shared KerML `FeaturePrefix` parser.
//!
//! ```text
//! FeaturePrefix =
//!     ( EndFeaturePrefix ( ownedRelationship += OwnedCrossFeatureMember )?
//!     | BasicFeaturePrefix
//!     )
//!     ( ownedRelationship += PrefixMetadataMember )*        -- KerML BNF 584
//!
//! EndFeaturePrefix  : Feature = ( isConstant ?= 'const' )? isEnd ?= 'end'          -- 573
//! BasicFeaturePrefix : Feature = FeatureDirection? 'derived'? 'abstract'?
//!                                ('composite' | 'portion')? ('var' | 'const')?     -- 577
//! ```
//!
//! One parser for `Feature` (562), `Step` (863), `Expression` (895) and `BooleanExpression` (908),
//! which spell this prefix identically; see `planning/kerml-feature-prefix-matrix.md`.
//!
//! The two alternatives are tried in the grammar's own order, and that order is load-bearing:
//! `const` is the last slot of `BasicFeaturePrefix` *and* the first of `EndFeaturePrefix`, so a
//! leading `const` is only an end prefix when `end` actually follows it. [`feature_prefix_head`]
//! therefore probes `('const')? 'end'` first and rewinds to the basic alternative when the `end`
//! never arrives, rather than committing on the shared first keyword.
//!
//! Because the alternatives are exclusive, a direction beside `end` is refused here rather than
//! normalized: `in end feature x;` leaves `end` unconsumed, the owning production fails, and the
//! member reaches recovery. `tests/snapshots/spec42/end_prefix_recovery.md` pins that conclusion.
//!
//! Slots are consumed strictly in the production's order, so an out-of-order spelling such as
//! `derived in feature x;` leaves the second keyword unconsumed and the owning production fails.
//! Nothing here ever consumes input it does not record.

use crate::ast::{
    BasicFeaturePrefix, EndFeaturePrefix, FeaturePortionKind, FeaturePrefix, FeaturePrefixHead,
    FeatureVariability, Node,
};
use crate::parser::lex::{starts_with_any_keyword, ws_and_comments};
use crate::parser::occurrence_prefix::{optional_alternative, optional_direction, slot_keyword};
use crate::parser::Input;
use nom::IResult;

/// `BasicFeaturePrefix : Feature = FeatureDirection? 'derived'? 'abstract'?
/// ('composite' | 'portion')? ('var' | 'const')?` (KerML BNF 577).
///
/// Every slot is optional, so this never fails; it consumes nothing when the next token is already
/// the kind keyword. Takes trivia-free input, like every slot parser in
/// [`crate::parser::occurrence_prefix`].
pub(crate) fn basic_feature_prefix(input: Input<'_>) -> (Input<'_>, BasicFeaturePrefix) {
    let (input, direction) = optional_direction(input);
    let (input, derived_span) = match slot_keyword(input, b"derived") {
        Some((rest, span)) => (rest, Some(span)),
        None => (input, None),
    };
    let (input, abstract_span) = match slot_keyword(input, b"abstract") {
        Some((rest, span)) => (rest, Some(span)),
        None => (input, None),
    };
    let (input, portioning) = optional_alternative(
        input,
        [
            (&b"composite"[..], FeaturePortionKind::Composite),
            (&b"portion"[..], FeaturePortionKind::Portion),
        ],
    );
    let (input, variability) = optional_alternative(
        input,
        [
            (&b"var"[..], FeatureVariability::Var),
            (&b"const"[..], FeatureVariability::Const),
        ],
    );
    (
        input,
        BasicFeaturePrefix {
            direction,
            derived_span,
            abstract_span,
            portioning,
            variability,
        },
    )
}

/// `EndFeaturePrefix : Feature = ( isConstant ?= 'const' )? isEnd ?= 'end'` (KerML BNF 573).
///
/// Returns `None` -- consuming nothing -- when `end` does not follow, so a bare `const` falls
/// through to the basic alternative rather than being stranded. Takes trivia-free input.
fn end_feature_prefix(input: Input<'_>) -> (Input<'_>, Option<EndFeaturePrefix>) {
    let (after_const, constant_span) = match slot_keyword(input, b"const") {
        Some((rest, span)) => (rest, Some(span)),
        None => (input, None),
    };
    match slot_keyword(after_const, b"end") {
        Some((rest, end_span)) => (
            rest,
            Some(EndFeaturePrefix {
                constant_span,
                end_span,
            }),
        ),
        // No `end`, so this was never `EndFeaturePrefix`. Rewind past any `const` we probed.
        None => (input, None),
    }
}

/// The `EndFeaturePrefix | BasicFeaturePrefix` choice (KerML BNF 584).
///
/// The `cross` slot is left empty here; the owning production fills it, because
/// `OwnedCrossFeatureMember` is a full `FeatureDeclaration` and needs the declaration parsers the
/// caller already has in scope.
///
/// Takes trivia-free input.
pub(crate) fn feature_prefix_head(input: Input<'_>) -> (Input<'_>, FeaturePrefixHead) {
    if let (rest, Some(prefix)) = end_feature_prefix(input) {
        return (
            rest,
            FeaturePrefixHead::End {
                prefix,
                cross: None,
            },
        );
    }
    let (rest, basic) = basic_feature_prefix(input);
    (rest, FeaturePrefixHead::Basic(basic))
}

/// `FeaturePrefix` (KerML BNF 584): the choice, then `( PrefixMetadataMember )*`.
///
/// The metadata run trails the prefix in the grammar's order, so `derived #Tag feature z;` is
/// legal -- and was refused outright before this seam, because the feature parser stopped at the
/// `#`. Each keyword is `'#' OwnedFeatureTyping`, parsed by the crate's one owning parser for that
/// pair, so the annotation is a qualified reference in the document arena rather than copied text.
///
/// A `#`-led member is *not* claimed here: the owning scopes dispatch their metadata arm before
/// their feature arm, so `#Tag feature z;` still parses as a prefix member followed by the
/// feature. That dispatch contract is deliberate and shared with every other member kind (see
/// `calc_def_body_element`'s `#` arm); folding it into this prefix is a metadata-seam change
/// rather than a `FeaturePrefix` one, and is recorded as deferred in
/// `planning/kerml-feature-prefix-matrix.md` §11.
///
/// Takes trivia-free input.
pub(crate) fn feature_prefix(input: Input<'_>) -> IResult<Input<'_>, FeaturePrefix> {
    let (after_head, head) = feature_prefix_head(input);
    // The run belongs to this prefix only when a feature-kind keyword follows it. `#` also heads
    // `PrefixMetadataMember` as a member of its own, in front of any declaration at all, and
    // `#service port def Authorisation { … }` is one: consuming the sigil here without that check
    // would leave the feature parser holding an authored prefix and let it mis-claim the `port
    // def` behind it. Transactional, so a rewind takes the annotation's arena entries with it.
    let (after_metadata, metadata_keywords) = if after_head.fragment().starts_with(b"#") {
        match crate::parser::span::reference_transaction(after_head, metadata_keyword_run) {
            Ok((rest, keywords)) => (rest, keywords),
            Err(_) => (after_head, Vec::new()),
        }
    } else {
        (after_head, Vec::new())
    };
    Ok((
        after_metadata,
        FeaturePrefix {
            head,
            metadata_keywords,
        },
    ))
}

/// `( ownedRelationship += PrefixMetadataMember )*` followed by a feature-kind keyword.
///
/// Fails -- so [`feature_prefix`] rewinds -- when the run is empty or is not this feature's.
fn metadata_keyword_run(
    input: Input<'_>,
) -> IResult<Input<'_>, Vec<Node<crate::ast::UsageExtensionKeyword>>> {
    let start = input;
    let mut input = input;
    let mut keywords = Vec::new();
    while input.fragment().starts_with(b"#") {
        let (rest, keyword) = crate::parser::occurrence_prefix::usage_extension_keyword(input)?;
        keywords.push(keyword);
        let (rest, _) = ws_and_comments(rest)?;
        input = rest;
    }
    if keywords.is_empty()
        || !starts_with_any_keyword(
            input.fragment(),
            &[&b"feature"[..], b"step", b"expr", b"bool"],
        )
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            start,
            nom::error::ErrorKind::Tag,
        )));
    }
    Ok((input, keywords))
}
