# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Working backlog for the gaps-doc PAR-002..006 items (definitions/usages in every owning context,
typed declaration modifiers, typed relationship AST nodes, complete expression AST, non-semantic
recovery). Entries below land incrementally; the version stays unreleased until the whole backlog
is done.

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
