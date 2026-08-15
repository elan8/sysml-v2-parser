//! Occurrence-style definition bodies structure the members their grammar allows.
//!
//! `DefinitionBody` -- the body shared by `flow def`, `occurrence def`, `allocation def` and the
//! rest -- tried its opaque unsupported-member capture *before* the structured dispatch, the
//! reverse of every other body in the parser. Any member whose first token was one of the capture's
//! starter keywords (`private`, `ref`, `abstract`, `in`, `connection`) was therefore swallowed
//! whole, even when a structured parser directly below would have handled it: `private attribute
//! seBeforeNum : Natural[1] = ...;` (`sysml.library/Systems Library/Flows.sysml`) is an ordinary
//! attribute usage, and `attribute_usage` has parsed a visibility prefix all along.
//!
//! Separately, these bodies dispatched no `ref` member at all.

use sysml_v2_parser::{emit_sysml, parse_for_editor};

/// The member parses with no diagnostics.
#[track_caller]
fn assert_parses(scope: &str, member: &str) {
    let source = format!("package P {{\n    {scope} {{\n        {member}\n    }}\n}}\n");
    let parsed = parse_for_editor(&source);
    let codes: Vec<_> = parsed
        .errors
        .iter()
        .map(|error| error.code.clone().unwrap_or_default())
        .collect();
    assert!(
        codes.is_empty(),
        "{member:?} in {scope:?} produced {codes:?}"
    );
}

/// The member parses with no diagnostics and comes back out as written.
///
/// Only used with `occurrence def`: `FlowDef` has no emitter at all
/// (`emit::root` refuses it as an unsupported construct), so a flow definition cannot round-trip
/// yet regardless of what its body contains. That is a separate, declared gap -- the emitter
/// refuses rather than dropping the member.
#[track_caller]
fn assert_structured(scope: &str, member: &str) {
    assert_parses(scope, member);
    let source = format!("package P {{\n    {scope} {{\n        {member}\n    }}\n}}\n");
    let parsed = parse_for_editor(&source);
    let emitted = emit_sysml(&parsed.document)
        .unwrap_or_else(|error| panic!("emit {member:?} in {scope:?}: {error:?}"));
    assert_eq!(
        emitted.lines().nth(2).expect("the member line").trim(),
        member,
        "in {scope:?}"
    );
}

/// The keyword after the visibility prefix decides the member, so a prefixed member must reach the
/// same parser its unprefixed form reaches.
#[test]
fn a_visibility_prefix_does_not_hide_the_member_behind_it() {
    assert_parses("flow def F", "private attribute seBeforeNum : Natural[1];");
    assert_structured(
        "occurrence def O",
        "private attribute seBeforeNum : Natural[1];",
    );
    assert_structured("occurrence def O", "private part p : T;");
    assert_structured("occurrence def O", "protected attribute x : T;");
}

#[test]
fn an_occurrence_body_accepts_ref_members() {
    assert_parses("flow def F", "ref self : SuccessionFlow :>> Flow::self;");
    assert_parses("flow def F", "private ref action thisConnection = self;");
    assert_structured("occurrence def O", "ref x : T;");
    assert_structured(
        "occurrence def O",
        "private ref action thisConnection = self;",
    );
}

/// The opaque capture is still reachable -- it is the fallback, not the first choice. A
/// connection usage that redefines rather than types its connector (`connection :>> c connect a to
/// b;`) is a form the parser does not model yet, and it must still be retained with a diagnostic
/// rather than parsed into something wrong.
///
/// This test used `connection :HappensDuring connect a to b;` until occurrence bodies learned to
/// dispatch connection usages; that member is now structured, which is why the example moved.
#[test]
fn an_unmodelled_member_is_still_captured_with_a_diagnostic() {
    let source =
        "package P {\n    flow def F {\n        connection :>> c connect a to b;\n    }\n}\n";
    let parsed = parse_for_editor(source);
    let codes: Vec<_> = parsed
        .errors
        .iter()
        .map(|error| error.code.clone().unwrap_or_default())
        .collect();
    assert_eq!(codes, vec!["unsupported_grammar_form"]);
}
