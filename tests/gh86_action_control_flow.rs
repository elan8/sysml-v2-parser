//! GH-86: action-body control-flow gaps triaged from #83's `examples/` roundtrip scan.
//! Each test below uses the exact (trimmed) real source that motivated the fix.

use sysml_v2_parser::ast::{
    ActionBranchBody, ActionDefBody, ActionDefBodyElement, ActionUsageBody, ActionUsageBodyElement,
    PackageBody, PackageBodyElement, RootElement, ThenTarget,
};
use sysml_v2_parser::parse_with_diagnostics;

fn package_elements(input: &str) -> Vec<PackageBodyElement> {
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
    let PackageBody::Brace { elements, .. } = &pkg.body else {
        panic!("expected brace package body");
    };
    elements.iter().map(|e| e.value.clone()).collect()
}

/// Real usage: `Analysis Examples/Dynamics.sysml:64`:
/// ```text
/// out attribute :>> a_out : AccelerationValue = Acceleration(dt, tm, tp);
/// ```
/// Previously: the `in_out_decl` `:>>` redefinition branch had no path for a trailing `: Type`
/// clause between the redefinition target and the `= value`, so `type_name` was always left
/// empty for this form.
#[test]
fn gh86_6_in_out_decl_redefinition_accepts_trailing_type_clause() {
    let elements = package_elements(
        r#"package P {
            action def AD {
                action a1 : AD {
                    attribute tp : Real = 1.0;
                    out attribute :>> a_out : AccelerationValue = tp;
                }
            }
        }"#,
    );
    let PackageBodyElement::ActionDef(action_def) = &elements[0] else {
        panic!("expected ActionDef, got {:?}", elements[0]);
    };
    let ActionDefBody::Brace { elements, .. } = &action_def.value.body else {
        panic!("expected brace action def body");
    };
    let ActionDefBodyElement::ActionUsage(action_usage) = &elements[0].value else {
        panic!("expected nested ActionUsage, got {:?}", elements[0]);
    };
    let ActionUsageBody::Brace { elements, .. } =
        action_usage.value.body.as_ref().expect("an authored body")
    else {
        panic!("expected brace action usage body");
    };
    let in_out_decl = elements.iter().find_map(|e| match &e.value {
        ActionUsageBodyElement::InOutDecl(decl) => Some(&decl.value),
        _ => None,
    });
    let in_out_decl = in_out_decl.expect("expected an InOutDecl element");
    assert!(
        in_out_decl.redefines.is_some(),
        "expected structured redefinition target"
    );
    assert!(
        in_out_decl.name.is_empty(),
        "a redefinition target is not a declaration name"
    );
    assert!(
        in_out_decl.type_name.is_some(),
        "expected trailing type clause to be captured, not left empty"
    );
    assert!(
        in_out_decl.value.is_some(),
        "expected the trailing = value to still parse"
    );
}

/// Real usage: `Analysis Examples/AnalysisAnnotation.sysml:6-9`:
/// ```text
/// action def ComputeDynamics {
///     metadata ToolExecution {
///         toolName = "ModelCenter";
///     }
/// }
/// ```
/// Previously: the literal `metadata` keyword form (BNF `MetadataUsage`'s `('@' | 'metadata')`
/// alternative) was only dispatched at package-body scope, even though
/// `crate::parser::metadata::metadata_usage` already fully implemented it. Neither
/// `metadata_annotation` (requires `@`) nor `metadata_keyword_usage` (requires `#`) covers this
/// form, so it fell through to a parse error inside action bodies.
#[test]
fn gh86_4_bare_metadata_keyword_dispatched_inside_action_def_body() {
    let elements = package_elements(
        r#"package P {
            action def ComputeDynamics {
                metadata ToolExecution {
                    toolName = "ModelCenter";
                }
            }
        }"#,
    );
    let PackageBodyElement::ActionDef(action_def) = &elements[0] else {
        panic!("expected ActionDef, got {:?}", elements[0]);
    };
    let ActionDefBody::Brace { elements, .. } = &action_def.value.body else {
        panic!("expected brace action def body");
    };
    let metadata_usage = elements.iter().find_map(|e| match &e.value {
        ActionDefBodyElement::MetadataUsage(m) => Some(&m.value),
        _ => None,
    });
    let metadata_usage = metadata_usage.expect("expected a MetadataUsage element");
    assert_eq!(metadata_usage.name, "ToolExecution");
}

/// Real usage: `Simple Tests/TextualRepresentationTest.sysml:14-18`:
/// ```text
/// action def setX {
///     language "alf"
///         /* c.x = newX; */
/// }
/// ```
/// Previously two bugs stacked: (1) `textual_representation` parsed `rep` as a mandatory prefix
/// even though the BNF makes `('rep' Identification)?` fully optional, so a bare `language
/// "alf" ...` failed even where the function *was* dispatched; and (2) it wasn't dispatched from
/// any action body at all. Both fixed together for GH-86.
#[test]
fn gh86_5_bare_language_textual_representation_dispatched_inside_action_def_body() {
    let elements = package_elements(
        r#"package P {
            item def C { attribute x : Real; }
            action def setX {
                in c : C;
                in newX : Real;
                language "alf"
                    /* c.x = newX; */
            }
        }"#,
    );
    let PackageBodyElement::ActionDef(action_def) = &elements[1] else {
        panic!("expected ActionDef, got {:?}", elements[1]);
    };
    let ActionDefBody::Brace { elements, .. } = &action_def.value.body else {
        panic!("expected brace action def body");
    };
    let rep = elements.iter().find_map(|e| match &e.value {
        ActionDefBodyElement::TextualRep(r) => Some(&r.value),
        _ => None,
    });
    let rep = rep.expect("expected a TextualRep element");
    assert!(rep.rep_identification.is_none());
    assert_eq!(rep.language, "alf");
    assert!(rep.text.contains("c.x = newX;"));
}

/// Real usage: `Simple Tests/ActionTest.sysml:17`:
/// ```text
/// attribute def S;
/// action a1 {
///     first start;
///     then merge m;
///     then accept S;
///     ...
/// }
/// ```
/// Previously: `then_action`'s target list handled `merge`/`perform`/`action_usage`/feature-path
/// but had no path for a bare `accept <expr>` control-node reference, even though the exact same
/// shape (`TransitionAccept::Shorthand`) already fully parses after a state `transition`.
#[test]
fn gh86_2_then_accept_shorthand_target() {
    let elements = package_elements(
        r#"package P {
            attribute def S;
            action a1 {
                first start;
                then merge m;
                then accept S;
                merge m;
            }
        }"#,
    );
    let PackageBodyElement::ActionUsage(action_usage) = &elements[1] else {
        panic!("expected ActionUsage, got {:?}", elements[1]);
    };
    let ActionUsageBody::Brace { elements, .. } =
        action_usage.value.body.as_ref().expect("an authored body")
    else {
        panic!("expected brace action usage body");
    };
    let then_accept = elements.iter().find_map(|e| match &e.value {
        ActionUsageBodyElement::ThenAction(t) => match &t.value.target {
            ThenTarget::Accept(a) => Some(&a.value),
            _ => None,
        },
        _ => None,
    });
    let accept = then_accept.expect("expected a `then accept ...;` element");
    match accept {
        sysml_v2_parser::ast::TransitionAccept::Shorthand(expr, via) => {
            assert!(matches!(
                &expr.value,
                sysml_v2_parser::ast::Expression::FeatureRef(_)
            ));
            assert!(via.is_none());
        }
        other => panic!("expected Shorthand(S, None), got {other:?}"),
    }
}

/// Real usage: `Simple Tests/ControlNodeTest.sysml:12-16`:
/// ```text
/// join J;
/// then fork F {
///     in a;
///     out b1;
///     out b2;
/// }
/// then B1;
/// ```
/// Previously: `then_action`'s target list had no path for a bare `fork <name> { ... }`
/// control-node reference, even though `fork_stmt` already fully parses it standalone.
#[test]
fn gh86_2_then_fork_target() {
    let elements = package_elements(
        r#"package P {
            action def ControlNodeTest {
                action A1;
                then J;
                join J;
                then fork F {
                    in a;
                    out b1;
                    out b2;
                }
                then B1;
                action B1;
            }
        }"#,
    );
    let PackageBodyElement::ActionDef(action_def) = &elements[0] else {
        panic!("expected ActionDef, got {:?}", elements[0]);
    };
    let ActionDefBody::Brace { elements, .. } = &action_def.value.body else {
        panic!("expected brace action def body");
    };
    let then_fork = elements.iter().find_map(|e| match &e.value {
        ActionDefBodyElement::ThenAction(t) => match &t.value.target {
            ThenTarget::Fork(f) => Some(&f.value),
            _ => None,
        },
        _ => None,
    });
    let fork = then_fork.expect("expected a `then fork ...;` element");
    assert!(matches!(
        &fork.fork.value,
        sysml_v2_parser::ast::Expression::FeatureRef(_)
    ));
}

/// Real usage: `Simple Tests/DecisionTest.sysml:5-7`:
/// ```text
/// if x == 1 then A1;
/// if x > 1 then A2;
/// else A3;
/// ```
/// Previously: `if_stmt` only accepted a braced `{ ... }` then/else body. The non-brace
/// `then`/`else` shorthand is real usage (a guarded succession, BNF `GuardExpressionMember` +
/// `TransitionSuccessionMember`), not a parser convenience.
#[test]
fn gh86_3_if_then_non_brace_shorthand_without_else() {
    let elements = package_elements(
        r#"package P {
            action def AD {
                attribute x = 1;
                action A1;
                if x == 1 then A1;
            }
        }"#,
    );
    let PackageBodyElement::ActionDef(action_def) = &elements[0] else {
        panic!("expected ActionDef, got {:?}", elements[0]);
    };
    let ActionDefBody::Brace { elements, .. } = &action_def.value.body else {
        panic!("expected brace action def body");
    };
    let if_stmt = elements.iter().find_map(|e| match &e.value {
        ActionDefBodyElement::IfStmt(i) => Some(&i.value),
        _ => None,
    });
    let if_stmt = if_stmt.expect("expected an IfStmt element");
    // The brace-less spelling keeps its own state: it is a single member, not a body with
    // delimiters the author never wrote.
    let ActionBranchBody::Shorthand(then_member) = &if_stmt.then_body else {
        panic!("expected a brace-less then branch");
    };
    assert!(matches!(
        &then_member.value,
        ActionDefBodyElement::ThenAction(_)
    ));
    assert!(if_stmt.else_body.is_none());
}

/// Same fixture, second/third lines: `if x > 1 then A2; else A3;` -- non-brace then *and* else.
#[test]
fn gh86_3_if_then_else_non_brace_shorthand() {
    let elements = package_elements(
        r#"package P {
            action def AD {
                attribute x = 1;
                action A2;
                action A3;
                if x > 1 then A2;
                else A3;
            }
        }"#,
    );
    let PackageBodyElement::ActionDef(action_def) = &elements[0] else {
        panic!("expected ActionDef, got {:?}", elements[0]);
    };
    let ActionDefBody::Brace { elements, .. } = &action_def.value.body else {
        panic!("expected brace action def body");
    };
    let if_stmt = elements.iter().find_map(|e| match &e.value {
        ActionDefBodyElement::IfStmt(i) => Some(&i.value),
        _ => None,
    });
    let if_stmt = if_stmt.expect("expected an IfStmt element");
    let ActionBranchBody::Shorthand(else_member) =
        if_stmt.else_body.as_ref().expect("expected an else_body")
    else {
        panic!("expected a brace-less else branch");
    };
    assert!(matches!(
        &else_member.value,
        ActionDefBodyElement::ThenAction(_)
    ));
}

/// Real usage: `Simple Tests/DecisionTest.sysml:9`: `then decide D;` -- a bare `decide`
/// control-node reference as a `then` target, same class of gap as `then accept`/`then fork`.
#[test]
fn gh86_3_then_decide_target() {
    let elements = package_elements(
        r#"package P {
            attribute def S;
            action a1 {
                first start;
                then decide D;
                decide D;
            }
        }"#,
    );
    let PackageBodyElement::ActionUsage(action_usage) = &elements[1] else {
        panic!("expected ActionUsage, got {:?}", elements[1]);
    };
    let ActionUsageBody::Brace { elements, .. } =
        action_usage.value.body.as_ref().expect("an authored body")
    else {
        panic!("expected brace action usage body");
    };
    let then_decide = elements.iter().find_map(|e| match &e.value {
        ActionUsageBodyElement::ThenAction(t) => match &t.value.target {
            ThenTarget::Decide(d) => Some(&d.value),
            _ => None,
        },
        _ => None,
    });
    let decide = then_decide.expect("expected a `then decide ...;` element");
    assert!(matches!(
        &decide.decide.value,
        sysml_v2_parser::ast::Expression::FeatureRef(_)
    ));
}

/// Real usage: `Simple Tests/StructuredControlTest.sysml:7-13`:
/// ```text
/// if i < 0 {
///     assign i := 0;
/// } else if i == 0 {
///     assign i := 1;
/// } else {
///     assign i := i + 1;
/// }
/// ```
/// Previously: the else branch only accepted a body, not a nested `if` (BNF `IfNode`'s
/// `('else' (ActionBodyParameterMember | IfNodeParameterMember))?` -- the else alternative can
/// itself be another `IfNode`).
#[test]
fn gh86_3_else_if_chaining() {
    let elements = package_elements(
        r#"package P {
            action def AD {
                attribute i = 0;
                if i < 0 {
                    assign i := 0;
                } else if i == 0 {
                    assign i := 1;
                } else {
                    assign i := i + 1;
                }
            }
        }"#,
    );
    let PackageBodyElement::ActionDef(action_def) = &elements[0] else {
        panic!("expected ActionDef, got {:?}", elements[0]);
    };
    let ActionDefBody::Brace { elements, .. } = &action_def.value.body else {
        panic!("expected brace action def body");
    };
    let outer_if = elements
        .iter()
        .find_map(|e| match &e.value {
            ActionDefBodyElement::IfStmt(i) => Some(&i.value),
            _ => None,
        })
        .expect("expected outer IfStmt element");
    // `else if ...` is written without braces, so the branch is the nested `if` itself.
    let ActionBranchBody::Shorthand(outer_else_member) = outer_if
        .else_body
        .as_ref()
        .expect("expected outer else_body")
    else {
        panic!("expected a brace-less else branch wrapping the nested if");
    };
    let inner_if = match &outer_else_member.value {
        ActionDefBodyElement::IfStmt(i) => &i.value,
        other => panic!("expected nested IfStmt in else_body, got {other:?}"),
    };
    assert!(
        inner_if.else_body.is_some(),
        "expected the innermost else clause to still be present"
    );
}

/// Same gap, but for the `action <name> : Type { ... }` usage body form (`ActionUsageBodyElement`)
/// rather than `action def`.
#[test]
fn gh86_4_bare_metadata_keyword_dispatched_inside_action_usage_body() {
    let elements = package_elements(
        r#"package P {
            action def AD;
            action a1 : AD {
                metadata ToolExecution {
                    toolName = "ModelCenter";
                }
            }
        }"#,
    );
    let PackageBodyElement::ActionUsage(action_usage) = &elements[1] else {
        panic!("expected ActionUsage, got {:?}", elements[1]);
    };
    let ActionUsageBody::Brace { elements, .. } =
        action_usage.value.body.as_ref().expect("an authored body")
    else {
        panic!("expected brace action usage body");
    };
    let metadata_usage = elements.iter().find_map(|e| match &e.value {
        ActionUsageBodyElement::MetadataUsage(m) => Some(&m.value),
        _ => None,
    });
    let metadata_usage = metadata_usage.expect("expected a MetadataUsage element");
    assert_eq!(metadata_usage.name, "ToolExecution");
}

/// Real usage: `Interaction Sequencing Examples/ServerSequenceOutsideRealization-2.sysml`:
/// ```text
/// perform action producerBehavior {
///     action publish send new Publish(someTopic, somePublication) via publicationPort;
/// }
/// ```
/// Previously: `control_node_payload_stmt` only accepted `typed_payload_clause` (`name : Type`)
/// for the payload, and `send` had no `via` clause at all -- BNF `SendNode`'s payload is a
/// general expression (`NodeParameterMember` = `FeatureBinding` = `OwnedExpression`), and
/// `SenderReceiverPart` adds the optional `via`/`to` targeting clauses.
#[test]
fn gh86_1_send_expression_payload_with_via() {
    let elements = package_elements(
        r#"package P {
            item def Publish;
            action def AD {
                attribute someTopic = 1;
                attribute somePublication = 2;
                attribute publicationPort = 3;
                action publish send new Publish(someTopic, somePublication) via publicationPort;
            }
        }"#,
    );
    let PackageBodyElement::ActionDef(action_def) = &elements[1] else {
        panic!("expected ActionDef, got {:?}", elements[1]);
    };
    let ActionDefBody::Brace { elements, .. } = &action_def.value.body else {
        panic!("expected brace action def body");
    };
    let publish = elements.iter().find_map(|e| match &e.value {
        ActionDefBodyElement::ActionUsage(a) if a.value.name == "publish" => Some(&a.value),
        _ => None,
    });
    let publish = publish.expect("expected the `publish` ActionUsage");
    assert!(publish.accept.is_none());
    match publish.send.as_ref().expect("expected a send payload") {
        sysml_v2_parser::ast::SendPayload::Expression(e) => {
            assert!(
                matches!(
                    &e.value,
                    sysml_v2_parser::ast::Expression::Constructor { .. }
                ),
                "expected `new Publish(...)` to parse as a Constructor, got {:?}",
                e.value
            );
        }
        other => panic!("expected an Expression payload, got {other:?}"),
    }
    let via = publish.via.as_ref().expect("expected a via clause");
    assert!(matches!(
        &via.value,
        sysml_v2_parser::ast::Expression::FeatureRef(_)
    ));
}

/// Real usage: same file, `consumerBehavior` action:
/// ```text
/// action subscribe send new Subscribe(myTopic, consumer_2) to server_2;
/// ```
/// A `send` with a `to` clause and no `via` -- BNF `SenderReceiverPart`'s second alternative
/// (`EmptyParameterMember 'to' NodeParameterMember`).
#[test]
fn gh86_1_send_expression_payload_with_to_only() {
    let elements = package_elements(
        r#"package P {
            item def Subscribe;
            action def AD {
                attribute myTopic = 1;
                attribute consumer_2 = 2;
                attribute server_2 = 3;
                action subscribe send new Subscribe(myTopic, consumer_2) to server_2;
            }
        }"#,
    );
    let PackageBodyElement::ActionDef(action_def) = &elements[1] else {
        panic!("expected ActionDef, got {:?}", elements[1]);
    };
    let ActionDefBody::Brace { elements, .. } = &action_def.value.body else {
        panic!("expected brace action def body");
    };
    let subscribe = elements.iter().find_map(|e| match &e.value {
        ActionDefBodyElement::ActionUsage(a) if a.value.name == "subscribe" => Some(&a.value),
        _ => None,
    });
    let subscribe = subscribe.expect("expected the `subscribe` ActionUsage");
    assert!(subscribe.via.is_none());
    let to = subscribe.to.as_ref().expect("expected a to clause");
    assert!(matches!(
        &to.value,
        sysml_v2_parser::ast::Expression::FeatureRef(_)
    ));
}

/// The pre-existing `send name : Type;` typed shorthand (Systems Library `SendAction`) must
/// keep working after widening the payload to also accept a general expression.
#[test]
fn gh86_1_send_typed_payload_still_works() {
    let elements = package_elements(
        r#"package P {
            attribute def T;
            action def AD {
                action a1 {
                    send payload : T;
                }
            }
        }"#,
    );
    let PackageBodyElement::ActionDef(action_def) = &elements[1] else {
        panic!("expected ActionDef, got {:?}", elements[1]);
    };
    let ActionDefBody::Brace { elements, .. } = &action_def.value.body else {
        panic!("expected brace action def body");
    };
    let a1 = elements.iter().find_map(|e| match &e.value {
        ActionDefBodyElement::ActionUsage(a) if a.value.name == "a1" => Some(&a.value),
        _ => None,
    });
    let a1 = a1.expect("expected the `a1` ActionUsage");
    let ActionUsageBody::Brace { elements, .. } = a1.body.as_ref().expect("an authored body")
    else {
        panic!("expected brace action usage body");
    };
    let send = elements.iter().find_map(|e| match &e.value {
        ActionUsageBodyElement::ActionUsage(a) if a.value.name == "send" => Some(&a.value),
        _ => None,
    });
    let send = send.expect("expected the standalone `send` control-node ActionUsage");
    match send.send.as_ref().expect("expected a send payload") {
        sysml_v2_parser::ast::SendPayload::Typed(p) => {
            assert_eq!(p.name, "payload");
            assert!(p.type_name.is_some());
        }
        other => panic!("expected a Typed payload, got {other:?}"),
    }
}

/// Real usage: `Simple Tests/ActionTest.sysml`:
/// ```text
/// action snd2 send via this to aa.target;
/// ```
/// A named `action <name> send ...` with *no* payload at all before `via`/`to` (BNF
/// `SenderReceiverPart`'s `EmptyParameterMember 'to' NodeParameterMember` alternative -- `via`
/// can also be present, per `'via' NodeParameterMember ('to' NodeParameterMember)?`). Regression
/// coverage for a bug found while fixing this: naively collapsing "send keyword with no payload"
/// down to the same `None` used for "no send keyword at all" made the `to` clause silently stop
/// parsing whenever the payload was omitted.
#[test]
fn gh86_1_send_no_payload_with_via_and_to() {
    let elements = package_elements(
        r#"package P {
            action def AD {
                attribute this = 1;
                attribute aa = 2;
                action snd2 send via this to aa;
            }
        }"#,
    );
    let PackageBodyElement::ActionDef(action_def) = &elements[0] else {
        panic!("expected ActionDef, got {:?}", elements[0]);
    };
    let ActionDefBody::Brace { elements, .. } = &action_def.value.body else {
        panic!("expected brace action def body");
    };
    let snd2 = elements.iter().find_map(|e| match &e.value {
        ActionDefBodyElement::ActionUsage(a) if a.value.name == "snd2" => Some(&a.value),
        _ => None,
    });
    let snd2 = snd2.expect("expected the `snd2` ActionUsage");
    assert!(
        snd2.send.is_none(),
        "expected no payload to be captured, got {:?}",
        snd2.send
    );
    let via = snd2.via.as_ref().expect("expected a via clause");
    assert!(matches!(
        &via.value,
        sysml_v2_parser::ast::Expression::FeatureRef(_)
    ));
    let to = snd2.to.as_ref().expect("expected a to clause");
    assert!(matches!(
        &to.value,
        sysml_v2_parser::ast::Expression::FeatureRef(_)
    ));
}
