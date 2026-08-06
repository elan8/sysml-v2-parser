//! Structure emission: part / attribute (and shared helpers).

use super::expr::{emit_expression, emit_feature_value};
use super::root::{emit_comment, emit_doc, emit_identification, emit_import};
use super::writer::{emit_visibility, format_name, EmitWriter};
use super::EmitError;
use crate::ast::{
    AttributeBody, AttributeBodyElement, AttributeDef, AttributeUsage, Bind, ConnectBody,
    DefinitionPrefix, InOut, Multiplicity, Node, PartDef, PartDefBody, PartDefBodyElement,
    PartUsage, PartUsageBody, PartUsageBodyElement, RefBody, RefDecl, SubsettingKind,
    SubsettingRelationship, TypingKind, TypingRelationship,
};

pub(crate) fn emit_part_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &PartDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    emit_definition_prefix(w, def.definition_prefix.as_ref());
    if def.is_individual {
        w.push_str("individual ");
    }
    w.push_str("part def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_part_def_body(w, path, &def.body)
}

pub(crate) fn emit_part_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &PartUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    emit_definition_prefix(w, usage.usage_prefix.as_ref());
    if let Some(dir) = usage.direction {
        emit_direction(w, dir);
    }
    if usage.is_derived {
        w.push_str("derived ");
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
    w.push_str("part ");
    if let Some(short) = &usage.short_name {
        w.push_char('<');
        w.push_str(&format_name(short));
        w.push_str("> ");
    }
    if !usage.name.is_empty() {
        w.push_str(&format_name(&usage.name));
    }
    if let Some(typing) = &usage.typing {
        emit_typing_clause(w, &typing.value)?;
    } else if !usage.type_name.is_empty() {
        w.push_str(" : ");
        w.push_str(&usage.type_name);
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if usage.ordered {
        w.push_str(" ordered");
    }
    if let Some((subsets, subset_value)) = &usage.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
        if let Some(expr) = subset_value {
            w.push_str(" = ");
            emit_expression(w, &expr.value)?;
        }
    }
    if let Some(redefines) = &usage.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    if let Some(value) = &usage.value {
        // Avoid double-emitting when subsets already carried `= expr`.
        if usage
            .subsets
            .as_ref()
            .and_then(|(_, v)| v.as_ref())
            .is_none()
        {
            emit_feature_value(w, value)?;
        }
    }
    emit_part_usage_body(w, path, &usage.body)
}

pub(crate) fn emit_attribute_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &AttributeDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("attribute def ");
    if let Some(short) = &def.short_name {
        w.push_char('<');
        w.push_str(&format_name(short));
        w.push_str("> ");
    }
    w.push_str(&format_name(&def.name));
    if let Some(typing) = &def.typing {
        emit_typing_clause(w, &typing.value)?;
    }
    if def.ordered {
        w.push_str(" ordered");
    }
    if def.nonunique {
        w.push_str(" nonunique");
    }
    if let Some(value) = &def.value {
        emit_feature_value(w, value)?;
    }
    emit_attribute_body(w, path, &def.body)
}

pub(crate) fn emit_attribute_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &AttributeUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if usage.is_end {
        w.push_str("end ");
    }
    if let Some(dir) = usage.direction {
        emit_direction(w, dir);
    }
    if usage.is_derived {
        w.push_str("derived ");
    }
    if usage.is_constant {
        w.push_str("constant ");
    }
    w.push_str("attribute ");
    if let Some(short) = &usage.short_name {
        w.push_char('<');
        w.push_str(&format_name(short));
        w.push_str("> ");
    }
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
    emit_attribute_body(w, path, &usage.body)
}

fn emit_part_def_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &PartDefBody,
) -> Result<(), EmitError> {
    match body {
        PartDefBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        PartDefBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_part_def_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

fn emit_part_def_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &PartDefBodyElement,
) -> Result<(), EmitError> {
    match el {
        PartDefBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        PartDefBodyElement::Other(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::Other,
        }),
        PartDefBodyElement::OpaqueMember(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::OpaqueMember,
        }),
        PartDefBodyElement::Doc(d) => emit_doc(w, &d.value),
        PartDefBodyElement::Comment(c) => emit_comment(w, &c.value),
        PartDefBodyElement::AttributeDef(a) => emit_attribute_def(w, path, &a.value),
        PartDefBodyElement::AttributeUsage(a) => emit_attribute_usage(w, path, &a.value),
        PartDefBodyElement::PartDef(p) => emit_part_def(w, path, &p.value),
        PartDefBodyElement::PartUsage(p) => emit_part_usage(w, path, &p.value),
        PartDefBodyElement::Import(i) => emit_import(w, &i.value),
        PartDefBodyElement::Bind(b) => emit_bind(w, path, &b.value),
        PartDefBodyElement::Ref(r) => emit_ref_decl(w, path, &r.value),
        other => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

fn emit_part_usage_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &PartUsageBody,
) -> Result<(), EmitError> {
    match body {
        PartUsageBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        PartUsageBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_part_usage_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

fn emit_part_usage_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &PartUsageBodyElement,
) -> Result<(), EmitError> {
    match el {
        PartUsageBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        PartUsageBodyElement::Doc(d) => emit_doc(w, &d.value),
        PartUsageBodyElement::AttributeUsage(a) => emit_attribute_usage(w, path, &a.value),
        PartUsageBodyElement::PartUsage(p) => emit_part_usage(w, path, &p.value),
        PartUsageBodyElement::Import(i) => emit_import(w, &i.value),
        PartUsageBodyElement::Ref(r) => emit_ref_decl(w, path, &r.value),
        PartUsageBodyElement::Bind(b) => emit_bind(w, path, &b.value),
        other => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

fn emit_attribute_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &AttributeBody,
) -> Result<(), EmitError> {
    match body {
        AttributeBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        AttributeBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_attribute_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

fn emit_attribute_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &AttributeBodyElement,
) -> Result<(), EmitError> {
    match el {
        AttributeBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        AttributeBodyElement::Other(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::Other,
        }),
        AttributeBodyElement::Doc(d) => emit_doc(w, &d.value),
        AttributeBodyElement::AttributeDef(a) => emit_attribute_def(w, path, &a.value),
        AttributeBodyElement::AttributeUsage(a) => emit_attribute_usage(w, path, &a.value),
        other => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

fn emit_ref_decl(w: &mut EmitWriter<'_>, path: &str, decl: &RefDecl) -> Result<(), EmitError> {
    emit_visibility(w, decl.membership.visibility);
    w.push_str("ref ");
    w.push_str(&format_name(&decl.name));
    if let Some(redefines) = &decl.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    if let Some(subsets) = &decl.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(typing) = &decl.typing {
        emit_typing_clause(w, &typing.value)?;
    } else if !decl.type_name.is_empty() {
        w.push_str(" : ");
        w.push_str(&decl.type_name);
    }
    if let Some(value) = &decl.value {
        emit_feature_value(w, value)?;
    }
    emit_ref_body(w, path, &decl.body)
}

fn emit_ref_body(w: &mut EmitWriter<'_>, path: &str, body: &RefBody) -> Result<(), EmitError> {
    match body {
        RefBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        RefBody::Brace { elements } => {
            if elements.is_empty() {
                w.push_str(" {}");
                Ok(())
            } else {
                // Nested action-body members inside ref braces are not yet emitted.
                w.unsupported(path, "RefBody with nested ActionDefBodyElement members")
            }
        }
    }
}

fn emit_bind(w: &mut EmitWriter<'_>, _path: &str, bind: &Bind) -> Result<(), EmitError> {
    if bind.binding_name.is_some()
        || bind.binding_type.is_some()
        || bind.binding_multiplicity.is_some()
    {
        w.push_str("binding");
        if let Some(mult) = &bind.binding_multiplicity {
            w.push_char(' ');
            emit_multiplicity(w, &mult.value)?;
        }
        if let Some(name) = &bind.binding_name {
            w.push_char(' ');
            w.push_str(&format_name(name));
        }
        if let Some(ty) = &bind.binding_type {
            w.push_str(" : ");
            w.push_str(ty);
        }
        w.push_char(' ');
    }
    w.push_str("bind ");
    if let Some(mult) = &bind.left_multiplicity {
        emit_multiplicity(w, &mult.value)?;
        w.push_char(' ');
    }
    emit_expression(w, &bind.left.value)?;
    w.push_str(" = ");
    if let Some(mult) = &bind.right_multiplicity {
        emit_multiplicity(w, &mult.value)?;
        w.push_char(' ');
    }
    emit_expression(w, &bind.right.value)?;
    emit_optional_connect_body(w, bind.body.as_ref())
}

fn emit_optional_connect_body(
    w: &mut EmitWriter<'_>,
    body: Option<&ConnectBody>,
) -> Result<(), EmitError> {
    match body {
        None | Some(ConnectBody::Semicolon) => {
            w.push_char(';');
            Ok(())
        }
        Some(ConnectBody::Brace) => {
            // Opaque brace (no structured members); preserve empty `{}` for AST-eq.
            w.push_str(" {}");
            Ok(())
        }
    }
}

fn emit_definition_prefix(w: &mut EmitWriter<'_>, prefix: Option<&DefinitionPrefix>) {
    match prefix {
        Some(DefinitionPrefix::Abstract) => w.push_str("abstract "),
        Some(DefinitionPrefix::Variation) => w.push_str("variation "),
        None => {}
    }
}

fn emit_direction(w: &mut EmitWriter<'_>, dir: InOut) {
    match dir {
        InOut::In => w.push_str("in "),
        InOut::Out => w.push_str("out "),
        InOut::InOut => w.push_str("inout "),
    }
}

fn emit_typing_clause(
    w: &mut EmitWriter<'_>,
    typing: &TypingRelationship,
) -> Result<(), EmitError> {
    match typing.kind {
        TypingKind::Typing => w.push_str(" : "),
        TypingKind::Subclassification => w.push_str(" :> "),
    }
    if typing.is_conjugated {
        w.push_char('~');
    }
    w.push_str(&typing.target_display());
    Ok(())
}

fn emit_subsetting_clause(
    w: &mut EmitWriter<'_>,
    rel: &SubsettingRelationship,
) -> Result<(), EmitError> {
    match rel.kind {
        SubsettingKind::Subsets => w.push_str(" :> "),
        SubsettingKind::References => w.push_str(" ::> "),
        SubsettingKind::Redefines => w.push_str(" :>> "),
        SubsettingKind::Crosses => w.push_str(" => "),
        SubsettingKind::Intersects => w.push_str(" intersects "),
    }
    w.push_str(&rel.target_display());
    Ok(())
}

fn emit_multiplicity(w: &mut EmitWriter<'_>, mult: &Multiplicity) -> Result<(), EmitError> {
    w.push_char('[');
    if mult.lower == mult.upper {
        emit_bound(w, &mult.lower)?;
    } else {
        emit_bound(w, &mult.lower)?;
        w.push_str("..");
        emit_bound(w, &mult.upper)?;
    }
    w.push_char(']');
    Ok(())
}

fn emit_bound(
    w: &mut EmitWriter<'_>,
    bound: &Option<Box<Node<crate::ast::Expression>>>,
) -> Result<(), EmitError> {
    match bound {
        None => {
            w.push_char('*');
            Ok(())
        }
        Some(expr) => emit_expression(w, &expr.value),
    }
}
