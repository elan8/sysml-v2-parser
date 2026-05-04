# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.0] - 2026-05-04

### Breaking

- **`AttributeDef`**: added optional field `value: Option<Node<Expression>>` for default / value parts after `=`, `:=`, or `default =` on attribute definitions (e.g. `attribute n: Integer = 0;`). Update any exhaustive matches or struct literals that construct `AttributeDef`.
- **Expression `Span` for parenthesized grouping**: a single expression in parentheses `( expr )` now uses a node span covering the full `(` … `)` in the source (not only the inner expression). Tools that slice source by `Span` (e.g. joining `require constraint` text) may see different byte ranges than in 0.8.x for the same logical tree.
- **Numeric literal parsing**: `literal_only` tries `literal_real` before `literal_integer`, so decimals such as `0.9` parse as reals instead of integer `0` with a stray `.9`. Rare integer-vs-real edge cases in malformed or unusual inputs may produce a different AST than before.

### Fixed

- **Quantity literals**: bracket units such as `[m/s]` or library-style names with `::` inside `[` … `]` parse more reliably into `LiteralWithUnit`.
- **Constraint and calc brace bodies**: optional terminating `;` after each body item is accepted, so chained expressions split with `;` (e.g. `(a <= b); and (c <= d);`) map to multiple `Expression` elements instead of falling through to `Other`.
- **Recovery**: `inout` is included in constraint/calc body recovery keyword lists alongside `in` / `out`.

### Reliability

- Slightly longer preview text for `Other` placeholders in constraint/calc recovery paths (diagnostics).

### Migration (Spec42 and similar hosts)

1. Bump the `sysml-v2-parser` dependency to `0.9.0` (or the matching git revision).
2. Add `value: None` (or the parsed value) wherever you construct `AttributeDef` manually; re-run tests that assert on expression source spans inside parentheses or on joined constraint text.

**Local smoke (optional):** In a Spec42 checkout, add to `.cargo/config.toml` a `[patch."https://github.com/elan8/sysml-v2-parser"]` entry with `sysml-v2-parser = { path = "../sysml-v2-parser" }`, then run `cargo update -p sysml-v2-parser` and `cargo check -p kernel`. Remove the patch afterward unless you intend to keep developing against a local parser build.

[0.9.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.8.0...v0.9.0
