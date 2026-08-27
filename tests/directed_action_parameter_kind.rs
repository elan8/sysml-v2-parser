use sysml_v2_parser::parse_for_editor;

#[cfg(feature = "serde")]
#[test]
fn an_action_kind_span_covering_other_text_is_rejected() {
    let document = parse_for_editor("action def A { in action body {} }").document;
    let mut tampered = serde_json::to_value(&document).expect("the parsed document serializes");
    let kind_span = "/root/elements/0/value/Member/value/ActionDef/value/body/Brace/elements/0/value/InOutDecl/value/kind/span";
    *tampered
        .pointer_mut(&format!("{kind_span}/offset"))
        .expect("the directed parameter kind span") = serde_json::json!(0);

    let error = serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(tampered)
        .expect_err("an action-kind span that does not cover `action` must be rejected");
    assert!(
        error
            .to_string()
            .contains("directed parameter `action` keyword"),
        "expected the action-kind check to name itself, got: {error}"
    );
}
