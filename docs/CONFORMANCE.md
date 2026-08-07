# Textual-notation conformance

This crate claims **SysML/KerML textual notation** conformance against a pinned
OMG [SysML-v2-Release](https://github.com/Systems-Modeling/SysML-v2-Release) tag.
It does **not** claim semantic conformance (name resolution, typing, well-formedness).

## Pinned target

Single source of truth:

- [`docs/conformance-target`](./conformance-target)

Fetch scripts, CI, and tests all read `release_tag` (and BNF production counts)
from that file. After fetch, the unpacked tree contains
`sysml-v2-release/.elan8-conformance-target` so CI can prove the fixture tree
matches the pin.

```powershell
./scripts/fetch-sysml-v2-release.ps1          # uses pinned tag
./scripts/fetch-sysml-v2-release.ps1 2026-04  # optional override
```

```bash
./scripts/fetch-sysml-v2-release.sh           # uses pinned tag
./scripts/fetch-sysml-v2-release.sh 2026-04   # optional override
```

## Entry points

Conformance/roundtrip gates (L1/L2/L2.5 below) must use `parse()` (strict, all-or-nothing),
never `parse_for_editor()` (partial AST + diagnostics, for IDE/LSP use). The two are guaranteed
to agree on clean input -- same errors-empty verdict, same AST once spans are normalized out --
but must not be mixed for the same document within one caller (e.g. parsing once with each and
comparing across them): that was the GH-66/GH-69 roundtrip-harness bug class, where an apparent
AST mismatch was really the two entry points disagreeing, not an emit/parser bug. See `src/lib.rs`
crate docs and `tests/validation/parse_entry_point_equivalence.rs` (GH-70).

## Claim layers

| Layer | Claim | Evidence |
| ----- | ----- | -------- |
| **L1 — Syntax acceptance** | Every textual BNF production in the pinned release is classified `implemented` in [`bnf_coverage.map`](./bnf_coverage.map) | `cargo test --test bnf_compliance`; scorecard `layers.L1` |
| **L2 — Structured AST** | Systems Library and full `sysml.library` parse with zero diagnostics and zero `ExtendedLibraryDecl` | ignored validation gates; scorecard `layers.L2` |
| **L2.5 — Emit fidelity** | Selected release validation fixtures roundtrip: parse → opacity-clean → emit canonical SysML → reparse → AST-eq | `cargo test --test roundtrip_validation -- --include-ignored` |
| **L3 — Semantics** | Not claimed (Spec42 / other tools) | scorecard `layers.L3.status = not_claimed` |

L2.5 does **not** claim semantic correctness. It claims that the AST for listed fixtures is structured enough to reprint as SysML and reparse to an equivalent tree. Opaque / recovery nodes (`Other`, `OpaqueMember`, KerML fallbacks, `ParseError`, …) fail the gate rather than fake-passing by reprinting raw text.

**Iteration 2:** known-gap scan covers the full pinned `sysml/src/validation/` tree. Required-pass fixtures currently include all of `01-Parts Tree/` and `02-Parts Interconnection/`, all `3a` plus `3c-1`/`3c-3`, `05-1a`, `4a`, `13b`, plus incidental passers in `14`/`15` (`14b`, `15_01`, `15_02`, `15_03`, `15_06`, `15_07`). Remaining fixtures must still fail until promoted into `ROUNDTRIP_PASS`.

Emitter coverage for the validation set is largely in place; remaining known gaps are dominated by **parser opacity** (`Other`, `ExtendedLibraryDecl`, KerML fallbacks), **AST-eq / reparse fidelity**, and a few structured constructs that still need parser work (not missing emit arms). Run `ROUNDTRIP_DIAG=1 cargo test --test roundtrip_validation roundtrip_known_gaps_must_still_fail -- --include-ignored --nocapture` for a per-fixture classification.

Debug AST snapshots under `tests/validation/snapshots/` remain **regression canaries** only (they detect that parser output changed). They are not a correctness oracle: regenerating them locks in whatever the parser currently produces. New validation coverage should prefer roundtrip over growing Debug snapshots.

## Robustness tracker (`examples/`, not a conformance gate)

The pinned `sysml/src/validation/` tree above is a curated conformance target, not a general
robustness benchmark -- passing 100% of it proves conformance to that specific corpus, not that
the parser handles arbitrary valid SysML v2 source. The release also ships `sysml/src/examples/`
(95 files across 22 folders): a much wider, less curated sample of real-world models. GH-83 tracks
this with the same roundtrip pipeline via `EXAMPLES_ROUNDTRIP_PASS` / `examples_roundtrip_scan`
in `tests/roundtrip_validation.rs` -- currently 22/95 roundtrip. Unlike the `validation/` gate,
this one does **not** require 100%: it only fails on a regression (something in
`EXAMPLES_ROUNDTRIP_PASS` stops roundtripping) or an unpromoted pass (something outside the list
starts roundtripping and should be added). Run `ROUNDTRIP_DIAG=1 cargo test --test
roundtrip_validation examples_roundtrip_scan -- --include-ignored --nocapture` for the per-file
gap list.

## Scorecard (CI artifact)

```bash
./scripts/fetch-sysml-v2-release.sh
# L1 always; set CONFORMANCE_SCORECARD_L2=1 to include library AST gates
CONFORMANCE_SCORECARD_L2=1 cargo test --test conformance_scorecard -- --nocapture
```

Writes (override directory with `CONFORMANCE_SCORECARD_DIR`):

- `target/conformance-scorecard.json`
- `target/conformance-scorecard.md`

CI validation uploads both as the `conformance-scorecard` artifact. The markdown
file is the human-readable split of what we claim vs what we explicitly do not
claim for the pinned release.
