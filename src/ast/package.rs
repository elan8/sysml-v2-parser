use super::behavior::{
    ActionDef, ActionUsage, AllocationDef, AllocationUsage, FlowDef, FlowUsage, StateDef,
    StateUsage,
};
use super::common::FilterMember;
use super::common::{CommentAnnotation, DocComment, Import, ParseErrorNode, TextualRepresentation};
use super::kerml_fallback::{
    ClassifierDecl, ExtendedLibraryDecl, FeatureDecl, KermlFeatureDecl, KermlSemanticDecl,
};
use super::requirement::{
    ActorDecl, AnalysisCaseDef, AnalysisCaseUsage, CaseDef, CaseUsage, ConcernUsage, Dependency,
    EnumerationUsage, ItemUsage, RequirementDef, RequirementUsage, Satisfy, UseCaseDef,
    UseCaseUsage, VerificationCaseDef, VerificationCaseUsage,
};
use super::structure::{
    AliasDef, AssertConstraintMember, AttributeDef, AttributeUsage, Connect, ConnectionDef,
    ConnectionUsageMember, DefaultReferenceUsage, EnumDef, IndividualDef, InterfaceDef,
    InterfaceUsage, ItemDef, MetadataDef, MetadataKeywordUsage, MetadataUsage, OccurrenceDef,
    OccurrenceUsage, PartDef, PartUsage, PortDef, PortUsage, RefDecl,
};
use super::view::{
    CalcDef, ConstraintDef, ConstraintUsage, RenderingDef, RenderingUsage, ViewDef, ViewUsage,
    ViewpointDef, ViewpointUsage,
};
use crate::ast::core::Node;
use crate::ast::QualifiedReferenceId;

/// A qualified declaration name stored in the document's packed qualified-name arena.
///
/// This wrapper deliberately distinguishes a namespace declaration's identity from a semantic
/// reference, even though both reuse the same source-backed storage primitive. Consumers resolve
/// it through [`crate::ast::ParsedDocument::qualified_declaration_name`] rather than treating the
/// underlying arena identity as a reference role.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QualifiedDeclarationName {
    reference: QualifiedReferenceId,
}

impl QualifiedDeclarationName {
    pub(crate) const fn new(reference: QualifiedReferenceId) -> Self {
        Self { reference }
    }

    pub(crate) const fn storage_id(self) -> QualifiedReferenceId {
        self.reference
    }
}

/// The authored main name of a package, library package, or namespace declaration.
///
/// Simple declaration labels are not references. Qualified declaration paths need packed segment,
/// separator, scope, and span provenance, so they use a distinct declaration-role wrapper around
/// the shared source-backed arena storage.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeclarationName {
    Simple(String),
    Qualified(QualifiedDeclarationName),
}

/// Identification used by namespace-owning declarations.
///
/// Unlike the general [`Identification`] grammar node, its main name may be a qualified path.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QualifiedIdentification {
    pub short_name: Option<String>,
    pub name: Option<DeclarationName>,
}

impl QualifiedIdentification {
    /// Return the declaration label only when the authored name is the simple-name alternative.
    /// Qualified declarations remain arena-backed and must be resolved through their document.
    pub fn simple_name(&self) -> Option<&str> {
        match self.name.as_ref()? {
            DeclarationName::Simple(name) => Some(name),
            DeclarationName::Qualified(_) => None,
        }
    }
}

/// A package declaration: `package` Identification PackageBody
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Package {
    pub identification: QualifiedIdentification,
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
    pub identification: QualifiedIdentification,
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
    /// Keyword-less `name;` / `name = expr;` feature binding at package scope (§6 G26, GH-87),
    /// e.g. `pressure = force / length^2;` (v1 Spec Examples/8.4.1 Wheel Hub Assembly/Wheel
    /// Package.sysml:9). Previously only reachable inside part/attribute/action bodies.
    DefaultReferenceUsage(Node<DefaultReferenceUsage>),
    /// `assert (not)? (constraint)? <name>? (: Type)? { ... }` at package scope (GH-89), e.g.
    /// `assert not massLimitation { :>> mass = vehicle3.mass; ... }` (Simple Tests/
    /// ConstraintTest.sysml:89). Previously dispatched in six other body contexts (action, part
    /// def/usage, connection def, occurrence, attribute) but not at package scope.
    AssertConstraint(Node<AssertConstraintMember>),
}
