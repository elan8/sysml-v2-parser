//! Requirement / use-case / dependency / item usage emission.

use super::expr::{emit_expression, emit_feature_value};
use super::root::{emit_doc, emit_identification, emit_import};
use super::structure::{
    self, emit_attribute_body, emit_direction, emit_multiplicity, emit_subsetting_clause,
    emit_typing_clause,
};
use super::writer::{emit_visibility, format_name, EmitWriter};
use super::EmitError;
use crate::ast::{
    ConcernUsage, ConnectBody, Dependency, EnumerationUsage, ItemUsage, RelationshipBodyElement,
    RequireConstraint, RequirementDef, RequirementDefBody, RequirementDefBodyElement,
    RequirementUsage, Satisfy, SubjectDecl, UseCaseDef, UseCaseDefBody, UseCaseDefBodyElement,
    UseCaseUsage,
};

pub(crate) fn emit_requirement_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &RequirementDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    if def.is_abstract {
        w.push_str("abstract ");
    }
    w.push_str("requirement def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_requirement_body(w, path, &def.body)
}

pub(crate) fn emit_requirement_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &RequirementUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if usage.is_abstract {
        w.push_str("abstract ");
    }
    if usage.is_variation {
        w.push_str("variation ");
    }
    w.push_str("requirement ");
    if !usage.name.is_empty() {
        w.push_str(&format_name(&usage.name));
    }
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(ty);
    }
    if let Some(subsets) = &usage.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    emit_requirement_body(w, path, &usage.body)
}

fn emit_requirement_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &RequirementDefBody,
) -> Result<(), EmitError> {
    match body {
        RequirementDefBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        RequirementDefBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_requirement_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

fn emit_requirement_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &RequirementDefBodyElement,
) -> Result<(), EmitError> {
    match el {
        RequirementDefBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        RequirementDefBodyElement::Other(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::Other,
        }),
        RequirementDefBodyElement::Doc(d) => emit_doc(w, &d.value),
        RequirementDefBodyElement::Import(i) => emit_import(w, &i.value),
        RequirementDefBodyElement::AttributeDef(a) => {
            structure::emit_attribute_def(w, path, &a.value)
        }
        RequirementDefBodyElement::AttributeUsage(a) => {
            structure::emit_attribute_usage(w, path, &a.value)
        }
        RequirementDefBodyElement::RequirementUsage(r) => {
            emit_requirement_usage(w, path, &r.value)
        }
        RequirementDefBodyElement::SubjectDecl(s) => emit_subject_decl(w, &s.value),
        RequirementDefBodyElement::RequirementActorDecl(a) => {
            w.push_str("actor ");
            if !a.value.name.is_empty() {
                w.push_str(&format_name(&a.value.name));
            }
            if !a.value.type_name.is_empty() {
                w.push_str(" : ");
                w.push_str(&a.value.type_name);
            }
            w.push_char(';');
            Ok(())
        }
        RequirementDefBodyElement::RequireConstraint(r) => emit_require_constraint(w, path, &r.value),
        RequirementDefBodyElement::Constraint(c) => {
            super::view::emit_constraint_usage(w, path, &c.value)
        }
        RequirementDefBodyElement::Stakeholder(s) => {
            w.push_str("stakeholder ");
            w.push_str(&format_name(&s.value.name));
            if let Some(ty) = &s.value.type_name {
                w.push_str(" : ");
                w.push_str(ty);
            }
            w.push_char(';');
            Ok(())
        }
        RequirementDefBodyElement::Purpose(p) => {
            w.push_str("purpose ");
            w.push_str(&p.value.target);
            w.push_char(';');
            Ok(())
        }
        RequirementDefBodyElement::Frame(f) => {
            w.push_str("frame ");
            w.push_str(&format_name(&f.value.name));
            emit_requirement_body(w, path, &f.value.body)
        }
        other => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

fn emit_subject_decl(w: &mut EmitWriter<'_>, subject: &SubjectDecl) -> Result<(), EmitError> {
    w.push_str("subject ");
    if !subject.name.is_empty() {
        w.push_str(&format_name(&subject.name));
    }
    if !subject.type_name.is_empty() {
        w.push_str(" : ");
        w.push_str(&subject.type_name);
    }
    w.push_char(';');
    Ok(())
}

fn emit_require_constraint(
    w: &mut EmitWriter<'_>,
    path: &str,
    req: &RequireConstraint,
) -> Result<(), EmitError> {
    // Parser currently drops assume vs require; emit the `require` form.
    w.push_str("require constraint");
    match &req.body {
        crate::ast::RequireConstraintBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::RequireConstraintBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                super::view::emit_constraint_body_element(
                    w,
                    &format!("{path}/body[{i}]"),
                    &el.value,
                )?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

pub(crate) fn emit_dependency(
    w: &mut EmitWriter<'_>,
    path: &str,
    dep: &Dependency,
) -> Result<(), EmitError> {
    w.push_str("dependency ");
    if let Some(id) = &dep.identification {
        if id.name.is_some() || id.short_name.is_some() {
            emit_identification(w, id);
            w.push_char(' ');
        }
    }
    w.push_str("from ");
    let clients: Vec<&String> = dep.clients.iter().filter(|c| !c.is_empty()).collect();
    if clients.is_empty() {
        return w.unsupported(path, "dependency with empty clients");
    }
    for (i, c) in clients.iter().enumerate() {
        if i > 0 {
            w.push_str(", ");
        }
        w.push_str(&format_name(c));
    }
    w.push_str(" to ");
    for (i, s) in dep.suppliers.iter().enumerate() {
        if i > 0 {
            w.push_str(", ");
        }
        w.push_str(&format_name(s));
    }
    match (&dep.body, &dep.body_elements) {
        (ConnectBody::Semicolon, _) => {
            w.push_char(';');
            Ok(())
        }
        (ConnectBody::Brace, elements) => {
            let els = elements.as_deref().unwrap_or(&[]);
            if els.is_empty() {
                w.push_str(" {}");
                Ok(())
            } else {
                w.push_str(" {");
                w.newline();
                w.indent();
                for (i, el) in els.iter().enumerate() {
                    emit_rel_body(w, &format!("{path}/body[{i}]"), &el.value)?;
                    w.newline();
                }
                w.dedent();
                w.push_char('}');
                Ok(())
            }
        }
    }
}

fn emit_rel_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &RelationshipBodyElement,
) -> Result<(), EmitError> {
    match el {
        RelationshipBodyElement::Doc(d) => emit_doc(w, &d.value),
        RelationshipBodyElement::Comment(c) => super::root::emit_comment(w, &c.value),
        RelationshipBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        RelationshipBodyElement::Other(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::Other,
        }),
        other => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

pub(crate) fn emit_item_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &ItemUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if let Some(dir) = usage.direction {
        emit_direction(w, dir);
    }
    w.push_str("item ");
    if let Some(short) = &usage.short_name {
        w.push_char('<');
        w.push_str(&format_name(short));
        w.push_str("> ");
    }
    if !usage.name.is_empty() {
        w.push_str(&format_name(&usage.name));
    }
    if let Some(redefines) = &usage.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(ty);
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if let Some(value) = &usage.value {
        emit_feature_value(w, value)?;
    }
    emit_attribute_body(w, path, &usage.body)
}

pub(crate) fn emit_concern_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    concern: &ConcernUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, concern.membership.visibility);
    w.push_str("concern ");
    if concern.is_definition {
        w.push_str("def ");
    }
    w.push_str(&format_name(&concern.name));
    if let Some(ty) = &concern.type_name {
        w.push_str(" : ");
        w.push_str(ty);
    }
    emit_requirement_body(w, path, &concern.body)
}

pub(crate) fn emit_use_case_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &UseCaseDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    if def.is_abstract {
        w.push_str("abstract ");
    }
    w.push_str("use case def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_use_case_body(w, path, &def.body)
}

pub(crate) fn emit_use_case_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &UseCaseUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if usage.is_abstract {
        w.push_str("abstract ");
    }
    w.push_str("use case ");
    w.push_str(&format_name(&usage.name));
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(ty);
    }
    emit_use_case_body(w, path, &usage.body)
}

pub(crate) fn emit_analysis_case_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::AnalysisCaseDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    if def.is_abstract {
        w.push_str("abstract ");
    }
    w.push_str("analysis def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_use_case_body(w, path, &def.body)
}

pub(crate) fn emit_analysis_case_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &crate::ast::AnalysisCaseUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if usage.is_abstract {
        w.push_str("abstract ");
    }
    w.push_str("analysis ");
    w.push_str(&format_name(&usage.name));
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(ty);
    }
    emit_use_case_body(w, path, &usage.body)
}

fn emit_use_case_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &UseCaseDefBody,
) -> Result<(), EmitError> {
    match body {
        UseCaseDefBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        UseCaseDefBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_use_case_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

fn emit_use_case_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &UseCaseDefBodyElement,
) -> Result<(), EmitError> {
    match el {
        UseCaseDefBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        UseCaseDefBodyElement::Other(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::Other,
        }),
        UseCaseDefBodyElement::Doc(d) => emit_doc(w, &d.value),
        UseCaseDefBodyElement::SubjectDecl(s) => emit_subject_decl(w, &s.value),
        UseCaseDefBodyElement::SubjectRef(_) => {
            w.push_str("subject;");
            Ok(())
        }
        UseCaseDefBodyElement::ActorUsage(a) => {
            emit_visibility(w, a.value.membership.visibility);
            w.push_str("actor ");
            w.push_str(&format_name(&a.value.name));
            if !a.value.type_name.is_empty() {
                w.push_str(" : ");
                w.push_str(&a.value.type_name);
            }
            w.push_char(';');
            Ok(())
        }
        UseCaseDefBodyElement::FirstSuccession(f) => {
            w.push_str("first ");
            w.push_str(&format_name(&f.value.target));
            w.push_char(';');
            Ok(())
        }
        UseCaseDefBodyElement::ThenDone(_) => {
            w.push_str("then done;");
            Ok(())
        }
        UseCaseDefBodyElement::IncludeUseCase(i) => {
            w.push_str("include ");
            w.push_str(&format_name(&i.value.name));
            if let Some(mult) = &i.value.multiplicity {
                emit_multiplicity(w, &mult.value)?;
            }
            emit_use_case_body(w, path, &i.value.body)
        }
        UseCaseDefBodyElement::ThenIncludeUseCase(t) => {
            w.push_str("then include ");
            w.push_str(&format_name(&t.value.include.value.name));
            if let Some(mult) = &t.value.include.value.multiplicity {
                emit_multiplicity(w, &mult.value)?;
            }
            emit_use_case_body(w, path, &t.value.include.value.body)
        }
        UseCaseDefBodyElement::ThenUseCaseUsage(t) => {
            w.push_str("then ");
            emit_use_case_usage(w, path, &t.value.use_case.value)
        }
        UseCaseDefBodyElement::Objective(o) => {
            if let Some(vis) = o.value.visibility {
                emit_visibility(w, Some(vis));
            }
            w.push_str("objective ");
            emit_requirement_usage(w, path, &o.value.requirement.value)
        }
        UseCaseDefBodyElement::AttributeDef(a) => {
            structure::emit_attribute_def(w, path, &a.value)
        }
        UseCaseDefBodyElement::Assign(a) => {
            w.push_str("assign ");
            emit_expression(w, &a.value.lhs.value)?;
            w.push_str(" := ");
            emit_expression(w, &a.value.rhs.value)?;
            w.push_char(';');
            Ok(())
        }
        UseCaseDefBodyElement::ThenAction(t) => {
            super::behavior::emit_then_action_pub(w, path, &t.value)
        }
        UseCaseDefBodyElement::ReturnRef(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::RawBodyString,
        }),
        UseCaseDefBodyElement::ActorRedefinitionAssignment(a) => Err(EmitError::Opaque {
            path: format!("{path}:actor:>>{}", a.value.name),
            kind: super::OpacityKind::RawRhsString,
        }),
        UseCaseDefBodyElement::RefRedefinition(r) => Err(EmitError::Opaque {
            path: format!("{path}:ref:>>{}", r.value.name),
            kind: super::OpacityKind::RawBodyString,
        }),
        other => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

pub(crate) fn emit_enumeration_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &EnumerationUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if usage.is_end {
        w.push_str("end ");
    }
    w.push_str("enum ");
    w.push_str(&format_name(&usage.name));
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(ty);
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    emit_attribute_body(w, path, &usage.body)
}

pub(crate) fn emit_satisfy(
    w: &mut EmitWriter<'_>,
    _path: &str,
    satisfy: &Satisfy,
) -> Result<(), EmitError> {
    if satisfy.is_negated {
        w.push_str("not ");
    }
    w.push_str("satisfy ");
    if let Some(inline) = &satisfy.inline_requirement {
        w.push_str("requirement ");
        w.push_str(&format_name(&inline.name));
        if let Some(ty) = &inline.type_name {
            w.push_str(" : ");
            w.push_str(ty);
        }
        w.push_str(" by ");
        emit_expression(w, &satisfy.target.value)?;
    } else {
        emit_expression(w, &satisfy.source.value)?;
        w.push_str(" by ");
        emit_expression(w, &satisfy.target.value)?;
    }
    match &satisfy.body {
        ConnectBody::Semicolon => w.push_char(';'),
        ConnectBody::Brace => w.push_str(" {}"),
    }
    Ok(())
}
