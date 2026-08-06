//! Requirement / use-case / dependency / item usage emission.

use super::expr::{emit_expression, emit_feature_value};
use super::root::{emit_doc, emit_identification, emit_import};
use super::structure::{
    self, emit_attribute_body, emit_direction, emit_multiplicity, emit_subsetting_clause,
    emit_typing_clause,
};
use super::writer::{emit_visibility, format_name, format_qualified_name, EmitWriter};
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
    if let Some(dir) = usage.direction {
        emit_direction(w, dir);
    }
    if usage.is_abstract {
        w.push_str("abstract ");
    }
    if usage.is_variation {
        w.push_str("variation ");
    }
    w.push_str("requirement ");
    if let Some(short) = &usage.short_name {
        w.push_char('<');
        w.push_str(&format_name(short));
        w.push_str("> ");
    }
    if !usage.name.is_empty() {
        w.push_str(&format_name(&usage.name));
    }
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    if let Some(subsets) = &usage.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(references) = &usage.references {
        emit_subsetting_clause(w, &references.value)?;
    }
    if let Some(value) = &usage.value {
        emit_feature_value(w, value)?;
    }
    emit_requirement_body(w, path, &usage.body)
}

pub(crate) fn emit_requirement_body_pub(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &RequirementDefBody,
) -> Result<(), EmitError> {
    emit_requirement_body(w, path, body)
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
            // Keyword-less `:>> name = …` bindings store the redefines target as the usage name.
            // Prefer the original shorthand when that is the only specialization present.
            if a.value.redefines.is_some()
                && a.value.subsets.is_none()
                && a.value.references.is_none()
                && a.value.direction.is_none()
                && !a.value.is_end
                && a.value.short_name.is_none()
                && a.value
                    .redefines
                    .as_ref()
                    .is_some_and(|r| r.value.target_display() == a.value.name)
            {
                emit_redefinition_attribute_binding(w, path, &a.value)
            } else {
                structure::emit_attribute_usage(w, path, &a.value)
            }
        }
        RequirementDefBodyElement::VariantUsage(v) => {
            structure::emit_variant_usage(w, path, &v.value)
        }
        RequirementDefBodyElement::RequirementUsage(r) => emit_requirement_usage(w, path, &r.value),
        RequirementDefBodyElement::SubjectDecl(s) => emit_subject_decl(w, &s.value),
        RequirementDefBodyElement::SubjectRef(_) => {
            w.push_str("subject;");
            Ok(())
        }
        RequirementDefBodyElement::RequirementActorDecl(a) => {
            w.push_str("actor ");
            if !a.value.name.is_empty() {
                w.push_str(&format_name(&a.value.name));
            }
            if !a.value.type_name.is_empty() {
                w.push_str(" : ");
                w.push_str(&format_qualified_name(&a.value.type_name));
            }
            w.push_char(';');
            Ok(())
        }
        RequirementDefBodyElement::RequireConstraint(r) => {
            emit_require_constraint(w, path, &r.value)
        }
        RequirementDefBodyElement::Constraint(c) => {
            super::view::emit_constraint_usage(w, path, &c.value)
        }
        RequirementDefBodyElement::Stakeholder(s) => {
            w.push_str("stakeholder ");
            if s.value.is_redefinition {
                w.push_str(":>> ");
            }
            w.push_str(&format_name(&s.value.name));
            if let Some(ty) = &s.value.type_name {
                w.push_str(" : ");
                w.push_str(&format_qualified_name(ty));
            }
            w.push_char(';');
            Ok(())
        }
        RequirementDefBodyElement::Purpose(p) => {
            w.push_str("purpose ");
            w.push_str(&format_qualified_name(&p.value.target));
            w.push_char(';');
            Ok(())
        }
        RequirementDefBodyElement::Frame(f) => {
            w.push_str("frame ");
            w.push_str(&format_name(&f.value.name));
            emit_requirement_body(w, path, &f.value.body)
        }
        RequirementDefBodyElement::VerifyRequirement(v) => {
            emit_verify_requirement(w, path, &v.value)
        }
        RequirementDefBodyElement::Annotation(a) => emit_requirement_annotation(w, &a.value),
        RequirementDefBodyElement::MetadataAnnotation(a) => {
            structure::emit_metadata_annotation(w, path, &a.value)
        }
        RequirementDefBodyElement::MetadataKeywordUsage(m) => {
            structure::emit_metadata_keyword_usage(w, path, &m.value)
        }
        RequirementDefBodyElement::TextualRep(t) => emit_requirement_textual_rep(w, &t.value),
    }
}

fn emit_requirement_annotation(
    w: &mut EmitWriter<'_>,
    ann: &crate::ast::Annotation,
) -> Result<(), EmitError> {
    w.push_str(&ann.sigil);
    w.push_str(&format_name(&ann.head));
    if let Some(ty) = &ann.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    match &ann.body {
        ConnectBody::Semicolon => w.push_char(';'),
        ConnectBody::Brace => w.push_str(" {}"),
    }
    Ok(())
}

fn emit_requirement_textual_rep(
    w: &mut EmitWriter<'_>,
    rep: &crate::ast::TextualRepresentation,
) -> Result<(), EmitError> {
    if let Some(id) = &rep.rep_identification {
        w.push_str("rep ");
        emit_identification(w, id);
        w.push_char(' ');
    }
    w.push_str("language \"");
    w.push_str(&rep.language);
    w.push_char('"');
    w.newline();
    w.push_str("/*");
    w.push_str(&rep.text);
    w.push_str("*/");
    Ok(())
}

pub(crate) fn emit_redefinition_attribute_binding(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &crate::ast::AttributeUsage,
) -> Result<(), EmitError> {
    w.push_str(":>> ");
    w.push_str(&format_name(&usage.name));
    if let Some(typing) = &usage.typing {
        emit_typing_clause(w, &typing.value)?;
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if usage.ordered {
        w.push_str(" ordered");
    }
    if usage.nonunique {
        w.push_str(" nonunique");
    }
    if let Some(value) = &usage.value {
        emit_feature_value(w, value)?;
    }
    emit_attribute_body(w, path, &usage.body)
}

fn emit_verify_requirement(
    w: &mut EmitWriter<'_>,
    path: &str,
    v: &crate::ast::VerifyRequirementMember,
) -> Result<(), EmitError> {
    w.push_str("verify ");
    if v.explicit_requirement_keyword {
        w.push_str("requirement ");
        if let Some(req) = &v.requirement {
            // Avoid double `requirement` keyword from emit_requirement_usage.
            let usage = &req.value;
            if let Some(short) = &usage.short_name {
                w.push_char('<');
                w.push_str(&format_name(short));
                w.push_str("> ");
            }
            if !usage.name.is_empty() {
                w.push_str(&format_name(&usage.name));
            }
            if let Some(ty) = &usage.type_name {
                w.push_str(" : ");
                w.push_str(&format_qualified_name(ty));
            }
            if let Some(subsets) = &usage.subsets {
                emit_subsetting_clause(w, &subsets.value)?;
            }
            if let Some(references) = &usage.references {
                emit_subsetting_clause(w, &references.value)?;
            }
            return emit_requirement_body(w, path, &usage.body);
        }
        w.push_char(';');
        return Ok(());
    }
    if let Some(target) = &v.target {
        w.push_str(&format_qualified_name(target));
    }
    if let Some(redefines) = &v.redefines {
        w.push_str(" :>> ");
        w.push_str(&format_qualified_name(redefines));
    }
    w.push_char(';');
    Ok(())
}

fn emit_subject_decl(w: &mut EmitWriter<'_>, subject: &SubjectDecl) -> Result<(), EmitError> {
    w.push_str("subject ");
    if !subject.name.is_empty() {
        w.push_str(&format_name(&subject.name));
    }
    if !subject.type_name.is_empty() {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(&subject.type_name));
    }
    if let Some(mult) = &subject.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if let Some(value) = &subject.value {
        w.push_str(" = ");
        emit_expression(w, &value.value)?;
    }
    w.push_char(';');
    Ok(())
}

fn emit_require_constraint(
    w: &mut EmitWriter<'_>,
    path: &str,
    req: &RequireConstraint,
) -> Result<(), EmitError> {
    if req.is_assume {
        w.push_str("assume");
    } else {
        w.push_str("require");
    }
    if req.has_constraint_keyword {
        w.push_str(" constraint");
    }
    if let Some(name) = &req.name {
        w.push_char(' ');
        w.push_str(&format_name(name));
    }
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
        w.push_str(&format_qualified_name(ty));
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
        w.push_str(&format_qualified_name(ty));
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
        w.push_str(&format_qualified_name(ty));
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
        w.push_str(&format_qualified_name(ty));
    }
    emit_use_case_body(w, path, &usage.body)
}

pub(crate) fn emit_verification_case_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::VerificationCaseDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    if def.is_abstract {
        w.push_str("abstract ");
    }
    w.push_str("verification def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_use_case_body(w, path, &def.body)
}

pub(crate) fn emit_verification_case_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &crate::ast::VerificationCaseUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if usage.is_abstract {
        w.push_str("abstract ");
    }
    w.push_str("verification ");
    w.push_str(&format_name(&usage.name));
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    emit_use_case_body(w, path, &usage.body)
}

pub(crate) fn emit_case_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::CaseDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    if def.is_abstract {
        w.push_str("abstract ");
    }
    w.push_str("case def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_use_case_body(w, path, &def.body)
}

pub(crate) fn emit_case_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &crate::ast::CaseUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if usage.is_abstract {
        w.push_str("abstract ");
    }
    w.push_str("case ");
    w.push_str(&format_name(&usage.name));
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
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
                w.push_str(&format_qualified_name(&a.value.type_name));
            }
            if let Some(mult) = &a.value.multiplicity {
                emit_multiplicity(w, &mult.value)?;
            }
            w.push_char(';');
            Ok(())
        }
        UseCaseDefBodyElement::FirstSuccession(f) => {
            w.push_str("first ");
            w.push_str(&format_qualified_name(&f.value.target));
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
            let req = &o.value.requirement.value;
            // `objective { … }` stores a synthetic name `"objective"` from the payload default;
            // do not reprint `requirement objective`.
            if req.name != "objective" && !req.name.is_empty() {
                w.push_str(&format_name(&req.name));
                w.push_char(' ');
            }
            if let Some(ty) = &req.type_name {
                w.push_str(": ");
                w.push_str(&format_qualified_name(ty));
                w.push_char(' ');
            }
            if let Some(subsets) = &req.subsets {
                emit_subsetting_clause(w, &subsets.value)?;
            }
            if let Some(references) = &req.references {
                emit_subsetting_clause(w, &references.value)?;
            }
            emit_requirement_body(w, path, &req.body)
        }
        UseCaseDefBodyElement::AttributeDef(a) => structure::emit_attribute_def(w, path, &a.value),
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
        UseCaseDefBodyElement::ActionUsage(a) => {
            super::behavior::emit_action_usage(w, path, &a.value)
        }
        UseCaseDefBodyElement::AnalysisCaseUsage(a) => emit_analysis_case_usage(w, path, &a.value),
        UseCaseDefBodyElement::CalcUsage(c) => super::view::emit_calc_usage(w, path, &c.value),
        UseCaseDefBodyElement::AttributeUsage(a) => {
            structure::emit_attribute_usage(w, path, &a.value)
        }
        UseCaseDefBodyElement::RequirementUsage(r) => emit_requirement_usage(w, path, &r.value),
        UseCaseDefBodyElement::PartUsage(p) => structure::emit_part_usage(w, path, &p.value),
        UseCaseDefBodyElement::Expression(e) => {
            emit_expression(w, &e.value)?;
            w.push_char(';');
            Ok(())
        }
        UseCaseDefBodyElement::CaseReturnDecl(c) => emit_case_return_decl(w, &c.value),
        UseCaseDefBodyElement::ReturnRef(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::RawBodyString,
        }),
        UseCaseDefBodyElement::ActorRedefinitionAssignment(a) => {
            w.push_str("actor :>> ");
            w.push_str(&format_name(&a.value.name));
            w.push_str(" = ");
            w.push_str(a.value.rhs.trim());
            w.push_char(';');
            Ok(())
        }
        UseCaseDefBodyElement::RefRedefinition(r) => {
            w.push_str("ref :>> ");
            w.push_str(&format_name(&r.value.name));
            w.push_char(' ');
            w.push_str(r.value.body.trim());
            Ok(())
        }
        UseCaseDefBodyElement::ForLoop(f) => {
            w.push_str("for ");
            w.push_str(&format_name(&f.value.var));
            w.push_str(" in ");
            emit_expression(w, &f.value.range.value)?;
            w.push_char(' ');
            super::behavior::emit_action_def_body(w, path, &f.value.body)
        }
        UseCaseDefBodyElement::FlowUsage(f) => super::behavior::emit_flow_usage(w, path, &f.value),
        UseCaseDefBodyElement::Annotation(a) => emit_requirement_annotation(w, &a.value),
        UseCaseDefBodyElement::MetadataAnnotation(a) => {
            structure::emit_metadata_annotation(w, path, &a.value)
        }
        UseCaseDefBodyElement::MetadataKeywordUsage(m) => {
            structure::emit_metadata_keyword_usage(w, path, &m.value)
        }
    }
}

fn emit_case_return_decl(
    w: &mut EmitWriter<'_>,
    decl: &crate::ast::CaseReturnDecl,
) -> Result<(), EmitError> {
    w.push_str("return ");
    if let Some(kind) = decl.feature_kind {
        match kind {
            crate::ast::CaseReturnFeatureKind::Part => w.push_str("part "),
            crate::ast::CaseReturnFeatureKind::Attribute => w.push_str("attribute "),
        }
    }
    if decl.is_redefine {
        w.push_str(":>> ");
    }
    if !decl.name.is_empty() {
        w.push_str(&format_name(&decl.name));
    }
    if let Some(ty) = &decl.type_name {
        if decl.is_subsetting {
            w.push_str(":>");
        } else if decl.name.is_empty() {
            w.push_str(": ");
        } else {
            w.push_str(" : ");
        }
        w.push_str(&format_qualified_name(ty));
    }
    if let Some(mult) = &decl.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if let Some(value) = &decl.value {
        emit_feature_value(w, value)?;
    }
    w.push_char(';');
    Ok(())
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
        w.push_str(&format_qualified_name(ty));
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
            w.push_str(&format_qualified_name(ty));
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
