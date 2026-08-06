//! View / calc / constraint emission.

use super::behavior::emit_inout_decl;
use super::expr::{emit_expression, emit_feature_value};
use super::root::{emit_doc, emit_identification};
use super::structure::emit_typing_clause;
use super::writer::{emit_visibility, format_name, format_qualified_name, EmitWriter};
use super::EmitError;
use crate::ast::{
    AssertConstraintMember, CalcDef, CalcDefBody, CalcDefBodyElement, CalcUsage, ConstraintDef,
    ConstraintDefBody, ConstraintDefBodyElement, ConstraintUsage, ReturnDecl,
};

pub(crate) fn emit_constraint_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &ConstraintDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("constraint def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_constraint_body(w, path, &def.body)
}

pub(crate) fn emit_constraint_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &ConstraintUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    w.push_str("constraint ");
    if !usage.name.is_empty() {
        w.push_str(&format_name(&usage.name));
    }
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    emit_constraint_body(w, path, &usage.body)
}

pub(crate) fn emit_constraint_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &ConstraintDefBody,
) -> Result<(), EmitError> {
    match body {
        ConstraintDefBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        ConstraintDefBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_constraint_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

pub(crate) fn emit_constraint_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &ConstraintDefBodyElement,
) -> Result<(), EmitError> {
    match el {
        ConstraintDefBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        ConstraintDefBodyElement::Other(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::Other,
        }),
        ConstraintDefBodyElement::Doc(d) => emit_doc(w, &d.value),
        ConstraintDefBodyElement::InOutDecl(d) => emit_inout_decl(w, path, &d.value),
        ConstraintDefBodyElement::Expression(e) => {
            emit_expression(w, &e.value)?;
            w.push_char(';');
            Ok(())
        }
        ConstraintDefBodyElement::Constraint(c) => emit_constraint_usage(w, path, &c.value),
        ConstraintDefBodyElement::AttributeUsage(a) => {
            // Keyword-less `:>> name = …` inside `require name { … }` (validation `10c`).
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
                super::requirement::emit_redefinition_attribute_binding(w, path, &a.value)
            } else {
                super::structure::emit_attribute_usage(w, path, &a.value)
            }
        }
        ConstraintDefBodyElement::MetadataAnnotation(_) => {
            w.unsupported(path, "Constraint MetadataAnnotation")
        }
    }
}

pub(crate) fn emit_calc_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &CalcDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("calc def ");
    emit_identification(w, &def.identification);
    emit_calc_body(w, path, &def.body)
}

pub(crate) fn emit_calc_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &CalcUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if let Some(dir) = usage.direction {
        super::structure::emit_direction(w, dir);
    }
    w.push_str("calc ");
    if let Some(redefines) = &usage.redefines {
        w.push_str(":>> ");
        w.push_str(&format_qualified_name(redefines));
    } else {
        emit_identification(w, &usage.identification);
    }
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    if let Some(value) = &usage.value {
        emit_feature_value(w, value)?;
    }
    emit_calc_body(w, path, &usage.body)
}

fn emit_calc_body(w: &mut EmitWriter<'_>, path: &str, body: &CalcDefBody) -> Result<(), EmitError> {
    match body {
        CalcDefBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        CalcDefBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_calc_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

fn emit_calc_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &CalcDefBodyElement,
) -> Result<(), EmitError> {
    match el {
        CalcDefBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        CalcDefBodyElement::Other(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::Other,
        }),
        CalcDefBodyElement::Doc(d) => emit_doc(w, &d.value),
        CalcDefBodyElement::InOutDecl(d) => emit_inout_decl(w, path, &d.value),
        CalcDefBodyElement::ReturnDecl(r) => emit_return_decl(w, &r.value),
        CalcDefBodyElement::CalcUsage(c) => emit_calc_usage(w, path, &c.value),
        CalcDefBodyElement::PartUsage(p) => {
            super::structure::emit_part_usage(w, path, &p.value)
        }
        CalcDefBodyElement::Expression(e) => {
            emit_expression(w, &e.value)?;
            w.push_char(';');
            Ok(())
        }
        CalcDefBodyElement::MetadataAnnotation(_) => w.unsupported(path, "Calc MetadataAnnotation"),
    }
}

fn emit_return_decl(w: &mut EmitWriter<'_>, ret: &ReturnDecl) -> Result<(), EmitError> {
    w.push_str("return ");
    if ret.is_redefine {
        w.push_str(":>> ");
    }
    if !ret.name.is_empty() {
        w.push_str(&format_name(&ret.name));
    }
    if ret.is_subsetting {
        w.push_str(":>");
    } else if ret.name.is_empty() {
        w.push_str(": ");
    } else {
        w.push_str(" : ");
    }
    w.push_str(&format_qualified_name(&ret.type_name));
    if let Some(value) = &ret.value {
        w.push_str(" = ");
        emit_expression(w, &value.value)?;
    }
    w.push_char(';');
    Ok(())
}

pub(crate) fn emit_assert_constraint(
    w: &mut EmitWriter<'_>,
    path: &str,
    assert: &AssertConstraintMember,
) -> Result<(), EmitError> {
    emit_visibility(w, assert.membership.visibility);
    w.push_str("assert ");
    if assert.is_negated {
        w.push_str("not ");
    }
    w.push_str("constraint ");
    if let Some(name) = &assert.name {
        w.push_str(&format_name(name));
    }
    if let Some(ty) = &assert.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    emit_constraint_body(w, path, &assert.body)
}

pub(crate) fn emit_view_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::ViewDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("view def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    match &def.body {
        crate::ast::ViewDefBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::ViewDefBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                match &el.value {
                    crate::ast::ViewDefBodyElement::Error(_) => {
                        return Err(EmitError::Opaque {
                            path: format!("{path}/body[{i}]"),
                            kind: super::OpacityKind::ParseError,
                        });
                    }
                    crate::ast::ViewDefBodyElement::Other(_) => {
                        return Err(EmitError::Opaque {
                            path: format!("{path}/body[{i}]"),
                            kind: super::OpacityKind::Other,
                        });
                    }
                    crate::ast::ViewDefBodyElement::Doc(d) => emit_doc(w, &d.value)?,
                    crate::ast::ViewDefBodyElement::MetadataAnnotation(m) => {
                        super::structure::emit_metadata_annotation(w, path, &m.value)?;
                    }
                    crate::ast::ViewDefBodyElement::Filter(f) => {
                        super::root::emit_filter(w, &f.value)?;
                    }
                    crate::ast::ViewDefBodyElement::ViewRendering(r) => {
                        emit_view_rendering(w, path, &r.value)?;
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

pub(crate) fn emit_view_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &crate::ast::ViewUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    w.push_str("view ");
    if !usage.name.is_empty() {
        w.push_str(&format_name(&usage.name));
    }
    if let Some(redefines) = &usage.redefines {
        emit_typing_clause_as_subset(w, &redefines.value)?;
    }
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    if let Some(mult) = &usage.multiplicity {
        super::structure::emit_multiplicity(w, &mult.value)?;
    }
    match &usage.body {
        crate::ast::ViewBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::ViewBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                match &el.value {
                    crate::ast::ViewBodyElement::Error(_) => {
                        return Err(EmitError::Opaque {
                            path: format!("{path}/body[{i}]"),
                            kind: super::OpacityKind::ParseError,
                        });
                    }
                    crate::ast::ViewBodyElement::Other(_) => {
                        return Err(EmitError::Opaque {
                            path: format!("{path}/body[{i}]"),
                            kind: super::OpacityKind::Other,
                        });
                    }
                    crate::ast::ViewBodyElement::Doc(d) => emit_doc(w, &d.value)?,
                    crate::ast::ViewBodyElement::Filter(f) => {
                        super::root::emit_filter(w, &f.value)?;
                    }
                    crate::ast::ViewBodyElement::ViewRendering(r) => {
                        emit_view_rendering(w, path, &r.value)?;
                    }
                    crate::ast::ViewBodyElement::Expose(e) => {
                        w.push_str("expose ");
                        w.push_str(&format_qualified_name(&e.value.target));
                        w.push_char(';');
                    }
                    crate::ast::ViewBodyElement::Satisfy(s) => {
                        w.push_str("satisfy ");
                        w.push_str(&format_qualified_name(&s.value.viewpoint_ref));
                        match &s.value.body {
                            crate::ast::ConnectBody::Semicolon => w.push_char(';'),
                            crate::ast::ConnectBody::Brace => w.push_str(" {}"),
                        }
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

fn emit_typing_clause_as_subset(
    w: &mut EmitWriter<'_>,
    rel: &crate::ast::SubsettingRelationship,
) -> Result<(), EmitError> {
    super::structure::emit_subsetting_clause(w, rel)
}

fn emit_view_rendering(
    w: &mut EmitWriter<'_>,
    path: &str,
    r: &crate::ast::ViewRenderingUsage,
) -> Result<(), EmitError> {
    w.push_str("render ");
    w.push_str(&format_name(&r.name));
    if let Some(ty) = &r.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    match &r.body {
        crate::ast::RenderingUsageBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::RenderingUsageBody::Brace { elements } if elements.is_empty() => {
            w.push_str(" {}");
            Ok(())
        }
        crate::ast::RenderingUsageBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                match &el.value {
                    crate::ast::RenderingUsageBodyElement::Error(_) => {
                        return Err(EmitError::Opaque {
                            path: format!("{path}/render[{i}]"),
                            kind: super::OpacityKind::ParseError,
                        });
                    }
                    crate::ast::RenderingUsageBodyElement::Doc(d) => emit_doc(w, &d.value)?,
                    crate::ast::RenderingUsageBodyElement::ViewUsage(v) => {
                        emit_view_usage(w, path, &v.value)?;
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

pub(crate) fn emit_viewpoint_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::ViewpointDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("viewpoint def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    super::requirement::emit_requirement_body_pub(w, path, &def.body)
}

pub(crate) fn emit_viewpoint_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &crate::ast::ViewpointUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    w.push_str("viewpoint ");
    w.push_str(&format_name(&usage.name));
    if !usage.type_name.is_empty() {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(&usage.type_name));
    }
    super::requirement::emit_requirement_body_pub(w, path, &usage.body)
}

pub(crate) fn emit_rendering_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::RenderingDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("rendering def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    match &def.body {
        crate::ast::RenderingDefBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::RenderingDefBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                match &el.value {
                    crate::ast::RenderingDefBodyElement::Error(_) => {
                        return Err(EmitError::Opaque {
                            path: format!("{path}/body[{i}]"),
                            kind: super::OpacityKind::ParseError,
                        });
                    }
                    crate::ast::RenderingDefBodyElement::Other(_) => {
                        return Err(EmitError::Opaque {
                            path: format!("{path}/body[{i}]"),
                            kind: super::OpacityKind::Other,
                        });
                    }
                    crate::ast::RenderingDefBodyElement::Doc(d) => emit_doc(w, &d.value)?,
                    crate::ast::RenderingDefBodyElement::Filter(f) => {
                        super::root::emit_filter(w, &f.value)?;
                    }
                    crate::ast::RenderingDefBodyElement::ViewRendering(r) => {
                        emit_view_rendering(w, path, &r.value)?;
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

pub(crate) fn emit_rendering_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &crate::ast::RenderingUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    w.push_str("rendering ");
    w.push_str(&format_name(&usage.name));
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    match &usage.body {
        crate::ast::RenderingUsageBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::RenderingUsageBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                match &el.value {
                    crate::ast::RenderingUsageBodyElement::Error(_) => {
                        return Err(EmitError::Opaque {
                            path: format!("{path}/body[{i}]"),
                            kind: super::OpacityKind::ParseError,
                        });
                    }
                    crate::ast::RenderingUsageBodyElement::Doc(d) => emit_doc(w, &d.value)?,
                    crate::ast::RenderingUsageBodyElement::ViewUsage(v) => {
                        emit_view_usage(w, path, &v.value)?;
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
