//! Root namespace / package / import emission.

use super::writer::{emit_visibility, format_name, EmitWriter};
use super::EmitError;
use super::{behavior, requirement, structure, view};
use crate::ast::{
    CommentAnnotation, DeclarationName, DocComment, FilterMember, Identification, Import,
    ImportShape, ImportTarget, LibraryPackage, Package, PackageBody, PackageBodyElement,
    QualifiedIdentification, RootElement, RootNamespace, TextualRepresentation,
};

pub(crate) fn emit_root(w: &mut EmitWriter<'_>, root: &RootNamespace) -> Result<(), EmitError> {
    for (i, el) in root.elements.iter().enumerate() {
        if i > 0 {
            w.newline();
        }
        emit_root_element(w, &format!("root[{i}]"), &el.value)?;
        w.newline();
    }
    Ok(())
}

fn emit_root_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &RootElement,
) -> Result<(), EmitError> {
    match el {
        RootElement::Package(p) => emit_package(w, path, &p.value),
        RootElement::LibraryPackage(p) => emit_library_package(w, path, &p.value),
        RootElement::Namespace(n) => {
            w.push_str("namespace ");
            emit_qualified_identification(w, path, &n.value.identification)?;
            emit_package_body(w, path, &n.value.body)
        }
        RootElement::Import(i) => emit_import(w, &i.value),
        RootElement::Member(m) => emit_package_body_node(w, path, m),
    }
}

pub(crate) fn emit_package(
    w: &mut EmitWriter<'_>,
    path: &str,
    pkg: &Package,
) -> Result<(), EmitError> {
    w.push_str("package ");
    emit_qualified_identification(w, path, &pkg.identification)?;
    emit_package_body(w, path, &pkg.body)
}

pub(crate) fn emit_library_package(
    w: &mut EmitWriter<'_>,
    path: &str,
    pkg: &LibraryPackage,
) -> Result<(), EmitError> {
    if pkg.is_standard {
        w.push_str("standard ");
    }
    w.push_str("library package ");
    emit_qualified_identification(w, path, &pkg.identification)?;
    emit_package_body(w, path, &pkg.body)
}

fn emit_extended_definition(
    w: &mut EmitWriter<'_>,
    path: &str,
    def: &crate::ast::ExtendedDefinition,
) -> Result<(), EmitError> {
    if let Some(prefix) = &def.definition_prefix {
        structure::emit_definition_prefix(w, Some(prefix));
    }
    for keyword in &def.prefix_keywords {
        structure::emit_metadata_keyword_usage(w, path, &keyword.value)?;
        w.push_char(' ');
    }
    w.push_str("def ");
    emit_identification(w, &def.identification);
    if let Some(spec) = &def.specializes {
        structure::emit_typing_clause(w, &spec.value)?;
    }
    emit_package_body(w, path, &def.body)
}

pub(crate) fn emit_package_body(
    w: &mut EmitWriter<'_>,
    path: &str,
    body: &PackageBody,
) -> Result<(), EmitError> {
    match body {
        PackageBody::Semicolon { .. } => {
            w.push_char(';');
            Ok(())
        }
        PackageBody::Brace { elements, .. } => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_package_body_node(w, &format!("{path}/body[{i}]"), el)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
            Ok(())
        }
    }
}

fn emit_package_body_node(
    w: &mut EmitWriter<'_>,
    path: &str,
    node: &crate::ast::Node<PackageBodyElement>,
) -> Result<(), EmitError> {
    if matches!(
        node.value,
        PackageBodyElement::Error(_)
            | PackageBodyElement::Unsupported(_)
            | PackageBodyElement::FeatureDecl(_)
            | PackageBodyElement::ClassifierDecl(_)
            | PackageBodyElement::KermlSemanticDecl(_)
            | PackageBodyElement::KermlFeatureDecl(_)
            | PackageBodyElement::ExtendedLibraryDecl(_)
    ) {
        return w.push_recovery_span(path, &node.span);
    }
    emit_package_body_element(w, path, &node.value)
}

fn emit_kerml_relationship_decl(
    w: &mut EmitWriter<'_>,
    path: &str,
    decl: &crate::ast::KermlRelationshipDecl,
) -> Result<(), EmitError> {
    use crate::ast::KermlRelationshipKeyword as Kw;
    emit_visibility(w, decl.membership.visibility);
    // The prefix keyword and identification placement depend on the relationship family.
    let declaration_keyword = match decl.keyword {
        Kw::Subtype | Kw::Subclassifier | Kw::Typing | Kw::Subset | Kw::Redefinition => {
            Some("specialization ")
        }
        Kw::Disjoint => Some("disjoining "),
        Kw::Inverse => Some("inverting "),
        Kw::Featuring => None,
    };
    if let Some(declaration_keyword) = declaration_keyword {
        if decl.declaration_keyword_span.is_some() {
            w.push_str(declaration_keyword);
            if let Some(identification) = &decl.identification {
                emit_identification(w, identification);
                w.push_char(' ');
            }
        } else if let Some(identification) = &decl.identification {
            // The doubled spelling names the identification with the relationship's own
            // keyword: `typing t1 typing f typed by B;` (`kerml/coverage_relationships`).
            w.push_str(decl.keyword.as_str());
            w.push_char(' ');
            emit_identification(w, identification);
            w.push_char(' ');
        }
    }
    w.push_str(decl.keyword.as_str());
    w.push_char(' ');
    if decl.keyword == Kw::Featuring {
        if let Some(identification) = &decl.identification {
            emit_identification(w, identification);
            w.push_str(" of ");
        }
    }
    w.push_qualified_reference(&format!("{path}/source"), decl.source)?;
    w.push_str(match decl.keyword {
        Kw::Subtype | Kw::Subclassifier => " specializes ",
        Kw::Typing => " typed by ",
        Kw::Subset => " subsets ",
        Kw::Redefinition => " redefines ",
        Kw::Disjoint => " from ",
        Kw::Inverse => " of ",
        Kw::Featuring => " by ",
    });
    w.push_qualified_reference(&format!("{path}/target"), decl.target)?;
    // Annotation-only RelationshipBody; `None` is the `;` form. Annotations inside the brace
    // form are rare and re-emitted via the shared relationship-body element emitter.
    match &decl.body {
        None => w.push_char(';'),
        Some(elements) if elements.is_empty() => w.push_str(" {}"),
        Some(elements) => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                super::structure::emit_relationship_body_element_local(
                    w,
                    &format!("{path}/body[{i}]"),
                    &el.value,
                )?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
        }
    }
    Ok(())
}

pub(crate) fn emit_kerml_classifier_decl(
    w: &mut EmitWriter<'_>,
    path: &str,
    decl: &crate::ast::KermlClassifierDecl,
) -> Result<(), EmitError> {
    emit_visibility(w, decl.membership.visibility);
    if decl.is_abstract {
        w.push_str("abstract ");
    }
    w.push_str(decl.keyword.as_str());
    if decl.is_all {
        w.push_str(" all");
    }
    w.push_char(' ');
    emit_identification(w, &decl.identification);
    if let Some(multiplicity) = &decl.multiplicity {
        structure::emit_multiplicity(w, &multiplicity.value)?;
    }
    if let Some(spec) = &decl.specializes {
        structure::emit_typing_clause(w, &spec.value)?;
    }
    for (index, clause) in decl.type_relationships.iter().enumerate() {
        w.push_char(' ');
        w.push_str(clause.value.keyword.as_str());
        w.push_char(' ');
        for (target_index, target) in clause.value.targets.iter().copied().enumerate() {
            if target_index > 0 {
                w.push_str(", ");
            }
            w.push_qualified_reference(
                &format!("{path}/type-relationship[{index}][{target_index}]"),
                target,
            )?;
        }
    }
    super::view::emit_calc_body(w, path, &decl.body)
}

fn emit_kerml_bare_declaration(
    w: &mut EmitWriter<'_>,
    declaration: &crate::ast::KermlBareDeclaration,
) -> Result<(), EmitError> {
    w.push_str(declaration.keyword.as_str());
    if let Some(name_span) = &declaration.name_span {
        w.push_char(' ');
        w.push_authored_name("kerml-bare-declaration/name", name_span)?;
    }
    if let Some(multiplicity) = &declaration.multiplicity {
        w.push_char(' ');
        structure::emit_multiplicity(w, &multiplicity.value)?;
    }
    w.push_char(';');
    Ok(())
}

pub(crate) fn emit_package_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &PackageBodyElement,
) -> Result<(), EmitError> {
    match el {
        PackageBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
        PackageBodyElement::Unsupported(unsupported) => {
            w.push_recovery_span(path, &unsupported.span)
        }
        PackageBodyElement::Annotating(member) => emit_annotating_member(w, path, member),
        PackageBodyElement::Filter(f) => emit_filter(w, &f.value),
        PackageBodyElement::Package(p) => emit_package(w, path, &p.value),
        PackageBodyElement::LibraryPackage(p) => emit_library_package(w, path, &p.value),
        PackageBodyElement::Import(i) => emit_import(w, &i.value),
        PackageBodyElement::PartDef(p) => structure::emit_part_def(w, path, &p.value),
        PackageBodyElement::PartUsage(p) => structure::emit_part_usage(w, path, &p.value),
        PackageBodyElement::AttributeDef(a) => structure::emit_attribute_def(w, path, &a.value),
        PackageBodyElement::AttributeUsage(a) => structure::emit_attribute_usage(w, path, &a.value),
        PackageBodyElement::PortDef(p) => structure::emit_port_def(w, path, &p.value),
        PackageBodyElement::PortUsage(p) => structure::emit_port_usage(w, path, &p.value),
        PackageBodyElement::InterfaceDef(i) => structure::emit_interface_def(w, path, &i.value),
        PackageBodyElement::InterfaceUsage(i) => structure::emit_interface_usage(w, path, &i.value),
        PackageBodyElement::Connect(c) => structure::emit_connect(w, path, &c.value),
        PackageBodyElement::AliasDef(a) => structure::emit_alias_def(w, path, &a.value),
        PackageBodyElement::ItemDef(i) => structure::emit_item_def(w, path, &i.value),
        PackageBodyElement::ItemUsage(i) => requirement::emit_item_usage(w, path, &i.value),
        PackageBodyElement::IndividualDef(i) => structure::emit_individual_def(w, path, &i.value),
        PackageBodyElement::ActionDef(a) => behavior::emit_action_def(w, path, &a.value),
        PackageBodyElement::ActionUsage(a) => behavior::emit_action_usage(w, path, &a.value),
        PackageBodyElement::RequirementDef(r) => {
            requirement::emit_requirement_def(w, path, &r.value)
        }
        PackageBodyElement::RequirementUsage(r) => {
            requirement::emit_requirement_usage(w, path, &r.value)
        }
        PackageBodyElement::ConnectionDef(c) => structure::emit_connection_def(w, path, &c.value),
        PackageBodyElement::ConnectionUsage(c) => {
            structure::emit_connection_usage(w, path, &c.value)
        }
        PackageBodyElement::MetadataDef(m) => structure::emit_metadata_def(w, path, &m.value),
        PackageBodyElement::MetadataUsage(m) => structure::emit_metadata_usage(w, path, &m.value),
        PackageBodyElement::EnumDef(e) => structure::emit_enum_def(w, path, &e.value),
        PackageBodyElement::Dependency(d) => requirement::emit_dependency(w, path, &d.value),
        PackageBodyElement::ConstraintDef(c) => view::emit_constraint_def(w, path, &c.value),
        PackageBodyElement::ConstraintUsage(c) => view::emit_constraint_usage(w, path, &c.value),
        PackageBodyElement::CalcDef(c) => view::emit_calc_def(w, path, &c.value),
        PackageBodyElement::CalcUsage(c) => view::emit_calc_usage(w, path, &c.value),
        PackageBodyElement::ConcernUsage(c) => requirement::emit_concern_usage(w, path, &c.value),
        PackageBodyElement::UseCaseDef(u) => requirement::emit_use_case_def(w, path, &u.value),
        PackageBodyElement::UseCaseUsage(u) => requirement::emit_use_case_usage(w, path, &u.value),
        PackageBodyElement::StateDef(s) => behavior::emit_state_def(w, path, &s.value),
        PackageBodyElement::StateUsage(s) => behavior::emit_state_usage(w, path, &s.value),
        PackageBodyElement::Satisfy(s) => requirement::emit_satisfy(w, path, &s.value),
        PackageBodyElement::EnumerationUsage(e) => {
            requirement::emit_enumeration_usage(w, path, &e.value)
        }
        PackageBodyElement::FlowUsage(f) => behavior::emit_flow_usage(w, path, &f.value),
        PackageBodyElement::AllocationDef(a) => behavior::emit_allocation_def(w, path, &a.value),
        PackageBodyElement::AllocationUsage(a) => {
            behavior::emit_allocation_usage(w, path, &a.value)
        }
        PackageBodyElement::AnalysisCaseDef(a) => {
            requirement::emit_analysis_case_def(w, path, &a.value)
        }
        PackageBodyElement::AnalysisCaseUsage(a) => {
            requirement::emit_analysis_case_usage(w, path, &a.value)
        }
        PackageBodyElement::VerificationCaseDef(v) => {
            requirement::emit_verification_case_def(w, path, &v.value)
        }
        PackageBodyElement::VerificationCaseUsage(v) => {
            requirement::emit_verification_case_usage(w, path, &v.value)
        }
        PackageBodyElement::CaseDef(c) => requirement::emit_case_def(w, path, &c.value),
        PackageBodyElement::CaseUsage(c) => requirement::emit_case_usage(w, path, &c.value),
        PackageBodyElement::OccurrenceUsage(o) => {
            behavior::emit_occurrence_usage(w, path, &o.value)
        }
        PackageBodyElement::OccurrenceDef(o) => behavior::emit_occurrence_def(w, path, &o.value),
        PackageBodyElement::ViewDef(v) => view::emit_view_def(w, path, &v.value),
        PackageBodyElement::ViewUsage(v) => view::emit_view_usage(w, path, &v.value),
        PackageBodyElement::ViewpointDef(v) => view::emit_viewpoint_def(w, path, &v.value),
        PackageBodyElement::ViewpointUsage(v) => view::emit_viewpoint_usage(w, path, &v.value),
        PackageBodyElement::RenderingDef(r) => view::emit_rendering_def(w, path, &r.value),
        PackageBodyElement::RenderingUsage(r) => view::emit_rendering_usage(w, path, &r.value),
        PackageBodyElement::MetadataKeywordUsage(m) => {
            structure::emit_metadata_keyword_usage(w, path, &m.value)
        }
        PackageBodyElement::Ref(r) => structure::emit_ref_decl(w, path, &r.value),
        PackageBodyElement::DefaultReferenceUsage(d) => {
            structure::emit_default_reference_usage(w, path, &d.value)
        }
        PackageBodyElement::AssertConstraint(a) => view::emit_assert_constraint(w, path, &a.value),
        PackageBodyElement::PerformUsage(p) => behavior::emit_perform(w, path, &p.value),
        PackageBodyElement::BindingConnectorUsage(b) => {
            structure::emit_binding_connector_usage(w, path, &b.value)
        }
        PackageBodyElement::Succession(f) => behavior::emit_first_stmt(w, path, &f.value),
        PackageBodyElement::ExhibitState(e) => behavior::emit_exhibit_state(w, path, &e.value),
        PackageBodyElement::IncludeUseCase(i) => {
            requirement::emit_include_use_case(w, path, &i.value)
        }
        PackageBodyElement::ExtendedDefinition(d) => emit_extended_definition(w, path, &d.value),
        PackageBodyElement::ExtendedUsage(u) => structure::emit_extended_usage(w, path, &u.value),
        PackageBodyElement::FeatureDecl(_)
        | PackageBodyElement::ClassifierDecl(_)
        | PackageBodyElement::KermlSemanticDecl(_)
        | PackageBodyElement::KermlFeatureDecl(_)
        | PackageBodyElement::ExtendedLibraryDecl(_) => Err(EmitError::Opaque {
            path: path.to_string(),
            kind: super::OpacityKind::ExtendedLibraryDecl,
        }),
        PackageBodyElement::KermlBareDeclaration(declaration) => {
            emit_kerml_bare_declaration(w, &declaration.value)
        }
        PackageBodyElement::KermlClassifier(declaration) => {
            emit_kerml_classifier_decl(w, path, &declaration.value)
        }
        PackageBodyElement::KermlConnector(connector) => {
            super::view::emit_kerml_connector_member(w, path, &connector.value)
        }
        PackageBodyElement::KermlRelationship(relationship) => {
            emit_kerml_relationship_decl(w, path, &relationship.value)
        }
        PackageBodyElement::KermlInvariant(invariant) => {
            super::view::emit_kerml_invariant_member(w, path, &invariant.value)
        }
        PackageBodyElement::KermlFeature(feature) => {
            super::view::emit_kerml_feature(w, path, &feature.value)
        }
        PackageBodyElement::FlowDef(f) => behavior::emit_flow_def(w, path, &f.value),
        other @ PackageBodyElement::Actor(_) => w.unsupported(
            path,
            format!("{other:?}").chars().take(64).collect::<String>(),
        ),
    }
}

pub(crate) fn emit_import(w: &mut EmitWriter<'_>, import: &Import) -> Result<(), EmitError> {
    emit_visibility(w, import.membership.visibility);
    w.push_str("import ");
    if import.target.all_span.is_some() {
        w.push_str("all ");
    }
    emit_import_target(w, "import/target", &import.target)?;
    // Preserve RelationshipBody shape: `None` → `;`, `Some` → `{ ... }` even when empty
    // (brace bodies with only trivia comments parse as `Some([])`).
    match &import.body_elements {
        None => w.push_char(';'),
        Some(elements) if elements.is_empty() => w.push_str(" {}"),
        Some(elements) => {
            w.push_str(" {");
            w.newline();
            w.indent();
            for (i, el) in elements.iter().enumerate() {
                emit_relationship_body_element(w, &format!("import/body[{i}]"), &el.value)?;
                w.newline();
            }
            w.dedent();
            w.push_char('}');
        }
    }
    Ok(())
}

pub(crate) fn emit_import_target(
    w: &mut EmitWriter<'_>,
    path: &str,
    target: &ImportTarget,
) -> Result<(), EmitError> {
    w.push_qualified_reference(path, target.reference)?;
    match &target.shape {
        ImportShape::Membership { recursive_suffix } => {
            if recursive_suffix.is_some() {
                w.push_str("::**");
            }
        }
        ImportShape::Namespace {
            wildcard_suffix: _,
            recursive_suffix,
            combined_recursive_suffix_span: _,
        } => {
            w.push_str("::*");
            if recursive_suffix.is_some() {
                w.push_str("::**");
            }
        }
        ImportShape::Filter {
            recursive_suffix,
            members,
        } => {
            if recursive_suffix.is_some() {
                w.push_str("::**");
            }
            for member in members {
                w.push_str(" [");
                super::expr::emit_expression(w, &member.value.expression.value)?;
                w.push_char(']');
            }
        }
    }
    Ok(())
}

fn emit_relationship_body_element(
    w: &mut EmitWriter<'_>,
    path: &str,
    el: &crate::ast::RelationshipBodyElement,
) -> Result<(), EmitError> {
    use crate::ast::RelationshipBodyElement;
    match el {
        RelationshipBodyElement::Annotating(member) => emit_annotating_member(w, path, member),
        RelationshipBodyElement::KermlFeature(n) => {
            super::view::emit_kerml_feature(w, path, &n.value)
        }
        RelationshipBodyElement::Error(error) => w.push_recovery_span(path, &error.span),
    }
}

/// The annotating members are one grammar production, so they emit the same way wherever a scope
/// accepts them.
pub(crate) fn emit_annotating_member(
    w: &mut EmitWriter<'_>,
    path: &str,
    member: &crate::ast::AnnotatingMember,
) -> Result<(), EmitError> {
    use crate::ast::AnnotatingMember;
    match member {
        AnnotatingMember::Doc(d) => emit_doc(w, &d.value),
        AnnotatingMember::Comment(c) => emit_comment(w, &c.value),
        AnnotatingMember::TextualRep(r) => emit_textual_rep(w, &r.value),
        AnnotatingMember::MetadataAnnotation(annotation) => {
            super::structure::emit_metadata_annotation(w, path, &annotation.value)
        }
    }
}

pub(crate) fn emit_identification(w: &mut EmitWriter<'_>, id: &Identification) {
    if let Some(short) = &id.short_name {
        w.push_char('<');
        w.push_str(&format_name(short));
        w.push_char('>');
        // The separator belongs between the two halves, not to the short name. Writing it
        // unconditionally left a trailing space on `Identification = ( '<' NAME '>' )? ( NAME )?`
        // written with only its first half, which the caller then doubled up against whatever it
        // wrote next.
        if id.name.is_some() {
            w.push_char(' ');
        }
    }
    if let Some(name) = &id.name {
        w.push_str(&format_name(name));
    }
}

fn emit_qualified_identification(
    w: &mut EmitWriter<'_>,
    path: &str,
    id: &QualifiedIdentification,
) -> Result<(), EmitError> {
    if let Some(short) = &id.short_name {
        w.push_char('<');
        w.push_str(&format_name(short));
        w.push_str("> ");
    }
    match &id.name {
        Some(DeclarationName::Simple(name)) => w.push_str(&format_name(name)),
        Some(DeclarationName::Qualified(name)) => {
            w.push_qualified_reference(&format!("{path}/declaration-name"), name.storage_id())?
        }
        None => {}
    }
    Ok(())
}

pub(crate) fn emit_doc(w: &mut EmitWriter<'_>, doc: &DocComment) -> Result<(), EmitError> {
    if !w.emit_comments() {
        return Ok(());
    }
    w.push_str("doc");
    if let Some(id) = &doc.identification {
        w.push_char(' ');
        emit_identification(w, id);
    }
    if let Some(locale) = &doc.locale {
        w.push_str(" locale \"");
        w.push_str(locale);
        w.push_char('"');
    }
    w.newline();
    emit_regular_comment_body(w, &doc.text);
    Ok(())
}

pub(crate) fn emit_comment(
    w: &mut EmitWriter<'_>,
    comment: &CommentAnnotation,
) -> Result<(), EmitError> {
    if !w.emit_comments() {
        return Ok(());
    }
    // The `comment` keyword is optional in the grammar, but a member that omits it emits as a bare
    // block comment, which reparses as trivia rather than as a member. Reproduce what was authored.
    let has_keyword = comment.keyword_span.is_some();
    if has_keyword || comment.locale.is_some() {
        if has_keyword {
            w.push_str("comment");
        }
        if let Some(id) = &comment.identification {
            if has_keyword {
                w.push_char(' ');
            }
            emit_identification(w, id);
        }
        // `about` follows the identification in the production, and the elements it names are
        // part of what the comment says. Omitting them once cost the reader the annotation
        // itself; omitting them now would also fail the reparse gate.
        if !comment.about_targets.is_empty() {
            w.push_str(" about ");
            for (index, target) in comment.about_targets.iter().enumerate() {
                if index > 0 {
                    w.push_str(", ");
                }
                w.push_qualified_reference("comment about target", *target)?;
            }
        }
        if let Some(locale) = &comment.locale {
            if has_keyword || comment.identification.is_some() {
                w.push_char(' ');
            }
            w.push_str("locale \"");
            w.push_str(locale);
            w.push_char('"');
        }
        w.newline();
    }
    emit_regular_comment_body(w, &comment.text);
    Ok(())
}

fn emit_textual_rep(w: &mut EmitWriter<'_>, rep: &TextualRepresentation) -> Result<(), EmitError> {
    if !w.emit_comments() {
        return Ok(());
    }
    if let Some(id) = &rep.rep_identification {
        w.push_str("rep ");
        emit_identification(w, id);
        w.push_char(' ');
    }
    w.push_str("language \"");
    w.push_str(&rep.language);
    w.push_char('"');
    w.newline();
    emit_regular_comment_body(w, &rep.text);
    Ok(())
}

pub(crate) fn emit_filter(w: &mut EmitWriter<'_>, filter: &FilterMember) -> Result<(), EmitError> {
    emit_visibility(w, filter.visibility);
    w.push_str("filter ");
    super::expr::emit_expression(w, &filter.condition.value)?;
    w.push_char(';');
    Ok(())
}

fn emit_regular_comment_body(w: &mut EmitWriter<'_>, text: &str) {
    w.push_str("/*");
    w.push_str(text);
    w.push_str("*/");
}
