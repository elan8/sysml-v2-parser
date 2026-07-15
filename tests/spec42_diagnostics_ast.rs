//! AST shape tests for Spec42 diagnostics parser roadmap items.

use std::fs;
use std::path::PathBuf;

use sysml_v2_parser::ast::*;
use sysml_v2_parser::parse;

fn fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);
    fs::read_to_string(path).expect("read fixture")
}

fn first_package<'a>(root: &'a RootNamespace) -> &'a Package {
    match &root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    }
}

fn package_body_elements(pkg: &Package) -> &[Node<PackageBodyElement>] {
    match &pkg.body {
        PackageBody::Brace { elements } => elements.as_slice(),
        _ => panic!("expected brace package body"),
    }
}

#[test]
fn transition_accept_retained_with_spans() {
    let root = parse(&fixture("transition-accept-typed.sysml")).expect("parse");
    let pkg = first_package(&root);
    let state_def = match &package_body_elements(pkg)[0].value {
        PackageBodyElement::StateDef(sd) => &sd.value,
        other => panic!("expected state def, got {other:?}"),
    };
    let transitions: Vec<&Transition> = match &state_def.body {
        StateDefBody::Brace { elements } => elements
            .iter()
            .filter_map(|e| match &e.value {
                StateDefBodyElement::Transition(t) => Some(&t.value),
                _ => None,
            })
            .collect(),
        _ => panic!("expected brace state body"),
    };
    assert_eq!(transitions.len(), 2);
    assert!(
        !transitions[0].is_initial,
        "named transition to_running must not be classified as initial"
    );
    let accept = transitions[0]
        .accept
        .as_ref()
        .expect("shorthand accept");
    match accept {
        TransitionAccept::Shorthand(expr, via) => {
            assert!(matches!(expr.value, Expression::FeatureRef(ref n) if n == "StartPressed"));
            assert!(expr.span.len > 0);
            assert!(via.is_none(), "fixture has no `via` clause on this trigger");
        }
        _ => panic!("expected shorthand accept"),
    }
    let typed = transitions[1]
        .accept
        .as_ref()
        .expect("typed accept");
    match typed {
        TransitionAccept::Payload(p, via) => {
            assert_eq!(p.name, "evt");
            assert_eq!(p.type_name.as_deref(), Some("StartEvent"));
            assert!(p.name_span.len > 0);
            assert!(p.type_span.is_some());
            assert!(via.is_none(), "fixture has no `via` clause on this trigger");
        }
        _ => panic!("expected typed accept"),
    }
}

#[test]
fn final_state_members_parsed() {
    let root = parse(&fixture("final-state.sysml")).expect("parse");
    let pkg = first_package(&root);
    let state_def = match &package_body_elements(pkg)[0].value {
        PackageBodyElement::StateDef(sd) => &sd.value,
        other => panic!("expected state def, got {other:?}"),
    };
    let finals: Vec<&FinalState> = match &state_def.body {
        StateDefBody::Brace { elements } => elements
            .iter()
            .filter_map(|e| match &e.value {
                StateDefBodyElement::FinalState(f) => Some(&f.value),
                _ => None,
            })
            .collect(),
        _ => panic!("expected brace state body"),
    };
    assert_eq!(finals.len(), 2);
    assert_eq!(finals[0].state_name, "expired");
    assert_eq!(finals[1].state_name, "completed");
    assert!(finals[0].name_span.len > 0);
}

#[test]
fn send_payload_on_control_node_action() {
    let root = parse(&fixture("send-payload.sysml")).expect("parse");
    let pkg = first_package(&root);
    let action_def = match &package_body_elements(pkg)[0].value {
        PackageBodyElement::ActionDef(ad) => &ad.value,
        other => panic!("expected action def, got {other:?}"),
    };
    let send_usage = match &action_def.body {
        ActionDefBody::Brace { elements } => elements
            .iter()
            .find_map(|e| match &e.value {
                ActionDefBodyElement::ActionUsage(a) => Some(&a.value),
                _ => None,
            })
            .expect("send action usage"),
        _ => panic!("expected brace action body"),
    };
    assert_eq!(send_usage.name, "send");
    let send = send_usage.send.as_ref().expect("send payload");
    assert_eq!(send.name, "message");
    assert_eq!(send.type_name.as_deref(), Some("AlertMessage"));
    assert!(send.name_span.len > 0);
    assert!(send.type_span.is_some());
}

#[test]
fn viewpoint_stakeholder_and_purpose_members() {
    let root = parse(&fixture("viewpoint-stakeholder-purpose.sysml")).expect("parse");
    let pkg = first_package(&root);
    let vp = match &package_body_elements(pkg)[0].value {
        PackageBodyElement::ViewpointDef(v) => &v.value,
        other => panic!("expected viewpoint def, got {other:?}"),
    };
    let body = match &vp.body {
        RequirementDefBody::Brace { elements } => elements,
        _ => panic!("expected brace viewpoint body"),
    };
    assert!(body.iter().any(|e| matches!(
        e.value,
        RequirementDefBodyElement::Stakeholder(_)
    )));
    assert!(body.iter().any(|e| matches!(
        e.value,
        RequirementDefBodyElement::Purpose(_)
    )));
}

#[test]
fn metadata_keyword_usage_in_part_body() {
    let root = parse(&fixture("metadata-keyword-usage.sysml")).expect("parse");
    let pkg = first_package(&root);
    let part_def = package_body_elements(pkg)
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(pd) => Some(&pd.value),
            _ => None,
        })
        .expect("part def");
    let keyword = match &part_def.body {
        PartDefBody::Brace { elements } => elements
            .iter()
            .find_map(|e| match &e.value {
                PartDefBodyElement::MetadataKeywordUsage(k) => Some(&k.value),
                _ => None,
            })
            .expect("metadata keyword usage"),
        _ => panic!("expected brace part body"),
    };
    assert_eq!(keyword.keyword, "Tag");
    assert!(keyword.keyword_span.len > 0);
}

#[test]
fn verification_local_attribute_has_name_span() {
    let root = parse(&fixture("verification-local-attribute.sysml")).expect("parse");
    let pkg = first_package(&root);
    let verification = match &package_body_elements(pkg)[0].value {
        PackageBodyElement::VerificationCaseDef(v) => &v.value,
        other => panic!("expected verification def, got {other:?}"),
    };
    let attr = match &verification.body {
        UseCaseDefBody::Brace { elements } => elements
            .iter()
            .find_map(|e| match &e.value {
                UseCaseDefBodyElement::AttributeDef(a) => Some(&a.value),
                _ => None,
            })
            .expect("attribute def"),
        _ => panic!("expected brace verification body"),
    };
    assert_eq!(attr.name, "count");
    assert!(attr.name_span.is_some());
    assert_eq!(attr.typing.as_ref().map(|n| n.value.target_display()), Some("Integer".to_string()));
}

#[test]
fn requirement_body_rep_language_parsed() {
    let root = parse(&fixture("requirement-rep-language.sysml")).expect("parse");
    let pkg = first_package(&root);
    let req = match &package_body_elements(pkg)[0].value {
        PackageBodyElement::RequirementDef(r) => &r.value,
        other => panic!("expected requirement def, got {other:?}"),
    };
    let rep = match &req.body {
        RequirementDefBody::Brace { elements } => elements
            .iter()
            .find_map(|e| match &e.value {
                RequirementDefBodyElement::TextualRep(t) => Some(&t.value),
                _ => None,
            })
            .expect("textual rep"),
        _ => panic!("expected brace requirement body"),
    };
    assert_eq!(rep.language, "sysml");
    assert!(rep.language_span.is_some());
}

#[test]
fn diagnostic_catalog_documents_stable_codes() {
    use sysml_v2_parser::parser::diagnostic_catalog::DOCUMENTED_CODES;
    assert!(DOCUMENTED_CODES.contains(&"missing_member_name"));
    assert!(DOCUMENTED_CODES.contains(&"missing_closing_brace"));
}

#[test]
fn unnamed_transition_first_sets_is_initial_flag() {
    let input = "package P { state def S { transition first idle then running; } }";
    let root = parse(input).expect("parse");
    let pkg = first_package(&root);
    let state_def = match &package_body_elements(pkg)[0].value {
        PackageBodyElement::StateDef(sd) => &sd.value,
        _ => panic!("expected state def"),
    };
    let transition = match &state_def.body {
        StateDefBody::Brace { elements } => elements
            .iter()
            .find_map(|e| match &e.value {
                StateDefBodyElement::Transition(t) => Some(&t.value),
                _ => None,
            })
            .expect("transition"),
        _ => panic!("expected brace body"),
    };
    assert!(transition.is_initial);
    assert!(transition.source.is_some());
}

#[test]
fn named_transition_first_source_is_not_initial() {
    let input = "package P { state def S { transition t first idle then running; } }";
    let root = parse(input).expect("parse");
    let pkg = first_package(&root);
    let state_def = match &package_body_elements(pkg)[0].value {
        PackageBodyElement::StateDef(sd) => &sd.value,
        _ => panic!("expected state def"),
    };
    let transition = match &state_def.body {
        StateDefBody::Brace { elements } => elements
            .iter()
            .find_map(|e| match &e.value {
                StateDefBodyElement::Transition(t) => Some(&t.value),
                _ => None,
            })
            .expect("transition"),
        _ => panic!("expected brace body"),
    };
    assert!(
        !transition.is_initial,
        "named transition first source must not be classified as initial"
    );
    assert!(transition.source.is_some());
}

fn filter_conditions(pkg: &Package) -> Vec<&Node<Expression>> {
    for element in package_body_elements(pkg) {
        if let PackageBodyElement::ViewDef(v) = &element.value {
            if let ViewDefBody::Brace { elements } = &v.value.body {
                return elements
                    .iter()
                    .filter_map(|el| match &el.value {
                        ViewDefBodyElement::Filter(f) => Some(&f.value.condition),
                        _ => None,
                    })
                    .collect();
            }
        }
    }
    Vec::new()
}

#[test]
fn filter_expressions_use_classification_ast() {
    let root = parse(&fixture("expression-classification.sysml")).expect("parse");
    let pkg = first_package(&root);
    let filters = filter_conditions(pkg);
    assert_eq!(filters.len(), 4);

    match &filters[0].value {
        Expression::BinaryOp { op, left, right } => {
            assert_eq!(op.as_str(), "||");
            assert!(matches!(
                left.value,
                Expression::Classification { ref metaclass }
                    if metaclass == "SysML::PartUsage"
            ));
            assert!(matches!(
                right.value,
                Expression::Classification { ref metaclass }
                    if metaclass == "SysML::PortUsage"
            ));
        }
        other => panic!("expected or of classifications, got {other:?}"),
    }

    match &filters[1].value {
        Expression::UnaryOp { op, operand } => {
            assert_eq!(op.as_str(), "not");
            assert!(matches!(
                operand.value,
                Expression::Classification { ref metaclass }
                    if metaclass == "SysML::ConnectionUsage"
            ));
        }
        other => panic!("expected not classification, got {other:?}"),
    }

    match &filters[2].value {
        Expression::BinaryOp { op, left, right } => {
            assert_eq!(op.as_str(), "&&");
            assert!(matches!(
                left.value,
                Expression::Classification { ref metaclass } if metaclass == "Approval"
            ));
            assert!(
                matches!(
                    &right.value,
                    Expression::MemberAccess(_, member) if member == "approved"
                ) || matches!(
                    &right.value,
                    Expression::FeatureRef(name) if name.ends_with("approved")
                )
            );
        }
        other => panic!("expected and of classification + member access, got {other:?}"),
    }

    assert!(matches!(
        filters[3].value,
        Expression::FeatureRef(ref name) if name == "guardExpr"
    ));
    assert!(filters[0].span.len > 0);
}

#[test]
fn transition_guard_feature_ref_retained() {
    let root = parse(&fixture("expression-classification.sysml")).expect("parse");
    let pkg = first_package(&root);
    let state_def = package_body_elements(pkg)
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::StateDef(sd) => Some(&sd.value),
            _ => None,
        })
        .expect("state def");
    let transition = match &state_def.body {
        StateDefBody::Brace { elements } => elements
            .iter()
            .find_map(|e| match &e.value {
                StateDefBodyElement::Transition(t) => Some(&t.value),
                _ => None,
            })
            .expect("transition"),
        _ => panic!("expected brace state body"),
    };
    let guard = transition.guard.as_ref().expect("guard");
    assert!(matches!(
        guard.value,
        Expression::FeatureRef(ref name) if name == "guardExpr"
    ));
}

#[test]
fn typed_stakeholder_parameter_parsed() {
    let root = parse(&fixture("stakeholder-typed.sysml")).expect("parse");
    let pkg = first_package(&root);
    let req = match &package_body_elements(pkg)[0].value {
        PackageBodyElement::RequirementDef(r) => &r.value,
        other => panic!("expected requirement def, got {other:?}"),
    };
    let body = match &req.body {
        RequirementDefBody::Brace { elements } => elements,
        _ => panic!("expected brace requirement body"),
    };
    let stakeholders: Vec<&StakeholderMember> = body
        .iter()
        .filter_map(|e| match &e.value {
            RequirementDefBodyElement::Stakeholder(s) => Some(&s.value),
            _ => None,
        })
        .collect();
    assert_eq!(stakeholders.len(), 2);
    assert_eq!(stakeholders[0].name, "driver");
    assert_eq!(stakeholders[0].type_name.as_deref(), Some("Person"));
    assert!(stakeholders[0].name_span.len > 0);
    assert!(stakeholders[0].type_span.is_some());
    assert_eq!(stakeholders[1].name, "SafetyConcern");
    assert!(stakeholders[1].type_name.is_none());
}

#[test]
fn constraint_body_metadata_annotation_parsed() {
    let root = parse(&fixture("constraint-metadata-annotation.sysml")).expect("parse");
    let pkg = first_package(&root);
    let constraint = match &package_body_elements(pkg)[0].value {
        PackageBodyElement::ConstraintDef(c) => &c.value,
        other => panic!("expected constraint def, got {other:?}"),
    };
    let meta = match &constraint.body {
        ConstraintDefBody::Brace { elements } => elements
            .iter()
            .find_map(|e| match &e.value {
                ConstraintDefBodyElement::MetadataAnnotation(m) => Some(&m.value),
                _ => None,
            })
            .expect("metadata annotation in constraint body"),
        _ => panic!("expected brace constraint body"),
    };
    assert_eq!(meta.name, "Approval");
    assert_eq!(meta.type_name.as_deref(), Some("ApprovalKind"));
    assert!(meta.head_span.as_ref().is_some_and(|s| s.len > 0));
}

#[test]
fn metadata_annotation_brace_body_parses_shorthand_bindings() {
    let root = parse(
        r#"package P {
  part def Design {
    @ApprovalAnnotation {
      approved = true;
      approver = "John";
    }
  }
}"#,
    )
    .expect("parse");
    let pkg = first_package(&root);
    let part_def = package_body_elements(pkg)
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(pd) => Some(&pd.value),
            _ => None,
        })
        .expect("part def");
    let meta = match &part_def.body {
        PartDefBody::Brace { elements } => elements
            .iter()
            .find_map(|e| match &e.value {
                PartDefBodyElement::MetadataAnnotation(m) => Some(&m.value),
                _ => None,
            })
            .expect("metadata annotation"),
        _ => panic!("expected brace part body"),
    };
    let AttributeBody::Brace { elements } = &meta.body else {
        panic!("expected brace metadata body");
    };
    assert_eq!(elements.len(), 2);
    assert!(matches!(
        &elements[0].value,
        AttributeBodyElement::AttributeUsage(u) if u.value.name == "approved"
    ));
}

#[test]
fn metadata_usage_about_clause_parses_targets() {
    let root = parse(
        r#"package P {
  metadata def SecurityRelated;
  metadata securityNote : SecurityRelated about SecurityReq, Design;
}"#,
    )
    .expect("parse");
    let pkg = first_package(&root);
    let usage = package_body_elements(pkg)
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::MetadataUsage(mu) => Some(&mu.value),
            _ => None,
        })
        .expect("metadata usage");
    assert_eq!(usage.about_targets, vec!["SecurityReq", "Design"]);
}

#[test]
fn metadata_annotation_about_clause_parses_targets() {
    let root = parse(
        r#"package P {
  metadata def Tag;
  part def Design {
    @Tag about OtherPart;
  }
  part def OtherPart;
}"#,
    )
    .expect("parse");
    let pkg = first_package(&root);
    let part_def = package_body_elements(pkg)
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(pd) if pd.value.identification.name.as_deref() == Some("Design") => {
                Some(&pd.value)
            }
            _ => None,
        })
        .expect("Design part def");
    let meta = match &part_def.body {
        PartDefBody::Brace { elements } => elements
            .iter()
            .find_map(|e| match &e.value {
                PartDefBodyElement::MetadataAnnotation(m) => Some(&m.value),
                _ => None,
            })
            .expect("metadata annotation"),
        _ => panic!("expected brace part body"),
    };
    assert_eq!(meta.about_targets, vec!["OtherPart"]);
}

#[test]
fn action_def_body_metadata_keyword_parses() {
    let root = parse(
        r#"package P {
  metadata def Tag;
  action def A {
    #Tag;
  }
}"#,
    )
    .expect("parse");
    let pkg = first_package(&root);
    let action_def = package_body_elements(pkg)
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::ActionDef(a) => Some(&a.value),
            _ => None,
        })
        .expect("action def");
    let keyword = match &action_def.body {
        ActionDefBody::Brace { elements } => elements
            .iter()
            .find_map(|e| match &e.value {
                ActionDefBodyElement::MetadataKeywordUsage(k) => Some(&k.value),
                _ => None,
            })
            .expect("metadata keyword"),
        _ => panic!("expected brace action body"),
    };
    assert_eq!(keyword.keyword, "Tag");
}

#[test]
fn meta_cast_expression_parses_in_attribute_binding() {
    let root = parse(
        r#"package P {
  metadata def SituationMetadata :> SemanticMetadata {
    attribute :>> baseType = userActions meta SysML::Usage;
  }
}"#,
    )
    .expect("parse");
    let pkg = first_package(&root);
    let metadata_def = match &package_body_elements(pkg)[0].value {
        PackageBodyElement::MetadataDef(m) => &m.value,
        other => panic!("expected metadata def, got {other:?}"),
    };
    let attr = match &metadata_def.body {
        AttributeBody::Brace { elements } => elements
            .iter()
            .find_map(|e| match &e.value {
                AttributeBodyElement::AttributeUsage(a) => Some(&a.value),
                _ => None,
            })
            .expect("attribute usage"),
        _ => panic!("expected brace metadata def body"),
    };
    let Some(expr) = attr.value.as_ref() else {
        panic!("expected value expression");
    };
    assert!(matches!(
        &expr.value.expression.value,
        Expression::MetaCast { metaclass, .. } if metaclass == "SysML::Usage"
    ));
}

fn metadata_def_body_elements(metadata_def: &MetadataDef) -> &[Node<AttributeBodyElement>] {
    match &metadata_def.body {
        AttributeBody::Brace { elements } => elements.as_slice(),
        _ => panic!("expected brace metadata def body"),
    }
}

fn attribute_body_error_count(elements: &[Node<AttributeBodyElement>]) -> usize {
    elements
        .iter()
        .filter(|e| matches!(e.value, AttributeBodyElement::Error(_)))
        .count()
}

#[test]
fn metadata_def_shorthand_annotated_element() {
    let root = parse(
        r#"package P {
  metadata def RequirementRole {
    :> annotatedElement : SysML::RequirementUsage;
    attribute role;
  }
}"#,
    )
    .expect("parse");
    let pkg = first_package(&root);
    let metadata_def = match &package_body_elements(pkg)[0].value {
        PackageBodyElement::MetadataDef(m) => &m.value,
        other => panic!("expected metadata def, got {other:?}"),
    };
    let elements = metadata_def_body_elements(metadata_def);
    assert_eq!(
        attribute_body_error_count(elements),
        0,
        "unexpected attribute body errors"
    );
    let attr = elements
        .iter()
        .find_map(|e| match &e.value {
            AttributeBodyElement::AttributeUsage(a) if a.value.name == "annotatedElement" => {
                Some(&a.value)
            }
            _ => None,
        })
        .expect("annotatedElement shorthand binding");
    assert_eq!(attr.subsets.as_ref().map(|n| n.value.target_display()), Some("annotatedElement".to_string()));
    assert_eq!(attr.typing.as_ref().map(|n| n.value.target_display()), Some("SysML::RequirementUsage".to_string()));
}

#[test]
fn metadata_def_shorthand_base_type_meta_cast() {
    let root = parse(
        r#"package P {
  metadata def UserRequirementRole :> SemanticMetadata {
    :>> baseType = requirementChecks meta SysML::Usage;
  }
}"#,
    )
    .expect("parse");
    let pkg = first_package(&root);
    let metadata_def = match &package_body_elements(pkg)[0].value {
        PackageBodyElement::MetadataDef(m) => &m.value,
        other => panic!("expected metadata def, got {other:?}"),
    };
    let attr = metadata_def_body_elements(metadata_def)
        .iter()
        .find_map(|e| match &e.value {
            AttributeBodyElement::AttributeUsage(a) => Some(&a.value),
            _ => None,
        })
        .expect("baseType shorthand binding");
    assert_eq!(attr.redefines.as_ref().map(|n| n.value.target_display()), Some("baseType".to_string()));
    let Some(expr) = attr.value.as_ref() else {
        panic!("expected value expression");
    };
    assert!(matches!(
        &expr.value.expression.value,
        Expression::MetaCast { metaclass, .. } if metaclass == "SysML::Usage"
    ));
}

#[test]
fn requirement_metadata_def_body_no_errors() {
    let root = parse(
        r#"package RequirementMetadata {
  enum def RequirementRoleKind {
    enum user;
    enum system;
  }

  metadata def RequirementRole {
    :> annotatedElement : SysML::RequirementUsage;
    attribute role : RequirementRoleKind;
  }

  metadata def RequirementIdentity {
    :> annotatedElement : SysML::RequirementUsage;
    attribute requirementId;
  }

  metadata def <user> UserRequirementRole :> SemanticMetadata {
    :> annotatedElement : SysML::RequirementUsage;
    :>> baseType = requirementChecks meta SysML::Usage;
  }

  metadata def <system> SystemRequirementRole :> SemanticMetadata {
    :> annotatedElement : SysML::RequirementUsage;
    :>> baseType = requirementChecks meta SysML::Usage;
  }
}"#,
    )
    .expect("parse");
    let pkg = first_package(&root);
    let metadata_defs: Vec<&MetadataDef> = package_body_elements(pkg)
        .iter()
        .filter_map(|e| match &e.value {
            PackageBodyElement::MetadataDef(m) => Some(&m.value),
            _ => None,
        })
        .collect();
    assert_eq!(metadata_defs.len(), 4, "expected four metadata defs");
    for metadata_def in metadata_defs {
        let errors = attribute_body_error_count(metadata_def_body_elements(metadata_def));
        assert_eq!(
            errors, 0,
            "metadata def body should have no parse errors: {:?}",
            metadata_def.identification.name.as_deref().unwrap_or("<unnamed>")
        );
    }
}

#[test]
fn metadata_annotation_in_use_case_and_view_bodies() {
    let src = fixture("metadata-annotation-usecase-view.sysml");
    let root = parse(&src).expect("parse");
    let pkg = first_package(&root);
    let elements = package_body_elements(pkg);

    // use case def body
    let use_case = match &elements[0].value {
        PackageBodyElement::UseCaseDef(u) => &u.value,
        other => panic!("expected use case def, got {other:?}"),
    };
    let uc_body = match &use_case.body {
        UseCaseDefBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let ann = match &uc_body[0].value {
        UseCaseDefBodyElement::MetadataAnnotation(a) => &a.value,
        other => panic!("expected MetadataAnnotation in use case body, got {other:?}"),
    };
    assert_eq!(ann.name, "SomeMetaclass");
    assert!(ann.head_span.is_some(), "head_span should be set");

    // view def body
    let view = match &elements[1].value {
        PackageBodyElement::ViewDef(v) => &v.value,
        other => panic!("expected view def, got {other:?}"),
    };
    let v_body = match &view.body {
        ViewDefBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let ann = match &v_body[0].value {
        ViewDefBodyElement::MetadataAnnotation(a) => &a.value,
        other => panic!("expected MetadataAnnotation in view def body, got {other:?}"),
    };
    assert_eq!(ann.name, "AnotherMetaclass");
    assert!(ann.head_span.is_some(), "head_span should be set");

    // calc def body
    let calc = match &elements[2].value {
        PackageBodyElement::CalcDef(c) => &c.value,
        other => panic!("expected calc def, got {other:?}"),
    };
    let c_body = match &calc.body {
        CalcDefBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let ann = match &c_body[0].value {
        CalcDefBodyElement::MetadataAnnotation(a) => &a.value,
        other => panic!("expected MetadataAnnotation in calc def body, got {other:?}"),
    };
    assert_eq!(ann.name, "YetAnotherMetaclass");
    assert!(ann.head_span.is_some(), "head_span should be set");
}

#[test]
fn case_return_decl_in_verification_and_analysis_bodies() {
    let src = fixture("analysis-case-return-decl.sysml");
    let root = parse(&src).expect("parse");
    let pkg = first_package(&root);
    let elements = package_body_elements(pkg);

    // verification case def — multiple return decls
    let vcase = match &elements[0].value {
        PackageBodyElement::VerificationCaseDef(v) => &v.value,
        other => panic!("expected verification case def, got {other:?}"),
    };
    let vc_body = match &vcase.body {
        UseCaseDefBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };

    // element 1: `return verdict : VerdictKind = ...`
    let ret1 = match &vc_body[1].value {
        UseCaseDefBodyElement::CaseReturnDecl(r) => &r.value,
        other => panic!("expected CaseReturnDecl (plain return), got {other:?}"),
    };
    assert_eq!(ret1.name, "verdict");
    assert!(!ret1.is_redefine);
    assert_eq!(ret1.type_name.as_deref(), Some("VerdictKind"));
    assert!(ret1.name_span.is_some(), "name_span should be set");

    // element 2: `return :>> result = ...`
    let ret2 = match &vc_body[2].value {
        UseCaseDefBodyElement::CaseReturnDecl(r) => &r.value,
        other => panic!("expected CaseReturnDecl (:>> form), got {other:?}"),
    };
    assert_eq!(ret2.name, "result");
    assert!(ret2.is_redefine);

    // analysis case def — one return decl
    let acase = match &elements[1].value {
        PackageBodyElement::AnalysisCaseDef(a) => &a.value,
        other => panic!("expected analysis case def, got {other:?}"),
    };
    let ac_body = match &acase.body {
        UseCaseDefBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let ret3 = match &ac_body[1].value {
        UseCaseDefBodyElement::CaseReturnDecl(r) => &r.value,
        other => panic!("expected CaseReturnDecl in analysis body, got {other:?}"),
    };
    assert_eq!(ret3.name, "thrust");
    assert_eq!(ret3.type_name.as_deref(), Some("ForceValue"));
}

#[test]
fn rep_missing_language_emits_diagnostic() {
    let src = fixture("rep-missing-language.sysml");
    let result = sysml_v2_parser::parse_for_editor(&src);
    let codes: Vec<&str> = result
        .errors
        .iter()
        .filter_map(|e| e.code.as_deref())
        .collect();
    assert!(
        codes.contains(&"missing_rep_language"),
        "expected missing_rep_language diagnostic, got: {codes:?}"
    );
}

#[test]
fn rep_empty_language_emits_invalid_diagnostic() {
    let src = r#"package P { rep language "" /* body */ }"#;
    let result = sysml_v2_parser::parse_for_editor(src);
    let codes: Vec<&str> = result
        .errors
        .iter()
        .filter_map(|e| e.code.as_deref())
        .collect();
    assert!(
        codes.contains(&"invalid_rep_language"),
        "expected invalid_rep_language diagnostic, got: {codes:?}"
    );
}
