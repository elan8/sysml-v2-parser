# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
