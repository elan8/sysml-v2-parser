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
    RequirementDef, RequirementUsage, Satisfy, UseCaseDef, UseCaseUsage, VerificationCaseDef,
    VerificationCaseUsage,
};
use super::structure::{
    AliasDef, AttributeDef, ConnectionDef, EnumDef, IndividualDef, InterfaceDef, ItemDef,
    MetadataDef, MetadataUsage, OccurrenceDef, OccurrenceUsage, PartDef, PartUsage, PortDef,
};
use super::view::{
    CalcDef, ConstraintDef, RenderingDef, RenderingUsage, ViewDef, ViewUsage, ViewpointDef,
    ViewpointUsage,
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
}
