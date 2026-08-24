//! Requirement / use-case / dependency / item usage emission.

use super::expr::{emit_expression, emit_feature_value};
use super::root::{emit_identification, emit_import};
use super::structure::{
    self, emit_attribute_body, emit_definition_prefix, emit_direction, emit_multiplicity,
    emit_multiplicity_modifiers, emit_subsetting_clause, emit_typing_clause,
};
use super::writer::{emit_visibility, format_name, EmitWriter};
use super::EmitError;
use crate::ast::{
    ConcernUsage, Dependency, EnumerationUsage, ItemUsage, RequireConstraint, RequirementDef,
    RequirementDefBody, RequirementDefBodyElement, RequirementUsage, ReturnRef, ReturnRefBody,
    ReturnRefBodyElement, SatisfiedRequirement, SatisfyRequirementUsage, SubjectDecl, UseCaseDef,
    UseCaseDefBody, UseCaseDefBodyElement, UseCaseUsage,
};

pub(crate) fn emit_requirement_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &RequirementDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    emit_definition_prefix(w, def.definition_prefix.as_ref());
    if def.is_individual {
        w.push_str("individual ");
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
        w.push_qualified_reference(&format!("{path}/type"), *ty)?;
    }
    if let Some(multiplicity) = &usage.multiplicity {
        emit_multiplicity(w, &multiplicity.value)?;
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
        RequirementDefBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        RequirementDefBody::Brace { elements, .. } => {
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
        RequirementDefBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
        RequirementDefBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
        RequirementDefBodyElement::ConcernUsage(c) => emit_concern_usage(w, path, &c.value),
        RequirementDefBodyElement::CalcUsage(c) => {
            crate::emit::view::emit_calc_usage(w, path, &c.value)
        }
        RequirementDefBodyElement::RefDecl(r) => {
            crate::emit::structure::emit_ref_decl(w, path, &r.value)
        }
        RequirementDefBodyElement::Import(i) => emit_import(w, &i.value),
        RequirementDefBodyElement::AttributeDef(a) => {
            structure::emit_attribute_def(w, path, &a.value)
        }
        RequirementDefBodyElement::AttributeUsage(a) => {
            // A keyword-less `:>> target = …` binding has no declared usage name; its target is
            // represented only by the redefinition relationship.
            if a.value.redefines.is_some()
                && a.value.subsets.is_none()
                && a.value.references.is_none()
                && a.value.direction.is_none()
                && !a.value.is_end
                && a.value.short_name.is_none()
                && a.value.name.is_empty()
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
        RequirementDefBodyElement::RequirementDef(r) => emit_requirement_def(w, path, &r.value),
        RequirementDefBodyElement::PortUsage(p) => structure::emit_port_usage(w, path, &p.value),
        RequirementDefBodyElement::AllocationUsage(a) => {
            super::behavior::emit_allocation_usage(w, path, &a.value)
        }
        RequirementDefBodyElement::Satisfy(s) => emit_satisfy(w, path, &s.value),
        // The general usage families `RequirementBodyItem` inherits from `DefinitionBodyItem`,
        // each emitted through the same writer the scope that owns it already uses.
        RequirementDefBodyElement::ActionUsage(a) => {
            super::behavior::emit_action_usage(w, path, &a.value)
        }
        RequirementDefBodyElement::SuccessionUsage(s) => {
            super::behavior::emit_succession_usage(w, path, &s.value)
        }
        RequirementDefBodyElement::Perform(p) => super::behavior::emit_perform(w, path, &p.value),
        RequirementDefBodyElement::StateUsage(s) => {
            super::behavior::emit_state_usage(w, path, &s.value)
        }
        RequirementDefBodyElement::ItemUsage(i) => emit_item_usage(w, path, &i.value),
        RequirementDefBodyElement::PartUsage(p) => {
            super::structure::emit_part_usage(w, path, &p.value)
        }
        RequirementDefBodyElement::Connect(c) => super::structure::emit_connect(w, path, &c.value),
        RequirementDefBodyElement::ConnectionUsage(c) => {
            super::structure::emit_connection_usage(w, path, &c.value)
        }
        RequirementDefBodyElement::SubjectDecl(s) => emit_subject_decl(w, &s.value),
        RequirementDefBodyElement::SubjectRef(_) => {
            w.push_str("subject;");
            Ok(())
        }
        RequirementDefBodyElement::RequirementActorDecl(a) => {
            w.push_str("actor ");
            if let Some(short) = &a.value.short_name {
                w.push_char('<');
                w.push_str(&format_name(short));
                w.push_str("> ");
            }
            if !a.value.name.is_empty() {
                w.push_str(&format_name(&a.value.name));
            }
            w.push_str(" : ");
            w.push_qualified_reference(&format!("{path}/actor/type"), a.value.type_name)?;
            if let Some(multiplicity) = &a.value.multiplicity {
                emit_multiplicity(w, &multiplicity.value)?;
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
            if let Some(target) = s.value.target {
                w.push_qualified_reference(&format!("{path}/stakeholder/target"), target)?;
            } else {
                w.push_str(&format_name(&s.value.declaration_name));
            }
            if let Some(ty) = &s.value.type_name {
                w.push_str(" : ");
                w.push_qualified_reference(&format!("{path}/stakeholder/type"), *ty)?;
            }
            w.push_char(';');
            Ok(())
        }
        RequirementDefBodyElement::Purpose(p) => {
            w.push_str("purpose ");
            w.push_qualified_reference(&format!("{path}/purpose/target"), p.value.target)?;
            w.push_char(';');
            Ok(())
        }
        RequirementDefBodyElement::Frame(f) => {
            w.push_str("frame ");
            if f.value.has_concern_keyword {
                w.push_str("concern ");
            }
            if let Some(short_name) = &f.value.short_name {
                w.push_char('<');
                w.push_str(&format_name(short_name));
                w.push_str("> ");
            }
            w.push_str(&format_name(&f.value.name));
            if let Some(type_name) = f.value.type_name {
                w.push_str(" : ");
                w.push_qualified_reference(&format!("{path}/frame/type"), type_name)?;
            }
            if let Some(multiplicity) = &f.value.multiplicity {
                emit_multiplicity(w, &multiplicity.value)?;
            }
            if let Some(subsets) = &f.value.subsets {
                emit_subsetting_clause(w, &subsets.value)?;
            }
            if let Some(redefines) = &f.value.redefines {
                emit_subsetting_clause(w, &redefines.value)?;
            }
            if let Some(value) = &f.value.value {
                emit_feature_value(w, value)?;
            }
            emit_requirement_body(w, path, &f.value.body)
        }
        RequirementDefBodyElement::VerifyRequirement(v) => {
            emit_verify_requirement(w, path, &v.value)
        }
        RequirementDefBodyElement::Dependency(d) => emit_dependency(w, path, &d.value),
        RequirementDefBodyElement::MetadataKeywordUsage(m) => {
            structure::emit_metadata_keyword_usage(w, path, &m.value)
        }
    }
}

pub(crate) fn emit_redefinition_attribute_binding(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &crate::ast::AttributeUsage,
) -> Result<(), EmitError> {
    w.push_str(":>> ");
    let target = usage
        .redefines
        .as_ref()
        .and_then(|relationship| relationship.value.first_target())
        .ok_or_else(|| EmitError::Unsupported {
            path: path.to_owned(),
            construct: "redefinition binding without a target".to_owned(),
        })?;
    w.push_qualified_reference(&format!("{path}/redefines[0]"), target)?;
    if let Some(typing) = &usage.typing {
        emit_typing_clause(w, &typing.value)?;
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    emit_multiplicity_modifiers(w, &usage.multiplicity_modifiers);
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
                w.push_qualified_reference(&format!("{path}/requirement/type"), *ty)?;
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
        w.push_qualified_reference(&format!("{path}/target"), *target)?;
    }
    if let Some(redefines) = &v.redefines {
        w.push_str(" :>> ");
        w.push_qualified_reference(&format!("{path}/redefines"), *redefines)?;
    }
    w.push_char(';');
    Ok(())
}

fn emit_subject_decl(w: &mut EmitWriter<'_>, subject: &SubjectDecl) -> Result<(), EmitError> {
    w.push_str("subject");
    if let Some(short) = &subject.short_name {
        w.push_str(" <");
        w.push_str(&format_name(short));
        w.push_char('>');
    }
    if !subject.name.is_empty() {
        w.push_char(' ');
        w.push_str(&format_name(&subject.name));
    }
    if let Some(typing) = &subject.typing {
        emit_typing_clause(w, &typing.value)?;
    }
    // Multiplicity before the subsetting clause: it binds to the declared feature, and emitting
    // it after the target produced `:>> RequirementCheck::subj[1]`, which reparses as a
    // multiplicity on the *target*.
    if let Some(mult) = &subject.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    emit_multiplicity_modifiers(w, &subject.multiplicity_modifiers);
    if let Some(subsets) = &subject.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(redefines) = &subject.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    if let Some(references) = &subject.references {
        emit_subsetting_clause(w, &references.value)?;
    }
    if let Some(crosses) = &subject.crosses {
        emit_subsetting_clause(w, &crosses.value)?;
    }
    if let Some(intersects) = &subject.intersects {
        emit_subsetting_clause(w, &intersects.value)?;
    }
    if let Some(value) = &subject.value {
        super::expr::emit_feature_value(w, value)?;
    }
    super::behavior::emit_definition_body(w, "subject/body", &subject.body)
}

pub(crate) fn emit_require_constraint(
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
    if let Some(target) = req.target {
        w.push_char(' ');
        w.push_qualified_reference(&format!("{path}/target"), target)?;
    }
    // `ConstraintUsageDeclaration`'s specialization part, authored between the name and the body.
    if let Some(typing) = &req.typing {
        super::structure::emit_typing_clause(w, &typing.value)?;
    }
    match &req.body {
        crate::ast::ConstraintDefBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::ConstraintDefBody::Brace { elements, .. } => {
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
    if dep.clients.is_empty() {
        return w.unsupported(path, "dependency with empty clients");
    }
    for (i, client) in dep.clients.iter().copied().enumerate() {
        if i > 0 {
            w.push_str(", ");
        }
        w.push_qualified_reference(&format!("{path}/clients[{i}]"), client)?;
    }
    w.push_str(" to ");
    for (i, supplier) in dep.suppliers.iter().copied().enumerate() {
        if i > 0 {
            w.push_str(", ");
        }
        w.push_qualified_reference(&format!("{path}/suppliers[{i}]"), supplier)?;
    }
    super::structure::emit_relationship_body(w, path, &dep.body)
}
pub(crate) fn emit_item_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &ItemUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    crate::emit::structure::emit_occurrence_usage_prefix(w, path, &usage.prefix)?;
    w.push_str("item ");
    if let Some(short) = &usage.short_name {
        w.push_char('<');
        w.push_str(&format_name(short));
        w.push_str("> ");
    }
    if !usage.name.is_empty() {
        w.push_str(&format_name(&usage.name));
    }
    // `item :>> shape : Cylinder` declares no label, so the `item ` keyword's trailing space has
    // no declaration to separate it from and the clause below brings its own.
    w.trim_trailing_space();
    if let Some(redefines) = &usage.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_qualified_reference(&format!("{path}/type"), *ty)?;
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    emit_multiplicity_modifiers(w, &usage.multiplicity_modifiers);
    if let Some(subsets) = &usage.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
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
    if concern.is_abstract {
        w.push_str("abstract ");
    }
    if concern.is_individual {
        w.push_str("individual ");
    }
    w.push_str("concern ");
    if concern.is_definition {
        w.push_str("def ");
    }
    w.push_authored_name(&format!("{path}/name"), &concern.name_span)?;
    if let Some(mult) = &concern.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if let Some(ty) = &concern.type_name {
        w.push_str(" : ");
        w.push_qualified_reference(&format!("{path}/type"), *ty)?;
    }
    if let Some(subsets) = &concern.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(redefines) = &concern.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    emit_requirement_body(w, path, &concern.body)
}

pub(crate) fn emit_use_case_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &UseCaseDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    emit_definition_prefix(w, def.definition_prefix.as_ref());
    if def.is_individual {
        w.push_str("individual ");
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
        w.push_qualified_reference(&format!("{path}/type"), *ty)?;
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if let Some(subsets) = &usage.subsets {
        crate::emit::structure::emit_subsetting_clause(w, &subsets.value)?;
    }
    emit_use_case_body(w, path, &usage.body)
}

pub(crate) fn emit_analysis_case_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::AnalysisCaseDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    emit_definition_prefix(w, def.definition_prefix.as_ref());
    if def.is_individual {
        w.push_str("individual ");
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
    structure::emit_occurrence_usage_prefix(w, path, &usage.prefix)?;
    w.push_str("analysis ");
    w.push_str(&format_name(&usage.name));
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_qualified_reference(&format!("{path}/type"), *ty)?;
    }
    if let Some(subsets) = &usage.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(redefines) = &usage.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    emit_use_case_body(w, path, &usage.body)
}

pub(crate) fn emit_verification_case_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::VerificationCaseDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    emit_definition_prefix(w, def.definition_prefix.as_ref());
    if def.is_individual {
        w.push_str("individual ");
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
        w.push_qualified_reference(&format!("{path}/type"), *ty)?;
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if let Some(subsets) = &usage.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    emit_use_case_body(w, path, &usage.body)
}

pub(crate) fn emit_case_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::CaseDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    emit_definition_prefix(w, def.definition_prefix.as_ref());
    if def.is_individual {
        w.push_str("individual ");
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
        w.push_qualified_reference(&format!("{path}/type"), *ty)?;
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
    emit_use_case_body(w, path, &usage.body)
}

/// Shared by `UseCaseDefBodyElement::IncludeUseCase` and `PartUsageBodyElement::IncludeUseCase`
/// (GH-89, `part system : System { include uc2; }`, Simple Tests/UseCaseTest.sysml:33) -- same
/// `IncludeUseCase` shape in both positions.
pub(crate) fn emit_include_use_case(
    w: &mut EmitWriter<'_>,
    path: &str,
    include: &crate::ast::IncludeUseCase,
) -> Result<(), EmitError> {
    w.push_str("include ");
    // The `use case` keyword pair selects `IncludeUseCaseUsage`'s declaration alternative; the
    // reference alternative writes its target directly. Re-emitting the declaration form as a
    // reference would rewrite the member into a different production.
    match include.target {
        Some(target) => w.push_qualified_reference(&format!("{path}/target"), target)?,
        None => {
            w.push_str("use case");
            if let Some(name) = &include.name {
                w.push_char(' ');
                w.push_str(&format_name(name));
            }
            if let Some(typing) = &include.typing {
                super::structure::emit_typing_clause(w, &typing.value)?;
            }
        }
    }
    if let Some(mult) = &include.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    emit_use_case_body(w, path, &include.body)
}

pub(crate) fn emit_use_case_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &UseCaseDefBody,
) -> Result<(), EmitError> {
    match body {
        UseCaseDefBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        UseCaseDefBody::Brace { elements, .. } => {
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
        UseCaseDefBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
        UseCaseDefBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
        UseCaseDefBodyElement::SubjectDecl(s) => emit_subject_decl(w, &s.value),
        UseCaseDefBodyElement::SubjectRef(_) => {
            w.push_str("subject;");
            Ok(())
        }
        UseCaseDefBodyElement::ActorUsage(a) => {
            emit_visibility(w, a.value.membership.visibility);
            w.push_str("actor");
            if let Some(short) = &a.value.short_name {
                w.push_str(" <");
                w.push_str(&format_name(short));
                w.push_char('>');
            }
            if !a.value.name.is_empty() {
                w.push_char(' ');
                w.push_str(&format_name(&a.value.name));
            }
            if let Some(type_name) = a.value.type_name {
                w.push_str(" : ");
                w.push_qualified_reference(&format!("{path}/actor/type"), type_name)?;
            }
            if let Some(mult) = &a.value.multiplicity {
                emit_multiplicity(w, &mult.value)?;
            }
            w.push_char(';');
            Ok(())
        }
        UseCaseDefBodyElement::FirstSuccession(f) => {
            w.push_str("first ");
            w.push_qualified_reference(&format!("{path}/first/target"), f.value.target)?;
            w.push_char(';');
            Ok(())
        }
        UseCaseDefBodyElement::ThenDone(_) => {
            w.push_str("then done;");
            Ok(())
        }
        UseCaseDefBodyElement::IncludeUseCase(i) => emit_include_use_case(w, path, &i.value),
        UseCaseDefBodyElement::ThenIncludeUseCase(t) => {
            w.push_str("then ");
            emit_include_use_case(w, &format!("{path}/include"), &t.value.include.value)
        }
        UseCaseDefBodyElement::ThenUseCaseUsage(t) => {
            w.push_str("then ");
            emit_use_case_usage(w, path, &t.value.use_case.value)
        }
        UseCaseDefBodyElement::UseCaseUsage(u) => emit_use_case_usage(w, path, &u.value),
        UseCaseDefBodyElement::CaseUsage(u) => emit_case_usage(w, path, &u.value),
        UseCaseDefBodyElement::VerificationCaseUsage(u) => {
            emit_verification_case_usage(w, path, &u.value)
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
                w.push_qualified_reference(&format!("{path}/objective/type"), *ty)?;
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
        UseCaseDefBodyElement::ReturnRef(return_ref) => emit_return_ref(w, path, &return_ref.value),
        UseCaseDefBodyElement::ActorRedefinitionAssignment(a) => {
            w.push_str("actor :>> ");
            w.push_qualified_reference(&format!("{path}/target"), a.value.target)?;
            w.push_str(" = ");
            emit_expression(w, &a.value.value.value)?;
            w.push_char(';');
            Ok(())
        }
        UseCaseDefBodyElement::RefRedefinition(r) => {
            w.push_str("ref :>> ");
            w.push_qualified_reference(&format!("{path}/target"), r.value.target)?;
            emit_use_case_body(w, path, &r.value.body.value)
        }
        UseCaseDefBodyElement::Ref(r) => super::structure::emit_ref_decl(w, path, &r.value),
        UseCaseDefBodyElement::InOutDecl(d) => super::behavior::emit_inout_decl(w, path, &d.value),
        UseCaseDefBodyElement::AssertConstraint(assert) => {
            crate::emit::view::emit_assert_constraint(w, path, &assert.value)
        }
        UseCaseDefBodyElement::ForLoop(f) => super::behavior::emit_for_loop(w, path, &f.value),
        UseCaseDefBodyElement::FlowUsage(f) => super::behavior::emit_flow_usage(w, path, &f.value),
        UseCaseDefBodyElement::MetadataKeywordUsage(m) => {
            structure::emit_metadata_keyword_usage(w, path, &m.value)
        }
    }
}

fn emit_return_ref(
    w: &mut EmitWriter<'_>,
    path: &str,
    return_ref: &ReturnRef,
) -> Result<(), EmitError> {
    w.push_str("return ref ");
    w.push_str(&format_name(&return_ref.name));
    if let Some(multiplicity) = &return_ref.multiplicity {
        emit_multiplicity(w, &multiplicity.value)?;
    }
    match &return_ref.body.value {
        ReturnRefBody::Semicolon { .. } => {
            w.push_char(';');
        }
        ReturnRefBody::Brace { elements, .. } => {
            w.push_str(" {");
            if !elements.is_empty() {
                w.newline();
                w.indent();
                for (index, element) in elements.iter().enumerate() {
                    match &element.value {
                        ReturnRefBodyElement::Annotating(member) => {
                            super::root::emit_annotating_member(w, path, member)?
                        }
                        ReturnRefBodyElement::Result(expression) => {
                            w.push_str("return ");
                            emit_expression(w, &expression.value)?;
                            w.push_char(';');
                        }
                        ReturnRefBodyElement::Error(error) => w.push_recovery_span(
                            &format!("{path}/return-ref-body[{index}]"),
                            &error.span,
                        )?,
                    }
                    w.newline();
                }
                w.dedent();
            }
            w.push_char('}');
        }
    }
    Ok(())
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
    if let Some(target) = decl.target {
        w.push_str(":>> ");
        w.push_qualified_reference("case-return/target", target)?;
    } else if !decl.declaration_name.is_empty() {
        let Some(name_span) = &decl.name_span else {
            return w.unsupported(
                "case-return",
                "return declaration name without an authored source span",
            );
        };
        w.push_authored_name("case-return/name", name_span)?;
    }
    if let Some(ty) = &decl.type_name {
        let has_head = decl.target.is_some() || !decl.declaration_name.is_empty();
        if decl.is_subsetting && has_head {
            w.push_str(" :> ");
        } else if decl.is_subsetting {
            w.push_str(":> ");
        } else if has_head {
            w.push_str(" : ");
        } else {
            w.push_str(": ");
        }
        w.push_qualified_reference("case-return/type", *ty)?;
    }
    if let Some(mult) = &decl.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if let Some(redefines) = &decl.redefines {
        crate::emit::structure::emit_subsetting_clause(w, &redefines.value)?;
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
        w.push_qualified_reference(&format!("{path}/type"), *ty)?;
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    emit_attribute_body(w, path, &usage.body)
}

/// `SatisfyRequirementUsage` emission.
///
/// Every clause comes from a structured field: the two prefixes from their authored keyword
/// spans, the requirement clause from whichever alternative the AST holds, and the `by` clause
/// only when a subject was authored. Nothing here inspects source text, `Display` output, or the
/// spelling of a declaration label to decide what to write.
pub(crate) fn emit_satisfy(
    w: &mut EmitWriter<'_>,
    path: &str,
    satisfy: &SatisfyRequirementUsage,
) -> Result<(), EmitError> {
    crate::emit::writer::emit_visibility(w, satisfy.membership.visibility);
    super::structure::emit_occurrence_usage_prefix(w, path, &satisfy.prefix)?;
    if satisfy.assert_span.is_some() {
        w.push_str("assert ");
    }
    if satisfy.not_span.is_some() {
        w.push_str("not ");
    }
    w.push_str("satisfy ");
    match &satisfy.requirement {
        SatisfiedRequirement::Reference { reference } => {
            w.push_qualified_reference(&format!("{path}/satisfy/requirement"), *reference)?;
        }
        SatisfiedRequirement::Declaration(declaration) => {
            w.push_str("requirement");
            let identification = &declaration.value.identification;
            if identification.short_name.is_some() || identification.name.is_some() {
                w.push_char(' ');
                emit_identification(w, identification);
            }
        }
    }
    if let Some(typing) = &satisfy.typing {
        emit_typing_clause(w, &typing.value)?;
    }
    if let Some(multiplicity) = &satisfy.multiplicity {
        emit_multiplicity(w, &multiplicity.value)?;
    }
    emit_multiplicity_modifiers(w, &satisfy.multiplicity_modifiers);
    for clause in [
        satisfy.subsets.as_ref(),
        satisfy.references.as_ref(),
        satisfy.redefines.as_ref(),
        satisfy.crosses.as_ref(),
    ]
    .into_iter()
    .flatten()
    {
        emit_subsetting_clause(w, &clause.value)?;
    }
    if let Some(value) = &satisfy.value {
        emit_feature_value(w, value)?;
    }
    if let Some(subject) = &satisfy.subject {
        w.push_str(" by ");
        w.push_qualified_reference(&format!("{path}/satisfy/subject"), subject.value.reference)?;
    }
    emit_requirement_body(w, path, &satisfy.body)
}
