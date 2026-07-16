//! Attribute body parsing for unit catalog `unitConversion` redefinitions.

use sysml_v2_parser::ast::{
    AttributeBody, AttributeBodyElement, AttributeUsage, PackageBodyElement, RootElement,
};
use sysml_v2_parser::parse;

fn first_package_attribute_def(
    root: &sysml_v2_parser::RootNamespace,
) -> &sysml_v2_parser::ast::AttributeDef {
    let package = root
        .elements
        .iter()
        .find_map(|el| match &el.value {
            RootElement::Package(pkg) => Some(pkg),
            _ => None,
        })
        .expect("package");
    let attr = match &package.value.body {
        sysml_v2_parser::ast::PackageBody::Brace { elements } => {
            match &elements.first().expect("member").value {
                PackageBodyElement::AttributeDef(def) => def,
                other => panic!("expected attribute def, got {other:?}"),
            }
        }
        _ => panic!("expected brace package body"),
    };
    &attr.value
}

fn attribute_body_usages(body: &AttributeBody) -> Vec<&AttributeUsage> {
    let AttributeBody::Brace { elements } = body else {
        return Vec::new();
    };
    elements
        .iter()
        .filter_map(|el| match &el.value {
            AttributeBodyElement::AttributeUsage(usage) => Some(&usage.value),
            _ => None,
        })
        .collect()
}

#[test]
fn parses_conversion_by_prefix_in_attribute_body() {
    let root = parse(
        "package SI {
            attribute <km> kilometre : LengthUnit {
                :>> unitConversion: ConversionByPrefix {
                    :>> prefix = kilo;
                    :>> referenceUnit = m;
                }
            }
        }",
    )
    .expect("parse");
    let def = first_package_attribute_def(&root);
    let usages = attribute_body_usages(&def.body);
    assert_eq!(usages.len(), 1, "expected one unitConversion binding");
    let conversion = usages[0];
    assert_eq!(
        conversion
            .redefines
            .as_ref()
            .map(|n| n.value.target_display()),
        Some("unitConversion".to_string())
    );
    assert_eq!(
        conversion.typing.as_ref().map(|n| n.value.target_display()),
        Some("ConversionByPrefix".to_string())
    );
    let AttributeBody::Brace { elements } = &conversion.body else {
        panic!("expected nested brace body");
    };
    assert_eq!(
        elements.len(),
        2,
        "expected prefix and referenceUnit bindings"
    );
    assert!(
        !elements
            .iter()
            .any(|el| matches!(el.value, AttributeBodyElement::Error(_))),
        "unit conversion body should not use error recovery"
    );
}

#[test]
fn parses_conversion_by_convention_in_attribute_body() {
    let root = parse(
        "package SI {
            attribute <ft> foot : LengthUnit {
                :>> unitConversion: ConversionByConvention {
                    :>> referenceUnit = m;
                    :>> conversionFactor = 3.048E-01;
                }
            }
        }",
    )
    .expect("parse");
    let def = first_package_attribute_def(&root);
    let usages = attribute_body_usages(&def.body);
    assert_eq!(usages.len(), 1);
    let conversion = usages[0];
    assert_eq!(
        conversion
            .redefines
            .as_ref()
            .map(|n| n.value.target_display()),
        Some("unitConversion".to_string())
    );
    assert_eq!(
        conversion.typing.as_ref().map(|n| n.value.target_display()),
        Some("ConversionByConvention".to_string())
    );
    let AttributeBody::Brace { elements } = &conversion.body else {
        panic!("expected nested brace body");
    };
    assert_eq!(elements.len(), 2);
    assert!(
        !elements
            .iter()
            .any(|el| matches!(el.value, AttributeBodyElement::Error(_))),
        "conversion body should parse structurally"
    );
}
