# sysml-v2-parser

[![crates.io](https://img.shields.io/crates/v/sysml-v2-parser.svg)](https://crates.io/crates/sysml-v2-parser)

A SysML v2 and KerML textual notation parser for Rust. It has a strict mode for validation and a
resilient mode for editors, which returns a partial tree with diagnostics for any input.

```toml
[dependencies]
sysml-v2-parser = "0.59"
# Add `features = ["serde"]` to serialize complete documents.
```

Requires Rust 1.88. Upgrading? See the [changelog](CHANGELOG.md) and the
[v0.59.0 migration guide](docs/MIGRATION_0_59.md).

## Usage

```rust
use sysml_v2_parser::{parse, parse_for_editor};

// Strict: rejects anything the grammar does not allow.
let document = parse("package Demo;").expect("valid SysML");
assert_eq!(document.root.elements.len(), 1);

// Editor: never fails, and keeps what it can of an unfinished document.
let result = parse_for_editor("package Demo { part def Motor;");
assert!(!result.is_ok());
assert!(!result.errors.is_empty());
```

| Entry point | Use it for |
| --- | --- |
| `parse` | Validation and anywhere invalid text must be rejected. |
| `parse_for_editor` | Editors and language servers. Returns a partial `ParsedDocument` plus diagnostics. |
| `parse_with_name_language` | KerML sources, which reserve different keywords. Pass `ast::NameLanguage::Kerml`. |
| `emit_sysml` | Formatting a fully recognized document. Refuses recovered content. |
| `emit_recovered_sysml` | Formatting an editor document, reprinting malformed text in place. |

A `ParsedDocument` keeps the source, reference arena and tree together. References are
document-local IDs: resolve them with `ParsedDocument::qualified_reference`. With the `serde`
feature, the document serializes as one envelope carrying `PARSE_AST_VERSION`, and deserializing
validates it and rejects a version mismatch.

## Conformance

The grammar is pinned in [`docs/conformance-target`](docs/conformance-target) and exposed as
`SUPPORTED_GRAMMAR`. CI checks the parser against the release corpus and against an independent
parser. Known gaps are listed in [`docs/SYSML_V2_COMPLIANCE_GAP.md`](docs/SYSML_V2_COMPLIANCE_GAP.md).

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo snapshot check
```

See [`docs/DEVELOPMENT.md`](docs/DEVELOPMENT.md) for the full validation suite, snapshots and
benchmarks.

## Documentation

| Topic | Doc |
| --- | --- |
| Migration | [v0.59.0](docs/MIGRATION_0_59.md), [v0.58.0](docs/MIGRATION_0_58.md) |
| Development | [`docs/DEVELOPMENT.md`](docs/DEVELOPMENT.md) |
| Release procedure | [`docs/RELEASING.md`](docs/RELEASING.md) |
| Error recovery | [`docs/ERROR_RECOVERY.md`](docs/ERROR_RECOVERY.md) |
| Conformance | [`docs/CONFORMANCE.md`](docs/CONFORMANCE.md), [`docs/BNF_COVERAGE.md`](docs/BNF_COVERAGE.md), [`docs/PARSER-ORACLE.md`](docs/PARSER-ORACLE.md) |
| Backlog | [`docs/PARSER_BACKLOG_ROADMAP.md`](docs/PARSER_BACKLOG_ROADMAP.md), [`docs/PARSER_TECHNICAL_DEBT.md`](docs/PARSER_TECHNICAL_DEBT.md) |

## License

MIT. See [LICENSE](LICENSE).
