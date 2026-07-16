//! Multi-context FlowUsage parser tests (S42-LIM-005).

use sysml_v2_parser::ast::{
    ActionDefBody, ActionDefBodyElement, DefinitionBody, FlowUsageKind, OccurrenceBodyElement,
    PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement, PartUsageBodyElement,
    RootElement, UseCaseDefBodyElement,
};
use sysml_v2_parser::parse;

fn package_body(input: &str) -> Vec<sysml_v2_parser::ast::Node<PackageBodyElement>> {
    let result = parse(input).expect("parse");
    match &result.elements[0].value {
        RootElement::Package(p) => match &p.value.body {
            PackageBody::Brace { elements } => elements.clone(),
            _ => panic!("expected brace package body"),
        },
        _ => panic!("expected package"),
    }
}

fn part_def_body(input: &str) -> Vec<sysml_v2_parser::ast::Node<PartDefBodyElement>> {
    let result = parse(input).expect("parse");
    match &result.elements[0].value {
        RootElement::Package(p) => {
            let elements = match &p.value.body {
                PackageBody::Brace { elements } => elements,
                _ => panic!("expected brace package body"),
            };
            match &elements[0].value {
                PackageBodyElement::PartDef(part) => match &part.value.body {
                    PartDefBody::Brace { elements } => elements.clone(),
                    _ => panic!("expected part def brace body"),
                },
                _ => panic!("expected part def"),
            }
        }
        _ => panic!("expected package"),
    }
}

#[test]
fn anonymous_flow_in_part_def() {
    let elements = part_def_body("package P { part def V { part a; part b; flow a.x to b.y; } }");
    assert!(elements.iter().any(|e| {
        matches!(
            e.value,
            PartDefBodyElement::FlowUsage(ref flow) if flow.value.name.is_none()
                && flow.value.from.is_some()
                && flow.value.to.is_some()
        )
    }));
}

#[test]
fn named_flow_in_package() {
    let elements = package_body("package P { flow transfer : Fuel from src to dst; }");
    match &elements[0].value {
        PackageBodyElement::FlowUsage(flow) => {
            assert_eq!(flow.value.name.as_deref(), Some("transfer"));
            assert_eq!(flow.value.type_name.as_deref(), Some("Fuel"));
            assert!(flow.value.from.is_some());
            assert!(flow.value.to.is_some());
        }
        other => panic!("expected FlowUsage, got {other:?}"),
    }
}

#[test]
fn flow_in_occurrence_def_body() {
    let result = parse("package P { occurrence def O { flow a to b; } }").expect("parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let part = match &pkg.body {
        PackageBody::Brace { elements } => match &elements[0].value {
            PackageBodyElement::OccurrenceDef(occ) => occ,
            other => panic!("expected occurrence def, got {other:?}"),
        },
        _ => panic!("expected brace body"),
    };
    match &part.body {
        DefinitionBody::Brace { elements } => {
            assert!(elements.iter().any(|e| matches!(
                &e.value,
                sysml_v2_parser::ast::DefinitionBodyElement::OccurrenceMember(member)
                    if matches!(member.value, OccurrenceBodyElement::FlowUsage(_))
            )));
        }
        _ => panic!("expected brace definition body"),
    }
}

#[test]
fn succession_flow_in_action_def() {
    let result =
        parse("package P { action def A { succession flow focus.image to shoot.image; } }")
            .expect("parse");
    let action = match &result.elements[0].value {
        RootElement::Package(p) => match &p.value.body {
            PackageBody::Brace { elements } => match &elements[0].value {
                PackageBodyElement::ActionDef(a) => a,
                other => panic!("expected action def, got {other:?}"),
            },
            _ => panic!("expected brace body"),
        },
        _ => panic!("expected package"),
    };
    match &action.value.body {
        ActionDefBody::Brace { elements } => {
            assert!(elements.iter().any(|e| matches!(
                &e.value,
                ActionDefBodyElement::FlowUsage(flow)
                    if flow.value.kind == FlowUsageKind::SuccessionFlow
                        && flow.value.name.is_none()
            )));
        }
        _ => panic!("expected brace action body"),
    }
}

#[test]
fn message_flow_in_part_def() {
    let elements = part_def_body("package P { part def V { message evt from a to b; } }");
    assert!(elements.iter().any(|e| matches!(
        &e.value,
        PartDefBodyElement::FlowUsage(flow) if flow.value.kind == FlowUsageKind::Message
    )));
}

#[test]
fn flow_in_use_case_def() {
    let result =
        parse("package P { use case def UC { flow actor.msg to system.inbox; } }").expect("parse");
    let uc = match &result.elements[0].value {
        RootElement::Package(p) => match &p.value.body {
            PackageBody::Brace { elements } => match &elements[0].value {
                PackageBodyElement::UseCaseDef(u) => u,
                other => panic!("expected use case def, got {other:?}"),
            },
            _ => panic!("expected brace body"),
        },
        _ => panic!("expected package"),
    };
    match &uc.value.body {
        sysml_v2_parser::ast::UseCaseDefBody::Brace { elements } => {
            assert!(elements
                .iter()
                .any(|e| matches!(e.value, UseCaseDefBodyElement::FlowUsage(_))));
        }
        _ => panic!("expected brace use case body"),
    }
}

#[test]
fn flow_in_part_usage_body() {
    let result = parse("package P { part p : P { flow a to b; } }").expect("parse");
    let part = match &result.elements[0].value {
        RootElement::Package(p) => match &p.value.body {
            PackageBody::Brace { elements } => match &elements[0].value {
                PackageBodyElement::PartUsage(part) => part,
                other => panic!("expected part usage, got {other:?}"),
            },
            _ => panic!("expected brace body"),
        },
        _ => panic!("expected package"),
    };
    match &part.value.body {
        sysml_v2_parser::ast::PartUsageBody::Brace { elements } => {
            assert!(elements
                .iter()
                .any(|e| matches!(e.value, PartUsageBodyElement::FlowUsage(_))));
        }
        _ => panic!("expected brace part usage body"),
    }
}
