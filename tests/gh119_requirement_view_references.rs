use sysml_v2_parser::ast::{
    PackageBody, PackageBodyElement, ReferenceSeparator, RequirementDefBody,
    RequirementDefBodyElement, RootElement, ViewBody, ViewBodyElement,
};
use sysml_v2_parser::{emit_sysml, parse};

fn package_elements(
    document: &sysml_v2_parser::ast::ParsedDocument,
) -> &[sysml_v2_parser::ast::Node<PackageBodyElement>] {
    let package = match &document.root.elements[0].value {
        RootElement::Package(package) => &package.value,
        other => panic!("expected package, got {other:?}"),
    };
    match &package.body {
        PackageBody::Brace { elements, .. } => elements,
        other => panic!("expected package body, got {other:?}"),
    }
}

#[test]
fn requirement_and_dependency_references_use_document_local_ids() {
    let source = r#"package P {
        dependency from Client::A to $::Supplier::B;
        requirement def R {
            subject s : $::Domain::Subject;
            actor a : Domain::Actor;
            verify Requirements::Req :>> Base::Req;
        }
    }"#;
    let document = parse(source).expect("parse typed requirement references");
    let elements = package_elements(&document);

    let dependency = match &elements[0].value {
        PackageBodyElement::Dependency(dependency) => &dependency.value,
        other => panic!("expected dependency, got {other:?}"),
    };
    let client = document
        .qualified_reference(dependency.clients[0])
        .expect("client reference");
    assert_eq!(client.authored_text(), "Client::A");
    assert_eq!(client.segments.len(), 2);
    assert_eq!(
        client.segments[1].separator_before,
        Some(ReferenceSeparator::ColonColon)
    );
    let supplier = document
        .qualified_reference(dependency.suppliers[0])
        .expect("supplier reference");
    assert_eq!(supplier.authored_text(), "$::Supplier::B");
    assert!(supplier.metadata.is_absolute);

    let requirement = match &elements[1].value {
        PackageBodyElement::RequirementDef(requirement) => &requirement.value,
        other => panic!("expected requirement definition, got {other:?}"),
    };
    let RequirementDefBody::Brace { elements, .. } = &requirement.body else {
        panic!("expected requirement body");
    };
    let subject = match &elements[0].value {
        RequirementDefBodyElement::SubjectDecl(subject) => &subject.value,
        other => panic!("expected subject, got {other:?}"),
    };
    assert_eq!(
        document
            .qualified_reference(subject.type_name.expect("subject type"))
            .expect("subject reference")
            .authored_text(),
        "$::Domain::Subject"
    );
    let actor = match &elements[1].value {
        RequirementDefBodyElement::RequirementActorDecl(actor) => &actor.value,
        other => panic!("expected actor, got {other:?}"),
    };
    assert_eq!(
        document
            .qualified_reference(actor.type_name)
            .expect("actor reference")
            .authored_text(),
        "Domain::Actor"
    );
    let verify = match &elements[2].value {
        RequirementDefBodyElement::VerifyRequirement(verify) => &verify.value,
        other => panic!("expected verify, got {other:?}"),
    };
    assert_eq!(
        document
            .qualified_reference(verify.target.expect("verify target"))
            .expect("verify target reference")
            .authored_text(),
        "Requirements::Req"
    );
    assert_eq!(
        document
            .qualified_reference(verify.redefines.expect("verify redefinition"))
            .expect("verify redefinition reference")
            .authored_text(),
        "Base::Req"
    );

    let emitted = emit_sysml(&document).expect("emit typed requirement references");
    parse(&emitted).expect("reparse emitted requirement references");
}

#[test]
fn view_type_and_satisfy_target_use_source_backed_ids() {
    let document =
        parse("package Views { view v : $::Views::General { satisfy Viewpoints::VP; } }")
            .expect("parse typed view references");
    let view = match &package_elements(&document)[0].value {
        PackageBodyElement::ViewUsage(view) => &view.value,
        other => panic!("expected view usage, got {other:?}"),
    };
    let view_type = document
        .qualified_reference(view.type_name.expect("view type"))
        .expect("view type reference");
    assert_eq!(view_type.authored_text(), "$::Views::General");
    assert!(view_type.metadata.is_absolute);

    let satisfy = match &view.body {
        ViewBody::Brace { elements, .. } => match &elements[0].value {
            ViewBodyElement::Satisfy(satisfy) => &satisfy.value,
            other => panic!("expected satisfy member, got {other:?}"),
        },
        other => panic!("expected view body, got {other:?}"),
    };
    let target = document
        .qualified_reference(satisfy.viewpoint_ref)
        .expect("viewpoint reference");
    assert_eq!(target.authored_text(), "Viewpoints::VP");
    assert_eq!(target.segments.len(), 2);

    let emitted = emit_sysml(&document).expect("emit typed view references");
    parse(&emitted).expect("reparse emitted view references");
}
