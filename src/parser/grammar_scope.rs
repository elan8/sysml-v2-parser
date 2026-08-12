//! Scope-specific grammar support classification.
//!
//! Each scope owns one declaration of its recognized FIRST keywords. The recovery starter slice is
//! generated from the same declaration, so dispatch/recovery cannot silently drift from support
//! taxonomy.

#[cfg(test)]
#[path = "../../tests/common/kebnf.rs"]
mod kebnf_test_support;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GrammarStarter {
    pub keyword: &'static [u8],
    pub production: PackageProduction,
    pub origin: GrammarOrigin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GrammarOrigin {
    Spec,
    ParserExtension,
}

macro_rules! grammar_origin {
    () => {
        GrammarOrigin::Spec
    };
    (Extension) => {
        GrammarOrigin::ParserExtension
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PackageProduction {
    PrefixMetadataMember,
    DefinitionElement,
    Action,
    ActorUsage,
    AliasMember,
    Allocation,
    AnalysisCase,
    AssertConstraintUsage,
    Constraint,
    Attribute,
    BindingConnectorAsUsage,
    Calculation,
    Case,
    Comment,
    Concern,
    Connection,
    Dependency,
    Documentation,
    Enumeration,
    Expose,
    ElementFilterMember,
    Flow,
    RequirementBodyItem,
    Import,
    Individual,
    Interface,
    Item,
    LibraryPackage,
    Metadata,
    Namespace,
    Occurrence,
    Package,
    Part,
    Port,
    MemberPrefix,
    ViewRenderingUsage,
    Rendering,
    TextualRepresentation,
    Requirement,
    SatisfyRequirementUsage,
    UseCase,
    VerificationCase,
    View,
    Viewpoint,
    FeaturePrefix,
    UsagePrefix,
    Connector,
    Message,
    Succession,
    PerformActionUsage,
    ExhibitStateUsage,
    IncludeUseCaseUsage,
    TextualRepresentationLanguage,
}

impl PackageProduction {
    /// Whether this production only marks what *may* follow, leaving the element's real keyword
    /// still ahead: a `#` metadata tag, a visibility or feature/usage prefix, the
    /// `abstract`/`variation` markers that precede any definition, or `individual`, which
    /// qualifies the occurrence definition or usage that follows it (`individual part def P`).
    /// A starter of this kind cannot select a production.
    pub(crate) const fn is_prefix(self) -> bool {
        matches!(
            self,
            Self::PrefixMetadataMember
                | Self::Individual
                | Self::MemberPrefix
                | Self::UsagePrefix
                | Self::FeaturePrefix
                | Self::DefinitionElement
        )
    }

    pub(crate) const fn bnf_name(self) -> &'static str {
        match self {
            Self::PrefixMetadataMember => "PrefixMetadataMember",
            Self::DefinitionElement => "DefinitionElement",
            Self::Action => "ActionDefinition | ActionUsage",
            Self::ActorUsage => "ActorUsage",
            Self::AliasMember => "AliasMember",
            Self::Allocation => "AllocationDefinition | AllocationUsage",
            Self::AnalysisCase => "AnalysisCaseDefinition | AnalysisCaseUsage",
            Self::AssertConstraintUsage => "AssertConstraintUsage",
            Self::Constraint => "ConstraintDefinition | ConstraintUsage",
            Self::Attribute => "AttributeDefinition | AttributeUsage",
            Self::BindingConnectorAsUsage => "BindingConnectorAsUsage",
            Self::Calculation => "CalculationDefinition | CalculationUsage",
            Self::Case => "CaseDefinition | CaseUsage",
            Self::Comment => "Comment",
            Self::Concern => "ConcernDefinition | ConcernUsage",
            Self::Connection => "ConnectionDefinition | ConnectionUsage",
            Self::Dependency => "Dependency",
            Self::Documentation => "Documentation",
            Self::Enumeration => "EnumerationDefinition | EnumerationUsage",
            Self::Expose => "Expose",
            Self::ElementFilterMember => "ElementFilterMember",
            Self::Flow => "FlowDefinition | FlowUsage",
            Self::RequirementBodyItem => "RequirementBodyItem",
            Self::Import => "Import",
            Self::Individual => "IndividualDefinition | IndividualUsage",
            Self::Interface => "InterfaceDefinition | InterfaceUsage",
            Self::Item => "ItemDefinition | ItemUsage",
            Self::LibraryPackage => "LibraryPackage",
            Self::Metadata => "MetadataDefinition | MetadataUsage",
            Self::Namespace => "Namespace",
            Self::Occurrence => "OccurrenceDefinition | OccurrenceUsage",
            Self::Package => "Package",
            Self::Part => "PartDefinition | PartUsage",
            Self::Port => "PortDefinition | PortUsage",
            Self::MemberPrefix => "MemberPrefix",
            Self::ViewRenderingUsage => "ViewRenderingUsage",
            Self::Rendering => "RenderingDefinition | RenderingUsage",
            Self::TextualRepresentation => "TextualRepresentation",
            Self::Requirement => "RequirementDefinition | RequirementUsage",
            Self::SatisfyRequirementUsage => "SatisfyRequirementUsage",
            Self::UseCase => "UseCaseDefinition | UseCaseUsage",
            Self::VerificationCase => "VerificationCaseDefinition | VerificationCaseUsage",
            Self::View => "ViewDefinition | ViewUsage",
            Self::Viewpoint => "ViewpointDefinition | ViewpointUsage",
            Self::FeaturePrefix => "FeaturePrefix",
            Self::UsagePrefix => "UsagePrefix",
            Self::Connector => "Connector",
            Self::Message => "Message",
            Self::Succession => "Succession | SuccessionAsUsage",
            Self::PerformActionUsage => "PerformActionUsage",
            Self::ExhibitStateUsage => "ExhibitStateUsage",
            Self::IncludeUseCaseUsage => "IncludeUseCaseUsage",
            Self::TextualRepresentationLanguage => "TextualRepresentation",
        }
    }
}

macro_rules! grammar_scope {
    ($entries:ident, $keywords:ident, [$(($keyword:literal, $production:ident $(, $origin:ident)?)),* $(,)?]) => {
        pub(crate) const $entries: &[GrammarStarter] = &[
            $(GrammarStarter {
                keyword: $keyword,
                production: PackageProduction::$production,
                origin: grammar_origin!($($origin)?),
            }),*
        ];
        pub(crate) const $keywords: &[&[u8]] = &[$($keyword),*];
    };
}

grammar_scope!(
    PACKAGE_BODY_GRAMMAR,
    PACKAGE_BODY_STARTERS,
    [
        (b"#", PrefixMetadataMember),
        (b"abstract", DefinitionElement),
        (b"action", Action),
        (b"alias", AliasMember),
        (b"allocate", Allocation),
        (b"allocation", Allocation),
        (b"analysis", AnalysisCase),
        (b"assert", AssertConstraintUsage),
        (b"attribute", Attribute),
        (b"bind", BindingConnectorAsUsage),
        (b"binding", BindingConnectorAsUsage),
        (b"calc", Calculation),
        (b"case", Case),
        (b"comment", Comment),
        (b"concern", Concern),
        (b"connection", Connection),
        (b"constraint", Constraint),
        (b"dependency", Dependency),
        (b"doc", Documentation),
        (b"enum", Enumeration),
        (b"filter", ElementFilterMember),
        (b"flow", Flow),
        (b"individual", Individual),
        (b"interface", Interface),
        (b"item", Item),
        (b"metadata", Metadata),
        (b"occurrence", Occurrence),
        (b"package", Package),
        (b"part", Part),
        (b"port", Port),
        (b"private", MemberPrefix),
        (b"protected", MemberPrefix),
        (b"public", MemberPrefix),
        (b"rendering", Rendering),
        (b"rep", TextualRepresentation),
        (b"requirement", Requirement),
        (b"snapshot", Occurrence),
        (b"state", Occurrence),
        (b"timeslice", Occurrence),
        (b"use", UseCase),
        (b"variation", DefinitionElement),
        (b"verification", VerificationCase),
        (b"view", View),
        (b"viewpoint", Viewpoint),
        (b"connect", Connector),
        (b"constant", UsagePrefix),
        (b"crosses", FeaturePrefix),
        (b"default", FeaturePrefix),
        (b"defined", FeaturePrefix),
        (b"derived", UsagePrefix),
        (b"end", UsagePrefix),
        (b"event", Occurrence),
        (b"exhibit", ExhibitStateUsage),
        (b"first", Succession),
        (b"in", UsagePrefix),
        (b"include", IncludeUseCaseUsage),
        (b"inout", UsagePrefix),
        (b"language", TextualRepresentationLanguage),
        (b"locale", FeaturePrefix),
        (b"message", Message),
        (b"ordered", FeaturePrefix),
        (b"out", UsagePrefix),
        (b"perform", PerformActionUsage),
        (b"redefines", FeaturePrefix),
        (b"ref", UsagePrefix),
        (b"references", FeaturePrefix),
        (b"standard", LibraryPackage),
        (b"subsets", FeaturePrefix),
        (b"succession", Succession),
        // Accepted parser extensions that remain recovery boundaries, but are deliberately excluded
        // from the spec-FIRST equality lint below.
        (b"actor", ActorUsage, Extension),
        (b"assume", Constraint, Extension),
        (b"expose", Expose, Extension),
        (b"frame", RequirementBodyItem, Extension),
        (b"import", Import, Extension),
        (b"library", LibraryPackage, Extension),
        (b"namespace", Namespace, Extension),
        (b"render", ViewRenderingUsage, Extension),
        (b"require", Requirement, Extension),
        (b"satisfy", SatisfyRequirementUsage, Extension),
    ]
);

pub(crate) fn package_body_starter(fragment: &[u8]) -> Option<GrammarStarter> {
    PACKAGE_BODY_GRAMMAR
        .iter()
        .copied()
        .find(|starter| super::lex::starts_with_keyword(fragment, starter.keyword))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn package_scope_table_matches_spec_derived_first_keywords() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("sysml-v2-release/bnf/SysML-textual-bnf.kebnf");
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let grammar = super::kebnf_test_support::parse_grammar(&source);
        let derived = super::kebnf_test_support::first_terminals(&grammar, "PackageBodyElement")
            .into_iter()
            .filter(|terminal| {
                terminal
                    .as_bytes()
                    .first()
                    .is_some_and(u8::is_ascii_alphabetic)
            })
            .collect::<BTreeSet<_>>();
        let classified = PACKAGE_BODY_GRAMMAR
            .iter()
            .filter(|starter| starter.origin == GrammarOrigin::Spec)
            .filter_map(|starter| std::str::from_utf8(starter.keyword).ok())
            .filter(|keyword| {
                keyword
                    .as_bytes()
                    .first()
                    .is_some_and(u8::is_ascii_alphabetic)
            })
            .map(str::to_owned)
            .collect::<BTreeSet<_>>();

        let missing = derived.difference(&classified).cloned().collect::<Vec<_>>();
        let extra = classified.difference(&derived).cloned().collect::<Vec<_>>();
        assert!(
            missing.is_empty() && extra.is_empty(),
            "package grammar taxonomy drift\nmissing from table: {missing:?}\nnot in derived FIRST: {extra:?}"
        );
    }
}
