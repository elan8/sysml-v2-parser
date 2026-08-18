# SysML v2 Parser Status

> **Open work index:** see [`PARSER_BACKLOG_ROADMAP.md`](./PARSER_BACKLOG_ROADMAP.md) for the consolidated backlog and suggested execution order.

This document is the narrative status view of the parser as of April 9, 2026. It complements the compact snapshot in [`docs/BNF_COMPLIANCE_MATRIX.md`](C:\Git\sysml-v2-parser\docs\BNF_COMPLIANCE_MATRIX.md).

Reference grammar:

- [`sysml-v2-release/bnf/SysML-textual-bnf.kebnf`](C:\Git\sysml-v2-parser\sysml-v2-release\bnf\SysML-textual-bnf.kebnf)

## Scope

This is a parser-status document, not a semantic conformance claim.

The **pinned textual-notation target** and layered CI scorecard live in
[`docs/conformance-target`](./conformance-target) and [`docs/CONFORMANCE.md`](./CONFORMANCE.md).

It describes:

- which major textual grammar families currently have dedicated parser coverage
- where the parser still relies on permissive or shell-style parsing
- what remains unfinished even though the validation and library suites are green

It does not claim semantic conformance for:

- name resolution
- type checking
- cross-reference validation
- KerML/SysML well-formedness constraints
- derived or implicit abstract-syntax behavior

## Current baseline

The parser currently has a stronger baseline than the earlier gap analysis implied:

- `cargo test` is green
- `cargo test --test validation -- --include-ignored` is green
- the full validation suite is green
- the strict Systems Library and full-library syntax gates are green
- the Systems Library and full-library node-shape gates both pass with `ExtendedLibraryDecl = 0`

That means the project is no longer in a state where large parser families are simply absent from the implementation baseline. The remaining work is now mostly about depth, fidelity, and editor-grade resilience.

## What is implemented and validated

The parser has dedicated modules for the major SysML-facing construct families in [`src/parser`](C:\Git\sysml-v2-parser\src\parser):

- packages and imports
- aliases
- attributes
- enumerations
- items
- parts
- ports
- connections
- interfaces
- allocations
- flows
- actions
- states
- calculations and constraints
- requirements and concerns
- cases, analysis cases, and verification cases
- use cases
- views, viewpoints, and renderings
- metadata
- occurrences and individuals

This does not mean every production in those families is fully grammar-faithful, but it does mean these families are represented by real parser entry points and are covered by the current validation baseline.

## What is still partial or permissive

### Grammar-production-owned definition and usage components remain the largest architectural gap

The spec's shared definition/usage/specialization productions are still only partially modeled as
shared typed components. Much of that logic remains distributed across construct-specific parsers
such as parts, ports, attributes, actions, and states.

The intended “unified grammar layer” is compositional: one precise component per justified pinned
production, retained inside distinct family ASTs. It is not a universal definition/usage node,
generic optional-field bag, or monolithic parser rewrite. Migrations proceed through the matrix and
vertical-slice gates in [`planning/shared-grammar.md`](../planning/shared-grammar.md).

Impact:

- many legal combinations from the spec are still unavailable
- support is broader for common fixture forms than for the full grammar surface
- extending support consistently across families is harder than it should be

**Progress (2026-07-30 diagnostics audit):** bare `name : Type;` in part bodies is now first-class [`DefaultReferenceUsage`](../src/ast/structure.rs) (was mis-modeled as `AttributeUsage`). Root-level `DefinitionElement`/`UsageElement` members are accepted. Anonymous keyword usages (`part: T`, `actor : T`, `state: T`, …) parse. Remaining open: unified specialization grammar + completing `#`/`@` metadata coverage (see [`PARSER_BACKLOG_ROADMAP.md`](./PARSER_BACKLOG_ROADMAP.md)).

### Several families are parsed as useful subsets rather than full productions

The parser now covers flows, allocations, case families, and other formerly missing areas, but many of those families still operate as subset parsers rather than complete grammar implementations.

Common remaining limitations:

- subset-only body item coverage
- incomplete specialization/prefix combinations
- simplified handling of nested forms
- dedicated nodes for top-level declarations without equally deep body modeling

**Progress on usage prefixes:** `OccurrenceUsagePrefix` and the two productions it nests are now
one shared typed component, and `OccurrenceUsage` (with its `IndividualUsage`, `PortionUsage` and
`EventOccurrenceUsage` spellings), `ItemUsage` and `SatisfyRequirementUsage` carry the complete
prefix with each authored keyword's span. Every other usage family still carries whatever partial
prefix fields it had; which slots each is missing, and the continuation path, are in
[`planning/occurrence-usage-prefix-matrix.md`](../planning/occurrence-usage-prefix-matrix.md) §9.
"incomplete prefix combinations" above therefore no longer applies to those three families and
still applies to the rest.

The strongest areas today are still packages/imports, parts, requirements, and the general library-validation path.

### Some bodies are still accepted permissively

`skip_until_brace_end()` in [`src/parser/lex.rs`](src/parser/lex.rs) remains for lex-level helpers and the legacy `semicolon_or_opaque_brace_body` stub in [`src/parser/body.rs`](src/parser/body.rs). Most construct modules now use `advance_to_closing_brace` or `parse_structured_brace_members` for brace bodies.

Remaining opaque or partial body consumption:

- [`src/parser/body.rs`](src/parser/body.rs) — structured-loop escape uses `advance_to_closing_brace` (P3); legacy `semicolon_or_opaque_brace_body` still uses `skip_until_brace_end`
- [`src/parser/lex.rs`](src/parser/lex.rs) — `skip_until_brace_end` definition and internal callers
- alias, import, interface, connection, enumeration, use case, constraint, view, part — mostly `advance_to_closing_brace` for unmodeled inner regions, not full member grammars

Impact:

- broad fixture compatibility is preserved
- top-level declaration coverage is stronger than deep body fidelity
- body-level AST precision is still the main technical debt

### Expression support is useful but still incomplete

[`src/parser/expr.rs`](C:\Git\sysml-v2-parser\src\parser\expr.rs) now has precedence-aware parsing for its supported operators, so the earlier flat-chain description is outdated. Even so, expression coverage is still only a subset of full SysML/KerML `OwnedExpression`.

Impact:

- common arithmetic, logical, member-access, and simple literal forms work
- broader expression-family coverage is still needed for full spec fidelity across constraints, calculations, guards, filters, and value parts

## Modeled fallback nodes

Package-level fallback handling is no longer just "recover and emit diagnostics." The parser now models several declaration families directly in the AST:

- `KermlSemanticDecl`
- `KermlFeatureDecl`
- `ExtendedLibraryDecl`

These nodes are defined in [`src/ast.rs`](C:\Git\sysml-v2-parser\src\ast.rs) and produced by package-level parsing in [`src/parser/package.rs`](C:\Git\sysml-v2-parser\src\parser\package.rs).

Current status:

- `KermlSemanticDecl` and `KermlFeatureDecl` are intentional modeled families for broader library coverage
- `ExtendedLibraryDecl` has been driven down to `0` in the current Systems Library and full std-library node-shape gates

This means package-level fallback elimination is no longer the main quality problem. The remaining issue is how deeply the parser models bodies after a declaration has already been recognized.

## What "not finished" means now

The parser is not blocked on missing top-level support for major SysML families anymore. "Not finished" now means:

1. shared definition/usage/specialization productions are not yet consistently represented by
   grammar-owned components across all families
2. body-level modeling is still permissive in several modules
3. action/state/expression coverage is still subset-oriented compared with the full grammar
4. language-server recovery quality still needs hardening beyond the current solid baseline
5. semantic conformance remains largely out of scope

For a maintainability-focused view (duplication, refactors, priorities), see [`PARSER_TECHNICAL_DEBT.md`](./PARSER_TECHNICAL_DEBT.md).

## Recommended next work

BNF coverage map status is now 100% `implemented` for all 640 textual productions (see `cargo test --test bnf_compliance`). Prioritized open items are tracked in [`PARSER_BACKLOG_ROADMAP.md`](./PARSER_BACKLOG_ROADMAP.md) (Spec42 follow-up, body fidelity, expression depth, LSP hardening, P5 grammar layer).

## Summary

The parser now covers a broad, validated SysML v2 subset and passes the repo's strict library and validation gates. The main story is no longer "large language families are missing." The main story is that the parser has broad declaration coverage, but still needs deeper body parsing, a better shared grammar foundation, and stronger editor-oriented recovery to approach full grammar fidelity.
