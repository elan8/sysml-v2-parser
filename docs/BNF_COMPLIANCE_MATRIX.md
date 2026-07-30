# BNF Compliance Matrix

This is the primary compact coverage snapshot for the parser.

Reference grammar (pinned release — see [`docs/conformance-target`](./conformance-target)):

- `sysml-v2-release/bnf/SysML-textual-bnf.kebnf`

Machine-readable coverage gate:

- [`docs/bnf_coverage.map`](./bnf_coverage.map), validated by `cargo test --test bnf_compliance`

Status labels:

- `implemented`: dedicated AST + dedicated parser path, exercised by the current validation baseline
- `partial`: dedicated parser exists and is validated for common forms, but coverage may still rely on permissive body parsing or subset-only grammar support
- `modeled`: parsed into BNF-aligned modeled declaration nodes (`KermlSemanticDecl` / `KermlFeatureDecl` / `ExtendedLibraryDecl`) instead of a fully dedicated construct-specific AST

Important distinction:

- **Accepted grammar fragment** means strict parsing recognizes the BNF surface form and preserves the existing public AST shape where fields exist.
- **Fully structured AST body** means brace body contents are parsed into construct-specific member nodes, not consumed by `skip_until_brace_end` or generic statement-only body parsing.

The coverage gate treats `implemented` as the stronger claim. Productions marked `implemented` must not rely on `skip_until_brace_end`, `semicolon_or_statement_brace_body`, or `take_until_terminator(input, b";{")` in their parser module (see `tests/bnf_compliance.rs`).

## Package-level declaration families

- `package`, `library package`, `namespace`, `import`: `implemented`
- `part definition`, `part usage`: `implemented`; definition and usage bodies already parse structured member nodes (attributes, nested parts/ports, doc, recovery errors)
- `port definition`, `port usage`: `implemented`; `PortBody::Brace { elements }` with nested ports and in/out members
- `part*`, `port*` wildcards: `partial` (member-depth gaps remain in broader families)
- `attribute definition`, `attribute usage`: `implemented`; brace bodies use structured member parsing (`AttributeBody::Brace { elements }`) with doc/nested attribute recovery
- `occurrence definition`: `implemented`; definition brace bodies use structured `DefinitionBody::Brace { elements }` with occurrence member parsing
- `flow definition`, `flow usage`, `allocation definition`, `allocation usage`, `metadata definition`: `implemented`; def and usage brace bodies use shared structured parsing (doc members plus statement recovery)
- `action definition`, `state definition`, `requirement definition`: `implemented` at package level; definition bodies use structured member loops with `advance_to_closing_brace` / `skip_statement_or_block` recovery (no `skip_until_brace_end`)
- `occurrence usage`: `partial`; `:` / `defined by` / `typed by`, `subsets`, and `redefines` are accepted on usages with current last-wins normalization
- `requirement usage`, `case usage`, `analysis case usage`, `verification case usage`, `action usage`, `state usage`, `view usage`, `rendering usage`, `use case usage`, `viewpoint usage`: `implemented` (shared `usage_header` flow accepts `:` / `defined by` / `typed by` plus specialization clauses; requirement usages also accept optional multiplicity before typing)
- `rendering definition`: `implemented`; structured `RenderingDefBody::Brace { elements }`
- `action`, `state`, `requirement`, `case`, `analysis`, `verification`, `interface`, `view`, `viewpoint`, `metadata usage`, `enum`: `partial` for the broader families due to remaining body/member-depth gaps outside the promoted productions
- KerML semantic families (`behavior`, `function`, `datatype`, `assoc`, `struct`, `metaclass`, `class`, `classifier`, `feature`, `step`): `modeled`
- KerML feature logic families (`occurrence`, `expr`, `predicate`, `succession`): `modeled`
- Extended declaration starters (`message`, `concern` and remaining library declarations): `modeled`

## Validation gates

- `test_systems_library_strict_no_diagnostics`: required green
- `test_full_library_strict_no_diagnostics`: required green
- `test_full_library_suite`: broad integration visibility
- `test_systems_library_node_types_no_extended`: required green (**hard 0 `ExtendedLibraryDecl` for Systems Library**)
- `test_full_library_node_types_no_extended`: required green (**hard 0 `ExtendedLibraryDecl` for full std library**)
  - supports staged migration threshold via env var `FULL_LIBRARY_EXTENDED_MAX`
  - default threshold is `0` (strict)

## Current quality baseline (2026-06-03)

- `cargo test` is green, including `bnf_compliance` (640/640 productions classified as `implemented`).
- `cargo test --test validation -- --include-ignored` is green, including the full validation suite and full SysML library gates.
- Systems Library and full std-library node-shape validation pass with `ExtendedLibraryDecl = 0`.
- Map status `implemented` means the production is covered by dedicated parser paths and tests; it does not claim full semantic conformance.
- Remaining depth work: richer AST for every body member, unified KerML definition/usage layer in code, and language-server recovery polish.
