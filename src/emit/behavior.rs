//! Behavior emission (actions, states, perform, allocate).

use super::expr::{emit_expression, emit_feature_value};
use super::root::{emit_doc, emit_identification};
use super::structure::{
    self, emit_definition_prefix, emit_direction, emit_multiplicity, emit_subsetting_clause,
    emit_typing_clause,
};
use super::writer::{
    emit_visibility, format_feature_path, format_name, format_qualified_name, EmitWriter,
};
use super::EmitError;
use crate::ast::{
    ActionDef, ActionDefBody, ActionDefBodyElement, ActionUsage, ActionUsageBody,
    ActionUsageBodyElement, Allocate, AssignStmt, ExhibitState, InOutDecl, Perform, PerformBody,
    PerformBodyElement, PerformInOutBinding, StateDef, StateDefBody, StateDefBodyElement,
    StateUsage, ThenAction, ThenTarget,
};

pub(crate) fn emit_inout_decl(
    w: &mut EmitWriter<'_>,
    _path: &str,
    decl: &InOutDecl,
) -> Result<(), EmitError> {
    emit_direction(w, decl.direction);
    if decl.is_redefinition {
        w.push_str(":>> ");
    }
    w.push_str(&format_name(&decl.name));
    if !decl.type_name.is_empty() {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(&decl.type_name));
    }
    if let Some(value) = &decl.value {
        w.push_str(" = ");
        emit_expression(w, &value.value)?;
    }
    w.push_char(';');
    Ok(())
}

pub(crate) fn emit_action_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &ActionDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("action def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_action_def_body(w, path, &def.body)
}

pub(crate) fn emit_action_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &ActionUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if usage.is_abstract {
        w.push_str("abstract ");
    }
    if usage.is_variation {
        w.push_str("variation ");
    }
    if usage.is_reference {
        w.push_str("ref ");
    }
    // Standalone control nodes (`accept name : Type;`, `send new Publish(x, y) via p;`) are
    // stored as ActionUsage with `name == "accept"|"send"` plus a payload — do not emit
    // `action accept accept …`.
    let is_standalone_accept =
        usage.name == "accept" && usage.accept.is_some() && usage.send.is_none();
    let is_standalone_send = usage.name == "send" && usage.accept.is_none() && usage.send.is_some();
    if is_standalone_accept || is_standalone_send {
        let kw = if is_standalone_accept {
            "accept"
        } else {
            "send"
        };
        w.push_str(kw);
        if let Some(accept) = &usage.accept {
            w.push_char(' ');
            emit_payload_clause(w, accept);
        }
        if let Some(send) = &usage.send {
            w.push_char(' ');
            emit_send_payload(w, path, send)?;
        }
        if let Some(via) = &usage.via {
            w.push_str(" via ");
            emit_expression(w, &via.value)?;
        }
        if let Some(to) = &usage.to {
            w.push_str(" to ");
            emit_expression(w, &to.value)?;
        }
        return emit_action_usage_body(w, path, &usage.body);
    }
    w.push_str("action ");
    if !usage.name.is_empty() {
        w.push_str(&format_name(&usage.name));
    }
    if let Some(typing) = &usage.typing {
        emit_typing_clause(w, &typing.value)?;
    } else if !usage.type_name.is_empty() {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(&usage.type_name));
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if let Some(subsets) = &usage.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(redefines) = &usage.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    if let Some(accept) = &usage.accept {
        w.push_str(" accept ");
        emit_payload_clause(w, accept);
    }
    if let Some(send) = &usage.send {
        w.push_str(" send ");
        emit_send_payload(w, path, send)?;
    }
    if let Some(via) = &usage.via {
        w.push_str(" via ");
        emit_expression(w, &via.value)?;
    }
    if let Some(to) = &usage.to {
        w.push_str(" to ");
        emit_expression(w, &to.value)?;
    }
    emit_action_usage_body(w, path, &usage.body)
}

fn emit_payload_clause(w: &mut EmitWriter<'_>, payload: &crate::ast::PayloadClause) {
    w.push_str(&format_name(&payload.name));
    if let Some(ty) = &payload.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
}

fn emit_send_payload(
    w: &mut EmitWriter<'_>,
    _path: &str,
    payload: &crate::ast::SendPayload,
) -> Result<(), EmitError> {
    match payload {
        crate::ast::SendPayload::Typed(p) => {
            emit_payload_clause(w, p);
            Ok(())
        }
        crate::ast::SendPayload::Expression(e) => emit_expression(w, &e.value),
    }
}

pub(crate) fn emit_action_def_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &ActionDefBody,
) -> Result<(), EmitError> {
    match body {
        ActionDefBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        ActionDefBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_action_def_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

fn emit_action_def_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &ActionDefBodyElement,
) -> Result<(), EmitError> {
    match el {
        ActionDefBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        ActionDefBodyElement::Decl(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ActionBodyDecl,
        }),
        ActionDefBodyElement::Doc(d) => emit_doc(w, &d.value),
        ActionDefBodyElement::InOutDecl(d) => emit_inout_decl(w, path, &d.value),
        ActionDefBodyElement::ActionUsage(a) => emit_action_usage(w, path, &a.value),
        ActionDefBodyElement::PartUsage(p) => structure::emit_part_usage(w, path, &p.value),
        ActionDefBodyElement::ItemUsage(i) => {
            super::requirement::emit_item_usage(w, path, &i.value)
        }
        ActionDefBodyElement::Perform(p) => emit_perform(w, path, &p.value),
        ActionDefBodyElement::Bind(b) => structure::emit_bind(w, path, &b.value),
        ActionDefBodyElement::RefDecl(r) => structure::emit_ref_decl(w, path, &r.value),
        ActionDefBodyElement::Assign(a) => emit_assign(w, &a.value),
        ActionDefBodyElement::ThenAction(t) => emit_then_action(w, path, &t.value),
        ActionDefBodyElement::AssertConstraint(a) => {
            super::view::emit_assert_constraint(w, path, &a.value)
        }
        ActionDefBodyElement::StateUsage(s) => emit_state_usage(w, path, &s.value),
        ActionDefBodyElement::DefaultReferenceUsage(d) => {
            structure::emit_default_reference_usage(w, path, &d.value)
        }
        ActionDefBodyElement::FlowUsage(f) => emit_flow_usage(w, path, &f.value),
        ActionDefBodyElement::FirstStmt(f) => emit_first_stmt(w, &f.value),
        ActionDefBodyElement::MergeStmt(m) => {
            w.push_str("merge ");
            emit_expression(w, &m.value.merge.value)?;
            match &m.value.body {
                crate::ast::FirstMergeBody::Semicolon => w.push_char(';'),
                crate::ast::FirstMergeBody::Brace => w.push_str(" {}"),
            }
            Ok(())
        }
        ActionDefBodyElement::LoopStmt(l) => {
            w.push_str("loop ");
            emit_action_def_body(w, path, &l.value.body)
        }
        ActionDefBodyElement::WhileStmt(wh) => {
            w.push_str("while ");
            emit_expression(w, &wh.value.condition.value)?;
            w.push_char(' ');
            emit_action_def_body(w, path, &wh.value.body)
        }
        ActionDefBodyElement::IfStmt(i) => {
            w.push_str("if ");
            emit_expression(w, &i.value.condition.value)?;
            w.push_char(' ');
            emit_action_def_body(w, path, &i.value.then_body)?;
            if let Some(else_body) = &i.value.else_body {
                w.push_str(" else ");
                emit_action_def_body(w, path, else_body)?;
            }
            Ok(())
        }
        ActionDefBodyElement::ForLoop(f) => {
            w.push_str("for ");
            w.push_str(&format_name(&f.value.var));
            w.push_str(" in ");
            emit_expression(w, &f.value.range.value)?;
            w.push_char(' ');
            emit_action_def_body(w, path, &f.value.body)
        }
        other => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

fn emit_action_usage_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &ActionUsageBody,
) -> Result<(), EmitError> {
    match body {
        ActionUsageBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        ActionUsageBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_action_usage_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

pub(crate) fn emit_action_usage_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &ActionUsageBodyElement,
) -> Result<(), EmitError> {
    match el {
        ActionUsageBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        ActionUsageBodyElement::Decl(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ActionBodyDecl,
        }),
        ActionUsageBodyElement::Doc(d) => emit_doc(w, &d.value),
        ActionUsageBodyElement::InOutDecl(d) => emit_inout_decl(w, path, &d.value),
        ActionUsageBodyElement::ActionUsage(a) => emit_action_usage(w, path, &a.value),
        ActionUsageBodyElement::PartUsage(p) => structure::emit_part_usage(w, path, &p.value),
        ActionUsageBodyElement::ItemUsage(i) => {
            super::requirement::emit_item_usage(w, path, &i.value)
        }
        ActionUsageBodyElement::Bind(b) => structure::emit_bind(w, path, &b.value),
        ActionUsageBodyElement::RefDecl(r) => structure::emit_ref_decl(w, path, &r.value),
        ActionUsageBodyElement::Assign(a) => emit_assign(w, &a.value),
        ActionUsageBodyElement::ThenAction(t) => emit_then_action(w, path, &t.value),
        ActionUsageBodyElement::AssertConstraint(a) => {
            super::view::emit_assert_constraint(w, path, &a.value)
        }
        ActionUsageBodyElement::StateUsage(s) => emit_state_usage(w, path, &s.value),
        ActionUsageBodyElement::DefaultReferenceUsage(d) => {
            structure::emit_default_reference_usage(w, path, &d.value)
        }
        ActionUsageBodyElement::FlowUsage(f) => emit_flow_usage(w, path, &f.value),
        ActionUsageBodyElement::FirstStmt(f) => emit_first_stmt(w, &f.value),
        ActionUsageBodyElement::MergeStmt(m) => {
            w.push_str("merge ");
            emit_expression(w, &m.value.merge.value)?;
            match &m.value.body {
                crate::ast::FirstMergeBody::Semicolon => w.push_char(';'),
                crate::ast::FirstMergeBody::Brace => w.push_str(" {}"),
            }
            Ok(())
        }
        ActionUsageBodyElement::LoopStmt(l) => {
            w.push_str("loop ");
            emit_action_def_body(w, path, &l.value.body)
        }
        ActionUsageBodyElement::WhileStmt(wh) => {
            w.push_str("while ");
            emit_expression(w, &wh.value.condition.value)?;
            w.push_char(' ');
            emit_action_def_body(w, path, &wh.value.body)
        }
        ActionUsageBodyElement::IfStmt(i) => {
            w.push_str("if ");
            emit_expression(w, &i.value.condition.value)?;
            w.push_char(' ');
            emit_action_def_body(w, path, &i.value.then_body)?;
            if let Some(else_body) = &i.value.else_body {
                w.push_str(" else ");
                emit_action_def_body(w, path, else_body)?;
            }
            Ok(())
        }
        other => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

fn emit_assign(w: &mut EmitWriter<'_>, assign: &AssignStmt) -> Result<(), EmitError> {
    if assign.is_then {
        w.push_str("then ");
    }
    w.push_str("assign ");
    emit_expression(w, &assign.lhs.value)?;
    w.push_str(" := ");
    emit_expression(w, &assign.rhs.value)?;
    w.push_char(';');
    Ok(())
}

fn emit_then_action(
    w: &mut EmitWriter<'_>,
    path: &str,
    then: &ThenAction,
) -> Result<(), EmitError> {
    emit_then_action_pub(w, path, then)
}

pub(crate) fn emit_then_action_pub(
    w: &mut EmitWriter<'_>,
    path: &str,
    then: &ThenAction,
) -> Result<(), EmitError> {
    w.push_str("then ");
    match &then.target {
        ThenTarget::Action(a) => emit_action_usage(w, path, &a.value),
        ThenTarget::Perform(p) => emit_perform(w, path, &p.value),
        ThenTarget::Merge(m) => {
            w.push_str("merge ");
            emit_expression(w, &m.value.merge.value)?;
            w.push_char(';');
            Ok(())
        }
        // `FirstMergeBody::Brace` doesn't capture its pin contents (same pre-existing limitation
        // as standalone `fork F { ... };` -- ForkStmt has no emit arm at all yet, GH-93 territory).
        ThenTarget::Fork(_) => w.unsupported(path, "ThenTarget::Fork"),
        // Same `FirstMergeBody::Brace` opacity as Fork; DecisionStmt has no emit arm either.
        ThenTarget::Decide(_) => w.unsupported(path, "ThenTarget::Decide"),
        ThenTarget::Accept(a) => {
            w.push_str("accept ");
            emit_transition_accept(w, &a.value)?;
            w.push_char(';');
            Ok(())
        }
        ThenTarget::Feature(f) => {
            emit_expression(w, &f.value)?;
            w.push_char(';');
            Ok(())
        }
    }
}

pub(crate) fn emit_perform(
    w: &mut EmitWriter<'_>,
    path: &str,
    perform: &Perform,
) -> Result<(), EmitError> {
    emit_definition_prefix(w, perform.usage_prefix.as_ref());
    w.push_str("perform ");
    // Part-usage bodies accept `perform action <name>` for simple names, but dotted
    // feature-path performs must stay bare (`perform providePower.generateTorque :>> …`).
    // Emitting `perform action a.b` reparse-fails (validation `12b-Allocation-1`).
    if !perform.action_name.contains('.') {
        w.push_str("action ");
    }
    if !perform.action_name.is_empty() {
        if perform.action_name.contains('.') {
            w.push_str(&format_feature_path(&perform.action_name));
        } else {
            w.push_str(&format_name(&perform.action_name));
        }
    }
    if let Some(ty) = &perform.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    if let Some(redef) = &perform.redefines {
        w.push_str(" :>> ");
        w.push_str(&format_qualified_name(redef));
    }
    if let Some(value) = &perform.value {
        emit_feature_value(w, value)?;
    }
    emit_perform_body(w, path, &perform.body)
}

fn emit_perform_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &PerformBody,
) -> Result<(), EmitError> {
    match body {
        PerformBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        PerformBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_perform_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

fn emit_perform_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &PerformBodyElement,
) -> Result<(), EmitError> {
    match el {
        PerformBodyElement::Doc(d) => emit_doc(w, &d.value),
        PerformBodyElement::InOut(b) => emit_perform_inout(w, &b.value),
        PerformBodyElement::Variant(v) => structure::emit_variant_usage(w, path, &v.value),
        PerformBodyElement::Action(a) => emit_action_usage_body_element(w, path, &a.value),
        PerformBodyElement::PartUsage(p) => structure::emit_part_usage(w, path, &p.value),
        PerformBodyElement::ItemUsage(i) => super::requirement::emit_item_usage(w, path, &i.value),
        PerformBodyElement::AttributeUsage(a) => structure::emit_attribute_usage(w, path, &a.value),
    }
}

fn emit_perform_inout(
    w: &mut EmitWriter<'_>,
    binding: &PerformInOutBinding,
) -> Result<(), EmitError> {
    emit_direction(w, binding.direction);
    w.push_str(&format_name(&binding.name));
    w.push_str(" = ");
    emit_expression(w, &binding.value.value)?;
    w.push_char(';');
    Ok(())
}

pub(crate) fn emit_state_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &StateDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("state def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_state_def_body(w, path, &def.body)
}

pub(crate) fn emit_state_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &StateUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if let Some(dir) = usage.direction {
        emit_direction(w, dir);
    }
    if usage.is_derived {
        w.push_str("derived ");
    }
    if usage.is_abstract {
        w.push_str("abstract ");
    }
    if usage.is_reference {
        w.push_str("ref ");
    }
    if usage.is_individual {
        w.push_str("individual ");
    }
    w.push_str("state ");
    if !usage.name.is_empty() {
        w.push_str(&format_feature_path(&usage.name));
    }
    if let Some(typing) = &usage.typing {
        emit_typing_clause(w, &typing.value)?;
    } else if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if let Some(subsets) = &usage.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(redefines) = &usage.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    emit_state_def_body(w, path, &usage.body)
}

pub(crate) fn emit_exhibit_state(
    w: &mut EmitWriter<'_>,
    path: &str,
    exhibit: &ExhibitState,
) -> Result<(), EmitError> {
    emit_visibility(w, exhibit.membership.visibility);
    if let Some(dir) = exhibit.direction {
        emit_direction(w, dir);
    }
    if exhibit.is_derived {
        w.push_str("derived ");
    }
    if exhibit.is_abstract {
        w.push_str("abstract ");
    }
    if exhibit.is_reference {
        w.push_str("ref ");
    }
    if exhibit.is_individual {
        w.push_str("individual ");
    }
    w.push_str("exhibit ");
    // `state` after `exhibit` is optional (§6 G18); omit so `exhibit vehicleStates.on` roundtrips.
    if !exhibit.name.is_empty() {
        w.push_str(&format_feature_path(&exhibit.name));
    }
    if let Some(typing) = &exhibit.typing {
        emit_typing_clause(w, &typing.value)?;
    } else if let Some(ty) = &exhibit.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    if let Some(mult) = &exhibit.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if let Some(subsets) = &exhibit.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(redefines) = &exhibit.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    emit_state_def_body(w, path, &exhibit.body)
}

fn emit_state_def_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &StateDefBody,
) -> Result<(), EmitError> {
    match body {
        StateDefBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        StateDefBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_state_def_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

fn emit_state_def_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &StateDefBodyElement,
) -> Result<(), EmitError> {
    match el {
        StateDefBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        StateDefBodyElement::Other(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::Other,
        }),
        StateDefBodyElement::InOutDecl(d) => emit_inout_decl(w, path, &d.value),
        StateDefBodyElement::Doc(d) => emit_doc(w, &d.value),
        StateDefBodyElement::Entry(e) => {
            w.push_str("entry");
            if let Some(name) = &e.value.action_name {
                if e.value.has_action_keyword {
                    w.push_str(" action ");
                } else {
                    w.push_char(' ');
                }
                w.push_str(&format_name(name));
            }
            emit_state_def_body(w, path, &e.value.body)
        }
        StateDefBodyElement::Do(d) => {
            w.push_str("do");
            if let Some(name) = &d.value.action_name {
                if d.value.has_action_keyword {
                    w.push_str(" action ");
                } else {
                    w.push_char(' ');
                }
                w.push_str(&format_name(name));
            }
            emit_state_def_body(w, path, &d.value.body)
        }
        StateDefBodyElement::Exit(e) => {
            w.push_str("exit");
            if let Some(name) = &e.value.action_name {
                if e.value.has_action_keyword {
                    w.push_str(" action ");
                } else {
                    w.push_char(' ');
                }
                w.push_str(&format_name(name));
            }
            emit_state_def_body(w, path, &e.value.body)
        }
        StateDefBodyElement::Then(t) => {
            w.push_str("then ");
            w.push_str(&format_name(&t.value.state_name));
            w.push_char(';');
            Ok(())
        }
        StateDefBodyElement::FinalState(f) => {
            w.push_str("final ");
            w.push_str(&format_name(&f.value.state_name));
            w.push_char(';');
            Ok(())
        }
        StateDefBodyElement::Ref(r) => structure::emit_ref_decl(w, path, &r.value),
        StateDefBodyElement::StateUsage(s) => emit_state_usage(w, path, &s.value),
        StateDefBodyElement::RequirementUsage(r) => {
            super::requirement::emit_requirement_usage(w, path, &r.value)
        }
        StateDefBodyElement::Transition(t) => emit_transition(w, path, &t.value),
        other => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

pub(crate) fn emit_allocate(
    w: &mut EmitWriter<'_>,
    _path: &str,
    allocate: &Allocate,
) -> Result<(), EmitError> {
    w.push_str("allocate ");
    emit_expression(w, &allocate.source.value)?;
    w.push_str(" to ");
    emit_expression(w, &allocate.target.value)?;
    match &allocate.body {
        crate::ast::ConnectBody::Semicolon => w.push_char(';'),
        crate::ast::ConnectBody::Brace => w.push_str(" {}"),
    }
    Ok(())
}

pub(crate) fn emit_allocation_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::AllocationDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("allocation def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_definition_body(w, path, &def.body)
}

pub(crate) fn emit_allocation_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &crate::ast::AllocationUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    // Bare `allocate src to dst` (package-level `allocate_usage`) must not be rewritten as
    // `allocation allocate …` — that form reparses as ExtendedLibraryDecl (validation `12b`).
    let shorthand = usage.name.is_empty() && usage.type_name.is_none();
    if shorthand {
        w.push_str("allocate ");
        let (Some(source), Some(target)) = (&usage.source, &usage.target) else {
            return w.unsupported(path, "allocate shorthand without ends");
        };
        emit_expression(w, &source.value)?;
        w.push_str(" to ");
        emit_expression(w, &target.value)?;
        return emit_definition_body(w, path, &usage.body);
    }
    w.push_str("allocation ");
    if !usage.name.is_empty() {
        w.push_str(&format_name(&usage.name));
    }
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    if let (Some(source), Some(target)) = (&usage.source, &usage.target) {
        w.push_str(" allocate ");
        emit_expression(w, &source.value)?;
        w.push_str(" to ");
        emit_expression(w, &target.value)?;
    }
    emit_definition_body(w, path, &usage.body)
}

pub(crate) fn emit_flow_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    flow: &crate::ast::FlowUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, flow.membership.visibility);
    match flow.kind {
        crate::ast::FlowUsageKind::Flow => w.push_str("flow "),
        crate::ast::FlowUsageKind::Message => w.push_str("message "),
        crate::ast::FlowUsageKind::SuccessionFlow => w.push_str("succession flow "),
    }
    if let Some(name) = &flow.name {
        w.push_str(&format_name(name));
    }
    if let Some(ty) = &flow.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    if let Some(payload) = &flow.payload {
        w.push_str(" of ");
        if let Some(n) = &payload.value.name {
            w.push_str(&format_name(n));
            if payload.value.type_name.is_some() || payload.value.multiplicity.is_some() {
                w.push_str(" : ");
            }
        }
        if let Some(ty) = &payload.value.type_name {
            w.push_str(&format_qualified_name(ty));
        }
        if let Some(mult) = &payload.value.multiplicity {
            emit_multiplicity(w, &mult.value)?;
        }
    }
    if let Some(from) = &flow.from {
        // Unnamed flows use shorthand `flow <from> to <to>` so `from` is not reparsed as a name.
        if flow.name.is_some() || flow.payload.is_some() || flow.type_name.is_some() {
            w.push_str(" from ");
        } else {
            w.push_char(' ');
        }
        emit_expression(w, &from.value)?;
    }
    if let Some(to) = &flow.to {
        w.push_str(" to ");
        emit_expression(w, &to.value)?;
    }
    emit_definition_body(w, path, &flow.body)
}

fn emit_definition_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &crate::ast::DefinitionBody,
) -> Result<(), EmitError> {
    match body {
        crate::ast::DefinitionBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::DefinitionBody::Brace { elements } => {
            if elements.is_empty() {
                w.push_str(" {}");
                Ok(())
            } else {
                w.push_str(" {");
                w.newline();
                w.indent();
                for (i, el) in elements.iter().enumerate() {
                    match &el.value {
                        crate::ast::DefinitionBodyElement::Error(_) => {
                            return Err(EmitError::Opaque {
                                path: format!("{path}/body[{i}]"),
                                kind: super::OpacityKind::ParseError,
                            });
                        }
                        crate::ast::DefinitionBodyElement::Other(_) => {
                            return Err(EmitError::Opaque {
                                path: format!("{path}/body[{i}]"),
                                kind: super::OpacityKind::Other,
                            });
                        }
                        crate::ast::DefinitionBodyElement::Doc(d) => emit_doc(w, &d.value)?,
                        crate::ast::DefinitionBodyElement::OccurrenceMember(o) => {
                            emit_occurrence_body_element(
                                w,
                                &format!("{path}/body[{i}]"),
                                &o.value,
                            )?;
                        }
                    }
                    w.newline();
                }
                w.dedent();
                w.push_char('}');
                Ok(())
            }
        }
    }
}

/// Shared by `emit_transition`'s `accept` trigger and `ThenTarget::Accept` (GH-86, `then accept
/// S;`) -- same `TransitionAccept` shape in both positions.
fn emit_transition_accept(
    w: &mut EmitWriter<'_>,
    accept: &crate::ast::TransitionAccept,
) -> Result<(), EmitError> {
    match accept {
        crate::ast::TransitionAccept::Payload(p, via) => {
            w.push_str(&format_name(&p.name));
            if let Some(ty) = &p.type_name {
                w.push_str(" : ");
                w.push_str(&format_qualified_name(ty));
            }
            if let Some(v) = via {
                w.push_str(" via ");
                emit_expression(w, &v.value)?;
            }
        }
        crate::ast::TransitionAccept::Shorthand(e, via) => {
            emit_expression(w, &e.value)?;
            if let Some(v) = via {
                w.push_str(" via ");
                emit_expression(w, &v.value)?;
            }
        }
        crate::ast::TransitionAccept::TimeTrigger(kind, e) => {
            match kind {
                crate::ast::TriggerKind::At => w.push_str("at "),
                crate::ast::TriggerKind::When => w.push_str("when "),
                crate::ast::TriggerKind::After => w.push_str("after "),
            }
            emit_expression(w, &e.value)?;
        }
    }
    Ok(())
}

fn emit_transition(
    w: &mut EmitWriter<'_>,
    path: &str,
    t: &crate::ast::Transition,
) -> Result<(), EmitError> {
    w.push_str("transition ");
    if let Some(name) = &t.name {
        w.push_str(&format_name(name));
        w.push_char(' ');
    }
    if let Some(source) = &t.source {
        // Always emit `first` when a source was parsed; `is_initial` only marks unnamed
        // `transition first …` forms and must not gate the keyword.
        w.push_str("first ");
        emit_expression(w, &source.value)?;
        w.push_char(' ');
    }
    if let Some(accept) = &t.accept {
        w.push_str("accept ");
        emit_transition_accept(w, accept)?;
        w.push_char(' ');
    }
    if let Some(guard) = &t.guard {
        w.push_str("if ");
        emit_expression(w, &guard.value)?;
        w.push_char(' ');
    }
    if let Some(effect) = &t.effect {
        w.push_str("do ");
        match effect {
            crate::ast::TransitionEffect::Perform { name, type_name } => {
                w.push_str("action ");
                if let Some(n) = name {
                    w.push_str(&format_name(n));
                }
                if let Some(ty) = type_name {
                    w.push_str(" : ");
                    w.push_str(&format_qualified_name(ty));
                }
            }
            crate::ast::TransitionEffect::Accept {
                payload,
                type_name,
                via,
            } => {
                w.push_str("accept ");
                emit_expression(w, &payload.value)?;
                if let Some(ty) = type_name {
                    w.push_str(" : ");
                    w.push_str(&format_qualified_name(ty));
                }
                if let Some(v) = via {
                    w.push_str(" via ");
                    emit_expression(w, &v.value)?;
                }
            }
            crate::ast::TransitionEffect::Send {
                payload,
                type_name,
                via,
                to,
            } => {
                w.push_str("send ");
                emit_expression(w, &payload.value)?;
                if let Some(ty) = type_name {
                    w.push_str(" : ");
                    w.push_str(&format_qualified_name(ty));
                }
                if let Some(v) = via {
                    w.push_str(" via ");
                    emit_expression(w, &v.value)?;
                }
                if let Some(t) = to {
                    w.push_str(" to ");
                    emit_expression(w, &t.value)?;
                }
            }
            crate::ast::TransitionEffect::Assign { lhs, rhs } => {
                w.push_str("assign ");
                emit_expression(w, &lhs.value)?;
                w.push_str(" := ");
                emit_expression(w, &rhs.value)?;
            }
            crate::ast::TransitionEffect::Expression(e) => emit_expression(w, &e.value)?,
        }
        w.push_char(' ');
    }
    w.push_str("then ");
    emit_expression(w, &t.target.value)?;
    match &t.body {
        crate::ast::ConnectBody::Semicolon => w.push_char(';'),
        crate::ast::ConnectBody::Brace => w.push_str(" {}"),
    }
    let _ = path;
    Ok(())
}

fn emit_first_stmt(w: &mut EmitWriter<'_>, first: &crate::ast::FirstStmt) -> Result<(), EmitError> {
    if first.succession_name.is_some()
        || first.succession_type.is_some()
        || first.succession_multiplicity.is_some()
    {
        w.push_str("succession ");
        if let Some(mult) = &first.succession_multiplicity {
            emit_multiplicity(w, &mult.value)?;
            w.push_char(' ');
        }
        if let Some(name) = &first.succession_name {
            w.push_str(&format_name(name));
            w.push_char(' ');
        }
        if let Some(ty) = &first.succession_type {
            w.push_str(": ");
            w.push_str(&format_qualified_name(ty));
            w.push_char(' ');
        }
    }
    w.push_str("first ");
    if let Some(mult) = &first.first_multiplicity {
        emit_multiplicity(w, &mult.value)?;
        w.push_char(' ');
    }
    emit_expression(w, &first.first.value)?;
    if let Some(then) = &first.then {
        w.push_str(" then ");
        if let Some(mult) = &first.then_multiplicity {
            emit_multiplicity(w, &mult.value)?;
            w.push_char(' ');
        }
        emit_expression(w, &then.value)?;
    }
    match &first.body {
        crate::ast::FirstMergeBody::Semicolon => w.push_char(';'),
        crate::ast::FirstMergeBody::Brace => w.push_str(" {}"),
    }
    Ok(())
}

pub(crate) fn emit_occurrence_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::OccurrenceDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    if def.is_abstract {
        w.push_str("abstract ");
    }
    w.push_str("occurrence def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_definition_body(w, path, &def.body)
}

pub(crate) fn emit_occurrence_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &crate::ast::OccurrenceUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if usage.is_then {
        w.push_str("then ");
    }
    if usage.is_abstract {
        w.push_str("abstract ");
    }
    if usage.is_constant {
        w.push_str("constant ");
    }
    if usage.is_reference {
        w.push_str("ref ");
    }
    if usage.is_individual {
        w.push_str("individual ");
    }
    if usage.is_event {
        w.push_str("event ");
    }
    if let Some(portion) = &usage.portion_kind {
        // `snapshot` / `timeslice` usages parse without an `occurrence` keyword.
        w.push_str(portion);
        w.push_char(' ');
    } else if usage.is_event || !usage.is_individual {
        // Plain `occurrence …` and `event occurrence …`. Bare `individual <name>` omits
        // the keyword (see `individual_usage` → `occurrence_usage_tail`).
        w.push_str("occurrence ");
    }
    if !usage.name.is_empty() {
        w.push_str(&format_name(&usage.name));
    }
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if let Some(subsets) = &usage.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(redefines) = &usage.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    if let Some(references) = &usage.references {
        emit_subsetting_clause(w, &references.value)?;
    }
    if let Some(crosses) = &usage.crosses {
        emit_subsetting_clause(w, &crosses.value)?;
    }
    if let Some(intersects) = &usage.intersects {
        emit_subsetting_clause(w, &intersects.value)?;
    }
    match &usage.body {
        crate::ast::OccurrenceUsageBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::OccurrenceUsageBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_occurrence_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

pub(crate) fn emit_occurrence_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &crate::ast::OccurrenceBodyElement,
) -> Result<(), EmitError> {
    match el {
        crate::ast::OccurrenceBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        crate::ast::OccurrenceBodyElement::Other(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::Other,
        }),
        crate::ast::OccurrenceBodyElement::Doc(d) => emit_doc(w, &d.value),
        crate::ast::OccurrenceBodyElement::AssertConstraint(a) => {
            super::view::emit_assert_constraint(w, path, &a.value)
        }
        crate::ast::OccurrenceBodyElement::FlowUsage(f) => emit_flow_usage(w, path, &f.value),
        crate::ast::OccurrenceBodyElement::AttributeUsage(a) => {
            structure::emit_attribute_usage(w, path, &a.value)
        }
        crate::ast::OccurrenceBodyElement::PartUsage(p) => {
            structure::emit_part_usage(w, path, &p.value)
        }
        crate::ast::OccurrenceBodyElement::OccurrenceUsage(o) => {
            emit_occurrence_usage(w, path, &o.value)
        }
        crate::ast::OccurrenceBodyElement::Allocate(a) => emit_allocate(w, path, &a.value),
        crate::ast::OccurrenceBodyElement::EndDecl(e) => {
            structure::emit_end_decl(w, path, &e.value)
        }
        crate::ast::OccurrenceBodyElement::Satisfy(s) => {
            super::requirement::emit_satisfy(w, path, &s.value)
        }
        crate::ast::OccurrenceBodyElement::StateUsage(s) => {
            // Occurrence-body `StateUsage` nodes are exhibit usages (§6 G30 / G18).
            emit_occurrence_exhibit(w, path, &s.value)
        }
        crate::ast::OccurrenceBodyElement::SuccessionUsage(s) => emit_succession_usage(w, &s.value),
        other => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

fn emit_occurrence_exhibit(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &StateUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if let Some(dir) = usage.direction {
        emit_direction(w, dir);
    }
    if usage.is_derived {
        w.push_str("derived ");
    }
    if usage.is_abstract {
        w.push_str("abstract ");
    }
    if usage.is_reference {
        w.push_str("ref ");
    }
    if usage.is_individual {
        w.push_str("individual ");
    }
    w.push_str("exhibit ");
    if !usage.name.is_empty() {
        w.push_str(&format_feature_path(&usage.name));
    }
    if let Some(typing) = &usage.typing {
        emit_typing_clause(w, &typing.value)?;
    } else if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if let Some(subsets) = &usage.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(redefines) = &usage.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    emit_state_def_body(w, path, &usage.body)
}

fn emit_succession_usage(
    w: &mut EmitWriter<'_>,
    succ: &crate::ast::SuccessionUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, succ.membership.visibility);
    w.push_str("succession ");
    if let Some(mult) = &succ.multiplicity {
        emit_multiplicity(w, &mult.value)?;
        w.push_char(' ');
    }
    if let Some(name) = &succ.name {
        w.push_str(&format_name(name));
        w.push_char(' ');
    }
    w.push_str("first ");
    if let Some(mult) = &succ.source_multiplicity {
        emit_multiplicity(w, &mult.value)?;
        w.push_char(' ');
    }
    emit_expression(w, &succ.source.value)?;
    w.push_str(" then ");
    if let Some(mult) = &succ.target_multiplicity {
        emit_multiplicity(w, &mult.value)?;
        w.push_char(' ');
    }
    emit_expression(w, &succ.target.value)?;
    match &succ.body {
        crate::ast::ConnectBody::Semicolon => w.push_char(';'),
        crate::ast::ConnectBody::Brace => w.push_str(" {}"),
    }
    Ok(())
}
