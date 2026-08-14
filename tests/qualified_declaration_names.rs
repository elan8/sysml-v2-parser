use sysml_v2_parser::ast::{
    DeclarationName, QualifiedDeclarationName, ReferenceSeparator, RootElement,
};
use sysml_v2_parser::{emit_sysml, parse, parse_with_diagnostics, ParsedDocument};

fn qualified_name(element: &RootElement) -> QualifiedDeclarationName {
    let identification = match element {
        RootElement::Package(package) => &package.value.identification,
        RootElement::LibraryPackage(package) => &package.value.identification,
        RootElement::Namespace(namespace) => &namespace.value.identification,
        other => panic!("expected namespace-owning declaration, got {other:?}"),
    };
    match &identification.name {
        Some(DeclarationName::Qualified(name)) => *name,
        ref other => panic!("expected qualified declaration name, got {other:?}"),
    }
}

fn assert_two_segment_name(
    document: &ParsedDocument,
    name: QualifiedDeclarationName,
    first: &str,
    second: &str,
) {
    let view = document
        .qualified_declaration_name(name)
        .expect("qualified declaration name must resolve in its document");
    assert_eq!(view.segments.len(), 2);
    assert_eq!(view.segment_decoded_text(0).as_deref(), Some(first));
    assert_eq!(view.segment_decoded_text(1).as_deref(), Some(second));
    assert_eq!(
        view.segments[1].separator_before,
        Some(ReferenceSeparator::ColonColon)
    );
}

#[test]
fn qualified_package_namespace_and_library_names_are_typed_and_emittable() {
    let source = concat!(
        "package AstronomyReference::Domain;\n",
        "namespace Mission::Views;\n",
        "standard library package Kernel::Types;\n",
    );
    let document = parse(source).expect("qualified namespace declarations should parse");
    assert_eq!(document.root.elements.len(), 3);

    assert_two_segment_name(
        &document,
        qualified_name(&document.root.elements[0].value),
        "AstronomyReference",
        "Domain",
    );
    assert_two_segment_name(
        &document,
        qualified_name(&document.root.elements[1].value),
        "Mission",
        "Views",
    );
    assert_two_segment_name(
        &document,
        qualified_name(&document.root.elements[2].value),
        "Kernel",
        "Types",
    );
    let emitted = emit_sysml(&document).expect("emit qualified declarations");
    assert_eq!(
        emitted,
        concat!(
            "package AstronomyReference::Domain;\n\n",
            "namespace Mission::Views;\n\n",
            "standard library package Kernel::Types;\n",
        )
    );
    parse(&emitted).expect("emitted qualified declarations should reparse");
}

#[test]
fn malformed_qualified_declaration_rolls_back_before_later_sibling() {
    let result =
        parse_with_diagnostics("package Ghost::Broken unexpected;\npackage Live::Valid;\n");
    assert!(!result.errors.is_empty(), "fixture must exercise recovery");

    let live = result
        .document
        .root
        .elements
        .iter()
        .find_map(|element| match &element.value {
            RootElement::Package(package)
                if matches!(
                    package.value.identification.name.as_ref(),
                    Some(DeclarationName::Qualified(_))
                ) =>
            {
                Some(qualified_name(&element.value))
            }
            _ => None,
        })
        .expect("valid later package must survive recovery");
    assert_two_segment_name(&result.document, live, "Live", "Valid");
    assert_eq!(
        result.document.qualified_references.len(),
        1,
        "failed containing production must not publish Ghost::Broken"
    );
}

#[cfg(feature = "serde")]
#[test]
fn qualified_declaration_name_survives_atomic_document_serde() {
    let document = parse("package AstronomyReference::Domain;")
        .expect("qualified package declaration should parse");
    let encoded = serde_json::to_vec(&document).expect("serialize parsed document envelope");
    let decoded: ParsedDocument =
        serde_json::from_slice(&encoded).expect("deserialize parsed document envelope");
    let name = qualified_name(&decoded.root.elements[0].value);
    assert_two_segment_name(&decoded, name, "AstronomyReference", "Domain");
}
