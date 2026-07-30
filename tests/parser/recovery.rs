//! Parser tests: recovery

use sysml_v2_parser::ast::*;
use sysml_v2_parser::{parse, parse_with_diagnostics};

#[test]
fn test_parse_with_diagnostics_partial_ast_and_multiple_errors() {
    // One valid element, two invalid lines, then another valid element. Recovery should collect
    // two errors and still produce a partial AST with both valid packages.
    let input = "package Foo;\nnot valid\nalso bad\npackage Bar;";
    let result = parse_with_diagnostics(input);
    assert!(!result.is_ok(), "should have parse errors");
    assert_eq!(result.errors.len(), 2, "should report two parse errors");
    assert_eq!(
        result.root.elements.len(),
        2,
        "partial AST should contain both valid packages"
    );
    // First element is Foo, second is Bar
    let names: Vec<&str> = result
        .root
        .elements
        .iter()
        .filter_map(|n| {
            if let RootElement::Package(p) = &n.value {
                p.value.identification.name.as_deref()
            } else {
                None
            }
        })
        .collect();
    assert_eq!(names, ["Foo", "Bar"]);

    // Error quality: each error should have "found" snippet and expected context
    for err in &result.errors {
        assert!(
            err.found.is_some(),
            "error should have 'found' snippet: {}",
            err.message
        );
        assert!(
            err.expected.is_some(),
            "error should have 'expected' context: {}",
            err.message
        );
        assert!(
            err.expected
                .as_deref()
                .is_some_and(|e| e.contains("package") || e.contains("namespace")),
            "expected should mention package or namespace: {:?}",
            err.expected
        );
        assert!(err.code.is_some(), "error should have a code");
    }
    // First error is at "not valid"
    assert!(
        result.errors[0]
            .found
            .as_deref()
            .is_some_and(|f| f.contains("not")),
        "first error found should mention invalid token: {:?}",
        result.errors[0].found
    );
}

#[test]
fn test_parse_error_expected_end_of_input_has_found() {
    // Trailing text after valid packages: parse succeeds for "package Foo; package Bar;" then rest "garbage" triggers "expected end of input"
    let input = "package Foo; package Bar; garbage";
    let result = parse(input);
    let err = result.unwrap_err();
    assert!(
        err.message.contains("expected end of input"),
        "error should be 'expected end of input': {}",
        err
    );
    assert!(
        err.found.is_some(),
        "expected end of input error should have 'found': {}",
        err
    );
    assert!(
        err.found.as_deref().is_some_and(|f| f.contains("garbage")),
        "found should show trailing text: {:?}",
        err.found
    );
    assert_eq!(err.code.as_deref(), Some("expected_end_of_input"));
}

#[test]
fn test_parse_error_display_includes_found_and_location() {
    let input = "package Foo;\nxyz";
    let result = parse_with_diagnostics(input);
    let err = &result.errors[0];
    let display = err.to_string();
    assert!(
        display.contains("line"),
        "Display should include line number"
    );
    assert!(
        err.found.as_ref().is_some_and(|f| display.contains(f)),
        "Display should include found snippet: {}",
        display
    );
}

#[test]
fn test_action_def_is_not_parsed_as_action_usage() {
    let input = r#"package P {
action def ExecutePatrol {
}
}"#;
    let root = sysml_v2_parser::parse_root(input).expect("should parse");
    let pkg = match &root.elements[0].value {
        sysml_v2_parser::ast::RootElement::Package(p) => &p.value,
        _ => panic!("expected package root element"),
    };
    let sysml_v2_parser::ast::PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace body");
    };
    let first = elements.first().expect("expected a body element");
    match &first.value {
        sysml_v2_parser::ast::PackageBodyElement::ActionDef(a) => {
            assert_eq!(
                a.value.identification.name.as_deref(),
                Some("ExecutePatrol"),
                "expected ActionDef name ExecutePatrol"
            );
        }
        other => panic!("expected ActionDef, got {:?}", other),
    }
}

#[test]
fn test_parse_with_diagnostics_recovers_and_reports_later_errors() {
    // Intentionally malformed body statements (unknown keywords) followed by a valid member.
    // The goal is to ensure we report multiple diagnostics and still parse later valid elements.
    let input = r#"package P {
action def A {
  badstmt {};
  badstmt2 {};
}
action def B { }
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.len() >= 2,
        "expected 2+ diagnostics, got: {:?}",
        result.errors
    );

    // Ensure we still parsed the later action def `B`.
    let pkg = match &result.root.elements[0].value {
        sysml_v2_parser::ast::RootElement::Package(p) => &p.value,
        _ => panic!("expected package root element"),
    };
    let sysml_v2_parser::ast::PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace body");
    };
    let has_b = elements.iter().any(|e| match &e.value {
        sysml_v2_parser::ast::PackageBodyElement::ActionDef(a) => {
            a.value.identification.name.as_deref() == Some("B")
        }
        _ => false,
    });
    assert!(has_b, "expected later ActionDef `B` to still be parsed");
}

#[test]
fn test_package_body_recovery_skips_annotated_member_and_keeps_later_sibling() {
    // parse_root now rejects any document containing an embedded recovery placeholder (GH-2):
    // an unsupported annotation is exactly such a placeholder, so strict must reject it even
    // though the annotation itself is legal SysML the parser doesn't fully support yet.
    //
    // `#tag : Type trailing-garbage;` (not `#fmeaspec requirement req1 { }`, which package
    // bodies now fully support as a PrefixMetadataMember-style tag on the following
    // `requirement` member -- PARSER_BACKLOG_ROADMAP.md §6) -- the typed short-name form still
    // has no support for anything after the type besides `;`/`{`/`about`.
    let input = "package P {\n#tag : Foo::Bar::Baz weirdstuff;\npart def Good;\n}";
    let strict_err = parse(input).expect_err("strict should reject the unsupported annotation");
    assert_eq!(
        strict_err.code.as_deref(),
        Some("unsupported_annotation_syntax")
    );

    // parse_with_diagnostics is the entry point meant to tolerate this: it should still recover
    // and keep the later valid sibling, with the annotation surfaced as a diagnostic.
    let result = parse_with_diagnostics(input);
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    assert!(
        elements.iter().any(|e| matches!(e.value, PackageBodyElement::PartDef(_))),
        "later valid sibling should still be present after recovering from annotated unsupported member"
    );
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, PackageBodyElement::Error(_))),
        "recovered package region should be represented explicitly in the AST"
    );
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.code.as_deref() == Some("unsupported_annotation_syntax")),
        "the same diagnostic parse_root rejected on should be reported here too"
    );
}

#[test]
fn test_package_body_recovery_skips_malformed_abstract_part_and_keeps_next_member() {
    // parse_root now rejects documents containing a genuinely malformed member (GH-2): `invalid`
    // is not a valid part-definition body element, so strict must reject the whole document.
    let input = "package P {\nabstract part def Broken { invalid }\npart def Good;\n}";
    let strict_err = parse(input).expect_err("strict should reject the malformed member");
    assert_eq!(
        strict_err.code.as_deref(),
        Some("unexpected_keyword_in_scope")
    );

    // parse_with_diagnostics still recovers and preserves both part declarations.
    let result = parse_with_diagnostics(input);
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let elements = match &pkg.body {
        PackageBody::Brace { elements } => elements,
        _ => panic!("expected brace body"),
    };
    assert_eq!(
        elements
            .iter()
            .filter(|e| matches!(e.value, PackageBodyElement::PartDef(_)))
            .count(),
        2,
        "both part declarations should map to dedicated PartDef nodes"
    );
}

#[test]
fn test_part_def_nested_state_and_attribute_siblings() {
    // Previously `state …` in a part body was only preserved as Other/OpaqueMember.
    // 0.46.0 parses it as a real StateUsage; the later attribute sibling must still parse.
    let input =
        "package P {\npart def Vehicle {\nstate monitor: Mode;\nattribute mass: MassValue;\n}\n}";
    let result = parse_with_diagnostics(input);
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace body");
    };
    let part_def = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p) => Some(&p.value),
            _ => None,
        })
        .expect("part def should be present");
    let sysml_v2_parser::ast::PartDefBody::Brace { elements } = &part_def.body else {
        panic!("expected part def body");
    };
    assert!(
        elements.iter().any(|e| matches!(
            &e.value,
            sysml_v2_parser::ast::PartDefBodyElement::StateUsage(s)
                if s.value.name == "monitor"
        )),
        "nested state usage in part body should be a real StateUsage, not opaque"
    );
    assert!(
        elements.iter().any(|e| matches!(
            &e.value,
            sysml_v2_parser::ast::PartDefBodyElement::AttributeUsage(a)
                if a.value.typing.is_some()
        )),
        "later modeled members should still parse"
    );
}

#[test]
fn test_part_def_recovery_preserves_other_member_and_later_sibling() {
    // Truly unmodeled keyword still lands in Other/Error recovery; later siblings stay intact.
    let input =
        "package P {\npart def Vehicle {\nwidget monitor: Mode;\nattribute mass: MassValue;\n}\n}";
    let result = parse_with_diagnostics(input);
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace body");
    };
    let part_def = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartDef(p) => Some(&p.value),
            _ => None,
        })
        .expect("part def should be present");
    let sysml_v2_parser::ast::PartDefBody::Brace { elements } = &part_def.body else {
        panic!("expected part def body");
    };
    assert!(
        elements.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::PartDefBodyElement::Other(_)
                | sysml_v2_parser::ast::PartDefBodyElement::OpaqueMember(_)
                | sysml_v2_parser::ast::PartDefBodyElement::Error(_)
        )),
        "library-tolerant unmodeled part members should be preserved explicitly"
    );
    assert!(
        elements.iter().any(|e| matches!(
            &e.value,
            sysml_v2_parser::ast::PartDefBodyElement::AttributeUsage(a)
                if a.value.typing.is_some()
        )),
        "later modeled members should still parse"
    );
}

#[test]
fn test_state_def_recovery_no_longer_truncates_body() {
    let input = "package P {\nstate def Machine {\nunknown stuff;\ntransition t then Ready;\n}\n}";
    let result = parse_with_diagnostics(input);
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace body");
    };
    let state_def = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::StateDef(s) => Some(&s.value),
            _ => None,
        })
        .expect("state def should be present");
    let sysml_v2_parser::ast::StateDefBody::Brace { elements } = &state_def.body else {
        panic!("expected state body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(e.value, sysml_v2_parser::ast::StateDefBodyElement::Other(_))),
        "unknown state members should be preserved explicitly instead of truncating the body"
    );
    assert!(
        elements.iter().any(|e| matches!(
            e.value,
            sysml_v2_parser::ast::StateDefBodyElement::Transition(_)
        )),
        "later valid state members should still parse"
    );
}

#[test]
fn test_parse_with_diagnostics_accepts_structured_requirement_attributes() {
    let input = "package P {\nrequirement def R {\nsubject vehicle : Vehicle;\nattribute massActual: MassValue;\nrequire constraint { }\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "structured requirement attributes should not produce recovery diagnostics: {:?}",
        result.errors
    );
}

#[test]
fn test_parse_with_diagnostics_accepts_anonymous_actor_in_use_case_body() {
    let input = "package P {\nuse case def U {\nactor: User;\nobjective { }\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.is_ok(),
        "anonymous actor usage is valid SysML: {:?}",
        result.errors
    );
}

#[test]
fn test_parse_with_diagnostics_reports_missing_subject_type_in_requirement_body() {
    let input = "package P {\nrequirement def R {\nsubject laptop: ;\nrequire constraint { }\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        !result.is_ok(),
        "missing subject type should produce diagnostics"
    );
    let err = result
        .errors
        .iter()
        .find(|e| e.code.as_deref() == Some("missing_type_reference"))
        .expect("expected missing_type_reference diagnostic");
    assert_eq!(err.expected.as_deref(), Some("subject type after ':'"));
    assert!(
        err.suggestion
            .as_deref()
            .is_some_and(|s| s.contains("subject laptop: Laptop;")),
        "diagnostic should show a subject type example fix"
    );
}

#[test]
fn test_parse_with_diagnostics_reports_missing_actor_type_in_use_case_body() {
    let input = "package P {\nuse case def U {\nactor user: ;\nobjective { }\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        !result.is_ok(),
        "missing actor type should produce diagnostics"
    );
    let err = result
        .errors
        .iter()
        .find(|e| e.code.as_deref() == Some("missing_type_reference"))
        .expect("expected missing_type_reference diagnostic");
    assert_eq!(err.expected.as_deref(), Some("actor type after ':'"));
    assert!(
        err.suggestion
            .as_deref()
            .is_some_and(|s| s.contains("actor user: User;")),
        "diagnostic should show an actor type example fix"
    );
}

#[test]
fn test_parse_with_diagnostics_accepts_anonymous_state_in_state_body() {
    let input = "package P {\nstate def Machine {\nstate: Mode;\ntransition t then Ready;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.is_ok(),
        "anonymous `state: Mode` is valid SysML; unexpected errors: {:?}",
        result.errors
    );
}

#[test]
fn test_parse_with_diagnostics_reports_missing_part_type_in_part_body() {
    let input = "package P {\npart def Vehicle {\npart wheel: ;\nattribute mass: MassValue;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        !result.is_ok(),
        "missing part type should produce diagnostics"
    );
    let err = result
        .errors
        .iter()
        .find(|e| e.expected.as_deref() == Some("part type after ':'"))
        .expect("expected part-type diagnostic");
    assert!(
        err.suggestion
            .as_deref()
            .is_some_and(|s| s.contains("part wheel: Wheel;")),
        "diagnostic should show a part type example fix"
    );
}

#[test]
fn test_parse_with_diagnostics_reports_missing_attribute_type_in_part_body() {
    let input = "package P {\npart Vehicle {\nattribute bad : ;\nattribute ok : MassValue;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        !result.is_ok(),
        "missing attribute type should produce diagnostics"
    );
    assert!(
        !result.errors.is_empty(),
        "missing attribute type should be reported"
    );
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    let part = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::PartUsage(p) => Some(&p.value),
            _ => None,
        })
        .expect("part usage should survive recovery");
    let PartUsageBody::Brace { elements } = &part.body else {
        panic!("expected part body");
    };
    assert!(
        elements.iter().any(|e| matches!(
            &e.value,
            PartUsageBodyElement::AttributeUsage(a) if a.value.name == "ok"
        )),
        "later attribute sibling should remain parseable"
    );
}

#[test]
fn test_parse_with_diagnostics_reports_missing_occurrence_type_and_keeps_sibling() {
    let input = "package P {\noccurrence bad defined by ;\npart def Good;\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        !result.is_ok(),
        "missing occurrence type should produce diagnostics"
    );
    assert!(
        !result.errors.is_empty(),
        "missing occurrence type should be reported"
    );
    let pkg = match &result.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected package body");
    };
    assert!(
        elements
            .iter()
            .any(|e| matches!(&e.value, PackageBodyElement::PartDef(p) if p.value.identification.name.as_deref() == Some("Good"))),
        "later package sibling should remain parseable"
    );
}

#[test]
fn test_parse_with_diagnostics_accepts_anonymous_part_in_part_body() {
    let input = "package P {\npart def Vehicle {\npart: Wheel;\nattribute mass: MassValue;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.is_ok(),
        "anonymous `part: Wheel` is valid SysML; unexpected errors: {:?}",
        result.errors
    );
}

#[test]
fn test_parse_with_diagnostics_reports_local_package_recovery() {
    // See test_package_body_recovery_skips_annotated_member_and_keeps_later_sibling for why this
    // uses `#tag : Type trailing-garbage;` rather than `#fmeaspec requirement req1 { }`.
    let input = "package P {\n#tag : Foo::Bar::Baz weirdstuff;\npart def Good;\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        !result.is_ok(),
        "package-level recovery should surface as diagnostics"
    );
    let err = result
        .errors
        .iter()
        .find(|e| e.code.as_deref() == Some("unsupported_annotation_syntax"))
        .expect("expected local package recovery diagnostic");
    assert_eq!(err.line, Some(2));
    assert!(
        err.found.as_deref().is_some_and(|f| f.contains("#tag")),
        "diagnostic should preserve recovered snippet"
    );
    assert!(
        err.message.contains("annotation"),
        "annotation recovery should say why the declaration could not be parsed: {}",
        err.message
    );
    assert_eq!(
        err.severity,
        Some(sysml_v2_parser::DiagnosticSeverity::Warning)
    );
}

#[test]
fn test_parse_with_diagnostics_reports_missing_semicolon_between_package_members() {
    let input = "package P {\npart def A {\nexhibit state s : S\npart b : B;\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        !result.is_ok(),
        "missing semicolon should produce diagnostics"
    );
    let err = result
        .errors
        .iter()
        .find(|e| e.code.as_deref() == Some("missing_semicolon"))
        .expect("expected missing_semicolon diagnostic");
    assert_eq!(err.expected.as_deref(), Some("';'"));
    assert!(
        err.suggestion
            .as_deref()
            .is_some_and(|s| s.contains("Insert ';'")),
        "diagnostic should include a semicolon suggestion"
    );
}

#[test]
fn test_parse_with_diagnostics_accepts_expose_feature_chain() {
    let input = "package Views { view structure: GeneralView { expose SurveillanceDrone.SurveillanceQuadrotorDrone; } }";
    let result = parse_with_diagnostics(input);
    assert!(
        result.is_ok(),
        "expose feature chains should parse without separator diagnostics: {:?}",
        result.errors
    );
    assert!(
        !result
            .errors
            .iter()
            .any(|e| e.code.as_deref() == Some("invalid_qualified_name_separator")),
        "feature-chain expose should not emit invalid_qualified_name_separator: {:?}",
        result.errors
    );
}

#[test]
fn test_parse_with_diagnostics_accepts_root_level_part_definition() {
    let input = "part def TopLevel;";
    let result = parse_with_diagnostics(input);
    assert!(
        result.is_ok(),
        "root-level part def is legal RootNamespace content: {:?}",
        result.errors
    );
    assert!(
        matches!(
            result.root.elements[0].value,
            RootElement::Member(ref m) if matches!(m.value, sysml_v2_parser::ast::PackageBodyElement::PartDef(_))
        ),
        "expected RootElement::Member(PartDef), got {:?}",
        result.root.elements[0].value
    );
}

#[test]
fn test_parse_reports_missing_closing_brace_for_unterminated_package() {
    let input = "package P {\npart def A;\n";
    let err = parse(input).expect_err("unterminated package should fail");
    assert_eq!(err.code.as_deref(), Some("missing_closing_brace"));
    assert_eq!(err.expected.as_deref(), Some("'}'"));
    assert!(
        err.suggestion
            .as_deref()
            .is_some_and(|s| s.contains("Add '}'")),
        "missing brace diagnostic should suggest how to close the body"
    );
}

#[test]
fn test_parse_with_diagnostics_reports_missing_closing_brace_for_unterminated_package() {
    let input = "package P {\npart def A;\n";
    let result = parse_with_diagnostics(input);
    assert!(
        !result.is_ok(),
        "unterminated package should produce diagnostics"
    );
    let err = result
        .errors
        .iter()
        .find(|e| e.code.as_deref() == Some("missing_closing_brace"))
        .expect("expected missing closing brace diagnostic");
    assert_eq!(err.expected.as_deref(), Some("'}'"));
}

#[test]
fn test_parse_accepts_root_level_part_definition() {
    let input = "part def TopLevel;";
    let root = parse(input).expect("root-level part def is legal RootNamespace content");
    assert!(
        matches!(
            root.elements[0].value,
            RootElement::Member(ref m) if matches!(m.value, PackageBodyElement::PartDef(_))
        ),
        "expected RootElement::Member(PartDef), got {:?}",
        root.elements[0].value
    );
}

#[test]
fn test_root_level_part_def_with_reference_usage_body_parses() {
    let input = "part def V {\n\tm : ScalarValues::Integer;\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.is_ok(),
        "root part def with bare typed member should parse: {:?}",
        result.errors
    );
}

#[test]
fn test_invalid_input_corpus_is_handled_gracefully() {
    let invalid_inputs = [
        "package P {",
        "package P { part def A {",
        "package P { @@@ ??? }",
        "package P { /* unterminated",
        "namespace N { part def X { ;;; }",
    ];

    for input in invalid_inputs {
        let strict = std::panic::catch_unwind(|| {
            let _ = parse(input).is_ok();
        });
        assert!(strict.is_ok(), "parse should not panic for {:?}", input);

        let recovered = std::panic::catch_unwind(|| parse_with_diagnostics(input));
        assert!(
            recovered.is_ok(),
            "parse_with_diagnostics should not panic for {:?}",
            input
        );
    }
}
