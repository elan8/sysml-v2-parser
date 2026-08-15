//! Parser tests: occurrence-style DefinitionBody members on flow/allocation defs.

use sysml_v2_parser::ast::*;

fn parse_package(input: &str) -> Package {
    let result = sysml_v2_parser::parse(input).expect("parse should succeed");
    match result.elements[0].value.clone() {
        RootElement::Package(package) => package.value,
        _ => panic!("expected package"),
    }
}

fn brace_package_elements(pkg: &Package) -> &[Node<PackageBodyElement>] {
    match &pkg.body {
        PackageBody::Brace { elements, .. } => elements,
        _ => panic!("expected brace package body"),
    }
}

fn brace_definition_elements(body: &DefinitionBody) -> &[Node<DefinitionBodyElement>] {
    match body {
        DefinitionBody::Brace { elements, .. } => elements,
        _ => panic!("expected brace definition body"),
    }
}

fn has_occurrence_attribute_member(elements: &[Node<DefinitionBodyElement>], name: &str) -> bool {
    elements.iter().any(|element| {
        matches!(
            &element.value,
            DefinitionBodyElement::OccurrenceMember(member)
                if matches!(
                    &member.value,
                    OccurrenceBodyElement::AttributeUsage(attribute)
                        if attribute.value.name == name
                )
        )
    })
}

fn has_occurrence_part_member(elements: &[Node<DefinitionBodyElement>], name: &str) -> bool {
    elements.iter().any(|element| {
        matches!(
            &element.value,
            DefinitionBodyElement::OccurrenceMember(member)
                if matches!(
                    &member.value,
                    OccurrenceBodyElement::PartUsage(part)
                        if part.value.name == name
                )
        )
    })
}

fn has_occurrence_doc_member(elements: &[Node<DefinitionBodyElement>]) -> bool {
    elements.iter().any(|element| {
        matches!(
            &element.value,
            DefinitionBodyElement::OccurrenceMember(member)
                if matches!(
                    &member.value,
                    OccurrenceBodyElement::Annotating(AnnotatingMember::Doc(_))
                )
        )
    })
}

#[test]
fn flow_def_body_parses_inner_attribute() {
    let pkg = parse_package("package P { flow def Power { attribute rate : Real; } }");
    let flow = match &brace_package_elements(&pkg)[0].value {
        PackageBodyElement::FlowDef(flow) => flow,
        _ => panic!("expected FlowDef"),
    };
    let elements = brace_definition_elements(&flow.value.body);
    assert!(
        has_occurrence_attribute_member(elements, "rate"),
        "expected attribute rate in flow def body"
    );
}

#[test]
fn flow_def_body_parses_nested_part() {
    let pkg = parse_package("package P { part def Wheel; flow def Event { part wheel : Wheel; } }");
    let elements = brace_package_elements(&pkg);
    let flow = match &elements[1].value {
        PackageBodyElement::FlowDef(flow) => flow,
        _ => panic!("expected FlowDef"),
    };
    let body_elements = brace_definition_elements(&flow.value.body);
    assert!(
        has_occurrence_part_member(body_elements, "wheel"),
        "expected part wheel in flow def body"
    );
}

#[test]
fn flow_usage_brace_body_parses_attribute() {
    let pkg = parse_package(
        "package P { item def Payload; flow cargo : Payload { attribute weight : Real; } }",
    );
    let flow = brace_package_elements(&pkg)
        .iter()
        .find_map(|element| match &element.value {
            PackageBodyElement::FlowUsage(flow) => Some(flow),
            _ => None,
        })
        .expect("expected FlowUsage");
    let elements = brace_definition_elements(&flow.body);
    assert!(
        has_occurrence_attribute_member(elements, "weight"),
        "expected attribute weight in flow usage body"
    );
}

#[test]
fn allocation_def_body_parses_attribute() {
    let pkg = parse_package("package P { allocation def Map { attribute id : String; } }");
    let alloc = match &brace_package_elements(&pkg)[0].value {
        PackageBodyElement::AllocationDef(alloc) => alloc,
        _ => panic!("expected AllocationDef"),
    };
    let elements = brace_definition_elements(&alloc.value.body);
    assert!(
        has_occurrence_attribute_member(elements, "id"),
        "expected attribute id in allocation def body"
    );
}

#[test]
fn flow_def_doc_only_body_regression() {
    let pkg = parse_package("package P { flow def Power { doc /* note */ } }");
    let flow = match &brace_package_elements(&pkg)[0].value {
        PackageBodyElement::FlowDef(flow) => flow,
        _ => panic!("expected FlowDef"),
    };
    let elements = brace_definition_elements(&flow.value.body);
    assert!(
        has_occurrence_doc_member(elements),
        "expected doc member in flow def body"
    );
}

#[test]
fn flow_def_body_parses_standalone_succession_usage() {
    // Regression: bare `succession first A then B;` directly in a definition body was
    // swallowed as opaque text via `DEFINITION_BODY_OPAQUE_STARTERS` (which listed
    // "succession"). It's now a real `SuccessionUsage` node.
    let pkg =
        parse_package("package P { flow def Sequence { succession first stepA then stepB; } }");
    let flow = match &brace_package_elements(&pkg)[0].value {
        PackageBodyElement::FlowDef(flow) => flow,
        _ => panic!("expected FlowDef"),
    };
    let elements = brace_definition_elements(&flow.value.body);
    let succession = elements
        .iter()
        .find_map(|element| match &element.value {
            DefinitionBodyElement::OccurrenceMember(member) => match &member.value {
                OccurrenceBodyElement::SuccessionUsage(s) => Some(&s.value),
                _ => None,
            },
            _ => None,
        })
        .expect("expected a SuccessionUsage node, not an opaque Other(String)");
    assert!(matches!(
        &succession.source.value,
        Expression::FeatureRef(_)
    ));
    assert!(matches!(
        &succession.target.value,
        Expression::FeatureRef(_)
    ));
}

#[test]
fn flow_def_body_still_parses_succession_flow_as_flow_usage_not_succession_usage() {
    // "Didn't break the neighbor" check: `succession flow X to Y;` must keep routing through
    // `flow_usage_member` (FlowUsageKind::SuccessionFlow), not the new `succession_usage()`.
    let pkg = parse_package(
        "package P { part def Image; part def Camera { part image : Image; } part def Target { part image : Image; } flow def Relay { succession flow focus.image to shoot.image; } }",
    );
    let flow = brace_package_elements(&pkg)
        .iter()
        .find_map(|element| match &element.value {
            PackageBodyElement::FlowDef(flow) => Some(flow),
            _ => None,
        })
        .expect("expected FlowDef");
    let elements = brace_definition_elements(&flow.value.body);
    assert!(
        elements.iter().any(|element| matches!(
            &element.value,
            DefinitionBodyElement::OccurrenceMember(member)
                if matches!(&member.value, OccurrenceBodyElement::FlowUsage(_))
        )),
        "succession flow should still parse as FlowUsage, got {:?}",
        elements
    );
    assert!(
        !elements.iter().any(|element| matches!(
            &element.value,
            DefinitionBodyElement::OccurrenceMember(member)
                if matches!(&member.value, OccurrenceBodyElement::SuccessionUsage(_))
        )),
        "succession flow must not be misclassified as a standalone SuccessionUsage"
    );
}

#[test]
fn flow_def_body_parses_succession_usage_with_multiplicities_like_systems_library() {
    // Regression for the exact real-world form used by the SysML Systems Library's
    // `Flows.sysml` (`succession [seBeforeNum] first [0..1] sourceEvent then [0..1] self;`):
    // a multiplicity on the succession itself, and on both the `first` and `then` ends.
    let document = sysml_v2_parser::parse(
        "package P { flow def M { attribute seBeforeNum : Natural; succession [seBeforeNum] first [0..1] sourceEvent then [0..1] self; } }",
    )
    .expect("parse should succeed");
    let pkg = match &document.elements[0].value {
        RootElement::Package(package) => &package.value,
        _ => panic!("expected package"),
    };
    let flow = match &brace_package_elements(pkg)[0].value {
        PackageBodyElement::FlowDef(flow) => flow,
        _ => panic!("expected FlowDef"),
    };
    let elements = brace_definition_elements(&flow.value.body);
    let succession = elements
        .iter()
        .find_map(|element| match &element.value {
            DefinitionBodyElement::OccurrenceMember(member) => match &member.value {
                OccurrenceBodyElement::SuccessionUsage(s) => Some(&s.value),
                _ => None,
            },
            _ => None,
        })
        .expect("expected a SuccessionUsage node");
    let multiplicity = &succession
        .multiplicity
        .as_ref()
        .expect("succession multiplicity")
        .value;
    let Some(Expression::FeatureRef(reference)) =
        multiplicity.lower.as_deref().map(|node| &node.value)
    else {
        panic!("expected feature-reference multiplicity bound");
    };
    assert_eq!(
        document
            .qualified_reference(*reference)
            .expect("multiplicity reference")
            .authored_text(),
        "seBeforeNum"
    );
    assert_eq!(multiplicity.lower, multiplicity.upper);

    for multiplicity in [
        succession
            .source_multiplicity
            .as_ref()
            .expect("source multiplicity"),
        succession
            .target_multiplicity
            .as_ref()
            .expect("target multiplicity"),
    ] {
        assert!(matches!(
            multiplicity.value.lower.as_deref().map(|node| &node.value),
            Some(Expression::LiteralInteger(0))
        ));
        assert!(matches!(
            multiplicity.value.upper.as_deref().map(|node| &node.value),
            Some(Expression::LiteralInteger(1))
        ));
    }
    assert!(matches!(
        &succession.source.value,
        Expression::FeatureRef(_)
    ));
    assert!(matches!(
        &succession.target.value,
        Expression::FeatureRef(_)
    ));
}
