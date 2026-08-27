#[cfg(feature = "serde")]
fn find_key_mut<'a>(
    value: &'a mut serde_json::Value,
    key: &str,
) -> Option<&'a mut serde_json::Value> {
    match value {
        serde_json::Value::Object(object) => {
            if object.contains_key(key) {
                return object.get_mut(key);
            }
            object
                .values_mut()
                .find_map(|value| find_key_mut(value, key))
        }
        serde_json::Value::Array(array) => {
            array.iter_mut().find_map(|value| find_key_mut(value, key))
        }
        _ => None,
    }
}

#[cfg(feature = "serde")]
#[test]
fn a_binding_all_span_covering_other_text_is_rejected() {
    let document = sysml_v2_parser::parse_for_editor("classifier C { binding all; }").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let all_span = find_key_mut(&mut tampered, "all_span").expect("the `all` keyword span");
    *all_span
        .pointer_mut("/offset")
        .expect("the `all` keyword offset") = serde_json::json!(0);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("an `all` span that does not cover `all` must be rejected");
    assert!(
        error
            .to_string()
            .contains("binding connector `all` keyword"),
        "expected the binding `all` check to name itself, got: {error}"
    );
}

#[cfg(feature = "serde")]
#[test]
fn an_inline_binding_of_span_covering_other_text_is_rejected() {
    let document =
        sysml_v2_parser::parse_for_editor("classifier C { binding named of left = right; }")
            .document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let inline_ends =
        find_key_mut(&mut tampered, "inline_ends").expect("the binding connector inline end pair");
    *inline_ends
        .pointer_mut("/value/of_span/offset")
        .expect("the `of` introducer offset") = serde_json::json!(0);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("an `of` span that does not cover `of` must be rejected");
    assert!(
        error
            .to_string()
            .contains("binding connector `of` introducer"),
        "expected the binding `of` check to name itself, got: {error}"
    );
}

#[cfg(feature = "serde")]
#[test]
fn an_inline_binding_equals_span_covering_other_text_is_rejected() {
    let document =
        sysml_v2_parser::parse_for_editor("classifier C { binding of left = right; }").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let inline_ends =
        find_key_mut(&mut tampered, "inline_ends").expect("the binding connector inline end pair");
    *inline_ends
        .pointer_mut("/value/equals_span/offset")
        .expect("the equals delimiter span") = serde_json::json!(0);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("an equals span that does not cover `=` must be rejected");
    assert!(
        error
            .to_string()
            .contains("binding connector equals delimiter"),
        "expected the binding delimiter check to name itself, got: {error}"
    );
}

#[cfg(feature = "serde")]
#[test]
fn declared_inline_binding_ends_without_of_are_rejected() {
    let document =
        sysml_v2_parser::parse_for_editor("classifier C { binding named of left = right; }")
            .document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let inline_ends =
        find_key_mut(&mut tampered, "inline_ends").expect("the binding connector inline end pair");
    *inline_ends
        .pointer_mut("/value/of_span")
        .expect("the `of` introducer span") = serde_json::Value::Null;

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("declared inline ends without `of` must be rejected");
    assert!(
        error
            .to_string()
            .contains("declared binding connector inline ends have no `of` introducer"),
        "expected the declared binding shape check to name itself, got: {error}"
    );
}
