use sysml_v2_parser::ast::{
    CalcDefBody, CalcDefBodyElement, PackageBody, PackageBodyElement, RootElement,
};
use sysml_v2_parser::parse_with_diagnostics;

#[test]
fn calc_body_parses_return_expression_without_swallowing_siblings() {
    let input = r#"package P {
  calc def SubsystemPowerSum {
    in parts;
    return sum(parts.powerDrawW);
  }
  calc def SubsystemCostSum {
    in parts;
    return sum(parts.bomCost);
  }
}"#;
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "calc return-expression bodies should parse cleanly: {:?}",
        result.errors
    );
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace body");
    };
    let calc_defs: Vec<_> = elements
        .iter()
        .filter_map(|e| match &e.value {
            PackageBodyElement::CalcDef(c) => Some(&c.value),
            _ => None,
        })
        .collect();
    assert_eq!(calc_defs.len(), 2, "expected two calc defs");
    for calc in calc_defs {
        let CalcDefBody::Brace { elements } = &calc.body else {
            panic!("expected brace calc body");
        };
        let expressions: Vec<_> = elements
            .iter()
            .filter_map(|e| match &e.value {
                CalcDefBodyElement::Expression(expr) => Some(expr),
                _ => None,
            })
            .collect();
        assert_eq!(
            expressions.len(),
            1,
            "each calc should have exactly one return expression"
        );
        assert!(
            !elements
                .iter()
                .any(|e| matches!(e.value, CalcDefBodyElement::Error(_))),
            "return expression must not fall through to recovery"
        );
    }
}
