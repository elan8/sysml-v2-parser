//! Declaration-name recovery is an explicit lexical fact, shared by every consumer.
use sysml_v2_parser::{
    ast::{visit::Visitor, DeclarationName, DeclarationNameState, NameLanguage},
    *,
};

#[derive(Default)]
struct Names(Vec<DeclarationName>);
impl Visitor for Names {
    fn visit_declaration_name(&mut self, name: &DeclarationName) {
        self.0.push(*name);
    }
}

#[test]
fn reserved_name_is_recovered_without_becoming_a_decoded_name() {
    let source = "package P { part def port; part def Good; }";
    let result = parse_for_editor(source);
    assert_eq!(result.errors.len(), 1);
    let error = &result.errors[0];
    assert_eq!(error.code.as_deref(), Some("reserved_declaration_name"));
    assert_eq!(parse(source).unwrap_err(), *error);
    let mut names = Names::default();
    names.visit_root_namespace(&result.document.root);
    let invalid: Vec<_> = names
        .0
        .iter()
        .filter(|n| n.state() == DeclarationNameState::ReservedKeyword)
        .collect();
    assert_eq!(invalid.len(), 1);
    assert_eq!(
        result.document.source.slice(invalid[0].span()),
        Some("port")
    );
    assert_eq!(result.document.decoded_declaration_name(*invalid[0]), None);
    assert!(matches!(
        emit_sysml(&result.document),
        Err(EmitError::Opaque {
            kind: OpacityKind::ParseError,
            ..
        })
    ));
    let emitted = emit_recovered_sysml(&result.document).unwrap();
    let recovered = parse_for_editor(&emitted);
    assert_eq!(recovered.errors.len(), 1);
    assert_eq!(recovered.errors[0].code, error.code);
}

#[test]
fn name_language_and_quoting_control_keyword_validity() {
    assert!(parse("package P { part def 'port'; part def Port; }").is_ok());
    assert!(parse_with_name_language("package P { feature port; }", NameLanguage::Kerml).is_ok());
    assert!(parse_with_name_language("package P { feature port; }", NameLanguage::Sysml).is_err());
    for language in [NameLanguage::Sysml, NameLanguage::Kerml] {
        assert!(parse_with_name_language("package P { feature from; }", language).is_err());
        assert!(parse_with_name_language("package P { feature 'from'; }", language).is_ok());
    }
}

#[cfg(feature = "serde")]
#[test]
fn serialized_names_cannot_change_language_or_validity() {
    let document =
        parse_with_name_language("package P { feature port; }", NameLanguage::Kerml).unwrap();
    let encoded = serde_json::to_value(&document).unwrap();
    let decoded: ast::ParsedDocument = serde_json::from_value(encoded.clone()).unwrap();
    assert_eq!(decoded.name_language, NameLanguage::Kerml);
    let mut corrupt = encoded;
    corrupt["name_language"] = serde_json::json!("sysml");
    assert!(serde_json::from_value::<ast::ParsedDocument>(corrupt).is_err());
    let recovered = parse_for_editor("package P { part def port; }").document;
    let encoded = serde_json::to_value(&recovered).unwrap();
    let decoded: ast::ParsedDocument = serde_json::from_value(encoded).unwrap();
    let mut names = Names::default();
    names.visit_root_namespace(&decoded.root);
    assert!(names
        .0
        .iter()
        .any(|n| n.state() == DeclarationNameState::ReservedKeyword));
}

#[test]
fn explicit_language_owned_apis_keep_the_callers_source_allocation() {
    let source = String::from("package P { feature port; }");
    let pointer = source.as_ptr();
    let document = parse_owned_with_name_language(source, NameLanguage::Kerml).unwrap();
    assert_eq!(document.name_language, NameLanguage::Kerml);
    assert_eq!(document.source.as_str().as_ptr(), pointer);
    let source = String::from("package P { feature from; }");
    let pointer = source.as_ptr();
    let editor = parse_for_editor_owned_with_name_language(source, NameLanguage::Kerml);
    assert_eq!(editor.document.name_language, NameLanguage::Kerml);
    assert_eq!(editor.document.source.as_str().as_ptr(), pointer);
    assert_eq!(editor.errors.len(), 1);
    assert_eq!(
        editor.errors[0].code.as_deref(),
        Some("reserved_declaration_name")
    );
}
