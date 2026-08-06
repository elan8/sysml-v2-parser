//! Roundtrip validation: parse → opacity gate → emit → parse → AST-eq.
//!
//! Debug AST snapshots only catch "output changed". This suite checks that
//! structured AST can be reconstructed as SysML and reparsed equivalently.
//!
//! Requires `SYSML_V2_RELEASE_DIR` or `./sysml-v2-release`. Run with:
//! `cargo test --test roundtrip_validation -- --include-ignored`

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use sysml_v2_parser::{emit_sysml, opacity_report, parse, EmitError, RootNamespace};

/// Fixtures that must currently roundtrip successfully.
///
/// Every other `.sysml` under the pinned release `sysml/src/validation/` tree is a known
/// gap and must still fail. When a gap starts passing, add it here.
const ROUNDTRIP_PASS: &[&str] = &[
    "01-Parts Tree/1a-Parts Tree.sysml",
    "01-Parts Tree/1c-Parts Tree Redefinition.sysml",
    "01-Parts Tree/1d-Parts Tree with Reference.sysml",
    "02-Parts Interconnection/2a-Parts Interconnection.sysml",
    "02-Parts Interconnection/2c-Parts Interconnection-Multiple Decompositions.sysml",
    // Promoted by the full-tree known-gap scan once port/interface/connect emit landed;
    // not deliberately targeted by this iteration beyond inventory.
    "14-Language Extensions/14b-Language Extensions.sysml",
    "15-Properties-Values-Expressions/15_02-Basic Value Properties.sysml",
    "15-Properties-Values-Expressions/15_03-Value Expression.sysml",
    "15-Properties-Values-Expressions/15_06-System of Quantities.sysml",
    "15-Properties-Values-Expressions/15_07-System of Units and Scales.sysml",
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

fn folder_of(rel: &str) -> &str {
    rel.split('/').next().unwrap_or(rel)
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
        let (pos, orig_snip, reparse_snip) = diff_debug_both(&pa, &pb);
        return RoundtripOutcome::Failed(format!(
            "AST-eq failed at char {pos};\n  original: ...{orig_snip}...\n  reparse:  ...{reparse_snip}...\n  emitted head:\n{}",
            emitted.chars().take(600).collect::<String>()
        ));
    }

    RoundtripOutcome::Ok
}

fn diff_debug_both(original: &str, reparsed: &str) -> (usize, String, String) {
    let pos = original
        .chars()
        .zip(reparsed.chars())
        .position(|(a, b)| a != b)
        .unwrap_or(original.len().min(reparsed.len()));
    let snip = |s: &str| -> String { s.chars().skip(pos.saturating_sub(80)).take(200).collect() };
    (pos, snip(original), snip(reparsed))
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

    let files = find_sysml_files(&root).expect("list validation files");
    assert!(
        !files.is_empty(),
        "no .sysml files under {}",
        root.display()
    );

    let pass: std::collections::HashSet<&str> = ROUNDTRIP_PASS.iter().copied().collect();
    let mut unexpected_passes = Vec::new();
    let mut gap_count = 0usize;
    let mut inventory: BTreeMap<String, (usize, usize)> = BTreeMap::new();

    for file in &files {
        let rel = rel_validation_path(file, &root);
        let folder = folder_of(&rel).to_string();
        let entry = inventory.entry(folder).or_insert((0, 0));
        if pass.contains(rel.as_str()) {
            entry.0 += 1;
            continue;
        }
        entry.1 += 1;
        gap_count += 1;
        let src = fs::read_to_string(file).expect("read known-gap fixture");
        match try_roundtrip(&src) {
            RoundtripOutcome::Ok => unexpected_passes.push(rel),
            RoundtripOutcome::Failed(msg) => {
                if std::env::var_os("ROUNDTRIP_DIAG").is_some() {
                    let head = msg.lines().next().unwrap_or(&msg);
                    let brief: String = head.chars().take(140).collect();
                    eprintln!("GAP {rel}: {brief}");
                }
            }
        }
    }

    eprintln!("L2.5 validation inventory (pass / known-gap) against pinned release:");
    for (folder, (pass_n, gap_n)) in &inventory {
        eprintln!("  {folder}: {pass_n} pass, {gap_n} known-gap");
    }
    eprintln!(
        "totals: {} required-pass, {} known-gap",
        ROUNDTRIP_PASS.len(),
        gap_count
    );
    assert!(
        unexpected_passes.is_empty(),
        "these validation fixtures now roundtrip — add them to ROUNDTRIP_PASS:\n{}",
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
        private import Definitions::* {}
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

#[test]
fn roundtrip_handwritten_ports_connect_smoke() {
    let src = r#"
package PortsConnect {
    port def FuelCmdPort;
    port def WheelToRoadPort;
    port def VehicleToRoadPort {
        port wheelToRoadPort: WheelToRoadPort[2];
    }
    part def Engine {
        port fuelCmdPort: FuelCmdPort;
    }
    part def C1 {
        port pa;
        port pb;
    }
    part def C2 {
        port pc;
    }
    part a {
        part c1: C1;
        part c2: C2;
        connect c1.pa to c2.pc;
        port :>> pe = c1.pb;
    }
}
"#;
    match try_roundtrip(src) {
        RoundtripOutcome::Ok => {}
        RoundtripOutcome::Failed(msg) => panic!("ports/connect smoke failed: {msg}"),
    }
}

#[test]
fn roundtrip_handwritten_interface_smoke() {
    let src = r#"
package Interfaces {
    port def DrivePwrPort;
    port def ClutchPort;
    part def Engine {
        port drivePwrPort: DrivePwrPort;
    }
    part def Transmission {
        port clutchPort: ClutchPort;
    }
    interface def EngineToTransmissionInterface {
        end drivePwrPort: DrivePwrPort;
        end clutchPort: ClutchPort;
    }
    part vehicle {
        part engine: Engine;
        part transmission: Transmission;
        interface :EngineToTransmissionInterface
            connect engine.drivePwrPort to transmission.clutchPort;
    }
}
"#;
    match try_roundtrip(src) {
        RoundtripOutcome::Ok => {}
        RoundtripOutcome::Failed(msg) => panic!("interface smoke failed: {msg}"),
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
