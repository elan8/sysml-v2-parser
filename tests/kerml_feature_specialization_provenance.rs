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
fn a_feature_specialization_variant_with_the_wrong_relationship_kind_is_rejected() {
    let document = sysml_v2_parser::parse_for_editor("feature f crosses target;").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let crossing = find_key_mut(&mut tampered, "CrossSubsetting")
        .expect("the cross-subsetting specialization");
    *crossing
        .pointer_mut("/value/kind")
        .expect("the nested subsetting kind") = serde_json::json!("References");

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("the specialization variant and nested relationship kind must agree");
    assert!(
        error
            .to_string()
            .contains("feature specialization cross subsetting contains References relationship"),
        "expected the feature-specialization invariant to name itself, got: {error}"
    );
}
