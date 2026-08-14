use super::common::Import;
use super::package::{
    LibraryPackage, Package, PackageBody, PackageBodyElement, QualifiedDeclarationName,
    QualifiedIdentification,
};
#[cfg(feature = "serde")]
use super::provenance_visit::validate_ast_provenance;
use super::qualified_reference::{
    QualifiedReferenceArena, QualifiedReferenceId, QualifiedReferenceView, SourceRange,
    SourceStorage,
};
use crate::ast::core::{Node, Span};

/// KerML top-level element (BNF `RootNamespace = PackageBodyElement*`).
///
/// Package / library package / namespace / import are modeled as dedicated variants for
/// consumers that walk the package tree. Any other legal package-body member at file root
/// (definitions and usages) is [`RootElement::Member`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RootElement {
    Package(Node<Package>),
    LibraryPackage(Node<LibraryPackage>),
    Namespace(Node<NamespaceDecl>),
    Import(Box<Node<Import>>),
    /// Definition or usage (or other package-body member) at root, per SysML `RootNamespace`.
    Member(Box<Node<PackageBodyElement>>),
}

/// KerML NamespaceDeclaration: `namespace` Identification NamespaceBody (same body structure as Package).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamespaceDecl {
    pub identification: QualifiedIdentification,
    pub body: PackageBody,
}

/// Root of a SysML/KerML document: a sequence of top-level package or namespace elements.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RootNamespace {
    pub elements: Vec<Node<RootElement>>,
}

/// One parsed SysML/KerML source file and all document-local identities referenced by its AST.
///
/// [`RootNamespace`] nodes use opaque qualified-reference identities whose source spelling and
/// segment metadata live in [`qualified_references`](Self::qualified_references). Keeping the
/// source, arena, and AST in one envelope prevents consumers from accidentally treating those
/// identities as globally stable or interpreting an AST without the arena that owns them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedDocument {
    /// BOM-stripped source text whose byte offsets back AST/reference spans.
    pub source: SourceStorage,
    /// Document-local qualified references shared by all semantic AST nodes.
    pub qualified_references: QualifiedReferenceArena,
    /// Parsed root namespace.
    pub root: RootNamespace,
}

impl ParsedDocument {
    /// Resolve a parser span through this document's canonical source-position index.
    pub fn range(&self, span: &Span) -> Option<SourceRange> {
        self.source.range_of(span)
    }

    /// Resolve a qualified-reference identity to a source-backed borrowed view.
    pub fn qualified_reference(
        &self,
        id: QualifiedReferenceId,
    ) -> Option<QualifiedReferenceView<'_>> {
        self.qualified_references.get(&self.source, id)
    }

    /// Resolve a qualified declaration name without erasing its declaration role in the AST.
    pub fn qualified_declaration_name(
        &self,
        name: QualifiedDeclarationName,
    ) -> Option<QualifiedReferenceView<'_>> {
        self.qualified_references
            .get(&self.source, name.storage_id())
    }
}

impl std::ops::Deref for ParsedDocument {
    type Target = RootNamespace;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ParsedDocument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::Error as _;

        validate_serialized_document(self).map_err(S::Error::custom)?;
        ParsedDocumentWireRef {
            ast_version: crate::PARSE_AST_VERSION,
            source: &self.source,
            qualified_references: &self.qualified_references,
            root: &self.root,
        }
        .serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ParsedDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error as _;

        let wire = ParsedDocumentWire::deserialize(deserializer)?;
        if wire.ast_version != crate::PARSE_AST_VERSION {
            return Err(D::Error::custom(format_args!(
                "unsupported parsed-document AST version {}; expected {}",
                wire.ast_version,
                crate::PARSE_AST_VERSION
            )));
        }
        let document = Self {
            source: wire.source,
            qualified_references: wire.qualified_references,
            root: wire.root,
        };
        validate_serialized_document(&document).map_err(D::Error::custom)?;
        Ok(document)
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize)]
struct ParsedDocumentWireRef<'a> {
    ast_version: u32,
    source: &'a SourceStorage,
    qualified_references: &'a QualifiedReferenceArena,
    root: &'a RootNamespace,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct ParsedDocumentWire {
    ast_version: u32,
    source: SourceStorage,
    qualified_references: QualifiedReferenceArena,
    root: RootNamespace,
}

#[cfg(feature = "serde")]
fn validate_serialized_document(document: &ParsedDocument) -> Result<(), String> {
    document
        .qualified_references
        .validate(&document.source)
        .map_err(|error| error.to_string())?;
    validate_ast_provenance(document)
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;
    use crate::ast::qualified_reference::{QualifiedReferenceArenaBuilder, ReferenceSegment};
    use crate::ast::{Expression, FilterMember, PackageBodyElement, Span};

    fn source_span() -> Span {
        Span {
            offset: 0,
            line: 1,
            column: 1,
            len: 1,
        }
    }

    fn document_with_reference() -> ParsedDocument {
        let source = SourceStorage::from("A");
        let mut builder = QualifiedReferenceArenaBuilder::new();
        let reference = builder
            .add_reference(
                false,
                source_span(),
                [ReferenceSegment {
                    source_span: source_span(),
                    separator_before: None,
                }],
            )
            .expect("test reference should fit");
        let root = RootNamespace {
            elements: vec![Node::new(
                Span::dummy(),
                RootElement::Member(Box::new(Node::new(
                    Span::dummy(),
                    PackageBodyElement::Filter(Node::new(
                        Span::dummy(),
                        FilterMember {
                            visibility: None,
                            condition: Node::new(source_span(), Expression::FeatureRef(reference)),
                        },
                    )),
                ))),
            )],
        };
        ParsedDocument {
            source,
            qualified_references: builder.finish(),
            root,
        }
    }

    #[test]
    fn parsed_document_roundtrips_as_one_versioned_envelope() {
        let document = document_with_reference();
        let encoded = serde_json::to_value(&document).expect("serialize document");
        assert_eq!(
            encoded
                .get("ast_version")
                .and_then(serde_json::Value::as_u64),
            Some(u64::from(crate::PARSE_AST_VERSION))
        );

        let decoded: ParsedDocument =
            serde_json::from_value(encoded).expect("deserialize document");
        assert_eq!(decoded, document);
    }

    #[test]
    fn parsed_document_rejects_a_mismatched_ast_version() {
        let mut encoded =
            serde_json::to_value(document_with_reference()).expect("serialize document");
        encoded["ast_version"] = serde_json::json!(crate::PARSE_AST_VERSION + 1);

        let error = serde_json::from_value::<ParsedDocument>(encoded)
            .expect_err("mismatched version must be rejected");
        assert!(error
            .to_string()
            .contains("unsupported parsed-document AST version"));
    }

    #[test]
    fn parsed_document_rejects_a_dangling_ast_identity() {
        let mut encoded =
            serde_json::to_value(document_with_reference()).expect("serialize document");
        assert!(replace_variant_number(&mut encoded, "FeatureRef", 99));

        let error = serde_json::from_value::<ParsedDocument>(encoded)
            .expect_err("dangling reference must be rejected");
        assert!(error.to_string().contains("DanglingReference"));
    }

    #[test]
    fn parsed_document_rejects_an_out_of_bounds_segment_range() {
        let mut encoded =
            serde_json::to_value(document_with_reference()).expect("serialize document");
        encoded["qualified_references"]["references"][0]["segments"]["len"] = serde_json::json!(2);

        let error = serde_json::from_value::<ParsedDocument>(encoded)
            .expect_err("invalid segment range must be rejected");
        assert!(error.to_string().contains("SegmentRangeOutOfBounds"));
    }

    #[test]
    fn parsed_document_rejects_tampered_import_suffix_token_provenance() {
        let document = crate::parse("import A:: /* wildcard */ *;").expect("parse import");
        let mut encoded = serde_json::to_value(document).expect("serialize document");
        let marker = find_field_mut(&mut encoded, "marker_span").expect("marker span");
        marker["len"] = serde_json::json!(2);

        let error = serde_json::from_value::<ParsedDocument>(encoded)
            .expect_err("tampered marker token must be rejected");
        assert!(error.to_string().contains("suffix marker"));
    }

    #[test]
    fn parsed_document_rejects_tampered_combined_suffix_provenance() {
        let document = crate::parse("import A::* /* gap */ ::**;").expect("parse import");
        let mut encoded = serde_json::to_value(document).expect("serialize document");
        let combined = find_field_mut(&mut encoded, "combined_recursive_suffix_span")
            .expect("combined suffix span");
        combined["len"] = serde_json::json!(1);

        let error = serde_json::from_value::<ParsedDocument>(encoded)
            .expect_err("tampered combined suffix must be rejected");
        assert!(error.to_string().contains("combined recursive suffix"));
    }

    #[test]
    fn parsed_document_rejects_tampered_expose_filter_delimiter_provenance() {
        let document = crate::parse(
            "package P { view v : V { expose Filtered [ /* gap */ Filters::One ]; } }",
        )
        .expect("parse expose filter");
        let mut encoded = serde_json::to_value(document).expect("serialize document");
        let close = find_field_mut(&mut encoded, "close_bracket_span").expect("close bracket span");
        close["len"] = serde_json::json!(2);

        let error = serde_json::from_value::<ParsedDocument>(encoded)
            .expect_err("tampered filter delimiter must be rejected");
        assert!(error.to_string().contains("filter close bracket"));
    }

    #[test]
    fn parsed_document_rejects_tampered_first_merge_delimiter_provenance() {
        let document = crate::parse(
            "package P { action def A { first Actions::start then Actions::finish { out pin; } } }",
        )
        .expect("parse first/merge body");
        let mut encoded = serde_json::to_value(document).expect("serialize document");
        let open = find_field_mut(&mut encoded, "open_brace_span").expect("open brace span");
        open["len"] = serde_json::json!(2);

        let error = serde_json::from_value::<ParsedDocument>(encoded)
            .expect_err("tampered first/merge delimiter must be rejected");
        assert!(error.to_string().contains("first/merge open brace"));
    }

    #[test]
    fn parsed_document_validates_imports_nested_beneath_action_and_use_case_members() {
        for source in [
            "action def A { part p { import Nested::ActionTarget::*; } }",
            "use case def U { part p { import Nested::UseCaseTarget::*; } }",
        ] {
            let document = crate::parse(source).expect("nested import");
            let mut encoded = serde_json::to_value(document).expect("serialize document");
            let target_span = find_import_target_mut(&mut encoded)
                .and_then(|target| target.get_mut("span"))
                .expect("nested import target span");
            target_span["len"] = serde_json::json!(1);

            let error = serde_json::from_value::<ParsedDocument>(encoded)
                .expect_err("nested import provenance must be validated");
            assert!(error.to_string().contains("import target"));
        }
    }

    #[test]
    fn a_partial_root_is_not_a_complete_serialized_document() {
        let document = document_with_reference();
        let root_only = serde_json::to_value(&document.root).expect("serialize root component");
        assert!(serde_json::from_value::<ParsedDocument>(root_only).is_err());
    }

    fn replace_variant_number(
        value: &mut serde_json::Value,
        variant: &str,
        replacement: u64,
    ) -> bool {
        match value {
            serde_json::Value::Array(values) => values
                .iter_mut()
                .any(|value| replace_variant_number(value, variant, replacement)),
            serde_json::Value::Object(fields) => {
                if let Some(value) = fields.get_mut(variant) {
                    if value.is_number() {
                        *value = serde_json::json!(replacement);
                        return true;
                    }
                }
                fields
                    .values_mut()
                    .any(|value| replace_variant_number(value, variant, replacement))
            }
            _ => false,
        }
    }

    fn find_field_mut<'a>(
        value: &'a mut serde_json::Value,
        field: &str,
    ) -> Option<&'a mut serde_json::Value> {
        match value {
            serde_json::Value::Array(values) => values
                .iter_mut()
                .find_map(|value| find_field_mut(value, field)),
            serde_json::Value::Object(fields) => {
                if fields.contains_key(field) {
                    return fields.get_mut(field);
                }
                fields
                    .values_mut()
                    .find_map(|value| find_field_mut(value, field))
            }
            _ => None,
        }
    }

    fn find_import_target_mut(
        value: &mut serde_json::Value,
    ) -> Option<&mut serde_json::Map<String, serde_json::Value>> {
        match value {
            serde_json::Value::Array(values) => values.iter_mut().find_map(find_import_target_mut),
            serde_json::Value::Object(fields) => {
                if fields.contains_key("span")
                    && fields.contains_key("all_span")
                    && fields.contains_key("reference")
                    && fields.contains_key("shape")
                {
                    return Some(fields);
                }
                fields.values_mut().find_map(find_import_target_mut)
            }
            _ => None,
        }
    }
}
