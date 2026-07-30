use super::behavior::{
    ActionDef, ActionUsage, AllocationDef, AllocationUsage, FlowDef, FlowUsage, StateDef,
    StateUsage,
};
use super::common::FilterMember;
use super::common::{
    CommentAnnotation, DocComment, Identification, Import, ParseErrorNode, TextualRepresentation,
};
use super::kerml_fallback::{
    ClassifierDecl, ExtendedLibraryDecl, FeatureDecl, KermlFeatureDecl, KermlSemanticDecl,
};
use super::requirement::{
    ActorDecl, AnalysisCaseDef, AnalysisCaseUsage, CaseDef, CaseUsage, ConcernUsage, Dependency,
    EnumerationUsage, ItemUsage, RequirementDef, RequirementUsage, Satisfy, UseCaseDef,
    UseCaseUsage, VerificationCaseDef, VerificationCaseUsage,
};
use super::structure::{
    AliasDef, AttributeDef, AttributeUsage, Connect, ConnectionDef, ConnectionUsageMember, EnumDef,
    IndividualDef, InterfaceDef, InterfaceUsage, ItemDef, MetadataDef, MetadataKeywordUsage,
    MetadataUsage, OccurrenceDef, OccurrenceUsage, PartDef, PartUsage, PortDef, PortUsage, RefDecl,
};
use super::view::{
    CalcDef, ConstraintDef, ConstraintUsage, RenderingDef, RenderingUsage, ViewDef, ViewUsage,
    ViewpointDef, ViewpointUsage,
};
use crate::ast::core::Node;

/// A package declaration: `package` Identification PackageBody
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Package {
    pub identification: Identification,
    pub body: PackageBody,
}
/// Package body: either `;` or `{` PackageBodyElement* `}`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PackageBody {
    /// Semicolon form: no body elements.
    Semicolon,
    /// Brace form: list of body elements (may be empty).
    Brace {
        elements: Vec<Node<PackageBodyElement>>,
    },
}
/// Library package: `library` (optional `standard`) `package` Identification PackageBody (BNF LibraryPackage).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LibraryPackage {
    pub is_standard: bool,
    pub identification: Identification,
    pub body: PackageBody,
}
/// Top-level element inside a namespace or package body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PackageBodyElement {
    Error(Node<ParseErrorNode>),
    Doc(Node<DocComment>),
    Comment(Node<CommentAnnotation>),
    TextualRep(Node<TextualRepresentation>),
    Filter(Node<FilterMember>),
    Package(Node<Package>),
    LibraryPackage(Node<LibraryPackage>),
    Import(Node<Import>),
    PartDef(Node<PartDef>),
    PartUsage(Node<PartUsage>),
    PortDef(Node<PortDef>),
    InterfaceDef(Node<InterfaceDef>),
    AliasDef(Node<AliasDef>),
    AttributeDef(Node<AttributeDef>),
    ActionDef(Node<ActionDef>),
    ActionUsage(Node<ActionUsage>),
    RequirementDef(Node<RequirementDef>),
    RequirementUsage(Node<RequirementUsage>),
    Satisfy(Node<Satisfy>),
    UseCaseDef(Node<UseCaseDef>),
    Actor(Node<ActorDecl>),
    StateDef(Node<StateDef>),
    StateUsage(Node<StateUsage>),
    ItemDef(Node<ItemDef>),
    IndividualDef(Node<IndividualDef>),
    ConstraintDef(Node<ConstraintDef>),
    ConstraintUsage(Node<ConstraintUsage>),
    CalcDef(Node<CalcDef>),
    ViewDef(Node<ViewDef>),
    ViewpointDef(Node<ViewpointDef>),
    RenderingDef(Node<RenderingDef>),
    ViewUsage(Node<ViewUsage>),
    ViewpointUsage(Node<ViewpointUsage>),
    RenderingUsage(Node<RenderingUsage>),
    ConnectionDef(Node<ConnectionDef>),
    MetadataDef(Node<MetadataDef>),
    MetadataUsage(Node<MetadataUsage>),
    EnumDef(Node<EnumDef>),
    OccurrenceDef(Node<OccurrenceDef>),
    OccurrenceUsage(Node<OccurrenceUsage>),
    Dependency(Node<Dependency>),
    AllocationDef(Node<AllocationDef>),
    AllocationUsage(Node<AllocationUsage>),
    FlowDef(Node<FlowDef>),
    FlowUsage(Node<FlowUsage>),
    ConcernUsage(Node<ConcernUsage>),
    CaseDef(Node<CaseDef>),
    CaseUsage(Node<CaseUsage>),
    AnalysisCaseDef(Node<AnalysisCaseDef>),
    AnalysisCaseUsage(Node<AnalysisCaseUsage>),
    VerificationCaseDef(Node<VerificationCaseDef>),
    VerificationCaseUsage(Node<VerificationCaseUsage>),
    UseCaseUsage(Node<UseCaseUsage>),
    FeatureDecl(Node<FeatureDecl>),
    ClassifierDecl(Node<ClassifierDecl>),
    KermlSemanticDecl(Node<KermlSemanticDecl>),
    KermlFeatureDecl(Node<KermlFeatureDecl>),
    ExtendedLibraryDecl(Node<ExtendedLibraryDecl>),
    /// Standalone attribute usage at package level (PAR-002: `PackageMember` in the BNF allows
    /// `DefinitionElement | UsageElement`, so a bare attribute usage is legal package content,
    /// not just attribute definitions).
    AttributeUsage(Node<AttributeUsage>),
    /// Standalone item usage at package level. See `AttributeUsage`.
    ItemUsage(Node<ItemUsage>),
    /// Standalone port usage at package level. See `AttributeUsage`.
    PortUsage(Node<PortUsage>),
    /// Standalone connection usage at package level. See `AttributeUsage`.
    ConnectionUsage(Node<ConnectionUsageMember>),
    /// Standalone interface usage at package level (PAR-007: previously there was no
    /// package-level `interface_usage` dispatch arm at all, so `interface iface : Type connect a
    /// to b;` fell through to `interface_def` and was silently accepted as a definition with the
    /// `connect` clause discarded -- see `interface_def`'s doc comment). See `AttributeUsage` for
    /// the general PAR-002 rationale.
    InterfaceUsage(Node<InterfaceUsage>),
    /// Standalone `ref` declaration at package level. See `AttributeUsage`.
    Ref(Node<RefDecl>),
    /// Standalone enumeration usage at package level. See `AttributeUsage`.
    EnumerationUsage(Node<EnumerationUsage>),
    /// `#keyword` metadata tag at package level, either the bare `#keyword (: Type)? (about
    /// ...)? (;|{...})` form or the `PrefixMetadataMember`-style form prefixing the next package
    /// member (e.g. `#fmeaspec requirement req1 { ... }`, OMG spec Annex `14c-Language
    /// Extensions.sysml`, FMEA library example) -- previously package bodies had no `#`/`@`
    /// annotation support at all, unlike part/item/action/etc. bodies.
    MetadataKeywordUsage(Node<MetadataKeywordUsage>),
    /// Standalone `connect a to b;` connector usage at package level (e.g. the FMEA library's
    /// `#violation connect 'Glucose Meter in Use' to req2;` in `14c-Language Extensions.sysml`).
    /// See `AttributeUsage` for the general PAR-002 rationale.
    Connect(Node<Connect>),
}
