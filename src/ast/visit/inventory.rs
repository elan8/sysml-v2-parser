//! The single traversal inventory for the AST, expanded once per borrow kind.
//!
//! `ast_traversal!` is invoked twice -- by `super::immutable` as `Visitor` and by
//! `super::mutable` as `VisitorMut` -- with `$mutability` empty or `mut`. Everything else in the
//! body is shared, so the two directions cannot diverge. Match ergonomics do most of the work:
//! the same destructuring and iteration code binds `&T` or `&mut T` depending on the borrow, and
//! `&$($mutability)?` appears only where a `Box` or a `Node` field has to be reborrowed.
//!
//! # Extending
//!
//! Adding a field or variant to a type reachable from `RootNamespace` makes this file fail to
//! compile: struct patterns list every field and enum matches list every variant, with no `..`
//! and no `_`. Answer the question the failure asks -- does the new syntax carry child nodes,
//! spans, references, or authored text? -- and every consumer of the traversal inherits it.
//!
//! Naming: a type that always appears wrapped gets `visit_x(&Node<X>)`; a type that also appears
//! bare gets `visit_x(&Node<X>)` plus `visit_x_value(&X)` so consumers keep node spans where
//! they exist. Types with no children still get a walker so a variant that gains a payload
//! breaks here too.

macro_rules! ast_traversal {
    ($(#[$trait_doc:meta])* $Visitor:ident, $($mutability:ident)?) => {
        $(#[$trait_doc])*
        pub trait $Visitor: Sized {
            /// Called for every source span in the tree, including `Node` spans.
            fn visit_span(&mut self, _span: &$($mutability)? Span) {}

            /// Called for every qualified-reference identity stored in the tree.
            fn visit_qualified_reference(&mut self, _reference: &$($mutability)? QualifiedReferenceId) {}

            /// Called on entering a node, before its span and children, and paired with
            /// [`leave_node`](Self::leave_node) on the way out.
            ///
            /// Consumers that need to know where they are -- which declaration owns the span they
            /// are looking at -- track it here rather than inferring it from traversal order.
            fn enter_node(&mut self, _span: &$($mutability)? Span) {}

            /// Called on leaving a node, after its children.
            fn leave_node(&mut self, _span: &$($mutability)? Span) {}

            /// Called for a brace body's delimiter pair, in addition to visiting each span.
            ///
            /// Every scope's body reports through here, so a consumer that checks delimiter
            /// provenance states the rule once instead of overriding 25 body methods.
            fn visit_body_braces<E>(
                &mut self,
                _open: &$($mutability)? Span,
                _elements: &$($mutability)? [Node<E>],
                _close: &$($mutability)? Span,
            ) {
            }

            /// Called for a semicolon body's `;`, in addition to visiting its span.
            fn visit_body_semicolon(&mut self, _semicolon: &$($mutability)? Span) {}

            /// Called for every authored string the tree stores directly: declaration and
            /// short names, literal spellings, annotation text, and opaque retained syntax.
            fn visit_text(&mut self, _text: &$($mutability)? String) {}

            /// Visits [`BinaryOperator`]; the default implementation walks its children.
            fn visit_binary_operator(&mut self, node: &$($mutability)? BinaryOperator) {
                walk_binary_operator(self, node)
            }

            /// Visits [`TypeCheckKind`]; the default implementation walks its children.
            fn visit_type_check_kind(&mut self, node: &$($mutability)? TypeCheckKind) {
                walk_type_check_kind(self, node)
            }

            /// Visits [`UnaryOperator`]; the default implementation walks its children.
            fn visit_unary_operator(&mut self, node: &$($mutability)? UnaryOperator) {
                walk_unary_operator(self, node)
            }

            /// Visits [`Expression`]; the default implementation walks its children.
            fn visit_expression(&mut self, node: &$($mutability)? Node<Expression>) {
                walk_expression(self, node)
            }

            /// Visits [`CollectionOperatorBody`]; the default implementation walks its children.
            fn visit_collection_operator_body(&mut self, node: &$($mutability)? Node<CollectionOperatorBody>) {
                walk_collection_operator_body(self, node)
            }

            /// Visits [`CollectionOperatorParameter`]; the default implementation walks its children.
            fn visit_collection_operator_parameter(&mut self, node: &$($mutability)? Node<CollectionOperatorParameter>) {
                walk_collection_operator_parameter(self, node)
            }

            /// Visits [`CollectionOperatorParameterTerminator`]; the default implementation walks its children.
            fn visit_collection_operator_parameter_terminator(&mut self, node: &$($mutability)? CollectionOperatorParameterTerminator) {
                walk_collection_operator_parameter_terminator(self, node)
            }

            /// Visits [`CollectionOperatorParameterTyping`]; the default implementation walks its children.
            fn visit_collection_operator_parameter_typing(&mut self, node: &$($mutability)? CollectionOperatorParameterTyping) {
                walk_collection_operator_parameter_typing(self, node)
            }

            /// Visits [`Argument`]; the default implementation walks its children.
            fn visit_argument(&mut self, node: &$($mutability)? Argument) {
                walk_argument(self, node)
            }

            /// Visits [`CollectionOperator`]; the default implementation walks its children.
            fn visit_collection_operator(&mut self, node: &$($mutability)? CollectionOperator) {
                walk_collection_operator(self, node)
            }

            /// Visits [`Multiplicity`]; the default implementation walks its children.
            fn visit_multiplicity(&mut self, node: &$($mutability)? Node<Multiplicity>) {
                walk_multiplicity(self, node)
            }

            /// Visits [`TypingKind`]; the default implementation walks its children.
            fn visit_typing_kind(&mut self, node: &$($mutability)? TypingKind) {
                walk_typing_kind(self, node)
            }

            /// Visits [`TypingRelationship`]; the default implementation walks its children.
            fn visit_typing_relationship(&mut self, node: &$($mutability)? Node<TypingRelationship>) {
                walk_typing_relationship(self, node)
            }

            /// Visits [`TypingSpelling`]; the default implementation walks its children.
            fn visit_typing_spelling(&mut self, node: &$($mutability)? TypingSpelling) {
                walk_typing_spelling(self, node)
            }

            /// Visits [`SubsettingKind`]; the default implementation walks its children.
            fn visit_subsetting_kind(&mut self, node: &$($mutability)? SubsettingKind) {
                walk_subsetting_kind(self, node)
            }

            /// Visits [`SubsettingRelationship`]; the default implementation walks its children.
            fn visit_subsetting_relationship(&mut self, node: &$($mutability)? Node<SubsettingRelationship>) {
                walk_subsetting_relationship(self, node)
            }

            /// Visits [`ConnectionEnd`]; the default implementation walks its children.
            fn visit_connection_end(&mut self, node: &$($mutability)? Node<ConnectionEnd>) {
                walk_connection_end(self, node)
            }

            /// Visits [`FilterMember`]; the default implementation walks its children.
            fn visit_filter_member(&mut self, node: &$($mutability)? Node<FilterMember>) {
                walk_filter_member(self, node)
            }

            /// Visits [`ParseErrorNode`]; the default implementation walks its children.
            fn visit_parse_error_node(&mut self, node: &$($mutability)? Node<ParseErrorNode>) {
                walk_parse_error_node(self, node)
            }

            /// Visits a [`ParseErrorNode`] that its parent stores without a node wrapper.
            fn visit_parse_error_node_value(&mut self, value: &$($mutability)? ParseErrorNode) {
                walk_parse_error_node_value(self, value)
            }

            /// Visits [`UnsupportedProduction`]; the default implementation walks its children.
            fn visit_unsupported_production(&mut self, node: &$($mutability)? UnsupportedProduction) {
                walk_unsupported_production(self, node)
            }

            /// Visits [`UnsupportedGrammarNode`]; the default implementation walks its children.
            fn visit_unsupported_grammar_node(&mut self, node: &$($mutability)? Node<UnsupportedGrammarNode>) {
                walk_unsupported_grammar_node(self, node)
            }

            /// Visits [`Identification`]; the default implementation walks its children.
            fn visit_identification(&mut self, node: &$($mutability)? Identification) {
                walk_identification(self, node)
            }

            /// Visits [`Visibility`]; the default implementation walks its children.
            fn visit_visibility(&mut self, node: &$($mutability)? Visibility) {
                walk_visibility(self, node)
            }

            /// Visits [`FilterPackageMember`]; the default implementation walks its children.
            fn visit_filter_package_member(&mut self, node: &$($mutability)? Node<FilterPackageMember>) {
                walk_filter_package_member(self, node)
            }

            /// Visits [`ImportSuffixSpans`]; the default implementation walks its children.
            fn visit_import_suffix_spans(&mut self, node: &$($mutability)? ImportSuffixSpans) {
                walk_import_suffix_spans(self, node)
            }

            /// Visits [`ImportShape`]; the default implementation walks its children.
            fn visit_import_shape(&mut self, node: &$($mutability)? ImportShape) {
                walk_import_shape(self, node)
            }

            /// Visits [`ImportTarget`]; the default implementation walks its children.
            fn visit_import_target(&mut self, node: &$($mutability)? ImportTarget) {
                walk_import_target(self, node)
            }

            /// Visits [`Import`]; the default implementation walks its children.
            fn visit_import(&mut self, node: &$($mutability)? Node<Import>) {
                walk_import(self, node)
            }

            /// Visits [`DocComment`]; the default implementation walks its children.
            fn visit_doc_comment(&mut self, node: &$($mutability)? Node<DocComment>) {
                walk_doc_comment(self, node)
            }

            /// Visits [`CommentAnnotation`]; the default implementation walks its children.
            fn visit_comment_annotation(&mut self, node: &$($mutability)? Node<CommentAnnotation>) {
                walk_comment_annotation(self, node)
            }

            /// Visits [`TextualRepresentation`]; the default implementation walks its children.
            fn visit_textual_representation(&mut self, node: &$($mutability)? Node<TextualRepresentation>) {
                walk_textual_representation(self, node)
            }

            /// Visits [`AnnotatingMember`]; the default implementation walks its children.
            fn visit_annotating_member(&mut self, node: &$($mutability)? AnnotatingMember) {
                walk_annotating_member(self, node)
            }

            /// Visits [`FeatureValueKind`]; the default implementation walks its children.
            fn visit_feature_value_kind(&mut self, node: &$($mutability)? FeatureValueKind) {
                walk_feature_value_kind(self, node)
            }

            /// Visits [`FeatureValue`]; the default implementation walks its children.
            fn visit_feature_value(&mut self, node: &$($mutability)? Node<FeatureValue>) {
                walk_feature_value(self, node)
            }

            /// Visits [`MembershipKind`]; the default implementation walks its children.
            fn visit_membership_kind(&mut self, node: &$($mutability)? MembershipKind) {
                walk_membership_kind(self, node)
            }

            /// Visits [`Membership`]; the default implementation walks its children.
            fn visit_membership(&mut self, node: &$($mutability)? Membership) {
                walk_membership(self, node)
            }

            /// Visits [`ReferenceSeparator`]; the default implementation walks its children.
            fn visit_reference_separator(&mut self, node: &$($mutability)? ReferenceSeparator) {
                walk_reference_separator(self, node)
            }

            /// Visits [`RootElement`]; the default implementation walks its children.
            fn visit_root_element(&mut self, node: &$($mutability)? Node<RootElement>) {
                walk_root_element(self, node)
            }

            /// Visits [`NamespaceDecl`]; the default implementation walks its children.
            fn visit_namespace_decl(&mut self, node: &$($mutability)? Node<NamespaceDecl>) {
                walk_namespace_decl(self, node)
            }

            /// Visits [`RootNamespace`]; the default implementation walks its children.
            fn visit_root_namespace(&mut self, node: &$($mutability)? RootNamespace) {
                walk_root_namespace(self, node)
            }

            /// Visits [`QualifiedDeclarationName`]; the default implementation walks its children.
            fn visit_qualified_declaration_name(&mut self, node: &$($mutability)? QualifiedDeclarationName) {
                walk_qualified_declaration_name(self, node)
            }

            /// Visits [`DeclarationName`]; the default implementation walks its children.
            fn visit_declaration_name(&mut self, node: &$($mutability)? DeclarationName) {
                walk_declaration_name(self, node)
            }

            /// Visits [`QualifiedIdentification`]; the default implementation walks its children.
            fn visit_qualified_identification(&mut self, node: &$($mutability)? QualifiedIdentification) {
                walk_qualified_identification(self, node)
            }

            /// Visits [`Package`]; the default implementation walks its children.
            fn visit_package(&mut self, node: &$($mutability)? Node<Package>) {
                walk_package(self, node)
            }

            /// Visits [`PackageBody`]; the default implementation walks its children.
            fn visit_package_body(&mut self, node: &$($mutability)? PackageBody) {
                walk_package_body(self, node)
            }

            /// Visits [`LibraryPackage`]; the default implementation walks its children.
            fn visit_library_package(&mut self, node: &$($mutability)? Node<LibraryPackage>) {
                walk_library_package(self, node)
            }

            /// Visits [`PackageBodyElement`]; the default implementation walks its children.
            fn visit_package_body_element(&mut self, node: &$($mutability)? Node<PackageBodyElement>) {
                walk_package_body_element(self, node)
            }

            /// Visits [`PartDef`]; the default implementation walks its children.
            fn visit_part_def(&mut self, node: &$($mutability)? Node<PartDef>) {
                walk_part_def(self, node)
            }

            /// Visits [`ExtendedDefinition`]; the default implementation walks its children.
            fn visit_extended_definition(&mut self, node: &$($mutability)? Node<ExtendedDefinition>) {
                walk_extended_definition(self, node)
            }

            /// Visits [`DefinitionPrefix`]; the default implementation walks its children.
            fn visit_definition_prefix(&mut self, node: &$($mutability)? Node<DefinitionPrefix>) {
                walk_definition_prefix(self, node)
            }

            /// Visits a [`DefinitionPrefix`] that its parent stores without a node wrapper.
            fn visit_definition_prefix_value(&mut self, value: &$($mutability)? DefinitionPrefix) {
                walk_definition_prefix_value(self, value)
            }

            /// Visits [`PartDefBody`]; the default implementation walks its children.
            fn visit_part_def_body(&mut self, node: &$($mutability)? PartDefBody) {
                walk_part_def_body(self, node)
            }

            /// Visits [`PartDefBodyElement`]; the default implementation walks its children.
            fn visit_part_def_body_element(&mut self, node: &$($mutability)? Node<PartDefBodyElement>) {
                walk_part_def_body_element(self, node)
            }

            /// Visits [`ConnectionUsageMember`]; the default implementation walks its children.
            fn visit_connection_usage_member(&mut self, node: &$($mutability)? Node<ConnectionUsageMember>) {
                walk_connection_usage_member(self, node)
            }

            /// Visits [`ExhibitState`]; the default implementation walks its children.
            fn visit_exhibit_state(&mut self, node: &$($mutability)? Node<ExhibitState>) {
                walk_exhibit_state(self, node)
            }

            /// Visits [`AttributeDef`]; the default implementation walks its children.
            fn visit_attribute_def(&mut self, node: &$($mutability)? Node<AttributeDef>) {
                walk_attribute_def(self, node)
            }

            /// Visits [`AttributeBody`]; the default implementation walks its children.
            fn visit_attribute_body(&mut self, node: &$($mutability)? AttributeBody) {
                walk_attribute_body(self, node)
            }

            /// Visits [`AttributeBodyElement`]; the default implementation walks its children.
            fn visit_attribute_body_element(&mut self, node: &$($mutability)? Node<AttributeBodyElement>) {
                walk_attribute_body_element(self, node)
            }

            /// Visits [`ItemDef`]; the default implementation walks its children.
            fn visit_item_def(&mut self, node: &$($mutability)? Node<ItemDef>) {
                walk_item_def(self, node)
            }

            /// Visits [`IndividualDef`]; the default implementation walks its children.
            fn visit_individual_def(&mut self, node: &$($mutability)? Node<IndividualDef>) {
                walk_individual_def(self, node)
            }

            /// Visits [`PartUsage`]; the default implementation walks its children.
            fn visit_part_usage(&mut self, node: &$($mutability)? Node<PartUsage>) {
                walk_part_usage(self, node)
            }

            /// Visits [`PartUsageBody`]; the default implementation walks its children.
            fn visit_part_usage_body(&mut self, node: &$($mutability)? PartUsageBody) {
                walk_part_usage_body(self, node)
            }

            /// Visits [`MetadataAnnotation`]; the default implementation walks its children.
            fn visit_metadata_annotation(&mut self, node: &$($mutability)? Node<MetadataAnnotation>) {
                walk_metadata_annotation(self, node)
            }

            /// Visits [`MetadataKeywordUsage`]; the default implementation walks its children.
            fn visit_metadata_keyword_usage(&mut self, node: &$($mutability)? Node<MetadataKeywordUsage>) {
                walk_metadata_keyword_usage(self, node)
            }

            /// Visits [`MetadataDeclaredName`]; the default implementation walks its children.
            fn visit_metadata_declared_name(&mut self, node: &$($mutability)? Node<MetadataDeclaredName>) {
                walk_metadata_declared_name(self, node)
            }

            /// Visits [`PartUsageBodyElement`]; the default implementation walks its children.
            fn visit_part_usage_body_element(&mut self, node: &$($mutability)? Node<PartUsageBodyElement>) {
                walk_part_usage_body_element(self, node)
            }

            /// Visits [`VariantUsage`]; the default implementation walks its children.
            fn visit_variant_usage(&mut self, node: &$($mutability)? Node<VariantUsage>) {
                walk_variant_usage(self, node)
            }

            /// Visits [`VariantTypedUsage`]; the default implementation walks its children.
            fn visit_variant_typed_usage(&mut self, node: &$($mutability)? VariantTypedUsage) {
                walk_variant_typed_usage(self, node)
            }

            /// Visits [`Perform`]; the default implementation walks its children.
            fn visit_perform(&mut self, node: &$($mutability)? Node<Perform>) {
                walk_perform(self, node)
            }

            /// Visits [`PerformBody`]; the default implementation walks its children.
            fn visit_perform_body(&mut self, node: &$($mutability)? PerformBody) {
                walk_perform_body(self, node)
            }

            /// Visits [`PerformBodyElement`]; the default implementation walks its children.
            fn visit_perform_body_element(&mut self, node: &$($mutability)? Node<PerformBodyElement>) {
                walk_perform_body_element(self, node)
            }

            /// Visits [`PerformInOutBinding`]; the default implementation walks its children.
            fn visit_perform_in_out_binding(&mut self, node: &$($mutability)? Node<PerformInOutBinding>) {
                walk_perform_in_out_binding(self, node)
            }

            /// Visits [`AttributeUsage`]; the default implementation walks its children.
            fn visit_attribute_usage(&mut self, node: &$($mutability)? Node<AttributeUsage>) {
                walk_attribute_usage(self, node)
            }

            /// Visits [`DefaultReferenceUsage`]; the default implementation walks its children.
            fn visit_default_reference_usage(&mut self, node: &$($mutability)? Node<DefaultReferenceUsage>) {
                walk_default_reference_usage(self, node)
            }

            /// Visits [`FeatureBodyElement`]; the default implementation walks its children.
            fn visit_feature_body_element(&mut self, node: &$($mutability)? Node<FeatureBodyElement>) {
                walk_feature_body_element(self, node)
            }

            /// Visits [`PortDef`]; the default implementation walks its children.
            fn visit_port_def(&mut self, node: &$($mutability)? Node<PortDef>) {
                walk_port_def(self, node)
            }

            /// Visits [`PortDefBody`]; the default implementation walks its children.
            fn visit_port_def_body(&mut self, node: &$($mutability)? PortDefBody) {
                walk_port_def_body(self, node)
            }

            /// Visits [`PortDefBodyElement`]; the default implementation walks its children.
            fn visit_port_def_body_element(&mut self, node: &$($mutability)? Node<PortDefBodyElement>) {
                walk_port_def_body_element(self, node)
            }

            /// Visits [`PortUsage`]; the default implementation walks its children.
            fn visit_port_usage(&mut self, node: &$($mutability)? Node<PortUsage>) {
                walk_port_usage(self, node)
            }

            /// Visits [`PortBody`]; the default implementation walks its children.
            fn visit_port_body(&mut self, node: &$($mutability)? PortBody) {
                walk_port_body(self, node)
            }

            /// Visits [`PortBodyElement`]; the default implementation walks its children.
            fn visit_port_body_element(&mut self, node: &$($mutability)? Node<PortBodyElement>) {
                walk_port_body_element(self, node)
            }

            /// Visits [`ConnectStmt`]; the default implementation walks its children.
            fn visit_connect_stmt(&mut self, node: &$($mutability)? Node<ConnectStmt>) {
                walk_connect_stmt(self, node)
            }

            /// Visits [`InterfaceDef`]; the default implementation walks its children.
            fn visit_interface_def(&mut self, node: &$($mutability)? Node<InterfaceDef>) {
                walk_interface_def(self, node)
            }

            /// Visits [`InterfaceDefBody`]; the default implementation walks its children.
            fn visit_interface_def_body(&mut self, node: &$($mutability)? InterfaceDefBody) {
                walk_interface_def_body(self, node)
            }

            /// Visits [`InterfaceDefBodyElement`]; the default implementation walks its children.
            fn visit_interface_def_body_element(&mut self, node: &$($mutability)? Node<InterfaceDefBodyElement>) {
                walk_interface_def_body_element(self, node)
            }

            /// Visits [`EndNestedUsage`]; the default implementation walks its children.
            fn visit_end_nested_usage(&mut self, node: &$($mutability)? EndNestedUsage) {
                walk_end_nested_usage(self, node)
            }

            /// Visits [`EndDecl`]; the default implementation walks its children.
            fn visit_end_decl(&mut self, node: &$($mutability)? Node<EndDecl>) {
                walk_end_decl(self, node)
            }

            /// Visits [`EndIdentity`]; the default implementation walks its children.
            fn visit_end_identity(&mut self, node: &$($mutability)? EndIdentity) {
                walk_end_identity(self, node)
            }

            /// Visits [`DerivationEndRole`]; the default implementation walks its children.
            fn visit_derivation_end_role(&mut self, node: &$($mutability)? Node<DerivationEndRole>) {
                walk_derivation_end_role(self, node)
            }

            /// Visits [`RefDecl`]; the default implementation walks its children.
            fn visit_ref_decl(&mut self, node: &$($mutability)? Node<RefDecl>) {
                walk_ref_decl(self, node)
            }

            /// Visits [`RefDeclKind`]; the default implementation walks its children.
            fn visit_ref_decl_kind(&mut self, node: &$($mutability)? RefDeclKind) {
                walk_ref_decl_kind(self, node)
            }

            /// Visits [`RefBody`]; the default implementation walks its children.
            fn visit_ref_body(&mut self, node: &$($mutability)? RefBody) {
                walk_ref_body(self, node)
            }

            /// Visits [`RelationshipBodyElement`]; the default implementation walks its children.
            fn visit_relationship_body_element(&mut self, node: &$($mutability)? Node<RelationshipBodyElement>) {
                walk_relationship_body_element(self, node)
            }

            /// Visits [`DerivationConnectionRole`]; the default implementation walks its children.
            fn visit_derivation_connection_role(&mut self, node: &$($mutability)? Node<DerivationConnectionRole>) {
                walk_derivation_connection_role(self, node)
            }

            /// Visits [`ConnectionDef`]; the default implementation walks its children.
            fn visit_connection_def(&mut self, node: &$($mutability)? Node<ConnectionDef>) {
                walk_connection_def(self, node)
            }

            /// Visits [`ConnectionDefBody`]; the default implementation walks its children.
            fn visit_connection_def_body(&mut self, node: &$($mutability)? ConnectionDefBody) {
                walk_connection_def_body(self, node)
            }

            /// Visits [`ConnectionDefBodyElement`]; the default implementation walks its children.
            fn visit_connection_def_body_element(&mut self, node: &$($mutability)? Node<ConnectionDefBodyElement>) {
                walk_connection_def_body_element(self, node)
            }

            /// Visits [`MetadataDef`]; the default implementation walks its children.
            fn visit_metadata_def(&mut self, node: &$($mutability)? Node<MetadataDef>) {
                walk_metadata_def(self, node)
            }

            /// Visits [`MetadataUsage`]; the default implementation walks its children.
            fn visit_metadata_usage(&mut self, node: &$($mutability)? Node<MetadataUsage>) {
                walk_metadata_usage(self, node)
            }

            /// Visits [`EnumDef`]; the default implementation walks its children.
            fn visit_enum_def(&mut self, node: &$($mutability)? Node<EnumDef>) {
                walk_enum_def(self, node)
            }

            /// Visits [`EnumerationBody`]; the default implementation walks its children.
            fn visit_enumeration_body(&mut self, node: &$($mutability)? EnumerationBody) {
                walk_enumeration_body(self, node)
            }

            /// Visits [`EnumeratedValue`]; the default implementation walks its children.
            fn visit_enumerated_value(&mut self, node: &$($mutability)? Node<EnumeratedValue>) {
                walk_enumerated_value(self, node)
            }

            /// Visits [`EnumerationBodyElement`]; the default implementation walks its children.
            fn visit_enumeration_body_element(&mut self, node: &$($mutability)? Node<EnumerationBodyElement>) {
                walk_enumeration_body_element(self, node)
            }

            /// Visits [`OccurrenceDef`]; the default implementation walks its children.
            fn visit_occurrence_def(&mut self, node: &$($mutability)? Node<OccurrenceDef>) {
                walk_occurrence_def(self, node)
            }

            /// Visits [`OccurrenceUsage`]; the default implementation walks its children.
            fn visit_occurrence_usage(&mut self, node: &$($mutability)? Node<OccurrenceUsage>) {
                walk_occurrence_usage(self, node)
            }

            /// Visits [`OccurrencePortionKind`]; the default implementation walks its children.
            fn visit_occurrence_portion_kind(&mut self, node: &$($mutability)? Node<OccurrencePortionKind>) {
                walk_occurrence_portion_kind(self, node)
            }

            /// Visits an [`OccurrencePortionKind`] stored without a node wrapper.
            fn visit_occurrence_portion_kind_value(&mut self, value: &$($mutability)? OccurrencePortionKind) {
                walk_occurrence_portion_kind_value(self, value)
            }

            /// Visits a [`MultiplicityOrdering`] keyword slot.
            fn visit_multiplicity_ordering(&mut self, node: &$($mutability)? Node<MultiplicityOrdering>) {
                walk_multiplicity_ordering(self, node)
            }

            /// Visits a [`MultiplicityOrdering`] stored without a node wrapper.
            fn visit_multiplicity_ordering_value(&mut self, value: &$($mutability)? MultiplicityOrdering) {
                walk_multiplicity_ordering_value(self, value)
            }

            /// Visits a [`MultiplicityUniqueness`] keyword slot.
            fn visit_multiplicity_uniqueness(&mut self, node: &$($mutability)? Node<MultiplicityUniqueness>) {
                walk_multiplicity_uniqueness(self, node)
            }

            /// Visits a [`MultiplicityUniqueness`] stored without a node wrapper.
            fn visit_multiplicity_uniqueness_value(&mut self, value: &$($mutability)? MultiplicityUniqueness) {
                walk_multiplicity_uniqueness_value(self, value)
            }

            /// Visits [`MultiplicityModifiers`]; the default implementation walks its children.
            fn visit_multiplicity_modifiers(&mut self, node: &$($mutability)? MultiplicityModifiers) {
                walk_multiplicity_modifiers(self, node)
            }

            /// Visits [`RefPrefix`]; the default implementation walks its children.
            fn visit_ref_prefix(&mut self, node: &$($mutability)? RefPrefix) {
                walk_ref_prefix(self, node)
            }

            /// Visits [`BasicUsagePrefix`]; the default implementation walks its children.
            fn visit_basic_usage_prefix(&mut self, node: &$($mutability)? BasicUsagePrefix) {
                walk_basic_usage_prefix(self, node)
            }

            /// Visits [`UsageExtensionKeyword`]; the default implementation walks its children.
            fn visit_usage_extension_keyword(&mut self, node: &$($mutability)? Node<UsageExtensionKeyword>) {
                walk_usage_extension_keyword(self, node)
            }

            /// Visits [`OccurrenceUsagePrefix`]; the default implementation walks its children.
            fn visit_occurrence_usage_prefix(&mut self, node: &$($mutability)? OccurrenceUsagePrefix) {
                walk_occurrence_usage_prefix(self, node)
            }

            /// Visits [`FeaturePortionKind`]; the default implementation walks its children.
            fn visit_feature_portion_kind(&mut self, node: &$($mutability)? Node<FeaturePortionKind>) {
                walk_feature_portion_kind(self, node)
            }

            /// Visits a [`FeaturePortionKind`] stored without a node wrapper.
            fn visit_feature_portion_kind_value(&mut self, value: &$($mutability)? FeaturePortionKind) {
                walk_feature_portion_kind_value(self, value)
            }

            /// Visits [`FeatureVariability`]; the default implementation walks its children.
            fn visit_feature_variability(&mut self, node: &$($mutability)? Node<FeatureVariability>) {
                walk_feature_variability(self, node)
            }

            /// Visits a [`FeatureVariability`] stored without a node wrapper.
            fn visit_feature_variability_value(&mut self, value: &$($mutability)? FeatureVariability) {
                walk_feature_variability_value(self, value)
            }

            /// Visits [`BasicFeaturePrefix`]; the default implementation walks its children.
            fn visit_basic_feature_prefix(&mut self, node: &$($mutability)? BasicFeaturePrefix) {
                walk_basic_feature_prefix(self, node)
            }

            /// Visits [`EndFeaturePrefix`]; the default implementation walks its children.
            fn visit_end_feature_prefix(&mut self, node: &$($mutability)? EndFeaturePrefix) {
                walk_end_feature_prefix(self, node)
            }

            /// Visits [`OwnedCrossFeature`]; the default implementation walks its children.
            fn visit_owned_cross_feature(&mut self, node: &$($mutability)? Node<OwnedCrossFeature>) {
                walk_owned_cross_feature(self, node)
            }

            /// Visits [`FeaturePrefixHead`]; the default implementation walks its children.
            fn visit_feature_prefix_head(&mut self, node: &$($mutability)? FeaturePrefixHead) {
                walk_feature_prefix_head(self, node)
            }

            /// Visits [`FeaturePrefix`]; the default implementation walks its children.
            fn visit_feature_prefix(&mut self, node: &$($mutability)? FeaturePrefix) {
                walk_feature_prefix(self, node)
            }

            /// Visits [`OccurrenceUsageBody`]; the default implementation walks its children.
            fn visit_occurrence_usage_body(&mut self, node: &$($mutability)? OccurrenceUsageBody) {
                walk_occurrence_usage_body(self, node)
            }

            /// Visits [`AssertConstraintMember`]; the default implementation walks its children.
            fn visit_assert_constraint_member(&mut self, node: &$($mutability)? Node<AssertConstraintMember>) {
                walk_assert_constraint_member(self, node)
            }

            /// Visits [`OccurrenceBodyElement`]; the default implementation walks its children.
            fn visit_occurrence_body_element(&mut self, node: &$($mutability)? Node<OccurrenceBodyElement>) {
                walk_occurrence_body_element(self, node)
            }

            /// Visits [`SuccessionUsage`]; the default implementation walks its children.
            fn visit_succession_usage(&mut self, node: &$($mutability)? Node<SuccessionUsage>) {
                walk_succession_usage(self, node)
            }

            /// Visits [`DefinitionBody`]; the default implementation walks its children.
            fn visit_definition_body(&mut self, node: &$($mutability)? DefinitionBody) {
                walk_definition_body(self, node)
            }

            /// Visits [`DefinitionBodyElement`]; the default implementation walks its children.
            fn visit_definition_body_element(&mut self, node: &$($mutability)? Node<DefinitionBodyElement>) {
                walk_definition_body_element(self, node)
            }

            /// Visits [`Bind`]; the default implementation walks its children.
            fn visit_bind(&mut self, node: &$($mutability)? Node<Bind>) {
                walk_bind(self, node)
            }

            /// Visits [`InterfaceUsage`]; the default implementation walks its children.
            fn visit_interface_usage(&mut self, node: &$($mutability)? Node<InterfaceUsage>) {
                walk_interface_usage(self, node)
            }

            /// Visits [`InterfaceUsageBodyElement`]; the default implementation walks its children.
            fn visit_interface_usage_body_element(&mut self, node: &$($mutability)? Node<InterfaceUsageBodyElement>) {
                walk_interface_usage_body_element(self, node)
            }

            /// Visits [`Connect`]; the default implementation walks its children.
            fn visit_connect(&mut self, node: &$($mutability)? Node<Connect>) {
                walk_connect(self, node)
            }

            /// Visits [`BindingConnectorUsage`]; the default implementation walks its children.
            fn visit_binding_connector_usage(&mut self, node: &$($mutability)? Node<BindingConnectorUsage>) {
                walk_binding_connector_usage(self, node)
            }

            /// Visits [`AliasDef`]; the default implementation walks its children.
            fn visit_alias_def(&mut self, node: &$($mutability)? Node<AliasDef>) {
                walk_alias_def(self, node)
            }

            /// Visits [`AliasBody`]; the default implementation walks its children.
            fn visit_alias_body(&mut self, node: &$($mutability)? AliasBody) {
                walk_alias_body(self, node)
            }

            /// Visits [`ActionDef`]; the default implementation walks its children.
            fn visit_action_def(&mut self, node: &$($mutability)? Node<ActionDef>) {
                walk_action_def(self, node)
            }

            /// Visits [`ActionBranchBody`]; the default implementation walks its children.
            fn visit_action_branch_body(&mut self, node: &$($mutability)? ActionBranchBody) {
                walk_action_branch_body(self, node)
            }

            /// Visits [`ActionDefBody`]; the default implementation walks its children.
            fn visit_action_def_body(&mut self, node: &$($mutability)? ActionDefBody) {
                walk_action_def_body(self, node)
            }

            /// Visits [`ActionDefBodyElement`]; the default implementation walks its children.
            fn visit_action_def_body_element(&mut self, node: &$($mutability)? Node<ActionDefBodyElement>) {
                walk_action_def_body_element(self, node)
            }

            /// Visits [`AssignStmt`]; the default implementation walks its children.
            fn visit_assign_stmt(&mut self, node: &$($mutability)? Node<AssignStmt>) {
                walk_assign_stmt(self, node)
            }

            /// Visits [`ForLoop`]; the default implementation walks its children.
            fn visit_for_loop(&mut self, node: &$($mutability)? Node<ForLoop>) {
                walk_for_loop(self, node)
            }

            /// Visits [`ThenAction`]; the default implementation walks its children.
            fn visit_then_action(&mut self, node: &$($mutability)? Node<ThenAction>) {
                walk_then_action(self, node)
            }

            /// Visits [`ThenTarget`]; the default implementation walks its children.
            fn visit_then_target(&mut self, node: &$($mutability)? ThenTarget) {
                walk_then_target(self, node)
            }

            /// Visits [`InOutDecl`]; the default implementation walks its children.
            fn visit_in_out_decl(&mut self, node: &$($mutability)? Node<InOutDecl>) {
                walk_in_out_decl(self, node)
            }

            /// Visits [`InOut`]; the default implementation walks its children.
            fn visit_in_out(&mut self, node: &$($mutability)? Node<InOut>) {
                walk_in_out(self, node)
            }

            /// Visits a [`InOut`] that its parent stores without a node wrapper.
            fn visit_in_out_value(&mut self, value: &$($mutability)? InOut) {
                walk_in_out_value(self, value)
            }

            /// Visits [`PayloadClause`]; the default implementation walks its children.
            fn visit_payload_clause(&mut self, node: &$($mutability)? PayloadClause) {
                walk_payload_clause(self, node)
            }

            /// Visits [`SendPayload`]; the default implementation walks its children.
            fn visit_send_payload(&mut self, node: &$($mutability)? SendPayload) {
                walk_send_payload(self, node)
            }

            /// Visits [`TransitionAccept`]; the default implementation walks its children.
            fn visit_transition_accept(&mut self, node: &$($mutability)? Node<TransitionAccept>) {
                walk_transition_accept(self, node)
            }

            /// Visits a [`TransitionAccept`] that its parent stores without a node wrapper.
            fn visit_transition_accept_value(&mut self, value: &$($mutability)? TransitionAccept) {
                walk_transition_accept_value(self, value)
            }

            /// Visits [`TriggerKind`]; the default implementation walks its children.
            fn visit_trigger_kind(&mut self, node: &$($mutability)? TriggerKind) {
                walk_trigger_kind(self, node)
            }

            /// Visits [`TransitionEffect`]; the default implementation walks its children.
            fn visit_transition_effect(&mut self, node: &$($mutability)? TransitionEffect) {
                walk_transition_effect(self, node)
            }

            /// Visits [`ActionUsage`]; the default implementation walks its children.
            fn visit_action_usage(&mut self, node: &$($mutability)? Node<ActionUsage>) {
                walk_action_usage(self, node)
            }

            /// Visits [`ActionUsageBody`]; the default implementation walks its children.
            fn visit_action_usage_body(&mut self, node: &$($mutability)? ActionUsageBody) {
                walk_action_usage_body(self, node)
            }

            /// Visits [`ActionUsageBodyElement`]; the default implementation walks its children.
            fn visit_action_usage_body_element(&mut self, node: &$($mutability)? Node<ActionUsageBodyElement>) {
                walk_action_usage_body_element(self, node)
            }

            /// Visits [`FlowDef`]; the default implementation walks its children.
            fn visit_flow_def(&mut self, node: &$($mutability)? Node<FlowDef>) {
                walk_flow_def(self, node)
            }

            /// Visits [`FlowUsageKind`]; the default implementation walks its children.
            fn visit_flow_usage_kind(&mut self, node: &$($mutability)? FlowUsageKind) {
                walk_flow_usage_kind(self, node)
            }

            /// Visits [`PayloadFeature`]; the default implementation walks its children.
            fn visit_payload_feature(&mut self, node: &$($mutability)? Node<PayloadFeature>) {
                walk_payload_feature(self, node)
            }

            /// Visits [`FlowUsage`]; the default implementation walks its children.
            fn visit_flow_usage(&mut self, node: &$($mutability)? Node<FlowUsage>) {
                walk_flow_usage(self, node)
            }

            /// Visits [`FirstStmt`]; the default implementation walks its children.
            fn visit_first_stmt(&mut self, node: &$($mutability)? Node<FirstStmt>) {
                walk_first_stmt(self, node)
            }

            /// Visits [`MergeStmt`]; the default implementation walks its children.
            fn visit_merge_stmt(&mut self, node: &$($mutability)? Node<MergeStmt>) {
                walk_merge_stmt(self, node)
            }

            /// Visits [`DecisionStmt`]; the default implementation walks its children.
            fn visit_decision_stmt(&mut self, node: &$($mutability)? Node<DecisionStmt>) {
                walk_decision_stmt(self, node)
            }

            /// Visits [`JoinStmt`]; the default implementation walks its children.
            fn visit_join_stmt(&mut self, node: &$($mutability)? Node<JoinStmt>) {
                walk_join_stmt(self, node)
            }

            /// Visits [`ForkStmt`]; the default implementation walks its children.
            fn visit_fork_stmt(&mut self, node: &$($mutability)? Node<ForkStmt>) {
                walk_fork_stmt(self, node)
            }

            /// Visits [`FirstMergeBody`]; the default implementation walks its children.
            fn visit_first_merge_body(&mut self, node: &$($mutability)? FirstMergeBody) {
                walk_first_merge_body(self, node)
            }

            /// Visits [`FirstMergeBraceBody`]; the default implementation walks its children.
            fn visit_first_merge_brace_body(&mut self, node: &$($mutability)? Node<FirstMergeBraceBody>) {
                walk_first_merge_brace_body(self, node)
            }

            /// Visits [`FirstMergeBodyElement`]; the default implementation walks its children.
            fn visit_first_merge_body_element(&mut self, node: &$($mutability)? Node<FirstMergeBodyElement>) {
                walk_first_merge_body_element(self, node)
            }

            /// Visits [`TerminateStmt`]; the default implementation walks its children.
            fn visit_terminate_stmt(&mut self, node: &$($mutability)? Node<TerminateStmt>) {
                walk_terminate_stmt(self, node)
            }

            /// Visits [`WhileStmt`]; the default implementation walks its children.
            fn visit_while_stmt(&mut self, node: &$($mutability)? Node<WhileStmt>) {
                walk_while_stmt(self, node)
            }

            /// Visits [`LoopStmt`]; the default implementation walks its children.
            fn visit_loop_stmt(&mut self, node: &$($mutability)? Node<LoopStmt>) {
                walk_loop_stmt(self, node)
            }

            /// Visits [`IfStmt`]; the default implementation walks its children.
            fn visit_if_stmt(&mut self, node: &$($mutability)? Node<IfStmt>) {
                walk_if_stmt(self, node)
            }

            /// Visits [`Allocate`]; the default implementation walks its children.
            fn visit_allocate(&mut self, node: &$($mutability)? Node<Allocate>) {
                walk_allocate(self, node)
            }

            /// Visits [`AllocationDef`]; the default implementation walks its children.
            fn visit_allocation_def(&mut self, node: &$($mutability)? Node<AllocationDef>) {
                walk_allocation_def(self, node)
            }

            /// Visits [`AllocationUsage`]; the default implementation walks its children.
            fn visit_allocation_usage(&mut self, node: &$($mutability)? Node<AllocationUsage>) {
                walk_allocation_usage(self, node)
            }

            /// Visits [`StateDef`]; the default implementation walks its children.
            fn visit_state_def(&mut self, node: &$($mutability)? Node<StateDef>) {
                walk_state_def(self, node)
            }

            /// Visits [`StateDefBody`]; the default implementation walks its children.
            fn visit_state_def_body(&mut self, node: &$($mutability)? StateDefBody) {
                walk_state_def_body(self, node)
            }

            /// Visits [`StateDefBodyElement`]; the default implementation walks its children.
            fn visit_state_def_body_element(&mut self, node: &$($mutability)? Node<StateDefBodyElement>) {
                walk_state_def_body_element(self, node)
            }

            /// Visits [`EntryAction`]; the default implementation walks its children.
            fn visit_entry_action(&mut self, node: &$($mutability)? Node<EntryAction>) {
                walk_entry_action(self, node)
            }

            /// Visits [`DoAction`]; the default implementation walks its children.
            fn visit_do_action(&mut self, node: &$($mutability)? Node<DoAction>) {
                walk_do_action(self, node)
            }

            /// Visits [`ExitAction`]; the default implementation walks its children.
            fn visit_exit_action(&mut self, node: &$($mutability)? Node<ExitAction>) {
                walk_exit_action(self, node)
            }

            /// Visits [`ThenStmt`]; the default implementation walks its children.
            fn visit_then_stmt(&mut self, node: &$($mutability)? Node<ThenStmt>) {
                walk_then_stmt(self, node)
            }

            /// Visits [`FinalState`]; the default implementation walks its children.
            fn visit_final_state(&mut self, node: &$($mutability)? Node<FinalState>) {
                walk_final_state(self, node)
            }

            /// Visits [`StateUsage`]; the default implementation walks its children.
            fn visit_state_usage(&mut self, node: &$($mutability)? Node<StateUsage>) {
                walk_state_usage(self, node)
            }

            /// Visits [`Transition`]; the default implementation walks its children.
            fn visit_transition(&mut self, node: &$($mutability)? Node<Transition>) {
                walk_transition(self, node)
            }

            /// Visits [`RequirementDef`]; the default implementation walks its children.
            fn visit_requirement_def(&mut self, node: &$($mutability)? Node<RequirementDef>) {
                walk_requirement_def(self, node)
            }

            /// Visits [`RequirementDefBody`]; the default implementation walks its children.
            fn visit_requirement_def_body(&mut self, node: &$($mutability)? RequirementDefBody) {
                walk_requirement_def_body(self, node)
            }

            /// Visits [`RequirementDefBodyElement`]; the default implementation walks its children.
            fn visit_requirement_def_body_element(&mut self, node: &$($mutability)? Node<RequirementDefBodyElement>) {
                walk_requirement_def_body_element(self, node)
            }

            /// Visits [`StakeholderMember`]; the default implementation walks its children.
            fn visit_stakeholder_member(&mut self, node: &$($mutability)? Node<StakeholderMember>) {
                walk_stakeholder_member(self, node)
            }

            /// Visits [`PurposeMember`]; the default implementation walks its children.
            fn visit_purpose_member(&mut self, node: &$($mutability)? Node<PurposeMember>) {
                walk_purpose_member(self, node)
            }

            /// Visits [`SubjectDecl`]; the default implementation walks its children.
            fn visit_subject_decl(&mut self, node: &$($mutability)? Node<SubjectDecl>) {
                walk_subject_decl(self, node)
            }

            /// Visits [`RequirementActorDecl`]; the default implementation walks its children.
            fn visit_requirement_actor_decl(&mut self, node: &$($mutability)? Node<RequirementActorDecl>) {
                walk_requirement_actor_decl(self, node)
            }

            /// Visits [`RequireConstraint`]; the default implementation walks its children.
            fn visit_require_constraint(&mut self, node: &$($mutability)? Node<RequireConstraint>) {
                walk_require_constraint(self, node)
            }

            /// Visits [`VerifyRequirementMember`]; the default implementation walks its children.
            fn visit_verify_requirement_member(&mut self, node: &$($mutability)? Node<VerifyRequirementMember>) {
                walk_verify_requirement_member(self, node)
            }

            /// Visits [`SatisfyRequirementUsage`]; the default implementation walks its children.
            fn visit_satisfy_requirement_usage(&mut self, node: &$($mutability)? Node<SatisfyRequirementUsage>) {
                walk_satisfy_requirement_usage(self, node)
            }

            /// Visits [`SatisfiedRequirement`]; the default implementation walks its children.
            fn visit_satisfied_requirement(&mut self, node: &$($mutability)? SatisfiedRequirement) {
                walk_satisfied_requirement(self, node)
            }

            /// Visits [`InlineRequirementDeclaration`]; the default implementation walks its children.
            fn visit_inline_requirement_declaration(&mut self, node: &$($mutability)? Node<InlineRequirementDeclaration>) {
                walk_inline_requirement_declaration(self, node)
            }

            /// Visits [`SatisfactionSubject`]; the default implementation walks its children.
            fn visit_satisfaction_subject(&mut self, node: &$($mutability)? Node<SatisfactionSubject>) {
                walk_satisfaction_subject(self, node)
            }

            /// Visits [`RequirementUsage`]; the default implementation walks its children.
            fn visit_requirement_usage(&mut self, node: &$($mutability)? Node<RequirementUsage>) {
                walk_requirement_usage(self, node)
            }

            /// Visits [`ItemUsage`]; the default implementation walks its children.
            fn visit_item_usage(&mut self, node: &$($mutability)? Node<ItemUsage>) {
                walk_item_usage(self, node)
            }

            /// Visits [`EnumerationUsage`]; the default implementation walks its children.
            fn visit_enumeration_usage(&mut self, node: &$($mutability)? Node<EnumerationUsage>) {
                walk_enumeration_usage(self, node)
            }

            /// Visits [`Dependency`]; the default implementation walks its children.
            fn visit_dependency(&mut self, node: &$($mutability)? Node<Dependency>) {
                walk_dependency(self, node)
            }

            /// Visits [`FrameMember`]; the default implementation walks its children.
            fn visit_frame_member(&mut self, node: &$($mutability)? Node<FrameMember>) {
                walk_frame_member(self, node)
            }

            /// Visits [`ConcernUsage`]; the default implementation walks its children.
            fn visit_concern_usage(&mut self, node: &$($mutability)? Node<ConcernUsage>) {
                walk_concern_usage(self, node)
            }

            /// Visits [`CaseDef`]; the default implementation walks its children.
            fn visit_case_def(&mut self, node: &$($mutability)? Node<CaseDef>) {
                walk_case_def(self, node)
            }

            /// Visits [`CaseUsage`]; the default implementation walks its children.
            fn visit_case_usage(&mut self, node: &$($mutability)? Node<CaseUsage>) {
                walk_case_usage(self, node)
            }

            /// Visits [`AnalysisCaseDef`]; the default implementation walks its children.
            fn visit_analysis_case_def(&mut self, node: &$($mutability)? Node<AnalysisCaseDef>) {
                walk_analysis_case_def(self, node)
            }

            /// Visits [`AnalysisCaseUsage`]; the default implementation walks its children.
            fn visit_analysis_case_usage(&mut self, node: &$($mutability)? Node<AnalysisCaseUsage>) {
                walk_analysis_case_usage(self, node)
            }

            /// Visits [`VerificationCaseDef`]; the default implementation walks its children.
            fn visit_verification_case_def(&mut self, node: &$($mutability)? Node<VerificationCaseDef>) {
                walk_verification_case_def(self, node)
            }

            /// Visits [`VerificationCaseUsage`]; the default implementation walks its children.
            fn visit_verification_case_usage(&mut self, node: &$($mutability)? Node<VerificationCaseUsage>) {
                walk_verification_case_usage(self, node)
            }

            /// Visits [`UseCaseUsage`]; the default implementation walks its children.
            fn visit_use_case_usage(&mut self, node: &$($mutability)? Node<UseCaseUsage>) {
                walk_use_case_usage(self, node)
            }

            /// Visits [`ActorDecl`]; the default implementation walks its children.
            fn visit_actor_decl(&mut self, node: &$($mutability)? Node<ActorDecl>) {
                walk_actor_decl(self, node)
            }

            /// Visits [`UseCaseDef`]; the default implementation walks its children.
            fn visit_use_case_def(&mut self, node: &$($mutability)? Node<UseCaseDef>) {
                walk_use_case_def(self, node)
            }

            /// Visits [`UseCaseDefBody`]; the default implementation walks its children.
            fn visit_use_case_def_body(&mut self, node: &$($mutability)? Node<UseCaseDefBody>) {
                walk_use_case_def_body(self, node)
            }

            /// Visits a [`UseCaseDefBody`] that its parent stores without a node wrapper.
            fn visit_use_case_def_body_value(&mut self, value: &$($mutability)? UseCaseDefBody) {
                walk_use_case_def_body_value(self, value)
            }

            /// Visits [`FirstSuccession`]; the default implementation walks its children.
            fn visit_first_succession(&mut self, node: &$($mutability)? Node<FirstSuccession>) {
                walk_first_succession(self, node)
            }

            /// Visits [`ThenDone`]; the default implementation walks its children.
            fn visit_then_done(&mut self, node: &$($mutability)? Node<ThenDone>) {
                walk_then_done(self, node)
            }

            /// Visits [`IncludeUseCase`]; the default implementation walks its children.
            fn visit_include_use_case(&mut self, node: &$($mutability)? Node<IncludeUseCase>) {
                walk_include_use_case(self, node)
            }

            /// Visits [`ThenIncludeUseCase`]; the default implementation walks its children.
            fn visit_then_include_use_case(&mut self, node: &$($mutability)? Node<ThenIncludeUseCase>) {
                walk_then_include_use_case(self, node)
            }

            /// Visits [`ThenUseCaseUsage`]; the default implementation walks its children.
            fn visit_then_use_case_usage(&mut self, node: &$($mutability)? Node<ThenUseCaseUsage>) {
                walk_then_use_case_usage(self, node)
            }

            /// Visits [`SubjectRef`]; the default implementation walks its children.
            fn visit_subject_ref(&mut self, node: &$($mutability)? Node<SubjectRef>) {
                walk_subject_ref(self, node)
            }

            /// Visits [`ActorRedefinitionAssignment`]; the default implementation walks its children.
            fn visit_actor_redefinition_assignment(&mut self, node: &$($mutability)? Node<ActorRedefinitionAssignment>) {
                walk_actor_redefinition_assignment(self, node)
            }

            /// Visits [`RefRedefinition`]; the default implementation walks its children.
            fn visit_ref_redefinition(&mut self, node: &$($mutability)? Node<RefRedefinition>) {
                walk_ref_redefinition(self, node)
            }

            /// Visits [`CaseReturnFeatureKind`]; the default implementation walks its children.
            fn visit_case_return_feature_kind(&mut self, node: &$($mutability)? CaseReturnFeatureKind) {
                walk_case_return_feature_kind(self, node)
            }

            /// Visits [`CaseReturnDecl`]; the default implementation walks its children.
            fn visit_case_return_decl(&mut self, node: &$($mutability)? Node<CaseReturnDecl>) {
                walk_case_return_decl(self, node)
            }

            /// Visits [`ReturnRef`]; the default implementation walks its children.
            fn visit_return_ref(&mut self, node: &$($mutability)? Node<ReturnRef>) {
                walk_return_ref(self, node)
            }

            /// Visits [`ReturnRefBody`]; the default implementation walks its children.
            fn visit_return_ref_body(&mut self, node: &$($mutability)? Node<ReturnRefBody>) {
                walk_return_ref_body(self, node)
            }

            /// Visits [`ReturnRefBodyElement`]; the default implementation walks its children.
            fn visit_return_ref_body_element(&mut self, node: &$($mutability)? Node<ReturnRefBodyElement>) {
                walk_return_ref_body_element(self, node)
            }

            /// Visits [`UseCaseDefBodyElement`]; the default implementation walks its children.
            fn visit_use_case_def_body_element(&mut self, node: &$($mutability)? Node<UseCaseDefBodyElement>) {
                walk_use_case_def_body_element(self, node)
            }

            /// Visits [`ActorUsage`]; the default implementation walks its children.
            fn visit_actor_usage(&mut self, node: &$($mutability)? Node<ActorUsage>) {
                walk_actor_usage(self, node)
            }

            /// Visits [`Objective`]; the default implementation walks its children.
            fn visit_objective(&mut self, node: &$($mutability)? Node<Objective>) {
                walk_objective(self, node)
            }

            /// Visits [`ConstraintDef`]; the default implementation walks its children.
            fn visit_constraint_def(&mut self, node: &$($mutability)? Node<ConstraintDef>) {
                walk_constraint_def(self, node)
            }

            /// Visits [`ConstraintUsage`]; the default implementation walks its children.
            fn visit_constraint_usage(&mut self, node: &$($mutability)? Node<ConstraintUsage>) {
                walk_constraint_usage(self, node)
            }

            /// Visits [`ConstraintDefBody`]; the default implementation walks its children.
            fn visit_constraint_def_body(&mut self, node: &$($mutability)? ConstraintDefBody) {
                walk_constraint_def_body(self, node)
            }

            /// Visits [`ConstraintDefBodyElement`]; the default implementation walks its children.
            fn visit_constraint_def_body_element(&mut self, node: &$($mutability)? Node<ConstraintDefBodyElement>) {
                walk_constraint_def_body_element(self, node)
            }

            /// Visits [`CalcDef`]; the default implementation walks its children.
            fn visit_calc_def(&mut self, node: &$($mutability)? Node<CalcDef>) {
                walk_calc_def(self, node)
            }

            /// Visits [`CalcUsage`]; the default implementation walks its children.
            fn visit_calc_usage(&mut self, node: &$($mutability)? Node<CalcUsage>) {
                walk_calc_usage(self, node)
            }

            /// Visits [`CalcDefBody`]; the default implementation walks its children.
            fn visit_calc_def_body(&mut self, node: &$($mutability)? CalcDefBody) {
                walk_calc_def_body(self, node)
            }

            /// Visits [`CalcDefBodyElement`]; the default implementation walks its children.
            fn visit_calc_def_body_element(&mut self, node: &$($mutability)? Node<CalcDefBodyElement>) {
                walk_calc_def_body_element(self, node)
            }

            /// Visits [`ReturnDecl`]; the default implementation walks its children.
            fn visit_return_decl(&mut self, node: &$($mutability)? Node<ReturnDecl>) {
                walk_return_decl(self, node)
            }

            /// Visits [`ReturnKindKeyword`]; the default implementation walks its children.
            fn visit_return_kind_keyword(&mut self, node: &$($mutability)? ReturnKindKeyword) {
                walk_return_kind_keyword(self, node)
            }

            /// Visits [`ViewDef`]; the default implementation walks its children.
            fn visit_view_def(&mut self, node: &$($mutability)? Node<ViewDef>) {
                walk_view_def(self, node)
            }

            /// Visits [`ViewDefBody`]; the default implementation walks its children.
            fn visit_view_def_body(&mut self, node: &$($mutability)? ViewDefBody) {
                walk_view_def_body(self, node)
            }

            /// Visits [`ViewDefBodyElement`]; the default implementation walks its children.
            fn visit_view_def_body_element(&mut self, node: &$($mutability)? Node<ViewDefBodyElement>) {
                walk_view_def_body_element(self, node)
            }

            /// Visits [`ViewRenderingUsage`]; the default implementation walks its children.
            fn visit_view_rendering_usage(&mut self, node: &$($mutability)? Node<ViewRenderingUsage>) {
                walk_view_rendering_usage(self, node)
            }

            /// Visits [`RenderingUsageBody`]; the default implementation walks its children.
            fn visit_rendering_usage_body(&mut self, node: &$($mutability)? RenderingUsageBody) {
                walk_rendering_usage_body(self, node)
            }

            /// Visits [`RenderingUsageBodyElement`]; the default implementation walks its children.
            fn visit_rendering_usage_body_element(&mut self, node: &$($mutability)? Node<RenderingUsageBodyElement>) {
                walk_rendering_usage_body_element(self, node)
            }

            /// Visits [`ViewpointDef`]; the default implementation walks its children.
            fn visit_viewpoint_def(&mut self, node: &$($mutability)? Node<ViewpointDef>) {
                walk_viewpoint_def(self, node)
            }

            /// Visits [`RenderingDef`]; the default implementation walks its children.
            fn visit_rendering_def(&mut self, node: &$($mutability)? Node<RenderingDef>) {
                walk_rendering_def(self, node)
            }

            /// Visits [`RenderingDefBody`]; the default implementation walks its children.
            fn visit_rendering_def_body(&mut self, node: &$($mutability)? RenderingDefBody) {
                walk_rendering_def_body(self, node)
            }

            /// Visits [`RenderingDefBodyElement`]; the default implementation walks its children.
            fn visit_rendering_def_body_element(&mut self, node: &$($mutability)? Node<RenderingDefBodyElement>) {
                walk_rendering_def_body_element(self, node)
            }

            /// Visits [`ViewUsage`]; the default implementation walks its children.
            fn visit_view_usage(&mut self, node: &$($mutability)? Node<ViewUsage>) {
                walk_view_usage(self, node)
            }

            /// Visits [`ViewBody`]; the default implementation walks its children.
            fn visit_view_body(&mut self, node: &$($mutability)? ViewBody) {
                walk_view_body(self, node)
            }

            /// Visits [`ViewBodyElement`]; the default implementation walks its children.
            fn visit_view_body_element(&mut self, node: &$($mutability)? Node<ViewBodyElement>) {
                walk_view_body_element(self, node)
            }

            /// Visits [`ExposeMember`]; the default implementation walks its children.
            fn visit_expose_member(&mut self, node: &$($mutability)? Node<ExposeMember>) {
                walk_expose_member(self, node)
            }

            /// Visits [`ViewpointUsage`]; the default implementation walks its children.
            fn visit_viewpoint_usage(&mut self, node: &$($mutability)? Node<ViewpointUsage>) {
                walk_viewpoint_usage(self, node)
            }

            /// Visits [`RenderingUsage`]; the default implementation walks its children.
            fn visit_rendering_usage(&mut self, node: &$($mutability)? Node<RenderingUsage>) {
                walk_rendering_usage(self, node)
            }

            /// Visits [`KermlBareDeclaration`]; the default implementation walks its children.
            fn visit_kerml_bare_declaration(&mut self, node: &$($mutability)? Node<KermlBareDeclaration>) {
                walk_kerml_bare_declaration(self, node)
            }

            /// Visits [`KermlBareDeclarationKeyword`]; the default implementation walks its children.
            fn visit_kerml_bare_declaration_keyword(&mut self, node: &$($mutability)? KermlBareDeclarationKeyword) {
                walk_kerml_bare_declaration_keyword(self, node)
            }

            /// Visits [`KermlSemanticDecl`]; the default implementation walks its children.
            fn visit_kerml_semantic_decl(&mut self, node: &$($mutability)? Node<KermlSemanticDecl>) {
                walk_kerml_semantic_decl(self, node)
            }

            /// Visits [`KermlFeatureDecl`]; the default implementation walks its children.
            fn visit_kerml_feature_decl(&mut self, node: &$($mutability)? Node<KermlFeatureDecl>) {
                walk_kerml_feature_decl(self, node)
            }

            /// Visits [`FeatureDecl`]; the default implementation walks its children.
            fn visit_feature_decl(&mut self, node: &$($mutability)? Node<FeatureDecl>) {
                walk_feature_decl(self, node)
            }

            /// Visits [`ClassifierDecl`]; the default implementation walks its children.
            fn visit_classifier_decl(&mut self, node: &$($mutability)? Node<ClassifierDecl>) {
                walk_classifier_decl(self, node)
            }

            /// Visits [`ExtendedLibraryDecl`]; the default implementation walks its children.
            fn visit_extended_library_decl(&mut self, node: &$($mutability)? Node<ExtendedLibraryDecl>) {
                walk_extended_library_decl(self, node)
            }

            /// Visits [`KermlClassifierDecl`]; the default implementation walks its children.
            fn visit_kerml_classifier_decl(&mut self, node: &$($mutability)? Node<KermlClassifierDecl>) {
                walk_kerml_classifier_decl(self, node)
            }

            /// Visits [`KermlClassifierKeyword`]; the default implementation walks its children.
            fn visit_kerml_classifier_keyword(&mut self, node: &$($mutability)? KermlClassifierKeyword) {
                walk_kerml_classifier_keyword(self, node)
            }

            /// Visits [`KermlTypeRelationship`]; the default implementation walks its children.
            fn visit_kerml_type_relationship(&mut self, node: &$($mutability)? Node<KermlTypeRelationship>) {
                walk_kerml_type_relationship(self, node)
            }

            /// Visits [`KermlTypeRelationshipKeyword`]; the default implementation walks its children.
            fn visit_kerml_type_relationship_keyword(&mut self, node: &$($mutability)? KermlTypeRelationshipKeyword) {
                walk_kerml_type_relationship_keyword(self, node)
            }

            /// Visits [`KermlFeatureMember`]; the default implementation walks its children.
            fn visit_kerml_feature_member(&mut self, node: &$($mutability)? Node<KermlFeatureMember>) {
                walk_kerml_feature_member(self, node)
            }

            /// Visits [`KermlFeatureKind`]; the default implementation walks its children.
            fn visit_kerml_feature_kind(&mut self, node: &$($mutability)? Node<KermlFeatureKind>) {
                walk_kerml_feature_kind(self, node)
            }

            /// Visits a [`KermlFeatureKind`] stored without a node wrapper.
            fn visit_kerml_feature_kind_value(&mut self, value: &$($mutability)? KermlFeatureKind) {
                walk_kerml_feature_kind_value(self, value)
            }

            /// Visits [`KermlInvariantMember`]; the default implementation walks its children.
            fn visit_kerml_invariant_member(&mut self, node: &$($mutability)? Node<KermlInvariantMember>) {
                walk_kerml_invariant_member(self, node)
            }

            /// Visits [`KermlConnectorEnd`]; the default implementation walks its children.
            fn visit_kerml_connector_end(&mut self, node: &$($mutability)? Node<KermlConnectorEnd>) {
                walk_kerml_connector_end(self, node)
            }

            /// Visits [`KermlConnectorMember`]; the default implementation walks its children.
            fn visit_kerml_connector_member(&mut self, node: &$($mutability)? Node<KermlConnectorMember>) {
                walk_kerml_connector_member(self, node)
            }

            /// Visits [`KermlBindingMember`]; the default implementation walks its children.
            fn visit_kerml_binding_member(&mut self, node: &$($mutability)? Node<KermlBindingMember>) {
                walk_kerml_binding_member(self, node)
            }

            /// Visits [`KermlSuccessionMember`]; the default implementation walks its children.
            fn visit_kerml_succession_member(&mut self, node: &$($mutability)? Node<KermlSuccessionMember>) {
                walk_kerml_succession_member(self, node)
            }

            /// Visits [`KermlRelationshipDecl`]; the default implementation walks its children.
            fn visit_kerml_relationship_decl(&mut self, node: &$($mutability)? Node<KermlRelationshipDecl>) {
                walk_kerml_relationship_decl(self, node)
            }

            /// Visits [`KermlRelationshipKeyword`]; the default implementation walks its children.
            fn visit_kerml_relationship_keyword(&mut self, node: &$($mutability)? KermlRelationshipKeyword) {
                walk_kerml_relationship_keyword(self, node)
            }
        }

        pub fn walk_binary_operator<V: $Visitor>(visitor: &mut V, node: &$($mutability)? BinaryOperator) {
            match node {
                BinaryOperator::Eq => {}
                BinaryOperator::Ne => {}
                BinaryOperator::StrictEq => {}
                BinaryOperator::StrictNe => {}
                BinaryOperator::Lt => {}
                BinaryOperator::Le => {}
                BinaryOperator::Gt => {}
                BinaryOperator::Ge => {}
                BinaryOperator::Add => {}
                BinaryOperator::Sub => {}
                BinaryOperator::Mul => {}
                BinaryOperator::Div => {}
                BinaryOperator::Mod => {}
                BinaryOperator::Exp => {}
                BinaryOperator::Pow => {}
                BinaryOperator::And => {}
                BinaryOperator::Or => {}
                BinaryOperator::Xor => {}
                BinaryOperator::Implies => {}
                BinaryOperator::Range => {}
                BinaryOperator::BitOr => {}
                BinaryOperator::BitAnd => {}
                BinaryOperator::NullCoalesce => {}
                BinaryOperator::Other(field_0) => {
                    visitor.visit_text(field_0);
                }
            }
        }

        pub fn walk_type_check_kind<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? TypeCheckKind) {
            match node {
                TypeCheckKind::Istype => {}
                TypeCheckKind::Hastype => {}
                TypeCheckKind::As => {}
            }
        }

        pub fn walk_unary_operator<V: $Visitor>(visitor: &mut V, node: &$($mutability)? UnaryOperator) {
            match node {
                UnaryOperator::Plus => {}
                UnaryOperator::Minus => {}
                UnaryOperator::Not => {}
                UnaryOperator::BitNot => {}
                UnaryOperator::Other(field_0) => {
                    visitor.visit_text(field_0);
                }
            }
        }

        pub fn walk_expression<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<Expression>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                Expression::LiteralInteger(field_0) => {
                    let _ = field_0;
                }
                Expression::LiteralReal(field_0) => {
                    visitor.visit_text(field_0);
                }
                Expression::LiteralString(field_0) => {
                    visitor.visit_text(field_0);
                }
                Expression::LiteralBoolean(field_0) => {
                    let _ = field_0;
                }
                Expression::Unit(field_0) => {
                    visitor.visit_text(field_0);
                }
                Expression::FeatureRef(field_0) => {
                    visitor.visit_qualified_reference(field_0);
                }
                Expression::MemberAccess { base, member, separator } => {
                    visitor.visit_expression(&$($mutability)? **base);
                    visitor.visit_qualified_reference(member);
                    visitor.visit_reference_separator(separator);
                }
                Expression::Index { base, index } => {
                    visitor.visit_expression(&$($mutability)? **base);
                    visitor.visit_expression(&$($mutability)? **index);
                }
                Expression::Bracket(field_0) => {
                    visitor.visit_expression(&$($mutability)? **field_0);
                }
                Expression::LiteralWithUnit { value, unit } => {
                    visitor.visit_expression(&$($mutability)? **value);
                    visitor.visit_expression(&$($mutability)? **unit);
                }
                Expression::BinaryOp { op, left, right } => {
                    visitor.visit_binary_operator(op);
                    visitor.visit_expression(&$($mutability)? **left);
                    visitor.visit_expression(&$($mutability)? **right);
                }
                Expression::UnaryOp { op, operand } => {
                    visitor.visit_unary_operator(op);
                    visitor.visit_expression(&$($mutability)? **operand);
                }
                Expression::Invocation { callee, args } => {
                    visitor.visit_expression(&$($mutability)? **callee);
                    for inner in args {
                        visitor.visit_argument(inner);
                    }
                }
                Expression::Tuple(field_0) => {
                    for inner in field_0 {
                        visitor.visit_expression(inner);
                    }
                }
                Expression::Classification { metaclass } => {
                    visitor.visit_qualified_reference(metaclass);
                }
                Expression::MetaCast { base, metaclass } => {
                    visitor.visit_expression(&$($mutability)? **base);
                    visitor.visit_qualified_reference(metaclass);
                }
                Expression::TypeCheck { kind, operand, type_name } => {
                    visitor.visit_type_check_kind(kind);
                    if let Some(inner) = operand {
                        visitor.visit_expression(&$($mutability)? **inner);
                    }
                    visitor.visit_qualified_reference(type_name);
                }
                Expression::Select { base, selector } => {
                    visitor.visit_expression(&$($mutability)? **base);
                    visitor.visit_qualified_reference(selector);
                }
                Expression::Collect { base, selector } => {
                    visitor.visit_expression(&$($mutability)? **base);
                    visitor.visit_qualified_reference(selector);
                }
                Expression::Null => {}
                Expression::Parenthesized(field_0) => {
                    visitor.visit_expression(&$($mutability)? **field_0);
                }
                Expression::Constructor { type_name, args } => {
                    visitor.visit_qualified_reference(type_name);
                    for inner in args {
                        visitor.visit_argument(inner);
                    }
                }
                Expression::FeatureChainRef(field_0) => {
                    visitor.visit_qualified_reference(field_0);
                }
                Expression::CollectionOp { op, base, args, brace_body, dot_shorthand } => {
                    visitor.visit_collection_operator(op);
                    visitor.visit_expression(&$($mutability)? **base);
                    for inner in args {
                        visitor.visit_argument(inner);
                    }
                    if let Some(inner) = brace_body {
                        visitor.visit_collection_operator_body(&$($mutability)? **inner);
                    }
                    let _ = dot_shorthand;
                }
                Expression::MetadataAccess(field_0) => {
                    visitor.visit_expression(&$($mutability)? **field_0);
                }
                Expression::Conditional { test, then_expr, else_expr } => {
                    visitor.visit_expression(&$($mutability)? **test);
                    visitor.visit_expression(&$($mutability)? **then_expr);
                    visitor.visit_expression(&$($mutability)? **else_expr);
                }
                Expression::Extent { target } => {
                    visitor.visit_qualified_reference(target);
                }
                Expression::BodyExpr(field_0) => {
                    visitor.visit_collection_operator_body(&$($mutability)? **field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_collection_operator_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<CollectionOperatorBody>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let CollectionOperatorBody { open_brace_span, doc, parameters, result, close_brace_span } = &$($mutability)? node.value;
            visitor.visit_span(open_brace_span);
            if let Some(inner) = doc {
                visitor.visit_doc_comment(&$($mutability)? **inner);
            }
            for inner in parameters {
                visitor.visit_collection_operator_parameter(inner);
            }
            if let Some(inner) = result {
                visitor.visit_expression(&$($mutability)? **inner);
            }
            visitor.visit_span(close_brace_span);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_collection_operator_parameter<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<CollectionOperatorParameter>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let CollectionOperatorParameter { direction, reference_keyword_span, name, name_span, typing, terminator } = &$($mutability)? node.value;
            if let Some(inner) = direction {
                visitor.visit_in_out(inner);
            }
            if let Some(inner) = reference_keyword_span {
                visitor.visit_span(inner);
            }
            visitor.visit_text(name);
            visitor.visit_span(name_span);
            if let Some(inner) = typing {
                visitor.visit_collection_operator_parameter_typing(inner);
            }
            visitor.visit_collection_operator_parameter_terminator(terminator);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_collection_operator_parameter_terminator<V: $Visitor>(visitor: &mut V, node: &$($mutability)? CollectionOperatorParameterTerminator) {
            match node {
                CollectionOperatorParameterTerminator::Semicolon { span } => {
                    visitor.visit_span(span);
                }
                CollectionOperatorParameterTerminator::Body { open_brace_span, doc, close_brace_span } => {
                    visitor.visit_span(open_brace_span);
                    if let Some(inner) = doc {
                        visitor.visit_doc_comment(&$($mutability)? **inner);
                    }
                    visitor.visit_span(close_brace_span);
                }
            }
        }

        pub fn walk_collection_operator_parameter_typing<V: $Visitor>(visitor: &mut V, node: &$($mutability)? CollectionOperatorParameterTyping) {
            let CollectionOperatorParameterTyping { separator_span, target } = node;
            visitor.visit_span(separator_span);
            visitor.visit_qualified_reference(target);
        }

        pub fn walk_argument<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Argument) {
            let Argument { parameter, value } = node;
            if let Some(inner) = parameter {
                visitor.visit_qualified_reference(inner);
            }
            visitor.visit_expression(value);
        }

        pub fn walk_collection_operator<V: $Visitor>(visitor: &mut V, node: &$($mutability)? CollectionOperator) {
            match node {
                CollectionOperator::Collect => {}
                CollectionOperator::Select => {}
                CollectionOperator::SelectOne => {}
                CollectionOperator::Size => {}
                CollectionOperator::IsEmpty => {}
                CollectionOperator::NotEmpty => {}
                CollectionOperator::Includes => {}
                CollectionOperator::Including => {}
                CollectionOperator::Excludes => {}
                CollectionOperator::Excluding => {}
                CollectionOperator::ExcludingAt => {}
                CollectionOperator::ExcludingOnce => {}
                CollectionOperator::Equals => {}
                CollectionOperator::ForAll => {}
                CollectionOperator::Exists => {}
                CollectionOperator::Sum => {}
                CollectionOperator::Sort => {}
                CollectionOperator::Filter => {}
                CollectionOperator::Reduce => {}
                CollectionOperator::Other(field_0) => {
                    visitor.visit_text(field_0);
                }
            }
        }

        pub fn walk_multiplicity<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<Multiplicity>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let Multiplicity { lower, upper, span } = &$($mutability)? node.value;
            if let Some(inner) = lower {
                visitor.visit_expression(&$($mutability)? **inner);
            }
            if let Some(inner) = upper {
                visitor.visit_expression(&$($mutability)? **inner);
            }
            visitor.visit_span(span);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_typing_kind<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? TypingKind) {
            match node {
                TypingKind::Typing => {}
                TypingKind::Subclassification => {}
            }
        }

        pub fn walk_typing_relationship<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<TypingRelationship>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let TypingRelationship { target, kind, span, is_conjugated, is_implied, spelling } = &$($mutability)? node.value;
            for inner in target {
                visitor.visit_qualified_reference(inner);
            }
            visitor.visit_typing_kind(kind);
            visitor.visit_span(span);
            let _ = is_conjugated;
            let _ = is_implied;
            visitor.visit_typing_spelling(spelling);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_typing_spelling<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? TypingSpelling) {
            match node {
                TypingSpelling::Operator => {}
                TypingSpelling::Specializes => {}
                TypingSpelling::DefinedBy => {}
                TypingSpelling::TypedBy => {}
            }
        }

        pub fn walk_subsetting_kind<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? SubsettingKind) {
            match node {
                SubsettingKind::Subsets => {}
                SubsettingKind::References => {}
                SubsettingKind::Redefines => {}
                SubsettingKind::Crosses => {}
                SubsettingKind::Intersects => {}
            }
        }

        pub fn walk_subsetting_relationship<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<SubsettingRelationship>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let SubsettingRelationship { target, kind, span, is_implied } = &$($mutability)? node.value;
            for inner in target {
                visitor.visit_qualified_reference(inner);
            }
            visitor.visit_subsetting_kind(kind);
            visitor.visit_span(span);
            let _ = is_implied;
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_connection_end<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ConnectionEnd>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ConnectionEnd { expression, multiplicity, span } = &$($mutability)? node.value;
            visitor.visit_expression(expression);
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_span(span);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_filter_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FilterMember>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let FilterMember { visibility, condition } = &$($mutability)? node.value;
            if let Some(inner) = visibility {
                visitor.visit_visibility(inner);
            }
            visitor.visit_expression(condition);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_parse_error_node<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ParseErrorNode>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            visitor.visit_parse_error_node_value(&$($mutability)? node.value);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_parse_error_node_value<V: $Visitor>(visitor: &mut V, node: &$($mutability)? ParseErrorNode) {
            let ParseErrorNode { message, code, expected, found, suggestion, category } = node;
            visitor.visit_text(message);
            visitor.visit_text(code);
            if let Some(inner) = expected {
                visitor.visit_text(inner);
            }
            if let Some(inner) = found {
                visitor.visit_text(inner);
            }
            if let Some(inner) = suggestion {
                visitor.visit_text(inner);
            }
            let _ = category;
        }

        pub fn walk_unsupported_production<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? UnsupportedProduction) {
            match node {
                UnsupportedProduction::BindingConnectorAsUsage => {}
                UnsupportedProduction::Message => {}
                UnsupportedProduction::SuccessionAsUsage => {}
                UnsupportedProduction::PerformActionUsage => {}
                UnsupportedProduction::ExhibitStateUsage => {}
                UnsupportedProduction::IncludeUseCaseUsage => {}
                UnsupportedProduction::ReferenceConnectionUsage => {}
                UnsupportedProduction::ConnectionUsageInPartDefinition => {}
                UnsupportedProduction::ActionBodyMember => {}
                UnsupportedProduction::UnmodelledBodyMember => {}
            }
        }

        pub fn walk_unsupported_grammar_node<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<UnsupportedGrammarNode>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let UnsupportedGrammarNode { production, diagnostic } = &$($mutability)? node.value;
            visitor.visit_unsupported_production(production);
            visitor.visit_parse_error_node_value(diagnostic);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_identification<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Identification) {
            let Identification { short_name, name } = node;
            if let Some(inner) = short_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = name {
                visitor.visit_text(inner);
            }
        }

        pub fn walk_visibility<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? Visibility) {
            match node {
                Visibility::Public => {}
                Visibility::Private => {}
                Visibility::Protected => {}
            }
        }

        pub fn walk_filter_package_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FilterPackageMember>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let FilterPackageMember { open_bracket_span, expression, close_bracket_span } = &$($mutability)? node.value;
            visitor.visit_span(open_bracket_span);
            visitor.visit_expression(expression);
            visitor.visit_span(close_bracket_span);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_import_suffix_spans<V: $Visitor>(visitor: &mut V, node: &$($mutability)? ImportSuffixSpans) {
            let ImportSuffixSpans { span, separator_span, marker_span } = node;
            visitor.visit_span(span);
            visitor.visit_span(separator_span);
            visitor.visit_span(marker_span);
        }

        pub fn walk_import_shape<V: $Visitor>(visitor: &mut V, node: &$($mutability)? ImportShape) {
            match node {
                ImportShape::Membership { recursive_suffix } => {
                    if let Some(inner) = recursive_suffix {
                        visitor.visit_import_suffix_spans(inner);
                    }
                }
                ImportShape::Namespace { wildcard_suffix, recursive_suffix, combined_recursive_suffix_span } => {
                    visitor.visit_import_suffix_spans(wildcard_suffix);
                    if let Some(inner) = recursive_suffix {
                        visitor.visit_import_suffix_spans(inner);
                    }
                    if let Some(inner) = combined_recursive_suffix_span {
                        visitor.visit_span(inner);
                    }
                }
                ImportShape::Filter { recursive_suffix, members } => {
                    if let Some(inner) = recursive_suffix {
                        visitor.visit_import_suffix_spans(inner);
                    }
                    for inner in members {
                        visitor.visit_filter_package_member(inner);
                    }
                }
            }
        }

        pub fn walk_import_target<V: $Visitor>(visitor: &mut V, node: &$($mutability)? ImportTarget) {
            let ImportTarget { span, all_span, reference, shape } = node;
            visitor.visit_span(span);
            if let Some(inner) = all_span {
                visitor.visit_span(inner);
            }
            visitor.visit_qualified_reference(reference);
            visitor.visit_import_shape(shape);
        }

        pub fn walk_import<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<Import>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let Import { membership, target, body_elements } = &$($mutability)? node.value;
            visitor.visit_membership(membership);
            visitor.visit_import_target(target);
            if let Some(inner) = body_elements {
                for inner in inner {
                    visitor.visit_relationship_body_element(inner);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_doc_comment<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<DocComment>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let DocComment { identification, locale, text, body_span } = &$($mutability)? node.value;
            if let Some(inner) = identification {
                visitor.visit_identification(inner);
            }
            if let Some(inner) = locale {
                visitor.visit_text(inner);
            }
            visitor.visit_text(text);
            visitor.visit_span(body_span);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_comment_annotation<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<CommentAnnotation>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let CommentAnnotation { keyword_span, identification, about_targets, locale, text, body_span } = &$($mutability)? node.value;
            if let Some(inner) = keyword_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = identification {
                visitor.visit_identification(inner);
            }
            for inner in about_targets {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = locale {
                visitor.visit_text(inner);
            }
            visitor.visit_text(text);
            visitor.visit_span(body_span);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_textual_representation<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<TextualRepresentation>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let TextualRepresentation { rep_identification, language, language_span, text, body_span } = &$($mutability)? node.value;
            if let Some(inner) = rep_identification {
                visitor.visit_identification(inner);
            }
            visitor.visit_text(language);
            if let Some(inner) = language_span {
                visitor.visit_span(inner);
            }
            visitor.visit_text(text);
            visitor.visit_span(body_span);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_annotating_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? AnnotatingMember) {
            match node {
                AnnotatingMember::Doc(field_0) => {
                    visitor.visit_doc_comment(field_0);
                }
                AnnotatingMember::Comment(field_0) => {
                    visitor.visit_comment_annotation(field_0);
                }
                AnnotatingMember::TextualRep(field_0) => {
                    visitor.visit_textual_representation(field_0);
                }
                AnnotatingMember::MetadataAnnotation(field_0) => {
                    visitor.visit_metadata_annotation(field_0);
                }
            }
        }

        pub fn walk_feature_value_kind<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? FeatureValueKind) {
            match node {
                FeatureValueKind::Bind => {}
                FeatureValueKind::Assign => {}
            }
        }

        pub fn walk_feature_value<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FeatureValue>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let FeatureValue { kind, is_default, has_operator, expression, span } = &$($mutability)? node.value;
            visitor.visit_feature_value_kind(kind);
            let _ = is_default;
            let _ = has_operator;
            visitor.visit_expression(expression);
            visitor.visit_span(span);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_membership_kind<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? MembershipKind) {
            match node {
                MembershipKind::OwningMembership => {}
                MembershipKind::FeatureMembership => {}
                MembershipKind::Import => {}
                MembershipKind::Alias => {}
                MembershipKind::VariantMembership => {}
                MembershipKind::ActorMembership => {}
            }
        }

        pub fn walk_membership<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Membership) {
            let Membership { kind, visibility, span } = node;
            visitor.visit_membership_kind(kind);
            if let Some(inner) = visibility {
                visitor.visit_visibility(inner);
            }
            visitor.visit_span(span);
        }

        pub fn walk_reference_separator<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? ReferenceSeparator) {
            match node {
                ReferenceSeparator::ColonColon => {}
                ReferenceSeparator::Dot => {}
            }
        }

        pub fn walk_root_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<RootElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                RootElement::Package(field_0) => {
                    visitor.visit_package(field_0);
                }
                RootElement::LibraryPackage(field_0) => {
                    visitor.visit_library_package(field_0);
                }
                RootElement::Namespace(field_0) => {
                    visitor.visit_namespace_decl(field_0);
                }
                RootElement::Import(field_0) => {
                    visitor.visit_import(&$($mutability)? **field_0);
                }
                RootElement::Member(field_0) => {
                    visitor.visit_package_body_element(&$($mutability)? **field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_namespace_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<NamespaceDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let NamespaceDecl { identification, body } = &$($mutability)? node.value;
            visitor.visit_qualified_identification(identification);
            visitor.visit_package_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_root_namespace<V: $Visitor>(visitor: &mut V, node: &$($mutability)? RootNamespace) {
            let RootNamespace { elements } = node;
            for inner in elements {
                visitor.visit_root_element(inner);
            }
        }

        pub fn walk_qualified_declaration_name<V: $Visitor>(visitor: &mut V, node: &$($mutability)? QualifiedDeclarationName) {
            let QualifiedDeclarationName { reference } = node;
            visitor.visit_qualified_reference(reference);
        }

        pub fn walk_declaration_name<V: $Visitor>(visitor: &mut V, node: &$($mutability)? DeclarationName) {
            match node {
                DeclarationName::Simple(field_0) => {
                    visitor.visit_text(field_0);
                }
                DeclarationName::Qualified(field_0) => {
                    visitor.visit_qualified_declaration_name(field_0);
                }
            }
        }

        pub fn walk_qualified_identification<V: $Visitor>(visitor: &mut V, node: &$($mutability)? QualifiedIdentification) {
            let QualifiedIdentification { short_name, name } = node;
            if let Some(inner) = short_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = name {
                visitor.visit_declaration_name(inner);
            }
        }

        pub fn walk_package<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<Package>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let Package { identification, body } = &$($mutability)? node.value;
            visitor.visit_qualified_identification(identification);
            visitor.visit_package_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_package_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? PackageBody) {
            match node {
                PackageBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                PackageBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_package_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_library_package<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<LibraryPackage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let LibraryPackage { is_standard, identification, body } = &$($mutability)? node.value;
            let _ = is_standard;
            visitor.visit_qualified_identification(identification);
            visitor.visit_package_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_package_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<PackageBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                PackageBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                PackageBodyElement::Unsupported(field_0) => {
                    visitor.visit_unsupported_grammar_node(field_0);
                }
                PackageBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                PackageBodyElement::Filter(field_0) => {
                    visitor.visit_filter_member(field_0);
                }
                PackageBodyElement::Package(field_0) => {
                    visitor.visit_package(field_0);
                }
                PackageBodyElement::LibraryPackage(field_0) => {
                    visitor.visit_library_package(field_0);
                }
                PackageBodyElement::Import(field_0) => {
                    visitor.visit_import(field_0);
                }
                PackageBodyElement::PartDef(field_0) => {
                    visitor.visit_part_def(field_0);
                }
                PackageBodyElement::PartUsage(field_0) => {
                    visitor.visit_part_usage(field_0);
                }
                PackageBodyElement::PortDef(field_0) => {
                    visitor.visit_port_def(field_0);
                }
                PackageBodyElement::InterfaceDef(field_0) => {
                    visitor.visit_interface_def(field_0);
                }
                PackageBodyElement::AliasDef(field_0) => {
                    visitor.visit_alias_def(field_0);
                }
                PackageBodyElement::AttributeDef(field_0) => {
                    visitor.visit_attribute_def(field_0);
                }
                PackageBodyElement::ActionDef(field_0) => {
                    visitor.visit_action_def(field_0);
                }
                PackageBodyElement::ActionUsage(field_0) => {
                    visitor.visit_action_usage(field_0);
                }
                PackageBodyElement::RequirementDef(field_0) => {
                    visitor.visit_requirement_def(field_0);
                }
                PackageBodyElement::RequirementUsage(field_0) => {
                    visitor.visit_requirement_usage(field_0);
                }
                PackageBodyElement::Satisfy(field_0) => {
                    visitor.visit_satisfy_requirement_usage(&$($mutability)? **field_0);
                }
                PackageBodyElement::UseCaseDef(field_0) => {
                    visitor.visit_use_case_def(field_0);
                }
                PackageBodyElement::Actor(field_0) => {
                    visitor.visit_actor_decl(field_0);
                }
                PackageBodyElement::StateDef(field_0) => {
                    visitor.visit_state_def(field_0);
                }
                PackageBodyElement::StateUsage(field_0) => {
                    visitor.visit_state_usage(field_0);
                }
                PackageBodyElement::ItemDef(field_0) => {
                    visitor.visit_item_def(field_0);
                }
                PackageBodyElement::IndividualDef(field_0) => {
                    visitor.visit_individual_def(field_0);
                }
                PackageBodyElement::ConstraintDef(field_0) => {
                    visitor.visit_constraint_def(field_0);
                }
                PackageBodyElement::ConstraintUsage(field_0) => {
                    visitor.visit_constraint_usage(field_0);
                }
                PackageBodyElement::CalcUsage(field_0) => {
                    visitor.visit_calc_usage(field_0);
                }
                PackageBodyElement::CalcDef(field_0) => {
                    visitor.visit_calc_def(field_0);
                }
                PackageBodyElement::ViewDef(field_0) => {
                    visitor.visit_view_def(field_0);
                }
                PackageBodyElement::ViewpointDef(field_0) => {
                    visitor.visit_viewpoint_def(field_0);
                }
                PackageBodyElement::RenderingDef(field_0) => {
                    visitor.visit_rendering_def(field_0);
                }
                PackageBodyElement::ViewUsage(field_0) => {
                    visitor.visit_view_usage(field_0);
                }
                PackageBodyElement::ViewpointUsage(field_0) => {
                    visitor.visit_viewpoint_usage(field_0);
                }
                PackageBodyElement::RenderingUsage(field_0) => {
                    visitor.visit_rendering_usage(field_0);
                }
                PackageBodyElement::ConnectionDef(field_0) => {
                    visitor.visit_connection_def(field_0);
                }
                PackageBodyElement::MetadataDef(field_0) => {
                    visitor.visit_metadata_def(field_0);
                }
                PackageBodyElement::MetadataUsage(field_0) => {
                    visitor.visit_metadata_usage(field_0);
                }
                PackageBodyElement::EnumDef(field_0) => {
                    visitor.visit_enum_def(field_0);
                }
                PackageBodyElement::OccurrenceDef(field_0) => {
                    visitor.visit_occurrence_def(field_0);
                }
                PackageBodyElement::OccurrenceUsage(field_0) => {
                    visitor.visit_occurrence_usage(field_0);
                }
                PackageBodyElement::Dependency(field_0) => {
                    visitor.visit_dependency(field_0);
                }
                PackageBodyElement::AllocationDef(field_0) => {
                    visitor.visit_allocation_def(field_0);
                }
                PackageBodyElement::AllocationUsage(field_0) => {
                    visitor.visit_allocation_usage(field_0);
                }
                PackageBodyElement::FlowDef(field_0) => {
                    visitor.visit_flow_def(field_0);
                }
                PackageBodyElement::FlowUsage(field_0) => {
                    visitor.visit_flow_usage(field_0);
                }
                PackageBodyElement::ConcernUsage(field_0) => {
                    visitor.visit_concern_usage(field_0);
                }
                PackageBodyElement::CaseDef(field_0) => {
                    visitor.visit_case_def(field_0);
                }
                PackageBodyElement::CaseUsage(field_0) => {
                    visitor.visit_case_usage(field_0);
                }
                PackageBodyElement::AnalysisCaseDef(field_0) => {
                    visitor.visit_analysis_case_def(field_0);
                }
                PackageBodyElement::AnalysisCaseUsage(field_0) => {
                    visitor.visit_analysis_case_usage(field_0);
                }
                PackageBodyElement::VerificationCaseDef(field_0) => {
                    visitor.visit_verification_case_def(field_0);
                }
                PackageBodyElement::VerificationCaseUsage(field_0) => {
                    visitor.visit_verification_case_usage(field_0);
                }
                PackageBodyElement::UseCaseUsage(field_0) => {
                    visitor.visit_use_case_usage(field_0);
                }
                PackageBodyElement::FeatureDecl(field_0) => {
                    visitor.visit_feature_decl(field_0);
                }
                PackageBodyElement::ClassifierDecl(field_0) => {
                    visitor.visit_classifier_decl(field_0);
                }
                PackageBodyElement::KermlSemanticDecl(field_0) => {
                    visitor.visit_kerml_semantic_decl(field_0);
                }
                PackageBodyElement::KermlClassifier(field_0) => {
                    visitor.visit_kerml_classifier_decl(&$($mutability)? **field_0);
                }
                PackageBodyElement::KermlInvariant(field_0) => {
                    visitor.visit_kerml_invariant_member(&$($mutability)? **field_0);
                }
                PackageBodyElement::KermlConnector(field_0) => {
                    visitor.visit_kerml_connector_member(&$($mutability)? **field_0);
                }
                PackageBodyElement::KermlRelationship(field_0) => {
                    visitor.visit_kerml_relationship_decl(&$($mutability)? **field_0);
                }
                PackageBodyElement::KermlFeatureMember(field_0) => {
                    visitor.visit_kerml_feature_member(&$($mutability)? **field_0);
                }
                PackageBodyElement::KermlFeatureDecl(field_0) => {
                    visitor.visit_kerml_feature_decl(field_0);
                }
                PackageBodyElement::KermlBareDeclaration(field_0) => {
                    visitor.visit_kerml_bare_declaration(field_0);
                }
                PackageBodyElement::ExtendedLibraryDecl(field_0) => {
                    visitor.visit_extended_library_decl(field_0);
                }
                PackageBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(field_0);
                }
                PackageBodyElement::ItemUsage(field_0) => {
                    visitor.visit_item_usage(field_0);
                }
                PackageBodyElement::PortUsage(field_0) => {
                    visitor.visit_port_usage(field_0);
                }
                PackageBodyElement::ConnectionUsage(field_0) => {
                    visitor.visit_connection_usage_member(field_0);
                }
                PackageBodyElement::InterfaceUsage(field_0) => {
                    visitor.visit_interface_usage(field_0);
                }
                PackageBodyElement::Ref(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
                PackageBodyElement::EnumerationUsage(field_0) => {
                    visitor.visit_enumeration_usage(field_0);
                }
                PackageBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                PackageBodyElement::Connect(field_0) => {
                    visitor.visit_connect(field_0);
                }
                PackageBodyElement::DefaultReferenceUsage(field_0) => {
                    visitor.visit_default_reference_usage(field_0);
                }
                PackageBodyElement::AssertConstraint(field_0) => {
                    visitor.visit_assert_constraint_member(field_0);
                }
                PackageBodyElement::PerformUsage(field_0) => {
                    visitor.visit_perform(field_0);
                }
                PackageBodyElement::BindingConnectorUsage(field_0) => {
                    visitor.visit_binding_connector_usage(field_0);
                }
                PackageBodyElement::Succession(field_0) => {
                    visitor.visit_first_stmt(field_0);
                }
                PackageBodyElement::ExhibitState(field_0) => {
                    visitor.visit_exhibit_state(field_0);
                }
                PackageBodyElement::IncludeUseCase(field_0) => {
                    visitor.visit_include_use_case(field_0);
                }
                PackageBodyElement::ExtendedDefinition(field_0) => {
                    visitor.visit_extended_definition(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_part_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<PartDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let PartDef { definition_prefix, is_individual, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            let _ = is_individual;
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_part_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_extended_definition<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ExtendedDefinition>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ExtendedDefinition { prefix_keywords, definition_prefix, has_def_keyword, identification, specializes, body } = &$($mutability)? node.value;
            for inner in prefix_keywords {
                visitor.visit_metadata_keyword_usage(inner);
            }
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            let _ = has_def_keyword;
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_package_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_definition_prefix<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<DefinitionPrefix>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            visitor.visit_definition_prefix_value(&$($mutability)? node.value);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_definition_prefix_value<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? DefinitionPrefix) {
            match node {
                DefinitionPrefix::Abstract => {}
                DefinitionPrefix::Variation => {}
            }
        }

        pub fn walk_part_def_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? PartDefBody) {
            match node {
                PartDefBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                PartDefBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_part_def_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_part_def_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<PartDefBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                PartDefBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                PartDefBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                PartDefBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                PartDefBodyElement::Dependency(field_0) => {
                    visitor.visit_dependency(field_0);
                }
                PartDefBodyElement::AttributeDef(field_0) => {
                    visitor.visit_attribute_def(field_0);
                }
                PartDefBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(field_0);
                }
                PartDefBodyElement::DefaultReferenceUsage(field_0) => {
                    visitor.visit_default_reference_usage(field_0);
                }
                PartDefBodyElement::RequirementUsage(field_0) => {
                    visitor.visit_requirement_usage(field_0);
                }
                PartDefBodyElement::ItemDef(field_0) => {
                    visitor.visit_item_def(field_0);
                }
                PartDefBodyElement::ItemUsage(field_0) => {
                    visitor.visit_item_usage(field_0);
                }
                PartDefBodyElement::Ref(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
                PartDefBodyElement::PortUsage(field_0) => {
                    visitor.visit_port_usage(field_0);
                }
                PartDefBodyElement::PartUsage(field_0) => {
                    visitor.visit_part_usage(&$($mutability)? **field_0);
                }
                PartDefBodyElement::PartDef(field_0) => {
                    visitor.visit_part_def(field_0);
                }
                PartDefBodyElement::OccurrenceUsage(field_0) => {
                    visitor.visit_occurrence_usage(&$($mutability)? **field_0);
                }
                PartDefBodyElement::InterfaceDef(field_0) => {
                    visitor.visit_interface_def(field_0);
                }
                PartDefBodyElement::InterfaceUsage(field_0) => {
                    visitor.visit_interface_usage(field_0);
                }
                PartDefBodyElement::Connect(field_0) => {
                    visitor.visit_connect(field_0);
                }
                PartDefBodyElement::FlowUsage(field_0) => {
                    visitor.visit_flow_usage(field_0);
                }
                PartDefBodyElement::Connection(field_0) => {
                    visitor.visit_connection_usage_member(field_0);
                }
                PartDefBodyElement::Perform(field_0) => {
                    visitor.visit_perform(field_0);
                }
                PartDefBodyElement::Allocate(field_0) => {
                    visitor.visit_allocate(field_0);
                }
                PartDefBodyElement::UnsupportedMember(field_0) => {
                    visitor.visit_unsupported_grammar_node(field_0);
                }
                PartDefBodyElement::ExhibitState(field_0) => {
                    visitor.visit_exhibit_state(field_0);
                }
                PartDefBodyElement::CalcUsage(field_0) => {
                    visitor.visit_calc_usage(field_0);
                }
                PartDefBodyElement::ConstraintDef(field_0) => {
                    visitor.visit_constraint_def(field_0);
                }
                PartDefBodyElement::ConstraintUsage(field_0) => {
                    visitor.visit_constraint_usage(field_0);
                }
                PartDefBodyElement::Import(field_0) => {
                    visitor.visit_import(field_0);
                }
                PartDefBodyElement::ActionUsage(field_0) => {
                    visitor.visit_action_usage(&$($mutability)? **field_0);
                }
                PartDefBodyElement::ActionDef(field_0) => {
                    visitor.visit_action_def(field_0);
                }
                PartDefBodyElement::StateUsage(field_0) => {
                    visitor.visit_state_usage(field_0);
                }
                PartDefBodyElement::EnumerationUsage(field_0) => {
                    visitor.visit_enumeration_usage(field_0);
                }
                PartDefBodyElement::AssertConstraint(field_0) => {
                    visitor.visit_assert_constraint_member(field_0);
                }
                PartDefBodyElement::Satisfy(field_0) => {
                    visitor.visit_satisfy_requirement_usage(&$($mutability)? **field_0);
                }
                PartDefBodyElement::VariantUsage(field_0) => {
                    visitor.visit_variant_usage(field_0);
                }
                PartDefBodyElement::StateDef(field_0) => {
                    visitor.visit_state_def(field_0);
                }
                PartDefBodyElement::MetadataDef(field_0) => {
                    visitor.visit_metadata_def(field_0);
                }
                PartDefBodyElement::MetadataUsage(field_0) => {
                    visitor.visit_metadata_usage(field_0);
                }
                PartDefBodyElement::FlowDef(field_0) => {
                    visitor.visit_flow_def(field_0);
                }
                PartDefBodyElement::RequirementDef(field_0) => {
                    visitor.visit_requirement_def(field_0);
                }
                PartDefBodyElement::OccurrenceDef(field_0) => {
                    visitor.visit_occurrence_def(field_0);
                }
                PartDefBodyElement::ConnectionDef(field_0) => {
                    visitor.visit_connection_def(field_0);
                }
                PartDefBodyElement::PortDef(field_0) => {
                    visitor.visit_port_def(field_0);
                }
                PartDefBodyElement::CalcDef(field_0) => {
                    visitor.visit_calc_def(field_0);
                }
                PartDefBodyElement::EnumDef(field_0) => {
                    visitor.visit_enum_def(field_0);
                }
                PartDefBodyElement::AllocationDef(field_0) => {
                    visitor.visit_allocation_def(field_0);
                }
                PartDefBodyElement::AllocationUsage(field_0) => {
                    visitor.visit_allocation_usage(field_0);
                }
                PartDefBodyElement::ViewDef(field_0) => {
                    visitor.visit_view_def(field_0);
                }
                PartDefBodyElement::ViewUsage(field_0) => {
                    visitor.visit_view_usage(field_0);
                }
                PartDefBodyElement::ViewpointDef(field_0) => {
                    visitor.visit_viewpoint_def(field_0);
                }
                PartDefBodyElement::ViewpointUsage(field_0) => {
                    visitor.visit_viewpoint_usage(field_0);
                }
                PartDefBodyElement::RenderingDef(field_0) => {
                    visitor.visit_rendering_def(field_0);
                }
                PartDefBodyElement::RenderingUsage(field_0) => {
                    visitor.visit_rendering_usage(field_0);
                }
                PartDefBodyElement::KermlClassifier(field_0) => {
                    visitor.visit_kerml_classifier_decl(&$($mutability)? **field_0);
                }
                PartDefBodyElement::CaseDef(field_0) => {
                    visitor.visit_case_def(field_0);
                }
                PartDefBodyElement::CaseUsage(field_0) => {
                    visitor.visit_case_usage(field_0);
                }
                PartDefBodyElement::UseCaseDef(field_0) => {
                    visitor.visit_use_case_def(field_0);
                }
                PartDefBodyElement::UseCaseUsage(field_0) => {
                    visitor.visit_use_case_usage(field_0);
                }
                PartDefBodyElement::AnalysisCaseDef(field_0) => {
                    visitor.visit_analysis_case_def(field_0);
                }
                PartDefBodyElement::AnalysisCaseUsage(field_0) => {
                    visitor.visit_analysis_case_usage(field_0);
                }
                PartDefBodyElement::VerificationCaseDef(field_0) => {
                    visitor.visit_verification_case_def(field_0);
                }
                PartDefBodyElement::VerificationCaseUsage(field_0) => {
                    visitor.visit_verification_case_usage(field_0);
                }
                PartDefBodyElement::FirstStmt(field_0) => {
                    visitor.visit_first_stmt(field_0);
                }
                PartDefBodyElement::Bind(field_0) => {
                    visitor.visit_bind(field_0);
                }
                PartDefBodyElement::AliasDef(field_0) => {
                    visitor.visit_alias_def(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_connection_usage_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ConnectionUsageMember>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ConnectionUsageMember { name, type_reference, multiplicity, connect_from, connect_to, connect_extra_ends, body, subsets, redefines, membership, by_reference } = &$($mutability)? node.value;
            if let Some(inner) = name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = type_reference {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = connect_from {
                visitor.visit_connection_end(inner);
            }
            if let Some(inner) = connect_to {
                visitor.visit_connection_end(inner);
            }
            for inner in connect_extra_ends {
                visitor.visit_connection_end(inner);
            }
            visitor.visit_connection_def_body(body);
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            visitor.visit_membership(membership);
            let _ = by_reference;
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_exhibit_state<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ExhibitState>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ExhibitState { direction, is_derived, is_abstract, is_reference, is_individual, name, state_reference, typing, multiplicity, subsets, redefines, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = direction {
                visitor.visit_in_out_value(inner);
            }
            let _ = is_derived;
            let _ = is_abstract;
            let _ = is_reference;
            let _ = is_individual;
            visitor.visit_text(name);
            if let Some(inner) = state_reference {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = typing {
                visitor.visit_typing_relationship(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            visitor.visit_state_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_attribute_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<AttributeDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let AttributeDef { definition_prefix, name, short_name, typing, multiplicity, value, body, name_span, typing_span, value_span, multiplicity_modifiers, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            visitor.visit_text(name);
            if let Some(inner) = short_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = typing {
                visitor.visit_typing_relationship(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            visitor.visit_attribute_body(body);
            if let Some(inner) = name_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = typing_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = value_span {
                visitor.visit_span(inner);
            }
            visitor.visit_multiplicity_modifiers(multiplicity_modifiers);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_attribute_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? AttributeBody) {
            match node {
                AttributeBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                AttributeBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_attribute_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_attribute_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<AttributeBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                AttributeBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                AttributeBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                AttributeBodyElement::AttributeDef(field_0) => {
                    visitor.visit_attribute_def(field_0);
                }
                AttributeBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(field_0);
                }
                AttributeBodyElement::OccurrenceUsage(field_0) => {
                    visitor.visit_occurrence_usage(&$($mutability)? **field_0);
                }
                AttributeBodyElement::Connect(field_0) => {
                    visitor.visit_connect(field_0);
                }
                AttributeBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                AttributeBodyElement::AssertConstraint(field_0) => {
                    visitor.visit_assert_constraint_member(field_0);
                }
                AttributeBodyElement::RefDecl(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
                AttributeBodyElement::PartUsage(field_0) => {
                    visitor.visit_part_usage(&$($mutability)? **field_0);
                }
                AttributeBodyElement::ItemUsage(field_0) => {
                    visitor.visit_item_usage(&$($mutability)? **field_0);
                }
                AttributeBodyElement::KermlFeature(field_0) => {
                    visitor.visit_kerml_feature_member(&$($mutability)? **field_0);
                }
                AttributeBodyElement::Invariant(field_0) => {
                    visitor.visit_kerml_invariant_member(&$($mutability)? **field_0);
                }
                AttributeBodyElement::KermlConnector(field_0) => {
                    visitor.visit_kerml_connector_member(&$($mutability)? **field_0);
                }
                AttributeBodyElement::KermlClassifier(field_0) => {
                    visitor.visit_kerml_classifier_decl(&$($mutability)? **field_0);
                }
                AttributeBodyElement::Bind(field_0) => {
                    visitor.visit_bind(&$($mutability)? **field_0);
                }
                AttributeBodyElement::Connection(field_0) => {
                    visitor.visit_connection_usage_member(&$($mutability)? **field_0);
                }
                AttributeBodyElement::CalcDef(field_0) => {
                    visitor.visit_calc_def(&$($mutability)? **field_0);
                }
                AttributeBodyElement::CalcUsage(field_0) => {
                    visitor.visit_calc_usage(&$($mutability)? **field_0);
                }
                AttributeBodyElement::ConstraintUsage(field_0) => {
                    visitor.visit_constraint_usage(&$($mutability)? **field_0);
                }
                AttributeBodyElement::Unsupported(field_0) => {
                    visitor.visit_unsupported_grammar_node(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_item_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ItemDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ItemDef { definition_prefix, is_individual, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            let _ = is_individual;
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_attribute_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_individual_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<IndividualDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let IndividualDef { definition_prefix, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_attribute_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_part_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<PartUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let PartUsage { prefix, then_span, name, short_name, typing, multiplicity, multiplicity_modifiers, subsets, redefines, value, body, name_span, type_ref_span, membership } = &$($mutability)? node.value;
            visitor.visit_occurrence_usage_prefix(prefix);
            if let Some(inner) = then_span {
                visitor.visit_span(inner);
            }
            visitor.visit_text(name);
            if let Some(inner) = short_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = typing {
                visitor.visit_typing_relationship(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_multiplicity_modifiers(multiplicity_modifiers);
            if let Some(inner) = subsets {
                let (tuple_0, tuple_1) = inner;
                visitor.visit_subsetting_relationship(tuple_0);
                if let Some(inner) = tuple_1 {
                    visitor.visit_expression(inner);
                }
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            visitor.visit_part_usage_body(body);
            if let Some(inner) = name_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = type_ref_span {
                visitor.visit_span(inner);
            }
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_part_usage_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? PartUsageBody) {
            match node {
                PartUsageBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                PartUsageBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_part_usage_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_metadata_annotation<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<MetadataAnnotation>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let MetadataAnnotation { prefixes, introducer, declared_name, type_reference, type_span, about_targets, body } = &$($mutability)? node.value;
            for inner in prefixes {
                visitor.visit_metadata_keyword_usage(inner);
            }
            match introducer {
                MetadataFeatureIntroducer::At { span } | MetadataFeatureIntroducer::Metadata { span } => visitor.visit_span(span),
            }
            if let Some(inner) = declared_name {
                visitor.visit_metadata_declared_name(inner);
            }
            visitor.visit_qualified_reference(type_reference);
            visitor.visit_span(type_span);
            for inner in about_targets {
                visitor.visit_qualified_reference(inner);
            }
            visitor.visit_attribute_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_metadata_declared_name<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<MetadataDeclaredName>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let MetadataDeclaredName { identification, typed_by, typed_by_span } = &$($mutability)? node.value;
            visitor.visit_identification(identification);
            match typed_by {
                MetadataTypedBy::Colon => {}
                MetadataTypedBy::TypedBy => {}
            }
            visitor.visit_span(typed_by_span);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_metadata_keyword_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<MetadataKeywordUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let MetadataKeywordUsage { hash_span, reference, body } = &$($mutability)? node.value;
            visitor.visit_span(hash_span);
            visitor.visit_qualified_reference(reference);
            if let Some(inner) = body {
                visitor.visit_attribute_body(inner);
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_part_usage_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<PartUsageBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                PartUsageBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                PartUsageBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                PartUsageBodyElement::InOutDecl(field_0) => {
                    visitor.visit_in_out_decl(field_0);
                }
                PartUsageBodyElement::EndDecl(field_0) => {
                    visitor.visit_end_decl(field_0);
                }
                PartUsageBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(field_0);
                }
                PartUsageBodyElement::DefaultReferenceUsage(field_0) => {
                    visitor.visit_default_reference_usage(field_0);
                }
                PartUsageBodyElement::EnumerationUsage(field_0) => {
                    visitor.visit_enumeration_usage(field_0);
                }
                PartUsageBodyElement::PartUsage(field_0) => {
                    visitor.visit_part_usage(&$($mutability)? **field_0);
                }
                PartUsageBodyElement::OccurrenceUsage(field_0) => {
                    visitor.visit_occurrence_usage(&$($mutability)? **field_0);
                }
                PartUsageBodyElement::PortUsage(field_0) => {
                    visitor.visit_port_usage(field_0);
                }
                PartUsageBodyElement::Bind(field_0) => {
                    visitor.visit_bind(field_0);
                }
                PartUsageBodyElement::Ref(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
                PartUsageBodyElement::InterfaceUsage(field_0) => {
                    visitor.visit_interface_usage(field_0);
                }
                PartUsageBodyElement::Connect(field_0) => {
                    visitor.visit_connect(field_0);
                }
                PartUsageBodyElement::FlowUsage(field_0) => {
                    visitor.visit_flow_usage(field_0);
                }
                PartUsageBodyElement::Perform(field_0) => {
                    visitor.visit_perform(field_0);
                }
                PartUsageBodyElement::SuccessionUsage(field_0) => {
                    visitor.visit_succession_usage(field_0);
                }
                PartUsageBodyElement::Allocate(field_0) => {
                    visitor.visit_allocate(field_0);
                }
                PartUsageBodyElement::Satisfy(field_0) => {
                    visitor.visit_satisfy_requirement_usage(&$($mutability)? **field_0);
                }
                PartUsageBodyElement::StateUsage(field_0) => {
                    visitor.visit_state_usage(field_0);
                }
                PartUsageBodyElement::ActionUsage(field_0) => {
                    visitor.visit_action_usage(&$($mutability)? **field_0);
                }
                PartUsageBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                PartUsageBodyElement::VariantUsage(field_0) => {
                    visitor.visit_variant_usage(field_0);
                }
                PartUsageBodyElement::StateDef(field_0) => {
                    visitor.visit_state_def(field_0);
                }
                PartUsageBodyElement::MetadataDef(field_0) => {
                    visitor.visit_metadata_def(field_0);
                }
                PartUsageBodyElement::FlowDef(field_0) => {
                    visitor.visit_flow_def(field_0);
                }
                PartUsageBodyElement::RequirementDef(field_0) => {
                    visitor.visit_requirement_def(field_0);
                }
                PartUsageBodyElement::OccurrenceDef(field_0) => {
                    visitor.visit_occurrence_def(field_0);
                }
                PartUsageBodyElement::PortDef(field_0) => {
                    visitor.visit_port_def(field_0);
                }
                PartUsageBodyElement::CalcDef(field_0) => {
                    visitor.visit_calc_def(field_0);
                }
                PartUsageBodyElement::ConnectionDef(field_0) => {
                    visitor.visit_connection_def(field_0);
                }
                PartUsageBodyElement::EnumDef(field_0) => {
                    visitor.visit_enum_def(field_0);
                }
                PartUsageBodyElement::Connection(field_0) => {
                    visitor.visit_connection_usage_member(field_0);
                }
                PartUsageBodyElement::AssertConstraint(field_0) => {
                    visitor.visit_assert_constraint_member(field_0);
                }
                PartUsageBodyElement::ConstraintDef(field_0) => {
                    visitor.visit_constraint_def(field_0);
                }
                PartUsageBodyElement::ConstraintUsage(field_0) => {
                    visitor.visit_constraint_usage(field_0);
                }
                PartUsageBodyElement::CalcUsage(field_0) => {
                    visitor.visit_calc_usage(field_0);
                }
                PartUsageBodyElement::Import(field_0) => {
                    visitor.visit_import(field_0);
                }
                PartUsageBodyElement::RequirementUsage(field_0) => {
                    visitor.visit_requirement_usage(field_0);
                }
                PartUsageBodyElement::ItemDef(field_0) => {
                    visitor.visit_item_def(field_0);
                }
                PartUsageBodyElement::ItemUsage(field_0) => {
                    visitor.visit_item_usage(field_0);
                }
                PartUsageBodyElement::MetadataUsage(field_0) => {
                    visitor.visit_metadata_usage(field_0);
                }
                PartUsageBodyElement::AnalysisCaseDef(field_0) => {
                    visitor.visit_analysis_case_def(field_0);
                }
                PartUsageBodyElement::AnalysisCaseUsage(field_0) => {
                    visitor.visit_analysis_case_usage(field_0);
                }
                PartUsageBodyElement::AliasDef(field_0) => {
                    visitor.visit_alias_def(field_0);
                }
                PartUsageBodyElement::IncludeUseCase(field_0) => {
                    visitor.visit_include_use_case(field_0);
                }
                PartUsageBodyElement::UseCaseUsage(field_0) => {
                    visitor.visit_use_case_usage(field_0);
                }
                PartUsageBodyElement::VerificationCaseUsage(field_0) => {
                    visitor.visit_verification_case_usage(field_0);
                }
                PartUsageBodyElement::ViewDef(field_0) => {
                    visitor.visit_view_def(field_0);
                }
                PartUsageBodyElement::ViewUsage(field_0) => {
                    visitor.visit_view_usage(field_0);
                }
                PartUsageBodyElement::ViewpointDef(field_0) => {
                    visitor.visit_viewpoint_def(field_0);
                }
                PartUsageBodyElement::ViewpointUsage(field_0) => {
                    visitor.visit_viewpoint_usage(field_0);
                }
                PartUsageBodyElement::RenderingDef(field_0) => {
                    visitor.visit_rendering_def(field_0);
                }
                PartUsageBodyElement::RenderingUsage(field_0) => {
                    visitor.visit_rendering_usage(field_0);
                }
                PartUsageBodyElement::KermlClassifier(field_0) => {
                    visitor.visit_kerml_classifier_decl(&$($mutability)? **field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_variant_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<VariantUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let VariantUsage { reference, typed, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = reference {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = typed {
                visitor.visit_variant_typed_usage(inner);
            }
            if let Some(inner) = body {
                visitor.visit_part_usage_body(inner);
            }
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_variant_typed_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? VariantTypedUsage) {
            match node {
                VariantTypedUsage::Part(field_0) => {
                    visitor.visit_part_usage(&$($mutability)? **field_0);
                }
                VariantTypedUsage::Attribute(field_0) => {
                    visitor.visit_attribute_usage(&$($mutability)? **field_0);
                }
                VariantTypedUsage::Item(field_0) => {
                    visitor.visit_item_usage(&$($mutability)? **field_0);
                }
                VariantTypedUsage::Port(field_0) => {
                    visitor.visit_port_usage(&$($mutability)? **field_0);
                }
                VariantTypedUsage::Perform(field_0) => {
                    visitor.visit_perform(&$($mutability)? **field_0);
                }
                VariantTypedUsage::Requirement(field_0) => {
                    visitor.visit_requirement_usage(&$($mutability)? **field_0);
                }
            }
        }

        pub fn walk_perform<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<Perform>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let Perform { usage_prefix, action_name, action_reference, typing, multiplicity, redefines, subsets, value, body } = &$($mutability)? node.value;
            if let Some(inner) = usage_prefix {
                visitor.visit_definition_prefix_value(inner);
            }
            visitor.visit_text(action_name);
            if let Some(inner) = action_reference {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = typing {
                visitor.visit_typing_relationship(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            visitor.visit_perform_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_perform_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? PerformBody) {
            match node {
                PerformBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                PerformBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_perform_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_perform_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<PerformBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                PerformBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                PerformBodyElement::InOut(field_0) => {
                    visitor.visit_perform_in_out_binding(field_0);
                }
                PerformBodyElement::Variant(field_0) => {
                    visitor.visit_variant_usage(field_0);
                }
                PerformBodyElement::Action(field_0) => {
                    visitor.visit_action_usage_body_element(&$($mutability)? **field_0);
                }
                PerformBodyElement::PartUsage(field_0) => {
                    visitor.visit_part_usage(&$($mutability)? **field_0);
                }
                PerformBodyElement::ItemUsage(field_0) => {
                    visitor.visit_item_usage(&$($mutability)? **field_0);
                }
                PerformBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(&$($mutability)? **field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_perform_in_out_binding<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<PerformInOutBinding>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let PerformInOutBinding { direction, target, value } = &$($mutability)? node.value;
            visitor.visit_in_out_value(direction);
            visitor.visit_qualified_reference(target);
            visitor.visit_expression(value);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_attribute_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<AttributeUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let AttributeUsage { name, short_name, typing, subsets, redefines, references, crosses, intersects, value, body, name_span, typing_span, redefines_span, direction, multiplicity, multiplicity_modifiers, is_derived, usage_prefix, is_constant, is_reference, is_end, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = short_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = typing {
                visitor.visit_typing_relationship(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = references {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = crosses {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = intersects {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            visitor.visit_attribute_body(body);
            if let Some(inner) = name_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = typing_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = redefines_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = direction {
                visitor.visit_in_out_value(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_multiplicity_modifiers(multiplicity_modifiers);
            let _ = is_derived;
            if let Some(inner) = usage_prefix {
                visitor.visit_definition_prefix_value(inner);
            }
            let _ = is_constant;
            let _ = is_reference;
            let _ = is_end;
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_default_reference_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<DefaultReferenceUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let DefaultReferenceUsage { name, typing, subsets, redefines, value, multiplicity, name_span, typing_span, membership, has_feature_keyword, body } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = typing {
                visitor.visit_typing_relationship(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = name_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = typing_span {
                visitor.visit_span(inner);
            }
            visitor.visit_membership(membership);
            let _ = has_feature_keyword;
            if let Some(inner) = body {
                for inner in inner {
                    visitor.visit_feature_body_element(inner);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_feature_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FeatureBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                FeatureBodyElement::Binding(field_0) => {
                    visitor.visit_default_reference_usage(&$($mutability)? **field_0);
                }
                FeatureBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_port_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<PortDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let PortDef { definition_prefix, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_port_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_port_def_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? PortDefBody) {
            match node {
                PortDefBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                PortDefBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_port_def_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_port_def_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<PortDefBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                PortDefBodyElement::InOutDecl(field_0) => {
                    visitor.visit_in_out_decl(field_0);
                }
                PortDefBodyElement::RefDecl(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
                PortDefBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                PortDefBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                PortDefBodyElement::AttributeDef(field_0) => {
                    visitor.visit_attribute_def(field_0);
                }
                PortDefBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(field_0);
                }
                PortDefBodyElement::ItemDef(field_0) => {
                    visitor.visit_item_def(field_0);
                }
                PortDefBodyElement::ItemUsage(field_0) => {
                    visitor.visit_item_usage(field_0);
                }
                PortDefBodyElement::EnumerationUsage(field_0) => {
                    visitor.visit_enumeration_usage(field_0);
                }
                PortDefBodyElement::PortUsage(field_0) => {
                    visitor.visit_port_usage(field_0);
                }
                PortDefBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                PortDefBodyElement::Unsupported(field_0) => {
                    visitor.visit_unsupported_grammar_node(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_port_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<PortUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let PortUsage { prefix, name, short_name, typing, multiplicity, multiplicity_modifiers, subsets, redefines, references, crosses, intersects, value, body, name_span, type_ref_span, membership } = &$($mutability)? node.value;
            visitor.visit_occurrence_usage_prefix(prefix);
            visitor.visit_text(name);
            if let Some(inner) = short_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = typing {
                visitor.visit_typing_relationship(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_multiplicity_modifiers(multiplicity_modifiers);
            if let Some(inner) = subsets {
                let (tuple_0, tuple_1) = inner;
                visitor.visit_subsetting_relationship(tuple_0);
                if let Some(inner) = tuple_1 {
                    visitor.visit_expression(inner);
                }
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = references {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = crosses {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = intersects {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            visitor.visit_port_body(body);
            if let Some(inner) = name_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = type_ref_span {
                visitor.visit_span(inner);
            }
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_port_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? PortBody) {
            match node {
                PortBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                PortBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_port_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_port_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<PortBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                PortBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                PortBodyElement::InOutDecl(field_0) => {
                    visitor.visit_in_out_decl(field_0);
                }
                PortBodyElement::PortUsage(field_0) => {
                    visitor.visit_port_usage(field_0);
                }
                PortBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                PortBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(field_0);
                }
                PortBodyElement::ItemUsage(field_0) => {
                    visitor.visit_item_usage(field_0);
                }
                PortBodyElement::RefDecl(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_connect_stmt<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ConnectStmt>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ConnectStmt { from, to, extra_ends, body } = &$($mutability)? node.value;
            visitor.visit_connection_end(from);
            visitor.visit_connection_end(to);
            for inner in extra_ends {
                visitor.visit_connection_end(inner);
            }
            match body {
                PartUsageBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                PartUsageBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_part_usage_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_interface_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<InterfaceDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let InterfaceDef { definition_prefix: _, is_individual: _, identification, specializes, body, membership } = &$($mutability)? node.value;
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_interface_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_interface_def_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? InterfaceDefBody) {
            match node {
                InterfaceDefBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                InterfaceDefBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_interface_def_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_interface_def_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<InterfaceDefBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                InterfaceDefBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                InterfaceDefBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                InterfaceDefBodyElement::EndDecl(field_0) => {
                    visitor.visit_end_decl(field_0);
                }
                InterfaceDefBodyElement::RefDecl(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
                InterfaceDefBodyElement::ConnectStmt(field_0) => {
                    visitor.visit_connect_stmt(field_0);
                }
                InterfaceDefBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                InterfaceDefBodyElement::AttributeDef(field_0) => {
                    visitor.visit_attribute_def(field_0);
                }
                InterfaceDefBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(field_0);
                }
                InterfaceDefBodyElement::ItemDef(field_0) => {
                    visitor.visit_item_def(field_0);
                }
                InterfaceDefBodyElement::ItemUsage(field_0) => {
                    visitor.visit_item_usage(field_0);
                }
                InterfaceDefBodyElement::PortDef(field_0) => {
                    visitor.visit_port_def(field_0);
                }
                InterfaceDefBodyElement::PortUsage(field_0) => {
                    visitor.visit_port_usage(field_0);
                }
                InterfaceDefBodyElement::FlowUsage(field_0) => {
                    visitor.visit_flow_usage(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_end_nested_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? EndNestedUsage) {
            match node {
                EndNestedUsage::Occurrence(field_0) => {
                    visitor.visit_occurrence_usage(&$($mutability)? **field_0);
                }
                EndNestedUsage::Item(field_0) => {
                    visitor.visit_item_usage(&$($mutability)? **field_0);
                }
            }
        }

        pub fn walk_end_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<EndDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let EndDecl { short_name, identity, typing, references, multiplicity, redefines, crosses, nested_usage, type_ref_span } = &$($mutability)? node.value;
            if let Some(inner) = short_name { visitor.visit_text(inner); }
            visitor.visit_end_identity(identity);
            if let Some(inner) = typing {
                visitor.visit_typing_relationship(inner);
            }
            if let Some(inner) = references {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = crosses {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = nested_usage {
                visitor.visit_end_nested_usage(&$($mutability)? **inner);
            }
            if let Some(inner) = type_ref_span {
                visitor.visit_span(inner);
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_end_identity<V: $Visitor>(visitor: &mut V, node: &$($mutability)? EndIdentity) {
            match node {
                EndIdentity::Declaration(field_0) => {
                    visitor.visit_span(&$($mutability)? field_0.span);
                    visitor.visit_text(&$($mutability)? field_0.value);
                }
                EndIdentity::Derivation(field_0) => {
                    visitor.visit_derivation_end_role(field_0);
                }
            }
        }

        pub fn walk_derivation_end_role<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<DerivationEndRole>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                DerivationEndRole::Original => {}
                DerivationEndRole::Derive => {}
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_ref_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<RefDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let RefDecl { direction, is_derived, usage_prefix, is_constant, kind_keyword, name, short_name, typing, redefines, subsets, multiplicity, multiplicity_modifiers, value, body, name_span, type_ref_span, membership } = &$($mutability)? node.value;
            if let Some(inner) = direction {
                visitor.visit_in_out_value(inner);
            }
            let _ = is_derived;
            if let Some(inner) = usage_prefix {
                visitor.visit_definition_prefix_value(inner);
            }
            let _ = is_constant;
            if let Some(inner) = kind_keyword {
                visitor.visit_ref_decl_kind(inner);
            }
            visitor.visit_text(name);
            if let Some(inner) = short_name { visitor.visit_text(inner); }
            if let Some(inner) = typing {
                visitor.visit_typing_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_multiplicity_modifiers(multiplicity_modifiers);
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            visitor.visit_ref_body(body);
            if let Some(inner) = name_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = type_ref_span {
                visitor.visit_span(inner);
            }
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_ref_decl_kind<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? RefDeclKind) {
            match node {
                RefDeclKind::Part => {}
                RefDeclKind::Port => {}
                RefDeclKind::Item => {}
                RefDeclKind::Requirement => {}
                RefDeclKind::UseCase => {}
                RefDeclKind::Concern => {}
                RefDeclKind::Viewpoint => {}
                RefDeclKind::Rendering => {}
                RefDeclKind::View => {}
                RefDeclKind::Action => {}
                RefDeclKind::Case => {}
                RefDeclKind::Verification => {}
            }
        }

        pub fn walk_ref_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? RefBody) {
            match node {
                RefBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                RefBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_part_usage_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_relationship_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<RelationshipBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                RelationshipBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                RelationshipBodyElement::KermlFeature(field_0) => {
                    visitor.visit_kerml_feature_member(&$($mutability)? **field_0);
                }
                RelationshipBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_derivation_connection_role<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<DerivationConnectionRole>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                DerivationConnectionRole::Derivation => {}
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_connection_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ConnectionDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ConnectionDef { definition_prefix: _, is_individual, derivation_role, identification, specializes, body, membership } = &$($mutability)? node.value;
            let _ = is_individual;
            if let Some(inner) = derivation_role {
                visitor.visit_derivation_connection_role(inner);
            }
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_connection_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_connection_def_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? ConnectionDefBody) {
            match node {
                ConnectionDefBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                ConnectionDefBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_connection_def_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_connection_def_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ConnectionDefBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                ConnectionDefBodyElement::EndDecl(field_0) => {
                    visitor.visit_end_decl(field_0);
                }
                ConnectionDefBodyElement::RefDecl(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
                ConnectionDefBodyElement::ConnectStmt(field_0) => {
                    visitor.visit_connect_stmt(field_0);
                }
                ConnectionDefBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                ConnectionDefBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                ConnectionDefBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                ConnectionDefBodyElement::AttributeDef(field_0) => {
                    visitor.visit_attribute_def(field_0);
                }
                ConnectionDefBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(field_0);
                }
                ConnectionDefBodyElement::ItemDef(field_0) => {
                    visitor.visit_item_def(field_0);
                }
                ConnectionDefBodyElement::ItemUsage(field_0) => {
                    visitor.visit_item_usage(field_0);
                }
                ConnectionDefBodyElement::PortDef(field_0) => {
                    visitor.visit_port_def(field_0);
                }
                ConnectionDefBodyElement::PortUsage(field_0) => {
                    visitor.visit_port_usage(field_0);
                }
                ConnectionDefBodyElement::AssertConstraint(field_0) => {
                    visitor.visit_assert_constraint_member(field_0);
                }
                ConnectionDefBodyElement::OccurrenceUsage(field_0) => {
                    visitor.visit_occurrence_usage(&$($mutability)? **field_0);
                }
                ConnectionDefBodyElement::SuccessionUsage(field_0) => {
                    visitor.visit_succession_usage(field_0);
                }
                ConnectionDefBodyElement::PartUsage(field_0) => {
                    visitor.visit_part_usage(&$($mutability)? **field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_metadata_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<MetadataDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let MetadataDef { is_abstract, identification, specializes, body, membership } = &$($mutability)? node.value;
            let _ = is_abstract;
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_attribute_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_metadata_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<MetadataUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let MetadataUsage { name, type_reference, about_targets, body, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = type_reference {
                visitor.visit_qualified_reference(inner);
            }
            for inner in about_targets {
                visitor.visit_qualified_reference(inner);
            }
            visitor.visit_attribute_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_enum_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<EnumDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let EnumDef { identification, specializes, body, membership } = &$($mutability)? node.value;
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_enumeration_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_enumeration_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? EnumerationBody) {
            match node {
                EnumerationBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                EnumerationBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_enumeration_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_enumeration_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<EnumerationBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                EnumerationBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                EnumerationBodyElement::Value(field_0) => {
                    visitor.visit_enumerated_value(field_0);
                }
                EnumerationBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_enumerated_value<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<EnumeratedValue>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let EnumeratedValue { name, short_name, value, body, name_span } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = short_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            visitor.visit_part_usage_body(body);
            if let Some(inner) = name_span {
                visitor.visit_span(inner);
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_occurrence_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<OccurrenceDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let OccurrenceDef { definition_prefix, is_individual, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            let _ = is_individual;
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_definition_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_occurrence_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<OccurrenceUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let OccurrenceUsage { prefix, then_span, is_event, has_occurrence_keyword, name, short_name, occurrence_reference, type_name, type_is_conjugated, multiplicity, subsets, redefines, references, crosses, intersects, value, body, membership } = &$($mutability)? node.value;
            visitor.visit_occurrence_usage_prefix(prefix);
            if let Some(inner) = then_span {
                visitor.visit_span(inner);
            }
            let _ = is_event;
            let _ = has_occurrence_keyword;
            visitor.visit_text(name);
            if let Some(inner) = short_name { visitor.visit_text(inner); }
            if let Some(inner) = occurrence_reference {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            let _ = type_is_conjugated;
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = references {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = crosses {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = intersects {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            visitor.visit_occurrence_usage_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_occurrence_portion_kind<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<OccurrencePortionKind>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            visitor.visit_occurrence_portion_kind_value(&$($mutability)? node.value);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_occurrence_portion_kind_value<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? OccurrencePortionKind) {
            match node {
                OccurrencePortionKind::Snapshot => {}
                OccurrencePortionKind::Timeslice => {}
            }
        }

        pub fn walk_multiplicity_ordering<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<MultiplicityOrdering>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            visitor.visit_multiplicity_ordering_value(&$($mutability)? node.value);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_multiplicity_ordering_value<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? MultiplicityOrdering) {
            match node {
                MultiplicityOrdering::Ordered => {}
                MultiplicityOrdering::Nonordered => {}
            }
        }

        pub fn walk_multiplicity_uniqueness<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<MultiplicityUniqueness>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            visitor.visit_multiplicity_uniqueness_value(&$($mutability)? node.value);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_multiplicity_uniqueness_value<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? MultiplicityUniqueness) {
            match node {
                MultiplicityUniqueness::Unique => {}
                MultiplicityUniqueness::Nonunique => {}
            }
        }

        pub fn walk_multiplicity_modifiers<V: $Visitor>(visitor: &mut V, node: &$($mutability)? MultiplicityModifiers) {
            let MultiplicityModifiers { ordering, uniqueness } = node;
            if let Some(inner) = ordering {
                visitor.visit_multiplicity_ordering(inner);
            }
            if let Some(inner) = uniqueness {
                visitor.visit_multiplicity_uniqueness(inner);
            }
        }

        pub fn walk_ref_prefix<V: $Visitor>(visitor: &mut V, node: &$($mutability)? RefPrefix) {
            let RefPrefix { direction, derived_span, variance, constant_span } = node;
            if let Some(inner) = direction {
                visitor.visit_in_out(inner);
            }
            if let Some(inner) = derived_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = variance {
                visitor.visit_definition_prefix(inner);
            }
            if let Some(inner) = constant_span {
                visitor.visit_span(inner);
            }
        }

        pub fn walk_basic_usage_prefix<V: $Visitor>(visitor: &mut V, node: &$($mutability)? BasicUsagePrefix) {
            let BasicUsagePrefix { ref_prefix, reference_span } = node;
            visitor.visit_ref_prefix(ref_prefix);
            if let Some(inner) = reference_span {
                visitor.visit_span(inner);
            }
        }

        pub fn walk_usage_extension_keyword<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<UsageExtensionKeyword>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let UsageExtensionKeyword { hash_span, annotation } = &$($mutability)? node.value;
            visitor.visit_span(hash_span);
            visitor.visit_qualified_reference(annotation);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_occurrence_usage_prefix<V: $Visitor>(visitor: &mut V, node: &$($mutability)? OccurrenceUsagePrefix) {
            let OccurrenceUsagePrefix { basic, individual_span, portion, extension_keywords } = node;
            visitor.visit_basic_usage_prefix(basic);
            if let Some(inner) = individual_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = portion {
                visitor.visit_occurrence_portion_kind(inner);
            }
            for inner in extension_keywords {
                visitor.visit_usage_extension_keyword(inner);
            }
        }

        pub fn walk_feature_portion_kind<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FeaturePortionKind>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            visitor.visit_feature_portion_kind_value(&$($mutability)? node.value);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_feature_portion_kind_value<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? FeaturePortionKind) {
            match node {
                FeaturePortionKind::Composite => {}
                FeaturePortionKind::Portion => {}
            }
        }

        pub fn walk_feature_variability<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FeatureVariability>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            visitor.visit_feature_variability_value(&$($mutability)? node.value);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_feature_variability_value<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? FeatureVariability) {
            match node {
                FeatureVariability::Var => {}
                FeatureVariability::Const => {}
            }
        }

        pub fn walk_basic_feature_prefix<V: $Visitor>(visitor: &mut V, node: &$($mutability)? BasicFeaturePrefix) {
            let BasicFeaturePrefix { direction, derived_span, abstract_span, portioning, variability } = node;
            if let Some(inner) = direction {
                visitor.visit_in_out(inner);
            }
            if let Some(inner) = derived_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = abstract_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = portioning {
                visitor.visit_feature_portion_kind(inner);
            }
            if let Some(inner) = variability {
                visitor.visit_feature_variability(inner);
            }
        }

        pub fn walk_end_feature_prefix<V: $Visitor>(visitor: &mut V, node: &$($mutability)? EndFeaturePrefix) {
            let EndFeaturePrefix { constant_span, end_span } = node;
            if let Some(inner) = constant_span {
                visitor.visit_span(inner);
            }
            visitor.visit_span(end_span);
        }

        pub fn walk_owned_cross_feature<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<OwnedCrossFeature>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let OwnedCrossFeature { prefix, name, multiplicity, multiplicity_modifiers, subsets } = &$($mutability)? node.value;
            visitor.visit_basic_feature_prefix(prefix);
            visitor.visit_text(name);
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_multiplicity_modifiers(multiplicity_modifiers);
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_feature_prefix_head<V: $Visitor>(visitor: &mut V, node: &$($mutability)? FeaturePrefixHead) {
            match node {
                FeaturePrefixHead::End { prefix, cross } => {
                    visitor.visit_end_feature_prefix(prefix);
                    if let Some(inner) = cross {
                        visitor.visit_owned_cross_feature(inner);
                    }
                }
                FeaturePrefixHead::Basic(field_0) => {
                    visitor.visit_basic_feature_prefix(field_0);
                }
            }
        }

        pub fn walk_feature_prefix<V: $Visitor>(visitor: &mut V, node: &$($mutability)? FeaturePrefix) {
            let FeaturePrefix { head, metadata_keywords } = node;
            visitor.visit_feature_prefix_head(head);
            for inner in metadata_keywords {
                visitor.visit_usage_extension_keyword(inner);
            }
        }

        pub fn walk_occurrence_usage_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? OccurrenceUsageBody) {
            match node {
                OccurrenceUsageBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                OccurrenceUsageBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_occurrence_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_assert_constraint_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<AssertConstraintMember>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let AssertConstraintMember { declaration_name, target, type_name, body, is_negated, membership } = &$($mutability)? node.value;
            if let Some(inner) = declaration_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = target {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            visitor.visit_constraint_def_body(body);
            let _ = is_negated;
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_occurrence_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<OccurrenceBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                OccurrenceBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                OccurrenceBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                OccurrenceBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                OccurrenceBodyElement::AssertConstraint(field_0) => {
                    visitor.visit_assert_constraint_member(field_0);
                }
                OccurrenceBodyElement::FlowUsage(field_0) => {
                    visitor.visit_flow_usage(field_0);
                }
                OccurrenceBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(field_0);
                }
                OccurrenceBodyElement::PartUsage(field_0) => {
                    visitor.visit_part_usage(&$($mutability)? **field_0);
                }
                OccurrenceBodyElement::ItemUsage(field_0) => {
                    visitor.visit_item_usage(field_0);
                }
                OccurrenceBodyElement::OccurrenceUsage(field_0) => {
                    visitor.visit_occurrence_usage(&$($mutability)? **field_0);
                }
                OccurrenceBodyElement::SuccessionUsage(field_0) => {
                    visitor.visit_succession_usage(field_0);
                }
                OccurrenceBodyElement::Satisfy(field_0) => {
                    visitor.visit_satisfy_requirement_usage(&$($mutability)? **field_0);
                }
                OccurrenceBodyElement::Allocate(field_0) => {
                    visitor.visit_allocate(field_0);
                }
                OccurrenceBodyElement::EndDecl(field_0) => {
                    visitor.visit_end_decl(field_0);
                }
                OccurrenceBodyElement::StateUsage(field_0) => {
                    visitor.visit_state_usage(field_0);
                }
                OccurrenceBodyElement::RefDecl(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
                OccurrenceBodyElement::ConnectionUsage(field_0) => {
                    visitor.visit_connection_usage_member(&$($mutability)? **field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_succession_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<SuccessionUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let SuccessionUsage { name, type_name, multiplicity, source, source_multiplicity, target, target_multiplicity, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_expression(source);
            if let Some(inner) = source_multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_expression(target);
            if let Some(inner) = target_multiplicity {
                visitor.visit_multiplicity(inner);
            }
            walk_part_usage_body(visitor, body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_definition_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? DefinitionBody) {
            match node {
                DefinitionBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                DefinitionBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_definition_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_definition_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<DefinitionBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                DefinitionBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                DefinitionBodyElement::OccurrenceMember(field_0) => {
                    visitor.visit_occurrence_body_element(field_0);
                }
                DefinitionBodyElement::Unsupported(field_0) => {
                    visitor.visit_unsupported_grammar_node(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_bind<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<Bind>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let Bind { binding_name, binding_type, binding_multiplicity, left, left_multiplicity, right, right_multiplicity, body } = &$($mutability)? node.value;
            if let Some(inner) = binding_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = binding_type {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = binding_multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_expression(left);
            if let Some(inner) = left_multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_expression(right);
            if let Some(inner) = right_multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_part_usage_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_interface_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<InterfaceUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                InterfaceUsage::TypedConnect { name, interface_type, subsets, redefines, from, to, body } => {
                    if let Some(inner) = name {
                        visitor.visit_text(inner);
                    }
                    if let Some(inner) = interface_type {
                        visitor.visit_qualified_reference(inner);
                    }
                    if let Some(inner) = subsets {
                        visitor.visit_subsetting_relationship(inner);
                    }
                    if let Some(inner) = redefines {
                        visitor.visit_subsetting_relationship(inner);
                    }
                    visitor.visit_expression(from);
                    visitor.visit_expression(to);
                    walk_interface_usage_body(visitor, body);
                }
                InterfaceUsage::Connection { subsets, redefines, from, to, body } => {
                    if let Some(inner) = subsets {
                        visitor.visit_subsetting_relationship(inner);
                    }
                    if let Some(inner) = redefines {
                        visitor.visit_subsetting_relationship(inner);
                    }
                    visitor.visit_expression(from);
                    visitor.visit_expression(to);
                    walk_interface_usage_body(visitor, body);
                }
                InterfaceUsage::Declaration { name, interface_type, subsets, redefines, body } => {
                    if let Some(inner) = name {
                        visitor.visit_text(inner);
                    }
                    if let Some(inner) = interface_type {
                        visitor.visit_qualified_reference(inner);
                    }
                    if let Some(inner) = subsets {
                        visitor.visit_subsetting_relationship(inner);
                    }
                    if let Some(inner) = redefines {
                        visitor.visit_subsetting_relationship(inner);
                    }
                    walk_interface_usage_body(visitor, body);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_interface_usage_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<InterfaceUsageBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                InterfaceUsageBodyElement::RefRedef { target, value, body } => {
                    visitor.visit_qualified_reference(target);
                    visitor.visit_expression(value);
                    visitor.visit_ref_body(body);
                }
                InterfaceUsageBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                InterfaceUsageBodyElement::EndDecl(field_0) => {
                    visitor.visit_end_decl(&$($mutability)? **field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_connect<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<Connect>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let Connect { from, to, body, subsets, redefines } = &$($mutability)? node.value;
            visitor.visit_connection_end(from);
            visitor.visit_connection_end(to);
            walk_part_usage_body(visitor, body);
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_binding_connector_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<BindingConnectorUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let BindingConnectorUsage { all, name_span, multiplicity, uses_of_keyword, uses_bind_keyword, left, right, body } = &$($mutability)? node.value;
            let _ = all;
            if let Some(inner) = name_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            let _ = uses_of_keyword;
            let _ = uses_bind_keyword;
            visitor.visit_qualified_reference(left);
            visitor.visit_qualified_reference(right);
            walk_part_usage_body(visitor, body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_alias_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<AliasDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let AliasDef { identification, target, body, membership } = &$($mutability)? node.value;
            visitor.visit_identification(identification);
            visitor.visit_qualified_reference(target);
            visitor.visit_alias_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_alias_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? AliasBody) {
            match node {
                AliasBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                AliasBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_relationship_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_action_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ActionDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ActionDef { definition_prefix, is_individual, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            let _ = is_individual;
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_action_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_action_branch_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? ActionBranchBody) {
            match node {
                ActionBranchBody::Braced(field_0) => {
                    visitor.visit_action_def_body(field_0);
                }
                ActionBranchBody::Shorthand(field_0) => {
                    visitor.visit_action_def_body_element(&$($mutability)? **field_0);
                }
            }
        }

        pub fn walk_action_def_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? ActionDefBody) {
            match node {
                ActionDefBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                ActionDefBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_action_def_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_action_def_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ActionDefBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                ActionDefBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                ActionDefBodyElement::InOutDecl(field_0) => {
                    visitor.visit_in_out_decl(field_0);
                }
                ActionDefBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                ActionDefBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                ActionDefBodyElement::Dependency(field_0) => {
                    visitor.visit_dependency(field_0);
                }
                ActionDefBodyElement::MetadataUsage(field_0) => {
                    visitor.visit_metadata_usage(field_0);
                }
                ActionDefBodyElement::RefDecl(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
                ActionDefBodyElement::Perform(field_0) => {
                    visitor.visit_perform(field_0);
                }
                ActionDefBodyElement::Bind(field_0) => {
                    visitor.visit_bind(field_0);
                }
                ActionDefBodyElement::FlowUsage(field_0) => {
                    visitor.visit_flow_usage(field_0);
                }
                ActionDefBodyElement::FirstStmt(field_0) => {
                    visitor.visit_first_stmt(field_0);
                }
                ActionDefBodyElement::MergeStmt(field_0) => {
                    visitor.visit_merge_stmt(field_0);
                }
                ActionDefBodyElement::DecisionStmt(field_0) => {
                    visitor.visit_decision_stmt(field_0);
                }
                ActionDefBodyElement::JoinStmt(field_0) => {
                    visitor.visit_join_stmt(field_0);
                }
                ActionDefBodyElement::ForkStmt(field_0) => {
                    visitor.visit_fork_stmt(field_0);
                }
                ActionDefBodyElement::TerminateStmt(field_0) => {
                    visitor.visit_terminate_stmt(field_0);
                }
                ActionDefBodyElement::WhileStmt(field_0) => {
                    visitor.visit_while_stmt(field_0);
                }
                ActionDefBodyElement::LoopStmt(field_0) => {
                    visitor.visit_loop_stmt(field_0);
                }
                ActionDefBodyElement::IfStmt(field_0) => {
                    visitor.visit_if_stmt(field_0);
                }
                ActionDefBodyElement::StateUsage(field_0) => {
                    visitor.visit_state_usage(field_0);
                }
                ActionDefBodyElement::ActionUsage(field_0) => {
                    visitor.visit_action_usage(&$($mutability)? **field_0);
                }
                ActionDefBodyElement::PartUsage(field_0) => {
                    visitor.visit_part_usage(&$($mutability)? **field_0);
                }
                ActionDefBodyElement::ItemUsage(field_0) => {
                    visitor.visit_item_usage(field_0);
                }
                ActionDefBodyElement::AssertConstraint(field_0) => {
                    visitor.visit_assert_constraint_member(field_0);
                }
                ActionDefBodyElement::OccurrenceUsage(field_0) => {
                    visitor.visit_occurrence_usage(&$($mutability)? **field_0);
                }
                ActionDefBodyElement::Assign(field_0) => {
                    visitor.visit_assign_stmt(field_0);
                }
                ActionDefBodyElement::ForLoop(field_0) => {
                    visitor.visit_for_loop(field_0);
                }
                ActionDefBodyElement::ThenAction(field_0) => {
                    visitor.visit_then_action(field_0);
                }
                ActionDefBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(&$($mutability)? **field_0);
                }
                ActionDefBodyElement::CalcUsage(field_0) => {
                    visitor.visit_calc_usage(&$($mutability)? **field_0);
                }
                ActionDefBodyElement::ActionDef(field_0) => {
                    visitor.visit_action_def(&$($mutability)? **field_0);
                }
                ActionDefBodyElement::DefaultReferenceUsage(field_0) => {
                    visitor.visit_default_reference_usage(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_assign_stmt<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<AssignStmt>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let AssignStmt { is_then, lhs, rhs } = &$($mutability)? node.value;
            let _ = is_then;
            visitor.visit_expression(lhs);
            visitor.visit_expression(rhs);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_for_loop<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ForLoop>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ForLoop { var, range, body } = &$($mutability)? node.value;
            visitor.visit_text(var);
            visitor.visit_expression(range);
            visitor.visit_action_def_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_then_action<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ThenAction>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ThenAction { target } = &$($mutability)? node.value;
            visitor.visit_then_target(target);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_then_target<V: $Visitor>(visitor: &mut V, node: &$($mutability)? ThenTarget) {
            match node {
                ThenTarget::Action(field_0) => {
                    visitor.visit_action_usage(&$($mutability)? **field_0);
                }
                ThenTarget::Perform(field_0) => {
                    visitor.visit_perform(&$($mutability)? **field_0);
                }
                ThenTarget::Merge(field_0) => {
                    visitor.visit_merge_stmt(field_0);
                }
                ThenTarget::Fork(field_0) => {
                    visitor.visit_fork_stmt(field_0);
                }
                ThenTarget::Decide(field_0) => {
                    visitor.visit_decision_stmt(field_0);
                }
                ThenTarget::Accept(field_0) => {
                    visitor.visit_transition_accept(field_0);
                }
                ThenTarget::Send(field_0) => {
                    visitor.visit_action_usage(&$($mutability)? **field_0);
                }
                ThenTarget::Feature(field_0) => {
                    visitor.visit_expression(field_0);
                }
            }
        }

        pub fn walk_in_out_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<InOutDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let InOutDecl { direction, is_reference, is_var, name, subsets, type_name, multiplicity, multiplicity_modifiers, redefines, value, body } = &$($mutability)? node.value;
            visitor.visit_in_out_value(direction);
            let _ = is_reference;
            let _ = is_var;
            visitor.visit_text(name);
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_multiplicity_modifiers(multiplicity_modifiers);
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            if let Some(inner) = body {
                for inner in inner {
                    visitor.visit_action_def_body_element(inner);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_in_out<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<InOut>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            visitor.visit_in_out_value(&$($mutability)? node.value);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_in_out_value<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? InOut) {
            match node {
                InOut::In => {}
                InOut::Out => {}
                InOut::InOut => {}
            }
        }

        pub fn walk_payload_clause<V: $Visitor>(visitor: &mut V, node: &$($mutability)? PayloadClause) {
            let PayloadClause { name, type_name, name_span, type_span } = node;
            visitor.visit_text(name);
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            visitor.visit_span(name_span);
            if let Some(inner) = type_span {
                visitor.visit_span(inner);
            }
        }

        pub fn walk_send_payload<V: $Visitor>(visitor: &mut V, node: &$($mutability)? SendPayload) {
            match node {
                SendPayload::Typed(field_0) => {
                    visitor.visit_payload_clause(field_0);
                }
                SendPayload::Expression(field_0) => {
                    visitor.visit_expression(field_0);
                }
            }
        }

        pub fn walk_transition_accept<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<TransitionAccept>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            visitor.visit_transition_accept_value(&$($mutability)? node.value);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_transition_accept_value<V: $Visitor>(visitor: &mut V, node: &$($mutability)? TransitionAccept) {
            match node {
                TransitionAccept::Payload(field_0, field_1) => {
                    visitor.visit_payload_clause(field_0);
                    if let Some(inner) = field_1 {
                        visitor.visit_expression(inner);
                    }
                }
                TransitionAccept::Shorthand(field_0, field_1) => {
                    visitor.visit_expression(field_0);
                    if let Some(inner) = field_1 {
                        visitor.visit_expression(inner);
                    }
                }
                TransitionAccept::TimeTrigger(field_0, field_1) => {
                    visitor.visit_trigger_kind(field_0);
                    visitor.visit_expression(field_1);
                }
            }
        }

        pub fn walk_trigger_kind<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? TriggerKind) {
            match node {
                TriggerKind::At => {}
                TriggerKind::When => {}
                TriggerKind::After => {}
            }
        }

        pub fn walk_transition_effect<V: $Visitor>(visitor: &mut V, node: &$($mutability)? TransitionEffect) {
            match node {
                TransitionEffect::Perform { name, type_name } => {
                    if let Some(inner) = name {
                        visitor.visit_text(inner);
                    }
                    if let Some(inner) = type_name {
                        visitor.visit_qualified_reference(inner);
                    }
                }
                TransitionEffect::Accept { payload, type_name, via } => {
                    visitor.visit_expression(payload);
                    if let Some(inner) = type_name {
                        visitor.visit_qualified_reference(inner);
                    }
                    if let Some(inner) = via {
                        visitor.visit_expression(inner);
                    }
                }
                TransitionEffect::Send { payload, type_name, via, to } => {
                    visitor.visit_expression(payload);
                    if let Some(inner) = type_name {
                        visitor.visit_qualified_reference(inner);
                    }
                    if let Some(inner) = via {
                        visitor.visit_expression(inner);
                    }
                    if let Some(inner) = to {
                        visitor.visit_expression(inner);
                    }
                }
                TransitionEffect::Assign { lhs, rhs } => {
                    visitor.visit_expression(lhs);
                    visitor.visit_expression(rhs);
                }
                TransitionEffect::Expression(field_0) => {
                    visitor.visit_expression(field_0);
                }
            }
        }

        pub fn walk_action_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ActionUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ActionUsage { is_abstract, is_variation, is_reference, is_individual, name, short_name, type_name, typing, multiplicity, subsets, redefines, accept, send, via, to, body, name_span, type_ref_span, membership } = &$($mutability)? node.value;
            let _ = is_abstract;
            let _ = is_variation;
            let _ = is_reference;
            let _ = is_individual;
            visitor.visit_text(name);
            if let Some(inner) = short_name { visitor.visit_text(inner); }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = typing {
                visitor.visit_typing_relationship(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = accept {
                visitor.visit_payload_clause(inner);
            }
            if let Some(inner) = send {
                visitor.visit_send_payload(inner);
            }
            if let Some(inner) = via {
                visitor.visit_expression(inner);
            }
            if let Some(inner) = to {
                visitor.visit_expression(inner);
            }
            if let Some(inner) = body {
                visitor.visit_action_usage_body(inner);
            }
            if let Some(inner) = name_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = type_ref_span {
                visitor.visit_span(inner);
            }
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_action_usage_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? ActionUsageBody) {
            match node {
                ActionUsageBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                ActionUsageBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_action_usage_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_action_usage_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ActionUsageBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                ActionUsageBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                ActionUsageBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                ActionUsageBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                ActionUsageBodyElement::Dependency(field_0) => {
                    visitor.visit_dependency(field_0);
                }
                ActionUsageBodyElement::MetadataUsage(field_0) => {
                    visitor.visit_metadata_usage(field_0);
                }
                ActionUsageBodyElement::InOutDecl(field_0) => {
                    visitor.visit_in_out_decl(field_0);
                }
                ActionUsageBodyElement::RefDecl(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
                ActionUsageBodyElement::Bind(field_0) => {
                    visitor.visit_bind(field_0);
                }
                ActionUsageBodyElement::FlowUsage(field_0) => {
                    visitor.visit_flow_usage(field_0);
                }
                ActionUsageBodyElement::FirstStmt(field_0) => {
                    visitor.visit_first_stmt(field_0);
                }
                ActionUsageBodyElement::MergeStmt(field_0) => {
                    visitor.visit_merge_stmt(field_0);
                }
                ActionUsageBodyElement::DecisionStmt(field_0) => {
                    visitor.visit_decision_stmt(field_0);
                }
                ActionUsageBodyElement::JoinStmt(field_0) => {
                    visitor.visit_join_stmt(field_0);
                }
                ActionUsageBodyElement::ForkStmt(field_0) => {
                    visitor.visit_fork_stmt(field_0);
                }
                ActionUsageBodyElement::TerminateStmt(field_0) => {
                    visitor.visit_terminate_stmt(field_0);
                }
                ActionUsageBodyElement::WhileStmt(field_0) => {
                    visitor.visit_while_stmt(field_0);
                }
                ActionUsageBodyElement::LoopStmt(field_0) => {
                    visitor.visit_loop_stmt(field_0);
                }
                ActionUsageBodyElement::IfStmt(field_0) => {
                    visitor.visit_if_stmt(field_0);
                }
                ActionUsageBodyElement::StateUsage(field_0) => {
                    visitor.visit_state_usage(field_0);
                }
                ActionUsageBodyElement::ActionUsage(field_0) => {
                    visitor.visit_action_usage(&$($mutability)? **field_0);
                }
                ActionUsageBodyElement::PartUsage(field_0) => {
                    visitor.visit_part_usage(&$($mutability)? **field_0);
                }
                ActionUsageBodyElement::ItemUsage(field_0) => {
                    visitor.visit_item_usage(field_0);
                }
                ActionUsageBodyElement::AssertConstraint(field_0) => {
                    visitor.visit_assert_constraint_member(field_0);
                }
                ActionUsageBodyElement::OccurrenceUsage(field_0) => {
                    visitor.visit_occurrence_usage(&$($mutability)? **field_0);
                }
                ActionUsageBodyElement::Assign(field_0) => {
                    visitor.visit_assign_stmt(field_0);
                }
                ActionUsageBodyElement::ForLoop(field_0) => {
                    visitor.visit_for_loop(field_0);
                }
                ActionUsageBodyElement::ThenAction(field_0) => {
                    visitor.visit_then_action(field_0);
                }
                ActionUsageBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(&$($mutability)? **field_0);
                }
                ActionUsageBodyElement::CalcUsage(field_0) => {
                    visitor.visit_calc_usage(&$($mutability)? **field_0);
                }
                ActionUsageBodyElement::ActionDef(field_0) => {
                    visitor.visit_action_def(&$($mutability)? **field_0);
                }
                ActionUsageBodyElement::DefaultReferenceUsage(field_0) => {
                    visitor.visit_default_reference_usage(field_0);
                }
                ActionUsageBodyElement::VariantUsage(field_0) => {
                    visitor.visit_variant_usage(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_flow_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FlowDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let FlowDef { definition_prefix: _, is_individual: _, identification, specializes, body, membership } = &$($mutability)? node.value;
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_definition_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_flow_usage_kind<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? FlowUsageKind) {
            match node {
                FlowUsageKind::Flow => {}
                FlowUsageKind::Message => {}
                FlowUsageKind::SuccessionFlow => {}
            }
        }

        pub fn walk_payload_feature<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<PayloadFeature>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let PayloadFeature { name, type_name, type_is_conjugated, multiplicity } = &$($mutability)? node.value;
            if let Some(inner) = name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            let _ = type_is_conjugated;
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_flow_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FlowUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let FlowUsage { kind, name, type_name, type_is_conjugated, subsets, redefines, payload, from, to, body, membership } = &$($mutability)? node.value;
            visitor.visit_flow_usage_kind(kind);
            if let Some(inner) = name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            let _ = type_is_conjugated;
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = payload {
                visitor.visit_payload_feature(inner);
            }
            if let Some(inner) = from {
                visitor.visit_kerml_connector_end(inner);
            }
            if let Some(inner) = to {
                visitor.visit_kerml_connector_end(inner);
            }
            visitor.visit_definition_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_first_stmt<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FirstStmt>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let FirstStmt { succession_name, succession_type, succession_multiplicity, first, first_multiplicity, then, then_multiplicity, body } = &$($mutability)? node.value;
            if let Some(inner) = succession_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = succession_type {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = succession_multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_expression(first);
            if let Some(inner) = first_multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = then {
                visitor.visit_expression(inner);
            }
            if let Some(inner) = then_multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_first_merge_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_merge_stmt<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<MergeStmt>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let MergeStmt { merge, body } = &$($mutability)? node.value;
            visitor.visit_expression(merge);
            visitor.visit_first_merge_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_decision_stmt<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<DecisionStmt>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let DecisionStmt { decide, body } = &$($mutability)? node.value;
            visitor.visit_expression(decide);
            visitor.visit_first_merge_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_join_stmt<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<JoinStmt>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let JoinStmt { join, body } = &$($mutability)? node.value;
            visitor.visit_expression(join);
            visitor.visit_first_merge_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_fork_stmt<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ForkStmt>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ForkStmt { fork, body } = &$($mutability)? node.value;
            visitor.visit_expression(fork);
            visitor.visit_first_merge_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_first_merge_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? FirstMergeBody) {
            match node {
                FirstMergeBody::Semicolon => {}
                FirstMergeBody::Brace(field_0) => {
                    visitor.visit_first_merge_brace_body(field_0);
                }
            }
        }

        pub fn walk_first_merge_brace_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FirstMergeBraceBody>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let FirstMergeBraceBody { open_brace_span, elements, close_brace_span } = &$($mutability)? node.value;
            visitor.visit_span(open_brace_span);
            for inner in elements {
                visitor.visit_first_merge_body_element(inner);
            }
            visitor.visit_span(close_brace_span);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_first_merge_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FirstMergeBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                FirstMergeBodyElement::Member(field_0) => {
                    visitor.visit_action_def_body_element(&$($mutability)? **field_0);
                }
                FirstMergeBodyElement::Unsupported(field_0) => {
                    visitor.visit_unsupported_grammar_node(field_0);
                }
                FirstMergeBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_terminate_stmt<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<TerminateStmt>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let TerminateStmt { target } = &$($mutability)? node.value;
            if let Some(inner) = target {
                visitor.visit_expression(inner);
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_while_stmt<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<WhileStmt>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let WhileStmt { condition, body } = &$($mutability)? node.value;
            visitor.visit_expression(condition);
            visitor.visit_action_def_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_loop_stmt<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<LoopStmt>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let LoopStmt { body } = &$($mutability)? node.value;
            visitor.visit_action_def_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_if_stmt<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<IfStmt>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let IfStmt { condition, then_body, else_body } = &$($mutability)? node.value;
            visitor.visit_expression(condition);
            visitor.visit_action_branch_body(then_body);
            if let Some(inner) = else_body {
                visitor.visit_action_branch_body(inner);
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_allocate<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<Allocate>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let Allocate { source, target, body } = &$($mutability)? node.value;
            visitor.visit_expression(source);
            visitor.visit_expression(target);
            walk_part_usage_body(visitor, body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_allocation_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<AllocationDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let AllocationDef { definition_prefix: _, is_individual: _, identification, specializes, body, membership } = &$($mutability)? node.value;
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_definition_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_allocation_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<AllocationUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let AllocationUsage { name, type_name, type_is_conjugated, subsets, redefines, source, target, body, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            let _ = type_is_conjugated;
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = source {
                visitor.visit_kerml_connector_end(inner);
            }
            if let Some(inner) = target {
                visitor.visit_kerml_connector_end(inner);
            }
            visitor.visit_definition_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_state_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<StateDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let StateDef { definition_prefix, is_individual, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            let _ = is_individual;
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_state_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_state_def_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? StateDefBody) {
            match node {
                StateDefBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                StateDefBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_state_def_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_state_def_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<StateDefBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                StateDefBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                StateDefBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                StateDefBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                StateDefBodyElement::InOutDecl(field_0) => {
                    visitor.visit_in_out_decl(field_0);
                }
                StateDefBodyElement::Entry(field_0) => {
                    visitor.visit_entry_action(field_0);
                }
                StateDefBodyElement::Do(field_0) => {
                    visitor.visit_do_action(field_0);
                }
                StateDefBodyElement::Exit(field_0) => {
                    visitor.visit_exit_action(field_0);
                }
                StateDefBodyElement::Then(field_0) => {
                    visitor.visit_then_stmt(field_0);
                }
                StateDefBodyElement::FinalState(field_0) => {
                    visitor.visit_final_state(field_0);
                }
                StateDefBodyElement::Ref(field_0) => {
                    visitor.visit_ref_decl(&$($mutability)? **field_0);
                }
                StateDefBodyElement::RequirementUsage(field_0) => {
                    visitor.visit_requirement_usage(field_0);
                }
                StateDefBodyElement::StateUsage(field_0) => {
                    visitor.visit_state_usage(field_0);
                }
                StateDefBodyElement::Transition(field_0) => {
                    visitor.visit_transition(&$($mutability)? **field_0);
                }
                StateDefBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(&$($mutability)? **field_0);
                }
                StateDefBodyElement::ActionUsage(field_0) => {
                    visitor.visit_action_usage(&$($mutability)? **field_0);
                }
                StateDefBodyElement::SuccessionUsage(field_0) => {
                    visitor.visit_succession_usage(field_0);
                }
                StateDefBodyElement::AssertConstraint(field_0) => {
                    visitor.visit_assert_constraint_member(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_entry_action<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<EntryAction>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let EntryAction { action_reference, has_action_keyword, declared_name, type_name, redefines, effect, body } = &$($mutability)? node.value;
            if let Some(inner) = action_reference {
                visitor.visit_qualified_reference(inner);
            }
            let _ = has_action_keyword;
            if let Some(inner) = declared_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = effect {
                visitor.visit_transition_effect(inner);
            }
            visitor.visit_state_def_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_do_action<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<DoAction>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let DoAction { action_reference, has_action_keyword, declared_name, type_name, redefines, effect, body } = &$($mutability)? node.value;
            if let Some(inner) = action_reference {
                visitor.visit_qualified_reference(inner);
            }
            let _ = has_action_keyword;
            if let Some(inner) = declared_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = effect {
                visitor.visit_transition_effect(inner);
            }
            visitor.visit_state_def_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_exit_action<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ExitAction>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ExitAction { action_reference, has_action_keyword, declared_name, type_name, redefines, effect, body } = &$($mutability)? node.value;
            if let Some(inner) = action_reference {
                visitor.visit_qualified_reference(inner);
            }
            let _ = has_action_keyword;
            if let Some(inner) = declared_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = effect {
                visitor.visit_transition_effect(inner);
            }
            visitor.visit_state_def_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_then_stmt<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ThenStmt>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ThenStmt { state_reference } = &$($mutability)? node.value;
            visitor.visit_qualified_reference(state_reference);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_final_state<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FinalState>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let FinalState { state_name, name_span } = &$($mutability)? node.value;
            visitor.visit_text(state_name);
            visitor.visit_span(name_span);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_state_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<StateUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let StateUsage { direction, is_derived, is_abstract, is_reference, is_individual, name, state_reference, type_name, typing, multiplicity, subsets, redefines, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = direction {
                visitor.visit_in_out_value(inner);
            }
            let _ = is_derived;
            let _ = is_abstract;
            let _ = is_reference;
            let _ = is_individual;
            visitor.visit_text(name);
            if let Some(inner) = state_reference {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = typing {
                visitor.visit_typing_relationship(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            visitor.visit_state_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_transition<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<Transition>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let Transition { name, source, is_initial, accept, guard, effect, target, body } = &$($mutability)? node.value;
            if let Some(inner) = name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = source {
                visitor.visit_expression(inner);
            }
            let _ = is_initial;
            if let Some(inner) = accept {
                visitor.visit_transition_accept_value(inner);
            }
            if let Some(inner) = guard {
                visitor.visit_expression(inner);
            }
            if let Some(inner) = effect {
                visitor.visit_transition_effect(inner);
            }
            visitor.visit_expression(target);
            walk_action_def_body(visitor, body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_requirement_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<RequirementDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let RequirementDef { definition_prefix, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_requirement_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_requirement_def_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? RequirementDefBody) {
            match node {
                RequirementDefBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                RequirementDefBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_requirement_def_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_requirement_def_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<RequirementDefBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                RequirementDefBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                RequirementDefBodyElement::Satisfy(field_0) => {
                    visitor.visit_satisfy_requirement_usage(&$($mutability)? **field_0);
                }
                RequirementDefBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                RequirementDefBodyElement::Dependency(field_0) => {
                    visitor.visit_dependency(field_0);
                }
                RequirementDefBodyElement::Import(field_0) => {
                    visitor.visit_import(field_0);
                }
                RequirementDefBodyElement::SubjectDecl(field_0) => {
                    visitor.visit_subject_decl(field_0);
                }
                RequirementDefBodyElement::SubjectRef(field_0) => {
                    visitor.visit_subject_ref(field_0);
                }
                RequirementDefBodyElement::RequirementActorDecl(field_0) => {
                    visitor.visit_requirement_actor_decl(field_0);
                }
                RequirementDefBodyElement::RequirementUsage(field_0) => {
                    visitor.visit_requirement_usage(&$($mutability)? **field_0);
                }
                RequirementDefBodyElement::RequirementDef(field_0) => {
                    visitor.visit_requirement_def(&$($mutability)? **field_0);
                }
                RequirementDefBodyElement::Stakeholder(field_0) => {
                    visitor.visit_stakeholder_member(field_0);
                }
                RequirementDefBodyElement::Purpose(field_0) => {
                    visitor.visit_purpose_member(field_0);
                }
                RequirementDefBodyElement::AttributeDef(field_0) => {
                    visitor.visit_attribute_def(field_0);
                }
                RequirementDefBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(field_0);
                }
                RequirementDefBodyElement::VariantUsage(field_0) => {
                    visitor.visit_variant_usage(field_0);
                }
                RequirementDefBodyElement::VerifyRequirement(field_0) => {
                    visitor.visit_verify_requirement_member(field_0);
                }
                RequirementDefBodyElement::RequireConstraint(field_0) => {
                    visitor.visit_require_constraint(field_0);
                }
                RequirementDefBodyElement::Constraint(field_0) => {
                    visitor.visit_constraint_usage(field_0);
                }
                RequirementDefBodyElement::Frame(field_0) => {
                    visitor.visit_frame_member(field_0);
                }
                RequirementDefBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                RequirementDefBodyElement::RefDecl(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
                RequirementDefBodyElement::ConcernUsage(field_0) => {
                    visitor.visit_concern_usage(field_0);
                }
                RequirementDefBodyElement::CalcUsage(field_0) => {
                    visitor.visit_calc_usage(&$($mutability)? **field_0);
                }
                RequirementDefBodyElement::PortUsage(field_0) => {
                    visitor.visit_port_usage(&$($mutability)? **field_0);
                }
                RequirementDefBodyElement::AllocationUsage(field_0) => {
                    visitor.visit_allocation_usage(&$($mutability)? **field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_stakeholder_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<StakeholderMember>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let StakeholderMember { declaration_name, target, type_name, is_redefinition } = &$($mutability)? node.value;
            visitor.visit_text(declaration_name);
            if let Some(inner) = target {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            let _ = is_redefinition;
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_purpose_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<PurposeMember>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let PurposeMember { target } = &$($mutability)? node.value;
            visitor.visit_qualified_reference(target);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_subject_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<SubjectDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let SubjectDecl { name, short_name, type_name, redefines, multiplicity, value } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = short_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_requirement_actor_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<RequirementActorDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let RequirementActorDecl { name, short_name, type_name, multiplicity } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = short_name {
                visitor.visit_text(inner);
            }
            visitor.visit_qualified_reference(type_name);
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_require_constraint<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<RequireConstraint>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let RequireConstraint { is_assume, has_constraint_keyword, name, target, body } = &$($mutability)? node.value;
            let _ = is_assume;
            let _ = has_constraint_keyword;
            if let Some(inner) = name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = target {
                visitor.visit_qualified_reference(inner);
            }
            visitor.visit_constraint_def_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_verify_requirement_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<VerifyRequirementMember>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let VerifyRequirementMember { explicit_requirement_keyword, requirement, target, redefines } = &$($mutability)? node.value;
            let _ = explicit_requirement_keyword;
            if let Some(inner) = requirement {
                visitor.visit_requirement_usage(inner);
            }
            if let Some(inner) = target {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_qualified_reference(inner);
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_satisfy_requirement_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<SatisfyRequirementUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let SatisfyRequirementUsage {
                prefix,
                membership,
                assert_span,
                not_span,
                satisfy_span,
                requirement,
                typing,
                multiplicity,
                multiplicity_modifiers,
                subsets,
                redefines,
                references,
                crosses,
                value,
                subject,
                body,
            } = &$($mutability)? node.value;
            visitor.visit_occurrence_usage_prefix(prefix);
            visitor.visit_membership(membership);
            if let Some(inner) = assert_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = not_span {
                visitor.visit_span(inner);
            }
            visitor.visit_span(satisfy_span);
            visitor.visit_satisfied_requirement(requirement);
            if let Some(inner) = typing {
                visitor.visit_typing_relationship(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_multiplicity_modifiers(multiplicity_modifiers);
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = references {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = crosses {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            if let Some(inner) = subject {
                visitor.visit_satisfaction_subject(inner);
            }
            walk_requirement_def_body(visitor, body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_satisfied_requirement<V: $Visitor>(visitor: &mut V, node: &$($mutability)? SatisfiedRequirement) {
            match node {
                SatisfiedRequirement::Reference { reference } => {
                    visitor.visit_qualified_reference(reference);
                }
                SatisfiedRequirement::Declaration(field_0) => {
                    visitor.visit_inline_requirement_declaration(field_0);
                }
            }
        }

        pub fn walk_inline_requirement_declaration<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<InlineRequirementDeclaration>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let InlineRequirementDeclaration { keyword_span, identification } = &$($mutability)? node.value;
            visitor.visit_span(keyword_span);
            visitor.visit_identification(identification);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_satisfaction_subject<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<SatisfactionSubject>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let SatisfactionSubject { by_span, reference } = &$($mutability)? node.value;
            visitor.visit_span(by_span);
            visitor.visit_qualified_reference(reference);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_requirement_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<RequirementUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let RequirementUsage { name, short_name, type_name, multiplicity, subsets, references, is_abstract, is_variation, value, direction, body, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = short_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = references {
                visitor.visit_subsetting_relationship(inner);
            }
            let _ = is_abstract;
            let _ = is_variation;
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            if let Some(inner) = direction {
                visitor.visit_in_out_value(inner);
            }
            visitor.visit_requirement_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_item_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ItemUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ItemUsage { prefix, name, type_name, redefines, subsets, short_name, multiplicity, multiplicity_modifiers, value, body, membership } = &$($mutability)? node.value;
            visitor.visit_occurrence_usage_prefix(prefix);
            visitor.visit_text(name);
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = short_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_multiplicity_modifiers(multiplicity_modifiers);
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            visitor.visit_attribute_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_enumeration_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<EnumerationUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let EnumerationUsage { name, type_name, multiplicity, body, is_end, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_attribute_body(body);
            let _ = is_end;
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_interface_usage_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Body<InterfaceUsageBodyElement>) {
            match node {
                Body::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                Body::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_interface_usage_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_relationship_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Body<RelationshipBodyElement>) {
            match node {
                Body::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                Body::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_relationship_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_dependency<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<Dependency>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let Dependency { identification, clients, suppliers, body } = &$($mutability)? node.value;
            if let Some(inner) = identification {
                visitor.visit_identification(inner);
            }
            for inner in clients {
                visitor.visit_qualified_reference(inner);
            }
            for inner in suppliers {
                visitor.visit_qualified_reference(inner);
            }
            walk_relationship_body(visitor, body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_frame_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FrameMember>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let FrameMember { has_concern_keyword, name, short_name, type_name, multiplicity, subsets, redefines, value, body } = &$($mutability)? node.value;
            let _ = has_concern_keyword;
            visitor.visit_text(name);
            if let Some(inner) = short_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            visitor.visit_requirement_def_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_concern_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ConcernUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ConcernUsage { name, is_abstract, type_name, multiplicity, subsets, redefines, body, is_definition, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            let _ = is_abstract;
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            visitor.visit_requirement_def_body(body);
            let _ = is_definition;
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_case_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<CaseDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let CaseDef { definition_prefix, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_use_case_def_body_value(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_case_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<CaseUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let CaseUsage { name, type_name, multiplicity, subsets, redefines, is_abstract, body, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            let _ = is_abstract;
            visitor.visit_use_case_def_body_value(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_analysis_case_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<AnalysisCaseDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let AnalysisCaseDef { definition_prefix, identification, specializes, is_individual, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            let _ = is_individual;
            visitor.visit_use_case_def_body_value(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_analysis_case_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<AnalysisCaseUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let AnalysisCaseUsage { name, type_name, subsets, redefines, is_abstract, is_individual, body, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            let _ = is_abstract;
            let _ = is_individual;
            visitor.visit_use_case_def_body_value(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_verification_case_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<VerificationCaseDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let VerificationCaseDef { definition_prefix, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_use_case_def_body_value(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_verification_case_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<VerificationCaseUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let VerificationCaseUsage { name, type_name, multiplicity, subsets, is_abstract, body, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            let _ = is_abstract;
            visitor.visit_use_case_def_body_value(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_use_case_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<UseCaseUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let UseCaseUsage { name, type_name, is_abstract, multiplicity, subsets, body, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            let _ = is_abstract;
            visitor.visit_use_case_def_body_value(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_actor_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ActorDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ActorDecl { identification } = &$($mutability)? node.value;
            visitor.visit_identification(identification);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_use_case_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<UseCaseDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let UseCaseDef { definition_prefix, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_use_case_def_body_value(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_use_case_def_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<UseCaseDefBody>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            visitor.visit_use_case_def_body_value(&$($mutability)? node.value);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_use_case_def_body_value<V: $Visitor>(visitor: &mut V, node: &$($mutability)? UseCaseDefBody) {
            match node {
                UseCaseDefBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                UseCaseDefBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_use_case_def_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_first_succession<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FirstSuccession>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let FirstSuccession { target } = &$($mutability)? node.value;
            visitor.visit_qualified_reference(target);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_then_done<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ThenDone>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ThenDone {} = &$($mutability)? node.value;
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_include_use_case<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<IncludeUseCase>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let IncludeUseCase { target, multiplicity, body } = &$($mutability)? node.value;
            visitor.visit_qualified_reference(target);
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_use_case_def_body_value(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_then_include_use_case<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ThenIncludeUseCase>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ThenIncludeUseCase { include } = &$($mutability)? node.value;
            visitor.visit_include_use_case(include);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_then_use_case_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ThenUseCaseUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ThenUseCaseUsage { use_case } = &$($mutability)? node.value;
            visitor.visit_use_case_usage(use_case);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_subject_ref<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<SubjectRef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let SubjectRef {} = &$($mutability)? node.value;
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_actor_redefinition_assignment<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ActorRedefinitionAssignment>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ActorRedefinitionAssignment { target, value } = &$($mutability)? node.value;
            visitor.visit_qualified_reference(target);
            visitor.visit_expression(value);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_ref_redefinition<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<RefRedefinition>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let RefRedefinition { target, body } = &$($mutability)? node.value;
            visitor.visit_qualified_reference(target);
            visitor.visit_use_case_def_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_case_return_feature_kind<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? CaseReturnFeatureKind) {
            match node {
                CaseReturnFeatureKind::Part => {}
                CaseReturnFeatureKind::Attribute => {}
            }
        }

        pub fn walk_case_return_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<CaseReturnDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let CaseReturnDecl { declaration_name, name_span, target, type_name, value, is_subsetting, feature_kind, multiplicity, redefines } = &$($mutability)? node.value;
            visitor.visit_text(declaration_name);
            if let Some(inner) = name_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = target {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            let _ = is_subsetting;
            if let Some(inner) = feature_kind {
                visitor.visit_case_return_feature_kind(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_return_ref<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ReturnRef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ReturnRef { name, multiplicity, body } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_return_ref_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_return_ref_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ReturnRefBody>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                ReturnRefBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                ReturnRefBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_return_ref_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_return_ref_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ReturnRefBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                ReturnRefBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                ReturnRefBodyElement::Result(field_0) => {
                    visitor.visit_expression(field_0);
                }
                ReturnRefBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_use_case_def_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<UseCaseDefBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                UseCaseDefBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                UseCaseDefBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                UseCaseDefBodyElement::AttributeDef(field_0) => {
                    visitor.visit_attribute_def(field_0);
                }
                UseCaseDefBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                UseCaseDefBodyElement::SubjectDecl(field_0) => {
                    visitor.visit_subject_decl(field_0);
                }
                UseCaseDefBodyElement::SubjectRef(field_0) => {
                    visitor.visit_subject_ref(field_0);
                }
                UseCaseDefBodyElement::ActorUsage(field_0) => {
                    visitor.visit_actor_usage(field_0);
                }
                UseCaseDefBodyElement::ActorRedefinitionAssignment(field_0) => {
                    visitor.visit_actor_redefinition_assignment(field_0);
                }
                UseCaseDefBodyElement::Objective(field_0) => {
                    visitor.visit_objective(field_0);
                }
                UseCaseDefBodyElement::FirstSuccession(field_0) => {
                    visitor.visit_first_succession(field_0);
                }
                UseCaseDefBodyElement::ThenIncludeUseCase(field_0) => {
                    visitor.visit_then_include_use_case(field_0);
                }
                UseCaseDefBodyElement::ThenUseCaseUsage(field_0) => {
                    visitor.visit_then_use_case_usage(field_0);
                }
                UseCaseDefBodyElement::UseCaseUsage(field_0) => {
                    visitor.visit_use_case_usage(&$($mutability)? **field_0);
                }
                UseCaseDefBodyElement::CaseUsage(field_0) => {
                    visitor.visit_case_usage(&$($mutability)? **field_0);
                }
                UseCaseDefBodyElement::VerificationCaseUsage(field_0) => {
                    visitor.visit_verification_case_usage(&$($mutability)? **field_0);
                }
                UseCaseDefBodyElement::ThenDone(field_0) => {
                    visitor.visit_then_done(field_0);
                }
                UseCaseDefBodyElement::IncludeUseCase(field_0) => {
                    visitor.visit_include_use_case(field_0);
                }
                UseCaseDefBodyElement::RefRedefinition(field_0) => {
                    visitor.visit_ref_redefinition(field_0);
                }
                UseCaseDefBodyElement::Ref(field_0) => {
                    visitor.visit_ref_decl(&$($mutability)? **field_0);
                }
                UseCaseDefBodyElement::InOutDecl(field_0) => {
                    visitor.visit_in_out_decl(&$($mutability)? **field_0);
                }
                UseCaseDefBodyElement::AssertConstraint(field_0) => {
                    visitor.visit_assert_constraint_member(field_0);
                }
                UseCaseDefBodyElement::ReturnRef(field_0) => {
                    visitor.visit_return_ref(field_0);
                }
                UseCaseDefBodyElement::CaseReturnDecl(field_0) => {
                    visitor.visit_case_return_decl(field_0);
                }
                UseCaseDefBodyElement::Assign(field_0) => {
                    visitor.visit_assign_stmt(field_0);
                }
                UseCaseDefBodyElement::ForLoop(field_0) => {
                    visitor.visit_for_loop(field_0);
                }
                UseCaseDefBodyElement::ThenAction(field_0) => {
                    visitor.visit_then_action(field_0);
                }
                UseCaseDefBodyElement::ActionUsage(field_0) => {
                    visitor.visit_action_usage(&$($mutability)? **field_0);
                }
                UseCaseDefBodyElement::AnalysisCaseUsage(field_0) => {
                    visitor.visit_analysis_case_usage(&$($mutability)? **field_0);
                }
                UseCaseDefBodyElement::CalcUsage(field_0) => {
                    visitor.visit_calc_usage(&$($mutability)? **field_0);
                }
                UseCaseDefBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(field_0);
                }
                UseCaseDefBodyElement::RequirementUsage(field_0) => {
                    visitor.visit_requirement_usage(&$($mutability)? **field_0);
                }
                UseCaseDefBodyElement::PartUsage(field_0) => {
                    visitor.visit_part_usage(&$($mutability)? **field_0);
                }
                UseCaseDefBodyElement::Expression(field_0) => {
                    visitor.visit_expression(field_0);
                }
                UseCaseDefBodyElement::FlowUsage(field_0) => {
                    visitor.visit_flow_usage(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_actor_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ActorUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ActorUsage { name, short_name, type_name, multiplicity, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = short_name {
                visitor.visit_text(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_objective<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<Objective>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let Objective { visibility, requirement } = &$($mutability)? node.value;
            if let Some(inner) = visibility {
                visitor.visit_visibility(inner);
            }
            visitor.visit_requirement_usage(requirement);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_constraint_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ConstraintDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ConstraintDef { definition_prefix, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_constraint_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_constraint_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ConstraintUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ConstraintUsage { prefix, name, short_name, type_name, multiplicity, subsets, redefines, body, membership } = &$($mutability)? node.value;
            visitor.visit_occurrence_usage_prefix(prefix);
            visitor.visit_text(name);
            if let Some(inner) = short_name { visitor.visit_text(inner); }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            visitor.visit_constraint_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_constraint_def_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? ConstraintDefBody) {
            match node {
                ConstraintDefBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                ConstraintDefBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_constraint_def_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_constraint_def_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ConstraintDefBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                ConstraintDefBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                ConstraintDefBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                ConstraintDefBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                ConstraintDefBodyElement::InOutDecl(field_0) => {
                    visitor.visit_in_out_decl(&$($mutability)? **field_0);
                }
                ConstraintDefBodyElement::Expression(field_0) => {
                    visitor.visit_expression(field_0);
                }
                ConstraintDefBodyElement::Constraint(field_0) => {
                    visitor.visit_constraint_usage(field_0);
                }
                ConstraintDefBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(&$($mutability)? **field_0);
                }
                ConstraintDefBodyElement::FeatureDecl(field_0) => {
                    visitor.visit_default_reference_usage(&$($mutability)? **field_0);
                }
                ConstraintDefBodyElement::PartUsage(field_0) => {
                    visitor.visit_part_usage(&$($mutability)? **field_0);
                }
                ConstraintDefBodyElement::RequireConstraint(field_0) => {
                    visitor.visit_require_constraint(&$($mutability)? **field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_calc_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<CalcDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let CalcDef { definition_prefix, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_calc_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_calc_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<CalcUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let CalcUsage { identification, is_abstract, type_name, multiplicity, subsets, redefines, value, direction, is_reference: _, body, membership } = &$($mutability)? node.value;
            visitor.visit_identification(identification);
            let _ = is_abstract;
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = redefines {
                for inner in inner {
                    visitor.visit_qualified_reference(inner);
                }
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            if let Some(inner) = direction {
                visitor.visit_in_out_value(inner);
            }
            visitor.visit_calc_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_calc_def_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? CalcDefBody) {
            match node {
                CalcDefBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                CalcDefBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_calc_def_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_calc_def_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<CalcDefBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                CalcDefBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                CalcDefBodyElement::ActionMember(field_0) => {
                    visitor.visit_action_def_body_element(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                CalcDefBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                CalcDefBodyElement::InOutDecl(field_0) => {
                    visitor.visit_in_out_decl(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::KermlFeature(field_0) => {
                    visitor.visit_kerml_feature_member(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::Invariant(field_0) => {
                    visitor.visit_kerml_invariant_member(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::Connector(field_0) => {
                    visitor.visit_kerml_connector_member(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::Binding(field_0) => {
                    visitor.visit_kerml_binding_member(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::Succession(field_0) => {
                    visitor.visit_kerml_succession_member(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::Import(field_0) => {
                    visitor.visit_import(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::AttributeUsage(field_0) => {
                    visitor.visit_attribute_usage(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::AssertConstraint(field_0) => {
                    visitor.visit_assert_constraint_member(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::KermlClassifier(field_0) => {
                    visitor.visit_kerml_classifier_decl(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::DefaultReferenceUsage(field_0) => {
                    visitor.visit_default_reference_usage(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::ReturnDecl(field_0) => {
                    visitor.visit_return_decl(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::Expression(field_0) => {
                    visitor.visit_expression(field_0);
                }
                CalcDefBodyElement::CalcUsage(field_0) => {
                    visitor.visit_calc_usage(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::CalcDef(field_0) => {
                    visitor.visit_calc_def(&$($mutability)? **field_0);
                }
                CalcDefBodyElement::PartUsage(field_0) => {
                    visitor.visit_part_usage(&$($mutability)? **field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_return_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ReturnDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ReturnDecl { kind_keyword, name, short_name, type_name, is_redefine, is_subsetting, multiplicity, multiplicity_modifiers, redefines, value, body } = &$($mutability)? node.value;
            if let Some(inner) = kind_keyword {
                visitor.visit_return_kind_keyword(inner);
            }
            visitor.visit_text(name);
            if let Some(inner) = short_name { visitor.visit_text(inner); }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            let _ = is_redefine;
            let _ = is_subsetting;
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_multiplicity_modifiers(multiplicity_modifiers);
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            visitor.visit_calc_def_body(body);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_return_kind_keyword<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? ReturnKindKeyword) {
            match node {
                ReturnKindKeyword::Attribute => {}
                ReturnKindKeyword::Feature => {}
            }
        }

        pub fn walk_view_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ViewDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ViewDef { definition_prefix, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_view_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_view_def_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? ViewDefBody) {
            match node {
                ViewDefBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                ViewDefBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_view_def_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_view_def_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ViewDefBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                ViewDefBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                ViewDefBodyElement::Unsupported(field_0) => {
                    visitor.visit_unsupported_grammar_node(field_0);
                }
                ViewDefBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                ViewDefBodyElement::MetadataKeywordUsage(field_0) => {
                    visitor.visit_metadata_keyword_usage(field_0);
                }
                ViewDefBodyElement::RefDecl(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
                ViewDefBodyElement::ViewpointUsage(field_0) => {
                    visitor.visit_viewpoint_usage(field_0);
                }
                ViewDefBodyElement::Satisfy(field_0) => {
                    visitor.visit_satisfy_requirement_usage(&$($mutability)? **field_0);
                }
                ViewDefBodyElement::Filter(field_0) => {
                    visitor.visit_filter_member(field_0);
                }
                ViewDefBodyElement::ViewRendering(field_0) => {
                    visitor.visit_view_rendering_usage(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_view_rendering_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ViewRenderingUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ViewRenderingUsage { name, type_name, body, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            visitor.visit_rendering_usage_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_rendering_usage_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? RenderingUsageBody) {
            match node {
                RenderingUsageBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                RenderingUsageBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_rendering_usage_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_rendering_usage_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<RenderingUsageBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                RenderingUsageBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                RenderingUsageBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                RenderingUsageBodyElement::ViewUsage(field_0) => {
                    visitor.visit_view_usage(&$($mutability)? **field_0);
                }
                RenderingUsageBodyElement::Rendering(field_0) => {
                    visitor.visit_rendering_usage(&$($mutability)? **field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_viewpoint_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ViewpointDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ViewpointDef { definition_prefix, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_requirement_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_rendering_def<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<RenderingDef>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let RenderingDef { definition_prefix, identification, specializes, body, membership } = &$($mutability)? node.value;
            if let Some(inner) = definition_prefix {
                visitor.visit_definition_prefix(inner);
            }
            visitor.visit_identification(identification);
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            visitor.visit_rendering_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_rendering_def_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? RenderingDefBody) {
            match node {
                RenderingDefBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                RenderingDefBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_rendering_def_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_rendering_def_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<RenderingDefBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                RenderingDefBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                RenderingDefBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                RenderingDefBodyElement::RefDecl(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
                RenderingDefBodyElement::Filter(field_0) => {
                    visitor.visit_filter_member(field_0);
                }
                RenderingDefBodyElement::ViewRendering(field_0) => {
                    visitor.visit_view_rendering_usage(field_0);
                }
                RenderingDefBodyElement::Unsupported(field_0) => {
                    visitor.visit_unsupported_grammar_node(field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_view_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ViewUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ViewUsage { name, short_name, type_name, subsets, redefines, multiplicity, multiplicity_modifiers, body, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = short_name { visitor.visit_text(inner); }
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_multiplicity_modifiers(multiplicity_modifiers);
            visitor.visit_view_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_view_body<V: $Visitor>(visitor: &mut V, node: &$($mutability)? ViewBody) {
            match node {
                ViewBody::Semicolon { semicolon_span } => {
                    visitor.visit_body_semicolon(semicolon_span);
                    visitor.visit_span(semicolon_span);
                }
                ViewBody::Brace { open_span, elements, close_span } => {
                    visitor.visit_body_braces(open_span, elements, close_span);
                    visitor.visit_span(open_span);
                    for inner in elements {
                        visitor.visit_view_body_element(inner);
                    }
                    visitor.visit_span(close_span);
                }
            }
        }

        pub fn walk_view_body_element<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ViewBodyElement>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            match &$($mutability)? node.value {
                ViewBodyElement::Error(field_0) => {
                    visitor.visit_parse_error_node(field_0);
                }
                ViewBodyElement::Annotating(field_0) => {
                    visitor.visit_annotating_member(field_0);
                }
                ViewBodyElement::RefDecl(field_0) => {
                    visitor.visit_ref_decl(field_0);
                }
                ViewBodyElement::Filter(field_0) => {
                    visitor.visit_filter_member(field_0);
                }
                ViewBodyElement::ViewRendering(field_0) => {
                    visitor.visit_view_rendering_usage(field_0);
                }
                ViewBodyElement::Expose(field_0) => {
                    visitor.visit_expose_member(field_0);
                }
                ViewBodyElement::Satisfy(field_0) => {
                    visitor.visit_satisfy_requirement_usage(&$($mutability)? **field_0);
                }
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_expose_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ExposeMember>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ExposeMember { target, body } = &$($mutability)? node.value;
            visitor.visit_import_target(target);
            walk_relationship_body(visitor, body);
            visitor.leave_node(&$($mutability)? node.span);
        }


        pub fn walk_viewpoint_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ViewpointUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ViewpointUsage { name, type_name, subsets, redefines, body, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            visitor.visit_requirement_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_rendering_usage<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<RenderingUsage>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let RenderingUsage { is_abstract, name, type_name, multiplicity, multiplicity_modifiers, subsets, redefines, value, body, membership } = &$($mutability)? node.value;
            let _ = is_abstract;
            visitor.visit_text(name);
            if let Some(inner) = type_name {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_multiplicity_modifiers(multiplicity_modifiers);
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            visitor.visit_rendering_usage_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_kerml_bare_declaration<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<KermlBareDeclaration>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let KermlBareDeclaration { keyword, name_span, multiplicity } = &$($mutability)? node.value;
            visitor.visit_kerml_bare_declaration_keyword(keyword);
            if let Some(inner) = name_span {
                visitor.visit_span(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_kerml_bare_declaration_keyword<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? KermlBareDeclarationKeyword) {
            match node {
                KermlBareDeclarationKeyword::Behavior => {}
                KermlBareDeclarationKeyword::Bool => {}
                KermlBareDeclarationKeyword::Function => {}
                KermlBareDeclarationKeyword::Interaction => {}
                KermlBareDeclarationKeyword::Datatype => {}
                KermlBareDeclarationKeyword::Inv => {}
                KermlBareDeclarationKeyword::Invariant => {}
                KermlBareDeclarationKeyword::Multiplicity => {}
                KermlBareDeclarationKeyword::Assoc => {}
                KermlBareDeclarationKeyword::Association => {}
                KermlBareDeclarationKeyword::Metaclass => {}
                KermlBareDeclarationKeyword::Step => {}
                KermlBareDeclarationKeyword::Occurrence => {}
                KermlBareDeclarationKeyword::Expr => {}
                KermlBareDeclarationKeyword::Predicate => {}
                KermlBareDeclarationKeyword::Succession => {}
                KermlBareDeclarationKeyword::Classifier => {}
            }
        }

        pub fn walk_kerml_semantic_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<KermlSemanticDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let KermlSemanticDecl { bnf_production, text } = &$($mutability)? node.value;
            visitor.visit_text(bnf_production);
            visitor.visit_text(text);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_kerml_feature_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<KermlFeatureDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let KermlFeatureDecl { bnf_production, text } = &$($mutability)? node.value;
            visitor.visit_text(bnf_production);
            visitor.visit_text(text);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_feature_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<FeatureDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let FeatureDecl { keyword, text } = &$($mutability)? node.value;
            visitor.visit_text(keyword);
            visitor.visit_text(text);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_classifier_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ClassifierDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ClassifierDecl { keyword, text } = &$($mutability)? node.value;
            visitor.visit_text(keyword);
            visitor.visit_text(text);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_extended_library_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<ExtendedLibraryDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let ExtendedLibraryDecl { bnf_production, text } = &$($mutability)? node.value;
            visitor.visit_text(bnf_production);
            visitor.visit_text(text);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_kerml_classifier_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<KermlClassifierDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let KermlClassifierDecl { is_abstract, keyword, is_all, identification, multiplicity, specializes, type_relationships, body, membership } = &$($mutability)? node.value;
            let _ = is_abstract;
            visitor.visit_kerml_classifier_keyword(keyword);
            let _ = is_all;
            visitor.visit_identification(identification);
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = specializes {
                visitor.visit_typing_relationship(inner);
            }
            for inner in type_relationships {
                visitor.visit_kerml_type_relationship(inner);
            }
            visitor.visit_calc_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_kerml_classifier_keyword<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? KermlClassifierKeyword) {
            match node {
                KermlClassifierKeyword::Function => {}
                KermlClassifierKeyword::Datatype => {}
                KermlClassifierKeyword::Metaclass => {}
                KermlClassifierKeyword::Struct => {}
                KermlClassifierKeyword::Assoc => {}
                KermlClassifierKeyword::Association => {}
                KermlClassifierKeyword::Behavior => {}
                KermlClassifierKeyword::Interaction => {}
                KermlClassifierKeyword::Predicate => {}
                KermlClassifierKeyword::Multiplicity => {}
                KermlClassifierKeyword::Type => {}
                KermlClassifierKeyword::Classifier => {}
                KermlClassifierKeyword::Class => {}
                KermlClassifierKeyword::AssocStruct => {}
            }
        }

        pub fn walk_kerml_type_relationship<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<KermlTypeRelationship>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let KermlTypeRelationship { keyword, targets, span } = &$($mutability)? node.value;
            visitor.visit_kerml_type_relationship_keyword(keyword);
            for inner in targets {
                visitor.visit_qualified_reference(inner);
            }
            visitor.visit_span(span);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_kerml_type_relationship_keyword<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? KermlTypeRelationshipKeyword) {
            match node {
                KermlTypeRelationshipKeyword::DisjointFrom => {}
                KermlTypeRelationshipKeyword::Unions => {}
                KermlTypeRelationshipKeyword::Intersects => {}
                KermlTypeRelationshipKeyword::Differences => {}
            }
        }

        pub fn walk_kerml_feature_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<KermlFeatureMember>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let KermlFeatureMember { is_member, prefix, kind, is_all, name, typing, multiplicity, multiplicity_modifiers, subsets, redefines, references, crosses, chains, inverse_of, type_relationships, value, body, membership } = &$($mutability)? node.value;
            let _ = is_member;
            visitor.visit_feature_prefix(prefix);
            if let Some(inner) = kind {
                visitor.visit_kerml_feature_kind(inner);
            }
            let _ = is_all;
            visitor.visit_text(name);
            if let Some(inner) = typing {
                visitor.visit_typing_relationship(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_multiplicity_modifiers(multiplicity_modifiers);
            if let Some(inner) = subsets {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = redefines {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = references {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = crosses {
                visitor.visit_subsetting_relationship(inner);
            }
            if let Some(inner) = chains {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = inverse_of {
                visitor.visit_qualified_reference(inner);
            }
            for inner in type_relationships {
                visitor.visit_kerml_type_relationship(inner);
            }
            if let Some(inner) = value {
                visitor.visit_feature_value(inner);
            }
            visitor.visit_calc_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_kerml_feature_kind<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<KermlFeatureKind>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            visitor.visit_kerml_feature_kind_value(&$($mutability)? node.value);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_kerml_feature_kind_value<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? KermlFeatureKind) {
            match node {
                KermlFeatureKind::Feature => {}
                KermlFeatureKind::Step => {}
                KermlFeatureKind::Expr => {}
                KermlFeatureKind::Bool => {}
            }
        }

        pub fn walk_kerml_invariant_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<KermlInvariantMember>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let KermlInvariantMember { is_negated, name, body, membership } = &$($mutability)? node.value;
            let _ = is_negated;
            visitor.visit_text(name);
            visitor.visit_calc_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_kerml_connector_end<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<KermlConnectorEnd>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let KermlConnectorEnd { multiplicity, target, references } = &$($mutability)? node.value;
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_qualified_reference(target);
            if let Some(inner) = references {
                visitor.visit_qualified_reference(inner);
            }
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_kerml_connector_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<KermlConnectorMember>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let KermlConnectorMember { is_all, name, typing, multiplicity, from, to, body, membership } = &$($mutability)? node.value;
            let _ = is_all;
            visitor.visit_text(name);
            if let Some(inner) = typing {
                visitor.visit_qualified_reference(inner);
            }
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            if let Some(inner) = from {
                visitor.visit_kerml_connector_end(inner);
            }
            if let Some(inner) = to {
                visitor.visit_kerml_connector_end(inner);
            }
            visitor.visit_calc_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_kerml_binding_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<KermlBindingMember>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let KermlBindingMember { name, multiplicity, left, right, body, membership } = &$($mutability)? node.value;
            visitor.visit_text(name);
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_kerml_connector_end(left);
            visitor.visit_kerml_connector_end(right);
            visitor.visit_calc_def_body(body);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_kerml_succession_member<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<KermlSuccessionMember>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let KermlSuccessionMember { is_all, name, multiplicity, first, then, membership } = &$($mutability)? node.value;
            let _ = is_all;
            visitor.visit_text(name);
            if let Some(inner) = multiplicity {
                visitor.visit_multiplicity(inner);
            }
            visitor.visit_kerml_connector_end(first);
            visitor.visit_kerml_connector_end(then);
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_kerml_relationship_decl<V: $Visitor>(visitor: &mut V, node: &$($mutability)? Node<KermlRelationshipDecl>) {
            visitor.enter_node(&$($mutability)? node.span);
            visitor.visit_span(&$($mutability)? node.span);
            let KermlRelationshipDecl { keyword, identification, source, target, body, membership } = &$($mutability)? node.value;
            visitor.visit_kerml_relationship_keyword(keyword);
            if let Some(inner) = identification {
                visitor.visit_identification(inner);
            }
            visitor.visit_qualified_reference(source);
            visitor.visit_qualified_reference(target);
            if let Some(inner) = body {
                for inner in inner {
                    visitor.visit_relationship_body_element(inner);
                }
            }
            visitor.visit_membership(membership);
            visitor.leave_node(&$($mutability)? node.span);
        }

        pub fn walk_kerml_relationship_keyword<V: $Visitor>(_visitor: &mut V, node: &$($mutability)? KermlRelationshipKeyword) {
            match node {
                KermlRelationshipKeyword::Subtype => {}
                KermlRelationshipKeyword::Subclassifier => {}
                KermlRelationshipKeyword::Typing => {}
                KermlRelationshipKeyword::Subset => {}
                KermlRelationshipKeyword::Redefinition => {}
                KermlRelationshipKeyword::Disjoint => {}
                KermlRelationshipKeyword::Inverse => {}
                KermlRelationshipKeyword::Featuring => {}
            }
        }

    };
}
