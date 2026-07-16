//! Parser tests: behavior

use sysml_v2_parser::ast::*;
use sysml_v2_parser::{parse, parse_with_diagnostics};

#[test]
fn test_state_def_body_parses_members() {
    let input =
        "package P { state def S { then Ready; state Running : Mode; transition t then Ready; } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let state_def = match &elements[0].value {
        PackageBodyElement::StateDef(sd) => &sd.value,
        _ => panic!("expected StateDef"),
    };
    let body_elements = match &state_def.body {
        sysml_v2_parser::ast::StateDefBody::Brace { elements } => elements,
        _ => panic!("expected state brace body"),
    };
    assert!(body_elements
        .iter()
        .any(|e| matches!(e.value, sysml_v2_parser::ast::StateDefBodyElement::Then(_))));
    assert!(body_elements.iter().any(|e| matches!(
        e.value,
        sysml_v2_parser::ast::StateDefBodyElement::StateUsage(_)
    )));
    assert!(body_elements.iter().any(|e| matches!(
        e.value,
        sysml_v2_parser::ast::StateDefBodyElement::Transition(_)
    )));
}

#[test]
fn test_constraint_and_calc_bodies_parse_members() {
    let input = "package P { constraint def C { in x : Real; out y : Real; x >= y; } calc def K { in x : Real; return r : Real; x; } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let constraint_def = match &elements[0].value {
        PackageBodyElement::ConstraintDef(cd) => &cd.value,
        _ => panic!("expected ConstraintDef"),
    };
    let constraint_elements = match &constraint_def.body {
        sysml_v2_parser::ast::ConstraintDefBody::Brace { elements } => elements,
        _ => panic!("expected constraint brace body"),
    };
    assert!(
        !constraint_elements.is_empty(),
        "constraint body should not be empty"
    );
    let calc_def = match &elements[1].value {
        PackageBodyElement::CalcDef(cd) => &cd.value,
        _ => panic!("expected CalcDef"),
    };
    let calc_elements = match &calc_def.body {
        sysml_v2_parser::ast::CalcDefBody::Brace { elements } => elements,
        _ => panic!("expected calc brace body"),
    };
    assert!(!calc_elements.is_empty(), "calc body should not be empty");
}

#[test]
fn test_perform_action_decl_body_parses_bindings() {
    let input = "package P { part def Carrier { perform action run : Runner { in speed = speedInput; out torque = torqueOutput; } } }";
    let result = parse(input).expect("parse should succeed");
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    let part_def = match &elements[0].value {
        PackageBodyElement::PartDef(pd) => &pd.value,
        _ => panic!("expected PartDef"),
    };
    let part_body = match &part_def.body {
        sysml_v2_parser::ast::PartDefBody::Brace { elements } => elements,
        _ => panic!("expected part def brace body"),
    };
    let perform = match &part_body[0].value {
        sysml_v2_parser::ast::PartDefBodyElement::Perform(p) => &p.value,
        _ => panic!("expected perform action declaration"),
    };
    assert!(
        matches!(&perform.body, sysml_v2_parser::ast::PerformBody::Brace { elements } if !elements.is_empty()),
        "perform action brace body should retain parsed in/out bindings"
    );
}

#[test]
fn test_action_def_body_allows_doc_and_nested_action_usages_without_semicolon_after_doc() {
    let input = r#"package P {
action def ExecutePatrol {
  in route : String;
  out status : String;
  doc /* Execute patrol/overwatch mission along route. */

  action validateRoute { out validationStatus : String; };
  action startMission { out missionStarted : String; };

  first validateRoute then startMission;
  bind status = startMission::missionStarted;
}
}"#;

    let result = parse_with_diagnostics(input);
    assert!(
        result.is_ok(),
        "action def with doc + nested actions should parse without recovery diagnostics: {:?}",
        result.errors
    );
    assert!(
        !result
            .errors
            .iter()
            .any(|e| e.code.as_deref() == Some("missing_semicolon")),
        "should not report missing_semicolon around doc/nested action usages: {:?}",
        result.errors
    );
}

#[test]
fn test_action_usage_body_allows_untyped_out_pin_decl() {
    // Common SysML v2 shorthand in action usage bodies: `out foo;` (no `: Type`)
    // to reference the corresponding typed parameter on the referenced action definition.
    let input = r#"package P {
action def CaptureVideo { out videoStream : String; }
action def ExecutePatrol {
  action capture : CaptureVideo { out videoStream; };
  first capture then capture;
}
}"#;

    let result = parse_with_diagnostics(input);
    assert!(
        result.is_ok(),
        "untyped out pin decl in action usage body should not trigger recovery diagnostics: {:?}",
        result.errors
    );
}

#[test]
fn test_action_def_accepts_specializes_keyword_as_specialization() {
    let input = r#"package P {
action def Run specializes BaseAction;
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
    let action_def = match &elements[0].value {
        PackageBodyElement::ActionDef(p) => p,
        other => panic!("expected action def, got {:?}", other),
    };
    assert_eq!(
        action_def
            .value
            .specializes
            .as_ref()
            .map(|n| n.value.target_display()),
        Some("BaseAction".to_string())
    );
}

#[test]
fn test_action_def_preserves_multiple_specializes_targets() {
    let input = r#"package P {
action def Run :> BaseAction, LoggedAction;
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
    let action_def = match &elements[0].value {
        PackageBodyElement::ActionDef(action) => action,
        other => panic!("expected action definition, got {:?}", other),
    };
    assert_eq!(
        action_def
            .value
            .specializes
            .as_ref()
            .map(|n| n.value.target_display()),
        Some("BaseAction, LoggedAction".to_string())
    );
    assert!(action_def.value.specializes.is_some());
}

#[test]
fn test_assign_stmt_rhs_parses_as_structured_expression() {
    // Regression: `AssignStmt.rhs` used to be a raw `String` captured via
    // `take_until_terminator`. Now that the expression grammar supports the KerML
    // arrow-invocation operator (`->`), the RHS is a real `Node<Expression>`.
    let input = r#"package P {
action def Compute {
  in collection;
  attribute total : Integer;
  assign total := collection->size();
}
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
    let action_def = match &elements[0].value {
        PackageBodyElement::ActionDef(a) => a,
        other => panic!("expected action def, got {:?}", other),
    };
    let body_elements = match &action_def.value.body {
        sysml_v2_parser::ast::ActionDefBody::Brace { elements } => elements,
        other => panic!("expected action def brace body, got {:?}", other),
    };
    let assign = body_elements
        .iter()
        .find_map(|el| match &el.value {
            sysml_v2_parser::ast::ActionDefBodyElement::Assign(a) => Some(&a.value),
            _ => None,
        })
        .expect("action def body should contain an AssignStmt");
    match &assign.rhs.value {
        // PAR-005 item 2: `->size()` is now a dedicated `CollectionOp`, not a generic
        // `Invocation` wrapping `MemberAccess`.
        Expression::CollectionOp { op, base, args } => {
            assert_eq!(op, &sysml_v2_parser::ast::CollectionOperator::Size);
            assert!(args.is_empty());
            assert!(matches!(&base.value, Expression::FeatureRef(s) if s == "collection"));
        }
        other => panic!(
            "expected rhs to be a structured CollectionOp expression, got {:?}",
            other
        ),
    }
}

#[test]
fn test_for_loop_range_uses_structured_arrow_invocation_not_raw_text_fallback() {
    // Regression: `for_loop()` falls back to a raw-text `Expression::FeatureRef` when
    // `expression()` can't parse the range. Arrow-invocation (`->`) used to be the common
    // case that hit this fallback; now that expr.rs supports `->`, the range should parse
    // as a structured Invocation, not the raw fallback text.
    let input = r#"package P {
action def Iterate {
  in powerProfile;
  for x in powerProfile->size() {
    assign x := x + 1;
  }
}
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
    let action_def = match &elements[0].value {
        PackageBodyElement::ActionDef(a) => a,
        other => panic!("expected action def, got {:?}", other),
    };
    let body_elements = match &action_def.value.body {
        sysml_v2_parser::ast::ActionDefBody::Brace { elements } => elements,
        other => panic!("expected action def brace body, got {:?}", other),
    };
    let for_loop = body_elements
        .iter()
        .find_map(|el| match &el.value {
            sysml_v2_parser::ast::ActionDefBodyElement::ForLoop(f) => Some(&f.value),
            _ => None,
        })
        .expect("action def body should contain a ForLoop");
    match &for_loop.range.value {
        // PAR-005 item 2: `->size()` is now a dedicated `CollectionOp`, not a generic
        // `Invocation` wrapping `MemberAccess`.
        Expression::CollectionOp { op, .. } => {
            assert_eq!(op, &sysml_v2_parser::ast::CollectionOperator::Size);
        }
        other => panic!(
            "expected structured CollectionOp range (not the raw-text FeatureRef fallback), got {:?}",
            other
        ),
    }
}

fn action_def_body_elements(
    result: &sysml_v2_parser::ast::RootNamespace,
) -> Vec<sysml_v2_parser::ast::ActionDefBodyElement> {
    let pkg = match &result.elements[0].value {
        RootElement::Package(p) => p,
        other => panic!("expected package, got {:?}", other),
    };
    let elements = match &pkg.value.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {:?}", other),
    };
    let action_def = match &elements[0].value {
        PackageBodyElement::ActionDef(a) => a,
        other => panic!("expected action def, got {:?}", other),
    };
    match &action_def.value.body {
        sysml_v2_parser::ast::ActionDefBody::Brace { elements } => {
            elements.iter().map(|el| el.value.clone()).collect()
        }
        other => panic!("expected action def brace body, got {:?}", other),
    }
}

#[test]
fn test_terminate_stmt_parses_bare_and_targeted_forms() {
    // Regression: `terminate` was listed in ACTION_BODY_STARTERS but had no parser function,
    // so it fell through to generic error recovery.
    let input = r#"package P {
action def Run {
  action step;
  terminate;
  terminate step;
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let body_elements = action_def_body_elements(&result);
    let terminates: Vec<_> = body_elements
        .iter()
        .filter_map(|el| match el {
            sysml_v2_parser::ast::ActionDefBodyElement::TerminateStmt(t) => Some(&t.value),
            _ => None,
        })
        .collect();
    assert_eq!(
        terminates.len(),
        2,
        "expected two TerminateStmt nodes, got {:?}",
        body_elements
    );
    assert!(
        terminates[0].target.is_none(),
        "bare `terminate;` should have no target"
    );
    assert!(
        matches!(&terminates[1].target, Some(t) if matches!(&t.value, Expression::FeatureRef(s) if s == "step")),
        "expected `terminate step;` to target `step`, got {:?}",
        terminates[1].target
    );
}

#[test]
fn test_while_stmt_parses_condition_and_nested_body() {
    // Regression: `while` was listed in ACTION_BODY_STARTERS but had no parser function.
    let input = r#"package P {
action def Run {
  attribute x : Integer;
  while x < 10 {
    assign x := x + 1;
  }
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let body_elements = action_def_body_elements(&result);
    let while_stmt = body_elements
        .iter()
        .find_map(|el| match el {
            sysml_v2_parser::ast::ActionDefBodyElement::WhileStmt(w) => Some(&w.value),
            _ => None,
        })
        .expect("expected a WhileStmt node");
    assert!(matches!(
        &while_stmt.condition.value,
        Expression::BinaryOp { .. }
    ));
    match &while_stmt.body {
        sysml_v2_parser::ast::ActionDefBody::Brace { elements } => {
            assert!(
                elements.iter().any(|el| matches!(
                    el.value,
                    sysml_v2_parser::ast::ActionDefBodyElement::Assign(_)
                )),
                "while body should retain the nested assign statement, got {:?}",
                elements
            );
        }
        other => panic!("expected structured while body, got {:?}", other),
    }
}

#[test]
fn test_if_stmt_parses_then_and_optional_else_with_nested_control_node() {
    // Regression: `if` was listed in ACTION_BODY_STARTERS but had no parser function.
    let input = r#"package P {
action def Run {
  attribute x : Integer;
  if x > 0 {
    decide x;
  } else {
    assign x := 0;
  }
  if x > 0 {
    assign x := x - 1;
  }
}
}"#;
    let result = parse(input).expect("parse should succeed");
    let body_elements = action_def_body_elements(&result);
    let if_stmts: Vec<_> = body_elements
        .iter()
        .filter_map(|el| match el {
            sysml_v2_parser::ast::ActionDefBodyElement::IfStmt(i) => Some(&i.value),
            _ => None,
        })
        .collect();
    assert_eq!(
        if_stmts.len(),
        2,
        "expected two IfStmt nodes, got {:?}",
        body_elements
    );

    let with_else = &if_stmts[0];
    match &with_else.then_body {
        sysml_v2_parser::ast::ActionDefBody::Brace { elements } => {
            assert!(
                elements.iter().any(|el| matches!(
                    el.value,
                    sysml_v2_parser::ast::ActionDefBodyElement::DecisionStmt(_)
                )),
                "then-body should retain the nested decide control node (proves real recursion), got {:?}",
                elements
            );
        }
        other => panic!("expected structured then-body, got {:?}", other),
    }
    assert!(
        with_else.else_body.is_some(),
        "expected an else-body to be present"
    );
    match with_else.else_body.as_ref().unwrap() {
        sysml_v2_parser::ast::ActionDefBody::Brace { elements } => {
            assert!(elements.iter().any(|el| matches!(
                el.value,
                sysml_v2_parser::ast::ActionDefBodyElement::Assign(_)
            )));
        }
        other => panic!("expected structured else-body, got {:?}", other),
    }

    let without_else = &if_stmts[1];
    assert!(
        without_else.else_body.is_none(),
        "expected no else-body for the second if"
    );
}

#[test]
fn test_transition_trigger_accept_supports_via_port() {
    // Regression: the transition TRIGGER form (`first source accept X ... then target;`) had
    // no `via` support at all — only the `do`-effect `accept`/`send` forms did. Real spec
    // examples like `accept TurnOn via commPort` failed to parse the `via` clause.
    let input = r#"package P {
state def S {
  state Idle;
  state Running;
  transition first Idle accept StartPressed via commPort then Running;
  transition second first Idle accept evt : StartEvent via commPort then Running;
}
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
    let state_def = match &elements[0].value {
        PackageBodyElement::StateDef(sd) => &sd.value,
        other => panic!("expected state def, got {:?}", other),
    };
    let transitions: Vec<_> = match &state_def.body {
        sysml_v2_parser::ast::StateDefBody::Brace { elements } => elements
            .iter()
            .filter_map(|el| match &el.value {
                sysml_v2_parser::ast::StateDefBodyElement::Transition(t) => Some(&t.value),
                _ => None,
            })
            .collect(),
        other => panic!("expected state def brace body, got {:?}", other),
    };
    assert_eq!(transitions.len(), 2);

    let shorthand_accept = transitions[0].accept.as_ref().expect("shorthand accept");
    match shorthand_accept {
        sysml_v2_parser::ast::TransitionAccept::Shorthand(expr, via) => {
            assert!(matches!(&expr.value, Expression::FeatureRef(n) if n == "StartPressed"));
            let via = via
                .as_ref()
                .expect("expected via clause on shorthand accept");
            assert!(matches!(&via.value, Expression::FeatureRef(n) if n == "commPort"));
        }
        other => panic!("expected shorthand accept, got {:?}", other),
    }

    let typed_accept = transitions[1].accept.as_ref().expect("typed accept");
    match typed_accept {
        sysml_v2_parser::ast::TransitionAccept::Payload(payload, via) => {
            assert_eq!(payload.name, "evt");
            assert_eq!(payload.type_name.as_deref(), Some("StartEvent"));
            let via = via.as_ref().expect("expected via clause on typed accept");
            assert!(matches!(&via.value, Expression::FeatureRef(n) if n == "commPort"));
        }
        other => panic!("expected typed accept, got {:?}", other),
    }
}
