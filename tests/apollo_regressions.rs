use sysml_v2_parser::ast::{
    ActionDefBody, AnnotatingMember, Body, CalcDefBody, CalcDefBodyElement, ConnectionDefBody,
    ConnectionDefBodyElement, ConstraintDefBody, ConstraintDefBodyElement, Expression, InOut,
    OccurrenceBodyElement, OccurrenceUsageBody, PackageBody, PackageBodyElement, PartDefBody,
    PartDefBodyElement, PartUsageBody, PartUsageBodyElement, RequirementDefBody,
    RequirementDefBodyElement, RootElement, StateDefBody, StateDefBodyElement,
    UseCaseDefBodyElement,
};
use sysml_v2_parser::{parse, parse_with_diagnostics};

fn package_elements(
    input: &str,
) -> (
    sysml_v2_parser::ParsedDocument,
    Vec<sysml_v2_parser::Node<PackageBodyElement>>,
) {
    let root = parse(input).expect("input should parse");
    let pkg = match &root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements, .. } => elements.clone(),
        _ => panic!("expected brace package body"),
    };
    (root, elements)
}

#[test]
fn individual_part_definition_and_usage_parse_as_parts() {
    let input = "package P {\nindividual part def 'Neil Armstrong' :> Astronaut { }\nindividual part crewMember : Astronaut { }\n}";
    let (doc, elements) = package_elements(input);

    match &elements[0].value {
        PackageBodyElement::PartDef(def) => {
            assert!(def.value.is_individual);
            assert_eq!(
                def.value
                    .identification
                    .name
                    .and_then(|n| doc.declaration_name(n)),
                Some("'Neil Armstrong'")
            );
        }
        other => panic!("expected individual part def, got {other:?}"),
    }

    match &elements[1].value {
        PackageBodyElement::PartUsage(usage) => {
            assert!(usage.value.prefix.individual_span().is_some());
            assert_eq!(
                usage.value.name.and_then(|n| doc.declaration_name(n)),
                Some("crewMember")
            );
        }
        other => panic!("expected individual part usage, got {other:?}"),
    }
}

#[test]
fn requirement_usage_supports_trailing_subsets_after_body() {
    let input = "package P {\npart def Mission {\nrequirement goals[1..*] : Goal;\n}\npart def ApolloMission :> Mission {\nrequirement goToMoon : Goal {\ndoc /* Perform a crewed lunar landing and return to Earth. */\n} :> goals;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let part = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(def)
                if def
                    .value
                    .identification
                    .name
                    .and_then(|n| result.document.declaration_name(n))
                    == Some("ApolloMission") =>
            {
                Some(&def.value)
            }
            _ => None,
        })
        .expect("ApolloMission part def should be present");
    let PartDefBody::Brace { elements, .. } = &part.body else {
        panic!("expected part body");
    };
    let req = elements.iter().find_map(|e| match &e.value {
        PartDefBodyElement::RequirementUsage(req) => Some(&req.value),
        _ => None,
    });
    let req = req.expect("requirement usage should parse in part body");
    assert!(req.subsets.is_some());
}

#[test]
fn exhibit_state_body_supports_unnamed_and_accepting_transitions() {
    let input = "package P {\npart def Mission {\nexhibit state phases {\nstate initial : Initial;\nstate launch : Launch;\ntransition first initial then launch;\ntransition first launch accept LaunchDone then initial {\ndoc /* Example transition body. */\n}\n}\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let mission = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(def)
                if def
                    .value
                    .identification
                    .name
                    .and_then(|n| result.document.declaration_name(n))
                    == Some("Mission") =>
            {
                Some(&def.value)
            }
            _ => None,
        })
        .expect("expected Mission part def");
    let PartDefBody::Brace { elements, .. } = &mission.body else {
        panic!("expected part body");
    };
    let exhibit = elements
        .iter()
        .find_map(|e| match &e.value {
            PartDefBodyElement::ExhibitState(exhibit) => Some(&exhibit.value),
            _ => None,
        })
        .expect("exhibit state should be present");
    let StateDefBody::Brace { elements, .. } = &exhibit.body else {
        panic!("expected exhibit state body");
    };
    let transitions: Vec<_> = elements
        .iter()
        .filter_map(|e| match &e.value {
            StateDefBodyElement::Transition(t) => Some(&t.value),
            _ => None,
        })
        .collect();
    assert_eq!(transitions.len(), 2);
    assert_eq!(transitions[0].name, None);
    assert_eq!(transitions[1].name, None);
}

#[test]
fn transition_name_with_do_prefix_is_not_confused_with_do_keyword() {
    let input = "package P {\nstate def M {\nstate docking;\nstate charging;\ntransition docked first docking then charging;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let machine = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::StateDef(def)
                if def
                    .value
                    .identification
                    .name
                    .and_then(|n| result.document.declaration_name(n))
                    == Some("M") =>
            {
                Some(&def.value)
            }
            _ => None,
        })
        .expect("expected state def M");
    let StateDefBody::Brace { elements, .. } = &machine.body else {
        panic!("expected state body");
    };
    let transition = elements
        .iter()
        .find_map(|e| match &e.value {
            StateDefBodyElement::Transition(t) => Some(&t.value),
            _ => None,
        })
        .expect("expected named transition");
    assert_eq!(
        transition
            .name
            .and_then(|n| result.document.declaration_name(n)),
        Some("docked")
    );
}

#[test]
fn timeslice_and_snapshot_parse_inside_part_and_occurrence_bodies() {
    let input = "package P {\nindividual part def MissionIndividual :> Mission;\nindividual part mission : MissionIndividual {\ntimeslice liftoff {\nsnapshot atT0 {\nattribute missionTime = 0;\n}\n}\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let usage = match &elements[1].value {
        PackageBodyElement::PartUsage(usage) => &usage.value,
        _ => panic!("expected part usage"),
    };
    let PartUsageBody::Brace { elements, .. } = &usage.body else {
        panic!("expected part usage body");
    };
    let timeslice = elements
        .iter()
        .find_map(|e| match &e.value {
            PartUsageBodyElement::OccurrenceUsage(occ) => Some(&occ.value),
            _ => None,
        })
        .expect("timeslice should parse in part body");
    assert_eq!(
        timeslice.prefix.portion().map(|node| node.value),
        Some(sysml_v2_parser::OccurrencePortionKind::Timeslice)
    );
    let OccurrenceUsageBody::Brace { elements, .. } = &timeslice.body else {
        panic!("expected timeslice body");
    };
    let snapshot = elements
        .iter()
        .find_map(|e| match &e.value {
            OccurrenceBodyElement::OccurrenceUsage(occ) => Some(&occ.value),
            _ => None,
        })
        .expect("snapshot should parse in timeslice body");
    assert_eq!(
        snapshot.prefix.portion().map(|node| node.value),
        Some(sysml_v2_parser::OccurrencePortionKind::Snapshot)
    );
}

#[test]
fn then_timeslice_and_specialized_snapshot_parse_inside_individual_part() {
    let input = "package P {\nindividual part def MissionIndividual :> Mission;\nindividual part mission : MissionIndividual {\ntimeslice ingress {\nassert constraint { ready }\nsnapshot atIngress :> system : MissionSystem {\nattribute missionTime = 0;\n}\n}\nthen timeslice liftoff {\nsnapshot atT0 :> system : MissionSystem {\nattribute missionTime = 1;\n}\n}\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let usage = match &elements[1].value {
        PackageBodyElement::PartUsage(usage) => &usage.value,
        _ => panic!("expected part usage"),
    };
    let PartUsageBody::Brace { elements, .. } = &usage.body else {
        panic!("expected part usage body");
    };
    let occurrences: Vec<_> = elements
        .iter()
        .filter_map(|e| match &e.value {
            PartUsageBodyElement::OccurrenceUsage(occ) => Some(&occ.value),
            _ => None,
        })
        .collect();
    assert_eq!(occurrences.len(), 2);
    assert_eq!(
        occurrences[0].prefix.portion().map(|node| node.value),
        Some(sysml_v2_parser::OccurrencePortionKind::Timeslice)
    );
    assert!(occurrences[0].subsets.is_none());
    assert_eq!(
        occurrences[1].prefix.portion().map(|node| node.value),
        Some(sysml_v2_parser::OccurrencePortionKind::Timeslice)
    );

    let OccurrenceUsageBody::Brace {
        elements: ingress_elements,
        ..
    } = &occurrences[0].body
    else {
        panic!("expected ingress timeslice body");
    };
    // An occurrence body member can no longer degrade to opaque text: the scope has no such
    // variant, so the structured member below is the only representation available.
    let assert_constraint = ingress_elements
        .iter()
        .find_map(|e| match &e.value {
            OccurrenceBodyElement::AssertConstraint(member) => Some(&member.value),
            _ => None,
        })
        .expect("assert constraint should parse as a structured occurrence member");
    let ConstraintDefBody::Brace {
        elements: assert_elements,
        ..
    } = &assert_constraint.body
    else {
        panic!("expected assert constraint body");
    };
    assert!(
        assert_elements.iter().any(|e| {
            matches!(
                &e.value,
                ConstraintDefBodyElement::Expression(expr)
                    if matches!(&expr.value, Expression::FeatureRef(name)
                        if result.document.qualified_reference(*name).is_some_and(|r| r.authored_text() == "ready"))
            )
        }),
        "assert constraint body should preserve the `ready` expression"
    );

    let OccurrenceUsageBody::Brace { elements, .. } = &occurrences[1].body else {
        panic!("expected timeslice body");
    };
    let snapshot = elements
        .iter()
        .find_map(|e| match &e.value {
            OccurrenceBodyElement::OccurrenceUsage(occ) => Some(&occ.value),
            _ => None,
        })
        .expect("snapshot should parse in then timeslice body");
    assert!(snapshot.subsets.is_some());
    assert!(snapshot.type_name.is_some());
}

#[test]
fn anonymous_individual_parts_and_body_trailing_subsets_parse() {
    let input = "package P {\npart def Mission {\npart crew;\n}\npart apolloProgram {\npart apollo1 : Mission {\nindividual part : 'Gus Grissom' :> crew;\nindividual part : 'Ed White' :> crew;\n} :> missions;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let program = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartUsage(usage)
                if usage
                    .value
                    .name
                    .and_then(|n| result.document.declaration_name(n))
                    == Some("apolloProgram") =>
            {
                Some(&usage.value)
            }
            _ => None,
        })
        .expect("expected program part usage");
    let PartUsageBody::Brace { elements, .. } = &program.body else {
        panic!("expected program body");
    };
    let apollo1 = match &elements[0].value {
        PartUsageBodyElement::PartUsage(usage) => &usage.value,
        _ => panic!("expected nested part usage"),
    };
    assert!(apollo1.subsets.is_some());

    let PartUsageBody::Brace { elements, .. } = &apollo1.body else {
        panic!("expected nested mission body");
    };
    let crew_members: Vec<_> = elements
        .iter()
        .filter_map(|e| match &e.value {
            PartUsageBodyElement::PartUsage(usage)
                if usage.value.prefix.individual_span().is_some() =>
            {
                Some(&usage.value)
            }
            _ => None,
        })
        .collect();
    assert_eq!(crew_members.len(), 2);
    assert!(crew_members[0].name.is_none());
    assert!(crew_members[0].typing.is_some());
    assert!(crew_members[0].subsets.is_some());
}

#[test]
fn exhibit_state_supports_trailing_redefinition_after_body() {
    let input = "package P {\npart def Mission {\nexhibit state phases {\nstate initial : Initial;\ntransition first initial then initial;\n} :>> missionPhases;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let mission = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(def)
                if def
                    .value
                    .identification
                    .name
                    .and_then(|n| result.document.declaration_name(n))
                    == Some("Mission") =>
            {
                Some(&def.value)
            }
            _ => None,
        })
        .expect("expected Mission part def");
    let PartDefBody::Brace { elements, .. } = &mission.body else {
        panic!("expected part body");
    };
    let exhibit = elements
        .iter()
        .find_map(|e| match &e.value {
            PartDefBodyElement::ExhibitState(exhibit) => Some(&exhibit.value),
            _ => None,
        })
        .expect("exhibit state should be present");
    assert!(exhibit.redefines.is_some());
}

#[test]
fn exhibit_state_supports_parallel_modifier() {
    // GH-17: `exhibit state` shares `StateUsageBody` with plain `state` usages (OMG spec
    // §8.2.2.18.2), so `parallel` is legal here too -- mirrors the `vehicleStates` example in
    // Annex `5-State-based Behavior-2.sysml`.
    let input = "package P {\npart def Vehicle {\nexhibit state vehicleStates parallel {\nstate operatingStates;\nstate healthStates;\n}\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let vehicle = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(def)
                if def
                    .value
                    .identification
                    .name
                    .and_then(|n| result.document.declaration_name(n))
                    == Some("Vehicle") =>
            {
                Some(&def.value)
            }
            _ => None,
        })
        .expect("expected Vehicle part def");
    let PartDefBody::Brace { elements, .. } = &vehicle.body else {
        panic!("expected part body");
    };
    let exhibit = elements
        .iter()
        .find_map(|e| match &e.value {
            PartDefBodyElement::ExhibitState(exhibit) => Some(&exhibit.value),
            _ => None,
        })
        .expect("exhibit state should be present");
    assert_eq!(
        exhibit
            .name
            .and_then(|n| result.document.declaration_name(n)),
        Some("vehicleStates")
    );
    let StateDefBody::Brace { elements, .. } = &exhibit.body else {
        panic!("expected exhibit state body");
    };
    assert_eq!(elements.len(), 2);
}

#[test]
fn exhibit_state_supports_occurrence_usage_prefix_modifiers() {
    // GH-27: `exhibit_state` was a narrow, bespoke parser never wired up to the shared
    // prefix/specialization machinery `state_usage` already uses, so modifiers legal on plain
    // `state` (visibility, `abstract`, `:>` subsets, multiplicity, `ordered`) were rejected on
    // `exhibit state`. Rebuilt on the same shared helpers `state_usage` composes
    // (`visibility_prefix`, `abstract`/`ref`, `specialization_clauses`, `optional_typings`,
    // `multiplicity_node`, `skip_usage_feature_modifiers`) so the two stay in sync.
    let input = "package P {\npart def Base {\nstate baseStates;\n}\npart def Vehicle {\nabstract exhibit state abstractStates { state a; }\nprivate exhibit state privateStates { state a; }\nexhibit state subsetStates :> Base::baseStates { state a; }\nexhibit state multiplicityStates[1] { state a; }\nexhibit state orderedStates ordered { state a; }\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let vehicle = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(def)
                if def
                    .value
                    .identification
                    .name
                    .and_then(|n| result.document.declaration_name(n))
                    == Some("Vehicle") =>
            {
                Some(&def.value)
            }
            _ => None,
        })
        .expect("expected Vehicle part def");
    let PartDefBody::Brace { elements, .. } = &vehicle.body else {
        panic!("expected part body");
    };
    let exhibits: Vec<_> = elements
        .iter()
        .filter_map(|e| match &e.value {
            PartDefBodyElement::ExhibitState(exhibit) => Some(&exhibit.value),
            _ => None,
        })
        .collect();
    assert_eq!(exhibits.len(), 5, "expected 5 exhibit state usages");

    let abstract_states = exhibits
        .iter()
        .find(|e| {
            e.name.and_then(|n| result.document.declaration_name(n)) == Some("abstractStates")
        })
        .expect("abstractStates");
    assert!(abstract_states.is_abstract);

    let private_states = exhibits
        .iter()
        .find(|e| e.name.and_then(|n| result.document.declaration_name(n)) == Some("privateStates"))
        .expect("privateStates");
    assert_eq!(
        private_states.membership.visibility,
        Some(sysml_v2_parser::ast::Visibility::Private)
    );

    let subset_states = exhibits
        .iter()
        .find(|e| e.name.and_then(|n| result.document.declaration_name(n)) == Some("subsetStates"))
        .expect("subsetStates");
    assert!(subset_states.subsets.is_some());

    let multiplicity_states = exhibits
        .iter()
        .find(|e| {
            e.name.and_then(|n| result.document.declaration_name(n)) == Some("multiplicityStates")
        })
        .expect("multiplicityStates");
    assert!(multiplicity_states.multiplicity.is_some());

    let ordered_states = exhibits
        .iter()
        .find(|e| e.name.and_then(|n| result.document.declaration_name(n)) == Some("orderedStates"))
        .expect("orderedStates");
    let StateDefBody::Brace { elements, .. } = &ordered_states.body else {
        panic!("expected exhibit state body");
    };
    assert_eq!(elements.len(), 1);
}

#[test]
fn state_and_exhibit_state_support_direction_derived_individual_prefixes() {
    // GH-45: `state_usage`/`exhibit_state` only implemented the visibility/`abstract`/`ref`
    // slice of BNF `RefPrefix`/`OccurrenceUsagePrefix` (§8.2.2.9.2, §8.2.2.18.2); `direction`
    // (`in`/`out`/`inout`), `derived`, and `individual` were rejected on both, even though the
    // same helpers (`crate::parser::attribute::direction_prefix` and the `derived`/`individual`
    // keyword pattern) already back `part`/`item`/`attribute` usages elsewhere in this parser.
    let input = "package P {\npart def Vehicle {\nin state directionStates { state a; }\nderived state derivedStates { state a; }\nindividual state individualStates { state a; }\nin exhibit state directionExhibit { state a; }\nderived exhibit state derivedExhibit { state a; }\nindividual exhibit state individualExhibit { state a; }\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let vehicle = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(def)
                if def
                    .value
                    .identification
                    .name
                    .and_then(|n| result.document.declaration_name(n))
                    == Some("Vehicle") =>
            {
                Some(&def.value)
            }
            _ => None,
        })
        .expect("expected Vehicle part def");
    let PartDefBody::Brace { elements, .. } = &vehicle.body else {
        panic!("expected part body");
    };

    let state_usages: Vec<_> = elements
        .iter()
        .filter_map(|e| match &e.value {
            PartDefBodyElement::StateUsage(state) => Some(&state.value),
            _ => None,
        })
        .collect();
    let direction_states = state_usages
        .iter()
        .find(|s| {
            s.name.and_then(|n| result.document.declaration_name(n)) == Some("directionStates")
        })
        .expect("directionStates");
    assert_eq!(direction_states.direction, Some(InOut::In));
    let derived_states = state_usages
        .iter()
        .find(|s| s.name.and_then(|n| result.document.declaration_name(n)) == Some("derivedStates"))
        .expect("derivedStates");
    assert!(derived_states.is_derived);
    let individual_states = state_usages
        .iter()
        .find(|s| {
            s.name.and_then(|n| result.document.declaration_name(n)) == Some("individualStates")
        })
        .expect("individualStates");
    assert!(individual_states.is_individual);

    let exhibits: Vec<_> = elements
        .iter()
        .filter_map(|e| match &e.value {
            PartDefBodyElement::ExhibitState(exhibit) => Some(&exhibit.value),
            _ => None,
        })
        .collect();
    let direction_exhibit = exhibits
        .iter()
        .find(|e| {
            e.name.and_then(|n| result.document.declaration_name(n)) == Some("directionExhibit")
        })
        .expect("directionExhibit");
    assert_eq!(direction_exhibit.direction, Some(InOut::In));
    let derived_exhibit = exhibits
        .iter()
        .find(|e| {
            e.name.and_then(|n| result.document.declaration_name(n)) == Some("derivedExhibit")
        })
        .expect("derivedExhibit");
    assert!(derived_exhibit.is_derived);
    let individual_exhibit = exhibits
        .iter()
        .find(|e| {
            e.name.and_then(|n| result.document.declaration_name(n)) == Some("individualExhibit")
        })
        .expect("individualExhibit");
    assert!(individual_exhibit.is_individual);
}

#[test]
fn exhibit_state_body_accepts_requirement_usage_members() {
    let input = "package P {\nrequirement def Goal;\nstate def MissionPhase;\npart def Mission {\nrequirement goals[1..*] : Goal;\nexhibit state phases : MissionPhase {\nrequirement goToMoon : Goal {\ndoc /* Example */\n} :> goals;\nstate launch;\n}\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let mission = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(def)
                if def
                    .value
                    .identification
                    .name
                    .and_then(|n| result.document.declaration_name(n))
                    == Some("Mission") =>
            {
                Some(&def.value)
            }
            _ => None,
        })
        .expect("expected Mission part def");
    let PartDefBody::Brace { elements, .. } = &mission.body else {
        panic!("expected part body");
    };
    let exhibit = elements
        .iter()
        .find_map(|e| match &e.value {
            PartDefBodyElement::ExhibitState(exhibit) => Some(&exhibit.value),
            _ => None,
        })
        .expect("exhibit state should be present");
    let StateDefBody::Brace { elements, .. } = &exhibit.body else {
        panic!("expected exhibit state body");
    };
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, StateDefBodyElement::RequirementUsage(_))));
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, StateDefBodyElement::StateUsage(_))));
}

#[test]
fn part_usage_accepts_multiplicity_before_type() {
    let input = "package P {\npart def System {\npart spaceSuits[2] : ExtravehicularMobilityUnit :> constituentSystems;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let system = match &elements[0].value {
        PackageBodyElement::PartDef(def) => &def.value,
        _ => panic!("expected part def"),
    };
    let PartDefBody::Brace { elements, .. } = &system.body else {
        panic!("expected part body");
    };
    let suit = match &elements[0].value {
        PartDefBodyElement::PartUsage(usage) => &usage.value,
        _ => panic!("expected part usage"),
    };
    assert_eq!(
        suit.name.and_then(|n| result.document.declaration_name(n)),
        Some("spaceSuits")
    );
    assert!(suit.multiplicity.is_some());
    assert!(suit.typing.is_some());
    assert!(suit.subsets.is_some());
}

/// spec42#100 form 3 / parser#132. BNF `FeatureSpecializationPart` also permits `MultiplicityPart
/// FeatureSpecialization*`, so a nested `action` usage whose multiplicity precedes the typing
/// (`action subfunctions[*] : Function :>> subactions;`, Apollo 11 `abstract action def Function`,
/// SysML v2.0 §8.2.2.6.5) must land the typing, multiplicity, and redefinition on the typed
/// `ActionUsage` rather than recovering as `recovered_action_body_element`.
#[test]
fn action_usage_accepts_multiplicity_before_type_and_redefinition() {
    let input = "package P {\nabstract action def Function {\naction subfunctions[*] : Function :>> subactions;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let function = match &elements[0].value {
        PackageBodyElement::ActionDef(def) => &def.value,
        _ => panic!("expected action def"),
    };
    let ActionDefBody::Brace { elements, .. } = &function.body else {
        panic!("expected action body");
    };
    let usage = match &elements[0].value {
        sysml_v2_parser::ActionDefBodyElement::ActionUsage(usage) => &usage.value,
        other => panic!("expected action usage, got {other:?}"),
    };
    assert_eq!(
        usage.name.and_then(|n| result.document.declaration_name(n)),
        Some("subfunctions")
    );
    assert!(
        usage.multiplicity.is_some(),
        "multiplicity should be retained"
    );
    assert!(
        usage.typing.is_some(),
        "typing after multiplicity should be retained"
    );
    assert!(
        usage.redefines.is_some(),
        "`:>>` redefinition should be retained"
    );
}

/// Negative control for the form above: a multiplicity-first `action` usage with a dangling
/// typing colon is still malformed and must recover, not be accepted as an empty typing.
#[test]
fn action_usage_multiplicity_before_missing_type_still_recovers() {
    let input = "package P {\naction def Function {\naction subfunctions[*] : ;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        !result.errors.is_empty(),
        "a dangling typing colon should still be diagnosed"
    );
}

/// spec42#100 form 4 / parser#132. An `entry`/`do`/`exit` action body is a SysML `ActionBody`
/// (`PerformActionUsage ... ActionBody`), so the action-flow statements `first <node>;` and
/// `then <target>;` are legal in it -- as in Apollo 11's `state def PrepareForMissionPhase`'s
/// `do action prepareForMissionPhaseOperations { first start; then action ...; then done; }`.
/// They previously recovered as `recovered_state_body_element` because the body was parsed with
/// the plain `state` body grammar.
#[test]
fn do_action_body_accepts_first_and_then_action_flow_statements() {
    let input = "package P {\nstate def PrepareForMissionPhase :> Phase {\ndo action prepareForMissionPhaseOperations {\nfirst start;\nthen action loadConsumablesAndPropellants : LoadConsumablesAndPropellants;\nthen done;\n}\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let state = match &elements[0].value {
        PackageBodyElement::StateDef(def) => &def.value,
        _ => panic!("expected state def"),
    };
    let StateDefBody::Brace { elements, .. } = &state.body else {
        panic!("expected state body");
    };
    let do_action = elements
        .iter()
        .find_map(|e| match &e.value {
            StateDefBodyElement::Do(node) => Some(&node.value),
            _ => None,
        })
        .expect("do action should be present");
    let StateDefBody::Brace { elements, .. } = &do_action.body else {
        panic!("expected do action body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, StateDefBodyElement::FirstStmt(_))),
        "`first start;` should lower to a FirstStmt member"
    );
    assert_eq!(
        elements
            .iter()
            .filter(|e| matches!(e.value, StateDefBodyElement::ThenAction(_)))
            .count(),
        2,
        "both `then` statements should lower to ThenAction members"
    );
}

/// spec42#100 form 4 / parser#134. `do action <name> { <body> }` with a body but no `: Type` /
/// `:>>` clause was parsed as an `action_reference` -- so `prepareForMissionPhaseOperations`
/// looked unresolved and every feature chain through `.<name>.` cascaded. `state_behavior_action_
/// target` now routes a name immediately followed by `{` to `declared_name` when the `action`
/// keyword was written.
#[test]
fn do_action_with_body_declares_the_name_rather_than_referencing_it() {
    let input = "package P {\nstate def PrepareForMissionPhase {\ndo action prepareForMissionPhaseOperations {\nfirst start;\nthen done;\n}\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let state = match &elements[0].value {
        PackageBodyElement::StateDef(def) => &def.value,
        _ => panic!("expected state def"),
    };
    let StateDefBody::Brace { elements, .. } = &state.body else {
        panic!("expected state body");
    };
    let do_action = elements
        .iter()
        .find_map(|e| match &e.value {
            StateDefBodyElement::Do(node) => Some(&node.value),
            _ => None,
        })
        .expect("do action should be present");
    assert!(
        do_action.action_reference.is_none(),
        "a body-carrying `do action <name>` is not a reference"
    );
    assert_eq!(
        do_action
            .declared_name
            .and_then(|n| result.document.declaration_name(n)),
        Some("prepareForMissionPhaseOperations")
    );
}

/// A bare `do action <name>;` (no body, no typing) stays a referenced action usage.
#[test]
fn bare_do_action_name_still_references() {
    let input = "package P {\nstate def S {\ndo action existingAction;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(result.errors.is_empty(), "unexpected: {:?}", result.errors);
    let RootElement::Package(p) = &result.document.root.elements[0].value else {
        panic!("expected package");
    };
    let PackageBody::Brace { elements, .. } = &p.value.body else {
        panic!("expected brace body");
    };
    let PackageBodyElement::StateDef(def) = &elements[0].value else {
        panic!("expected state def");
    };
    let StateDefBody::Brace { elements, .. } = &def.value.body else {
        panic!("expected state body");
    };
    let do_action = elements
        .iter()
        .find_map(|e| match &e.value {
            StateDefBodyElement::Do(node) => Some(&node.value),
            _ => None,
        })
        .expect("do action present");
    assert!(do_action.declared_name.is_none());
    assert!(do_action.action_reference.is_some());
}

/// Negative control: `first <node>;` is *not* legal in a plain `state def` body (only in the
/// entry/do/exit action bodies), so it must still recover there.
#[test]
fn plain_state_body_still_rejects_bare_first_statement() {
    let input = "package P {\nstate def S {\nfirst start;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        !result.errors.is_empty(),
        "`first start;` in a plain state body should still be diagnosed"
    );
}

/// spec42#100 form 5 / parser#132. A `def`-less `abstract connection` *usage* whose multiplicity
/// precedes the typing (`abstract connection capabilityToGoals[*] : CapabilityToGoalDerivation;`,
/// Apollo 11) previously fell through to `ExtendedLibraryDecl` and surfaced as
/// `unsupported_grammar_form`: `connection_def`'s header text-scan only accepts a typing that
/// comes before the multiplicity, and `connection_usage_member` did not accept `abstract`. The
/// usage parser now models the `abstract` prefix (`RefPrefix.isAbstract`).
#[test]
fn abstract_connection_usage_accepts_multiplicity_before_type() {
    let input =
        "package P {\nabstract connection capabilityToGoals[*] : CapabilityToGoalDerivation;\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let usage = match &elements[0].value {
        PackageBodyElement::ConnectionUsage(usage) => &usage.value,
        other => panic!("expected connection usage, got {other:?}"),
    };
    assert!(usage.is_abstract, "`abstract` prefix should be retained");
    assert!(
        usage.multiplicity.is_some(),
        "multiplicity should be retained"
    );
    assert!(
        usage.type_reference.is_some(),
        "typing after multiplicity should be retained"
    );
}

/// Negative control: an `abstract` multiplicity-first connection usage with a dangling typing
/// colon is still malformed and must recover.
#[test]
fn abstract_connection_usage_multiplicity_before_missing_type_still_recovers() {
    let input = "package P {\nabstract connection capabilityToGoals[*] : ;\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        !result.errors.is_empty(),
        "a dangling typing colon should still be diagnosed"
    );
}

/// spec42#100 form 1 / parser#132. A keyword-less feature usage with an explicit typing and
/// value (`launchVehicle : SaturnV = apollo11Mission…launchVehicle;`, Apollo 11 `analysis` body;
/// official Vehicle Analysis Demo) previously recovered as `recovered_use_case_body_element`.
#[test]
fn analysis_body_accepts_bare_typed_valued_feature() {
    let input = "package P {\nanalysis Apollo11MissionDeltaVBudgetAnalysis {\nlaunchVehicle : SaturnV = apollo11Mission.apollo11MissionSystem.launchVehicle;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let analysis = match &elements[0].value {
        PackageBodyElement::AnalysisCaseUsage(node) => &node.value,
        other => panic!("expected analysis case usage, got {other:?}"),
    };
    let Body::Brace { elements, .. } = &analysis.body else {
        panic!("expected analysis body");
    };
    let usage = match &elements[0].value {
        UseCaseDefBodyElement::DefaultReferenceUsage(node) => &node.value,
        other => panic!("expected default reference usage, got {other:?}"),
    };
    assert_eq!(
        usage.name.and_then(|n| result.document.declaration_name(n)),
        Some("launchVehicle")
    );
    assert!(
        usage.typing.is_some(),
        "`: SaturnV` typing should be retained"
    );
    assert!(usage.value.is_some(), "the `= …` value should be retained");
}

/// Negative control: a bare `name;` in a use-case-family body stays a result expression, not a
/// (nameless) feature usage.
#[test]
fn analysis_body_bare_name_is_still_an_expression() {
    let input = "package P {\nanalysis A {\nvehicle;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let analysis = match &elements[0].value {
        PackageBodyElement::AnalysisCaseUsage(node) => &node.value,
        other => panic!("expected analysis case usage, got {other:?}"),
    };
    let Body::Brace { elements, .. } = &analysis.body else {
        panic!("expected analysis body");
    };
    assert!(
        matches!(elements[0].value, UseCaseDefBodyElement::Expression(_)),
        "bare `vehicle;` should stay an expression, got {:?}",
        elements[0].value
    );
}

/// spec42#100 form 2 / parser#132. `return :>>` in a calc/constraint body parsed its leading
/// redefinition target with a single-identifier `name()`, truncating `ISQ::power` to `ISQ` and
/// then failing (`recovered_calc_body_element`). The Apollo 11 form additionally chains
/// `->select { … }->collect { … }->sum()` as the value.
#[test]
fn calc_return_redefine_accepts_qualified_target_and_chained_select_collect() {
    let input = "package P {\ncalc def PowerBudget {\nreturn :>> ISQ::power = system.subparts->select { in sys : Part; sys istype PowerProvider }->collect { in sys : PowerProvider; sys.powerGenerated }->sum();\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let calc = match &elements[0].value {
        PackageBodyElement::CalcDef(node) => &node.value,
        other => panic!("expected calc def, got {other:?}"),
    };
    let CalcDefBody::Brace { elements, .. } = &calc.body else {
        panic!("expected calc body");
    };
    let ret = match &elements[0].value {
        CalcDefBodyElement::ReturnDecl(node) => &node.value,
        other => panic!("expected return decl, got {other:?}"),
    };
    assert!(ret.is_redefine, "the `:>>` form should set is_redefine");
    assert!(
        ret.name.is_none(),
        "the `:>>` target is not a declaration name"
    );
    let redefines = ret
        .redefines
        .as_ref()
        .expect("redefinition target retained");
    let target = result
        .document
        .qualified_reference(redefines.value.target[0])
        .expect("target resolves");
    assert_eq!(target.authored_text(), "ISQ::power");
    assert!(
        ret.value.is_some(),
        "the chained select/collect value should be retained"
    );
}

/// spec42#100 form 2 / parser#134. The Apollo 11 `rollupPowerGeneration` calc uses the
/// *anonymous subsetting* spelling `return :> ISQ::power = …`, not `:>>`. `return_decl`'s
/// name/anonymous dispatch only excused a bare `:` from `name()`, so `return :>` fell through to
/// `name()`, which failed to parse `:>` and recovered the whole body as
/// `recovered_calc_body_element` -- with or without the chained `->select { … }->collect { … }`
/// value.
#[test]
fn calc_return_anonymous_subsetting_accepts_qualified_target_and_chained_select_collect() {
    let input = "package P {\ncalc def rollupPowerGeneration {\nin system : Anything;\nreturn :> ISQ::power = system.subparts->select { in sys : Part; sys istype PowerProvider }->collect { in sys : PowerProvider; sys.powerGenerated }->sum();\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let calc = match &elements[0].value {
        PackageBodyElement::CalcDef(node) => &node.value,
        other => panic!("expected calc def, got {other:?}"),
    };
    let CalcDefBody::Brace { elements, .. } = &calc.body else {
        panic!("expected calc body");
    };
    let ret = match &elements[1].value {
        CalcDefBodyElement::ReturnDecl(node) => &node.value,
        other => panic!("expected return decl, got {other:?}"),
    };
    assert!(ret.is_subsetting, "the `:>` form should set is_subsetting");
    assert!(!ret.is_redefine);
    assert!(
        ret.name.is_none(),
        "the `:>` target is not a declaration name"
    );
    let target = result
        .document
        .qualified_reference(ret.type_name.expect("subsetting target retained"))
        .expect("target resolves");
    assert_eq!(target.authored_text(), "ISQ::power");
    assert!(
        ret.value.is_some(),
        "the chained select/collect value should be retained"
    );
}

/// The anonymous `:>` return declaration round-trips through emit.
#[test]
fn calc_return_anonymous_subsetting_round_trips() {
    let src = "package P { calc def C { return :> ISQ::power = computePower(); } }";
    let doc = parse(src).expect("parses");
    let emitted = sysml_v2_parser::emit_sysml(&doc).expect("emits");
    let reparsed = parse_with_diagnostics(&emitted);
    assert!(
        reparsed.errors.is_empty(),
        "emitted `{emitted}` should re-parse cleanly: {:?}",
        reparsed.errors
    );
    assert!(
        emitted.contains("return :> ISQ::power"),
        "emitted form keeps the `:>` spelling: {emitted}"
    );
}

/// Negative control: `return :>>` with no target at all is still malformed.
#[test]
fn calc_return_redefine_without_target_still_recovers() {
    let input = "package P {\ncalc def C {\nreturn :>> = x;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        !result.errors.is_empty(),
        "`return :>> = x;` should still be diagnosed"
    );
}

/// Negative control: anonymous `return :>` with no subsetting target is still malformed.
#[test]
fn calc_return_anonymous_subsetting_without_target_still_recovers() {
    let input = "package P {\ncalc def C {\nreturn :> = x;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        !result.errors.is_empty(),
        "`return :> = x;` should still be diagnosed"
    );
}

/// The single-identifier `return :>> <name>` spelling keeps working: the target moves from
/// `ReturnDecl::name` to `ReturnDecl::redefines`, and the declaration still round-trips.
#[test]
fn calc_return_redefine_single_name_round_trips() {
    let src = "package P { calc def C { return :>> result = computeResult(); } }";
    let doc = parse(src).expect("parses");
    let emitted = sysml_v2_parser::emit_sysml(&doc).expect("emits");
    let reparsed = parse_with_diagnostics(&emitted);
    assert!(
        reparsed.errors.is_empty(),
        "emitted `{emitted}` should re-parse cleanly: {:?}",
        reparsed.errors
    );

    let pkg = match &doc.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let calc = match &elements[0].value {
        PackageBodyElement::CalcDef(node) => &node.value,
        other => panic!("expected calc def, got {other:?}"),
    };
    let CalcDefBody::Brace { elements, .. } = &calc.body else {
        panic!("expected calc body");
    };
    let ret = match &elements[0].value {
        CalcDefBodyElement::ReturnDecl(node) => &node.value,
        other => panic!("expected return decl, got {other:?}"),
    };
    assert!(ret.is_redefine);
    assert!(ret.name.is_none());
    let target = doc
        .qualified_reference(ret.redefines.as_ref().expect("redefines").value.target[0])
        .expect("target resolves");
    assert_eq!(target.authored_text(), "result");
}

#[test]
fn rationale_and_refinement_annotations_stay_localized() {
    let input = "package P {\naction def PerformCrewIngress {\nout isCrewAboard: Boolean;\n@Rationale { }\n#refinement dependency PerformCrewIngress to OperationsPackage::TransferCrewToVehicle;\n}\nrequirement def R {\n@Rationale { }\n#refinement dependency 'HLR-R001' to CapabilitiesPackage::DeepSpaceHabitationAndLifeSupport;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "annotations should parse without recovery cascades: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    match &elements[0].value {
        PackageBodyElement::ActionDef(action) => {
            let ActionDefBody::Brace { elements, .. } = &action.value.body else {
                panic!("expected action body");
            };
            // `#refinement dependency X to Y;` is `PrefixMetadataAnnotation` + `Dependency`,
            // both members of `ActionBodyItem → NonBehaviorBodyItem → DefinitionMember`. It was
            // one opaque `Annotation` node holding the copied text `refinement dependency
            // PerformCrewIngress to ...`; now the tag and the dependency are separate typed
            // siblings and the dependency's endpoints are real references.
            assert!(elements.iter().any(|e| matches!(
                e.value,
                sysml_v2_parser::ActionDefBodyElement::MetadataKeywordUsage(_)
            )));
            assert!(elements.iter().any(|e| matches!(
                e.value,
                sysml_v2_parser::ActionDefBodyElement::Dependency(_)
            )));
        }
        _ => panic!("expected action def"),
    }
    match &elements[1].value {
        PackageBodyElement::RequirementDef(req) => {
            let RequirementDefBody::Brace { elements, .. } = &req.value.body else {
                panic!("expected requirement body");
            };
            assert!(elements
                .iter()
                .any(|e| matches!(e.value, RequirementDefBodyElement::MetadataKeywordUsage(_))));
            assert!(elements
                .iter()
                .any(|e| matches!(e.value, RequirementDefBodyElement::Dependency(_))));
        }
        _ => panic!("expected requirement def"),
    }
}

#[test]
fn quoted_requirement_identifier_parses() {
    let input = "package P {\nrequirement def <'HLR-R001'> CrewReturnSafetyRequirement { }\n}";
    let (doc, elements) = package_elements(input);
    match &elements[0].value {
        PackageBodyElement::RequirementDef(req) => {
            assert_eq!(
                req.value
                    .identification
                    .short_name
                    .and_then(|n| doc.declaration_name(n)),
                Some("'HLR-R001'")
            );
            assert_eq!(
                req.value
                    .identification
                    .name
                    .and_then(|n| doc.declaration_name(n)),
                Some("CrewReturnSafetyRequirement")
            );
        }
        other => panic!("expected requirement def, got {other:?}"),
    }
}

#[test]
fn mission_capability_connections_with_trailing_subsets_parse() {
    let input = "package P {\npart def Mission {\nconnection : CapabilityToGoalDerivation {\nend capa ::> toolDevelopment;\nend goal ::> deploy;\n} :> capabilityToGoals;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let mission = match &elements[0].value {
        PackageBodyElement::PartDef(def) => &def.value,
        _ => panic!("expected part def"),
    };
    let PartDefBody::Brace { elements, .. } = &mission.body else {
        panic!("expected part body");
    };
    let connection = elements
        .iter()
        .find_map(|e| match &e.value {
            PartDefBodyElement::Connection(connection) => Some(&connection.value),
            _ => None,
        })
        .expect("expected structured connection member in part body");
    assert!(connection.type_reference.is_some());
    assert!(connection.subsets.is_some());
    let ConnectionDefBody::Brace { elements, .. } = &connection.body else {
        panic!("expected connection body");
    };
    assert_eq!(
        elements
            .iter()
            .filter(|el| matches!(el.value, ConnectionDefBodyElement::EndDecl(_)))
            .count(),
        2
    );
}

#[test]
fn part_definition_comment_members_parse_structurally() {
    let input =
        "package P {\npart def Mission {\ncomment source /* https://example.test/source */\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let mission = match &elements[0].value {
        PackageBodyElement::PartDef(def) => &def.value,
        _ => panic!("expected part def"),
    };
    let PartDefBody::Brace { elements, .. } = &mission.body else {
        panic!("expected part body");
    };
    let comment = elements
        .iter()
        .find_map(|e| match &e.value {
            PartDefBodyElement::Annotating(AnnotatingMember::Comment(comment)) => {
                Some(&comment.value)
            }
            _ => None,
        })
        .expect("expected structured comment member in part body");
    assert_eq!(
        comment
            .identification
            .as_ref()
            .and_then(|id| id.name.and_then(|n| result.document.declaration_name(n))),
        Some("source")
    );
    assert!(result
        .document
        .comment_body(comment.body)
        .is_some_and(|text| text.contains("https://example.test/source")));
}

#[test]
fn system_part_body_accepts_named_interface_and_individual_members() {
    let input = "package P {\npart def Apollo11MissionSystem :> SystemOfSystems {\nindividual part launchVehicle : 'SA-506' :> constituentSystems;\npart spacecraft : ApolloSpacecraft :> constituentSystems {\nindividual part csm : 'CSM-107' :>> commandServiceModule;\nindividual part lm : 'LM-5' :>> lunarModule;\n}\npart spaceSuits[2] : ExtravehicularMobilityUnit :> constituentSystems;\ninterface lvToPayload : LVPayloadInterface connect launchVehicle.instrumentUnit.payloadInterfacePort to spacecraft.spacecraftLMAdapter.launchVehicleInterfacePort;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let system = match &elements[0].value {
        PackageBodyElement::PartDef(def) => &def.value,
        _ => panic!("expected part def"),
    };
    let PartDefBody::Brace { elements, .. } = &system.body else {
        panic!("expected part body");
    };
    let spacecraft = match &elements[1].value {
        PartDefBodyElement::PartUsage(usage) => &usage.value,
        _ => panic!("expected part usage"),
    };
    let PartUsageBody::Brace {
        elements: spacecraft_elements,
        ..
    } = &spacecraft.body
    else {
        panic!("expected nested part body");
    };
    let csm = match &spacecraft_elements[0].value {
        PartUsageBodyElement::PartUsage(usage) => &usage.value,
        _ => panic!("expected individual part usage"),
    };
    assert_eq!(
        csm.name.and_then(|n| result.document.declaration_name(n)),
        Some("csm")
    );
    assert!(csm.prefix.individual_span().is_some());
    assert!(csm.typing.is_some());
    assert!(csm.redefines.is_some());
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, PartDefBodyElement::InterfaceUsage(_))));
}

#[test]
fn part_defs_accept_multiple_specialization_targets() {
    let input = "package P {\npart def ApolloSpacecraft :> System, Spacecraft {\npart commandServiceModule;\n}\npart def ExtravehicularMobilityUnit :> System, EVASystem;\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    assert!(matches!(elements[0].value, PackageBodyElement::PartDef(_)));
    assert!(matches!(elements[1].value, PackageBodyElement::PartDef(_)));
}

#[test]
fn part_redefinition_value_parses_parenthesized_tuple_of_engines() {
    let input = "package P {\npart def SII :> RocketStage {\npart engine1;\npart engine2;\npart engine3;\npart engine4;\npart engine5;\npart :>> engines[5] = (engine1, engine2, engine3, engine4, engine5);\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let sii = match &elements[0].value {
        PackageBodyElement::PartDef(def) => &def.value,
        _ => panic!("expected part def"),
    };
    let PartDefBody::Brace { elements, .. } = &sii.body else {
        panic!("expected part body");
    };
    let engines = elements
        .iter()
        .find_map(|e| match &e.value {
            PartDefBodyElement::PartUsage(p) if p.value.redefines.is_some() => Some(&**p),
            _ => None,
        })
        .expect("engines part usage");
    assert!(engines.value.multiplicity.is_some());
    let value = engines
        .value
        .value
        .as_ref()
        .expect("tuple value should parse");
    let Expression::Sequence { operands, .. } = &value.value.expression.value else {
        panic!(
            "expected Expression::Sequence, got {:?}",
            value.value.expression.value
        );
    };
    let items: Vec<_> = operands
        .value
        .elements
        .iter()
        .map(|element| &element.expression)
        .collect();
    assert_eq!(items.len(), 5);
    let expected = ["engine1", "engine2", "engine3", "engine4", "engine5"];
    for (i, name) in expected.iter().enumerate() {
        assert!(
            matches!(&items[i].value, Expression::FeatureRef(s)
                if result.document.qualified_reference(*s).is_some_and(|r| r.authored_text() == *name)),
            "element {i}: expected FeatureRef({name:?}), got {:?}",
            items[i].value
        );
    }
}

#[test]
fn part_def_attribute_redefinition_usage_keeps_redefines_and_value() {
    let input = "package P {\npart def RocketStage {\nattribute propellantMass :> ISQ::mass;\nattribute dryMass :> ISQ::mass;\n}\npart def S_IC :> RocketStage {\nattribute :>> propellantMass = 2077000 [kg];\nattribute :>> dryMass = 137000 [kg];\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let sic = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(def)
                if def
                    .value
                    .identification
                    .name
                    .and_then(|n| result.document.declaration_name(n))
                    == Some("S_IC") =>
            {
                Some(&def.value)
            }
            _ => None,
        })
        .expect("expected S_IC part def");
    let PartDefBody::Brace { elements, .. } = &sic.body else {
        panic!("expected part body");
    };

    let attrs: Vec<_> = elements
        .iter()
        .filter_map(|e| match &e.value {
            PartDefBodyElement::AttributeUsage(attr) => Some(&attr.value),
            _ => None,
        })
        .collect();
    assert_eq!(
        attrs.len(),
        2,
        "expected both attribute redefinitions as usages"
    );
    let redefine_name = |attribute: &sysml_v2_parser::ast::AttributeUsage| {
        let target = attribute
            .redefines
            .as_ref()
            .and_then(|relationship| relationship.value.target.first())
            .copied()
            .expect("expected redefinition target");
        result
            .document
            .qualified_reference(target)
            .and_then(|reference| reference.segment_decoded_text(0))
            .expect("expected source-backed redefinition name")
    };
    assert_eq!(redefine_name(attrs[0]).as_ref(), "propellantMass");
    assert!(
        attrs[0].value.is_some(),
        "propellantMass should keep assigned value"
    );
    assert_eq!(redefine_name(attrs[1]).as_ref(), "dryMass");
    assert!(
        attrs[1].value.is_some(),
        "dryMass should keep assigned value"
    );
}

#[test]
fn part_usage_ordered_before_colon_parses_without_recovery() {
    let input =
        "package P {\npart def RocketStage {\npart engines[1..*] ordered : RocketEngine;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let (doc, elements) = package_elements(input);
    let stage = match &elements[0].value {
        PackageBodyElement::PartDef(def) => &def.value,
        _ => panic!("expected part def"),
    };
    let PartDefBody::Brace { elements, .. } = &stage.body else {
        panic!("expected brace body");
    };
    let engines = elements
        .iter()
        .find_map(|e| match &e.value {
            PartDefBodyElement::PartUsage(p)
                if p.value.name.and_then(|n| doc.declaration_name(n)) == Some("engines") =>
            {
                Some(&p.value)
            }
            _ => None,
        })
        .expect("engines part usage");
    assert!(engines.multiplicity_modifiers.is_ordered());
    assert!(engines.typing.is_some());
}

#[test]
fn part_usage_default_null_without_equals_parses() {
    let input = "package P {\npart def MassedComponent { }\npart def X {\npart subcomponents : MassedComponent [*] default null;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );
}

#[test]
fn part_def_body_item_usage_parses() {
    let input = "package P {\npart def Stakeholder {\nitem concerns[*]: Concern;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let (doc, elements) = package_elements(input);
    let stakeholder = match &elements[0].value {
        PackageBodyElement::PartDef(def) => &def.value,
        _ => panic!("expected part def"),
    };
    let PartDefBody::Brace { elements, .. } = &stakeholder.body else {
        panic!("expected brace body");
    };
    let item = elements
        .iter()
        .find_map(|e| match &e.value {
            PartDefBodyElement::ItemUsage(item) => Some(&item.value),
            _ => None,
        })
        .expect("item usage in part def body");
    assert_eq!(
        item.name.and_then(|n| doc.declaration_name(n)),
        Some("concerns")
    );
    assert!(item.multiplicity.is_some());
    assert!(item.type_name.is_some());
}
