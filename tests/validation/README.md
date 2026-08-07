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

### Remaining known-gap classes (after emitter expansion)

With emit arms in place for ports, interfaces, actions, states, requirements, constraints, calcs, flows, views, metadata, analysis parameters/returns, etc., the pinned validation tree currently has **no known-gap fixtures**. New failures classify roughly as:

| Class | Meaning | Typical next work |
| ----- | ------- | ----------------- |
| `opacity` / `emit opaque` (`Other`, `ExtendedLibraryDecl`, KerML decls) | Parser recovered opaquely | Parser structure recovery |
| `AST-eq` | Emits and reparses, but AST differs | Emit fidelity or lost parse fields (e.g. assume vs require) |
| `reparse` | Emitted text is not accepted | Emit quoting/shape bugs, or missing grammar in reparse path |

### ExtendedLibraryDecl / KermlFeatureDecl (#73)

Package-level fallbacks for SysML keywords that failed structured dispatch. Cleared for validation
fixtures `08`, `09`, `12b-Allocation-1`, and `14c` by parsing requirement short names, allocation
`::>` ends, and `nonunique` occurrence usages. `12b-Allocation-1` is in `ROUNDTRIP_PASS` (AllocationDef
emit + structured body `end`). Remaining failures on `08`/`09`/`14c` are nested `Other` /
unsupported emit (not those fallback kinds).

### Intentional AST PartialEq normalization (#74 / #78)

Roundtrip AST equality ignores source locations that do not affect semantics. `Membership` / `Node`
already did this; #74 covered `PayloadClause` / `ActionUsage`; #78 extended the same convention to
attribute/part/port/ref/metadata/import/state then-final and related `*_span` fields. Remaining
`12b-Allocation` fails structurally (reparse → `ExtendedLibraryDecl`), not as a span phantom.

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
