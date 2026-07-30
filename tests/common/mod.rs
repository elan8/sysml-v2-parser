//! Shared helpers for conformance pin + BNF coverage classification.

#![allow(dead_code)] // Shared across integration-test binaries; not every helper is used in each.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceTarget {
    pub release_tag: String,
    pub release_repo: String,
    pub sysml_bnf_productions: usize,
    pub kerml_bnf_productions: usize,
}

pub fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn release_root() -> PathBuf {
    if let Some(path) = std::env::var_os("SYSML_V2_RELEASE_DIR") {
        return PathBuf::from(path);
    }
    manifest_dir().join("sysml-v2-release")
}

pub fn conformance_target_path() -> PathBuf {
    manifest_dir().join("docs").join("conformance-target")
}

pub fn release_stamp_path(root: &Path) -> PathBuf {
    root.join(".elan8-conformance-target")
}

pub fn load_conformance_target() -> ConformanceTarget {
    load_conformance_target_from(&conformance_target_path())
}

pub fn load_conformance_target_from(path: &Path) -> ConformanceTarget {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read conformance target {}: {err}", path.display()));
    let mut release_tag = None;
    let mut release_repo = None;
    let mut sysml_bnf_productions = None;
    let mut kerml_bnf_productions = None;

    for (idx, line) in text.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            panic!(
                "invalid conformance-target line {line_no} in {}: expected key=value",
                path.display()
            );
        };
        let key = key.trim();
        let value = value.trim();
        match key {
            "release_tag" => release_tag = Some(value.to_string()),
            "release_repo" => release_repo = Some(value.to_string()),
            "sysml_bnf_productions" => {
                sysml_bnf_productions = Some(value.parse::<usize>().unwrap_or_else(|_| {
                    panic!("invalid sysml_bnf_productions at {line_no}: {value}");
                }));
            }
            "kerml_bnf_productions" => {
                kerml_bnf_productions = Some(value.parse::<usize>().unwrap_or_else(|_| {
                    panic!("invalid kerml_bnf_productions at {line_no}: {value}");
                }));
            }
            _ => panic!(
                "unknown conformance-target key '{key}' at {}:{line_no}",
                path.display()
            ),
        }
    }

    ConformanceTarget {
        release_tag: release_tag.unwrap_or_else(|| {
            panic!("conformance-target missing release_tag: {}", path.display())
        }),
        release_repo: release_repo.unwrap_or_else(|| {
            panic!("conformance-target missing release_repo: {}", path.display())
        }),
        sysml_bnf_productions: sysml_bnf_productions.unwrap_or_else(|| {
            panic!(
                "conformance-target missing sysml_bnf_productions: {}",
                path.display()
            )
        }),
        kerml_bnf_productions: kerml_bnf_productions.unwrap_or_else(|| {
            panic!(
                "conformance-target missing kerml_bnf_productions: {}",
                path.display()
            )
        }),
    }
}

/// Release stamp written by fetch scripts (`release_tag=...`).
pub fn load_release_stamp(root: &Path) -> Option<String> {
    let path = release_stamp_path(root);
    if !path.exists() {
        return None;
    }
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read release stamp {}: {err}", path.display()));
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = trimmed.split_once('=') {
            if key.trim() == "release_tag" {
                return Some(value.trim().to_string());
            }
        } else {
            // Allow a bare tag on a single line.
            return Some(trimmed.to_string());
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoverageStatus {
    Implemented,
    Partial,
    Opaque,
    Fallback,
    Untested,
    NotSupported,
}

impl CoverageStatus {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "implemented" => Some(Self::Implemented),
            "partial" => Some(Self::Partial),
            "opaque" => Some(Self::Opaque),
            "fallback" => Some(Self::Fallback),
            "untested" => Some(Self::Untested),
            "not_supported" => Some(Self::NotSupported),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grammar {
    SysML,
    KerML,
    Any,
}

impl Grammar {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "SysML" => Some(Self::SysML),
            "KerML" => Some(Self::KerML),
            "*" => Some(Self::Any),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct CoverageRule {
    pub grammar: Grammar,
    pub pattern: String,
    pub status: CoverageStatus,
    pub line: usize,
}

impl CoverageRule {
    pub fn matches(&self, grammar: Grammar, production: &str) -> bool {
        if self.grammar != Grammar::Any && self.grammar != grammar {
            return false;
        }
        pattern_matches(&self.pattern, production)
    }

    pub fn specificity(&self) -> usize {
        let non_wildcard = self.pattern.chars().filter(|ch| *ch != '*').count();
        let grammar_bonus = usize::from(self.grammar != Grammar::Any) * 1_000;
        let exact_bonus = usize::from(!self.pattern.contains('*')) * 10_000;
        exact_bonus + grammar_bonus + non_wildcard
    }
}

pub fn extract_productions(path: &Path) -> Vec<String> {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read BNF file {}: {err}", path.display()));
    let mut productions = Vec::new();
    for line in text.lines() {
        let Some(first) = line.as_bytes().first().copied() else {
            continue;
        };
        if !first.is_ascii_alphabetic() {
            continue;
        }
        let name_len = line
            .bytes()
            .take_while(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
            .count();
        if name_len == 0 {
            continue;
        }
        let rest = line[name_len..].trim_start();
        if rest.starts_with('=') || rest.contains(" =") {
            productions.push(line[..name_len].to_string());
        }
    }
    productions.sort();
    productions.dedup();
    productions
}

pub fn parse_coverage_rules(path: &Path) -> Vec<CoverageRule> {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read coverage map {}: {err}", path.display()));
    let mut rules = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let fields = trimmed.split_whitespace().collect::<Vec<_>>();
        assert_eq!(
            fields.len(),
            3,
            "invalid coverage rule at {}:{line_no}: expected 3 fields",
            path.display()
        );
        let grammar = Grammar::parse(fields[0]).unwrap_or_else(|| {
            panic!(
                "invalid grammar '{}' at {}:{line_no}",
                fields[0],
                path.display()
            )
        });
        let status = CoverageStatus::parse(fields[2]).unwrap_or_else(|| {
            panic!(
                "invalid coverage status '{}' at {}:{line_no}",
                fields[2],
                path.display()
            )
        });
        rules.push(CoverageRule {
            grammar,
            pattern: fields[1].to_string(),
            status,
            line: line_no,
        });
    }
    rules
}

pub fn pattern_matches(pattern: &str, value: &str) -> bool {
    match (pattern.starts_with('*'), pattern.ends_with('*')) {
        (false, false) => pattern == value,
        (false, true) => value.starts_with(&pattern[..pattern.len() - 1]),
        (true, false) => value.ends_with(&pattern[1..]),
        (true, true) => {
            let needle = &pattern[1..pattern.len() - 1];
            !needle.is_empty() && value.contains(needle)
        }
    }
}

pub fn classify<'a>(
    rules: &'a [CoverageRule],
    grammar: Grammar,
    production: &str,
) -> Result<&'a CoverageRule, String> {
    let matches = rules
        .iter()
        .filter(|rule| rule.matches(grammar, production))
        .collect::<Vec<_>>();
    let Some(best_specificity) = matches.iter().map(|rule| rule.specificity()).max() else {
        return Err(format!("no coverage rule for {grammar:?}.{production}"));
    };
    let best = matches
        .into_iter()
        .filter(|rule| rule.specificity() == best_specificity)
        .collect::<Vec<_>>();
    let first = best[0];
    if best.iter().any(|rule| rule.status != first.status) {
        let lines = best
            .iter()
            .map(|rule| rule.line.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "conflicting equally-specific coverage rules for {grammar:?}.{production} at lines {lines}"
        ));
    }
    Ok(first)
}

pub type ClassifyAllResult = (
    BTreeMap<CoverageStatus, usize>,
    BTreeMap<CoverageStatus, Vec<String>>,
    Vec<String>,
);

pub fn classify_all(
    grammar: Grammar,
    productions: &[String],
    rules: &[CoverageRule],
) -> ClassifyAllResult {
    let mut counts = BTreeMap::<CoverageStatus, usize>::new();
    let mut productions_by_status = BTreeMap::<CoverageStatus, Vec<String>>::new();
    let mut errors = Vec::new();
    for production in productions {
        match classify(rules, grammar, production) {
            Ok(rule) => {
                *counts.entry(rule.status).or_insert(0) += 1;
                productions_by_status
                    .entry(rule.status)
                    .or_default()
                    .push(production.clone());
            }
            Err(err) => errors.push(err),
        }
    }
    (counts, productions_by_status, errors)
}

pub fn load_bnf_productions() -> (Vec<String>, Vec<String>, Vec<CoverageRule>) {
    let root = release_root();
    let sysml_bnf = root.join("bnf").join("SysML-textual-bnf.kebnf");
    let kerml_bnf = root.join("bnf").join("KerML-textual-bnf.kebnf");
    let rules = parse_coverage_rules(&manifest_dir().join("docs").join("bnf_coverage.map"));
    (
        extract_productions(&sysml_bnf),
        extract_productions(&kerml_bnf),
        rules,
    )
}
