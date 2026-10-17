# sysml-v2-parser

[![crates.io](https://img.shields.io/crates/v/sysml-v2-parser.svg)](https://crates.io/crates/sysml-v2-parser)

SysML v2 textual notation parser for Rust. Parses SysML v2 and KerML textual syntax into an AST, with a resilient editor mode that returns partial trees plus diagnostics.

Release notes: [`CHANGELOG.md`](CHANGELOG.md).

## Features

- Strict `parse()` and resilient `parse_for_editor()` entry points
- Broad SysML v2 subset including port-def directed features (`in`/`out`/`inout` attribute and item usages)
- BNF coverage gate: 640 textual productions classified as `implemented` ([`docs/BNF_COVERAGE.md`](docs/BNF_COVERAGE.md))
- Pinned textual-notation conformance target + CI scorecard ([`docs/CONFORMANCE.md`](docs/CONFORMANCE.md))
- Green default test suite; full validation and std-library gates with SysML v2 release fixtures

## API

```rust
use sysml_v2_parser::parse;

fn main() {
    let document = parse("package Demo;").expect("valid SysML");
    assert_eq!(document.root.elements.len(), 1);
}
```

- `parse(input)` — strict parse; returns an atomic `ParsedDocument` containing the BOM-stripped
  source, document-local qualified-reference arena, and root AST
- `parse_for_editor(input)` — partial `ParsedDocument` + diagnostics for editors and language
  servers

The default entry points use SysML reserved names. For a KerML source, use
`parse_with_name_language(input, ast::NameLanguage::Kerml)` or the corresponding owned/editor
entry point. `ParsedDocument::name_language` travels with the atomic document, including its
serialized envelope. This option selects the lexical name vocabulary; it does not restrict the
parser's structural production subset to one language. Unquoted reserved declaration names are
retained as `DeclarationNameState::ReservedKeyword`, produce `reserved_declaration_name` diagnostics,
and have no decoded valid name. Quote keyword names with single quotes to make them valid names.

Semantic references in the AST are opaque, document-local identities. Resolve them through
`ParsedDocument::qualified_reference` to borrow their authored segments, separator kinds, and
source spans without splitting or reparsing display strings.

Imports require explicit `public`, `private`, or `protected` visibility. A bare import is retained
as malformed source and reports `missing_import_visibility` on its keyword.

Declaration names are optional independently of short names in `Identification`. For example,
`verification <V1> : VerificationType;` retains its short name while its AST `name` is `None`.

`KermlSuccessionMember::multiplicity_modifiers` retains authored declaration ordering and
uniqueness keywords separately from its endpoint multiplicities.

`RequireConstraint::specializations` retains ordered, typed `FeatureSpecialization` clauses,
including repeated typings, references and crosses. It replaces the former `typing`, `subsets`
and `redefines` fields. `RequireConstraint::value` separately retains the declaration's binding,
assignment or default value; formatting and semantic snapshots consume these typed fields.

`ParseErrorNode::diagnostic_span` optionally highlights an offending token within the retained
malformed source slice. The enclosing node span remains the authority for recovery emission;
deserialization rejects diagnostic spans outside it or with invalid source coordinates.

`PARSE_AST_VERSION` 284 replaces the flattened state-action fields on `EntryAction`, `DoAction`
and `ExitAction` with `action: StateActionUsage`. Match `Empty`, `Perform { target, value }`,
or `Effect`; the perform target reuses `PerformActionTarget`. Its `Action` branch owns a full
`UsageDeclaration`, including anonymous identification, all typing targets and specialization
clauses. The `value` retains the authored assignment/default operator and expression. State
subaction `body` fields now use `ActionDefBody`, matching the `ActionBody` production; the
obsolete state-body-only `FirstStmt` and `ThenAction` proxy variants are removed.

Action bodies expose typed part, item, view, rendering and viewpoint definitions and the case
usage families. Calculation bodies delegate those members through `ActionMember`.
`UseCaseDefBodyElement` and `StateDefBodyElement` retain `VariantUsage` members; the typed
variant alternatives include case families, state, calculation and constraint usages.

`PARSE_AST_VERSION` 283 removes the opaque declaration variants `ExtendedLibraryDecl`,
`FeatureDecl`, `ClassifierDecl`, `KermlSemanticDecl` and `KermlFeatureDecl`, together with
`OpaqueText` and `ParsedDocument::opaque_text`. Consumers should handle typed declarations
and `ParseErrorNode` recovery. `emit_recovered_sysml` preserves malformed source in place;
a failed declaration parser reports an error instead of accepting its leading keyword as
proof of valid grammar. The conformance scorecard now reports `unstructured_nodes` through
the shared opacity scan.

With the optional `serde` feature, serialize and deserialize `ParsedDocument` as the atomic cache
unit. Its wire envelope includes `PARSE_AST_VERSION` and rejects version mismatches, invalid arena
ranges, and dangling AST reference identities during deserialization.

## Development

```bash
cargo test
cargo clippy -- -W clippy::all
```

**Full validation suite** (CI validation job — includes ignored slow/corpus tests):

```bash
./scripts/fetch-sysml-v2-release.sh   # or scripts/fetch-sysml-v2-release.ps1 (reads docs/conformance-target)
cargo test -- --include-ignored
```

Set `SYSML_V2_RELEASE_DIR` if fixtures are not in `./sysml-v2-release`.
The pin lives in [`docs/conformance-target`](docs/conformance-target); see [`docs/CONFORMANCE.md`](docs/CONFORMANCE.md).

**Optional MBSE vacuum corpus** (ignored integration tests; skips when unset):

```bash
export MBSE_VACUUM_EXAMPLE_DIR=/path/to/MBSE_AG_vacuum-cleaner-robot-example
cargo test --test vacuuming_types_parse -- --include-ignored
```

When changing AST fields or body-element shapes, refresh checked-in snapshots in the same PR — see [`tests/validation/README.md`](tests/validation/README.md).

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

## Documentation

| Topic | Doc |
|-------|-----|
| Backlog & roadmap | [`docs/PARSER_BACKLOG_ROADMAP.md`](docs/PARSER_BACKLOG_ROADMAP.md) |
| Spec42 diagnostics | [`docs/SPEC42-DIAGNOSTICS-PARSER-IMPROVEMENTS.md`](docs/SPEC42-DIAGNOSTICS-PARSER-IMPROVEMENTS.md) |
| Error recovery | [`docs/ERROR_RECOVERY.md`](docs/ERROR_RECOVERY.md) |
| BNF coverage | [`docs/BNF_COVERAGE.md`](docs/BNF_COVERAGE.md) |
| Conformance target & scorecard | [`docs/CONFORMANCE.md`](docs/CONFORMANCE.md) |
| Compliance gap | [`docs/SYSML_V2_COMPLIANCE_GAP.md`](docs/SYSML_V2_COMPLIANCE_GAP.md) |
| Technical debt | [`docs/PARSER_TECHNICAL_DEBT.md`](docs/PARSER_TECHNICAL_DEBT.md) |

Snapshot META may end with `(name-language kerml)` or `(name-language sysml)` after its description;
omitting it selects SysML. The snapshot and serialization corpus share the same authored metadata
reader, and emission round trips preserve the selected vocabulary. Keyword tables are generated
from the pinned `RESERVED_KEYWORD` productions with
`python tools/generate_reserved_keywords.py --bnf-root sysml-v2-release/bnf`; use `--check` to verify
both the grammar-content pin and the inventory without rewriting it.

Syntax consumers can borrow `NameLanguage::reserved_keywords()` to use the same pinned vocabulary.
