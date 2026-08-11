# AGENTS.md

These principles apply across the repository. More local instructions may add constraints for a
specific area, but must not weaken them.

They describe the intended long-term parser architecture. They are normative for new and modified
code even where the current implementation has not reached them yet. Existing violations are debt,
not precedent: do not copy or extend them, and do not disguise them with compatibility helpers.

## Own source, syntax, and meaning once

- A parsed document is the atomic unit: normalized source storage, syntax and recovery tree,
  diagnostics, and document-local arenas must describe the same input revision and travel together.
  Do not expose identity-bearing trees detached from the storage that resolves their identities.
- The original document is authoritative for authored spelling and provenance. Store spans into that
  source rather than copying token text into each node. Decode or normalize through one owning API,
  preserving the authored span alongside the result.
- The typed syntax tree is authoritative for grammatical structure. Do not rediscover separators,
  nesting, modifiers, wildcard shape, or relationship kind by splitting, joining, searching, or
  pattern-matching rendered text.
- Every syntactic fact has one representation. Remove superseded string, boolean, and display-field
  mirrors when introducing a typed form; synchronized representations inevitably drift.
- Keep declaration names, references, literals, keywords, and opaque source distinct even when their
  spelling happens to be identical. A declaration label is not a reference, and a reference must not
  be synthesized from a declaration label for emission or comparison.
- Preserve authored, decoded, normalized, inferred, and recovered facts as different states. Never
  overwrite provenance or make recovered content look like successfully recognized syntax.

## Source-backed identities and allocation discipline

- Represent semantic references with typed, domain-specific IDs into document-owned packed arenas.
  IDs are opaque, cheap to copy, and meaningful only with their owning document. Do not use raw
  integers or compare IDs from different documents as if they represented equal spelling.
- Intern and deduplicate repeated strings only at an explicit owning boundary. Use distinct ID types
  for distinct identity domains so unrelated indexes cannot be mixed accidentally.
- A qualified reference records its absolute/relative scope, ordered segment spans, and typed
  separators. `$` is scope metadata, not a fabricated name segment. `::` and `.` are syntax, not
  characters to reconstruct later.
- Prefer one source allocation plus packed metadata/segment vectors. Avoid a `String` or `Vec` per
  reference, token, or small node. Use borrowed views and iterators for access; do not reconstruct an
  owned path merely to inspect, emit, validate, or test it.
- Treat allocation as an architectural decision, not a cleanup detail. Before adding ownership,
  determine whether a source span, arena range, small enum, iterator, or borrowed view expresses the
  same fact more accurately.
- Parser speculation must not leak observable or unbounded arena entries. Allocate only after the
  syntax is accepted, or use checkpoint/rollback, deterministic deduplication, or a validated
  compaction phase with explicit identity remapping.
- Source spans use one documented coordinate system and source normalization policy. BOM handling,
  UTF-8 boundaries, line/column conventions, and aggregate-versus-token span rules must be consistent
  across parsing, diagnostics, slicing, serialization, and snapshots.

## Parsing and recovery are one contract

- Every input produces a coherent editor parse result, including empty or entirely malformed input.
  Unrecognized text becomes an explicit malformed/recovery node with an exact source span and stable
  diagnostic; it must not disappear, abort the remaining document, or masquerade as valid syntax.
- Strict parsing is a validation view over the same parser and document model, not a separate grammar.
  On diagnostic-free input, strict and editor entry points must produce equivalent documents,
  references, spans, and ordering.
- Recovery synchronizes on grammatical structure. Track balanced braces and other paired delimiters,
  respecting quoted text, escapes, and comments, then resume at the next structurally safe boundary.
  Never scan for a delimiter with a raw substring search when it may occur in nested or lexical
  content.
- Recovery must make forward progress without consuming valid later siblings. Test malformed content
  before, between, and inside valid constructs, unmatched delimiters, an entirely malformed document,
  and valid declarations following recovery.
- Preserve recovery content by source span. Formatting a recovered document streams the captured
  malformed slice at its tree position and continues with later typed siblings; it does not substitute
  the entire original document or invent valid syntax.
- Unsupported, malformed, ambiguous, and incomplete are explicit states. Do not use empty strings,
  sentinel IDs, omitted children, permissive fallbacks, or successful-looking defaults to encode
  failure.
- Diagnostics are part of the parser contract: use stable codes, precise spans, deterministic order,
  and explicit severity/category. Recovery behavior and its diagnostic must be tested together.

## Typed grammar and exhaustive evolution

- Model grammar alternatives with enums and structured fields. Booleans are appropriate only for
  genuinely independent binary properties, not as a compressed encoding of mutually exclusive
  syntax shapes.
- Matches over AST and syntax enums are exhaustive. Do not use `_`, silent omission, or generic
  fallback output in emitters, validators, serializers, normalizers, visitors, or snapshot formatters.
  Adding a variant must produce compile failures at every policy boundary that needs a decision.
- When several consumers traverse the same structure, provide one owning visitor or typed access API.
  Do not maintain parallel hand-written lists that can omit new variants independently.
- Parse from the original input and retain its offsets. Never extract text, construct a fresh parser
  input, and then claim its local offsets as document provenance.
- Parse once. Emitters, validators, serializers, and tests consume typed nodes and borrowed arena
  views; they never invoke parser helpers or infer structure from `Display`, debug output, source
  suffixes, or string containment.
- Opaque syntax is explicit and narrowly scoped. Do not widen an opaque string because a typed parser
  is inconvenient, and do not silently consume recognized semantic clauses without retaining their
  structure or a recovery node.

## Streaming boundaries

- Emitters and semantic formatters stream to a caller-provided `std::io::Write`. The sink may be a
  file, pipe, buffered writer, or memory arena; parser code must not require an intermediate document
  `String` or tree of formatted fragments.
- Formatting policy belongs beside the type system in a dedicated module or trait, not scattered
  through test helpers or reconstructed by downstream consumers. Keep transport and filesystem policy
  at the outer tool boundary.
- Render structured data from structured fields. Rendering may choose layout and escaping, but it may
  not become a second parser or semantic implementation.
- Error propagation is explicit. A dangling ID, invalid span, unsupported opaque node, or failed write
  is not replaced with placeholder text that could be mistaken for complete output.
- Deterministic output must not depend on hash-map iteration, parser speculation garbage, allocation
  addresses, thread scheduling, or incidental traversal order. Define and test canonical ordering at
  the owning layer.

## Serialization and public contracts

- Serialize identity-bearing syntax only as an atomic document envelope containing source, arenas,
  tree, schema version, and any normalization policy required to interpret spans.
- Deserialization is validation, not blind reconstruction. Reject version mismatches, dangling IDs,
  invalid or overflowing arena ranges, malformed separator invariants, out-of-source spans, invalid
  UTF-8 boundaries, and AST/arena shape inconsistencies.
- Document-local IDs are an implementation of identity, not portable spelling. Wire formats either
  preserve and validate the whole owning envelope or deliberately remap identities at the boundary.
- Breaking AST, arena, recovery, or serialized-shape changes update the authoritative version,
  changelog, public documentation, snapshot format, and affected consumers together.
- Public constructors must preserve invariants. If naked IDs cannot form a valid tree, provide a
  document/builder API or require parsing; do not expose constructors that permit dangling syntax.

## End-to-end semantic snapshots

- The repository snapshot driver is the primary end-to-end parser contract. Keep the driver under
  `tools/` and fixtures under `tests/snapshots/`; support deterministic `check` and explicit `update`
  workflows suitable for local development and CI.
- Parser snapshots contain only parser-owned sections: authored `SOURCE`, structured `DIAGNOSTICS`,
  deterministic `FORMAT`, and semantic `AST`. Do not copy downstream compiler sections whose facts
  the parser does not own.
- Snapshot AST output is a nested semantic S-expression. References appear at their language-level
  role, while their arena definitions expose scope, ordered segments, typed separators, and spans.
  Never encode Rust field access as path strings or stringify structured qualified names.
- Snapshot formatting streams through the same typed public boundary as other consumers and matches
  enums exhaustively. The harness must fail to compile when a new syntax variant lacks a deliberate
  representation.
- Generated sections are derived artifacts. Change `SOURCE` or the owning formatter, run the snapshot
  update tool, and review the complete diff. Check mode rejects stale output; tests must not silently
  rewrite fixtures.
- Snapshots decouple implementation layout from enduring semantics, but do not replace focused owning-
  layer tests. Arena range validation, separator parsing, recovery synchronization, and diagnostic
  spans still require narrow regression tests.

## Verification and change discipline

- Fix the owning abstraction rather than adding display-string compatibility layers. If a consumer
  lacks information, extend the typed syntax or document view that owns it.
- For a behavior change, test the narrow rule and its end-to-end projection. Include both sides where
  relevant: absolute/relative, `::`/`.`, quoted/unquoted, valid/malformed, strict/editor,
  serialize/deserialize, and parse/format/reparse.
- Allocation-sensitive work includes evidence appropriate to its claim: structural tests for borrowed
  views and packed ranges, plus benchmarks or profiles for performance assertions. Avoid proxy claims
  based only on code shape.
- Run focused checks while iterating, then formatting, all-feature compilation, snapshot check, serde
  validation, and the broader test suite appropriate to the change. Do not weaken assertions or skip
  tests merely to make a migration compile.
- Preserve unrelated work and keep migrations atomic. Temporary dual representations, fake IDs, span
  rebasing, and root-only compatibility wrappers are not acceptable intermediate states in a merged
  change.
- Review parser changes adversarially for copied token text, hidden per-node allocations, detached IDs,
  string reparsing, non-exhaustive matches, swallowed input, imprecise spans, recovery that consumes
  valid siblings, nondeterministic output, and snapshots coupled to Rust layout.
