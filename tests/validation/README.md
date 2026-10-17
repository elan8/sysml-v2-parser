# Validation fixture tests

Each SysML file under `sysml-v2-release/sysml/src/validation/` can have a corresponding Rust test module in this directory. Populate `./sysml-v2-release/` with `scripts/fetch-sysml-v2-release.ps1` or `scripts/fetch-sysml-v2-release.sh`, or point `SYSML_V2_RELEASE_DIR` at an unpacked release tree.

## Preferred quality gate: roundtrip (L2.5)

Primary fidelity evidence for validation fixtures is **`tests/roundtrip_validation.rs`**:

`parse → opacity-clean → emit → reparse → AST-eq`

**Iteration 2 scope:** the full pinned `sysml/src/validation/` tree is under known-gap scan. Fixtures in `ROUNDTRIP_PASS` must roundtrip; every other `.sysml` must still fail until promoted.

- Add fixtures to `ROUNDTRIP_PASS` when they roundtrip.
- Currently required-pass: the full pinned validation tree (`56` fixtures). See `ROUNDTRIP_PASS`
  in `tests/roundtrip_validation.rs`.
- Run: `cargo test --test roundtrip_validation -- --include-ignored` (with release tree available). The known-gap test prints a per-folder pass/gap inventory.
- Diagnose remaining gaps: `$env:ROUNDTRIP_DIAG=1; cargo test --test roundtrip_validation roundtrip_known_gaps_must_still_fail -- --include-ignored --nocapture`

### Inventory (pinned `2026-04`)

| Folder | Pass | Known gap |
| ------ | ---- | --------- |
| 01-Parts Tree | 3 | 0 |
| 02-Parts Interconnection | 2 | 0 |
| 03-Function-based Behavior | 8 | 0 |
| 04-Functional Allocation | 1 | 0 |
| 05-State-based Behavior | 3 | 0 |
| 06-Individual and Snapshots | 1 | 0 |
| 07-Variant Configuration | 3 | 0 |
| 08-Requirements | 1 | 0 |
| 09-Verification | 1 | 0 |
| 10-Analysis and Trades | 4 | 0 |
| 11-View and Viewpoint | 2 | 0 |
| 12-Dependency Relationships | 3 | 0 |
| 13-Model Containment | 4 | 0 |
| 14-Language Extensions | 3 | 0 |
| 15-Properties-Values-Expressions | 14 | 0 |
| 17-Sequence Modeling | 2 | 0 |
| 18-Use Case | 1 | 0 |
| **Totals** | **56** | **0** |

### Failure classification

The pinned validation tree currently has no known-gap fixtures. New failures distinguish parser
recovery/unstructured nodes, unsupported emission, reparse errors and AST fidelity mismatches.
The opaque declaration variants were removed in v0.58.0; do not restore a fallback to make a
fixture pass. Source locations that do not affect meaning are normalized for AST comparison.

The primary parser contract is the semantic Markdown suite under `tests/snapshots`, managed by
`cargo snapshot check` and `cargo snapshot update`. Its fixtures expose diagnostics, retained
syntax, recovery, emission and parse/format/reparse stability. The three legacy Debug snapshots
and their unwired Rust modules are historical artifacts, not current test gates. There is no
active `UPDATE_VALIDATION_AST` regeneration path.

## Layout

- `tests/validation.rs` wires the active modules and owns release-path/logging helpers.
- `tests/validation/full_validation_suite.rs` parses the entire pinned validation corpus.
- `tests/validation/full_library_suite.rs` owns the strict library diagnostics/structure gates.
- `tests/roundtrip_validation.rs` owns validation and broader-example round trips.
- `tests/snapshots/` contains the primary semantic parser contract.
- `tests/validation/snapshots/` retains three historical Debug dumps; no current test reads them.

## Adding coverage

1. Promote a new fixture into `ROUNDTRIP_PASS` only after the strict round-trip pipeline passes.
2. For targeted language behavior, add a semantic/provenance/recovery Markdown snapshot under
   `tests/snapshots`; migrate a touched private-field assertion to that public projection.
3. Reserve focused Rust tests for owning mechanics such as arena validation, rollback, resource
   limits and failing output sinks. Avoid new Debug snapshots.

After changing fixture source or an owning formatter, run `cargo snapshot update`, review the
complete diff, then run `cargo snapshot check`. Generated sections must not be edited by hand.
Run `cargo test --all-features -- --include-ignored` with the pinned fixtures for the broad gates;
ordinary `cargo test` does not execute ignored corpus tests.
