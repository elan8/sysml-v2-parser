# Inventory: silent parse loss in the SysML v2 release corpus

> ## Provenance and staleness — read first
>
> **Measured against `b6291cc` (`main`), not against the branch that carries this file** — the
> report states this itself in section 1. Entries fixed by commits `ec47463`..`3941261` still
> appear below as live findings.
>
> Verified stale on `3941261` — **fixed**, ignore here:
>
> | Entry | Status on the branch |
> |---|---|
> | Root cause 1, `/* … */` Comment element discarded (40 occurrences / 22 files) | **Mostly fixed.** `ec47463` covered the calc-shaped, action and constraint bodies; `3941261` covered `import`/`dependency`/`alias` relationship bodies. Re-measured on the branch: **15 comments across 12 files** still lost, in scopes neither commit reached. The finding is real but its size is now less than half what is stated below. |
> | Root cause 2, `attribute`/`redefines` shredded in constraint and calc bodies | Fixed by `e7355c6` and `6d54b85` |
> | The excluded `flow` case | Fixed by `6d54b85` |
>
> The agent was mid-way through re-measuring against the branch when it terminated on an API
> error, having reported that a new root cause had surfaced there. **That finding was never
> recovered** and this file does not contain it.
>
> Its methodological conclusion is commit-independent and is the most valuable thing here: a
> round-trip equivalence check **alone is not sufficient** to detect silent loss. Of 16 files where
> an authored keyword was proven to have become an ordinary name, 8 round-trip perfectly.


Read-only investigation. No repository source file was modified. Two throwaway harnesses were
created under `examples/` and deleted afterwards.

Parser under test: `sysml-v2-parser` @ `b6291cc` (worktree `agent-a080e649528396b87`),
`PARSE_AST_VERSION = 184`.

---

## 1. Method, and what it can and cannot see

For each of the 403 `.sysml`/`.kerml` files under `sysml-v2-release/kerml/`,
`sysml-v2-release/sysml/` and `sysml-v2-release/sysml.library/`:

1. `parse_for_editor(src)` → record diagnostic count `d1` and `opacity_report` hits.
2. `emit_sysml(&document)` → record `EmitError` if any.
3. `parse_for_editor(emitted)` → record `d2`.
4. Compare `ParsedDocument::write_semantic_ast_for_comparison` of both documents.

**Comparison projection chosen: `write_semantic_ast_for_comparison`** (`src/ast/semantic_format.rs:51`),
not `RootNamespace::normalize_for_test_comparison`. Reasons:

* It is span-insensitive by construction (`include_source_spans = false`) and resolves qualified
  references *through each document's own arena*, which is exactly the cross-document boundary
  `normalize_for_test_comparison` cannot give you — `normalize_for_test_comparison` operates on a
  detached `RootNamespace`, and `tests/roundtrip_validation.rs` itself documents that "qualified
  reference IDs are document-local, so comparing detached roots from separate parses is invalid".
* It is the same projection the snapshot driver's canonical-output gate uses
  (`tools/snapshot_tool/support.rs:378` `comparison_projection`, used at `:445-451`), so my notion
  of equivalence matches the repo's rather than a weaker invented one.
* It surfaces the *authored token spelling* of every reference (`(token "flow")` vs `(token "'flow'")`),
  which is what makes keyword-shredding visible at all.

### 1a. The round trip alone is NOT sufficient — this is the most important methodological result

I validated the harness against the known-good example first:

```kerml
package P { classifier C { flow a.y to b.x1; } }
```

It *is* detected as UNSTABLE — but only incidentally, because the emitter must quote `flow` and `to`
to make its own output reparseable, so the reference `token` changes from `flow` to `'flow'`.

Two independent blind spots make emit→reparse equivalence systematically under-report:

* **The comparison projection has coarse leaves.** `ActionDefBodyElement` is projected as opaque
  atoms for most variants — `(bind)`, `(flow-usage)`, `(state-usage)`, `(assert-constraint)`,
  `(attribute-usage)`, `(assign)`, `(if)`, `(merge)`, `(decision)`, `(join)`, `(fork)`,
  `(terminate)`, `(while)`, `(loop)`, `(then-action)`, `(calc-usage)`, `(action-def)`,
  `(metadata-usage)`, `(default-reference-usage)` (`src/ast/semantic_format.rs:2189-2239`), and
  `RelationshipBodyElement::KermlFeature` as `(kerml-feature)` (`:1698`). Anything lost *inside*
  those subtrees is invisible, and so are the references they own.
* **A loss that is stable under re-parse round-trips cleanly.** If the parser drops a clause, the
  emitter cannot print it, and the second parse drops nothing more — the projections match.

Concretely: of the 16 files where I proved a keyword was silently accepted as an ordinary name,
**8 round-trip perfectly stably**. And the whole "`Comment` element discarded" family (22 files) is
invisible to the round trip because comments are not in the comparison projection at all.

So I added two direct detectors that do not depend on round-trip stability:

* **KWNAME** — the emitter must quote any name colliding with a reserved keyword
  (`src/parser/lex.rs:733-865`). A `'kw'` appearing in emitted text that the source never wrote as
  `'kw'` proves the parser accepted an authored keyword as a name or reference.
* **COMMENTLOSS** — count of `/*` in source vs emitted. Both grammars make `/* … */` a *significant*
  token forming a `Comment` element (see §3, RC-1), so a drop is real semantic loss.

Everything below was then reduced to a minimal `package P { … }` snippet and re-verified.

---

## 2. Headline counts

| Measure | Count |
| --- | --- |
| Files scanned | **403** |
| Files parsing with **zero diagnostics** (`d1 = 0`) | **304** |
| Files with diagnostics (`d1 > 0`) — other agent's inventory | 99 |
| `emit_sysml` returned `Err` | **104** (99 of them are the `d1 > 0` files; **5 have a clean parse**) |
| **Clean-parse files whose round trip is UNSTABLE** | **12** |
| **Clean-parse files that silently discard a `Comment` element** | **22** (40 elements) |
| **Clean-parse files where an authored keyword became a name/reference** | **16** (10 with real loss, 6 emitter-quoting only) |
| **Union: clean-parse files with proven silent loss** | **33** |
| Reparse produced diagnostics the original did not (`d1 = 0`, `d2 > 0`) | 3 |

The 12 UNSTABLE clean-parse files:

```
kerml/src/examples/Simple Tests/Behaviors.kerml
kerml/src/examples/Simple Tests/Inheritance.kerml
sysml/src/examples/Analysis Examples/Turbojet Stage Analysis.sysml
sysml/src/examples/Simple Tests/ConstraintTest.sysml
sysml/src/training/31. Constraints/Analytical Constraints.sysml
sysml/src/training/31. Constraints/Constraints Example-2.sysml
sysml/src/validation/13-Model Containment/13a-Model Containment.sysml
sysml.library/Kernel Libraries/Kernel Semantic Library/Objects.kerml            (d2=1)
sysml.library/Kernel Libraries/Kernel Semantic Library/StatePerformances.kerml  (d2=1)
sysml.library/Kernel Libraries/Kernel Semantic Library/Transfers.kerml          (d2=2)
sysml.library/Kernel Libraries/Kernel Semantic Library/TransitionPerformances.kerml
sysml.library/Systems Library/Actions.sysml
```

The 5 clean-parse `emit_sysml` failures (all `EmitError::Unsupported`, i.e. the emitter, not opacity):

| File | Construct |
| --- | --- |
| `sysml/src/examples/Analysis Examples/AnalysisAnnotation.sysml` | `MetadataUsage` |
| `sysml/src/examples/Association Examples/ProductSelection_N_ary.sysml` | `EndDecl nested_usage Item` |
| `sysml/src/training/19. Terminate Actions/Terminate Actions Example-2.sysml` | `TerminateStmt` |
| `sysml.library/Domain Libraries/Cause and Effect/CausationConnections.sysml` | `EndDecl nested_usage Occurrence` |
| `sysml.library/Systems Library/Items.sysml` | `EndDecl nested_usage Item` |

The other 99 emit failures are all `EmitError::Opaque` on files that already reported recovery
diagnostics — expected, and out of scope here.

---

## 3. Root causes, ranked by occurrence count

Every entry records both the `.kebnf` spec citation and the OMG Pilot Implementation (Xtext)
verdict. Pilot paths are relative to `/Users/luke/Documents/GitHub/SysML-v2-Pilot-Implementation`;
`SysML.xtext` = `org.omg.sysml.xtext/src/org/omg/sysml/xtext/SysML.xtext`,
`KerML.xtext` = `org.omg.kerml.xtext/src/org/omg/kerml/xtext/KerML.xtext`,
`KerMLExpressions.xtext` = `org.omg.kerml.expressions.xtext/src/org/omg/kerml/expressions/xtext/KerMLExpressions.xtext`.
`.kebnf` paths are relative to `sysml-v2-release/bnf/`.

---

### RC-1 — A `/* … */` `Comment` element is silently discarded, and in expression-fallback scopes the *following member is shredded too*   — **40 occurrences, 22 files** — PARSER

**Minimal reproductions (all verified, all `d1 = 0`, `opacity = 0`):**

```sysml
// (a) comment dropped only
package P { action def A { /* c */ action a1; } }
//  -> action def A { action a1; }                        Comment element gone

package P { connection def K { /* c */ end e1; } }        // same
package P { constraint def C { /* c */ x >= y } }         // same
package P { package D { part def V; } import D::* { /* c */ } }
//  -> import D::* {}                                     Import relationship body gone entirely

// (b) comment dropped AND next member shredded
package P { struct S { /* c */ feature g; } }
//  -> struct S { feature; g; }                           two bare Expression members
package P { class C  { /* c */ feature f; } }             // same
package P { behavior C { /* c */ feature f; } }           // same
package P { datatype C { /* c */ feature f; } }           // same
package P { feature y  { /* c */ feature f; } }           // same
package P { assoc A  { /* c */ end feature f; } }
//  -> assoc A { 'end'; feature f; }
package P { calc def C { /* c */ in x : Real; } }
//  -> calc def C { 'in'; x : Real; }
package P { calc def C { in x : Real; /* c */ return : Real; } }
//  -> calc def C { in x : Real; 'return' : Real; }       usage *named* `return`
package P { part v { constraint c { /* c1 */ doc /* d1 */ x >= y } } }
//  -> constraint c { 'doc'; x >= y; }                    Comment AND Documentation both gone
```

**Scope matrix** (34 body scopes probed). Comment **KEPT** correctly in: package, `part def`,
`part usage`, `state def`, `requirement def`, `item def`, `port def`, `interface def`,
`use case def`, `case def`, `analysis def`, `verification def`, `attribute def`, `occurrence def`,
`metadata def`, `enum def`, `flow def`, `allocation def`, `concern def`, `viewpoint def`.
Comment **DROPPED** in: KerML `class`/`struct`/`behavior`/`datatype`/`feature`/`assoc` bodies,
SysML `calc def`, `action def`, `constraint def`, `connection def` bodies, and `import`
relationship bodies. Only the KerML type bodies, `assoc` and `calc def` additionally shred.

A `//` single-line note is handled correctly everywhere — the defect is specific to
`REGULAR_COMMENT`.

**What it parses to vs. what it should be.** It should be an `AnnotatingElement`/`Comment`
member of the enclosing body (our parser *does* build `(comment (keyword none) (name none) … (normalized "c"))`
in the scopes that work, so the typed node exists). Instead the element vanishes, and in the
shredding scopes the following declaration's leading keyword becomes an
`Expression::FeatureRef` and its tail becomes a second, unrelated member.

**BNF (`.kebnf`).** `REGULAR_COMMENT = '/*' COMMENT_TEXT '*/'` — `KerML-textual-bnf.kebnf:38`,
distinct from `SINGLE_LINE_NOTE` (`:32`) and `MULTILINE_NOTE` (`:35`), which are the trivia forms.
`Comment` — `SysML-textual-bnf.kebnf:82`, `KerML-textual-bnf.kebnf:199`.
`AnnotatingElement = Comment | Documentation | TextualRepresentation | MetadataFeature` —
`SysML-textual-bnf.kebnf:74`, `KerML-textual-bnf.kebnf:188`.
Reachability: SysML `DefinitionBodyItem:237` → `DefinitionMember:246` → `DefinitionElement:180` →
`AnnotatingElement:74`; `ActionBodyItem:901` → `NonBehaviorBodyItem:910` → `DefinitionMember:246`;
`CalculationBodyItem:1366` → `ActionBodyItem:901`; `RelationshipBody:46` (`'{' (OwnedAnnotation)* '}'`)
reached from `Import:149`. KerML `TypeBodyElement:434` → `NonFeatureMember:270` →
`MemberElement:329` → `AnnotatingElement:188`; `Import:297` → `RelationshipBody`.

**Pilot verdict: agrees, and settles the key question.**
`KerMLExpressions.xtext:29` declares `grammar … hidden(WS, ML_NOTE, SL_NOTE)`, and
`KerMLExpressions.xtext:568` defines `terminal REGULAR_COMMENT: '/*' ->'*/';`. `REGULAR_COMMENT` is
**not** in the hidden set — `/* … */` is a *significant* token, and
`Comment returns SysML::Comment : (…)? (…)? body = REGULAR_COMMENT` (`SysML.xtext:84`,
`KerML.xtext:93`) makes it an owned `Comment` element. `SysML.xtext:29` inherits the same hidden
set. `AnnotatingElement` `SysML.xtext:75`; `RelationshipBody : ';' | '{' (ownedRelationship +=
OwnedAnnotation)* '}'` `SysML.xtext:48`, `KerML.xtext:51`; KerML `TypeBody` `KerML.xtext:363-371`,
`NonFeatureMember` `KerML.xtext:153`, `Import … RelationshipBody` `KerML.xtext:173`.
So dropping `/* */` is **not** legitimate trivia elision: it destroys a model element.

**Owning parser function / body-element enum.** The `Comment` is dropped by the annotating-member
dispatch of the KerML type-body and `CalculationBody`-family member parsers; the shredding is the
expression fallback of the same dispatcher (the reparse diagnostic it later emits reads
`unrecognized declaration \`feature\` in calc body`, confirming KerML type bodies route through the
shared `CalcDefBody` member parser). Enums: `RelationshipBodyElement` (import bodies, no
`Annotating`-reaching path taken), `ActionDefBodyElement`, and the KerML type-body element enum.

**Occurrences: 40 dropped `Comment` elements across 22 clean-parse files.** Examples:

* `sysml/src/validation/01-Parts Tree/1a-Parts Tree.sysml:28` — comment inside
  `private import Definitions::* { … }`; emitted as `private import Definitions::* {}`.
  (This file is in `ROUNDTRIP_PASS`.)
* `sysml.library/Kernel Libraries/Kernel Semantic Library/Objects.kerml:133` — comment inside a
  `struct` body; shreds `feature genus : Natural[0..1] default 0;` at `:134` into `feature;` +
  `genus : Natural[0..1] default 0;`.
* `sysml.library/Kernel Libraries/Kernel Semantic Library/StatePerformances.kerml:32` — comment in a
  `behavior` body; shreds `step entry[1];` at `:37` into `step;` + `entry['1']` (the multiplicity
  `[1]` is re-read as a *unit* on a quantity expression).
* `sysml.library/Domain Libraries/Quantities and Units/Time.sysml:211, 257, 270` — comment in a
  `calc` body; each shreds the following `return : Real;` into a usage *named* `return`.
* `sysml/src/validation/08-Requirements/8-Requirements.sysml:112` — comment in an
  `assume constraint` body; shreds `doc /* full fuel tank */` at `:116` into a bare `doc`
  expression, losing both the `Comment` and the `Documentation` element with its body text.

Full file list (with per-file dropped counts) — `Turbojet Stage Analysis.sysml` 1,
`01. Packages/Comment Example.sysml` 1, `33. Analysis/Analysis Case Usage Example.sysml` 2,
`1a-Parts Tree.sysml` 1, `2a-Parts Interconnection.sysml` 2,
`3a-Function-based Behavior-1.sysml` 3, `3a-Function-based Behavior-2.sysml` 1,
`4a-Functional Allocation.sysml` 2, `5-State-based Behavior-1.sysml` 3,
`5-State-based Behavior-1a.sysml` 2, `6-Individual and Snapshots.sysml` 2,
`8-Requirements.sysml` 4, `13a-Model Containment.sysml` 1, `TradeStudies.sysml` 2,
`Time.sysml` 3, `DerivationConnections.sysml` 1, `Collections.kerml` 4, `Metaobjects.kerml` 1,
`Objects.kerml` 1, `StatePerformances.kerml` 1, `Transfers.kerml` 1, `Cases.sysml` 1.

**Parser or emitter:** parser. The element never reaches the AST.

---

### RC-2 — `attribute` / `redefines` members in `CalculationBody` (constraint & calc) bodies shredded into bare `Expression` members   — **9 occurrences, 4 files** — PARSER

**Minimal reproductions:**

```sysml
package P { constraint def MC { attribute m : Real; } }
//  -> constraint def MC { 'attribute'; m : Real; }        two Expression members

package P { part v { constraint c { attribute a : Real; } } }
//  -> constraint c { 'attribute'; a : Real; }

package P { part v { constraint mc : MC { redefines partMasses = 1; } } }
//  -> constraint mc : MC { 'redefines'; partMasses = 1; }

package P { part v { assert constraint ma : MA { attribute redefines totalMass; } } }
//  -> assert constraint ma : MA { 'attribute' :>> totalMass; }   usage *named* `attribute`
```

No leading comment is needed — this is independent of RC-1.

**What it should be.** One `AttributeUsage` member (or, for the third case, one
`DefaultReferenceUsage` with an owned redefinition). It becomes either two unrelated
`Expression::FeatureRef` members, or one usage whose *declaration name* is the keyword.

**BNF (`.kebnf`).** `ConstraintDefinition:1378` and `ConstraintUsage:1382` and
`AssertConstraintUsage:1386` all use `CalculationBody:1359` → `CalculationBodyPart:1362` →
`CalculationBodyItem:1366` → `ActionBodyItem:901` → `NonBehaviorBodyItem:910` →
`NonOccurrenceUsageMember:254` → `NonOccurrenceUsageElement:345` →
`AttributeUsage:513` / `DefaultReferenceUsage:332`.

**Pilot verdict: agrees.** `ConstraintDefinition` `SysML.xtext:1993`, `ConstraintUsage` `:2003`,
`AssertConstraintUsage` `:2007`, all → `CalculationBody` `:1947` → `CalculationBodyPart` `:1951` →
`CalculationBodyItem` `:1956` → `ActionBodyItem` `:1368` → `NonOccurrenceUsageMember` →
`NonOccurrenceUsageElement` `:647` → `AttributeUsage` `:750` / `DefaultReferenceUsage` `:630`.
Both sources admit the syntax; this is a parser gap.

**Owning parser function / enum.** `CalculationBody` member dispatch (shared `CalcDefBody`
member parser); `ActionDefBodyElement`, falling through to `ActionDefBodyElement`'s expression arm.

**Occurrences (9) / examples:**

* `sysml/src/examples/Simple Tests/ConstraintTest.sysml:7, 8` (constraint def body),
  `:25, 26` and `:37, 38` (assert-constraint bodies → `'attribute' :>> totalMass`)
* `sysml/src/training/31. Constraints/Constraints Example-2.sysml:10, 11` (attribute),
  `:18, 19` (anonymous `redefines`)
* `sysml/src/training/31. Constraints/Analytical Constraints.sysml:15`
* `sysml/src/examples/Analysis Examples/Dynamics.sysml:41`

**Parser or emitter:** parser.

---

### RC-3 — A visibility modifier on a KerML `inv` member is shredded   — **4 occurrences, 3 files** — PARSER

**Minimal reproduction:**

```kerml
package P { behavior B { private inv { 1 == 1 } } }
//  -> behavior B { 'private'; inv { 1 == 1; } }
package P { class C { private inv { 1 == 1 } } }   // same
```

`protected feature f;` and `public feature f;` in the same scope are handled correctly, so the
defect is specific to `inv` (and by inspection the other `FeatureElement` alternatives that take a
`FeaturePrefix`).

**What it should be.** One `OwnedFeatureMember` whose `MemberPrefix` carries
`visibility = private` and whose element is an `Invariant`. Instead the visibility keyword becomes a
bare `Expression::FeatureRef` sibling and the invariant loses its visibility.

**BNF (`.kebnf`).** `TypeBodyElement:434` → `FeatureMember:519` → `OwnedFeatureMember:527`
(`MemberPrefix ownedRelatedElement += FeatureElement`), `MemberPrefix:260`
(`( visibility = VisibilityIndicator )?`), `VisibilityIndicator:263`, `Invariant:913`.

**Pilot verdict: agrees.** `TypeBody` `KerML.xtext:363`, `FeatureMember` `:375`,
`OwnedFeatureMember` `:383` (`MemberPrefix ownedRelatedElement += FeatureElement`),
`MemberPrefix` `:145`, `FeatureElement` includes `Invariant` `:268`, `Invariant` `:980`.

**Owning parser function / enum.** KerML type-body member parser (`CalcDefBody`-shared);
KerML type-body element enum.

**Occurrences (4) / examples:**

* `sysml.library/Kernel Libraries/Kernel Semantic Library/Transfers.kerml:135, 144`
* `sysml.library/Kernel Libraries/Kernel Semantic Library/StatePerformances.kerml:55`
* `sysml.library/Kernel Libraries/Kernel Semantic Library/TransitionPerformances.kerml:64`

**Parser or emitter:** parser.

---

### RC-4 — Emitter drops authored quoting on reference segments   — **6 occurrences, 2 files** — EMITTER

**Minimal reproduction:**

```sysml
package P { package Q { part x; } public import 'Q'; }
//  -> public import Q;
//  projection: (token "'Q'") becomes (token "Q")
```

**What it should be.** The reference's authored spelling is `'Q'` (an `UNRESTRICTED_NAME`); the
decoded name is `Q`. AGENTS.md: "The original document is authoritative for authored spelling and
provenance" and "Keep declaration names, references, literals, keywords, and opaque source distinct
even when their spelling happens to be identical." The emitter re-derives the segment text from the
decoded name and loses the authored form.

**BNF (`.kebnf`).** `NAME = BASIC_NAME | UNRESTRICTED_NAME` (`KerML-textual-bnf.kebnf:50`, with
`BASIC_NAME:53` and `UNRESTRICTED_NAME:59`); `QualifiedName:285`. The two spellings are
distinct lexemes for the same name.

**Pilot verdict: agrees.** `Name : ID | UNRESTRICTED_NAME` (`KerMLExpressions.xtext:535`), with
`terminal UNRESTRICTED_NAME` at `:562` — two distinct terminals, so the authored form is a real
lexical fact, not a rendering choice.

Note this is *not* a semantic-model change (the resolved name is identical), which is why I rank it
below the parser causes. It is nevertheless the thing that made two files round-trip-unstable.

**Owning layer.** Reference emission in `src/emit/`.

**Occurrences (6) / examples:**

* `sysml/src/examples/Analysis Examples/Turbojet Stage Analysis.sysml:62` (`'Ideal Gas Parcel'::'Pressure'`
  → `'Ideal Gas Parcel'::Pressure`), `:89`, `:96` (`'Density'` → `Density`) — 5 references
* `sysml/src/validation/13-Model Containment/13a-Model Containment.sysml:48` (`public import 'PowerTrain';`
  → `public import PowerTrain;`) — 1 reference

**Parser or emitter:** emitter.

---

### RC-5 — `ref NAME : TYPE :>> TARGET;` loses its redefinition in action-family bodies   — **2 occurrences, 1 file** — PARSER

**Minimal reproduction:**

```sysml
package P { action def A { ref m : T1 :>> y; } }
//  -> action def A { ref m : T1; }         `:>> y` gone, zero diagnostics
package P { state def A { ref m : T1 :>> y; } }   // same
package P { calc def A { ref m : T1 :>> y; } }
//  -> calc def A { 'ref'; m : T1 :>> y; }  (RC-2-shaped shredding instead)
```

Discriminators verified: `part m : T1 :>> y;` and `attribute m : T1 :>> y;` in the same scope are
fine; `ref m :>> y;` (untyped) is fine; `ref m : T1 :> y;` (subsets) is fine; the same declaration
in a `part def` body is fine. The loss needs `ref` + a typing part + `:>>` in an action/state body.

**What it should be.** One `ReferenceUsage` whose `FeatureSpecializationPart` carries both the
typing and an `OwnedRedefinition`.

**BNF (`.kebnf`).** `ReferenceUsage:335` (`( EndUsagePrefix | RefPrefix ) 'ref' Usage`),
`Usage` → `UsageDeclaration` → `FeatureSpecializationPart:424` → `Redefinitions:472` /
`OwnedRedefinition:478`. `FeatureSpecializationPart` explicitly allows a typing part followed by
further specializations.

**Pilot verdict: agrees.** `ReferenceUsage` `SysML.xtext:635`, `FeatureSpecialization` /
`FeatureSpecializationPart` in the same file; `ActionBodyItem` `:1368` reaches it through
`NonOccurrenceUsageMember` → `NonOccurrenceUsageElement:647` → `ReferenceUsage`.

**Aggravating emitter behaviour.** In the *source* clause order `ref m :>> y : T1, T2 { … }` the
parser reads the redefinition correctly; the emitter then re-orders it to
`ref m : T1, T2 :>> y { … }`, at which point the *reparse* hits this parser bug and loses it. So
the round trip for `Actions.sysml` is broken by an emitter re-ordering feeding a parser gap.

**Owning parser function / enum.** `RefDecl` declaration-tail parsing in the action/state body
member parser; `ActionDefBodyElement::RefDecl` / `StateDefBodyElement`.

**Occurrences (2) / examples:**

* `sysml.library/Systems Library/Actions.sysml:195` (`ref sentMessage :>> sentTransfer: MessageTransfer, MessageAction { … }`)
* `sysml.library/Systems Library/Actions.sysml:214` (`ref acceptedMessage :>> acceptedTransfer: …`)

**Parser or emitter:** parser (exposed by an emitter clause re-ordering).

---

### RC-6 — Anonymous control node after `then` becomes a reference to a feature named after the keyword   — **2 occurrences, 2 files** — PARSER

**Minimal reproduction:**

```sysml
package P { action def A { action a1; then decide; } }
//  -> action def A { action a1; then 'decide'; }
package P { action def A { action a1; then fork;  } }   // then 'fork';
package P { action def A { action a1; then merge; } }   // then 'merge';
package P { action def A { action a1; then join;  } }   // then 'join';
```

`then decide d1;` (named control node) parses correctly, so the defect is specific to the
declaration-less form.

**What it should be.** An `EmptySuccessionMember` (`'then' …`) followed by an `ActionNodeMember`
holding a `DecisionNode` / `ForkNode` / `MergeNode` / `JoinNode` with an empty `UsageDeclaration`.
Instead one `ThenTarget` is produced whose target is a `QualifiedReference` to a feature named
`decide`/`fork`/… — and because those are reserved keywords, the reference could not have been
authored that way at all.

**BNF (`.kebnf`).** `ActionBodyItem:901` alternative
`( ownedRelationship += SourceSuccessionMember )? ownedRelationship += ActionBehaviorMember …`;
`SourceSuccessionMember:597` (`'then' ownedRelatedElement += SourceSuccession`);
`ActionBehaviorMember:919` → `ActionNodeMember:926` → `ActionNode:954` → `ControlNode` →
`DecisionNode:985`, `ForkNode:995`, `MergeNode:980`, `JoinNode:990`.

**Pilot verdict: agrees, with one benign wording difference.**
`ActionBodyItem` `SysML.xtext:1368-1382` has
`( ownedRelationship += EmptySuccessionMember )? ownedRelationship += ( BehaviorUsageMember | ActionNodeMember )`;
`EmptySuccessionMember` `:874`, `EmptySuccession` `:878` (`'then' …`); `DecisionNode` `:1670`,
`ForkNode` `:1682`, `MergeNode` `:1664`, `JoinNode` `:1676`. The Pilot writes
`UsageDeclaration?` (explicitly optional) where the `.kebnf` writes `UsageDeclaration`; since
`UsageDeclaration = Identification? FeatureSpecializationPart?` can derive empty, the two agree that
`then decide;` is legal. Recorded here only for completeness — see §5.

**Owning parser function / enum.** `then`-target / succession parsing in the action-body member
parser; `ActionDefBodyElement::ThenAction` / `ThenTarget`.

**Occurrences (2) / examples:**

* `sysml/src/training/17. Control/Decision Example.sysml:22` (`then decide;`)
* `sysml/src/training/17. Control/Fork Join Example.sysml:14` (`then fork;`)

**Parser or emitter:** parser.

---

### RC-7 — `alias N for Q;` inside a KerML type or feature body shredded into four members   — **1 occurrence, 1 file** — PARSER

**Minimal reproduction:**

```kerml
package P { class A { feature f; } feature y : A { alias x for A::f; } }
//  -> feature y : A { 'alias'; x; 'for'; A::f; }     four unrelated Expression members
package P { class C { feature f; alias a for f; } }   // same
```

`alias a for C::f;` at *package* level parses correctly, so this is a type-body dispatch gap.

**What it should be.** One `AliasMember`.

**BNF (`.kebnf`).** `TypeBodyElement:434` includes `ownedRelationship += AliasMember`;
`AliasMember:278` (`MemberPrefix 'alias' ( '<' … '>' )? ( memberName = NAME )? 'for'
memberElement = [QualifiedName] RelationshipBody`).

**Pilot verdict: agrees.** `TypeBody` `KerML.xtext:363-371` lists
`ownedRelationship += AliasMember`; `AliasMember` `KerML.xtext:161-166`.

**Owning parser function / enum.** KerML type-body member dispatch (shared `CalcDefBody`);
KerML type-body element enum.

**Occurrence:** `kerml/src/examples/Simple Tests/Inheritance.kerml:11`.

**Parser or emitter:** parser.

---

### RC-8 — `ref` usage prefix on an `analysis` usage shredded   — **1 occurrence, 1 file** — PARSER

**Minimal reproduction:**

```sysml
package P { analysis def A { ref analysis self : A; } }
//  -> analysis def A { 'ref'; analysis self : A; }
```

`ref part`, `ref action`, `ref calc`, `ref case`, `ref state`, `ref occurrence`, `ref item`,
`ref port` are all handled correctly — only the analysis-case family rejects the prefix.

**What it should be.** One `AnalysisCaseUsage` with `isReference = true`.

**BNF (`.kebnf`).** `AnalysisCaseUsage:1533` (`OccurrenceUsagePrefix 'analysis' …`),
`OccurrenceUsagePrefix:564` → `BasicUsagePrefix:281` (`RefPrefix ( isReference ?= 'ref' )?`),
`RefPrefix:275`.

**Pilot verdict: agrees.** `AnalysisCaseUsage` `SysML.xtext:2236`
(`OccurrenceUsagePrefix AnalysisCaseUsageKeyword ActionUsageDeclaration CaseBody`),
`OccurrenceUsagePrefix` `:836`, `BasicUsagePrefix` `:563` (`RefPrefix ( isReference ?= 'ref' )?`),
`RefPrefix` `:556`.

**Owning parser function / enum.** Analysis-case usage prefix parsing; `PartDefBodyElement` /
`ActionDefBodyElement` analysis arm.

**Occurrence:** `sysml.library/Systems Library/AnalysisCases.sysml:21`
(`ref analysis self : AnalysisCase :>> Case::self;`).

**Parser or emitter:** parser.

---

### RC-9 — Anonymous `ref :>> TARGET;` mis-parsed, differently in four scopes   — **1 occurrence, 1 file** — PARSER

**Minimal reproduction:**

```sysml
package P { part def V { ref :>> y; } }      // -> ref y;         `:>>` gone, `y` became the NAME
package P { part v     { ref :>> y; } }      // -> ref y;         same
package P {              ref :>> y; }        // -> ref y;         same
package P { state def V { ref :>> y; } }     // -> ref;           TARGET LOST ENTIRELY
package P { calc def V  { ref :>> y; } }     // -> 'ref' :>> y;   `ref` became the NAME
```

The same declaration is handled correctly in `item def`, `port def`, `interface def`,
`connection def`, `view def`, `occurrence def`, `attribute def` and `requirement def` bodies —
eight scopes right, four scopes wrong, each wrong in a different way.

**What it should be.** One `ReferenceUsage` with no `Identification` and an `OwnedRedefinition`
targeting `y`. The `part def`/`part usage`/package variant is the most dangerous: it produces a
*named* reference usage `ref y`, which is a legal but completely different model element, and it
round-trips stably.

**BNF (`.kebnf`).** `ReferenceUsage:335` → `Usage` → `UsageDeclaration` (`Identification?
FeatureSpecializationPart?`) → `Redefinitions:472` / `OwnedRedefinition:478`.

**Pilot verdict: agrees.** `ReferenceUsage` `SysML.xtext:635` → `Usage` → `UsageDeclaration`;
`declaredName = Name` appears inside an optional `Identification` fragment (`KerMLExpressions.xtext:384`).

**Owning parser function / enum.** `RefDecl` parsing in the part-def / part-usage / package /
state-def member parsers; `PartDefBodyElement::RefDecl`, `PackageBodyElement::RefDecl`.

**Occurrence:** `sysml.library/Systems Library/Views.sysml:48`
(`ref :>> ownedPerformances::this, subperformances::this default that.that;`).

**Parser or emitter:** parser.

---

### RC-10 — `flow SRC to TGT;` in a KerML type body shredded into four members   — **1 occurrence, 1 file** — PARSER — **EXCLUDED, being fixed separately**

Listed only so the inventory is complete and the count reconciles.
Occurrence: `kerml/src/examples/Simple Tests/Behaviors.kerml:18` (`flow a.y to b.x1;`).
`.kebnf` `FlowUsage` / KerML `Flow`; Pilot `KerML.xtext` `Flow`. Not investigated per instruction.

---

## 4. Already-pinned debt vs. genuinely new

`tools/snapshot_tool/support.rs:397-412` `CANONICAL_OUTPUT_DEBT` has exactly two live entries:

| Pinned entry | Relation to this inventory |
| --- | --- |
| `tests/snapshots/spec42/sysml/validation/13a_model_containment.md` ("Containment references change semantic shape after emission") | **Covers RC-4 for that one file.** The 13a occurrence is already-known debt. The identical defect in `Turbojet Stage Analysis.sysml` (5 references) is **not** pinned and is new. |
| `tests/snapshots/kerml/type_body_relationship_members.md` ("The emitted KerML type body does not strictly reparse") | **Unrelated.** Its `SOURCE` is a KerML `assoc`/`function` body exercising connector, binding, succession and end-cross-feature members; none of RC-1…RC-9 appears in it. |

**Everything else in §3 is new.** In particular:

* RC-1 (40 dropped `Comment` elements, 22 files) is unpinned and unmeasured. Comments are absent
  from the semantic comparison projection, and `tests/roundtrip_validation.rs::try_roundtrip`
  compares *emitted vs re-emitted bytes*, which is stable once the comment is gone — so the loss is
  invisible to both existing gates.
* RC-2 occurs in `sysml/src/examples/Simple Tests/ConstraintTest.sysml` and
  `sysml/src/examples/Analysis Examples/Dynamics.sysml`, **both of which are on the
  `EXAMPLES_ROUNDTRIP_PASS` allow-list** in `tests/roundtrip_validation.rs`. They pass that test
  today while silently turning `attribute` into a feature reference.
* RC-1 also affects `sysml/src/validation/01-Parts Tree/1a-Parts Tree.sysml`,
  `02-Parts Interconnection/2a-…`, `03-Function-based Behavior/3a-…-1`, `-2`,
  `04-Functional Allocation/4a-…`, `05-State-based Behavior/5-…-1`, `-1a`,
  `06-Individual and Snapshots/6-…`, `08-Requirements/8-Requirements.sysml` and
  `13-Model Containment/13a-…` — **ten files on the `ROUNDTRIP_PASS` conformance list**.
* RC-5, RC-6, RC-8, RC-9 are unpinned and untested.

---

## 5. INVALID INPUT and spec-vs-Pilot notes

**No finding in §3 is classified INVALID INPUT.** Every root cause is admitted by *both* the pinned
`.kebnf` and the OMG Pilot Xtext grammar.

### 5a. Detector hits that are *not* loss (recorded so the KWNAME count reconciles)

Six of the 16 KWNAME files are emitter *over-quoting*, not parse loss. In each case the source uses,
as an ordinary name, a word that is reserved in SysML but **not** in KerML, or a SysML enum literal
that happens to spell a keyword. Our parser accepts it as a name (correct) and the emitter quotes it
on output (necessary for the output to reparse), so the model is preserved and the round trip is
stable — only the authored spelling changes.

* `sysml.library/Kernel Libraries/Kernel Semantic Library/SpatialFrames.kerml` — `in frame : SpatialFrame[1]`
* `sysml.library/Kernel Libraries/Kernel Semantic Library/StatePerformances.kerml:37-39` — `step entry[1]; step do[1]; step exit[1];`
* `sysml.library/Kernel Libraries/Kernel Semantic Library/TransitionPerformances.kerml:43` — `step accept: AcceptPerformance[accNum]`
* `sysml.library/Systems Library/SysML.sysml` — `enum def PortionKind { timeslice; snapshot; }`,
  `enum def StateSubactionKind { entry; do; exit; }`, `enum def RequirementConstraintKind { assumption; requirement; }`,
  `enum def TriggerKind { when; at; after; }`, plus metadata feature names `rendering`, `action`

Both grammars confirm the KerML half: `KerML-textual-bnf.kebnf` contains zero occurrences of
`'entry'`, `'exit'`, `'frame'` or `'accept'`, and `KerML.xtext` likewise contains zero. Our
`SYSML_RESERVED_KEYWORDS` (`src/parser/lex.rs:733-865`) is a single merged list applied to both
languages, which is why `.kerml` names get quoted. **This is a real fidelity defect worth its own
ticket** (authored spelling is not preserved, and the emitted `.kerml` is no longer idiomatic), but
it is not information loss and I have deliberately kept it out of the ranked list.

The SysML half is less clear-cut. `enum def PortionKind { timeslice; snapshot; }` uses reserved
words (`SysML.xtext:862` `snapshot = 'snapshot' | timeslice = 'timeslice'`) as unquoted enum
literal names, and `Name : ID | UNRESTRICTED_NAME` (`KerMLExpressions.xtext:535`) would normally
have the keyword token win in an Xtext lexer. The file ships inside the Pilot's own model library,
so the Pilot evidently accepts it; I could not determine from the grammar alone whether that is by
design. **Flagged as uncertain rather than INVALID INPUT** — it does not change any finding above.

### 5b. Spec (`.kebnf`) vs Pilot (Xtext) disagreements encountered

Only one, and it is cosmetic:

* **Control node declarations.** `.kebnf` writes `DecisionNode = ControlNodePrefix isComposite ?=
  'decide' UsageDeclaration ActionBody` (`SysML-textual-bnf.kebnf:985`, and likewise `MergeNode:980`,
  `JoinNode:990`, `ForkNode:995`) — `UsageDeclaration` unqualified. The Pilot writes
  `UsageDeclaration?` (`SysML.xtext:1664, 1670, 1676, 1682`). Since `UsageDeclaration =
  Identification? FeatureSpecializationPart?` derives the empty string, both admit `then decide;`,
  so RC-6 stands under either reading. No other disagreement was found across the ten root causes.

---

## 6. Reproduction notes

The harnesses were `examples/scratch_roundtrip_scan.rs` (corpus sweep: round-trip projection
equality, KWNAME detector, COMMENTLOSS detector, emitted-text dump) and
`examples/scratch_reduce.rs` (single-snippet reducer printing diagnostics, opacity, emitted text,
projection diff). Both have been deleted; no repository source file was modified, and nothing was
committed or pushed.

Raw scan output retained at
`…/scratchpad/scan.tsv`, per-file emitted text at `…/scratchpad/scan/emit/`, and the unstable-file
projection pairs at `…/scratchpad/scan/*.{a,b}.txt`.
