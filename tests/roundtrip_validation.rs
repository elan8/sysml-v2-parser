//! Roundtrip validation: parse → opacity gate → emit → parse → AST-eq.
//!
//! Debug AST snapshots only catch "output changed". This suite checks that
//! structured AST can be reconstructed as SysML and reparsed equivalently.
//!
//! Requires `SYSML_V2_RELEASE_DIR` or `./sysml-v2-release`. Run with:
//! `cargo test --test roundtrip_validation -- --include-ignored`

use std::fs;
use std::path::{Path, PathBuf};

use sysml_v2_parser::{emit_sysml, opacity_report, parse, EmitError, RootNamespace};

/// Iteration-1 scope: only fixtures under `01-Parts Tree/`.
const ROUNDTRIP_SCOPE_PREFIX: &str = "01-Parts Tree/";

/// Fixtures in scope that must currently roundtrip successfully.
///
/// Other `.sysml` files under [`ROUNDTRIP_SCOPE_PREFIX`] are known gaps and must still
/// fail. When a gap starts passing, add it here. Fixtures outside this folder are ignored
/// in this iteration.
const ROUNDTRIP_PASS: &[&str] = &[
    "01-Parts Tree/1a-Parts Tree.sysml",
    "01-Parts Tree/1c-Parts Tree Redefinition.sysml",
    "01-Parts Tree/1d-Parts Tree with Reference.sysml",
];

fn release_root() -> PathBuf {
    std::env::var_os("SYSML_V2_RELEASE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml-v2-release"))
}

fn validation_dir() -> PathBuf {
    release_root().join("sysml").join("src").join("validation")
}

fn find_sysml_files(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();
    if !dir.exists() {
        return Ok(files);
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(find_sysml_files(&path)?);
        } else if path.extension().and_then(|s| s.to_str()) == Some("sysml") {
            files.push(path);
        }
    }
    Ok(files)
}

fn rel_validation_path(file: &Path, root: &Path) -> String {
    file.strip_prefix(root)
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/")
}

fn diff_debug_strings(parsed: &str, expected: &str) -> (usize, String) {
    let pos = parsed
        .chars()
        .zip(expected.chars())
        .position(|(a, b)| a != b)
        .unwrap_or(parsed.len().min(expected.len()));
    let snippet: String = parsed
        .chars()
        .skip(pos.saturating_sub(80))
        .take(160)
        .collect();
    (pos, snippet)
}

#[derive(Debug)]
enum RoundtripOutcome {
    Ok,
    Failed(String),
}

fn try_roundtrip(src: &str) -> RoundtripOutcome {
    // Normalize newlines so Windows CRLF checkouts and Linux CI fixtures compare equally.
    let src = src.replace("\r\n", "\n").replace('\r', "\n");

    let ast1 = match parse(&src) {
        Ok(a) => a,
        Err(e) => return RoundtripOutcome::Failed(format!("parse failed: {e}")),
    };

    let opacity = opacity_report(&ast1);
    if !opacity.is_clean() {
        return RoundtripOutcome::Failed(format!(
            "opacity: {:?}",
            opacity
                .hits
                .iter()
                .take(5)
                .map(|h| format!("{}:{:?}", h.path, h.kind))
                .collect::<Vec<_>>()
        ));
    }

    let emitted = match emit_sysml(&ast1) {
        Ok(s) => s,
        Err(EmitError::Opaque { path, kind }) => {
            return RoundtripOutcome::Failed(format!("emit opaque at {path}: {kind:?}"))
        }
        Err(EmitError::Unsupported { path, construct }) => {
            return RoundtripOutcome::Failed(format!("emit unsupported at {path}: {construct}"))
        }
    };

    let ast2 = match parse(&emitted) {
        Ok(a) => a,
        Err(e) => {
            return RoundtripOutcome::Failed(format!(
                "reparse failed: {e}; emitted=\n{}",
                emitted.chars().take(500).collect::<String>()
            ))
        }
    };

    let na = ast1.normalize_for_test_comparison();
    let nb = ast2.normalize_for_test_comparison();
    if na != nb {
        // Debug includes spans that PartialEq ignores; strip them so the snippet
        // points at a real semantic mismatch rather than offset noise.
        let pa = strip_span_noise(&format!("{na:?}"));
        let pb = strip_span_noise(&format!("{nb:?}"));
        let (pos, snippet) = diff_debug_strings(&pa, &pb);
        return RoundtripOutcome::Failed(format!(
            "AST-eq failed at char {pos}; snippet ...{snippet}...; emitted head:\n{}",
            emitted.chars().take(400).collect::<String>()
        ));
    }

    RoundtripOutcome::Ok
}

/// Remove `Span { ... }` blobs from Debug output used only for mismatch location.
fn strip_span_noise(s: &str) -> String {
    let mut s = s.to_string();
    while let Some(start) = s.find("Span {") {
        let rest = &s[start + 6..];
        let mut depth = 1usize;
        let mut end = None;
        for (i, ch) in rest.char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(start + 6 + i + 1);
                        break;
                    }
                }
                _ => {}
            }
        }
        match end {
            Some(e) => s.replace_range(start..e, "Span(_)"),
            None => break,
        }
    }
    s
}

#[test]
#[ignore = "requires SysML v2 release; run with --include-ignored"]
fn roundtrip_required_pass_fixtures() {
    let root = validation_dir();
    if !root.exists() {
        eprintln!(
            "skipping: validation dir missing at {} (set SYSML_V2_RELEASE_DIR)",
            root.display()
        );
        return;
    }

    let mut failures = Vec::new();
    for rel in ROUNDTRIP_PASS {
        let path = root.join(rel.replace('/', std::path::MAIN_SEPARATOR_STR));
        let src = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                failures.push(format!("{rel}: read failed: {e}"));
                continue;
            }
        };
        match try_roundtrip(&src) {
            RoundtripOutcome::Ok => eprintln!("✓ roundtrip {rel}"),
            RoundtripOutcome::Failed(msg) => {
                failures.push(format!("{rel}: {msg}"));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "required roundtrip failures:\n{}",
        failures.join("\n")
    );
}

#[test]
#[ignore = "requires SysML v2 release; run with --include-ignored"]
fn roundtrip_known_gaps_must_still_fail() {
    let root = validation_dir();
    if !root.exists() {
        eprintln!(
            "skipping: validation dir missing at {} (set SYSML_V2_RELEASE_DIR)",
            root.display()
        );
        return;
    }

    let scope_root = root.join(ROUNDTRIP_SCOPE_PREFIX.trim_end_matches('/'));
    let files = find_sysml_files(&scope_root).expect("list scoped validation files");
    assert!(
        !files.is_empty(),
        "no .sysml files under {}",
        ROUNDTRIP_SCOPE_PREFIX
    );

    let pass: std::collections::HashSet<&str> = ROUNDTRIP_PASS.iter().copied().collect();
    let mut unexpected_passes = Vec::new();
    let mut gap_count = 0usize;
    let mut missing_from_pass = Vec::new();

    for file in &files {
        let rel = rel_validation_path(file, &root);
        assert!(
            rel.starts_with(ROUNDTRIP_SCOPE_PREFIX),
            "scoped scan escaped prefix: {rel}"
        );
        if pass.contains(rel.as_str()) {
            continue;
        }
        gap_count += 1;
        missing_from_pass.push(rel.clone());
        let src = fs::read_to_string(file).expect("read known-gap fixture");
        match try_roundtrip(&src) {
            RoundtripOutcome::Ok => unexpected_passes.push(rel),
            RoundtripOutcome::Failed(_) => {}
        }
    }

    eprintln!(
        "scope {}: {} required-pass, {} known-gap ({})",
        ROUNDTRIP_SCOPE_PREFIX,
        ROUNDTRIP_PASS.len(),
        gap_count,
        missing_from_pass.join(", ")
    );
    assert!(
        unexpected_passes.is_empty(),
        "these scoped fixtures now roundtrip — add them to ROUNDTRIP_PASS:\n{}",
        unexpected_passes.join("\n")
    );
}

#[test]
fn roundtrip_handwritten_part_tree_smoke() {
    let src = r#"
package P {
    private import SI::kg;
    package Definitions {
        part def Vehicle {
            attribute mass :> ISQ::mass {
                doc
                /* mass doc */
            }
        }
        part def Axle;
        part def HitchBall;
    }
    package Usages {
        private import Definitions::*;
        part vehicle1: Vehicle {
            attribute mass :>> Vehicle::mass = 1750 [kg];
            part wheel: Axle[2] ordered;
            part w1 :> wheel = wheel#(1);
            ref hitchBall : HitchBall {}
            bind hitchBall = hitchBall {}
        }
    }
}
"#;
    match try_roundtrip(src) {
        RoundtripOutcome::Ok => {}
        RoundtripOutcome::Failed(msg) => panic!("handwritten smoke failed: {msg}"),
    }
}

#[allow(dead_code)]
fn _assert_ast_eq(a: &RootNamespace, b: &RootNamespace, msg: &str) {
    let na = a.normalize_for_test_comparison();
    let nb = b.normalize_for_test_comparison();
    if na == nb {
        return;
    }
    let pa = format!("{na:?}");
    let pb = format!("{nb:?}");
    let (pos, snippet) = diff_debug_strings(&pa, &pb);
    panic!(
        "{msg}: AST mismatch at char {pos} (a {} chars, b {} chars). Snippet: ...{snippet}...",
        pa.len(),
        pb.len(),
    );
}
