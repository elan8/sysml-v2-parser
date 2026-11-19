//! gh-156: recovery from a malformed KerML connector body must stop before the binding
//! members that follow it, so both bindings stay in the AST rather than being swallowed into
//! the recovered region.
use sysml_v2_parser::ast::visit::{walk_kerml_binding_member, Visitor};
use sysml_v2_parser::ast::{KermlBindingMember, NameLanguage, Node};
use sysml_v2_parser::parse_for_editor_with_name_language;

const SOURCE: &str = "package Connectors {
    class A {
        feature a : A;
        feature b : A;
        connector c1 from a to b;
        abstract connector c2 = c1;
        connector = c2 {
            end feature references a;
            end feature references b;
        }
        binding a = b;
        binding ab of a = b;
    }
}
";

#[derive(Default)]
struct Bindings {
    found: Vec<(Option<String>, bool, String)>,
}

impl Visitor for Bindings {
    fn visit_kerml_binding_member(&mut self, node: &Node<KermlBindingMember>) {
        let text = &SOURCE[node.span.offset..node.span.offset + node.span.len];
        self.found.push((
            node.value.name.as_ref().map(|name| {
                let span = name.span();
                SOURCE[span.offset..span.offset + span.len].to_owned()
            }),
            node.value.inline_ends.is_some(),
            text.to_owned(),
        ));
        walk_kerml_binding_member(self, node);
    }
}

#[test]
fn both_bindings_after_a_malformed_connector_body_survive_recovery() {
    let result = parse_for_editor_with_name_language(SOURCE, NameLanguage::Kerml);
    assert!(
        !result.errors.is_empty(),
        "the connector members are still diagnosed"
    );
    let mut bindings = Bindings::default();
    bindings.visit_root_namespace(&result.document);
    let texts: Vec<_> = bindings
        .found
        .iter()
        .map(|(_, _, text)| text.as_str())
        .collect();
    assert_eq!(
        texts,
        ["binding a = b;", "binding ab of a = b;"],
        "{:?}",
        bindings.found
    );
    assert!(
        bindings.found.iter().all(|(_, ends, _)| *ends),
        "both keep their `a = b` ends"
    );
    assert!(
        bindings.found[0].0.is_none(),
        "the first binding is anonymous"
    );
    assert_eq!(bindings.found[1].0.as_deref(), Some("ab"));
}
