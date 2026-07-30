//! Conformance pin verification + layered claim scorecard.
//!
//! Writes `conformance-scorecard.json` and `.md` under
//! `CONFORMANCE_SCORECARD_DIR` (default: `target/`).

#[path = "common/mod.rs"]
mod common;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use sysml_v2_parser::ast::{PackageBody, PackageBodyElement, RootElement, RootNamespace};
use sysml_v2_parser::parse_with_diagnostics;

use common::{
    classify_all, load_bnf_productions, load_conformance_target, load_release_stamp, manifest_dir,
    release_root, CoverageStatus, Grammar,
};

#[derive(Debug, Default)]
struct LibraryScan {
    files: usize,
    files_with_diagnostics: usize,
    diagnostic_count: usize,
    extended_library_decl: usize,
    kerml_semantic_decl: usize,
    kerml_feature_decl: usize,
}

fn scorecard_dir() -> PathBuf {
    if let Some(path) = std::env::var_os("CONFORMANCE_SCORECARD_DIR") {
        return PathBuf::from(path);
    }
    manifest_dir().join("target")
}

fn find_library_files(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();
    if !dir.exists() {
        return Ok(files);
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(find_library_files(&path)?);
        } else if matches!(
            path.extension().and_then(|s| s.to_str()),
            Some("sysml") | Some("kerml")
        ) {
            files.push(path);
        }
    }
    Ok(files)
}

fn collect_package_body_type_counts(root: &RootNamespace, counts: &mut BTreeMap<String, usize>) {
    for element in &root.elements {
        match &element.value {
            RootElement::Package(p) => collect_body_type_counts(&p.value.body, counts),
            RootElement::LibraryPackage(p) => collect_body_type_counts(&p.value.body, counts),
            RootElement::Namespace(n) => collect_body_type_counts(&n.value.body, counts),
            RootElement::Import(_) => {}
            RootElement::Member(_) => {}
        }
    }
}

fn collect_body_type_counts(body: &PackageBody, counts: &mut BTreeMap<String, usize>) {
    let PackageBody::Brace { elements } = body else {
        return;
    };
    for element in elements {
        let key = match &element.value {
            PackageBodyElement::ExtendedLibraryDecl(_) => "ExtendedLibraryDecl",
            PackageBodyElement::KermlSemanticDecl(_) => "KermlSemanticDecl",
            PackageBodyElement::KermlFeatureDecl(_) => "KermlFeatureDecl",
            PackageBodyElement::Package(n) => {
                collect_body_type_counts(&n.value.body, counts);
                "Package"
            }
            PackageBodyElement::LibraryPackage(n) => {
                collect_body_type_counts(&n.value.body, counts);
                "LibraryPackage"
            }
            _ => "Other",
        };
        *counts.entry(key.to_string()).or_insert(0) += 1;
    }
}

fn scan_library(dir: &Path) -> LibraryScan {
    let mut scan = LibraryScan::default();
    let mut files = find_library_files(dir).unwrap_or_else(|err| {
        panic!("failed to walk library {}: {err}", dir.display());
    });
    files.sort();
    scan.files = files.len();

    for file in &files {
        let content = fs::read_to_string(file).unwrap_or_else(|err| {
            panic!("failed to read {}: {err}", file.display());
        });
        let result = parse_with_diagnostics(&content);
        if !result.errors.is_empty() {
            scan.files_with_diagnostics += 1;
            scan.diagnostic_count += result.errors.len();
        }
        let mut type_counts = BTreeMap::new();
        collect_package_body_type_counts(&result.root, &mut type_counts);
        scan.extended_library_decl += *type_counts.get("ExtendedLibraryDecl").unwrap_or(&0);
        scan.kerml_semantic_decl += *type_counts.get("KermlSemanticDecl").unwrap_or(&0);
        scan.kerml_feature_decl += *type_counts.get("KermlFeatureDecl").unwrap_or(&0);
    }
    scan
}

fn status_count(counts: &BTreeMap<CoverageStatus, usize>, status: CoverageStatus) -> usize {
    counts.get(&status).copied().unwrap_or(0)
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn layer_status(pass: bool) -> &'static str {
    if pass {
        "pass"
    } else {
        "fail"
    }
}

fn write_scorecard(
    json_path: &Path,
    md_path: &Path,
    parser_version: &str,
    target: &common::ConformanceTarget,
    stamp_tag: Option<&str>,
    stamp_ok: bool,
    sysml_counts: &BTreeMap<CoverageStatus, usize>,
    kerml_counts: &BTreeMap<CoverageStatus, usize>,
    l1_ok: bool,
    systems: Option<&LibraryScan>,
    full: Option<&LibraryScan>,
    l2_skip_reason: Option<&str>,
) {
    let l2_systems_ok = systems.is_some_and(|s| {
        s.files > 0 && s.files_with_diagnostics == 0 && s.extended_library_decl == 0
    });
    let l2_full_ok = full.is_some_and(|s| {
        s.files > 0 && s.files_with_diagnostics == 0 && s.extended_library_decl == 0
    });
    let l2_ok = match (systems, full) {
        (Some(_), Some(_)) => l2_systems_ok && l2_full_ok,
        _ => false,
    };
    let l2_status = if systems.is_none() || full.is_none() {
        "skipped"
    } else {
        layer_status(l2_ok)
    };

    let sysml_impl = status_count(sysml_counts, CoverageStatus::Implemented);
    let kerml_impl = status_count(kerml_counts, CoverageStatus::Implemented);
    let total_impl = sysml_impl + kerml_impl;
    let total_expected = target.sysml_bnf_productions + target.kerml_bnf_productions;

    let systems_json = match systems {
        Some(s) => format!(
            r#"{{"files":{},"files_with_diagnostics":{},"diagnostic_count":{},"extended_library_decl":{},"kerml_semantic_decl":{},"kerml_feature_decl":{}}}"#,
            s.files,
            s.files_with_diagnostics,
            s.diagnostic_count,
            s.extended_library_decl,
            s.kerml_semantic_decl,
            s.kerml_feature_decl
        ),
        None => "null".to_string(),
    };
    let full_json = match full {
        Some(s) => format!(
            r#"{{"files":{},"files_with_diagnostics":{},"diagnostic_count":{},"extended_library_decl":{},"kerml_semantic_decl":{},"kerml_feature_decl":{}}}"#,
            s.files,
            s.files_with_diagnostics,
            s.diagnostic_count,
            s.extended_library_decl,
            s.kerml_semantic_decl,
            s.kerml_feature_decl
        ),
        None => "null".to_string(),
    };

    let stamp_json = stamp_tag
        .map(|t| format!(r#""{}""#, json_escape(t)))
        .unwrap_or_else(|| "null".to_string());

    let json = format!(
        r#"{{
  "parser_version": "{parser_version}",
  "target": {{
    "release_tag": "{tag}",
    "release_repo": "{repo}",
    "sysml_bnf_productions": {sysml_n},
    "kerml_bnf_productions": {kerml_n}
  }},
  "release_stamp": {{
    "tag": {stamp_json},
    "matches_pin": {stamp_ok}
  }},
  "layers": {{
    "L1_syntax_acceptance": {{
      "claimed": true,
      "status": "{l1_status}",
      "bnf_productions": {{
        "sysml": {sysml_n},
        "kerml": {kerml_n},
        "implemented": {total_impl},
        "expected": {total_expected},
        "partial": {partial},
        "opaque": {opaque},
        "fallback": {fallback},
        "untested": {untested},
        "not_supported": {not_supported}
      }}
    }},
    "L2_structured_ast": {{
      "claimed": true,
      "status": "{l2_status}",
      "systems_library": {systems_json},
      "full_library": {full_json}
    }},
    "L3_semantics": {{
      "claimed": false,
      "status": "not_claimed",
      "note": "Name resolution, typing, and KerML/SysML well-formedness are Spec42 / other-tool scope."
    }}
  }}
}}
"#,
        parser_version = json_escape(parser_version),
        tag = json_escape(&target.release_tag),
        repo = json_escape(&target.release_repo),
        sysml_n = target.sysml_bnf_productions,
        kerml_n = target.kerml_bnf_productions,
        l1_status = layer_status(l1_ok),
        partial = status_count(sysml_counts, CoverageStatus::Partial)
            + status_count(kerml_counts, CoverageStatus::Partial),
        opaque = status_count(sysml_counts, CoverageStatus::Opaque)
            + status_count(kerml_counts, CoverageStatus::Opaque),
        fallback = status_count(sysml_counts, CoverageStatus::Fallback)
            + status_count(kerml_counts, CoverageStatus::Fallback),
        untested = status_count(sysml_counts, CoverageStatus::Untested)
            + status_count(kerml_counts, CoverageStatus::Untested),
        not_supported = status_count(sysml_counts, CoverageStatus::NotSupported)
            + status_count(kerml_counts, CoverageStatus::NotSupported),
    );

    let systems_md = match systems {
        Some(s) => format!(
            "- Systems Library: {} files, {} with diagnostics, ExtendedLibraryDecl={}, status={}",
            s.files,
            s.files_with_diagnostics,
            s.extended_library_decl,
            layer_status(l2_systems_ok)
        ),
        None => format!(
            "- Systems Library: skipped ({})",
            l2_skip_reason.unwrap_or("unavailable")
        ),
    };
    let full_md = match full {
        Some(s) => format!(
            "- Full `sysml.library`: {} files, {} with diagnostics, ExtendedLibraryDecl={}, status={}",
            s.files,
            s.files_with_diagnostics,
            s.extended_library_decl,
            layer_status(l2_full_ok)
        ),
        None => format!(
            "- Full `sysml.library`: skipped ({})",
            l2_skip_reason.unwrap_or("unavailable")
        ),
    };

    let md = format!(
        r#"# Conformance scorecard

Parser version: `{parser_version}`

## Target

- OMG SysML-v2-Release tag: `{tag}`
- Repo: {repo}
- Expected BNF productions: SysML {sysml_n} + KerML {kerml_n} = {total_expected}
- Release stamp matches pin: **{stamp_ok}**{stamp_extra}

## Layers

### L1 — Syntax acceptance (claimed)

Status: **{l1_status}**

- Implemented productions: {total_impl} / {total_expected}
- Partial / opaque / fallback / untested / not_supported: {partial} / {opaque} / {fallback} / {untested} / {not_supported}

### L2 — Structured AST (claimed)

Status: **{l2_status}**

{systems_md}
{full_md}

### L3 — Semantics (not claimed)

Status: **not_claimed**

Name resolution, typing, and KerML/SysML well-formedness remain Spec42 / other-tool scope.
"#,
        parser_version = parser_version,
        tag = target.release_tag,
        repo = target.release_repo,
        sysml_n = target.sysml_bnf_productions,
        kerml_n = target.kerml_bnf_productions,
        total_expected = total_expected,
        stamp_ok = stamp_ok,
        stamp_extra = stamp_tag
            .map(|t| format!(" (stamp=`{t}`)"))
            .unwrap_or_default(),
        l1_status = layer_status(l1_ok),
        total_impl = total_impl,
        partial = status_count(sysml_counts, CoverageStatus::Partial)
            + status_count(kerml_counts, CoverageStatus::Partial),
        opaque = status_count(sysml_counts, CoverageStatus::Opaque)
            + status_count(kerml_counts, CoverageStatus::Opaque),
        fallback = status_count(sysml_counts, CoverageStatus::Fallback)
            + status_count(kerml_counts, CoverageStatus::Fallback),
        untested = status_count(sysml_counts, CoverageStatus::Untested)
            + status_count(kerml_counts, CoverageStatus::Untested),
        not_supported = status_count(sysml_counts, CoverageStatus::NotSupported)
            + status_count(kerml_counts, CoverageStatus::NotSupported),
        l2_status = l2_status,
        systems_md = systems_md,
        full_md = full_md,
    );

    if let Some(parent) = json_path.parent() {
        fs::create_dir_all(parent).unwrap_or_else(|err| {
            panic!("failed to create scorecard dir {}: {err}", parent.display());
        });
    }
    fs::write(json_path, json).unwrap_or_else(|err| {
        panic!("failed to write {}: {err}", json_path.display());
    });
    fs::write(md_path, md).unwrap_or_else(|err| {
        panic!("failed to write {}: {err}", md_path.display());
    });
}

#[test]
fn conformance_target_file_is_well_formed() {
    let target = load_conformance_target();
    assert!(
        !target.release_tag.is_empty(),
        "release_tag must not be empty"
    );
    assert!(
        target
            .release_repo
            .starts_with("https://github.com/Systems-Modeling/SysML-v2-Release"),
        "unexpected release_repo: {}",
        target.release_repo
    );
    assert!(target.sysml_bnf_productions > 0);
    assert!(target.kerml_bnf_productions > 0);
}

#[test]
fn fetched_release_matches_pinned_conformance_target() {
    let target = load_conformance_target();
    let root = release_root();
    if !root.exists() {
        eprintln!(
            "skipping stamp check: release root missing at {} (run fetch script)",
            root.display()
        );
        return;
    }

    let stamp = load_release_stamp(&root);
    match stamp {
        Some(tag) => assert_eq!(
            tag, target.release_tag,
            "release stamp tag does not match docs/conformance-target; re-run fetch scripts"
        ),
        None => {
            // Legacy trees fetched before stamping: require BNF layout + warn.
            let sysml_bnf = root.join("bnf").join("SysML-textual-bnf.kebnf");
            assert!(
                sysml_bnf.exists(),
                "release tree at {} has no .elan8-conformance-target stamp and no BNF; re-run fetch scripts",
                root.display()
            );
            eprintln!(
                "warning: release tree at {} has no .elan8-conformance-target stamp; re-run fetch scripts to pin {}",
                root.display(),
                target.release_tag
            );
        }
    }
}

#[test]
fn write_layered_conformance_scorecard() {
    let target = load_conformance_target();
    let parser_version = env!("CARGO_PKG_VERSION");
    let root = release_root();

    let stamp_tag = load_release_stamp(&root);
    let stamp_ok = stamp_tag.as_deref() == Some(target.release_tag.as_str());

    let (sysml, kerml, rules) = load_bnf_productions();
    assert_eq!(sysml.len(), target.sysml_bnf_productions);
    assert_eq!(kerml.len(), target.kerml_bnf_productions);

    let (sysml_counts, _, sysml_errors) = classify_all(Grammar::SysML, &sysml, &rules);
    let (kerml_counts, _, kerml_errors) = classify_all(Grammar::KerML, &kerml, &rules);
    assert!(
        sysml_errors.is_empty() && kerml_errors.is_empty(),
        "BNF classification errors: {:?} {:?}",
        sysml_errors,
        kerml_errors
    );

    let l1_ok = status_count(&sysml_counts, CoverageStatus::Implemented) == sysml.len()
        && status_count(&kerml_counts, CoverageStatus::Implemented) == kerml.len()
        && status_count(&sysml_counts, CoverageStatus::Partial) == 0
        && status_count(&kerml_counts, CoverageStatus::Partial) == 0;

    // L2 library scan is expensive; enable explicitly in CI validation via
    // CONFORMANCE_SCORECARD_L2=1 so default `cargo test` stays fast.
    let run_l2 = std::env::var_os("CONFORMANCE_SCORECARD_L2").is_some();
    let library_root = root.join("sysml.library");
    let l2_skip_reason = if !run_l2 {
        Some("set CONFORMANCE_SCORECARD_L2=1 to enable")
    } else if !library_root.exists() {
        Some("sysml.library not found under release root")
    } else {
        None
    };
    let (systems, full) = if l2_skip_reason.is_none() {
        let systems = scan_library(&library_root.join("Systems Library"));
        let full = scan_library(&library_root);
        (Some(systems), Some(full))
    } else {
        (None, None)
    };

    let out_dir = scorecard_dir();
    let json_path = out_dir.join("conformance-scorecard.json");
    let md_path = out_dir.join("conformance-scorecard.md");
    write_scorecard(
        &json_path,
        &md_path,
        parser_version,
        &target,
        stamp_tag.as_deref(),
        stamp_ok,
        &sysml_counts,
        &kerml_counts,
        l1_ok,
        systems.as_ref(),
        full.as_ref(),
        l2_skip_reason,
    );

    eprintln!("Wrote {}", json_path.display());
    eprintln!("Wrote {}", md_path.display());

    assert!(
        l1_ok,
        "L1 syntax-acceptance claim failed for pin {}",
        target.release_tag
    );
    if let (Some(systems), Some(full)) = (systems.as_ref(), full.as_ref()) {
        assert_eq!(
            systems.files_with_diagnostics, 0,
            "L2 Systems Library diagnostics: {}",
            systems.diagnostic_count
        );
        assert_eq!(
            systems.extended_library_decl, 0,
            "L2 Systems Library ExtendedLibraryDecl != 0"
        );
        assert_eq!(
            full.files_with_diagnostics, 0,
            "L2 full library diagnostics: {}",
            full.diagnostic_count
        );
        assert_eq!(
            full.extended_library_decl, 0,
            "L2 full library ExtendedLibraryDecl != 0"
        );
    }

    // Stamp is required in CI (fetch scripts always write it). Local legacy trees warn only.
    if std::env::var_os("CI").is_some() {
        assert!(
            stamp_ok,
            "CI requires .elan8-conformance-target to match docs/conformance-target"
        );
    }
}
