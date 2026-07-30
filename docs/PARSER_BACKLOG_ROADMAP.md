# Parser backlog & roadmap

**Single entry point** for open work on `sysml-v2-parser` and the Spec42 diagnostics integration. Historical plans remain as references; this document is updated when items open or close.

**Last updated:** 2026-07-30 (§6 fully closed — rebased [PR #3](https://github.com/elan8/sysml-v2-parser/pull/3)
(GH-2's `parse_root` verdict-parity fix) onto the completed G1–G30 work and found one last gap:
package/item-def bodies had no `#`/`@` annotation or `connect a to b;` support at all, needed by
`14c-Language Extensions.sysml`'s FMEA library example. Closed with a new `metadata_keyword_prefix`
function (kept separate from `metadata_keyword_usage` to avoid regressing `hash_annotation`'s
opaque-capture fallback used elsewhere) plus `PackageBodyElement`/`AttributeBodyElement::{Connect,
MetadataKeywordUsage}`. `cargo test --test validation -- --include-ignored` is now a genuine
56/56 with #3 applied — **§6 is closed, #3 is ready to merge.** See
[§6](#6-strict-vs-recovery-verdict-parity-2026-07-30-audit).)

**Previously:** 2026-07-30 (§6 G4–G20 closed — constraint/variation/perform-parameter/transition/
flow/loop/exhibit/import/allocate/anonymous-action gaps from the 2026-07-30 audit; surfaced
G21–G30 as narrower follow-ups discovered while closing them; `PARSE_AST_VERSION` 51 → 52.)

**Previously:** 2026-07-30 (§6 G2/G3 closed — `connection`/`assert constraint` usage wired into
`PartUsageBodyElement`, plus the multiplicity/name gaps found underneath them
(`ConnectionUsageMember.multiplicity`, `AssertConstraintMember.name`); `PARSE_AST_VERSION` 48 → 49.
Surfaced two narrower follow-up gaps as G19/G20. 22 of the original 25 spec-Annex files in §6
still fail; **Spec42 v1.0 remains blocked** — see [§6](#6-strict-vs-recovery-verdict-parity-2026-07-30-audit).)

**Previously:** 2026-07-30 (§6 G1 closed — `perform <path>` now accepts a `;` body and an
optional `:>>` redefinition clause; `PARSE_AST_VERSION` 47 → 48. Surfaced three narrower follow-up
gaps as G16-G18. 24 of the original 25 spec-Annex files in §6 still fail; **Spec42 v1.0 remains
blocked** — see [§6](#6-strict-vs-recovery-verdict-parity-2026-07-30-audit).)

**Previously:** 2026-07-30 (§6 opened — full-validation-suite audit found 25 real OMG spec Annex example files (of 56) hit grammar gaps that were previously masked by `parse_root` silently accepting them; see [§6](#6-strict-vs-recovery-verdict-parity-2026-07-30-audit) for the file-by-file breakdown. **Blocking Spec42 v1.0** — see that section for why and the release-gate note.)

**Previously:** 2026-07-30 (diagnostics-spec-audit — root `PackageBodyElement*` accepted; `missing_member_name` / `illegal_top_level_definition` removed as non-spec; bare `name : Type;` is `DefaultReferenceUsage` AST. Shared specialization layer + full `#`/`@` metadata surface remain open.)

**Previously:** 2026-07-23 (0.46.0 — Systems Library `ref action` / `ref state` / nested action·state in part bodies parse as real `ActionUsage`/`StateUsage` AST with `is_reference`, structured typing/multiplicity/`:>`/` :>>`, instead of `OpaqueMember`. Visibility on plain `part_ref_usage`. Full P5+ unified definition/usage/specialization layer remains deferred to 1.x.)

**Previously:** 2026-07-20 (0.44.0 — `Intersecting` closed: `intersects` clauses were tokenized and discarded (`skip_intersects_clause`); now kept structured as `AttributeUsage`/`PortUsage`/`OccurrenceUsage::intersects`, same shape as `references`/`crosses`. Investigated and ruled out as parser gaps in the same pass: `TypeFeaturing`/`FeatureInverting`/`Unioning`/`Disjoining`/`Differencing`/`FeatureChaining`-as-metaclass have zero real usage in the systems library or examples (backlog, not urgent); general non-port Conjugation needs no parser change at all — `~` conjugated typing already parses generically for every usage kind, the remaining gap is Spec42-side only. See CHANGELOG.md 0.44.0 and `babel42-v2/docs/spec42-systems-modeling-api-gaps.md` S42-002/S42-008.)

**Previously:** 2026-07-03 (§5 — all 7 open follow-ups from the 2026-07 audit closed: if/while/terminate, standalone succession, transition trigger `via`, satisfy inline requirement form, assert/satisfy scope wiring, arrow-invocation operator, AssignStmt.rhs. §2.3 — `doc` support added for port usage, connection def, and interface usage connect bodies; `connection def` body recovery migrated to the shared structured-body loop)

## Spec42 v1.0 checklist

Items that must close before Spec42 can release v1.0. Everything else is 1.x.

### Diagnostic-blocking (parser)

- [x] `MetadataAnnotation` variant wired in **all** body enums where `@` can appear — §2.1 (done 0.28.0)
- [x] `missing_rep_language` / `invalid_rep_language` emitted by `textual_representation()` — §2.2 (done 0.28.0)
- [x] `rep` recognised in frame and concern bodies — already covered via `requirement_def_body()` (confirmed 0.28.0)
- [x] `CaseReturnDecl` models `return name : Type = expr` and `return :>> name` in analysis/verification bodies — §2.5 (done 0.28.0)

### Editor quality (LSP P0)

- [x] `ParseErrorNode` in view, constraint, and calc bodies — already deployed via `parse_structured_brace_members` (confirmed 0.28.0)
- [x] Silent reshaping audit — no `advance_to_closing_brace` in view.rs or constraint.rs (confirmed 0.28.0)
- [x] Recovery range-and-code tests for view def and constraint def bodies (done 0.28.0)

### Explicitly deferred to 1.x

- Diagnostic catalog string literals → named constants migration (§2.6)
- `rep` in package-adjacent frame/concern bodies beyond what a Spec42 diagnostic requires (§2.2)
- `head_span` gaps on non-critical annotation paths (§2.1)
- Unified definition/usage/specialization grammar layer (§4, P5+)
- Full `OwnedExpression` / complete KerML expression family (§2.4)
- User-defined declaration keywords (`metadata def` as header starter, deferred 1.5b)

---

## How to use this document

| If you want to… | Start here |
| ---------------- | ---------- |
| See **all open work** in one place | This file — sections below |
| Understand **what the parser already ships** for Spec42 | [§ Completed — Spec42 parser wave](#completed--spec42-parser-wave-june-2026) |
| Wire **Spec42 graph builders / collectors** | [§ 1 — Spec42 cross-repo](#1-spec42-cross-repo-follow-up) (**done** in Spec42 0.29.0) |
| Improve **editor / LSP** behavior | [§ 3 — Language server](#3-language-server--recovery) |
| Go deeper on **grammar fidelity** | [§ 4 — Grammar & compliance](#4-grammar-depth--compliance) |
| See the **state/action/connector fidelity pass** (2026-07) and its open follow-ups | [§ 5 — State machine, action & connector grammar fidelity](#5-state-machine-action--connector-grammar-fidelity-2026-07-audit) |
| See the **Spec42 v1.0-blocking grammar gap audit** (2026-07-30) and what's confirmed vs. still needs isolation | [§ 6 — Strict-vs-recovery verdict parity audit](#6-strict-vs-recovery-verdict-parity-2026-07-30-audit) |
| Read the **original Spec42 parser spec** | [SPEC42-DIAGNOSTICS-PARSER-IMPROVEMENTS.md](./SPEC42-DIAGNOSTICS-PARSER-IMPROVEMENTS.md) |

### Regression gates (every parser PR)

- `cargo test`
- `cargo test --test validation -- --include-ignored`
- `test_systems_library_strict_no_diagnostics` / `test_full_library_strict_no_diagnostics` (validation suite)
- `ExtendedLibraryDecl = 0` in library node-shape gates

### AST snapshot refresh (when AST shape changes)

CI runs the full validation suite (`cargo test -- --include-ignored`). Several fixtures compare against checked-in AST text under [`tests/validation/snapshots/`](../tests/validation/snapshots/). **Any PR that changes AST shape must refresh those snapshots in the same PR** — do not rely on the default `cargo test` alone (snapshot tests are `#[ignore]` unless `--include-ignored`).

Regenerate after changes such as:

- new or renamed AST fields (e.g. `value_span`, `MetadataAnnotation` variants)
- new body-element enum variants or different parse classification (e.g. `@` metadata vs generic `Annotation`)
- structured-body parsing replacing silent skip

```powershell
$env:UPDATE_VALIDATION_AST = "1"
cargo test --test validation -- --include-ignored
Remove-Item Env:UPDATE_VALIDATION_AST
```

See [`tests/validation/README.md`](../tests/validation/README.md) for layout and per-fixture commands. Review the snapshot diff before committing — it should reflect intentional parser output only.

---

## Status snapshot

| Area | State |
| ---- | ----- |
| CI & library gates | Green |
| BNF coverage map | 640/640 productions classified `implemented` |
| Spec42 diagnostics **parser AST** (P0–P2 wave) | **Largely done** — see completed table; partial items listed in § 2 |
| Spec42 **semantic** diagnostics (§1 wave) | **Done** in Spec42 0.29.0 — partial §2 items remain parser-side |
| Deep body fidelity | **Open** — many `advance_to_closing_brace` call sites remain |
| Full `OwnedExpression` | **Open** — operator enums added; full KerML expression family not modeled |
| Systems Library `ref <kind>` in part bodies | **Done** (0.46.0) — `ActionUsage`/`StateUsage` (+ Spec42 graph wire); not full P5+ |
| Unified definition/usage grammar layer | **Open** — P5+ architectural work (still deferred; do not big-bang rewrite) |

```mermaid
flowchart TB
  subgraph done [Parser done June 2026]
    ast[AST fields + spans]
    fixtures[spec42_diagnostics_ast tests]
  end
  subgraph open_parser [Parser open]
    partial[Partial items §2]
    bodies[Opaque bodies §2.3]
    expr[Expression depth §2.4]
  end
  subgraph open_spec42 [Spec42 open]
    graph[Graph builders]
    coll[Diagnostic collectors]
  end
  ast --> graph --> coll
  partial --> graph
```

---

## 1. Spec42 cross-repo follow-up

Parser changes unlock diagnostics only after Spec42 projects new AST fields. **Highest ROI** after the June 2026 parser wave.

| Diagnostic / theme | Parser AST (ready?) | Spec42 work | Spec42 doc |
| ------------------ | ------------------- | ----------- | ---------- |
| `accept_payload_incompatible` on transitions | Yes — `Transition.accept`, `PayloadClause` | Graph: transition trigger `payloadType`; collector | [DIAGNOSTIC-CHECKS-ROADMAP](https://github.com/spec42/spec42/blob/main/docs/engineering/DIAGNOSTIC-CHECKS-ROADMAP.md) |
| `send_payload_incompatible` | Yes — `ActionUsage.send` | Graph: control-node send payload | same |
| Final-state cardinality (`multiple_final_states`, …) | Yes — `FinalState` member | Graph: final-state edges; drop sink heuristics | same |
| `metadata_keyword_unresolved` (`#Tag`) | Yes — `MetadataKeywordUsage` (simple `#name`) | Walk new node in part/state/requirement builders | [AST-SEMANTIC-COVERAGE](https://github.com/spec42/spec42/blob/main/docs/engineering/AST-SEMANTIC-COVERAGE.md) |
| `viewpoint_reference_unresolved` (stakeholder/purpose) | Yes — `StakeholderMember`, `PurposeMember` | Extend collector for new ref spans | same |
| `viewpoint_rep_language_unresolved` | Partial — `rep` in requirement body | Wire `TextualRep` + `language_span` in graph | same |
| `transition_guard_non_boolean`, filters, assignments | **Done** — `Expression::Classification`, `exprClass` AST walk (0.23.0) | — | same |
| Typed `stakeholder name : Type` | **Done** — `StakeholderMember` with optional typing (0.23.0) | Graph: `stakeholderType` + typing edge | same |
| `assignment_value_incompatible` in case bodies | **Done** — `AttributeDef.value_span` + verification graph (0.23.0) | — | same |
| Initial state via `first` | Yes — `Transition.is_initial` | Align with `ThenStmt` initial edges | same |

**Release train:** parser release → Spec42 graph_builder PR → collector + catalog entry → move item **Deferred → Done** in Spec42 roadmap.

---

## 2. Parser — open & partial (Spec42 wave)

Items from [SPEC42-DIAGNOSTICS-PARSER-IMPROVEMENTS.md](./SPEC42-DIAGNOSTICS-PARSER-IMPROVEMENTS.md) that are **not fully closed** in the parser.

### 2.1 Metadata & annotations

| Item | Status | Remaining work |
| ---- | ------ | -------------- |
| `#keyword` in bodies | **Done** (simple `#Tag;`) | — |
| Extended `#refinement dependency …` | **Done** (opaque `Annotation`) | — |
| User-defined **declaration** keywords (`metadata def` short name as header starter) | **Not started** (deferred 1.5b) | Dynamic dispatch in `feature_decl` / `classifier_decl`; package-local metadata def index |
| `MetadataAnnotation` in all bodies | **Partial** | Part **def**, state, requirement, part usage, action bodies; constraint `@` still generic `Annotation` in some paths |
| `head_span` on all annotation usages | **Partial** | Wired on parse paths; not all body enums expose `MetadataAnnotation` variant |

### 2.2 TextualRepresentation (`rep`)

| Item | Status | Remaining work |
| ---- | ------ | -------------- |
| `rep` in requirement / viewpoint body | **Done** | Fixture: `tests/fixtures/requirement-rep-language.sysml` |
| `rep` in frame, concern, package-adjacent bodies | **Partial** | Package-level `TextualRep` exists; frame/concern may need explicit wiring |
| `language_span` | **Done** on parse path | — |
| Parser diagnostics `missing_rep_language` / `invalid_rep_language` | **Catalog only** | Constants in [`diagnostic_catalog.rs`](../src/parser/diagnostic_catalog.rs); not emitted by `textual_representation()` yet |

### 2.3 Opaque brace-body skipping

**Problem:** Unmodeled inner regions are invisible to Spec42 and the LSP.

| Module | `advance_to_closing_brace` uses (approx.) | Priority |
| ------ | ---------------------------------------- | -------- |
| `action.rs` | 0 (was 7) | High — behavior / control nodes |
| `requirement.rs` | 0 (was 4) | High |
| `state.rs` | 0 (was 2) | Medium (transition connect bodies unified) |
| `part/usage.rs` | 0 (was 3) | Medium |
| `usecase.rs` | 0 (was 2) | Lower — structured case bodies + return-ref expressions |
| `connection.rs` (top-level `connection def` body loop) | 0 (was 1, 2026-07) | Was untracked in this table; `connection_member_body` had its own hand-rolled loop whose only fallback for an unrecognized member was `advance_to_closing_brace`, silently discarding every member declared *after* the bad one, not just the bad one — worse than a plain opaque skip. Migrated to `parse_structured_brace_members` (added `ConnectionDefBodyElement::Error`); now recovers per-element like `port_body_brace`. Nested `ref`/`connect`-statement bodies inside a connection def (`ref_body`, `connect_body`) still use `advance_to_closing_brace` and remain opaque — smaller, separate scope. |

**Direction:** Per construct family, replace silent skip with `ParseErrorNode` + partial member lists ([LANGUAGE_SERVER_BACKLOG.md](./LANGUAGE_SERVER_BACKLOG.md) P0). One family per PR; track remaining sites here. When auditing a family, also check for **whole-body-truncation** fallbacks like the one fixed in `connection.rs` — not just `advance_to_closing_brace` call counts, since a hand-rolled recovery loop can have the same effect without calling that function directly.

### 2.4 Expression AST

| Item | Status | Remaining work |
| ---- | ------ | -------------- |
| Operator classification | **Done** — `BinaryOperator`, `UnaryOperator` | — |
| `@Metaclass` classification | **Done** — `Expression::Classification` | Spec42 `exprClass` on filter/guard nodes |
| `istype` / `hastype` / `as` | **Done** — `Expression::TypeCheck` | Filter/guard contexts |
| `select` / `collect` | **Done** — `Expression::Select` / `Collect` | — |

### 2.5 Case & verification bodies

| Item | Status | Remaining work |
| ---- | ------ | -------------- |
| `AttributeDef.name_span` in case bodies | **Done** | — |
| `value_span` on `AttributeDef` | **Done** | Populated on parse path; verification/analysis graph builders project local attributes |
| Verdict / return forms, `:>>` in analysis bodies | **Partial** — `ReturnRef.return_expression`, structured `ref :>>` bodies | Typed objective + remaining library `:>>` nesting |

### 2.6 Parser diagnostic contract

| Item | Status | Remaining work |
| ---- | ------ | -------------- |
| `diagnostic_catalog.rs` | **Done** (registry file) | Wire constants into `diagnostics.rs` / `recovery.rs` instead of string literals |
| Range-text regression tests | **Partial** | `recovery_diagnostics_integration.rs` exists; add transition/import/type range matrix |
| Scope labels (`"state body"`, …) | **Done** in major bodies | Extend to nested families per § 2.3 |

---

## 3. Language server & recovery

Consolidated from [LANGUAGE_SERVER_BACKLOG.md](./LANGUAGE_SERVER_BACKLOG.md). **Not duplicated** — see that file for narrative detail.

| Priority | Theme | Open? |
| -------- | ----- | ----- |
| P0 | Tighten recovery diagnostics (`expected` / `suggestion` precision) | Yes |
| P0 | Expand `ParseErrorNode` to view/constraint/calc nested scopes | Yes |
| P0 | Remove silent reshaping on malformed input | Yes |
| P0 | Recovery tests per construct (codes + ranges + siblings) | Partial — good baseline, gaps in views/constraints |
| P1 | Normalize recovery loops across modules | Partial — `parse_structured_brace_members` exists |
| P1 | Finer grammar-aware sync helpers | Yes |
| P1 | Span robustness under recovery | Yes |
| P2 | Strict vs resilient parse path separation (internal) | Yes |
| P2 | Richer error infrastructure (`nom-supreme`, custom state) | Investigate |

---

## 4. Grammar depth & compliance

Consolidated from [SYSML_V2_COMPLIANCE_GAP.md](./SYSML_V2_COMPLIANCE_GAP.md) and [PARSER_TECHNICAL_DEBT.md](./PARSER_TECHNICAL_DEBT.md).

| Theme | Priority | Notes |
| ----- | -------- | ----- |
| Unified definition / usage / specialization grammar layer | **P5+** | Largest architectural gap; do not big-bang rewrite. The 0.46.0 `ref action`/`ref state` part-body slice is **not** this rewrite — only the Systems Library forms that were opaque catch-alls. |
| `DefaultReferenceUsage` AST (bare `name : Type;`) | **Done** (diagnostics-spec-audit) | Part def/usage bodies; was mis-modeled as `AttributeUsage` via shorthand |
| Metadata `#` / `@` / user-defined keyword surface | Medium | Partial parse; remaining legal forms still hit `unsupported_annotation_syntax` (coverage Warning, not a language ban). §6's audit closed the `PrefixMetadataMember`-style `#<tag>` prefix syntactically (`metadata_keyword_prefix`, package/attribute bodies) — still doesn't resolve whether `<tag>` is an actually-declared `metadata def <tag>` short name (that semantic check + a package-local short-name index is the remaining 1.5b item, §2.1) |
| Part-body `ref action` / `ref state` (Systems Library) | **Done** (0.46.0) | Real `ActionUsage`/`StateUsage` with `is_reference`, `:>`/` :>>`, visibility on plain `ref` |
| `take_until_terminator` header scraping → structured headers | Medium | Per-family as library fixtures expose gaps |
| `part_def` prelude unify with `definition_prefix` | Low | Intentionally local for disambiguation |
| `package_body_element` sub-dispatchers | **Done** (P2) | Maintain when adding keywords |
| AST shape dedup (`DefinitionDecl` internal) | P5+ | Drive from grammar work |
| Semantic conformance (types, resolution) | Out of scope | Spec42 / other tools |

---

## 5. State machine, action & connector grammar fidelity (2026-07 audit)

An ad-hoc spec-vs-parser audit (state body `entry`/`do`/`exit`, then broadened to
control nodes, transitions, requirements, and connectors) found and closed several
gaps where valid SysML v2 textual syntax fell through to opaque `Other`/`Error`
recovery nodes instead of a real AST shape. Closed in this pass:

| Item | AST / parser | Notes |
| ---- | ------------- | ----- |
| State `entry`/`do`/`exit` actions | `EntryAction`, `DoAction`, `ExitAction`, `StateDefBodyElement::{Entry,Do,Exit}` — [state.rs](../src/parser/state.rs) | Previously only `entry` was implemented |
| Control nodes `decide`/`join`/`fork` | `DecisionStmt`, `JoinStmt`, `ForkStmt` — [action.rs](../src/parser/action.rs) | Keyword list had a typo (`decision` instead of spec's `decide`); `join`/`fork` had no parser at all |
| `assert not` / negated satisfy | `AssertConstraintMember.is_negated`, `Satisfy.is_negated` — [occurrence_body.rs](../src/parser/occurrence_body.rs), [requirement.rs](../src/parser/requirement.rs) | `satisfy()` now accepts optional `assert`/`not` prefixes; bare `not satisfy X by Y;` (spec §7.x example) also parses |
| Transition `do` effect | `TransitionEffect::{Perform,Accept,Send,Assign,Expression}` — [state.rs](../src/parser/state.rs) | Previously a raw `expression`; now recognizes `do action name : Type`, `do accept/send payload (via/to expr)?`, `do assign lhs := rhs` |
| `for` loop range | `ForLoop.range: Node<Expression>` (was `String`) — [action.rs](../src/parser/action.rs) | Falls back to raw text only when the expression grammar can't parse the range (see open item below) |
| `assign` LHS | `AssignStmt.lhs: Node<Expression>` (was `String`) — [action.rs](../src/parser/action.rs) | RHS is still raw `String` (out of scope for this pass) |
| N-ary connector/interface | `ConnectStmt.extra_ends: Vec<Node<Expression>>` — [interface.rs](../src/parser/interface.rs), [connection.rs](../src/parser/connection.rs) | `connect (a, b, c);` now parses; binary `from ... to ...` unaffected |

### Open follow-ups from this pass — closed 2026-07-03

All seven items below were closed in a follow-up pass. Kept here (marked done) so the
audit trail from the original pass isn't lost.

- ~~**`if` / `while` / `terminate` control nodes**~~ — **Done.** `IfStmt`, `WhileStmt`, `TerminateStmt` + matching `ActionDefBodyElement`/`ActionUsageBodyElement` variants — [action.rs](../src/parser/action.rs), [behavior.rs](../src/ast/behavior.rs). `if`/`while` bodies are fully structured (`ActionDefBody`, reusing `action_def_body_brace`), not the opaque `FirstMergeBody` `decide`/`join`/`fork` use. `terminate` accepts an optional target expression. Both dispatcher `alt()`s needed nesting into a sub-`alt()` to stay under nom's 21-branch limit.
- ~~**Standalone `succession` usage**~~ — **Done.** New `SuccessionUsage` AST node, `occurrence_body.rs`. Also supports the multiplicity-bearing form actually used by the SysML Systems Library (`succession [seBeforeNum] first [0..1] sourceEvent then [0..1] self;` in `Flows.sysml`) — discovered via the strict library gate failing on the first cut of this parser, which only handled the bare form.
- ~~**Transition trigger `accept ... via port`**~~ — **Done.** `TransitionAccept::Payload`/`Shorthand` now each carry `Option<Node<Expression>>` for `via` — [payload.rs](../src/parser/payload.rs).
- ~~**`satisfy requirement <name> : <Type> by <expr>`**~~ — **Done.** `Satisfy.inline_requirement: Option<InlineSatisfyRequirement>`, reusing `optional_typings()` from [usage.rs](../src/parser/usage.rs) — [requirement.rs](../src/parser/requirement.rs).
- ~~**`assert constraint` / `satisfy` body wiring inconsistent across scopes**~~ — **Done.** Both now reachable from `part def` bodies (`PartDefBodyElement::AssertConstraint`/`Satisfy`) and `satisfy` also from `occurrence def` bodies (`OccurrenceBodyElement::Satisfy`) — [part/body.rs](../src/parser/part/body.rs), [occurrence_body.rs](../src/parser/occurrence_body.rs).
- ~~**KerML arrow-invocation operator** (`x->size()`)~~ — **Done.** New `->` branch in `postfix()` — [expr.rs](../src/parser/expr.rs). Desugars into the existing `Expression::MemberAccess`/`Invocation` shapes rather than a new variant, so no downstream exhaustive matches needed updating.
- ~~**`AssignStmt.rhs` still a raw `String`**~~ — **Done**, unblocked by the arrow-invocation operator above — [action.rs](../src/parser/action.rs), [behavior.rs](../src/ast/behavior.rs).

No Spec42 diagnostic currently depends on any of these; cross-check against § 1 before wiring Spec42-side consumers.

---

## 6. Strict vs. recovery verdict parity (2026-07-30 audit)

**Was blocking Spec42 v1.0; now closed.** [GH-2](https://github.com/elan8/sysml-v2-parser/issues/2)
reported that `parse`/`parse_root` (strict) and `parse_for_editor`/`parse_with_diagnostics`
(recovery) disagree on whether a document is valid: strict silently accepted documents where the
grammar had internally given up on a body member and embedded a recovery placeholder in the AST,
because `parse_root` never walked the AST for those placeholders the way `parse_with_diagnostics`
already did via `collect_recovery_errors`. A fix (`parse_root` now runs that same walk and rejects
if any placeholder is present) is on
[`fix/gh-2-strict-recovery-verdict-parity`](https://github.com/elan8/sysml-v2-parser/pull/3).

It was **held, not merged** for a while: running it against the full (normally `#[ignore]`d)
validation suite (`cargo test --test validation -- --include-ignored`) originally dropped
`full_validation_suite::test_full_validation_suite` from 56/56 to **31/56**. The 25 failing files
were official OMG SysML v2 spec Annex examples using real, valid constructs the grammar didn't
support yet in specific nested contexts — `parse_with_diagnostics` was already flagging all 25 as
invalid before that fix; `parse_root` was just the one silently disagreeing. Merging the verdict
fix without first closing (most of) these gaps would have made `parse()` reject ~45% of realistic
spec-conformant models.

**Now unblocked: the table below closed all 25, plus the last remaining gap it surfaced
(`14c-Language Extensions.sysml`'s `#`-prefix and `connect a to b;` support in package/item-def
bodies, found while rebasing #3 onto the G1–G30 work — see CHANGELOG.md). The full validation
suite is a genuine 56/56 with #3's `parse_root` fix applied; #3 is ready to merge.**

**This section is the file-by-file audit of those 25 failures**, grouped by root-cause construct
family. Each group lists its confidence: **Confirmed** means isolated outside the real file with a
minimal repro that reproduces the exact failure; **Suspected** means the failing file's reported
error position is consistent with the listed cause but recovery's sync-point-skip can attribute a
diagnostic to a line *after* the true failure, so a minimal repro is still needed before treating
it as scoped.

| # | Root cause (construct family) | Confidence | Files (of the 25) | Example (from the real file) |
| - | ------------------------------ | ---------- | ------------------ | ----------------------------- |
| G1 | `perform <path>` (part usage body, no `action` keyword) requires a brace body — `perform_body()` has no `;` alternative — and has no `:>>` redefinition clause at all (`Perform` AST has no `redefines` field) | **Done** | Was 4 | `perform 'provide power';` / `perform providePower.generateTorque :>> generateTorque;` |
| G2 | `connection <name> : Type[mult];` **usage** form (as opposed to `connection def`, or the `connect a to b;` shorthand) is wired into `PartDefBodyElement` (`connection_usage_member`) but not `PartUsageBodyElement` | **Done** | Was 2 | `connection trailerHitch : TrailerHitch[0..1];` |
| G3 | `assert constraint { }` / `assert constraint <name> { }` is wired into `PartDefBodyElement` (closed in the [§5 audit](#5-state-machine-action--connector-grammar-fidelity-2026-07-audit)) but not `PartUsageBodyElement` | **Done** | Was 2 | `assert constraint engineSelectionRational { }` |
| G4 | `constraint <name>[: Type] { }` **usage** keyword form is wired at package level (`ConstraintDef`/`ConstraintUsage`) but not inside `part def` bodies | **Done** | Was 2 | `constraint discBrakeConstraint : DiscBrakeConstraint { }` |
| G5 | `variation` prefix not recognized before `perform`/`requirement` members in part usage bodies (only `part`/`item` currently accept it) | **Done** | Was 2 | `variation requirement engineRqtChoice : EnginePerformanceReq` |
| G6 | Suspected: parameter-direction forms inside a `perform { }` body — `in item '<quoted name>' : Type { }` and `in part :>> name = value;` — aren't recognized by `perform_body_element`; outer `perform` itself parses fine in isolation | **Done** | Was 2 | `in part :>> testVehicle = vehicleUnderTest;` inside a `perform vehicleMassTest { }` body |
| G7 | Suspected: `event occurrence <name>;` (or a related occurrence-usage prefix) not recognized inside a `part` usage body nested in an `occurrence def` body; the outer `part producer[1] { }` parses fine empty in isolation | **Done** | Was 2 | `event occurrence publish_source_event;` inside `part producer[1] { }` |
| G8 | `transition '<quoted name>'` followed by the full `first <src> accept <trigger> then <target>;` structure isn't recognized as one declaration — grammar expects `;` immediately after the name (only bare `transition <name>;` or an unnamed `first ... then ...;` are supported, not combined) | **Done** | Was 2 | `transition 'normal-maintenance'` / `first normal` / `accept at vehicle1_c1.maintenanceTime` / `then maintenance;` |
| G9 | `value :>> name : Type;` — `value` is a real SysML usage-kind keyword (parallel to `part`/`item`/`ref part`) not wired in attribute-def bodies; the sibling `ref part :>> elements : SparePart;` form in the same file already works | **Done** | Was 1 | `value :>> elements: Integer;` |
| G10 | Suspected: `attribute occurs[0..1]: Real;` inside an `occurrence def` body — possible keyword/identifier collision on the name `occurs`, or a gap specific to attribute usage inside occurrence-def bodies | **Done** | Was 1 | `attribute occurs[0..1]: Real;` |
| G11 | Suspected: `port :>> name = value { body }` — redefinition + bound value + brace body combined isn't recognized by `port_usage`, though each piece works individually elsewhere | **Done** | Was 1 | `port :>> pe = c1.pb { doc /* ... */ }` |
| G12 | `flow of <name> : Type;` (alternate keyword order — `of` before the flow item, vs. the already-supported `flow <name> : Type from ... to ...;`) not recognized in part usage bodies | **Done** | Was 1 | `flow of  fuel : Fuel;` (double space in source) |
| G13 | Standalone `first <name>;` (an initial-node marker, no `then`) not recognized in action bodies — only the `first ... then ...;` succession form is | **Done** | Was 1 | `first start;` |
| G14 | `loop { }` control node not implemented at all (`decide`/`join`/`fork`/`if`/`while` were closed in the [§5 audit](#5-state-machine-action--connector-grammar-fidelity-2026-07-audit); `loop` was missed) | **Done** | Was 1 | `loop { ... }` |
| G15 | Suspected: `:>> name = value { body }` redefinition-with-value-and-body form inside an occurrence usage body (parallel to G11 but for occurrence, not port, usages) | **Done** | Was 1 | `:>> t = t0 { ... }` |
| G16 | *(found while fixing G1)* `private import '<quoted target>'::*;` not recognized inside a part usage body | **Done** | Was 1 | `private import 'vehicle1-c1 Specification'::*;` |
| G17 | *(found while fixing G1)* Nested `allocate <path> to <path>;` inside an allocation **usage**'s own brace body — not recognized by `DefinitionBodyElement` (`AllocationUsage`/`AllocationDef` bodies route through the shared `DefinitionBody`) | **Done** | Was 1 | `allocation allocation2 : Logical_to_Physical allocate torqueGenerator to powerTrain { allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque; }` |
| G18 | *(found while fixing G1)* `exhibit <name> :>> <target>;` — `exhibit state` usage with a `:>>` redefinition clause — not recognized in a part usage body | **Done** | Was 1 | `exhibit 'vehicle states' :>> VehicleA::'vehicle states';` |
| G19 | *(found while fixing G2/G3)* Anonymous `action { }` (no name) not recognized in a part usage body | **Done** | Was 1 | `action { // Create a link ... }` |
| G20 | *(found while fixing G2/G3)* Anonymous `perform action { }` (no name) not recognized — `perform_action_decl`'s `name(input)` call is mandatory, unlike the def-form parsers elsewhere in this table that already guard against a bare `def`/anonymous case | **Done** | Was 1 | `perform action { action 'connect trailer to vehicle' { ... } }` |
| G21 | *(found while closing G4–G20)* `<shortName>` on `attribute`/`part`/`item`/`port` **usage** members (BNF `Identification`) — only the `def` side parsed it | **Done** | — | `attribute <wcf> wheelCoordinateFrame : CoordinateFrame;` (`VehicleGeometryAndCoordinateFrames.sysml`) |
| G22 | *(found while fixing G4)* `occurrence :>> causes;` — anonymous occurrence redefinition without renaming | **Done** | — | `occurrence :>> causes;` (`14c-Language Extensions.sysml`) |
| G23 | *(found while fixing G13)* `then merge <name>;` / `then <name>;` succession shorthand (not just `then action …`) | **Done** | — | `then merge join1;` |
| G24 | *(found while closing G4–G20)* `connect [mult] a to [mult] b;` — per-endpoint multiplicity on binary connect | **Done** | — | `connect [0..1] a.p1 to [1] b.p2;` |
| G25 | *(found while closing G4–G20)* `item` def/usage members missing from `PartUsageBodyElement` (reachable from part *def* bodies only) | **Done** | — | nested `item` in part usage bodies |
| G26 | *(found while closing G6)* keyword-less `name = expr;` bindings in action/perform bodies (`DefaultReferenceUsage`) | **Done** | — | `measurement = testVehicle.mass;` (`9-Verification-simplified.sysml`) |
| G27 | *(found while closing G10)* `occurrence` members inside shared attribute/item bodies | **Done** | — | occurrence nested under `item def` bodies |
| G29 | *(found while closing G7)* `ref` prefix on occurrence usages (`ref individual :>> …`) | **Done** | — | `ref individual :>> vehicleUnderTest;` |
| G30 | *(found while closing G15/G18)* `exhibit (state)? <name> …` inside occurrence/snapshot **usage** bodies (part usage already had it via G18) | **Done** | — | `exhibit vehicleStates.on { ... }` (`6-Individual and Snapshots.sysml`) |

**Total: 25 files** originally, matching the `56 → 31` drop. G1's fix (see CHANGELOG.md) closed
the `perform` gap cleanly wherever it was the *only* problem in a file (`12b-Allocation-1.sysml`
→ fully clean now), but 3 of G1's 4 files had a second, distinct gap sitting directly behind it
that recovery's sync-point-skip had been masking — those are now G16-G18, same 3 files. G2/G3's
fix closed 2 of their 4 target files fully clean (`7a-...General Concept.sysml`,
`10b-Trade-off...sysml`); the other 2 (`3c-...structure mod-1.sysml`/`-2.sysml`) each had a
second gap behind them too — now G19/G20, same 2 files. This "closing one gap reveals the next"
pattern is expected in a file with multiple unrelated constructs — not evidence either fix was
scoped wrong — but it does mean file-count progress is slower than the per-group count alone
suggests.

Net effect: **`cargo test --test validation -- --include-ignored` is a genuine 56/56** with #3's
`parse_root` fix rebased onto the G1–G30 work, confirmed 2026-07-30. `test_full_validation_suite`
needed no exception list or expectation change — every one of the original 25 spec-Annex failures
is closed.

### Recommended order

1. ~~**G1 (`perform` semicolon body + `:>>` redefine)**~~ — **Done.** See CHANGELOG.md.
2. ~~**G2/G3 (`connection`/`assert constraint` usage wiring for `PartUsageBodyElement`)**~~ —
   **Done.** See CHANGELOG.md.
3. ~~**G4–G20**~~ — **Done** (2026-07-30). See CHANGELOG.md Unreleased. Surfaced G21–G30 as
   narrower follow-ups discovered while closing them — same "closing one gap reveals the next"
   pattern as G1/G2/G3; each is its own small item, not evidence the G4–G20 pass was scoped wrong.
4. ~~**G21–G30**~~ — **Done.** See CHANGELOG.md Unreleased.
5. ~~**Last remaining gap: `#`-prefix / `connect` support in package & item-def bodies**~~ —
   **Done** (found and closed while rebasing #3 onto the G1–G30 work). `metadata_keyword_prefix`
   (a new function, kept separate from `metadata_keyword_usage` so its stricter guard still
   protects `hash_annotation`'s opaque-capture fallback elsewhere) plus
   `PackageBodyElement`/`AttributeBodyElement::{Connect, MetadataKeywordUsage}`, wired after every
   more-specific dispatcher so `connection_def`'s own hash-annotation prefix still wins where it
   applies. See CHANGELOG.md.

§6 is now fully closed. [PR #3](https://github.com/elan8/sysml-v2-parser/pull/3) is ready to
merge.

---

## Completed — Spec42 parser wave (June 2026)

Parser-side delivery for [SPEC42-DIAGNOSTICS-PARSER-IMPROVEMENTS.md](./SPEC42-DIAGNOSTICS-PARSER-IMPROVEMENTS.md).

| # | Item | Key types / files | Test |
| - | ---- | ----------------- | ---- |
| P0 §1 | Transition `accept` | `TransitionAccept`, `PayloadClause`, [`payload.rs`](../src/parser/payload.rs) | `transition_accept_retained_with_spans` |
| P0 §2 | Final state | `FinalState`, `StateDefBodyElement::FinalState` | `final_state_members_parsed` |
| P0 §3 | Send payload | `ActionUsage.send`, `control_node_action_usage` | `send_payload_on_control_node_action` |
| P0 §4 | `#keyword` (bodies) | `MetadataKeywordUsage` | `metadata_keyword_usage_in_part_body` |
| P0 §5 | Stakeholder / purpose | `StakeholderMember`, `PurposeMember` | `viewpoint_stakeholder_and_purpose_members` |
| P1 §6 | Expression operators | `BinaryOperator`, `UnaryOperator` | `expression_parses_implies_lower_than_or` |
| P1 §7 | Case attribute spans | `AttributeDef.name_span` | `verification_local_attribute_has_name_span` |
| P1 §8 | `rep` in requirement body | `TextualRep`, `language_span` | `requirement_body_rep_language_parsed` |
| P1 §9 | Annotations | `head_span` on `Annotation` / `MetadataAnnotation`; use-case bodies | apollo_regressions (rationale) |
| P2 §10 | Brace skipping (increment) | Transition uses `connect_body` | — |
| P2 §11 | Diagnostic catalog | [`diagnostic_catalog.rs`](../src/parser/diagnostic_catalog.rs) | `diagnostic_catalog_documents_stable_codes` |
| P2 §12 | `first` = initial | `Transition.is_initial` | `transition_first_sets_is_initial_flag` |
| S42-LIM-005 | Generic `FlowUsage` | `FlowUsageKind`, `flow_usage_member`, body wiring | [tests/parser/flow_usage.rs](../tests/parser/flow_usage.rs) |

**Fixtures:** [tests/fixtures/](../tests/fixtures/) (`transition-accept-typed.sysml`, `final-state.sysml`, `send-payload.sysml`, `metadata-keyword-usage.sysml`, `viewpoint-stakeholder-purpose.sysml`, `verification-local-attribute.sysml`, `requirement-rep-language.sysml`)

**Integration test file:** [tests/spec42_diagnostics_ast.rs](../tests/spec42_diagnostics_ast.rs)

---

## Completed — technical debt tranches (reference)

| Plan | Status | Doc |
| ---- | ------ | --- |
| P1 definition prefix + opaque bodies | Complete | [PARSER_DEBT_P1_PLAN.md](./PARSER_DEBT_P1_PLAN.md) |
| P2 structured body loops | Complete | [PARSER_DEBT_P2_PLAN.md](./PARSER_DEBT_P2_PLAN.md) |
| P3 AST split, action/requirement bodies | Complete | [PARSER_DEBT_P3_PLAN.md](./PARSER_DEBT_P3_PLAN.md) |
| P4 view/part bodies, implies, part split | Complete | [PARSER_DEBT_P4_PLAN.md](./PARSER_DEBT_P4_PLAN.md) |

---

## Suggested execution order

1. **Spec42 graph builders** for completed P0 AST (§ 1) — unlocks user-visible diagnostics.
2. **Parser partials** that block Spec42 (§ 2.1 declaration keywords, § 2.2 rep diagnostics, § 2.3 action/requirement bodies).
3. **LSP P0** (§ 3) in parallel with body fidelity.
4. **Expression depth** (§ 2.4) and **P5 grammar layer** (§ 4) as longer horizons.

---

## Document map

| Document | Role |
| -------- | ---- |
| **This file** | Open backlog & roadmap (maintain here) |
| [SPEC42-DIAGNOSTICS-PARSER-IMPROVEMENTS.md](./SPEC42-DIAGNOSTICS-PARSER-IMPROVEMENTS.md) | Spec42-facing parser requirements & fixture index |
| [LANGUAGE_SERVER_BACKLOG.md](./LANGUAGE_SERVER_BACKLOG.md) | LSP/recovery detail |
| [SYSML_V2_COMPLIANCE_GAP.md](./SYSML_V2_COMPLIANCE_GAP.md) | Grammar fidelity narrative |
| [PARSER_TECHNICAL_DEBT.md](./PARSER_TECHNICAL_DEBT.md) | Duplication & architecture notes |
| [BNF_COMPLIANCE_MATRIX.md](./BNF_COMPLIANCE_MATRIX.md) | Compact grammar-family snapshot |
| [ERROR_RECOVERY.md](./ERROR_RECOVERY.md) | Recovery behavior reference |
| PARSER_DEBT_P1–P4_PLAN.md | Completed implementation checklists |
