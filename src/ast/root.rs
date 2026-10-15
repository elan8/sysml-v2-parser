use super::common::Import;
use super::package::{
    LibraryPackage, Package, PackageBody, PackageBodyElement, QualifiedDeclarationName,
    QualifiedIdentification,
};
#[cfg(feature = "serde")]
use super::provenance_visit::validate_ast_provenance;
use super::qualified_reference::{
    decode_authored_name, QualifiedReferenceArena, QualifiedReferenceId, QualifiedReferenceView,
    SourceRange, SourceStorage,
};
use crate::ast::common::{normalize_comment_body, CommentBody, DeclarationName};
use crate::ast::core::{
    decode_string_literal, Node, OperatorSpelling, RealLiteral, Span, StringLiteral,
};
use std::borrow::Cow;

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

/// Language whose lexical NAME rules govern one parsed document.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum NameLanguage {
    #[default]
    Sysml,
    Kerml,
}

mod reserved_keywords {
    include!(concat!(env!("OUT_DIR"), "/reserved_keywords.rs"));
}

impl NameLanguage {
    /// Borrow the sorted pinned keyword inventory, without maintaining a consumer-owned copy.
    pub const fn reserved_keywords(self) -> &'static [&'static str] {
        match self {
            Self::Sysml => reserved_keywords::SYSML,
            Self::Kerml => reserved_keywords::KERML,
        }
    }

    /// Classify a bare lexical token using the pinned language keyword inventory.
    pub fn is_reserved(self, word: &str) -> bool {
        self.reserved_keywords().binary_search(&word).is_ok()
    }
}

/// One parsed SysML/KerML source file and all document-local identities referenced by its AST.
///
/// [`RootNamespace`] nodes use opaque qualified-reference identities whose source spelling and
/// segment metadata live in [`qualified_references`](Self::qualified_references). Keeping the
/// source, arena, and AST in one envelope prevents consumers from accidentally treating those
/// identities as globally stable or interpreting an AST without the arena that owns them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedDocument {
    /// Lexical name vocabulary used to interpret the source and retained recovery states.
    pub name_language: NameLanguage,
    /// BOM-stripped source text whose byte offsets back AST/reference spans.
    pub source: SourceStorage,
    /// Document-local qualified references shared by all semantic AST nodes.
    pub qualified_references: QualifiedReferenceArena,
    /// Parsed root namespace.
    pub root: RootNamespace,
}

// `Send`, `Sync` and the other auto traits are implemented explicitly, with a structural proof,
// in `super::auto_traits`: proving them field by field costs downstream crates more trait-solver
// frames than the default `recursion_limit` allows.

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

    /// Authored spelling of a declaration name or short name, exactly as written in the source
    /// (an unrestricted name keeps its quotes and escapes). `None` only for a span that does not
    /// belong to this document.
    pub fn declaration_name(&self, name: DeclarationName) -> Option<&str> {
        self.source.slice(name.span())
    }

    /// Decoded `NAME` value of a declaration name: a basic name as written, or the contents of
    /// an unrestricted name with its quotes removed and `\'` escapes decoded. Only an escaped
    /// unrestricted name allocates. Retained reserved-keyword syntax has no valid decoded name
    /// and returns `None`, as does a span outside this document.
    pub fn decoded_declaration_name(&self, name: DeclarationName) -> Option<Cow<'_, str>> {
        match name.state() {
            super::DeclarationNameState::Parsed => {
                decode_authored_name(self.declaration_name(name)?)
            }
            super::DeclarationNameState::ReservedKeyword => None,
        }
    }

    /// Authored spelling of a real literal.
    pub fn real_literal(&self, literal: RealLiteral) -> Option<&str> {
        self.source.slice(literal.span())
    }

    /// Authored spelling of a string literal, quotes and escapes intact.
    pub fn string_literal(&self, literal: StringLiteral) -> Option<&str> {
        self.source.slice(literal.span())
    }

    /// Contents of a string literal with its quotes removed and `\"` escapes decoded. Only a
    /// literal containing an escape allocates.
    pub fn decoded_string_literal(&self, literal: StringLiteral) -> Option<Cow<'_, str>> {
        decode_string_literal(self.string_literal(literal)?)
    }

    /// The authored bytes of a `REGULAR_COMMENT` body, without its `/*` `*/` delimiters.
    pub fn comment_body(&self, body: CommentBody) -> Option<&str> {
        self.source.slice(body.span())
    }

    /// A comment body with the pinned processing rules applied; see
    /// [`normalize_comment_body`](crate::ast::normalize_comment_body). Borrows when the rules
    /// change nothing.
    pub fn normalized_comment_body(&self, body: CommentBody) -> Option<Cow<'_, str>> {
        Some(normalize_comment_body(self.comment_body(body)?))
    }

    /// The authored spelling of an operator the grammar did not classify.
    pub fn operator_spelling(&self, operator: OperatorSpelling) -> Option<&str> {
        self.source.slice(operator.span())
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
            name_language: self.name_language,
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
            name_language: wire.name_language,
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
    name_language: NameLanguage,
    source: &'a SourceStorage,
    qualified_references: &'a QualifiedReferenceArena,
    root: &'a RootNamespace,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct ParsedDocumentWire {
    ast_version: u32,
    name_language: NameLanguage,
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
    use crate::ast::visit::mutable::{walk_kerml_binding_member, VisitorMut};
    use crate::ast::{
        Expression, FeatureSpecialization, FilterMember, FlowPayloadClause, InOutDecl,
        KermlBindingEndPair, KermlBindingMember, PackageBodyElement, Span, SubsettingKind,
    };

    fn serialize_unvalidated_wire(document: &ParsedDocument) -> serde_json::Value {
        serde_json::to_value(ParsedDocumentWireRef {
            ast_version: crate::PARSE_AST_VERSION,
            name_language: document.name_language,
            source: &document.source,
            qualified_references: &document.qualified_references,
            root: &document.root,
        })
        .expect("the deliberately invalid wire document serializes")
    }

    fn assert_typed_corruption_is_rejected(source: &str, mutate: impl FnOnce(&mut ParsedDocument)) {
        let mut document = crate::parse_for_editor(source).document;
        mutate(&mut document);
        let encoded = serialize_unvalidated_wire(&document);
        serde_json::from_value::<ParsedDocument>(encoded)
            .expect_err("deserialization must reject the typed provenance corruption");
    }

    #[test]
    fn malformed_name_cannot_be_serialized_as_a_successful_name() {
        struct HideInvalidName;
        impl VisitorMut for HideInvalidName {
            fn visit_declaration_name(&mut self, name: &mut DeclarationName) {
                if name.state == crate::ast::DeclarationNameState::ReservedKeyword {
                    name.state = crate::ast::DeclarationNameState::Parsed;
                }
            }
        }
        assert_typed_corruption_is_rejected("package P { part def port; }", |document| {
            HideInvalidName.visit_root_namespace(&mut document.root);
        });
    }

    struct CorruptRecoveryDiagnostic(Span);

    impl VisitorMut for CorruptRecoveryDiagnostic {
        fn visit_parse_error_node_value(&mut self, error: &mut crate::ast::ParseErrorNode) {
            assert!(
                error.diagnostic_span.is_some(),
                "the fixture must exercise a token diagnostic"
            );
            error.diagnostic_span = Some(self.0);
        }
    }

    #[test]
    fn recovery_token_span_is_validated_with_its_source_and_owner() {
        let source = "constraint bad { 1 == 1; }";
        let document = crate::parse_for_editor(source).document;
        let wire = serde_json::to_value(&document).expect("valid recovered document serializes");
        let restored: ParsedDocument =
            serde_json::from_value(wire).expect("valid token span roundtrips");
        assert_eq!(document, restored);
        for span in [
            // Valid source coordinates, outside this malformed expression's owner.
            Span {
                offset: 0,
                line: 1,
                column: 1,
                len: 1,
            },
            // The offending token's offset with a fabricated line.
            Span {
                offset: 22,
                line: 2,
                column: 23,
                len: 1,
            },
            Span {
                offset: 22,
                line: 1,
                column: 23,
                len: 0,
            },
            Span {
                offset: usize::MAX,
                line: 1,
                column: 1,
                len: 1,
            },
        ] {
            assert_typed_corruption_is_rejected(source, |document| {
                apply_visitor(&mut CorruptRecoveryDiagnostic(span), document);
            });
        }
    }

    struct RemoveImportVisibility;

    impl VisitorMut for RemoveImportVisibility {
        fn visit_import(&mut self, node: &mut Node<crate::ast::Import>) {
            node.value.membership.visibility = None;
        }
    }

    #[test]
    fn import_requires_visibility_in_the_serialized_document() {
        assert_typed_corruption_is_rejected("private import A::*;", |document| {
            apply_visitor(&mut RemoveImportVisibility, document);
        });
    }

    struct CorruptDirectedActionKind(bool);

    impl VisitorMut for CorruptDirectedActionKind {
        fn visit_in_out_decl(&mut self, node: &mut Node<InOutDecl>) {
            if let Some(kind) = &mut node.value.kind {
                kind.span.offset = 0;
                self.0 = true;
            }
        }
    }

    struct CorruptFlowPayload(bool);

    impl VisitorMut for CorruptFlowPayload {
        fn visit_flow_payload_clause(&mut self, node: &mut Node<FlowPayloadClause>) {
            node.value.of_span.offset = 0;
            self.0 = true;
        }
    }

    struct CorruptCrossSubsetting(bool);

    impl VisitorMut for CorruptCrossSubsetting {
        fn visit_feature_specialization(&mut self, node: &mut FeatureSpecialization) {
            if let FeatureSpecialization::CrossSubsetting(relationship) = node {
                relationship.value.kind = SubsettingKind::References;
                self.0 = true;
            }
        }
    }

    #[derive(Clone, Copy)]
    enum BindingCorruption {
        AllSpan,
        OfSpan,
        EqualsSpan,
        RemoveDeclaredOf,
    }

    struct CorruptBinding {
        corruption: BindingCorruption,
        applied: bool,
    }

    impl VisitorMut for CorruptBinding {
        fn visit_kerml_binding_member(&mut self, node: &mut Node<KermlBindingMember>) {
            match self.corruption {
                BindingCorruption::AllSpan => {
                    if let Some(span) = &mut node.value.all_span {
                        span.offset = 0;
                        self.applied = true;
                        return;
                    }
                }
                BindingCorruption::RemoveDeclaredOf if node.value.name.is_some() => {
                    if let Some(pair) = &mut node.value.inline_ends {
                        pair.value.of_span = None;
                        self.applied = true;
                        return;
                    }
                }
                BindingCorruption::OfSpan
                | BindingCorruption::EqualsSpan
                | BindingCorruption::RemoveDeclaredOf => {}
            }
            walk_kerml_binding_member(self, node);
        }

        fn visit_kerml_binding_end_pair(&mut self, node: &mut Node<KermlBindingEndPair>) {
            match self.corruption {
                BindingCorruption::OfSpan => {
                    if let Some(span) = &mut node.value.of_span {
                        span.offset = 0;
                        self.applied = true;
                    }
                }
                BindingCorruption::EqualsSpan => {
                    node.value.equals_span.offset = 0;
                    self.applied = true;
                }
                BindingCorruption::AllSpan | BindingCorruption::RemoveDeclaredOf => {}
            }
        }
    }

    fn apply_visitor(visitor: &mut impl VisitorMut, document: &mut ParsedDocument) {
        visitor.visit_root_namespace(&mut document.root);
    }

    #[test]
    fn rejects_a_directed_action_kind_span_covering_other_text() {
        assert_typed_corruption_is_rejected("action def A { in action body {} }", |document| {
            let mut corruption = CorruptDirectedActionKind(false);
            apply_visitor(&mut corruption, document);
            assert!(
                corruption.0,
                "the typed traversal must find the action kind"
            );
        });
    }

    #[test]
    fn rejects_a_flow_payload_of_span_covering_other_text() {
        assert_typed_corruption_is_rejected("flow of Thing from source to target;", |document| {
            let mut corruption = CorruptFlowPayload(false);
            apply_visitor(&mut corruption, document);
            assert!(corruption.0, "the typed traversal must find the payload");
        });
    }

    #[test]
    fn rejects_a_cross_subsetting_with_the_wrong_relationship_kind() {
        assert_typed_corruption_is_rejected("feature f crosses target;", |document| {
            let mut corruption = CorruptCrossSubsetting(false);
            apply_visitor(&mut corruption, document);
            assert!(
                corruption.0,
                "the typed traversal must find the cross subsetting"
            );
        });
    }

    fn assert_binding_corruption_is_rejected(source: &str, kind: BindingCorruption) {
        assert_typed_corruption_is_rejected(source, |document| {
            let mut corruption = CorruptBinding {
                corruption: kind,
                applied: false,
            };
            apply_visitor(&mut corruption, document);
            assert!(
                corruption.applied,
                "the typed traversal must find the binding syntax"
            );
        });
    }

    #[test]
    fn rejects_a_binding_all_span_covering_other_text() {
        assert_binding_corruption_is_rejected(
            "classifier C { binding all; }",
            BindingCorruption::AllSpan,
        );
    }

    #[test]
    fn rejects_an_inline_binding_of_span_covering_other_text() {
        assert_binding_corruption_is_rejected(
            "classifier C { binding named of left = right; }",
            BindingCorruption::OfSpan,
        );
    }

    #[test]
    fn rejects_an_inline_binding_equals_span_covering_other_text() {
        assert_binding_corruption_is_rejected(
            "classifier C { binding of left = right; }",
            BindingCorruption::EqualsSpan,
        );
    }

    #[test]
    fn rejects_declared_inline_binding_ends_without_of() {
        assert_binding_corruption_is_rejected(
            "classifier C { binding named of left = right; }",
            BindingCorruption::RemoveDeclaredOf,
        );
    }

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
            name_language: NameLanguage::Sysml,
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
        let document = crate::parse("private import A:: /* wildcard */ *;").expect("parse import");
        let mut encoded = serde_json::to_value(document).expect("serialize document");
        let marker = find_field_mut(&mut encoded, "marker_span").expect("marker span");
        marker["len"] = serde_json::json!(2);

        let error = serde_json::from_value::<ParsedDocument>(encoded)
            .expect_err("tampered marker token must be rejected");
        assert!(error.to_string().contains("suffix marker"));
    }

    #[test]
    fn parsed_document_rejects_tampered_combined_suffix_provenance() {
        let document = crate::parse("private import A::* /* gap */ ::**;").expect("parse import");
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
            "action def A { part p { private import Nested::ActionTarget::*; } }",
            "use case def U { part p { private import Nested::UseCaseTarget::*; } }",
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
