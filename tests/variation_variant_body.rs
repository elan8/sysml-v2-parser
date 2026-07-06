//! Regression coverage for S42-LIM-014: `variant` members inside a `variation part def` body.

use sysml_v2_parser::ast::{PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement, RootElement};
use sysml_v2_parser::parse;

#[test]
fn variation_part_def_accepts_variant_members_without_recovery_errors() {
    let input = r#"
package P {
  part def SensorAssembly;

  variation part def NavigationSensorSuiteChoice :> SensorAssembly {
    variant tofImuOnly;
    variant lidarSlamSuite;
    variant aiVisionSuite;
  }
}
"#;
    let result = parse(input).expect("parse should succeed");

    let package = result
        .elements
        .iter()
        .find_map(|el| match &el.value {
            RootElement::Package(n) => Some(&n.value),
            _ => None,
        })
        .expect("top-level package P should be present");

    let elements = match &package.body {
        PackageBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {other:?}"),
    };

    let variation = elements
        .iter()
        .find_map(|el| match &el.value {
            PackageBodyElement::PartDef(n)
                if n.value.identification.name.as_deref()
                    == Some("NavigationSensorSuiteChoice") =>
            {
                Some(&n.value)
            }
            _ => None,
        })
        .expect("NavigationSensorSuiteChoice part def should be present");

    let variant_elements = match &variation.body {
        PartDefBody::Brace { elements } => elements,
        other => panic!("expected brace body, got {other:?}"),
    };

    let variant_names: Vec<&str> = variant_elements
        .iter()
        .filter_map(|el| match &el.value {
            PartDefBodyElement::VariantUsage(n) => Some(n.value.name.as_str()),
            PartDefBodyElement::Error(err) => {
                panic!("unexpected recovery error in variation body: {err:?}")
            }
            _ => None,
        })
        .collect();

    assert_eq!(
        variant_names,
        vec!["tofImuOnly", "lidarSlamSuite", "aiVisionSuite"],
        "all three variant members should be parsed as owned VariantUsage elements"
    );
}
