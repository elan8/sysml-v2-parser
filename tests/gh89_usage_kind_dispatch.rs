//! GH-89: usage-kind body-member dispatch gaps triaged from #83's `examples/` roundtrip scan.
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
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace package body");
    };
    elements.iter().map(|e| e.value.clone()).collect()
}

/// Real usage: `Simple Tests/ConnectionTest.sysml:31-35`:
/// ```text
/// abstract connection def C {
///     part p;
///     end end1;
///     end end2;
///     end end3;
/// }
/// ```
/// Previously: `connection_def_body_element` had no `part_usage` dispatch at all.
#[test]
fn gh89_1_bare_part_usage_in_connection_def_body() {
    let elements = package_elements(
        r#"package P {
            abstract connection def C {
                part p;
                end end1 : Real;
            }
        }"#,
    );
    let PackageBodyElement::ConnectionDef(c) = &elements[0] else {
        panic!("expected ConnectionDef, got {:?}", elements[0]);
    };
    let sysml_v2_parser::ast::ConnectionDefBody::Brace { elements, .. } = &c.value.body else {
        panic!("expected brace connection def body");
    };
    let part = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::ConnectionDefBodyElement::PartUsage(p) => Some(&p.value),
        _ => None,
    });
    let part = part.expect("expected a PartUsage element");
    assert_eq!(part.name, "p");
}

/// Same gap, but for the anonymous, unnamed `connection { ... }` form (Simple Tests/
/// ConnectionTest.sysml:51-54: `connection { part q; end ref end1 ::> d1 :> q; end end2 ::> d2; }`
/// -- parses as an anonymous `ConnectionDef` since it has no name or type), which shares the same
/// `connection_def_body_element` dispatch as the named def form above.
#[test]
fn gh89_1_bare_part_usage_in_anonymous_connection_body() {
    let elements = package_elements(
        r#"package P {
            part def T;
            connection {
                part q;
                end e : T;
            }
        }"#,
    );
    let PackageBodyElement::ConnectionDef(c) = &elements[1] else {
        panic!("expected ConnectionDef, got {:?}", elements[1]);
    };
    let sysml_v2_parser::ast::ConnectionDefBody::Brace { elements, .. } = &c.value.body else {
        panic!("expected brace connection body");
    };
    let part = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::ConnectionDefBodyElement::PartUsage(p) => Some(&p.value),
        _ => None,
    });
    let part = part.expect("expected a PartUsage element");
    assert_eq!(part.name, "q");
}

/// Real usage: `Camera Example/Camera.sysml:4`:
/// ```text
/// part def Camera {
///     perform action takePicture[*] :> PictureTaking::takePicture;
/// }
/// ```
/// Previously: `perform_action_decl` handled `:>>` redefines and `: Type`, but had no path for a
/// multiplicity (`[*]`) after the name or a `:>` subsets clause.
/// Real usage: `Simple Tests/AliasTest.sysml:5-8`:
/// ```text
/// part def P1 {
///     port porig1;
///     alias po1 for porig1;
/// }
/// ```
/// Previously: `alias_def` was only dispatched at package-body scope.
#[test]
fn gh89_3_alias_in_part_def_body() {
    let elements = package_elements(
        r#"package P {
            part def P1 {
                port porig1;
                alias po1 for porig1;
            }
        }"#,
    );
    let PackageBodyElement::PartDef(p1) = &elements[0] else {
        panic!("expected PartDef, got {:?}", elements[0]);
    };
    let sysml_v2_parser::ast::PartDefBody::Brace { elements, .. } = &p1.value.body else {
        panic!("expected brace part def body");
    };
    let alias = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartDefBodyElement::AliasDef(a) => Some(&a.value),
        _ => None,
    });
    let alias = alias.expect("expected an AliasDef element");
    assert_eq!(alias.identification.name.as_deref(), Some("po1"));
}

/// Same gap, but for a part *usage* body (Simple Tests/AliasTest.sysml:14-17):
/// ```text
/// part p2 : P1 {
///     port pdest;
///     alias pd1 for pdest;
/// }
/// ```
#[test]
fn gh89_3_alias_in_part_usage_body() {
    let elements = package_elements(
        r#"package P {
            part def P1;
            part p2 : P1 {
                port pdest;
                alias pd1 for pdest;
            }
        }"#,
    );
    let PackageBodyElement::PartUsage(p2) = &elements[1] else {
        panic!("expected PartUsage, got {:?}", elements[1]);
    };
    let sysml_v2_parser::ast::PartUsageBody::Brace { elements, .. } = &p2.value.body else {
        panic!("expected brace part usage body");
    };
    let alias = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartUsageBodyElement::AliasDef(a) => Some(&a.value),
        _ => None,
    });
    let alias = alias.expect("expected an AliasDef element");
    assert_eq!(alias.identification.name.as_deref(), Some("pd1"));
}

/// Real usage: `Simple Tests/UseCaseTest.sysml:32-36`:
/// ```text
/// part system : System {
///     include uc2;
///     perform u;
///     use case uc1 : UC1;
/// }
/// ```
/// Previously: `include_use_case` was only dispatched inside a use case definition body. Also
/// discovered alongside it in the same real fixture: `use_case_usage` was already dispatched in
/// `PartDefBodyElement` but not `PartUsageBodyElement`, so `use case uc1 : UC1;` failed too.
#[test]
fn gh89_4_include_and_use_case_usage_in_part_usage_body() {
    let elements = package_elements(
        r#"package P {
            use case def UC1;
            use case uc2;
            part def System;
            part u;
            part system : System {
                include uc2;
                perform u;
                use case uc1 : UC1;
            }
        }"#,
    );
    let PackageBodyElement::PartUsage(system) = &elements[4] else {
        panic!("expected PartUsage, got {:?}", elements[4]);
    };
    let sysml_v2_parser::ast::PartUsageBody::Brace { elements, .. } = &system.value.body else {
        panic!("expected brace part usage body");
    };
    let include = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartUsageBodyElement::IncludeUseCase(i) => Some(&i.value),
        _ => None,
    });
    let include = include.expect("expected an IncludeUseCase element");
    let _target_reference = include.target;

    let use_case = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartUsageBodyElement::UseCaseUsage(u) => Some(&u.value),
        _ => None,
    });
    let use_case = use_case.expect("expected a UseCaseUsage element");
    assert_eq!(use_case.name, "uc1");
}

/// Real usage: `Simple Tests/ConstraintTest.sysml:78-81`:
/// ```text
/// part vehicle3 :> vehicle {
///     assert massAnalysis3 {
///         in totalMass = mass;
///         in componentMasses = (engine.mass, frontAxleAssembly.mass, rearAxleAssembly.mass);
///     }
/// }
/// ```
/// Previously: `assert_constraint_member` required the literal `constraint` keyword. A named
/// `assert <name> { ... }`, referencing a previously-declared standalone `constraint` by name and
/// rebinding its `in` parameters, is real usage, richer than the already-supported `assert
/// constraint ...` form.
#[test]
fn gh89_5_named_assert_without_constraint_keyword() {
    let elements = package_elements(
        r#"package P {
            part def Component { attribute mass = 1.0; }
            part vehicle : Component;
            constraint massAnalysis3 {
                in totalMass = 1.0;
                in componentMasses = 2.0;
            }
            part vehicle3 :> vehicle {
                assert massAnalysis3 {
                    in totalMass = mass;
                    in componentMasses = mass;
                }
            }
        }"#,
    );
    let PackageBodyElement::PartUsage(vehicle3) = &elements[3] else {
        panic!("expected PartUsage, got {:?}", elements[3]);
    };
    let sysml_v2_parser::ast::PartUsageBody::Brace { elements, .. } = &vehicle3.value.body else {
        panic!("expected brace part usage body");
    };
    let assert_member = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartUsageBodyElement::AssertConstraint(a) => Some(&a.value),
        _ => None,
    });
    let assert_member = assert_member.expect("expected an AssertConstraint element");
    assert!(assert_member.declaration_name.is_none());
    assert!(assert_member.target.is_some());
    assert!(assert_member.type_name.is_none());
    assert!(!assert_member.is_negated);
}

/// Real usage: `Simple Tests/ConstraintTest.sysml:89`:
/// ```text
/// assert not massLimitation { :>> mass = vehicle3.mass; :>> massLimit = vehicle4.mass; }
/// ```
/// Found alongside the gap above in the same real fixture: `assert_constraint_member` was
/// dispatched in six other body contexts (action, part def/usage, connection def, occurrence,
/// attribute) but not at package scope.
#[test]
fn gh89_5_assert_not_at_package_scope() {
    let elements = package_elements(
        r#"package P {
            attribute mass = 1.0;
            attribute massLimit = 2.0;
            constraint massLimitation {
                mass : Real;
                massLimit : Real;
            }
            assert not massLimitation { :>> mass = 1.0; :>> massLimit = 2.0; }
        }"#,
    );
    let assert_member = elements.iter().find_map(|e| match e {
        PackageBodyElement::AssertConstraint(a) => Some(&a.value),
        _ => None,
    });
    let assert_member = assert_member.expect("expected an AssertConstraint element");
    assert!(assert_member.declaration_name.is_none());
    assert!(assert_member.target.is_some());
    assert!(assert_member.is_negated);
}

/// Real usage: `Simple Tests/VerificationTest.sysml:34-36`:
/// ```text
/// part verificationContext {
///     verification verificationPlan : VerificationPlan {
///         subject v = vv;
///     }
/// }
/// ```
/// Previously: `verification_case_usage` was only dispatched at package scope and inside part
/// *definition* bodies, not plain part usage bodies.
#[test]
fn gh89_6_verification_usage_in_part_usage_body() {
    let elements = package_elements(
        r#"package P {
            verification def VerificationPlan;
            part def V;
            part vv : V;
            part verificationContext {
                verification verificationPlan : VerificationPlan {
                    subject v = vv;
                }
            }
        }"#,
    );
    let PackageBodyElement::PartUsage(vc) = &elements[3] else {
        panic!("expected PartUsage, got {:?}", elements[3]);
    };
    let sysml_v2_parser::ast::PartUsageBody::Brace { elements, .. } = &vc.value.body else {
        panic!("expected brace part usage body");
    };
    let verification = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartUsageBodyElement::VerificationCaseUsage(v) => Some(&v.value),
        _ => None,
    });
    let verification = verification.expect("expected a VerificationCaseUsage element");
    assert_eq!(verification.name, "verificationPlan");
}

/// Real usage: `Simple Tests/ViewTest.sysml:23-32`:
/// ```text
/// rendering def R;
/// view def V {
///     render rendering r1: R[0..1];
/// }
/// ```
/// `ViewRenderingUsage`'s BNF (Clause 8.2.2.26.1) has two alternatives after the `render`
/// keyword: a bare reference-subsetting shorthand (`render r;`, already supported) or an
/// explicit `'rendering' Usage` declaration. Previously `view_rendering_usage` had no path for
/// the second alternative's leading `rendering` keyword, so it was consumed as the usage's own
/// *name* and the following `r1: R[0..1]` broke the header parse.
#[test]
fn gh89_8_render_rendering_member_in_view_def_body() {
    let elements = package_elements(
        r#"package ViewTest {
            rendering def R;
            view def V {
                render rendering r1: R[0..1];
            }
        }"#,
    );
    let PackageBodyElement::ViewDef(v) = &elements[1] else {
        panic!("expected ViewDef, got {:?}", elements[1]);
    };
    let sysml_v2_parser::ast::ViewDefBody::Brace { elements, .. } = &v.value.body else {
        panic!("expected brace view def body");
    };
    let rendering = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::ViewDefBodyElement::ViewRendering(r) => Some(&r.value),
        _ => None,
    });
    let rendering = rendering.expect("expected a ViewRenderingUsage element");
    assert_eq!(rendering.name, "r1");
    assert!(rendering.type_name.is_some());
}

/// Real usage: `Timeslice and Snapshot Examples/TimeVaryingAttribute.sysml:14`:
/// ```text
/// part def Transport2 {
///     out item pwrCmd:PwrCmd;
/// }
/// ```
/// `action_def_body_element` already dispatches `directed_item_usage` (`in`/`out` item), but
/// `part_def_body_element` only had the plain `item_usage` arm (no direction-prefix handling),
/// so `out` was reported as an unrecognized keyword rather than falling through to a structure
/// usage member.
#[test]
fn gh89_9_directed_item_usage_in_part_def_body() {
    let elements = package_elements(
        r#"package TimeVaryingAttribute {
            item def PwrCmd;
            part def Transport2 {
                out item pwrCmd:PwrCmd;
            }
        }"#,
    );
    let PackageBodyElement::PartDef(transport) = &elements[1] else {
        panic!("expected PartDef, got {:?}", elements[1]);
    };
    let sysml_v2_parser::ast::PartDefBody::Brace { elements, .. } = &transport.value.body else {
        panic!("expected brace part def body");
    };
    let item = elements.iter().find_map(|e| match &e.value {
        sysml_v2_parser::ast::PartDefBodyElement::ItemUsage(i) => Some(&i.value),
        _ => None,
    });
    let item = item.expect("expected an ItemUsage element");
    assert_eq!(item.name, "pwrCmd");
    assert!(item.type_name.is_some());
    assert_eq!(
        item.prefix
            .basic
            .ref_prefix
            .direction
            .as_ref()
            .map(|node| node.value),
        Some(sysml_v2_parser::ast::InOut::Out)
    );
}
