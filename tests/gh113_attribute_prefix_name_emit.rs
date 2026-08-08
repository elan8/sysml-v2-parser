//! GH-113: `AttributeUsage`'s name-standing-in-prefix forms (`attribute :>> target;`,
//! `attribute ::> target;`, `attribute :> target;` -- no separate name written, per
//! `AttributeUsageHead::PrefixRedefines`/`PrefixReferences`/`PrefixSubsets` in
//! `src/parser/attribute.rs`) used to round-trip incorrectly: `emit_attribute_usage`
//! unconditionally emitted the derived display name *and* the subsets/redefines/references
//! clause, duplicating the target (`attribute :>> target;` re-emitted as `attribute target :>>
//! target;`, a structurally different, self-referential construct).
//!
//! Each test below parses the original anonymous form, emits it, reparses the emitted text, and
//! checks the two ASTs are equivalent -- the actual bug was that they weren't.

use sysml_v2_parser::ast::{
    PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement, RootElement,
};
use sysml_v2_parser::{emit_sysml, parse};

/// Digs out the *last* `AttributeUsage` in the first `part def`'s body of a `package P { part def
/// Q { ... } }`-shaped fixture -- the fixtures below declare a plain named attribute first (so
/// the redefines/subsets target resolves) and the name-standing-in-prefix form second.
fn part_def_attribute(src: &str) -> sysml_v2_parser::ast::AttributeUsage {
    let ast = parse(src).expect("parse");
    let RootElement::Package(pkg) = &ast.elements[0].value else {
        panic!("expected Package");
    };
    let PackageBody::Brace { elements } = &pkg.value.body else {
        panic!("expected brace package body");
    };
    let PackageBodyElement::PartDef(q) = &elements[0].value else {
        panic!("expected PartDef");
    };
    let PartDefBody::Brace { elements } = &q.value.body else {
        panic!("expected brace part def body");
    };
    elements
        .iter()
        .filter_map(|e| match &e.value {
            PartDefBodyElement::AttributeUsage(a) => Some(a.value.clone()),
            _ => None,
        })
        .next_back()
        .expect("expected an AttributeUsage element")
}

fn assert_roundtrips(src: &str) {
    let ast1 = parse(src).expect("parse original");
    let emitted = emit_sysml(&ast1).expect("emit");
    let ast2 = parse(&emitted)
        .unwrap_or_else(|e| panic!("reparse of emitted text failed: {e}\nemitted:\n{emitted}"));
    assert_eq!(
        ast1.normalize_for_test_comparison(),
        ast2.normalize_for_test_comparison(),
        "roundtrip AST mismatch; emitted:\n{emitted}"
    );
}

/// `attribute :>> target;` (redefines form) -- name is derived from `target`, must not also be
/// written explicitly on emit.
#[test]
fn gh113_redefines_prefix_name_not_duplicated_on_emit() {
    let src = r#"package P {
        part def Q {
            attribute differencesOf[1];
            attribute :>> differencesOf[1];
        }
    }"#;
    let attr = part_def_attribute(src);
    assert_eq!(attr.name, "differencesOf");
    assert!(attr.redefines.is_some());
    assert_roundtrips(src);
}

/// `attribute ::> target;` (references form), real usage: `Simple Tests/CalculationTest.sysml:14`.
#[test]
fn gh113_references_prefix_name_not_duplicated_on_emit() {
    let src = r#"package P {
        part def VehiclePart { attribute m; }
        part def Vehicle;
        part vehicle : Vehicle {
            part ms : VehiclePart;
            attribute ::> m = ms.m;
        }
    }"#;
    assert_roundtrips(src);
}

/// `attribute :> target;` (subsets form), real usage: `Geometry Examples/
/// CarWithShapeAndCSG.sysml:84`.
#[test]
fn gh113_subsets_prefix_name_not_duplicated_on_emit() {
    let src = r#"package P {
        part def Q {
            attribute differencesOf[1];
            attribute :> differencesOf[1];
        }
    }"#;
    let attr = part_def_attribute(src);
    assert_eq!(attr.name, "differencesOf");
    assert!(attr.subsets.is_some());
    assert_roundtrips(src);
}
