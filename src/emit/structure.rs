//! Structure emission: part / attribute (and shared helpers).

use super::expr::{emit_expression, emit_feature_value};
use super::root::{emit_comment, emit_doc, emit_identification, emit_import};
use super::writer::{
    emit_visibility, format_name, format_qualified_name, format_relationship_target, EmitWriter,
};
use super::EmitError;
use crate::ast::{
    AttributeBody, AttributeBodyElement, AttributeDef, AttributeUsage, Bind, Connect, ConnectBody,
    ConnectStmt, ConnectionEnd, DefinitionPrefix, EndDecl, InOut, InterfaceDef, InterfaceDefBody,
    InterfaceDefBodyElement, InterfaceUsage, InterfaceUsageBodyElement, Multiplicity, Node,
    PartDef, PartDefBody, PartDefBodyElement, PartUsage, PartUsageBody, PartUsageBodyElement,
    PortBody, PortBodyElement, PortDef, PortDefBody, PortDefBodyElement, PortUsage, RefBody,
    RefDecl, SubsettingKind, SubsettingRelationship, TypingKind, TypingRelationship,
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
        w.push_str(&format_qualified_name(&usage.type_name));
    }
    // Redefines-only form (`part :>> target[0..1];`) attaches multiplicity after `:>> target`
    // (BNF / `part_usage_redefines_only`), not before the clause.
    let redefines_only = usage.name.is_empty()
        && usage.redefines.is_some()
        && usage.typing.is_none()
        && usage.type_name.is_empty()
        && usage.subsets.is_none();
    if !redefines_only {
        if let Some(mult) = &usage.multiplicity {
            emit_multiplicity(w, &mult.value)?;
        }
        if usage.ordered {
            w.push_str(" ordered");
        }
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
    if redefines_only {
        if let Some(mult) = &usage.multiplicity {
            emit_multiplicity(w, &mult.value)?;
        }
        if usage.ordered {
            w.push_str(" ordered");
        }
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
        PartDefBodyElement::PortDef(p) => emit_port_def(w, path, &p.value),
        PartDefBodyElement::PortUsage(p) => emit_port_usage(w, path, &p.value),
        PartDefBodyElement::InterfaceDef(i) => emit_interface_def(w, path, &i.value),
        PartDefBodyElement::InterfaceUsage(i) => emit_interface_usage(w, path, &i.value),
        PartDefBodyElement::Connect(c) => emit_connect(w, path, &c.value),
        PartDefBodyElement::ItemDef(i) => emit_item_def(w, path, &i.value),
        PartDefBodyElement::ItemUsage(i) => super::requirement::emit_item_usage(w, path, &i.value),
        PartDefBodyElement::ActionDef(a) => super::behavior::emit_action_def(w, path, &a.value),
        PartDefBodyElement::ActionUsage(a) => super::behavior::emit_action_usage(w, path, &a.value),
        PartDefBodyElement::Perform(p) => super::behavior::emit_perform(w, path, &p.value),
        PartDefBodyElement::ExhibitState(e) => {
            super::behavior::emit_exhibit_state(w, path, &e.value)
        }
        PartDefBodyElement::VariantUsage(v) => emit_variant_usage(w, path, &v.value),
        PartDefBodyElement::RequirementDef(r) => {
            super::requirement::emit_requirement_def(w, path, &r.value)
        }
        PartDefBodyElement::RequirementUsage(r) => {
            super::requirement::emit_requirement_usage(w, path, &r.value)
        }
        PartDefBodyElement::DefaultReferenceUsage(d) => {
            emit_default_reference_usage(w, path, &d.value)
        }
        PartDefBodyElement::ConnectionDef(c) => emit_connection_def(w, path, &c.value),
        PartDefBodyElement::Connection(c) => emit_connection_usage(w, path, &c.value),
        PartDefBodyElement::CalcDef(c) => super::view::emit_calc_def(w, path, &c.value),
        PartDefBodyElement::CalcUsage(c) => super::view::emit_calc_usage(w, path, &c.value),
        PartDefBodyElement::MetadataDef(m) => emit_metadata_def(w, path, &m.value),
        PartDefBodyElement::MetadataUsage(m) => emit_metadata_usage(w, path, &m.value),
        PartDefBodyElement::EnumDef(e) => emit_enum_def(w, path, &e.value),
        PartDefBodyElement::ConstraintDef(c) => super::view::emit_constraint_def(w, path, &c.value),
        PartDefBodyElement::ConstraintUsage(c) => {
            super::view::emit_constraint_usage(w, path, &c.value)
        }
        PartDefBodyElement::AssertConstraint(a) => {
            super::view::emit_assert_constraint(w, path, &a.value)
        }
        PartDefBodyElement::StateDef(s) => super::behavior::emit_state_def(w, path, &s.value),
        PartDefBodyElement::StateUsage(s) => super::behavior::emit_state_usage(w, path, &s.value),
        PartDefBodyElement::Allocate(a) => super::behavior::emit_allocate(w, path, &a.value),
        PartDefBodyElement::Satisfy(s) => super::requirement::emit_satisfy(w, path, &s.value),
        PartDefBodyElement::Dependency(d) => super::requirement::emit_dependency(w, path, &d.value),
        PartDefBodyElement::EnumerationUsage(e) => {
            super::requirement::emit_enumeration_usage(w, path, &e.value)
        }
        PartDefBodyElement::FlowUsage(f) => super::behavior::emit_flow_usage(w, path, &f.value),
        PartDefBodyElement::AllocationDef(a) => {
            super::behavior::emit_allocation_def(w, path, &a.value)
        }
        PartDefBodyElement::AllocationUsage(a) => {
            super::behavior::emit_allocation_usage(w, path, &a.value)
        }
        PartDefBodyElement::MetadataAnnotation(m) => emit_metadata_annotation(w, path, &m.value),
        PartDefBodyElement::MetadataKeywordUsage(m) => {
            emit_metadata_keyword_usage(w, path, &m.value)
        }
        PartDefBodyElement::OccurrenceUsage(o) => {
            super::behavior::emit_occurrence_usage(w, path, &o.value)
        }
        PartDefBodyElement::AnalysisCaseDef(a) => {
            super::requirement::emit_analysis_case_def(w, path, &a.value)
        }
        PartDefBodyElement::AnalysisCaseUsage(a) => {
            super::requirement::emit_analysis_case_usage(w, path, &a.value)
        }
        PartDefBodyElement::AliasDef(a) => emit_alias_def(w, path, &a.value),
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
        PartUsageBodyElement::PortDef(p) => emit_port_def(w, path, &p.value),
        PartUsageBodyElement::PortUsage(p) => emit_port_usage(w, path, &p.value),
        PartUsageBodyElement::InterfaceUsage(i) => emit_interface_usage(w, path, &i.value),
        PartUsageBodyElement::Connect(c) => emit_connect(w, path, &c.value),
        PartUsageBodyElement::ItemDef(i) => emit_item_def(w, path, &i.value),
        PartUsageBodyElement::ItemUsage(i) => {
            super::requirement::emit_item_usage(w, path, &i.value)
        }
        PartUsageBodyElement::ActionUsage(a) => {
            super::behavior::emit_action_usage(w, path, &a.value)
        }
        PartUsageBodyElement::Perform(p) => super::behavior::emit_perform(w, path, &p.value),
        PartUsageBodyElement::VariantUsage(v) => emit_variant_usage(w, path, &v.value),
        PartUsageBodyElement::RequirementDef(r) => {
            super::requirement::emit_requirement_def(w, path, &r.value)
        }
        PartUsageBodyElement::RequirementUsage(r) => {
            super::requirement::emit_requirement_usage(w, path, &r.value)
        }
        PartUsageBodyElement::DefaultReferenceUsage(d) => {
            emit_default_reference_usage(w, path, &d.value)
        }
        PartUsageBodyElement::ConnectionDef(c) => emit_connection_def(w, path, &c.value),
        PartUsageBodyElement::Connection(c) => emit_connection_usage(w, path, &c.value),
        PartUsageBodyElement::CalcDef(c) => super::view::emit_calc_def(w, path, &c.value),
        PartUsageBodyElement::MetadataDef(m) => emit_metadata_def(w, path, &m.value),
        PartUsageBodyElement::MetadataUsage(m) => emit_metadata_usage(w, path, &m.value),
        PartUsageBodyElement::EnumDef(e) => emit_enum_def(w, path, &e.value),
        PartUsageBodyElement::ConstraintDef(c) => {
            super::view::emit_constraint_def(w, path, &c.value)
        }
        PartUsageBodyElement::ConstraintUsage(c) => {
            super::view::emit_constraint_usage(w, path, &c.value)
        }
        PartUsageBodyElement::AssertConstraint(a) => {
            super::view::emit_assert_constraint(w, path, &a.value)
        }
        PartUsageBodyElement::StateDef(s) => super::behavior::emit_state_def(w, path, &s.value),
        PartUsageBodyElement::StateUsage(s) => super::behavior::emit_state_usage(w, path, &s.value),
        PartUsageBodyElement::Allocate(a) => super::behavior::emit_allocate(w, path, &a.value),
        PartUsageBodyElement::Satisfy(s) => super::requirement::emit_satisfy(w, path, &s.value),
        PartUsageBodyElement::EnumerationUsage(e) => {
            super::requirement::emit_enumeration_usage(w, path, &e.value)
        }
        PartUsageBodyElement::FlowUsage(f) => super::behavior::emit_flow_usage(w, path, &f.value),
        PartUsageBodyElement::MetadataAnnotation(m) => emit_metadata_annotation(w, path, &m.value),
        PartUsageBodyElement::MetadataKeywordUsage(m) => {
            emit_metadata_keyword_usage(w, path, &m.value)
        }
        PartUsageBodyElement::OccurrenceUsage(o) => {
            super::behavior::emit_occurrence_usage(w, path, &o.value)
        }
        PartUsageBodyElement::AnalysisCaseDef(a) => {
            super::requirement::emit_analysis_case_def(w, path, &a.value)
        }
        PartUsageBodyElement::AnalysisCaseUsage(a) => {
            super::requirement::emit_analysis_case_usage(w, path, &a.value)
        }
        PartUsageBodyElement::AliasDef(a) => emit_alias_def(w, path, &a.value),
        PartUsageBodyElement::IncludeUseCase(i) => {
            super::requirement::emit_include_use_case(w, path, &i.value)
        }
        PartUsageBodyElement::UseCaseUsage(u) => {
            super::requirement::emit_use_case_usage(w, path, &u.value)
        }
        PartUsageBodyElement::VerificationCaseUsage(v) => {
            super::requirement::emit_verification_case_usage(w, path, &v.value)
        }
        other => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

pub(crate) fn emit_attribute_body(
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
        AttributeBodyElement::OccurrenceUsage(o) => {
            super::behavior::emit_occurrence_usage(w, path, &o.value)
        }
        AttributeBodyElement::Connect(c) => emit_connect(w, path, &c.value),
        AttributeBodyElement::MetadataKeywordUsage(m) => {
            emit_metadata_keyword_usage(w, path, &m.value)
        }
        AttributeBodyElement::AssertConstraint(a) => {
            super::view::emit_assert_constraint(w, path, &a.value)
        }
        AttributeBodyElement::RefDecl(r) => emit_ref_decl(w, path, &r.value),
        AttributeBodyElement::PartUsage(p) => emit_part_usage(w, path, &p.value),
    }
}

pub(crate) fn emit_port_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &PortDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("port def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_port_def_body(w, path, &def.body)
}

pub(crate) fn emit_port_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &PortUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if let Some(dir) = usage.direction {
        emit_direction(w, dir);
    }
    if usage.is_abstract {
        w.push_str("abstract ");
    }
    if usage.is_derived {
        w.push_str("derived ");
    }
    if usage.is_constant {
        w.push_str("constant ");
    }
    w.push_str("port ");
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
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
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
        if usage
            .subsets
            .as_ref()
            .and_then(|(_, v)| v.as_ref())
            .is_none()
        {
            emit_feature_value(w, value)?;
        }
    }
    emit_port_body(w, path, &usage.body)
}

fn emit_port_def_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &PortDefBody,
) -> Result<(), EmitError> {
    match body {
        PortDefBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        PortDefBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_port_def_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

fn emit_port_def_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &PortDefBodyElement,
) -> Result<(), EmitError> {
    match el {
        PortDefBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        PortDefBodyElement::Other(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::Other,
        }),
        PortDefBodyElement::Doc(d) => emit_doc(w, &d.value),
        PortDefBodyElement::AttributeDef(a) => emit_attribute_def(w, path, &a.value),
        PortDefBodyElement::AttributeUsage(a) => emit_attribute_usage(w, path, &a.value),
        PortDefBodyElement::PortUsage(p) => emit_port_usage(w, path, &p.value),
        PortDefBodyElement::InOutDecl(d) => super::behavior::emit_inout_decl(w, path, &d.value),
        PortDefBodyElement::ItemDef(i) => emit_item_def(w, path, &i.value),
        PortDefBodyElement::ItemUsage(i) => super::requirement::emit_item_usage(w, path, &i.value),
        PortDefBodyElement::EnumerationUsage(e) => {
            super::requirement::emit_enumeration_usage(w, path, &e.value)
        }
    }
}

fn emit_port_body(w: &mut EmitWriter<'_>, path: &str, body: &PortBody) -> Result<(), EmitError> {
    match body {
        PortBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        PortBody::Brace { elements } => {
            if elements.is_empty() {
                w.push_str(" {}");
                Ok(())
            } else {
                w.push_str(" {");
                w.newline();
                w.indent();
                for (i, el) in elements.iter().enumerate() {
                    emit_port_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                    w.newline();
                }
                w.dedent();
                w.push_char('}');
                Ok(())
            }
        }
    }
}

fn emit_port_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &PortBodyElement,
) -> Result<(), EmitError> {
    match el {
        PortBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        PortBodyElement::Doc(d) => emit_doc(w, &d.value),
        PortBodyElement::PortUsage(p) => emit_port_usage(w, path, &p.value),
        PortBodyElement::AttributeUsage(a) => emit_attribute_usage(w, path, &a.value),
        PortBodyElement::InOutDecl(d) => super::behavior::emit_inout_decl(w, path, &d.value),
        PortBodyElement::ItemUsage(i) => super::requirement::emit_item_usage(w, path, &i.value),
    }
}

pub(crate) fn emit_connect(
    w: &mut EmitWriter<'_>,
    path: &str,
    connect: &Connect,
) -> Result<(), EmitError> {
    w.push_str("connect ");
    emit_connection_end(w, &connect.from.value)?;
    w.push_str(" to ");
    emit_connection_end(w, &connect.to.value)?;
    // Brace bodies are opacity-gated for `Connect` (no structured members).
    match &connect.body {
        ConnectBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        ConnectBody::Brace => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::OpaqueConnectBrace,
        }),
    }
}

fn emit_connection_end(w: &mut EmitWriter<'_>, end: &ConnectionEnd) -> Result<(), EmitError> {
    if let Some(mult) = &end.multiplicity {
        emit_multiplicity(w, &mult.value)?;
        w.push_char(' ');
    }
    emit_expression(w, &end.expression.value)
}

pub(crate) fn emit_interface_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &InterfaceDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("interface def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_interface_def_body(w, path, &def.body)
}

fn emit_interface_def_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &InterfaceDefBody,
) -> Result<(), EmitError> {
    match body {
        InterfaceDefBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        InterfaceDefBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_interface_def_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

fn emit_interface_def_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &InterfaceDefBodyElement,
) -> Result<(), EmitError> {
    match el {
        InterfaceDefBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        InterfaceDefBodyElement::Doc(d) => emit_doc(w, &d.value),
        InterfaceDefBodyElement::EndDecl(e) => emit_end_decl(w, path, &e.value),
        InterfaceDefBodyElement::RefDecl(r) => emit_ref_decl(w, path, &r.value),
        InterfaceDefBodyElement::ConnectStmt(c) => emit_connect_stmt(w, path, &c.value),
        InterfaceDefBodyElement::AttributeDef(a) => emit_attribute_def(w, path, &a.value),
        InterfaceDefBodyElement::AttributeUsage(a) => emit_attribute_usage(w, path, &a.value),
        InterfaceDefBodyElement::PortDef(p) => emit_port_def(w, path, &p.value),
        InterfaceDefBodyElement::PortUsage(p) => emit_port_usage(w, path, &p.value),
        InterfaceDefBodyElement::FlowUsage(f) => {
            super::behavior::emit_flow_usage(w, path, &f.value)
        }
        other => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

pub(crate) fn emit_end_decl(
    w: &mut EmitWriter<'_>,
    path: &str,
    end: &EndDecl,
) -> Result<(), EmitError> {
    w.push_str("end ");
    w.push_str(&format_name(&end.name));
    if let Some(nested) = &end.nested_usage {
        return w.unsupported(
            path,
            format!("EndDecl nested_usage {nested:?}")
                .chars()
                .take(64)
                .collect::<String>(),
        );
    }
    // GH-85: `references` may trail an explicit `: Type` instead of replacing it
    // (`uses_derived_syntax: false`) -- only skip the `: Type` when `references` *is* the whole
    // target (the original GH-19 `uses_derived_syntax: true` case, where `type_name` holds a
    // display-only copy of the same target and would otherwise duplicate it).
    if !end.uses_derived_syntax && !end.type_name.is_empty() {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(&end.type_name));
    }
    if let Some(references) = &end.references {
        emit_subsetting_clause(w, &references.value)?;
    }
    if let Some(mult) = &end.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    if let Some(redefines) = &end.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    if let Some(crosses) = &end.crosses {
        emit_subsetting_clause(w, &crosses.value)?;
    }
    w.push_char(';');
    Ok(())
}

fn emit_connect_stmt(
    w: &mut EmitWriter<'_>,
    path: &str,
    stmt: &ConnectStmt,
) -> Result<(), EmitError> {
    w.push_str("connect ");
    emit_connection_end(w, &stmt.from.value)?;
    if stmt.extra_ends.is_empty() {
        w.push_str(" to ");
        emit_connection_end(w, &stmt.to.value)?;
    } else {
        // N-ary form: connect (a, b, c, ...)
        // Keep a conservative unsupported until a fixture needs it.
        let _ = path;
        return w.unsupported(path, "n-ary ConnectStmt");
    }
    emit_connect_stmt_body(w, path, stmt)
}

fn emit_connect_stmt_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    stmt: &ConnectStmt,
) -> Result<(), EmitError> {
    match &stmt.body {
        ConnectBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        ConnectBody::Brace => {
            if stmt.body_elements.is_empty() {
                w.push_str(" {}");
                Ok(())
            } else {
                w.push_str(" {");
                w.newline();
                w.indent();
                for (i, el) in stmt.body_elements.iter().enumerate() {
                    emit_relationship_body_element_local(
                        w,
                        &format!("{path}/connect-body[{i}]"),
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
}

fn emit_relationship_body_element_local(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &crate::ast::RelationshipBodyElement,
) -> Result<(), EmitError> {
    use crate::ast::RelationshipBodyElement;
    match el {
        RelationshipBodyElement::Doc(d) => emit_doc(w, &d.value),
        RelationshipBodyElement::Comment(c) => emit_comment(w, &c.value),
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

pub(crate) fn emit_interface_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &InterfaceUsage,
) -> Result<(), EmitError> {
    match usage {
        InterfaceUsage::TypedConnect {
            name,
            interface_type,
            from,
            to,
            body,
            body_elements,
        } => {
            w.push_str("interface ");
            if let Some(n) = name {
                w.push_str(&format_name(n));
                w.push_char(' ');
            }
            if let Some(ty) = interface_type {
                w.push_str(": ");
                w.push_str(&format_qualified_name(ty));
                w.push_char(' ');
            }
            w.push_str("connect ");
            emit_expression(w, &from.value)?;
            w.push_str(" to ");
            emit_expression(w, &to.value)?;
            emit_interface_usage_body(w, path, body, body_elements)
        }
        InterfaceUsage::Connection {
            from,
            to,
            body_elements,
        } => {
            w.push_str("interface ");
            emit_expression(w, &from.value)?;
            w.push_str(" to ");
            emit_expression(w, &to.value)?;
            emit_interface_usage_body(w, path, &ConnectBody::Brace, body_elements)
        }
        InterfaceUsage::Declaration {
            name,
            interface_type,
            body,
            body_elements,
        } => {
            w.push_str("interface ");
            if let Some(n) = name {
                w.push_str(&format_name(n));
            }
            if let Some(ty) = interface_type {
                if name.is_some() {
                    w.push_str(" : ");
                } else {
                    w.push_str(": ");
                }
                w.push_str(&format_qualified_name(ty));
            }
            emit_interface_usage_body(w, path, body, body_elements)
        }
    }
}

fn emit_interface_usage_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &ConnectBody,
    elements: &[Node<InterfaceUsageBodyElement>],
) -> Result<(), EmitError> {
    match body {
        ConnectBody::Semicolon if elements.is_empty() => {
            w.push_char(';');
            Ok(())
        }
        ConnectBody::Semicolon | ConnectBody::Brace => {
            if elements.is_empty() {
                // Preserve brace vs semicolon: empty TypedConnect brace → `{}`.
                if matches!(body, ConnectBody::Brace) {
                    w.push_str(" {}");
                } else {
                    w.push_char(';');
                }
                Ok(())
            } else {
                w.push_str(" {");
                w.newline();
                w.indent();
                for (i, el) in elements.iter().enumerate() {
                    emit_interface_usage_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                    w.newline();
                }
                w.dedent();
                w.push_char('}');
                Ok(())
            }
        }
    }
}

fn emit_interface_usage_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &InterfaceUsageBodyElement,
) -> Result<(), EmitError> {
    match el {
        InterfaceUsageBodyElement::Doc(d) => emit_doc(w, &d.value),
        InterfaceUsageBodyElement::RefRedef { name, value, body } => {
            w.push_str("ref :>> ");
            w.push_str(&format_name(name));
            w.push_str(" = ");
            emit_expression(w, &value.value)?;
            emit_ref_body(w, path, body)
        }
        InterfaceUsageBodyElement::EndDecl(e) => emit_end_decl(w, path, &e.value),
    }
}

pub(crate) fn emit_ref_decl(
    w: &mut EmitWriter<'_>,
    path: &str,
    decl: &RefDecl,
) -> Result<(), EmitError> {
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
        w.push_str(&format_qualified_name(&decl.type_name));
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
                w.push_str(" {");
                w.newline();
                w.indent();
                for (i, el) in elements.iter().enumerate() {
                    emit_ref_body_element(w, &format!("{path}/ref-body[{i}]"), &el.value)?;
                    w.newline();
                }
                w.dedent();
                w.push_char('}');
                Ok(())
            }
        }
    }
}

fn emit_ref_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &crate::ast::RefBodyElement,
) -> Result<(), EmitError> {
    use crate::ast::RefBodyElement;
    match el {
        RefBodyElement::Doc(d) => emit_doc(w, &d.value),
        RefBodyElement::Comment(c) => emit_comment(w, &c.value),
        RefBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        RefBodyElement::Other(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::Other,
        }),
        other => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

pub(crate) fn emit_bind(w: &mut EmitWriter<'_>, _path: &str, bind: &Bind) -> Result<(), EmitError> {
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
            w.push_str(&format_qualified_name(ty));
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

pub(crate) fn emit_definition_prefix(w: &mut EmitWriter<'_>, prefix: Option<&DefinitionPrefix>) {
    match prefix {
        Some(DefinitionPrefix::Abstract) => w.push_str("abstract "),
        Some(DefinitionPrefix::Variation) => w.push_str("variation "),
        None => {}
    }
}

pub(crate) fn emit_direction(w: &mut EmitWriter<'_>, dir: InOut) {
    match dir {
        InOut::In => w.push_str("in "),
        InOut::Out => w.push_str("out "),
        InOut::InOut => w.push_str("inout "),
    }
}

pub(crate) fn emit_typing_clause(
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
    let formatted = typing
        .target
        .iter()
        .map(|n| format_relationship_target(&n.value))
        .collect::<Vec<_>>()
        .join(", ");
    w.push_str(&formatted);
    Ok(())
}

pub(crate) fn emit_subsetting_clause(
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
    let formatted = rel
        .target
        .iter()
        .map(|n| format_relationship_target(&n.value))
        .collect::<Vec<_>>()
        .join(", ");
    w.push_str(&formatted);
    Ok(())
}

pub(crate) fn emit_multiplicity(
    w: &mut EmitWriter<'_>,
    mult: &Multiplicity,
) -> Result<(), EmitError> {
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

pub(crate) fn emit_alias_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    alias: &crate::ast::AliasDef,
) -> Result<(), EmitError> {
    emit_visibility(w, alias.membership.visibility);
    w.push_str("alias ");
    emit_identification(w, &alias.identification);
    w.push_str(" for ");
    w.push_str(&format_relationship_target(&alias.target));
    match &alias.body {
        crate::ast::AliasBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::AliasBody::Brace { elements } => {
            if elements.is_empty() {
                w.push_str(" {}");
                Ok(())
            } else {
                w.push_str(" {");
                w.newline();
                w.indent();
                for (i, el) in elements.iter().enumerate() {
                    emit_relationship_body_element_local(
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
}

pub(crate) fn emit_item_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::ItemDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("item def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_attribute_body(w, path, &def.body)
}

pub(crate) fn emit_individual_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::IndividualDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("individual def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_attribute_body(w, path, &def.body)
}

pub(crate) fn emit_default_reference_usage(
    w: &mut EmitWriter<'_>,
    _path: &str,
    usage: &crate::ast::DefaultReferenceUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    w.push_str(&format_name(&usage.name));
    if let Some(typing) = &usage.typing {
        emit_typing_clause(w, &typing.value)?;
    }
    if let Some(subsets) = &usage.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(redefines) = &usage.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    if let Some(value) = &usage.value {
        emit_feature_value(w, value)?;
    }
    w.push_char(';');
    Ok(())
}

pub(crate) fn emit_metadata_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::MetadataDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    if def.is_abstract {
        w.push_str("abstract ");
    }
    w.push_str("metadata def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_attribute_body(w, path, &def.body)
}

pub(crate) fn emit_metadata_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &crate::ast::MetadataUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    w.push_str("metadata ");
    w.push_str(&format_name(&usage.name));
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    if !usage.about_targets.is_empty() {
        w.push_str(" about ");
        for (i, t) in usage.about_targets.iter().enumerate() {
            if i > 0 {
                w.push_str(", ");
            }
            w.push_str(t);
        }
    }
    emit_attribute_body(w, path, &usage.body)
}

pub(crate) fn emit_enum_def(
    w: &mut EmitWriter<'_>,
    _path: &str,
    def: &crate::ast::EnumDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("enum def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    match &def.body {
        crate::ast::EnumerationBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::EnumerationBody::Brace { values } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for v in values {
                w.push_str(&format_name(&v.value.name));
                w.push_char(';');
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

pub(crate) fn emit_variant_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    variant: &crate::ast::VariantUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, variant.membership.visibility);
    w.push_str("variant ");
    match &variant.typed {
        None => {
            w.push_str(&format_name(&variant.name));
            match &variant.body {
                Some(body) => emit_part_usage_body(w, path, body),
                None => {
                    w.push_char(';');
                    Ok(())
                }
            }
        }
        Some(crate::ast::VariantTypedUsage::Part(p)) => emit_part_usage(w, path, &p.value),
        Some(crate::ast::VariantTypedUsage::Attribute(a)) => {
            emit_attribute_usage(w, path, &a.value)
        }
        Some(crate::ast::VariantTypedUsage::Item(i)) => {
            super::requirement::emit_item_usage(w, path, &i.value)
        }
        Some(crate::ast::VariantTypedUsage::Port(p)) => emit_port_usage(w, path, &p.value),
        Some(crate::ast::VariantTypedUsage::Perform(p)) => {
            super::behavior::emit_perform(w, path, &p.value)
        }
    }
}

pub(crate) fn emit_metadata_annotation(
    w: &mut EmitWriter<'_>,
    path: &str,
    ann: &crate::ast::MetadataAnnotation,
) -> Result<(), EmitError> {
    w.push_char('@');
    w.push_str(&format_name(&ann.name));
    if let Some(ty) = &ann.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    if !ann.about_targets.is_empty() {
        w.push_str(" about ");
        for (i, t) in ann.about_targets.iter().enumerate() {
            if i > 0 {
                w.push_str(", ");
            }
            w.push_str(t);
        }
    }
    emit_attribute_body(w, path, &ann.body)
}

pub(crate) fn emit_metadata_keyword_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &crate::ast::MetadataKeywordUsage,
) -> Result<(), EmitError> {
    w.push_char('#');
    w.push_str(&usage.keyword);
    if let Some(ty) = &usage.type_name {
        w.push_str(" : ");
        w.push_str(&format_qualified_name(ty));
    }
    if !usage.about_targets.is_empty() {
        w.push_str(" about ");
        for (i, t) in usage.about_targets.iter().enumerate() {
            if i > 0 {
                w.push_str(", ");
            }
            w.push_str(t);
        }
    }
    emit_attribute_body(w, path, &usage.body)
}

pub(crate) fn emit_connection_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::ConnectionDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("connection def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    emit_connection_def_body(w, path, &def.body)
}

pub(crate) fn emit_connection_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &crate::ast::ConnectionUsageMember,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    w.push_str("connection ");
    if let Some(name) = &usage.name {
        w.push_str(&format_name(name));
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
    if let (Some(from), Some(to)) = (&usage.connect_from, &usage.connect_to) {
        if !usage.connect_extra_ends.is_empty() {
            return w.unsupported(path, "n-ary connection usage");
        }
        w.push_str(" connect ");
        emit_connection_end(w, &from.value)?;
        w.push_str(" to ");
        emit_connection_end(w, &to.value)?;
    }
    emit_connection_def_body(w, path, &usage.body)
}

fn emit_connection_def_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &crate::ast::ConnectionDefBody,
) -> Result<(), EmitError> {
    match body {
        crate::ast::ConnectionDefBody::Semicolon => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::ConnectionDefBody::Brace { elements } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_connection_def_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

fn emit_connection_def_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &crate::ast::ConnectionDefBodyElement,
) -> Result<(), EmitError> {
    match el {
        crate::ast::ConnectionDefBodyElement::Error(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ParseError,
        }),
        crate::ast::ConnectionDefBodyElement::Doc(d) => emit_doc(w, &d.value),
        crate::ast::ConnectionDefBodyElement::EndDecl(e) => emit_end_decl(w, path, &e.value),
        crate::ast::ConnectionDefBodyElement::RefDecl(r) => emit_ref_decl(w, path, &r.value),
        crate::ast::ConnectionDefBodyElement::ConnectStmt(c) => {
            emit_connect_stmt(w, path, &c.value)
        }
        crate::ast::ConnectionDefBodyElement::AttributeDef(a) => {
            emit_attribute_def(w, path, &a.value)
        }
        crate::ast::ConnectionDefBodyElement::AttributeUsage(a) => {
            emit_attribute_usage(w, path, &a.value)
        }
        crate::ast::ConnectionDefBodyElement::ItemDef(i) => emit_item_def(w, path, &i.value),
        crate::ast::ConnectionDefBodyElement::ItemUsage(i) => {
            super::requirement::emit_item_usage(w, path, &i.value)
        }
        crate::ast::ConnectionDefBodyElement::PortDef(p) => emit_port_def(w, path, &p.value),
        crate::ast::ConnectionDefBodyElement::PortUsage(p) => emit_port_usage(w, path, &p.value),
        crate::ast::ConnectionDefBodyElement::AssertConstraint(a) => {
            super::view::emit_assert_constraint(w, path, &a.value)
        }
        crate::ast::ConnectionDefBodyElement::PartUsage(p) => emit_part_usage(w, path, &p.value),
        other => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}
