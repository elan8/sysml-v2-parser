//! Real and string literals are re-emitted from their authored source spans.
//!
//! The decoded contents of a `STRING_VALUE` are not the token: an escaped quote inside the
//! literal must survive `parse -> emit -> parse`, and a real literal keeps its authored form
//! (`6.022e23` is not normalized to a decimal). Both are resolved through the document rather
//! than copied into the tree.

use sysml_v2_parser::ast::{Expression, PackageBodyElement};
use sysml_v2_parser::{emit_sysml, parse};

#[test]
fn escaped_quote_in_string_literal_survives_emission() {
    let source = "package P {\n    attribute label = \"say \\\"hi\\\" now\";\n}\n";
    let parsed = parse(source).expect("parse");
    let emitted = emit_sysml(&parsed).expect("emit");
    assert!(
        emitted.contains("\"say \\\"hi\\\" now\""),
        "authored escapes must be preserved, got:\n{emitted}"
    );
    parse(&emitted).expect("emitted text reparses");
}

#[test]
fn literals_resolve_through_the_document() {
    let source =
        "package P {\n    attribute a = 6.022e23;\n    attribute b = \"x \\\"y\\\"\";\n}\n";
    let parsed = parse(source).expect("parse");
    let mut reals = Vec::new();
    let mut strings = Vec::new();
    for element in &parsed.root.elements {
        let sysml_v2_parser::ast::RootElement::Package(package) = &element.value else {
            continue;
        };
        let sysml_v2_parser::ast::PackageBody::Brace { elements, .. } = &package.value.body else {
            continue;
        };
        for member in elements {
            let PackageBodyElement::AttributeUsage(usage) = &member.value else {
                continue;
            };
            let Some(value) = &usage.value.value else {
                continue;
            };
            match &value.value.expression.value {
                Expression::LiteralReal(literal) => reals.push(parsed.real_literal(*literal)),
                Expression::LiteralString(literal) => strings.push((
                    parsed.string_literal(*literal),
                    parsed.decoded_string_literal(*literal),
                )),
                _ => {}
            }
        }
    }
    assert_eq!(reals, vec![Some("6.022e23")]);
    assert_eq!(strings.len(), 1);
    assert_eq!(strings[0].0, Some("\"x \\\"y\\\"\""));
    assert_eq!(strings[0].1.as_deref(), Some("x \"y\""));
}
