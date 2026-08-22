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
    // Modifier-before-`end`: the rule broken is the ordering every production shares, not the
    // exclusive-choice rule, so the message must say that and not cite the wrong production.
    assert!(
        message.contains("`composite`") && message.contains("cannot precede `end`"),
        "the message must name the keyword and the rule it breaks: {message}"
    );
    assert!(
        !message.contains("not a SysML keyword"),
        "`composite` is a SysML keyword; the old message was simply false: {message}"
    );
    let suggestion = error.suggestion.as_deref().unwrap_or_default();
    assert!(
        suggestion.contains("end composite"),
        "the suggestion must point at the ordering that would be legal: {suggestion}"
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

// ---------------------------------------------------------------------------
// `DefaultReferenceUsage = ( isEnd ?= 'end' )? RefPrefix UsageDeclaration …` (SysML BNF 630)
// ---------------------------------------------------------------------------
//
// Verified against the reference implementation, not only the published BNF:
// `org.omg.sysml.xtext/src/org/omg/sysml/xtext/SysML.xtext:630-633`. This is the one production
// that spells `end` beside a `RefPrefix`, and it is what makes the two end-feature constraints
// reachable from textual notation -- the Pilot's own textual validator
// (`KerMLValidator.xtend:669-677`) checks both, which it could not do if no text could violate
// them.

use sysml_v2_parser::ast::{
    Body, ConnectionDefBodyElement, InOut, PackageBody, PackageBodyElement, RootElement,
};

fn sole_end_decl(body_source: &str) -> sysml_v2_parser::ast::EndDecl {
    let source = format!("package P {{ connection def C {{ {body_source} }} }}");
    let result = parse_with_diagnostics(&source);
    assert!(
        result.errors.is_empty(),
        "`{body_source}` is legal SysML: {:?}",
        result.errors
    );
    let RootElement::Package(pkg) = &result.document.root.elements[0].value else {
        panic!("expected package");
    };
    let PackageBody::Brace { elements, .. } = &pkg.value.body else {
        panic!("expected brace body");
    };
    let PackageBodyElement::ConnectionDef(def) = &elements[0].value else {
        panic!("expected connection def, got {:?}", elements[0].value);
    };
    let Body::Brace { elements, .. } = &def.value.body else {
        panic!("expected brace body");
    };
    match &elements[0].value {
        ConnectionDefBodyElement::EndDecl(end) => end.value.clone(),
        other => panic!("expected an end declaration, got {other:?}"),
    }
}

#[test]
fn an_end_carries_a_ref_prefix_with_its_spans() {
    let end = sole_end_decl("end derived x : T;");
    let span = end
        .ref_prefix
        .derived_span
        .expect("`derived` must be retained, not consumed and dropped");
    assert_eq!(span.len, "derived".len());

    let end = sole_end_decl("end in x : T;");
    let direction = end.ref_prefix.direction.expect("direction retained");
    assert_eq!(direction.value, InOut::In);

    let end = sole_end_decl("end constant x : T;");
    assert!(end.ref_prefix.constant_span.is_some());
}

#[test]
fn an_end_without_a_prefix_records_none() {
    let end = sole_end_decl("end x : T;");
    assert!(
        !end.ref_prefix.is_authored(),
        "an unprefixed end must stay distinguishable from a prefixed one"
    );
}

#[test]
fn a_modifier_after_end_is_reported_only_before_a_declaration_keyword() {
    // Keyworded: `UnextendedUsagePrefix = EndUsagePrefix | BasicUsagePrefix` is exclusive.
    assert_eq!(
        sole_diagnostic_code("package P { connection def C { end derived part p : T; } }")
            .as_deref(),
        Some(END_FEATURE_INVALID_PREFIX)
    );
    // Keyword-less: `DefaultReferenceUsage` spells both, so this is legal and must be silent.
    assert_eq!(
        sole_diagnostic_code("package P { connection def C { end derived x : T; } }"),
        None
    );
}

#[test]
fn a_modifier_before_end_has_no_derivation_in_either_language() {
    // Every production that spells both puts `end` first, so this order is always wrong.
    for source in [
        "package P { connection def C { derived end x : T; } }",
        "classifier C { in end feature f : T; }",
    ] {
        assert_eq!(
            sole_diagnostic_code(source).as_deref(),
            Some(END_FEATURE_INVALID_PREFIX),
            "`{source}`"
        );
    }
}
