//! The shared body container (`ast::Body`) and the distinctions it has to keep.
//!
//! Every declaration body in the grammar is `;` or `{ ... }`, so one container carries that shape
//! for every scope while the member set stays typed per scope. These tests pin the facts that
//! sharing the container must not blur: an absent body is not an empty one, members keep their
//! authored order, and each scope still accepts only its own members.

use sysml_v2_parser::ast::{
    Body, PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement, RootElement, Span,
};
use sysml_v2_parser::{emit_sysml, parse};

fn package_body(source: &str) -> PackageBody {
    let root = parse(source).expect("parse");
    match &root.elements[0].value {
        RootElement::Package(package) => package.value.body.clone(),
        other => panic!("expected a package, got {other:?}"),
    }
}

fn first_part_def_body(source: &str) -> PartDefBody {
    match package_body(source).braced_elements().expect("brace body") {
        [element] => match &element.value {
            PackageBodyElement::PartDef(part_def) => part_def.value.body.clone(),
            other => panic!("expected a part def, got {other:?}"),
        },
        elements => panic!("expected exactly one member, got {}", elements.len()),
    }
}

/// `;` and `{}` are different authored syntax, and a shared container must keep them different:
/// one declares that the element owns no members, the other that it owns an empty body.
#[test]
fn a_semicolon_body_is_not_an_empty_brace_body() {
    let semicolon = first_part_def_body("package P { part def A; }");
    let empty_braces = first_part_def_body("package P { part def A {} }");

    assert!(semicolon.is_semicolon());
    assert!(!empty_braces.is_semicolon());
    assert_ne!(semicolon, empty_braces);

    assert_eq!(semicolon.braced_elements(), None);
    assert_eq!(empty_braces.braced_elements(), Some(&[][..]));

    // `members` deliberately flattens the two for consumers that only want members; that is why
    // it is not the way to ask whether a body was written.
    assert_eq!(semicolon.members().count(), 0);
    assert_eq!(empty_braces.members().count(), 0);
}

/// The distinction survives a format/reparse cycle rather than collapsing to one spelling.
#[test]
fn the_semicolon_and_brace_forms_both_round_trip() {
    for source in ["package P { part def A; }", "package P { part def A {} }"] {
        let parsed = parse(source).expect("parse");
        let emitted = emit_sysml(&parsed).expect("emit");
        let reparsed =
            parse(&emitted).unwrap_or_else(|error| panic!("reparse: {error}\n{emitted}"));
        assert_eq!(
            parsed.normalize_for_test_comparison(),
            reparsed.normalize_for_test_comparison(),
            "body form changed across format/reparse; emitted:\n{emitted}"
        );
    }
}

/// Members stay in authored order, including malformed ones, which is what lets a consumer report
/// against source position without re-deriving it.
#[test]
fn members_keep_their_authored_order() {
    let body = first_part_def_body(
        "package P { part def A { attribute one : Real; attribute two : Real; } }",
    );
    let names: Vec<_> = body
        .members()
        .map(|member| match &member.value {
            PartDefBodyElement::AttributeUsage(attribute) => attribute.value.name.clone(),
            other => panic!("expected an attribute usage, got {other:?}"),
        })
        .collect();
    assert_eq!(names, vec!["one".to_owned(), "two".to_owned()]);
}

/// Sharing the container does not share the member set: each scope still names its own element
/// type, so a member cannot be moved to a scope whose grammar does not accept it. This is a
/// type-level property, asserted here by construction.
#[test]
fn scopes_keep_their_own_member_types() {
    let package: PackageBody = Body::Brace {
        open_span: Span::dummy(),
        close_span: Span::dummy(),
        elements: Vec::new(),
    };
    let part_def: PartDefBody = Body::Brace {
        open_span: Span::dummy(),
        close_span: Span::dummy(),
        elements: Vec::new(),
    };
    assert_eq!(package.braced_elements().map(<[_]>::len), Some(0));
    assert_eq!(part_def.braced_elements().map(<[_]>::len), Some(0));
    // `package` and `part_def` have different types here: assigning one to the other, or pushing a
    // `PackageBodyElement` into the part-def body, does not compile.
}

#[cfg(feature = "serde")]
#[test]
fn the_serialized_body_shape_carries_its_delimiters() {
    let semicolon: PartDefBody = Body::Semicolon {
        semicolon_span: Span::dummy(),
    };
    assert_eq!(
        serde_json::to_value(&semicolon).expect("serialize"),
        serde_json::json!({
            "Semicolon": {
                "semicolon_span": {"offset": 0, "line": 1, "column": 1, "len": 0}
            }
        })
    );

    let empty: PartDefBody = Body::Brace {
        open_span: Span::dummy(),
        elements: Vec::new(),
        close_span: Span::dummy(),
    };
    assert_eq!(
        serde_json::to_value(&empty).expect("serialize"),
        serde_json::json!({
            "Brace": {
                "open_span": {"offset": 0, "line": 1, "column": 1, "len": 0},
                "elements": [],
                "close_span": {"offset": 0, "line": 1, "column": 1, "len": 0}
            }
        })
    );
}

/// The delimiters are the authored tokens, not derived positions: each one slices back to itself.
#[test]
fn the_delimiters_slice_to_the_tokens_they_came_from() {
    let source = "package P {
  part def A {
    attribute x : Real;
  }
  part def B;
}
";
    let document = sysml_v2_parser::parse_for_editor(source);
    let RootElement::Package(package) = &document.document.root.elements[0].value else {
        panic!("expected a package");
    };
    let PackageBody::Brace {
        open_span,
        elements,
        close_span,
    } = &package.value.body
    else {
        panic!("expected a brace package body");
    };
    assert_eq!(
        &source[open_span.offset..open_span.offset + open_span.len],
        "{"
    );
    assert_eq!(
        &source[close_span.offset..close_span.offset + close_span.len],
        "}"
    );
    assert!(open_span.offset < close_span.offset);

    let PackageBodyElement::PartDef(part_def) = &elements[0].value else {
        panic!("expected a part def");
    };
    let PartDefBody::Brace {
        open_span: inner_open,
        close_span: inner_close,
        ..
    } = &part_def.value.body
    else {
        panic!("expected a brace part def body");
    };
    assert_eq!(
        &source[inner_open.offset..inner_open.offset + inner_open.len],
        "{"
    );
    assert_eq!(
        &source[inner_close.offset..inner_close.offset + inner_close.len],
        "}"
    );
    assert!(
        open_span.offset < inner_open.offset && inner_close.offset < close_span.offset,
        "the nested body's delimiters must sit inside the enclosing one's"
    );

    let PackageBodyElement::PartDef(semicolon_def) = &elements[1].value else {
        panic!("expected a second part def");
    };
    let PartDefBody::Semicolon { semicolon_span } = &semicolon_def.value.body else {
        panic!("expected a semicolon body");
    };
    assert_eq!(
        &source[semicolon_span.offset..semicolon_span.offset + semicolon_span.len],
        ";"
    );
}

/// Spelling the delimiters correctly is not enough: a wire document could point them at any other
/// `{ ... }` pair. They have to be *this* body's pair -- inside the owning declaration, and around
/// its own members -- and deserialization rejects them when they are not.
#[cfg(feature = "serde")]
#[test]
fn a_delimiter_redirected_to_another_body_is_rejected() {
    let source = "package P {\n  part def A {\n    attribute x : Real;\n  }\n  part def B { }\n}\n";
    let document = sysml_v2_parser::parse_for_editor(source).document;
    let encoded = serde_json::to_value(&document).expect("the parsed document serializes");

    // Both braces of the *first* part def, redirected to the well-formed pair of the second: the
    // tokens still spell `{` and `}`, and the open still precedes the close.
    let path =
        "/root/elements/0/value/Package/value/body/Brace/elements/0/value/PartDef/value/body/Brace";
    let sibling =
        "/root/elements/0/value/Package/value/body/Brace/elements/1/value/PartDef/value/body/Brace";
    let mut tampered = encoded.clone();
    for delimiter in ["open_span", "close_span"] {
        let replacement = tampered
            .pointer(&format!("{sibling}/{delimiter}"))
            .cloned()
            .expect("the sibling body's delimiter");
        *tampered
            .pointer_mut(&format!("{path}/{delimiter}"))
            .expect("the first body's delimiter") = replacement;
    }
    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("delimiters belonging to another body must be rejected");
    let message = error.to_string();
    assert!(
        message.contains("outside"),
        "expected a containment failure, got: {message}"
    );
}
