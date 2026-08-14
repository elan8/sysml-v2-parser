//! GH-91: standalone `locale` package member and quoted `calc` usage name/type.
//! Each test below uses the exact (trimmed) real source that motivated the fix.

use sysml_v2_parser::ast::{PackageBody, PackageBodyElement, RootElement};
use sysml_v2_parser::parse_with_diagnostics;

fn package_elements(input: &str) -> Vec<PackageBodyElement> {
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace package body");
    };
    elements.iter().map(|e| e.value.clone()).collect()
}

/// Real usage: `Simple Tests/CommentTest.sysml:25-28`:
/// ```text
/// locale "en_US" /*
/// * AAAA
/// * BBBB
/// */
/// ```
/// Previously: `comment_annotation` requires the `comment` keyword unconditionally, but KerML
/// `Comment`'s `('comment' Identification?)?` prefix is entirely optional -- a bare `locale
/// "..." /* ... */` with no `comment` keyword at all was never dispatched at package scope.
#[test]
fn gh91_1_bare_locale_package_member() {
    let elements = package_elements(
        r#"package P {
            locale "en_US" /* AAAA */
        }"#,
    );
    let PackageBodyElement::Comment(c) = &elements[0] else {
        panic!("expected Comment, got {:?}", elements[0]);
    };
    assert!(c.value.identification.is_none());
    assert_eq!(c.value.locale.as_deref(), Some("en_US"));
    assert_eq!(c.value.text.trim(), "AAAA");
}

/// Real usage: `Simple Tests/CommentTest.sysml:32`:
/// ```text
/// doc locale "en_US" /* Documentation about Package */
/// ```
/// Previously: `doc_comment`'s identification-then-locale parsing greedily consumed the bare
/// word `locale` itself as the doc comment's own name (since `identification` doesn't reserve
/// keyword names), leaving nothing for the subsequent `locale` keyword check to match.
#[test]
fn gh91_1_doc_locale_without_identification() {
    let elements = package_elements(
        r#"package P {
            doc locale "en_US" /* Documentation about Package */
        }"#,
    );
    let PackageBodyElement::Doc(d) = &elements[0] else {
        panic!("expected Doc, got {:?}", elements[0]);
    };
    assert!(d.value.identification.is_none());
    assert_eq!(d.value.locale.as_deref(), Some("en_US"));
    assert_eq!(d.value.text.trim(), "Documentation about Package");
}

/// Real usage: `Analysis Examples/Turbojet Stage Analysis.sysml:88` (nested inside `part
/// 'Inlet Gas' : ... { ... }`):
/// ```text
/// calc 'Solve for Pressure1' : 'Ideal Gas Law';
/// ```
/// `calc_usage` itself already fully supports quoted names (both identification and type) --
/// the real gap was that `part_usage_body_element` only dispatched `calc_def_required`
/// (`CalcDef`), never `calc_usage`, so a calc *usage* (as opposed to a calc *definition*)
/// nested in a part usage body had no dispatch path at all, quoted or not.
#[test]
fn gh91_2_quoted_calc_usage_name_and_type() {
    let elements = package_elements(
        r#"package P {
            calc def 'Ideal Gas Law';
            part def MovingIdealGasParcel;
            part 'Inlet Gas' : MovingIdealGasParcel {
                calc 'Solve for Pressure1' : 'Ideal Gas Law';
            }
        }"#,
    );
    let PackageBodyElement::PartUsage(inlet_gas) = &elements[2] else {
        panic!("expected PartUsage, got {:?}", elements[2]);
    };
    let sysml_v2_parser::ast::PartUsageBody::Brace { elements } = &inlet_gas.value.body else {
        panic!("expected brace part usage body");
    };
    let calc = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartUsageBodyElement::CalcUsage(c) => Some(&c.value),
        _ => None,
    });
    let calc = calc.expect("expected a CalcUsage element");
    assert_eq!(
        calc.identification.name.as_deref(),
        Some("Solve for Pressure1")
    );
    assert!(calc.type_name.is_some());
}
