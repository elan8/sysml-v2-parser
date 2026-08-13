//! Collect ParseError diagnostics from recovery nodes embedded in the AST.

use super::recovery::parse_error_from_recovery_node;
use crate::ast::{
    ActionDefBody, ActionDefBodyElement, ActionUsageBody, ActionUsageBodyElement, AliasBody,
    AttributeBody, AttributeBodyElement, CalcDefBody, CalcDefBodyElement, ConnectionDefBody,
    ConnectionDefBodyElement, ConstraintDefBody, ConstraintDefBodyElement, DefinitionBody,
    DefinitionBodyElement, FirstMergeBody, FirstMergeBodyElement, InterfaceDefBody,
    InterfaceDefBodyElement, OccurrenceBodyElement, OccurrenceUsageBody, PackageBody,
    PackageBodyElement, PartDefBody, PartDefBodyElement, PartUsageBody, PartUsageBodyElement,
    PortDefBody, PortDefBodyElement, RefBody, RefBodyElement, RelationshipBodyElement,
    RenderingDefBody, RenderingDefBodyElement, RequirementDefBody, RequirementDefBodyElement,
    ReturnRefBody, ReturnRefBodyElement, RootNamespace, StateDefBody, StateDefBodyElement,
    TextualRepresentation, UseCaseDefBody, UseCaseDefBodyElement, ViewBody, ViewBodyElement,
    ViewDefBody, ViewDefBodyElement,
};
use crate::error::{DiagnosticCategory, DiagnosticSeverity, ParseError};

fn textual_rep_language_diagnostic(
    node_span: &crate::ast::Span,
    rep: &TextualRepresentation,
) -> Option<ParseError> {
    use crate::parser::diagnostic_catalog;
    if rep.language_span.is_none() {
        return Some(
            ParseError::new("rep body is missing the required 'language' keyword and string value")
                .with_location(node_span.offset, node_span.line, node_span.column)
                .with_length(node_span.len.max(1))
                .with_code(diagnostic_catalog::MISSING_REP_LANGUAGE)
                .with_severity(DiagnosticSeverity::Error)
                .with_category(DiagnosticCategory::ParseError),
        );
    }
    if rep.language.trim().is_empty() {
        let ls = rep.language_span.as_ref()?;
        return Some(
            ParseError::new("rep language value must be a non-empty string")
                .with_location(ls.offset, ls.line, ls.column)
                .with_length(ls.len.max(1))
                .with_code(diagnostic_catalog::INVALID_REP_LANGUAGE)
                .with_severity(DiagnosticSeverity::Error)
                .with_category(DiagnosticCategory::ParseError),
        );
    }
    None
}

fn unsupported_fallback_diagnostic(span: &crate::ast::Span, production: &str) -> ParseError {
    ParseError::new(format!(
        "the spec-valid {production} production is retained but not structurally implemented"
    ))
    .with_location(span.offset, span.line, span.column)
    .with_length(span.len.max(1))
    .with_code("unsupported_grammar_form")
    .with_severity(DiagnosticSeverity::Warning)
    .with_category(DiagnosticCategory::UnsupportedGrammarForm)
}

fn collect_requirement_body_errors(body: &RequirementDefBody, errors: &mut Vec<ParseError>) {
    match body {
        RequirementDefBody::Semicolon => {}
        RequirementDefBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    RequirementDefBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    RequirementDefBodyElement::Frame(n) => {
                        collect_requirement_body_errors(&n.value.body, errors)
                    }
                    RequirementDefBodyElement::RequirementUsage(n) => {
                        collect_requirement_body_errors(&n.value.body, errors)
                    }
                    RequirementDefBodyElement::TextualRep(n) => {
                        if let Some(diag) = textual_rep_language_diagnostic(&element.span, &n.value)
                        {
                            errors.push(diag);
                        }
                    }
                    RequirementDefBodyElement::Constraint(n) => {
                        collect_constraint_body_errors(&n.value.body, errors)
                    }
                    RequirementDefBodyElement::RequireConstraint(n) => match &n.value.body {
                        crate::ast::RequireConstraintBody::Semicolon => {}
                        crate::ast::RequireConstraintBody::Brace { elements } => {
                            collect_constraint_body_element_errors(elements, errors)
                        }
                    },
                    RequirementDefBodyElement::AttributeDef(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    RequirementDefBodyElement::AttributeUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    RequirementDefBodyElement::VariantUsage(n) => {
                        collect_variant_usage_errors(&n.value, errors)
                    }
                    RequirementDefBodyElement::VerifyRequirement(n) => {
                        if let Some(requirement) = &n.value.requirement {
                            collect_requirement_body_errors(&requirement.value.body, errors);
                        }
                    }
                    RequirementDefBodyElement::MetadataAnnotation(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    RequirementDefBodyElement::MetadataKeywordUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    RequirementDefBodyElement::Import(n) => collect_import_errors(&n.value, errors),
                    RequirementDefBodyElement::Other(_)
                    | RequirementDefBodyElement::Annotation(_)
                    | RequirementDefBodyElement::SubjectDecl(_)
                    | RequirementDefBodyElement::SubjectRef(_)
                    | RequirementDefBodyElement::RequirementActorDecl(_)
                    | RequirementDefBodyElement::Stakeholder(_)
                    | RequirementDefBodyElement::Purpose(_)
                    | RequirementDefBodyElement::Doc(_) => {}
                }
            }
        }
    }
}

fn collect_action_def_body_errors(body: &ActionDefBody, errors: &mut Vec<ParseError>) {
    match body {
        ActionDefBody::Semicolon => {}
        ActionDefBody::Brace { elements } => {
            for element in elements {
                collect_action_def_body_element_errors(element, errors);
            }
        }
    }
}

fn collect_first_merge_body_errors(body: &FirstMergeBody, errors: &mut Vec<ParseError>) {
    match body {
        FirstMergeBody::Semicolon => {}
        FirstMergeBody::Brace(body) => {
            for element in &body.value.elements {
                match &element.value {
                    FirstMergeBodyElement::Member(member) => {
                        collect_action_def_body_element_errors(member, errors)
                    }
                    FirstMergeBodyElement::Unsupported(unsupported) => {
                        errors.push(parse_error_from_recovery_node(
                            &element.span,
                            &unsupported.value.diagnostic,
                        ));
                    }
                    FirstMergeBodyElement::Error(error) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &error.value));
                    }
                }
            }
        }
    }
}

fn collect_action_def_body_element_errors(
    element: &crate::ast::Node<ActionDefBodyElement>,
    errors: &mut Vec<ParseError>,
) {
    match &element.value {
        ActionDefBodyElement::Error(n) => {
            errors.push(parse_error_from_recovery_node(&element.span, &n.value));
        }
        ActionDefBodyElement::MetadataAnnotation(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        ActionDefBodyElement::MetadataKeywordUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        ActionDefBodyElement::MetadataUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        ActionDefBodyElement::TextualRep(n) => {
            if let Some(diag) = textual_rep_language_diagnostic(&element.span, &n.value) {
                errors.push(diag);
            }
        }
        ActionDefBodyElement::RefDecl(n) => collect_ref_body_errors(&n.value.body, errors),
        ActionDefBodyElement::Perform(n) => collect_perform_body_errors(&n.value.body, errors),
        ActionDefBodyElement::Bind(n) => collect_bind_errors(&n.value, errors),
        ActionDefBodyElement::FlowUsage(n) => collect_definition_body_errors(&n.value.body, errors),
        ActionDefBodyElement::WhileStmt(n) => collect_action_def_body_errors(&n.value.body, errors),
        ActionDefBodyElement::LoopStmt(n) => collect_action_def_body_errors(&n.value.body, errors),
        ActionDefBodyElement::IfStmt(n) => collect_if_stmt_errors(&n.value, errors),
        ActionDefBodyElement::StateUsage(n) => collect_state_body_errors(&n.value.body, errors),
        ActionDefBodyElement::ActionUsage(n) => {
            collect_action_usage_body_errors(&n.value.body, errors)
        }
        ActionDefBodyElement::PartUsage(n) => collect_part_usage_body_errors(&n.value.body, errors),
        ActionDefBodyElement::ItemUsage(n) => collect_attribute_body_errors(&n.value.body, errors),
        ActionDefBodyElement::AssertConstraint(n) => {
            collect_constraint_body_errors(&n.value.body, errors)
        }
        ActionDefBodyElement::OccurrenceUsage(n) => {
            collect_occurrence_usage_body_errors(&n.value.body, errors)
        }
        ActionDefBodyElement::ForLoop(n) => collect_action_def_body_errors(&n.value.body, errors),
        ActionDefBodyElement::ThenAction(n) => collect_then_action_errors(&n.value, errors),
        ActionDefBodyElement::FirstStmt(n) => {
            collect_first_merge_body_errors(&n.value.body, errors)
        }
        ActionDefBodyElement::MergeStmt(n) => {
            collect_first_merge_body_errors(&n.value.body, errors)
        }
        ActionDefBodyElement::DecisionStmt(n) => {
            collect_first_merge_body_errors(&n.value.body, errors)
        }
        ActionDefBodyElement::JoinStmt(n) => collect_first_merge_body_errors(&n.value.body, errors),
        ActionDefBodyElement::ForkStmt(n) => collect_first_merge_body_errors(&n.value.body, errors),
        ActionDefBodyElement::InOutDecl(_)
        | ActionDefBodyElement::Doc(_)
        | ActionDefBodyElement::Annotation(_)
        | ActionDefBodyElement::TerminateStmt(_)
        | ActionDefBodyElement::Assign(_)
        | ActionDefBodyElement::Decl(_)
        | ActionDefBodyElement::DefaultReferenceUsage(_) => {}
    }
}

fn collect_ref_body_errors(body: &RefBody, errors: &mut Vec<ParseError>) {
    match body {
        RefBody::Semicolon => {}
        RefBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    RefBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    RefBodyElement::Action(n) => {
                        collect_action_def_body_element_errors(n, errors);
                    }
                    RefBodyElement::PartUsage(n) => {
                        collect_part_usage_body_element_errors(n, errors);
                    }
                    RefBodyElement::State(n) => {
                        collect_state_body_element_errors(n, errors);
                    }
                    RefBodyElement::TextualRep(n) => {
                        if let Some(diag) = textual_rep_language_diagnostic(&element.span, &n.value)
                        {
                            errors.push(diag);
                        }
                    }
                    RefBodyElement::Doc(_)
                    | RefBodyElement::Comment(_)
                    | RefBodyElement::Other(_) => {}
                    RefBodyElement::MetadataAnnotation(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                }
            }
        }
    }
}

fn collect_constraint_body_element_errors(
    elements: &[crate::ast::Node<ConstraintDefBodyElement>],
    errors: &mut Vec<ParseError>,
) {
    for element in elements {
        match &element.value {
            ConstraintDefBodyElement::Error(n) => {
                errors.push(parse_error_from_recovery_node(&element.span, &n.value));
            }
            ConstraintDefBodyElement::Constraint(n) => {
                collect_constraint_body_errors(&n.value.body, errors)
            }
            ConstraintDefBodyElement::AttributeUsage(n) => {
                collect_attribute_body_errors(&n.value.body, errors)
            }
            ConstraintDefBodyElement::MetadataAnnotation(n) => {
                collect_attribute_body_errors(&n.value.body, errors)
            }
            ConstraintDefBodyElement::Doc(_)
            | ConstraintDefBodyElement::InOutDecl(_)
            | ConstraintDefBodyElement::Expression(_)
            | ConstraintDefBodyElement::Other(_) => {}
        }
    }
}

fn collect_relationship_body_element_errors(
    elements: &[crate::ast::Node<RelationshipBodyElement>],
    errors: &mut Vec<ParseError>,
) {
    for element in elements {
        match &element.value {
            RelationshipBodyElement::Error(n) => {
                errors.push(parse_error_from_recovery_node(&element.span, &n.value));
            }
            RelationshipBodyElement::TextualRep(n) => {
                if let Some(diag) = textual_rep_language_diagnostic(&element.span, &n.value) {
                    errors.push(diag);
                }
            }
            RelationshipBodyElement::MetadataAnnotation(n) => {
                collect_attribute_body_errors(&n.value.body, errors)
            }
            RelationshipBodyElement::Doc(_)
            | RelationshipBodyElement::Comment(_)
            | RelationshipBodyElement::Other(_) => {}
        }
    }
}

fn collect_action_usage_body_errors(body: &ActionUsageBody, errors: &mut Vec<ParseError>) {
    match body {
        ActionUsageBody::Semicolon => {}
        ActionUsageBody::Brace { elements } => {
            for element in elements {
                collect_action_usage_body_element_errors(element, errors);
            }
        }
    }
}

fn collect_action_usage_body_element_errors(
    element: &crate::ast::Node<ActionUsageBodyElement>,
    errors: &mut Vec<ParseError>,
) {
    match &element.value {
        ActionUsageBodyElement::Error(n) => {
            errors.push(parse_error_from_recovery_node(&element.span, &n.value));
        }
        ActionUsageBodyElement::MetadataAnnotation(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::MetadataKeywordUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::MetadataUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::TextualRep(n) => {
            if let Some(diag) = textual_rep_language_diagnostic(&element.span, &n.value) {
                errors.push(diag);
            }
        }
        ActionUsageBodyElement::RefDecl(n) => collect_ref_body_errors(&n.value.body, errors),
        ActionUsageBodyElement::Bind(n) => collect_bind_errors(&n.value, errors),
        ActionUsageBodyElement::FlowUsage(n) => {
            collect_definition_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::WhileStmt(n) => {
            collect_action_def_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::LoopStmt(n) => {
            collect_action_def_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::IfStmt(n) => collect_if_stmt_errors(&n.value, errors),
        ActionUsageBodyElement::StateUsage(n) => collect_state_body_errors(&n.value.body, errors),
        ActionUsageBodyElement::ActionUsage(n) => {
            collect_action_usage_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::PartUsage(n) => {
            collect_part_usage_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::ItemUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::AssertConstraint(n) => {
            collect_constraint_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::OccurrenceUsage(n) => {
            collect_occurrence_usage_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::ForLoop(n) => collect_action_def_body_errors(&n.value.body, errors),
        ActionUsageBodyElement::ThenAction(n) => collect_then_action_errors(&n.value, errors),
        ActionUsageBodyElement::VariantUsage(n) => collect_variant_usage_errors(&n.value, errors),
        ActionUsageBodyElement::FirstStmt(n) => {
            collect_first_merge_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::MergeStmt(n) => {
            collect_first_merge_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::DecisionStmt(n) => {
            collect_first_merge_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::JoinStmt(n) => {
            collect_first_merge_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::ForkStmt(n) => {
            collect_first_merge_body_errors(&n.value.body, errors)
        }
        ActionUsageBodyElement::Doc(_)
        | ActionUsageBodyElement::Annotation(_)
        | ActionUsageBodyElement::InOutDecl(_)
        | ActionUsageBodyElement::TerminateStmt(_)
        | ActionUsageBodyElement::Assign(_)
        | ActionUsageBodyElement::Decl(_)
        | ActionUsageBodyElement::DefaultReferenceUsage(_) => {}
    }
}

fn collect_state_body_errors(body: &StateDefBody, errors: &mut Vec<ParseError>) {
    match body {
        StateDefBody::Semicolon => {}
        StateDefBody::Brace { elements } => {
            for element in elements {
                collect_state_body_element_errors(element, errors);
            }
        }
    }
}

fn collect_state_body_element_errors(
    element: &crate::ast::Node<StateDefBodyElement>,
    errors: &mut Vec<ParseError>,
) {
    match &element.value {
        StateDefBodyElement::Error(n) => {
            errors.push(parse_error_from_recovery_node(&element.span, &n.value));
        }
        StateDefBodyElement::Entry(n) => collect_state_body_errors(&n.value.body, errors),
        StateDefBodyElement::Do(n) => collect_state_body_errors(&n.value.body, errors),
        StateDefBodyElement::Exit(n) => collect_state_body_errors(&n.value.body, errors),
        StateDefBodyElement::Ref(n) => collect_ref_body_errors(&n.value.body, errors),
        StateDefBodyElement::RequirementUsage(n) => {
            collect_requirement_body_errors(&n.value.body, errors)
        }
        StateDefBodyElement::StateUsage(n) => collect_state_body_errors(&n.value.body, errors),
        StateDefBodyElement::MetadataAnnotation(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        StateDefBodyElement::MetadataKeywordUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        StateDefBodyElement::Doc(_)
        | StateDefBodyElement::Annotation(_)
        | StateDefBodyElement::Other(_)
        | StateDefBodyElement::InOutDecl(_)
        | StateDefBodyElement::Then(_)
        | StateDefBodyElement::FinalState(_)
        | StateDefBodyElement::Transition(_) => {}
    }
}

fn collect_use_case_body_errors(body: &UseCaseDefBody, errors: &mut Vec<ParseError>) {
    match body {
        UseCaseDefBody::Semicolon => {}
        UseCaseDefBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    UseCaseDefBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    UseCaseDefBodyElement::MetadataAnnotation(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    UseCaseDefBodyElement::MetadataKeywordUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    UseCaseDefBodyElement::AttributeDef(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    UseCaseDefBodyElement::Objective(n) => {
                        collect_requirement_body_errors(&n.value.requirement.value.body, errors)
                    }
                    UseCaseDefBodyElement::ThenIncludeUseCase(n) => {
                        collect_use_case_body_errors(&n.value.include.value.body, errors)
                    }
                    UseCaseDefBodyElement::ThenUseCaseUsage(n) => {
                        collect_use_case_body_errors(&n.value.use_case.value.body, errors)
                    }
                    UseCaseDefBodyElement::IncludeUseCase(n) => {
                        collect_use_case_body_errors(&n.value.body, errors)
                    }
                    UseCaseDefBodyElement::RefRedefinition(reference) => {
                        collect_use_case_body_errors(&reference.value.body.value, errors);
                    }
                    UseCaseDefBodyElement::ReturnRef(return_ref) => {
                        collect_return_ref_body_errors(&return_ref.value.body.value, errors)
                    }
                    UseCaseDefBodyElement::AssertConstraint(assert) => {
                        collect_constraint_body_errors(&assert.value.body, errors);
                    }
                    UseCaseDefBodyElement::ForLoop(n) => {
                        collect_action_def_body_errors(&n.value.body, errors)
                    }
                    UseCaseDefBodyElement::ThenAction(n) => {
                        collect_then_action_errors(&n.value, errors)
                    }
                    UseCaseDefBodyElement::ActionUsage(action) => {
                        collect_action_usage_body_errors(&action.value.body, errors);
                    }
                    UseCaseDefBodyElement::AnalysisCaseUsage(analysis) => {
                        collect_use_case_body_errors(&analysis.value.body, errors);
                    }
                    UseCaseDefBodyElement::RequirementUsage(requirement) => {
                        collect_requirement_body_errors(&requirement.value.body, errors);
                    }
                    UseCaseDefBodyElement::PartUsage(part) => {
                        collect_part_usage_body_errors(&part.value.body, errors);
                    }
                    UseCaseDefBodyElement::CalcUsage(calc) => {
                        collect_calc_body_errors(&calc.value.body, errors)
                    }
                    UseCaseDefBodyElement::AttributeUsage(attribute) => {
                        collect_attribute_body_errors(&attribute.value.body, errors)
                    }
                    UseCaseDefBodyElement::FlowUsage(flow) => {
                        collect_definition_body_errors(&flow.value.body, errors)
                    }
                    UseCaseDefBodyElement::Other(_)
                    | UseCaseDefBodyElement::Annotation(_)
                    | UseCaseDefBodyElement::Doc(_)
                    | UseCaseDefBodyElement::SubjectDecl(_)
                    | UseCaseDefBodyElement::SubjectRef(_)
                    | UseCaseDefBodyElement::ActorUsage(_)
                    | UseCaseDefBodyElement::ActorRedefinitionAssignment(_)
                    | UseCaseDefBodyElement::FirstSuccession(_)
                    | UseCaseDefBodyElement::ThenDone(_)
                    | UseCaseDefBodyElement::CaseReturnDecl(_)
                    | UseCaseDefBodyElement::Assign(_)
                    | UseCaseDefBodyElement::Expression(_) => {}
                }
            }
        }
    }
}

fn collect_return_ref_body_errors(body: &ReturnRefBody, errors: &mut Vec<ParseError>) {
    match body {
        ReturnRefBody::Semicolon => {}
        ReturnRefBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    ReturnRefBodyElement::Error(error) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &error.value))
                    }
                    ReturnRefBodyElement::Doc(_) | ReturnRefBodyElement::Result(_) => {}
                }
            }
        }
    }
}

fn collect_constraint_body_errors(body: &ConstraintDefBody, errors: &mut Vec<ParseError>) {
    match body {
        ConstraintDefBody::Semicolon => {}
        ConstraintDefBody::Brace { elements } => {
            collect_constraint_body_element_errors(elements, errors)
        }
    }
}

fn collect_calc_body_errors(body: &CalcDefBody, errors: &mut Vec<ParseError>) {
    match body {
        CalcDefBody::Semicolon => {}
        CalcDefBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    CalcDefBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    CalcDefBodyElement::CalcUsage(n) => {
                        collect_calc_body_errors(&n.value.body, errors)
                    }
                    CalcDefBodyElement::CalcDef(n) => {
                        collect_calc_body_errors(&n.value.body, errors)
                    }
                    CalcDefBodyElement::PartUsage(n) => {
                        collect_part_usage_body_errors(&n.value.body, errors)
                    }
                    CalcDefBodyElement::MetadataAnnotation(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    CalcDefBodyElement::Doc(_)
                    | CalcDefBodyElement::InOutDecl(_)
                    | CalcDefBodyElement::ReturnDecl(_)
                    | CalcDefBodyElement::Expression(_)
                    | CalcDefBodyElement::Other(_) => {}
                }
            }
        }
    }
}

fn collect_view_def_body_errors(body: &ViewDefBody, errors: &mut Vec<ParseError>) {
    match body {
        ViewDefBody::Semicolon => {}
        ViewDefBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    ViewDefBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    ViewDefBodyElement::ViewRendering(n) => {
                        collect_rendering_usage_body_errors(&n.value.body, errors)
                    }
                    ViewDefBodyElement::MetadataAnnotation(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    ViewDefBodyElement::Other(_)
                    | ViewDefBodyElement::Doc(_)
                    | ViewDefBodyElement::Filter(_) => {}
                }
            }
        }
    }
}

fn collect_view_body_errors(body: &ViewBody, errors: &mut Vec<ParseError>) {
    match body {
        ViewBody::Semicolon => {}
        ViewBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    ViewBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    ViewBodyElement::ViewRendering(n) => {
                        collect_rendering_usage_body_errors(&n.value.body, errors)
                    }
                    ViewBodyElement::Other(_)
                    | ViewBodyElement::Doc(_)
                    | ViewBodyElement::Filter(_)
                    | ViewBodyElement::Expose(_)
                    | ViewBodyElement::Satisfy(_) => {}
                }
            }
        }
    }
}

fn collect_attribute_body_errors(body: &AttributeBody, errors: &mut Vec<ParseError>) {
    match body {
        AttributeBody::Semicolon => {}
        AttributeBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    AttributeBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    AttributeBodyElement::AttributeDef(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    AttributeBodyElement::AttributeUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    AttributeBodyElement::OccurrenceUsage(n) => {
                        collect_occurrence_usage_body_errors(&n.value.body, errors)
                    }
                    AttributeBodyElement::AssertConstraint(n) => {
                        collect_constraint_body_errors(&n.value.body, errors)
                    }
                    AttributeBodyElement::RefDecl(n) => {
                        collect_ref_body_errors(&n.value.body, errors)
                    }
                    AttributeBodyElement::PartUsage(n) => {
                        collect_part_usage_body_errors(&n.value.body, errors)
                    }
                    AttributeBodyElement::MetadataKeywordUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    AttributeBodyElement::Doc(_)
                    | AttributeBodyElement::Connect(_)
                    | AttributeBodyElement::Other(_) => {}
                }
            }
        }
    }
}

fn collect_port_def_body_errors(body: &PortDefBody, errors: &mut Vec<ParseError>) {
    match body {
        PortDefBody::Semicolon => {}
        PortDefBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    PortDefBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    PortDefBodyElement::AttributeDef(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PortDefBodyElement::AttributeUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PortDefBodyElement::ItemDef(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PortDefBodyElement::ItemUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PortDefBodyElement::EnumerationUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PortDefBodyElement::PortUsage(n) => {
                        collect_port_body_errors(&n.value.body, errors)
                    }
                    PortDefBodyElement::MetadataKeywordUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PortDefBodyElement::InOutDecl(_)
                    | PortDefBodyElement::Doc(_)
                    | PortDefBodyElement::Other(_) => {}
                }
            }
        }
    }
}

fn collect_rendering_def_body_errors(body: &RenderingDefBody, errors: &mut Vec<ParseError>) {
    match body {
        RenderingDefBody::Semicolon => {}
        RenderingDefBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    RenderingDefBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    RenderingDefBodyElement::ViewRendering(n) => {
                        collect_rendering_usage_body_errors(&n.value.body, errors)
                    }
                    RenderingDefBodyElement::Doc(_)
                    | RenderingDefBodyElement::Filter(_)
                    | RenderingDefBodyElement::Other(_) => {}
                }
            }
        }
    }
}

fn collect_occurrence_usage_body_errors(body: &OccurrenceUsageBody, errors: &mut Vec<ParseError>) {
    match body {
        OccurrenceUsageBody::Semicolon => {}
        OccurrenceUsageBody::Brace { elements } => {
            for element in elements {
                collect_occurrence_body_element_errors(element, errors);
            }
        }
    }
}

fn collect_occurrence_body_element_errors(
    element: &crate::ast::Node<OccurrenceBodyElement>,
    errors: &mut Vec<ParseError>,
) {
    match &element.value {
        OccurrenceBodyElement::Error(n) => {
            errors.push(parse_error_from_recovery_node(&element.span, &n.value));
        }
        OccurrenceBodyElement::AssertConstraint(n) => {
            collect_constraint_body_errors(&n.value.body, errors)
        }
        OccurrenceBodyElement::FlowUsage(n) => {
            collect_definition_body_errors(&n.value.body, errors)
        }
        OccurrenceBodyElement::AttributeUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        OccurrenceBodyElement::PartUsage(n) => {
            collect_part_usage_body_errors(&n.value.body, errors)
        }
        OccurrenceBodyElement::ItemUsage(n) => collect_attribute_body_errors(&n.value.body, errors),
        OccurrenceBodyElement::OccurrenceUsage(n) => {
            collect_occurrence_usage_body_errors(&n.value.body, errors)
        }
        OccurrenceBodyElement::Satisfy(n) => {
            if let Some(elements) = &n.value.body_elements {
                collect_constraint_body_element_errors(elements, errors);
            }
        }
        OccurrenceBodyElement::EndDecl(n) => collect_end_decl_errors(&n.value, errors),
        OccurrenceBodyElement::StateUsage(n) => collect_state_body_errors(&n.value.body, errors),
        OccurrenceBodyElement::Doc(_)
        | OccurrenceBodyElement::Annotation(_)
        | OccurrenceBodyElement::Other(_)
        | OccurrenceBodyElement::SuccessionUsage(_)
        | OccurrenceBodyElement::Allocate(_) => {}
    }
}

fn collect_definition_body_errors(body: &DefinitionBody, errors: &mut Vec<ParseError>) {
    match body {
        DefinitionBody::Semicolon => {}
        DefinitionBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    DefinitionBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    DefinitionBodyElement::OccurrenceMember(n) => {
                        collect_occurrence_body_element_errors(n, errors)
                    }
                    DefinitionBodyElement::Doc(_) | DefinitionBodyElement::Other(_) => {}
                }
            }
        }
    }
}

fn collect_part_def_body_errors(body: &PartDefBody, errors: &mut Vec<ParseError>) {
    match body {
        PartDefBody::Semicolon => {}
        PartDefBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    PartDefBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    PartDefBodyElement::UnsupportedMember(n) => {
                        errors.push(parse_error_from_recovery_node(
                            &element.span,
                            &n.value.diagnostic,
                        ));
                    }
                    PartDefBodyElement::PartUsage(n) => {
                        collect_part_usage_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::PartDef(n) => {
                        collect_part_def_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::Perform(n) => {
                        collect_perform_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::AttributeDef(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::AttributeUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::RequirementUsage(n) => {
                        collect_requirement_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::ExhibitState(n) => {
                        collect_state_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::OccurrenceUsage(n) => {
                        collect_occurrence_usage_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::ConnectionDef(n) => {
                        collect_connection_def_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::InterfaceDef(n) => {
                        collect_interface_def_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::Dependency(n) => {
                        if let Some(elements) = &n.value.body_elements {
                            collect_relationship_body_element_errors(elements, errors);
                        }
                    }
                    PartDefBodyElement::ItemDef(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::ItemUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::Ref(n) => collect_ref_body_errors(&n.value.body, errors),
                    PartDefBodyElement::PortUsage(n) => {
                        collect_port_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::InterfaceUsage(n) => {
                        collect_interface_usage_errors(&n.value, errors)
                    }
                    PartDefBodyElement::FlowUsage(n) => {
                        collect_definition_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::Connection(n) => {
                        collect_connection_def_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::CalcUsage(n) => {
                        collect_calc_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::ConstraintDef(n) => {
                        collect_constraint_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::ConstraintUsage(n) => {
                        collect_constraint_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::Import(n) => collect_import_errors(&n.value, errors),
                    PartDefBodyElement::ActionUsage(n) => {
                        collect_action_usage_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::ActionDef(n) => {
                        collect_action_def_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::StateUsage(n) => {
                        collect_state_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::EnumerationUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::AssertConstraint(n) => {
                        collect_constraint_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::Satisfy(n) => {
                        if let Some(elements) = &n.value.body_elements {
                            collect_constraint_body_element_errors(elements, errors);
                        }
                    }
                    PartDefBodyElement::VariantUsage(n) => {
                        collect_variant_usage_errors(&n.value, errors)
                    }
                    PartDefBodyElement::StateDef(n) => {
                        collect_state_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::MetadataDef(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::MetadataUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::FlowDef(n) => {
                        collect_definition_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::RequirementDef(n) => {
                        collect_requirement_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::OccurrenceDef(n) => {
                        collect_definition_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::PortDef(n) => {
                        collect_port_def_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::CalcDef(n) => {
                        collect_calc_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::AllocationDef(n) => {
                        collect_definition_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::AllocationUsage(n) => {
                        collect_definition_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::ViewDef(n) => {
                        collect_view_def_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::ViewUsage(n) => {
                        collect_view_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::ViewpointDef(n) => {
                        collect_requirement_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::ViewpointUsage(n) => {
                        collect_requirement_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::RenderingDef(n) => {
                        collect_rendering_def_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::RenderingUsage(n) => {
                        collect_rendering_usage_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::CaseDef(n) => {
                        collect_use_case_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::CaseUsage(n) => {
                        collect_use_case_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::UseCaseDef(n) => {
                        collect_use_case_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::UseCaseUsage(n) => {
                        collect_use_case_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::AnalysisCaseDef(n) => {
                        collect_use_case_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::AnalysisCaseUsage(n) => {
                        collect_use_case_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::VerificationCaseDef(n) => {
                        collect_use_case_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::VerificationCaseUsage(n) => {
                        collect_use_case_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::Bind(n) => collect_bind_errors(&n.value, errors),
                    PartDefBodyElement::AliasDef(n) => {
                        collect_alias_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::MetadataAnnotation(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::MetadataKeywordUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    PartDefBodyElement::Doc(_)
                    | PartDefBodyElement::Comment(_)
                    | PartDefBodyElement::Annotation(_)
                    | PartDefBodyElement::Other(_)
                    | PartDefBodyElement::DefaultReferenceUsage(_)
                    | PartDefBodyElement::Connect(_)
                    | PartDefBodyElement::Allocate(_)
                    | PartDefBodyElement::EnumDef(_) => {}
                    PartDefBodyElement::FirstStmt(n) => {
                        collect_first_merge_body_errors(&n.value.body, errors)
                    }
                }
            }
        }
    }
}

fn collect_perform_body_errors(body: &crate::ast::PerformBody, errors: &mut Vec<ParseError>) {
    match body {
        crate::ast::PerformBody::Semicolon => {}
        crate::ast::PerformBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    crate::ast::PerformBodyElement::PartUsage(n) => {
                        collect_part_usage_body_errors(&n.value.body, errors)
                    }
                    crate::ast::PerformBodyElement::AttributeUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    crate::ast::PerformBodyElement::Action(n) => {
                        collect_action_usage_body_element_errors(n, errors)
                    }
                    crate::ast::PerformBodyElement::Variant(n) => {
                        collect_variant_usage_errors(&n.value, errors)
                    }
                    crate::ast::PerformBodyElement::ItemUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    crate::ast::PerformBodyElement::Doc(_)
                    | crate::ast::PerformBodyElement::InOut(_) => {}
                }
            }
        }
    }
}

fn collect_part_usage_body_errors(body: &PartUsageBody, errors: &mut Vec<ParseError>) {
    match body {
        PartUsageBody::Semicolon => {}
        PartUsageBody::Brace { elements } => {
            for element in elements {
                collect_part_usage_body_element_errors(element, errors);
            }
        }
    }
}

fn collect_part_usage_body_element_errors(
    element: &crate::ast::Node<PartUsageBodyElement>,
    errors: &mut Vec<ParseError>,
) {
    match &element.value {
        PartUsageBodyElement::Error(n) => {
            errors.push(parse_error_from_recovery_node(&element.span, &n.value));
        }
        PartUsageBodyElement::AttributeUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::EnumerationUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::PartUsage(n) => collect_part_usage_body_errors(&n.value.body, errors),
        PartUsageBodyElement::OccurrenceUsage(n) => {
            collect_occurrence_usage_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::PortUsage(n) => collect_port_body_errors(&n.value.body, errors),
        PartUsageBodyElement::Bind(n) => collect_bind_errors(&n.value, errors),
        PartUsageBodyElement::Ref(n) => collect_ref_body_errors(&n.value.body, errors),
        PartUsageBodyElement::InterfaceUsage(n) => collect_interface_usage_errors(&n.value, errors),
        PartUsageBodyElement::FlowUsage(n) => collect_definition_body_errors(&n.value.body, errors),
        PartUsageBodyElement::Perform(n) => collect_perform_body_errors(&n.value.body, errors),
        PartUsageBodyElement::StateUsage(n) => collect_state_body_errors(&n.value.body, errors),
        PartUsageBodyElement::Satisfy(n) => {
            if let Some(elems) = &n.value.body_elements {
                collect_constraint_body_element_errors(elems, errors);
            }
        }
        PartUsageBodyElement::ConnectionDef(n) => {
            collect_connection_def_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::ActionUsage(n) => {
            collect_action_usage_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::VariantUsage(n) => collect_variant_usage_errors(&n.value, errors),
        PartUsageBodyElement::StateDef(n) => collect_state_body_errors(&n.value.body, errors),
        PartUsageBodyElement::MetadataDef(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::FlowDef(n) => collect_definition_body_errors(&n.value.body, errors),
        PartUsageBodyElement::RequirementDef(n) => {
            collect_requirement_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::OccurrenceDef(n) => {
            collect_definition_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::PortDef(n) => collect_port_def_body_errors(&n.value.body, errors),
        PartUsageBodyElement::CalcDef(n) => collect_calc_body_errors(&n.value.body, errors),
        PartUsageBodyElement::Connection(n) => {
            collect_connection_def_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::AssertConstraint(n) => {
            collect_constraint_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::ConstraintDef(n) => {
            collect_constraint_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::ConstraintUsage(n) => {
            collect_constraint_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::CalcUsage(n) => collect_calc_body_errors(&n.value.body, errors),
        PartUsageBodyElement::Import(n) => collect_import_errors(&n.value, errors),
        PartUsageBodyElement::RequirementUsage(n) => {
            collect_requirement_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::ItemDef(n) => collect_attribute_body_errors(&n.value.body, errors),
        PartUsageBodyElement::ItemUsage(n) => collect_attribute_body_errors(&n.value.body, errors),
        PartUsageBodyElement::MetadataUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::MetadataAnnotation(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::MetadataKeywordUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::AnalysisCaseDef(n) => {
            collect_use_case_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::AnalysisCaseUsage(n) => {
            collect_use_case_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::AliasDef(n) => collect_alias_body_errors(&n.value.body, errors),
        PartUsageBodyElement::IncludeUseCase(n) => {
            collect_use_case_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::UseCaseUsage(n) => {
            collect_use_case_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::VerificationCaseUsage(n) => {
            collect_use_case_body_errors(&n.value.body, errors)
        }
        PartUsageBodyElement::Doc(_)
        | PartUsageBodyElement::Annotation(_)
        | PartUsageBodyElement::DefaultReferenceUsage(_)
        | PartUsageBodyElement::Connect(_)
        | PartUsageBodyElement::SuccessionUsage(_)
        | PartUsageBodyElement::Allocate(_)
        | PartUsageBodyElement::EnumDef(_) => {}
    }
}

/// GH-51: `ConnectionDef`/`InterfaceDef` bodies previously had no dispatch arm anywhere in this
/// file, so their `Error` recovery nodes -- even `ConnectionDefBodyElement::Error`, which already
/// existed -- were never collected into `parse_with_diagnostics`'s `result.errors`, regardless of
/// nesting context. Fixing `interface_def_body`'s own recovery loop alone wasn't sufficient
/// without this.
fn collect_connection_def_body_errors(body: &ConnectionDefBody, errors: &mut Vec<ParseError>) {
    match body {
        ConnectionDefBody::Semicolon => {}
        ConnectionDefBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    ConnectionDefBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    ConnectionDefBodyElement::EndDecl(n) => {
                        collect_end_decl_errors(&n.value, errors)
                    }
                    ConnectionDefBodyElement::RefDecl(n) => {
                        collect_ref_body_errors(&n.value.body, errors)
                    }
                    ConnectionDefBodyElement::ConnectStmt(n) => {
                        collect_relationship_body_element_errors(&n.value.body_elements, errors)
                    }
                    ConnectionDefBodyElement::AttributeDef(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    ConnectionDefBodyElement::AttributeUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    ConnectionDefBodyElement::ItemDef(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    ConnectionDefBodyElement::ItemUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    ConnectionDefBodyElement::PortDef(n) => {
                        collect_port_def_body_errors(&n.value.body, errors)
                    }
                    ConnectionDefBodyElement::PortUsage(n) => {
                        collect_port_body_errors(&n.value.body, errors)
                    }
                    ConnectionDefBodyElement::AssertConstraint(n) => {
                        collect_constraint_body_errors(&n.value.body, errors)
                    }
                    ConnectionDefBodyElement::OccurrenceUsage(n) => {
                        collect_occurrence_usage_body_errors(&n.value.body, errors)
                    }
                    ConnectionDefBodyElement::PartUsage(n) => {
                        collect_part_usage_body_errors(&n.value.body, errors)
                    }
                    ConnectionDefBodyElement::Doc(_)
                    | ConnectionDefBodyElement::SuccessionUsage(_) => {}
                }
            }
        }
    }
}

/// See [`collect_connection_def_body_errors`].
fn collect_interface_def_body_errors(body: &InterfaceDefBody, errors: &mut Vec<ParseError>) {
    match body {
        InterfaceDefBody::Semicolon => {}
        InterfaceDefBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    InterfaceDefBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    InterfaceDefBodyElement::EndDecl(n) => {
                        collect_end_decl_errors(&n.value, errors)
                    }
                    InterfaceDefBodyElement::RefDecl(n) => {
                        collect_ref_body_errors(&n.value.body, errors)
                    }
                    InterfaceDefBodyElement::ConnectStmt(n) => {
                        collect_relationship_body_element_errors(&n.value.body_elements, errors)
                    }
                    InterfaceDefBodyElement::AttributeDef(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    InterfaceDefBodyElement::AttributeUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    InterfaceDefBodyElement::ItemDef(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    InterfaceDefBodyElement::ItemUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    InterfaceDefBodyElement::PortDef(n) => {
                        collect_port_def_body_errors(&n.value.body, errors)
                    }
                    InterfaceDefBodyElement::PortUsage(n) => {
                        collect_port_body_errors(&n.value.body, errors)
                    }
                    InterfaceDefBodyElement::FlowUsage(n) => {
                        collect_definition_body_errors(&n.value.body, errors)
                    }
                    InterfaceDefBodyElement::Doc(_) => {}
                }
            }
        }
    }
}

fn collect_if_stmt_errors(stmt: &crate::ast::IfStmt, errors: &mut Vec<ParseError>) {
    collect_action_def_body_errors(&stmt.then_body, errors);
    if let Some(else_body) = &stmt.else_body {
        collect_action_def_body_errors(else_body, errors);
    }
}

fn collect_then_action_errors(action: &crate::ast::ThenAction, errors: &mut Vec<ParseError>) {
    match &action.target {
        crate::ast::ThenTarget::Action(n) => {
            collect_action_usage_body_errors(&n.value.body, errors)
        }
        crate::ast::ThenTarget::Perform(n) => collect_perform_body_errors(&n.value.body, errors),
        crate::ast::ThenTarget::Merge(n) => collect_first_merge_body_errors(&n.value.body, errors),
        crate::ast::ThenTarget::Fork(n) => collect_first_merge_body_errors(&n.value.body, errors),
        crate::ast::ThenTarget::Decide(n) => collect_first_merge_body_errors(&n.value.body, errors),
        crate::ast::ThenTarget::Accept(_) | crate::ast::ThenTarget::Feature(_) => {}
    }
}

fn collect_variant_usage_errors(variant: &crate::ast::VariantUsage, errors: &mut Vec<ParseError>) {
    if let Some(body) = &variant.body {
        collect_part_usage_body_errors(body, errors);
    }
    if let Some(typed) = &variant.typed {
        match typed {
            crate::ast::VariantTypedUsage::Part(n) => {
                collect_part_usage_body_errors(&n.value.body, errors)
            }
            crate::ast::VariantTypedUsage::Attribute(n) => {
                collect_attribute_body_errors(&n.value.body, errors)
            }
            crate::ast::VariantTypedUsage::Item(n) => {
                collect_attribute_body_errors(&n.value.body, errors)
            }
            crate::ast::VariantTypedUsage::Port(n) => {
                collect_port_body_errors(&n.value.body, errors)
            }
            crate::ast::VariantTypedUsage::Perform(n) => {
                collect_perform_body_errors(&n.value.body, errors)
            }
        }
    }
}

fn collect_bind_errors(bind: &crate::ast::Bind, errors: &mut Vec<ParseError>) {
    for element in &bind.body_elements {
        collect_part_usage_body_element_errors(element, errors);
    }
}

fn collect_alias_body_errors(body: &AliasBody, errors: &mut Vec<ParseError>) {
    match body {
        AliasBody::Semicolon => {}
        AliasBody::Brace { elements } => collect_relationship_body_element_errors(elements, errors),
    }
}

fn collect_import_errors(import: &crate::ast::Import, errors: &mut Vec<ParseError>) {
    if let Some(elements) = &import.body_elements {
        collect_relationship_body_element_errors(elements, errors);
    }
}

fn collect_port_body_errors(body: &crate::ast::PortBody, errors: &mut Vec<ParseError>) {
    match body {
        crate::ast::PortBody::Semicolon => {}
        crate::ast::PortBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    crate::ast::PortBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    crate::ast::PortBodyElement::PortUsage(n) => {
                        collect_port_body_errors(&n.value.body, errors)
                    }
                    crate::ast::PortBodyElement::AttributeUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    crate::ast::PortBodyElement::ItemUsage(n) => {
                        collect_attribute_body_errors(&n.value.body, errors)
                    }
                    crate::ast::PortBodyElement::InOutDecl(_)
                    | crate::ast::PortBodyElement::Doc(_) => {}
                }
            }
        }
    }
}

fn collect_rendering_usage_body_errors(
    body: &crate::ast::RenderingUsageBody,
    errors: &mut Vec<ParseError>,
) {
    match body {
        crate::ast::RenderingUsageBody::Semicolon => {}
        crate::ast::RenderingUsageBody::Brace { elements } => {
            for element in elements {
                match &element.value {
                    crate::ast::RenderingUsageBodyElement::Error(n) => {
                        errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                    }
                    crate::ast::RenderingUsageBodyElement::ViewUsage(n) => {
                        collect_view_body_errors(&n.value.body, errors)
                    }
                    crate::ast::RenderingUsageBodyElement::Doc(_) => {}
                }
            }
        }
    }
}

fn collect_end_decl_errors(end: &crate::ast::EndDecl, errors: &mut Vec<ParseError>) {
    if let Some(nested) = end.nested_usage.as_deref() {
        match nested {
            crate::ast::EndNestedUsage::Occurrence(n) => {
                collect_occurrence_usage_body_errors(&n.value.body, errors)
            }
            crate::ast::EndNestedUsage::Item(n) => {
                collect_attribute_body_errors(&n.value.body, errors)
            }
        }
    }
}

fn collect_interface_usage_errors(
    usage: &crate::ast::InterfaceUsage,
    errors: &mut Vec<ParseError>,
) {
    let elements = match usage {
        crate::ast::InterfaceUsage::TypedConnect { body_elements, .. }
        | crate::ast::InterfaceUsage::Connection { body_elements, .. }
        | crate::ast::InterfaceUsage::Declaration { body_elements, .. } => body_elements,
    };
    for element in elements {
        match &element.value {
            crate::ast::InterfaceUsageBodyElement::RefRedef { body, .. } => {
                collect_ref_body_errors(body, errors)
            }
            crate::ast::InterfaceUsageBodyElement::EndDecl(n) => {
                collect_end_decl_errors(&n.value, errors)
            }
            crate::ast::InterfaceUsageBodyElement::Doc(_) => {}
        }
    }
}

fn collect_package_body_errors(body: &PackageBody, errors: &mut Vec<ParseError>) {
    match body {
        PackageBody::Semicolon => {}
        PackageBody::Brace { elements } => {
            for element in elements {
                collect_package_body_element_errors(element, errors);
            }
        }
    }
}

fn collect_package_body_element_errors(
    element: &crate::ast::Node<PackageBodyElement>,
    errors: &mut Vec<ParseError>,
) {
    match &element.value {
        PackageBodyElement::Error(n) => {
            errors.push(parse_error_from_recovery_node(&element.span, &n.value));
        }
        PackageBodyElement::Unsupported(n) => {
            errors.push(parse_error_from_recovery_node(
                &element.span,
                &n.value.diagnostic,
            ));
        }
        // An extended-library fallback keeps the declaration the grammar could not model. Some of
        // those declarations are recognizable dialect forms with specific guidance to offer, so
        // classify the node's own text here: the alternative -- searching the whole document for
        // the dialect's spelling -- cannot tell a declaration from the same words inside a comment
        // or a string literal.
        PackageBodyElement::ExtendedLibraryDecl(n) => {
            if let Some((code, message, expected, suggestion)) =
                crate::parser::diagnostics::invalid_requirement_short_name_syntax_diagnostic(
                    n.value.text.as_bytes(),
                )
            {
                errors.push(
                    ParseError::new(message)
                        .with_location(element.span.offset, element.span.line, element.span.column)
                        .with_length(element.span.len)
                        .with_code(code)
                        .with_expected(expected)
                        .with_suggestion(suggestion)
                        .with_category(DiagnosticCategory::ParseError),
                );
            } else {
                errors.push(unsupported_fallback_diagnostic(
                    &element.span,
                    "extended-library declaration",
                ));
            }
        }
        PackageBodyElement::FeatureDecl(_) => errors.push(unsupported_fallback_diagnostic(
            &element.span,
            "KerML feature declaration",
        )),
        PackageBodyElement::ClassifierDecl(_) => errors.push(unsupported_fallback_diagnostic(
            &element.span,
            "KerML classifier declaration",
        )),
        PackageBodyElement::KermlSemanticDecl(_) => errors.push(unsupported_fallback_diagnostic(
            &element.span,
            "KerML semantic declaration",
        )),
        PackageBodyElement::KermlFeatureDecl(_) => errors.push(unsupported_fallback_diagnostic(
            &element.span,
            "KerML feature form",
        )),
        // Structurally recognized: keyword, optional name, optional multiplicity, `;`. No
        // diagnostic -- this is not a fallback node.
        PackageBodyElement::KermlBareDeclaration(_) => {}
        PackageBodyElement::Package(n) => collect_package_body_errors(&n.value.body, errors),
        PackageBodyElement::LibraryPackage(n) => collect_package_body_errors(&n.value.body, errors),
        PackageBodyElement::Import(n) => collect_import_errors(&n.value, errors),
        PackageBodyElement::PartDef(n) => collect_part_def_body_errors(&n.value.body, errors),
        PackageBodyElement::PartUsage(n) => collect_part_usage_body_errors(&n.value.body, errors),
        PackageBodyElement::PortDef(n) => collect_port_def_body_errors(&n.value.body, errors),
        PackageBodyElement::ConnectionDef(n) => {
            collect_connection_def_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::InterfaceDef(n) => {
            collect_interface_def_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::AttributeDef(n) => collect_attribute_body_errors(&n.value.body, errors),
        PackageBodyElement::ActionDef(n) => collect_action_def_body_errors(&n.value.body, errors),
        PackageBodyElement::ActionUsage(n) => {
            collect_action_usage_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::RequirementDef(n) => {
            collect_requirement_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::RequirementUsage(n) => {
            collect_requirement_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::UseCaseDef(n) => collect_use_case_body_errors(&n.value.body, errors),
        PackageBodyElement::UseCaseUsage(n) => collect_use_case_body_errors(&n.value.body, errors),
        PackageBodyElement::CaseDef(n) => collect_use_case_body_errors(&n.value.body, errors),
        PackageBodyElement::CaseUsage(n) => collect_use_case_body_errors(&n.value.body, errors),
        PackageBodyElement::AnalysisCaseDef(n) => {
            collect_use_case_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::AnalysisCaseUsage(n) => {
            collect_use_case_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::VerificationCaseDef(n) => {
            collect_use_case_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::VerificationCaseUsage(n) => {
            collect_use_case_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::ConcernUsage(n) => {
            collect_requirement_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::ViewpointDef(n) => {
            collect_requirement_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::ViewpointUsage(n) => {
            collect_requirement_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::StateDef(n) => collect_state_body_errors(&n.value.body, errors),
        PackageBodyElement::StateUsage(n) => collect_state_body_errors(&n.value.body, errors),
        PackageBodyElement::ConstraintDef(n) => {
            collect_constraint_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::ConstraintUsage(n) => {
            collect_constraint_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::CalcDef(n) => collect_calc_body_errors(&n.value.body, errors),
        PackageBodyElement::ViewDef(n) => collect_view_def_body_errors(&n.value.body, errors),
        PackageBodyElement::ViewUsage(n) => collect_view_body_errors(&n.value.body, errors),
        PackageBodyElement::RenderingDef(n) => {
            collect_rendering_def_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::RenderingUsage(n) => {
            collect_rendering_usage_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::MetadataDef(n) => collect_attribute_body_errors(&n.value.body, errors),
        PackageBodyElement::MetadataUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::ItemDef(n) => collect_attribute_body_errors(&n.value.body, errors),
        PackageBodyElement::IndividualDef(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::OccurrenceDef(n) => {
            collect_definition_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::OccurrenceUsage(n) => {
            collect_occurrence_usage_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::AllocationDef(n) => {
            collect_definition_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::AllocationUsage(n) => {
            collect_definition_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::FlowDef(n) => collect_definition_body_errors(&n.value.body, errors),
        PackageBodyElement::FlowUsage(n) => collect_definition_body_errors(&n.value.body, errors),
        PackageBodyElement::Satisfy(n) => {
            if let Some(elems) = &n.value.body_elements {
                collect_constraint_body_element_errors(elems, errors);
            }
        }
        PackageBodyElement::AliasDef(n) => {
            collect_alias_body_errors(&n.value.body, errors);
        }
        PackageBodyElement::Dependency(n) => {
            if let Some(elems) = &n.value.body_elements {
                collect_relationship_body_element_errors(elems, errors);
            }
        }
        PackageBodyElement::TextualRep(n) => {
            if let Some(diag) = textual_rep_language_diagnostic(&element.span, &n.value) {
                errors.push(diag);
            }
        }
        PackageBodyElement::Doc(_)
        | PackageBodyElement::Comment(_)
        | PackageBodyElement::Filter(_)
        | PackageBodyElement::Actor(_)
        | PackageBodyElement::EnumDef(_)
        | PackageBodyElement::Connect(_)
        | PackageBodyElement::DefaultReferenceUsage(_) => {}
        PackageBodyElement::MetadataKeywordUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::MetadataAnnotation(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::AttributeUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::ItemUsage(n) => collect_attribute_body_errors(&n.value.body, errors),
        PackageBodyElement::PortUsage(n) => collect_port_body_errors(&n.value.body, errors),
        PackageBodyElement::ConnectionUsage(n) => {
            collect_connection_def_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::InterfaceUsage(n) => collect_interface_usage_errors(&n.value, errors),
        PackageBodyElement::Ref(n) => collect_ref_body_errors(&n.value.body, errors),
        PackageBodyElement::EnumerationUsage(n) => {
            collect_attribute_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::AssertConstraint(n) => {
            collect_constraint_body_errors(&n.value.body, errors)
        }
        PackageBodyElement::PerformUsage(n) => collect_perform_body_errors(&n.value.body, errors),
        PackageBodyElement::BindingConnectorUsage(_) => {}
        PackageBodyElement::ClassDef(n) => collect_attribute_body_errors(&n.value.body, errors),
        PackageBodyElement::Succession(n) => collect_first_merge_body_errors(&n.value.body, errors),
        PackageBodyElement::ExhibitState(n) => collect_state_body_errors(&n.value.body, errors),
        PackageBodyElement::IncludeUseCase(n) => {
            collect_use_case_body_errors(&n.value.body, errors)
        }
    }
}

pub(crate) fn collect_recovery_errors(root: &RootNamespace) -> Vec<ParseError> {
    let mut errors = Vec::new();
    for element in &root.elements {
        match &element.value {
            crate::ast::RootElement::Package(n) => {
                collect_package_body_errors(&n.value.body, &mut errors)
            }
            crate::ast::RootElement::LibraryPackage(n) => {
                collect_package_body_errors(&n.value.body, &mut errors)
            }
            crate::ast::RootElement::Namespace(n) => {
                collect_package_body_errors(&n.value.body, &mut errors)
            }
            crate::ast::RootElement::Import(n) => collect_import_errors(&n.value, &mut errors),
            crate::ast::RootElement::Member(n) => {
                collect_package_body_element_errors(n, &mut errors)
            }
        }
    }
    errors
}
