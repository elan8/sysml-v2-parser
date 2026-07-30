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
    let model = parse("package Demo;").expect("valid SysML");
    assert_eq!(model.elements.len(), 1);
}
```

- `parse(input)` — strict parse; returns `Result<RootNamespace, ParseError>`
- `parse_for_editor(input)` — partial AST + diagnostics for editors and language servers

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
