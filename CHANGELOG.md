# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.46.0] - 2026-07-23

### Added

- **`PartDefBodyElement::ActionUsage` / `StateUsage`** and matching
  `PartUsageBodyElement::ActionUsage` — Systems Library forms such as
  `abstract ref action performedActions: Action[0..*] :> actions, enactedPerformances`
  and `ref state …` now parse as real usage AST instead of
  `OpaqueMember` / recovery.
- **`ActionUsage` / `StateUsage` reference and specialization fields:**
  `is_abstract`, `is_reference`, structured `typing`, `multiplicity`,
  `subsets`, and `redefines`. Feature-style headers preserve `[0..*]` and
  multi-target `:>` clauses.
- **`PortUsage::is_abstract`** and acceptance of a leading `abstract`
  keyword (e.g. `abstract port ownedPorts: Port[0..*] :> …`).
- **`RefDecl::membership`** with visibility capture on
  `part_ref_usage` (`private ref …`). Kinded refs (`ref action` /
  `ref state` / …) are rejected by `part_ref_usage` so dedicated parsers
  own them.
- **Implicit empty `ActionUsage` bodies** when the next token starts a
  sibling statement/succession (Systems Library `LoopAction` style
  without braces).

### Changed

- **`PARSE_AST_VERSION`** bumped `42` → `43`.
- Opaque part-def catch-all no longer claims `action` / `state` / bare
  `port`; those go through dedicated parsers.

## [0.45.1] - 2026-07-23

### Fixed

- Bounded parser stack consumption for adversarially deep models. Package-body dispatch now
  heap-allocates its large transient `Node<PackageBodyElement>` result without changing the public
  AST or serialized shape, and inputs deeper than 32 structural brace scopes are rejected with
  `nesting_too_deep` before recursive descent. Flat models remain unrestricted by this guard; a
  10,000-member flat package is covered by regression testing.

## [0.45.0] - 2026-07-20

Closes the S42-004 "multi-target typing" gap flagged in Babel42's Systems Modeling API gaps
document, plus a more severe bug found while verifying it against real usage: a `ref` declaration
combining `:>>` redefinition with a `:` typing clause (e.g. Systems Library `Actions.sysml`'s `ref
sentMessage :>> sentTransfer: MessageTransfer, MessageAction { ... }`, used in both `SendAction`
and `AcceptMessageAction`) previously silently discarded the redefines target *and* the entire
typing clause as unparsed text once `:>>` was seen — worse than the "only the first type survives"
framing in the gaps doc, since neither type nor the redefines target survived at all. Confirmed via
direct parse probes, not just static reading: `PartUsage`/`ref` declarations already parsed a
comma-separated multi-target clause structurally in most call sites (`typings`/`optional_typings`
already returned every target), but every consumer collapsed it to a single joined `type_name:
String` before it ever reached the AST — the fix reuses the same `TypingRelationship`/
`typing_node` machinery `AttributeUsage`/`AttributeDef` already ship, rather than inventing a new
mechanism.

Two related gaps found during the real-usage audit are deliberately left open, out of scope for
this release: `ref` inside a `part def`/`port def` body has no visibility/direction-prefix support
at all, and `ref <kind> name : Type[mult] :> subsets` forms (`ref action`, `ref state`, `ref port`,
etc., real usage confirmed in the Systems Library, e.g. `Parts.sysml`'s `abstract ref action
performedActions: Action[0..*] :> actions, enactedPerformances`) fall through to an opaque,
inert `Other(...)` catch-all rather than a real AST node. Both are real, but distinct bug classes
from multi-target typing, and the second is comparable in size to the P5+ "unified definition/
usage/specialization grammar layer" item this repo's own backlog already flags as "do not
big-bang rewrite."

### Added

- **`PartUsage::typing`, `RefDecl::typing`** (`Option<Node<TypingRelationship>>`): structured,
  multi-target-capable typing clause alongside the existing joined-display-string `type_name`,
  mirroring `AttributeUsage.typing`/`AttributeDef.typing`. Populated in every `PartUsage`
  constructor (`part_usage_named`, `anonymous_part_usage`) from the same `typings`/
  `optional_typings` result already computed for `type_name` — no new grammar. `RefDecl`'s ad hoc
  single-target call sites (`connection.rs`, `interface.rs`, `part_ref_usage`, `state.rs`) wrap
  their existing single `qualified_name` result into the same field via a new
  `usage::single_target_typing` helper, for API consistency, with no parsing-behavior change.
- **`RefDecl::redefines`** (`Option<Node<SubsettingRelationship>>`): the `:>>` target, via a new
  `usage::single_target_redefines` helper. Only `action_ref_decl` (action/action-usage bodies)
  actually parses this clause today — the other `ref`-declaration sites have no confirmed real
  `:>>` usage and are left as `None`, unchanged from before.
- `usage::typing_fields_from_result`: shared helper turning a `typings`/`optional_typings` result
  into the `(type_ref_span, type_name, typing)` triple, factoring out the pattern
  `part_usage_named`/`anonymous_part_usage`/`action_ref_decl` each need.

### Fixed

- **`action_ref_decl`** (`ref` inside action def/usage bodies): now parses an optional `:>>`
  redefines clause followed by an optional `:` typing clause (multi-target aware) as two separate,
  sequential clauses, instead of one `:>>`/`:>`/`:` token deciding a single all-or-nothing branch.
  Previously, `:>>` matched but never consumed a target, and everything up to the body/terminator
  — including a following `: Type1, Type2` clause — was silently skipped as opaque text via
  `take_until_terminator`. This is live in the gated Systems Library today (`Actions.sysml`'s
  `SendAction.sentMessage`/`AcceptMessageAction.acceptedMessage`), confirmed via
  `cargo test --test validation -- --include-ignored` (`full_library_suite::
  test_full_library_strict_no_diagnostics` and the rest of the 25-test gate still pass — the bug
  produced no diagnostic before or after, only silent data loss, so the strict gate alone would
  not have caught either the bug or the fix).

### Changed

- **`PARSE_AST_VERSION`** bumped `41` → `42`: `PartUsage` and `RefDecl` each gained new fields.
- Regenerated AST snapshot fixtures (`UPDATE_VALIDATION_AST=1 cargo test --test validation --
  --include-ignored`) and updated the hand-authored `parts_interconnection_2a.rs` expected-AST
  fixture for the new `PartUsage`/`RefDecl` fields.

## [0.44.0] - 2026-07-20

Closes the `Intersecting` gap flagged in Babel42's Systems Modeling API gaps document
(S42-002/S42-008, 2026-07-19 audit) as a "confirmed genuine parser gap": `intersects
<target>` (KerML `Intersecting`, e.g. `attribute reading : Weight intersects a, b;`) was
grammatically recognized (`usage::skip_intersects_clause`) but the parsed targets were
discarded entirely via `opt(...)` with no captured value — the same bug class already fixed
for `references`/`crosses` (`ReferenceSubsetting`/`CrossSubsetting`). The same audit's other
flagged items were investigated and found out of scope for this repo: `TypeFeaturing`,
`FeatureInverting`, `Unioning`, `Disjoining`, `Differencing`, and `FeatureChaining`-as-a-
relationship-metaclass have zero real usage in the SysML systems library or examples
(`sysml-v2-release/`) and remain backlog, not a release blocker. General (non-port)
Conjugation, also flagged, turned out not to be a parser gap at all: the `~` conjugated-typing
prefix is already parsed generically for every usage kind via `usage::conjugated_qualified_name`
— nothing about it is port-specific in this parser — so that item requires only a Spec42-side
fix (its graph builder only materializes the implicit conjugate/`PortConjugation` edge for
`PortDefinition` today), not a `sysml-v2-parser` change.

### Added

- **`SubsettingKind::Intersects`** (`src/ast/core.rs`) and `parser::usage::intersecting()`,
  mirroring `cross_subsetting`. `intersects` is now a normal `SpecializationClause` alternative
  parsed by `specialization_clauses()` instead of being tokenized and dropped by a separate
  `skip_intersects_clause` (removed).
- **`AttributeUsage::intersects`, `PortUsage::intersects`, `OccurrenceUsage::intersects`**
  (`src/ast/structure.rs`): `Option<Node<SubsettingRelationship>>`, matching the existing
  `subsets`/`redefines`/`references`/`crosses` field set on these three usage kinds — the only
  ones that already carried the full subsetting-family field set before this change.
- Parser tests covering `intersects` in `specialization_clauses()` (single target, multi-target
  comma-separated, and mixed with `subsets`/`crosses` in one clause) and end-to-end coverage on
  `AttributeUsage`, `PortUsage`, and `OccurrenceUsage`.

### Changed

- **`PARSE_AST_VERSION`** bumped `40` → `41`: `AttributeUsage`, `PortUsage`, and `OccurrenceUsage`
  each gained a new field.

## [0.43.0] - 2026-07-19

Closes a flow-payload parsing gap found while auditing Babel42's flow-payload-resolution work
(Spec42 0.44.11) against the OMG SysML v2.0 spec text: `succession flow dataFlow of Payload from
a to b;`'s `of X` clause was parsed by the generic `expression` combinator, which only handles a
bare type reference. Per §8.2.2.16, `of X` denotes a `PayloadFeature` — an optionally-named
`Feature`, typed and/or given a multiplicity — not a plain expression. Confirmed by direct
testing: `of qty : Payload` (a named payload feature, no multiplicity) and `of qty :
Payload[1..3]` (named + multiplicity) both previously failed to parse entirely, dropping the
whole flow statement into an `Error` recovery node.

### Added

- **`ast::PayloadFeature`** (`src/ast/behavior.rs`): `{ name: Option<String>, type_name:
  Option<String>, multiplicity: Option<Node<Multiplicity>> }`, mirroring `ItemUsage`'s shape with
  the name relaxed to optional (a payload feature's `Identification` is itself optional per
  spec). `parser::flow::payload_feature` disambiguates the named form (`name : Type`) from a bare
  type reference by trying the named form first and only committing to it if a typing clause
  genuinely follows the identifier; multiplicity is accepted after either form (leading
  multiplicity, the grammar's third alternative, is out of scope — no observed real-world usage).
  `FlowUsage::payload` changes from `Option<Node<Expression>>` to `Option<Node<PayloadFeature>>`.
- Parser tests in `tests/parser/flow_usage.rs` covering all four payload forms: bare type
  reference, named (no multiplicity), named with multiplicity, and bare qualified type reference,
  plus a regression test confirming a flow with no `of` clause still has `payload: None`.

### Changed

- **`PARSE_AST_VERSION`** bumped `39` → `40`: `FlowUsage::payload`'s type changed.

## [0.42.0] - 2026-07-18

Closes the `expose` classification gap flagged in Babel42's Systems Modeling API gaps document:
`expose_member` already distinguished the four suffix forms (`::*::**` / `::**` / `::*` / plain)
to build the concatenated `target` string, but discarded which one matched instead of retaining
it structurally — `expose` is normatively an Import per its own BNF doc comment
(`MembershipImport = QualifiedName (::**)?`, `NamespaceImport = QualifiedName :: * (::**)?`), the
same distinction `ast::Import` already makes for ordinary `import` statements.

### Added

- **`ast::ExposeMember::is_import_all`/`::is_recursive`** (`src/ast/view.rs`), mirroring
  `Import`'s fields of the same name. `parser::view::expose_member` now records which of the four
  suffix branches matched instead of only concatenating `target`.
- Parser tests in `src/parser/view.rs`'s `expose_diagnostic_tests` module covering all four forms:
  plain (`expose vehicle;`), `::*`, `::**`, and `::*::**`.

### Changed

- **`PARSE_AST_VERSION`** bumped `38` → `39`: `ExposeMember` gained two new fields, so cached
  parses built against 0.41.x schema must be invalidated.

## [0.41.0] - 2026-07-18

Closes the `ConcernDefinition`/`ConcernUsage` gap flagged in Babel42's Systems Modeling API gaps
document: `concern_usage` already parses both the `concern` and `concern def` textual forms, but
discarded which one was used, so downstream consumers could not distinguish a concern definition
from a concern usage the way they already can for `case`/`case def`.

### Added

- **`ast::ConcernUsage::is_definition`** (`src/ast/requirement.rs`): true when the optional `def`
  keyword was present. `parser::requirement::concern_usage` now records the match instead of
  discarding it via `nom::combinator::opt(...)`.
- Parser tests in `src/parser/requirement.rs`'s `membership_tests` module covering the bare
  `concern c1 : ConcernType;` form (`is_definition: false`) and the `concern def ...` form,
  including with a typing target and a body (`is_definition: true`).

### Changed

- **`PARSE_AST_VERSION`** bumped `37` → `38`: `ConcernUsage` gained a new field, so cached parses
  built against 0.40.x schema must be invalidated.

## [0.40.0] - 2026-07-17

Closes the `constraint`/`ConstraintUsage` gap flagged in Babel42's Systems Modeling API gaps
document: `constraint c : C;` had no distinct usage-side AST node and folded into `ConstraintDef`
at parse time.

### Added

- **`ast::ConstraintUsage`** (`src/ast/view.rs`) and its parser, `constraint_usage`
  (`src/parser/constraint.rs`), built on `parser::usage::feature_usage_header` — the same shared
  header parser `allocation_usage`/`flow_usage`/`requirement_usage`/etc. already use for
  typing/subsetting/multiplicity/`nonunique`. New `PackageBodyElement::ConstraintUsage` variant,
  dispatched right after `constraint_def` in `package.rs`'s package-level `alt(...)` (mirroring
  `case_def`/`case_usage`).
- Regression tests in `src/parser/constraint.rs`'s new `constraint_usage_tests` module locking in
  every real `Systems Library`/example-corpus bare-`constraint` shape: simple typed
  (`constraint c : C;`), typed-and-braced, untyped-and-braced, the `constraintChecks` shape
  (`abstract` + typing + trailing multiplicity + `nonunique` + subsetting, from
  `Systems Library/Constraints.sysml`), subsetting-only with multiple targets
  (`assertedConstraintChecks`), leading-multiplicity-then-subsetting with no typing
  (`Requirements.sysml`'s `assumptions[0..*] :> constraintChecks, subperformances`), and
  redefinition with a qualified feature-chain target (`assumptions :>>
  RequirementConstraintCheck::assumptions`).

### Changed

- **`constraint_def` now requires the `def` keyword** (`.def_required()`), the PAR-001
  disambiguation pattern, now that `constraint_usage` exists to catch the bare form instead of
  silently misclassifying it as a definition.
- **`PARSE_AST_VERSION`** bumped `36` → `37`: `constraint`/`ConstraintUsage` classification
  changed, so cached parses built against 0.39.x schema must be invalidated.

### Notes

- **This is not the same change CHANGELOG 0.33.0 reverted.** That earlier attempt made
  `constraint_def` (and `calc_def`/`port_def`) require `def` with **no usage-parser fallback at
  all**, so the standard library's bare, `def`-less namespace-level forms became unparseable —
  breaking the full `SYSML_V2_RELEASE_DIR` validation gate. This change adds the missing
  `constraint_usage` parser first, verified to cover every real bare-`constraint` shape in the
  release corpus (see the regression tests above), before requiring `def` on the definition side.
  `calc_def`/`port_def` remain deliberately `Optional` — this change does not touch them.
- Verified against the full SysML v2 release validation suite (`cargo test --test validation --
  --include-ignored` with `SYSML_V2_RELEASE_DIR` pointing at the fetched
  `Systems-Modeling/SysML-v2-Release` checkout): all 25 tests pass, including
  `full_library_suite::test_full_library_suite`,
  `full_library_suite::test_full_library_strict_no_diagnostics`,
  `full_library_suite::test_systems_library_strict_no_diagnostics`, and
  `full_validation_suite::test_full_validation_suite` — the exact gates the 0.33.0 attempt broke.

## [0.39.0] - 2026-07-16

- **Enumerated values are now spanned AST nodes** — `EnumerationBody::Brace { values }` changed
  from `Vec<String>` to `Vec<Node<EnumeratedValue>>` (new `EnumeratedValue { name: String }`
  struct). `enumerated_value()` (`src/parser/enumeration.rs`) now returns a `Node<EnumeratedValue>`
  whose span covers the value's name, so each enumerated value inside `enum def { ... }` can
  become an addressable Spec42/Babel42 element instead of a bare display string. The optional
  inline `{ ... }` body and `= expr` initializer remain discarded — only name + span are
  retained, matching the existing `enumerated_value` production's scope. Breaking AST change;
  minor bump.

## [0.38.0] - 2026-07-16

- **PAR-007: `connection`/`interface` usage misclassified when typed with an inline `connect`
  clause** — `connection link : Link connect a to b;` and
  `interface iface : IfaceType connect a to b;` were silently accepted by `connection_def`/
  `interface_def` as empty-bodied *definitions*, with the `connect ... to ...` clause discarded
  entirely: the shared plain `: Type` header scan (`specialization::
  parse_optional_definition_header_after_identification`) greedily consumes everything up to
  `;`/`{`, extracts only a leading type name, and silently drops the rest. Fixed narrowly, not by
  reordering dispatch or requiring `def` (an earlier PAR-006b attempt at the latter broke real
  Systems Library parsing and was reverted — see `connection_def`'s doc comment): the header scan
  now also returns the raw swallowed text
  (`specialization::parse_optional_definition_header_with_raw`), and
  `DefinitionPrefixOptions::reject_header_keyword` fails the definition parse when that text
  contains a top-level `connect` keyword, so `connection_def`/`interface_def` correctly leave the
  input for the usage parser instead. Widened `connection_usage_member`
  (`part::body::connection_usage_member`) to parse an inline `connect` clause (binary and n-ary
  `connect (a, b, c, ...)` forms), populating new `ConnectionUsageMember::connect_from`/
  `connect_to`/`connect_extra_ends` fields. Added a package-level `interface_usage` dispatch arm
  (`part::usage::interface_usage`, previously only reachable nested inside a part body) and a new
  `PackageBodyElement::InterfaceUsage` variant, since no package-level interface-usage fallback
  existed at all before this fix. `PARSE_AST_VERSION` is 36.

## [0.37.0] - 2026-07-16

- **AttributeUsage multiplicity** — `AttributeUsage` now retains a structured
  `multiplicity: Option<Node<Multiplicity>>` from `MultiplicityPart` instead of
  discarding the parsed range after reading `ordered`/`nonunique`. `PARSE_AST_VERSION`
  is 35.

## [0.36.0] - 2026-07-16

Closes the gaps-doc "Parser work still required" backlog (typed `FeatureValue`, structured
relationship targets, a modifier-completeness audit, and a first-class `Membership` node), on top
of 0.35.0's PAR-002..006 backlog. `PARSE_AST_VERSION` moved from 15 (at 0.35.0) to 34 across this
backlog's many breaking AST-schema changes — most of it from the `Membership` rollout, which now
covers 51 of 53 member-bearing structs crate-wide (the 2 exclusions are BNF-confirmed: no textual
visibility position exists for them). Along the way this also found and fixed ~12 real parser bugs
beyond each item's original scope: 11 instances of a `*_def` parser that legally accepts a
`private`/`protected`/`public` prefix per the BNF but never parsed one (across `part`, `port`,
`item`, `connection`, the requirement/case family, `action`, `alias`, `flow`, `allocation`,
`state`/`individual`/`interface`, `metadata`/`enum`, `occurrence`/`succession`,
`constraint`/`calc`, and the view family), plus a case where `:=` was mis-parsed as the start of a
`:` typing clause. This release also includes a rustfmt/clippy technical-debt pass (see below) and
folds in two small fixes that had landed on top of the 0.35.0 tag but were never separately
published (`1eec56f` clippy fix, `1dfdcd3` header type-reference-drop fix). See the individual
entries below for what changed and why.

### Technical debt: rustfmt and clippy cleanup

Housekeeping pass after the PAR-002..006 and post-PAR-006 "Parser work still required" backlogs
(neither ran `cargo fmt`/full `cargo clippy` as part of their per-increment gate, only
`cargo test`/`clippy -W clippy::all`'s error count, so warnings and formatting drift accumulated).

- **`cargo fmt`**: 48 files had formatting drift (239 diff hunks); applied mechanically, no
  behavior change.
- **`clippy::large_enum_variant`** (`AttributeBodyElement`, `RequirementDefBodyElement`):
  `AttributeUsage`'s size relative to `Doc`/`Error` is inherent (it carries a `Membership` plus
  relationship nodes) and shared by ~10 other body-element enums crate-wide with the same variant
  shape; boxing it in just these two flagged enums would be an inconsistent partial fix, so both
  are `#[allow(clippy::large_enum_variant)]` with a documented reason instead.
- **`clippy::type_complexity`** (`connection.rs`/`interface.rs`'s `connect_ends`,
  `usage.rs`'s `optional_typings`): fixed properly with named type aliases (`ConnectEnds`,
  `TypingsResult`) rather than suppressed — genuine readability win, no behavior change.
- **`clippy::needless_lifetimes`** (3 test helper functions): auto-fixed via `cargo clippy --fix`.
- **Removed 10 blanket `#![allow(dead_code, unused_imports)]` module attributes**
  (`bnf_surface.rs`, `connection.rs`, `constraint.rs`, `interface.rs`, `part/mod.rs`, `port.rs`,
  `requirement.rs`, `state.rs`, `usecase.rs`, `view.rs`) that were masking real warnings.
  Underneath them: ~15 genuinely unused imports (auto-fixed via `cargo fix --lib`), and 3
  genuinely dead functions removed entirely (`safe_constraint_def_body_element` in
  `constraint.rs`, `constraint_body` in `requirement.rs`, `keyword_use_case_def` in
  `usecase.rs`, and `parse_usage_header` in `definition_header.rs` -- all confirmed unreferenced
  anywhere, including tests, before deleting). `bnf_surface.rs`'s five functions are a deliberate
  exception: each names and exercises one BNF production for grammar-coverage traceability, and is
  called only by that module's own `#[cfg(test)]` block -- restored `#![allow(dead_code)]` there
  with a comment explaining why, rather than deleting genuinely-intentional documentation code.
  `part/mod.rs`'s `part_def`/`part_def_body` re-exports got the same narrow, documented
  `#[allow(unused_imports)]` treatment for the same reason (used only by a `#[cfg(test)]` block in
  `package.rs`, so a plain non-test `cargo build` sees the re-export as unused).

All three gates green throughout: `cargo build --all-targets` (zero warnings), `cargo test`
(100% pass), `cargo clippy --all-targets -- -W clippy::all` (zero warnings, down from 5 lib + 3
test-file warnings), and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25).

### Item 4b closing sweep: scope-boundary list is now empty

Per the task's closing instruction, re-swept the whole `src/ast` tree for every `*Def`/`*Usage`-
shaped struct after all six increments above (and the `VariantUsage`/`ActorUsage`/
`MetadataKeywordUsage` BNF checks) landed. Every `pub struct *Def`/`pub struct *Usage` declaration
lives in exactly four files (`src/ast/behavior.rs`, `src/ast/requirement.rs`,
`src/ast/structure.rs`, `src/ast/view.rs`) -- confirmed by grepping the whole `src/ast` tree, no
other file declares one. Mechanically checked all 53 matching struct declarations for a
`membership:` field:

- **51 of 53 have it**, closing every struct this rollout (across this session and every prior
  session) has touched: `ActionDef`/`ActionUsage`, `FlowDef`/`FlowUsage`,
  `AllocationDef`/`AllocationUsage`, `StateDef`/`StateUsage`, `RequirementDef`/`RequirementUsage`,
  `ItemUsage`, `EnumerationUsage`, `ConcernUsage`, `CaseDef`/`CaseUsage`,
  `AnalysisCaseDef`/`AnalysisCaseUsage`, `VerificationCaseDef`/`VerificationCaseUsage`,
  `UseCaseDef`/`UseCaseUsage`, `ActorUsage`, `PartDef`/`PartUsage`, `AttributeDef`/`AttributeUsage`,
  `ItemDef`, `IndividualDef`, `VariantUsage`, `PortDef`/`PortUsage`, `InterfaceDef`,
  `ConnectionDef`, `MetadataDef`/`MetadataUsage`, `EnumDef`, `OccurrenceDef`/`OccurrenceUsage`,
  `SuccessionUsage`, `AliasDef`, `ConstraintDef`, `CalcDef`/`CalcUsage`, and the full view family
  (`ViewDef`/`ViewUsage`/`ViewpointDef`/`ViewpointUsage`/`RenderingDef`/`RenderingUsage`/
  `ViewRenderingUsage`).
- **2 of 53 deliberately do not**, both already documented with a confirmed (not guessed) grammar
  reason: `ThenUseCaseUsage` (`src/ast/requirement.rs`, wraps an already-`membership`-bearing
  `UseCaseUsage` -- it is the `then use case ...` clause, not a member) and
  `MetadataKeywordUsage` (`src/ast/structure.rs`, the `#keyword` shorthand -- confirmed above its
  BNF production `PrefixMetadataMember : OwningMembership = '#' ownedRelatedElement =
  PrefixMetadataUsage` has no `MemberPrefix`, so no textual position exists for a visibility
  keyword). `Objective` and `ExhibitState` (not `*Def`/`*Usage`-named, so outside this grep's
  matching set, but already individually confirmed and documented as non-members or out-of-scope
  in earlier entries in this file) round out the complete set of deliberate exclusions.

No code changes in this entry -- it is a verification pass confirming the prior six increments'
combined effect actually closed the scope, not a new increment. `cargo build --all-targets`,
`cargo test` (246 lib tests + full integration suite), `cargo clippy -- -W clippy::all` (same 5
pre-existing warnings), and the `SYSML_V2_RELEASE_DIR` validation gate (25/25) all remain green
against the final combined state. `PARSE_AST_VERSION` stays at 34 (last bumped by the
`VariantUsage`/`ActorUsage` entry above).

**Item 4b (the first-class `Membership` node item from the "Parser work still required" gaps doc)
is now complete.** The scope-boundary list every prior entry in this rollout carried forward is
empty except for the small set of struct-level, BNF-confirmed exclusions listed above -- there is
no remaining "not yet covered" struct family.

### Changed: `VariantUsage`/`ActorUsage` join the `Membership` rollout with new grammar-backed kinds; `MetadataKeywordUsage` confirmed out of scope

Resolves the three "unverified against BNF" structs the "Item 4b final sweep" entry flagged
(`VariantUsage`, `MetadataKeywordUsage`, `ActorUsage`) by actually checking each production in
`sysml-v2-release/bnf/SysML-textual-bnf.kebnf`, per the task's "don't invent fields with no grammar
backing" discipline used throughout this whole backlog.

**`VariantUsage`** (`src/ast/structure.rs`, the `variant ...;` member inside a `variation part def`
body) -- confirmed **in scope**: BNF `VariantUsageMember : VariantMembership = MemberPrefix
'variant' ownedVariantUsage = VariantUsageElement` carries its own `MemberPrefix` (visibility-
legal) and is explicitly typed as a distinct `VariantMembership` kind in the grammar itself, not a
plain `FeatureMembership`. Added `membership: Membership` with a **new**
`MembershipKind::VariantMembership` variant (`src/ast/membership.rs`, plus a `Membership::variant`
convenience constructor) rather than reusing `FeatureMembership`, since the BNF names it
distinctly. `variant_usage` (`src/parser/part/usage.rs`) previously accepted no visibility prefix at
all -- confirmed by probing `variation part def P { variant part v1: V1; }` vs. a `private`-prefixed
variant -- now calls `lex::visibility_prefix` at its start, threading the resulting `Membership`
through all five of its internal branches (typed `part`/`attribute`/`item`/`port` forms and the
untyped bare-reference form).

**`ActorUsage`** (`src/ast/requirement.rs`, distinct from the larger `RequirementActorDecl`/
`ActorDecl`) -- confirmed **in scope**: BNF `ActorMember : ActorMembership = MemberPrefix
ownedRelatedElement += ActorUsage` also carries its own `MemberPrefix` and its own distinctly-named
`ActorMembership` kind. Added `membership: Membership` with a **new**
`MembershipKind::ActorMembership` variant (plus `Membership::actor`). `actor_usage`
(`src/parser/usecase.rs`) previously accepted no visibility prefix -- now calls
`lex::visibility_prefix` at its start, ordered before the `actor` keyword.

**`MetadataKeywordUsage`** (`src/ast/structure.rs`, the `#keyword` annotation shorthand) --
confirmed **out of scope, for real grammar reasons**: its production is `PrefixMetadataMember :
OwningMembership = '#' ownedRelatedElement = PrefixMetadataUsage` -- no `MemberPrefix` anywhere in
this production, unlike every other struct touched by this rollout. The `#` sigil is the entire
prefix; there is no legal position for `private`/`protected`/`public` before it anywhere in the
grammar. Left with no `membership` field, matching the PAR-003b `unique`/`readonly`/`variable`
precedent for confirmed (not guessed) grammar exclusions.

No test-fixture struct-literal ripple and no `src/ast/mod.rs` normalize-match ripple -- neither
`VariantUsage` nor `ActorUsage` has hand-built literal construction sites or a dedicated normalize
helper.

`PARSE_AST_VERSION` bumped 33 -> 34. Added five regression tests locking in visibility capture (and
the new `VariantMembership`/`ActorMembership` kinds) across both the typed and untyped
`variant_usage` forms and `actor_usage` (`src/parser/part/usage.rs`, `src/parser/usecase.rs`). Full
`cargo test` (246 lib tests + full integration suite), `cargo clippy -- -W clippy::all` (same 5
pre-existing warnings, zero new), and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25) are
green, with **zero validation snapshot regeneration required** this increment.

This closes the "three unverified-against-BNF structs" item from the "Item 4b final sweep" entry:
two were genuine gaps (now fixed, with new grammar-backed `MembershipKind` variants), one is a
confirmed, documented exclusion.

### Changed: `Membership` rollout extended to the view family (`ViewDef`/`ViewUsage`/`ViewpointDef`/`ViewpointUsage`/`RenderingDef`/`RenderingUsage`/`ViewRenderingUsage`)

Sixth increment of the Item 4b final-sweep follow-up list, and the last of the "same mechanical
shape" batch the sweep entry called out as cheap, high-confidence follow-up work. Landed as one
increment (all seven structs, `src/ast/view.rs`/`src/parser/view.rs`) since they share one file and
one mechanical shape, matching the `RequirementUsage`-family precedent from earlier in this rollout.
Adds `membership: Membership` to `ViewDef`/`ViewpointDef`/`RenderingDef` (`kind:
OwningMembership`) and `ViewUsage`/`ViewpointUsage`/`RenderingUsage`/`ViewRenderingUsage` (`kind:
FeatureMembership`).

Confirmed all seven productions legally carry a visibility prefix before writing any code:
`ViewDefinition`/`ViewpointDefinition`/`RenderingDefinition` are all `OccurrenceDefinitionPrefix`-
backed, `ViewUsage`/`ViewpointUsage`/`RenderingUsage` are all `OccurrenceUsagePrefix`-backed, and
`ViewRenderingMember : ViewRenderingMembership = MemberPrefix 'render' ownedRelatedElement +=
ViewRenderingUsage` explicitly carries its own `MemberPrefix` (the same `render r1 : R1;` member
form nested inside a view/rendering definition body) -- all confirmed against
`SysML-textual-bnf.kebnf`'s Clause 8.2.2.26.

**Found the same genuine parsing gap an eleventh time, across all seven parsers.** None of
`view_def`, `viewpoint_def`, `rendering_def` (`DefinitionPrefixOptions`-routed), or the hand-rolled
`view_usage`, `viewpoint_usage`, `rendering_usage`, `view_rendering_usage` accepted a
`private`/`protected`/`public` prefix before this increment -- confirmed by probing `package P {
private view def V1; }` and `package P { view v: V1 { private render r1 : R1; } }` against the
pre-change parser (both fell through to recovery). Fixed the three `*Def` parsers via
`.with_captured_visibility()`, and the four hand-rolled `*Usage` parsers via a direct
`lex::visibility_prefix` call at each one's start (ordered before the existing `abstract` prefix on
`view_usage`/`viewpoint_usage`/`rendering_usage`; `view_rendering_usage` has no `abstract` prefix of
its own).

No test-fixture struct-literal ripple and no `src/ast/mod.rs` normalize-match ripple -- none of the
seven structs have hand-built literal construction sites or dedicated normalize helpers in this
crate.

`PARSE_AST_VERSION` bumped 32 -> 33. Added eleven regression tests locking in visibility capture and
the no-prefix default across the family (`src/parser/view.rs`). Full `cargo test` (241 lib tests +
full integration suite), `cargo clippy -- -W clippy::all` (same 5 pre-existing warnings, zero new),
and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25) are green, with **zero validation
snapshot regeneration required** this increment.

Updates the Item 4b scope-boundary list: the entire view family is no longer in the "not yet
covered" set. This closes the "same mechanical shape" batch the "Item 4b final sweep" entry called
out; still not covered: `VariantUsage`/`MetadataKeywordUsage`/`ActorUsage` (the three
unverified-against-BNF structs) -- see the next increment.

### Changed: `Membership` rollout continues to `ConstraintDef`/`CalcDef`/`CalcUsage`

Fifth increment of the Item 4b final-sweep follow-up list. Adds `membership: Membership` to
`ConstraintDef` (`src/ast/view.rs`, `kind: OwningMembership`), `CalcDef` (`kind:
OwningMembership`), and `CalcUsage` (`kind: FeatureMembership`).

**Found the same genuine parsing gap a tenth time, but with a twist**: unlike every prior struct in
this rollout, `constraint_def` and `parse_calc_def` (`src/parser/constraint.rs`) already had *some*
visibility handling -- both used `DefinitionPrefixOptions::with_private()`
(`VisibilityPrefix::OptionalPrivate`), which matched a bare `private` prefix but **discarded** it
(never populated `DefinitionPrefixResult::visibility`) and, unlike `Captured`, did not accept
`protected`/`public` at all. Both switched to `.with_captured_visibility()`, a strict superset (adds
`protected`/`public` support and captures the result instead of discarding it) confirmed
behavior-preserving for every previously-accepted input. `calc_usage` (hand-rolled, no
`DefinitionPrefixOptions` involved) had no visibility handling of any kind and now calls
`lex::visibility_prefix` directly at its start, same as every other hand-rolled `*_usage` parser in
this rollout.

**Removed the now-dead `VisibilityPrefix::OptionalPrivate` variant and
`DefinitionPrefixOptions::with_private()`** (`src/parser/definition_prefix.rs`) -- `constraint`/
`calc` were the only two call sites (per that enum variant's own doc comment), and both migrated to
`Captured` above, so keeping the discard-only mode around would have been orphaned dead code (it
was in fact flagging `cargo build`'s `dead_code` lint until removed). The one unit test exercising
`OptionalPrivate`'s `private`-before-`abstract` ordering (`prefix_private_before_abstract_constraint`,
`src/parser/definition_prefix.rs`) was updated to use `.with_captured_visibility()` instead, keeping
the same ordering assertion and additionally asserting the now-captured `visibility` field.

No test-fixture struct-literal ripple and no `src/ast/mod.rs` normalize-match ripple -- none of the
three structs have hand-built literal construction sites or dedicated normalize helpers in this
crate (`ConstraintDef`/`CalcDef`/`CalcUsage` all use the whole-value clone path where they appear in
body enums).

`PARSE_AST_VERSION` bumped 31 -> 32. Added six regression tests locking in visibility capture and
the no-prefix default for `constraint_def`/`calc_def`/`calc_usage` (`src/parser/constraint.rs`).
Full `cargo test` (231 lib tests + full integration suite), `cargo clippy -- -W clippy::all` (same 5
pre-existing warnings, zero new -- the `OptionalPrivate`/`with_private` dead-code warnings introduced
by this increment's own migration were fixed by removing the dead code rather than suppressed), and
the full `SYSML_V2_RELEASE_DIR` validation gate (25/25) are green, with **zero validation snapshot
regeneration required** this increment.

Updates the Item 4b scope-boundary list: `ConstraintDef`/`CalcDef`/`CalcUsage` are no longer in the
"not yet covered" set. Still not covered: the view family and the three unverified-against-BNF
structs -- same reasoning as the "Item 4b final sweep" entry below.

### Changed: `Membership` rollout continues to `OccurrenceDef`/`OccurrenceUsage`/`SuccessionUsage`

Fourth increment of the Item 4b final-sweep follow-up list. Adds `membership: Membership` to
`OccurrenceDef` (`src/ast/structure.rs`, `kind: OwningMembership`), `OccurrenceUsage` (`kind:
FeatureMembership`), and `SuccessionUsage` (`kind: FeatureMembership`).

**Found the same genuine parsing gap a ninth time.** `occurrence_def` (`src/parser/occurrence.rs`)
never accepted a `private`/`protected`/`public` prefix -- confirmed by probing `package P { private
occurrence def O1; }` against the pre-change parser (fell through to recovery). Fixed via
`.with_captured_visibility()` on its `DefinitionPrefixOptions`.

**`OccurrenceUsage`'s four real member-position entry points** (`src/parser/occurrence_body.rs`)
-- `occurrence_usage`, `individual_usage`, `snapshot_usage`, `timeslice_usage` -- each independently
verified against the BNF first: `OccurrenceUsage`/`IndividualUsage`/`PortionUsage` (the
`snapshot`/`timeslice` production) are all `OccurrenceUsagePrefix`-backed (`BasicUsagePrefix` +
optional `individual`/portion-kind), so all four legally carry a visibility prefix and each now
calls `lex::visibility_prefix` at its own start (threaded through the shared `occurrence_usage_tail`
helper via a new `membership: Membership` parameter, since each entry point captures its own
distinct visibility before delegating).

**`then_timeslice_usage`** (the `then timeslice ...` succession-continuation form) does **not** get
real visibility capture -- confirmed against the BNF this is not a distinct production with its own
`BasicUsagePrefix` the way the other four are (no `ThenTimeSliceUsage`/similar production exists
anywhere in `SysML-textual-bnf.kebnf`; `timeslice`/`snapshot` only appear via the shared
`PortionKind` alternative on `OccurrenceUsagePrefix`). Treated as an ad hoc site, `visibility: None`,
same as this rollout's other no-grammar-backing sites, and documented inline on the function.

**`succession_usage`** (the standalone `succession ... first ... then ...;` form,
`src/parser/occurrence_body.rs`) also gets real visibility capture: BNF `SuccessionAsUsage =
UsagePrefix ('succession' UsageDeclaration)? 'first' ... 'then' ...` is `UsagePrefix`-backed (which
includes visibility), and this parser is invoked directly from `occurrence_body_element`'s real
member-position dispatch, not a payload-sharing helper -- confirmed the same way `allocate_usage`
was confirmed in this session's first increment.

`src/ast/mod.rs`'s dedicated `normalize_occurrence_def` helper needed a one-line
`membership: o.membership.clone()` addition; `OccurrenceUsage`/`SuccessionUsage` have no dedicated
normalize functions and use the whole-value clone path, and neither has hand-built literal
construction sites in the test suite.

`PARSE_AST_VERSION` bumped 30 -> 31. Added nine regression tests locking in visibility capture (and
`then_timeslice_usage`'s deliberate lack of it) across `occurrence_def`, `occurrence_usage`,
`individual_usage`, `snapshot_usage`, `timeslice_usage`, `then_timeslice_usage`, and
`succession_usage` (`src/parser/occurrence.rs`, `src/parser/occurrence_body.rs`). Full `cargo test`
(225 lib tests + full integration suite), `cargo clippy -- -W clippy::all` (same 5 pre-existing
warnings, zero new), and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25) are green, with
**zero validation snapshot regeneration required** this increment.

Updates the Item 4b scope-boundary list: `OccurrenceDef`/`OccurrenceUsage`/`SuccessionUsage` are no
longer in the "not yet covered" set. Still not covered: `ConstraintDef`, `CalcDef`/`CalcUsage`, the
view family, and the three unverified-against-BNF structs -- same reasoning as the "Item 4b final
sweep" entry below.

### Changed: `Membership` rollout continues to `MetadataDef`/`MetadataUsage`/`EnumDef`/`EnumerationUsage`

Third increment of the Item 4b final-sweep follow-up list. Adds `membership: Membership` to
`MetadataDef` (`src/ast/structure.rs`, `kind: OwningMembership`), `MetadataUsage` (`kind:
FeatureMembership`), `EnumDef` (`kind: OwningMembership`), and `EnumerationUsage`
(`src/ast/requirement.rs`, `kind: FeatureMembership`) -- `EnumerationUsage` already had `is_end`
from the earlier modifier-completeness audit item; this adds the still-missing `membership`
alongside it.

**Found the same genuine parsing gap an eighth time, across all four parsers.** None of
`metadata_def` (`src/parser/metadata.rs`), `metadata_usage`, `enum_def` (`src/parser/enumeration.rs`),
or `enum_usage` accepted a `private`/`protected`/`public` prefix before this increment -- confirmed
by probing `package P { private metadata def M1; }` and `package P { private enum def E1; }`
against the pre-change parser (both fell through to recovery). Fixed `metadata_def`/`enum_def` via
`.with_captured_visibility()` on their `DefinitionPrefixOptions`, and `metadata_usage`/`enum_usage`
via a direct `lex::visibility_prefix` call at each one's start. `enum_usage`'s visibility capture is
ordered before its existing `is_end` prefix check (visibility, then `end`, then the `enum` keyword),
matching `attribute_usage`'s established `visibility_prefix` -> `EndUsagePrefix`/`RefPrefix` ordering
from the modifier-completeness audit.

`src/ast/mod.rs`'s dedicated `normalize_metadata_def`/`normalize_enum_def`/
`normalize_enumeration_usage` helpers each needed a one-line `membership: ...clone()` addition (they
null out individual fields rather than whole-value-clone); `MetadataUsage` has no dedicated
normalize function and uses the whole-value clone path. No test-fixture struct-literal ripple --
none of the four structs have hand-built literal construction sites in the test suite.

`PARSE_AST_VERSION` bumped 29 -> 30. Added twelve regression tests locking in visibility capture and
the no-prefix default (`src/parser/metadata.rs`, `src/parser/enumeration.rs`). Full `cargo test`
(215 lib tests + full integration suite), `cargo clippy -- -W clippy::all` (same 5 pre-existing
warnings, zero new), and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25) are green, with
**zero validation snapshot regeneration required** this increment.

Updates the Item 4b scope-boundary list: `MetadataDef`/`MetadataUsage`/`EnumDef`/`EnumerationUsage`
are no longer in the "not yet covered" set. Still not covered:
`OccurrenceDef`/`OccurrenceUsage`/`SuccessionUsage`, `ConstraintDef`, `CalcDef`/`CalcUsage`, the view
family, and the three unverified-against-BNF structs -- same reasoning as the "Item 4b final sweep"
entry below.

### Changed: `Membership` rollout continues to `StateDef`/`StateUsage`/`IndividualDef`/`InterfaceDef`

Second increment of the Item 4b final-sweep follow-up list. Adds `membership: Membership` to
`StateDef` (`src/ast/behavior.rs`, `kind: OwningMembership`), `StateUsage` (`kind:
FeatureMembership`), `IndividualDef` (`src/ast/structure.rs`, `kind: OwningMembership`), and
`InterfaceDef` (`src/ast/structure.rs`, `kind: OwningMembership`) -- `Transition` (also in
`state.rs`) is a distinct control-flow construct with no independent membership form of its own and
was left untouched, matching this rollout's precedent of only wiring the field onto real
member-bearing `*Def`/`*Usage` structs.

**Found the same genuine parsing gap a seventh time, across all four parsers.** None of `state_def`
(`src/parser/state.rs`), `state_usage`, `individual_def` (`src/parser/individual.rs`), or
`parse_interface_def` (`src/parser/interface.rs`, backing both `interface_def` and
`interface_def_required`) accepted a `private`/`protected`/`public` prefix before this increment --
confirmed by probing `package P { private state def S1; }`, `package P { private individual def
I1; }`, and `package P { private interface def I1; }` against the pre-change parser (all fell
through to recovery). Fixed `state_def`/`individual_def`/`parse_interface_def` via
`.with_captured_visibility()` on their `DefinitionPrefixOptions`, and `state_usage` via a direct
`lex::visibility_prefix` call at its start, ordered before the optional `abstract` prefix.

**`ExhibitState`** (`src/ast/structure.rs`, the `exhibit state name ...` shorthand parsed by
`part/body.rs::exhibit_state`) has its own `OccurrenceUsagePrefix`-backed visibility grammar per the
BNF's `ExhibitStateUsage` production, but `ExhibitState` itself has no `membership` field and is not
in this item's scope (it is a distinct struct from `StateUsage`, not one of the eleven families this
sweep covers). Its adapter site in `src/parser/part/usage.rs::exhibit_state_as_state_usage`, which
repackages an already-parsed `ExhibitState` into a `StateUsage` (used where a state usage is
expected but the source wrote the `exhibit state` shorthand instead), therefore has no visibility
data available to thread through and sets `visibility: None`, documented inline as an ad hoc site
per this rollout's established convention -- adding `membership` to `ExhibitState` itself, if ever
needed, is a separate, explicitly out-of-scope follow-up.

One test-fixture struct-literal ripple: `tests/validation/parts_interconnection_2a.rs`'s two
hand-built `InterfaceDef` literals (`interface_def_engine_to_transmission`,
`interface_def_driveshaft`) needed `membership: owning_membership()` added (both source
declarations have no explicit visibility prefix). `src/ast/mod.rs`'s `normalize_interface_def`
needed a one-line `membership: i.membership.clone()` addition; `StateDef`/`StateUsage`/
`IndividualDef` have zero hand-built literal construction sites and their consumers all use the
whole-value `n.value.clone()` path.

`PARSE_AST_VERSION` bumped 28 -> 29. Added twelve regression tests locking in visibility capture
and the no-prefix default (`src/parser/state.rs`, `src/parser/individual.rs`,
`src/parser/interface.rs`). Full `cargo test` (207 lib tests + full integration suite), `cargo
clippy -- -W clippy::all` (same 5 pre-existing warnings, zero new), and the full
`SYSML_V2_RELEASE_DIR` validation gate (25/25) are green, with **zero validation snapshot
regeneration required** this increment.

Updates the Item 4b scope-boundary list: `StateDef`/`StateUsage`/`IndividualDef`/`InterfaceDef` are
no longer in the "not yet covered" set. Still not covered: `MetadataDef`/`MetadataUsage`,
`EnumDef`/`EnumerationUsage`, `OccurrenceDef`/`OccurrenceUsage`/`SuccessionUsage`, `ConstraintDef`,
`CalcDef`/`CalcUsage`, the view family, and the three unverified-against-BNF structs -- same
reasoning as the "Item 4b final sweep" entry below.

### Changed: `Membership` rollout continues to `FlowDef`/`FlowUsage`/`AllocationDef`/`AllocationUsage`

First increment of the Item 4b final-sweep follow-up list (see the "Item 4b final sweep" entry
below for the full remaining scope this continues). Adds `membership: Membership` to `FlowDef`
(`src/ast/behavior.rs`, `kind: OwningMembership`), `FlowUsage` (`kind: FeatureMembership`),
`AllocationDef` (`kind: OwningMembership`), and `AllocationUsage` (`kind: FeatureMembership`).

**Found the same genuine parsing gap a sixth time, across all four parsers.** None of `flow_def`
(`src/parser/flow.rs`), `flow_usage_member`, `allocation_def` (`src/parser/allocation.rs`), or
`allocation_usage` accepted a `private`/`protected`/`public` prefix before this increment --
confirmed by probing `package P { private flow def F1; }` and `package P { private allocation def
A1; }` against the pre-change parser (both fell through to recovery). Fixed `flow_def`/
`allocation_def` via `.with_captured_visibility()` on their `DefinitionPrefixOptions`, and
`flow_usage_member`/`allocation_usage` via a direct `lex::visibility_prefix` call at each one's
start, ordered before the optional `abstract` prefix, matching this rollout's established ordering.

**`allocate_usage`** (`src/parser/allocation.rs`, the bare `allocate a to b;` shorthand form) also
got a `lex::visibility_prefix` call, unlike this rollout's usual "ad hoc site, `visibility: None`"
treatment for shared/no-grammar sites -- confirmed against the BNF first: `AllocationUsage =
OccurrenceUsagePrefix AllocationUsageDeclaration UsageBody` where `AllocationUsageDeclaration :
AllocationUsage = 'allocation' UsageDeclaration ('allocate' ConnectorPart)? | 'allocate'
ConnectorPart` -- both the `allocation ... allocate ...` and bare `allocate ... to ...` forms are
alternatives of the *same* production reachable through the *same* `OccurrenceUsagePrefix`, so the
bare-`allocate` shorthand legally carries a visibility prefix too, and `allocate_usage` is invoked
directly from real member-position call sites (`package.rs`), not a payload-sharing helper with no
visibility grammar of its own -- unlike `AllocationUsage`'s truly ad hoc analogues elsewhere in this
rollout (e.g. `ActionUsage`'s `control_node_payload_stmt` site).

No test-fixture struct-literal ripple (`FlowDef`/`FlowUsage`/`AllocationDef`/`AllocationUsage` have
zero hand-built literal construction sites in the test suite; every consumer in `src/ast/mod.rs`'s
normalize matches uses the whole-value `n.value.clone()` path). `PARSE_AST_VERSION` bumped 27 -> 28.
Added ten regression tests locking in visibility capture and the no-prefix default
(`src/parser/flow.rs`, `src/parser/allocation.rs`), including one covering `allocate_usage`'s
visibility prefix specifically. Regenerated the `function_based_behavior_3a` validation snapshot
(contains `flow`/`message` declarations) whose `Debug` output changed shape from the new field; full
`cargo test` (199 lib tests + full integration suite), `cargo clippy -- -W clippy::all` (same 5
pre-existing warnings, zero new), and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25) are
green.

Updates the Item 4b scope-boundary list: `FlowDef`/`FlowUsage`/`AllocationDef`/`AllocationUsage` are
no longer in the "not yet covered" set. Still not covered: `StateDef`/`StateUsage`, `IndividualDef`,
`InterfaceDef`, `MetadataDef`/`MetadataUsage`, `EnumDef`/`EnumerationUsage`,
`OccurrenceDef`/`OccurrenceUsage`/`SuccessionUsage`, `ConstraintDef`, `CalcDef`/`CalcUsage`, the view
family, and the three unverified-against-BNF structs (`VariantUsage`/`MetadataKeywordUsage`/
`ActorUsage`) -- same reasoning as the "Item 4b final sweep" entry below.

### Changed: `AliasDef`/`Import` join the `Membership` rollout -- closes the two structs deferred since the very first increment

Closes the two struct families every prior entry in this rollout explicitly deferred ("`AliasDef`/
`Import` -- `MembershipKind::Alias`/`MembershipKind::Import` variants exist ... but nothing
constructs them yet"). This is the third and last of this session's three increments (see the two
entries below for `RequirementUsage`-family and `ActionDef`/`ActionUsage`); together they close the
item's confirmed scope and this session runs the final sweep documented below.

**`AliasDef`** gets a non-optional `membership: Membership` field, `kind:
`[`MembershipKind::Alias`]` -- the variant reserved since the rollout's first increment, previously
unconstructed. **Found the same genuine parsing gap a fifth time**: BNF `AliasMember : Membership =
MemberPrefix 'alias' ( '<' memberShortName = NAME '>' )? ( memberName = NAME )? 'for' memberElement
= [QualifiedName] RelationshipBody` (`SysML-textual-bnf.kebnf`) legally permits a
`private`/`protected`/`public` prefix before `alias`, but `alias_def` (`src/parser/alias.rs`) never
parsed one at all before this increment -- confirmed by probing `package P { private alias m for
ISQ::mass; }` against the pre-change parser (fell through to recovery). Fixed by calling the shared
`lex::visibility_prefix` at the very start of `alias_def`, same as every hand-rolled usage parser in
this rollout.

**`Import`** gets `membership: Membership`, `kind:` `[`MembershipKind::Import`]`. **Design
decision, since this struct is different from every other one in this rollout**: `Import` already
had its own `visibility: Option<Visibility>` field from before this item (`src/ast/common.rs`), so
the two options left open by the very first increment's scope-boundary note were: (a) add
`membership: Membership` alongside the old field, keeping both, or (b) replace the old field with
`membership`. This lands **(b), a straight replacement** -- `Import.visibility` already captured
exactly the same information `Membership.visibility` does (an optional
`private`/`protected`/`public` prefix, no separate ownership/kind data worth preserving alongside
it), and grepping the whole crate (`src/`, `tests/`) found no in-crate consumer reading
`Import.visibility` other than its own constructor in `import.rs` -- a dual-field design would only
have added a redundant, confusing field with no compatibility benefit for zero real consumers. The
old field's hand-inlined `opt(alt((tag("public"), tag("private"), tag("protected"))))` visibility
match in `import_` was also replaced by the shared `lex::visibility_prefix` call (for consistency
and span capture), so this is a pure behavior-preserving refactor plus the new wrapper -- `Import`
already had full visibility grammar coverage, so (unlike every other struct in this rollout) there
was no parsing gap to fix here.

Fixed the two downstream `Import` struct-literal construction sites in
`tests/validation/parts_interconnection_2a.rs` (both `public import ...;`) via a new
`public_import_membership()` test helper alongside the fixture's existing
`owning_membership()`/`feature_membership()` helpers. `PARSE_AST_VERSION` bumped 26 -> 27 for the
breaking AST-schema change. Added four regression tests (`alias_def`/`import_` visibility capture
and no-prefix default, `src/parser/alias.rs`, `src/parser/import.rs`). Regenerated the
`parts_tree_1a`, `function_based_behavior_3a`, and `functional_allocation_4a` validation snapshots
(all contain `import` declarations) whose `Debug` output changed shape from the field rename; full
`cargo test`, `cargo clippy -- -W clippy::all` (same 5 pre-existing warnings, zero new), and the
full `SYSML_V2_RELEASE_DIR` validation gate (25/25, including `full_library_suite` which exercises
every `import`/`alias` declaration in the Kernel and Systems libraries) are green.

### Changed: first-class `Membership` node extended to `ActionDef`/`ActionUsage`

Second of this session's three increments. Adds `membership: Membership` to `ActionDef`
(`src/ast/behavior.rs`, `kind: OwningMembership`) and `ActionUsage` (`kind: FeatureMembership`).
**Found the same genuine parsing gap again**: `action_def` (`src/parser/action.rs`) never accepted
a `private`/`protected`/`public` prefix -- confirmed by probing `package P { private action def
Foo; }` pre-change. Fixed via `.with_captured_visibility()` on `action_def`'s
`DefinitionPrefixOptions`, and a direct `lex::visibility_prefix` call at the start of the primary
`action_usage` parser (`src/parser/action.rs`), matching every hand-rolled usage parser in this
rollout.

`ActionUsage` has a second, ad hoc construction site with no visibility grammar of its own:
`control_node_payload_stmt` (`src/parser/payload.rs`), the standalone `accept`/`send`
control-node-statement shorthand (e.g. `accept msg : Type;` as its own statement, not part of an
`action ...` declaration) -- this always sets `visibility: None`, matching this rollout's
established convention for ad hoc, no-visibility-grammar sites (see `AttributeUsage`'s three ad hoc
sites in the item's first increment).

No test-fixture struct-literal ripple (`ActionDef`/`ActionUsage` have zero hand-built literal
construction sites in the test suite); only `src/ast/mod.rs`'s dedicated `normalize_action_def`/
`normalize_action_usage` helpers needed a one-line `membership: ...clone()` addition (they null out
individual fields rather than whole-value-clone). `PARSE_AST_VERSION` bumped 25 -> 26. Added four
regression tests locking in visibility capture and the no-prefix default for `action_def`/
`action_usage` (`src/parser/action.rs`). All three gates green (see the combined gate run noted in
the entry above for the full three-increment session).

### Changed: first-class `Membership` node extended to the `RequirementUsage`-family (`RequirementDef`/`RequirementUsage`/`ConcernUsage`/`CaseDef`/`CaseUsage`/`AnalysisCaseDef`/`AnalysisCaseUsage`/`VerificationCaseDef`/`VerificationCaseUsage`/`UseCaseDef`/`UseCaseUsage`)

First of this session's three increments closing the item's confirmed remaining scope (see the
"still not covered" list at the bottom of the previous `ConnectionDef`/`ConnectionUsageMember`
entry). Landed as one increment rather than several smaller ones because every struct in this list
shares one of two parser helpers (`parse_definition_prefix` for every `*Def`, and either a shared
payload builder or a small hand-rolled `case_like_usage_body`/`use_case_usage_tail` helper for every
`*Usage`), so the mechanical shape is identical across all eleven structs -- splitting further would
not have reduced risk, only increased the number of gate cycles for no benefit. Confirmed the
family's exact membership by reading `src/ast/requirement.rs` in full first, per the task's own
instruction not to guess: it covers `RequirementDef`/`RequirementUsage` (`src/parser/requirement.rs`),
`ConcernUsage` (same file -- no separate `ConcernDef` struct exists in this AST; see below),
`CaseDef`/`CaseUsage`/`AnalysisCaseDef`/`AnalysisCaseUsage`/`VerificationCaseDef`/
`VerificationCaseUsage` (`src/parser/case.rs`), and `UseCaseDef`/`UseCaseUsage`
(`src/parser/usecase.rs`). All eleven get a non-optional `membership: Membership` field, `kind:
OwningMembership` on the five `*Def` structs and `kind: FeatureMembership` on the six `*Usage`
structs.

**Found the same genuine parsing gap a fourth time, across all five `*Def` parsers and four of the
six `*Usage` parsers.** None of `requirement_def`, `case_def`, `analysis_case_def`,
`verification_case_def`, or `use_case_def` accepted a visibility prefix before this increment
(confirmed the same way as every prior entry: `package P { private requirement def Foo; }` and
siblings all fell through to recovery pre-change) -- fixed via `.with_captured_visibility()` on each
one's `DefinitionPrefixOptions`. The hand-rolled `case_usage`/`analysis_case_usage`/
`verification_case_usage`/`use_case_usage`/`concern_usage` had the same gap -- fixed via a direct
`lex::visibility_prefix` call at each one's start, ordered before the optional `abstract` keyword
(matching this rollout's established prefix ordering: visibility, then abstract, then the type
keyword).

Two structural wrinkles specific to this family, both handled the same way this rollout has handled
every ad hoc/shared-payload site so far (real visibility only at the true member-position parser,
`visibility: None` everywhere else):
- `RequirementUsage`'s payload is built by a single shared function
  (`parse_requirement_usage_payload_with_abstract`) called from three places: the member-position
  `requirement_usage` parser (captures real visibility, overrides the payload's `membership` field
  after the call), `verify_requirement`'s `verify requirement ...` form, and `usecase.rs`'s
  `objective { requirement ... }` form (neither of the latter two has visibility grammar of its
  own, so the payload always builds `visibility: None` and those two callers don't override it).
- `CaseUsage` is similarly shared by `AnalysisCaseUsage`/`VerificationCaseUsage`, which each parse
  their own visibility prefix independently (all three keywords are mutually exclusive
  alternatives, not a shared entry point) and pass the resulting `Membership` into
  `case_like_usage_body` as a parameter rather than re-deriving it. `UseCaseUsage` similarly has two
  construction call sites: the true member-position `use_case_usage` (captures real visibility) and
  `use_case_usage_in_body` (only reachable via the `then use case ...` control-flow production in
  `then_use_case_usage`, which has no visibility grammar of its own -- `visibility: None`).
- `ConcernUsage` has no separate `ConcernDef` struct in this AST even though the BNF has a distinct
  `ConcernDefinition` production -- `concern_usage` already parses both the `concern` and `concern
  def` textual forms into the one `ConcernUsage` struct, a pre-existing design predating this item
  and out of scope to change here; it gets `kind: FeatureMembership` regardless of which textual
  form matched.

No test-fixture struct-literal ripple (all eleven structs have zero hand-built literal construction
sites in the test suite; every consumer in `src/ast/mod.rs`'s normalize matches uses the whole-value
`n.value.clone()` path). `PARSE_AST_VERSION` bumped 24 -> 25. Added ten regression tests locking in
visibility capture and the no-prefix default across the family (`src/parser/requirement.rs`,
`src/parser/case.rs`, `src/parser/usecase.rs`), including one confirming the `verify requirement
...` payload site stays `visibility: None`.

**Combined gate run for all three of this session's increments** (`RequirementUsage`-family above,
`ActionDef`/`ActionUsage`, and `AliasDef`/`Import` below): `cargo build --all-targets`, `cargo test`
(189 lib tests + full integration suite, all green), `cargo clippy -- -W clippy::all` (same 5
pre-existing `large_enum_variant`/`type_complexity` warnings as every prior entry, zero new), and
the full `SYSML_V2_RELEASE_DIR` validation gate (25/25, including `full_library_suite`, which
exercises every `requirement`/`case`/`action`/`import`/`alias` declaration in the real Kernel and
Systems libraries) are green.

### Item 4b final sweep: remaining member-bearing struct families not covered by this rollout

Per the task's closing instruction, swept the whole `src/ast` tree for every `*Def`/`*Usage`-shaped
struct and cross-checked which already have a `membership` field after this session's three
increments land. **This rollout (Item 4b) is not fully complete** -- the sweep found substantially
more member-bearing struct families than the task anticipated when framing this session as the
"last increment." The following are explicitly *not* covered, left as a documented follow-up rather
than guessed at or rushed through with reduced gate discipline:

- **Same mechanical shape as every struct this rollout has already covered** (a `*Def` parser
  routed through `parse_definition_prefix`, needing only `.with_captured_visibility()`, plus a
  sibling `*Usage` parser needing a `lex::visibility_prefix` call at its start) -- expected to be
  cheap, high-confidence follow-up increments of the identical shape as every entry in this
  changelog, deferred purely because this session's scope was bounded to the three items the task
  explicitly named plus this sweep, not because of any grammar or design obstacle:
  `FlowDef`/`FlowUsage` (`src/parser/flow.rs`), `AllocationDef`/`AllocationUsage`
  (`src/parser/allocation.rs`), `StateDef`/`StateUsage` (`src/parser/state.rs`), `IndividualDef`
  (`src/parser/individual.rs`), `InterfaceDef` (`src/parser/interface.rs`), `MetadataDef`/
  `MetadataUsage` (`src/parser/metadata.rs`), `EnumDef`/`EnumerationUsage`
  (`src/parser/enumeration.rs` -- `EnumerationUsage` already has `is_end` from the modifier-audit
  item but no `membership`), `OccurrenceDef`/`OccurrenceUsage`/`SuccessionUsage`
  (`src/parser/occurrence.rs`/`occurrence_body.rs`), `ConstraintDef` (`src/parser/constraint.rs`),
  `CalcDef`/`CalcUsage` (`src/parser/constraint.rs`), and the view family `ViewDef`/`ViewUsage`/
  `ViewpointDef`/`ViewpointUsage`/`RenderingDef`/`RenderingUsage`/`ViewRenderingUsage`
  (`src/parser/view.rs`).
- **`VariantUsage`** (`src/ast/structure.rs`) -- a variant member inside a `variation part def`
  body; not yet checked against the BNF for whether `VariantUsageElement` legally carries its own
  `MemberPrefix` distinct from the nested usage it wraps. Left unverified rather than guessed at;
  should be confirmed against the grammar before either wiring it in or excluding it for real
  (grammar-backed) reasons.
- **`MetadataKeywordUsage`** (`src/ast/structure.rs`) -- the `#keyword` annotation shorthand
  attached to an element; likely closer to an annotation than a first-class member, but not
  confirmed against the BNF's `MetadataUsage`/annotation productions in this session. Same
  "unverified, not excluded for a confirmed reason" caveat as `VariantUsage`.
- **`ActorUsage`** (`src/ast/requirement.rs`, `usecase.rs`'s `actor_usage`, distinct from the
  larger `RequirementActorDecl`/`ActorDecl` already in this AST) -- a use-case-body actor
  declaration; same "same mechanical shape, not yet verified against the BNF's `MemberPrefix`
  applicability at this exact production" status as the two above.
- **Confirmed *not* real memberships, no field needed**: `ThenUseCaseUsage` (`src/ast/
  requirement.rs`) wraps an already-`membership`-bearing `UseCaseUsage` inside the `then use case
  ...` control-flow clause -- it is the clause, not a member, and its wrapped `UseCaseUsage` already
  carries `visibility: None` at that position (see the `RequirementUsage`-family entry above).
  `Objective` (`src/ast/requirement.rs`) similarly wraps an already-`membership`-bearing
  `RequirementUsage` and is not itself a member.

This list should be treated as the new scope-boundary state for any future continuation of Item 4b,
replacing the shorter "still not covered" lists in every entry below.

### Fixed: `PARSE_AST_VERSION` was left at 20 instead of 21 after the Part extension

Found while starting the next `Membership` continuation: the "extended to `PartDef`/`PartUsage`"
entry below documents `PARSE_AST_VERSION` bumped 20 -> 21, but the actual constant in `src/lib.rs`
was still 20 (only the prior 19 -> 20 bump for the `AttributeDef`/`AttributeUsage` entry had
actually landed in code) -- a genuine documentation/code drift in this still-uncommitted working
tree, not a re-interpretation of either entry's intent. Corrected `PARSE_AST_VERSION` to 21 to match
what the Part entry always claimed, before bumping further for this session's own work below.

### Changed: first-class `Membership` node extended to `ConnectionDef`/`ConnectionUsageMember`

Continues the `Membership` rollout (see `ItemDef`/`ItemUsage` above) to `ConnectionDef` and
`ConnectionUsageMember` (`src/ast/structure.rs`). Note the plan's original "`ConnectionUsage`" name
does not exist in the current AST -- the struct backing a nested `connection` usage member (inside a
part definition body or at package level) is `ConnectionUsageMember`, confirmed against
`src/ast/structure.rs` before assuming otherwise (its parser is `connection_usage_member`, shared
between `src/parser/part/body.rs`'s part-body dispatch and `src/parser/package.rs`'s package-level
dispatch -- both call sites automatically pick up the new field with no changes needed of their own,
since they only invoke the shared parser function). Same mechanical shape as every prior entry:
non-optional `membership: Membership`, `kind: OwningMembership` on `ConnectionDef`, `kind:
FeatureMembership` on `ConnectionUsageMember`.

**Found the same genuine parsing gap a third time, in both directions again.** Neither
`connection_def`/`connection_def_required` nor `connection_usage_member` accepted a visibility
prefix before this increment -- confirmed the same way as `PortDef`/`ItemDef`: `package P { private
connection def Foo; }` and `package P { part p: T { private connection c: C; } }` both failed to
parse as visibility-prefixed declarations pre-change. Fixed using the same `.with_captured_visibility()`
infrastructure from the `PortDef`/`PortUsage` increment: both `connection_def` (which also sets
`.with_hash_annotation()` for its def-less `#annotation` form) and `connection_def_required` now set
`.with_captured_visibility()` on their `DefinitionPrefixOptions`; `connection_usage_member` (a
bespoke hand-rolled parser, not routed through `parse_definition_prefix`) calls the shared
`lex::visibility_prefix` directly at its start, same as `port_usage`/`item_usage`/`part_usage`.

Unlike `PortDef`/`PortUsage`, and matching `ItemDef`/`ItemUsage`, **no test-fixture struct-literal
ripple was needed** -- `ConnectionDef`/`ConnectionUsageMember` have zero hand-built literal
construction sites anywhere in the test suite (every existing consumer only pattern-matches and
clones the whole value via `n.value.clone()` in `src/ast/mod.rs`'s normalize matches; only the
dedicated `normalize_connection_def` helper needed a one-line `membership: c.membership.clone()`
addition, since `ConnectionUsageMember`-bearing enum variants use the direct-clone path with no
dedicated normalize function). `cargo test` and the full `SYSML_V2_RELEASE_DIR` validation gate
(25/25) both stayed green with **zero validation snapshot regeneration required**.

`PARSE_AST_VERSION` bumped 23 -> 24. Added seven regression tests locking in visibility capture
(`private`/`protected`/`public` and the no-prefix default) for `connection_def` and
`connection_usage_member` (`src/parser/connection.rs`). All three gates green: `cargo test`, `cargo
clippy -- -W clippy::all` (same pre-existing warnings only, zero new), and the full
`SYSML_V2_RELEASE_DIR` validation gate (25/25).

Updates the scope-boundary list: `ConnectionDef`/`ConnectionUsageMember` are no longer in the "not
yet covered" set. Still not covered: `RequirementUsage`-family, `ActionDef`/`ActionUsage`, etc., and
`AliasDef`/`Import` -- same reasoning as before. This closes the three struct families explicitly
scoped for this continuation (`PortDef`/`PortUsage`, `ItemDef`/`ItemUsage`,
`ConnectionDef`/`ConnectionUsageMember`); further struct families remain a documented follow-up per
the same "start narrow, extend as needed" discipline this whole `Membership` rollout has used.

### Changed: first-class `Membership` node extended to `ItemDef`/`ItemUsage`

Continues the `Membership` rollout (see `PortDef`/`PortUsage` above) to `ItemDef`
(`src/ast/structure.rs`) and `ItemUsage` (`src/ast/requirement.rs` -- note the struct lives outside
`structure.rs` despite the "Item" name; its parsers are in `src/parser/item.rs` regardless). Same
mechanical shape as every prior entry in this rollout: non-optional `membership: Membership`, `kind:
OwningMembership` on `ItemDef`, `kind: FeatureMembership` on `ItemUsage`.

**Found the same genuine parsing gap again, in both directions.** Neither `item_def`/
`item_def_required` (routed through `parse_definition_prefix`, previously with no visibility option
set) nor the hand-rolled `item_usage` accepted a visibility prefix before this increment --
confirmed the same way as `PortDef`/`PortUsage`: `package P { private item def Foo; }` and `package P
{ part p: T { private item q: Q; } }` both failed to parse as visibility-prefixed declarations
pre-change. Fixed using the exact infrastructure the `PortDef`/`PortUsage` increment introduced:
`item_def`'s options now set `.with_captured_visibility()` (the `VisibilityPrefix::Captured` mode
added last increment), and `item_usage` calls the shared `lex::visibility_prefix` directly at its
start, same as `port_usage`/`part_usage`. `directed_item_usage` (the `in`/`out`/`inout item` form
used in port definition bodies) needed no separate change -- it delegates to `item_usage` internally
and inherits its `membership` field automatically; visibility must precede the direction keyword,
same ordering constraint as every other prefix-stacked usage parser in this crate.

Unlike the `PortDef`/`PortUsage` increment, **no test-fixture struct-literal ripple** was needed:
`ItemDef`/`ItemUsage` have zero hand-built literal construction sites in
`tests/validation/parts_interconnection_2a.rs` or anywhere else in the test suite (every existing
consumer only pattern-matches and clones the whole value via `n.value.clone()` in `src/ast/mod.rs`'s
normalize matches, which needed no changes since neither struct had spans to null out). `cargo test`
and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25) both stayed green with **zero validation
snapshot regeneration required** -- the Systems/Full Library sources exercised by that gate contain
no `item`/`item def` declaration whose `Debug` output the gate checks byte-for-byte against a stored
snapshot.

`PARSE_AST_VERSION` bumped 22 -> 23. Added six regression tests locking in visibility capture
(`private`/`protected`/`public` and the no-prefix default) for both `item_def` and `item_usage`
(`src/parser/item.rs`). All three gates green: `cargo test`, `cargo clippy -- -W clippy::all` (same
pre-existing warnings only, zero new), and the full `SYSML_V2_RELEASE_DIR` validation gate (25/25).

Updates the scope-boundary list: `ItemDef`/`ItemUsage` are no longer in the "not yet covered" set.
Still not covered: `ConnectionDef`/`ConnectionUsageMember`, `RequirementUsage`-family,
`ActionDef`/`ActionUsage`, etc., and `AliasDef`/`Import` -- same reasoning as before.

### Changed: first-class `Membership` node extended to `PortDef`/`PortUsage`

Continues the `Membership` rollout (see the `PartDef`/`PartUsage` entry below) to `PortDef`/
`PortUsage` (`src/ast/structure.rs`, `src/parser/port.rs`). Same mechanical shape: a non-optional
`membership: Membership` field, `kind: OwningMembership` on `PortDef`, `kind: FeatureMembership` on
`PortUsage`, visibility captured at the point each parser previously matched-and-discarded (in
`PortUsage`'s case, "previously" means "never matched at all" -- see the gap below) a
`private`/`protected`/`public` prefix.

**Found the same genuine parsing gap `PartDef` had, in two places this time.** Neither
`port_def`/`port_def_required` nor `port_usage` accepted a visibility prefix before this increment
-- confirmed by probing `package P { private port def Foo; }` and `package P { part p: T { private
port q: Q; } }` against the pre-change parser: both fell through to recovery instead of parsing as a
visibility-prefixed `PortDef`/`PortUsage`. Fixed as part of wiring `Membership` in, matching the
`PartDef` precedent (BNF `DefinitionMember`/`UsageMember` productions both legally permit a
visibility prefix wherever a `*Def`/`*Usage` is legal).

`port_usage` (a bespoke hand-rolled parser, not routed through `parse_definition_prefix`) now calls
the shared `lex::visibility_prefix` directly at its very start, same as `part_usage`. `port_def`/
`port_def_required` (both routed through the shared `parse_definition_prefix`/
`DefinitionPrefixOptions` helper, unlike `part_def`) needed a different mechanism: the existing
`VisibilityPrefix::OptionalPrivate` mode on `DefinitionPrefixOptions` only matched-and-discarded a
bare `private` (kept for its two pre-existing `constraint`/`calc` call sites, unchanged here), so a
new `VisibilityPrefix::Captured` mode plus `DefinitionPrefixOptions::with_captured_visibility()`
builder were added (`src/parser/definition_prefix.rs`) that call `lex::visibility_prefix` and thread
the result through two new `DefinitionPrefixResult` fields (`visibility`, `visibility_span`) --
`None`/zero-span for every other existing call site that doesn't opt in, so this is additive and
does not change behavior for `constraint`/`calc`/`state`/`flow`/etc. `port_def`'s options now set
`.with_captured_visibility()` for both the `def`-optional and `def`-required entry points.

Fixed all 24 downstream `PortDef`/`PortUsage` struct-literal construction sites in
`tests/validation/parts_interconnection_2a.rs` (2 `PortDef`, 22 `PortUsage`, including two nested
inside another `PortUsage`'s body that a mechanical brace-matching pass initially missed and were
fixed by hand). `PARSE_AST_VERSION` bumped 21 -> 22. Added six regression tests locking in
visibility capture (`private`/`protected`/`public` and the no-prefix default) for both `port_def`
and `port_usage` (`src/parser/port.rs`). Regenerated the `functional_allocation_4a` validation
snapshot whose `Debug` output changed shape from the new field. All three gates green: `cargo test`,
`cargo clippy -- -W clippy::all` (same pre-existing warnings only, zero new), and the full
`SYSML_V2_RELEASE_DIR` validation gate (25/25).

Updates the scope-boundary list: `PortDef`/`PortUsage` are no longer in the "not yet covered" set.
Still not covered: `ConnectionDef`/`ConnectionUsageMember`, `ItemDef`/`ItemUsage`,
`RequirementUsage`-family, `ActionDef`/`ActionUsage`, etc., and `AliasDef`/`Import` -- same reasoning
as before.

### Changed: first-class `Membership` node extended to `PartDef`/`PartUsage`

Continues the "first-class `Membership` node" item above from `AttributeDef`/`AttributeUsage` to
`PartDef`/`PartUsage`, following the exact mechanical shape that entry's scope-boundary note
predicted: added the `membership: Membership` field (`OwningMembership` on `PartDef`,
`FeatureMembership` on `PartUsage`), captured via `lex::visibility_prefix` (`src/parser/part/def.rs`,
`src/parser/part/prelude.rs`, `src/parser/part/usage.rs`).

**Found a genuine parsing gap, not just discarded data, while doing this**: unlike
`attribute_def`/`attribute_usage`, `part_def` never parsed a `private`/`protected`/`public` prefix
at all. Confirmed by probing `package P { private part def Foo; }` against the pre-change parser: it
fell through to `ExtendedLibraryDecl` recovery instead of parsing as a visibility-prefixed
`PartDef`. The BNF (`DefinitionMember : OwningMembership = MemberPrefix ownedRelatedElement +=
DefinitionElement`) legally permits a visibility prefix before any definition, including
`PartDefinition`, so this is now fixed as part of wiring `Membership` in, not scoped out.

Fixed all 17 downstream `PartDef`/`PartUsage` struct-literal construction sites in
`tests/validation/parts_interconnection_2a.rs` (a hand-built expected-AST fixture) that needed the
new field. `PARSE_AST_VERSION` bumped 20 → 21. Regenerated the `parts_tree_1a` and
`functional_allocation_4a` validation snapshots for the new field's `Debug` shape. All three gates
green: `cargo test`, `cargo clippy -- -W clippy::all` (zero new warnings), and the full
`SYSML_V2_RELEASE_DIR` validation gate (25/25, including the full/systems library suites).

Updates the previous entry's scope-boundary list: `PartDef`/`PartUsage` are no longer in the "not
yet covered" set. Still not covered: `PortDef`/`PortUsage`, `ConnectionDef`/`ConnectionUsage`,
`ItemDef`/`ItemUsage`, `RequirementUsage`-family, `ActionDef`/`ActionUsage`, etc., and
`AliasDef`/`Import` — same reasoning as before.

### Changed: typed `FeatureValue` on `AttributeDef`/`AttributeUsage`/`PartUsage`/`RefDecl`

Closes gaps-doc item 1 ("Parser work still required" backlog, post-PAR-006). `value:
Option<Node<Expression>>` on these four structs discarded which of the BNF `FeatureValue`
production's five legal forms had actually matched -- bare `= expr` (bind), bare `:= expr`
(assign), `default = expr`, `default := expr`, and bare `default expr` -- even though
`value_part` (`attribute.rs`) and `usage_value_part` (`part/usage.rs`) already syntactically
distinguished all five via `alt()` before calling `expression()` and throwing that information
away. The field is now `Option<Node<FeatureValue>>`, where a new `FeatureValue`
(`src/ast/feature_value.rs`) carries `kind: FeatureValueKind` (`Bind` for `=`, `Assign` for `:=`;
bare `default expr` is `Bind`, matching `=`'s semantics), `is_default: bool` (independent of
`kind` -- `default =`, `default :=`, and bare `default expr` all set this), and the wrapped
`expression: Node<Expression>`.

The two near-duplicate parsers were collapsed into one shared `feature_value_part`
(`src/parser/feature_value.rs`), used by both `attribute.rs` and `part::usage`. A few other call
sites construct a `RefDecl`/`AttributeDef`/`AttributeUsage`'s `value` from a bare expression parsed
by a *different*, `=`-only grammar production (the `subsets target = expr` shorthand's optional
value, and the ad hoc `ref` value parses in `action.rs`/`state.rs`/`part/usage.rs`'s
`part_ref_usage` that predate this shared type): these now go through a new
`wrap_bind_expression` helper that packages the bare expression as a non-`default`
`FeatureValueKind::Bind` `FeatureValue`, since `=` is the only operator those productions ever
accepted.

Fixed a real, previously-undetected parsing bug found while writing this item's regression test:
`usage::optional_typings` treated any `:`-prefixed lookahead as the start of a `: Type` typing
clause unless it was `:>` or `:>>`, but never excluded `:=` -- so `attribute foo := 1;` inside a
usage context (e.g. a part definition body) mis-parsed the `:` of `:=` as a typing colon, failed
to find a valid type name after it, and the whole attribute usage fell through to error recovery.
`optional_typings` now also excludes `:=` from that lookahead, matching its existing `:>`/`:>>`
exclusions.

`PARSE_AST_VERSION` bumped 16 -> 17 for the breaking AST-schema change. Added a regression test
covering all five `FeatureValue` forms across all four in-scope structs (`tests/parser/
structure.rs`), including the `:=`-in-part-def-body fix above. Regenerated the `parts_tree_1a`
validation snapshot whose `Debug` output changed shape from this field-type change; full
`cargo test`, `cargo clippy -- -W clippy::all` (zero new warnings/errors), and the
`SYSML_V2_RELEASE_DIR` validation gate (including full/systems library suites) are green.

### Changed: structured relationship targets on `TypingRelationship`/`SubsettingRelationship`

Closes gaps-doc item 2 ("Parser work still required" backlog, post-PAR-006).
`TypingRelationship::target` and `SubsettingRelationship::target` were plain `String`s built by
two lossy joins: `.`-dotted feature-chain segments and `::`-qualified namespace/type segments were
joined into one string with no way to tell them apart afterward (`specialization_target` in
`usage.rs`), and comma-separated multi-target clauses (e.g. `:> Base, Other`) were joined with
`", "`, losing the fact there were multiple distinct targets (`specialization_targets`). Both
fields are now `Vec<Node<RelationshipTarget>>`, where a new `RelationshipTarget` (`src/ast/
relationship_target.rs`) holds an ordered `Vec<RelationshipTargetSegment>`, each segment carrying
its own `name` and an `Option<SegmentSeparator>` (`ColonColon` or `Dot`) recording how it joins to
the previous segment. `first_target()`/`target_display()` convenience methods on both relationship
structs keep the overwhelmingly common single-target case simple for existing callers that only
need the display string.

This is a new, narrow type deliberately kept separate from the existing `FeatureChain`
(`src/ast/feature_chain.rs`, wired into `Expression::FeatureChainRef` for expression-level dot-chain
parsing) -- `FeatureChain` intentionally excludes `::`-qualification and other expression-postfix
concerns, and widening it would have pulled relationship-target parsing into `expr.rs`'s complexity
for no benefit.

Parser changes: `usage.rs`'s `specialization_target`/`specialization_targets` (subsetting-family
`:>`/`:>>`/`::>`/`=>` targets, dot-chains allowed), `typings`/`conjugated_qualified_name` (`:`/
`defined by` targets, no dot-chains), and `specialization.rs`'s
`parse_optional_definition_specialization`/`typing_target_from_header`/
`specializes_from_header_text` (definition-level `:>`/`specializes` and the package-level bare
`: Type` header fallback from the entry above) all now build `RelationshipTarget` segments via a
new `lex::qualified_name_segments` instead of losing structure through `qualified_name`'s joined
`String`. Every `TypingRelationship`/`SubsettingRelationship` construction site from the prior
PAR-004 work (`attribute.rs`, `part/body.rs`) was updated to match. Plain `type_name: String`
display fields on usage structs (e.g. `PartUsage.type_name`, unrelated to these two relationship
types) keep their prior joined-string behavior via a new `targets_display_string` helper, so this
change is AST-shape-only for those fields -- no parsing behavior changed for them.
`PARSE_AST_VERSION` bumped 15 -> 16 for the breaking AST-schema change.

Added regression tests locking in that a multi-target `:>` clause stays as two distinct
`RelationshipTarget`s rather than one joined string, and that `Vehicle::mass.value`'s `::` and `.`
joins stay distinguishable in the segment list (`src/parser/usage.rs`). Regenerated the two
`sysml-v2-release` validation snapshots (`parts_tree_1a`, `functional_allocation_4a`) whose
`Debug` output changed shape from this field-type change; both were confirmed to still parse
correctly against the real Systems Library and Full Library gates
(`SYSML_V2_RELEASE_DIR=... cargo test --test validation -- --include-ignored`).

### Fixed: package-level bare `: Type` header silently dropped the type reference

Found while scoping PAR-002 work (flagged there as out of scope, fixed separately): the shared
`parse_definition_prefix`/`parse_optional_definition_header_after_identification` header parsing
used by the package-level, `def`-optional `port`, `item`, `connection`, and `constraint`/`calc`
def parsers (kept `DefKeywordMode::Optional` per `definition_prefix.rs`'s module doc, to accept
the real Systems Library's bare `def`-less namespace-level forms) captured a `:>`/`specializes`
clause when present but otherwise discarded the entire `: Type` header text once
`specializes_from_header_text` found no `:>`, with nothing downstream to catch the loss. A
package-level declaration with only a plain type, e.g. `port p1: MyPortType;`, parsed successfully
but produced `PortDef { specializes: None, .. }` -- the `MyPortType` reference vanished with no
diagnostic.

`specialization.rs::parse_optional_definition_header_after_identification` now falls back to
parsing the bare type as a `Typing`-kind `TypingRelationship` (via a new `typing_target_from_header`
helper reusing the existing `qualified_name` lexer, handling the `~`-conjugated form too) when no
`:>`/`specializes` clause is present, instead of returning `None`. This reuses the existing
`specializes: Option<Node<TypingRelationship>>` field already on `PortDef`/`ItemDef`/
`ConnectionDef`/etc. -- `TypingRelationship::kind` already distinguished `Typing` from
`Subclassification` (PAR-004 item 1) but nothing previously populated a package-level `Typing`
variant through this path, so no AST schema change or `PARSE_AST_VERSION` bump was needed. The
existing combined library-shorthand form (`: Type[mult] nonunique :> Base`) is unaffected: when a
`:>`/`specializes` clause is present it still wins, matching prior (tested) behavior.

### Changed: modifier-completeness audit -- `derived`/`constant`/direction swept onto `PartUsage`/`PortUsage`, `end` added

Closes gaps-doc item 3 ("Parser work still required" backlog, post-PAR-006), a completeness audit
of eight usage-prefix modifier concepts against `sysml-v2-release/bnf/*.kebnf`. Re-verified each
finding against the current codebase (post Items 1/2) before writing any code:

- **`unique`/`readonly`**: confirmed no textual grammar production anywhere in
  `sysml-v2-release/bnf/*.kebnf` (only `nonunique` exists as a real keyword; `readonly` appears
  only in the separate graphical-notation BNF, `SysML-graphical-bnf.kgbnf`, not the textual one).
  Out of scope, matching the PAR-003b precedent of not inventing fields with no textual syntax.
- **`variable`**: re-checked the grammar directly -- no `variable`/`'var'` keyword production
  exists anywhere in `SysML-textual-bnf.kebnf`. KerML's own `EndFeaturePrefix`/`BasicFeaturePrefix`
  productions (`KerML-textual-bnf.kebnf`) do have `isVariable ?= 'var'`/`'const' { isVariable =
  true }`, but this parser targets the SysML textual notation (`SysML-textual-bnf.kebnf`'s
  `RefPrefix` uses `'derived'`/`'constant'` instead, already typed as `is_derived`/`is_constant`),
  not KerML's textual notation. Out of scope; no field added.
- **Reference/composite ownership**: confirmed `RefDecl` already structurally captures this
  distinction -- every body enum that can contain a `ref`-declared feature carries a dedicated
  `Ref(Node<RefDecl>)` (or `RefDecl(Node<RefDecl>)`) variant, populated by a separate `ref`-keyword
  parser path (`part_ref_usage`, `action_ref_decl`, `connection.rs`/`interface.rs`/`state.rs`'s
  `ref_decl`/`state_ref`), coexisting alongside the ordinary `PartUsage`/`ItemUsage`/etc. member
  variants in the same enums. Which variant a member parses into already *is* the reference-vs-
  composite distinction; the BNF's only `isComposite` production is on unrelated action control-node
  keywords (`merge`/`decide`/`join`/`fork`), not feature ownership. Not a gap; no field added.
- **`derived`/`constant`**: were typed as `is_derived`/`is_constant: bool` on `AttributeUsage` only.
  BNF `RefPrefix` (§8.2.2.6.2) is reachable through `OccurrenceUsagePrefix -> BasicUsagePrefix ->
  RefPrefix` for every occurrence-based usage kind, confirmed for `PartUsage`
  (`PartUsage = OccurrenceUsagePrefix 'part' Usage`) and `PortUsage`
  (`PortUsage = OccurrenceUsagePrefix 'port' Usage`) specifically. Added `is_derived`/`is_constant:
  bool` to both, parsed via the same `derived`/`constant` keyword-prefix pattern `AttributeUsage`
  already used (`part/usage.rs::part_usage`, `port.rs::port_usage`). `ItemUsage` already had
  `direction: Option<InOut>` from a prior increment and needed no change.
- **`direction`**: swept `Option<InOut>` onto `PartUsage`/`PortUsage` alongside `derived`/`constant`
  above (same `RefPrefix` production carries all three), reusing the existing
  `attribute::direction_prefix` parser. `AttributeUsage`, `PerformInOutBinding`, `ItemUsage`, and
  the `RequirementUsage`-family already had it from prior increments.
- **`end`** (`isEnd ?= 'end'`): confirmed a genuine gap. BNF `UnextendedUsagePrefix : Usage =
  EndUsagePrefix | BasicUsagePrefix` makes `end` and `RefPrefix`'s `derived`/`constant`/direction
  mutually exclusive alternatives (not combinable), reachable only through the full `UsagePrefix`
  production, used by exactly four usage kinds: `AttributeUsage`, `EnumerationUsage`,
  `BindingConnectorAsUsage`, `SuccessionAsUsage`. Added `is_end: bool` to `AttributeUsage`
  (`structure.rs`) and `EnumerationUsage` (`requirement.rs`), parsed in `attribute_usage` and
  `enum_usage` respectively as a mutually-exclusive alternative to the `derived`/`constant` prefix
  (matching the BNF's `isEnd` XOR `RefPrefix` structure). Left `Bind`/`SuccessionUsage` (the AST
  types for `BindingConnectorAsUsage`/`SuccessionAsUsage`) unchanged: no occurrence of a
  keyword-prefixed `end bind`/`end succession`/anonymous `end`-prefixed succession was found
  anywhere in the Systems Library, Kernel Library, or example sources the validation gate exercises,
  and their prefix-parsing entry points are more structurally invasive to extend; flagged as a
  narrower follow-up if a real model ever needs it, rather than speculatively wired now.
  Separately confirmed this is unrelated to the existing `EndDecl`/`end_decl` construct
  (`connection.rs`/`interface.rs`), which models a different grammar production entirely (a
  named connector-end declaration, `end name : Type;` / `DefaultInterfaceEnd`) that this parser
  already handles.

`PARSE_AST_VERSION` bumped 17 -> 18 for the breaking AST-schema change (new fields on
`PartUsage`/`PortUsage`/`AttributeUsage`/`EnumerationUsage`). Added regression tests covering all
new fields and their defaults (`src/parser/attribute.rs`, `tests/parser/structure.rs`).
Regenerated the two `sysml-v2-release` validation snapshots (`parts_tree_1a`,
`functional_allocation_4a`) whose `Debug` output changed shape from the new fields; full
`cargo test`, `cargo clippy -- -W clippy::all` (zero new warnings), and the `SYSML_V2_RELEASE_DIR`
validation gate (including the full/systems library suites) are green.

### Changed: structured `AliasDef.target`

Closes gaps-doc item 4a ("Parser work still required" backlog, post-PAR-006). `AliasDef.target`
(the `for` clause of `alias m for ISQ::mass;`) had the same lossy-textual-target problem item 2
already fixed for `TypingRelationship`/`SubsettingRelationship`: it was a plain `String` built by
`qualified_name`'s `::`-join, with no span and no way to distinguish `::`-qualified segments from
one another once joined. It's now a single `RelationshipTarget`
(`src/ast/relationship_target.rs`, introduced by item 2), holding the same ordered
`Vec<RelationshipTargetSegment>`/`Option<SegmentSeparator>` shape plus its own span. Unlike
`TypingRelationship`/`SubsettingRelationship::target`, this is a bare `RelationshipTarget`, not a
`Vec<Node<RelationshipTarget>>` -- an alias target is always exactly one qualified name per the
BNF's `memberElement = [QualifiedName]` (no comma-separated multi-target concept exists for
`alias ... for`), so the plural wrapper item 2 introduced for `:`/`:>` clauses doesn't apply here.

`src/parser/alias.rs::alias_def` now builds the target via the existing `lex::qualified_name_segments`
(the same segment-building helper item 2's `specialization_target`/`conjugated_qualified_name`
already use) instead of the joined-`String` `qualified_name`, and records the target's own span
around just the qualified-name text (not the whole `alias ... ;` clause).

`PARSE_AST_VERSION` bumped 18 -> 19 for the breaking AST-schema change. Added regression tests
covering both a `::`-qualified alias target (`alias m for ISQ::mass;`) and a bare single-segment
name (`tests/parser/structure.rs`). Regenerated the `function_based_behavior_3a` validation
snapshot (the Systems Library's `alias Torque for ISQ::TorqueValue;` re-export shorthand) whose
`Debug` output changed shape from this field-type change; full `cargo test`,
`cargo clippy -- -W clippy::all` (zero new warnings), and the `SYSML_V2_RELEASE_DIR` validation
gate (including the full/systems library suites) are green.

### Changed: first-class `Membership` node (scoped start, `AttributeDef`/`AttributeUsage` only)

Closes gaps-doc item 4b ("Parser work still required" backlog, post-PAR-006) -- the confirmed-scope
"full architectural treatment" alternative rather than a minimal field patch, but landed as its own
multi-increment sub-effort per the plan, following the same "start narrow, extend as needed"
discipline PAR-004 used for `TypingRelationship`/`SubsettingRelationship`/`ConnectionEnd`/
`InterfaceEnd`. Every def/usage element previously lived as a bare, undifferentiated child in its
owning body's `Vec<Node<XBodyElement>>` -- ownership, visibility, and membership *kind* were not
represented independently of which enum variant the element happened to parse into, and an explicit
`private`/`protected`/`public` prefix (matched by `attribute_def`/`attribute_usage` for years, per
this crate's history with `DefinitionPrefixOptions::with_private()`-style prefix parsing) was
consumed and thrown away rather than captured anywhere.

New types (`src/ast/membership.rs`): `MembershipKind` (`OwningMembership` -- a nested `*Def` that
becomes a new named member of its owning namespace; `FeatureMembership` -- a nested `*Usage` that
contributes a feature to its owning type; `Import` and `Alias`, reserved for the still-unmigrated
`Import`/`AliasDef` struct families noted below, not yet constructed by any parser) and
`Membership { kind, visibility: Option<Visibility>, span }`, reusing the existing `Visibility` enum
(`src/ast/common.rs`). `Membership::owning`/`Membership::feature` are convenience constructors for
the two kinds actually wired up this increment.

**Design decision -- direct field, not a generic wrapper.** Of the two shapes considered (wrap every
body-enum element in a generic `Member<T> { membership, element }`, vs. add a `membership: Membership`
field directly to the shared element structs), this lands the direct-field approach: wrapping every
`Vec<Node<XBodyElement>>` in the crate would have been the single largest mechanical change in this
entire backlog for uniformity most consumers don't need, whereas a direct field lets each struct
family be migrated independently and matches the deliberately incremental style every prior PAR/gaps
item in this backlog has used.

**Scope landed this increment: `AttributeDef` and `AttributeUsage` only** (`structure.rs`) -- chosen
as the highest-traffic starting family, matching Item 1/4a's own working area and PAR-004's precedent
of starting with the field shared by the most structs before extending outward. `AttributeDef` gets a
non-optional `membership: Membership` with `kind: OwningMembership`; `AttributeUsage` gets the same
with `kind: FeatureMembership`. Every one of `AttributeUsage`'s four construction sites in
`attribute.rs` (`attribute_usage` -- the primary parser, now the only one that actually parses a
visibility prefix; `attribute_feature_binding`, `metadata_binding`, and `attribute_usage_shorthand`
-- three ad hoc shapes with no visibility syntax of their own) now build a `Membership`, the latter
three always with `visibility: None` since their grammar productions have no visibility prefix to
capture. `attribute_def`/`attribute_usage`'s inline `alt((tag("private"), tag("protected"),
tag("public")))` visibility-matching (previously duplicated in each function, and in `filter_member`/
`import`/`package.rs`) was consolidated into a new shared `lex::visibility_prefix` helper returning
`(Span, Option<Visibility>)`, used by both.

**Explicitly NOT covered by this increment** -- documented here as the scope boundary for a future
continuation, per this backlog's "deviation noted, scoped explicitly" precedent:
- Every other member-bearing `*Def`/`*Usage` struct family (`PortDef`/
  `PortUsage`, `ConnectionDef`/`ConnectionUsage`, `ItemDef`/`ItemUsage`, `RequirementUsage`-family,
  `ActionDef`/`ActionUsage`, etc.) -- none of these got a `membership` field yet as of this
  increment (`PartDef`/`PartUsage` were extended in the very next entry above). Their body-enum
  parsers largely already have the same discarded `private`/`protected`/`public`-prefix pattern
  `attribute_def`/`attribute_usage` had (many share `DefinitionPrefixOptions`/ad hoc `alt()` prefixes
  the same way), so the mechanical shape of the follow-up is expected to closely mirror this
  increment's `AttributeDef`/`AttributeUsage` work -- add the field, call `lex::visibility_prefix` at
  the same point the def/usage prefix is currently matched-and-discarded, wire the `Membership::owning`/
  `Membership::feature` constructor at each construction site, fix the `src/ast/mod.rs` normalize-match
  ripple, regenerate any validation snapshots whose `Debug` output shape changes.
- `AliasDef`/`Import` -- `MembershipKind::Alias`/`MembershipKind::Import` variants exist (since the
  gaps doc explicitly calls out aliasing as itself a membership form) but nothing constructs them yet;
  `Import` already has its own `visibility: Option<Visibility>` field from before this item, so wiring
  a `Membership` there would need to either replace or wrap that existing field -- left as a follow-up
  design decision rather than guessed at here.
- The generic `Member<T>` wrapper alternative was not built at all (see the design decision above);
  if a future increment finds the per-struct-family approach doesn't scale (e.g. some body-enum
  `Other`/`Error`/`Doc` variant genuinely needs membership info), reconsider then rather than building
  the wrapper speculatively now.
- No new `MembershipKind` variants beyond the four above were added speculatively; per this backlog's
  `unique`/`readonly`/`variable` out-of-scope precedent (CHANGELOG 0.35.0's item 3 entry), further
  kinds (e.g. a dedicated parameter/variant/return-parameter membership) should only be added once a
  real parser site needs to distinguish them.

`PARSE_AST_VERSION` bumped 19 -> 20 for the breaking AST-schema change (new non-optional field on
`AttributeDef`/`AttributeUsage`). Added four regression tests locking in visibility capture (`private`/
`protected`/`public` prefixes and the no-prefix default) and the `OwningMembership`/`FeatureMembership`
kind split for `attribute_def`/`attribute_usage` (`src/parser/attribute.rs`). Regenerated the two
`sysml-v2-release` validation snapshots (`parts_tree_1a`, `function_based_behavior_3a`) whose `Debug`
output changed shape from the new field; full `cargo test`, `cargo clippy -- -W clippy::all` (the same
5 pre-existing `large_enum_variant`/`type_complexity` warnings as before this change, confirmed via a
throwaway `git worktree` checkout of the pre-change commit -- zero *new* warnings), and the
`SYSML_V2_RELEASE_DIR` validation gate (including the full/systems library suites) are green.

## [0.35.0] - 2026-07-15

Closes the gaps-doc PAR-002..006 backlog (definitions/usages in every owning context, typed
declaration modifiers, typed relationship AST nodes, complete expression AST, non-semantic
recovery), on top of 0.34.0's PAR-001 fix. Along the way this also found and fixed 7 real parser
bugs beyond the original scope of each PAR item — 5 def/usage-ambiguity bugs in `flow`, `port`,
`calc`, and `connection` (×2 contexts) parsers that lacked the PAR-001-style `def_required()`
guard, and one case where named invocation arguments (`F(x = a)`, legal SysML v2 syntax) had no
parse path at all and silently dropped the enclosing declaration into recovery. `PARSE_AST_VERSION`
moved from 8 (at 0.34.0) to 15 across this backlog's many breaking AST-schema changes; see the
individual entries below for what changed and why.

### PAR-006b: final disambiguation/recovery audit

Closing audit pass for PAR-006 (make recovery non-semantic), on top of the foundation
`def_required()` documentation landed as PAR-006a. Systematically grepped every `alt(...)`
body-element dispatch site across `src/parser/*.rs` and `src/parser/part/*.rs` for `*_def`/
`*_usage` pairs sharing a keyword, and every `Other`/opaque-capture recovery variant, to confirm
none silently misclassify or silently swallow input.

- **Investigated a possible new PAR-001-class gap, found and documented a real (but different)
  issue**: `package.rs::try_package_body_structure` dispatches `connection_def` then
  `connection_usage_member` at package level, and `connection_def`'s own doc comment claimed
  "nothing else shares the `connection` keyword" at package level -- stale as of PAR-002, which
  added that `connection_usage_member` dispatch. Tried making `connection_def` require `def`
  unless hash-annotated (new `DefKeywordMode::RequiredUnlessAnnotated` in
  `definition_prefix.rs`, since reverted) to close the apparent gap. This broke the
  `SYSML_V2_RELEASE_DIR` gate (`test_systems_library_node_types_no_extended`/
  `test_full_library_node_types_no_extended`): the real Systems Library uses bare, `def`-less
  `connection` declarations with `abstract`/multiplicity/`nonunique`/leading `:>` subsets (e.g.
  `abstract connection connections: Connection[0..*] nonunique :> linkObjects, parts { ... }` in
  `Systems Library/Connections.sysml`) that `connection_usage_member`'s narrower grammar can't
  parse, so they fell all the way through to `ExtendedLibraryDecl` instead of `ConnectionDef` --
  worse than the status quo, not a fix. Root cause: `connection_def`'s generic header parsing
  (`parse_optional_definition_header_after_identification`, a text-scan for `: Type[mult]
  nonunique :> target`) is already a grammar superset of `connection_usage_member` for every
  practical package-level input, so `connection_usage_member`'s package-level dispatch arm is a
  narrow fallback, not a competing classification -- there is no live misclassification bug.
  Documented this finding on `connection.rs::connection_def` and in `definition_prefix.rs`'s
  module doc instead of forcing a guard; same precedent class as the `port`/`calc`/`constraint`
  "tried and reverted" note from CHANGELOG 0.33.0.
- **Confirmed every other `def_required()` site already covers its ambiguous pair**: reviewed
  `action`, `allocation`, `case`/`analysis`/`verification`/use-case, `enum`, `flow`,
  `individual`, `interface`, `item`, `metadata`, `occurrence`, `port` (nested-body via
  `port_def_required`), `requirement`, `state`, and `view`/`viewpoint`/`rendering` dispatch sites
  across `action.rs`, `connection.rs`, `interface.rs`, `part/body.rs`, `part/usage.rs`,
  `package.rs`, `port.rs`, `requirement.rs`, `state.rs`, `usecase.rs`, `view.rs` -- all correctly
  guarded, none missing the pattern documented in `definition_prefix.rs`. `attribute_def`'s
  bespoke `disambiguate_from_usage` parameter (the original PAR-001 mechanism, predates
  `def_required()`) is likewise applied everywhere `attribute_usage` is a dispatch sibling.
- **Confirmed recovery `Other`/opaque-capture variants are a consistent, bounded pattern, not
  silent catch-alls**: `RequirementDefBodyElement::Other`, `UseCaseDefBodyElement::Other`,
  `StateDefBodyElement::Other`, `CalcDefBodyElement::Other`/`ConstraintDefBodyElement::Other`,
  `ViewDefBodyElement::Other`/`ViewBodyElement::Other`, and `PartDefBodyElement::Other` all use
  the same gate: `build_recovery_error_node`'s diagnostic classifier decides whether content
  looks like a genuine syntax error (known diagnostic codes like `missing_member_name`/
  `missing_type_reference`/`unexpected_keyword_in_scope` -> `Error(ParseErrorNode)` with a real
  diagnostic) versus an unrecognized-but-plausible, not-yet-modeled library construct (falls
  through to `Other(preview)`, a bounded text snippet, not an unbounded silent accept). This is
  the same design already reviewed once by PAR-006a (which found and removed the one genuinely
  dead/unreachable `Other` variant, `PortBodyElement::Other`) -- no further dead code or silent
  catch-alls found this pass.
- Added a regression test in `src/parser/connection.rs`
  (`connection_def_accepts_the_bare_abstract_multiplicity_nonunique_subsets_form_that_makes_
  def_required_unsafe`) that locks in the specific real-library shape
  (`abstract connection connections: Connection[0..*] nonunique :> linkObjects, parts { ... }`)
  that made the `def_required_unless_annotated` attempt above unsafe, so a future attempt to
  tighten `connection_def` gets an immediate, specific failure pointing back at this note instead
  of only failing the much slower full-library gate.
- Gate: `cargo test` and `SYSML_V2_RELEASE_DIR` `cargo test --test validation --
  --include-ignored` both green. No functional parser change landed this increment -- the
  `def_required_unless_annotated` fix attempt was reverted once it broke the gate; only the audit
  documentation and the new regression test describing the audited, already-correct behavior were
  kept.

### PAR-005: complete the expression AST

Six items against `src/ast/core.rs`'s `Expression` enum and `src/parser/expr.rs`
(`PARSE_AST_VERSION` bumped 14 -> 15 for the schema change).

- **Item 1 (constructor expressions) + item 3 (feature-chain expressions), landed together**:
  `new Type(...)` is now `Expression::Constructor { type_name: String, args: Vec<Argument> }`
  instead of the synthetic `FeatureRef("new TypeName")` string. `type_name` stays a plain
  qualified-name `String` (consistent with `TypeCheck`/`MetaCast`/`Classification`, which already
  represent qualified type references this way -- a constructor names a *type*, not a dot
  feature-access path). Separately, `path_expression` (`bind`/`connect`/`allocate`/interface-
  connect endpoints) now produces `Expression::FeatureChainRef(FeatureChain)` for genuine
  multi-segment dotted chains (e.g. `engine.fuelCmdPort.flowRate`, `rearAxleAssembly.leftWheel
  .wheelToRoadPort` from `2a-Parts Interconnection.sysml`), adopting the standalone `FeatureChain`
  type PAR-004 item 6 built for exactly this and left unwired. A single segment stays
  `FeatureRef`. The general `expression()` grammar's postfix `.` chaining (ordinary value
  expressions, e.g. inside a calc/constraint body) intentionally still folds into nested
  `MemberAccess` -- it's interleaved with `(...)`/`#(...)`/`::`/`meta`/`->op(...)` postfix
  operators a pure feature chain doesn't carry, so widening it further is out of this item's scope.
  **Bug found and fixed along the way**: `FeatureChain` derived `PartialEq` including its `span`
  field, unlike every other span-bearing AST type in this crate (`Node<T>`, `Multiplicity`,
  `TypingRelationship` all have custom `PartialEq` that ignores span so hand-built expected ASTs
  in tests don't need real source spans). Left as derived, it broke the `2a-Parts Interconnection`
  validation fixture the moment a real span reached the field. Fixed with the same custom-impl
  convention (`src/ast/feature_chain.rs`).
- **Item 2 (collection operators)**: `base->op(...)` (`->collect`, `->select`, `->size()`,
  `->includes()`, etc.) is now `Expression::CollectionOp { op: CollectionOperator, base, args }`
  instead of desugaring into an untyped `MemberAccess` + `Invocation` pair. `CollectionOperator`
  covers the names actually seen in the SysML v2 release tree (`collect`, `select`, `selectOne`,
  `size`, `isEmpty`, `notEmpty`, `includes`, `including`, `excludes`, `excluding`, `excludingAt`,
  `excludingOnce`, `equals`, `forAll`, `exists`, `sum`, `sort`, `filter`, `reduce`) plus `Other
  (String)` so no arrow-invoked name is ever lost or misclassified. A bare `->name` with no
  trailing `(...)` (rare) still falls back to plain `MemberAccess`, matching prior behavior for
  that shape.
- **Item 5 (argument relationships), real bug found and fixed**: `ArgumentList` in the KerML BNF
  (8.2.5.8.3) is `PositionalArgumentList | NamedArgumentList` -- named arguments
  (`NAME '=' ArgumentValue`) are legal, real syntax, not just positional. Confirmed against actual
  Systems Library / example usage (`new RiskLevel(probability = LevelEnum::low)` in
  `RiskMetadata.sysml`; `F(q = 1, p = a)` in `ParameterTest.sysml`; `new IgnitionCmd
  (ignitionOnOff=IgnitionOnOff::on)` in the Simple Vehicle Model). Before this change, `expression()`
  had no way to parse `name = value` inside `(...)` (`=` isn't a binary operator token), so every
  one of these real constructs silently fell into parser recovery for the entire enclosing
  declaration -- a data-loss bug, not just a missing-feature gap. Fixed by giving
  `Expression::Invocation`/`Expression::Constructor`/`Expression::CollectionOp` a shared
  `args: Vec<Argument>` (`Argument { name: Option<String>, value: Node<Expression> }`) and a new
  `argument_list_tail` parser that tries `NAME '='` (rejecting `==`/`===` so equality expressions
  are never misread as a named argument) before falling back to a positional `expression()`.
  Return-parameter relationships (`ReturnRef.return_expression`, `src/ast/requirement.rs`) stay
  scoped to requirement/verification bodies as before -- generalizing a return-parameter concept
  onto every invocation is a materially larger design change, out of this item's scope.
- **Item 4 (metadata access), real grammar gap, not previously covered by `Classification`/
  `MetaCast`**: checked `KerML-textual-bnf.kebnf` before assuming a gap existed (per the PAR-003b
  precedent of not inventing nodes for non-existent productions) and found
  `MetadataAccessExpression = ownedRelationship += ElementReferenceMember '.' 'metadata'` is a
  real, separate production -- distinct from `@Metaclass` (`Classification`, tests classification)
  and `expr meta Metaclass` (`MetaCast`, reflective cast). Added `Expression::MetadataAccess
  (Box<Node<Expression>>)`, parsed as a `.metadata` postfix suffix (literal keyword, not a
  general member name) in `postfix()`.
- **Item 6 (parenthesized marker)**: `Expression::Parenthesized(Box<Node<Expression>>)`. Chose the
  wrapping-variant design over a bool flag threaded onto every variant since the blast radius was
  small and contained (only `parenthesized()` in `expr.rs` produces it, and the two exhaustive
  matches on `Expression` in the whole crate -- `src/ast/mod.rs::normalize_expression_node` and
  `expr.rs` itself -- were the only places needing a new arm). `Expression::Tuple` (multi-element
  parenthesized sequences) doesn't get this wrapper -- it's inherently only expressible with
  parens, so there's nothing extra to mark.
- **All six items ripple-checked against the two exhaustive `Expression` matches in the crate**
  (`src/ast/mod.rs::normalize_expression_node`, `src/parser/expr.rs`'s own tests) -- both are the
  full extent of exhaustive matching on `Expression` outside `expr.rs`; the handful of other
  `Expression::` references across the crate (`action.rs`, `bnf_surface.rs`, `payload.rs`,
  `requirement.rs`) only construct or single-arm-match specific variants and needed no changes.
- New public exports: `Argument`, `CollectionOperator`, `FeatureChain` added to the crate root
  `pub use` list in `src/lib.rs` (previously `FeatureChain` wasn't re-exported at all, since
  PAR-004 built it but deliberately left it unwired).

### PAR-003b (item 1 of 4): typed `ordered`/`nonunique`/`derived`/`constant` on attributes

- **`AttributeDef`/`AttributeUsage` gain typed `ordered: bool` / `nonunique: bool` fields**: the
  `MultiplicityPart` modifiers (BNF §8.2.2.6.6) were previously consumed and thrown away by
  `ignored_feature_modifiers` at all 8 call sites across `src/parser/attribute.rs` (attribute def,
  attribute usage, the `:>>`/`:>` feature-binding shape, and the metadata binding shape). Replaced
  with `feature_modifiers`, which returns a small `FeatureModifiers { ordered, nonunique }` struct
  that callers OR-merge across the (up to three) positions a modifier can legally appear in a
  single declaration, so a later empty match can't silently clear an earlier `true`. Confirmed via
  the validation gate that the Systems Library fixtures actually exercise `ordered` (4 real
  occurrences surfaced in `tests/validation/snapshots/parts_tree_1a.txt` once regenerated).
- **`AttributeUsage` gains typed `is_derived: bool` / `is_constant: bool`**: `attribute_usage` now
  parses the `derived`/`constant` keywords from `RefPrefix` (BNF §8.2.2.6.2) before the `attribute`
  keyword. These are usage-only per the grammar -- `AttributeDefinition` uses `DefinitionPrefix`,
  which has no `derived`/`constant` production, so `AttributeDef` does not get these fields.
- **Scope judgment -- `readonly`, `variable`, `sufficient` are not textual keywords**: checked the
  reserved-word list and every relevant production in `sysml-v2-release/bnf/SysML-textual-bnf.kebnf`
  (and the KerML grammar it builds on). `readonly` never appears as a keyword anywhere in the SysML
  or KerML textual grammars -- `Feature::isReadOnly` is a computed/semantic property, not something
  a `def`/`derived`/`constant`-style prefix keyword expresses syntactically, so there is nothing
  for this parser to capture. `variable` likewise has no SysML textual production (KerML's
  `'var'`/`'const'` pair only appears in an unrelated parameter-declaration context that SysML
  doesn't use). `sufficient` corresponds to KerML's `all` keyword on `ClassifierDeclaration`, but
  `DefinitionDeclaration` in the SysML grammar (`Identification SubclassificationPart?`) never
  includes it -- SysML defs have no textual "sufficient" marker. All three are out of this parser's
  scope; noted here rather than inventing fields with no parseable syntax behind them.

### PAR-003b (items 3-4): effective name and source-range assessment (no code change)

- **"Effective name" is out of parser scope, confirmed by design, not by omission**: KerML defines
  `Element::effectiveName` as the name a feature *without its own declared name* inherits from what
  it redefines -- resolving it requires walking the redefinition/conjugation chain across
  potentially other files/packages, which is exactly the kind of cross-reference resolution this
  crate deliberately does not do (it is a syntax parser producing one file's AST, not a resolver).
  `Identification` (`src/ast/common.rs`) already exposes the two concepts that *are* syntactic --
  `name` and `short_name` -- and that's the full extent of what a single parse can determine.
  Recorded here explicitly per the task's own guidance not to force invented scope.
- **Source ranges for `subsets`/`references`/`crosses` targets: already closed by PAR-004**.
  `SubsettingRelationship` (`src/ast/core.rs`) has carried a real `span` field (covering the
  operator/keyword through the target) since PAR-004 item 2, and `references`/`crosses` on
  `AttributeUsage` are typed as `Option<Node<SubsettingRelationship>>`, so they inherit the same
  span coverage as `subsets`. No further work needed for relationship-target spans.
- **Keyword-only spans (`abstract`, `ordered`, `direction`, etc.) intentionally left as bools**:
  these are captured as plain `bool`/enum fields (e.g. `PartUsage.ordered: bool`,
  `AttributeUsage.direction: Option<InOut>`) with no separate span for the keyword token itself.
  Judgment call: adding a dedicated `Span` next to every single-keyword flag across the AST would
  be a wide, low-value restructuring (these tokens are typically one word, immediately adjacent to
  already-spanned constructs, and no consumer has asked for standalone keyword highlighting) for
  marginal value versus the relationship/target spans that matter for semantic tokens and
  diagnostics. Left as-is; flagging here rather than silently skipping.

### PAR-002 (increment 1 of N): nested `def` kinds in `PartDefBodyElement`

- **Added `StateDef`, `MetadataDef`, `MetadataUsage`, `FlowDef`, `RequirementDef`,
  `OccurrenceDef` to `PartDefBodyElement`** (`src/ast/structure.rs`), wired into
  `part_def_body_element` (`src/parser/part/body.rs`) using the existing standalone
  `state_def`/`metadata_def`/`metadata_usage`/`flow_def`/`requirement_def`/`occurrence_def`
  parsers -- no new parsers needed, these already existed and were already reachable at package
  level (`PackageBodyElement` already had all six), just not nested inside a part definition body.
  Before this, `state def`, `metadata def`, `flow def`, `requirement def`, and `occurrence def`
  written inside a `part def { ... }` body could only be reached indirectly (`exhibit state` for
  state, or not at all for the others) or fell through to `Other`/`Error`.
- **`def_required()` reuse, no new guard needed**: `state_def`, `requirement_def`, and
  `occurrence_def` already call `DefinitionPrefixOptions::def_required()` internally (per
  PAR-006a), so wiring them in required no extra disambiguation work -- a bare (`def`-less)
  declaration always still falls through to the sibling usage arm.
- **Found and fixed a real ambiguity bug while wiring `flow_def`**: `flow_usage_member` (used for
  bare `FlowUsage` dispatch) has no guard against the `def` keyword and was misparsing `flow def
  DataFlow;` as `FlowUsage { name: "def" }` when tried first. Fixed by reordering `flow_def` ahead
  of `flow_usage_member` in `part_def_body_element`'s `alt`, matching the order package-level
  dispatch (`try_package_body_structure` in `src/parser/package.rs`) already used. Caught by the
  new `part_def_body_accepts_nested_flow_def` test before it could regress silently -- exactly the
  PAR-001-class bug this backlog's process discipline is meant to catch.
- **PAR-002 acceptance-criterion tests added**: `state_def_is_same_variant_kind_at_package_level_and_nested_in_part`
  and `requirement_def_is_same_variant_kind_at_package_level_and_nested_in_part`
  (`src/parser/part/body.rs`) parse the same snippet at package level and nested in a part body,
  asserting the same AST variant kind both times.
- **Scope remaining for `PartDefBodyElement`** (not done in this increment, see gaps doc /
  PAR-002 task description): `ConnectionDef` (only `ConnectionUsageMember`, a usage shape, exists
  today), `AllocationDef`/`AllocationUsage`, `ViewDef`/`ViewUsage`, `ViewpointDef`/`ViewpointUsage`,
  `RenderingDef`/`RenderingUsage`, `CaseDef`/`CaseUsage`, `UseCaseDef`/`UseCaseUsage`,
  `AnalysisCaseDef`/`AnalysisCaseUsage`, `VerificationCaseDef`/`VerificationCaseUsage`, `PortDef`
  (has `PortUsage` only -- needs a new `port_def_required()` variant since the standalone `port_def`
  deliberately keeps `def` optional, see its own doc comment, so it cannot be wired in as-is without
  reintroducing a PAR-001-class ambiguity against `port_usage`), `CalcDef` (same issue: `calc_def`
  deliberately keeps `def` optional at namespace level, needs a `calc_def_required()` variant before
  it can be safely nested). `PartUsageBodyElement` (zero Def-kind variants), `PortDefBodyElement`/
  `PortBodyElement`, `InterfaceDefBodyElement`/`ConnectionDefBodyElement` are still unaddressed.
  `PackageBodyElement`'s own gaps (standalone `AttributeUsage`/`ItemUsage`/`PortUsage`,
  `ConnectionUsage`, `RefDecl` usage, `EnumerationUsage`) are also still unaddressed.

### PAR-002 (increment 2 of N): remaining `PartDefBodyElement` def/usage pairs

- **Added `ConnectionDef`, `PortDef`, `CalcDef`, `AllocationDef`/`AllocationUsage`,
  `ViewDef`/`ViewUsage`, `ViewpointDef`/`ViewpointUsage`, `RenderingDef`/`RenderingUsage`,
  `CaseDef`/`CaseUsage`, `UseCaseDef`/`UseCaseUsage`, `AnalysisCaseDef`/`AnalysisCaseUsage`,
  `VerificationCaseDef`/`VerificationCaseUsage` to `PartDefBodyElement`** (`src/ast/structure.rs`),
  wired into `part_def_body_element` (`src/parser/part/body.rs`).
- **Built three new `_required()` def parsers before wiring, per the flagged risk**:
  `port_def_required` (`src/parser/port.rs`), `calc_def_required` (`src/parser/constraint.rs`),
  and `connection_def_required` (`src/parser/connection.rs`), each a thin wrapper around a shared
  `parse_*_def(input, options)` helper that the existing `port_def`/`calc_def`/`connection_def`
  (still `def`-optional, unchanged, still used at package level) now also delegate to. Mirrors
  `interface_def_required`/`item_def_required` in shape. `connection_def_required` intentionally
  does not support the hash-annotation def-less form that `connection_def` does -- nothing in the
  nested-part-body grammar needs that combination today.
- **Found and fixed three more real ambiguity bugs of the same class as `flow_def` in increment
  1**, each caught by a `_not_misparsed_as_*` unit test before landing: `port_usage`,
  `calc_usage`, and `connection_usage_member` all call a bare `name`/`identification` parse
  immediately after their keyword with no guard against the literal token `def`, so stacking the
  new `_def_required` parsers *after* them (the naive wiring) would have made `port def Foo;`,
  `calc def Foo;`, and `connection def Foo;` silently misparse as usages named `"def"`. Fixed by
  ordering every new def parser *before* its usage sibling in `part_def_body_element`'s `alt`,
  matching the precedent already set for `flow_def`/`flow_usage_member`. `allocation_usage`,
  `view_usage`, `viewpoint_usage`, and `rendering_usage` have the identical bare-`name` shape and
  were ordered def-before-usage from the start to avoid the same risk proactively.
- **`case`/`analysis`/`verification`/`use case` needed no new guard**: their `_def` parsers already
  use `DefinitionPrefixOptions::def_required()` (per PAR-006a), and their `_usage` counterparts go
  through `case_like_usage_body`/`use_case_usage_tail`, not a bare `name` call, so no ordering fix
  was needed for this family (still placed def-before-usage for consistency).
- **PAR-002 acceptance-criterion tests added**: `connection_def_is_same_variant_kind_at_package_level_and_nested_in_part`
  and `case_def_is_same_variant_kind_at_package_level_and_nested_in_part`
  (`src/parser/part/body.rs`), covering the two families most at risk of the ambiguity bug class
  found in this increment.
- **Scope remaining, updated**: `PartUsageBodyElement` (zero Def-kind variants), `PortDefBodyElement`/
  `PortBodyElement`, `InterfaceDefBodyElement`/`ConnectionDefBodyElement` still unaddressed.
  `PackageBodyElement`'s gaps (standalone `AttributeUsage`/`ItemUsage`/`PortUsage`,
  `ConnectionUsage`, `RefDecl` usage, `EnumerationUsage`) still unaddressed. `PartDefBodyElement`
  itself is now believed complete against the gaps-doc's original list for that enum.

### PAR-002 (increment 3 of N): standalone usages in `PackageBodyElement`

- **Added `AttributeUsage`, `ItemUsage`, `PortUsage`, `ConnectionUsage`, `Ref`, `EnumerationUsage`
  to `PackageBodyElement`** (`src/ast/package.rs`), wired into `try_package_body_annotations`/
  `try_package_body_structure`/`try_package_body_behavior` (`src/parser/package.rs`), each tried
  immediately after its existing `def`-optional definition counterpart (`attribute_def(.., false)`,
  `port_def`, `item_def`, `connection_def`, `enum_def`). `Ref` reuses `part_ref_usage` (now
  re-exported as `crate::parser::part::part_ref_usage`/`connection_usage_member`), which despite
  the name has no part-specific grammar. Confirmed via BNF: `PackageMember` explicitly allows
  `DefinitionElement | UsageElement`, so bare usages are legal package content, not just
  definitions -- contrary to `attribute_def`'s old doc comment claiming "only definitions are
  legal" at package level.
- **Important finding, confirmed empirically via the validation gate and targeted unit tests**:
  for the four keywords whose standalone `_def` parser is deliberately `def`-optional at package
  level (`attribute`, `port`, `item`, `connection`), that `_def` parser's header grammar
  (`parse_optional_definition_header_after_identification` in `src/parser/specialization.rs`)
  already swallows a plain `name: Type;` shape as part of its own (loosely-validated) header text,
  so ordinary typed usages like `port p1: MyPortType;` are captured as the `Def` variant before
  the new `Usage` variant is ever reached -- not a regression introduced here (this parsing
  predates this increment; the validation gate stayed green with no snapshot changes, confirming
  no stdlib content shifted variant). The new `Usage` variants are real and correctly reachable,
  but only for shapes the `Def` grammar cannot parse at all: attribute's `:>>`-prefixed
  redefinition head (`attribute :>> mass = 5;`, only `attribute_usage`'s `PrefixRedefines` shape
  handles a bare `:>>` immediately after the keyword), port's `:>>`-prefixed head likewise,
  item's `subsets`/`references`/`crosses` clauses (not part of `item_def`'s header grammar), and
  connection's anonymous no-name form (`connection: LinkType;`, `connection_def`'s
  `Identification` requires a name/short_name). Verified each via a dedicated
  `package_body_accepts_standalone_*_usage` test using exactly one of these distinguishing shapes,
  not the common `name: Type;` form (which would have silently asserted against dead code).
- **Not fixed, and intentionally out of scope here**: the `Def`-swallows-typed-usage behavior
  above (losing the `: Type` reference into `specializes: None` on `port p1: MyPortType;`, e.g.)
  looks like a real pre-existing accuracy bug in `parse_optional_definition_header_after_identification`'s
  typing-colon-blob handling, but fixing it is a separate, higher-risk change (it's shared header
  logic used by many definition kinds, not `Usage`-wiring) outside this increment's scope. Flagging
  it here rather than silently working around it.
- **PAR-002 acceptance-criterion test added**: `attribute_usage_with_redefines_is_same_variant_kind_at_package_level_and_nested_in_part`
  (`src/parser/package.rs`) parses the same `:>>`-prefixed attribute usage at package level and
  nested in a part body, asserting the same AST variant kind both times.
- **`connection_usage_member` and `part_ref_usage` made `pub(crate)`** and re-exported from
  `crate::parser::part` so `src/parser/package.rs` can reuse them instead of duplicating the
  parsing logic.

### PAR-002 (increment 4 of N): nested `def` kinds in `PartUsageBodyElement`

- **`PartUsageBodyElement` had zero `Def`-kind variants before this increment.** Added `StateDef`,
  `MetadataDef`, `FlowDef`, `RequirementDef`, `OccurrenceDef`, `PortDef`, `CalcDef`,
  `ConnectionDef` (`src/ast/structure.rs`), wired into `part_usage_body_element`
  (`src/parser/part/usage.rs`) using the same parsers already wired for `PartDefBodyElement` in
  increments 1-2 (`state_def`, `metadata_def`, `flow_def`, `requirement_def`, `occurrence_def`,
  `port_def_required`, `calc_def_required`, `connection_def_required`). Justified by BNF
  `UsageBody = DefinitionBody`: a usage body legally contains nested definitions, not just nested
  usages, the same as a definition body does.
- **Same ordering discipline applied proactively**: `port_def_required`/`flow_def` placed before
  `port_usage`/`flow_usage_member` (both lack a guard against a bare `def` token, per the bugs
  found and fixed in increment 2), avoided from the start rather than caught after the fact this
  time. `calc_def_required`/`connection_def_required`/`state_def`/`metadata_def`/
  `requirement_def`/`occurrence_def` had no usage sibling already dispatched in this body, so no
  ordering risk for those.
- **nom `alt` tuple-arity limit hit and fixed**: adding 8 more branches pushed the flat `alt(...)`
  past nom's ~21-element tuple ceiling (a real compile error, not a lint). Restructured into three
  top-level nested `alt` groups (annotations/state/behavior-ish forms; the new def-kind group;
  port/ref/bind/satisfy/interface/connect/flow) rather than one flat list -- same technique
  already used in `part_def_body_element`.
- **PAR-002 acceptance-criterion test added**:
  `state_def_is_same_variant_kind_in_part_usage_body_as_in_part_def_body`
  (`src/parser/part/usage.rs`) confirms the same `state def` declaration yields `StateDef` whether
  nested in a part *usage* body or a part *definition* body.
- **Scope remaining**: `PortDefBodyElement`/`PortBodyElement`, `InterfaceDefBodyElement`/
  `ConnectionDefBodyElement` still unaddressed (item 4 of the coordinator's remaining list).
  `RequirementUsage`/other Usage-kind gaps on `PartUsageBodyElement` were out of scope for this
  increment (task asked specifically for Def-kind variants here).

### PAR-002 (increment 5 of 5, final): widen `PortDefBodyElement`/`PortBodyElement`/
`InterfaceDefBodyElement`/`ConnectionDefBodyElement`

- **`PortDefBodyElement`**: added `ItemDef` (`item_def_required`) and `EnumerationUsage`
  (`enum_usage`); it already had `AttributeDef`/`AttributeUsage`/`ItemUsage`/`PortUsage` from
  before this backlog. **`PortBodyElement`**: had zero attribute/item coverage -- added
  `AttributeUsage` (`attribute_usage`) and `ItemUsage` (`item_usage`).
- **`InterfaceDefBodyElement`** and **`ConnectionDefBodyElement`**: both had only
  `Doc`/`EndDecl`/`RefDecl`/`ConnectStmt`(+`Error` on the latter) -- no attribute/item/port
  coverage at all. Added `AttributeDef`/`AttributeUsage`, `ItemDef`/`ItemUsage`, `PortDef`/
  `PortUsage` to both, reusing the exact same parsers already wired into `PartDefBodyElement`/
  `PartUsageBodyElement`/`PackageBodyElement` in increments 1-4 (`attribute_def`/`attribute_usage`,
  `item_def_required`/`item_usage`, `port_def_required`/`port_usage`) -- no new parsers needed
  anywhere in this increment.
- **Def-before-usage discipline applied proactively** (no new bugs found this time, unlike
  increments 1-2): `item_def_required` placed before `item_usage`/`directed_item_usage`,
  `port_def_required` before `port_usage`, in every body wired this increment. `port_usage` and
  `item_usage` are the same parsers already confirmed (in increments 1-2 and this one) to lack a
  guard against a bare `def` token, so the ordering matters everywhere they're dispatched, not
  just in the bodies where the bug was originally found.
- **New cross-module imports**: `interface.rs` and `connection.rs` now import from
  `attribute.rs`/`item.rs`/`port.rs`. Checked for import cycles first (`port.rs`/`item.rs`/
  `attribute.rs` import nothing back from `interface.rs`/`connection.rs`) -- none introduced.
- **PAR-002 acceptance-criterion tests added**: one per widened file
  (`item_def_is_same_variant_kind_in_port_def_body_as_item_def_required_parser` in
  `src/parser/port.rs`, `port_def_is_same_variant_kind_in_interface_def_body_as_port_def_required_parser`
  in `src/parser/interface.rs`, `attribute_usage_is_same_variant_kind_in_connection_def_body_as_shared_parser`
  in `src/parser/connection.rs`), each confirming the shared parser accepts the identical snippet
  the new body-enum variant wraps.
- **PAR-002 is now believed complete against the gaps-doc's literal list**: every body enum named
  in the gaps doc's PAR-002 section and the coordinator's backlog (`PackageBodyElement`,
  `PartDefBodyElement`, `PartUsageBodyElement`, `PortDefBodyElement`, `PortBodyElement`,
  `InterfaceDefBodyElement`, `ConnectionDefBodyElement`) now carries the definition/usage variants
  called out as missing, wired with the `def_required()`/`_required` disambiguation guard
  established in PAR-006a everywhere a bare-`def`-vulnerable usage sibling exists in the same
  dispatch.
- **Known out-of-scope follow-up, not fixed here** (flagged for a separate task, not chased in
  this backlog per the coordinator): `parse_optional_definition_header_after_identification`
  (`src/parser/specialization.rs`) silently drops a plain `: Type` reference into an unparsed
  header blob for `def`-optional definitions (e.g. `port p1: MyPortType;` parses as `PortDef` with
  `specializes: None`, losing the type). Found and documented in PAR-002 increment 3; not this
  increment's concern.
- **Final read-through against the gaps doc's PAR-002 section caught one real gap this backlog's
  own tracking list had dropped**: `PartDefBodyElement` had `EnumerationUsage` but no `EnumDef` --
  called out explicitly in the original triage ("has ... `EnumerationUsage` but not `EnumDef`")
  but never actually landed in increments 1-2. Added `EnumDef` to both `PartDefBodyElement` and
  `PartUsageBodyElement` (`enum_def` is `def_required()`-guarded internally, so no new disambiguation
  work needed), plus a `enum_def_is_same_variant_kind_at_package_level_and_nested_in_part`
  acceptance test. With this fix, every def/usage pair from the original per-enum gap list is
  confirmed present.

### PAR-006a: recovery-guard foundation

- **Confirmed the PAR-001 disambiguation fix was already generalized**: `attribute_def`'s
  `disambiguate_from_usage` parameter (the original PAR-001 fix) has a reusable form,
  `DefinitionPrefixOptions::def_required()` in `src/parser/definition_prefix.rs`, already adopted
  by `action`, `allocation`, `case`/`analysis`/`verification`/`use case`, `enum`, `flow`,
  `individual`, `interface` (`interface_def_required`), `item` (`item_def_required`), `metadata`,
  `occurrence`, `requirement`, `state`, and `view`/`viewpoint`/`rendering`. Documented this
  explicitly in `definition_prefix.rs` so PAR-002's new body-enum wiring reuses it instead of
  hand-rolling another bespoke guard, and documented which keywords (`connection`,
  `constraint`/`calc`, `port`) deliberately keep `def` optional and why (the real Systems
  Library uses bare, `def`-less forms for those at namespace level; making `def` required there
  was tried and reverted per the 0.33.0 entry below).
- **Removed dead `PortBodyElement::Other(String)` variant**: the enum declared it, but no parser
  ever constructed it (`port_body_element` in `src/parser/port.rs` only ever produces
  `PortUsage`/`InOutDecl`/`Doc`/`Error`); recovery already falls through to a real
  `Error(Node<ParseErrorNode>)`, not this variant. This was flagged as a potential silent-fallback
  gap during PAR-006 triage but turned out to be unreachable code, not a live bug — removed rather
  than "fixed".
- Reviewed `ParseErrorNode` (`src/ast/common.rs`): its existing fields (`message`, `code`,
  `expected`, `found`, `suggestion`, `category`) are sufficient for the recovery markers needed by
  the rest of this backlog — no new fields added.

### PAR-004 (item 6 of 6): standalone `FeatureChain` type

- **Added `ast::FeatureChain`**: a dot-separated feature chain (e.g.
  `engine.fuelCmdPort.flowRate`) had no dedicated AST node distinct from a `::`-qualified name —
  only `Expression::FeatureRef`/`Expression::MemberAccess` folded chains into nested expression
  nodes with no reusable, standalone shape. New `FeatureChain { segments: Vec<String>, span: Span
  }` in `src/ast/feature_chain.rs`, with a `parser::feature_chain` parser in
  `src/parser/feature_chain.rs` built on the existing `name`/`ws_and_comments` lexer helpers.
  Deliberately **not** wired into `Expression` or `src/parser/expr.rs` — PAR-005 (complete
  expression AST) is expected to adopt this type for its own `path_expression` parsing once it
  lands, without needing to touch `expr.rs`. The parser function is unused today
  (`#[allow(dead_code)]`) until a relationship parser or PAR-005 calls it.

### PAR-004/PAR-003 (item 5 of 6): structured `Multiplicity` bounds

- **Added `ast::Multiplicity`**: multiplicity brackets (`[1..*]`, `[0..1]`, `[3]`) were stored as
  raw `Option<String>` bracket text on 9 fields across `PartUsage`, `PortUsage`, `SuccessionUsage`
  (three fields), `ItemUsage`, `EnumerationUsage`, `IncludeUseCase`, and `ReturnRef` — no
  structured access to lower/upper bounds. New `Multiplicity { lower: Option<Box<Node<Expression>>>,
  upper: Option<Box<Node<Expression>>>, span: Span }`, populated by a new
  `parser::usage::multiplicity_node` that reuses the existing `expression()` parser for bound
  expressions. A bare `[3]` sets `lower == upper == Some(3)`; `[1..*]` sets `upper = None`
  (unbounded). `Multiplicity`'s `PartialEq` ignores `span`, matching `Node<T>`'s existing
  convention, so hand-built expected ASTs in tests don't need to reproduce real spans.
- **Found and fixed a real parsing regression while building this**: an initial version handed the
  whole bracket content to `expression()` in one call. That broke `[1..*]` — `expression()`'s
  binary-operator chain commits once it matches `..` as a range comparison operator and does not
  backtrack when the right-hand side (`*`) fails to parse as a primary expression, so it hard-errors
  instead of returning just the left operand. Caught by
  `part_usage_ordered_before_colon_parses_without_recovery` in `tests/apollo_regressions.rs`, which
  parses `part engines[1..*] ordered : RocketEngine;` — a real Systems Library shape. Fixed by
  scanning for the closing `]` and an optional top-level `..` first, then handing each isolated
  bound substring to `expression()` separately, rather than the whole bracket content at once. This
  is exactly the class of regression this backlog's process discipline (run the full validation
  gate after every change, not just once at the end) is meant to catch.
- **Bumps `PARSE_AST_VERSION` to 10**: changes the schema of 9 existing struct fields
  (`Option<String>` → `Option<Node<Multiplicity>>`), invalidating parse caches built against the
  0.34.x schema.

### PAR-004 (item 1 of 6, scoped): typed `TypingRelationship` for definition-level `specializes`

- **Added `ast::TypingKind`/`ast::TypingRelationship`**: `{ target: String, kind, span: Span,
  is_conjugated: bool, is_implied: bool }`, per the design brief's combined shape (folding in
  PAR-003's conjugation concept). Replaces the `specializes: Option<String>` /
  `specializes_span: Option<Span>` field pair — previously carrying no span, kind, or conjugation
  information — with a single `specializes: Option<Node<TypingRelationship>>` field on all 21
  structs sharing that exact pair (`PartDef`, `ItemDef`, `IndividualDef`, `PortDef`,
  `InterfaceDef`, `ConnectionDef`, `MetadataDef`, `EnumDef`, `OccurrenceDef`, `ActionDef`,
  `FlowDef`, `AllocationDef`, `StateDef`, `RequirementDef`, `CaseDef`, `AnalysisCaseDef`,
  `VerificationCaseDef`, `UseCaseDef`, `ConstraintDef`, `ViewDef`, `ViewpointDef`,
  `RenderingDef`). Tractable as one change because all 21 structs source `specializes` from a
  single parser choke point (`specialization.rs`'s `parse_optional_definition_specialization`),
  which now builds the `TypingRelationship` node directly — `kind` is always
  `Subclassification` and `is_conjugated` is always `false`, matching prior behavior exactly
  (definition-level `:>`/`specializes` clauses don't accept a leading `~` today).
- **Deviation from the brief, noted for a follow-up slice**: `PartUsage`/`PortUsage`'s
  `type_name: String` (required, not `Option<String>`) doesn't match the
  `typing`/`specializes: Option<String>` pair shape the brief describes, so it was left
  untouched. `AttributeDef.typing` / `AttributeUsage.typing` — the actual `:` typing side,
  populated by a different code path (`attribute.rs` and `usage.rs`'s
  `typings()`/`conjugated_qualified_name`, which is where real `~` conjugation parsing already
  happens) — were also not migrated in this pass; conjugation there remains a literal
  `~`-prefixed string, unchanged from before this session.
- Regenerated the `function_based_behavior_3a` and `parts_tree_1a` validation snapshot fixtures
  (schema-only Debug-format diff; confirmed target text/spans/kind are correct before
  regenerating).
- Bumps `PARSE_AST_VERSION` to 11.

### PAR-004 (item 1, remaining slice) + PAR-003 (item 4): typed `:` typing and real `~` conjugation

- **`AttributeDef.typing` / `AttributeUsage.typing`** now use the same
  `Option<Node<TypingRelationship>>` shape as the `specializes` side, closing the deviation noted
  in the previous entry.
- **Real conjugation, not a folded string**: `usage::conjugated_qualified_name` used to return the
  target with a literal `~` prefix folded into the string (`"~PortConjugate"`), so nothing could
  query "is this conjugated" without re-parsing the string. It now returns `(bool, String)` — the
  `~` is stripped from the string and exposed as a real flag — and `usage::typings`/
  `optional_typings` thread that flag through as `(Span, bool, String)` instead of `(Span,
  String)`. `TypingKind::Typing` relationships built from this (`attribute.rs`'s new
  `typing_node`/`typing_relationship_node` helpers, mirroring `specialization.rs`'s
  `subclassification_node`) set `is_conjugated` from the real flag instead of always `false`.
- **No regression for untouched fields**: `optional_typings`/`typings` are also called by
  `occurrence_body.rs`, `part/usage.rs`, `port.rs`, `requirement.rs`, and `usage.rs`'s own
  `merge_usage_header` for fields that stay a plain `String`/`Option<String>` (`PartUsage`/
  `PortUsage.type_name`, `UsageHeader.type_name`) — out of scope per the previous entry's noted
  deviation. Each of those call sites now re-embeds `~name` from the `(bool, String)` pair when
  building its string, so their external behavior is byte-for-byte unchanged; only the internal
  representation moved from "folded in the string" to "tracked separately, then re-folded where a
  typed home doesn't exist yet."
- One pre-existing quirk preserved as-is (not introduced by this change): `attribute_def`'s
  leading-`:>`-subset-as-typing fallback (used when an attribute has no separate `:` typing) now
  builds a `TypingRelationship` with `kind: Subclassification` — correct, since that fallback
  really is reading a `:>`-shaped clause — but has no span to attach (this code path never tracked
  one, even before this change), so it uses `Span::dummy()`.
- Bumps `PARSE_AST_VERSION` to 12.

### PAR-004 (item 2 of 6): typed `SubsettingKind`/`SubsettingRelationship`

- **Added `ast::SubsettingKind`/`ast::SubsettingRelationship`**: `{ target: String, kind, span:
  Span, is_implied: bool }`, mirroring `TypingRelationship`'s shape and its
  `PartialEq`-ignores-`span` convention. Replaces the separate `subsets`/`redefines`/
  `references`/`crosses` `Option<String>` fields with `Option<Node<SubsettingRelationship>>` —
  kept as **separate fields per clause kind, not collapsed into one**, since not every struct
  supports all four (`ConnectionUsageMember` only has `subsets`/`redefines`; `ExhibitState` only
  has `redefines`) — on `AttributeUsage`, `PartUsage`, `PortUsage`, `ConnectionUsageMember`,
  `ExhibitState`, `OccurrenceUsage`, and `RequirementUsage` (the last two found via the same
  grep-for-the-same-shape sweep used for item 1's `specializes` fields; not in the original
  brief's explicit list, but they share the exact pattern). `PartUsage`/`PortUsage.subsets` keeps
  its existing `(target, optional value expression)` tuple shape — just the target is now a typed
  node instead of a bare string.
- Most call sites route through `usage.rs`'s `specialization_clauses` (the `subsetting`/
  `redefinition`/`reference_subsetting`/`cross_subsetting` parsers now build the relationship node
  directly, with a real span from operator through target) or `definition_prefix`'s shared
  plumbing, so the ripple was mostly mechanical. A few ad hoc `:>`/`:>>` prefix shapes parsed
  directly outside `usage.rs` (`attribute.rs`'s `attribute_feature_binding`/`metadata_binding`,
  `part/body.rs`'s `exhibit_state`/`connection_usage_member`) needed their own local
  `subsetting_relationship_node` helper and span capture.
- **Side improvement to the previous entry's noted quirk**: `attribute_def`'s leading-`:>`-subset-
  as-typing fallback used `Span::dummy()` for the resulting `TypingRelationship` because the
  leading subset clause it read from had no span of its own. Now that subsetting clauses carry
  real spans (this item), that fallback reuses the `SubsettingRelationship` node's own span
  instead of a dummy one.
- Regenerated the `functional_allocation_4a` and `parts_tree_1a` validation snapshot fixtures
  (schema-only Debug-format diff; confirmed target text, kind, and real spans before
  regenerating).
- Bumps `PARSE_AST_VERSION` to 13.

### PAR-004 (item 3 of 6): typed `ConnectionEnd`/`InterfaceEnd`

- **Added `ast::ConnectionEnd`**: `{ expression: Node<Expression>, span: Span }`, and
  `ast::InterfaceEnd` as a type alias for it. Replaces the plain `Node<Expression>` endpoints on
  `Connect.from`/`to` and `ConnectStmt.from`/`to`/`extra_ends` with `Node<ConnectionEnd>`,
  distinguishing a connector endpoint from an arbitrary standalone expression elsewhere in the
  AST.
- **`InterfaceEnd` is a type alias, not a duplicate struct**: checked `ConnectStmt`'s actual usage
  sites (`src/parser/interface.rs` and `src/parser/connection.rs`, both building a `ConnectStmt`
  from the same shared `connect_ends` parser shape) and
  `InterfaceUsage::TypedConnect`/`Connection` (`src/parser/part/usage.rs`) — an interface end
  carries nothing beyond what a generic connection end carries, both are just a path expression
  with a span. A distinct struct would have no fields of its own.
- **Deviation, noted for a follow-up**: scoped to `Connect`/`ConnectStmt` only, per the requested
  scope. `InterfaceUsage::TypedConnect`/`Connection`'s own `from`/`to` fields (also
  `Node<Expression>` today, and structurally the same "interface end" case) were left untouched —
  a natural follow-up now that `InterfaceEnd` exists, not done here to keep this change to its
  requested shape.
- `connection.rs` and `interface.rs` each define their own `connect_ends` parser (duplicated, not
  shared between the two files); both were updated in lockstep with a small
  `connection_end`/`connect_end` wrapper. `part/usage.rs`'s `connect_` (the part-usage-level
  `connect` statement, distinct from `connect_stmt`) needed its own wrapper too.
- Bumps `PARSE_AST_VERSION` to 14.

## [0.34.0] - 2026-07-15

### Fixed

- **`attribute_usage` accepts a leading visibility modifier**: 0.33.0 made `attribute_def` require
  an explicit `def` keyword whenever it's dispatched alongside `attribute_usage`, but
  `attribute_usage` itself never learned to accept `private`/`protected`/`public` the way
  `attribute_def` already did. A `def`-less, visibility-prefixed declaration — e.g. `private
  attribute zeroDegreeCelsiusInKelvin: ThermodynamicTemperatureValue = 273.15 [K];`, the shape
  used by the official Systems Library's `IntervalScale` catalog entries in
  `SysML Quantities and Units Library/SI.sysml` — correctly failed `attribute_def` (no `def`) but
  then also failed `attribute_usage` (no visibility handling) and fell through to an opaque
  recovered element instead of a proper `AttributeUsage`. `attribute_usage` now accepts and
  discards the same optional visibility prefix as `attribute_def`.

## [0.33.0] - 2026-07-13

Fixes PAR-001: `attribute def` vs. `attribute` usage were ambiguous in every body that permits
both (part, port, requirement, metadata), and the same class of bug existed for non-connector
`interface` members in part-def bodies. Both are now classified solely by an explicit, mandatory
`def` keyword in those ambiguous body contexts, rather than inferred from typing, modifiers, or
fallback parser ordering.

### Fixed

- **`attribute_def` requires `def`**: previously, `attribute_def` was tried before
  `attribute_usage` in part/port/requirement/metadata bodies and only deferred to
  `attribute_usage` for two narrow cases (`redefines`/`:>>`, or a fully untyped `attribute name =
  value`). A plain typed declaration without `def` — e.g. `attribute temperature :
  Temperature;` — fell through and was silently accepted as an `AttributeDef`. `attribute_def`
  now unconditionally requires `def` whenever it's dispatched alongside `attribute_usage`, so
  classification no longer depends on typing or value presence.
- **`attribute_usage` no longer drops a value bound in a leading `:>` (subsets) clause**: exposed
  by the fix above — `subsetting()` parses `target = value` as one unit, but `attribute_usage`'s
  merge logic kept only the target and discarded the value (e.g. `attribute v :> ISQ::speed = 0.9
  [m/s];` lost its default). The captured value is now threaded through as a fallback when no
  separate value expression follows.
- **`interface_def_required` for part-def bodies**: same bug class as `attribute_def`.
  `interface_usage` only recognizes connector forms (`connect ... to ...`), so a non-connector
  interface member without `def` (e.g. `interface foo : SomeInterfaceType;`) fell through and was
  silently accepted as an `InterfaceDef`. Part-def bodies now dispatch a new
  `interface_def_required` (mirroring the existing `item_def`/`item_def_required` split) that
  mandates `def`; a non-connector interface usage form is not yet supported by this parser and
  correctly surfaces as an explicit recovery/error element with a diagnostic instead of a false
  `InterfaceDef`. Package-level `interface_def` keeps `def` optional — the standard library uses
  bare, `def`-less `interface` usages at namespace level (e.g. `abstract interface interfaces:
  Interface[0..*] nonunique :> connections { ... }` in `Systems Library/Interfaces.sysml`) that
  this parser currently folds into `InterfaceDef`, and there's no dedicated package-level
  `interface_usage` dispatch to catch them instead.
- **`PARSE_AST_VERSION`** bumped from `7` → `8`: definition/usage classification changed for the
  cases above, so cached parses built against 0.32.x schema must be invalidated.

An earlier draft of this fix also made `port_def`, `constraint_def`, and `calc_def` require `def`
unconditionally. That broke the full SysML v2 release validation suite (`cargo test --test
validation -- --include-ignored` with `SYSML_V2_RELEASE_DIR` set): the standard library uses the
same bare, `def`-less usage pattern at namespace level for `port`, `constraint`, and `calc` (e.g.
`abstract port ports : Port[0..*] nonunique :> objects { ... }` in `Systems Library/Ports.sysml`,
and similarly in `Calculations.sysml`/`Constraints.sysml`), and none of these three have a
dedicated usage-form parser to fall back to at that level. All three were reverted to optional
`def`, same as `connection_def` (which was correctly left alone from the start — its
hash-annotation prefix, e.g. `#derivation connection { ... }`, is itself a valid definitional
marker in place of `def`).

### Added

- Regression tests: `test_part_def_body_distinguishes_attribute_def_from_usage_by_def_keyword`
  (the `Sensor` acceptance case — one `AttributeDef`, three `AttributeUsage`s for typed, untyped,
  and initialized forms without `def`) and
  `test_part_def_body_never_misclassifies_non_connector_interface_as_definition` in
  [`tests/parser/structure.rs`](tests/parser/structure.rs).

### Notes

- Verified against the full SysML v2 release validation suite (`cargo test --test validation --
  --include-ignored` with `SYSML_V2_RELEASE_DIR` pointing at a fetched
  `Systems-Modeling/SysML-v2-Release` checkout, matching `.github/workflows/ci.yml`'s `validation`
  job) in addition to the default `cargo test`. Regenerated
  [`tests/validation/snapshots/parts_tree_1a.txt`](tests/validation/snapshots/parts_tree_1a.txt)
  for the expected attribute reclassification in `01-Parts Tree/1a-Parts Tree.sysml`.

## [0.32.0] - 2026-07-06

Extends `variant` members in variation part bodies to support typed inline declarations
(SysML §7.6.7), and surfaces the leading `abstract` prefix on requirement, case, analysis,
verification, and use-case definitions/usages as `is_abstract` on the AST node (previously
accepted at parse time but discarded).

### Added

- **`VariantUsage.typed` / `VariantTypedUsage`**: typed `variant` members inside variation part
  def/usage bodies — `variant part name : Type { … }`, `variant attribute name = expr;`,
  `variant item …`, `variant port …` — in addition to the existing untyped reference form
  (`variant name;`, `typed: None`). `VariantTypedUsage` carries `Part`, `Attribute`, `Item`, or
  `Port` nested usage nodes. `variant_usage()` tries each kind parser before falling back to the
  untyped name-and-semicolon form.
- **`is_abstract` on requirement and case families**: `RequirementDef`, `RequirementUsage`,
  `CaseDef`, `CaseUsage`, `AnalysisCaseDef`, `AnalysisCaseUsage`, `VerificationCaseDef`,
  `VerificationCaseUsage`, `UseCaseDef`, and `UseCaseUsage` each gain `is_abstract: bool` (`true`
  for `abstract requirement def …`, `abstract analysis …`, etc.). Definition parsers read the
  flag from `parse_definition_prefix`; usage parsers capture a leading `abstract` keyword before
  the kind keyword.
- **Regression tests** [`tests/abstract_requirement_analysis_flags.rs`](tests/abstract_requirement_analysis_flags.rs)
  for `is_abstract` on requirement/analysis/verification/use-case defs and usages; extended
  [`tests/variation_variant_body.rs`](tests/variation_variant_body.rs) with the spec's
  `TransmissionChoices` typed `variant part` example.
- **`PARSE_AST_VERSION`** bumped from `6` → `7` to invalidate caches built against the 0.31.x
  schema.

## [0.31.0] - 2026-07-06

Fixes S42-LIM-014: `variant` members inside a `variation part def` body now parse as structured
AST nodes instead of falling through to error recovery. Part usage bodies already supported this
since 0.25.2 (`PartUsageBodyElement::VariantUsage`); part definition bodies did not.

### Added

- **`PartDefBodyElement::VariantUsage`**: `variant` *name* `;` inside a `variation part def` brace body (e.g. `variation part def NavigationSensorSuiteChoice :> SensorAssembly { variant tofImuOnly; … }`). `part_def_body_element()` dispatches via the existing `variant_usage` parser (now `pub(crate)`); `b"variant"` was already in `PART_BODY_STARTERS` from 0.25.2 so recovery did not abort, but unrecognized members were previously misclassified as recovery errors rather than owned `VariantUsage` nodes.
- **Regression test** [`tests/variation_variant_body.rs`](tests/variation_variant_body.rs) for a three-variant `variation part def` body with no recovery `Error` nodes.
- **`PARSE_AST_VERSION`** bumped from `5` → `6` to invalidate caches built against the 0.30.x schema.

## [0.30.0] - 2026-07-03

Closes all 7 open follow-ups listed in `docs/PARSER_BACKLOG_ROADMAP.md` § 5 from the 0.29.0
state/action/connector audit, plus three unrelated `doc`-comment parsing gaps found while
fixing a Babel42 Components-tab bug report.

### Added

- **`doc` support in port usage bodies**: `PortBodyElement` gains a `Doc(Node<DocComment>)` variant. Previously a `doc /* ... */` block inside a `port name : Type { ... }` *usage* body (as opposed to a `port def` body, which already supported it) failed to parse, and error recovery misclassified the doc text as a "bare feature declaration in part definition body".
- **`doc` support in `connection def` bodies**: `ConnectionDefBodyElement` gains `Doc(Node<DocComment>)`. `connection_def_body_element()` also gains an `Error(Node<ParseErrorNode>)` variant; `connection_member_body` is migrated from a hand-rolled recovery loop (whose only fallback silently discarded every member *after* an unrecognized one, not just the bad one) to the shared `parse_structured_brace_members` helper, matching `port_body_brace`.
- **`doc` support in interface usage connect bodies**: `InterfaceUsageBodyElement` gains `Doc(Node<DocComment>)`. Previously a `doc` block inside `interface ... connect ... to ... { ... }` caused the entire interface usage to be discarded as a generic recovery `Error` node one level up (`connect_body_with_elements` had no recovery path at all).
- **`if` / `while` / `terminate` control nodes**: new `IfStmt`, `WhileStmt`, `TerminateStmt` AST nodes and matching `ActionDefBodyElement`/`ActionUsageBodyElement` variants. `if`/`while` bodies are fully structured (`ActionDefBody`, reusing `action_def_body_brace` for real recursion, including nested control nodes), not the opaque `FirstMergeBody` that `decide`/`join`/`fork` use. `terminate` accepts an optional target expression (`terminate;` / `terminate someAction;`). Both `alt()` dispatchers needed nesting into a sub-`alt()` to stay under nom's 21-branch tuple limit.
- **Standalone `succession` usage**: new `SuccessionUsage` AST node (`OccurrenceBodyElement::SuccessionUsage`) for `succession (first)? source then target;` written directly in a definition/occurrence body — distinct from the action-body `FirstStmt`/`MergeStmt` control node and from `succession flow X to Y;`. Also models the multiplicity-bearing form actually used by the SysML Systems Library (`succession [seBeforeNum] first [0..1] sourceEvent then [0..1] self;` in `Flows.sysml`): `SuccessionUsage.multiplicity`/`source_multiplicity`/`target_multiplicity`. Previously swallowed as opaque `Other(String)` text via `DEFINITION_BODY_OPAQUE_STARTERS`.
- **Transition trigger `via`**: `TransitionAccept::Payload`/`Shorthand` each gain an `Option<Node<Expression>>` for a trailing `via <port>` clause (e.g. `accept TurnOn via commPort`). Previously only the `do`-effect `accept`/`send` forms supported `via`; the trigger clause (before `if`/`do`) did not.
- **`satisfy requirement <name> : <Type> by <expr>`**: `Satisfy` gains `inline_requirement: Option<InlineSatisfyRequirement>` for the fuller named/typed requirement-usage form, reusing `optional_typings()` from `usage.rs`. The bare `satisfy <ref> (by <expr>)?;` shorthand is unaffected (`inline_requirement: None`).
- **`assert constraint` / `satisfy` reachable from more scopes**: `PartDefBodyElement` gains `AssertConstraint(Node<AssertConstraintMember>)` and `Satisfy(Node<Satisfy>)` — both previously only reachable from `occurrence def` bodies (`assert constraint`) or package level (`satisfy`), not `part def` bodies. `OccurrenceBodyElement` gains `Satisfy(Node<Satisfy>)` (occurrence def bodies already had `AssertConstraint`). `b"assert"` added to `PART_BODY_STARTERS`; `b"satisfy"` added to `OCCURRENCE_BODY_STARTERS`.
- **KerML arrow-invocation operator**: `postfix()` in `expr.rs` now parses `->name` / `->name(args)` (e.g. `collection->size()`, `powerProfile->size()-1`) at the same precedence level as `.` member access, desugaring into the existing `Expression::MemberAccess`/`Invocation` shapes — no new `Expression` variant, so no downstream exhaustive matches needed updating.
- **`PARSE_AST_VERSION`** bumped from `4` → `5` to invalidate caches built against the 0.29.x schema.

### Changed

- **`AssignStmt.rhs`**: changed from `String` to `Node<Expression>`, unblocked by the arrow-invocation operator above (real models commonly write `x := collection->size();`, which previously required the raw-text fallback).
- **`ForLoop.range` raw-text fallback**: retained as a defensive net (not removed — recovery/fallback paths are kept per project convention), but the arrow-invocation operator above means it's no longer hit for the common case; doc comment updated accordingly.

## [0.29.0] - 2026-07-01

### Added

- **State `do`/`exit` actions**: `StateDefBodyElement` gains `Do(Node<DoAction>)` and `Exit(Node<ExitAction>)` variants, mirroring the existing `Entry` handling. Previously `do action ...` / `exit action ...` in a state body fell through to error recovery; only `entry` was recognised. `b"do"` / `b"exit"` added to `STATE_BODY_STARTERS`.
- **Control nodes `decide` / `join` / `fork`**: new `DecisionStmt`, `JoinStmt`, `ForkStmt` AST nodes and `ActionDefBodyElement`/`ActionUsageBodyElement` variants, parsed the same way as the existing `MergeStmt`. The action-body keyword lists previously listed the non-spec keyword `decision` instead of `decide`, and had no parser at all for `join`/`fork`.
- **Negated `assert` / `satisfy`**: `AssertConstraintMember` and `Satisfy` both gain `is_negated: bool`. `assert not constraint { ... }` and `(assert)? not satisfy X (by Y)?;` now parse; previously `not` after `assert` was unhandled and `assert`-prefixed `satisfy` had no parse path at all (only the bare `satisfy X by Y;` shorthand worked).
- **Structured transition `do` effect**: new `TransitionEffect` enum (`Perform`, `Accept`, `Send`, `Assign`, `Expression`) replaces the raw `Node<Expression>` previously stored on `Transition.effect`. Recognises `do action name : Type`, `do accept payload (: Type)? (via expr)?`, `do send payload (: Type)? (via expr)? (to expr)?`, and `do assign lhs := rhs`; falls back to a bare expression for anything else.
- **`ConnectStmt.extra_ends`**: `ConnectStmt` gains `extra_ends: Vec<Node<Expression>>` for the SysML v2 n-ary connector/interface form `connect (a, b, c, ...)`. The ordinary binary `from ... to ...` form is unaffected (`extra_ends` is empty).
- **`PARSE_AST_VERSION`** bumped from `3` → `4` to invalidate caches built against the 0.28.x schema.

### Changed

- **`ForLoop.range`**: changed from `String` to `Node<Expression>`. The `for x in <range> { ... }` range now parses as a structured expression (e.g. `1..10`); falls back to raw text only when the expression grammar can't parse the range (e.g. KerML `->` arrow-invocation syntax, not yet modelled — see `docs/PARSER_BACKLOG_ROADMAP.md` § 5).
- **`AssignStmt.lhs`**: changed from `String` to `Node<Expression>`, parsed via `path_expression` (matches spec's `AssignmentTargetParameter` feature-chain shape). `AssignStmt.rhs` remains a raw `String` (out of scope for this pass).

See `docs/PARSER_BACKLOG_ROADMAP.md` § 5 for the full list of gaps this closes and the related follow-ups intentionally left open (`if`/`while`/`terminate` control nodes, standalone `succession` usage, transition trigger `accept ... via`, full `satisfy requirement name : Type` form, `assert`/`satisfy` scope wiring, KerML arrow-invocation expressions).

## [0.28.0] - 2026-06-29

### Added

- **`MetadataAnnotation` in use-case, view-def, and calc-def bodies**: `UseCaseDefBodyElement`, `ViewDefBodyElement`, and `CalcDefBodyElement` now carry a `MetadataAnnotation` variant. `@Name` annotations in those bodies are parsed via the structured `metadata_annotation()` path (previously fell through to the generic `Annotation` fallback or were silently skipped). Recovery starters updated: `b"@"` added to `USE_CASE_BODY_STARTERS`, `VIEW_DEF_BODY_STARTERS`, and `CALC_DEF_BODY_STARTERS`.
- **`CaseReturnDecl`**: new AST node modelling `return [attribute] name [: Type] [= expr] ;` and `return :>> name [= expr] ;` in analysis and verification case bodies. Adds `UseCaseDefBodyElement::CaseReturnDecl`. Fields: `name: String`, `name_span: Option<Span>`, `type_name: Option<String>`, `is_redefine: bool`. Previously these forms fell through to `Other`.
- **Rep language diagnostics**: `textual_representation()` now parses `language STRING_VALUE` resiliently. A missing `language` keyword produces a `TextualRepresentation` node with `language_span = None`; an empty language string is flagged. `collect_errors` emits `missing_rep_language` or `invalid_rep_language` (both already in `diagnostic_catalog.rs`) for these cases. Wired in requirement body and package body collectors.
- **`PARSE_AST_VERSION`** bumped from `2` → `3` to invalidate caches built against the 0.27.x schema.
- **Recovery tests for view-def and constraint-def bodies**: `view_def_recovery_inserts_error_node_and_keeps_later_render` and `constraint_def_recovery_inserts_error_node_and_keeps_later_sibling` added to `tests/recovery_diagnostics_integration.rs`, verifying that a malformed token produces a `ParseErrorNode`, surfaces as a diagnostic, and does not abort parsing of subsequent members.

### Fixed

- **`textual_representation()` resilience**: the `language STRING_VALUE` clause is now parsed with soft-failure so a missing `language` keyword produces a structured node (with `language_span = None`) rather than a hard nom error, enabling the error collector to emit a targeted `missing_rep_language` diagnostic.

## [0.27.0] - 2026-06-29

### Added

- **`InOutDecl.value`**: `in`/`out`/`inout` parameter declarations now carry an optional `value: Option<Node<Expression>>` field. The parser recognises `= expr` default-value initialisers after the type annotation and stores them; the error-recovery path yields `None`.
- **`RefBody::Brace { elements }`**: the unit variant `RefBody::Brace` is replaced by a struct variant carrying `elements: Vec<Node<ActionDefBodyElement>>`. Action-context ref bodies (`ref action name: Type { … }`) are fully parsed into structured elements via `parse_structured_brace_members`; other ref-body contexts (state, part, interface, connection) remain opaque and produce an empty element list.
- **`Satisfy.body_elements`**: `Satisfy` gains `body_elements: Option<Vec<Node<ConstraintDefBodyElement>>>`. Braced satisfy bodies now expose their structured constraint members instead of discarding them; semicolon-terminated satisfies yield `None`.
- **`Import.target_span`**: `Import` gains `target_span: Span` recording the source span of the qualified name (excluding `::*` / `::**` suffix), enabling semantic-token providers to highlight only the name portion.
- **`PARSE_AST_VERSION`** bumped from `1` → `2` to invalidate caches built against the 0.26.x schema.

### Fixed

- **Error collection for `RefBody::Brace { elements }`**: `collect_action_def_body_errors` now recurses into `ActionDefBodyElement::RefDecl` bodies via the new `collect_ref_body_errors` helper, surfacing `ParseErrorNode`s nested inside action ref bodies as LSP diagnostics.
- **Error collection for `Satisfy.body_elements`**: `collect_part_usage_body_errors` and `collect_package_body_errors` now walk `Satisfy.body_elements` via `collect_constraint_body_element_errors`, surfacing constraint-body recovery errors inside braced satisfy statements.

## [0.26.0] - 2026-06-26

### Added

- **`serde` feature**: all AST types (`Span`, `Node<T>`, `Expression`, operators, and every `pub struct` / `pub enum` in `core`, `common`, `root`, `package`, `structure`, `behavior`, `requirement`, `view`, `kerml_fallback`) now conditionally derive `serde::Serialize` / `serde::Deserialize` when the `serde` feature is enabled. Enable with `sysml-v2-parser = { version = "0.26.0", features = ["serde"] }`. `DiagnosticCategory` and `DiagnosticSeverity` from `error` also gain the derives so that `ParseErrorNode` (embedded in the AST) round-trips cleanly.
- **`PARSE_AST_VERSION: u32` constant** (in `lib.rs`): must be incremented on any breaking AST schema change. Consumers that cache serialized parse results (e.g. an LSP parse cache keyed by content hash) embed this value in cache entries and reject stale entries on mismatch.

### Fixed

- **Diagnostic collection gaps** in `collect_errors.rs`: `ParseErrorNode`s nested inside the bodies of `CaseDef`, `CaseUsage`, `AnalysisCaseDef`, `AnalysisCaseUsage`, `VerificationCaseDef`, `VerificationCaseUsage`, `ViewpointDef`, `ViewpointUsage`, `RenderingDef`, `PortDef`, `AttributeDef`, `ItemDef`, `IndividualDef`, `MetadataDef`, `MetadataUsage`, `OccurrenceDef`, `OccurrenceUsage`, `AllocationDef`, `AllocationUsage`, `FlowDef`, and `FlowUsage` were silently dropped and never surfaced as LSP diagnostics; they now propagate correctly.
- **`collect_part_def_body_errors`**: errors inside `ExhibitState`, `RequirementUsage`, `OccurrenceUsage`, `AttributeDef`, and `AttributeUsage` members of part definition bodies now surface.
- **`collect_part_usage_body_errors`**: errors inside `OccurrenceUsage` and `AttributeUsage` members of part usage bodies now surface.
- Five new helper collectors introduced: `collect_attribute_body_errors`, `collect_definition_body_errors`, `collect_occurrence_usage_body_errors`, `collect_port_def_body_errors`, `collect_rendering_def_body_errors`.
- **Parser gaps in SysML standard library**: attribute, metadata, occurrence-definition, port-definition, view-definition, and rendering-definition bodies now recognise and capture as `Other` a range of previously-unhandled syntax forms that appear in the SysML standard library (e.g. `ref self`, `item start`, `end source`, `abstract ref port`, `derived ref item`, `binding`, `connection`, `in event occurrence`, `succession`, multi-target `:>>`, and conditional `if ? else` expressions in attribute values). All 94 standard-library files now parse without diagnostics.

## [0.25.4] - 2026-06-12

### Added

- **Generic `FlowUsage`**: unified parser for `flow`, `message`, and `succession flow` with optional name, `of` payload, `from`/`to` or shorthand `expr to expr`, and `DefinitionBody` brace bodies.
- **`FlowUsageKind`** and extended **`FlowUsage`** AST (replaces anonymous **`Flow`** struct in action bodies).
- **`FlowUsage`** wiring in `OccurrenceBodyElement`, `PartDefBodyElement`, `PartUsageBodyElement`, `UseCaseDefBodyElement`, and action bodies (via shared `flow_usage_member`).

### Changed

- **`flow_usage` / `flow_usage_member`**: supersedes named-only package parser and action-body `flow_` helper.
- **`PART_BODY_STARTERS` / `OCCURRENCE_BODY_STARTERS`**: include `flow`, `message`, `succession`.

## [0.25.3] - 2026-06-12

### Fixed

- **State transitions**: `transition <name> first <source> then <target>` no longer sets `is_initial` on named transitions; only unnamed `transition first … then …` forms are initial transitions.

## [0.25.2] - 2026-06-11

### Added

- **`PartUsage.usage_prefix`**: optional `abstract` or `variation` prefix on part usages (package-level `variation part …` and nested part usage bodies).
- **`PartUsageBodyElement::VariantUsage`** and **`VariantUsage`**: `variant` *name* `;` inside variation part usage bodies.
- **`variation` / `variant`** in `PART_BODY_STARTERS` so part usage body recovery does not abort on variation modelling members.

### Changed

- **`part_usage`**: parses leading `abstract` / `variation` before `part` (nested bodies); package-level `part_def_or_usage` propagates `usage_prefix` onto usages.
- **Validation fixtures** updated for the new `PartUsage` field (`usage_prefix: None`).

## [0.25.1] - 2026-06-11

### Added

- **`MetadataKeywordUsage`** in action and usage body element parse paths.
- **`Expression::MetaCast`**: reflective `meta` cast expressions.
- **`about` targets** on metadata annotations and metadata usages.
- **Tests** in [`spec42_diagnostics_ast.rs`](tests/spec42_diagnostics_ast.rs) for metadata keyword, `about`, and meta-cast forms.

## [0.24.0] - 2026-06-10

### Added

- **`ConstraintDefBodyElement::MetadataAnnotation`**: `@Name : Type` in constraint definition bodies (not only generic expressions).
- **`b"@"`** in `CONSTRAINT_DEF_BODY_STARTERS` for structured recovery.
- **Fixture** [`constraint-metadata-annotation.sysml`](tests/fixtures/constraint-metadata-annotation.sysml).
- **`ReturnRef.return_expression`**: structured `return <expr>;` inside verification return-ref bodies.
- **Recovery tests** for state `ref` brace bodies and part-usage bind/ref connect bodies.

### Changed

- **Opaque brace bodies** in `state.rs` (`ref` bodies) and `part/usage.rs` (bind/ref connect bodies) use structured recovery instead of silent `advance_to_closing_brace` (5 sites removed).
- **`use_case_def_body_brace`** uses `parse_structured_brace_members` (removes body-level abort); `ref :>>` and `return ref` inner bodies parse with structured recovery.
- **`return_expression_stmt`** shared between calc and return-ref bodies (was private `calc_return_expression`).

## [0.23.0] - 2026-06-10

### Added

- **`Expression::Classification`**: `@Metaclass` filter/guard forms (e.g. `@SysML::PartUsage`) no longer stored as `FeatureRef("@…")`.
- **`Expression::TypeCheck`**: `istype`, `hastype`, and `as` type tests with optional operand.
- **`Expression::Select` / `Collect`**: structured `.?` / `.**` selector expressions.
- **Typed `stakeholder name : Type;`**: extends `StakeholderMember` while keeping shorthand `stakeholder ConcernRef;`.
- **`MetadataAnnotation`** variants on `StateDefBodyElement` and `PartDefBodyElement` parse paths.
- **Fixtures** [`expression-classification.sysml`](tests/fixtures/expression-classification.sysml), [`stakeholder-typed.sysml`](tests/fixtures/stakeholder-typed.sysml).

### Changed

- **Opaque brace bodies** in `action.rs` and `requirement.rs` use structured recovery instead of silent `advance_to_closing_brace` (11 sites removed).
- **`constraint_body` / `satisfy` connect bodies** delegate to `structured_constraint_body`.
- **`metadata_ref_primary`** emits `Classification` AST nodes.

## [0.22.0] - 2026-06-09

### Added

- **`ItemUsage.direction`**: optional `InOut` on item usages parsed as `in`/`out`/`inout item …` in port def bodies.
- **`directed_item_usage`**: parser path for direction-prefixed item usages (SysML §7.6.3 / port directed-features table).
- **`PortDefBodyElement::ItemUsage`**: port def brace bodies accept directed items before legacy `in_out_decl` pins.
- **`tests/vacuuming_types_parse`**: inline fixtures for directed port items, block/line comments with braces, and optional live-corpus checks via `MBSE_VACUUM_EXAMPLE_DIR`.
- **Fixture** [`tests/fixtures/port-directed-item-inout.sysml`](tests/fixtures/port-directed-item-inout.sysml).

### Fixed

- **`in_out_decl`**: directed port pins accept `:>` subsetting (e.g. `out volume :> ISQSpaceTime::volume`), not only `:` typing.
- **EOF brace balance**: `has_unclosed_brace` / `extra_closing_brace_at_eof` ignore `{`/`}` inside `//` and `/* */` comments (fixes false `missing_closing_brace` on commented-out blocks).
- **Recovery**: `local_recovery_line_boundary` skips `//` line comments correctly; bounds check in `balanced_inline_depth`.

### Changed

- **`direction_prefix`**: `pub(crate)` in [`attribute.rs`](src/parser/attribute.rs) for reuse by item and attribute directed-usage parsers.

## [0.21.0] - 2026-06-09

### Added

- **`AttributeUsage.direction`**: optional `InOut` when parsed as `in`/`out`/`inout attribute …` in port def bodies.
- **`directed_attribute_usage`**: parser path for direction-prefixed attribute usages, including `out attribute redefines …`.

### Fixed

- Port def bodies no longer mis-parse `out attribute redefines name :> Type` as `InOutDecl` with name `redefines`.

## [0.20.0] - 2026-06-08

### Added

- **DefinitionBody parity** — `flow def`, `flow`/`message` usage, `allocation def`, and `allocation`/`allocate` usage brace bodies now parse occurrence-style members (`attribute`, `part`, `occurrence`, `assert constraint`, `doc`, …) via shared [`occurrence_body.rs`](src/parser/occurrence_body.rs), aligned with `occurrence def` bodies.

### Changed

- Doc-only flow/allocation brace bodies now surface as `DefinitionBodyElement::OccurrenceMember(Doc(...))` instead of top-level `DefinitionBodyElement::Doc`.
- Unknown statements in generic `DefinitionBody` brace bodies emit recovery `Error` nodes instead of being silently skipped.

## [0.19.0] - 2026-06-08

### Breaking

- **`ActionUsage.accept` / `ActionUsage.send`**: now `Option<PayloadClause>` (was `Option<(String, String)>`).
- **`Transition`**: added `is_initial`, `accept: Option<TransitionAccept>`; `BinaryOperator` / `UnaryOperator` replace raw operator strings in expressions.

### Added

- **`PayloadClause`**, **`TransitionAccept`**, **`FinalState`** AST members for state-machine accept/send/final syntax.
- **`MetadataKeywordUsage`** (`#keyword`) in part, state, requirement, and use-case bodies.
- **`StakeholderMember`**, **`PurposeMember`**, **`TextualRep`** in requirement/viewpoint bodies.
- **`spec42_diagnostics_ast`** integration tests and fixtures for the June 2026 parser wave.

## [0.18.0] - 2026-06-05

### Breaking

- **`MetadataDef.body`**: now `AttributeBody` (was `DefinitionBody`) so `attribute` members in metadata definitions parse structurally like item definitions.
- **`PackageBodyElement`**: added `MetadataUsage` for package-level `metadata name : Type` declarations.

### Added

- **`MetadataUsage`** AST and `metadata_usage` parser (BNF MetadataUsageDeclaration).
- **Expose feature chains**: `expose` targets accept dot-separated usage segments after the initial qualified name (SysML §7.6.6).

### Removed

- **`invalid_qualified_name_separator`** diagnostic for valid expose feature-chain notation (dots between usage segments).

## [0.17.0] - 2026-06-04

### Breaking

- **`AttributeUsage`**: added `subsets`, `references`, and `crosses` (`Option<String>`) for `:>` / `::>` / `=>` clauses on attribute usages. Manual struct literals and destructuring must include the new fields (`None` when absent).
- **`PortUsage`**: added `references` and `crosses` for the same operators on port usages.

### Added

- **P2–P4 parser debt (complete)**: plans and status in [`docs/PARSER_DEBT_P2_PLAN.md`](docs/PARSER_DEBT_P2_PLAN.md), [`docs/PARSER_DEBT_P3_PLAN.md`](docs/PARSER_DEBT_P3_PLAN.md), [`docs/PARSER_DEBT_P4_PLAN.md`](docs/PARSER_DEBT_P4_PLAN.md); P4 checklist marked done in [`docs/PARSER_TECHNICAL_DEBT.md`](docs/PARSER_TECHNICAL_DEBT.md).
- **Structured view and part definition bodies**: `view_body` and `part_def_body_brace` use `parse_structured_brace_members_with_skip` with scoped recovery (`BodyElementRecover`, `view_body_recovery`); recovery nodes are retained when recovery ends at `}` (fixes empty view bodies and missing expose diagnostics).
- **Definition headers** ([`src/parser/definition_header.rs`](src/parser/definition_header.rs)): shared header parsing; pilot use in item and view definition paths.
- **Expression surface**: `implies` below `or` / `and` in [`src/parser/expr.rs`](src/parser/expr.rs) with unit coverage.
- **Recovery and LSP**: clearer `Many0` / `Many1` messages in [`src/parser/diagnostics.rs`](src/parser/diagnostics.rs); `expose_member` rejects `.` after a qualified expose name; `tests/recovery_body_scopes.rs` adds `part_def_recovery_keeps_later_members`.
- **Module layout (internal, public API unchanged)**: `src/ast.rs` split into `src/ast/`; parser helpers split into `diagnostics.rs`, `recovery.rs`, `collect_errors.rs`, `parse.rs`; `part.rs` split into `src/parser/part/` (`mod`, `prelude`, `body`, `def`, `usage`); integration tests split under `tests/parser/`.

### Changed

- **Port and attribute usages**: wire `references` / `crosses` (and attribute `subsets`) from shared `usage_header` parsing; AST normalization updated in [`src/ast/mod.rs`](src/ast/mod.rs).
- **Validation snapshots**: `parts_tree_1a` and `functional_allocation_4a` AST snapshots refreshed for structured-body and usage-header shapes.
- **BNF compliance test paths**: part grammar references `part/def.rs` and `part/usage.rs` after the part module split.

### Fixed

- **View body recovery**: structured brace loop no longer drops the final recovery `Error` node when the skip ends at `}` (regression that hid invalid `expose` separator diagnostics).
- **Clippy**: `#[allow(clippy::large_enum_variant)]` on affected enums in [`src/ast/structure.rs`](src/ast/structure.rs) after usage AST growth.

### Migration (Spec42 and similar hosts)

1. Bump to `sysml-v2-parser` `0.17.0` (crates.io or tag `v0.17.0`).
2. Extend `AttributeUsage` / `PortUsage` construction and matches with `subsets` / `references` / `crosses` as needed (`None` when not used).
3. Re-run `cargo test`, `cargo test --test validation -- --include-ignored`, and `cargo test --test bnf_compliance` with `SYSML_V2_RELEASE_DIR` set.
4. If you snapshot AST text, refresh fixtures after bumping.

[0.17.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.16.0...v0.17.0

## [0.16.0] - 2026-06-03

### Added

- **Requirement body actors**: `RequirementActorDecl` and `actor_decl` in requirement definition bodies (anonymous `actor : Type;` mirrors existing `subject : Type;`).
- **Enumeration usages in part bodies**: `EnumerationUsage`, `enum_usage` parser, and `PartDefBodyElement::EnumerationUsage` / `PartUsageBodyElement::EnumerationUsage` for `enum name : Type;` inside part definitions and usages.
- **Part definition members**: `ItemUsage` and `CalcUsage` in `part_def_body_element` (library-style `item` / `calc` usages in part defs).
- **Diagnostics taxonomy** ([`src/parser/mod.rs`](src/parser/mod.rs)): `DiagnosticCategory`, `DiagnosticSeverity` on `ParseError`; classification for invalid requirement short names (`id '…'`), bare features in part defs, invalid typing operators, and related recovery codes.
- **Editor-oriented post-processing**: cascade suppression (`recovery_cascade_suppressed`), deduplication by specificity, and `suppress_redundant_closing_brace_errors` when a line already reports an invalid `{…}` statement block.
- **Corpus-oriented checks**: `collect_implicit_attribute_in_part_def_warnings`, `collect_requirement_id_dialect_diagnostics`; Apollo regression test [`tests/apollo_regressions.rs`](tests/apollo_regressions.rs).
- **Recovery fixtures/tests**: anonymous actor in requirement, enum in part def, calc usage in part def, bare feature hint, nested part-def typed usages, requirement `id` dialect hint; glued `}package` now expected to parse cleanly.

### Changed

- **`parse_with_diagnostics`**: no longer emits `missing_statement_separator_between_members` for valid glued `}package` boundaries; stricter trailing-`}` handling at root with `unexpected_closing_brace` where appropriate.
- **Recovery**: `missing_member_name` skips anonymous `subject` / `actor` before `:` only in `"requirement body"` scope (use case `actor:` without a name still diagnosed).
- **`part_usage_body_element`**: nested `alt` to stay within nom tuple limits after new enum arm.

### Fixed

- **False positives** on spec-aligned models: `missing_member_name` on `actor : Type` in requirement bodies; `unexpected_keyword_in_scope` for `enum` in part defs; bogus separator errors at `}package`.
- **SurveillanceDrone-errors** validation expectations aligned with multi-package recovery (four root packages, three member-level errors).

### Migration (Spec42 and similar hosts)

1. Bump to `sysml-v2-parser` `0.16.0` (crates.io or tag `v0.16.0`).
2. Match on `RequirementDefBodyElement::RequirementActorDecl` (not a separate top-level `ActorDecl` in requirement bodies — use case `ActorDecl` remains distinct).
3. Handle `PartDefBodyElement::EnumerationUsage` and `PartUsageBodyElement::EnumerationUsage` in graph builders (or ignore like other usage members).
4. Remove handling for diagnostic code `missing_statement_separator_between_members` if you branched on it.
5. Re-run `cargo test` and validation fixtures after bumping.

[0.16.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.15.0...v0.16.0

## [0.15.0] - 2026-06-03

### Breaking

- **`PortBody`**: removed variant `BraceWithPorts { elements: Vec<Node<PortUsage>> }`. Nested port bodies now use `PortBody::Brace { elements: Vec<Node<PortBodyElement>> }` with structured members (`PortUsage`, `InOutDecl`, `Error`, `Other`). Update exhaustive matches and any code that assumed nested ports were only `PortUsage` nodes.
- **`AttributeBody`**: brace bodies are now `AttributeBody::Brace { elements: Vec<Node<AttributeBodyElement>> }` instead of an opaque skipped brace. Members include nested attributes, doc comments, annotations, and recovery `Error` nodes.
- **`DefinitionBody` / `RenderingDefBody`**: generic definition and rendering definition brace bodies now expose structured `DefinitionBodyElement` / `RenderingDefBodyElement` lists (doc, occurrence members, recovery errors) rather than opaque skipped content for occurrence, rendering, flow, allocation, and metadata families.

### Added

- **BNF compliance gate (100% `implemented`)**: machine-readable map [`docs/bnf_coverage.map`](docs/bnf_coverage.map) and [`tests/bnf_compliance.rs`](tests/bnf_compliance.rs) classify all 640 SysML/KerML textual productions; new tests assert zero `partial` map rules and full production coverage. See [`docs/BNF_COVERAGE.md`](docs/BNF_COVERAGE.md) and [`docs/BNF_COMPLIANCE_MATRIX.md`](docs/BNF_COMPLIANCE_MATRIX.md).
- **Shared usage grammar** ([`src/parser/usage.rs`](src/parser/usage.rs)): `usage_header`, `feature_usage_header`, `specialization_clauses`, `subsetting` / `redefinition`, plus `references` (`::>`) and `crosses` (`=>`) operators; supports `defined by`, `typed by`, conjugated types (`~`), and multiple specialization clauses (last-wins where the AST stores a single target).
- **Structured body parsing** ([`src/parser/body.rs`](src/parser/body.rs)): `parse_structured_brace_members` and `advance_to_closing_brace` replace opaque `skip_until_brace_end` in many high-traffic modules (attribute, part, port, occurrence, rendering, flow, allocation, metadata, connection, interface, import, alias, enumeration, constraint, use case).
- **Expression surface** ([`src/parser/expr.rs`](src/parser/expr.rs)): `select` (`.?`), `collect` (`.**`), and parenthesized sequence expressions; precedence-aware binary/unary chain unchanged as the main `expression()` entry point.
- **BNF surface helpers** ([`src/parser/bnf_surface.rs`](src/parser/bnf_surface.rs)): shared entry points and unit tests for lexical terminals, empty productions, and usage/definition declaration fragments.
- **Lexical operators** ([`src/parser/lex.rs`](src/parser/lex.rs)): `references_operator`, `crosses_operator`, `decimal_value_text`, `string_value`, plus lexical BNF unit tests.
- **Action control nodes**: action definition bodies recognize `accept`, `decision`, `fork`, `join`, `send`, `terminate`, `while`, and `if` starters as control-node action usages.
- **CI**: workflow fetches the pinned SysML v2 release tree and runs `cargo test` with `SYSML_V2_RELEASE_DIR` so the BNF gate and default tests run against normative fixtures on every push.
- **Docs**: updated [`docs/SYSML_V2_COMPLIANCE_GAP.md`](docs/SYSML_V2_COMPLIANCE_GAP.md), [`docs/PARSER_TECHNICAL_DEBT.md`](docs/PARSER_TECHNICAL_DEBT.md), and validation README/snapshots for structured parsing regressions.

### Changed

- **Part and port definitions/usages**: brace bodies parse structured member AST with recovery (`PartDefBody`, `PortDefBody`, `PortBodyElement`) instead of swallowing inner grammar.
- **Action and state definitions**: definition-level bodies use structured member loops with `skip_statement_or_block` recovery (no `skip_until_brace_end` on promoted top-level defs guarded by `bnf_compliance`).
- **Requirement, case, view, and usage families**: migrated to shared `usage_header` / `feature_usage_header` where applicable (requirement/case/analysis/verification/action/state/view/rendering/viewpoint/use-case usages, concern usage, calc definition prefix).
- **Specialization targets**: subsetting and redefinition accept qualified names with dotted feature chains and comma-separated target lists.
- **Validation tests**: `parts_tree_1a`, `parts_interconnection_2a`, `function_based_behavior_3a`, and `functional_allocation_4a` refactored to snapshot-based checks aligned with structured AST shapes.

### Fixed

- **Port nested bodies**: `port` usages inside `port` brace bodies (e.g. left/right redefinitions) parse into `PortBodyElement::PortUsage` instead of a separate `BraceWithPorts` shape.
- **Library typing headers**: `defined by` and `typed by` accepted alongside `:` on usage headers; multiple `:>` / `:>>` / `subsets` / `redefines` clauses parse without spurious recovery on common stdlib patterns.
- **Part `ref` lines**: optional comments and formatting around `ref part` assignments tolerated in part usage bodies.

### Migration (Spec42 and similar hosts)

1. Bump the `sysml-v2-parser` dependency to `0.15.0` (or the matching git revision / path).
2. Replace `PortBody::BraceWithPorts` matches with `PortBody::Brace` and handle `PortBodyElement` (nested ports are `PortBodyElement::PortUsage`).
3. If you read attribute or generic definition brace bodies as opaque text, switch to iterating `AttributeBodyElement` / `DefinitionBodyElement` (or keep using span recovery `Error` / `Other` members for unsupported inner forms).
4. For usage typing and specialization, prefer `usage_header` semantics: `references` / `crosses` may appear in the same clause stream as subsets (stored via the shared specialization path where the AST has a single subsets slot).
5. Run `cargo test`, `cargo test --test bnf_compliance`, and `cargo test --test validation -- --include-ignored` with `SYSML_V2_RELEASE_DIR` pointing at the release BNF tree.

[0.15.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.14.0...v0.15.0

## [0.14.0] - 2026-06-02

### Added

- **Qualified package identifiers**: package and namespace declarations now accept qualified names in the identification position (e.g. `package AstronomyReference::Domain { ... }`) and keep the full qualified path in the AST.
- **`ref part` assignment forms**: part usage bodies now parse `ref part` declarations with optional typing and optional value binding (e.g. `ref part centralBody = sun;`, `ref part orbitingBody : Body = earth;`) without recovery diagnostics.

### Fixed

- **Reference usage grammar coverage**: `ref part` declarations that omit explicit typing are no longer forced into a `:` parse path, aligning parser behavior with SysML v2 reference-usage notation.

### Migration (Spec42 and similar hosts)

1. Bump the `sysml-v2-parser` dependency to `0.14.0` (or the matching git revision / path).
2. If downstream code assumes `package`/`namespace` names are unqualified, update it to handle `::`-qualified identifiers in `Identification.name`.
3. Re-run parser and semantic smoke tests that cover `ref part` declarations with and without type annotations.

[0.14.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.13.0...v0.14.0

## [0.13.0] - 2026-06-01

### Breaking

- **Definition subclassification on AST nodes**: many `*Def` types now include `specializes: Option<String>` and `specializes_span: Option<Span>` when a declaration uses `:>` / `specializes` or a library-style typed header before subclassification (e.g. `abstract connection name : Connection[0..*] :> linkObjects, parts`). Affected types include (among others) `ItemDef`, `IndividualDef`, `InterfaceDef`, `ConnectionDef`, `PortDef`, `RequirementDef`, `ConstraintDef`, `StateDef`, `ActionDef`, `FlowDef`, `AllocationDef`, `MetadataDef`, `OccurrenceDef`, `EnumDef`, and the case/view/use-case definition families. Any manual struct literals or exhaustive construction must set these fields (`None` when absent).

### Added

- **Shared definition prelude** ([`src/parser/definition_prefix.rs`](src/parser/definition_prefix.rs)): `parse_definition_prefix` with `DefinitionPrefixOptions` centralizes `abstract`, optional `private`, optional `#` annotation, keyword/`def`, and header-after-ident parsing for migrated definition parsers.
- **Shared opaque body terminator** ([`src/parser/body.rs`](src/parser/body.rs)): `semicolon_or_opaque_brace_body` for `;` or brace bodies whose inner content is skipped (`flow`, `allocation`, `metadata`, and related usages).
- **Header helper** ([`src/parser/specialization.rs`](src/parser/specialization.rs)): `parse_optional_definition_header_after_identification` handles direct `:>` / `specializes` and typed headers (`: Type[multiplicity] … :> bases`) after `identification`.
- **Docs**: [`docs/PARSER_TECHNICAL_DEBT.md`](docs/PARSER_TECHNICAL_DEBT.md) and [`docs/PARSER_DEBT_P1_PLAN.md`](docs/PARSER_DEBT_P1_PLAN.md) document parser duplication, P1 consolidation (complete), and follow-up P2/P3 work.

### Changed

- **Internal refactor (P1)**: eighteen `*_def` entry points (item, individual, interface, metadata, connection, constraint, port, requirement, state, occurrence, flow, allocation, case/analysis/verification, view/viewpoint/rendering, use case, enum, action) delegate their prelude to `parse_definition_prefix`. `part_def`, `calc_def`, usages, `alias_def`, and `dependency` remain on local preludes by design.
- **Numeric literals**: decimal and scientific-notation forms are parsed more consistently in expression paths.

### Fixed

- **Systems / full library gates**: declarations such as `abstract connection … : Type[…] :> …` and `private abstract constraint def …` map to dedicated definition nodes again (`ExtendedLibraryDecl` count stays at zero with `cargo test -- --include-ignored`).
- **Calc and constraint bodies**: `return` expressions in calculation definitions and constraint bodies parse without swallowing following members.
- **Definition prefix modifier order**: `private` is accepted before `abstract` (stdlib `private abstract constraint def`).

### Migration (Spec42 and similar hosts)

1. Bump the `sysml-v2-parser` dependency to `0.13.0` (or the matching git revision / path).
2. Update any manual `*Def` struct literals to include `specializes` and `specializes_span` (use `None` when not modeled).
3. When building semantics from definitions, read `specializes` / `specializes_span` for subclassification edges; typed library headers populate `specializes` from the `:>` clause after the skipped typing fragment.
4. Re-run `cargo test --test validation -- --include-ignored` after upgrading.

[0.13.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.12.0...v0.13.0

## [0.12.0] - 2026-05-28

### Breaking

- **`AttributeUsage`**: added `typing: Option<String>` and `typing_span: Option<Span>` for the type after `:` or `:>` on attribute usages (e.g. `attribute totalMassKg : MassValue`). Any struct literals or manual construction of `AttributeUsage` must set these fields (use `None` when untyped).

### Fixed

- **Typed attribute usages in usage bodies**: `attribute` name followed by `:` or `:>` and a qualified type name now parses as `AttributeUsage` with `typing` populated, including inside `part` usage bodies. Previously the parser rejected this form in usage contexts (recovery / wrong classification). This matches OMG SysML v2 `AttributeUsage = UsagePrefix 'attribute' Usage`, where typing is part of the usage, not only of `attribute def`.
- **Attribute def vs usage disambiguation**: in definition bodies (`part def`, `port def`, `requirement def`), the parser tries `attribute def` before `attribute usage` so typed declaration members such as `attribute mass :> ISQ::mass` remain `AttributeDef`. Untyped value assignments (`attribute actualMass = measuredMass`), `redefines` / `:>>` forms, and prefix redefinitions (`attribute :>> propellantMass = …`) still parse as `AttributeUsage`. Package- and use-case-level attributes are unchanged (`attribute x = expr` stays `AttributeDef`). Fixes validation fixture `1a-Parts Tree.sysml` and similar library models.
- **`:>` vs `:>>` on attributes**: attribute typing no longer treats `:>>` as a `:>` prefix. Prefix-redefine usages (`attribute :>> currentTime : TimeInstantValue`) accept an optional `: Type` after the redefine target; a following `:>` is left for subsetting (e.g. `attribute :>> outlet :> electricGrid.outlets`). `attribute def` requires a declared name so bare `attribute :>> …` is not misclassified.

### Migration (Spec42 and similar hosts)

1. Bump the `sysml-v2-parser` dependency to `0.12.0` (or the matching git revision).
2. Update `AttributeUsage` struct literals to include `typing` and `typing_span`.
3. When building semantics from attribute usages, read `AttributeUsage::typing` for type edges in **usage** bodies (e.g. nested `part` usages).
4. In **definition** bodies, typed members without `def` (e.g. `attribute massActual: MassValue` in `requirement def`) continue to surface as `AttributeDef`; do not assume every typed `attribute` is an `AttributeUsage`.
5. Re-run `cargo test --test validation -- --include-ignored` after upgrading; the full validation and std-library gates should be green.

[0.12.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.11.0...v0.12.0

## [0.11.0] - 2026-05-28

### Breaking

- **`UseCaseDefBodyElement`**: added new variant `AttributeDef(Node<AttributeDef>)` so that `attribute` definitions inside a `use case def` body are surfaced in the AST. Any exhaustive `match` on `UseCaseDefBodyElement` must add an arm for `AttributeDef`.

### Fixed

- **Transition names vs transition keywords**: optional transition names such as `docked` are no longer dropped when the name shares a prefix with `first`, `if`, `do`, or `then`. The parser now uses whole-keyword detection (`starts_with_keyword`) so `transition docked first docking then charging;` parses correctly.

### Migration (Spec42 and similar hosts)

1. Bump the `sysml-v2-parser` dependency to `0.11.0` (or the matching git revision).
2. If you exhaustively match on `UseCaseDefBodyElement`, add an arm for the new `AttributeDef` variant (carry-through is usually identical to the existing `AttributeDef` arms on `PartDefBodyElement` or `RequirementDefBodyElement`).

[0.11.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.10.0...v0.11.0

## [0.10.0] - 2026-05-13

### Breaking

- **`PartDefBodyElement`**: added new variant `InterfaceDef(Node<InterfaceDef>)` so that nested `interface def` declarations inside a `part def` body are surfaced in the AST. Any exhaustive `match` on `PartDefBodyElement` must add an arm for `InterfaceDef`.
- **`parse_root` strict mode**: a stray trailing `}` after a well-formed root namespace is now reported as `unexpected_closing_brace` instead of being silently accepted. Inputs that previously parsed under `parse_root` but contained extra closing braces will now return an error (these inputs already produced diagnostics from `parse_with_diagnostics`).

### Added

- **Nested `interface def` in part definitions**: `part def` bodies now accept nested `interface def` (and continue to accept `interface` usages), matching the OMG SysML v2 Part 1 textual grammar which lists `InterfaceDefinition` under `DefinitionElement`. New fixtures cover the nested form and assert no recovery diagnostics.
- **Diagnostic code `invalid_bare_identifier_in_action_body` / `invalid_bare_identifier_in_state_body`**: bare identifiers in action and state bodies (e.g. `action a { batCap; maxBatCap; }`) now produce a targeted message naming valid forms (`perform`, `bind`, `in`/`out`, `entry`, `transition`, `then`, …) instead of the generic `unexpected_keyword_in_scope`.
- **Diagnostic code `recovery_cascade_suppressed`**: after three consecutive `missing_semicolon` or `recovered_*` diagnostics in the same body region, a single warning-severity summary replaces the remaining cascade entries, pointing back to the first error to fix.
- **Diagnostic code `recovered_root_body`**: when a root-level `package` / `library` / `standard package` / `namespace` body fails to parse, the recovery path emits one root-scoped error and skips to the next root element, preventing cascades across unrelated definitions in the same file.
- **Docs**: new `docs/CORPUS_MBSE_VACUUM_PARSER_SPEC42_FEEDBACK.md` capturing findings from running the parser/Spec42 stack against the public MBSE vacuum-cleaner robot example, plus a documentation index in `README.md`.

### Fixed

- **`interface` usage with no whitespace before `:`**: `interface : Foo;` (and similar forms without a space between the keyword and the colon) is now accepted.
- **`comment` annotation prefixes**: `comment` annotations tolerate arbitrary tokens between the optional name/about clauses and the opening `/* … */` comment body, matching real-world inputs that include extra metadata before the comment.
- **Part / state body recovery**: classification codes `invalid_bare_identifier_in_action_body`, `invalid_bare_identifier_in_state_body`, `unexpected_keyword_in_scope`, `missing_semicolon`, and `missing_body_or_semicolon` now produce `Other` placeholder elements in `PartDefBody` / `StateDefBody` so downstream tooling can see the skipped span.

### Reliability

- Cascade suppression and the root-body recovery error together significantly reduce diagnostic volume on large real-world corpora where a single structural error previously fanned out into hundreds of follow-up `missing_semicolon` / `recovered_*` entries.

### Migration (Spec42 and similar hosts)

1. Bump the `sysml-v2-parser` dependency to `0.10.0` (or the matching git revision).
2. If you exhaustively match on `PartDefBodyElement`, add an arm for the new `InterfaceDef` variant (carry-through is usually identical to the existing `InterfaceUsage` arm).
3. Diagnostic consumers can opt to treat `recovery_cascade_suppressed` as informational (it carries `severity: Warning`) and to display `recovered_root_body` as the primary error for affected root scopes.

[0.10.0]: https://github.com/elan8/sysml-v2-parser/compare/v0.9.0...v0.10.0

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
