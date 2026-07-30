#[path = "common/mod.rs"]
mod common;

use std::fs;

use common::{
    classify, classify_all, extract_productions, load_bnf_productions, load_conformance_target,
    manifest_dir, parse_coverage_rules, pattern_matches, release_root, CoverageStatus, Grammar,
};

fn assert_all_productions_are_classified(
    grammar: Grammar,
    productions: &[String],
    rules: &[common::CoverageRule],
) {
    let (counts, productions_by_status, errors) = classify_all(grammar, productions, rules);

    eprintln!("{grammar:?} BNF coverage counts: {counts:?}");
    for (status, productions) in &productions_by_status {
        eprintln!(
            "{grammar:?} {status:?} productions: {}",
            productions.join(", ")
        );
    }

    assert!(
        errors.is_empty(),
        "unclassified or ambiguous BNF productions:\n{}",
        errors.join("\n")
    );
}

#[test]
fn textual_bnf_productions_are_covered_by_status_map() {
    let target = load_conformance_target();
    let root = release_root();
    let sysml_bnf = root.join("bnf").join("SysML-textual-bnf.kebnf");
    let kerml_bnf = root.join("bnf").join("KerML-textual-bnf.kebnf");
    assert!(
        sysml_bnf.exists(),
        "SysML textual BNF not found at {}",
        sysml_bnf.display()
    );
    assert!(
        kerml_bnf.exists(),
        "KerML textual BNF not found at {}",
        kerml_bnf.display()
    );

    let rules = parse_coverage_rules(&manifest_dir().join("docs").join("bnf_coverage.map"));
    assert!(!rules.is_empty(), "coverage map must contain rules");

    let sysml = extract_productions(&sysml_bnf);
    let kerml = extract_productions(&kerml_bnf);
    assert_eq!(
        sysml.len(),
        target.sysml_bnf_productions,
        "unexpected SysML textual BNF production count for pin {}",
        target.release_tag
    );
    assert_eq!(
        kerml.len(),
        target.kerml_bnf_productions,
        "unexpected KerML textual BNF production count for pin {}",
        target.release_tag
    );

    assert_all_productions_are_classified(Grammar::SysML, &sysml, &rules);
    assert_all_productions_are_classified(Grammar::KerML, &kerml, &rules);
}

/// Wildcard map rules for Flow/Allocation/Metadata must not claim `implemented` while bodies
/// still use opaque skipping. Exact production names (e.g. `FlowDefinition`) are allowed.
#[test]
fn implemented_wildcard_patterns_do_not_target_opaque_body_helper_families() {
    let rules = parse_coverage_rules(&manifest_dir().join("docs").join("bnf_coverage.map"));
    let opaque_families = ["Flow", "Allocation", "Metadata"];
    let implemented_opaque_rules = rules
        .iter()
        .filter(|rule| rule.status == CoverageStatus::Implemented)
        .filter(|rule| rule.pattern.contains('*'))
        .filter(|rule| {
            opaque_families
                .iter()
                .any(|family| pattern_matches(&rule.pattern, family))
        })
        .collect::<Vec<_>>();

    assert!(
        implemented_opaque_rules.is_empty(),
        "opaque helper wildcard families must not be marked implemented: {implemented_opaque_rules:?}"
    );
}

#[test]
fn coverage_map_rules_use_no_partial_status() {
    let rules = parse_coverage_rules(&manifest_dir().join("docs").join("bnf_coverage.map"));
    let partial_rules: Vec<_> = rules
        .iter()
        .filter(|rule| rule.status == CoverageStatus::Partial)
        .map(|rule| format!("line {}: {:?} {}", rule.line, rule.grammar, rule.pattern))
        .collect();
    assert!(
        partial_rules.is_empty(),
        "bnf_coverage.map must not contain partial rules:\n{}",
        partial_rules.join("\n")
    );
}

#[test]
fn all_textual_bnf_productions_are_implemented() {
    let target = load_conformance_target();
    let (sysml, kerml, rules) = load_bnf_productions();
    assert_eq!(sysml.len(), target.sysml_bnf_productions);
    assert_eq!(kerml.len(), target.kerml_bnf_productions);

    for grammar in [Grammar::SysML, Grammar::KerML] {
        let productions = if grammar == Grammar::SysML {
            &sysml
        } else {
            &kerml
        };
        let (counts, _, errors) = classify_all(grammar, productions, &rules);
        assert!(
            errors.is_empty(),
            "{grammar:?} classification errors: {errors:?}"
        );
        assert_eq!(
            counts.get(&CoverageStatus::Partial).copied().unwrap_or(0),
            0,
            "{grammar:?} still has partial productions"
        );
        assert_eq!(
            counts
                .get(&CoverageStatus::Implemented)
                .copied()
                .unwrap_or(0),
            productions.len(),
            "{grammar:?} implemented count must equal production count"
        );
    }
}

#[test]
fn implemented_productions_do_not_use_skip_or_statement_only_bodies() {
    let rules = parse_coverage_rules(&manifest_dir().join("docs").join("bnf_coverage.map"));
    let guarded_productions = [
        ("AttributeDefinition", "src/parser/attribute.rs"),
        ("AttributeUsage", "src/parser/attribute.rs"),
        ("OccurrenceDefinition", "src/parser/occurrence.rs"),
        ("OccurrenceUsage", "src/parser/occurrence.rs"),
        ("PartDefinition", "src/parser/part/def.rs"),
        ("PartUsage", "src/parser/part/usage.rs"),
        ("PortDefinition", "src/parser/port.rs"),
        ("PortUsage", "src/parser/port.rs"),
        ("ConnectionDefinition", "src/parser/connection.rs"),
        ("InterfaceDefinition", "src/parser/interface.rs"),
        ("EnumerationDefinition", "src/parser/enumeration.rs"),
        ("RenderingDefinition", "src/parser/view.rs"),
        ("FlowDefinition", "src/parser/flow.rs"),
        ("FlowUsage", "src/parser/flow.rs"),
        ("AllocationDefinition", "src/parser/allocation.rs"),
        ("AllocationUsage", "src/parser/allocation.rs"),
        ("MetadataDefinition", "src/parser/metadata.rs"),
        ("MetadataUsage", "src/parser/metadata.rs"),
        ("ActionDefinition", "src/parser/action.rs"),
        ("StateDefinition", "src/parser/state.rs"),
        ("RequirementDefinition", "src/parser/requirement.rs"),
    ];

    let mut violations = Vec::new();
    for (production, parser_path) in guarded_productions {
        let rule = classify(&rules, Grammar::SysML, production)
            .unwrap_or_else(|err| panic!("guarded production must be classified: {err}"));
        if rule.status != CoverageStatus::Implemented {
            continue;
        }

        let path = manifest_dir().join(parser_path);
        let parser = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed to read parser file {}: {err}", path.display()));
        for forbidden in [
            "skip_until_brace_end",
            "semicolon_or_statement_brace_body",
            "take_until_terminator(input, b\";{\")",
        ] {
            if parser.contains(forbidden) {
                violations.push(format!(
                    "SysML.{production} is implemented by rule line {} but {} still contains {forbidden}",
                    rule.line,
                    path.display()
                ));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "implemented productions must not rely on opaque or statement-only body parsing:\n{}",
        violations.join("\n")
    );
}
