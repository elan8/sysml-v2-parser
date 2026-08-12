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
        let pkg = match &result.document.root.elements[0].value {
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
fn fixture_anonymous_actor_in_use_case_parses_without_missing_member_name() {
    let input = fixture("missing-semicolon-false-positive-name.sysml");
    let (result, elements) = package_elements(&input);

    assert!(
        result.errors.is_empty(),
        "anonymous `actor : User` is valid SysML; unexpected errors: {:?}",
        result.errors
    );
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
        .any(|e| matches!(e.value, UseCaseDefBodyElement::ActorUsage(_))));
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
        .document
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
    assert!(result.document.root.elements.iter().any(|element| {
        matches!(
            element.value,
            RootElement::Member(ref member)
                if matches!(member.value, PackageBodyElement::Error(_))
        )
    }));
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
        result
            .document
            .qualified_reference(expose_target.reference)
            .expect("expose target reference")
            .authored_text(),
        "SurveillanceDrone.SurveillanceQuadrotorDrone",
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
        !result
            .errors
            .iter()
            .any(|e| { matches!(e.code.as_deref(), Some("expected_keyword")) }),
        "malformed package body should not cascade as top-level errors: {:?}",
        result.errors
    );
    assert!(result
        .document
        .root
        .elements
        .iter()
        .any(|e| match &e.value {
            RootElement::Package(pkg) => pkg.value.identification.simple_name() == Some("Later"),
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
    let parse_err = parse_with_diagnostics("package P { part def A { part wheel: ; } }");
    let parse_err_entry = parse_err
        .errors
        .iter()
        .find(|e| e.code.as_deref() == Some("missing_type_reference"))
        .expect("missing type reference diagnostic expected");
    assert_eq!(
        parse_err_entry.category,
        Some(DiagnosticCategory::ParseError)
    );

    // `#fmeaspec requirement req1 { }` is now fully supported at package level as a
    // PrefixMetadataMember-style tag on the following `requirement` member
    // (PARSER_BACKLOG_ROADMAP.md §6); use a form still unsupported (a typed short-name tag
    // followed by anything other than `;`/`{`/`about`) to exercise this diagnostic.
    let unsupported = parse_with_diagnostics("package P { #tag : Foo::Bar::Baz weirdstuff; }");
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
fn fixture_reference_usage_in_part_def_parses_without_bare_feature_diagnostic() {
    // SysML §7.6.4: a usage without a kind keyword is a (default) reference usage.
    let input = fixture("bare-feature-in-part-def.sysml");
    let result = parse_with_diagnostics(&input);
    assert!(
        result.errors.is_empty(),
        "DefaultReferenceUsage `Capacity : Real;` is valid; unexpected errors: {:?}",
        result.errors
    );
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let part = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p) => Some(&p.value),
            _ => None,
        })
        .expect("part def");
    let PartDefBody::Brace { elements } = &part.body else {
        panic!("expected part body");
    };
    assert!(
        elements.iter().any(|e| matches!(
            &e.value,
            PartDefBodyElement::DefaultReferenceUsage(u) if u.value.name == "Capacity"
        )),
        "bare `Capacity : Real;` should be DefaultReferenceUsage, got {:?}",
        elements
            .iter()
            .map(|e| format!("{:?}", e.value))
            .collect::<Vec<_>>()
    );
}

/// Regression for https://github.com/elan8/sysml-v2-parser/issues/1
///
/// Multi-line `/** ... */` inside a part def body must not treat a continuation line that
/// looks like `Ident: prose` as a diagnostic.
#[test]
fn multiline_block_comment_colon_line_parses_cleanly_in_part_def() {
    let input = r#"package P {
    part def C {
        /** first line
            Optional: a profile may state the rate */
        attribute x : String;
    }
}
"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "probe should parse cleanly with diagnostics: {:?}",
        result.errors
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
        .document
        .root
        .elements
        .iter()
        .filter_map(|e| match &e.value {
            RootElement::Package(p) => Some(p.value.identification.simple_name()),
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
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace {
        elements: pkg_elements,
    } = &pkg.body
    else {
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
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, PackageBodyElement::PartDef(_))),
        "part def should parse despite malformed bind connect body"
    );
    assert!(
        !result
            .errors
            .iter()
            .any(|e| { matches!(e.code.as_deref(), Some("expected_keyword")) }),
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
        view_body
            .iter()
            .any(|e| matches!(e.value, sysml_v2_parser::ast::ViewDefBodyElement::Error(_))),
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
        elements
            .iter()
            .any(|e| matches!(e.value, PackageBodyElement::PartDef(_))),
        "sibling part def after malformed constraint should still parse"
    );

    // Error surfaces as a diagnostic.
    assert!(
        !result.errors.is_empty(),
        "at least one diagnostic expected for malformed constraint def body"
    );
}

#[test]
fn far_field_comment_bracket_does_not_poison_diagnostic() {
    // GH-18 (Problem 1): a bracketed doc-comment far below the real error (a `[ ]` TODO marker,
    // not a unit reference) must not override the diagnostic for the actual bad token with a
    // misleading "expected unit name inside '[ ]'" message.
    let input = fixture("far-field-comment-bracket-does-not-poison-diagnostic.sysml");
    let result = parse_with_diagnostics(&input);

    assert!(
        !result
            .errors
            .iter()
            .any(|e| e.code.as_deref() == Some("invalid_unit_reference")),
        "a bracket-like token inside a comment must not be misclassified as a unit reference error: {:?}",
        result.errors
    );
    assert!(
        result.errors.iter().any(|e| e.line == Some(3)),
        "the real error site (line 3) should be reported: {:?}",
        result.errors
    );
}

#[test]
fn far_field_comment_does_not_poison_missing_expression_diagnostic() {
    // GH-29: `missing_expression_after_operator_diagnostic` scanned the unbounded rest of the
    // file for `= ;`/` then ;`/etc. patterns, so an unrelated comment far below the real error
    // (containing `= ;`) could override the true diagnostic for a genuinely malformed `bind`
    // expression with a misleading "expected expression after '='" message.
    let input = fixture("far-field-comment-poisons-missing-expression-diagnostic.sysml");
    let result = parse_with_diagnostics(&input);

    assert!(
        !result
            .errors
            .iter()
            .any(|e| e.code.as_deref() == Some("missing_expression_after_operator")),
        "an '= ;'-like substring inside an unrelated comment must not be misclassified as a \
         missing expression after '=': {:?}",
        result.errors
    );
    assert!(
        result.errors.iter().any(|e| e.line == Some(3)),
        "the real error site (line 3) should be reported: {:?}",
        result.errors
    );
}

#[test]
fn unrecognized_identifier_is_not_reported_as_a_keyword() {
    // GH-18 (Problem 2): `test` is an ordinary identifier, not a SysML keyword, so it must not be
    // reported as "unexpected keyword" -- that would wrongly imply it's valid-but-unsupported
    // syntax rather than an input defect.
    let input = "package P { test; }";
    let result = parse_with_diagnostics(input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(
        err.code.as_deref(),
        Some("unrecognized_declaration_in_scope")
    );
    assert!(err.message.contains("unrecognized declaration `test`"));
    assert!(!err.message.contains("keyword"));
}

#[test]
fn misused_real_keyword_is_still_reported_as_unexpected_keyword() {
    // GH-18 (Problem 2, contrast case): a genuine SysML keyword used somewhere it isn't valid in
    // this scope is a true grammar-context mismatch, so `unexpected_keyword_in_scope` (with the
    // "unexpected keyword" wording) is still correct here.
    let input = "package P { then; }";
    let result = parse_with_diagnostics(input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(err.code.as_deref(), Some("unexpected_keyword_in_scope"));
    assert!(err.message.contains("unexpected keyword `then`"));
}

#[test]
fn bare_comma_sequence_reports_targeted_diagnostic() {
    // GH-18 (Problem 3): `part :>> readings = a, b;` has no sequence brackets, so the expression
    // parser stops at the comma. Recovery should point at the missing `( ... )` instead of a
    // generic "unexpected token in part definition body" message.
    let input = "package P { part def A { part :>> readings = a, b; } }";
    let result = parse_with_diagnostics(input);

    assert_eq!(
        result.errors.len(),
        1,
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let err = &result.errors[0];
    assert_eq!(err.code.as_deref(), Some("bare_comma_in_feature_value"));
    assert!(err.message.contains("use sequence brackets '(a, b)'"));
    assert!(err
        .suggestion
        .as_deref()
        .is_some_and(|s| s.contains("= (a, b)")));
}

#[test]
fn interface_def_body_recovery_surfaces_a_diagnostic() {
    // GH-51: `interface_def_body` used to be a hand-rolled `many0` loop that fell back to
    // `advance_to_closing_brace` with no diagnostic at all when an element failed to parse.
    // Routed through the same `parse_structured_brace_members` + recovery-node machinery
    // `connection_member_body` already used.
    let input = "package P { interface def I { this is not valid sysml at all; } }";
    let result = parse_with_diagnostics(input);

    assert!(
        !result.errors.is_empty(),
        "malformed interface def body content must surface a diagnostic, not be silently dropped"
    );
    assert_eq!(
        result.errors[0].code.as_deref(),
        Some("unrecognized_declaration_in_scope")
    );
}

#[test]
fn interface_def_body_recovery_works_nested_in_a_part_def() {
    // Same as above, but nested -- `PartDefBodyElement::InterfaceDef` previously had no dispatch
    // arm in `collect_errors.rs` either (see next test for the same pre-existing gap on
    // `connection_def`), so even a fixed `interface_def_body` alone would not have surfaced this.
    let input = "package P { part def PP { interface def I { this is not valid at all; } } }";
    let result = parse_with_diagnostics(input);

    assert!(
        !result.errors.is_empty(),
        "nested interface def recovery must also surface a diagnostic: {:?}",
        result.errors
    );
}

#[test]
fn recovery_inside_metadata_body_reaches_document_diagnostics() {
    // Metadata bodies use the shared attribute-body grammar. The metadata member itself parses
    // successfully even when that nested grammar inserts an Error node, so diagnostics traversal
    // must descend through the metadata wrapper rather than only inspect its containing package.
    let input = "package P { #audit { attribute broken: ; } part def StillParsedAfterRecovery; }";
    let result = parse_with_diagnostics(input);

    assert!(
        !result.errors.is_empty(),
        "recovery nested in a metadata body must reach document diagnostics"
    );
}

#[test]
fn connection_def_body_recovery_diagnostics_reach_parse_with_diagnostics() {
    // GH-51 (found while fixing the issue, not the issue's own repro): `connection_def_body`
    // already generated a proper `ConnectionDefBodyElement::Error` recovery node via
    // `connection_def_body_recovery`, but `collect_errors.rs` had no dispatch arm for
    // `PackageBodyElement::ConnectionDef`/`PartDefBodyElement::ConnectionDef`/
    // `PartUsageBodyElement::ConnectionDef` at all, so the diagnostic never reached
    // `parse_with_diagnostics`'s `result.errors` regardless of nesting -- confirmed via a minimal
    // repro before this fix. Both connection and interface defs now use the same collection path.
    let input = "package P { connection def C { this is not valid at all; } }";
    let result = parse_with_diagnostics(input);

    assert!(
        !result.errors.is_empty(),
        "connection def recovery diagnostics must reach parse_with_diagnostics: {:?}",
        result.errors
    );
}

#[test]
fn valid_interface_def_body_still_parses_without_diagnostics() {
    // No regression: the interface_def_body recovery-machinery change must not affect legitimate
    // content, including the `~` conjugation / per-endpoint-multiplicity capabilities GH-33 added.
    let input = "package P {\nport def PowerPort;\ninterface def I {\nend p1 : ~PowerPort;\nend p2 : PowerPort;\nconnect p1 to p2;\n}\n}";
    let result = parse_with_diagnostics(input);

    assert!(
        result.errors.is_empty(),
        "valid interface def body should not report diagnostics: {:?}",
        result.errors
    );
}
