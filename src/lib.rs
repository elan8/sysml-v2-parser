//! SysML v2 textual notation parser.
//!
//! Reusable library for parsing SysML v2 textual syntax into an AST.
//!
//! ## Entry points
//!
//! - [`parse`] -- strict, all-or-nothing. Use this for conformance/roundtrip gates and anywhere
//!   else a syntactically invalid document must be rejected rather than partially modeled.
//! - [`parse_for_editor`] -- partial AST + diagnostics, never fails. Use this for IDE/LSP
//!   features (outline, hover, semantic tokens) that need to keep working while the user is
//!   mid-edit.
//!
//! **Invariant:** for any input where [`parse`] succeeds, [`parse_for_editor`] on the same input
//! reports zero diagnostics and builds the identical document-local reference arena and AST (once
//! spans are normalized out -- see [`ast::RootNamespace::normalize_for_test_comparison`]). Don't
//! mix the two entry points for the same document within one caller (e.g. parsing once with each
//! and comparing/reparsing across them) -- that was the GH-66/GH-69 bug class, where an apparent
//! AST mismatch was really just the two entry points disagreeing, not a real emit/parser bug.
//! Covered by `tests/validation/parse_entry_point_equivalence.rs` (GH-70).
// The deeply mutually recursive AST (type bodies nest parameters that nest type bodies, e.g.
// `TypedParameterMember` -> `CalcDefBody` -> `InOutDecl` -> `ActionDefBodyElement` -> ...)
// exceeds the default trait-solver recursion limit when rustdoc computes auto traits such as
// `RefUnwindSafe`.
#![recursion_limit = "256"]
#![cfg_attr(
    not(test),
    deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)
)]
#![cfg_attr(not(test), warn(clippy::wildcard_enum_match_arm))]

pub mod ast;
pub mod emit;
pub mod error;
pub mod parser;

pub use ast::{
    ActionDef, ActionDefBody, ActionDefBodyElement, ActionUsage, ActionUsageBody,
    ActionUsageBodyElement, AliasBody, AliasDef, AllocationDef, AllocationUsage, AnalysisCaseDef,
    AnalysisCaseUsage, Argument, AstNode, AttributeBody, AttributeDef, AttributeUsage,
    BasicUsagePrefix, Bind, CaseDef, CaseUsage, CollectionOperator, CollectionOperatorBody,
    CollectionOperatorParameter, CollectionOperatorParameterTyping, CommentAnnotation, Connect,
    ConnectStmt, ConnectionDef, ConnectionDefBody, ConnectionDefBodyElement, DeclarationName,
    Dependency, DerivationConnectionRole, DerivationEndRole, DocComment, EndDecl, EndIdentity,
    Expression, FilterMember, FilterPackageMember, FirstMergeBody, FirstMergeBodyElement,
    FirstMergeBraceBody, FirstStmt, FlowDef, FlowUsage, FlowUsageKind, Identification, Import,
    ImportShape, ImportSuffixSpans, ImportTarget, InOut, InOutDecl, InlineRequirementDeclaration,
    InterfaceDef, InterfaceDefBody, InterfaceDefBodyElement, InterfaceUsage,
    InterfaceUsageBodyElement, ItemUsage, LoopStmt, MergeStmt, NamespaceDecl, Node,
    OccurrenceBodyElement, OccurrencePortionKind, OccurrenceUsage, OccurrenceUsageBody,
    OccurrenceUsagePrefix, Package, PackageBody, PackageBodyElement, ParseErrorNode,
    ParsedDocument, PartDef, PartDefBody, PartDefBodyElement, PartUsage, PartUsageBody,
    PartUsageBodyElement, PayloadFeature, Perform, PerformBody, PerformBodyElement,
    PerformInOutBinding, PortBody, PortBodyElement, PortDef, PortDefBody, PortDefBodyElement,
    PortUsage, QualifiedDeclarationName, QualifiedIdentification, QualifiedReferenceArena,
    QualifiedReferenceId, QualifiedReferenceMetadata, QualifiedReferenceValidationError,
    QualifiedReferenceView, RefBody, RefDecl, RefPrefix, ReferenceSegment, ReferenceSeparator,
    RelationshipBodyElement, RequireConstraint, RequirementDef, RequirementDefBody,
    RequirementDefBodyElement, RequirementUsage, ReturnRef, ReturnRefBody, ReturnRefBodyElement,
    RootElement, RootNamespace, SatisfactionSubject, SatisfiedRequirement, SatisfyRequirementUsage,
    SegmentRange, SourceStorage, Span, TextualRepresentation, ThenAction, ThenTarget,
    TypeCheckKind, UsageExtensionKeyword, VerificationCaseDef, VerificationCaseUsage, Visibility,
};
pub use emit::{
    emit_recovered_sysml, emit_sysml, emit_sysml_with_options, opacity_report, EmitError,
    EmitOptions, OpacityHit, OpacityKind, OpacityReport,
};
pub use error::{DiagnosticCategory, DiagnosticSeverity, ParseError};

/// Incremented on every breaking AST change. The parse cache uses this to
/// invalidate entries built against an older schema.
pub const PARSE_AST_VERSION: u32 = 175;

/// The pinned grammar release understood by this build of the parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupportedGrammar {
    pub release_tag: &'static str,
    pub release_repository: &'static str,
    /// Hash over the pinned SysML and KerML textual BNF filenames and bytes.
    pub content_hash: &'static str,
}

/// Authoritative SysML/KerML grammar pin, generated from `docs/conformance-target` at build time.
pub const SUPPORTED_GRAMMAR: SupportedGrammar = SupportedGrammar {
    release_tag: env!("SYSML_PARSER_RELEASE_TAG"),
    release_repository: env!("SYSML_PARSER_RELEASE_REPO"),
    content_hash: env!("SYSML_PARSER_GRAMMAR_CONTENT_HASH"),
};
pub use parser::{
    parse_root, parse_root_owned, parse_with_diagnostics, parse_with_diagnostics_owned, ParseResult,
};

/// Parse a SysML v2 textual input into an atomic source/arena/AST document.
///
/// Returns an error if the input is not valid SysML or if not all input is consumed. See the
/// crate-level "Entry points" section for how this relates to [`parse_for_editor`].
#[allow(clippy::result_large_err)]
pub fn parse(input: &str) -> Result<ParsedDocument, ParseError> {
    parse_root(input)
}

/// Parse an owned SysML v2 source buffer without cloning the complete document.
#[allow(clippy::result_large_err)]
pub fn parse_owned(input: String) -> Result<ParsedDocument, ParseError> {
    parse_root_owned(input)
}

/// Parse for editor/LSP use: returns a partial AST plus diagnostics, never fails.
///
/// Prefer this over [`parse`] when you want IDE features (outline/hover/semantic tokens) to keep
/// working even when the file contains syntax errors. See the crate-level "Entry points" section
/// for the equivalence guarantee between the two on clean input.
pub fn parse_for_editor(input: &str) -> ParseResult {
    parse_with_diagnostics(input)
}

/// Parse an owned source buffer for editor/LSP use without cloning the complete document.
pub fn parse_for_editor_owned(input: String) -> ParseResult {
    parse_with_diagnostics_owned(input)
}

#[cfg(test)]
mod owned_source_tests {
    #[test]
    fn parse_owned_reuses_the_callers_source_allocation() {
        let source = String::from("package Owned;");
        let allocation = source.as_ptr();
        let document = super::parse_owned(source).expect("owned parse");
        assert_eq!(document.source.as_str().as_ptr(), allocation);
    }

    #[test]
    fn parse_for_editor_owned_reuses_the_callers_source_allocation() {
        let source = String::from("not valid");
        let allocation = source.as_ptr();
        let result = super::parse_for_editor_owned(source);
        assert_eq!(result.document.source.as_str().as_ptr(), allocation);
    }
}
