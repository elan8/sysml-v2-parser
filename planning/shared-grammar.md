# Factor shared grammatical concepts

> **Status:** Phases 1-2 implemented; Phase 3 in progress; Phase 4 proposed

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

### Phase 1: one structural traversal boundary — done

Landed as `src/ast/visit`. The enduring rules now live with the code: the traversal contract and
the distinction between structural traversal and policy-complete consumers are documented in
`src/ast/visit/mod.rs` and `AGENTS.md`. What changed and why is in the changelog and commits.

Two constraints from that work carry forward into the phases below:

- shared families must not weaken the traversal's compile-time exhaustiveness, which is what lets
  a new production surface as a compile error rather than as silently missing behavior; and
- downstream semantic lowering deliberately keeps its own exhaustive matches. Representation
  changes are therefore felt as required edits there, not absorbed by the visitor.

### Phase 2: one body container — done

`ast::Body<E>` replaced 27 per-family body enums; the scopes keep their names as aliases, so the
member set stays typed per scope. `;` and `{}` remain distinct, and `braced_elements` reports the
semicolon form as `None` so consumers cannot flatten the two by accident.

Delimiters are retained: the semicolon form keeps its `;` span, the brace form keeps both brace
spans, captured where the shared brace-member routine consumes them. Deserialization checks that
each span still slices to its token and that a body is not inside out.

Two states that provenance forced into the open:

- `Absent` — no body was authored at all. A `#Name` prefix and an inferred action-usage terminator
  had been storing the semicolon form, so emission wrote a `;` nobody typed.
- `ActionBranchBody` — the brace-less `if x then y;` is not a one-member brace body, and is no
  longer re-emitted as one.

There is deliberately no state for a missing closing brace. An unterminated body does not produce a
body today: the enclosing declaration's parse fails and the scope above keeps the text as recovery.
A typed close outcome (`Authored | Recovered | Missing`) belongs with the recovery change that
makes such bodies representable, and adding it earlier would mean shipping unreachable variants.
That recovery change is worth doing: an unterminated body currently loses every member it
contained.

### Phase 3: Extract low-risk shared member families

Extract member categories that are already common grammar concepts and have compatible provenance
and recovery behavior. Start with the narrowest, highest-confidence families:

1. recovery members, while keeping malformed and unsupported as distinct variants; and
2. annotating members such as documentation, comments, textual representations, and applicable
   metadata forms.

#### Audit results

Both candidate families were audited across all 29 body scopes before extraction. The results
changed what is worth doing.

**Annotating members: grammar-backed, adopted where exact.** `AnnotatingElement = Comment |
Documentation | TextualRepresentation | MetadataFeature` is one production in both layers (KerML
8.2.3.3.1, SysML 8.2.2.4.1), reachable from every definition body through `DefinitionBodyItem ->
DefinitionMember -> DefinitionElement`. The AST accepted it unevenly -- documentation in 26 scopes,
metadata annotations in 13, textual representations in 6, comments in 4, in ten distinct
combinations. That spread is parser coverage, not language.

`AnnotatingMember` therefore exists, and relationship and `ref` bodies -- the scopes whose member
set is exactly the production -- use it. The others keep their own variants: adopting the family
where the parser cannot produce every member would make the type, and the deserialized contract,
claim coverage that does not exist. Closing each gap is now a variant swap plus dispatch instead of
four of each, which is the point of having the family.

**Recovery members: not adopted.** `Malformed` and `Unsupported` are parser states rather than
grammar productions, which the "deliberately defined syntax-layer concept" clause permits -- but
only 3 of 29 scopes carry both today, because the parser emits an unsupported node in exactly those
three. Wrapping the other 22 in a two-variant family would add a representable state the parser
cannot reach in those scopes, for a wrapper around what is already a single shared node type per
state. The traversal boundary from Phase 1 already gives recovery handling one owner. Revisit only
if unsupported-member support becomes general.

#### Follow-up work this audit surfaced

- Annotating-member coverage gaps: 22 scopes accept less than the grammar allows. Each is a
  conformance fix (parser dispatch plus the family variant), and each changes accepted language, so
  they are separate reviewed changes rather than part of a representation refactor.
- ~~`Other(String)` remains in 11 scopes~~ — done. Unrecognized content is a recovery node with an
  authored span and a report; a spec-valid member the scope does not model is an explicit
  `Unsupported` node with a warning. The two states exist only in the scopes that produce them, and
  `capture_opaque_member` is gone. Surfacing what it hid also closed two parse gaps.
- ~~`ref` body dispatch depends on its owner~~ — done. `UsageBody = DefinitionBody`, so there is
  now one `ref` body parser for all five owners and `RefBodyElement`, whose variants recorded which
  parser had run, is gone.
- A `ref` in a state body only dispatches when it is typed: `ref b { ... }` reaches recovery while
  `ref b : Anything { ... }` parses. It is now reported rather than silently captured, but the
  dispatch gap itself remains.
- Annotating coverage: the general usage-member scope now accepts the whole production. The
  remaining scopes are one commit each, with all four members plus a `#Name` prefix test.

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

A scope enum keeps a family out of scopes where the family does not belong, but it says nothing
about which *members* of that family the scope permits. If one scope allows documentation and
comments but not every metadata form, `Annotating(AnnotatingMember)` makes the type more
permissive than the grammar, and a public enum plus derived deserialization widens the accepted
AST contract even when the parser never constructs the illegal value. Resolve this when the family
is defined, by one of: splitting into narrower grammar-specific families; parameterizing the family
by scope; or making construction private and requiring deserialization validation that rejects
members illegal in their scope. Do not defer it to downstream validation.

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

The matrices and FIRST-set evidence below are a precondition for starting, not a deliverable
produced alongside the migration. Until they exist for a given family, that family is not ready to
factor, and each family is evaluated and migrated separately rather than as one broad sweep.

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

Generic containers and nested member families deepen the recursive type proofs the compiler has to
discharge, and those proofs are paid for by consumers, not by this crate. Each phase therefore also
gates on type-level cost, checked from an integration test -- which compiles as its own crate at the
*default* recursion limit, unlike the library, which raises it:

- `ParseResult` and `ParsedDocument` prove `Send` and `Sync` without the consumer raising
  `recursion_limit`;
- serde derivation and exhaustive matching still compile downstream;
- compile time for a representative consumer does not regress materially; and
- decoding and dropping a deeply nested document remains stack-safe.

Raising a consumer's recursion limit is a workaround, not a resolution: it hides which type cycle
grew and lets the next phase grow it further.

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
