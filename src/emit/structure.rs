//! Structure emission: part / attribute (and shared helpers).

use super::expr::{emit_expression, emit_feature_value};
use super::root::{emit_identification, emit_import};
use super::writer::{emit_visibility, format_name, EmitWriter};
use super::EmitError;
use crate::ast::{
    AttributeBody, AttributeBodyElement, AttributeDef, AttributeUsage, Bind, Connect, ConnectStmt,
    ConnectionEnd, DefinitionPrefix, DerivationConnectionRole, DerivationEndRole, EndDecl,
    EndIdentity, InOut, InterfaceDef, InterfaceDefBody, InterfaceDefBodyElement, InterfaceUsage,
    InterfaceUsageBodyElement, Multiplicity, Node, PartDef, PartDefBody, PartDefBodyElement,
    PartUsage, PartUsageBody, PartUsageBodyElement, PortBody, PortBodyElement, PortDef,
    PortDefBody, PortDefBodyElement, PortUsage, RefBody, RefDecl, SubsettingKind,
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
    // `( SourceSuccessionMember )? OccurrenceUsageMember`, and `OccurrenceUsageMember =
    // MemberPrefix …`: `then` precedes the visibility keyword, which precedes the prefix.
    if usage.then_span.is_some() {
        w.push_str("then ");
    }
    emit_visibility(w, usage.membership.visibility);
    // `PartUsage = OccurrenceUsagePrefix 'part' Usage`: the same typed prefix boundary the
    // already migrated families stream through, in the production's slot order, with each
    // keyword written because its slot holds an authored span.
    emit_occurrence_usage_prefix(w, path, &usage.prefix)?;
    // No trailing space for the anonymous target-only forms: whichever clause follows
    // (`: Type`, `:>> target`, `:> target`) emits its own leading space, so `part :>> elements`
    // came back out as `part  :>> elements` and `in part : Engine` as `in part  : Engine`.
    // Mirrors `emit_attribute_usage`'s identical handling.
    if usage.short_name.is_none() && usage.name_span.is_none() {
        w.push_str("part");
    } else {
        w.push_str("part ");
    }
    if let Some(short) = &usage.short_name {
        w.push_char('<');
        w.push_str(&format_name(short));
        w.push_str("> ");
    }
    if !usage.name.is_empty() {
        w.push_str(&format_name(&usage.name));
    }
    let target_only = usage.name.is_empty() && usage.redefines.is_some();
    if let (true, Some(redefines)) = (target_only, usage.redefines.as_ref()) {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    if let Some(typing) = &usage.typing {
        emit_typing_clause(w, &typing.value)?;
    }
    // Redefines-only form (`part :>> target[0..1];`) attaches multiplicity after `:>> target`
    // (BNF / `part_usage_redefines_only`), not before the clause.
    let redefines_only = usage.name.is_empty()
        && usage.redefines.is_some()
        && usage.typing.is_none()
        && usage.subsets.is_none();
    if !redefines_only {
        if let Some(mult) = &usage.multiplicity {
            emit_multiplicity(w, &mult.value)?;
        }
        emit_multiplicity_modifiers(w, &usage.multiplicity_modifiers);
    }
    if let Some((subsets, subset_value)) = &usage.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
        if let Some(expr) = subset_value {
            w.push_str(" = ");
            emit_expression(w, &expr.value)?;
        }
    }
    if !target_only {
        if let Some(redefines) = &usage.redefines {
            emit_subsetting_clause(w, &redefines.value)?;
        }
    }
    if redefines_only {
        if let Some(mult) = &usage.multiplicity {
            emit_multiplicity(w, &mult.value)?;
        }
        emit_multiplicity_modifiers(w, &usage.multiplicity_modifiers);
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
    if let Some(multiplicity) = &def.multiplicity {
        emit_multiplicity(w, &multiplicity.value)?;
    }
    emit_multiplicity_modifiers(w, &def.multiplicity_modifiers);
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
    emit_ref_prefix(
        w,
        usage.is_derived,
        usage.usage_prefix.as_ref(),
        usage.is_constant,
    );
    if usage.is_reference {
        w.push_str("ref ");
    }
    // No trailing space for the anonymous target-only forms: the subsetting clause emits its
    // own leading space (`attribute :>> target;`, previously double-spaced).
    if usage.short_name.is_none() && usage.name_span.is_none() {
        w.push_str("attribute");
    } else {
        w.push_str("attribute ");
    }
    if let Some(short) = &usage.short_name {
        w.push_char('<');
        w.push_str(&format_name(short));
        w.push_str("> ");
    }
    // `name_span` is `None` only for the `PrefixRedefines`/`PrefixReferences`/`PrefixSubsets`
    // target-only forms (`attribute :>> target : Type[1];` etc., src/parser/attribute.rs).
    // `usage.name` stays empty because no declaration name was written; the target spelling lives
    // only in its arena-backed relationship. Any typing/multiplicity/`ordered`/`nonunique` were
    // parsed *after* the target reference (they trail the clause in source, e.g. `Mass Roll-up Example/
    // MassConstraintExample.sysml:18`'s `attribute :>> m : MassValue;`), not after a name.
    // Emitting the name here would duplicate the target (GH-113: `attribute target :>> target;`
    // instead of the original anonymous `attribute :>> target;`); emitting the trailing modifiers
    // here too would strand them before any name at all (`attribute : Type :>> target;`, which
    // reparses as the *unrelated* anonymous-colon-typed form instead of a redefines clause).
    // Mirrors `emit_part_usage`'s `redefines_only` handling.
    let target_only = usage.name_span.is_none();
    if !target_only {
        w.push_str(&format_name(&usage.name));
    }
    if !target_only {
        if let Some(typing) = &usage.typing {
            emit_typing_clause(w, &typing.value)?;
        }
        if let Some(mult) = &usage.multiplicity {
            emit_multiplicity(w, &mult.value)?;
        }
        emit_multiplicity_modifiers(w, &usage.multiplicity_modifiers);
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
    if target_only {
        if let Some(typing) = &usage.typing {
            emit_typing_clause(w, &typing.value)?;
        }
        if let Some(mult) = &usage.multiplicity {
            emit_multiplicity(w, &mult.value)?;
        }
        emit_multiplicity_modifiers(w, &usage.multiplicity_modifiers);
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
        PartDefBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        PartDefBody::Brace { elements, .. } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                let element_path = format!("{path}/body[{i}]");
                if matches!(el.value, PartDefBodyElement::UnsupportedMember(_)) {
                    w.push_recovery_span(&element_path, &el.span)?;
                } else {
                    emit_part_def_body_element(w, &element_path, &el.value)?;
                }
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
        PartDefBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
        PartDefBodyElement::KermlClassifier(n) => {
            super::root::emit_kerml_classifier_decl(w, path, &n.value)
        }
        PartDefBodyElement::UnsupportedMember(unsupported) => {
            w.push_recovery_span(path, &unsupported.span)
        }
        PartDefBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
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
        PartDefBodyElement::FirstStmt(first) => {
            super::behavior::emit_first_stmt(w, path, &first.value)
        }
        PartDefBodyElement::FlowDef(f) => super::behavior::emit_flow_def(w, path, &f.value),
        PartDefBodyElement::ViewDef(n) => super::view::emit_view_def(w, path, &n.value),
        PartDefBodyElement::ViewUsage(n) => super::view::emit_view_usage(w, path, &n.value),
        PartDefBodyElement::ViewpointDef(n) => super::view::emit_viewpoint_def(w, path, &n.value),
        PartDefBodyElement::ViewpointUsage(n) => {
            super::view::emit_viewpoint_usage(w, path, &n.value)
        }
        PartDefBodyElement::RenderingDef(n) => super::view::emit_rendering_def(w, path, &n.value),
        PartDefBodyElement::RenderingUsage(n) => {
            super::view::emit_rendering_usage(w, path, &n.value)
        }
        other @ (PartDefBodyElement::OccurrenceDef(_)
        | PartDefBodyElement::CaseDef(_)
        | PartDefBodyElement::CaseUsage(_)
        | PartDefBodyElement::UseCaseDef(_)
        | PartDefBodyElement::UseCaseUsage(_)
        | PartDefBodyElement::VerificationCaseDef(_)
        | PartDefBodyElement::VerificationCaseUsage(_)) => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

pub(crate) fn emit_part_usage_body_public(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &PartUsageBody,
) -> Result<(), EmitError> {
    emit_part_usage_body(w, path, body)
}

fn emit_part_usage_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &PartUsageBody,
) -> Result<(), EmitError> {
    match body {
        PartUsageBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        // An empty brace body stays on one line, the form `emit_ref_decl` already writes for the
        // same `Body<PartUsageBodyElement>`. Without this a `connect a to b {}` and the
        // `ref x {}` beside it formatted differently.
        PartUsageBody::Brace { elements, .. } if elements.is_empty() => {
            w.push_str(" {}");
            Ok(())
        }
        PartUsageBody::Brace { elements, .. } => {
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
        PartUsageBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
        PartUsageBodyElement::KermlClassifier(n) => {
            super::root::emit_kerml_classifier_decl(w, path, &n.value)
        }
        PartUsageBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
        PartUsageBodyElement::InOutDecl(d) => super::behavior::emit_inout_decl(w, path, &d.value),
        PartUsageBodyElement::EndDecl(e) => emit_end_decl(w, path, &e.value),
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
        PartUsageBodyElement::CalcUsage(c) => super::view::emit_calc_usage(w, path, &c.value),
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
        PartUsageBodyElement::SuccessionUsage(s) => {
            super::behavior::emit_succession_usage(w, path, &s.value)
        }
        PartUsageBodyElement::StateDef(s) => super::behavior::emit_state_def(w, path, &s.value),
        PartUsageBodyElement::StateUsage(s) => super::behavior::emit_state_usage(w, path, &s.value),
        PartUsageBodyElement::Allocate(a) => super::behavior::emit_allocate(w, path, &a.value),
        PartUsageBodyElement::Satisfy(s) => super::requirement::emit_satisfy(w, path, &s.value),
        PartUsageBodyElement::EnumerationUsage(e) => {
            super::requirement::emit_enumeration_usage(w, path, &e.value)
        }
        PartUsageBodyElement::FlowUsage(f) => super::behavior::emit_flow_usage(w, path, &f.value),
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
        PartUsageBodyElement::FlowDef(f) => super::behavior::emit_flow_def(w, path, &f.value),
        PartUsageBodyElement::ViewDef(n) => super::view::emit_view_def(w, path, &n.value),
        PartUsageBodyElement::ViewUsage(n) => super::view::emit_view_usage(w, path, &n.value),
        PartUsageBodyElement::ViewpointDef(n) => super::view::emit_viewpoint_def(w, path, &n.value),
        PartUsageBodyElement::ViewpointUsage(n) => {
            super::view::emit_viewpoint_usage(w, path, &n.value)
        }
        PartUsageBodyElement::RenderingDef(n) => super::view::emit_rendering_def(w, path, &n.value),
        PartUsageBodyElement::RenderingUsage(n) => {
            super::view::emit_rendering_usage(w, path, &n.value)
        }
        other @ PartUsageBodyElement::OccurrenceDef(_) => w.unsupported(
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
        AttributeBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        AttributeBody::Brace { elements, .. } => {
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
        AttributeBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
        AttributeBodyElement::Unsupported(unsupported) => {
            w.push_recovery_span(path, &unsupported.span)
        }
        AttributeBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
        AttributeBodyElement::KermlFeature(n) => {
            super::view::emit_kerml_feature_member(w, path, &n.value)
        }
        AttributeBodyElement::Invariant(n) => {
            super::view::emit_kerml_invariant_member(w, path, &n.value)
        }
        AttributeBodyElement::KermlConnector(n) => {
            super::view::emit_kerml_connector_member(w, path, &n.value)
        }
        AttributeBodyElement::KermlClassifier(n) => {
            super::root::emit_kerml_classifier_decl(w, path, &n.value)
        }
        AttributeBodyElement::Bind(b) => emit_bind(w, path, &b.value),
        AttributeBodyElement::Connection(c) => emit_connection_usage(w, path, &c.value),
        AttributeBodyElement::CalcDef(c) => super::view::emit_calc_def(w, path, &c.value),
        AttributeBodyElement::CalcUsage(c) => super::view::emit_calc_usage(w, path, &c.value),
        AttributeBodyElement::ConstraintUsage(c) => {
            super::view::emit_constraint_usage(w, path, &c.value)
        }
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
        AttributeBodyElement::ItemUsage(i) => {
            super::requirement::emit_item_usage(w, path, &i.value)
        }
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
    // `PortUsage = OccurrenceUsagePrefix 'port' Usage`: the same typed prefix boundary the other
    // migrated families stream through, in the production's slot order, with each keyword written
    // because its slot holds an authored span.
    emit_occurrence_usage_prefix(w, path, &usage.prefix)?;
    // No trailing space for the anonymous target-only forms: whichever clause follows (`: Type`,
    // `:>> target`, `:> target`) emits its own leading space, so `port :>> pe` came back out as
    // `port  :>> pe`. Mirrors `emit_part_usage`/`emit_attribute_usage`.
    if usage.short_name.is_none() && usage.name_span.is_none() {
        w.push_str("port");
    } else {
        w.push_str("port ");
    }
    if let Some(short) = &usage.short_name {
        w.push_char('<');
        w.push_str(&format_name(short));
        w.push_str("> ");
    }
    if !usage.name.is_empty() {
        w.push_str(&format_name(&usage.name));
    }
    let target_only = usage.name.is_empty() && usage.redefines.is_some();
    if let (true, Some(redefines)) = (target_only, usage.redefines.as_ref()) {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    if let Some(typing) = &usage.typing {
        emit_typing_clause(w, &typing.value)?;
    }
    if let Some(mult) = &usage.multiplicity {
        emit_multiplicity(w, &mult.value)?;
    }
    emit_multiplicity_modifiers(w, &usage.multiplicity_modifiers);
    if let Some((subsets, subset_value)) = &usage.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
        if let Some(expr) = subset_value {
            w.push_str(" = ");
            emit_expression(w, &expr.value)?;
        }
    }
    if !target_only {
        if let Some(redefines) = &usage.redefines {
            emit_subsetting_clause(w, &redefines.value)?;
        }
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
        PortDefBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        PortDefBody::Brace { elements, .. } => {
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
        PortDefBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
        PortDefBodyElement::Unsupported(unsupported) => {
            w.push_recovery_span(path, &unsupported.span)
        }
        PortDefBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
        PortDefBodyElement::AttributeDef(a) => emit_attribute_def(w, path, &a.value),
        PortDefBodyElement::AttributeUsage(a) => emit_attribute_usage(w, path, &a.value),
        PortDefBodyElement::PortUsage(p) => emit_port_usage(w, path, &p.value),
        PortDefBodyElement::RefDecl(r) => emit_ref_decl(w, path, &r.value),
        PortDefBodyElement::InOutDecl(d) => super::behavior::emit_inout_decl(w, path, &d.value),
        PortDefBodyElement::ItemDef(i) => emit_item_def(w, path, &i.value),
        PortDefBodyElement::ItemUsage(i) => super::requirement::emit_item_usage(w, path, &i.value),
        PortDefBodyElement::EnumerationUsage(e) => {
            super::requirement::emit_enumeration_usage(w, path, &e.value)
        }
        PortDefBodyElement::MetadataKeywordUsage(m) => {
            emit_metadata_keyword_usage(w, path, &m.value)
        }
    }
}

fn emit_port_body(w: &mut EmitWriter<'_>, path: &str, body: &PortBody) -> Result<(), EmitError> {
    match body {
        PortBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        PortBody::Brace { elements, .. } => {
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
        PortBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
        PortBodyElement::Annotating(member) => super::root::emit_annotating_member(w, path, member),
        PortBodyElement::PortUsage(p) => emit_port_usage(w, path, &p.value),
        PortBodyElement::AttributeUsage(a) => emit_attribute_usage(w, path, &a.value),
        PortBodyElement::InOutDecl(d) => super::behavior::emit_inout_decl(w, path, &d.value),
        PortBodyElement::ItemUsage(i) => super::requirement::emit_item_usage(w, path, &i.value),
        PortBodyElement::RefDecl(r) => emit_ref_decl(w, path, &r.value),
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
    // The brace form used to abort emission with `OpacityKind::OpaqueConnectBrace`, because the
    // body kept no members to write. It is a `UsageBody` now, so it writes like any other.
    emit_part_usage_body(w, path, &connect.body)?;
    if let Some(subsets) = &connect.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(redefines) = &connect.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    if connect.subsets.is_some() || connect.redefines.is_some() {
        w.push_char(';');
    }
    Ok(())
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
    emit_definition_prefix(w, def.definition_prefix.as_ref());
    if def.is_individual {
        w.push_str("individual ");
    }
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
        InterfaceDefBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        InterfaceDefBody::Brace { elements, .. } => {
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
        InterfaceDefBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
        InterfaceDefBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
        InterfaceDefBodyElement::MetadataKeywordUsage(usage) => {
            emit_metadata_keyword_usage(w, path, &usage.value)
        }
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
        other @ (InterfaceDefBodyElement::ItemDef(_) | InterfaceDefBodyElement::ItemUsage(_)) => w
            .unsupported(
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
    if let Some(short_name) = &end.short_name {
        w.push_char('<');
        w.push_str(&format_name(short_name));
        w.push_str("> ");
    }
    match &end.identity {
        EndIdentity::Declaration(name) => w.push_str(&format_name(&name.value)),
        EndIdentity::Derivation(role) => match role.value {
            DerivationEndRole::Original => w.push_str("#original"),
            DerivationEndRole::Derive => w.push_str("#derive"),
        },
    }
    if let Some(nested) = &end.nested_usage {
        return w.unsupported(
            path,
            format!("EndDecl nested_usage {nested:?}")
                .chars()
                .take(64)
                .collect::<String>(),
        );
    }
    if let Some(typing) = &end.typing {
        emit_typing_clause(w, &typing.value)?;
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
    emit_part_usage_body(w, path, &stmt.body)
}

pub(crate) fn emit_relationship_body_element_local(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &crate::ast::RelationshipBodyElement,
) -> Result<(), EmitError> {
    use crate::ast::RelationshipBodyElement;
    match el {
        RelationshipBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
        RelationshipBodyElement::KermlFeature(n) => {
            super::view::emit_kerml_feature_member(w, path, &n.value)
        }
        RelationshipBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
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
            subsets,
            redefines,
            from,
            to,
            body,
        } => {
            w.push_str("interface");
            if let Some(n) = name {
                w.push_char(' ');
                w.push_str(&format_name(n));
            }
            if let Some(ty) = interface_type {
                w.push_str(" : ");
                w.push_qualified_reference("interface type", *ty)?;
            }
            if let Some(subsets) = subsets {
                emit_subsetting_clause(w, &subsets.value)?;
            }
            if let Some(redefines) = redefines {
                emit_subsetting_clause(w, &redefines.value)?;
            }
            w.push_char(' ');
            w.push_str("connect ");
            emit_expression(w, &from.value)?;
            w.push_str(" to ");
            emit_expression(w, &to.value)?;
            emit_interface_usage_body(w, path, body)
        }
        InterfaceUsage::Connection {
            subsets,
            redefines,
            from,
            to,
            body,
        } => {
            w.push_str("interface");
            if let Some(subsets) = subsets {
                emit_subsetting_clause(w, &subsets.value)?;
            }
            if let Some(redefines) = redefines {
                emit_subsetting_clause(w, &redefines.value)?;
            }
            w.push_char(' ');
            emit_expression(w, &from.value)?;
            w.push_str(" to ");
            emit_expression(w, &to.value)?;
            emit_interface_usage_body(w, path, body)
        }
        InterfaceUsage::Declaration {
            name,
            interface_type,
            subsets,
            redefines,
            body,
        } => {
            w.push_str("interface");
            if let Some(n) = name {
                w.push_char(' ');
                w.push_str(&format_name(n));
            }
            if let Some(ty) = interface_type {
                w.push_str(" : ");
                w.push_qualified_reference("interface type", *ty)?;
            }
            if let Some(subsets) = subsets {
                emit_subsetting_clause(w, &subsets.value)?;
            }
            if let Some(redefines) = redefines {
                emit_subsetting_clause(w, &redefines.value)?;
            }
            if name.is_none()
                && interface_type.is_none()
                && subsets.is_none()
                && redefines.is_none()
            {
                w.push_char(' ');
            }
            emit_interface_usage_body(w, path, body)
        }
    }
}

fn emit_interface_usage_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &crate::ast::Body<InterfaceUsageBodyElement>,
) -> Result<(), EmitError> {
    match body {
        crate::ast::Body::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::Body::Brace { elements, .. } if elements.is_empty() => {
            w.push_str(" {}");
            Ok(())
        }
        crate::ast::Body::Brace { elements, .. } => {
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

fn emit_interface_usage_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &InterfaceUsageBodyElement,
) -> Result<(), EmitError> {
    match el {
        InterfaceUsageBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
        InterfaceUsageBodyElement::RefRedef {
            target,
            value,
            body,
        } => {
            w.push_str("ref :>> ");
            w.push_qualified_reference("interface ref redefinition", *target)?;
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
    if let Some(dir) = decl.direction {
        emit_direction(w, dir);
    }
    emit_ref_prefix(
        w,
        decl.is_derived,
        decl.usage_prefix.as_ref(),
        decl.is_constant,
    );
    w.push_str("ref");
    if let Some(kind) = decl.kind_keyword {
        w.push_char(' ');
        w.push_str(kind.as_str());
    }
    if let Some(short_name) = &decl.short_name {
        w.push_str(" <");
        w.push_str(&format_name(short_name));
        w.push_char('>');
    }
    // Anonymous `ref :>> target;` / `ref redefines a, b;` declarations have no name; emitting
    // `''` fabricated a quoted empty name the author never wrote (spec42 Gap 49d fallout).
    if !decl.name.is_empty() {
        w.push_char(' ');
        w.push_str(&format_name(&decl.name));
    } else if decl.kind_keyword.is_some() && decl.multiplicity.is_some() {
        // Keep the anonymous kind keyword lexically separate from its leading multiplicity.
        // `ref requirement[1..*]` parses `requirement` as a declaration name; the authored
        // anonymous form is `ref requirement [1..*]`.
        w.push_char(' ');
    }
    // Typing first, then multiplicity, then the subsetting-family clauses: the one emission
    // order every `RefDecl` parser (`connector::ref_decl`, `part_ref_usage`) accepts, including
    // when a typing and a `:>` subsets clause co-occur (Systems Library `Interfaces.sysml`'s
    // `ref otherParticipants : Port [1..*] nonunique :> interfacingPorts default ...`).
    if let Some(typing) = &decl.typing {
        emit_typing_clause(w, &typing.value)?;
    }
    if let Some(multiplicity) = &decl.multiplicity {
        emit_multiplicity(w, &multiplicity.value)?;
    }
    emit_multiplicity_modifiers(w, &decl.multiplicity_modifiers);
    if let Some(redefines) = &decl.redefines {
        emit_subsetting_clause(w, &redefines.value)?;
    }
    if let Some(subsets) = &decl.subsets {
        emit_subsetting_clause(w, &subsets.value)?;
    }
    if let Some(value) = &decl.value {
        emit_feature_value(w, value)?;
    }
    emit_ref_body(w, path, &decl.body)
}

fn emit_ref_body(w: &mut EmitWriter<'_>, path: &str, body: &RefBody) -> Result<(), EmitError> {
    match body {
        RefBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        RefBody::Brace { elements, .. } => {
            if elements.is_empty() {
                w.push_str(" {}");
                Ok(())
            } else {
                w.push_str(" {");
                w.newline();
                w.indent();
                for (i, el) in elements.iter().enumerate() {
                    emit_part_usage_body_element(w, &format!("{path}/ref-body[{i}]"), &el.value)?;
                    w.newline();
                }
                w.dedent();
                w.push_char('}');
                Ok(())
            }
        }
    }
}

pub(crate) fn emit_bind(w: &mut EmitWriter<'_>, path: &str, bind: &Bind) -> Result<(), EmitError> {
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
            w.push_qualified_reference("binding type", *ty)?;
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
    emit_part_usage_body(w, path, &bind.body)
}

pub(crate) fn emit_binding_connector_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    usage: &crate::ast::BindingConnectorUsage,
) -> Result<(), EmitError> {
    w.push_str("binding");
    if usage.all {
        w.push_str(" all");
    }
    if let Some(name_span) = &usage.name_span {
        w.push_char(' ');
        w.push_span_name("binding-connector-usage/name", name_span)?;
    }
    if let Some(mult) = &usage.multiplicity {
        if usage.name_span.is_none() {
            w.push_char(' ');
        }
        emit_multiplicity(w, &mult.value)?;
    }
    w.push_char(' ');
    if usage.uses_of_keyword {
        w.push_str("of ");
    } else if usage.uses_bind_keyword {
        w.push_str("bind ");
    }
    w.push_qualified_reference("binding left", usage.left)?;
    w.push_str(" = ");
    w.push_qualified_reference("binding right", usage.right)?;
    emit_part_usage_body(w, path, &usage.body)
}

/// BNF `RefPrefix = 'derived'? ('abstract' | 'variation')? 'constant'?` (§8.2.2.6.2), in the one
/// order the grammar allows.
///
/// Each usage emitter used to spell this inline and the three had drifted into three different
/// orders, so `derived abstract x` came back out as `abstract derived x` -- a reordering the
/// grammar does not permit, and one no whole-AST comparison could see.
pub(crate) fn emit_ref_prefix(
    w: &mut EmitWriter<'_>,
    is_derived: bool,
    usage_prefix: Option<&DefinitionPrefix>,
    is_constant: bool,
) {
    if is_derived {
        w.push_str("derived ");
    }
    emit_definition_prefix_value(w, usage_prefix);
    if is_constant {
        w.push_str("constant ");
    }
}

pub(crate) fn emit_definition_prefix(
    w: &mut EmitWriter<'_>,
    prefix: Option<&Node<DefinitionPrefix>>,
) {
    emit_definition_prefix_value(w, prefix.map(|prefix| &prefix.value));
}

/// The same slot where it is stored without a node wrapper -- `RefPrefix`'s `variance` reached
/// through the spanless `usage_prefix` fields that have not yet migrated onto it.
pub(crate) fn emit_definition_prefix_value(
    w: &mut EmitWriter<'_>,
    prefix: Option<&DefinitionPrefix>,
) {
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

/// `OccurrenceUsagePrefix`, streamed in the production's own slot order.
///
/// One emitter for every family that owns the prefix. Each keyword is written because its slot
/// holds an authored span, never because a flag was inferred from something else, and the order is
/// the grammar's, not the order the fields happen to be declared in. A slot that was not authored
/// writes nothing; nothing here can invent a prefix, and no branch can omit one that is present.
pub(crate) fn emit_occurrence_usage_prefix(
    w: &mut EmitWriter<'_>,
    path: &str,
    prefix: &crate::ast::OccurrenceUsagePrefix,
) -> Result<(), EmitError> {
    let ref_prefix = &prefix.basic.ref_prefix;
    if let Some(direction) = &ref_prefix.direction {
        emit_direction(w, direction.value);
    }
    if ref_prefix.derived_span.is_some() {
        w.push_str("derived ");
    }
    match ref_prefix.variance.as_ref().map(|node| node.value) {
        Some(crate::ast::DefinitionPrefix::Abstract) => w.push_str("abstract "),
        Some(crate::ast::DefinitionPrefix::Variation) => w.push_str("variation "),
        None => {}
    }
    if ref_prefix.constant_span.is_some() {
        w.push_str("constant ");
    }
    if prefix.basic.reference_span.is_some() {
        w.push_str("ref ");
    }
    if prefix.individual_span.is_some() {
        w.push_str("individual ");
    }
    if let Some(portion) = &prefix.portion {
        w.push_str(portion.value.keyword());
        w.push_char(' ');
    }
    for (index, keyword) in prefix.extension_keywords.iter().enumerate() {
        w.push_char('#');
        w.push_qualified_reference(
            &format!("{path}/prefix/extension[{index}]"),
            keyword.value.annotation,
        )?;
        w.push_char(' ');
    }
    Ok(())
}

pub(crate) fn emit_typing_clause(
    w: &mut EmitWriter<'_>,
    typing: &TypingRelationship,
) -> Result<(), EmitError> {
    match typing.spelling {
        crate::ast::TypingSpelling::Operator => match typing.kind {
            TypingKind::Typing => w.push_str(" : "),
            TypingKind::Subclassification => w.push_str(" :> "),
        },
        crate::ast::TypingSpelling::Specializes => w.push_str(" specializes "),
        crate::ast::TypingSpelling::DefinedBy => w.push_str(" defined by "),
        crate::ast::TypingSpelling::TypedBy => w.push_str(" typed by "),
    }
    if typing.is_conjugated {
        w.push_char('~');
    }
    for (index, target) in typing.target.iter().enumerate() {
        if index > 0 {
            w.push_str(", ");
        }
        w.push_qualified_reference("typing relationship", *target)?;
    }
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
    for (index, target) in rel.target.iter().enumerate() {
        if index > 0 {
            w.push_str(", ");
        }
        w.push_qualified_reference("subsetting relationship", *target)?;
    }
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

/// Emit `MultiplicityPart`'s ordering and uniqueness keyword slots in the order the author
/// wrote them.
///
/// The two slots are independent, so the grammar admits `ordered nonunique` and
/// `nonunique ordered` alike. Ordering by the authored spans reproduces whichever was written
/// instead of imposing this emitter's own field order on the source.
pub(crate) fn emit_multiplicity_modifiers(
    w: &mut EmitWriter<'_>,
    modifiers: &crate::ast::MultiplicityModifiers,
) {
    let mut slots: [Option<(usize, &'static str)>; 2] = [
        modifiers
            .ordering
            .as_ref()
            .map(|slot| (slot.span.offset, slot.value.keyword())),
        modifiers
            .uniqueness
            .as_ref()
            .map(|slot| (slot.span.offset, slot.value.keyword())),
    ];
    slots.sort_by_key(|slot| slot.map(|(offset, _)| offset));
    for (_, keyword) in slots.into_iter().flatten() {
        w.push_char(' ');
        w.push_str(keyword);
    }
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
    w.push_qualified_reference("alias target", alias.target)?;
    emit_relationship_body(w, path, &alias.body)
}

/// A `RelationshipBody`: `;`, `{}`, or the brace form around its members.
///
/// One emitter for every owner of the shape. `expose` and the view-body `satisfy` used to write
/// `{}` for *any* brace body, so a body with members formatted as an empty one and the members
/// were gone.
pub(crate) fn emit_relationship_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &crate::ast::Body<crate::ast::RelationshipBodyElement>,
) -> Result<(), EmitError> {
    match body {
        crate::ast::Body::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::Body::Brace { elements, .. } => {
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
    if def.is_individual {
        w.push_str("individual ");
    }
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
    path: &str,
    usage: &crate::ast::DefaultReferenceUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, usage.membership.visibility);
    if usage.has_feature_keyword {
        w.push_str("feature ");
    }
    w.push_str(&format_name(&usage.name));
    if let Some(typing) = &usage.typing {
        emit_typing_clause(w, &typing.value)?;
    }
    if let Some(multiplicity) = &usage.multiplicity {
        emit_multiplicity(w, &multiplicity.value)?;
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
    match &usage.body {
        None => {
            w.push_char(';');
            Ok(())
        }
        Some(elements) => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_feature_body_element(w, &format!("{path}/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

fn emit_feature_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &crate::ast::FeatureBodyElement,
) -> Result<(), EmitError> {
    match el {
        crate::ast::FeatureBodyElement::Binding(b) => {
            emit_default_reference_usage(w, path, &b.value)
        }
        crate::ast::FeatureBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
    }
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
    if let Some(ty) = usage.type_reference {
        w.push_str(" : ");
        w.push_qualified_reference("metadata type", ty)?;
    }
    if !usage.about_targets.is_empty() {
        w.push_str(" about ");
        for (i, target) in usage.about_targets.iter().enumerate() {
            if i > 0 {
                w.push_str(", ");
            }
            w.push_qualified_reference("metadata about target", *target)?;
        }
    }
    emit_attribute_body(w, path, &usage.body)
}

pub(crate) fn emit_enum_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::EnumDef,
) -> Result<(), EmitError> {
    emit_visibility(w, def.membership.visibility);
    w.push_str("enum def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        emit_typing_clause(w, &spec.value)?;
    }
    match &def.body {
        crate::ast::EnumerationBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::EnumerationBody::Brace {
            elements: values, ..
        } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for element in values {
                match &element.value {
                    crate::ast::EnumerationBodyElement::Annotating(member) => {
                        super::root::emit_annotating_member(w, path, member)?;
                    }
                    crate::ast::EnumerationBodyElement::Value(value) => {
                        if let Some(short_name) = &value.value.short_name {
                            w.push_char('<');
                            w.push_str(&format_name(short_name));
                            w.push_str("> ");
                        }
                        w.push_str(&format_name(&value.value.name));
                        if let Some(initializer) = &value.value.value {
                            emit_feature_value(w, initializer)?;
                        }
                        emit_part_usage_body(w, path, &value.value.body)?;
                    }
                    crate::ast::EnumerationBodyElement::Error(error) => {
                        w.push_recovery_span(path, &error.span)?;
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

pub(crate) fn emit_variant_usage(
    w: &mut EmitWriter<'_>,
    path: &str,
    variant: &crate::ast::VariantUsage,
) -> Result<(), EmitError> {
    emit_visibility(w, variant.membership.visibility);
    w.push_str("variant ");
    match &variant.typed {
        None => {
            let reference = variant.reference.ok_or_else(|| EmitError::Unsupported {
                path: path.to_owned(),
                construct: "untyped variant without a reference".to_owned(),
            })?;
            w.push_qualified_reference(&format!("{path}/reference"), reference)?;
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
        Some(crate::ast::VariantTypedUsage::Requirement(r)) => {
            super::requirement::emit_requirement_usage(w, path, &r.value)
        }
    }
}

pub(crate) fn emit_metadata_annotation(
    w: &mut EmitWriter<'_>,
    path: &str,
    ann: &crate::ast::MetadataAnnotation,
) -> Result<(), EmitError> {
    for prefix in &ann.prefixes {
        emit_metadata_keyword_usage(w, path, &prefix.value)?;
        w.push_char(' ');
    }
    match ann.introducer {
        crate::ast::MetadataFeatureIntroducer::At { .. } => w.push_char('@'),
        crate::ast::MetadataFeatureIntroducer::Metadata { .. } => w.push_str("metadata "),
    }
    if let Some(declared) = &ann.declared_name {
        emit_identification(w, &declared.value.identification);
        match declared.value.typed_by {
            crate::ast::MetadataTypedBy::Colon => w.push_str(" : "),
            crate::ast::MetadataTypedBy::TypedBy => w.push_str(" typed by "),
        }
    }
    w.push_qualified_reference("metadata annotation type", ann.type_reference)?;
    if !ann.about_targets.is_empty() {
        w.push_str(" about ");
        for (i, target) in ann.about_targets.iter().enumerate() {
            if i > 0 {
                w.push_str(", ");
            }
            w.push_qualified_reference("metadata annotation about target", *target)?;
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
    w.push_qualified_reference("metadata keyword type", usage.reference)?;
    match &usage.body {
        Some(body) => emit_attribute_body(w, path, body),
        None => Ok(()),
    }
}

pub(crate) fn emit_connection_def(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::ConnectionDef,
) -> Result<(), EmitError> {
    if let Some(role) = &def.derivation_role {
        match role.value {
            DerivationConnectionRole::Derivation => w.push_str("#derivation "),
        }
    }
    emit_visibility(w, def.membership.visibility);
    emit_definition_prefix(w, def.definition_prefix.as_ref());
    if def.is_individual {
        w.push_str("individual ");
    }
    // The trailing space belongs to the identification, not the keyword: an anonymous
    // `#derivation connection { ... }` has none, and writing it unconditionally produced
    // `connection def  {` with a doubled space.
    w.push_str("connection def");
    if def.identification.short_name.is_some() || def.identification.name.is_some() {
        w.push_char(' ');
        emit_identification(w, &def.identification);
    }
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
    if usage.by_reference {
        w.push_str("ref ");
    }
    w.push_str("connection ");
    if let Some(name) = &usage.name {
        w.push_str(&format_name(name));
    }
    if let Some(ty) = usage.type_reference {
        w.push_str(" : ");
        w.push_qualified_reference("connection type", ty)?;
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
        crate::ast::ConnectionDefBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        crate::ast::ConnectionDefBody::Brace { elements, .. } => {
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
        crate::ast::ConnectionDefBodyElement::Error(error) => {
            w.push_recovery_span(path, &error.span)
        }
        crate::ast::ConnectionDefBodyElement::Annotating(member) => {
            super::root::emit_annotating_member(w, path, member)
        }
        crate::ast::ConnectionDefBodyElement::MetadataKeywordUsage(usage) => {
            emit_metadata_keyword_usage(w, path, &usage.value)
        }
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
        // A connection definition body is an ordinary definition body, so it owns occurrence
        // usages like any other; emission was simply never wired here, which made a `connection
        // def` the one legal owning scope of a migrated family that could not be re-emitted.
        crate::ast::ConnectionDefBodyElement::OccurrenceUsage(o) => {
            super::behavior::emit_occurrence_usage(w, path, &o.value)
        }
        // Same story as the occurrence usage above: `succession causalOrdering first [nCauses]
        // causes.startShot then [nEffects] effects { … }` (Systems Library
        // `CausationConnections.sysml`) is what a connection definition body owns them for, and
        // the emitter every other owning scope already uses is the one that writes it.
        crate::ast::ConnectionDefBodyElement::SuccessionUsage(s) => {
            super::behavior::emit_succession_usage(w, path, &s.value)
        }
    }
}
