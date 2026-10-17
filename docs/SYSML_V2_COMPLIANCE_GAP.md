# SysML v2 parser status and remaining limits

Current for **v0.58.0**, AST schema **284**. Use [CONFORMANCE.md](CONFORMANCE.md) for the
precise claim layers and [the migration guide](MIGRATION_0_58.md) for consumer changes.

## Verified baseline

The pinned target is the OMG `2026-04` release, identified by the release, production counts and
grammar hash in [conformance-target](conformance-target). The BNF map classifies all 640 textual
productions as implemented. The strict library gates cover 21 Systems Library files and 94
standard-library files, requiring zero diagnostics and zero unstructured nodes. All 56 pinned
validation models round-trip; 85 of 95 broader examples are tracked round-trip passes.

These are explicit corpus and coverage-map results. They do not establish acceptance of every
legal program or semantic conformance. The [differential oracle](PARSER-ORACLE.md) adds evidence
from an independent parser and an invalid-model corpus; its disagreements require grammar review.

## What changed in v0.58.0

Permissive declaration and body-member fallbacks have been removed. The old `ExtendedLibraryDecl`,
`FeatureDecl`, `ClassifierDecl`, `KermlSemanticDecl` and `KermlFeatureDecl` variants no longer exist.
A failed declaration is retained as malformed recovery, rather than accepted because its first
keyword was recognized. Supported KerML forms now use typed declarations.

Extended definitions and named usages reach their legal body owners. Action and calculation
bodies retain case-family usages and nested definitions; case/use-case/state bodies retain typed
variant membership. State entry/do/exit declarations and values keep their complete syntax, and
subactions use the action-body grammar. Migrated typing fields preserve every authored target.
Short names and anonymous identification use their grammar-owned slots. Reserved-name validation
uses the selected SysML or KerML vocabulary.

## Remaining work

- **Grammar combinations and field retention:** a coverage-map classification does not guarantee
  every nesting, prefix and specialization combination. New findings need source-based regression
  snapshots and fixes in the owning production.
- **Shared grammar components:** definition/usage/specialization ownership remains distributed
  across construct-specific ASTs. Continue justified, family-sized vertical migrations; do not
  replace them with a universal node or compatibility mirrors.
- **Formatting breadth:** the ten broader example gaps and future corpus findings still need
  triage. Keep strict parse, opacity, emission, reparse and AST-fidelity failures distinct.
- **Recovery quality:** malformed and incomplete editor input still needs precise local spans,
  stable diagnostic codes and evidence that later siblings survive. See [ERROR_RECOVERY.md](ERROR_RECOVERY.md).
- **Semantic rules:** name resolution, typing, cross-reference validity, well-formedness and
  derived/implicit abstract syntax remain outside this parser's conformance claim.

Use [GitHub issues](https://github.com/elan8/sysml-v2-parser/issues) for live open/closed status.
The [roadmap](PARSER_BACKLOG_ROADMAP.md) and [technical-debt notes](PARSER_TECHNICAL_DEBT.md)
also contain dated design history; they must not override the current grammar pin or CI evidence.
