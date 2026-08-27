//! GH-59: neither `ConstraintDefBodyElement` nor `RequirementDefBodyElement` had a dispatch arm
//! for a nested `constraint` member, so both fell back to a generic bucket (`Expression`/`Other`)
//! instead of a real `ConstraintUsage` node. `constraint_usage` itself already fully supported
//! every one of these shapes standalone -- it just was never dispatched from either body-element
//! enum. These tests use the exact (trimmed) real Systems Library lines that motivated the fix:
//! `Systems Library/Requirements.sysml`'s `RequirementConstraintCheck` (nested inside a
//! `constraint def { ... }` body) and `RequirementCheck` (nested inside a
//! `requirement def { ... }` body, redefining `RequirementConstraintCheck`'s members).

use sysml_v2_parser::ast::{
    ConstraintDefBody, ConstraintDefBodyElement, PackageBody, PackageBodyElement,
    RequirementDefBody, RequirementDefBodyElement, RootElement,
};
use sysml_v2_parser::parse_with_diagnostics;

fn package_elements(input: &str) -> (sysml_v2_parser::ParsedDocument, Vec<PackageBodyElement>) {
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
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace package body");
    };
    let elements = elements.iter().map(|e| e.value.clone()).collect();
    (result.document, elements)
}

/// Real usage: `Systems Library/Requirements.sysml`'s `RequirementConstraintCheck`:
/// ```text
/// private abstract constraint def RequirementConstraintCheck {
///     constraint assumptions[0..*] :> constraintChecks, subperformances { ... }
///     constraint constraints[0..*] :> constraintChecks, subperformances { ... }
/// }
/// ```
/// Previously: `constraint assumptions[0..*] :> ...` mis-parsed as a garbage `Expression`
/// (`FeatureRef("constraint")` followed by a spurious operator match against the rest).
#[test]
fn constraint_def_body_dispatches_nested_constraint_members() {
    let input = "package P {\nprivate abstract constraint def RequirementConstraintCheck {\nconstraint assumptions[0..*] :> constraintChecks, subperformances {\n}\nconstraint constraints[0..*] :> constraintChecks, subperformances {\n}\n}\n}";
    let (doc, elements) = package_elements(input);
    let constraint_def = elements
        .iter()
        .find_map(|e| match e {
            PackageBodyElement::ConstraintDef(c) => Some(&c.value),
            _ => None,
        })
        .expect("expected constraint def");
    assert_eq!(
        constraint_def
            .identification
            .name
            .and_then(|n| doc.declaration_name(n)),
        Some("RequirementConstraintCheck")
    );

    let ConstraintDefBody::Brace { elements, .. } = &constraint_def.body else {
        panic!("expected brace constraint def body");
    };
    let nested_names: Vec<&str> = elements
        .iter()
        .filter_map(|e| match &e.value {
            ConstraintDefBodyElement::Constraint(c) => {
                c.value.name.and_then(|n| doc.declaration_name(n))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        nested_names,
        vec!["assumptions", "constraints"],
        "expected both nested constraint members to be dispatched as ConstraintDefBodyElement::Constraint, got elements: {elements:?}"
    );
}

/// Real usage: `Systems Library/Requirements.sysml`'s `RequirementCheck`:
/// ```text
/// abstract requirement def RequirementCheck :> RequirementConstraintCheck {
///     constraint assumptions :>> RequirementConstraintCheck::assumptions;
///     constraint constraints :>> RequirementConstraintCheck::constraints;
/// }
/// ```
/// Distinct from `RequireConstraint` (the `assume`/`require`-prefixed member kind): this is a
/// bare `constraint` member redefining an inherited constraint, previously falling through to
/// `other_requirement_body_element`'s opaque text bucket.
#[test]
fn requirement_def_body_dispatches_nested_constraint_members() {
    let input = "package P {\nabstract requirement def RequirementCheck {\nconstraint assumptions :>> RequirementConstraintCheck::assumptions;\nconstraint constraints :>> RequirementConstraintCheck::constraints;\n}\n}";
    let (doc, elements) = package_elements(input);
    let requirement_def = elements
        .iter()
        .find_map(|e| match e {
            PackageBodyElement::RequirementDef(r) => Some(&r.value),
            _ => None,
        })
        .expect("expected requirement def");
    assert_eq!(
        requirement_def
            .identification
            .name
            .and_then(|n| doc.declaration_name(n)),
        Some("RequirementCheck")
    );

    let RequirementDefBody::Brace { elements, .. } = &requirement_def.body else {
        panic!("expected brace requirement def body");
    };
    let nested_names: Vec<&str> = elements
        .iter()
        .filter_map(|e| match &e.value {
            RequirementDefBodyElement::Constraint(c) => {
                c.value.name.and_then(|n| doc.declaration_name(n))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        nested_names,
        vec!["assumptions", "constraints"],
        "expected both nested constraint members to be dispatched as RequirementDefBodyElement::Constraint, got elements: {elements:?}"
    );
}

/// `RequireConstraint`'s `assume`/`require`-prefixed form must still dispatch to its own
/// dedicated variant, not the new bare `Constraint` arm -- the two are distinct grammar
/// productions (`RequirementConstraintMember` vs. the generic `DefinitionBodyItem` fallback) and
/// must not collide now that both start with different keywords reachable from the same body.
#[test]
fn requirement_def_body_keeps_prefixed_require_constraint_distinct() {
    // Real usage: `sysml/src/examples/Requirements Examples/VehicleRequirementDerivation.sysml`.
    let input = "package P {\nrequirement def R {\nrequire constraint { mass <= massLimit }\n}\n}";
    let (_, elements) = package_elements(input);
    let requirement_def = elements
        .iter()
        .find_map(|e| match e {
            PackageBodyElement::RequirementDef(r) => Some(&r.value),
            _ => None,
        })
        .expect("expected requirement def");
    let RequirementDefBody::Brace { elements, .. } = &requirement_def.body else {
        panic!("expected brace requirement def body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(&e.value, RequirementDefBodyElement::RequireConstraint(_))),
        "expected the prefixed form to still dispatch as RequireConstraint, got: {elements:?}"
    );
    assert!(
        !elements
            .iter()
            .any(|e| matches!(&e.value, RequirementDefBodyElement::Constraint(_))),
        "prefixed require-constraint form must not be captured by the new bare Constraint arm, got: {elements:?}"
    );
}
