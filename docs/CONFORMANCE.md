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

## Claim layers

| Layer | Claim | Evidence |
| ----- | ----- | -------- |
| **L1 — Syntax acceptance** | Every textual BNF production in the pinned release is classified `implemented` in [`bnf_coverage.map`](./bnf_coverage.map) | `cargo test --test bnf_compliance`; scorecard `layers.L1` |
| **L2 — Structured AST** | Systems Library and full `sysml.library` parse with zero diagnostics and zero `ExtendedLibraryDecl` | ignored validation gates; scorecard `layers.L2` |
| **L3 — Semantics** | Not claimed (Spec42 / other tools) | scorecard `layers.L3.status = not_claimed` |

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
