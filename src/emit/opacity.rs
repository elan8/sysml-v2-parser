//! Walk an AST and report opaque / recovery nodes that would fake-pass roundtrips.

use crate::ast::{
    ActionDefBody, ActionDefBodyElement, ActionUsageBody, ActionUsageBodyElement, AttributeBody,
    AttributeBodyElement, CalcDefBody, CalcDefBodyElement, ConnectionDefBody,
    ConnectionDefBodyElement, ConstraintDefBody, ConstraintDefBodyElement, DefinitionBody,
    DefinitionBodyElement, EndNestedUsage, FirstMergeBody, InterfaceDefBody,
    InterfaceDefBodyElement, InterfaceUsage, InterfaceUsageBodyElement, LibraryPackage,
    OccurrenceBodyElement, OccurrenceUsageBody, Package, PackageBody, PackageBodyElement,
    PartDefBody, PartDefBodyElement, PartUsageBody, PartUsageBodyElement, PerformBody,
    PerformBodyElement, PortBody, PortBodyElement, PortDefBody, PortDefBodyElement, RefBody,
    RelationshipBodyElement, RenderingDefBody, RenderingDefBodyElement, RenderingUsageBody,
    RenderingUsageBodyElement, RequirementDefBody, RequirementDefBodyElement, ReturnRefBody,
    ReturnRefBodyElement, RootElement, RootNamespace, StateDefBody, StateDefBodyElement,
    ThenTarget, UseCaseDefBody, UseCaseDefBodyElement, VariantTypedUsage, VariantUsage, ViewBody,
    ViewBodyElement, ViewDefBody, ViewDefBodyElement,
};

/// Kind of opaque or recovery content found in an AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpacityKind {
    ExtendedLibraryDecl,
    KermlSemanticDecl,
    KermlFeatureDecl,
    FeatureDecl,
    ClassifierDecl,
    RawRhsString,
    ParseError,
    UnsupportedGrammar,
}

/// One opaque hit with a path for diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpacityHit {
    pub path: String,
    pub kind: OpacityKind,
}

/// Aggregated opacity scan result.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OpacityReport {
    pub hits: Vec<OpacityHit>,
}

impl OpacityReport {
    pub fn is_clean(&self) -> bool {
        self.hits.is_empty()
    }
}

/// Scan `root` for opaque / recovery nodes.
pub fn opacity_report(root: &RootNamespace) -> OpacityReport {
    let mut report = OpacityReport::default();
    for (i, el) in root.elements.iter().enumerate() {
        let path = format!("root[{i}]");
        match &el.value {
            RootElement::Package(p) => walk_package(&mut report, &path, &p.value),
            RootElement::LibraryPackage(p) => walk_library_package(&mut report, &path, &p.value),
            RootElement::Namespace(n) => walk_package_body(&mut report, &path, &n.value.body),
            RootElement::Import(import) => walk_optional_relationship_body(
                &mut report,
                &path,
                import.value.body_elements.as_deref(),
            ),
            RootElement::Member(m) => walk_package_body_element(&mut report, &path, &m.value),
        }
    }
    report
}

fn hit(report: &mut OpacityReport, path: &str, kind: OpacityKind) {
    report.hits.push(OpacityHit {
        path: path.to_string(),
        kind,
    });
}

fn walk_package(report: &mut OpacityReport, path: &str, pkg: &Package) {
    walk_package_body(report, path, &pkg.body);
}

fn walk_library_package(report: &mut OpacityReport, path: &str, pkg: &LibraryPackage) {
    walk_package_body(report, path, &pkg.body);
}

fn walk_package_body(report: &mut OpacityReport, path: &str, body: &PackageBody) {
    let PackageBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        walk_package_body_element(report, &format!("{path}/body[{i}]"), &el.value);
    }
}

fn walk_package_body_element(report: &mut OpacityReport, path: &str, el: &PackageBodyElement) {
    match el {
        PackageBodyElement::Error(_) => hit(report, path, OpacityKind::ParseError),
        PackageBodyElement::Unsupported(_) => hit(report, path, OpacityKind::UnsupportedGrammar),
        PackageBodyElement::FeatureDecl(_) => hit(report, path, OpacityKind::FeatureDecl),
        PackageBodyElement::ClassifierDecl(_) => hit(report, path, OpacityKind::ClassifierDecl),
        PackageBodyElement::KermlSemanticDecl(_) => {
            hit(report, path, OpacityKind::KermlSemanticDecl)
        }
        PackageBodyElement::KermlFeatureDecl(_) => hit(report, path, OpacityKind::KermlFeatureDecl),
        PackageBodyElement::KermlClassifier(n) => walk_calc_def_body(report, path, &n.value.body),
        PackageBodyElement::KermlConnector(n) => walk_calc_def_body(report, path, &n.value.body),
        PackageBodyElement::KermlRelationship(_) => {}
        PackageBodyElement::KermlInvariant(n) => walk_calc_def_body(report, path, &n.value.body),
        PackageBodyElement::KermlFeatureMember(n) => {
            walk_calc_def_body(report, path, &n.value.body)
        }
        // Structurally recognized -- keyword, optional name, optional multiplicity, `;` -- not
        // an opaque/recovery node.
        PackageBodyElement::KermlBareDeclaration(_) => {}
        PackageBodyElement::ExtendedLibraryDecl(_) => {
            hit(report, path, OpacityKind::ExtendedLibraryDecl)
        }
        PackageBodyElement::Package(p) => walk_package(report, path, &p.value),
        PackageBodyElement::LibraryPackage(p) => walk_library_package(report, path, &p.value),
        PackageBodyElement::PartDef(p) => walk_part_def_body(report, path, &p.value.body),
        PackageBodyElement::PartUsage(p) => walk_part_usage_body(report, path, &p.value.body),
        PackageBodyElement::AttributeDef(a) => walk_attribute_body(report, path, &a.value.body),
        PackageBodyElement::AttributeUsage(a) => walk_attribute_body(report, path, &a.value.body),
        PackageBodyElement::PortDef(p) => walk_port_def_body(report, path, &p.value.body),
        PackageBodyElement::InterfaceDef(i) => walk_interface_def_body(report, path, &i.value.body),
        PackageBodyElement::ActionDef(a) => walk_action_def_body(report, path, &a.value.body),
        PackageBodyElement::ActionUsage(a) => {
            walk_optional_action_usage_body(report, path, &a.value.body)
        }
        PackageBodyElement::RequirementDef(r) => {
            walk_requirement_def_body(report, path, &r.value.body)
        }
        PackageBodyElement::RequirementUsage(r) => {
            walk_requirement_def_body(report, path, &r.value.body)
        }
        PackageBodyElement::StateDef(s) => walk_state_def_body(report, path, &s.value.body),
        PackageBodyElement::StateUsage(s) => walk_state_def_body(report, path, &s.value.body),
        PackageBodyElement::UseCaseDef(u) => walk_use_case_def_body(report, path, &u.value.body),
        PackageBodyElement::UseCaseUsage(u) => walk_use_case_def_body(report, path, &u.value.body),
        PackageBodyElement::CaseDef(c) => walk_use_case_def_body(report, path, &c.value.body),
        PackageBodyElement::CaseUsage(c) => walk_use_case_def_body(report, path, &c.value.body),
        PackageBodyElement::AnalysisCaseDef(c) => {
            walk_use_case_def_body(report, path, &c.value.body)
        }
        PackageBodyElement::AnalysisCaseUsage(c) => {
            walk_use_case_def_body(report, path, &c.value.body)
        }
        PackageBodyElement::VerificationCaseDef(c) => {
            walk_use_case_def_body(report, path, &c.value.body)
        }
        PackageBodyElement::VerificationCaseUsage(c) => {
            walk_use_case_def_body(report, path, &c.value.body)
        }
        PackageBodyElement::ViewDef(v) => walk_view_def_body(report, path, &v.value.body),
        PackageBodyElement::ViewUsage(v) => walk_view_body(report, path, &v.value.body),
        PackageBodyElement::ViewpointDef(v) => {
            walk_requirement_def_body(report, path, &v.value.body)
        }
        PackageBodyElement::ViewpointUsage(v) => {
            walk_requirement_def_body(report, path, &v.value.body)
        }
        PackageBodyElement::RenderingDef(r) => walk_rendering_def_body(report, path, &r.value.body),
        PackageBodyElement::RenderingUsage(r) => {
            walk_rendering_usage_body(report, path, &r.value.body)
        }
        PackageBodyElement::Connect(c) => walk_ref_body(report, path, &c.value.body),
        PackageBodyElement::ItemDef(i) => walk_attribute_body(report, path, &i.value.body),
        PackageBodyElement::ItemUsage(i) => walk_attribute_body(report, path, &i.value.body),
        PackageBodyElement::IndividualDef(i) => walk_attribute_body(report, path, &i.value.body),
        PackageBodyElement::OccurrenceDef(o) => walk_definition_body(report, path, &o.value.body),
        PackageBodyElement::OccurrenceUsage(o) => {
            walk_occurrence_usage_body(report, path, &o.value.body)
        }
        PackageBodyElement::AllocationDef(a) => walk_definition_body(report, path, &a.value.body),
        PackageBodyElement::AllocationUsage(a) => walk_definition_body(report, path, &a.value.body),
        PackageBodyElement::FlowDef(f) => walk_definition_body(report, path, &f.value.body),
        PackageBodyElement::FlowUsage(f) => walk_definition_body(report, path, &f.value.body),
        PackageBodyElement::ConnectionDef(c) => {
            walk_connection_def_body(report, path, &c.value.body)
        }
        PackageBodyElement::ConnectionUsage(c) => {
            walk_connection_def_body(report, path, &c.value.body)
        }
        PackageBodyElement::ConstraintDef(c) => {
            walk_constraint_def_body(report, path, &c.value.body)
        }
        PackageBodyElement::ConstraintUsage(c) => {
            walk_constraint_def_body(report, path, &c.value.body)
        }
        PackageBodyElement::CalcDef(c) => walk_calc_def_body(report, path, &c.value.body),
        PackageBodyElement::MetadataDef(m) => walk_attribute_body(report, path, &m.value.body),
        PackageBodyElement::MetadataUsage(m) => walk_attribute_body(report, path, &m.value.body),
        PackageBodyElement::ConcernUsage(c) => {
            walk_requirement_def_body(report, path, &c.value.body)
        }
        PackageBodyElement::InterfaceUsage(i) => walk_interface_usage(report, path, &i.value),
        PackageBodyElement::PortUsage(p) => walk_port_body(report, path, &p.value.body),
        PackageBodyElement::Ref(r) => walk_ref_body(report, path, &r.value.body),
        PackageBodyElement::EnumerationUsage(e) => walk_attribute_body(report, path, &e.value.body),
        PackageBodyElement::MetadataKeywordUsage(m) => {
            walk_optional_attribute_body(report, path, &m.value.body)
        }
        PackageBodyElement::Annotating(member) => walk_annotating_member(report, path, member),
        PackageBodyElement::AssertConstraint(a) => {
            walk_constraint_def_body(report, path, &a.value.body)
        }
        PackageBodyElement::PerformUsage(p) => walk_perform_body(report, path, &p.value.body),
        PackageBodyElement::BindingConnectorUsage(b) => walk_ref_body(report, path, &b.value.body),
        PackageBodyElement::ClassDef(c) => walk_attribute_body(report, path, &c.value.body),
        PackageBodyElement::Succession(first) => {
            walk_first_merge_body(report, path, &first.value.body)
        }
        PackageBodyElement::ExhibitState(exhibit) => {
            walk_state_def_body(report, path, &exhibit.value.body)
        }
        PackageBodyElement::IncludeUseCase(include) => {
            walk_use_case_def_body(report, path, &include.value.body)
        }
        PackageBodyElement::ExtendedDefinition(definition) => {
            walk_package_body(report, path, &definition.value.body)
        }
        PackageBodyElement::Satisfy(s) => walk_satisfy(report, path, &s.value),
        PackageBodyElement::Import(i) => {
            walk_optional_relationship_body(report, path, i.value.body_elements.as_deref())
        }
        PackageBodyElement::Dependency(d) => walk_relationship_body(report, path, &d.value.body),
        PackageBodyElement::AliasDef(a) => {
            if let crate::ast::AliasBody::Brace { elements, .. } = &a.value.body {
                walk_relationship_body_elements(report, path, elements);
            }
        }
        PackageBodyElement::Filter(_)
        | PackageBodyElement::Actor(_)
        | PackageBodyElement::EnumDef(_)
        | PackageBodyElement::DefaultReferenceUsage(_) => {}
    }
}

fn walk_interface_def_body(report: &mut OpacityReport, path: &str, body: &InterfaceDefBody) {
    let InterfaceDefBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, element) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &element.value {
            InterfaceDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            InterfaceDefBodyElement::AttributeDef(a) => {
                walk_attribute_body(report, &p, &a.value.body)
            }
            InterfaceDefBodyElement::AttributeUsage(a) => {
                walk_attribute_body(report, &p, &a.value.body)
            }
            InterfaceDefBodyElement::ItemDef(i) => walk_attribute_body(report, &p, &i.value.body),
            InterfaceDefBodyElement::ItemUsage(i) => walk_attribute_body(report, &p, &i.value.body),
            InterfaceDefBodyElement::PortDef(port) => {
                walk_port_def_body(report, &p, &port.value.body)
            }
            InterfaceDefBodyElement::PortUsage(port) => {
                walk_port_body(report, &p, &port.value.body)
            }
            InterfaceDefBodyElement::RefDecl(reference) => {
                walk_ref_body(report, &p, &reference.value.body)
            }
            InterfaceDefBodyElement::ConnectStmt(connect) => {
                walk_ref_body(report, &p, &connect.value.body)
            }
            InterfaceDefBodyElement::EndDecl(end) => walk_end_decl(report, &p, &end.value),
            InterfaceDefBodyElement::FlowUsage(flow) => {
                walk_definition_body(report, &p, &flow.value.body)
            }
            InterfaceDefBodyElement::Annotating(member) => {
                walk_annotating_member(report, &p, member)
            }
            InterfaceDefBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
        }
    }
}

fn walk_constraint_def_body(report: &mut OpacityReport, path: &str, body: &ConstraintDefBody) {
    let ConstraintDefBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, element) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &element.value {
            ConstraintDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            ConstraintDefBodyElement::Constraint(c) => {
                walk_constraint_def_body(report, &p, &c.value.body)
            }
            ConstraintDefBodyElement::AttributeUsage(a) => {
                walk_attribute_body(report, &p, &a.value.body)
            }
            // A keyword-less feature declaration owns only an optional binding list, which the
            // opacity report has nothing to walk into.
            ConstraintDefBodyElement::FeatureDecl(_) => {}
            ConstraintDefBodyElement::Annotating(member) => {
                walk_annotating_member(report, &p, member)
            }
            ConstraintDefBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
            ConstraintDefBodyElement::InOutDecl(n) => walk_in_out_decl(report, &p, &n.value),
            ConstraintDefBodyElement::RequireConstraint(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            ConstraintDefBodyElement::PartUsage(pu) => {
                walk_part_usage_body(report, &p, &pu.value.body)
            }
            ConstraintDefBodyElement::Expression(_) => {}
        }
    }
}

fn walk_calc_def_body(report: &mut OpacityReport, path: &str, body: &CalcDefBody) {
    let CalcDefBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, element) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &element.value {
            CalcDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            CalcDefBodyElement::CalcUsage(c) => walk_calc_def_body(report, &p, &c.value.body),
            CalcDefBodyElement::CalcDef(c) => walk_calc_def_body(report, &p, &c.value.body),
            CalcDefBodyElement::PartUsage(pu) => walk_part_usage_body(report, &p, &pu.value.body),
            CalcDefBodyElement::Annotating(member) => walk_annotating_member(report, &p, member),
            CalcDefBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
            CalcDefBodyElement::ActionMember(n) => {
                walk_action_def_body_elements(report, &p, std::slice::from_ref(n))
            }
            CalcDefBodyElement::InOutDecl(n) => walk_in_out_decl(report, &p, &n.value),
            CalcDefBodyElement::TypedParameter(n) => walk_calc_def_body(report, &p, &n.value.body),
            CalcDefBodyElement::KermlFeature(n) => walk_calc_def_body(report, &p, &n.value.body),
            CalcDefBodyElement::Invariant(n) => walk_calc_def_body(report, &p, &n.value.body),
            CalcDefBodyElement::Connector(n) => walk_calc_def_body(report, &p, &n.value.body),
            CalcDefBodyElement::AssertConstraint(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            CalcDefBodyElement::KermlClassifier(n) => walk_calc_def_body(report, &p, &n.value.body),
            CalcDefBodyElement::EndMember(n) => {
                walk_calc_def_body(report, &p, &n.value.feature.value.body)
            }
            CalcDefBodyElement::AttributeUsage(n) => walk_attribute_body(report, &p, &n.value.body),
            CalcDefBodyElement::Binding(_)
            | CalcDefBodyElement::Succession(_)
            | CalcDefBodyElement::Import(_)
            | CalcDefBodyElement::DefaultReferenceUsage(_) => {}
            CalcDefBodyElement::ReturnDecl(n) => walk_calc_def_body(report, &p, &n.value.body),
            CalcDefBodyElement::Expression(_) => {}
        }
    }
}

fn walk_rendering_def_body(report: &mut OpacityReport, path: &str, body: &RenderingDefBody) {
    let RenderingDefBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, element) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &element.value {
            RenderingDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            RenderingDefBodyElement::Unsupported(_) => {
                hit(report, &p, OpacityKind::UnsupportedGrammar)
            }
            RenderingDefBodyElement::ViewRendering(rendering) => {
                walk_rendering_usage_body(report, &p, &rendering.value.body)
            }
            RenderingDefBodyElement::RefDecl(n) => walk_ref_body(report, &p, &n.value.body),
            RenderingDefBodyElement::Annotating(member) => {
                walk_annotating_member(report, &p, member)
            }
            RenderingDefBodyElement::Filter(_) => {}
        }
    }
}

fn walk_connection_def_body(report: &mut OpacityReport, path: &str, body: &ConnectionDefBody) {
    let ConnectionDefBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, element) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &element.value {
            ConnectionDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            ConnectionDefBodyElement::AttributeDef(a) => {
                walk_attribute_body(report, &p, &a.value.body)
            }
            ConnectionDefBodyElement::AttributeUsage(a) => {
                walk_attribute_body(report, &p, &a.value.body)
            }
            ConnectionDefBodyElement::ItemDef(i) => walk_attribute_body(report, &p, &i.value.body),
            ConnectionDefBodyElement::ItemUsage(i) => {
                walk_attribute_body(report, &p, &i.value.body)
            }
            ConnectionDefBodyElement::OccurrenceUsage(o) => {
                walk_occurrence_usage_body(report, &p, &o.value.body)
            }
            ConnectionDefBodyElement::PartUsage(pu) => {
                walk_part_usage_body(report, &p, &pu.value.body)
            }
            ConnectionDefBodyElement::PortDef(port) => {
                walk_port_def_body(report, &p, &port.value.body)
            }
            ConnectionDefBodyElement::PortUsage(port) => {
                walk_port_body(report, &p, &port.value.body)
            }
            ConnectionDefBodyElement::RefDecl(reference) => {
                walk_ref_body(report, &p, &reference.value.body)
            }
            ConnectionDefBodyElement::ConnectStmt(connect) => {
                walk_ref_body(report, &p, &connect.value.body)
            }
            ConnectionDefBodyElement::EndDecl(end) => walk_end_decl(report, &p, &end.value),
            ConnectionDefBodyElement::AssertConstraint(assertion) => {
                walk_constraint_def_body(report, &p, &assertion.value.body)
            }
            ConnectionDefBodyElement::SuccessionUsage(succession) => {
                walk_ref_body(report, &p, &succession.value.body)
            }
            ConnectionDefBodyElement::Annotating(member) => {
                walk_annotating_member(report, &p, member)
            }
            ConnectionDefBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
        }
    }
}

fn walk_part_def_body(report: &mut OpacityReport, path: &str, body: &PartDefBody) {
    let PartDefBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            PartDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            PartDefBodyElement::KermlClassifier(n) => walk_calc_def_body(report, &p, &n.value.body),
            PartDefBodyElement::UnsupportedMember(_) => {
                hit(report, &p, OpacityKind::UnsupportedGrammar)
            }
            PartDefBodyElement::Connect(c) => walk_ref_body(report, &p, &c.value.body),
            PartDefBodyElement::PartDef(n) => walk_part_def_body(report, &p, &n.value.body),
            PartDefBodyElement::PartUsage(n) => walk_part_usage_body(report, &p, &n.value.body),
            PartDefBodyElement::AttributeDef(n) => walk_attribute_body(report, &p, &n.value.body),
            PartDefBodyElement::AttributeUsage(n) => walk_attribute_body(report, &p, &n.value.body),
            PartDefBodyElement::ActionDef(n) => walk_action_def_body(report, &p, &n.value.body),
            PartDefBodyElement::ActionUsage(n) => {
                walk_optional_action_usage_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::Annotating(member) => walk_annotating_member(report, &p, member),
            PartDefBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::Dependency(n) => walk_relationship_body(report, &p, &n.value.body),
            PartDefBodyElement::RequirementUsage(n) => {
                walk_requirement_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::ItemDef(n) => walk_attribute_body(report, &p, &n.value.body),
            PartDefBodyElement::ItemUsage(n) => walk_attribute_body(report, &p, &n.value.body),
            PartDefBodyElement::Ref(n) => walk_ref_body(report, &p, &n.value.body),
            PartDefBodyElement::PortDef(n) => walk_port_def_body(report, &p, &n.value.body),
            PartDefBodyElement::PortUsage(n) => walk_port_body(report, &p, &n.value.body),
            PartDefBodyElement::OccurrenceUsage(n) => {
                walk_occurrence_usage_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::InterfaceDef(n) => {
                walk_interface_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::InterfaceUsage(n) => walk_interface_usage(report, &p, &n.value),
            PartDefBodyElement::FlowUsage(n) => walk_definition_body(report, &p, &n.value.body),
            PartDefBodyElement::Connection(n) => {
                walk_connection_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::Perform(n) => walk_perform_body(report, &p, &n.value.body),
            PartDefBodyElement::Allocate(n) => walk_ref_body(report, &p, &n.value.body),
            PartDefBodyElement::ExhibitState(n) => walk_state_def_body(report, &p, &n.value.body),
            PartDefBodyElement::CalcUsage(n) => walk_calc_def_body(report, &p, &n.value.body),
            PartDefBodyElement::ConstraintDef(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::ConstraintUsage(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::StateUsage(n) => walk_state_def_body(report, &p, &n.value.body),
            PartDefBodyElement::EnumerationUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::AssertConstraint(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::Satisfy(n) => walk_satisfy(report, &p, &n.value),
            PartDefBodyElement::VariantUsage(n) => walk_variant_usage(report, &p, &n.value),
            PartDefBodyElement::StateDef(n) => walk_state_def_body(report, &p, &n.value.body),
            PartDefBodyElement::MetadataDef(n) => walk_attribute_body(report, &p, &n.value.body),
            PartDefBodyElement::MetadataUsage(n) => walk_attribute_body(report, &p, &n.value.body),
            PartDefBodyElement::FlowDef(n) => walk_definition_body(report, &p, &n.value.body),
            PartDefBodyElement::RequirementDef(n) => {
                walk_requirement_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::OccurrenceDef(n) => walk_definition_body(report, &p, &n.value.body),
            PartDefBodyElement::ConnectionDef(n) => {
                walk_connection_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::CalcDef(n) => walk_calc_def_body(report, &p, &n.value.body),
            PartDefBodyElement::AllocationDef(n) => walk_definition_body(report, &p, &n.value.body),
            PartDefBodyElement::AllocationUsage(n) => {
                walk_definition_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::ViewDef(n) => walk_view_def_body(report, &p, &n.value.body),
            PartDefBodyElement::ViewUsage(n) => walk_view_body(report, &p, &n.value.body),
            PartDefBodyElement::ViewpointDef(n) => {
                walk_requirement_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::ViewpointUsage(n) => {
                walk_requirement_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::RenderingDef(n) => {
                walk_rendering_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::RenderingUsage(n) => {
                walk_rendering_usage_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::CaseDef(n) => walk_use_case_def_body(report, &p, &n.value.body),
            PartDefBodyElement::CaseUsage(n) => walk_use_case_def_body(report, &p, &n.value.body),
            PartDefBodyElement::UseCaseDef(n) => walk_use_case_def_body(report, &p, &n.value.body),
            PartDefBodyElement::UseCaseUsage(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::AnalysisCaseDef(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::AnalysisCaseUsage(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::VerificationCaseDef(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::VerificationCaseUsage(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            PartDefBodyElement::Bind(n) => walk_bind(report, &p, &n.value),
            PartDefBodyElement::Import(import) => {
                walk_optional_relationship_body(report, &p, import.value.body_elements.as_deref())
            }
            PartDefBodyElement::AliasDef(alias) => {
                if let crate::ast::AliasBody::Brace { elements, .. } = &alias.value.body {
                    walk_relationship_body_elements(report, &p, elements);
                }
            }
            PartDefBodyElement::FirstStmt(first) => {
                walk_first_merge_body(report, &p, &first.value.body)
            }
            PartDefBodyElement::DefaultReferenceUsage(_) | PartDefBodyElement::EnumDef(_) => {}
        }
    }
}

fn walk_part_usage_body(report: &mut OpacityReport, path: &str, body: &PartUsageBody) {
    let PartUsageBody::Brace { elements, .. } = body else {
        return;
    };
    walk_part_usage_body_elements(report, path, elements);
}

fn walk_part_usage_body_elements(
    report: &mut OpacityReport,
    path: &str,
    elements: &[crate::ast::Node<PartUsageBodyElement>],
) {
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            PartUsageBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            PartUsageBodyElement::KermlClassifier(n) => {
                walk_calc_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::Connect(c) => walk_ref_body(report, &p, &c.value.body),
            PartUsageBodyElement::PartUsage(n) => walk_part_usage_body(report, &p, &n.value.body),
            PartUsageBodyElement::InOutDecl(n) => walk_in_out_decl(report, &p, &n.value),
            PartUsageBodyElement::EndDecl(_) => {}
            PartUsageBodyElement::AttributeUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::ActionUsage(n) => {
                walk_optional_action_usage_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::EnumerationUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::OccurrenceUsage(n) => {
                walk_occurrence_usage_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::PortDef(n) => walk_port_def_body(report, &p, &n.value.body),
            PartUsageBodyElement::PortUsage(n) => walk_port_body(report, &p, &n.value.body),
            PartUsageBodyElement::Bind(n) => walk_bind(report, &p, &n.value),
            PartUsageBodyElement::Ref(n) => walk_ref_body(report, &p, &n.value.body),
            PartUsageBodyElement::InterfaceUsage(n) => walk_interface_usage(report, &p, &n.value),
            PartUsageBodyElement::FlowUsage(n) => walk_definition_body(report, &p, &n.value.body),
            PartUsageBodyElement::Perform(n) => walk_perform_body(report, &p, &n.value.body),
            PartUsageBodyElement::SuccessionUsage(n) => walk_ref_body(report, &p, &n.value.body),
            PartUsageBodyElement::Allocate(n) => walk_ref_body(report, &p, &n.value.body),
            PartUsageBodyElement::Satisfy(n) => walk_satisfy(report, &p, &n.value),
            PartUsageBodyElement::StateUsage(n) => walk_state_def_body(report, &p, &n.value.body),
            PartUsageBodyElement::Annotating(member) => walk_annotating_member(report, &p, member),
            PartUsageBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::VariantUsage(n) => walk_variant_usage(report, &p, &n.value),
            PartUsageBodyElement::StateDef(n) => walk_state_def_body(report, &p, &n.value.body),
            PartUsageBodyElement::MetadataDef(n) => walk_attribute_body(report, &p, &n.value.body),
            PartUsageBodyElement::FlowDef(n) => walk_definition_body(report, &p, &n.value.body),
            PartUsageBodyElement::ViewDef(n) => walk_view_def_body(report, &p, &n.value.body),
            PartUsageBodyElement::ViewUsage(n) => walk_view_body(report, &p, &n.value.body),
            PartUsageBodyElement::ViewpointDef(n) => {
                walk_requirement_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::ViewpointUsage(n) => {
                walk_requirement_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::RenderingDef(n) => {
                walk_rendering_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::RenderingUsage(n) => {
                walk_rendering_usage_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::RequirementDef(n) => {
                walk_requirement_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::OccurrenceDef(n) => {
                walk_definition_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::CalcDef(n) => walk_calc_def_body(report, &p, &n.value.body),
            PartUsageBodyElement::ConnectionDef(n) => {
                walk_connection_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::Connection(n) => {
                walk_connection_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::AssertConstraint(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::ConstraintDef(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::ConstraintUsage(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::CalcUsage(n) => walk_calc_def_body(report, &p, &n.value.body),
            PartUsageBodyElement::RequirementUsage(n) => {
                walk_requirement_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::ItemDef(n) => walk_attribute_body(report, &p, &n.value.body),
            PartUsageBodyElement::ItemUsage(n) => walk_attribute_body(report, &p, &n.value.body),
            PartUsageBodyElement::MetadataUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::AnalysisCaseDef(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::AnalysisCaseUsage(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::IncludeUseCase(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::UseCaseUsage(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::VerificationCaseUsage(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::Import(import) => {
                walk_optional_relationship_body(report, &p, import.value.body_elements.as_deref())
            }
            PartUsageBodyElement::AliasDef(alias) => {
                if let crate::ast::AliasBody::Brace { elements, .. } = &alias.value.body {
                    walk_relationship_body_elements(report, &p, elements);
                }
            }
            PartUsageBodyElement::DefaultReferenceUsage(_) | PartUsageBodyElement::EnumDef(_) => {}
        }
    }
}

fn walk_attribute_body(report: &mut OpacityReport, path: &str, body: &AttributeBody) {
    let AttributeBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            AttributeBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            AttributeBodyElement::Unsupported(_) => {
                hit(report, &p, OpacityKind::UnsupportedGrammar)
            }
            AttributeBodyElement::KermlFeature(n) => walk_calc_def_body(report, &p, &n.value.body),
            AttributeBodyElement::Invariant(n) => walk_calc_def_body(report, &p, &n.value.body),
            AttributeBodyElement::KermlConnector(n) => {
                walk_calc_def_body(report, &p, &n.value.body)
            }
            AttributeBodyElement::ClassDef(n) => walk_attribute_body(report, &p, &n.value.body),
            AttributeBodyElement::Bind(n) => walk_bind(report, &p, &n.value),
            AttributeBodyElement::Connection(n) => {
                walk_connection_def_body(report, &p, &n.value.body)
            }
            AttributeBodyElement::CalcDef(n) => walk_calc_def_body(report, &p, &n.value.body),
            AttributeBodyElement::CalcUsage(n) => walk_calc_def_body(report, &p, &n.value.body),
            AttributeBodyElement::ConstraintUsage(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            AttributeBodyElement::KermlClassifier(n) => {
                walk_calc_def_body(report, &p, &n.value.body)
            }
            AttributeBodyElement::Connect(c) => walk_ref_body(report, &p, &c.value.body),
            AttributeBodyElement::AttributeDef(n) => walk_attribute_body(report, &p, &n.value.body),
            AttributeBodyElement::AttributeUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            AttributeBodyElement::OccurrenceUsage(n) => {
                walk_occurrence_usage_body(report, &p, &n.value.body)
            }
            AttributeBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
            AttributeBodyElement::AssertConstraint(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            AttributeBodyElement::RefDecl(n) => walk_ref_body(report, &p, &n.value.body),
            AttributeBodyElement::PartUsage(n) => walk_part_usage_body(report, &p, &n.value.body),
            AttributeBodyElement::ItemUsage(n) => walk_attribute_body(report, &p, &n.value.body),
            AttributeBodyElement::Annotating(member) => walk_annotating_member(report, &p, member),
        }
    }
}

fn walk_port_def_body(report: &mut OpacityReport, path: &str, body: &PortDefBody) {
    let PortDefBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            PortDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            PortDefBodyElement::Unsupported(_) => hit(report, &p, OpacityKind::UnsupportedGrammar),
            PortDefBodyElement::RefDecl(n) => walk_ref_body(report, &p, &n.value.body),
            PortDefBodyElement::AttributeDef(n) => walk_attribute_body(report, &p, &n.value.body),
            PortDefBodyElement::AttributeUsage(n) => walk_attribute_body(report, &p, &n.value.body),
            PortDefBodyElement::ItemDef(n) => walk_attribute_body(report, &p, &n.value.body),
            PortDefBodyElement::ItemUsage(n) => walk_attribute_body(report, &p, &n.value.body),
            PortDefBodyElement::EnumerationUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            PortDefBodyElement::PortUsage(n) => walk_port_body(report, &p, &n.value.body),
            PortDefBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
            PortDefBodyElement::InOutDecl(n) => walk_in_out_decl(report, &p, &n.value),
            PortDefBodyElement::Annotating(member) => walk_annotating_member(report, &p, member),
        }
    }
}

fn walk_port_body(report: &mut OpacityReport, path: &str, body: &PortBody) {
    let PortBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, element) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &element.value {
            PortBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            PortBodyElement::PortUsage(n) => walk_port_body(report, &p, &n.value.body),
            PortBodyElement::RefDecl(n) => walk_ref_body(report, &p, &n.value.body),
            PortBodyElement::AttributeUsage(n) => walk_attribute_body(report, &p, &n.value.body),
            PortBodyElement::ItemUsage(n) => walk_attribute_body(report, &p, &n.value.body),
            PortBodyElement::InOutDecl(n) => walk_in_out_decl(report, &p, &n.value),
            PortBodyElement::Annotating(member) => walk_annotating_member(report, &p, member),
        }
    }
}

fn walk_action_branch_body(
    report: &mut OpacityReport,
    path: &str,
    branch: &crate::ast::ActionBranchBody,
) {
    match branch {
        crate::ast::ActionBranchBody::Braced(body) => walk_action_def_body(report, path, body),
        crate::ast::ActionBranchBody::Shorthand(member) => {
            walk_action_def_body_elements(report, path, std::slice::from_ref(member))
        }
    }
}

fn walk_action_def_body(report: &mut OpacityReport, path: &str, body: &ActionDefBody) {
    let ActionDefBody::Brace { elements, .. } = body else {
        return;
    };
    walk_action_def_body_elements(report, path, elements);
}

/// Walk a direction-prefixed parameter declaration's retained `{ ... }` terminator body, which
/// shares the action-body member grammar.
fn walk_in_out_decl(report: &mut OpacityReport, path: &str, decl: &crate::ast::InOutDecl) {
    if let Some(elements) = &decl.body {
        walk_action_def_body_elements(report, path, elements);
    }
}

fn walk_action_def_body_elements(
    report: &mut OpacityReport,
    path: &str,
    elements: &[crate::ast::Node<ActionDefBodyElement>],
) {
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            ActionDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            ActionDefBodyElement::AttributeUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            ActionDefBodyElement::CalcUsage(n) => walk_calc_def_body(report, &p, &n.value.body),
            ActionDefBodyElement::ActionDef(n) => walk_action_def_body(report, &p, &n.value.body),
            ActionDefBodyElement::Annotating(member) => walk_annotating_member(report, &p, member),
            ActionDefBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
            ActionDefBodyElement::Dependency(n) => {
                walk_relationship_body(report, &p, &n.value.body)
            }
            ActionDefBodyElement::MetadataUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            ActionDefBodyElement::RefDecl(n) => walk_ref_body(report, &p, &n.value.body),
            ActionDefBodyElement::Perform(n) => walk_perform_body(report, &p, &n.value.body),
            ActionDefBodyElement::Bind(n) => walk_bind(report, &p, &n.value),
            ActionDefBodyElement::FlowUsage(n) => walk_definition_body(report, &p, &n.value.body),
            ActionDefBodyElement::WhileStmt(n) => walk_action_def_body(report, &p, &n.value.body),
            ActionDefBodyElement::LoopStmt(n) => walk_action_def_body(report, &p, &n.value.body),
            ActionDefBodyElement::IfStmt(n) => {
                walk_action_branch_body(report, &format!("{p}/then"), &n.value.then_body);
                if let Some(body) = &n.value.else_body {
                    walk_action_branch_body(report, &format!("{p}/else"), body);
                }
            }
            ActionDefBodyElement::StateUsage(n) => walk_state_def_body(report, &p, &n.value.body),
            ActionDefBodyElement::ActionUsage(n) => {
                walk_optional_action_usage_body(report, &p, &n.value.body)
            }
            ActionDefBodyElement::PartUsage(n) => walk_part_usage_body(report, &p, &n.value.body),
            ActionDefBodyElement::ItemUsage(n) => walk_attribute_body(report, &p, &n.value.body),
            ActionDefBodyElement::AssertConstraint(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            ActionDefBodyElement::OccurrenceUsage(n) => {
                walk_occurrence_usage_body(report, &p, &n.value.body)
            }
            ActionDefBodyElement::ForLoop(n) => walk_action_def_body(report, &p, &n.value.body),
            ActionDefBodyElement::ThenAction(n) => walk_then_target(report, &p, &n.value.target),
            ActionDefBodyElement::FirstStmt(first) => {
                walk_first_merge_body(report, &p, &first.value.body)
            }
            ActionDefBodyElement::MergeStmt(merge) => {
                walk_first_merge_body(report, &p, &merge.value.body)
            }
            ActionDefBodyElement::DecisionStmt(decision) => {
                walk_first_merge_body(report, &p, &decision.value.body)
            }
            ActionDefBodyElement::JoinStmt(join) => {
                walk_first_merge_body(report, &p, &join.value.body)
            }
            ActionDefBodyElement::ForkStmt(fork) => {
                walk_first_merge_body(report, &p, &fork.value.body)
            }
            ActionDefBodyElement::InOutDecl(n) => walk_in_out_decl(report, &p, &n.value),
            ActionDefBodyElement::TerminateStmt(_)
            | ActionDefBodyElement::Assign(_)
            | ActionDefBodyElement::DefaultReferenceUsage(_) => {}
        }
    }
}

/// The absent body of an inferred-terminator action usage or a `#Name` prefix has nothing to walk.
fn walk_optional_action_usage_body(
    report: &mut OpacityReport,
    path: &str,
    body: &Option<crate::ast::ActionUsageBody>,
) {
    if let Some(body) = body {
        walk_action_usage_body(report, path, body);
    }
}

fn walk_optional_attribute_body(
    report: &mut OpacityReport,
    path: &str,
    body: &Option<crate::ast::AttributeBody>,
) {
    if let Some(body) = body {
        walk_attribute_body(report, path, body);
    }
}

fn walk_action_usage_body(report: &mut OpacityReport, path: &str, body: &ActionUsageBody) {
    let ActionUsageBody::Brace { elements, .. } = body else {
        return;
    };
    walk_action_usage_body_elements(report, path, elements);
}

fn walk_action_usage_body_elements(
    report: &mut OpacityReport,
    path: &str,
    elements: &[crate::ast::Node<ActionUsageBodyElement>],
) {
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            ActionUsageBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            ActionUsageBodyElement::AttributeUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            ActionUsageBodyElement::CalcUsage(n) => walk_calc_def_body(report, &p, &n.value.body),
            ActionUsageBodyElement::ActionDef(n) => walk_action_def_body(report, &p, &n.value.body),
            ActionUsageBodyElement::Annotating(member) => {
                walk_annotating_member(report, &p, member)
            }
            ActionUsageBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
            ActionUsageBodyElement::Dependency(n) => {
                walk_relationship_body(report, &p, &n.value.body)
            }
            ActionUsageBodyElement::MetadataUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            ActionUsageBodyElement::RefDecl(n) => walk_ref_body(report, &p, &n.value.body),
            ActionUsageBodyElement::Bind(n) => walk_bind(report, &p, &n.value),
            ActionUsageBodyElement::FlowUsage(n) => walk_definition_body(report, &p, &n.value.body),
            ActionUsageBodyElement::WhileStmt(n) => walk_action_def_body(report, &p, &n.value.body),
            ActionUsageBodyElement::LoopStmt(n) => walk_action_def_body(report, &p, &n.value.body),
            ActionUsageBodyElement::IfStmt(n) => {
                walk_action_branch_body(report, &format!("{p}/then"), &n.value.then_body);
                if let Some(body) = &n.value.else_body {
                    walk_action_branch_body(report, &format!("{p}/else"), body);
                }
            }
            ActionUsageBodyElement::StateUsage(n) => walk_state_def_body(report, &p, &n.value.body),
            ActionUsageBodyElement::ActionUsage(n) => {
                walk_optional_action_usage_body(report, &p, &n.value.body)
            }
            ActionUsageBodyElement::PartUsage(n) => walk_part_usage_body(report, &p, &n.value.body),
            ActionUsageBodyElement::ItemUsage(n) => walk_attribute_body(report, &p, &n.value.body),
            ActionUsageBodyElement::AssertConstraint(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            ActionUsageBodyElement::OccurrenceUsage(n) => {
                walk_occurrence_usage_body(report, &p, &n.value.body)
            }
            ActionUsageBodyElement::ForLoop(n) => walk_action_def_body(report, &p, &n.value.body),
            ActionUsageBodyElement::ThenAction(n) => walk_then_target(report, &p, &n.value.target),
            ActionUsageBodyElement::VariantUsage(n) => walk_variant_usage(report, &p, &n.value),
            ActionUsageBodyElement::FirstStmt(first) => {
                walk_first_merge_body(report, &p, &first.value.body)
            }
            ActionUsageBodyElement::MergeStmt(merge) => {
                walk_first_merge_body(report, &p, &merge.value.body)
            }
            ActionUsageBodyElement::DecisionStmt(decision) => {
                walk_first_merge_body(report, &p, &decision.value.body)
            }
            ActionUsageBodyElement::JoinStmt(join) => {
                walk_first_merge_body(report, &p, &join.value.body)
            }
            ActionUsageBodyElement::ForkStmt(fork) => {
                walk_first_merge_body(report, &p, &fork.value.body)
            }
            ActionUsageBodyElement::InOutDecl(n) => walk_in_out_decl(report, &p, &n.value),
            ActionUsageBodyElement::TerminateStmt(_)
            | ActionUsageBodyElement::Assign(_)
            | ActionUsageBodyElement::DefaultReferenceUsage(_) => {}
        }
    }
}

fn walk_requirement_def_body(report: &mut OpacityReport, path: &str, body: &RequirementDefBody) {
    let RequirementDefBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            RequirementDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            RequirementDefBodyElement::Satisfy(n) => walk_satisfy(report, &p, &n.value),
            RequirementDefBodyElement::Dependency(n) => {
                walk_relationship_body(report, &p, &n.value.body)
            }
            RequirementDefBodyElement::Annotating(member) => {
                walk_annotating_member(report, &p, member)
            }
            RequirementDefBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
            RequirementDefBodyElement::Import(n) => {
                walk_optional_relationship_body(report, &p, n.value.body_elements.as_deref())
            }
            RequirementDefBodyElement::RequirementUsage(n) => {
                walk_requirement_def_body(report, &p, &n.value.body)
            }
            RequirementDefBodyElement::AttributeDef(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            RequirementDefBodyElement::AttributeUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            RequirementDefBodyElement::VariantUsage(n) => walk_variant_usage(report, &p, &n.value),
            RequirementDefBodyElement::VerifyRequirement(n) => {
                if let Some(requirement) = &n.value.requirement {
                    walk_requirement_def_body(report, &p, &requirement.value.body);
                }
            }
            RequirementDefBodyElement::RequireConstraint(n) => {
                if let crate::ast::ConstraintDefBody::Brace { elements, .. } = &n.value.body {
                    walk_constraint_body_elements(report, &p, elements);
                }
            }
            RequirementDefBodyElement::Constraint(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            RequirementDefBodyElement::Frame(n) => {
                walk_requirement_def_body(report, &p, &n.value.body)
            }
            RequirementDefBodyElement::RequirementDef(n) => {
                walk_requirement_def_body(report, &p, &n.value.body)
            }
            RequirementDefBodyElement::RefDecl(n) => walk_ref_body(report, &p, &n.value.body),
            RequirementDefBodyElement::ConcernUsage(n) => {
                walk_requirement_def_body(report, &p, &n.value.body)
            }
            RequirementDefBodyElement::CalcUsage(n) => {
                walk_calc_def_body(report, &p, &n.value.body)
            }
            RequirementDefBodyElement::PortUsage(_n) => {}
            RequirementDefBodyElement::AllocationUsage(n) => {
                walk_definition_body(report, &p, &n.value.body)
            }
            RequirementDefBodyElement::SubjectDecl(_)
            | RequirementDefBodyElement::SubjectRef(_)
            | RequirementDefBodyElement::RequirementActorDecl(_)
            | RequirementDefBodyElement::Stakeholder(_)
            | RequirementDefBodyElement::Purpose(_) => {}
        }
    }
}

fn walk_use_case_def_body(report: &mut OpacityReport, path: &str, body: &UseCaseDefBody) {
    let UseCaseDefBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            UseCaseDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            UseCaseDefBodyElement::Annotating(member) => walk_annotating_member(report, &p, member),
            UseCaseDefBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
            UseCaseDefBodyElement::AttributeDef(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            UseCaseDefBodyElement::Objective(n) => {
                walk_requirement_def_body(report, &p, &n.value.requirement.value.body)
            }
            UseCaseDefBodyElement::ThenIncludeUseCase(n) => {
                walk_use_case_def_body(report, &p, &n.value.include.value.body)
            }
            UseCaseDefBodyElement::ThenUseCaseUsage(n) => {
                walk_use_case_def_body(report, &p, &n.value.use_case.value.body)
            }
            UseCaseDefBodyElement::UseCaseUsage(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            UseCaseDefBodyElement::CaseUsage(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            UseCaseDefBodyElement::VerificationCaseUsage(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            UseCaseDefBodyElement::IncludeUseCase(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            UseCaseDefBodyElement::RefRedefinition(n) => {
                walk_use_case_def_body(report, &p, &n.value.body.value)
            }
            UseCaseDefBodyElement::Ref(n) => walk_ref_body(report, &p, &n.value.body),
            UseCaseDefBodyElement::InOutDecl(n) => walk_in_out_decl(report, &p, &n.value),
            UseCaseDefBodyElement::AssertConstraint(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            UseCaseDefBodyElement::ReturnRef(n) => {
                walk_return_ref_body(report, &p, &n.value.body.value)
            }
            UseCaseDefBodyElement::ForLoop(n) => walk_action_def_body(report, &p, &n.value.body),
            UseCaseDefBodyElement::ThenAction(n) => walk_then_target(report, &p, &n.value.target),
            UseCaseDefBodyElement::ActionUsage(n) => {
                walk_optional_action_usage_body(report, &p, &n.value.body)
            }
            UseCaseDefBodyElement::AnalysisCaseUsage(n) => {
                walk_use_case_def_body(report, &p, &n.value.body)
            }
            UseCaseDefBodyElement::CalcUsage(n) => walk_calc_def_body(report, &p, &n.value.body),
            UseCaseDefBodyElement::AttributeUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            UseCaseDefBodyElement::RequirementUsage(n) => {
                walk_requirement_def_body(report, &p, &n.value.body)
            }
            UseCaseDefBodyElement::PartUsage(n) => walk_part_usage_body(report, &p, &n.value.body),
            UseCaseDefBodyElement::FlowUsage(n) => walk_definition_body(report, &p, &n.value.body),
            UseCaseDefBodyElement::SubjectDecl(_)
            | UseCaseDefBodyElement::SubjectRef(_)
            | UseCaseDefBodyElement::ActorUsage(_)
            | UseCaseDefBodyElement::ActorRedefinitionAssignment(_)
            | UseCaseDefBodyElement::FirstSuccession(_)
            | UseCaseDefBodyElement::ThenDone(_)
            | UseCaseDefBodyElement::CaseReturnDecl(_)
            | UseCaseDefBodyElement::Assign(_)
            | UseCaseDefBodyElement::Expression(_) => {}
        }
    }
}

fn walk_state_def_body(report: &mut OpacityReport, path: &str, body: &StateDefBody) {
    let StateDefBody::Brace { elements, .. } = body else {
        return;
    };
    walk_state_def_body_elements(report, path, elements);
}

fn walk_state_def_body_elements(
    report: &mut OpacityReport,
    path: &str,
    elements: &[crate::ast::Node<StateDefBodyElement>],
) {
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            StateDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            StateDefBodyElement::Entry(n) => walk_state_def_body(report, &p, &n.value.body),
            StateDefBodyElement::Do(n) => walk_state_def_body(report, &p, &n.value.body),
            StateDefBodyElement::Exit(n) => walk_state_def_body(report, &p, &n.value.body),
            StateDefBodyElement::StateUsage(n) => walk_state_def_body(report, &p, &n.value.body),
            StateDefBodyElement::Ref(n) => walk_ref_body(report, &p, &n.value.body),
            StateDefBodyElement::Annotating(member) => walk_annotating_member(report, &p, member),
            StateDefBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
            StateDefBodyElement::RequirementUsage(n) => {
                walk_requirement_def_body(report, &p, &n.value.body)
            }
            StateDefBodyElement::Transition(n) => walk_action_def_body(report, &p, &n.value.body),
            StateDefBodyElement::InOutDecl(n) => walk_in_out_decl(report, &p, &n.value),
            StateDefBodyElement::AttributeUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            StateDefBodyElement::ActionUsage(n) => {
                walk_optional_action_usage_body(report, &p, &n.value.body)
            }
            StateDefBodyElement::SuccessionUsage(n) => walk_ref_body(report, &p, &n.value.body),
            StateDefBodyElement::AssertConstraint(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            StateDefBodyElement::Then(_) | StateDefBodyElement::FinalState(_) => {}
        }
    }
}

fn walk_view_def_body(report: &mut OpacityReport, path: &str, body: &ViewDefBody) {
    let ViewDefBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            ViewDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            ViewDefBodyElement::Unsupported(_) => hit(report, &p, OpacityKind::UnsupportedGrammar),
            ViewDefBodyElement::Annotating(member) => walk_annotating_member(report, &p, member),
            ViewDefBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
            ViewDefBodyElement::ViewRendering(n) => {
                walk_rendering_usage_body(report, &p, &n.value.body)
            }
            ViewDefBodyElement::RefDecl(n) => walk_ref_body(report, &p, &n.value.body),
            ViewDefBodyElement::ViewpointUsage(_)
            | ViewDefBodyElement::Satisfy(_)
            | ViewDefBodyElement::Filter(_) => {}
        }
    }
}

fn walk_view_body(report: &mut OpacityReport, path: &str, body: &ViewBody) {
    let ViewBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        walk_view_body_element(report, &format!("{path}/body[{i}]"), &el.value);
    }
}

fn walk_view_body_element(report: &mut OpacityReport, path: &str, el: &ViewBodyElement) {
    match el {
        ViewBodyElement::Error(_) => hit(report, path, OpacityKind::ParseError),
        ViewBodyElement::ViewRendering(n) => walk_rendering_usage_body(report, path, &n.value.body),
        ViewBodyElement::Expose(n) => walk_relationship_body(report, path, &n.value.body),
        ViewBodyElement::Satisfy(n) => walk_satisfy(report, path, &n.value),
        ViewBodyElement::RefDecl(n) => walk_ref_body(report, path, &n.value.body),
        ViewBodyElement::Annotating(member) => walk_annotating_member(report, path, member),
        ViewBodyElement::Filter(_) => {}
    }
}

fn walk_definition_body(report: &mut OpacityReport, path: &str, body: &DefinitionBody) {
    let DefinitionBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            DefinitionBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            DefinitionBodyElement::Unsupported(_) => {
                hit(report, &p, OpacityKind::UnsupportedGrammar)
            }
            DefinitionBodyElement::OccurrenceMember(n) => {
                walk_occurrence_body_element(report, &p, &n.value)
            }
        }
    }
}

fn walk_occurrence_usage_body(report: &mut OpacityReport, path: &str, body: &OccurrenceUsageBody) {
    let OccurrenceUsageBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        walk_occurrence_body_element(report, &format!("{path}/body[{i}]"), &el.value);
    }
}

fn walk_occurrence_body_element(
    report: &mut OpacityReport,
    path: &str,
    el: &OccurrenceBodyElement,
) {
    match el {
        OccurrenceBodyElement::Error(_) => hit(report, path, OpacityKind::ParseError),
        OccurrenceBodyElement::MetadataKeywordUsage(n) => {
            walk_optional_attribute_body(report, path, &n.value.body)
        }
        OccurrenceBodyElement::AssertConstraint(n) => {
            walk_constraint_def_body(report, path, &n.value.body)
        }
        OccurrenceBodyElement::FlowUsage(n) => walk_definition_body(report, path, &n.value.body),
        OccurrenceBodyElement::AttributeUsage(n) => {
            walk_attribute_body(report, path, &n.value.body)
        }
        OccurrenceBodyElement::PartUsage(n) => walk_part_usage_body(report, path, &n.value.body),
        OccurrenceBodyElement::ItemUsage(n) => walk_attribute_body(report, path, &n.value.body),
        OccurrenceBodyElement::OccurrenceUsage(n) => {
            walk_occurrence_usage_body(report, path, &n.value.body)
        }
        OccurrenceBodyElement::SuccessionUsage(n) => walk_ref_body(report, path, &n.value.body),
        OccurrenceBodyElement::Satisfy(n) => walk_satisfy(report, path, &n.value),
        OccurrenceBodyElement::Allocate(n) => walk_ref_body(report, path, &n.value.body),
        OccurrenceBodyElement::EndDecl(n) => walk_end_decl(report, path, &n.value),
        OccurrenceBodyElement::StateUsage(n) => walk_state_def_body(report, path, &n.value.body),
        OccurrenceBodyElement::RefDecl(n) => walk_ref_body(report, path, &n.value.body),
        OccurrenceBodyElement::ConnectionUsage(n) => {
            walk_connection_def_body(report, path, &n.value.body)
        }
        OccurrenceBodyElement::Annotating(member) => walk_annotating_member(report, path, member),
    }
}

fn walk_rendering_usage_body(report: &mut OpacityReport, path: &str, body: &RenderingUsageBody) {
    let RenderingUsageBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, element) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &element.value {
            RenderingUsageBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            RenderingUsageBodyElement::ViewUsage(view) => {
                walk_view_body(report, &p, &view.value.body)
            }
            RenderingUsageBodyElement::Rendering(nested) => {
                walk_rendering_usage_body(report, &p, &nested.value.body)
            }
            RenderingUsageBodyElement::Annotating(member) => {
                walk_annotating_member(report, &p, member)
            }
        }
    }
}

fn walk_return_ref_body(report: &mut OpacityReport, path: &str, body: &ReturnRefBody) {
    let ReturnRefBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, element) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &element.value {
            ReturnRefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            ReturnRefBodyElement::Annotating(member) => walk_annotating_member(report, &p, member),
            ReturnRefBodyElement::Result(_) => {}
        }
    }
}

fn walk_constraint_body_elements(
    report: &mut OpacityReport,
    path: &str,
    elements: &[crate::ast::Node<ConstraintDefBodyElement>],
) {
    for (i, element) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &element.value {
            ConstraintDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            ConstraintDefBodyElement::Constraint(constraint) => {
                walk_constraint_def_body(report, &p, &constraint.value.body)
            }
            ConstraintDefBodyElement::FeatureDecl(_) => {}
            ConstraintDefBodyElement::AttributeUsage(attribute) => {
                walk_attribute_body(report, &p, &attribute.value.body)
            }
            ConstraintDefBodyElement::Annotating(member) => {
                walk_annotating_member(report, &p, member)
            }
            ConstraintDefBodyElement::MetadataKeywordUsage(n) => {
                walk_optional_attribute_body(report, &p, &n.value.body)
            }
            ConstraintDefBodyElement::InOutDecl(n) => walk_in_out_decl(report, &p, &n.value),
            ConstraintDefBodyElement::RequireConstraint(n) => {
                walk_constraint_def_body(report, &p, &n.value.body)
            }
            ConstraintDefBodyElement::PartUsage(pu) => {
                walk_part_usage_body(report, &p, &pu.value.body)
            }
            ConstraintDefBodyElement::Expression(_) => {}
        }
    }
}

fn walk_ref_body(report: &mut OpacityReport, path: &str, body: &RefBody) {
    let RefBody::Brace { elements, .. } = body else {
        return;
    };
    walk_part_usage_body_elements(report, path, elements);
}

/// Only a metadata annotation owns a body that can hide opaque members; documentation, comments,
/// and textual representations are leaves.
fn walk_annotating_member(
    report: &mut OpacityReport,
    path: &str,
    member: &crate::ast::AnnotatingMember,
) {
    match member {
        crate::ast::AnnotatingMember::MetadataAnnotation(metadata) => {
            walk_attribute_body(report, path, &metadata.value.body)
        }
        crate::ast::AnnotatingMember::Doc(_)
        | crate::ast::AnnotatingMember::Comment(_)
        | crate::ast::AnnotatingMember::TextualRep(_) => {}
    }
}

fn walk_relationship_body_elements(
    report: &mut OpacityReport,
    path: &str,
    elements: &[crate::ast::Node<RelationshipBodyElement>],
) {
    for (i, element) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &element.value {
            RelationshipBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            RelationshipBodyElement::KermlFeature(n) => {
                walk_calc_def_body(report, &p, &n.value.body)
            }
            RelationshipBodyElement::Annotating(member) => {
                walk_annotating_member(report, &p, member)
            }
        }
    }
}

fn walk_relationship_body(
    report: &mut OpacityReport,
    path: &str,
    body: &crate::ast::Body<RelationshipBodyElement>,
) {
    if let Some(elements) = body.braced_elements() {
        walk_relationship_body_elements(report, path, elements);
    }
}

fn walk_optional_relationship_body(
    report: &mut OpacityReport,
    path: &str,
    elements: Option<&[crate::ast::Node<RelationshipBodyElement>]>,
) {
    if let Some(elements) = elements {
        walk_relationship_body_elements(report, path, elements);
    }
}

fn walk_interface_usage(report: &mut OpacityReport, path: &str, usage: &InterfaceUsage) {
    let elements: &[crate::ast::Node<InterfaceUsageBodyElement>] = match usage {
        InterfaceUsage::TypedConnect { body, .. }
        | InterfaceUsage::Connection { body, .. }
        | InterfaceUsage::Declaration { body, .. } => body.braced_elements().unwrap_or(&[]),
    };
    for (i, element) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &element.value {
            InterfaceUsageBodyElement::RefRedef { body, .. } => walk_ref_body(report, &p, body),
            InterfaceUsageBodyElement::EndDecl(end) => walk_end_decl(report, &p, &end.value),
            InterfaceUsageBodyElement::Annotating(member) => {
                walk_annotating_member(report, &p, member)
            }
        }
    }
}

fn walk_end_decl(report: &mut OpacityReport, path: &str, end: &crate::ast::EndDecl) {
    match end.nested_usage.as_deref() {
        Some(EndNestedUsage::Occurrence(occurrence)) => {
            walk_occurrence_usage_body(report, path, &occurrence.value.body)
        }
        Some(EndNestedUsage::Item(item)) => walk_attribute_body(report, path, &item.value.body),
        None => {}
    }
}

fn walk_satisfy(
    report: &mut OpacityReport,
    path: &str,
    satisfy: &crate::ast::SatisfyRequirementUsage,
) {
    walk_requirement_def_body(report, path, &satisfy.body);
}

fn walk_bind(report: &mut OpacityReport, path: &str, bind: &crate::ast::Bind) {
    walk_part_usage_body(report, path, &bind.body);
}

fn walk_variant_usage(report: &mut OpacityReport, path: &str, variant: &VariantUsage) {
    if let Some(body) = &variant.body {
        walk_part_usage_body(report, path, body);
    }
    match &variant.typed {
        Some(VariantTypedUsage::Part(part)) => walk_part_usage_body(report, path, &part.value.body),
        Some(VariantTypedUsage::Attribute(attribute)) => {
            walk_attribute_body(report, path, &attribute.value.body)
        }
        Some(VariantTypedUsage::Item(item)) => walk_attribute_body(report, path, &item.value.body),
        Some(VariantTypedUsage::Port(port)) => walk_port_body(report, path, &port.value.body),
        Some(VariantTypedUsage::Requirement(requirement)) => {
            walk_requirement_def_body(report, path, &requirement.value.body)
        }
        Some(VariantTypedUsage::Perform(perform)) => {
            walk_perform_body(report, path, &perform.value.body)
        }
        None => {}
    }
}

fn walk_perform_body(report: &mut OpacityReport, path: &str, body: &PerformBody) {
    let PerformBody::Brace { elements, .. } = body else {
        return;
    };
    for (i, element) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &element.value {
            PerformBodyElement::Variant(variant) => walk_variant_usage(report, &p, &variant.value),
            PerformBodyElement::Action(action) => {
                walk_action_usage_body_elements(report, &p, std::slice::from_ref(action))
            }
            PerformBodyElement::PartUsage(part) => {
                walk_part_usage_body(report, &p, &part.value.body)
            }
            PerformBodyElement::ItemUsage(item) => {
                walk_attribute_body(report, &p, &item.value.body)
            }
            PerformBodyElement::AttributeUsage(attribute) => {
                walk_attribute_body(report, &p, &attribute.value.body)
            }
            PerformBodyElement::Annotating(member) => walk_annotating_member(report, &p, member),
            PerformBodyElement::InOut(_) => {}
        }
    }
}

fn walk_then_target(report: &mut OpacityReport, path: &str, target: &ThenTarget) {
    match target {
        ThenTarget::Action(action) => {
            walk_optional_action_usage_body(report, path, &action.value.body)
        }
        ThenTarget::Perform(perform) => walk_perform_body(report, path, &perform.value.body),
        ThenTarget::Merge(merge) => walk_first_merge_body(report, path, &merge.value.body),
        ThenTarget::Fork(fork) => walk_first_merge_body(report, path, &fork.value.body),
        ThenTarget::Decide(decision) => walk_first_merge_body(report, path, &decision.value.body),
        ThenTarget::Send(action) => {
            walk_optional_action_usage_body(report, path, &action.value.body)
        }
        ThenTarget::Accept(_) | ThenTarget::Feature(_) => {}
    }
}

fn walk_first_merge_body(report: &mut OpacityReport, path: &str, body: &FirstMergeBody) {
    match body {
        FirstMergeBody::Semicolon => {}
        FirstMergeBody::Brace(body) => {
            for (index, element) in body.value.elements.iter().enumerate() {
                let element_path = format!("{path}/body[{index}]");
                match &element.value {
                    crate::ast::FirstMergeBodyElement::Member(member) => {
                        walk_action_def_body_elements(
                            report,
                            &element_path,
                            std::slice::from_ref(member.as_ref()),
                        );
                    }
                    crate::ast::FirstMergeBodyElement::Unsupported(_) => {
                        hit(report, &element_path, OpacityKind::UnsupportedGrammar)
                    }
                    crate::ast::FirstMergeBodyElement::Error(_) => {
                        hit(report, &element_path, OpacityKind::ParseError)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{opacity_report, OpacityHit, OpacityKind};

    #[test]
    fn reports_recovery_nested_inside_interface_definition() {
        let source = "package P { part def PP { interface def I { this is not valid at all; } } }";
        let parsed = crate::parse_for_editor(source);

        assert!(!parsed.errors.is_empty(), "fixture must exercise recovery");
        assert!(
            opacity_report(&parsed.document.root)
                .hits
                .contains(&OpacityHit {
                    path: "root[0]/body[0]/body[0]/body[0]".to_string(),
                    kind: OpacityKind::ParseError,
                }),
            "nested interface recovery must make the document opaque"
        );
    }
}
