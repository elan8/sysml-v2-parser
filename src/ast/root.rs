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

// SAFETY: every field of `ParsedDocument` is itself `Send` and `Sync`, which is exactly what the
// automatic implementations would have required. These explicit implementations exist only to stop
// the *proof* from being recomputed structurally in every consumer.
//
// The AST is a deeply mutually recursive tree -- a requirement body holds a part usage holds a
// perform holds a variant requirement holds a requirement body -- so the longest simple path
// through its type graph is over 120 distinct types. Proving `ParsedDocument: Send` structurally
// costs that many trait-solver frames, which is more than rustc's default `recursion_limit` of
// 128 allows, so every consumer that merely wants to move a document between threads had to raise
// its own limit. These implementations make the obligation O(1) for them.
//
// The safety obligation is not discharged by review: `send_sync_structural_proof` below proves it
// structurally, field by field, inside this crate (which raises `recursion_limit` for exactly this
// reason). Adding a field, or changing one to a type that is not `Send`/`Sync` such as `Rc`,
// `RefCell` or a raw pointer, fails that proof to compile.
unsafe impl Send for ParsedDocument {}
// SAFETY: as above -- `send_sync_structural_proof` proves every field `Sync`.
unsafe impl Sync for ParsedDocument {}

/// The compiler-checked proof that discharges the `unsafe impl Send`/`Sync` above.
///
/// This must never assert anything about [`ParsedDocument`] itself: the explicit implementations
/// would make such an assertion vacuously true and the guard would silently stop guarding. It
/// asserts the *field types*, which still resolve structurally, and destructures the struct so the
/// field list cannot drift away from the assertions.
///
/// It lives in the library rather than in a test so that it is checked by every build, including
/// `cargo check`, rather than only when the test suite runs.
#[allow(dead_code)]
fn send_sync_structural_proof(document: ParsedDocument) {
    fn assert_send_sync<T: Send + Sync>() {}

    // Exhaustive by construction: adding or renaming a field of `ParsedDocument` fails here, so a
    // new field cannot reach the `unsafe impl`s without an assertion of its own below.
    let ParsedDocument {
        source,
        qualified_references,
        root,
    } = document;
    let _ = (&source, &qualified_references, &root);

    assert_send_sync::<SourceStorage>();
    assert_send_sync::<QualifiedReferenceArena>();
    // The expensive one: this is the full structural walk of the AST type graph, the proof the
    // `unsafe impl`s stand on and the reason this crate raises `recursion_limit`.
    assert_send_sync::<RootNamespace>();
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
    fn parsed_document_rejects_a_tampered_body_delimiter() {
        let document = crate::parse_for_editor("package P { part def A; }").document;
        let mut encoded = serde_json::to_value(&document).expect("serialize document");
        // Point the package body's open brace at the wrong token.
        let body = encoded
            .pointer_mut("/root/elements/0/value/Package/value/body/Brace/open_span/offset")
            .expect("open brace span in the wire document");
        *body = serde_json::json!(0);
        let error = serde_json::from_value::<ParsedDocument>(encoded)
            .expect_err("a delimiter span that no longer covers its token must be rejected");
        assert!(
            error.to_string().contains("open brace"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn parsed_document_rejects_an_inside_out_body() {
        let document = crate::parse_for_editor("package P { part def A; }").document;
        let mut encoded = serde_json::to_value(&document).expect("serialize document");
        let open = encoded
            .pointer("/root/elements/0/value/Package/value/body/Brace/open_span")
            .cloned()
            .expect("open brace span");
        let close = encoded
            .pointer("/root/elements/0/value/Package/value/body/Brace/close_span")
            .cloned()
            .expect("close brace span");
        *encoded
            .pointer_mut("/root/elements/0/value/Package/value/body/Brace/open_span")
            .expect("open brace span") = close;
        *encoded
            .pointer_mut("/root/elements/0/value/Package/value/body/Brace/close_span")
            .expect("close brace span") = open;
        let error = serde_json::from_value::<ParsedDocument>(encoded)
            .expect_err("a body whose close precedes its open must be rejected");
        assert!(
            error.to_string().contains("brace"),
            "unexpected error: {error}"
        );
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

    /// A `first`/`merge` body is the innermost body of this fixture, so tampering the innermost
    /// `open_span` tampers that body specifically.
    ///
    /// It used to spell its delimiters as `open_brace_span`/`close_brace_span` on a body type of
    /// its own; sharing `Body` means those are the ordinary `open_span`/`close_span` every body
    /// carries, and the field name alone no longer selects this scope.
    #[test]
    fn parsed_document_rejects_tampered_first_merge_delimiter_provenance() {
        let document = crate::parse(
            "package P { action def A { first Actions::start then Actions::finish { out pin; } } }",
        )
        .expect("parse first/merge body");
        let mut encoded = serde_json::to_value(document).expect("serialize document");
        let open =
            find_innermost_field_mut(&mut encoded, "open_span").expect("first/merge open brace");
        open["len"] = serde_json::json!(2);

        let error = serde_json::from_value::<ParsedDocument>(encoded)
            .expect_err("tampered first/merge delimiter must be rejected");
        assert!(
            error.to_string().contains("first/merge open brace"),
            "unexpected error: {error}"
        );
    }

    /// The rules this scope keeps beyond the delimiter checks every body now gets: an element's
    /// span must match the span of the member it retains.
    #[test]
    fn parsed_document_rejects_a_first_merge_element_span_that_left_its_member_behind() {
        let document = crate::parse(
            "package P { action def A { first Actions::start then Actions::finish { out pin; } } }",
        )
        .expect("parse first/merge body");
        let mut encoded = serde_json::to_value(document).expect("serialize document");
        let elements = find_innermost_field_mut(&mut encoded, "elements")
            .and_then(|elements| elements.get_mut(0))
            .and_then(|element| element.get_mut("span"))
            .expect("first/merge body element span");
        let shrunk = elements["len"].as_u64().expect("element span length") - 1;
        elements["len"] = serde_json::json!(shrunk);

        let error = serde_json::from_value::<ParsedDocument>(encoded)
            .expect_err("a first/merge element span must still match its retained member");
        assert!(
            error
                .to_string()
                .contains("first/merge body element and retained member spans must match"),
            "unexpected error: {error}"
        );
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

    /// Like [`find_field_mut`], but returns the *deepest* match along the first matching path.
    ///
    /// Field names that used to name one scope's bespoke type now name the shared one, so the
    /// outermost match is a different body than the test means.
    fn find_innermost_field_mut<'a>(
        value: &'a mut serde_json::Value,
        field: &str,
    ) -> Option<&'a mut serde_json::Value> {
        match value {
            serde_json::Value::Array(values) => values
                .iter_mut()
                .find_map(|value| find_innermost_field_mut(value, field)),
            serde_json::Value::Object(fields) => {
                let nested = fields
                    .values_mut()
                    .any(|value| find_innermost_field_mut(value, field).is_some());
                if nested {
                    return fields
                        .values_mut()
                        .find_map(|value| find_innermost_field_mut(value, field));
                }
                fields.get_mut(field)
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
