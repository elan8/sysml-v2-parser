# Textual-notation conformance

This crate targets **SysML/KerML textual notation** conformance against a pinned
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
| **L1 — Syntax acceptance** | Every textual BNF production in the pinned release is classified `implemented` in [`bnf_coverage.map`](./bnf_coverage.map) | `cargo test --test bnf_compliance`; scorecard `layers.L1_syntax_acceptance` |
| **L2 — Structured AST** | Systems Library and full `sysml.library` have zero unexpected diagnostics and unstructured nodes outside the two pinned recovery controls below | ignored validation gates; scorecard `layers.L2_structured_ast` |
| **L2.5 — Emit fidelity** | Selected release validation fixtures roundtrip: parse → opacity-clean → emit canonical SysML → reparse → AST-eq | `cargo test --test roundtrip_validation -- --include-ignored` |
| **L3 — Semantics** | Not claimed (Spec42 / other tools) | scorecard `layers.L3_semantics.status = not_claimed` |

L2.5 does **not** claim semantic correctness. Strict parsing, a clean opacity scan, canonical
emission and reparsing must preserve the normalized AST. Recovery and unsupported nodes fail
this gate; raw source replay is not accepted as proof of a structured round trip.

The library scorecard retains two exact pre-existing policy exceptions: `Flows.sysml:82`
(`end occurrence source`) and `Interfaces.sysml:72` (`end port source`). If their expected primary
and cascade diagnostics match, the scorecard excludes those diagnostics and opacity hits. A passing
scorecard alone therefore does not prove these two files are diagnostic-free; the separate strict
library tests require zero diagnostics without those exceptions. See `is_pinned_policy_recovery`
in [`tests/conformance_scorecard.rs`](../tests/conformance_scorecard.rs).

The pinned `2026-04` validation tree contains **56 required-pass fixtures and no known gaps**.
`ROUNDTRIP_PASS` in `tests/roundtrip_validation.rs` records the exact inventory. The complementary
known-gap scan catches both regressions and a changed inventory; a newly passing fixture must be
promoted deliberately. Run:

```sh
cargo test --test roundtrip_validation -- --include-ignored
```

The semantic Markdown snapshots under `tests/snapshots/` are the primary end-to-end parser
contract. They record diagnostics, formatting and typed semantic projections, including recovery
spans. Three legacy Debug snapshots and their unwired fixture modules remain as historical artifacts;
they are not executed by the current validation target. New behavior coverage belongs in the
semantic snapshots, not private Rust field assertions.

## Robustness tracker (`examples/`, not a conformance gate)

The pinned `sysml/src/examples/` corpus contains **95 files**, with **85 tracked round trips**
in `EXAMPLES_ROUNDTRIP_PASS`. The remaining files are known gaps, not an acceptance claim. The
regression scan rejects a lost tracked pass or an unrecorded new pass. Use:

```sh
ROUNDTRIP_DIAG=1 cargo test --test roundtrip_validation examples_roundtrip_scan -- --include-ignored --nocapture
```

These counts describe this pinned corpus, not arbitrary SysML programs. The independent
[parser oracle](PARSER-ORACLE.md) supplies additional differential evidence; a disagreement
must be checked against the grammar rather than treating either parser as authoritative.

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


## Toolkit reference pilot

The optional reference audit uses `SysML.xtext` from a checksum-pinned
[Open-MBEE toolkit revision](https://github.com/Open-MBEE/sysml-toolkit/tree/7d740b37c0666be18bee2de1b467c86a234788bc/spec-refs).
This is supplementary Pilot grammar evidence for the existing OMG release conformance target,
for abstract item definitions and explicit reference-attribute redefinitions. Run it by hand:
the fetched grammar is fixed by its checksum, so CI would only add a dependency on GitHub's raw
file host. CI covers the two typed parser snapshots with the rest of the snapshot corpus.

```sh
python3 tools/check_toolkit_grammar.py .cache/SysML.xtext --fetch
python3 tools/check_toolkit_grammar.py .cache/SysML.xtext
cargo snapshot check --fixture sysml/definition_prefix_alternatives.md
cargo snapshot check --fixture sysml/ref_prefix_coverage.md
```
