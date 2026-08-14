# Factor shared grammatical concepts

> **Status:** Proposed

## Purpose

The AST currently represents many grammar scopes with separate body containers and body-element
enums. This preserves useful type distinctions, but it also repeats delimiter handling, traversal,
recovery variants, annotating members, and common declaration fields across otherwise related
types.

This plan reduces that repetition without replacing the typed AST with a flat, generically encoded
arena. The goal is to factor concepts that the pinned grammar itself shares while retaining:

- an atomic `ParsedDocument` that owns source, reference arenas, syntax, and diagnostics;
- scope-specific, exhaustive AST enums;
- exact authored spans and explicit malformed and unsupported states;
- typed qualified-reference identities and borrowed document views;
- streaming formatting from structured syntax; and
- compile failures when a new syntax variant needs a policy decision.

This is a representation and traversal refactor, not permission to widen accepted syntax, infer
structure from text, or merge distinct grammar productions because their Rust fields happen to
look alike.

## Problem statement

Types such as `PartDefBodyElement`, `PartUsageBodyElement`, `PortDefBodyElement`,
`PortBodyElement`, `ActionDefBodyElement`, `ActionUsageBodyElement`,
`RequirementDefBodyElement`, and `ConstraintDefBodyElement` overlap substantially. They commonly
repeat:

- the semicolon-versus-braced-body shape;
- ordered `Vec<Node<E>>` storage;
- documentation, comments, textual representations, and metadata;
- malformed and unsupported members;
- closely related structural or behavioral members; and
- parallel traversal in emitters, validators, serializers, snapshots, and test normalization.

The cost is not merely line count. Adding or changing one production can require several parallel
enum edits and several independently maintained walks. The large hand-written
`normalize_for_test_comparison` implementation is a visible symptom: a non-semantic operation must
reconstruct much of the AST because the repository has no single owning traversal boundary.

A universal `BodyElement` enum would reduce repetition but weaken the grammar model by allowing
members in scopes where they are not legal. A flat node arena with tag-dependent integer payloads
would reduce allocation and type count, but is outside this plan and would weaken the public typed
contract unless paired with a much larger view-layer redesign.

## Design principles

### Factor grammar, not coincidental shape

A shared type must correspond to an authoritative grammar production or a deliberately defined
syntax-layer concept. Similar field layouts alone are insufficient justification. Before extracting
a member family, record the relevant productions and the scopes in which they are valid.

### Keep scope-level exhaustiveness

Each body scope retains an exhaustive element enum. Shared families appear as explicit variants of
that enum rather than as a permissive universal member:

```rust
pub enum PartDefBodyElement {
    Structural(StructuralMember),
    Annotating(AnnotatingMember),
    Recovery(RecoveryMember),
    // Part-definition-specific alternatives remain explicit.
}
```

This ensures that adding a new shared family or scope-specific production still produces compile
failures at formatting, validation, serialization, visitation, and snapshot boundaries.

### Preserve authored distinctions

Factoring must not collapse declaration names into references, authored values into normalized
values, unsupported syntax into malformed syntax, or semicolon bodies into empty brace bodies.
Shared types retain exact source spans and all distinctions required for lossless recovery and
semantic snapshots.

### One representation after each migration

Each migrated concept has one AST representation. Do not retain old and new fields, add conversion
fallbacks, or introduce compatibility mirrors. A phase may be split into small commits or pull
requests, but each merged migration must update parser construction and all consumers atomically.

### Prefer typed owned nodes before considering arena storage

This plan operates within the current owned-tree architecture. Packed document arenas may be
evaluated separately using allocation benchmarks, but they are not required to remove repeated
grammar and traversal policy.

## Target shape

The intended result is a hierarchy of small shared concepts used by still-distinct public syntax
types:

```text
ParsedDocument
└── typed AST
    ├── Body<E>                         shared body delimiter/container shape
    ├── scope-specific element enums   compile-time grammar restrictions
    │   ├── shared grammatical families
    │   └── scope-specific alternatives
    ├── shared declaration components  only where grammar-backed
    └── one exhaustive visitor/folder boundary
```

An illustrative body container is:

```rust
pub enum Body<E> {
    Semicolon {
        semicolon_span: Span,
    },
    Brace {
        open_span: Span,
        elements: Vec<Node<E>>,
        close_span: Span,
    },
}
```

The exact fields must be derived from existing provenance requirements. In particular, the generic
container must not erase distinctions currently required to reproduce authored delimiters or
represent recovered/incomplete closing delimiters.

Likely shared member families include annotating members and recovery members. Broader structural
or behavioral families require a grammar audit before adoption. The names and boundaries in this
document are examples rather than pre-approved AST definitions.

## Refactor phases

### Phase 1: Establish one exhaustive traversal boundary

Introduce the owning visitor API used by all read-only AST consumers, plus a folder or equivalent
transformation API if transformations remain necessary. The APIs must enumerate syntax variants
exhaustively and must not contain `_` matches or generic fallback behavior.

Initial migration targets:

- semantic snapshot projection;
- serialized-document provenance and ID validation;
- emit/format traversal where the same structural walk is repeated; and
- test comparison normalization.

The immediate objective is to replace the hand-written tree reconstruction used solely to erase
spans during tests. Prefer changing comparison policy so spans can be handled at the owning
boundary; if a folder remains necessary, it must be generated from or share its variant inventory
with the visitor rather than becoming another parallel list.

Deliverables:

- one documented visitor/access API for every AST node reachable from `RootNamespace`;
- an exhaustive mechanism for transformations that genuinely need to rebuild nodes;
- migrated normalization, validation, and snapshot consumers; and
- compile-fail evidence or a focused test showing that an unhandled new variant cannot silently
  disappear.

Completion criteria:

- no independent whole-tree traversal remains for test normalization;
- consumers do not discover children through rendered text or parser helpers;
- malformed and unsupported nodes are visited at their original tree position; and
- existing semantic snapshots are unchanged unless the old projection was demonstrably wrong.

### Phase 2: Unify the body container

Introduce a generic body container for the grammar-common delimiter and ordered-element shape, then
migrate body aliases one family at a time. Element enums remain distinct during this phase.

Candidate migrations include definition, usage, relationship, and feature bodies that currently
encode the same semicolon/braces alternatives. Before migrating a type, compare:

- whether a semicolon is legal and what it means;
- whether an explicitly empty brace body differs from no body;
- open- and close-delimiter provenance;
- incomplete or recovered close delimiters;
- trivia and recovery adjacency requirements; and
- aggregate span conventions.

If two bodies differ on any of these facts, either parameterize the shared concept with an explicit
typed policy or leave them separate. Do not encode these differences using sentinel spans, empty
vectors, or undocumented booleans.

Deliverables:

- a single `Body<E>`-style type, or a small number of grammar-distinct body container types;
- shared borrowed iteration and delimiter access;
- shared streaming-format and snapshot handling for container structure; and
- removal of superseded per-family body containers.

Completion criteria:

- migrated scopes retain their original element enums and accepted language;
- semicolon, empty braces, populated braces, malformed members, and incomplete bodies have focused
  tests;
- parse/format/reparse behavior and recovery sibling preservation remain stable; and
- no temporary dual body representation remains.

### Phase 3: Extract low-risk shared member families

Extract member categories that are already common grammar concepts and have compatible provenance
and recovery behavior. Start with the narrowest, highest-confidence families:

1. recovery members, while keeping malformed and unsupported as distinct variants; and
2. annotating members such as documentation, comments, textual representations, and applicable
   metadata forms.

Scope-specific enums opt into these families explicitly:

```rust
pub enum RecoveryMember {
    Malformed(Node<ParseErrorNode>),
    Unsupported(Node<UnsupportedGrammarNode>),
}

pub enum SomeBodyElement {
    Annotating(AnnotatingMember),
    Recovery(RecoveryMember),
    Specific(SomeSpecificMember),
}
```

Do not place all metadata or annotations into one family without checking their different grammar
roles, ownership, and body forms. Likewise, a recovery wrapper must retain the stable diagnostic,
exact captured span, and authored malformed slice; it must not turn recovery into generic opaque
text.

Deliverables:

- a written grammar-to-family mapping for each extracted family;
- migrated body enums and parser construction sites;
- exhaustive visitor, formatter, validator, serde, and snapshot support; and
- deletion of superseded repeated variants and their duplicated handling.

Completion criteria:

- each scope accepts exactly the same member set as before, except for separately reviewed grammar
  fixes;
- recovery continues after malformed content without consuming valid later siblings;
- snapshots show shared members at their language-level role rather than exposing the Rust wrapper;
  and
- no `Other(String)` or opaque fallback is introduced to make the family fit.

### Phase 4: Audit and factor broader grammar families

Use the pinned grammar to evaluate broader shared concepts, particularly structural members,
behavioral members, declaration headers, and usage headers. This is deliberately last because
superficially similar declaration types often differ in legal prefixes, specialization clauses,
membership roles, or body productions.

For each proposed family:

1. list the authoritative productions and their FIRST sets;
2. create a matrix of allowed scopes and alternatives;
3. identify fields that are truly shared versus merely similarly named;
4. preserve distinct enums for mutually exclusive syntax shapes;
5. migrate one representative pair and validate the abstraction; and
6. migrate the remaining confirmed scopes atomically by concept.

Likely candidates include `DefinitionHeader`, `UsageHeader`, `StructuralMember`, and
`BehavioralMember`. These names do not imply that one header type must cover every definition or
usage. Prefer several precise components over a generic struct containing many irrelevant
`Option` fields or flags.

Deliverables:

- grammar matrices checked against the conformance pin;
- shared typed header/member components where the matrices justify them;
- removal of duplicated fields, parser construction, and consumer branches; and
- updated public documentation, changelog, AST/schema version, and snapshots for breaking shape
  changes.

Completion criteria:

- no migrated syntactic fact has both a shared and legacy representation;
- illegal scope/member combinations remain unrepresentable or are rejected during validated
  construction/deserialization;
- matches remain exhaustive at every policy boundary; and
- the resulting types contain no discriminator booleans or empty/sentinel values standing in for
  grammar alternatives.

## Verification strategy

Every phase uses focused tests while iterating and the complete repository gates before completion:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features`
- snapshot check through the repository snapshot driver
- strict/editor parse-entry-point equivalence
- serde round trips and corruption rejection when serialized shapes change
- parse/format/reparse coverage for affected body families
- recovery tests with malformed content before, between, and inside valid siblings

Snapshot updates must be explicit. A representation-only change should normally preserve semantic
`AST` and `FORMAT` output. If an output changes, document whether it corrects a lost grammatical
fact, provenance error, recovery error, or formatting policy rather than attributing it merely to
the refactor.

For phases that claim allocation or performance improvement, establish a reproducible baseline on
representative checked-in documents before implementation and retain the benchmark. Reduced source
line count alone is not performance evidence.

## Compatibility and sequencing

The AST is public, so some phases may require breaking API and serialized-shape changes. Minimize
the blast radius through concept-sized migrations, not through compatibility layers:

- update parser construction and every in-repository consumer together;
- increment the authoritative AST/schema version when the wire shape changes;
- update the changelog and public documentation in the same change;
- do not maintain old enum variants beside new family variants; and
- do not expose constructors that can combine a shared family with an illegal scope.

Phase 1 should land before representation changes so later phases have one place to update
traversal policy. Phase 2 is independent of the precise shared-member taxonomy. Phase 3 provides a
narrow trial of nested shared enums before Phase 4 addresses wider declaration families.

## Explicit non-goals

This plan does not:

- replace the AST with a flat node arena;
- create a universal body-element enum;
- combine parsing and semantic canonicalization;
- remove exact spans, recovery nodes, diagnostics, or source ownership;
- introduce string-based dispatch, formatting, validation, or comparison;
- widen opaque syntax to avoid modeling a production; or
- promise allocation improvements without measurements.

## Expected outcome

After all four phases, the AST should remain recognizably typed and grammar-directed, but adding a
new production should require decisions at one scope enum, one shared traversal boundary, and the
relevant semantic policies—not repeated mechanical edits across many near-identical body
containers and walks. The result should be simpler because shared grammatical facts are owned once,
not because distinctions or invariants have been removed.
