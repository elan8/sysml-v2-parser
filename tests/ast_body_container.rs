//! Properties of the shared body container (`ast::Body`) that no snapshot can state.
//!
//! Every declaration body in the grammar is `;` or `{ ... }`, so one container carries that shape
//! for every scope while the member set stays typed per scope.
//!
//! That the two spellings stay distinct, and that members keep their authored order, is pinned by
//! `tests/snapshots/sysml/body_container_forms.md`, whose AST section shows `(body semicolon)`
//! beside the brace forms and lists members in order.
//!
//! What remains here is what a fixture cannot express: a type-level property asserted by
//! construction, the serialized wire shape of the delimiters, and the rejection of a deserialized
//! document whose delimiters point somewhere else. The delimiter *spans* are checked on every
//! fixture by `serialized_provenance_corpus.rs`, which validates them during serialization; the
//! test here states the invariant directly rather than as a side effect of that.

use sysml_v2_parser::ast::{Body, PackageBody, PackageBodyElement, PartDefBody, RootElement, Span};

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
