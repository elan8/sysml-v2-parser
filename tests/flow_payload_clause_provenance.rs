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
fn an_of_span_covering_other_text_is_rejected() {
    let document =
        sysml_v2_parser::parse_for_editor("flow of Thing from source to target;").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let payloads = find_key_mut(&mut tampered, "payloads")
        .and_then(serde_json::Value::as_array_mut)
        .expect("the flow payload clauses");
    *payloads[0]
        .pointer_mut("/value/of_span/offset")
        .expect("the payload clause introducer span") = serde_json::json!(0);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("an `of` span that does not cover `of` must be rejected");
    assert!(
        error
            .to_string()
            .contains("flow payload clause `of` keyword"),
        "expected the payload-clause check to name itself, got: {error}"
    );
}
