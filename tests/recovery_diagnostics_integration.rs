use std::fs;
use std::path::PathBuf;

use sysml_v2_parser::ast::{
    PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement, RequirementDefBody,
    RequirementDefBodyElement, RootElement, StateDefBody, StateDefBodyElement, UseCaseDefBody,
    UseCaseDefBodyElement, ViewBody, ViewBodyElement,
};
use sysml_v2_parser::{parse_with_diagnostics, DiagnosticCategory};

fn fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);
    fs::read_to_string(path)
        .expect("fixture should be readable")
        .replace("\r\n", "\n")
        .replace('\r', "\n")
}

fn package_elements(
    input: &str,
) -> (
    sysml_v2_parser::ParseResult,
    Vec<sysml_v2_parser::ast::Node<PackageBodyElement>>,
) {
    let result = parse_with_diagnostics(input);
    let elements = {
        let pkg = match &result.root.elements[0].value {
            RootElement::Package(p) => &p.value,
            _ => panic!("expected package"),
        };
        let PackageBody::Brace { elements } = &pkg.body else {
            panic!("expected brace body");
        };
        elements.clone()
    };
    (result, elements)
}

#[test]
fn fixture_missing_semicolon_reports_specific_diagnostic_and_keeps_siblings() {
    let input = fixture("missing-semicolon-true-positive.sysml");
    let (result, elements) = package_elements(&input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(err.line, Some(3));
    assert_eq!(err.code.as_deref(), Some("missing_semicolon"));
    assert!(err
        .found
        .as_deref()
        .is_some_and(|found| found.contains("exhibit state s : S")));
    let part = elements
        .iter()
        .find_map(|element| match &element.value {
            PackageBodyElement::PartDef(part)
                if part.value.identification.name.as_deref() == Some("A") =>
            {
                Some(&part.value)
            }
            _ => None,
        })
        .expect("expected part definition A");
    let PartDefBody::Brace { elements } = &part.body else {
        panic!("expected part definition brace body");
    };
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, PartDefBodyElement::Error(_))));
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, PartDefBodyElement::PartUsage(_))));
}

#[test]
fn fixture_missing_name_does_not_fall_back_to_missing_semicolon() {
    let input = fixture("missing-semicolon-false-positive-name.sysml");
    let (result, elements) = package_elements(&input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(err.line, Some(3));
    assert_eq!(err.code.as_deref(), Some("missing_member_name"));
    assert_ne!(err.code.as_deref(), Some("missing_semicolon"));
    let use_case = elements
        .iter()
        .find_map(|element| match &element.value {
            PackageBodyElement::UseCaseDef(use_case) => Some(&use_case.value),
            _ => None,
        })
        .expect("expected use case definition");
    let UseCaseDefBody::Brace { elements } = &use_case.body else {
        panic!("expected use case brace body");
    };
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, UseCaseDefBodyElement::Error(_))));
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, UseCaseDefBodyElement::Objective(_))));
}

#[test]
fn fixture_missing_type_does_not_fall_back_to_missing_semicolon() {
    let input = fixture("missing-semicolon-false-positive-type.sysml");
    let (result, elements) = package_elements(&input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(err.line, Some(3));
    assert_eq!(err.code.as_deref(), Some("missing_type_reference"));
    assert_ne!(err.code.as_deref(), Some("missing_semicolon"));
    let requirement = elements
        .iter()
        .find_map(|element| match &element.value {
            PackageBodyElement::RequirementDef(requirement) => Some(&requirement.value),
            _ => None,
        })
        .expect("expected requirement definition");
    let RequirementDefBody::Brace { elements } = &requirement.body else {
        panic!("expected requirement brace body");
    };
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, RequirementDefBodyElement::Error(_))));
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, RequirementDefBodyElement::RequireConstraint(_))));
}

#[test]
fn fixture_single_bad_line_does_not_cascade_into_later_valid_lines() {
    let input = fixture("cascade-single-bad-line.sysml");
    let (result, elements) = package_elements(&input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(err.line, Some(2));
    assert_eq!(
        err.code.as_deref(),
        Some("unsupported_annotation_syntax"),
        "bad line should be reported as unsupported annotation syntax"
    );
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, PackageBodyElement::PartDef(_))));
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, PackageBodyElement::ActionDef(_))));
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, PackageBodyElement::RequirementDef(_))));
}

#[test]
fn fixture_nested_bad_block_recovers_inside_part_and_keeps_outer_siblings() {
    let input = fixture("cascade-bad-block-then-valid-siblings.sysml");
    let (result, elements) = package_elements(&input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(err.line, Some(3));
    assert_eq!(err.code.as_deref(), Some("missing_type_reference"));

    let broken = elements
        .iter()
        .find_map(|element| match &element.value {
            PackageBodyElement::PartDef(part)
                if part.value.identification.name.as_deref() == Some("Broken") =>
            {
                Some(&part.value)
            }
            _ => None,
        })
        .expect("expected Broken part");
    let PartDefBody::Brace { elements } = &broken.body else {
        panic!("expected Broken brace body");
    };
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, PartDefBodyElement::Error(_))));
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, PartDefBodyElement::Ref(_))));
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, PartDefBodyElement::Ref(_))));
    assert!(result
        .root
        .elements
        .iter()
        .any(|e| matches!(e.value, RootElement::Package(_))));
    assert!(package_elements(&input)
        .1
        .iter()
        .any(|e| matches!(e.value, PackageBodyElement::ActionDef(_))));
}

#[test]
fn fixture_unmatched_brace_reports_local_eof_error_without_extra_recovery_noise() {
    let input = fixture("unmatched-brace-locality.sysml");
    let result = parse_with_diagnostics(&input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(err.code.as_deref(), Some("missing_closing_brace"));
    assert!(
        err.line.is_some_and(|line| line >= 5),
        "EOF brace diagnostic should stay near the end: {:?}",
        err
    );
    assert!(
        result.root.elements.is_empty()
            || result
                .root
                .elements
                .iter()
                .any(|e| matches!(e.value, RootElement::Package(_)))
    );
}

#[test]
fn fixture_expose_feature_chain_parses_without_separator_diagnostic() {
    let input = fixture("invalid-qualified-name-separator.sysml");
    let (result, elements) = package_elements(&input);

    assert!(
        result.errors.is_empty(),
        "expose feature chains should parse without invalid_qualified_name_separator: {:?}",
        result.errors
    );
    let view_usage = elements
        .iter()
        .find(|e| matches!(e.value, PackageBodyElement::ViewUsage(_)))
        .expect("view usage");
    let expose_target = match &view_usage.value {
        PackageBodyElement::ViewUsage(view) => match &view.value.body {
            ViewBody::Brace { elements } => elements
                .iter()
                .find_map(|member| match &member.value {
                    ViewBodyElement::Expose(expose) => Some(expose.value.target.clone()),
                    _ => None,
                })
                .expect("expose member"),
            other => panic!("expected brace view body, got {other:?}"),
        },
        other => panic!("expected view usage, got {other:?}"),
    };
    assert_eq!(
        expose_target, "SurveillanceDrone.SurveillanceQuadrotorDrone",
        "feature-chain segments should be preserved in expose target"
    );
}

#[test]
fn fixture_incomplete_bind_expression_reports_missing_expression() {
    let input = fixture("incomplete-bind-expression.sysml");
    let (result, elements) = package_elements(&input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(err.line, Some(3));
    assert_eq!(
        err.code.as_deref(),
        Some("missing_expression_after_operator")
    );
    assert_eq!(
        err.expected.as_deref(),
        Some("binding expression after '='")
    );
    assert!(err
        .found
        .as_deref()
        .is_some_and(|found| found.contains("bind status = ;")));
    let action = elements
        .iter()
        .find_map(|element| match &element.value {
            PackageBodyElement::ActionDef(action)
                if action.value.identification.name.as_deref() == Some("ExecutePatrol") =>
            {
                Some(&action.value)
            }
            _ => None,
        })
        .expect("expected action definition");
    assert!(matches!(
        action.body,
        sysml_v2_parser::ast::ActionDefBody::Brace { .. }
    ));
}

#[test]
fn fixture_missing_body_or_semicolon_reports_declaration_terminator_error() {
    let input = fixture("missing-body-or-semicolon.sysml");
    let (result, elements) = package_elements(&input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(err.line, Some(2));
    assert_eq!(err.code.as_deref(), Some("missing_body_or_semicolon"));
    assert_eq!(
        err.expected.as_deref(),
        Some("';' or '{' after declaration header")
    );
    assert!(err
        .suggestion
        .as_deref()
        .is_some_and(|s| s.contains("part def Wheel")));
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, PackageBodyElement::PartDef(_))));
}

#[test]
fn fixture_unexpected_extra_closing_brace_is_localized() {
    let input = fixture("unexpected-extra-closing-brace.sysml");
    let result = parse_with_diagnostics(&input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(err.line, Some(4));
    assert_eq!(err.code.as_deref(), Some("unexpected_closing_brace"));
    assert_eq!(err.found.as_deref(), Some("}"));
}

#[test]
fn strict_parse_reports_unexpected_trailing_closing_brace() {
    let input = "package P {\npart def A;\n}\n}";
    let err = sysml_v2_parser::parse(input).expect_err("extra closing brace should fail");
    assert_eq!(err.line, Some(4));
    assert_eq!(err.code.as_deref(), Some("unexpected_closing_brace"));
    assert_eq!(err.found.as_deref(), Some("}"));
}

#[test]
fn repeated_recovery_diagnostics_are_summarized_after_first_few() {
    let input = r#"package P {
part def Vehicle {
  part a : A
  part b : B
  part c : C
  part d : D
  part e : E
}
action def Done { }
}"#;
    let result = parse_with_diagnostics(input);
    let missing_semicolons = result
        .errors
        .iter()
        .filter(|e| e.code.as_deref() == Some("missing_semicolon"))
        .count();
    assert_eq!(
        missing_semicolons, 1,
        "only the first cascade diagnostic should remain: {:?}",
        result.errors
    );
    let summary = result
        .errors
        .iter()
        .find(|e| e.code.as_deref() == Some("recovery_cascade_suppressed"))
        .expect("expected cascade summary diagnostic");
    assert_eq!(
        summary.severity,
        Some(sysml_v2_parser::DiagnosticSeverity::Warning)
    );
    assert!(
        summary.message.contains("suppressed"),
        "summary should explain suppression: {:?}",
        summary
    );

    let (_, elements) = package_elements(input);
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, PackageBodyElement::ActionDef(_))),
        "later valid package siblings should still parse"
    );
}

#[test]
fn malformed_root_package_body_recovers_without_top_level_cascade() {
    let input = r#"package Broken {
  part def BatteryLevelComputer {
    exhibit state BatteryLevelComputerStates {
      in ref maxBatCap = batteryCapacity;
    }
  }
  state def BatteryLevelComputerStates {
    entry; then x;
    state x {
      entry act { batCap; maxBatCap; computedColor; }
    }
  }
}
package Later {
  part def Good;
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result
            .errors
            .iter()
            .any(|e| { e.code.as_deref() == Some("invalid_bare_identifier_in_state_body") }),
        "malformed state body members should surface scoped recovery diagnostics: {:?}",
        result.errors
    );
    assert!(
        !result.errors.iter().any(|e| {
            matches!(
                e.code.as_deref(),
                Some("illegal_top_level_definition") | Some("expected_keyword")
            )
        }),
        "malformed package body should not cascade as top-level errors: {:?}",
        result.errors
    );
    assert!(result.root.elements.iter().any(|e| match &e.value {
        RootElement::Package(pkg) => pkg.value.identification.name.as_deref() == Some("Later"),
        _ => false,
    }));
}

#[test]
fn fixture_invalid_typing_operator_reports_specific_fix() {
    let input = fixture("invalid-typing-operator.sysml");
    let (result, elements) = package_elements(&input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(err.line, Some(2));
    assert_eq!(err.code.as_deref(), Some("invalid_typing_operator"));
    assert_eq!(
        err.expected.as_deref(),
        Some("':>' specialization operator")
    );
    assert!(err
        .suggestion
        .as_deref()
        .is_some_and(|s| s.contains(":> BaseVehicle")));
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, PackageBodyElement::PartDef(_))));
}

#[test]
fn fixture_calc_usage_in_part_def_body_parses_without_unexpected_keyword() {
    let input = fixture("calc-usage-in-part-def.sysml");
    let (result, _) = package_elements(&input);

    assert!(
        !result.errors.iter().any(|e| {
            e.code.as_deref() == Some("unexpected_keyword_in_scope") && e.message.contains("calc")
        }),
        "calc usage in part def body should parse: {:?}",
        result.errors
    );
}

#[test]
fn fixture_nested_part_def_typed_usages_no_invalid_typing_operator() {
    let input = fixture("nested-part-def-typed-usages.sysml");
    let (result, _) = package_elements(&input);

    assert!(
        !result
            .errors
            .iter()
            .any(|e| e.code.as_deref() == Some("invalid_typing_operator")),
        "nested part defs with typed usages should not emit invalid_typing_operator: {:?}",
        result.errors
    );
}

#[test]
fn fixture_unexpected_keyword_in_requirement_body_reports_scope_specific_error() {
    let input = fixture("unexpected-keyword-in-requirement-body.sysml");
    let (result, elements) = package_elements(&input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(err.line, Some(3));
    assert_eq!(err.code.as_deref(), Some("unexpected_keyword_in_scope"));
    assert!(err.message.contains("unexpected keyword `then`"));
    let requirement = elements
        .iter()
        .find_map(|element| match &element.value {
            PackageBodyElement::RequirementDef(requirement) => Some(&requirement.value),
            _ => None,
        })
        .expect("expected requirement definition");
    let RequirementDefBody::Brace { elements } = &requirement.body else {
        panic!("expected requirement brace body");
    };
    assert!(elements
        .iter()
        .any(|e| matches!(e.value, RequirementDefBodyElement::RequireConstraint(_))));
}

#[test]
fn diagnostics_include_taxonomy_categories() {
    let parse_err = parse_with_diagnostics("package P { part def A { part: Wheel; } }");
    let parse_err_entry = parse_err
        .errors
        .iter()
        .find(|e| e.code.as_deref() == Some("missing_member_name"))
        .expect("missing member name diagnostic expected");
    assert_eq!(
        parse_err_entry.category,
        Some(DiagnosticCategory::ParseError)
    );

    let unsupported = parse_with_diagnostics("package P { #fmeaspec requirement req1 { } }");
    let unsupported_entry = unsupported
        .errors
        .iter()
        .find(|e| e.code.as_deref() == Some("unsupported_annotation_syntax"))
        .expect("unsupported annotation diagnostic expected");
    assert_eq!(
        unsupported_entry.category,
        Some(DiagnosticCategory::UnsupportedGrammarForm)
    );
}

#[test]
fn invalid_unit_reference_reports_specific_diagnostic() {
    let input = "package P { action def Evaluate { bind measuredMass = []; in result: Real; } }";
    let (result, elements) = package_elements(input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(err.code.as_deref(), Some("invalid_unit_reference"));
    assert_eq!(err.expected.as_deref(), Some("unit name inside '[ ]'"));
    assert!(err
        .suggestion
        .as_deref()
        .is_some_and(|s| s.contains("[kg]")));

    let action = elements
        .iter()
        .find_map(|element| match &element.value {
            PackageBodyElement::ActionDef(action)
                if action.value.identification.name.as_deref() == Some("Evaluate") =>
            {
                Some(&action.value)
            }
            _ => None,
        })
        .expect("expected action definition Evaluate");
    let sysml_v2_parser::ast::ActionDefBody::Brace { elements } = &action.body else {
        panic!("expected action definition brace body");
    };
    assert!(elements.iter().any(|e| matches!(
        e.value,
        sysml_v2_parser::ast::ActionDefBodyElement::Error(_)
    )));
    assert!(elements.iter().any(|e| matches!(
        e.value,
        sysml_v2_parser::ast::ActionDefBodyElement::InOutDecl(_)
    )));
}

#[test]
fn fixture_requirement_def_id_keyword_reports_short_name_hint() {
    let input = fixture("requirement-def-id-keyword-dialect.sysml");
    let result = parse_with_diagnostics(&input);
    let err = result
        .errors
        .iter()
        .find(|e| {
            e.code.as_deref() == Some("invalid_requirement_short_name_syntax")
                || e.code.as_deref() == Some("missing_body_or_semicolon")
        })
        .expect("expected requirement header diagnostic");
    assert!(
        err.message.contains("short name")
            || err.suggestion.as_deref().is_some_and(|s| s.contains('<')),
        "expected short-name guidance: {:?}",
        err
    );
}

#[test]
fn fixture_bare_feature_in_part_def_reports_attribute_hint() {
    let input = fixture("bare-feature-in-part-def.sysml");
    let result = parse_with_diagnostics(&input);
    let err = result
        .errors
        .iter()
        .find(|e| e.code.as_deref() == Some("bare_feature_declaration_in_part_def"))
        .expect("expected part-def feature diagnostic");
    assert!(
        err.message.contains("Capacity")
            || err
                .suggestion
                .as_deref()
                .is_some_and(|s| s.contains("attribute") || s.contains("Capacity")),
        "expected Capacity-related guidance: {:?}",
        err
    );
}

#[test]
fn fixture_glued_package_member_parses_without_separator_diagnostic() {
    let input = fixture("glued-package-member.sysml");
    let result = parse_with_diagnostics(&input);
    assert!(
        result.errors.is_empty(),
        "glued `}}package` is valid surface syntax; unexpected errors: {:?}",
        result.errors
    );
    let packages: Vec<_> = result
        .root
        .elements
        .iter()
        .filter_map(|e| match &e.value {
            RootElement::Package(p) => Some(p.value.identification.name.as_deref()),
            _ => None,
        })
        .collect();
    assert_eq!(packages, vec![Some("A"), Some("B")]);
}

#[test]
fn state_ref_brace_body_recovers_without_aborting_siblings() {
    let input = r#"package P {
  state def S {
    ref r {
      Ready;
      entry;
    }
    transition t then S;
  }
  part def Good;
}"#;
    let result = parse_with_diagnostics(input);
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements: pkg_elements } = &pkg.body else {
        panic!("expected brace body");
    };
    let state_def = pkg_elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::StateDef(s) => Some(&s.value),
            _ => None,
        })
        .expect("state def should be present");
    let StateDefBody::Brace { elements } = &state_def.body else {
        panic!("expected state brace body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, StateDefBodyElement::Transition(_))),
        "transition after opaque ref body should still parse"
    );
    assert!(
        pkg_elements
            .iter()
            .any(|e| matches!(e.value, PackageBodyElement::PartDef(_))),
        "sibling part def should still parse"
    );
}

#[test]
fn part_usage_bind_brace_body_recovers_without_aborting_siblings() {
    let input = r#"package P {
  part def Host {
    port p;
    part child {
      port q;
      bind child.q = p {
        connect q to p;
        orphan junk;
        connect q to p;
      }
    }
  }
}"#;
    let result = parse_with_diagnostics(input);
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace body");
    };
    assert!(
        elements.iter().any(|e| matches!(e.value, PackageBodyElement::PartDef(_))),
        "part def should parse despite malformed bind connect body"
    );
    assert!(
        !result.errors.iter().any(|e| {
            matches!(
                e.code.as_deref(),
                Some("illegal_top_level_definition") | Some("expected_keyword")
            )
        }),
        "malformed bind body should not cascade as top-level errors: {:?}",
        result.errors
    );
}

#[test]
fn fixture_anonymous_actor_in_requirement_has_no_missing_member_name() {
    let input = fixture("anonymous-actor-in-requirement.sysml");
    let result = parse_with_diagnostics(&input);
    assert!(
        !result
            .errors
            .iter()
            .any(|e| e.code.as_deref() == Some("missing_member_name")),
        "anonymous `actor : Type` is valid; errors: {:?}",
        result.errors
    );
}

#[test]
fn fixture_enum_usage_in_part_def_has_no_unexpected_keyword() {
    let input = fixture("enum-fill-level-in-part-def.sysml");
    let result = parse_with_diagnostics(&input);
    assert!(
        !result.errors.iter().any(|e| {
            e.code.as_deref() == Some("unexpected_keyword_in_scope") && e.message.contains("enum")
        }),
        "enumeration usage in part def is allowed; errors: {:?}",
        result.errors
    );
}

#[test]
fn view_def_recovery_inserts_error_node_and_keeps_later_render() {
    // A malformed token inside a view def body must produce a ParseErrorNode
    // and not abort parsing of the subsequent `render` member.
    let input = r#"package P {
    view def MyView {
        @@@ invalid_token_here;
        render r : SomeRendering;
    }
}"#;
    let (result, elements) = package_elements(input);

    let view = match &elements[0].value {
        PackageBodyElement::ViewDef(v) => &v.value,
        other => panic!("expected view def, got {other:?}"),
    };
    let view_body = match &view.body {
        sysml_v2_parser::ast::ViewDefBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };

    // The body must contain an Error node for the bad token.
    assert!(
        view_body.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::ViewDefBodyElement::Error(_)
        )),
        "view def body should contain a ParseErrorNode for the bad token"
    );

    // The `render` member after the bad token must still parse.
    assert!(
        view_body.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::ViewDefBodyElement::ViewRendering(_)
        )),
        "render member after bad token should still be parsed"
    );

    // The error must surface as a diagnostic (not silently dropped).
    assert!(
        !result.errors.is_empty(),
        "at least one diagnostic expected for malformed view def body"
    );
}

#[test]
fn constraint_def_recovery_inserts_error_node_and_keeps_later_sibling() {
    // A malformed token in a constraint def body must produce a ParseErrorNode
    // and not abort parsing of the subsequent constraint or sibling.
    let input = r#"package P {
    constraint def MyConstraint {
        @@@ bad_token;
        thrust >= weight;
    }
    part def Sibling;
}"#;
    let (result, elements) = package_elements(input);

    let constraint = match &elements[0].value {
        PackageBodyElement::ConstraintDef(c) => &c.value,
        other => panic!("expected constraint def, got {other:?}"),
    };
    let c_body = match &constraint.body {
        sysml_v2_parser::ast::ConstraintDefBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };

    // Error node must be present for the bad token.
    assert!(
        c_body.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::ConstraintDefBodyElement::Error(_)
        )),
        "constraint def body should contain a ParseErrorNode"
    );

    // Expression member after the bad token should still parse.
    assert!(
        c_body.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::ConstraintDefBodyElement::Expression(_)
        )),
        "expression after bad token should still parse"
    );

    // Sibling part def after the constraint must parse.
    assert!(
        elements.iter().any(|e| matches!(e.value, PackageBodyElement::PartDef(_))),
        "sibling part def after malformed constraint should still parse"
    );

    // Error surfaces as a diagnostic.
    assert!(
        !result.errors.is_empty(),
        "at least one diagnostic expected for malformed constraint def body"
    );
}
