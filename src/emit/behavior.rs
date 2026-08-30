//! Behavior emission (actions, states, perform, allocate).

use super::expr::{emit_expression, emit_feature_value};
use super::root::emit_identification;
use super::structure::{
    self, emit_definition_prefix, emit_definition_prefix_value, emit_direction, emit_multiplicity,
    emit_multiplicity_modifiers, emit_subsetting_clause, emit_typing_clause,
};
use super::writer::{emit_visibility, EmitWriter};
use super::EmitError;
use crate::ast::{
    ActionDef, ActionDefBody, ActionDefBodyElement, ActionUsage, ActionUsageBody,
    ActionUsageBodyElement, ActionUsageKeyword, Allocate, AssignStmt, ExhibitState, InOutDecl,
    Perform, PerformActionTarget, PerformBody, PerformBodyElement, PerformInOutBinding, StateDef,
    StateDefBody, StateDefBodyElement, StateUsage, ThenAction, ThenTarget, UsageDeclaration,
};

pub(crate) fn emit_inout_decl(
    w: &mut EmitWriter<'_>,
    path: &str,
    decl: &InOutDecl,
) -> Result<(), EmitError> {
    emit_direction(w, decl.direction);
    if let Some(kind) = &decl.kind {
        match kind.value {
            crate::ast::InOutDeclKind::Action => w.push_str("action "),
        }
    }
    if decl.is_reference {
        w.push_str("ref ");
    }
    if decl.is_var {
        w.push_str("var ");
    }
    let leading_redefinition = decl.name.is_none();
    if let Some(name) = decl.name {
        w.push_declaration_name(&format!("{path}/name"), name)?;
    } else if let Some(redefines) = &decl.redefines {
        emit_inout_redefines(w, path, redefines)?;
    }
    if let Some(subsets) = &decl.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(type_name) = decl.type_name {
        w.push_str(" : ");
        w.push_qualified_reference(path, type_name)?;
    }
    if let Some(multiplicity) = &decl.multiplicity {
        emit_multiplicity(w, &multiplicity.value)?;
    }
    emit_multiplicity_modifiers(w, &decl.multiplicity_modifiers);
    if !leading_redefinition {
        if let Some(redefines) = &decl.redefines {
            w.push_char(' ');
            emit_inout_redefines(w, path, redefines)?;
        }
    }
    if let Some(value) = &decl.value {
        emit_feature_value(w, value)?;
    }
    match &decl.body {
        None => w.push_char(';'),
        Some(elements) => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_action_def_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
        }
    }
    Ok(())
}

fn emit_inout_redefines(
    w: &mut EmitWriter<'_>,
    path: &str,
    redefines: &crate::ast::Node<crate::ast::SubsettingRelationship>,
) -> Result<(), EmitError> {
    w.push_str(":>> ");
    for (index, target) in redefines.value.target.iter().copied().enumerate() {
        if index > 0 {
            w.push_str(", ");
        }
        w.push_qualified_reference(&format!("{path}/redefines[{index}]"), target)?;
    }
    Ok(())
}

pub(crate) fn emit_action_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &ActionDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    emit_definition_prefix(w, def.definition_prefix.as_ref());
    if def.is_individual {
        w.push_str("individual ");
    }
    w.push_str("action def ");
    emit_identification(w, &def.identification)?;
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_action_def_body(w, path, &def.body)
}

/// Stream the shared `ActionNodePrefix` before a loop-node keyword. The prefix owns its optional
/// `action` declaration and complete feature-specialization header; `while`, `loop`, and `for`
/// merely choose the following node alternative.
fn emit_action_node_prefix(
    w: &mut EmitWriter<'_>,
    path: &str,
    prefix: &crate::ast::ActionNodePrefix,
) -> Result<(), EmitError> {
    structure::emit_occurrence_usage_prefix(
        w,
        &format!("{path}/occurrence-prefix"),
        &prefix.occurrence_prefix,
    )?;
    if let Some(declaration) = &prefix.action_declaration {
        emit_action_node_usage_declaration(w, path, &declaration.value)?;
        w.push_char(' ');
    }
    Ok(())
}

fn emit_action_node_usage_declaration(
    w: &mut EmitWriter<'_>,
    path: &str,
    declaration: &crate::ast::ActionNodeUsageDeclaration,
) -> Result<(), EmitError> {
    w.push_str("action");
    let Some(declaration) = declaration.declaration.as_ref() else {
        return Ok(());
    };
    emit_usage_declaration(w, path, &declaration.value)
}

/// Stream the grammar-owned declaration shared by action-node and perform-action forms.
pub(crate) fn emit_usage_declaration(
    w: &mut EmitWriter<'_>,
    path: &str,
    declaration: &UsageDeclaration,
) -> Result<(), EmitError> {
    if declaration.identification_span.len != 0 {
        w.push_char(' ');
        w.push_authored_name(
            &format!("{path}/declaration/identification"),
            &declaration.identification_span,
        )?;
    }
    if let Some(typing) = &declaration.typing {
        emit_typing_clause(w, &typing.value)?;
    }
    if let Some(multiplicity) = &declaration.multiplicity {
        emit_multiplicity(w, &multiplicity.value)?;
    }
    emit_multiplicity_modifiers(w, &declaration.multiplicity_modifiers);
    if let Some((subsets, value)) = &declaration.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
        if let Some(value) = value {
            w.push_str(" = ");
            emit_expression(w, &value.value)?;
        }
    }
    for relationship in [
        declaration.redefines.as_ref(),
        declaration.references.as_ref(),
        declaration.crosses.as_ref(),
        declaration.intersects.as_ref(),
    ]
    .into_iter()
    .flatten()
    {
        emit_subsetting_clause(w, &relationship.value)?;
    }
    Ok(())
}

fn emit_action_body_parameter(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &crate::ast::ActionBodyParameter,
) -> Result<(), EmitError> {
    if let Some(declaration) = &body.action_declaration {
        w.push_char(' ');
        emit_action_node_usage_declaration(w, path, &declaration.value)?;
    }
    emit_action_def_body(w, path, &body.body)
}

/// Emit the complete `ForLoopNode` production.  Its variable is a declaration, so its name is
/// streamed from the aggregate authored span rather than re-escaped from the decoded label.
pub(crate) fn emit_for_loop(
    w: &mut EmitWriter<'_>,
    path: &str,
    for_loop: &crate::ast::ForLoop,
) -> Result<(), EmitError> {
    emit_action_node_prefix(w, path, &for_loop.prefix)?;
    w.push_str("for ");
    emit_for_variable_declaration(w, path, &for_loop.variable.value)?;
    w.push_str(" in ");
    emit_expression(w, &for_loop.in_parameter.expression.value)?;
    emit_action_body_parameter(w, path, &for_loop.body)
}

fn emit_for_variable_declaration(
    w: &mut EmitWriter<'_>,
    path: &str,
    declaration: &crate::ast::ForVariableDeclaration,
) -> Result<(), EmitError> {
    w.push_authored_name(
        &format!("{path}/for-variable/identification"),
        &declaration.identification_span,
    )?;
    if let Some(typing) = &declaration.typing {
        emit_typing_clause(w, &typing.value)?;
    }
    if let Some(multiplicity) = &declaration.multiplicity {
        emit_multiplicity(w, &multiplicity.value)?;
    }
    emit_multiplicity_modifiers(w, &declaration.multiplicity_modifiers);
    if let Some((subsets, value)) = &declaration.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
        if let Some(value) = value {
            w.push_str(" = ");
            emit_expression(w, &value.value)?;
        }
    }
    for relationship in [
        declaration.redefines.as_ref(),
        declaration.references.as_ref(),
        declaration.crosses.as_ref(),
        declaration.intersects.as_ref(),
    ]
    .into_iter()
    .flatten()
    {
        emit_subsetting_clause(w, &relationship.value)?;
    }
    Ok(())
}

fn emit_until_parameter(
    w: &mut EmitWriter<'_>,
    until: Option<&crate::ast::UntilParameter>,
) -> Result<(), EmitError> {
    if let Some(until) = until {
        w.push_str(" until ");
        emit_expression(w, &until.expression.value)?;
        w.push_char(';');
    }
    Ok(())
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
    if usage.is_individual {
        w.push_str("individual ");
    }
    // Standalone control nodes (`accept name : Type;`, `send new Publish(x, y) via p;`) are
    // stored as an `ActionUsage` whose `keyword` names the node kind -- do not emit
    // `action accept …`.
    let control_keyword = match usage.keyword {
        ActionUsageKeyword::Action => None,
        ActionUsageKeyword::Accept => Some("accept"),
        ActionUsageKeyword::Send => Some("send"),
    };
    if let Some(kw) = control_keyword {
        w.push_str(kw);
        if let Some(accept) = &usage.accept {
            w.push_char(' ');
            emit_transition_accept(w, path, accept)?;
        }
        if let Some(send) = &usage.send {
            w.push_char(' ');
            emit_send_payload(w, path, send)?;
        }
        if usage.keyword != ActionUsageKeyword::Accept {
            if let Some(via) = &usage.via {
                w.push_str(" via ");
                emit_expression(w, &via.value)?;
            }
        }
        if let Some(to) = &usage.to {
            w.push_str(" to ");
            emit_expression(w, &to.value)?;
        }
        return match &usage.body {
            Some(body) => emit_action_usage_body(w, path, body),
            None => Ok(()),
        };
    }
    // Anonymous usages (`action :>> subactions;`, `action { ... }`) get no trailing space
    // after the keyword -- the clause emitters below supply their own leading space.
    w.push_str("action");
    if let Some(short_name) = usage.short_name {
        w.push_str(" <");
        w.push_declaration_name(&format!("{path}/short-name"), short_name)?;
        w.push_char('>');
    }
    if let Some(name) = usage.name {
        w.push_char(' ');
        w.push_declaration_name(&format!("{path}/name"), name)?;
    }
    if let Some(typing) = &usage.typing {
        emit_typing_clause(w, &typing.value)?;
    } else if let Some(type_name) = usage.type_name {
        w.push_str(" : ");
        w.push_qualified_reference(path, type_name)?;
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    super::structure::emit_multiplicity_modifiers(w, &usage.multiplicity_modifiers);
    if let Some(subsets) = &usage.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(redefines) = &usage.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    if let Some(accept) = &usage.accept {
        w.push_str(" accept ");
        emit_transition_accept(w, path, accept)?;
    }
    if let Some(send) = &usage.send {
        w.push_str(" send ");
        emit_send_payload(w, path, send)?;
    }
    if usage.accept.is_none() {
        if let Some(via) = &usage.via {
            w.push_str(" via ");
            emit_expression(w, &via.value)?;
        }
    }
    if let Some(to) = &usage.to {
        w.push_str(" to ");
        emit_expression(w, &to.value)?;
    }
    match &usage.body {
        Some(body) => emit_action_usage_body(w, path, body),
        None => Ok(()),
    }
}

fn emit_payload_clause(
    w: &mut EmitWriter<'_>,
    path: &str,
    payload: &crate::ast::PayloadClause,
) -> Result<(), EmitError> {
    w.push_declaration_name(&format!("{path}/name"), payload.name)?;
    if let Some(ty) = payload.type_name {
        w.push_str(" : ");
        w.push_qualified_reference(path, ty)?;
    }
    Ok(())
}

fn emit_send_payload(
    w: &mut EmitWriter<'_>,
    path: &str,
    payload: &crate::ast::SendPayload,
) -> Result<(), EmitError> {
    match payload {
        crate::ast::SendPayload::Typed(p) => emit_payload_clause(w, path, p),
        crate::ast::SendPayload::Expression(e) => emit_expression(w, &e.value),
    }
}

/// Emits either spelling of an `if` branch: a braced body, or the brace-less shorthand member.
pub(crate) fn emit_action_branch_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    branch: &crate::ast::ActionBranchBody,
) -> Result<(), EmitError> {
    match branch {
        crate::ast::ActionBranchBody::Braced(body) => emit_action_def_body(w, path, body),
        crate::ast::ActionBranchBody::Shorthand(member) => {
            w.push_char(' ');
            emit_action_def_body_element(w, path, &member.value)
        }
    }
}

pub(crate) fn emit_action_def_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &ActionDefBody,
) -> Result<(), EmitError> {
    match body {
        ActionDefBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        ActionDefBody::Brace { elements, .. } => {
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

pub(crate) fn emit_action_def_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &ActionDefBodyElement,
) -> Result<(), EmitError> {
    match el {
        ActionDefBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
        ActionDefBodyElement::Import(import) => super::root::emit_import(w, &import.value),
        ActionDefBodyElement::AttributeUsage(a) => {
            structure::emit_attribute_usage(w, path, &a.value)
        }
        ActionDefBodyElement::CalcUsage(c) => super::view::emit_calc_usage(w, path, &c.value),
        ActionDefBodyElement::ActionDef(d) => emit_action_def(w, path, &d.value),
        ActionDefBodyElement::VariantUsage(v) => structure::emit_variant_usage(w, path, &v.value),
        ActionDefBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
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
        ActionDefBodyElement::GuardedSuccession(succession) => {
            emit_guarded_succession(w, path, &succession.value)
        }
        ActionDefBodyElement::FirstStmt(f) => emit_first_stmt(w, path, &f.value),
        ActionDefBodyElement::MergeStmt(m) => emit_merge_stmt(w, path, &m.value),
        ActionDefBodyElement::DecisionStmt(d) => emit_decision_stmt(w, path, &d.value),
        ActionDefBodyElement::JoinStmt(j) => emit_join_stmt(w, path, &j.value),
        ActionDefBodyElement::ForkStmt(f) => emit_fork_stmt(w, path, &f.value),
        ActionDefBodyElement::LoopStmt(l) => {
            emit_action_node_prefix(w, path, &l.value.prefix)?;
            w.push_str("loop");
            emit_action_body_parameter(w, path, &l.value.body)?;
            emit_until_parameter(w, l.value.until.as_ref())
        }
        ActionDefBodyElement::WhileStmt(wh) => {
            emit_action_node_prefix(w, path, &wh.value.prefix)?;
            w.push_str("while ");
            emit_expression(w, &wh.value.condition.value)?;
            emit_action_body_parameter(w, path, &wh.value.body)?;
            emit_until_parameter(w, wh.value.until.as_ref())
        }
        ActionDefBodyElement::IfStmt(i) => {
            w.push_str("if ");
            emit_expression(w, &i.value.condition.value)?;
            emit_action_branch_body(w, path, &i.value.then_body)?;
            if let Some(else_body) = &i.value.else_body {
                w.push_str(" else");
                emit_action_branch_body(w, path, else_body)?;
            }
            Ok(())
        }
        ActionDefBodyElement::Transition(transition) => emit_transition(w, path, &transition.value),
        ActionDefBodyElement::ForLoop(f) => emit_for_loop(w, path, &f.value),
        ActionDefBodyElement::OccurrenceUsage(o) => emit_occurrence_usage(w, path, &o.value),
        ActionDefBodyElement::MetadataKeywordUsage(m) => {
            structure::emit_metadata_keyword_usage(w, path, &m.value)
        }
        ActionDefBodyElement::MetadataUsage(m) => structure::emit_metadata_usage(w, path, &m.value),
        ActionDefBodyElement::Dependency(d) => {
            super::requirement::emit_dependency(w, path, &d.value)
        }
        ActionDefBodyElement::TerminateStmt(terminate) => emit_terminate_stmt(w, &terminate.value),
    }
}

fn emit_action_usage_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &ActionUsageBody,
) -> Result<(), EmitError> {
    match body {
        ActionUsageBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        ActionUsageBody::Brace { elements, .. } => {
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
        ActionUsageBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
        ActionUsageBodyElement::Import(import) => super::root::emit_import(w, &import.value),
        ActionUsageBodyElement::AttributeUsage(a) => {
            structure::emit_attribute_usage(w, path, &a.value)
        }
        ActionUsageBodyElement::CalcUsage(c) => super::view::emit_calc_usage(w, path, &c.value),
        ActionUsageBodyElement::ActionDef(d) => emit_action_def(w, path, &d.value),
        ActionUsageBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
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
        ActionUsageBodyElement::GuardedSuccession(succession) => {
            emit_guarded_succession(w, path, &succession.value)
        }
        ActionUsageBodyElement::FirstStmt(f) => emit_first_stmt(w, path, &f.value),
        ActionUsageBodyElement::MergeStmt(m) => emit_merge_stmt(w, path, &m.value),
        ActionUsageBodyElement::DecisionStmt(d) => emit_decision_stmt(w, path, &d.value),
        ActionUsageBodyElement::JoinStmt(j) => emit_join_stmt(w, path, &j.value),
        ActionUsageBodyElement::ForkStmt(f) => emit_fork_stmt(w, path, &f.value),
        ActionUsageBodyElement::LoopStmt(l) => {
            emit_action_node_prefix(w, path, &l.value.prefix)?;
            w.push_str("loop");
            emit_action_body_parameter(w, path, &l.value.body)?;
            emit_until_parameter(w, l.value.until.as_ref())
        }
        ActionUsageBodyElement::WhileStmt(wh) => {
            emit_action_node_prefix(w, path, &wh.value.prefix)?;
            w.push_str("while ");
            emit_expression(w, &wh.value.condition.value)?;
            emit_action_body_parameter(w, path, &wh.value.body)?;
            emit_until_parameter(w, wh.value.until.as_ref())
        }
        ActionUsageBodyElement::IfStmt(i) => {
            w.push_str("if ");
            emit_expression(w, &i.value.condition.value)?;
            emit_action_branch_body(w, path, &i.value.then_body)?;
            if let Some(else_body) = &i.value.else_body {
                w.push_str(" else");
                emit_action_branch_body(w, path, else_body)?;
            }
            Ok(())
        }
        ActionUsageBodyElement::Transition(transition) => {
            emit_transition(w, path, &transition.value)
        }
        ActionUsageBodyElement::VariantUsage(v) => structure::emit_variant_usage(w, path, &v.value),
        ActionUsageBodyElement::OccurrenceUsage(o) => emit_occurrence_usage(w, path, &o.value),
        ActionUsageBodyElement::MetadataKeywordUsage(m) => {
            structure::emit_metadata_keyword_usage(w, path, &m.value)
        }
        ActionUsageBodyElement::MetadataUsage(m) => {
            structure::emit_metadata_usage(w, path, &m.value)
        }
        ActionUsageBodyElement::Dependency(d) => {
            super::requirement::emit_dependency(w, path, &d.value)
        }
        ActionUsageBodyElement::TerminateStmt(terminate) => {
            emit_terminate_stmt(w, &terminate.value)
        }
        ActionUsageBodyElement::ForLoop(f) => emit_for_loop(w, path, &f.value),
    }
}

/// The currently supported semicolon form of `TerminateNode`: `terminate;` or
/// `terminate target;`. The target remains a typed expression from parsing through emission.
fn emit_terminate_stmt(
    w: &mut EmitWriter<'_>,
    terminate: &crate::ast::TerminateStmt,
) -> Result<(), EmitError> {
    w.push_str("terminate");
    if let Some(target) = &terminate.target {
        w.push_char(' ');
        emit_expression(w, &target.value)?;
    }
    w.push_char(';');
    Ok(())
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
        ThenTarget::Merge(m) => emit_merge_stmt(w, path, &m.value),
        ThenTarget::Fork(f) => emit_fork_stmt(w, path, &f.value),
        ThenTarget::Decide(d) => emit_decision_stmt(w, path, &d.value),
        ThenTarget::Join(j) => emit_join_stmt(w, path, &j.value),
        ThenTarget::Accept(a) => {
            w.push_str("accept ");
            emit_transition_accept(w, path, &a.value)?;
            w.push_char(';');
            Ok(())
        }
        ThenTarget::Send(a) => emit_action_usage(w, path, &a.value),
        ThenTarget::If(i) => {
            w.push_str("if ");
            emit_expression(w, &i.value.condition.value)?;
            emit_action_branch_body(w, path, &i.value.then_body)?;
            if let Some(else_body) = &i.value.else_body {
                w.push_str(" else");
                emit_action_branch_body(w, path, else_body)?;
            }
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
    emit_definition_prefix_value(w, perform.usage_prefix.as_ref());
    w.push_str("perform ");
    match &perform.target {
        PerformActionTarget::Action(declaration) => {
            w.push_str("action");
            emit_usage_declaration(w, path, &declaration.value)?;
        }
        PerformActionTarget::Reference { action, redefines } => {
            w.push_qualified_reference(path, *action)?;
            if let Some(redefines) = redefines {
                emit_subsetting_clause(w, &redefines.value)?;
            }
        }
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
        PerformBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        PerformBody::Brace { elements, .. } => {
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
        PerformBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
        PerformBodyElement::InOut(b) => emit_perform_inout(w, path, &b.value),
        PerformBodyElement::Variant(v) => structure::emit_variant_usage(w, path, &v.value),
        PerformBodyElement::Action(a) => emit_action_usage_body_element(w, path, &a.value),
        PerformBodyElement::PartUsage(p) => structure::emit_part_usage(w, path, &p.value),
        PerformBodyElement::ItemUsage(i) => super::requirement::emit_item_usage(w, path, &i.value),
        PerformBodyElement::AttributeUsage(a) => structure::emit_attribute_usage(w, path, &a.value),
    }
}

fn emit_perform_inout(
    w: &mut EmitWriter<'_>,
    path: &str,
    binding: &PerformInOutBinding,
) -> Result<(), EmitError> {
    emit_direction(w, binding.direction);
    w.push_qualified_reference(&format!("{path}/target"), binding.target)?;
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
    emit_definition_prefix(w, def.definition_prefix.as_ref());
    if def.is_individual {
        w.push_str("individual ");
    }
    w.push_str("state def ");
    emit_identification(w, &def.identification)?;
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
    if let Some(reference) = usage.state_reference {
        w.push_qualified_reference(&format!("{path}/state"), reference)?;
    } else if let Some(name) = usage.name {
        w.push_declaration_name(&format!("{path}/name"), name)?;
    }
    if let Some(typing) = &usage.typing {
        emit_typing_clause(w, &typing.value)?;
    } else if let Some(ty) = usage.type_name {
        w.push_str(" : ");
        w.push_qualified_reference(path, ty)?;
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    super::structure::emit_multiplicity_modifiers(w, &usage.multiplicity_modifiers);
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
    if let Some(reference) = exhibit.state_reference {
        w.push_qualified_reference(&format!("{path}/state"), reference)?;
    } else if let Some(name) = exhibit.name {
        w.push_str("state ");
        w.push_declaration_name(&format!("{path}/name"), name)?;
    }
    if let Some(typing) = &exhibit.typing {
        emit_typing_clause(w, &typing.value)?;
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
        StateDefBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        StateDefBody::Brace { elements, .. } => {
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
        StateDefBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
        StateDefBodyElement::InOutDecl(d) => emit_inout_decl(w, path, &d.value),
        StateDefBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
        StateDefBodyElement::Entry(e) => {
            w.push_str("entry");
            if let Some(effect) = &e.value.effect {
                w.push_char(' ');
                emit_transition_effect(w, path, effect)?;
            }
            if let Some(declared_name) = e.value.declared_name {
                if e.value.has_action_keyword {
                    w.push_str(" action ");
                } else {
                    w.push_char(' ');
                }
                w.push_declaration_name(&format!("{path}/name"), declared_name)?;
                if let Some(type_name) = e.value.type_name {
                    w.push_str(" : ");
                    w.push_qualified_reference(&format!("{path}/entry/type"), type_name)?;
                }
                if let Some(redefines) = &e.value.redefines {
                    emit_subsetting_clause(w, &redefines.value)?;
                }
            }
            if let Some(reference) = e.value.action_reference {
                if e.value.has_action_keyword {
                    w.push_str(" action ");
                } else {
                    w.push_char(' ');
                }
                w.push_qualified_reference(&format!("{path}/entry/action"), reference)?;
            }
            emit_state_def_body(w, path, &e.value.body)
        }
        StateDefBodyElement::Do(d) => {
            w.push_str("do");
            if let Some(effect) = &d.value.effect {
                w.push_char(' ');
                emit_transition_effect(w, path, effect)?;
            }
            if let Some(declared_name) = d.value.declared_name {
                if d.value.has_action_keyword {
                    w.push_str(" action ");
                } else {
                    w.push_char(' ');
                }
                w.push_declaration_name(&format!("{path}/name"), declared_name)?;
                if let Some(type_name) = d.value.type_name {
                    w.push_str(" : ");
                    w.push_qualified_reference(&format!("{path}/do/type"), type_name)?;
                }
                if let Some(redefines) = &d.value.redefines {
                    emit_subsetting_clause(w, &redefines.value)?;
                }
            }
            if let Some(reference) = d.value.action_reference {
                if d.value.has_action_keyword {
                    w.push_str(" action ");
                } else {
                    w.push_char(' ');
                }
                w.push_qualified_reference(&format!("{path}/do/action"), reference)?;
            }
            emit_state_def_body(w, path, &d.value.body)
        }
        StateDefBodyElement::Exit(e) => {
            w.push_str("exit");
            if let Some(effect) = &e.value.effect {
                w.push_char(' ');
                emit_transition_effect(w, path, effect)?;
            }
            if let Some(declared_name) = e.value.declared_name {
                if e.value.has_action_keyword {
                    w.push_str(" action ");
                } else {
                    w.push_char(' ');
                }
                w.push_declaration_name(&format!("{path}/name"), declared_name)?;
                if let Some(type_name) = e.value.type_name {
                    w.push_str(" : ");
                    w.push_qualified_reference(&format!("{path}/exit/type"), type_name)?;
                }
                if let Some(redefines) = &e.value.redefines {
                    emit_subsetting_clause(w, &redefines.value)?;
                }
            }
            if let Some(reference) = e.value.action_reference {
                if e.value.has_action_keyword {
                    w.push_str(" action ");
                } else {
                    w.push_char(' ');
                }
                w.push_qualified_reference(&format!("{path}/exit/action"), reference)?;
            }
            emit_state_def_body(w, path, &e.value.body)
        }
        StateDefBodyElement::Then(t) => {
            w.push_str("then ");
            w.push_qualified_reference(&format!("{path}/then/state"), t.value.state_reference)?;
            w.push_char(';');
            Ok(())
        }
        StateDefBodyElement::FinalState(f) => {
            w.push_str("final ");
            w.push_declaration_name(&format!("{path}/final-name"), f.value.state_name)?;
            w.push_char(';');
            Ok(())
        }
        StateDefBodyElement::Ref(r) => structure::emit_ref_decl(w, path, &r.value),
        StateDefBodyElement::StateUsage(s) => emit_state_usage(w, path, &s.value),
        StateDefBodyElement::RequirementUsage(r) => {
            super::requirement::emit_requirement_usage(w, path, &r.value)
        }
        StateDefBodyElement::Transition(t) => emit_transition(w, path, &t.value),
        StateDefBodyElement::AttributeUsage(a) => {
            structure::emit_attribute_usage(w, path, &a.value)
        }
        StateDefBodyElement::ActionUsage(a) => emit_action_usage(w, path, &a.value),
        StateDefBodyElement::SuccessionUsage(s) => emit_succession_usage(w, path, &s.value),
        StateDefBodyElement::AssertConstraint(a) => {
            super::view::emit_assert_constraint(w, path, &a.value)
        }
        StateDefBodyElement::PartUsage(p) => structure::emit_part_usage(w, path, &p.value),
        StateDefBodyElement::ConstraintUsage(c) => {
            super::view::emit_constraint_usage(w, path, &c.value)
        }
        StateDefBodyElement::MetadataKeywordUsage(m) => {
            structure::emit_metadata_keyword_usage(w, path, &m.value)
        }
    }
}

pub(crate) fn emit_allocate(
    w: &mut EmitWriter<'_>,
    path: &str,
    allocate: &Allocate,
) -> Result<(), EmitError> {
    w.push_str("allocate ");
    emit_expression(w, &allocate.source.value)?;
    w.push_str(" to ");
    emit_expression(w, &allocate.target.value)?;
    super::structure::emit_part_usage_body_public(w, path, &allocate.body)
}

pub(crate) fn emit_allocation_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::AllocationDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    emit_definition_prefix(w, def.definition_prefix.as_ref());
    if def.is_individual {
        w.push_str("individual ");
    }
    w.push_str("allocation def ");
    emit_identification(w, &def.identification)?;
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_definition_body(w, path, &def.body)
}

/// `FlowDefinition = OccurrenceDefinitionPrefix ( 'flow' | 'message' ) 'def' Definition`, whose
/// body is a `DefinitionBody` -- the same shape `emit_allocation_def` writes. It parsed into a
/// complete typed node in three scopes and was reported as an unsupported construct by all three,
/// so a document containing one could not be formatted at all.
pub(crate) fn emit_flow_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::FlowDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    emit_definition_prefix(w, def.definition_prefix.as_ref());
    if def.is_individual {
        w.push_str("individual ");
    }
    w.push_str("flow def ");
    emit_identification(w, &def.identification)?;
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
    let shorthand = usage.name.is_none() && usage.type_name.is_none();
    if shorthand {
        w.push_str("allocate ");
        let (Some(source), Some(target)) = (&usage.source, &usage.target) else {
            return w.unsupported(path, "allocate shorthand without ends");
        };
        super::view::emit_kerml_connector_end(w, path, &source.value)?;
        w.push_str(" to ");
        super::view::emit_kerml_connector_end(w, path, &target.value)?;
        return emit_definition_body(w, path, &usage.body);
    }
    w.push_str("allocation ");
    if let Some(name) = usage.name {
        w.push_declaration_name(&format!("{path}/name"), name)?;
    }
    if let Some(ty) = usage.type_name {
        w.push_str(" : ");
        if usage.type_is_conjugated {
            w.push_char('~');
        }
        w.push_qualified_reference(path, ty)?;
    }
    if let (Some(source), Some(target)) = (&usage.source, &usage.target) {
        w.push_str(" allocate ");
        super::view::emit_kerml_connector_end(w, path, &source.value)?;
        w.push_str(" to ");
        super::view::emit_kerml_connector_end(w, path, &target.value)?;
    }
    emit_definition_body(w, path, &usage.body)
}

pub(crate) fn emit_transition_effect(
    w: &mut EmitWriter<'_>,
    path: &str,
    effect: &crate::ast::TransitionEffect,
) -> Result<(), EmitError> {
    match effect {
        crate::ast::TransitionEffect::Perform {
            name,
            type_name,
            body,
        } => {
            w.push_str("action");
            if let Some(n) = *name {
                w.push_char(' ');
                w.push_declaration_name(&format!("{path}/name"), n)?;
            }
            if let Some(ty) = type_name {
                w.push_str(" : ");
                w.push_qualified_reference(path, *ty)?;
            }
            if let Some(body) = body {
                emit_action_def_body(w, path, body)?;
            }
        }
        crate::ast::TransitionEffect::Accept {
            payload,
            type_name,
            via,
            body,
        } => {
            w.push_str("accept ");
            emit_expression(w, &payload.value)?;
            if let Some(ty) = type_name {
                w.push_str(" : ");
                w.push_qualified_reference(path, *ty)?;
            }
            if let Some(v) = via {
                w.push_str(" via ");
                emit_expression(w, &v.value)?;
            }
            if let Some(body) = body {
                emit_action_def_body(w, path, body)?;
            }
        }
        crate::ast::TransitionEffect::Send {
            payload,
            type_name,
            via,
            to,
            body,
        } => {
            w.push_str("send ");
            emit_expression(w, &payload.value)?;
            if let Some(ty) = type_name {
                w.push_str(" : ");
                w.push_qualified_reference(path, *ty)?;
            }
            if let Some(v) = via {
                w.push_str(" via ");
                emit_expression(w, &v.value)?;
            }
            if let Some(t) = to {
                w.push_str(" to ");
                emit_expression(w, &t.value)?;
            }
            if let Some(body) = body {
                emit_action_def_body(w, path, body)?;
            }
        }
        crate::ast::TransitionEffect::Assign { lhs, rhs, body } => {
            w.push_str("assign ");
            emit_expression(w, &lhs.value)?;
            w.push_str(" := ");
            emit_expression(w, &rhs.value)?;
            if let Some(body) = body {
                emit_action_def_body(w, path, body)?;
            }
        }
        crate::ast::TransitionEffect::Expression(e) => emit_expression(w, &e.value)?,
    }
    Ok(())
}

pub(crate) fn emit_flow_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    flow: &crate::ast::FlowUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, flow.membership.visibility);
    match flow.kind {
        crate::ast::FlowUsageKind::Flow => w.push_str("flow"),
        crate::ast::FlowUsageKind::Message => w.push_str("message"),
        crate::ast::FlowUsageKind::SuccessionFlow => w.push_str("succession flow"),
    }
    match &flow.declaration {
        crate::ast::FlowDeclaration::Declared {
            declaration,
            value,
            payloads,
            endpoints,
        } => {
            let declaration = &declaration.value;
            if declaration.identification_span.len != 0 {
                w.push_char(' ');
                w.push_authored_name(
                    &format!("{path}/declaration/identification"),
                    &declaration.identification_span,
                )?;
            }
            if let Some(typing) = &declaration.typing {
                emit_typing_clause(w, &typing.value)?;
            }
            if let Some(multiplicity) = &declaration.multiplicity {
                emit_multiplicity(w, &multiplicity.value)?;
            }
            emit_multiplicity_modifiers(w, &declaration.multiplicity_modifiers);
            if let Some((subsets, value)) = &declaration.subsets {
                emit_subsetting_clause(w, &subsets.value)?;
                if let Some(value) = value {
                    w.push_str(" = ");
                    emit_expression(w, &value.value)?;
                }
            }
            for relationship in [
                declaration.redefines.as_ref(),
                declaration.references.as_ref(),
                declaration.crosses.as_ref(),
                declaration.intersects.as_ref(),
            ]
            .into_iter()
            .flatten()
            {
                emit_subsetting_clause(w, &relationship.value)?;
            }
            if let Some(value) = value {
                emit_feature_value(w, value)?;
            }
            for (index, payload) in payloads.iter().enumerate() {
                w.push_str(" of ");
                let feature = &payload.value.feature.value;
                if let Some(n) = feature.name {
                    w.push_declaration_name(&format!("{path}/payload[{index}]"), n)?;
                    if feature.type_name.is_some() || feature.multiplicity.is_some() {
                        w.push_str(" : ");
                    }
                }
                if let Some(ty) = feature.type_name {
                    if feature.type_is_conjugated {
                        w.push_char('~');
                    }
                    w.push_qualified_reference(path, ty)?;
                }
                if let Some(mult) = &feature.multiplicity {
                    emit_multiplicity(w, &mult.value)?;
                }
            }
            if let Some(endpoints) = &**endpoints {
                w.push_str(" from ");
                super::view::emit_kerml_connector_end(w, path, &endpoints.from.value)?;
                w.push_str(" to ");
                super::view::emit_kerml_connector_end(w, path, &endpoints.to.value)?;
            }
        }
        crate::ast::FlowDeclaration::EndpointOnly { endpoints } => {
            w.push_str(" from ");
            super::view::emit_kerml_connector_end(w, path, &endpoints.from.value)?;
            w.push_str(" to ");
            super::view::emit_kerml_connector_end(w, path, &endpoints.to.value)?;
        }
    }
    emit_definition_body(w, path, &flow.body)
}

pub(crate) fn emit_definition_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &crate::ast::DefinitionBody,
) -> Result<(), EmitError> {
    match body {
        crate::ast::DefinitionBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::DefinitionBody::Brace { elements, .. } => {
            if elements.is_empty() {
                w.push_str(" {}");
                Ok(())
            } else {
                w.push_str(" {");
                w.newline();
                w.indent();
                for (i, el) in elements.iter().enumerate() {
                    match &el.value {
                        crate::ast::DefinitionBodyElement::Error(error) => {
                            w.push_recovery_span(&format!("{path}/body[{i}]"), &error.span)?
                        }
                        crate::ast::DefinitionBodyElement::Unsupported(unsupported) => {
                            w.push_recovery_span(&format!("{path}/body[{i}]"), &unsupported.span)?
                        }
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
    path: &str,
    accept: &crate::ast::TransitionAccept,
) -> Result<(), EmitError> {
    match accept {
        crate::ast::TransitionAccept::Payload(p, via) => {
            w.push_declaration_name(&format!("{path}/payload"), p.name)?;
            if let Some(ty) = p.type_name {
                w.push_str(" : ");
                w.push_qualified_reference(path, ty)?;
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
    if let Some(name) = t.name {
        w.push_declaration_name(&format!("{path}/name"), name)?;
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
        emit_transition_accept(w, path, accept)?;
        w.push_char(' ');
    }
    if let Some(guard) = &t.guard {
        w.push_str("if ");
        emit_expression(w, &guard.value)?;
        w.push_char(' ');
    }
    if let Some(effect) = &t.effect {
        w.push_str("do ");
        emit_transition_effect(w, path, effect)?;
        w.push_char(' ');
    }
    w.push_str("then ");
    emit_expression(w, &t.target.value)?;
    emit_action_def_body(w, path, &t.body)
}

pub(crate) fn emit_first_stmt(
    w: &mut EmitWriter<'_>,
    path: &str,
    first: &crate::ast::FirstStmt,
) -> Result<(), EmitError> {
    if first.succession_name.is_some()
        || first.succession_type.is_some()
        || first.succession_multiplicity.is_some()
    {
        w.push_str("succession ");
        if let Some(mult) = &first.succession_multiplicity {
            emit_multiplicity(w, &mult.value)?;
            w.push_char(' ');
        }
        if let Some(name) = first.succession_name {
            w.push_declaration_name(&format!("{path}/succession-name"), name)?;
            w.push_char(' ');
        }
        if let Some(ty) = first.succession_type {
            w.push_str(": ");
            w.push_qualified_reference("first succession type", ty)?;
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
    emit_first_merge_body(w, path, &first.body)
}

/// Emit the action-only `GuardedSuccession` production from its grammar-owned parts.
///
/// This deliberately does not reuse `FirstStmt`: its source is an arena-backed feature-chain
/// reference and its target is a typed connector end, not expression placeholders.
pub(crate) fn emit_guarded_succession(
    w: &mut EmitWriter<'_>,
    path: &str,
    succession: &crate::ast::GuardedSuccession,
) -> Result<(), EmitError> {
    if let Some(declaration) = &succession.succession {
        w.push_str("succession");
        let declaration = &declaration.declaration.value;
        if declaration.identification_span.len != 0 {
            w.push_char(' ');
            w.push_authored_name(
                &format!("{path}/succession-declaration/identification"),
                &declaration.identification_span,
            )?;
        }
        if let Some(typing) = &declaration.typing {
            emit_typing_clause(w, &typing.value)?;
        }
        if let Some(multiplicity) = &declaration.multiplicity {
            emit_multiplicity(w, &multiplicity.value)?;
        }
        emit_multiplicity_modifiers(w, &declaration.multiplicity_modifiers);
        if let Some((subsets, value)) = &declaration.subsets {
            emit_subsetting_clause(w, &subsets.value)?;
            if let Some(value) = value {
                w.push_str(" = ");
                emit_expression(w, &value.value)?;
            }
        }
        for relationship in [
            declaration.redefines.as_ref(),
            declaration.references.as_ref(),
            declaration.crosses.as_ref(),
            declaration.intersects.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            emit_subsetting_clause(w, &relationship.value)?;
        }
        w.push_char(' ');
    }
    w.push_str("first ");
    w.push_qualified_reference(&format!("{path}/first"), succession.first)?;
    w.push_str(" if ");
    emit_expression(w, &succession.guard.value)?;
    w.push_str(" then ");
    super::view::emit_kerml_connector_end(w, &format!("{path}/target"), &succession.target.value)?;
    emit_definition_body(w, path, &succession.body)
}

fn emit_first_merge_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &crate::ast::FirstMergeBody,
) -> Result<(), EmitError> {
    match body {
        crate::ast::FirstMergeBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::FirstMergeBody::Brace { elements, .. } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (index, element) in elements.iter().enumerate() {
                let element_path = format!("{path}/body[{index}]");
                match &element.value {
                    crate::ast::FirstMergeBodyElement::Member(member) => {
                        emit_action_def_body_element(w, &element_path, &member.value)?;
                    }
                    crate::ast::FirstMergeBodyElement::Unsupported(unsupported) => {
                        w.push_recovery_span(&element_path, &unsupported.span)?;
                    }
                    crate::ast::FirstMergeBodyElement::Error(error) => {
                        w.push_recovery_span(&element_path, &error.span)?;
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

fn emit_merge_stmt(
    w: &mut EmitWriter<'_>,
    path: &str,
    merge: &crate::ast::MergeStmt,
) -> Result<(), EmitError> {
    emit_control_node(w, path, "merge", &merge.declaration, &merge.body)
}

fn emit_decision_stmt(
    w: &mut EmitWriter<'_>,
    path: &str,
    decision: &crate::ast::DecisionStmt,
) -> Result<(), EmitError> {
    emit_control_node(w, path, "decide", &decision.declaration, &decision.body)
}

fn emit_join_stmt(
    w: &mut EmitWriter<'_>,
    path: &str,
    join: &crate::ast::JoinStmt,
) -> Result<(), EmitError> {
    emit_control_node(w, path, "join", &join.declaration, &join.body)
}

fn emit_fork_stmt(
    w: &mut EmitWriter<'_>,
    path: &str,
    fork: &crate::ast::ForkStmt,
) -> Result<(), EmitError> {
    emit_control_node(w, path, "fork", &fork.declaration, &fork.body)
}

/// All four `ControlNode` productions share an empty-or-named declaration followed by their
/// mandatory action body. Keep declaration emission typed so anonymous controls do not acquire a
/// fabricated name or spacing before `;` / `{`.
fn emit_control_node(
    w: &mut EmitWriter<'_>,
    path: &str,
    keyword: &str,
    declaration: &crate::ast::ControlNodeDeclaration,
    body: &crate::ast::FirstMergeBody,
) -> Result<(), EmitError> {
    w.push_str(keyword);
    match declaration {
        crate::ast::ControlNodeDeclaration::Anonymous => {}
        crate::ast::ControlNodeDeclaration::Named(name) => {
            w.push_char(' ');
            emit_expression(w, &name.value)?;
        }
    }
    emit_first_merge_body(w, path, body)
}

pub(crate) fn emit_occurrence_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::OccurrenceDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    emit_definition_prefix(w, def.definition_prefix.as_ref());
    if def.is_individual {
        w.push_str("individual ");
    }
    w.push_str("occurrence def ");
    emit_identification(w, &def.identification)?;
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
    // `SourceSuccessionMember` precedes the membership, which precedes the usage's own prefix.
    if usage.then_span.is_some() {
        w.push_str("then ");
    }
    emit_visibility(w, usage.membership.visibility);
    super::structure::emit_occurrence_usage_prefix(w, path, &usage.prefix)?;
    if usage.is_event {
        w.push_str("event ");
    }
    if usage.has_occurrence_keyword {
        // Plain `occurrence …`, `event occurrence …`, and `individual occurrence …` (gap #7) all
        // authored the literal keyword; bare `individual <name>` did not (see `individual_usage`
        // → `occurrence_usage_tail`).
        w.push_str("occurrence ");
    }
    w.push_short_name_prefix(&format!("{path}/short-name"), usage.short_name)?;
    if let Some(reference) = usage.occurrence_reference {
        w.push_qualified_reference(&format!("{path}/occurrence"), reference)?;
    } else if let Some(name) = usage.name {
        w.push_declaration_name(&format!("{path}/name"), name)?;
    }
    // `ref individual :>> vehicleUnderTest : TestVehicle1` declares no label, so the prefix's
    // trailing space has nothing to separate and the clause below brings its own.
    w.trim_trailing_space();
    if let Some(ty) = usage.type_name {
        w.push_str(" : ");
        if usage.type_is_conjugated {
            w.push_char('~');
        }
        w.push_qualified_reference(path, ty)?;
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
    if let Some(value) = &usage.value {
        emit_feature_value(w, value)?;
    }
    match &usage.body {
        crate::ast::OccurrenceUsageBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::OccurrenceUsageBody::Brace { elements, .. } => {
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
        crate::ast::OccurrenceBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
        crate::ast::OccurrenceBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
        crate::ast::OccurrenceBodyElement::AssertConstraint(a) => {
            super::view::emit_assert_constraint(w, path, &a.value)
        }
        crate::ast::OccurrenceBodyElement::FlowUsage(f) => emit_flow_usage(w, path, &f.value),
        crate::ast::OccurrenceBodyElement::Bind(b) => structure::emit_bind(w, path, &b.value),
        crate::ast::OccurrenceBodyElement::AttributeUsage(a) => {
            structure::emit_attribute_usage(w, path, &a.value)
        }
        crate::ast::OccurrenceBodyElement::PartUsage(p) => {
            structure::emit_part_usage(w, path, &p.value)
        }
        crate::ast::OccurrenceBodyElement::ItemUsage(i) => {
            super::requirement::emit_item_usage(w, path, &i.value)
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
        crate::ast::OccurrenceBodyElement::RefDecl(r) => {
            crate::emit::structure::emit_ref_decl(w, path, &r.value)
        }
        crate::ast::OccurrenceBodyElement::ConnectionUsage(c) => {
            crate::emit::structure::emit_connection_usage(w, path, &c.value)
        }
        crate::ast::OccurrenceBodyElement::StateUsage(s) => {
            // Occurrence-body `StateUsage` nodes are exhibit usages (§6 G30 / G18).
            emit_occurrence_exhibit(w, path, &s.value)
        }
        crate::ast::OccurrenceBodyElement::SuccessionUsage(s) => {
            emit_succession_usage(w, path, &s.value)
        }
        crate::ast::OccurrenceBodyElement::MetadataKeywordUsage(m) => {
            structure::emit_metadata_keyword_usage(w, path, &m.value)
        }
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
    if let Some(reference) = usage.state_reference {
        w.push_qualified_reference(&format!("{path}/state"), reference)?;
    } else if let Some(name) = usage.name {
        w.push_declaration_name(&format!("{path}/name"), name)?;
    }
    if let Some(typing) = &usage.typing {
        emit_typing_clause(w, &typing.value)?;
    } else if let Some(ty) = usage.type_name {
        w.push_str(" : ");
        w.push_qualified_reference(path, ty)?;
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

pub(crate) fn emit_succession_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    succ: &crate::ast::SuccessionUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, succ.membership.visibility);
    if succ.succession_keyword_span.is_some() {
        w.push_str("succession ");
    }
    if let Some(mult) = &succ.multiplicity {
        emit_multiplicity(w, &mult.value)?;
        w.push_char(' ');
    }
    if let Some(name) = succ.name {
        w.push_declaration_name(&format!("{path}/name"), name)?;
        w.push_char(' ');
    }
    if let Some(type_name) = succ.type_name {
        w.push_str(": ");
        w.push_qualified_reference("succession usage type", type_name)?;
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
    super::structure::emit_part_usage_body_public(w, path, &succ.body)
}
