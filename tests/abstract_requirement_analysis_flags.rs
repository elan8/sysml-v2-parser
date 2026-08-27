//! Regression coverage: the `abstract` prefix on requirement/analysis/verification/use-case
//! definitions and usages reaches the AST node.
//!
//! The definition kinds carry `BasicDefinitionPrefix` as one spanned slot with two alternatives,
//! so these read the authored spelling rather than a boolean; the usage kinds still carry
//! `is_abstract` from `RefPrefix` and are read as such.

use sysml_v2_parser::ast::{DefinitionPrefix, Node, PackageBody, PackageBodyElement, RootElement};
use sysml_v2_parser::parse;

/// Whether an authored `BasicDefinitionPrefix` slot spells `abstract`.
fn spells_abstract(slot: Option<&Node<DefinitionPrefix>>) -> bool {
    matches!(
        slot.map(|prefix| prefix.value),
        Some(DefinitionPrefix::Abstract)
    )
}

fn package_elements(
    source: &str,
) -> (
    sysml_v2_parser::ParsedDocument,
    Vec<sysml_v2_parser::Node<PackageBodyElement>>,
) {
    let result = parse(source).expect("parse should succeed");
    let package = result
        .elements
        .iter()
        .find_map(|el| match &el.value {
            RootElement::Package(n) => Some(&n.value),
            _ => None,
        })
        .expect("top-level package should be present");
    let elements = match &package.body {
        PackageBody::Brace { elements, .. } => elements.clone(),
        other => panic!("expected brace body, got {other:?}"),
    };
    (result, elements)
}

#[test]
fn abstract_requirement_def_sets_is_abstract() {
    let (doc, elements) = package_elements(
        r#"package P {
  abstract requirement def AbstractReq;
  requirement def ConcreteReq;
}"#,
    );
    for (name, expected_abstract) in [("AbstractReq", true), ("ConcreteReq", false)] {
        let found = elements.iter().find_map(|el| match &el.value {
            PackageBodyElement::RequirementDef(n)
                if n.value
                    .identification
                    .name
                    .and_then(|n| doc.declaration_name(n))
                    == Some(name) =>
            {
                Some(spells_abstract(n.value.definition_prefix.as_ref()))
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
    let (doc, elements) = package_elements(
        r#"package P {
  abstract requirement abstractReq;
  requirement concreteReq;
}"#,
    );
    for (name, expected_abstract) in [("abstractReq", true), ("concreteReq", false)] {
        let found = elements.iter().find_map(|el| match &el.value {
            PackageBodyElement::RequirementUsage(n)
                if n.value.name.and_then(|n| doc.declaration_name(n)) == Some(name) =>
            {
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
    let (doc, elements) = package_elements(
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
            if n.value
                .identification
                .name
                .and_then(|n| doc.declaration_name(n))
                == Some("AbstractAnalysis") =>
        {
            Some(spells_abstract(n.value.definition_prefix.as_ref()))
        }
        _ => None,
    });
    assert_eq!(analysis_abstract, Some(true));
    let analysis_concrete = elements.iter().find_map(|el| match &el.value {
        PackageBodyElement::AnalysisCaseDef(n)
            if n.value
                .identification
                .name
                .and_then(|n| doc.declaration_name(n))
                == Some("ConcreteAnalysis") =>
        {
            Some(spells_abstract(n.value.definition_prefix.as_ref()))
        }
        _ => None,
    });
    assert_eq!(analysis_concrete, Some(false));

    let verification_abstract = elements.iter().find_map(|el| match &el.value {
        PackageBodyElement::VerificationCaseDef(n)
            if n.value
                .identification
                .name
                .and_then(|n| doc.declaration_name(n))
                == Some("AbstractVerification") =>
        {
            Some(spells_abstract(n.value.definition_prefix.as_ref()))
        }
        _ => None,
    });
    assert_eq!(verification_abstract, Some(true));

    let use_case_abstract = elements.iter().find_map(|el| match &el.value {
        PackageBodyElement::UseCaseDef(n)
            if n.value
                .identification
                .name
                .and_then(|n| doc.declaration_name(n))
                == Some("AbstractUseCase") =>
        {
            Some(spells_abstract(n.value.definition_prefix.as_ref()))
        }
        _ => None,
    });
    assert_eq!(use_case_abstract, Some(true));
    let use_case_concrete = elements.iter().find_map(|el| match &el.value {
        PackageBodyElement::UseCaseDef(n)
            if n.value
                .identification
                .name
                .and_then(|n| doc.declaration_name(n))
                == Some("ConcreteUseCase") =>
        {
            Some(spells_abstract(n.value.definition_prefix.as_ref()))
        }
        _ => None,
    });
    assert_eq!(use_case_concrete, Some(false));
}
