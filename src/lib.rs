//! SysML v2 textual notation parser.
//!
//! Reusable library for parsing SysML v2 textual syntax into an AST.
#![cfg_attr(
    not(test),
    deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)
)]

pub mod ast;
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
    ItemUsage, MergeStmt, NamespaceDecl, Node, OccurrenceBodyElement, OccurrenceUsage,
    OccurrenceUsageBody, Package, PackageBody, PackageBodyElement, ParseErrorNode, PartDef,
    PartDefBody, PartDefBodyElement, PartUsage, PartUsageBody, PartUsageBodyElement,
    PayloadFeature, Perform, PerformBody, PerformBodyElement, PerformInOutBinding, PortBody,
    PortBodyElement, PortDef, PortDefBody, PortDefBodyElement, PortUsage, RefBody, RefDecl,
    RequireConstraint, RequireConstraintBody, RequirementDef, RequirementDefBody,
    RequirementDefBodyElement, RequirementUsage, RootElement, RootNamespace, Span,
    TextualRepresentation, TypeCheckKind, VerificationCaseDef, VerificationCaseUsage, Visibility,
};
pub use error::{DiagnosticCategory, DiagnosticSeverity, ParseError};

/// Incremented on every breaking AST change. The parse cache uses this to
/// invalidate entries built against an older schema.
pub const PARSE_AST_VERSION: u32 = 50;
pub use parser::{parse_root, parse_with_diagnostics, ParseResult};

/// Parse a SysML v2 textual input into a root namespace AST.
///
/// Returns an error if the input is not valid SysML or if not all input is consumed.
#[allow(clippy::result_large_err)]
pub fn parse(input: &str) -> Result<RootNamespace, ParseError> {
    parse_root(input)
}

/// Parse for editor/LSP use: returns a partial AST plus diagnostics.
///
/// Prefer this over [`parse_root`] when you want IDE features (outline/hover/semantic tokens) to
/// keep working even when the file contains syntax errors.
pub fn parse_for_editor(input: &str) -> ParseResult {
    parse_with_diagnostics(input)
}
