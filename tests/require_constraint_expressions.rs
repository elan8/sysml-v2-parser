//! Regression tests for `require constraint { ... }` bodies (Spec42-style expression joins).
//! Ensures logical expressions are not split into `Other`, spans round-trip to source text,
//! and optional `;` between body items does not break multi-clause constraints.

use sysml_v2_parser::ast::{
    ConstraintDefBodyElement, Expression, PackageBody, PackageBodyElement, RequireConstraintBody,
    RequirementDefBody, RequirementDefBodyElement, RootElement,
};
use sysml_v2_parser::{parse, parse_with_diagnostics};

fn text_from_span(src: &str, span: &sysml_v2_parser::ast::Span) -> String {
    let bytes = src.as_bytes();
    let end = span.offset.saturating_add(span.len).min(bytes.len());
    if span.offset <= bytes.len() && span.offset <= end {
        String::from_utf8_lossy(&bytes[span.offset..end]).to_string()
    } else {
        String::new()
    }
}

fn compact_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn joined_constraint_expression_text(src: &str, body: &RequireConstraintBody) -> Option<String> {
    let RequireConstraintBody::Brace { elements } = body else {
        return None;
    };
    let mut frags = Vec::new();
    for el in elements {
        if let ConstraintDefBodyElement::Expression(expr) = &el.value {
            let t = text_from_span(src, &expr.span);
            if !t.trim().is_empty() {
                frags.push(t);
            }
        }
    }
    if frags.is_empty() {
        None
    } else {
        Some(compact_whitespace(&frags.join(" ")))
    }
}

fn first_require_constraint_body<'a>(
    root: &'a sysml_v2_parser::ast::RootNamespace,
) -> &'a RequireConstraintBody {
    let pkg = match &root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("expected brace package body");
    };
    let req = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::RequirementDef(r) => Some(&r.value),
            _ => None,
        })
        .expect("requirement def");
    let RequirementDefBody::Brace { elements } = &req.body else {
        panic!("requirement brace body");
    };
    let c = elements
        .iter()
        .find_map(|e| match &e.value {
            RequirementDefBodyElement::RequireConstraint(rc) => Some(&rc.value.body),
            _ => None,
        })
        .expect("require constraint");
    c
}

fn assert_no_other_in_constraint(src: &str) {
    let root = parse(src).expect("parse");
    let body = first_require_constraint_body(&root);
    let RequireConstraintBody::Brace { elements } = body else {
        panic!("brace body");
    };
    assert!(
        !elements
            .iter()
            .any(|e| matches!(e.value, ConstraintDefBodyElement::Other(_))),
        "unexpected Other in constraint body: {:?}",
        elements.iter().map(|e| &e.value).collect::<Vec<_>>()
    );
}

#[test]
fn require_constraint_single_line_multi_and_round_trip() {
    let src = "package P {\nrequirement def R {\nrequire constraint { (a <= b) and (c <= d) and (e <= f) };\n}\n}";
    assert_no_other_in_constraint(src);
    let root = parse(src).expect("parse");
    let body = first_require_constraint_body(&root);
    let joined = joined_constraint_expression_text(src, body).expect("expression text");
    assert_eq!(joined, "(a <= b) and (c <= d) and (e <= f)");
}

#[test]
fn require_constraint_multi_line_multi_and() {
    let src = concat!(
        "package P {\nrequirement def R {\nrequire constraint {\n",
        "  (a <= b)\n",
        "  and (c <= d)\n",
        "  and (e <= f)\n",
        "};\n}\n}",
    );
    assert_no_other_in_constraint(src);
    let root = parse(src).expect("parse");
    let body = first_require_constraint_body(&root);
    let joined = joined_constraint_expression_text(src, body).expect("expression text");
    assert_eq!(joined, "(a <= b) and (c <= d) and (e <= f)");
}

#[test]
fn require_constraint_not_and_comparison() {
    let src = "package P {\nrequirement def R {\nrequire constraint { not p and (x <= y) };\n}\n}";
    assert_no_other_in_constraint(src);
    let root = parse(src).expect("parse");
    let body = first_require_constraint_body(&root);
    let joined = joined_constraint_expression_text(src, body).expect("expression text");
    assert_eq!(joined, "not p and (x <= y)");
}

#[test]
fn require_constraint_semicolon_separated_ands_join() {
    let src =
        "package P {\nrequirement def R {\nrequire constraint { (a <= b); and (c <= d); };\n}\n}";
    assert_no_other_in_constraint(src);
    let root = parse(src).expect("parse");
    let body = first_require_constraint_body(&root);
    let joined = joined_constraint_expression_text(src, body).expect("expression text");
    assert_eq!(joined, "(a <= b) and (c <= d)");
}

#[test]
fn requirement_body_attribute_integer_default_and_quantity() {
    let src = concat!(
        "package P {\nrequirement def R {\n",
        "attribute n: Integer = 0;\n",
        "attribute v :> ISQ::speed = 0.9 [m/s];\n",
        "require constraint { true };\n",
        "}\n}",
    );
    let r = parse_with_diagnostics(src);
    assert!(r.errors.is_empty(), "unexpected errors: {:?}", r.errors);
    let pkg = match &r.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements } = &pkg.body else {
        panic!("brace body");
    };
    let req = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::RequirementDef(r) => Some(&r.value),
            _ => None,
        })
        .expect("req def");
    let RequirementDefBody::Brace { elements } = &req.body else {
        panic!("req brace");
    };
    let mut attrs = elements.iter().filter_map(|e| match &e.value {
        RequirementDefBodyElement::AttributeDef(a) => Some(&a.value),
        _ => None,
    });
    let n = attrs.next().expect("attribute n");
    assert_eq!(n.name, "n");
    assert_eq!(n.typing.as_deref(), Some("Integer"));
    let v0 = n.value.as_ref().expect("default 0");
    assert!(matches!(v0.value, Expression::LiteralInteger(0)));

    let v = attrs.next().expect("attribute v");
    assert_eq!(v.name, "v");
    assert_eq!(v.typing.as_deref(), Some("ISQ::speed"));
    let q = v.value.as_ref().expect("quantity default");
    assert!(matches!(&q.value, Expression::LiteralWithUnit { .. }));
}

#[test]
fn calc_body_inout_parameter_starts_recovery_sync_list() {
    // `inout` must be in CALC_DEF_BODY_STARTERS so recovery does not treat it as unknown.
    let src = "package P {\ncalc def C {\ninout z : Real;\nreturn r : Real;\n1 + 1;\n}\n}";
    let r = parse_with_diagnostics(src);
    assert!(
        r.errors.is_empty(),
        "inout should parse as calc body in/out: {:?}",
        r.errors
    );
}
