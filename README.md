# sysml-v2-parser

SysML v2 textual notation parser for Rust.

This crate parses SysML v2 and related KerML textual syntax into an AST and also exposes a resilient editor-oriented parsing mode that returns partial AST + diagnostics.

## Changelog

Release notes and migration hints: [`CHANGELOG.md`](CHANGELOG.md).

## Current status

- library parser for a broad SysML v2 subset
- strict and resilient parsing entry points
- green unit/integration test suite
- green full validation and std-library gates when run with the SysML v2 release fixtures

## API

The main public entry points are:

- `parse(input)` for strict parsing
- `parse_for_editor(input)` for partial AST + diagnostics

Example:

```rust
use sysml_v2_parser::parse;

fn main() {
    let model = parse("package Demo;").expect("valid SysML");
    assert_eq!(model.elements.len(), 1);
}
```

## Development

Run the default test suite:

```bash
cargo test
```

Run formatting/lint checks used in CI:

```bash
cargo clippy -- -W clippy::all
```

Run the full validation suite against the SysML v2 release tree:

```bash
cargo test --test validation -- --include-ignored
```

Fetch the pinned SysML v2 release fixtures into `./sysml-v2-release`:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\fetch-sysml-v2-release.ps1
```

```bash
./scripts/fetch-sysml-v2-release.sh
```

If the release fixtures live somewhere else, set:

```bash
SYSML_V2_RELEASE_DIR=/path/to/SysML-v2-Release
```
