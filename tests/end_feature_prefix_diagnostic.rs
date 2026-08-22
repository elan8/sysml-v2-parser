//! Upstream gaps 59 and 67: `end` spelled with a slot only `BasicFeaturePrefix` owns.
//!
//! ```text
//! FeaturePrefix      = ( EndFeaturePrefix … | BasicFeaturePrefix ) …        -- KerML BNF 584
//! EndFeaturePrefix   = ( isConstant ?= 'const' )? isEnd ?= 'end'            -- 573
//! BasicFeaturePrefix = FeatureDirection? 'derived'? 'abstract'?
//!                      ( 'composite' | 'portion' )? ( 'var' | 'const' )?    -- 577
//! ```
//!
//! The two are alternatives of one choice, so `in end feature f;` and `derived end feature f;`
//! have no derivation and the parser is right to refuse them -- `src/ast/feature_prefix.rs`
//! records why the combination is unrepresentable rather than merely unparsed. What was wrong was
//! the *report*: `composite`/`portion`/`var` reached the generic scope recovery as "`composite` is
//! not a SysML keyword", which is false, and a direction reached it as an anonymous "unexpected
//! token", which names nothing a consumer can act on. Both now name the authored keyword and the
//! rule it breaks, so the violation is observable instead of guessable.

use sysml_v2_parser::parse_with_diagnostics;
use sysml_v2_parser::parser::diagnostic_catalog::END_FEATURE_INVALID_PREFIX;

fn sole_diagnostic_code(input: &str) -> Option<String> {
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.len() <= 1,
        "expected at most one diagnostic for `{input}`, got {:?}",
        result.errors
    );
    result.errors.first().and_then(|e| e.code.clone())
}

/// Every `BasicFeaturePrefix` slot keyword except `const`, which both alternatives admit.
const BASIC_ONLY: &[&str] = &[
    "in",
    "out",
    "inout",
    "derived",
    "abstract",
    "composite",
    "portion",
    "var",
];

#[test]
fn every_basic_only_modifier_beside_end_is_reported_precisely() {
    for keyword in BASIC_ONLY {
        for source in [
            format!("classifier C {{ {keyword} end feature f : T; }}"),
            format!("classifier C {{ end {keyword} feature f : T; }}"),
        ] {
            assert_eq!(
                sole_diagnostic_code(&source).as_deref(),
                Some(END_FEATURE_INVALID_PREFIX),
                "`{source}` must report the prefix, not the scope"
            );
        }
    }
}

#[test]
fn the_diagnostic_names_the_offending_keyword_and_the_rule() {
    let result = parse_with_diagnostics("classifier C { composite end feature f : T; }");
    let error = &result.errors[0];
    let message = error.message.as_str();
    assert!(
        message.contains("`composite`") && message.contains("573"),
        "the message must name the keyword and its production: {message}"
    );
    assert!(
        !message.contains("not a SysML keyword"),
        "`composite` is a SysML keyword; the old message was simply false: {message}"
    );
    let suggestion = error.suggestion.as_deref().unwrap_or_default();
    assert!(
        suggestion.contains("Remove `composite`"),
        "the suggestion must be actionable: {suggestion}"
    );
}

#[test]
fn the_two_legal_spellings_still_parse_clean() {
    // `const` is the one modifier `EndFeaturePrefix` admits (BNF 573), and `end` alone is the
    // production's only required token.
    for source in [
        "classifier C { const end feature f : T; }",
        "classifier C { end feature f : T; }",
        "assoc A { end feature f : T; }",
    ] {
        assert_eq!(sole_diagnostic_code(source), None, "`{source}` is legal");
    }
}

#[test]
fn a_basic_prefix_without_end_is_untouched() {
    for source in [
        "classifier C { derived feature f : T; }",
        "classifier C { in feature f : T; }",
        "classifier C { composite feature f : T; }",
    ] {
        assert_eq!(sole_diagnostic_code(source), None, "`{source}` is legal");
    }
}

#[test]
fn end_as_part_of_a_name_does_not_trip_the_classification() {
    // The scan walks the leading keyword run only, so the first word that is not a prefix slot
    // ends it -- a feature *named* `end2`, or a `var connector` whose body has ends, is not a
    // malformed end prefix.
    for source in [
        "classifier C { feature end2 : T; }",
        "package P { part def A { part x; } }",
    ] {
        assert_eq!(sole_diagnostic_code(source), None, "`{source}` is legal");
    }
}
