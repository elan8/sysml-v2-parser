//! A `ref` member is legal in every usage body, and is structured in each of them.
//!
//! `connector::ref_decl` owns the `ref` feature-declaration forms, including the kind-keyword
//! variants (`ref part`, `ref port`, `ref item`, ...). Five body scopes never dispatched it at all
//! -- port definitions, requirement definitions, view definitions, rendering definitions and view
//! usages -- so every `ref` member written in them was captured as unsupported grammar or reported
//! as an unexpected keyword, even though the same member parsed fine one scope over.
//!
//! Each case below is a line from `sysml-v2-release/sysml.library`.

use sysml_v2_parser::{emit_sysml, parse_for_editor};

/// Parse `member` inside `scope`: it must produce no diagnostics and emit as `expected`.
///
/// `expected` is usually `member` verbatim, but the emitter has one canonical clause order (typing
/// before the subsetting family), so a member that writes `:>> self : T` comes back as
/// `: T :>> self`. That reordering is emission policy, not loss -- what matters here is that
/// nothing is dropped.
#[track_caller]
fn assert_structured_as(scope: &str, member: &str, expected: &str) {
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
    let emitted = emit_sysml(&parsed.document)
        .unwrap_or_else(|error| panic!("emit {member:?} in {scope:?}: {error:?}"));
    assert_eq!(
        emitted.lines().nth(2).expect("the member line").trim(),
        expected,
        "in {scope:?}"
    );
}

/// The common case: the member emits exactly as authored.
#[track_caller]
fn assert_structured(scope: &str, member: &str) {
    assert_structured_as(scope, member, member);
}

#[test]
fn a_port_definition_body_accepts_ref_members() {
    assert_structured("port def Port", "ref self : Port :>> Object::self;");
    assert_structured(
        "port def Port",
        "abstract ref port interfacingPorts : Port[0..*] nonunique :> ports;",
    );
    assert_structured(
        "port def Port",
        "ref :>> outgoingTransfersFromSelf :> interfacingPorts.incomingTransfersToSelf;",
    );
}

#[test]
fn a_requirement_definition_body_accepts_ref_members() {
    assert_structured_as(
        "requirement def RequirementCheck",
        "ref requirement :>> self : RequirementCheck;",
        "ref requirement : RequirementCheck :>> self;",
    );
    assert_structured("requirement def R", "ref part actors : Part[0..*];");
}

#[test]
fn a_view_definition_body_accepts_ref_members() {
    assert_structured(
        "view def View",
        "abstract ref view subviews : View[0..*] :> views;",
    );
    assert_structured_as(
        "view def View",
        "ref viewpoint :>> self : ViewpointCheck;",
        "ref viewpoint : ViewpointCheck :>> self;",
    );
}

#[test]
fn a_rendering_definition_body_accepts_ref_members() {
    assert_structured_as(
        "rendering def Rendering",
        "ref rendering :>> self : Rendering;",
        "ref rendering : Rendering :>> self;",
    );
    assert_structured(
        "rendering def Rendering",
        "abstract ref rendering subrenderings : Rendering[0..*] :> renderings;",
    );
}

#[test]
fn a_view_usage_body_accepts_ref_members() {
    // Written at package level rather than nested in a `view def`: a `view` usage inside a view
    // definition body is a separate, still-open gap (`missing_body_or_semicolon`), and this test is
    // about the `ref` member, not about where the view usage may appear.
    let source = concat!(
        "package P {\n",
        "    view columnView[0..*] ordered {\n",
        "        abstract ref rendering :>> viewRendering[0..1];\n",
        "    }\n",
        "}\n",
    );
    let parsed = parse_for_editor(source);
    let codes: Vec<_> = parsed
        .errors
        .iter()
        .map(|error| error.code.clone().unwrap_or_default())
        .collect();
    assert!(codes.is_empty(), "produced {codes:?}");
    let emitted = emit_sysml(&parsed.document).expect("emit");
    // The multiplicity moves to the emitter's canonical position ahead of the redefines clause;
    // it belongs to the declared feature either way.
    assert!(
        emitted.contains("abstract ref rendering[0..1] :>> viewRendering;"),
        "emitted:\n{emitted}"
    );
}

/// The kind keyword is authored syntax, so each one must survive the round trip rather than
/// collapsing to a bare `ref`.
#[test]
fn every_ref_kind_keyword_survives_emission() {
    for kind in [
        "part",
        "port",
        "item",
        "requirement",
        "use case",
        "concern",
        "viewpoint",
        "rendering",
        "view",
        "action",
    ] {
        assert_structured("requirement def R", &format!("ref {kind} x : T;"));
    }
}
