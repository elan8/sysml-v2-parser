//! Walk an AST and report opaque / recovery nodes that would fake-pass roundtrips.

use crate::ast::{
    ActionDefBody, ActionDefBodyElement, ActionUsageBody, ActionUsageBodyElement, AttributeBody,
    AttributeBodyElement, ConnectBody, DefinitionBody, DefinitionBodyElement, LibraryPackage,
    OccurrenceBodyElement, OccurrenceUsageBody, Package, PackageBody, PackageBodyElement,
    PartDefBody, PartDefBodyElement, PartUsageBody, PartUsageBodyElement, PortDefBody,
    PortDefBodyElement, RequirementDefBody, RequirementDefBodyElement, RootElement, RootNamespace,
    StateDefBody, StateDefBodyElement, UseCaseDefBody, UseCaseDefBodyElement, ViewBody,
    ViewBodyElement, ViewDefBody, ViewDefBodyElement,
};

/// Kind of opaque or recovery content found in an AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpacityKind {
    Other,
    OpaqueMember,
    ExtendedLibraryDecl,
    KermlSemanticDecl,
    KermlFeatureDecl,
    FeatureDecl,
    ClassifierDecl,
    ActionBodyDecl,
    RawRhsString,
    RawBodyString,
    OpaqueConnectBrace,
    ParseError,
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
            RootElement::Import(_) => {}
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
    let PackageBody::Brace { elements } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        walk_package_body_element(report, &format!("{path}/body[{i}]"), &el.value);
    }
}

fn walk_package_body_element(report: &mut OpacityReport, path: &str, el: &PackageBodyElement) {
    match el {
        PackageBodyElement::Error(_) => hit(report, path, OpacityKind::ParseError),
        PackageBodyElement::FeatureDecl(_) => hit(report, path, OpacityKind::FeatureDecl),
        PackageBodyElement::ClassifierDecl(_) => hit(report, path, OpacityKind::ClassifierDecl),
        PackageBodyElement::KermlSemanticDecl(_) => {
            hit(report, path, OpacityKind::KermlSemanticDecl)
        }
        PackageBodyElement::KermlFeatureDecl(_) => hit(report, path, OpacityKind::KermlFeatureDecl),
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
        PackageBodyElement::ActionDef(a) => walk_action_def_body(report, path, &a.value.body),
        PackageBodyElement::ActionUsage(a) => walk_action_usage_body(report, path, &a.value.body),
        PackageBodyElement::RequirementDef(r) => {
            walk_requirement_def_body(report, path, &r.value.body)
        }
        PackageBodyElement::RequirementUsage(r) => {
            walk_requirement_def_body(report, path, &r.value.body)
        }
        PackageBodyElement::StateDef(s) => walk_state_def_body(report, path, &s.value.body),
        PackageBodyElement::UseCaseDef(u) => walk_use_case_def_body(report, path, &u.value.body),
        PackageBodyElement::ViewDef(v) => walk_view_def_body(report, path, &v.value.body),
        PackageBodyElement::ViewUsage(v) => walk_view_body(report, path, &v.value.body),
        PackageBodyElement::Connect(c) => walk_connect_body(report, path, &c.value.body),
        PackageBodyElement::ItemDef(i) => walk_attribute_body(report, path, &i.value.body),
        PackageBodyElement::IndividualDef(i) => walk_attribute_body(report, path, &i.value.body),
        PackageBodyElement::OccurrenceDef(o) => walk_definition_body(report, path, &o.value.body),
        PackageBodyElement::OccurrenceUsage(o) => {
            walk_occurrence_usage_body(report, path, &o.value.body)
        }
        PackageBodyElement::AllocationDef(a) => walk_definition_body(report, path, &a.value.body),
        PackageBodyElement::AllocationUsage(a) => walk_definition_body(report, path, &a.value.body),
        _ => {}
    }
}

fn walk_part_def_body(report: &mut OpacityReport, path: &str, body: &PartDefBody) {
    let PartDefBody::Brace { elements } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            PartDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            PartDefBodyElement::Other(_) => hit(report, &p, OpacityKind::Other),
            PartDefBodyElement::OpaqueMember(_) => hit(report, &p, OpacityKind::OpaqueMember),
            PartDefBodyElement::Connect(c) => walk_connect_body(report, &p, &c.value.body),
            PartDefBodyElement::PartDef(n) => walk_part_def_body(report, &p, &n.value.body),
            PartDefBodyElement::PartUsage(n) => walk_part_usage_body(report, &p, &n.value.body),
            PartDefBodyElement::AttributeDef(n) => walk_attribute_body(report, &p, &n.value.body),
            PartDefBodyElement::AttributeUsage(n) => walk_attribute_body(report, &p, &n.value.body),
            PartDefBodyElement::ActionDef(n) => walk_action_def_body(report, &p, &n.value.body),
            PartDefBodyElement::ActionUsage(n) => walk_action_usage_body(report, &p, &n.value.body),
            _ => {}
        }
    }
}

fn walk_part_usage_body(report: &mut OpacityReport, path: &str, body: &PartUsageBody) {
    let PartUsageBody::Brace { elements } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            PartUsageBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            PartUsageBodyElement::Connect(c) => walk_connect_body(report, &p, &c.value.body),
            PartUsageBodyElement::PartUsage(n) => walk_part_usage_body(report, &p, &n.value.body),
            PartUsageBodyElement::AttributeUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            PartUsageBodyElement::ActionUsage(n) => {
                walk_action_usage_body(report, &p, &n.value.body)
            }
            _ => {}
        }
    }
}

fn walk_attribute_body(report: &mut OpacityReport, path: &str, body: &AttributeBody) {
    let AttributeBody::Brace { elements } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            AttributeBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            AttributeBodyElement::Other(_) => hit(report, &p, OpacityKind::Other),
            AttributeBodyElement::Connect(c) => walk_connect_body(report, &p, &c.value.body),
            AttributeBodyElement::AttributeDef(n) => walk_attribute_body(report, &p, &n.value.body),
            AttributeBodyElement::AttributeUsage(n) => {
                walk_attribute_body(report, &p, &n.value.body)
            }
            _ => {}
        }
    }
}

fn walk_port_def_body(report: &mut OpacityReport, path: &str, body: &PortDefBody) {
    let PortDefBody::Brace { elements } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            PortDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            PortDefBodyElement::Other(_) => hit(report, &p, OpacityKind::Other),
            _ => {}
        }
    }
}

fn walk_action_def_body(report: &mut OpacityReport, path: &str, body: &ActionDefBody) {
    let ActionDefBody::Brace { elements } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            ActionDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            ActionDefBodyElement::Decl(_) => hit(report, &p, OpacityKind::ActionBodyDecl),
            _ => {}
        }
    }
}

fn walk_action_usage_body(report: &mut OpacityReport, path: &str, body: &ActionUsageBody) {
    let ActionUsageBody::Brace { elements } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            ActionUsageBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            ActionUsageBodyElement::Decl(_) => hit(report, &p, OpacityKind::ActionBodyDecl),
            _ => {}
        }
    }
}

fn walk_requirement_def_body(report: &mut OpacityReport, path: &str, body: &RequirementDefBody) {
    let RequirementDefBody::Brace { elements } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            RequirementDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            RequirementDefBodyElement::Other(_) => hit(report, &p, OpacityKind::Other),
            _ => {}
        }
    }
}

fn walk_use_case_def_body(report: &mut OpacityReport, path: &str, body: &UseCaseDefBody) {
    let UseCaseDefBody::Brace { elements } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            UseCaseDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            UseCaseDefBodyElement::Other(_) => hit(report, &p, OpacityKind::Other),
            UseCaseDefBodyElement::ActorRedefinitionAssignment(_) => {
                hit(report, &p, OpacityKind::RawRhsString)
            }
            UseCaseDefBodyElement::RefRedefinition(_) => {
                hit(report, &p, OpacityKind::RawBodyString)
            }
            UseCaseDefBodyElement::ReturnRef(_) => hit(report, &p, OpacityKind::RawBodyString),
            _ => {}
        }
    }
}

fn walk_state_def_body(report: &mut OpacityReport, path: &str, body: &StateDefBody) {
    let StateDefBody::Brace { elements } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            StateDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            StateDefBodyElement::Other(_) => hit(report, &p, OpacityKind::Other),
            StateDefBodyElement::Entry(n) => walk_state_def_body(report, &p, &n.value.body),
            StateDefBodyElement::Do(n) => walk_state_def_body(report, &p, &n.value.body),
            StateDefBodyElement::Exit(n) => walk_state_def_body(report, &p, &n.value.body),
            StateDefBodyElement::StateUsage(n) => walk_state_def_body(report, &p, &n.value.body),
            StateDefBodyElement::Ref(n) => {
                if let crate::ast::RefBody::Brace { elements } = &n.value.body {
                    for (j, nested) in elements.iter().enumerate() {
                        if let crate::ast::RefBodyElement::State(s) = &nested.value {
                            let nested_path = format!("{p}/body[{j}]");
                            match &s.value {
                                StateDefBodyElement::Error(_) => {
                                    hit(report, &nested_path, OpacityKind::ParseError)
                                }
                                StateDefBodyElement::Other(_) => {
                                    hit(report, &nested_path, OpacityKind::Other)
                                }
                                StateDefBodyElement::Entry(e) => {
                                    walk_state_def_body(report, &nested_path, &e.value.body)
                                }
                                StateDefBodyElement::Do(d) => {
                                    walk_state_def_body(report, &nested_path, &d.value.body)
                                }
                                StateDefBodyElement::Exit(e) => {
                                    walk_state_def_body(report, &nested_path, &e.value.body)
                                }
                                StateDefBodyElement::StateUsage(u) => {
                                    walk_state_def_body(report, &nested_path, &u.value.body)
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn walk_view_def_body(report: &mut OpacityReport, path: &str, body: &ViewDefBody) {
    let ViewDefBody::Brace { elements } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            ViewDefBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            ViewDefBodyElement::Other(_) => hit(report, &p, OpacityKind::Other),
            _ => {}
        }
    }
}

fn walk_view_body(report: &mut OpacityReport, path: &str, body: &ViewBody) {
    let ViewBody::Brace { elements } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        walk_view_body_element(report, &format!("{path}/body[{i}]"), &el.value);
    }
}

fn walk_view_body_element(report: &mut OpacityReport, path: &str, el: &ViewBodyElement) {
    match el {
        ViewBodyElement::Error(_) => hit(report, path, OpacityKind::ParseError),
        ViewBodyElement::Other(_) => hit(report, path, OpacityKind::Other),
        _ => {}
    }
}

fn walk_definition_body(report: &mut OpacityReport, path: &str, body: &DefinitionBody) {
    let DefinitionBody::Brace { elements } = body else {
        return;
    };
    for (i, el) in elements.iter().enumerate() {
        let p = format!("{path}/body[{i}]");
        match &el.value {
            DefinitionBodyElement::Error(_) => hit(report, &p, OpacityKind::ParseError),
            DefinitionBodyElement::Other(_) => hit(report, &p, OpacityKind::Other),
            _ => {}
        }
    }
}

fn walk_occurrence_usage_body(report: &mut OpacityReport, path: &str, body: &OccurrenceUsageBody) {
    let OccurrenceUsageBody::Brace { elements } = body else {
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
        OccurrenceBodyElement::Other(_) => hit(report, path, OpacityKind::Other),
        _ => {}
    }
}

fn walk_connect_body(report: &mut OpacityReport, path: &str, body: &ConnectBody) {
    if matches!(body, ConnectBody::Brace) {
        hit(report, path, OpacityKind::OpaqueConnectBrace);
    }
}
