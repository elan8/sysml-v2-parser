//! Structural helpers for tests that deliberately corrupt serialized AST provenance.

/// Finds an occurrence-usage prefix by its serialized structural boundary rather than by the
/// complete path through every owning AST enum. Fixtures pass their expected prefix count so a
/// future wire change fails at this boundary with an actionable message.
pub fn occurrence_prefix_mut(
    wire: &mut serde_json::Value,
    index: usize,
    expected_count: usize,
) -> &mut serde_json::Map<String, serde_json::Value> {
    fn is_prefix(fields: &serde_json::Map<String, serde_json::Value>) -> bool {
        fields.contains_key("head") && fields.contains_key("extension_keywords")
    }

    fn count(value: &serde_json::Value) -> usize {
        match value {
            serde_json::Value::Array(values) => values.iter().map(count).sum(),
            serde_json::Value::Object(fields) => {
                usize::from(is_prefix(fields)) + fields.values().map(count).sum::<usize>()
            }
            _ => 0,
        }
    }

    fn find_nth<'a>(
        value: &'a mut serde_json::Value,
        target: usize,
        seen: &mut usize,
    ) -> Option<&'a mut serde_json::Map<String, serde_json::Value>> {
        match value {
            serde_json::Value::Array(values) => values
                .iter_mut()
                .find_map(|value| find_nth(value, target, seen)),
            serde_json::Value::Object(fields) => {
                if is_prefix(fields) {
                    if *seen == target {
                        return Some(fields);
                    }
                    *seen += 1;
                }
                fields
                    .values_mut()
                    .find_map(|value| find_nth(value, target, seen))
            }
            _ => None,
        }
    }

    assert_eq!(
        count(wire),
        expected_count,
        "fixture serialized an unexpected number of occurrence-usage prefixes"
    );
    find_nth(wire, index, &mut 0).expect("the requested occurrence-usage prefix")
}

/// Returns the fields of the basic alternative of an `OccurrenceUsagePrefix` head.
pub fn basic_occurrence_prefix_head_mut(
    wire: &mut serde_json::Value,
    index: usize,
    expected_count: usize,
) -> &mut serde_json::Map<String, serde_json::Value> {
    occurrence_prefix_mut(wire, index, expected_count)
        .get_mut("head")
        .and_then(|head| head.get_mut("Basic"))
        .and_then(serde_json::Value::as_object_mut)
        .expect("the basic occurrence-usage prefix head")
}
