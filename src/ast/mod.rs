//! Abstract syntax tree types for SysML v2 textual notation.

mod core;
mod kerml_fallback;

pub use core::*;
pub use kerml_fallback::*;
mod behavior;
mod common;
mod package;
mod requirement;
mod root;
mod structure;
mod view;

pub use behavior::*;
pub use common::*;
pub use package::*;
pub use requirement::*;
pub use root::*;
pub use structure::*;
pub use view::*;

// ---------------------------------------------------------------------------
// Normalization for test comparison (strips optional spans so parsed == expected)
// ---------------------------------------------------------------------------

impl RootNamespace {
    /// Returns a copy with all optional source spans set to `None` and all `Node` spans set to
    /// `Span::dummy()`. Use when comparing parser output to hand-built expected AST in tests.
    pub fn normalize_for_test_comparison(&self) -> Self {
        RootNamespace {
            elements: self
                .elements
                .iter()
                .map(normalize_root_element_node)
                .collect(),
        }
    }
}

fn dummy_node<T: Clone>(_n: &Node<T>, value: T) -> Node<T> {
    Node::new(Span::dummy(), value)
}

fn normalize_import(imp: &crate::ast::Import) -> crate::ast::Import {
    crate::ast::Import {
        target_span: Span::dummy(),
        ..imp.clone()
    }
}

fn normalize_root_element_node(el: &Node<RootElement>) -> Node<RootElement> {
    let value = match &el.value {
        RootElement::Package(p) => RootElement::Package(dummy_node(p, normalize_package(&p.value))),
        RootElement::LibraryPackage(lp) => {
            RootElement::LibraryPackage(dummy_node(lp, normalize_library_package(&lp.value)))
        }
        RootElement::Namespace(n) => {
            RootElement::Namespace(dummy_node(n, normalize_namespace_decl(&n.value)))
        }
        RootElement::Import(n) => RootElement::Import(dummy_node(n, normalize_import(&n.value))),
    };
    dummy_node(el, value)
}

fn normalize_library_package(lp: &LibraryPackage) -> LibraryPackage {
    LibraryPackage {
        is_standard: lp.is_standard,
        identification: lp.identification.clone(),
        body: normalize_package_body(&lp.body),
    }
}

fn normalize_namespace_decl(n: &NamespaceDecl) -> NamespaceDecl {
    NamespaceDecl {
        identification: n.identification.clone(),
        body: normalize_package_body(&n.body),
    }
}

fn normalize_package(p: &Package) -> Package {
    Package {
        identification: p.identification.clone(),
        body: normalize_package_body(&p.body),
    }
}

fn normalize_package_body(b: &PackageBody) -> PackageBody {
    match b {
        PackageBody::Semicolon => PackageBody::Semicolon,
        PackageBody::Brace { elements } => PackageBody::Brace {
            elements: elements
                .iter()
                .map(normalize_package_body_element_node)
                .collect(),
        },
    }
}

fn normalize_package_body_element_node(el: &Node<PackageBodyElement>) -> Node<PackageBodyElement> {
    let value = match &el.value {
        PackageBodyElement::Error(n) => PackageBodyElement::Error(dummy_node(n, n.value.clone())),
        PackageBodyElement::Doc(n) => PackageBodyElement::Doc(dummy_node(n, n.value.clone())),
        PackageBodyElement::Comment(n) => {
            PackageBodyElement::Comment(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::TextualRep(n) => {
            PackageBodyElement::TextualRep(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::Filter(n) => PackageBodyElement::Filter(dummy_node(n, n.value.clone())),
        PackageBodyElement::Package(n) => {
            PackageBodyElement::Package(dummy_node(n, normalize_package(&n.value)))
        }
        PackageBodyElement::LibraryPackage(n) => {
            PackageBodyElement::LibraryPackage(dummy_node(n, normalize_library_package(&n.value)))
        }
        PackageBodyElement::Import(n) => PackageBodyElement::Import(dummy_node(n, normalize_import(&n.value))),
        PackageBodyElement::PartDef(n) => {
            PackageBodyElement::PartDef(dummy_node(n, normalize_part_def(&n.value)))
        }
        PackageBodyElement::PartUsage(n) => {
            PackageBodyElement::PartUsage(dummy_node(n, normalize_part_usage(&n.value)))
        }
        PackageBodyElement::PortDef(n) => {
            PackageBodyElement::PortDef(dummy_node(n, normalize_port_def(&n.value)))
        }
        PackageBodyElement::InterfaceDef(n) => {
            PackageBodyElement::InterfaceDef(dummy_node(n, normalize_interface_def(&n.value)))
        }
        PackageBodyElement::ConnectionDef(n) => {
            PackageBodyElement::ConnectionDef(dummy_node(n, normalize_connection_def(&n.value)))
        }
        PackageBodyElement::MetadataDef(n) => {
            PackageBodyElement::MetadataDef(dummy_node(n, normalize_metadata_def(&n.value)))
        }
        PackageBodyElement::MetadataUsage(n) => {
            PackageBodyElement::MetadataUsage(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::EnumDef(n) => {
            PackageBodyElement::EnumDef(dummy_node(n, normalize_enum_def(&n.value)))
        }
        PackageBodyElement::OccurrenceDef(n) => {
            PackageBodyElement::OccurrenceDef(dummy_node(n, normalize_occurrence_def(&n.value)))
        }
        PackageBodyElement::OccurrenceUsage(n) => {
            PackageBodyElement::OccurrenceUsage(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::AliasDef(n) => {
            PackageBodyElement::AliasDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::AttributeDef(n) => {
            PackageBodyElement::AttributeDef(dummy_node(n, normalize_attribute_def(&n.value)))
        }
        PackageBodyElement::ActionDef(n) => {
            PackageBodyElement::ActionDef(dummy_node(n, normalize_action_def(&n.value)))
        }
        PackageBodyElement::ActionUsage(n) => {
            PackageBodyElement::ActionUsage(dummy_node(n, normalize_action_usage(&n.value)))
        }
        PackageBodyElement::RequirementDef(n) => {
            PackageBodyElement::RequirementDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::RequirementUsage(n) => {
            PackageBodyElement::RequirementUsage(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::Satisfy(n) => {
            PackageBodyElement::Satisfy(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::UseCaseDef(n) => {
            PackageBodyElement::UseCaseDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::Actor(n) => PackageBodyElement::Actor(dummy_node(n, n.value.clone())),
        PackageBodyElement::StateDef(n) => {
            PackageBodyElement::StateDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::StateUsage(n) => {
            PackageBodyElement::StateUsage(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::ItemDef(n) => {
            PackageBodyElement::ItemDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::IndividualDef(n) => {
            PackageBodyElement::IndividualDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::ConstraintDef(n) => {
            PackageBodyElement::ConstraintDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::CalcDef(n) => {
            PackageBodyElement::CalcDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::ViewDef(n) => {
            PackageBodyElement::ViewDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::ViewpointDef(n) => {
            PackageBodyElement::ViewpointDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::RenderingDef(n) => {
            PackageBodyElement::RenderingDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::ViewUsage(n) => {
            PackageBodyElement::ViewUsage(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::ViewpointUsage(n) => {
            PackageBodyElement::ViewpointUsage(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::RenderingUsage(n) => {
            PackageBodyElement::RenderingUsage(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::Dependency(n) => {
            PackageBodyElement::Dependency(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::AllocationDef(n) => {
            PackageBodyElement::AllocationDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::AllocationUsage(n) => {
            PackageBodyElement::AllocationUsage(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::FlowDef(n) => {
            PackageBodyElement::FlowDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::FlowUsage(n) => {
            PackageBodyElement::FlowUsage(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::ConcernUsage(n) => {
            PackageBodyElement::ConcernUsage(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::CaseDef(n) => {
            PackageBodyElement::CaseDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::CaseUsage(n) => {
            PackageBodyElement::CaseUsage(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::AnalysisCaseDef(n) => {
            PackageBodyElement::AnalysisCaseDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::AnalysisCaseUsage(n) => {
            PackageBodyElement::AnalysisCaseUsage(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::VerificationCaseDef(n) => {
            PackageBodyElement::VerificationCaseDef(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::VerificationCaseUsage(n) => {
            PackageBodyElement::VerificationCaseUsage(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::UseCaseUsage(n) => {
            PackageBodyElement::UseCaseUsage(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::FeatureDecl(n) => {
            PackageBodyElement::FeatureDecl(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::ClassifierDecl(n) => {
            PackageBodyElement::ClassifierDecl(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::KermlSemanticDecl(n) => {
            PackageBodyElement::KermlSemanticDecl(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::KermlFeatureDecl(n) => {
            PackageBodyElement::KermlFeatureDecl(dummy_node(n, n.value.clone()))
        }
        PackageBodyElement::ExtendedLibraryDecl(n) => {
            PackageBodyElement::ExtendedLibraryDecl(dummy_node(n, n.value.clone()))
        }
    };
    dummy_node(el, value)
}

fn normalize_attribute_def(a: &AttributeDef) -> AttributeDef {
    AttributeDef {
        name: a.name.clone(),
        short_name: a.short_name.clone(),
        typing: a.typing.clone(),
        value: a.value.clone(),
        body: a.body.clone(),
        name_span: None,
        typing_span: None,
        value_span: None,
    }
}

fn normalize_part_def(p: &PartDef) -> PartDef {
    PartDef {
        definition_prefix: p.definition_prefix.clone(),
        is_individual: p.is_individual,
        identification: p.identification.clone(),
        specializes: p.specializes.clone(),
        specializes_span: None,
        body: normalize_part_def_body(&p.body),
    }
}

fn normalize_part_def_body(b: &PartDefBody) -> PartDefBody {
    match b {
        PartDefBody::Semicolon => PartDefBody::Semicolon,
        PartDefBody::Brace { elements } => PartDefBody::Brace {
            elements: elements
                .iter()
                .map(normalize_part_def_body_element_node)
                .collect(),
        },
    }
}

fn normalize_part_def_body_element_node(el: &Node<PartDefBodyElement>) -> Node<PartDefBodyElement> {
    let value = match &el.value {
        PartDefBodyElement::Error(n) => PartDefBodyElement::Error(dummy_node(n, n.value.clone())),
        PartDefBodyElement::Doc(n) => PartDefBodyElement::Doc(dummy_node(n, n.value.clone())),
        PartDefBodyElement::Comment(n) => {
            PartDefBodyElement::Comment(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::Annotation(n) => {
            PartDefBodyElement::Annotation(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::MetadataAnnotation(n) => {
            PartDefBodyElement::MetadataAnnotation(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::MetadataKeywordUsage(n) => {
            PartDefBodyElement::MetadataKeywordUsage(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::Other(text) => PartDefBodyElement::Other(text.clone()),
        PartDefBodyElement::AttributeDef(n) => {
            PartDefBodyElement::AttributeDef(dummy_node(n, normalize_attribute_def(&n.value)))
        }
        PartDefBodyElement::AttributeUsage(n) => {
            PartDefBodyElement::AttributeUsage(dummy_node(n, normalize_attribute_usage(&n.value)))
        }
        PartDefBodyElement::RequirementUsage(n) => {
            PartDefBodyElement::RequirementUsage(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::ItemDef(n) => {
            PartDefBodyElement::ItemDef(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::ItemUsage(n) => {
            PartDefBodyElement::ItemUsage(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::Ref(n) => {
            PartDefBodyElement::Ref(dummy_node(n, normalize_ref_decl(&n.value)))
        }
        PartDefBodyElement::PortUsage(n) => {
            PartDefBodyElement::PortUsage(dummy_node(n, normalize_port_usage(&n.value)))
        }
        PartDefBodyElement::PartUsage(n) => {
            PartDefBodyElement::PartUsage(Box::new(dummy_node(n, normalize_part_usage(&n.value))))
        }
        PartDefBodyElement::PartDef(n) => {
            PartDefBodyElement::PartDef(dummy_node(n, normalize_part_def(&n.value)))
        }
        PartDefBodyElement::OccurrenceUsage(n) => {
            PartDefBodyElement::OccurrenceUsage(Box::new(dummy_node(n, n.value.clone())))
        }
        PartDefBodyElement::InterfaceDef(n) => {
            PartDefBodyElement::InterfaceDef(dummy_node(n, normalize_interface_def(&n.value)))
        }
        PartDefBodyElement::InterfaceUsage(n) => {
            PartDefBodyElement::InterfaceUsage(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::Connect(n) => {
            PartDefBodyElement::Connect(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::FlowUsage(n) => {
            PartDefBodyElement::FlowUsage(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::Connection(n) => {
            PartDefBodyElement::Connection(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::Perform(n) => {
            PartDefBodyElement::Perform(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::Allocate(n) => {
            PartDefBodyElement::Allocate(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::OpaqueMember(n) => {
            PartDefBodyElement::OpaqueMember(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::ExhibitState(n) => {
            PartDefBodyElement::ExhibitState(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::CalcUsage(n) => {
            PartDefBodyElement::CalcUsage(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::EnumerationUsage(n) => PartDefBodyElement::EnumerationUsage(
            dummy_node(n, normalize_enumeration_usage(&n.value)),
        ),
        PartDefBodyElement::AssertConstraint(n) => {
            PartDefBodyElement::AssertConstraint(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::Satisfy(n) => {
            PartDefBodyElement::Satisfy(dummy_node(n, n.value.clone()))
        }
        PartDefBodyElement::VariantUsage(n) => {
            PartDefBodyElement::VariantUsage(dummy_node(n, n.value.clone()))
        }
    };
    dummy_node(el, value)
}

fn normalize_enumeration_usage(u: &EnumerationUsage) -> EnumerationUsage {
    EnumerationUsage {
        name: u.name.clone(),
        type_name: u.type_name.clone(),
        multiplicity: u.multiplicity.clone(),
        body: u.body.clone(),
    }
}

fn normalize_attribute_usage(a: &AttributeUsage) -> AttributeUsage {
    AttributeUsage {
        name: a.name.clone(),
        typing: a.typing.clone(),
        subsets: a.subsets.clone(),
        redefines: a.redefines.clone(),
        references: a.references.clone(),
        crosses: a.crosses.clone(),
        value: a.value.clone(),
        body: a.body.clone(),
        name_span: None,
        typing_span: None,
        redefines_span: None,
        direction: a.direction,
    }
}

fn normalize_part_usage(p: &PartUsage) -> PartUsage {
    PartUsage {
        usage_prefix: p.usage_prefix.clone(),
        is_individual: p.is_individual,
        name: p.name.clone(),
        type_name: p.type_name.clone(),
        multiplicity: p.multiplicity.clone(),
        ordered: p.ordered,
        subsets: p.subsets.clone(),
        redefines: p.redefines.clone(),
        value: p.value.clone(),
        body: normalize_part_usage_body(&p.body),
        name_span: None,
        type_ref_span: None,
    }
}

fn normalize_part_usage_body(b: &PartUsageBody) -> PartUsageBody {
    match b {
        PartUsageBody::Semicolon => PartUsageBody::Semicolon,
        PartUsageBody::Brace { elements } => PartUsageBody::Brace {
            elements: elements
                .iter()
                .map(normalize_part_usage_body_element_node)
                .collect(),
        },
    }
}

fn normalize_perform(p: &Perform) -> Perform {
    Perform {
        action_name: p.action_name.clone(),
        type_name: p.type_name.clone(),
        body: normalize_perform_body(&p.body),
    }
}

fn normalize_perform_body(b: &PerformBody) -> PerformBody {
    match b {
        PerformBody::Semicolon => PerformBody::Semicolon,
        PerformBody::Brace { elements } => PerformBody::Brace {
            elements: elements
                .iter()
                .map(normalize_perform_body_element_node)
                .collect(),
        },
    }
}

fn normalize_perform_body_element_node(el: &Node<PerformBodyElement>) -> Node<PerformBodyElement> {
    let value = match &el.value {
        PerformBodyElement::Doc(n) => PerformBodyElement::Doc(dummy_node(n, n.value.clone())),
        PerformBodyElement::InOut(n) => PerformBodyElement::InOut(dummy_node(
            n,
            PerformInOutBinding {
                direction: n.value.direction,
                name: n.value.name.clone(),
                value: normalize_expression_node(&n.value.value),
            },
        )),
    };
    dummy_node(el, value)
}

fn normalize_expression_node(node: &Node<Expression>) -> Node<Expression> {
    let value = match &node.value {
        Expression::LiteralInteger(x) => Expression::LiteralInteger(*x),
        Expression::LiteralReal(s) => Expression::LiteralReal(s.clone()),
        Expression::LiteralString(s) => Expression::LiteralString(s.clone()),
        Expression::LiteralBoolean(b) => Expression::LiteralBoolean(*b),
        Expression::FeatureRef(s) => Expression::FeatureRef(s.clone()),
        Expression::MemberAccess(base, member) => {
            Expression::MemberAccess(Box::new(normalize_expression_node(base)), member.clone())
        }
        Expression::Index { base, index } => Expression::Index {
            base: Box::new(normalize_expression_node(base)),
            index: Box::new(normalize_expression_node(index)),
        },
        Expression::Bracket(inner) => {
            Expression::Bracket(Box::new(normalize_expression_node(inner)))
        }
        Expression::LiteralWithUnit { value: v, unit } => Expression::LiteralWithUnit {
            value: Box::new(normalize_expression_node(v)),
            unit: Box::new(normalize_expression_node(unit)),
        },
        Expression::BinaryOp { op, left, right } => Expression::BinaryOp {
            op: op.clone(),
            left: Box::new(normalize_expression_node(left)),
            right: Box::new(normalize_expression_node(right)),
        },
        Expression::UnaryOp { op, operand } => Expression::UnaryOp {
            op: op.clone(),
            operand: Box::new(normalize_expression_node(operand)),
        },
        Expression::Invocation { callee, args } => Expression::Invocation {
            callee: Box::new(normalize_expression_node(callee)),
            args: args.iter().map(normalize_expression_node).collect(),
        },
        Expression::Tuple(items) => {
            Expression::Tuple(items.iter().map(normalize_expression_node).collect())
        }
        Expression::Classification { metaclass } => Expression::Classification {
            metaclass: metaclass.clone(),
        },
        Expression::MetaCast { base, metaclass } => Expression::MetaCast {
            base: Box::new(normalize_expression_node(base)),
            metaclass: metaclass.clone(),
        },
        Expression::TypeCheck {
            kind,
            operand,
            type_name,
        } => Expression::TypeCheck {
            kind: kind.clone(),
            operand: operand
                .as_ref()
                .map(|node| Box::new(normalize_expression_node(node))),
            type_name: type_name.clone(),
        },
        Expression::Select { base, selector } => Expression::Select {
            base: Box::new(normalize_expression_node(base)),
            selector: selector.clone(),
        },
        Expression::Collect { base, selector } => Expression::Collect {
            base: Box::new(normalize_expression_node(base)),
            selector: selector.clone(),
        },
        Expression::Null => Expression::Null,
    };
    Node::new(Span::dummy(), value)
}

fn normalize_part_usage_body_element_node(
    el: &Node<PartUsageBodyElement>,
) -> Node<PartUsageBodyElement> {
    let value = match &el.value {
        PartUsageBodyElement::Error(n) => {
            PartUsageBodyElement::Error(dummy_node(n, n.value.clone()))
        }
        PartUsageBodyElement::Doc(n) => PartUsageBodyElement::Doc(dummy_node(n, n.value.clone())),
        PartUsageBodyElement::Annotation(n) => {
            PartUsageBodyElement::Annotation(dummy_node(n, n.value.clone()))
        }
        PartUsageBodyElement::AttributeUsage(n) => {
            PartUsageBodyElement::AttributeUsage(dummy_node(n, normalize_attribute_usage(&n.value)))
        }
        PartUsageBodyElement::EnumerationUsage(n) => PartUsageBodyElement::EnumerationUsage(
            dummy_node(n, normalize_enumeration_usage(&n.value)),
        ),
        PartUsageBodyElement::PartUsage(n) => {
            PartUsageBodyElement::PartUsage(Box::new(dummy_node(n, normalize_part_usage(&n.value))))
        }
        PartUsageBodyElement::OccurrenceUsage(n) => {
            PartUsageBodyElement::OccurrenceUsage(Box::new(dummy_node(n, n.value.clone())))
        }
        PartUsageBodyElement::PortUsage(n) => {
            PartUsageBodyElement::PortUsage(dummy_node(n, normalize_port_usage(&n.value)))
        }
        PartUsageBodyElement::Ref(n) => {
            PartUsageBodyElement::Ref(dummy_node(n, normalize_ref_decl(&n.value)))
        }
        PartUsageBodyElement::Bind(n) => PartUsageBodyElement::Bind(dummy_node(n, n.value.clone())),
        PartUsageBodyElement::InterfaceUsage(n) => {
            PartUsageBodyElement::InterfaceUsage(dummy_node(n, n.value.clone()))
        }
        PartUsageBodyElement::Connect(n) => {
            PartUsageBodyElement::Connect(dummy_node(n, n.value.clone()))
        }
        PartUsageBodyElement::FlowUsage(n) => {
            PartUsageBodyElement::FlowUsage(dummy_node(n, n.value.clone()))
        }
        PartUsageBodyElement::Perform(n) => {
            PartUsageBodyElement::Perform(dummy_node(n, normalize_perform(&n.value)))
        }
        PartUsageBodyElement::Allocate(n) => {
            PartUsageBodyElement::Allocate(dummy_node(n, n.value.clone()))
        }
        PartUsageBodyElement::Satisfy(n) => {
            PartUsageBodyElement::Satisfy(dummy_node(n, n.value.clone()))
        }
        PartUsageBodyElement::StateUsage(n) => {
            PartUsageBodyElement::StateUsage(dummy_node(n, n.value.clone()))
        }
        PartUsageBodyElement::MetadataAnnotation(n) => {
            PartUsageBodyElement::MetadataAnnotation(dummy_node(n, n.value.clone()))
        }
        PartUsageBodyElement::MetadataKeywordUsage(n) => {
            PartUsageBodyElement::MetadataKeywordUsage(dummy_node(n, n.value.clone()))
        }
        PartUsageBodyElement::VariantUsage(n) => {
            PartUsageBodyElement::VariantUsage(dummy_node(n, n.value.clone()))
        }
    };
    dummy_node(el, value)
}

fn normalize_port_usage(p: &PortUsage) -> PortUsage {
    PortUsage {
        name: p.name.clone(),
        type_name: p.type_name.clone(),
        multiplicity: p.multiplicity.clone(),
        subsets: p.subsets.clone(),
        redefines: p.redefines.clone(),
        references: p.references.clone(),
        crosses: p.crosses.clone(),
        body: normalize_port_body(&p.body),
        name_span: None,
        type_ref_span: None,
    }
}

fn normalize_port_body(b: &PortBody) -> PortBody {
    match b {
        PortBody::Semicolon => PortBody::Semicolon,
        PortBody::Brace { elements } => PortBody::Brace {
            elements: elements
                .iter()
                .map(normalize_port_body_element_node)
                .collect(),
        },
    }
}

fn normalize_port_body_element_node(el: &Node<PortBodyElement>) -> Node<PortBodyElement> {
    let value = match &el.value {
        PortBodyElement::Error(n) => PortBodyElement::Error(dummy_node(n, n.value.clone())),
        PortBodyElement::InOutDecl(n) => PortBodyElement::InOutDecl(dummy_node(n, n.value.clone())),
        PortBodyElement::PortUsage(n) => {
            PortBodyElement::PortUsage(dummy_node(n, normalize_port_usage(&n.value)))
        }
        PortBodyElement::Doc(n) => PortBodyElement::Doc(dummy_node(n, n.value.clone())),
        PortBodyElement::Other(text) => PortBodyElement::Other(text.clone()),
    };
    dummy_node(el, value)
}

fn normalize_port_def(p: &PortDef) -> PortDef {
    PortDef {
        identification: p.identification.clone(),
        specializes: p.specializes.clone(),
        specializes_span: None,
        body: normalize_port_def_body(&p.body),
    }
}

fn normalize_port_def_body(b: &PortDefBody) -> PortDefBody {
    match b {
        PortDefBody::Semicolon => PortDefBody::Semicolon,
        PortDefBody::Brace { elements } => PortDefBody::Brace {
            elements: elements
                .iter()
                .map(normalize_port_def_body_element_node)
                .collect(),
        },
    }
}

fn normalize_port_def_body_element_node(el: &Node<PortDefBodyElement>) -> Node<PortDefBodyElement> {
    let value = match &el.value {
        PortDefBodyElement::InOutDecl(n) => {
            PortDefBodyElement::InOutDecl(dummy_node(n, n.value.clone()))
        }
        PortDefBodyElement::Doc(n) => PortDefBodyElement::Doc(dummy_node(n, n.value.clone())),
        PortDefBodyElement::Error(n) => PortDefBodyElement::Error(dummy_node(n, n.value.clone())),
        PortDefBodyElement::AttributeDef(n) => {
            PortDefBodyElement::AttributeDef(dummy_node(n, normalize_attribute_def(&n.value)))
        }
        PortDefBodyElement::AttributeUsage(n) => {
            PortDefBodyElement::AttributeUsage(dummy_node(n, normalize_attribute_usage(&n.value)))
        }
        PortDefBodyElement::ItemUsage(n) => {
            PortDefBodyElement::ItemUsage(dummy_node(n, n.value.clone()))
        }
        PortDefBodyElement::PortUsage(n) => {
            PortDefBodyElement::PortUsage(dummy_node(n, normalize_port_usage(&n.value)))
        }
        PortDefBodyElement::Other(s) => PortDefBodyElement::Other(s.clone()),
    };
    dummy_node(el, value)
}

fn normalize_interface_def(i: &InterfaceDef) -> InterfaceDef {
    InterfaceDef {
        identification: i.identification.clone(),
        specializes: i.specializes.clone(),
        specializes_span: None,
        body: normalize_interface_def_body(&i.body),
    }
}

fn normalize_connection_def(c: &ConnectionDef) -> ConnectionDef {
    ConnectionDef {
        annotation: c.annotation.clone(),
        identification: c.identification.clone(),
        specializes: c.specializes.clone(),
        specializes_span: None,
        body: normalize_connection_def_body(&c.body),
    }
}

fn normalize_connection_def_body(b: &ConnectionDefBody) -> ConnectionDefBody {
    match b {
        ConnectionDefBody::Semicolon => ConnectionDefBody::Semicolon,
        ConnectionDefBody::Brace { elements } => ConnectionDefBody::Brace {
            elements: elements
                .iter()
                .map(normalize_connection_def_body_element_node)
                .collect(),
        },
    }
}

fn normalize_connection_def_body_element_node(
    el: &Node<ConnectionDefBodyElement>,
) -> Node<ConnectionDefBodyElement> {
    let value = match &el.value {
        ConnectionDefBodyElement::EndDecl(n) => {
            ConnectionDefBodyElement::EndDecl(dummy_node(n, normalize_end_decl(&n.value)))
        }
        ConnectionDefBodyElement::RefDecl(n) => {
            ConnectionDefBodyElement::RefDecl(dummy_node(n, normalize_ref_decl(&n.value)))
        }
        ConnectionDefBodyElement::ConnectStmt(n) => {
            ConnectionDefBodyElement::ConnectStmt(dummy_node(n, n.value.clone()))
        }
        ConnectionDefBodyElement::Doc(n) => {
            ConnectionDefBodyElement::Doc(dummy_node(n, n.value.clone()))
        }
        ConnectionDefBodyElement::Error(n) => {
            ConnectionDefBodyElement::Error(dummy_node(n, n.value.clone()))
        }
    };
    dummy_node(el, value)
}

fn normalize_metadata_def(m: &MetadataDef) -> MetadataDef {
    MetadataDef {
        is_abstract: m.is_abstract,
        identification: m.identification.clone(),
        specializes: m.specializes.clone(),
        specializes_span: None,
        body: m.body.clone(),
    }
}

fn normalize_enum_def(e: &EnumDef) -> EnumDef {
    EnumDef {
        identification: e.identification.clone(),
        specializes: e.specializes.clone(),
        specializes_span: None,
        body: e.body.clone(),
    }
}

fn normalize_occurrence_def(o: &OccurrenceDef) -> OccurrenceDef {
    OccurrenceDef {
        is_abstract: o.is_abstract,
        identification: o.identification.clone(),
        specializes: o.specializes.clone(),
        specializes_span: None,
        body: o.body.clone(),
    }
}

fn normalize_interface_def_body(b: &InterfaceDefBody) -> InterfaceDefBody {
    match b {
        InterfaceDefBody::Semicolon => InterfaceDefBody::Semicolon,
        InterfaceDefBody::Brace { elements } => InterfaceDefBody::Brace {
            elements: elements
                .iter()
                .map(normalize_interface_def_body_element_node)
                .collect(),
        },
    }
}

fn normalize_interface_def_body_element_node(
    el: &Node<InterfaceDefBodyElement>,
) -> Node<InterfaceDefBodyElement> {
    let value = match &el.value {
        InterfaceDefBodyElement::Doc(n) => {
            InterfaceDefBodyElement::Doc(dummy_node(n, n.value.clone()))
        }
        InterfaceDefBodyElement::EndDecl(n) => {
            InterfaceDefBodyElement::EndDecl(dummy_node(n, normalize_end_decl(&n.value)))
        }
        InterfaceDefBodyElement::RefDecl(n) => {
            InterfaceDefBodyElement::RefDecl(dummy_node(n, normalize_ref_decl(&n.value)))
        }
        InterfaceDefBodyElement::ConnectStmt(n) => {
            InterfaceDefBodyElement::ConnectStmt(dummy_node(n, n.value.clone()))
        }
    };
    dummy_node(el, value)
}

fn normalize_end_decl(e: &EndDecl) -> EndDecl {
    EndDecl {
        name: e.name.clone(),
        type_name: e.type_name.clone(),
        uses_derived_syntax: e.uses_derived_syntax,
        name_span: None,
        type_ref_span: None,
    }
}

fn normalize_ref_decl(r: &RefDecl) -> RefDecl {
    RefDecl {
        name: r.name.clone(),
        type_name: r.type_name.clone(),
        value: r.value.clone(),
        body: r.body.clone(),
        name_span: None,
        type_ref_span: None,
    }
}

fn normalize_action_def(a: &ActionDef) -> ActionDef {
    ActionDef {
        identification: a.identification.clone(),
        specializes: a.specializes.clone(),
        specializes_span: None,
        body: normalize_action_def_body(&a.body),
    }
}

fn normalize_action_def_body(b: &ActionDefBody) -> ActionDefBody {
    match b {
        ActionDefBody::Semicolon => ActionDefBody::Semicolon,
        ActionDefBody::Brace { elements } => ActionDefBody::Brace {
            elements: elements
                .iter()
                .map(normalize_action_def_body_element_node)
                .collect(),
        },
    }
}

fn normalize_action_def_body_element_node(
    el: &Node<ActionDefBodyElement>,
) -> Node<ActionDefBodyElement> {
    let value = match &el.value {
        ActionDefBodyElement::Error(n) => {
            ActionDefBodyElement::Error(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::InOutDecl(n) => {
            ActionDefBodyElement::InOutDecl(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::Doc(n) => ActionDefBodyElement::Doc(dummy_node(n, n.value.clone())),
        ActionDefBodyElement::Annotation(n) => {
            ActionDefBodyElement::Annotation(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::MetadataAnnotation(n) => {
            ActionDefBodyElement::MetadataAnnotation(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::MetadataKeywordUsage(n) => {
            ActionDefBodyElement::MetadataKeywordUsage(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::RefDecl(n) => {
            ActionDefBodyElement::RefDecl(dummy_node(n, normalize_ref_decl(&n.value)))
        }
        ActionDefBodyElement::Perform(n) => {
            ActionDefBodyElement::Perform(dummy_node(n, normalize_perform(&n.value)))
        }
        ActionDefBodyElement::Bind(n) => ActionDefBodyElement::Bind(dummy_node(n, n.value.clone())),
        ActionDefBodyElement::FlowUsage(n) => {
            ActionDefBodyElement::FlowUsage(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::FirstStmt(n) => {
            ActionDefBodyElement::FirstStmt(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::MergeStmt(n) => {
            ActionDefBodyElement::MergeStmt(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::DecisionStmt(n) => {
            ActionDefBodyElement::DecisionStmt(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::JoinStmt(n) => {
            ActionDefBodyElement::JoinStmt(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::ForkStmt(n) => {
            ActionDefBodyElement::ForkStmt(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::TerminateStmt(n) => {
            ActionDefBodyElement::TerminateStmt(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::WhileStmt(n) => {
            ActionDefBodyElement::WhileStmt(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::IfStmt(n) => {
            ActionDefBodyElement::IfStmt(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::StateUsage(n) => {
            ActionDefBodyElement::StateUsage(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::ActionUsage(n) => ActionDefBodyElement::ActionUsage(Box::new(
            dummy_node(n, normalize_action_usage(&n.value)),
        )),
        ActionDefBodyElement::Assign(n) => {
            ActionDefBodyElement::Assign(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::ForLoop(n) => {
            ActionDefBodyElement::ForLoop(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::ThenAction(n) => {
            ActionDefBodyElement::ThenAction(dummy_node(n, n.value.clone()))
        }
        ActionDefBodyElement::Decl(n) => ActionDefBodyElement::Decl(dummy_node(n, n.value.clone())),
    };
    dummy_node(el, value)
}

fn normalize_action_usage(a: &ActionUsage) -> ActionUsage {
    ActionUsage {
        name: a.name.clone(),
        type_name: a.type_name.clone(),
        accept: a.accept.clone(),
        send: a.send.clone(),
        body: normalize_action_usage_body(&a.body),
        name_span: None,
        type_ref_span: None,
    }
}

fn normalize_action_usage_body(b: &ActionUsageBody) -> ActionUsageBody {
    match b {
        ActionUsageBody::Semicolon => ActionUsageBody::Semicolon,
        ActionUsageBody::Brace { elements } => ActionUsageBody::Brace {
            elements: elements
                .iter()
                .map(normalize_action_usage_body_element_node)
                .collect(),
        },
    }
}

fn normalize_action_usage_body_element_node(
    el: &Node<ActionUsageBodyElement>,
) -> Node<ActionUsageBodyElement> {
    let value = match &el.value {
        ActionUsageBodyElement::Error(n) => {
            ActionUsageBodyElement::Error(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::Doc(n) => {
            ActionUsageBodyElement::Doc(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::Annotation(n) => {
            ActionUsageBodyElement::Annotation(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::MetadataAnnotation(n) => {
            ActionUsageBodyElement::MetadataAnnotation(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::MetadataKeywordUsage(n) => {
            ActionUsageBodyElement::MetadataKeywordUsage(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::InOutDecl(n) => {
            ActionUsageBodyElement::InOutDecl(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::RefDecl(n) => {
            ActionUsageBodyElement::RefDecl(dummy_node(n, normalize_ref_decl(&n.value)))
        }
        ActionUsageBodyElement::Bind(n) => {
            ActionUsageBodyElement::Bind(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::FlowUsage(n) => {
            ActionUsageBodyElement::FlowUsage(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::FirstStmt(n) => {
            ActionUsageBodyElement::FirstStmt(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::MergeStmt(n) => {
            ActionUsageBodyElement::MergeStmt(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::DecisionStmt(n) => {
            ActionUsageBodyElement::DecisionStmt(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::JoinStmt(n) => {
            ActionUsageBodyElement::JoinStmt(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::ForkStmt(n) => {
            ActionUsageBodyElement::ForkStmt(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::TerminateStmt(n) => {
            ActionUsageBodyElement::TerminateStmt(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::WhileStmt(n) => {
            ActionUsageBodyElement::WhileStmt(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::IfStmt(n) => {
            ActionUsageBodyElement::IfStmt(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::StateUsage(n) => {
            ActionUsageBodyElement::StateUsage(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::ActionUsage(n) => ActionUsageBodyElement::ActionUsage(Box::new(
            dummy_node(n, normalize_action_usage(&n.value)),
        )),
        ActionUsageBodyElement::Assign(n) => {
            ActionUsageBodyElement::Assign(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::ForLoop(n) => {
            ActionUsageBodyElement::ForLoop(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::ThenAction(n) => {
            ActionUsageBodyElement::ThenAction(dummy_node(n, n.value.clone()))
        }
        ActionUsageBodyElement::Decl(n) => {
            ActionUsageBodyElement::Decl(dummy_node(n, n.value.clone()))
        }
    };
    dummy_node(el, value)
}
