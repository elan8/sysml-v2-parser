//! GH-51: fixing `interface_def_body`'s silent recovery required wiring `collect_errors.rs` to
//! actually collect `ConnectionDef`/`InterfaceDef` diagnostics at all (previously never wired up,
//! regardless of nesting), which in turn surfaced several previously-invisible real gaps against
//! the vendored SysML v2 Systems/Domain Libraries. These tests cover each confirmed real-usage
//! fix, using the exact (trimmed) real-library lines that motivated it.

use sysml_v2_parser::ast::{
    ConnectionDefBody, ConnectionDefBodyElement, InterfaceDefBody, InterfaceDefBodyElement,
    PackageBody, PackageBodyElement, RootElement,
};
use sysml_v2_parser::parse_with_diagnostics;

fn package_elements(input: &str) -> Vec<sysml_v2_parser::Node<PackageBodyElement>> {
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        other => panic!("expected package, got {other:?}"),
    };
    match &pkg.body {
        PackageBody::Brace { elements, .. } => elements.clone(),
        _ => panic!("expected brace package body"),
    }
}

fn connection_def_elements(
    elements: &[sysml_v2_parser::Node<PackageBodyElement>],
) -> Vec<sysml_v2_parser::Node<ConnectionDefBodyElement>> {
    let connection = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::ConnectionDef(c) => Some(&c.value),
            _ => None,
        })
        .expect("expected connection def");
    match &connection.body {
        ConnectionDefBody::Brace { elements, .. } => elements.clone(),
        _ => panic!("expected connection def brace body"),
    }
}

/// Real usage: Systems Library `Connections.sysml`'s `end source: Anything :>>
/// BinaryLinkObject::source;` -- `:>>` redefines trailing the typed (`:`) form, distinct from the
/// `::>`/`references` form.
#[test]
fn end_decl_accepts_trailing_redefines_after_typed_form() {
    let input =
        "package P {\nconnection def C {\nend source: Anything :>> BinaryLinkObject::source;\n}\n}";
    let elements = connection_def_elements(&package_elements(input));
    let end = elements
        .iter()
        .find_map(|e| match &e.value {
            ConnectionDefBodyElement::EndDecl(end) => Some(&end.value),
            _ => None,
        })
        .expect("expected end decl");
    assert!(end.typing.is_some());
    assert_eq!(
        end.redefines.as_ref().map(|n| n.value.target.len()),
        Some(1)
    );
}

/// Real usage: Systems/Domain Library connection defs own `assert constraint` members with a
/// visibility prefix, e.g. `private assert constraint disjointCauseEffect { ... }`
/// (`CausationConnections.sysml`) -- previously not dispatched in connection def bodies at all,
/// and `assert_constraint_member` never parsed a visibility prefix regardless of context.
#[test]
fn connection_def_body_accepts_private_assert_constraint() {
    let input =
        "package P {\nconnection def C {\nprivate assert constraint disjointCauseEffect {\ntrue\n}\n}\n}";
    let elements = connection_def_elements(&package_elements(input));
    let assert_member = elements
        .iter()
        .find_map(|e| match &e.value {
            ConnectionDefBodyElement::AssertConstraint(a) => Some(&a.value),
            _ => None,
        })
        .expect("expected assert constraint member");
    assert_eq!(
        assert_member.declaration_name.as_deref(),
        Some("disjointCauseEffect")
    );
    assert_eq!(
        assert_member.membership.visibility,
        Some(sysml_v2_parser::ast::Visibility::Private)
    );
}

/// Real usage: Systems Library `Domain Libraries/Cause and Effect/CausationConnections.sysml`'s
/// `abstract constant ref occurrence causes[1..*] :>> causes :> participant { ... }` --
/// `occurrence_usage` previously had no `abstract`/`constant` prefix support and no multiplicity
/// support at all (regardless of context), and connection def bodies never dispatched
/// `occurrence_usage` in the first place.
#[test]
fn connection_def_body_accepts_abstract_constant_ref_occurrence_with_multiplicity() {
    let input = "package P {\nconnection def C {\nabstract constant ref occurrence causes[1..*] :>> causes :> participant {\n}\n}\n}";
    let elements = connection_def_elements(&package_elements(input));
    let occurrence = elements
        .iter()
        .find_map(|e| match &e.value {
            ConnectionDefBodyElement::OccurrenceUsage(o) => Some(&o.value),
            _ => None,
        })
        .expect("expected occurrence usage");
    let ref_prefix = &occurrence.prefix.basic.ref_prefix;
    assert_eq!(
        ref_prefix.variance.as_ref().map(|node| node.value),
        Some(sysml_v2_parser::ast::DefinitionPrefix::Abstract)
    );
    assert!(ref_prefix.constant_span.is_some());
    assert!(occurrence.prefix.basic.reference_span.is_some());
    assert!(occurrence.multiplicity.is_some());
}

/// Real usage: `CausationConnections.sysml`'s `private succession causalOrdering first [nCauses]
/// causes.startShot then [nEffects] effects { ... }` -- `succession_usage` previously had no
/// support for naming the succession usage itself, and connection def bodies never dispatched it.
#[test]
fn connection_def_body_accepts_named_succession_usage() {
    let input = "package P {\nconnection def C {\nprivate succession causalOrdering first [1] causes then [1] effects {\n}\n}\n}";
    let elements = connection_def_elements(&package_elements(input));
    let succession = elements
        .iter()
        .find_map(|e| match &e.value {
            ConnectionDefBodyElement::SuccessionUsage(s) => Some(&s.value),
            _ => None,
        })
        .expect("expected succession usage");
    assert_eq!(succession.name.as_deref(), Some("causalOrdering"));
    assert_eq!(
        succession.membership.visibility,
        Some(sysml_v2_parser::ast::Visibility::Private)
    );
}

/// Real usage: Systems Library `Interfaces.sysml`'s `ref port :>> participant : Port [2..*]
/// nonunique ordered { ... }` (anonymous -- no name at all, just a kind keyword + redefines +
/// type + multiplicity + modifiers) and `ref port :>> Interface::participant,
/// BinaryConnection::participant[2] nonunique ordered;` (comma-separated multi-target redefines).
///
/// `PortUsage = OccurrenceUsagePrefix 'port' Usage` and `BasicUsagePrefix` owns the `ref`, so
/// this is a port usage whose prefix authored `ref`, not a `ReferenceUsage`. It was a `RefDecl`
/// until `port_usage` could spell a `ref` at all; `ref_decl` keeps every other kind it models,
/// exercised by the `ref requirement` cases below.
#[test]
fn interface_def_body_accepts_anonymous_ref_port_with_redefines_type_and_modifiers() {
    let input = "package P {\nport def Port;\ninterface def I {\nref port :>> participant : Port [2..*] nonunique ordered {\n}\n}\n}";
    let result = parse_with_diagnostics(input);
    assert!(
        result.errors.is_empty(),
        "unexpected diagnostics: {:?}",
        result.errors
    );
    let pkg = match &result.document.root.elements[0].value {
        RootElement::Package(p) => &p.value,
        _ => panic!("expected package"),
    };
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace body");
    };
    let interface = elements
        .iter()
        .find_map(|e| match &e.value {
            PackageBodyElement::InterfaceDef(i) => Some(&i.value),
            _ => None,
        })
        .expect("expected interface def");
    let InterfaceDefBody::Brace { elements, .. } = &interface.body else {
        panic!("expected interface def brace body");
    };
    let port = elements
        .iter()
        .find_map(|e| match &e.value {
            InterfaceDefBodyElement::PortUsage(p) => Some(&p.value),
            _ => None,
        })
        .expect("expected port usage");
    assert!(
        port.prefix.basic.reference_span.is_some(),
        "the `ref` belongs to the port usage's own BasicUsagePrefix"
    );
    assert_eq!(port.name, "");
    assert_eq!(
        port.redefines.as_ref().map(|n| n.value.target.len()),
        Some(1)
    );
    assert!(port.typing.is_some());
    assert!(port.multiplicity.is_some());
    assert!(port.multiplicity_modifiers.is_ordered());
    assert!(!port.multiplicity_modifiers.is_unique());
}

/// Real usage: Domain Library `Requirement Derivation/DerivationConnections.sysml`'s `ref
/// requirement originalRequirement[1] :>> originalRequirements :> participant { ... }` and `ref
/// requirement :>> derivedRequirements[1..*] :> participant { ... }` -- `requirement` is just
/// another `ref_decl` kind keyword here (like `part`/`port`/`item`), not the separate
/// `requirement_usage` parser; `ref_decl` previously had no `:>` subsets support at all.
#[test]
fn connection_def_body_accepts_ref_requirement_with_redefines_and_subsets() {
    let input = "package P {\nrequirement def R1;\nrequirement def R2;\nconnection def C {\nref requirement originalRequirement[1] :>> R1 :> participant {\n}\nref requirement :>> R2[1..*] :> participant {\n}\n}\n}";
    let elements = connection_def_elements(&package_elements(input));
    let ref_decls: Vec<_> = elements
        .iter()
        .filter_map(|e| match &e.value {
            ConnectionDefBodyElement::RefDecl(r) => Some(&r.value),
            _ => None,
        })
        .collect();
    assert_eq!(ref_decls.len(), 2);
    let named = ref_decls
        .iter()
        .find(|r| r.name == "originalRequirement")
        .expect("expected named ref requirement");
    assert_eq!(
        named.redefines.as_ref().map(|n| n.value.target.len()),
        Some(1)
    );
    assert_eq!(
        named.subsets.as_ref().map(|n| n.value.target.len()),
        Some(1)
    );
    let anonymous = ref_decls
        .iter()
        .find(|r| r.name.is_empty())
        .expect("expected anonymous ref requirement");
    assert_eq!(
        anonymous.redefines.as_ref().map(|n| n.value.target.len()),
        Some(1)
    );
}
