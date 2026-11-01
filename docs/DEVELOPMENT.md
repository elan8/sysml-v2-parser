# Developing the parser

Principles for changing the parser are in [`AGENTS.md`](../AGENTS.md). Release steps are in
[`RELEASING.md`](RELEASING.md).

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo snapshot check
```

**Full validation suite** (CI validation job — includes ignored slow/corpus tests):

```bash
./scripts/fetch-sysml-v2-release.sh   # or scripts/fetch-sysml-v2-release.ps1 (reads docs/conformance-target)
cargo test --all-features -- --include-ignored
```

Set `SYSML_V2_RELEASE_DIR` if fixtures are not in `./sysml-v2-release`.
The pin lives in [`docs/conformance-target`](conformance-target); see [`docs/CONFORMANCE.md`](CONFORMANCE.md).
Enable `CONFORMANCE_SCORECARD_L2=1` and `RUN_STRICT_LIBRARY_GATES=1` for the library scorecard
and strict gates. Release preparation and the tag-triggered publishing workflow are documented in
[`docs/RELEASING.md`](RELEASING.md).

**Optional MBSE vacuum corpus** (ignored integration tests; skips when unset):

```bash
export MBSE_VACUUM_EXAMPLE_DIR=/path/to/MBSE_AG_vacuum-cleaner-robot-example
cargo test --test vacuuming_types_parse -- --include-ignored
```

When changing AST fields or body-element shapes, refresh checked-in snapshots in the same PR — see [`tests/validation/README.md`](../tests/validation/README.md).

The library exposes [`SUPPORTED_GRAMMAR`](https://docs.rs/sysml-v2-parser/latest/sysml_v2_parser/constant.SUPPORTED_GRAMMAR.html), containing the pinned SysML release tag, repository, and combined textual-BNF content hash. Build-time validation derives it from `docs/conformance-target` and rejects a checked-out grammar whose bytes do not match the pin.

The driver in `tools/snapshot_tool` manages every snapshot under `tests/snapshots`:
hand-authored qualified-reference fixtures in `qualified_references`, and the imported spec42
corpus in `spec42`, whose fixtures cover the SysML and KerML libraries, spec examples, training
models, and recorded fuzz findings. They use five canonical Markdown sections: human-authored
`META` and `SOURCE`, followed by runner-owned `DIAGNOSTICS`, `FORMAT`, and semantic S-expression
`AST`. `META` has the required shape
`(snapshot (type <type>) (description "..."))`; `<type>` is one of `semantic`, `provenance`,
`recovery`, or `malformed`, and the description must state the fixture's non-empty testing intent.
The driver validates and preserves META but never generates its description. The AST nests
reference uses at their language-level roles and separately
records each reference's scope and ordered identifier tokens, decoded names, separators, and
spans—without an aggregate path string. Recovery nodes retain their exact
source span, so malformed fixtures still produce all five sections and preserve valid siblings in
the formatted output.

`FORMAT` is always derived from `SOURCE`. When its canonical payload is byte-identical to the
authored source payload, the section uses the compact `~~~sexpr` sentinel
`(stable-idempotent)`. When formatting changes any byte, `FORMAT` instead contains the complete
emitted document in a `~~~sysml` fence. A valid parsed tree may intentionally retain opaque or
not-yet-emittable syntax; in that case the snapshot remains valid and uses the deterministic
`~~~sexpr` sentinel `(unavailable (reason opaque-ast))`. Other emitter failures still abort
snapshot generation. The driver recomputes this choice from `SOURCE`; it never uses either
sentinel as source text or as cached formatter output.

For diagnostic-free, emittable fixtures whose formatter changes `SOURCE`, the driver strictly
reparses the generated `FORMAT`, compares the two span-insensitive semantic S-expressions, and
requires a second formatting pass to reproduce `FORMAT` byte-for-byte. Thus non-canonical authored
input remains useful coverage while generated output is both meaning-preserving and stable. New
hand-authored semantic fixtures should normally use canonical source; imported, provenance,
recovery, malformed, and deliberately non-canonical source is migrated only when independently in
scope. Four pre-existing violations are pinned in the driver to their exact fixture path and
`SOURCE`/`FORMAT` fingerprint. Changing either payload re-enables the gate, so the baseline cannot
hide new regressions and each pin can be removed independently when its formatter path is repaired.

The driver delegates its AST section to the library's `ast::WriteSemanticAst` boundary, which
streams bytes to any `std::io::Write` destination (for example a file or `Vec<u8>`). Its exhaustive
enum matches make newly added AST variants a compile-time formatting decision.

```bash
cargo snapshot check
cargo snapshot update
cargo snapshot check --fixture semantic_references.md
cargo snapshot check --root path/to/snapshots
```

The driver processes fixtures sequentially in sorted path order. `check` fails on stale derived
sections; `update` rewrites them for normal `git diff` review.

### Parser benchmarks

The maintained Criterion benchmark extracts the authored `SOURCE` section from every parser
snapshot. This keeps timing and profiling inputs aligned with the same semantic, provenance,
recovery, and malformed cases used by the end-to-end contract instead of relying on a developer's
local checkout or a synthetic model.

```bash
cargo bench --bench parser_bench
cargo bench --bench parser_bench -- --save-baseline before
cargo bench --bench parser_bench -- --baseline before
cargo bench --bench parser_bench -- --profile-time 10
```

It reports each fixture separately and the complete corpus as one workload, over every fixture
under `tests/snapshots`. Keep benchmark changes
and their source snapshots reviewable together, and report unfavorable movements alongside wins.

For low-noise sampling, use the standalone release binary. It loads the identical snapshot corpus,
warms it once, then runs only the parser loop; release and bench profiles retain debug symbols.

```bash
cargo build --release --bin parser_profile
samply record target/release/parser_profile 1000
```

The optional integer is the number of complete corpus iterations and defaults to 1,000. Increase it
until the profile contains enough steady-state samples for the question being investigated.

## Snapshot name language and keywords

Snapshot META may end with `(name-language kerml)` or `(name-language sysml)` after its description;
omitting it selects SysML. The snapshot and serialization corpus share the same authored metadata
reader, and emission round trips preserve the selected vocabulary. Keyword tables are generated
from the pinned `RESERVED_KEYWORD` productions with
`python tools/generate_reserved_keywords.py --bnf-root sysml-v2-release/bnf`; use `--check` to verify
both the grammar-content pin and the inventory without rewriting it.

Syntax consumers can borrow `NameLanguage::reserved_keywords()` to use the same pinned vocabulary.
