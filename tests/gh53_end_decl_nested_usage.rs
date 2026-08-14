//! GH-53: `end` declarations previously only supported a bare `:` typed target or a `::>`/
//! `references` reference target. Two vendored library files use a third form where the target is
//! itself a complete, nested kind-prefixed usage (`occurrence`/`item`), with an additional
//! "middle" multiplicity position between the end's own name and that nested usage. These tests
//! cover each confirmed real-usage fix, using the exact (trimmed) real-library lines that
//! motivated it, plus confirmation that the pre-existing `end` forms still work unaffected.

use sysml_v2_parser::ast::{
    ConnectionDefBody, ConnectionDefBodyElement, EndIdentity, EndNestedUsage, PackageBody,
    PackageBodyElement, RootElement,
};
use sysml_v2_parser::parse_with_diagnostics;

fn connection_def_ends(input: &str) -> Vec<sysml_v2_parser::ast::EndDecl> {
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
    let connection = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::ConnectionDef(c) => Some(&c.value),
            _ => None,
        })
        .expect("expected connection def");
    let ConnectionDefBody::Brace { elements } = &connection.body else {
        panic!("expected connection def brace body");
    };
    elements
        .iter()
        .filter_map(|e| match &e.value {
            ConnectionDefBodyElement::EndDecl(end) => Some(end.value.clone()),
            _ => None,
        })
        .collect()
}

fn assert_declaration_name(end: &sysml_v2_parser::ast::EndDecl, expected: &str) {
    assert!(matches!(
        &end.identity,
        EndIdentity::Declaration(name) if name.value == expected
    ));
}

/// Real usage: Systems Library `Domain Libraries/Cause and Effect/CausationConnections.sysml`'s
/// `end theCauses [*] occurrence theCause :> causes :>> source { ... }` -- the target is a nested
/// `occurrence` usage, not a bare type/reference, with a multiplicity between the end's own name
/// (`theCauses`) and that nested usage.
#[test]
fn end_decl_accepts_nested_occurrence_usage_with_middle_multiplicity() {
    let input = "package P {\nconnection def Causation {\nend theCauses [*] occurrence theCause :> causes :>> source {\n}\nend theEffects [*] occurrence theEffect :> effects :>> target {\n}\n}\n}";
    let ends = connection_def_ends(input);
    assert_eq!(ends.len(), 2);

    let the_causes = &ends[0];
    assert_declaration_name(the_causes, "theCauses");
    assert!(the_causes.multiplicity.is_some());
    let nested = the_causes
        .nested_usage
        .as_deref()
        .expect("expected nested usage");
    let EndNestedUsage::Occurrence(occurrence) = nested else {
        panic!("expected occurrence nested usage, got {nested:?}");
    };
    assert_eq!(occurrence.value.name, "theCause");
}

/// Real usage: Systems Library `Domain Libraries/.../Items.sysml`'s `end touchesToo [0..*] item
/// touchedItemToo :>> separateSpaceToo, thisOccurrence;` -- the target is a nested `item` usage.
#[test]
fn end_decl_accepts_nested_item_usage_with_middle_multiplicity() {
    let input = "package P {\nconnection def Touches {\nend touchesToo [0..*] item touchedItemToo :>> separateSpaceToo, thisOccurrence;\nend touches [0..*] item touchedItem :>> separateSpace, thatOccurrence;\n}\n}";
    let ends = connection_def_ends(input);
    assert_eq!(ends.len(), 2);

    let touches_too = &ends[0];
    assert_declaration_name(touches_too, "touchesToo");
    assert!(touches_too.multiplicity.is_some());
    let nested = touches_too
        .nested_usage
        .as_deref()
        .expect("expected nested usage");
    let EndNestedUsage::Item(item) = nested else {
        panic!("expected item nested usage, got {nested:?}");
    };
    assert_eq!(item.value.name, "touchedItemToo");
}

/// Confirms the pre-existing `end` forms (typed, `::>` reference, and `:>>` redefines trailing the
/// typed form) still parse identically and leave `nested_usage` as `None` -- the new nested-usage
/// alternative and middle-multiplicity position must not affect these unrelated forms.
#[test]
fn end_decl_existing_forms_unaffected() {
    let input = "package P {\nconnection def C {\nend hub ::> mainSwitch[1];\nend source: Anything :>> BinaryLinkObject::source;\n}\n}";
    let ends = connection_def_ends(input);
    assert_eq!(ends.len(), 2);

    let hub = &ends[0];
    assert_declaration_name(hub, "hub");
    assert!(hub.references.is_some());
    assert!(hub.typing.is_none());
    assert!(hub.nested_usage.is_none());

    let source = &ends[1];
    assert_declaration_name(source, "source");
    assert!(source.typing.is_some());
    assert!(source.nested_usage.is_none());
    assert_eq!(
        source.redefines.as_ref().map(|n| n.value.target.len()),
        Some(1)
    );
}
