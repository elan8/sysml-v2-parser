# Validation fixture tests

Each SysML file under `sysml-v2-release/sysml/src/validation/` can have a corresponding Rust test module in this directory. Populate `./sysml-v2-release/` with `scripts/fetch-sysml-v2-release.ps1` or `scripts/fetch-sysml-v2-release.sh`, or point `SYSML_V2_RELEASE_DIR` at an unpacked release tree.

## Preferred quality gate: roundtrip (L2.5)

Primary fidelity evidence for validation fixtures is **`tests/roundtrip_validation.rs`**:

`parse → opacity-clean → emit → reparse → AST-eq`

**Iteration 2 scope:** the full pinned `sysml/src/validation/` tree is under known-gap scan. Fixtures in `ROUNDTRIP_PASS` must roundtrip; every other `.sysml` must still fail until promoted.

- Add fixtures to `ROUNDTRIP_PASS` when they roundtrip.
- Currently required-pass: all of `01-Parts Tree/` (`1a`/`1c`/`1d`), all of `02-Parts Interconnection/` (`2a`/`2c`), `04` (`4a`), `13b`, plus incidental `14b` / `15_02` / `15_03` / `15_06` / `15_07`.
- Run: `cargo test --test roundtrip_validation -- --include-ignored` (with release tree available). The known-gap test prints a per-folder pass/gap inventory.
- Diagnose remaining gaps: `$env:ROUNDTRIP_DIAG=1; cargo test --test roundtrip_validation roundtrip_known_gaps_must_still_fail -- --include-ignored --nocapture`

### Inventory (pinned `2026-04`)

| Folder | Pass | Known gap |
| ------ | ---- | --------- |
| 01-Parts Tree | 3 | 0 |
| 02-Parts Interconnection | 2 | 0 |
| 03-Function-based Behavior | 0 | 8 |
| 04-Functional Allocation | 1 | 0 |
| 05-State-based Behavior | 0 | 3 |
| 06-Individual and Snapshots | 0 | 1 |
| 07-Variant Configuration | 0 | 3 |
| 08-Requirements | 0 | 1 |
| 09-Verification | 0 | 1 |
| 10-Analysis and Trades | 0 | 4 |
| 11-View and Viewpoint | 0 | 2 |
| 12-Dependency Relationships | 0 | 3 |
| 13-Model Containment | 1 | 3 |
| 14-Language Extensions | 1 | 2 |
| 15-Properties-Values-Expressions | 4 | 10 |
| 17-Sequence Modeling | 0 | 2 |
| 18-Use Case | 0 | 1 |
| **Totals** | **12** | **44** |

### Remaining known-gap classes (after emitter expansion)

With emit arms in place for ports, interfaces, actions, states, requirements, constraints, calcs, flows, views, metadata, etc., remaining failures classify roughly as:

| Class | Meaning | Typical next work |
| ----- | ------- | ----------------- |
| `opacity` / `emit opaque` (`Other`, `ExtendedLibraryDecl`, KerML decls) | Parser recovered opaquely | Parser structure recovery |
| `AST-eq` | Emits and reparses, but AST differs | Emit fidelity or lost parse fields (e.g. assume vs require) |
| `reparse` | Emitted text is not accepted | Emit quoting/shape bugs, or missing grammar in reparse path |

Set `ROUNDTRIP_DIAG=1` when running the known-gap test to print the first failure reason per fixture.

Debug AST snapshots below are **regression canaries only**. They store `Debug` dumps of the AST and fail when that dump changes. Regenerating with `UPDATE_VALIDATION_AST=1` locks in current parser output — including wrong classifications — so snapshots are not a correctness oracle. Prefer roundtrip for new coverage; do not grow the snapshot set.

## Layout

- **`tests/validation.rs`** – Shared helpers (`release_root`, `validation_fixture_path`, `assert_ast_eq`, `assert_ast_snapshot`) and module wiring.
- **`tests/validation/<name>.rs`** – One module per validation fixture (parse / shape / snapshot).
- **`tests/validation/snapshots/`** – Normalized AST Debug snapshots (frozen regression canaries).
- **`tests/roundtrip_validation.rs`** – Emit-fidelity roundtrip suite (L2.5).

## Adding a new validation test

1. Prefer extending roundtrip coverage (promote a fixture into `ROUNDTRIP_PASS` once the emitter supports it).
2. For targeted construct probes, add a `.rs` file in `tests/validation/` and wire it in `tests/validation.rs`.
3. Use `assert_ast_eq` only for small hand-built expected trees; avoid new Debug snapshots.

## When to regenerate snapshots

Refresh checked-in AST snapshots **in the same PR** whenever parser output changes for the three existing snapshot fixtures, for example:

- new optional fields on existing AST structs
- new enum variants on body-element types
- a construct now parses into a different variant

CI always runs `cargo test -- --include-ignored`; local `cargo test` alone does **not** run these ignored tests.

## Regenerate

All snapshot fixtures:

```powershell
$env:UPDATE_VALIDATION_AST = "1"
cargo test --test validation -- --include-ignored
Remove-Item Env:UPDATE_VALIDATION_AST
```

Or a subset (faster while iterating):

```powershell
$env:UPDATE_VALIDATION_AST = "1"
cargo test --test validation test_parse_1a_parts_tree test_parse_3a_function_based_behavior -- --include-ignored
Remove-Item Env:UPDATE_VALIDATION_AST
```

Unset `UPDATE_VALIDATION_AST` before committing. Review the `.txt` diff in `snapshots/` — only intentional AST changes should appear.
