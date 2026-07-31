# Parser technical debt overview

This document describes structural duplication and architectural gaps in `sysml-v2-parser`. It complements:

- **[`PARSER_BACKLOG_ROADMAP.md`](./PARSER_BACKLOG_ROADMAP.md)** — **single backlog** for all open grammar-coverage work (start here for priorities)
- [`SYSML_V2_COMPLIANCE_GAP.md`](./SYSML_V2_COMPLIANCE_GAP.md) — what is implemented vs partial vs permissive
- [`BNF_COMPLIANCE_MATRIX.md`](./BNF_COMPLIANCE_MATRIX.md) — compact grammar-family snapshot

**This file is rationale and history, not a live tracker.** It explains *why* the codebase is shaped the way it is and records completed structural work for context. It intentionally does not carry a hand-maintained status table or priority list — those go stale (a markdown checkbox doesn't notice when it's wrong), and one already had: this doc didn't mention the `connection.rs`/`interface.rs` duplication below until it was found by reading the code, not by reading this file.

**Current, actively tracked tech-debt work lives in GitHub issues labeled [`tech-debt`](https://github.com/elan8/sysml-v2-parser/issues?q=is%3Aissue+label%3Atech-debt).** Open/closed issue state is self-accurate in a way prose never is. If you fix or find a duplication/architecture issue, file or close an issue — don't just edit a table in this file.

The parser currently passes `cargo test`, the full validation suite (`cargo test -- --include-ignored`), and strict library node-shape gates (`ExtendedLibraryDecl = 0`). Technical debt here is about **maintainability and grammar depth**, not about missing CI green.

## Current architecture (summary)

The codebase is in a **broad coverage, construct-specific modules** phase:

| Layer | Pattern |
|-------|---------|
| Top-level defs | ~25 `*_def` entry points (`item_def`, `connection_def`, `port_def`, …) |
| Package dispatch | Large ordered `if let Ok` chain in `package_body_element` (~50 branches) |
| Bodies | Per-family parsers; many still use `skip_until_brace_end` for inner content |
| Fallback | `KermlSemanticDecl`, `KermlFeatureDecl`, `ExtendedLibraryDecl` when no dedicated path matches |

That layout delivered green validation and drove `ExtendedLibraryDecl` to zero in library gates. The trade-off is **grammar unity** for **incremental delivery**.

A recent example: library declarations such as `abstract connection name : Type[multiplicity] nonunique :> redefines { ... }` require skipping a **typed header** before subclassification. When `parse_optional_definition_specialization` replaced `take_until_terminator` after `identification` without handling `: Type ... :>`, several defs failed and fell through to `ExtendedLibraryDecl`. The fix was `parse_optional_definition_header_after_identification` in [`src/parser/specialization.rs`](../src/parser/specialization.rs) — a small shared primitive, not a full rewrite.

## Where duplication appears

### 1. Definition prefix boilerplate

[`src/parser/definition_prefix.rs`](../src/parser/definition_prefix.rs) provides `parse_definition_prefix` with `DefinitionPrefixOptions` (`DefKeywordMode`, `VisibilityPrefix`, `AnnotationMode`, optional `second_keyword` for `use case`). Migrated `*_def` parsers: item, individual, interface, metadata, connection, constraint, port, requirement, state, occurrence, flow, allocation, case / analysis / verification, view / viewpoint / rendering, use case, enum, action.

**Still on local preludes (intentional):** `part_def` (usage disambiguation), `*_usage`, `alias_def`, `dependency`, `calc_def`, `attribute_def`.

**Growing escape-hatch pattern:** `DefinitionPrefixOptions` has picked up two narrow, one-off rejection options for def/usage disambiguation (`reject_header_keyword`, `reject_plain_typed_header_without_def`), each added to fix one specific reported bug rather than modeling the underlying question once. See [#34](https://github.com/elan8/sysml-v2-parser/issues/34).

### 2. Body terminators

[`src/parser/body.rs`](../src/parser/body.rs) exports `parse_structured_brace_members` and `semicolon_or_structured_definition_body`. Attribute, occurrence definition, rendering definition, flow, allocation, metadata, part/port, and connection def bodies now parse structured member nodes with recovery instead of opaque `skip_until_brace_end`/`advance_to_closing_brace` loops.

**Still local or opaque:** deep action/state/requirement body members beyond definition-level structured loops; alias/import paths; the *nested* `ref`/`connect`-statement bodies inside a connection def (`ref_body`, `connect_body` in [connection.rs](../src/parser/connection.rs)) still use `advance_to_closing_brace` — smaller in scope than the connection-def-body loop, since these are leaf statement bodies, not multi-member containers.

### 3. Package dispatch (large surface, mostly intentional)

[`package_body_element`](../src/parser/package.rs) is a long ordered dispatch chain, split into `try_package_body_*` sub-dispatchers grouped by keyword family. Much of its remaining size is **disambiguation policy** (e.g. `part_def` vs `part_usage`, `attribute_def` vs `attribute_usage`), not arbitrary repetition — see [#20](https://github.com/elan8/sysml-v2-parser/issues/20) for a recent example of a disambiguation bug in this dispatch (`connection name : Type { ... }` misclassified as a definition).

### 4. Recovery loops (medium duplication, high value if unified)

`recover_body_element` plus `build_recovery_error_node_from_span` loops appear in `part`, `action`, `state`, `requirement`, `constraint`, `view`, and others. The shape is always: try parse member → on failure recover and skip → push `Error` node → continue. `parse_structured_brace_members` in [`body.rs`](../src/parser/body.rs) is the shared entry point; not every family has migrated onto it yet.

### 5. AST shape duplication (structural, larger refactor)

Many `*Def` structs repeat `identification`, `specializes`, `specializes_span`, and `body`. This mirrors the compliance gap: the **shared KerML definition/usage layer** from the spec is not yet a single grammar layer in code. A larger refactor here (an internal `DefinitionDecl { keyword, prefixes, identification, header, body }` mapped to typed AST variants) should be driven by grammar work, not by deduplication alone.

**Concrete near-duplicate module pair:** `src/parser/connection.rs` and `src/parser/interface.rs` independently implement the same connector-end grammar — `end_decl`, `ref_body`, `ref_decl`, `connect_body`, the connection-end wrapper, `connect_ends`, and `connect_stmt` are each duplicated near-verbatim across the two files. This already cost real double work fixing [#19](https://github.com/elan8/sysml-v2-parser/issues/19) (had to fix the same `end_decl` bug in both files). Tracked in [#33](https://github.com/elan8/sysml-v2-parser/issues/33).

**Concrete triplicated helper:** `subsetting_relationship_node` (wrap a subsetting-family target in a `SubsettingRelationship` node) is independently implemented in `usage.rs` (the shared one), `attribute.rs`, and `part/body.rs`. Tracked in [#34](https://github.com/elan8/sysml-v2-parser/issues/34).

### 6. Shared usage grammar fragments

[`src/parser/usage.rs`](../src/parser/usage.rs) centralizes small `UsageDeclaration` / `FeatureSpecializationPart` fragments: multiplicity, `TypedBy` (`:` / `defined by` / `typed by`), subsetting, and redefinition. Most usage families (part, port, attribute, occurrence, requirement/case/analysis/verification, action/state, view/rendering/viewpoint/use-case, concern) route through this shared header parsing.

**Current AST caveat:** `attribute_usage` accepts extra specialization clauses for grammar coverage, but the existing public `AttributeUsage` AST only stores `typing` and `redefines`. `occurrence_usage` stores `type_name`, `subsets`, and `redefines`, using last-wins behavior for multiple clauses. Structured AST fidelity for `references` / `crosses` and richer body members remains a later tranche.

## What is not wasteful duplication

| Pattern | Why it stays |
|---------|----------------|
| Separate modules per SysML family (`part.rs`, `requirement.rs`, …) | Clear ownership, targeted tests, incremental BNF alignment |
| Per-fixture validation tests under `tests/validation/` | Catches regressions the aggregate suite might miss |
| `ExtendedLibraryDecl` as last resort | Safety net; library gates require count = 0 on the happy path |
| Ordered dispatch in `package_body_element` | Reflects real keyword disambiguation, not arbitrary repetition |

## Relationship to compliance gaps

From [`SYSML_V2_COMPLIANCE_GAP.md`](./SYSML_V2_COMPLIANCE_GAP.md):

1. **Generic definition/usage/specialization** — still distributed across construct-specific parsers instead of one unified layer (largest architectural gap).
2. **Permissive bodies** — `skip_until_brace_end` still appears in alias, import, connect-body fallbacks, and deep behavioral body parsers.
3. **Expression subset** — `expr.rs` is precedence-aware but not full `OwnedExpression`.
4. **Recovery / LSP** — solid baseline; more specific diagnostics and coverage still wanted (see the [`tech-debt`](https://github.com/elan8/sysml-v2-parser/issues?q=is%3Aissue+label%3Atech-debt) and general issue trackers for concrete open items).

Duplication in code and "partial grammar" in the spec sense overlap: the same missing shared header/body grammar shows up as copy-pasted parsers *and* as `ExtendedLibraryDecl` or opaque bodies when a shortcut fails.

## What to avoid

- **Monolithic "parser framework" rewrite** while validation and library gates are green — high risk of re-breaking `ExtendedLibraryDecl` and strict diagnostics tests.
- **Dedup-only refactors** without grammar tests — merging code paths without fixture coverage tends to hide regressions until the full library suite runs.
- **Removing fallback nodes prematurely** — keep `ExtendedLibraryDecl` at zero via dedicated parsers, not by deleting the fallback.

## Recommended workflow for refactors

1. Introduce a small shared primitive (like `parse_optional_definition_header_after_identification`).
2. Add or extend unit tests on the primitive and one representative family parser.
3. Migrate similar families in a single PR; run `cargo test -- --include-ignored`.
4. File or update a [`tech-debt`](https://github.com/elan8/sysml-v2-parser/issues?q=is%3Aissue+label%3Atech-debt)-labeled issue for any family that still uses opaque bodies or duplicated logic afterward — not a note in this file.

## Summary

| Question | Answer |
|----------|--------|
| Is there a lot of duplication? | **Yes** — especially definition prefixes, body terminators, recovery loops, and (concretely) `connection.rs`/`interface.rs`. |
| Is the codebase unmaintainable? | **No** — modules and tests are coherent; debt is known and issue-tracked. |
| Where's the current backlog? | Grammar-coverage work: [`PARSER_BACKLOG_ROADMAP.md`](./PARSER_BACKLOG_ROADMAP.md). Architecture/duplication work: issues labeled [`tech-debt`](https://github.com/elan8/sysml-v2-parser/issues?q=is%3Aissue+label%3Atech-debt). |
| Largest long-term gap? | **Unified definition/usage/specialization grammar** plus deeper body parsing, not more top-level `*_def` files. |
