//! Upstream gap 64: conjugation *declarations*, as opposed to the `~T` conjugated-typing flag.
//!
//! ```text
//! TypeDeclaration = … ( SpecializationPart | ConjugationPart )? TypeRelationshipPart*  -- BNF 455
//! ConjugationPart : Type = ( 'conjugates' | '~' ) ownedRelationship += OwnedConjugation -- 462
//! ```
//!
//! The parser previously modelled conjugation only as `TypingRelationship::is_conjugated` -- the
//! `~T` flag on the type a feature is *typed by* -- so `classifier One conjugates A;` had no node
//! and fell through to `unsupported_grammar_form`. That made both KerML conjugation constraints
//! (`validateTypeAtMostOneConjugator`, `validateSpecializationSpecificNotConjugated`)
//! unauthorable.

use sysml_v2_parser::ast::{
    ConjugationSpelling, KermlClassifierDecl, PackageBody, PackageBodyElement, RootElement,
};
use sysml_v2_parser::parse_with_diagnostics;

fn sole_classifier(input: &str) -> KermlClassifierDecl {
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics for `{input}`: {:?}",
        result.errors
    );
    match &result.document.root.elements[0].value {
        RootElement::Member(member) => match &member.value {
            PackageBodyElement::KermlClassifier(decl) => decl.value.clone(),
            other => panic!("expected a typed KerML classifier, got {other:?}"),
        },
        other => panic!("expected a root member, got {other:?}"),
    }
}

#[test]
fn the_keyword_spelling_produces_one_conjugation() {
    let decl = sole_classifier("classifier One conjugates A;");
    let conjugation = decl.conjugates.expect("`conjugates` must reach the AST");
    assert_eq!(conjugation.value.spelling, ConjugationSpelling::Keyword);
    assert!(
        decl.specializes.is_none(),
        "`SpecializationPart | ConjugationPart` is a choice: only one may be filled"
    );
}

#[test]
fn the_operator_spelling_is_recorded_as_such() {
    let decl = sole_classifier("classifier One ~ A;");
    let conjugation = decl.conjugates.expect("`~` must reach the AST");
    assert_eq!(conjugation.value.spelling, ConjugationSpelling::Operator);
}

#[test]
fn the_clause_span_locates_the_authored_text() {
    let source = "classifier One conjugates A;";
    let decl = sole_classifier(source);
    let span = decl.conjugates.expect("conjugation").value.span;
    assert_eq!(&source[span.offset..span.offset + span.len], "conjugates A");
}

#[test]
fn two_declarations_produce_two_conjugations() {
    let result =
        parse_with_diagnostics("package P { classifier One conjugates A; classifier Two ~ B; }");
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let RootElement::Package(pkg) = &result.document.root.elements[0].value else {
        panic!("expected package");
    };
    let PackageBody::Brace { elements, .. } = &pkg.value.body else {
        panic!("expected brace body");
    };
    let conjugations: Vec<_> = elements
        .iter()
        .map(|e| match &e.value {
            PackageBodyElement::KermlClassifier(decl) => decl
                .value
                .conjugates
                .as_ref()
                .map(|c| c.value.spelling)
                .expect("each declaration carries its own conjugation"),
            other => panic!("expected a KerML classifier, got {other:?}"),
        })
        .collect();
    assert_eq!(
        conjugations,
        vec![ConjugationSpelling::Keyword, ConjugationSpelling::Operator]
    );
}

#[test]
fn a_conjugated_declaration_still_takes_a_body_and_a_qualified_target() {
    let decl = sole_classifier("struct S conjugates A::B { feature f; }");
    assert!(decl.conjugates.is_some());
    assert!(matches!(
        decl.body,
        sysml_v2_parser::ast::CalcDefBody::Brace { .. }
    ));
}

#[test]
fn a_specializing_declaration_carries_no_conjugation() {
    let decl = sole_classifier("classifier One specializes A;");
    assert!(decl.conjugates.is_none());
    assert!(decl.specializes.is_some());
}
