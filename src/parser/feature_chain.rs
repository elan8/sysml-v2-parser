//! Parser for the standalone [`crate::ast::FeatureChain`] type (PAR-004 item 6).
//!
//! See `src/ast/feature_chain.rs` for how [`crate::ast::FeatureChain`] is used by
//! `Expression::FeatureChainRef` since PAR-005 item 3. This particular combinator (`NAME ('.'
//! NAME)*`, no `::` support) is still not called from `path_expression`
//! (`src/parser/expr.rs`) -- `path_expression` needs its first segment to allow a `::`-qualified
//! name (e.g. `Foo::bar.baz`), which this parser doesn't support, so `path_expression` builds its
//! `FeatureChain` inline instead. Kept as a standalone, reusable building block for any future
//! caller that only needs a pure dot-chain with no leading qualified name.

use super::lex::{name, ws_and_comments};
use super::span::span_from_to;
use crate::ast::FeatureChain;
use nom::character::complete::char as nchar;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::Parser;

use super::span::Input;

/// FeatureChain: NAME ( '.' NAME )*. Dot-separated, as opposed to the `::`-separated
/// [`super::lex::qualified_name`].
///
/// Not yet called from any production parser (see module doc comment) — allowed dead code until
/// PAR-005 or a relationship parser adopts it.
#[allow(dead_code)]
pub(crate) fn feature_chain(input: Input<'_>) -> nom::IResult<Input<'_>, FeatureChain> {
    let start = input;
    let (input, _) = ws_and_comments(input)?;
    let (input, first) = name(input)?;
    let (input, rest) = many0(preceded(
        preceded(ws_and_comments, nchar('.')),
        preceded(ws_and_comments, name),
    ))
    .parse(input)?;
    let mut segments = Vec::with_capacity(rest.len() + 1);
    segments.push(first);
    segments.extend(rest);
    let span = span_from_to(start, input);
    Ok((input, FeatureChain { segments, span }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom_locate::LocatedSpan;

    fn parse(src: &str) -> FeatureChain {
        let input = LocatedSpan::new(src.as_bytes());
        let (rest, chain) = feature_chain(input).expect("parse failed");
        assert!(
            rest.fragment().is_empty(),
            "leftover input: {:?}",
            String::from_utf8_lossy(rest.fragment())
        );
        chain
    }

    #[test]
    fn single_segment() {
        let chain = parse("engine");
        assert_eq!(chain.segments, vec!["engine".to_string()]);
        assert!(chain.is_single());
    }

    #[test]
    fn multi_segment_chain() {
        let chain = parse("engine.fuelCmdPort.flowRate");
        assert_eq!(
            chain.segments,
            vec![
                "engine".to_string(),
                "fuelCmdPort".to_string(),
                "flowRate".to_string()
            ]
        );
        assert!(!chain.is_single());
    }

    #[test]
    fn chain_span_covers_whole_chain() {
        let chain = parse("a.b.c");
        assert_eq!(chain.span.offset, 0);
        assert_eq!(chain.span.len, 5);
    }
}
