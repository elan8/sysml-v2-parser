//! Collect ParseError diagnostics from recovery nodes embedded in the AST.

use super::diagnostics::{bare_feature_declaration_in_part_def_diagnostic, trim_ascii_start};
use super::recovery::parse_error_from_recovery_node;
use crate::ast::{
    ActionDefBody, ActionDefBodyElement, ActionUsageBody, ActionUsageBodyElement,
    AttributeBody, AttributeBodyElement, CalcDefBody, CalcDefBodyElement, ConstraintDefBody,
    ConstraintDefBodyElement, DefinitionBody, DefinitionBodyElement, OccurrenceBodyElement,
    OccurrenceUsageBody, PackageBody, PackageBodyElement, PartDefBody, PartDefBodyElement,
    PartUsageBody, PartUsageBodyElement, PortDefBody, PortDefBodyElement, RefBody,
    RenderingDefBody, RenderingDefBodyElement, RequirementDefBody, RequirementDefBodyElement,
    RootNamespace, StateDefBody, StateDefBodyElement, TextualRepresentation, UseCaseDefBody,
    UseCaseDefBodyElement, ViewBody, ViewBodyElement, ViewDefBody, ViewDefBodyElement,
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

fn collect_requirement_body_errors(body: &RequirementDefBody, errors: &mut Vec<ParseError>) {
    if let RequirementDefBody::Brace { elements } = body {
        for element in elements {
            match &element.value {
                RequirementDefBodyElement::Error(n) => {
                    errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                }
                RequirementDefBodyElement::Frame(n) => {
                    collect_requirement_body_errors(&n.value.body, errors)
                }
                RequirementDefBodyElement::TextualRep(n) => {
                    if let Some(diag) = textual_rep_language_diagnostic(&element.span, &n.value) {
                        errors.push(diag);
                    }
                }
                _ => {}
            }
        }
    }
}

fn collect_action_def_body_errors(body: &ActionDefBody, errors: &mut Vec<ParseError>) {
    if let ActionDefBody::Brace { elements } = body {
        for element in elements {
            match &element.value {
                ActionDefBodyElement::Error(n) => {
                    errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                }
                ActionDefBodyElement::RefDecl(n) => {
                    collect_ref_body_errors(&n.value.body, errors);
                }
                _ => {}
            }
        }
    }
}

fn collect_ref_body_errors(body: &RefBody, errors: &mut Vec<ParseError>) {
    if let RefBody::Brace { elements } = body {
        for element in elements {
            if let ActionDefBodyElement::Error(n) = &element.value {
                errors.push(parse_error_from_recovery_node(&element.span, &n.value));
            }
        }
    }
}

fn collect_constraint_body_element_errors(
    elements: &[crate::ast::Node<ConstraintDefBodyElement>],
    errors: &mut Vec<ParseError>,
) {
    for element in elements {
        if let ConstraintDefBodyElement::Error(n) = &element.value {
            errors.push(parse_error_from_recovery_node(&element.span, &n.value));
        }
    }
}

fn collect_action_usage_body_errors(body: &ActionUsageBody, errors: &mut Vec<ParseError>) {
    if let ActionUsageBody::Brace { elements } = body {
        for element in elements {
            match &element.value {
                ActionUsageBodyElement::Error(n) => {
                    errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                }
                ActionUsageBodyElement::ActionUsage(n) => {
                    collect_action_usage_body_errors(&n.value.body, errors)
                }
                _ => {}
            }
        }
    }
}

fn collect_state_body_errors(body: &StateDefBody, errors: &mut Vec<ParseError>) {
    if let StateDefBody::Brace { elements } = body {
        for element in elements {
            match &element.value {
                StateDefBodyElement::Error(n) => {
                    errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                }
                StateDefBodyElement::Entry(n) => collect_state_body_errors(&n.value.body, errors),
                StateDefBodyElement::Do(n) => collect_state_body_errors(&n.value.body, errors),
                StateDefBodyElement::Exit(n) => collect_state_body_errors(&n.value.body, errors),
                StateDefBodyElement::RequirementUsage(n) => {
                    collect_requirement_body_errors(&n.value.body, errors)
                }
                StateDefBodyElement::StateUsage(n) => {
                    collect_state_body_errors(&n.value.body, errors)
                }
                _ => {}
            }
        }
    }
}

fn collect_use_case_body_errors(body: &UseCaseDefBody, errors: &mut Vec<ParseError>) {
    if let UseCaseDefBody::Brace { elements } = body {
        for element in elements {
            if let UseCaseDefBodyElement::Error(n) = &element.value {
                errors.push(parse_error_from_recovery_node(&element.span, &n.value));
            }
        }
    }
}

fn collect_constraint_body_errors(body: &ConstraintDefBody, errors: &mut Vec<ParseError>) {
    if let ConstraintDefBody::Brace { elements } = body {
        for element in elements {
            if let ConstraintDefBodyElement::Error(n) = &element.value {
                errors.push(parse_error_from_recovery_node(&element.span, &n.value));
            }
        }
    }
}

fn collect_calc_body_errors(body: &CalcDefBody, errors: &mut Vec<ParseError>) {
    if let CalcDefBody::Brace { elements } = body {
        for element in elements {
            if let CalcDefBodyElement::Error(n) = &element.value {
                errors.push(parse_error_from_recovery_node(&element.span, &n.value));
            }
        }
    }
}

fn collect_view_def_body_errors(body: &ViewDefBody, errors: &mut Vec<ParseError>) {
    if let ViewDefBody::Brace { elements } = body {
        for element in elements {
            if let ViewDefBodyElement::Error(n) = &element.value {
                errors.push(parse_error_from_recovery_node(&element.span, &n.value));
            }
        }
    }
}

fn collect_view_body_errors(body: &ViewBody, errors: &mut Vec<ParseError>) {
    if let ViewBody::Brace { elements } = body {
        for element in elements {
            if let ViewBodyElement::Error(n) = &element.value {
                errors.push(parse_error_from_recovery_node(&element.span, &n.value));
            }
        }
    }
}

fn collect_attribute_body_errors(body: &AttributeBody, errors: &mut Vec<ParseError>) {
    if let AttributeBody::Brace { elements } = body {
        for element in elements {
            if let AttributeBodyElement::Error(n) = &element.value {
                errors.push(parse_error_from_recovery_node(&element.span, &n.value));
            }
        }
    }
}

fn collect_port_def_body_errors(body: &PortDefBody, errors: &mut Vec<ParseError>) {
    if let PortDefBody::Brace { elements } = body {
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
                _ => {}
            }
        }
    }
}

fn collect_rendering_def_body_errors(body: &RenderingDefBody, errors: &mut Vec<ParseError>) {
    if let RenderingDefBody::Brace { elements } = body {
        for element in elements {
            if let RenderingDefBodyElement::Error(n) = &element.value {
                errors.push(parse_error_from_recovery_node(&element.span, &n.value));
            }
        }
    }
}

fn collect_occurrence_usage_body_errors(
    body: &OccurrenceUsageBody,
    errors: &mut Vec<ParseError>,
) {
    if let OccurrenceUsageBody::Brace { elements } = body {
        for element in elements {
            match &element.value {
                OccurrenceBodyElement::Error(n) => {
                    errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                }
                OccurrenceBodyElement::PartUsage(n) => {
                    collect_part_usage_body_errors(&n.value.body, errors)
                }
                OccurrenceBodyElement::OccurrenceUsage(n) => {
                    collect_occurrence_usage_body_errors(&n.value.body, errors)
                }
                _ => {}
            }
        }
    }
}

fn collect_definition_body_errors(body: &DefinitionBody, errors: &mut Vec<ParseError>) {
    if let DefinitionBody::Brace { elements } = body {
        for element in elements {
            match &element.value {
                DefinitionBodyElement::Error(n) => {
                    errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                }
                DefinitionBodyElement::OccurrenceMember(n) => match &n.value {
                    OccurrenceBodyElement::Error(err) => {
                        errors.push(parse_error_from_recovery_node(&n.span, &err.value));
                    }
                    OccurrenceBodyElement::PartUsage(p) => {
                        collect_part_usage_body_errors(&p.value.body, errors)
                    }
                    OccurrenceBodyElement::OccurrenceUsage(o) => {
                        collect_occurrence_usage_body_errors(&o.value.body, errors)
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn collect_part_def_body_errors(body: &PartDefBody, errors: &mut Vec<ParseError>) {
    if let PartDefBody::Brace { elements } = body {
        for element in elements {
            match &element.value {
                PartDefBodyElement::Error(n) => {
                    errors.push(parse_error_from_recovery_node(&element.span, &n.value));
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
                _ => {}
            }
        }
    }
}

fn collect_perform_body_errors(body: &crate::ast::PerformBody, _errors: &mut Vec<ParseError>) {
    match body {
        crate::ast::PerformBody::Semicolon => {}
        crate::ast::PerformBody::Brace { .. } => {}
    }
}

fn collect_part_usage_body_errors(body: &PartUsageBody, errors: &mut Vec<ParseError>) {
    if let PartUsageBody::Brace { elements } = body {
        for element in elements {
            match &element.value {
                PartUsageBodyElement::Error(n) => {
                    errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                }
                PartUsageBodyElement::PartUsage(n) => {
                    collect_part_usage_body_errors(&n.value.body, errors)
                }
                PartUsageBodyElement::Perform(n) => {
                    collect_perform_body_errors(&n.value.body, errors)
                }
                PartUsageBodyElement::StateUsage(n) => {
                    collect_state_body_errors(&n.value.body, errors)
                }
                PartUsageBodyElement::AttributeUsage(n) => {
                    collect_attribute_body_errors(&n.value.body, errors)
                }
                PartUsageBodyElement::OccurrenceUsage(n) => {
                    collect_occurrence_usage_body_errors(&n.value.body, errors)
                }
                PartUsageBodyElement::Satisfy(n) => {
                    if let Some(elems) = &n.value.body_elements {
                        collect_constraint_body_element_errors(elems, errors);
                    }
                }
                _ => {}
            }
        }
    }
}

fn collect_package_body_errors(body: &PackageBody, errors: &mut Vec<ParseError>) {
    if let PackageBody::Brace { elements } = body {
        for element in elements {
            match &element.value {
                PackageBodyElement::Error(n) => {
                    errors.push(parse_error_from_recovery_node(&element.span, &n.value));
                }
                PackageBodyElement::Package(n) => {
                    collect_package_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::LibraryPackage(n) => {
                    collect_package_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::PartDef(n) => {
                    collect_part_def_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::PartUsage(n) => {
                    collect_part_usage_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::PortDef(n) => {
                    collect_port_def_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::AttributeDef(n) => {
                    collect_attribute_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::ActionDef(n) => {
                    collect_action_def_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::ActionUsage(n) => {
                    collect_action_usage_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::RequirementDef(n) => {
                    collect_requirement_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::RequirementUsage(n) => {
                    collect_requirement_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::UseCaseDef(n) => {
                    collect_use_case_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::UseCaseUsage(n) => {
                    collect_use_case_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::CaseDef(n) => {
                    collect_use_case_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::CaseUsage(n) => {
                    collect_use_case_body_errors(&n.value.body, errors)
                }
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
                PackageBodyElement::StateDef(n) => {
                    collect_state_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::StateUsage(n) => {
                    collect_state_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::ConstraintDef(n) => {
                    collect_constraint_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::CalcDef(n) => {
                    collect_calc_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::ViewDef(n) => {
                    collect_view_def_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::ViewUsage(n) => {
                    collect_view_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::RenderingDef(n) => {
                    collect_rendering_def_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::MetadataDef(n) => {
                    collect_attribute_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::MetadataUsage(n) => {
                    collect_attribute_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::ItemDef(n) => {
                    collect_attribute_body_errors(&n.value.body, errors)
                }
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
                PackageBodyElement::FlowDef(n) => {
                    collect_definition_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::FlowUsage(n) => {
                    collect_definition_body_errors(&n.value.body, errors)
                }
                PackageBodyElement::Satisfy(n) => {
                    if let Some(elems) = &n.value.body_elements {
                        collect_constraint_body_element_errors(elems, errors);
                    }
                }
                PackageBodyElement::TextualRep(n) => {
                    if let Some(diag) = textual_rep_language_diagnostic(&element.span, &n.value) {
                        errors.push(diag);
                    }
                }
                _ => {}
            }
        }
    }
}

pub(crate) fn collect_implicit_attribute_in_part_def_warnings(bytes: &[u8]) -> Vec<ParseError> {
    let text = String::from_utf8_lossy(bytes);
    let mut errors = Vec::new();
    let mut in_part_def_body = false;
    let mut brace_depth = 0i32;
    let mut offset = 0usize;
    for (line_idx, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("part def") {
            in_part_def_body = false;
            brace_depth = 0;
        }
        if trimmed.contains('{') {
            if in_part_def_body || trimmed.starts_with("part def") {
                in_part_def_body = true;
            }
            brace_depth += trimmed.chars().filter(|&c| c == '{').count() as i32;
        }
        if trimmed.contains('}') {
            brace_depth -= trimmed.chars().filter(|&c| c == '}').count() as i32;
            if brace_depth <= 0 {
                in_part_def_body = false;
            }
        }
        if in_part_def_body && brace_depth > 0 {
            let skip = trimmed.starts_with("attribute")
                || trimmed.starts_with("part ")
                || trimmed.starts_with("port ")
                || trimmed.starts_with("interface")
                || trimmed.starts_with("connect")
                || trimmed.contains(":>")
                || trimmed.contains("::>")
                || trimmed.is_empty()
                || trimmed.starts_with("//")
                || trimmed.starts_with("/*")
                || trimmed.starts_with("doc ");
            if !skip {
                if let Some((code, message, expected, suggestion)) =
                    bare_feature_declaration_in_part_def_diagnostic(trimmed.as_bytes())
                {
                    let line_no = (line_idx + 1) as u32;
                    let column = line.find(trimmed).unwrap_or(0) + 1;
                    let line_offset = offset + line.find(trimmed).unwrap_or(0);
                    errors.push(
                        ParseError::new(message)
                            .with_location(line_offset, line_no, column)
                            .with_length(trimmed.len().max(1))
                            .with_code(code)
                            .with_expected(expected)
                            .with_suggestion(suggestion)
                            .with_severity(DiagnosticSeverity::Warning)
                            .with_category(DiagnosticCategory::ParseError),
                    );
                }
            }
        }
        offset += line.len() + 1;
    }
    errors
}

pub(crate) fn collect_requirement_id_dialect_diagnostics(bytes: &[u8]) -> Vec<ParseError> {
    let pattern = b"requirement def id ";
    let mut errors = Vec::new();
    let mut search_from = 0usize;
    while search_from < bytes.len() {
        let Some(rel) = bytes[search_from..]
            .windows(pattern.len())
            .position(|window| window == pattern)
        else {
            break;
        };
        let offset = search_from + rel;
        let after = trim_ascii_start(&bytes[offset + pattern.len()..]);
        if after.first() != Some(&b'\'') && after.first() != Some(&b'"') {
            search_from = offset + 1;
            continue;
        }
        let quote = after[0];
        let Some(close) = after[1..].iter().position(|&b| b == quote) else {
            search_from = offset + 1;
            continue;
        };
        let req_id = String::from_utf8_lossy(&after[1..1 + close]);
        let (line, column) = offset_to_line_column(bytes, offset);
        errors.push(
            ParseError::new(format!(
                "requirement definition uses non-standard `id '{req_id}'` syntax; use a short name in angle brackets"
            ))
            .with_location(offset, line, column)
            .with_length(pattern.len().max(1))
            .with_code("invalid_requirement_short_name_syntax")
            .with_expected("short name in angle brackets after `requirement def`".to_string())
            .with_suggestion(format!(
                "Use `requirement def <'{req_id}'> ...` instead of `requirement def id '{req_id}' ...`."
            ))
            .with_category(DiagnosticCategory::ParseError),
        );
        search_from = offset + pattern.len();
    }
    errors
}

fn offset_to_line_column(bytes: &[u8], offset: usize) -> (u32, usize) {
    let mut line = 1u32;
    let mut column = 1usize;
    for (idx, &b) in bytes.iter().enumerate() {
        if idx >= offset {
            break;
        }
        if b == b'\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
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
            crate::ast::RootElement::Import(_) => {}
        }
    }
    errors
}
