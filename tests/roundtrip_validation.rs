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
    // Promoted by #74 AST-eq / emit-shape work (perform action, accept control nodes, span eq).
    "03-Function-based Behavior/3a-Function-based Behavior-1.sysml",
    "03-Function-based Behavior/3a-Function-based Behavior-2.sysml",
    "03-Function-based Behavior/3a-Function-based Behavior-3.sysml",
    "03-Function-based Behavior/3c-Function-based Behavior-structure mod-1.sysml",
    "03-Function-based Behavior/3c-Function-based Behavior-structure mod-2.sysml",
    "03-Function-based Behavior/3c-Function-based Behavior-structure mod-3.sysml",
    "03-Function-based Behavior/3d-Function-based Behavior-item.sysml",
    // Promoted by #78 follow-up: `part` in item/attribute bodies.
    "03-Function-based Behavior/3e-Function-based Behavior-item.sysml",
    // Promoted by #72 Other-opacity work (state do/out/accept; attribute assert constraint).
    "05-State-based Behavior/5-State-based Behavior-1.sysml",
    "05-State-based Behavior/5-State-based Behavior-1a.sysml",
    "05-State-based Behavior/5-State-based Behavior-2.sysml",
    // Promoted by import/type quoting (#71) once spaced names reparse cleanly.
    "04-Functional Allocation/4a-Functional Allocation.sysml",
    // Promoted by #78 AST-eq span PartialEq + dependency `from` fix.
    "06-Individual and Snapshots/6-Individual and Snapshots.sysml",
    "07-Variant Configuration/7a-Variant Configuration - General Concept.sysml",
    "07-Variant Configuration/7a1-Variant Configuration - General Concept-a.sysml",
    // Promoted by #78 follow-up: Conditional `if ? else` + `->forAll {…}` brace bodies.
    "07-Variant Configuration/7b-Variant Configurations.sysml",
    // Promoted by #78 follow-up: `requirement references`, `require name`, `in :>>`.
    "08-Requirements/8-Requirements.sysml",
    // Promoted by #78 follow-up: verification case emit + action in case bodies + `:>>` bindings.
    "09-Verification/9-Verification-simplified.sysml",
    // Promoted by #78 follow-up: concern `subject;` / `stakeholder :>>` + viewpoint/view satisfy emit.
    "11-View and Viewpoint/11a-View-Viewpoint.sysml",
    "11-View and Viewpoint/11b-Safety and Security Feature Views.sysml",
    // Promoted by #78 follow-up: nested analysis usage emit in part/case bodies.
    "10-Analysis and Trades/10a-Analysis.sysml",
    // Promoted by #78 follow-up: `in part`/`in calc`/`in requirement`/`return part|attribute`,
    // nested calc rollups, and analysis for-loops.
    "10-Analysis and Trades/10b-Trade-off Among Alternative Configurations.sysml",
    "10-Analysis and Trades/10c-Fuel Economy Analysis.sysml",
    "10-Analysis and Trades/10d-Dynamics Analysis.sysml",
    "12-Dependency Relationships/12a-Dependency.sysml",
    // Promoted by #73: AllocationDef emit + structured `end` in allocation bodies.
    "12-Dependency Relationships/12b-Allocation-1.sysml",
    // Promoted by #78 follow-up: bare `allocate` shorthand emit (not `allocation allocate`).
    "12-Dependency Relationships/12b-Allocation.sysml",
    "13-Model Containment/13a-Model Containment.sysml",
    "13-Model Containment/13b-Safety and Security Features Element Group.sysml",
    "13-Model Containment/13b-Safety and Security Features Element Group-1.sysml",
    "13-Model Containment/13b-Safety and Security Features Element Group-2.sysml",
    // Promoted by the full-tree known-gap scan once port/interface/connect emit landed;
    // not deliberately targeted by this iteration beyond inventory.
    "14-Language Extensions/14a-Language Extensions.sysml",
    "14-Language Extensions/14b-Language Extensions.sysml",
    // Promoted by #78 follow-up: package `#` metadata prefix + `part` in item bodies.
    "14-Language Extensions/14c-Language Extensions.sysml",
    "15-Properties-Values-Expressions/15_01-Constants.sysml",
    "15-Properties-Values-Expressions/15_02-Basic Value Properties.sysml",
    "15-Properties-Values-Expressions/15_03-Value Expression.sysml",
    // Promoted by #78 follow-up: ConditionalExpression + CollectionOp brace body.
    "15-Properties-Values-Expressions/15_04-Logical Expressions.sysml",
    "15-Properties-Values-Expressions/15_05-Unification of Expression and Constraint Definition.sysml",
    "15-Properties-Values-Expressions/15_06-System of Quantities.sysml",
    "15-Properties-Values-Expressions/15_07-System of Units and Scales.sysml",
    "15-Properties-Values-Expressions/15_08-Range Restriction.sysml",
    "15-Properties-Values-Expressions/15_10-Primitive Data Types.sysml",
    // Promoted by #78 follow-up: `ref` / `ref part` in attribute & item bodies.
    "15-Properties-Values-Expressions/15_11-Variable Length Collection Types.sysml",
    "15-Properties-Values-Expressions/15_12-Compound Value Type.sysml",
    "15-Properties-Values-Expressions/15_13-Discretely Sampled Function Value.sysml",
    "15-Properties-Values-Expressions/15_19-Materials with Properties.sysml",
    "15-Properties-Values-Expressions/15_19a-Materials with Properties.sysml",
    // Promoted by #78 follow-up: occurrence def emit + event/message sequence bodies.
    "17-Sequence Modeling/17a-Sequence-Modeling.sysml",
    "17-Sequence Modeling/17b-Sequence-Modeling.sysml",
    // Promoted by #78 follow-up: objective body emit (no `requirement objective` reprint).
    "18-Use Case/18-Use Case.sysml",
];

/// Fixtures under the release's `sysml/src/examples/` tree that must currently roundtrip
/// successfully. Unlike [`ROUNDTRIP_PASS`] (the pinned, curated conformance target this parser
/// claims L1/L2/L2.5 against), `examples/` isn't a conformance gate: it's a much wider, less
/// curated sample of real-world SysML v2 source used only to track general parser
/// robustness (GH-83). `examples_roundtrip_scan` fails if any of these regress, or if a file
/// outside this list starts roundtripping (promote it here instead of letting it pass silently)
/// -- same discipline as `roundtrip_known_gaps_must_still_fail` applies to `ROUNDTRIP_PASS` --
/// but does *not* fail just because most of `examples/` still doesn't roundtrip; that's expected
/// and tracked as backlog, not a regression.
const EXAMPLES_ROUNDTRIP_PASS: &[&str] = &[
    "Arrowhead Framework Example/AHFProfileLib.sysml",
    "Camera Example/PictureTaking.sysml",
    "Comment Examples/Comments.sysml",
    "Geometry Examples/CarWithEnvelopingShape.sysml",
    "Import Tests/AliasImport.sysml",
    "Import Tests/CircularImport.sysml",
    "Import Tests/PrivateImportTest.sysml",
    "Import Tests/QualifiedNameImportTest.sysml",
    "Interaction Sequencing Examples/ServerSequenceModel.sysml",
    "Metadata Examples/IssueMetadataExample.sysml",
    "Packet Example/PacketUsage.sysml",
    "Packet Example/Packets.sysml",
    "Requirements Examples/HSUVRequirements.sysml",
    "Room Model/RoomModel.sysml",
    "Simple Tests/ConjugationTest.sysml",
    "Simple Tests/DefaultValueTest.sysml",
    "Simple Tests/DependencyTest.sysml",
    "Simple Tests/FeaturePathTest.sysml",
    "Simple Tests/ImportTest.sysml",
    "Simple Tests/MultiplicityTest.sysml",
    "Simple Tests/ParameterTest.sysml",
    "Simple Tests/RootPackageTest.sysml",
    "Simple Tests/TradeStudyTest.sysml",
    "Vehicle Example/VehicleDefinitions.sysml",
];

fn release_root() -> PathBuf {
    std::env::var_os("SYSML_V2_RELEASE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml-v2-release"))
}

fn validation_dir() -> PathBuf {
    release_root().join("sysml").join("src").join("validation")
}

fn examples_dir() -> PathBuf {
    release_root().join("sysml").join("src").join("examples")
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
        let orig_ch = pa.chars().nth(pos).unwrap_or('∅');
        let rep_ch = pb.chars().nth(pos).unwrap_or('∅');
        return RoundtripOutcome::Failed(format!(
            "AST-eq failed at char {pos} (orig={orig_ch:?} reparse={rep_ch:?}, lens {} vs {});\n  original: ...{orig_snip}...\n  reparse:  ...{reparse_snip}...\n  emitted head:\n{}",
            pa.len(),
            pb.len(),
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
fn emit_quoted_import_targets_reparse_gh71() {
    // Acceptance for #71: these fixtures previously failed *reparse* because import
    // targets dropped quotes. They may still fail opacity / AST-eq later.
    const FIXTURES: &[&str] = &[
        "04-Functional Allocation/4a-Functional Allocation.sysml",
        "06-Individual and Snapshots/6-Individual and Snapshots.sysml",
        "13-Model Containment/13a-Model Containment.sysml",
        "13-Model Containment/13b-Safety and Security Features Element Group.sysml",
        "14-Language Extensions/14a-Language Extensions.sysml",
    ];

    let root = validation_dir();
    if !root.exists() {
        eprintln!(
            "skipping: validation dir missing at {} (set SYSML_V2_RELEASE_DIR)",
            root.display()
        );
        return;
    }

    let mut failures = Vec::new();
    for rel in FIXTURES {
        let path = root.join(rel.replace('/', std::path::MAIN_SEPARATOR_STR));
        let src = match fs::read_to_string(&path) {
            Ok(s) => s.replace("\r\n", "\n").replace('\r', "\n"),
            Err(e) => {
                failures.push(format!("{rel}: read failed: {e}"));
                continue;
            }
        };
        let ast1 = match parse(&src) {
            Ok(a) => a,
            Err(e) => {
                failures.push(format!("{rel}: parse failed: {e}"));
                continue;
            }
        };
        let emitted = match emit_sysml(&ast1) {
            Ok(s) => s,
            Err(e) => {
                failures.push(format!("{rel}: emit failed: {e}"));
                continue;
            }
        };
        if let Err(e) = parse(&emitted) {
            failures.push(format!(
                "{rel}: reparse failed: {e}; emitted head=\n{}",
                emitted.chars().take(500).collect::<String>()
            ));
        } else {
            eprintln!("✓ reparse after emit {rel}");
        }
    }

    assert!(
        failures.is_empty(),
        "quoted-import reparse failures (#71):\n{}",
        failures.join("\n")
    );
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

/// GH-83: same roundtrip pipeline as `roundtrip_known_gaps_must_still_fail`, run against the
/// release's `sysml/src/examples/` tree instead of the pinned `validation/` conformance target.
///
/// This is a robustness tracker, not a conformance gate: `examples/` is a much wider, less
/// curated sample of real-world SysML v2 source, and most of it is expected to still fail (see
/// `EXAMPLES_ROUNDTRIP_PASS`'s doc comment). The test only fails on a *regression* -- something
/// in `EXAMPLES_ROUNDTRIP_PASS` no longer roundtripping, or something outside it now roundtripping
/// silently (promote it into the list instead). It never fails just because the overall pass rate
/// is low.
///
/// Run `ROUNDTRIP_DIAG=1 cargo test --test roundtrip_validation examples_roundtrip_scan --
/// --include-ignored --nocapture` for the per-file gap list.
#[test]
#[ignore = "requires SysML v2 release; run with --include-ignored"]
fn examples_roundtrip_scan() {
    let root = examples_dir();
    if !root.exists() {
        eprintln!(
            "skipping: examples dir missing at {} (set SYSML_V2_RELEASE_DIR)",
            root.display()
        );
        return;
    }

    let files = find_sysml_files(&root).expect("list example files");
    assert!(
        !files.is_empty(),
        "no .sysml files under {}",
        root.display()
    );

    let pass: std::collections::HashSet<&str> = EXAMPLES_ROUNDTRIP_PASS.iter().copied().collect();
    let mut regressions = Vec::new();
    let mut unexpected_passes = Vec::new();
    let mut gap_count = 0usize;
    let mut inventory: BTreeMap<String, (usize, usize)> = BTreeMap::new();

    for file in &files {
        let rel = rel_validation_path(file, &root);
        let folder = folder_of(&rel).to_string();
        let entry = inventory.entry(folder).or_insert((0, 0));
        let src = fs::read_to_string(file).expect("read example fixture");
        let outcome = try_roundtrip(&src);

        if pass.contains(rel.as_str()) {
            entry.0 += 1;
            if let RoundtripOutcome::Failed(msg) = outcome {
                regressions.push(format!("{rel}: {msg}"));
            }
            continue;
        }

        entry.1 += 1;
        gap_count += 1;
        match outcome {
            RoundtripOutcome::Ok => unexpected_passes.push(rel),
            RoundtripOutcome::Failed(msg) => {
                if std::env::var_os("ROUNDTRIP_DIAG").is_some() {
                    let head = msg.lines().next().unwrap_or(&msg);
                    let brief: String = head.chars().take(160).collect();
                    eprintln!("GAP {rel}: {brief}");
                }
            }
        }
    }

    eprintln!("examples/ robustness inventory (pass / known-gap):");
    for (folder, (pass_n, gap_n)) in &inventory {
        eprintln!("  {folder}: {pass_n} pass, {gap_n} known-gap");
    }
    eprintln!(
        "totals: {} tracked-pass, {gap_count} known-gap, {} files",
        EXAMPLES_ROUNDTRIP_PASS.len(),
        files.len()
    );

    assert!(
        regressions.is_empty(),
        "EXAMPLES_ROUNDTRIP_PASS regressions:\n{}",
        regressions.join("\n")
    );
    assert!(
        unexpected_passes.is_empty(),
        "these example fixtures now roundtrip — add them to EXAMPLES_ROUNDTRIP_PASS:\n{}",
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

#[test]
fn roundtrip_handwritten_quoted_import_smoke() {
    let src = r#"
package QuotedImports {
    private import '2a-Parts Interconnection'::*;
    public import 'Safety Features'::*;
    private import 'User Defined Extensions'::*;
    private import SI::kg;
}
"#;
    match try_roundtrip(src) {
        RoundtripOutcome::Ok => {}
        RoundtripOutcome::Failed(msg) => panic!("quoted import smoke failed: {msg}"),
    }
}

#[test]
fn roundtrip_handwritten_redefines_multiplicity_order_smoke() {
    // #74 / 7a1: multiplicity belongs after `:>> target`, not before the clause.
    let src = r#"
package RedefMult {
    part part3;
    abstract part def SubsystemA {
        abstract part :>> part3[0..1];
    }
}
"#;
    match try_roundtrip(src) {
        RoundtripOutcome::Ok => {}
        RoundtripOutcome::Failed(msg) => panic!("redefines/multiplicity order smoke failed: {msg}"),
    }
}

#[test]
fn roundtrip_handwritten_perform_action_in_part_usage_smoke() {
    // #74 / 3e: part-usage bodies require `perform action`, not bare `perform`.
    let src = r#"
package PerformAction {
    item def VehicleAssembly;
    item def Transmission;
    part AssemblyLine {
        perform action 'assemble vehicle' {
            action 'assemble transmission into vehicle' {
                in item transmission : Transmission;
            }
            flow 'assemble transmission into vehicle'.transmission
                to 'assemble transmission into vehicle'.transmission;
        }
        perform action providePower;
    }
}
"#;
    match try_roundtrip(src) {
        RoundtripOutcome::Ok => {}
        RoundtripOutcome::Failed(msg) => panic!("perform action in part usage smoke failed: {msg}"),
    }
}

#[test]
fn roundtrip_handwritten_state_do_out_and_assert_smoke() {
    // #72: state do/out + attribute assert constraint must stay structured (not Other).
    let src = r#"
package StateDoOut {
    attribute def TemperatureValue;
    attribute def Real;
    action def 'Sense Temperature' { out temp: TemperatureValue; }
    attribute e: Real {
        assert constraint { e > 0.0 }
    }
    state 'health' {
        do 'sense temperature' { out temp; }
        accept 'Return to Normal' then normal;
        state normal;
    }
}
"#;
    match try_roundtrip(src) {
        RoundtripOutcome::Ok => {}
        RoundtripOutcome::Failed(msg) => panic!("state do/out + assert smoke failed: {msg}"),
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
