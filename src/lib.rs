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
//! reports zero diagnostics and builds the identical AST (once spans are normalized out -- see
//! [`ast::RootNamespace::normalize_for_test_comparison`]). Don't mix the two entry points for the
//! same document within one caller (e.g. parsing once with each and comparing/reparsing across
//! them) -- that was the GH-66/GH-69 bug class, where an apparent AST mismatch was really just
//! the two entry points disagreeing, not a real emit/parser bug. Covered by
//! `tests/validation/parse_entry_point_equivalence.rs` (GH-70).
#![cfg_attr(
    not(test),
    deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)
)]

pub mod ast;
pub mod emit;
pub mod error;
pub mod parser;

pub use ast::{
    ActionDef, ActionDefBody, ActionDefBodyElement, ActionUsage, ActionUsageBody,
    ActionUsageBodyElement, AliasBody, AliasDef, AllocationDef, AllocationUsage, AnalysisCaseDef,
    AnalysisCaseUsage, Annotation, Argument, AstNode, AttributeBody, AttributeDef, AttributeUsage,
    Bind, CaseDef, CaseUsage, CollectionOperator, CommentAnnotation, Connect, ConnectBody,
    ConnectStmt, ConnectionDef, ConnectionDefBody, ConnectionDefBodyElement, DocComment, EndDecl,
    Expression, FeatureChain, FilterMember, FilterPackageMember, FirstMergeBody, FirstStmt,
    FlowDef, FlowUsage, FlowUsageKind, Identification, Import, InOut, InOutDecl, InterfaceDef,
    InterfaceDefBody, InterfaceDefBodyElement, InterfaceUsage, InterfaceUsageBodyElement,
    ItemUsage, LoopStmt, MergeStmt, NamespaceDecl, Node, OccurrenceBodyElement, OccurrenceUsage,
    OccurrenceUsageBody, Package, PackageBody, PackageBodyElement, ParseErrorNode, PartDef,
    PartDefBody, PartDefBodyElement, PartUsage, PartUsageBody, PartUsageBodyElement,
    PayloadFeature, Perform, PerformBody, PerformBodyElement, PerformInOutBinding, PortBody,
    PortBodyElement, PortDef, PortDefBody, PortDefBodyElement, PortUsage, RefBody, RefBodyElement,
    RefDecl, RelationshipBodyElement, RequireConstraint, RequireConstraintBody, RequirementDef,
    RequirementDefBody, RequirementDefBodyElement, RequirementUsage, RootElement, RootNamespace,
    Span, TextualRepresentation, ThenAction, ThenTarget, TypeCheckKind, VerificationCaseDef,
    VerificationCaseUsage, Visibility,
};
pub use emit::{
    emit_sysml, emit_sysml_with_options, opacity_report, EmitError, EmitOptions, OpacityHit,
    OpacityKind, OpacityReport,
};
pub use error::{DiagnosticCategory, DiagnosticSeverity, ParseError};

/// Incremented on every breaking AST change. The parse cache uses this to
/// invalidate entries built against an older schema.
pub const PARSE_AST_VERSION: u32 = 77;pub use parser::{parse_root, parse_with_diagnostics, ParseResult};

/// Parse a SysML v2 textual input into a root namespace AST.
///
/// Returns an error if the input is not valid SysML or if not all input is consumed. See the
/// crate-level "Entry points" section for how this relates to [`parse_for_editor`].
#[allow(clippy::result_large_err)]
pub fn parse(input: &str) -> Result<RootNamespace, ParseError> {
    parse_root(input)
}

/// Parse for editor/LSP use: returns a partial AST plus diagnostics, never fails.
///
/// Prefer this over [`parse`] when you want IDE features (outline/hover/semantic tokens) to keep
/// working even when the file contains syntax errors. See the crate-level "Entry points" section
/// for the equivalence guarantee between the two on clean input.
pub fn parse_for_editor(input: &str) -> ParseResult {
    parse_with_diagnostics(input)
}
