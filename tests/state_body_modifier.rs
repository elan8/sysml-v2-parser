//! Upstream gaps 65 and 80: the `parallel`/`initial` state body modifier.
//!
//! `StateDefBody = ';' | ( isParallel ?= 'parallel' )? '{' StateBodyItem* '}'` (SysML BNF 1192).
//! The modifier was previously rejected outright on a `state def` and accepted-then-discarded on a
//! `state` usage, so `StateUsage::isSubstateUsage` and the parallel-subaction library
//! specialization had nothing to read.

use sysml_v2_parser::ast::{
    PackageBody, PackageBodyElement, RootElement, StateBodyModifier, StateDefBody,
    StateDefBodyElement,
};
use sysml_v2_parser::parse_with_diagnostics;

fn package_elements(input: &str) -> (sysml_v2_parser::ParsedDocument, Vec<PackageBodyElement>) {
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let RootElement::Package(pkg) = &result.document.root.elements[0].value else {
        panic!("expected package");
    };
    let PackageBody::Brace { elements, .. } = &pkg.value.body else {
        panic!("expected brace package body");
    };
    let elements = elements.iter().map(|e| e.value.clone()).collect();
    (result.document, elements)
}

fn sole_state_def(input: &str) -> sysml_v2_parser::ast::StateDef {
    match package_elements(input).1.remove(0) {
        PackageBodyElement::StateDef(def) => def.value,
        other => panic!("expected state def, got {other:?}"),
    }
}

fn sole_state_usage(
    input: &str,
) -> (
    sysml_v2_parser::ParsedDocument,
    sysml_v2_parser::ast::StateUsage,
) {
    let (doc, mut elements) = package_elements(input);
    match elements.remove(0) {
        PackageBodyElement::StateUsage(usage) => (doc, usage.value),
        other => panic!("expected state usage, got {other:?}"),
    }
}

#[test]
fn state_def_accepts_parallel_body_modifier() {
    let def = sole_state_def("package P { state def Machine parallel { state a; state b; } }");
    let modifier = def
        .body_modifier
        .expect("`parallel` must reach the AST as a typed modifier");
    assert_eq!(modifier.value, StateBodyModifier::Parallel);

    let StateDefBody::Brace { elements, .. } = &def.body else {
        panic!("expected brace body");
    };
    let states = elements
        .iter()
        .filter(|e| matches!(e.value, StateDefBodyElement::StateUsage(_)))
        .count();
    assert_eq!(states, 2, "both substates must survive the modifier");
}

#[test]
fn state_def_without_modifier_records_none() {
    let def = sole_state_def("package P { state def Machine { state a; } }");
    assert!(def.body_modifier.is_none());
}

#[test]
fn state_def_parallel_modifier_span_covers_the_keyword_only() {
    let source = "package P { state def Machine parallel { state a; } }";
    let modifier = sole_state_def(source).body_modifier.expect("modifier");
    let span = modifier.span;
    assert_eq!(
        &source[span.offset..span.offset + span.len],
        "parallel",
        "the span must locate the authored keyword, not the whole body"
    );
}

#[test]
fn state_usage_preserves_parallel_and_initial_distinctly() {
    let (_, parallel) = sole_state_usage("package P { state S parallel { state child; } }");
    assert_eq!(
        parallel.body_modifier.map(|m| m.value),
        Some(StateBodyModifier::Parallel)
    );

    let (_, initial) = sole_state_usage("package P { state S initial { state child; } }");
    assert_eq!(
        initial.body_modifier.map(|m| m.value),
        Some(StateBodyModifier::Initial)
    );

    let (_, plain) = sole_state_usage("package P { state S { state child; } }");
    assert!(
        plain.body_modifier.is_none(),
        "the three spellings must stay distinguishable to lowering"
    );
}

#[test]
fn a_modifier_keyword_must_be_a_whole_word() {
    // `initialState` is a usage *name*, not an `initial` modifier on an anonymous state.
    let (doc, usage) = sole_state_usage("package P { state initialState { state child; } }");
    assert!(usage.body_modifier.is_none());
    assert_eq!(
        usage.name.and_then(|n| doc.declaration_name(n)),
        Some("initialState")
    );
}

#[test]
fn exhibit_state_preserves_its_body_modifier() {
    let (_, elements) = package_elements(
        "package P { part vehicle { exhibit state vehicleStates parallel { state off; } } }",
    );
    let PackageBodyElement::PartUsage(part) = &elements[0] else {
        panic!("expected part usage, got {:?}", elements[0]);
    };
    let found = format!("{part:?}");
    assert!(
        found.contains("Parallel"),
        "`ExhibitStateUsage` shares `StateUsageBody`, so its modifier must be preserved too"
    );
}
