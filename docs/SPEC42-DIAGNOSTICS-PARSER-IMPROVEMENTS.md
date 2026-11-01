# Parser improvements for Spec42 diagnostics

This document lists **sysml-v2-parser** changes that would unlock stronger, more precise semantic diagnostics in [Spec42](https://github.com/spec42/spec42). It is written for parser maintainers and cross-repo planning.

**Related Spec42 docs**

- [Diagnostic checks roadmap](https://github.com/spec42/spec42/blob/main/docs/engineering/DIAGNOSTIC-CHECKS-ROADMAP.md) — which checks exist, which are deferred
- [AST semantic coverage](https://github.com/spec42/spec42/blob/main/docs/engineering/AST-SEMANTIC-COVERAGE.md) — what Spec42 wires from the AST today

**Related parser docs**

- **[PARSER_BACKLOG_ROADMAP.md](./PARSER_BACKLOG_ROADMAP.md)** — **single backlog** for all open work (parser + Spec42 follow-up)
- [SYSML_V2_COMPLIANCE_GAP.md](./SYSML_V2_COMPLIANCE_GAP.md) — general parser fidelity status
- [LANGUAGE_SERVER_BACKLOG.md](./LANGUAGE_SERVER_BACKLOG.md) — recovery, ranges, and editor-grade parsing

## Parser implementation status (June 2026)

The items below were originally **parser gaps**. Most P0–P2 AST work is **done in sysml-v2-parser**; see the [completed table](./PARSER_BACKLOG_ROADMAP.md#completed--spec42-parser-wave-june-2026) and [fixture index](#parser-test-fixtures-this-repo). **Open work** (parser partials, Spec42 graph builders, LSP) lives in [PARSER_BACKLOG_ROADMAP.md](./PARSER_BACKLOG_ROADMAP.md).

| Priority | Theme | Parser | Spec42 |
| -------- | ----- | ------ | ------ |
| P0 | Transition accept, send, final state | Done | Open — graph + collectors |
| P0 | `#keyword` in bodies | Done (simple); declaration headers deferred | Open |
| P0 | Viewpoint stakeholder / purpose | Done | Open |
| P1 | Expression classification | Partial (`BinaryOperator` / `UnaryOperator`) | Open — replace heuristics |
| P1 | Case attribute spans | Done (`name_span`) | Open — wire `AttributeDef` in graph |
| P1 | Annotations in bodies | Partial | Open |
| P1 | `rep` / language | Partial (requirement body) | Open |
| P2 | Brace-body fidelity | Open (incremental) | — |
| P2 | Diagnostic catalog | Partial (file exists; wiring open) | — |
| P2 | `first` / initial state | Done (`is_initial`) | Open |

Sections §P0–§P12 below retain the **original requirements** for traceability; treat status column above as authoritative.

## How Spec42 uses the parser

```mermaid
flowchart LR
  parse[parse_with_diagnostics]
  ast[AST + spans]
  graph[semantic_core graph]
  diag[collect_diagnostics_from_graph]
  parse --> ast --> graph --> diag
```

Spec42 diagnostics are only as strong as the **structured facts** and **source ranges** the parser exposes:

1. **Syntax diagnostics** — parser-owned codes and token ranges (`source: sysml`).
2. **Semantic diagnostics** — graph builders in `semantic_core` project AST nodes into a workspace graph; collectors check relationships, types, and expressions.
3. **Range quality** — semantic checks prefer offending **reference tokens**; when the graph only has declaration-level spans, diagnostics fall back to whole lines.

Parser gaps therefore produce either **silent omissions** (check never runs) or **weaker heuristics** (string-based expression analysis instead of typed evaluation).

## Priority overview

| Priority | Theme | Spec42 impact |
| -------- | ----- | ------------- |
| **P0** | Behavior: transition triggers, final states, send payloads | Unblocks `accept_payload_incompatible`, `send_payload_incompatible`, final-state sanity |
| **P0** | Metadata: `#keyword` and user-defined declaration keywords | Unblocks `metadata_keyword_unresolved` beyond built-in `feature` / `class` |
| **P0** | Viewpoints: `stakeholder`, `purpose`, concern references | Unblocks viewpoint reference diagnostics currently deferred |
| **P1** | Expressions: typed AST for guards, filters, assignments | Strengthens `transition_guard_non_boolean`, `view_filter_non_boolean`, `assignment_value_incompatible`, `non_boolean_expression` |
| **P1** | Case bodies: local attributes and verdict/return forms | Improves verification/analysis shape and assignment checks without subject-qualified paths |
| **P1** | Annotations in structural bodies | Enables metadata and viewpoint diagnostics on `@` / `#` usages inside parts, states, requirements |
| **P2** | Body fidelity: replace opaque brace skipping | More body-level diagnostics; fewer false negatives when inner members are skipped |
| **P2** | Parser diagnostic precision and spans | Better LSP ranges for both syntax and semantic cascades |

---

## P0 — Unblocks deferred or partial P2 diagnostics

### 1. Transition `accept` trigger (structured, retained in AST)

**Spec42 today**

- `transition_guard_non_boolean` works when `guardExpression` is on the graph.
- `accept_payload_incompatible` works for **package/action-usage** accept clauses (`action wait accept evt : Type`).
- Transition-embedded accept (common in state machines) is **not** diagnosed:

  ```sysml
  transition to_running first idle accept StartPressed then running;
  ```

**Parser today**

- [`state.rs`](../src/parser/state.rs) parses `first <expr> accept <expr>` but **drops** the accept expression; only the source expression is stored on `Transition.source` (see lines 279–290).
- [`Transition`](../src/ast/behavior.rs) has no `accept` / `trigger` field.

**Requested change**

1. Extend `Transition` with optional structured accept, e.g.:

   ```rust
   pub accept: Option<TransitionTrigger>,  // name + optional type + spans

   pub struct TransitionTrigger {
       pub name: String,
       pub type_name: Option<String>,
       pub name_span: Span,
       pub type_span: Option<Span>,
   }
   ```

2. Parse `accept` as either:
   - `accept Name : Type` (preferred, mirrors `ActionUsage.accept`), or
   - `accept <expression>` with a separate `Expression` node when shorthand is used.

3. Preserve **spans** for payload name and type (same contract as `ActionUsage.type_ref_span`).

**Unblocks in Spec42**

- `accept_payload_incompatible` on transition triggers (graph builder: `payloadType` on transition or synthetic accept action node).
- Token-precise ranges on accept type references in state diagrams.

**Suggested parser tests**

- Fixture matching `StateMachineDemo.sysml` transition accept forms; assert `Transition.accept` populated and spans non-dummy.

---

### 2. Explicit `final state` syntax

> **Withdrawn.** Neither pinned grammar has a `final` state member, so AST 290 removed it. A state
> machine finishes with a transition to `done` (SysML 7.18). The request below is kept as history.

**Spec42 today**

- `multiple_initial_states` / `missing_initial_state` use `RelationshipKind::InitialState` from standalone `then name;` members.
- **Final-state cardinality** is deferred; no heuristic for sink states without outgoing transitions.

**Parser today**

- [`state.rs`](../src/parser/state.rs) supports `then` as initial state (`ThenStmt`) only.
- No `final state` / `final` member in `StateDefBodyElement`.

**Requested change**

1. Add `FinalState` (or equivalent) to `StateDefBodyElement` per SysML textual grammar.
2. Parse `final` / `final state` forms used in the spec and library fixtures.
3. Emit stable spans on the final state name.

**Unblocks in Spec42**

- `multiple_final_states`, `unreachable_non_final_state`, or similar generic checks.
- Removes need for sink-state heuristics on transition graphs.

---

### 3. Structured `send` payload on control-node actions

**Spec42 today**

- `send_payload_incompatible` is implemented but only when `payloadType` exists on an `action` graph node (name `send`/`accept` heuristic in graph builder).
- No integration test for `send` today because structured send payloads rarely appear on the graph.

**Parser today**

- `send` is listed in `CONTROL_NODE_KEYWORDS` but control-node parsing routes through `action_usage`, which requires the `action` keyword.
- There is no symmetric `send: Option<(String, String)>` on `ActionUsage` (only `accept`).

**Requested change**

1. Model `send` control-node usages with explicit payload name and type (mirror `accept`).
2. Support both:
   - `send payload : PayloadType;` (control-node statement), and
   - `action send accept ...` / library shorthand forms as fixtures require.
3. Add `send` / `payload` spans.

**Unblocks in Spec42**

- `send_payload_incompatible` in action bodies and state/behavior flows.
- Flow and sequence view validation of message types.

---

### 4. User-defined metadata keywords (`#keyword`, modeled declarations)

**Spec42 today**

- `metadata_keyword_collision` — duplicate `metadata def` short names (works).
- `metadata_keyword_unresolved` — only for `feature decl` / `classifier decl` where `keyword` is **not** a built-in starter; **`#Tag` annotations are skipped** in graph builders.
- `metadata_annotation_unresolved` — untyped `@` metadata at package level.

**Parser today**

- [`feature_decl`](../src/parser/package.rs) and [`classifier_decl`](../src/parser/package.rs) use **fixed** keyword starters (`feature`, `class`, …).
- [`Annotation`](../src/parser/metadata_annotation.rs) supports `#head` but bodies often land in `Annotation` nodes that Spec42 does not walk.
- User-defined keywords from `metadata def` extensions are not parsed as first-class declaration headers.

**Requested change**

1. When a metadata definition extends the textual notation, allow the **metadata short name** as a declaration keyword (per spec metaclass extension rules).
2. Represent `#keyword` usages as either:
   - `MetadataAnnotation` with resolved keyword + optional type + body, or
   - a dedicated `MetadataKeywordUsage` node (preferred for graph projection).
3. Distinguish `#keyword` from generic `Annotation` so downstream tools do not drop them in `Annotation(_)` match arms.

**Unblocks in Spec42**

- `metadata_keyword_unresolved` for `#UnknownMeta` on parts, requirements, viewpoints.
- Richer metadata conformance without overloading `metadata_annotation_unresolved`.

**Suggested parser tests**

- Package with `metadata def Tag;` and `#Tag;` on a `part` body member → structured node, not `Other` / silent skip.

---

### 5. Viewpoint `stakeholder` and `purpose` members

**Spec42 today**

- `viewpoint_reference_unresolved` covers **frame**, **import**, and missing **rep language**.
- **Stakeholder** and **purpose** reference checks are **deferred** (AST gap).

**Parser today**

- [`ViewpointDef`](../src/parser/view.rs) body reuses [`RequirementDefBody`](../src/ast/requirement.rs).
- `RequirementDefBodyElement` has `Frame`, `SubjectDecl`, `Import`, … but **no** `Stakeholder` or `Purpose`.

**Requested change**

1. Add AST variants and parsers for viewpoint concern members required by the spec (at minimum `stakeholder`, `purpose`; align with `SysML-textual-bnf.kebnf`).
2. Reuse or share parsers with requirement/view concern bodies where the grammar overlaps.
3. Preserve reference spans on qualified names.

**Unblocks in Spec42**

- Extend `viewpoint_reference_unresolved` (or split into specific codes) for stakeholder/purpose targets.
- Viewpoint conformance matrix completion in [CONFORMANCE-MATRIX](https://github.com/spec42/spec42/blob/main/docs/reference/CONFORMANCE-MATRIX.md).

---

## P1 — Strengthen existing diagnostics

### 6. Typed expression AST (beyond debug strings)

**Spec42 today**

- Boolean-ish heuristics: `is_booleanish_filter_expression` (contains `==`, `and`, literals, …).
- Full evaluation only when `analysisConstraints` / quantity paths succeed in `evaluation` module.
- Weak for: complex guards, view filters with function calls, assignment rhs inference.

**Parser today**

- [`expr.rs`](../src/parser/expr.rs) — precedence-aware but subset of `OwnedExpression`.
- Graph builders often store `expression_to_debug_string` only.

**Requested change**

1. Expand expression coverage for forms used in **guards**, **filters**, **assert** / **require constraint**, and **assign** rhs.
2. Where full typing is out of scope, emit **classified** expression nodes:
   - literal kind (boolean, string, integer, real),
   - operator kind,
   - function call with identifier span,
   - member access chain (for resolve-in-context).
3. Keep **sub-expression spans** for LSP range mapping.

**Unblocks in Spec42**

- Replace heuristics in `transition_guard_non_boolean`, `view_filter_non_boolean`, `invalid_import_filter`, `non_boolean_expression`.
- Enable `assignment_value_incompatible` for non-literal rhs when types are known.

---

### 7. Local attributes in verification / analysis case bodies

**Spec42 today**

- `assignment_value_incompatible` test uses `subject system : System` + `assign system.count := "text"` because **attributes declared inside verification def** are not on the graph (`AttributeDef` skipped in `verification.rs`).

**Parser today**

- [`UseCaseDefBodyElement::AttributeDef`](../src/parser/usecase.rs) is parsed for verification/analysis bodies.

**Requested change**

- No parser change strictly required; Spec42 could wire existing AST.
- Parser improvement: ensure **attribute usages/defs** in case bodies have the same header fidelity as part attributes (`:>`, multiplicity, value spans) so Spec42 kind/type checks are consistent.

**Unblocks in Spec42**

- Simpler fixtures for assignment diagnostics.
- `attribute_value_type_mismatch` on case-local attributes.

---

### 8. `TextualRepresentation` / `rep language` completeness

**Spec42 today**

- `viewpoint_rep_language_unresolved` when `language` attr is empty on `textualRep` graph nodes.

**Parser today**

- [`textual_representation`](../src/parser/requirement.rs) exists; wired for requirement/viewpoint bodies when `rep` members are parsed.

**Requested change**

1. Ensure `rep` members are recognized in **all** body contexts Spec42 walks (viewpoint, frame, concern, requirement).
2. Parse and retain **language string literal span** separately from body comment span.
3. Surface parse errors when `language` is missing or not a string literal (parser diagnostic), not only semantic warning.

---

### 9. Wire annotations in part / state / requirement bodies (AST already partial)

**Spec42 today**

- Graph builders explicitly ignore `Annotation(_)` in [`part_usage.rs`](https://github.com/spec42/spec42/blob/main/crates/semantic_core/src/semantic/graph_builder/part_usage.rs), [`state.rs`](https://github.com/spec42/spec42/blob/main/crates/semantic_core/src/semantic/graph_builder/state.rs), [`requirement_body.rs`](https://github.com/spec42/spec42/blob/main/crates/semantic_core/src/semantic/graph_builder/requirement_body.rs).

**Parser today**

- `annotation` and `metadata_annotation` produce `Annotation` / `MetadataAnnotation` nodes in many body parsers.

**Requested change**

1. Prefer `MetadataAnnotation` over generic `Annotation` when `@Name : Type` is used.
2. For `#keyword`, see P0 §4 — structured metadata keyword node.
3. Stable **head identifier span** on all annotation forms.

**Unblocks in Spec42**

- Metadata diagnostics on inline annotations (not only package-level `metadata usage`).

---

## P2 — Infrastructure for the next diagnostic wave

### 10. Reduce opaque brace-body skipping

**Problem**

Several modules still use `advance_to_closing_brace` or legacy skip helpers for unmodeled inner regions (see [SYSML_V2_COMPLIANCE_GAP.md](./SYSML_V2_COMPLIANCE_GAP.md)). Spec42 cannot diagnose what never becomes AST.

**Requested direction**

- Prefer `ParseErrorNode` + partial member lists over silent skipping (align with [LANGUAGE_SERVER_BACKLOG.md](./LANGUAGE_SERVER_BACKLOG.md) P0).
- For each construct family used in `spec42 check` fixtures, either model members or emit recovery nodes with scope labels.

---

### 11. Parser diagnostic codes and ranges as a stable contract

Spec42 and the LSP kernel suppress or cascade based on parser error presence. Stable contracts help:

| Contract | Used by |
| -------- | ------- |
| Stable `code` strings per malformed construct | Diagnostic catalog, CI fixtures |
| Scope labels (`"state body"`, `"verification body"`, …) | Recovery tests, user messages |
| Reference token spans on usages (type, import target, specializes) | `diagnostic_ranges` tests in Spec42 |
| No panic on incomplete input | CLI, LSP, MCP |

**Requested change**

- Document parser diagnostic codes in a small registry (similar to Spec42 `diagnostic_catalog.rs`).
- Add regression tests that assert **range text** for high-traffic codes (imports, types, transition syntax).

---

### 12. Initial vs `first` transition semantics (clarify AST)

**Spec42 today**

- `multiple_initial_states` counts `RelationshipKind::InitialState` edges from standalone `then off;` members.
- `transition ... first off then on` sets transition **source** but does **not** create an initial-state edge.

**Parser / spec alignment**

- Clarify in AST whether `first` on a transition also denotes initial state, or only `then` members do.
- If `first` implies initial state, expose a dedicated flag on `Transition` so Spec42 does not duplicate state-machine semantics in the graph builder.

**Unblocks**

- Consistent initial-state diagnostics between library style (`then idle;`) and transition-embedded `first` style (timers, state views).

---

## Suggested cross-repo workflow

1. **Parser PR** — AST + spans + fixture + `parse_with_diagnostics` range test.
2. **Spec42 graph builder** — project new fields to semantic node attributes.
3. **Spec42 collector** — add or extend diagnostic code; entry in `diagnostic_catalog.rs`.
4. **Roadmap** — move item from **Deferred** to **Done** in `DIAGNOSTIC-CHECKS-ROADMAP.md`.

## Acceptance checklist (per improvement)

- [ ] AST field or body element added with `Span`(s)
- [ ] At least one `.sysml` fixture in `sysml-v2-parser/tests/` or validation suite
- [ ] Recovery behavior documented if parsing can continue with partial input
- [ ] Spec42 integration test listed in this doc is linked or added in the same release train
- [ ] No regression in `test_systems_library_strict_no_diagnostics` / full-library gates

## Parser test fixtures (this repo)

| Roadmap item | Fixture | Test |
| ------------ | ------- | ---- |
| P0 §1 Transition accept | [tests/fixtures/transition-accept-typed.sysml](../tests/fixtures/transition-accept-typed.sysml) | [tests/spec42_diagnostics_ast.rs](../tests/spec42_diagnostics_ast.rs) |
| P0 §2 Final state | [tests/fixtures/final-state.sysml](../tests/fixtures/final-state.sysml) | `final_state_members_parsed` |
| P0 §3 Send payload | [tests/fixtures/send-payload.sysml](../tests/fixtures/send-payload.sysml) | `send_payload_on_control_node_action` |
| P0 §4 Metadata `#keyword` | [tests/fixtures/metadata-keyword-usage.sysml](../tests/fixtures/metadata-keyword-usage.sysml) | `metadata_keyword_usage_in_part_body` |
| P0 §5 Stakeholder / purpose | [tests/fixtures/viewpoint-stakeholder-purpose.sysml](../tests/fixtures/viewpoint-stakeholder-purpose.sysml) | `viewpoint_stakeholder_and_purpose_members` |
| P1 §7 Case attributes | [tests/fixtures/verification-local-attribute.sysml](../tests/fixtures/verification-local-attribute.sysml) | `verification_local_attribute_has_name_span` |
| P2 §12 Initial / `first` | inline in spec42_diagnostics_ast | `transition_first_sets_is_initial_flag` |
| Library regressions | [KitchenTimer.sysml](../tests/fixtures/KitchenTimer.sysml), [TrafficLightIntersection.sysml](../tests/fixtures/TrafficLightIntersection.sysml) | validation + apollo_regressions |

## Document history

| Date | Change |
| ---- | ------ |
| 2026-06-08 | Initial version from Spec42 P2 diagnostics wave and roadmap deferred items |
| 2026-06-08 | Added parser fixture table; P0–P2 AST items implemented in sysml-v2-parser |
| 2026-06-08 | Added implementation status table; open work consolidated in PARSER_BACKLOG_ROADMAP.md |
