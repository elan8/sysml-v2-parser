//! Parser tests: requirement

use sysml_v2_parser::ast::*;
use sysml_v2_parser::parse;

#[test]
fn test_objective_parses_named_typed_requirement_usage() {
    let input = "package P { use case def U { objective missionObjective : MaximizeObjective; } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let use_case = match &elements[0].value {
        PackageBodyElement::UseCaseDef(uc) => &uc.value,
        _ => panic!("expected UseCaseDef"),
    };
    let body_elements = match &use_case.body {
        sysml_v2_parser::ast::UseCaseDefBody::Brace { elements } => elements,
        _ => panic!("expected use case brace body"),
    };
    let objective = body_elements
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::UseCaseDefBodyElement::Objective(o) => Some(&o.value),
            _ => None,
        })
        .expect("objective should be present");
    assert_eq!(objective.requirement.value.name, "missionObjective");
    assert_eq!(
        objective.requirement.value.type_name.as_deref(),
        Some("MaximizeObjective")
    );
    assert!(matches!(
        objective.requirement.value.body,
        sysml_v2_parser::ast::RequirementDefBody::Semicolon
    ));
}

#[test]
fn test_objective_body_preserves_structured_requirement_members() {
    let input = "package P { use case def U { objective verificationObjective { doc /* verify behavior */ require constraint { true; } } } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let use_case = match &elements[0].value {
        PackageBodyElement::UseCaseDef(uc) => &uc.value,
        _ => panic!("expected UseCaseDef"),
    };
    let body_elements = match &use_case.body {
        sysml_v2_parser::ast::UseCaseDefBody::Brace { elements } => elements,
        _ => panic!("expected use case brace body"),
    };
    let objective = body_elements
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::UseCaseDefBodyElement::Objective(o) => Some(&o.value),
            _ => None,
        })
        .expect("objective should be present");
    let req_body_elements = match &objective.requirement.value.body {
        sysml_v2_parser::ast::RequirementDefBody::Brace { elements } => elements,
        _ => panic!("expected objective requirement brace body"),
    };
    assert!(req_body_elements.iter().any(|e| matches!(
        e.value,
        sysml_v2_parser::ast::RequirementDefBodyElement::Doc(_)
    )));
    assert!(req_body_elements.iter().any(|e| matches!(
        e.value,
        sysml_v2_parser::ast::RequirementDefBodyElement::RequireConstraint(_)
    )));
}

#[test]
fn test_objective_typed_semicolon_uses_default_name() {
    let input = "package P { use case def U { objective : MaximizeObjective; } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let use_case = match &elements[0].value {
        PackageBodyElement::UseCaseDef(uc) => &uc.value,
        _ => panic!("expected UseCaseDef"),
    };
    let body_elements = match &use_case.body {
        sysml_v2_parser::ast::UseCaseDefBody::Brace { elements } => elements,
        _ => panic!("expected use case brace body"),
    };
    let objective = body_elements
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::UseCaseDefBodyElement::Objective(o) => Some(&o.value),
            _ => None,
        })
        .expect("objective should be present");
    assert_eq!(objective.requirement.value.name, "objective");
    assert_eq!(
        objective.requirement.value.type_name.as_deref(),
        Some("MaximizeObjective")
    );
}

#[test]
fn test_objective_preserves_visibility_prefix() {
    let input = "package P { use case def U { private objective O { } } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let use_case = match &elements[0].value {
        PackageBodyElement::UseCaseDef(uc) => &uc.value,
        _ => panic!("expected UseCaseDef"),
    };
    let body_elements = match &use_case.body {
        sysml_v2_parser::ast::UseCaseDefBody::Brace { elements } => elements,
        _ => panic!("expected use case brace body"),
    };
    let objective = body_elements
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::UseCaseDefBodyElement::Objective(o) => Some(&o.value),
            _ => None,
        })
        .expect("objective should be present");
    assert!(matches!(
        objective.visibility,
        Some(sysml_v2_parser::ast::Visibility::Private)
    ));
}

#[test]
fn test_objective_body_parses_verify_shorthand_and_explicit_requirement() {
    let input = "package P { use case def U { objective O { verify vehicleMassRequirement; verify requirement vehicleMassRequirement : VehicleMassRequirement; } } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let use_case = match &elements[0].value {
        PackageBodyElement::UseCaseDef(uc) => &uc.value,
        _ => panic!("expected UseCaseDef"),
    };
    let body_elements = match &use_case.body {
        sysml_v2_parser::ast::UseCaseDefBody::Brace { elements } => elements,
        _ => panic!("expected use case brace body"),
    };
    let objective = body_elements
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::UseCaseDefBodyElement::Objective(o) => Some(&o.value),
            _ => None,
        })
        .expect("objective should be present");
    let req_body_elements = match &objective.requirement.value.body {
        sysml_v2_parser::ast::RequirementDefBody::Brace { elements } => elements,
        _ => panic!("expected objective requirement brace body"),
    };
    let shorthand = req_body_elements
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::RequirementDefBodyElement::VerifyRequirement(v)
                if !v.value.explicit_requirement_keyword =>
            {
                Some(&v.value)
            }
            _ => None,
        })
        .expect("shorthand verify should be present");
    assert_eq!(shorthand.target.as_deref(), Some("vehicleMassRequirement"));
    let explicit = req_body_elements
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::RequirementDefBodyElement::VerifyRequirement(v)
                if v.value.explicit_requirement_keyword =>
            {
                Some(&v.value)
            }
            _ => None,
        })
        .expect("explicit verify requirement should be present");
    let explicit_req = explicit
        .requirement
        .as_ref()
        .expect("explicit form should include parsed requirement usage");
    assert_eq!(explicit_req.value.name, "vehicleMassRequirement");
    assert_eq!(
        explicit_req.value.type_name.as_deref(),
        Some("VehicleMassRequirement")
    );
}

#[test]
fn test_verification_return_ref_parses_return_expression() {
    let input = "package P { verification def V { return ref verdictResult { return VerdictKind::unknown; } } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let body_elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected package brace body"),
    };
    let verification = body_elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::VerificationCaseDef(v) => Some(&v.value),
            _ => None,
        })
        .expect("verification def should be present");
    let case_body = match &verification.body {
        UseCaseDefBody::Brace { elements } => elements,
        _ => panic!("expected verification brace body"),
    };
    let return_ref = case_body
        .iter()
        .find_map(|e| match &e.value {
            UseCaseDefBodyElement::ReturnRef(r) => Some(&r.value),
            _ => None,
        })
        .expect("return ref should be present");
    assert_eq!(return_ref.name, "verdictResult");
    let expr = return_ref
        .return_expression
        .as_ref()
        .expect("return expression should be parsed");
    let token = match &expr.value {
        Expression::MemberAccess(_, member) => member.as_str(),
        Expression::FeatureRef(name) => name.as_str(),
        other => panic!("unexpected verdict expression: {other:?}"),
    };
    assert!(
        token.ends_with("unknown"),
        "expected VerdictKind::unknown token, got {token}"
    );
}

#[test]
fn test_analysis_ref_redefinition_is_structured_not_other() {
    let input = "package P { analysis def A { ref :>> inheritedResult { return true; } subject s : S; } part def S; }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let body_elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected package brace body"),
    };
    let analysis = body_elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::AnalysisCaseDef(a) => Some(&a.value),
            _ => None,
        })
        .expect("analysis def should be present");
    let case_body = match &analysis.body {
        UseCaseDefBody::Brace { elements } => elements,
        _ => panic!("expected analysis brace body"),
    };
    assert!(
        case_body.iter().any(|e| {
            matches!(e.value, UseCaseDefBodyElement::RefRedefinition(_))
        }),
        "expected RefRedefinition, got {:?}",
        case_body.iter().map(|e| format!("{:?}", e.value)).collect::<Vec<_>>()
    );
    assert!(
        !case_body.iter().any(|e| {
            matches!(e.value, UseCaseDefBodyElement::Other(_))
        }),
        "ref :>> should not land in Other"
    );
}

#[test]
fn test_requirement_body_keeps_structured_attributes_and_later_require_constraint() {
    let input = "package P {\nrequirement def R {\nsubject vehicle : Vehicle;\nattribute massActual: MassValue;\nattribute measuredMass = 42;\nrequire constraint { }\n}\n}";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let req = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::RequirementDef(r) => Some(&r.value),
            _ => None,
        })
        .expect("requirement def should be present");
    let body_elements = match &req.body {
        sysml_v2_parser::ast::RequirementDefBody::Brace { elements } => elements,
        _ => panic!("expected requirement brace body"),
    };
    assert!(
        body_elements.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::RequirementDefBodyElement::SubjectDecl(_)
        )),
        "subject should be parsed in requirement body"
    );
    assert!(
        body_elements.iter().any(|e| matches!(
            &e.value,
            sysml_v2_parser::ast::RequirementDefBodyElement::AttributeUsage(a)
                if a.value.typing.is_some()
        )),
        "typed attribute members without an explicit `def` keyword should be attribute usages"
    );
    assert!(
        body_elements.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::RequirementDefBodyElement::AttributeUsage(_)
        )),
        "value-based attribute members should be preserved as structured attribute usages"
    );
    assert!(
        body_elements.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::RequirementDefBodyElement::RequireConstraint(_)
        )),
        "require constraint should be preserved after structured attribute members"
    );
}

#[test]
fn test_parse_requirement_body_supports_attribute_def_and_usage_forms() {
    let input = "package P {\nrequirement def R {\nattribute def targetMass: MassValue;\nattribute actualMass = measuredMass;\n}\n}";
    let result = parse(input).expect("requirement body attributes should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let req = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::RequirementDef(r) => Some(&r.value),
            _ => None,
        })
        .expect("requirement def should be present");
    let body_elements = match &req.body {
        sysml_v2_parser::ast::RequirementDefBody::Brace { elements } => elements,
        _ => panic!("expected requirement brace body"),
    };
    assert!(
        body_elements.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::RequirementDefBodyElement::AttributeDef(_)
        )),
        "attribute def form should be preserved"
    );
    assert!(
        body_elements.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::RequirementDefBodyElement::AttributeUsage(_)
        )),
        "attribute usage form should be preserved"
    );
}

#[test]
fn test_parse_part_usage_body_satisfy_shorthand() {
    let input =
        "package P {\npart def Home {\npart livingRoom: Room {\nsatisfy heatSuff5;\n}\n}\n}";
    let result = parse(input).expect("satisfy shorthand in part usage should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let home = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p)
                if p.value.identification.name.as_deref() == Some("Home") =>
            {
                Some(&p.value)
            }
            _ => None,
        })
        .expect("Home part def should be present");
    let home_body = match &home.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        _ => panic!("expected Home part def body"),
    };
    let living_room = home_body
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::PartDefBodyElement::PartUsage(p)
                if p.value.name == "livingRoom" =>
            {
                Some(&p.value)
            }
            _ => None,
        })
        .expect("livingRoom part usage should be present");
    let living_room_body = match &living_room.body {
        sysml_v2_parser::ast::PartUsageBody::Brace { elements } => elements,
        _ => panic!("expected livingRoom part usage body"),
    };
    assert!(
        living_room_body.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::PartUsageBodyElement::Satisfy(_)
        )),
        "satisfy shorthand should be preserved in part usage body"
    );
}

#[test]
fn test_parse_require_constraint_keeps_inner_members() {
    let input = "package P {\nrequirement def R {\nrequire constraint {\ndoc /* requirement logic */\nin x : Real;\nout y : Real;\nx >= y;\n}\n}\n}";
    let result = parse(input).expect("require constraint body should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let req = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::RequirementDef(r) => Some(&r.value),
            _ => None,
        })
        .expect("requirement def should be present");
    let body_elements = match &req.body {
        sysml_v2_parser::ast::RequirementDefBody::Brace { elements } => elements,
        _ => panic!("expected requirement brace body"),
    };
    let require_constraint = body_elements
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::RequirementDefBodyElement::RequireConstraint(c) => Some(&c.value),
            _ => None,
        })
        .expect("require constraint should be present");
    let constraint_elements = match &require_constraint.body {
        sysml_v2_parser::ast::RequireConstraintBody::Brace { elements } => elements,
        _ => panic!("expected structured require constraint body"),
    };
    assert!(
        constraint_elements.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::ConstraintDefBodyElement::Doc(_)
        )),
        "doc should be preserved inside require constraint"
    );
    assert!(
        constraint_elements.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::ConstraintDefBodyElement::InOutDecl(_)
        )),
        "in/out declarations should be preserved inside require constraint"
    );
    assert!(
        constraint_elements.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::ConstraintDefBodyElement::Expression(_)
        )),
        "expressions should be preserved inside require constraint"
    );
}

#[test]
fn test_parse_requirement_subject_shorthand_without_name() {
    let input = "package P {\nrequirement def R {\nsubject: Laptop;\nrequire constraint { }\n}\n}";
    let result = parse(input).expect("subject shorthand should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let req = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::RequirementDef(r) => Some(&r.value),
            _ => None,
        })
        .expect("requirement def should be present");
    let body_elements = match &req.body {
        sysml_v2_parser::ast::RequirementDefBody::Brace { elements } => elements,
        _ => panic!("expected requirement brace body"),
    };
    let subject = body_elements
        .iter()
        .find_map(|e| match &e.value {
            sysml_v2_parser::ast::RequirementDefBodyElement::SubjectDecl(s) => Some(&s.value),
            _ => None,
        })
        .expect("subject decl should be present");
    assert_eq!(subject.name, "subject");
    assert_eq!(subject.type_name, "Laptop");
    assert!(
        body_elements.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::RequirementDefBodyElement::RequireConstraint(_)
        )),
        "later requirement members should still parse after subject shorthand"
    );
}

#[test]
fn test_requirement_usage_accepts_subsets_keyword_alias() {
    let input = r#"package P {
requirement VehicleReq; subsets BaseReq;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let req = match &elements[0].value {
        PackageBodyElement::RequirementUsage(r) => r,
        other => panic!("expected requirement usage, got {:?}", other),
    };
    assert_eq!(req.value.subsets.as_ref().map(|n| n.value.target_display()), Some("BaseReq".to_string()));
}

#[test]
fn test_requirement_usage_accepts_multiple_subsets_clauses() {
    let input = r#"package P {
requirement VehicleReq; subsets BaseReq :> LatestReq;
}"#;
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let req = match &elements[0].value {
        PackageBodyElement::RequirementUsage(r) => r,
        other => panic!("expected requirement usage, got {:?}", other),
    };
    assert_eq!(req.value.subsets.as_ref().map(|n| n.value.target_display()), Some("LatestReq".to_string()));
}

#[test]
fn test_requirement_body_attribute_typed_with_value_and_redefine_forms() {
    let input = r#"package P {
requirement def R {
  attribute targetMass : Real = (a - (b - c));
  attribute measuredMass :>> Vehicle::mass = ((a - b) - c);
}
}"#;
    let result = parse(input).expect("requirement attributes should parse");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let req = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::RequirementDef(r) => Some(&r.value),
            _ => None,
        })
        .expect("expected requirement definition");
    let sysml_v2_parser::ast::RequirementDefBody::Brace { elements } = &req.body else {
        panic!("expected requirement body");
    };
    assert!(elements.iter().any(|e| matches!(
        &e.value,
        sysml_v2_parser::ast::RequirementDefBodyElement::AttributeUsage(a)
            if a.value.typing.is_some()
    )));
    assert!(elements.iter().any(|e| matches!(
        &e.value,
        sysml_v2_parser::ast::RequirementDefBodyElement::AttributeUsage(a)
            if a.value.redefines.is_some()
    )));
}

#[test]
fn test_requirement_local_typed_real_attribute_is_clean_in_diagnostics() {
    let input = r#"package P {
requirement def VehicleMassRequirement {
  attribute targetMass : Real = (a - (b - c));
  require constraint {
    in actualMass : Real;
    actualMass >= targetMass;
  }
}
}"#;
    let result = sysml_v2_parser::parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "typed requirement-local Real attribute should not produce recovery diagnostics: {:?}",
        result.errors
    );

    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let req = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::RequirementDef(r) => Some(&r.value),
            _ => None,
        })
        .expect("expected requirement definition");
    let sysml_v2_parser::ast::RequirementDefBody::Brace { elements } = &req.body else {
        panic!("expected requirement body");
    };
    assert!(elements.iter().any(|e| matches!(
        &e.value,
        sysml_v2_parser::ast::RequirementDefBodyElement::AttributeUsage(a)
            if a.value.typing.is_some()
    )));
    assert!(elements.iter().any(|e| matches!(
        &e.value,
        sysml_v2_parser::ast::RequirementDefBodyElement::RequireConstraint(_)
    )));
}
