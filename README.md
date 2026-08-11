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

Semantic references in the AST are opaque, document-local identities. Resolve them through
`ParsedDocument::qualified_reference` to borrow their authored segments, separator kinds, and
source spans without splitting or reparsing display strings.

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

The driver in `tools/snapshot_tool` manages qualified-reference snapshots under
`tests/snapshots/qualified_references`. They use five canonical Markdown sections: human-authored
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
emitted document in a `~~~sysml` fence. The driver recomputes this choice from `SOURCE`; it never
uses the sentinel as source text or as cached formatter output.

The driver delegates its AST section to the library's `ast::WriteSemanticAst` boundary, which
streams bytes to any `std::io::Write` destination (for example a file or `Vec<u8>`). Its exhaustive
enum matches make newly added AST variants a compile-time formatting decision.

```bash
cargo run --bin snapshot_tool -- check
cargo run --bin snapshot_tool -- update
cargo run --bin snapshot_tool -- check --fixture semantic_references.md
cargo run --bin snapshot_tool -- check --root path/to/snapshots
```

The driver processes fixtures sequentially in sorted path order. `check` fails on stale derived
sections; `update` rewrites them for normal `git diff` review.

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
