//! Explicit auto-trait implementations that keep downstream trait proofs shallow.
//!
//! The AST is a deeply mutually recursive tree -- a requirement body holds a part usage holds a
//! perform holds a variant requirement holds a requirement body -- so the longest simple path
//! through its type graph is over 120 distinct types. Proving an auto trait (`Send`, `Sync`,
//! `Unpin`, `UnwindSafe`, `RefUnwindSafe`) structurally for anything that contains or borrows
//! AST content walks that path, which costs more trait-solver frames than rustc's default
//! `recursion_limit` of 128 allows. Every consumer that stores a document or an expression, holds
//! a borrowed view into a body, or merely documents such a type (rustdoc derives all five auto
//! traits for every documented item) then had to raise its own limit.
//!
//! Every recursive edge of the AST passes through [`Node`], so [`Node`] is where the walk is
//! cut. `Node<T>` implements the auto traits for every AST payload `T`, recognized by the sealed
//! [`AstPayload`] marker, whose check is a single impl lookup. A consumer's proof therefore stops
//! at the first `Node` it meets, however deep the tree behind it is.
//!
//! # Why this is sound
//!
//! `Node<T>` is a span plus a `T`, so it has an auto trait exactly when `T` does. The obligation
//! "every `AstPayload` type has the five auto traits" is not discharged by review: [`payloads!`]
//! asserts them structurally for each listed type, in this crate, where `cargo check` enforces it.
//! Those assertions are shallow -- they stop at the type's own `Node` fields -- and they hold
//! together by induction: each payload is proven from its fields, and a field `Node<U>` is in turn
//! covered by `U`'s own assertion, because `U` must be a listed payload for `Node<U>` to have the
//! traits at all.
//!
//! # Completeness
//!
//! A payload missing from the list makes `Node<Missing>` lose its auto traits, which fails
//! [`document_proof`] (and every consumer) to compile. A new AST node type therefore has to be
//! added here before anything can hold it.

use std::panic::{RefUnwindSafe, UnwindSafe};

use super::*;

mod sealed {
    /// Implemented only by [`super::payloads!`], next to each payload's structural proof.
    pub trait AstPayload {}
}
use sealed::AstPayload;

// SAFETY: `Node<T>` is `{ span: Span, value: T }`; `Span` is plain data and every `AstPayload`
// `T` is proven `Send` by `payloads!` below.
unsafe impl<T: AstPayload> Send for Node<T> {}
// SAFETY: as above, with every `AstPayload` proven `Sync`.
unsafe impl<T: AstPayload> Sync for Node<T> {}
impl<T: AstPayload> Unpin for Node<T> {}
impl<T: AstPayload> UnwindSafe for Node<T> {}
impl<T: AstPayload> RefUnwindSafe for Node<T> {}

fn assert_auto_traits<T: Send + Sync + Unpin + UnwindSafe + RefUnwindSafe>() {}

/// Marks each type as an AST payload and asserts, structurally, that it has every auto trait.
macro_rules! payloads {
    ($($payload:ty),* $(,)?) => {
        $(impl AstPayload for $payload {})*

        /// The structural proofs behind the `Node` implementations above.
        #[allow(dead_code)]
        fn payload_proofs() {
            $(assert_auto_traits::<$payload>();)*
        }
    };
}

payloads!(
    ActionDef,
    ActionDefBodyElement,
    ActionNodeUsageDeclaration,
    ActionUsage,
    ActionUsageBodyElement,
    ActorDecl,
    ActorUsage,
    AliasDef,
    Allocate,
    AllocationDef,
    AllocationUsage,
    AnalysisCaseDef,
    AnalysisCaseUsage,
    AssertConstraintMember,
    AssignStmt,
    AttributeBodyElement,
    AttributeDef,
    AttributeUsage,
    Bind,
    BindingConnectorUsage,
    CalcDef,
    CalcDefBodyElement,
    CalcUsage,
    CaseDef,
    CaseReturnDecl,
    CaseUsage,
    CollectionOperatorBody,
    CollectionOperatorParameter,
    CommentAnnotation,
    ConcernUsage,
    Conjugation,
    Connect,
    ConnectStmt,
    ConnectionDef,
    ConnectionDefBodyElement,
    ConnectionEnd,
    ConnectionUsageMember,
    ConstraintDef,
    ConstraintDefBodyElement,
    ConstraintUsage,
    DecisionStmt,
    DefaultReferenceUsage,
    DefinitionBodyElement,
    DefinitionPrefix,
    Dependency,
    DoAction,
    DocComment,
    EndDecl,
    EntryAction,
    EnumDef,
    EnumeratedValue,
    EnumerationBodyElement,
    EnumerationUsage,
    ExhibitState,
    ExitAction,
    ExposeMember,
    Expression,
    ExtendedDefinition,
    ExtendedUsage,
    FeaturePortionKind,
    FeatureRelationshipPart,
    FeatureValue,
    FeatureVariability,
    FilterMember,
    FilterPackageMember,
    FinalState,
    FirstMergeBodyElement,
    FirstStmt,
    FlowDef,
    FlowPayloadClause,
    FlowUsage,
    ForLoop,
    ForkStmt,
    FrameMember,
    GuardedSuccession,
    IfStmt,
    Import,
    InOut,
    InOutDecl,
    InOutDeclKind,
    IncludeUseCase,
    IndividualDef,
    InlineRequirementDeclaration,
    InterfaceDef,
    InterfaceDefBodyElement,
    InterfaceEnd,
    InterfacePart,
    InterfaceUsage,
    InterfaceUsageBodyElement,
    ItemDef,
    ItemUsage,
    JoinStmt,
    KermlBareDeclaration,
    KermlBindingEndPair,
    KermlBindingMember,
    KermlClassifierDecl,
    KermlConnectorEnd,
    KermlConnectorMember,
    KermlFeature,
    KermlFeatureKind,
    KermlInvariantMember,
    KermlRelationshipDecl,
    KermlSuccessionMember,
    KermlTypeRelationship,
    LibraryPackage,
    LoopStmt,
    MergeStmt,
    MetadataAnnotation,
    MetadataBodyElement,
    MetadataBodyUsage,
    MetadataDeclaredName,
    MetadataDef,
    MetadataKeywordUsage,
    MetadataUsage,
    Multiplicity,
    MultiplicityOrdering,
    MultiplicityUniqueness,
    NamespaceDecl,
    Objective,
    OccurrenceBodyElement,
    OccurrenceDef,
    OccurrencePortionKind,
    OccurrenceUsage,
    OwnedCrossFeature,
    OwnedCrossUsage,
    Package,
    PackageBodyElement,
    ParseErrorNode,
    PartDef,
    PartDefBodyElement,
    PartUsage,
    PartUsageBodyElement,
    PayloadFeature,
    Perform,
    PerformBodyElement,
    PerformInOutBinding,
    PortBodyElement,
    PortDef,
    PortDefBodyElement,
    PortUsage,
    PurposeMember,
    RefDecl,
    RefRedefinition,
    RelationshipBodyElement,
    RenderingDef,
    RenderingDefBodyElement,
    RenderingUsage,
    RenderingUsageBodyElement,
    RequireConstraint,
    RequirementActorDecl,
    RequirementDef,
    RequirementDefBodyElement,
    RequirementUsage,
    ReturnDecl,
    ReturnRef,
    ReturnRefBody,
    ReturnRefBodyElement,
    RootElement,
    SatisfactionSubject,
    SatisfyRequirementUsage,
    SequenceExpressionList,
    StakeholderMember,
    StateBodyModifier,
    StateActionUsage,
    StateDef,
    StateDefBodyElement,
    StateUsage,
    SubjectDecl,
    SubjectRef,
    SubsettingRelationship,
    SuccessionUsage,
    TerminateStmt,
    TextualRepresentation,
    ThenAction,
    ThenIncludeUseCase,
    ThenStmt,
    ThenUseCaseUsage,
    Transition,
    TransitionAccept,
    TypeFeaturingPart,
    TypingRelationship,
    UnsupportedGrammarNode,
    UsageDeclaration,
    UsageExtensionKeyword,
    UseCaseDef,
    UseCaseDefBody,
    UseCaseDefBodyElement,
    UseCaseUsage,
    VariantUsage,
    VerificationCaseDef,
    VerificationCaseUsage,
    VerifyRequirementMember,
    ViewBodyElement,
    ViewDef,
    ViewDefBodyElement,
    ViewRenderingUsage,
    ViewUsage,
    ViewpointDef,
    ViewpointUsage,
    WhileStmt,
);

/// The document itself, proven structurally: this is the check that fails when a payload type
/// reachable from a document is missing from [`payloads!`].
#[allow(dead_code)]
fn document_proof() {
    assert_auto_traits::<ParsedDocument>();
    assert_auto_traits::<Expression>();
}
