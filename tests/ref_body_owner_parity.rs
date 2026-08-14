//! A `ref` body holds the same members whichever declaration owns it.
//!
//! `ReferenceUsage` completes with a `UsageBody`, and `UsageBody = DefinitionBody`
//! (SysML 8.2.2.6.2), so the owner is irrelevant to what a `ref` body may contain. This is the
//! invariant behind having one `ref` body parser instead of one per owner, and it is checked here
//! against the semantic projection -- which shows *which* members parsed, in what order, with what
//! structure -- rather than only against emitted text, where two different member sets can format
//! to the same characters.

use sysml_v2_parser::ast::WriteSemanticAst;
use sysml_v2_parser::parse_for_editor;

const OWNERS: [(&str, &str); 4] = [
    ("connection", "connection def C"),
    ("part", "part def P"),
    ("action", "action def A"),
    ("state", "state def S"),
];

/// Every member the usage-body grammar offers that a `ref` body plausibly carries, including a
/// nested `ref` so the recursion is covered too.
const MEMBERS: &str = concat!(
    "            doc /* shared */\n",
    "            comment /* shared */\n",
    "            rep shared language \"text\" /* shared */\n",
    "            @Meta about x;\n",
    "            attribute mass : Real;\n",
    "            ref nested;\n",
);

fn semantic_ast(source: &str) -> String {
    let document = parse_for_editor(source);
    assert!(
        document.errors.is_empty(),
        "fixture should parse cleanly, got {:?}\n{source}",
        document.errors
    );
    let mut rendered = Vec::new();
    document
        .document
        .write_semantic_ast(&mut rendered)
        .expect("semantic projection");
    String::from_utf8(rendered).expect("semantic projection is UTF-8")
}

/// The projection of the `ref` declaration itself, with reference labels normalized: the labels
/// are assigned in document order, so they differ between fixtures without the structure differing.
fn projected_ref(owner_declaration: &str, ref_name: &str) -> String {
    let source =
        format!("package P {{\n    {owner_declaration} {{\n        ref {ref_name} : Anything {{\n{MEMBERS}        }}\n    }}\n}}\n");
    let ast = semantic_ast(&source);
    let start = ast
        .find(&format!("(ref (name \"{ref_name}\")"))
        .unwrap_or_else(|| panic!("no ref projection for {owner_declaration} in:\n{ast}"));
    let projection = &ast[start..];
    let normalized = projection.replace(&format!("\"{ref_name}\""), "\"r\"");
    let mut labels = 0;
    let mut result = String::new();
    let mut rest = normalized.as_str();
    while let Some(index) = rest.find("(ref r") {
        result.push_str(&rest[..index]);
        result.push_str("(ref rN");
        labels += 1;
        rest = &rest[index + "(ref r".len()..];
        rest = rest.trim_start_matches(|character: char| character.is_ascii_digit());
    }
    result.push_str(rest);
    assert!(labels > 0, "expected reference labels in {result}");
    result
}

#[test]
fn every_owner_projects_the_same_ref_body() {
    let mut projections = OWNERS
        .iter()
        .map(|(label, declaration)| (*label, projected_ref(declaration, "under")));
    let (first_label, first) = projections.next().expect("at least one owner");
    for (label, projection) in projections {
        assert_eq!(
            projection, first,
            "a `ref` body under `{label}` holds different members than under `{first_label}`"
        );
    }

    // Guard against the assertion passing because the projection is a bare marker: it has to show
    // the members, or it cannot detect a difference in them.
    assert!(
        first.contains("(doc)")
            && first.contains("(comment)")
            && first.contains("(textual-rep)")
            && first.contains("(attribute-usage")
            && first.contains("(ref (name \"nested\")"),
        "the ref projection must show its members, got: {first}"
    );
}

#[test]
fn a_semicolon_ref_body_projects_as_such_under_every_owner() {
    for (label, declaration) in OWNERS {
        let source = format!(
            "package P {{\n    {declaration} {{\n        ref plain : Anything;\n    }}\n}}\n"
        );
        let ast = semantic_ast(&source);
        assert!(
            ast.contains("(ref (name \"plain\")") && ast.contains("(body semicolon)"),
            "`{label}` did not project a semicolon ref body: {ast}"
        );
    }
}
