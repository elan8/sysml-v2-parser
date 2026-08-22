//! Members the pinned grammar spells that the parser rejected, one test per upstream gap.
//!
//! Every entry here was verified against the reference implementation
//! (`SysML-v2-Pilot-Implementation`) before being implemented, not against the published BNF
//! alone: the two have diverged before, and the gaps this file closes were reported with
//! spellings that in several neighbouring cases turned out not to be legal at all.

use sysml_v2_parser::parse_with_diagnostics;

#[track_caller]
fn parses_clean(source: &str) {
    let result = parse_with_diagnostics(source);
    assert!(
        result.errors.is_empty(),
        "`{source}` is legal per the reference grammar: {:?}",
        result.errors
    );
    assert!(
        !format!("{:?}", result.document.root).contains("ParseErrorNode"),
        "`{source}` must not reach a recovery node"
    );
}

/// Gap 81. `CalculationBodyItem -> ActionBodyItem` keeps the directed parameter spelling, but a
/// *kinded* parameter is a KerML `Feature` whose kind keyword names its production, not an
/// `InOutDecl`. The directed branch committed to `in_out_decl` with `?`, so the whole member fell
/// to recovery instead of reaching the KerML route the same function already ends in.
#[test]
fn a_kinded_directed_parameter_reaches_the_kerml_route_in_a_calc_body() {
    parses_clean("calc def C { in expr p : Boolean; }");
    parses_clean("calc def C { in bool redefines a; }");
    parses_clean("calc def C { in feature p : Boolean; }");
    // The two spellings the branch exists to keep apart are unaffected.
    parses_clean("calc def C { in p : Boolean; }");
    parses_clean("behavior B { in expr p : Boolean; }");
}

/// Gap 72. `PerformActionUsage = OccurrenceUsagePrefix 'perform' PerformActionUsageDeclaration
/// ActionBody`, whose declaration is `OwnedReferenceSubsetting FeatureSpecializationPart?` *or*
/// `ActionUsageKeyword UsageDeclaration?` (`SysML.xtext:1411-1417`). Only the second alternative
/// reached an action body; the reference form reached only a part body.
#[test]
fn an_action_body_accepts_the_perform_reference_form() {
    parses_clean("package P { action def G { perform L::doIt; } }");
    // The same production, dispatched from the part body, which already accepted it.
    parses_clean("package P { part def H { perform L::doIt; } }");
    parses_clean("package P { action def G { perform action doIt : DoIt; } }");
}

/// Gap 75. `PortDefinition`/`PortUsage` end in `Definition`/`Usage`, and `UsageBody =
/// DefinitionBody` (`SysML.xtext:604`), so both port bodies reach `DefinitionBodyItem ->
/// OccurrenceUsageMember -> StructureUsageMember -> PartUsage` like every other definition body.
#[test]
fn a_port_body_accepts_a_part_member_on_both_sides() {
    parses_clean("package P { port def PD { part x : T; } }");
    parses_clean("package P { port p { part x : T; } }");
}

/// Gap 74. `RequirementConstraintUsage`'s second alternative is `( … ConstraintUsageKeyword | … )
/// ConstraintUsageDeclaration CalculationBody` (`SysML.xtext:2066-2071`), and
/// `ConstraintUsageDeclaration` is an ordinary `UsageDeclaration` -- so the `constraint`-keyword
/// form declares a name *and* may specialize it. Only a bare name was parsed.
#[test]
fn a_declared_require_constraint_carries_its_typing() {
    parses_clean("package P { requirement def R { require constraint c : C; } }");
    parses_clean("package P { requirement def R { assume constraint a : A; } }");
    // The two forms that already worked stay working: the shorthand reference and the bodied
    // declaration.
    parses_clean("package P { requirement def R { require c; } }");
    parses_clean("package P { requirement def R { require constraint c { true } } }");

    let result = parse_with_diagnostics("package P { requirement def R { require constraint c : C; } }");
    assert!(
        format!("{:?}", result.document.root).contains("TypingRelationship"),
        "the typing clause must be retained, not consumed and dropped"
    );
}

/// Gap 76. `PayloadParameter = Payload | Identification? PayloadFeatureSpecializationPart?
/// TriggerValuePart` (`SysML.xtext:1459-1461`), and `TriggerValuePart` is the trigger expression
/// whose kinds are `'at' | 'after'` and `'when'` (1479-1484). An accept *node* reaches it exactly
/// as a transition does; the parser treated triggers as transition-only.
#[test]
fn an_accept_node_accepts_a_trigger() {
    parses_clean("package P { action def Q { accept when true; } }");
    parses_clean("package P { action def Q { accept at now; } }");
    parses_clean("package P { action def Q { accept after now; } }");
    // The payload spellings that already worked are unaffected.
    parses_clean("package P { action def Q { accept sig; } }");
    parses_clean("package P { action def Q { accept sig : Sig via port1; } }");
}

/// Gap 76, the `if`/`else` half. `IfNode = ActionNodePrefix 'if' ExpressionParameterMember
/// ActionBodyParameterMember ( 'else' … )?` with `ActionBodyParameter` always braced
/// (`SysML.xtext:1596-1608`) -- there is no `then` in the production at all. The braced spellings
/// the reference does define already parse, including `else if` chains.
#[test]
fn the_reference_spelling_of_if_else_parses() {
    parses_clean("package P { action def Q { if true { action a1; } else { action a2; } } }");
    parses_clean(
        "package P { action def Q { if true { action a1; } else if false { action a2; } } }",
    );
}
