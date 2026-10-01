//! Every production whose grammar includes `Identification` accepts a short name.
//!
//! SysML v2 textual BNF: `Identification = ( '<' declaredShortName = NAME '>' )?
//! ( declaredName = NAME )?`. It is reached through `DefinitionDeclaration`, `UsageDeclaration`
//! (and so every usage kind), `ConnectorDeclaration`, `TransitionUsage`, `SuccessionAsUsage`,
//! `AliasMember`, `Comment`, `Documentation`, `Dependency` and the package declarations. A short
//! name is therefore legal everywhere a declared name is, and on anonymous declarations too.
//!
//! Each case below is checked three ways:
//! 1. the same source *without* the short name parses cleanly (so a failure is attributable to
//!    the short name, not to the surrounding context);
//! 2. with the short name it parses with no diagnostics and the AST retains it as a
//!    `DeclarationName` (found through the structural visitor, so no per-node accessor is
//!    needed and a new production is covered by adding one line here);
//! 3. the emitter writes it back (where the emitter supports the construct at all).

use sysml_v2_parser::ast::visit::{walk_root_namespace, Visitor};
use sysml_v2_parser::ast::DeclarationName;
use sysml_v2_parser::{emit_sysml, parse_with_diagnostics};

const PRELUDE: &str = "
    part def T;
    port def PD;
    item def ID;
    attribute def AD;
    enum def ED { e1; }
    connection def CD { end a : T; end b : T; }
    interface def IFD { end p1 : PD; end p2 : PD; }
    allocation def ALD;
    flow def FD;
    action def ACD;
    calc def CLD;
    state def SD;
    constraint def CND;
    requirement def RD;
    concern def CCD;
    case def CSD;
    analysis def ANLD;
    verification def VD;
    use case def UCD;
    view def VWD;
    viewpoint def VPD;
    rendering def RND;
    metadata def MD;
    occurrence def OD;
";

/// Where a case's snippet is placed.
#[derive(Clone, Copy)]
enum Context {
    /// Directly in the package body.
    Package,
    /// In a part definition body that owns parts `a`, `b` and actions `s1`, `s2`.
    PartDef,
    /// In a state definition body that owns states `s1`, `s2`.
    StateDef,
    /// In a use case definition body.
    UseCaseDef,
    /// In a requirement definition body.
    RequirementDef,
    /// In a connection definition body.
    ConnectionDef,
    /// In an action definition body.
    ActionDef,
    /// In a calculation definition body.
    CalcDef,
}

fn wrap(context: Context, snippet: &str) -> String {
    let member = match context {
        Context::Package => snippet.to_owned(),
        Context::PartDef => {
            format!("part def Owner {{ part a : T; part b : T; action s1; action s2; {snippet} }}")
        }
        Context::StateDef => format!("state def Owner {{ state s1; state s2; {snippet} }}"),
        Context::UseCaseDef => format!("use case def Owner {{ {snippet} }}"),
        Context::RequirementDef => format!("requirement def Owner {{ {snippet} }}"),
        Context::ConnectionDef => format!("connection def Owner {{ {snippet} }}"),
        Context::ActionDef => format!("action def Owner {{ {snippet} }}"),
        Context::CalcDef => format!("calc def Owner {{ {snippet} }}"),
    };
    format!("package P {{ {PRELUDE} {member} }}\n")
}

/// `(context, snippet)`. `<'SN'>` marks the short name; the control strips it.
const CASES: &[(Context, &str)] = &[
    // Package-level declarations (`DefinitionDeclaration`, packages, alias, comment, dependency).
    (Context::Package, "package <'SN'> Inner;"),
    (Context::Package, "part def <'SN'> PartDef;"),
    (Context::Package, "attribute def <'SN'> AttrDef;"),
    (Context::Package, "enum def <'SN'> EnumDef { x; }"),
    (Context::Package, "item def <'SN'> ItemDef;"),
    (Context::Package, "port def <'SN'> PortDef;"),
    (Context::Package, "connection def <'SN'> ConnDef;"),
    (Context::Package, "interface def <'SN'> IfDef;"),
    (Context::Package, "allocation def <'SN'> AllocDef;"),
    (Context::Package, "flow def <'SN'> FlowDef;"),
    (Context::Package, "action def <'SN'> ActDef;"),
    (Context::Package, "calc def <'SN'> CalcDef;"),
    (Context::Package, "state def <'SN'> StateDef;"),
    (Context::Package, "constraint def <'SN'> ConsDef;"),
    (Context::Package, "requirement def <'SN'> ReqDef;"),
    (Context::Package, "concern def <'SN'> ConcDef;"),
    (Context::Package, "case def <'SN'> CaseDef;"),
    (Context::Package, "analysis def <'SN'> AnaDef;"),
    (Context::Package, "verification def <'SN'> VerDef;"),
    (Context::Package, "use case def <'SN'> UcDef;"),
    (Context::Package, "view def <'SN'> ViewDef;"),
    (Context::Package, "viewpoint def <'SN'> VpDef;"),
    (Context::Package, "rendering def <'SN'> RenDef;"),
    (Context::Package, "metadata def <'SN'> MetaDef;"),
    (Context::Package, "occurrence def <'SN'> OccDef;"),
    (Context::Package, "alias <'SN'> Al for T;"),
    (Context::Package, "comment <'SN'> c /* text */"),
    (Context::Package, "dependency <'SN'> dep from T to PD;"),
    // Usages at package level (`UsageDeclaration`).
    (Context::Package, "part <'SN'> p : T;"),
    (Context::Package, "action <'SN'> act : ACD;"),
    (Context::Package, "requirement <'SN'> req : RD;"),
    (Context::Package, "verification <'SN'> ver : VD;"),
    (Context::Package, "analysis <'SN'> ana : ANLD;"),
    (Context::Package, "case <'SN'> cs : CSD;"),
    (Context::Package, "use case <'SN'> uc : UCD;"),
    (Context::Package, "state <'SN'> st : SD;"),
    (Context::Package, "concern <'SN'> conc : CCD;"),
    (Context::Package, "view <'SN'> vw : VWD;"),
    (Context::Package, "viewpoint <'SN'> vp : VPD;"),
    (Context::Package, "rendering <'SN'> rn : RND;"),
    (Context::Package, "enum <'SN'> en : ED;"),
    (Context::Package, "allocation <'SN'> al : ALD;"),
    (Context::Package, "metadata <'SN'> md : MD;"),
    (Context::Package, "occurrence <'SN'> oc : OD;"),
    (Context::Package, "calc <'SN'> cl : CLD;"),
    (Context::Package, "constraint <'SN'> cn : CND;"),
    // Usages in a definition body.
    (Context::PartDef, "attribute <'SN'> at : AD;"),
    (Context::PartDef, "enum <'SN'> en : ED;"),
    (Context::PartDef, "item <'SN'> it : ID;"),
    (Context::PartDef, "part <'SN'> pt : T;"),
    (Context::PartDef, "port <'SN'> po : PD;"),
    (Context::PartDef, "ref <'SN'> rf : T;"),
    (
        Context::PartDef,
        "connection <'SN'> cn : CD connect a to b;",
    ),
    (Context::PartDef, "interface <'SN'> ifc : IFD;"),
    (Context::PartDef, "allocation <'SN'> al : ALD;"),
    (Context::PartDef, "flow <'SN'> fl : FD;"),
    (Context::PartDef, "action <'SN'> ac : ACD;"),
    (Context::PartDef, "perform action <'SN'> pa : ACD;"),
    (Context::PartDef, "calc <'SN'> cl : CLD;"),
    (Context::PartDef, "state <'SN'> st : SD;"),
    (Context::PartDef, "exhibit state <'SN'> ex : SD;"),
    (Context::PartDef, "constraint <'SN'> cs : CND;"),
    (Context::PartDef, "assert constraint <'SN'> ac2 : CND;"),
    (Context::PartDef, "requirement <'SN'> rq : RD;"),
    (Context::PartDef, "case <'SN'> ca : CSD;"),
    (Context::PartDef, "analysis <'SN'> an : ANLD;"),
    (Context::PartDef, "verification <'SN'> ve : VD;"),
    (Context::PartDef, "use case <'SN'> uc : UCD;"),
    (Context::PartDef, "view <'SN'> vw : VWD;"),
    (Context::PartDef, "viewpoint <'SN'> vp : VPD;"),
    (Context::PartDef, "rendering <'SN'> rn : RND;"),
    (Context::PartDef, "occurrence <'SN'> oc : OD;"),
    (Context::PartDef, "event occurrence <'SN'> ev : OD;"),
    (Context::PartDef, "snapshot <'SN'> sn : T;"),
    (Context::PartDef, "timeslice <'SN'> ts : T;"),
    (Context::PartDef, "metadata <'SN'> md : MD;"),
    (Context::PartDef, "succession <'SN'> sc first s1 then s2;"),
    (Context::PartDef, "binding <'SN'> bd bind a = b;"),
    (Context::PartDef, "part <'SN'> : T;"),
    (Context::PartDef, "doc <'SN'> /* text */"),
    (Context::PartDef, "comment <'SN'> cm /* text */"),
    (Context::PartDef, "flow <'SN'> fl2 from a to b;"),
    (Context::PartDef, "message <'SN'> ms of ID from a to b;"),
    (Context::PartDef, "satisfy requirement <'SN'> sr : RD by a;"),
    (
        Context::PartDef,
        "variation part <'SN'> vp : T { variant part <'SN2'> v1 : T; }",
    ),
    (Context::RequirementDef, "frame concern <'SN'> fc : CCD;"),
    (Context::CalcDef, "return <'SN'> r : AD;"),
    (Context::ActionDef, "out <'SN'> y : AD;"),
    (Context::StateDef, "transition <'SN'> tr first s1 then s2;"),
    (Context::UseCaseDef, "include use case <'SN'> inc : UCD;"),
    (Context::UseCaseDef, "subject <'SN'> sb : T;"),
    (Context::UseCaseDef, "actor <'SN'> ar : T;"),
    (Context::UseCaseDef, "objective <'SN'> ob;"),
    (Context::RequirementDef, "subject <'SN'> sb : T;"),
    (Context::RequirementDef, "stakeholder <'SN'> sh : T;"),
    (Context::ConnectionDef, "end <'SN'> e1 : T;"),
    (Context::ActionDef, "in <'SN'> x : AD;"),
];

#[derive(Default)]
struct Names(Vec<DeclarationName>);

impl Visitor for Names {
    fn visit_declaration_name(&mut self, name: &DeclarationName) {
        self.0.push(*name);
    }
}

/// Why a case fails, or `None` when it passes.
fn check(context: Context, snippet: &str) -> Option<String> {
    let control = wrap(context, &snippet.replace("<'SN'> ", ""));
    let control_result = parse_with_diagnostics(&control);
    if !control_result.errors.is_empty() {
        return Some(format!(
            "control (no short name) does not parse: {:?}",
            control_result.errors
        ));
    }

    let source = wrap(context, snippet);
    let result = parse_with_diagnostics(&source);
    if !result.errors.is_empty() {
        return Some(format!("diagnostics: {:?}", result.errors));
    }
    let mut names = Names::default();
    walk_root_namespace(&mut names, &result.document.root);
    if !names
        .0
        .iter()
        .any(|name| result.document.declaration_name(*name) == Some("'SN'"))
    {
        return Some("the short name is not retained in the AST".to_owned());
    }
    // Some body scopes have no emitter for a construct at all, with or without a short name.
    // That is an emitter coverage gap, not a short-name one, so the round trip is only checked
    // where the control emits.
    if emit_sysml(&control_result.document).is_err() {
        return None;
    }
    match emit_sysml(&result.document) {
        Ok(emitted) if emitted.contains("<'SN'>") => None,
        Ok(emitted) => Some(format!("the emitter drops the short name:\n{emitted}")),
        Err(error) => Some(format!("emit failed: {error:?}")),
    }
}

#[test]
fn every_identification_production_accepts_a_short_name() {
    let failures = CASES
        .iter()
        .filter_map(|(context, snippet)| {
            check(*context, snippet).map(|reason| format!("`{snippet}`: {reason}"))
        })
        .collect::<Vec<_>>();
    assert!(
        failures.is_empty(),
        "{} of {} short-name cases fail:\n{}",
        failures.len(),
        CASES.len(),
        failures.join("\n")
    );
}
