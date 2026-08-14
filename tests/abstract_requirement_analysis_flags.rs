//! Regression coverage: `abstract` prefix on requirement/analysis/verification/use-case
//! definitions and usages is captured as `is_abstract` on the AST node.

use sysml_v2_parser::ast::{PackageBody, PackageBodyElement, RootElement};
use sysml_v2_parser::parse;

fn package_elements(source: &str) -> Vec<sysml_v2_parser::Node<PackageBodyElement>> {
    let result = parse(source).expect("parse should succeed");
    let package = result
        .elements
        .iter()
        .find_map(|el| match &el.value {
            RootElement::Package(n) => Some(&n.value),
            _ => None,
        })
        .expect("top-level package should be present");
    match &package.body {
        PackageBody::Brace { elements, .. } => elements.clone(),
        other => panic!("expected brace body, got {other:?}"),
    }
}

#[test]
fn abstract_requirement_def_sets_is_abstract() {
    let elements = package_elements(
        r#"package P {
  abstract requirement def AbstractReq;
  requirement def ConcreteReq;
}"#,
    );
    for (name, expected_abstract) in [("AbstractReq", true), ("ConcreteReq", false)] {
        let found = elements.iter().find_map(|el| match &el.value {
            PackageBodyElement::RequirementDef(n)
                if n.value.identification.name.as_deref() == Some(name) =>
            {
                Some(n.value.is_abstract)
            }
            _ => None,
        });
        assert_eq!(
            found,
            Some(expected_abstract),
            "requirement def '{name}' is_abstract mismatch"
        );
    }
}

#[test]
fn abstract_requirement_usage_sets_is_abstract() {
    let elements = package_elements(
        r#"package P {
  abstract requirement abstractReq;
  requirement concreteReq;
}"#,
    );
    for (name, expected_abstract) in [("abstractReq", true), ("concreteReq", false)] {
        let found = elements.iter().find_map(|el| match &el.value {
            PackageBodyElement::RequirementUsage(n) if n.value.name == name => {
                Some(n.value.is_abstract)
            }
            _ => None,
        });
        assert_eq!(
            found,
            Some(expected_abstract),
            "requirement usage '{name}' is_abstract mismatch"
        );
    }
}

#[test]
fn abstract_analysis_and_verification_and_use_case_def_sets_is_abstract() {
    let elements = package_elements(
        r#"package P {
  abstract analysis def AbstractAnalysis;
  analysis def ConcreteAnalysis;
  abstract verification def AbstractVerification;
  verification def ConcreteVerification;
  abstract use case def AbstractUseCase;
  use case def ConcreteUseCase;
}"#,
    );

    let analysis_abstract = elements.iter().find_map(|el| match &el.value {
        PackageBodyElement::AnalysisCaseDef(n)
            if n.value.identification.name.as_deref() == Some("AbstractAnalysis") =>
        {
            Some(n.value.is_abstract)
        }
        _ => None,
    });
    assert_eq!(analysis_abstract, Some(true));
    let analysis_concrete = elements.iter().find_map(|el| match &el.value {
        PackageBodyElement::AnalysisCaseDef(n)
            if n.value.identification.name.as_deref() == Some("ConcreteAnalysis") =>
        {
            Some(n.value.is_abstract)
        }
        _ => None,
    });
    assert_eq!(analysis_concrete, Some(false));

    let verification_abstract = elements.iter().find_map(|el| match &el.value {
        PackageBodyElement::VerificationCaseDef(n)
            if n.value.identification.name.as_deref() == Some("AbstractVerification") =>
        {
            Some(n.value.is_abstract)
        }
        _ => None,
    });
    assert_eq!(verification_abstract, Some(true));

    let use_case_abstract = elements.iter().find_map(|el| match &el.value {
        PackageBodyElement::UseCaseDef(n)
            if n.value.identification.name.as_deref() == Some("AbstractUseCase") =>
        {
            Some(n.value.is_abstract)
        }
        _ => None,
    });
    assert_eq!(use_case_abstract, Some(true));
    let use_case_concrete = elements.iter().find_map(|el| match &el.value {
        PackageBodyElement::UseCaseDef(n)
            if n.value.identification.name.as_deref() == Some("ConcreteUseCase") =>
        {
            Some(n.value.is_abstract)
        }
        _ => None,
    });
    assert_eq!(use_case_concrete, Some(false));
}
