# KerML corpus gap inventory — `sysml-v2-release/kerml/` (58 files)

> ## Provenance and staleness — read first
>
> **Measured against `b6291cc` (`main`), not against the branch that carries this file.** The
> inventory was produced before its measurement tree was corrected, so entries fixed by commits
> `ec47463`..`3941261` still appear below as live gaps. Re-verify any entry against the current
> tree before acting on it.
>
> Verified stale on `3941261` — these are **fixed** and must be ignored here:
>
> | Entry | Status on the branch |
> |---|---|
> | Bare `/* … */` member shredding the following member in a KerML type body (ranked #1, 14 occurrences) | Fixed by `ec47463`; verified clean across all 11 KerML classifier keywords |
> | `intersects` on a feature silently dropped | Fixed by `ec47463`; `feature f : T intersects g;` parses and round-trips |
> | `flow` / bare `redefines` in a KerML type body (excluded from its ranking as in-flight) | Fixed by `6d54b85` |
>
> Verified **still live** on `3941261`, so the rest of the ranking stands: `featured by`
> (`unsupported_grammar_form`, 72 occurrences — the largest single item in either corpus),
> `succession redefines …` (`unexpected_keyword_in_scope`), and the keyword spelling of an unnamed
> `subsets` (`unrecognized_declaration_in_scope`).


Read-only inventory. Nothing in the repository was modified; the scratch `examples/*.rs` scanners
used to produce this were deleted afterwards.

## Method

* `parse_for_editor` + `opacity_report` over all 58 `.kerml` files under `sysml-v2-release/kerml/`
  (they live under `kerml/src/examples/…`, not directly in `kerml/`).
* Baseline reproduced exactly: **58 files, 29 with diagnostics**, opacity
  `{ClassifierDecl: 1, FeatureDecl: 8, ParseError: 103}`, codes
  `{unrecognized_declaration_in_scope: 62, unexpected_keyword_in_scope: 25,
  unsupported_grammar_form: 9, recovered_calc_body_element: 8, recovery_cascade_suppressed: 3,
  recovered_package_body_element: 1}`. (My `recovered_calc_body_element` count is 8 vs the
  quoted 7 and `ParseError` 103 vs 102 — one extra hit, everything else identical.)
* Every minimal reproduction below was **executed** against this tree. None is quoted untested.
* Because diagnostics and opacity understate the problem, I also ran an emit/round-trip sweep:
  for each file, `emit_sysml` → re-parse → re-emit, plus a scan of emitted text for shredding
  artifacts (a keyword re-emitted as a quoted name). The 29 diagnostic-free files are all
  emit-stable and idempotent; the 29 diagnostic-carrying files all fail `emit_sysml` with an
  opaque node, which is *why* silent damage inside them is invisible to the round-trip gate.
  Silent damage was therefore hunted at the snippet level instead, and several instances found.

### Grammar sources used for every verdict

1. **Spec rendering** — `sysml-v2-release/bnf/KerML-textual-bnf.kebnf` (cited as `kebnf:N`).
2. **Pilot reference implementation** — `/Users/luke/Documents/GitHub/SysML-v2-Pilot-Implementation/org.omg.kerml.xtext/src/org/omg/kerml/xtext/KerML.xtext` (cited as `xtext:N`).

Every entry carries both verdicts. Where they disagree it is called out; the disagreements found
are collected in their own section at the end. **No corpus construct in this inventory is
disagreed on by the two sources, and none is INVALID INPUT** — see "INVALID INPUT" below.

### Owning scopes (used throughout)

| scope | parser function | body-element enum |
|---|---|---|
| KerML type body (`type`/`classifier`/`class`/`struct`/`assoc`/`datatype`/`behavior`/`metaclass`/`function`/`expr` bodies) | `calc_def_body_element` — `src/parser/constraint.rs:1275` | `CalcDefBodyElement` |
| package / namespace body | `package_body_element` — `src/parser/package.rs:2147` | `PackageBodyElement` |
| KerML feature member | `kerml_feature` — `src/parser/constraint.rs:1022` | `ast::KermlFeature` |
| KerML connector member | `kerml_connector_member` — `src/parser/constraint.rs:579` | `ast::KermlConnectorMember` |
| KerML binding member | `kerml_binding_member` — `src/parser/constraint.rs:693` | `ast::KermlBindingMember` |
| KerML succession member | `kerml_succession_member` — `src/parser/constraint.rs:747` | `ast::KermlSuccessionMember` |

Note that every diagnostic inside a KerML *type* body says "calc body" — `CalcDefBody` is the
shared body node for KerML type bodies (consistent with the existing L2 note).

---

## The meta-cause behind half of this list

Both grammars define **one** `FeatureDeclaration` production (`kebnf:601`, `xtext:547`) and reuse
it verbatim in `Feature`, `Step`, `Connector`, `BindingConnector`, `Succession`, `Flow`,
`SuccessionFlow` and `Expression`. This parser instead hand-rolls a *different, weaker* subset of
it in each of `kerml_feature`, `kerml_connector_member`, `kerml_binding_member` and
`kerml_succession_member` — each a fixed pipeline of `name → multiplicity → typing →
specialization clauses`, with no `FeatureRelationshipPart*` tail and no way to express an ordered
`FeatureSpecialization` sequence.

Entries **#2, #3, #4, #5, #6, #8, #13, #16** are all consequences of that one divergence. A shared
`feature_declaration` component owned by the `FeatureDeclaration` production would close all of
them at once; that is the single highest-leverage change on this list and is called out again in
the burn-down order.

---

# Ranked inventory

Ranking is (occurrences × severity), with *silently shredded* weighted far above *rejected with a
diagnostic* — a rejected construct is visible and recoverable; a shredded one corrupts the model
and passes every existing gate.

---

## 1. Bare `/* … */` comment member inside a KerML type body — SILENTLY SHREDS THE NEXT MEMBER

**Reproduction** (verified):

```
package P { class K { /* c */ feature q : T; } }
```

Emits:

```
package P {
    class K {
        feature;
        q : T;
    }
}
```

No diagnostic, no opacity hit. The `Comment` member is **deleted** and the following member is
split into two bare expressions. In other combinations it degrades to a recovery node instead:

```
package P { behavior K { /* c */ feature x : T [*] unions a, b; } }
→ recovered_calc_body_element at the `: T [*] unions a, b;` remainder
```

A bare comment as the *last* member of a type body is simply dropped:
`package P { class K { feature q : T; /* note */ } }` → the comment vanishes.

**Grammar verdict.** *Legal in both.* `Comment` (`kebnf:199`, `xtext:93`) makes the whole
`( 'comment' Identification … )?` group optional, so `body = REGULAR_COMMENT` alone is the
production's shortest legal spelling. `AnnotatingElement` → `MemberElement` → `NonFeatureMember`
→ `TypeBodyElement` (`kebnf:188/329/270/434`, `xtext:84/230/153/365`). Sources agree.

**Owning scope.** `calc_def_body_element` / `CalcDefBodyElement`. The bug is one missing guard:
the function already calls `ws_and_notes` specifically so that "a bare `/* ... */` [is left] for
this scope's annotating member" (its own comment, `constraint.rs:1277-1279`), and
`crate::parser::body::annotating_member` (`src/parser/body.rs:84`) *already handles* `/*`
(`body.rs:94-100`). But the dispatch guard at `constraint.rs:1318-1323` only tests for
`doc`/`comment`/`rep`/`language`/`@` — never `/*` — so a bare comment falls through to the
bare-expression fallback at the bottom of the function. `package_body_element` gets this right
(`package P { /* c */ class Q; }` round-trips cleanly).

**Occurrences.** 14, all in
`KerML Spec Annex A Examples/A-3-7-DecisionsAndMerges.kerml` (lines 13, 17, 20, 26, 30, 33, 37,
107, 110, 114, 118, 121, 124, 128).

Proof of blast radius: the `behavior Manufacture { … }` block (A-3-7 lines 12–42) parses into
**32 body members** with the comments present, and into a perfectly correct 14-member body — no
diagnostics, no opacity, clean emit — with just the seven bare comments deleted. Every `step`,
`succession` and `connector` member in that block is currently shredded.

**Failure mode.** Accepted but shredded silently (majority) / accepted into a recovery node
(2 of 14).

**Fix size.** **Trivial** — add `|| input.fragment().starts_with(b"/*")` to the annotating-member
guard in `calc_def_body_element`. The parser (`annotating_member`) already exists and already
handles it; it simply is not dispatched in this scope.

---

## 2. `featured by` (`TypeFeaturingPart`) on any feature/step/connector declaration

**Reproductions** (all verified):

```
package P { feature y1 : A featured by C; }                       → unsupported_grammar_form, opaque FeatureDecl
package P { class K { feature y1 : A featured by C; }}            → unrecognized_declaration_in_scope
package P { struct K { member feature y1 : A featured by C; }}    → unrecognized_declaration_in_scope
package P { struct K { member step focus [0..1] featured by S; }} → unrecognized_declaration_in_scope
package P { struct S { member connector d featured by S from a to b { feature q; } }} → unrecognized_declaration_in_scope
```

Control: `package P { struct K { member feature y1 : A; }}` round-trips cleanly — `member` is not
the problem, `featured by` is.

**Grammar verdict.** *Legal in both.* `TypeFeaturingPart : 'featured' 'by' OwnedTypeFeaturing
( ',' OwnedTypeFeaturing )*` (`kebnf:628`, `xtext:568`), reached from `FeatureRelationshipPart`
(`kebnf:614`, `xtext:556`) which is the `FeatureDeclaration` tail. Sources agree.

**Owning scope.** `kerml_feature` / `ast::KermlFeature` (`constraint.rs:1022`). `KermlFeature`
already carries sibling `FeatureRelationshipPart` slots — `chains`, `inverse_of`,
`type_relationships` (`constraint.rs:1136-1160`) — but has no `featured_by` field at all. The
same tail is needed on `kerml_connector_member`. At package scope, `feature_decl`
(`src/parser/package.rs:1062`) swallows the whole declaration into the opaque
`ast::FeatureDecl { keyword, text }` string node.

**Occurrences.** **72**, across 9 files (excluding occurrences inside `//` notes):
`Association Examples/ProductSelection_N_ary.kerml` 18 (e.g. `:38`, `:42`, `:47`),
`Variable Feature Examples/TimeVaryingCarDriver.kerml` 20 (e.g. `:65`, `:75`, `:115`),
`Variable Feature Examples/TimeVaryingFeatures.kerml` 14 (e.g. `:28`, `:35`),
`Variable Feature Examples/Enhancements/TimeVaryingSteps.kerml` 12 (e.g. `:4`, `:11`),
`Association Examples/ProductSelection_OwnedEnds.kerml` 4 (`:41`, `:45`, `:71`, `:75`),
`ProductSelection_UnownedEnds.kerml` 1 (`:29`),
`Simple Tests/Features.kerml` 1 (`:18`), `Simple Tests/Inverses.kerml` 1 (`:14`),
`Variable Feature Examples/Enhancements/Moments.kerml` 1 (`:36`).

The reported diagnostic count is far lower than 72 because a rejected outer member swallows every
nested `featured by` inside its recovery span.

**Failure mode.** Rejected with a diagnostic (type-body scope) / accepted into an opaque
`FeatureDecl` string node (package scope).

**Fix size.** **Small.** Add a `featured_by: Vec<Node<…>>` slot to `KermlFeature` and parse it in
the `FeatureRelationshipPart` tail beside the existing `chains`/`inverse_of` handling; the
comma-separated target list can reuse `crate::parser::lex::qualified_reference` exactly as
`kerml_type_relationship_clauses` (`src/parser/package.rs:875`) already does. The relationship
*element* spelling (`featuring F of y by C;`) is already implemented and can supply the node
shape. Removing the package-level `FeatureDecl` fallback for this shape is then mechanical.

---

## 3. `succession` declaration forms

**Reproductions** (verified). Supported today: `succession a then b;`, `succession s first a then
b;`, `succession s [1] first [1] a then [1] b;`. Rejected:

```
package P { behavior K { succession redefines p : L [1] first paint then dry; }} → unexpected_keyword_in_scope
package P { struct K { succession first startShot then operated; }}              → unexpected_keyword_in_scope
package P { class K { succession s1 : AS first a then b; }}                      → unexpected_keyword_in_scope
package P { class K { succession { end feature references a; }}}                 → unexpected_keyword_in_scope
package P { class K { succession s; }}                                           → unexpected_keyword_in_scope
```

Also silent at package scope (not exercised by this corpus, but worth knowing):
`package P { succession first a then b; }` emits `first a then b;` — the `succession` keyword is
dropped with no diagnostic.

**Grammar verdict.** *Legal in both.* `SuccessionDeclaration` (`kebnf:845`, `xtext:891`) is
`FeatureDeclaration ('first' end 'then' end)? | ('all')? ('first'? end 'then' end)?`.
Alternative 1 admits a full `FeatureDeclaration`, i.e. `redefines p`, `: L`, `[1]` in that order
(`FeatureSpecializationPart = FeatureSpecialization+ MultiplicityPart? FeatureSpecialization*`).
Alternative 2 admits an explicit `first` with no name. Sources agree.

**Owning scope.** `kerml_succession_member` / `ast::KermlSuccessionMember`
(`constraint.rs:747`), dispatched from `calc_def_body_element`'s `succession` arm
(`constraint.rs:1362`).

**Occurrences.** **22.**
`succession redefines N : T [m] first … then …` — 9:
`A-3-7-DecisionsAndMerges.kerml:112,116,119,126`;
`A-3-6-Sequences.kerml:58,60`; `A-3-8-ChangingFeatureValues.kerml:119,177,189`.
`succession first … then …` (no name) — 11:
`TimeVaryingSteps.kerml:31,32,33,34,35,36`; `TimeVaryingCarDriver.kerml:17,45,86`;
`TimeVaryingFeaturesEnhanced.kerml:105,134`.
Typed declaration — 1 (`Simple Tests/Connectors.kerml:28`); anonymous with body — 1
(`Connectors.kerml:24`).

**Failure mode.** Rejected with a diagnostic.

**Fix size.** **Small** if entry #6's shared `FeatureDeclaration` component exists (then it is
`declaration → optional first/then ends`); **medium** standalone, because the current parser has
no way to hold `redefines`/`typing` on a succession node.

---

## 4. `connector` declaration forms

**Reproductions** (verified). Supported today: `connector c from a to b;`,
`connector c : L [2] from [1] a to [1] b;`. Rejected:

```
package P { class K { connector ps1 : PS (a, b, c); }}                    → unrecognized_declaration_in_scope
package P { class K { connector ps2 : PS ([1] a, [0..1] b, [1] c); }}     → unrecognized_declaration_in_scope
package P { class K { connector redefines fixWheel : L from a to b; }}    → unrecognized_declaration_in_scope
package P { class K { connector :> a.c1 from a.a to a.b; }}               → unrecognized_declaration_in_scope
package P { class K { abstract connector c2 = c1; }}                      → unrecognized_declaration_in_scope
package P { class K { connector = c2 { end feature references a; }}}      → unrecognized_declaration_in_scope
package P { struct K { var connector drive from engine to transmission; }}→ unrecognized_declaration_in_scope
package P { struct S { member connector d from a to b; }}                 → unrecognized_declaration_in_scope
```

**Grammar verdict.** *Legal in both.*
`NaryConnectorDeclaration` (`kebnf:804`, `xtext:842`) gives `( end, end, end… )`;
`ConnectorEnd = (OwnedCrossMultiplicityMember)? (Name REFERENCES)? OwnedReferenceSubsetting`
(`kebnf:814`, `xtext:849`) gives `[1] myCart`;
`Connector = FeaturePrefix 'connector' ( FeatureDeclaration? ValuePart? | ConnectorDeclaration )
TypeBody` (`kebnf:789`, `xtext:824`) gives `connector c2 = c1;` and `connector = c2 { … }`;
`BasicFeaturePrefix` includes `isVariable ?= 'var'` (`kebnf:576`, `xtext:519`) giving
`var connector`; `TypeFeatureMember = MemberPrefix 'member' FeatureElement` (`kebnf:523`,
`xtext:379`) with `FeatureElement ∋ Connector` gives `member connector`. Sources agree on all six.

**Owning scope.** `kerml_connector_member` / `ast::KermlConnectorMember` (`constraint.rs:579`).
The node has hand-rolled `name`/`typing`/`multiplicity`/`from`/`to` slots only — no `ValuePart`,
no n-ary end list, no `FeatureSpecializationPart`. The `var`/`member` cases are a *dispatch*
miss in `calc_def_body_element` (`constraint.rs:1348`: `starts_with_keyword(after_visibility,
b"connector")` does not look past a `FeaturePrefix`).

**Occurrences.** **12.**
n-ary — 2 (`ProductSelection_N_ary.kerml:122,124`);
`redefines`/`:>` declaration — 5 (`A-3-3-OneToOneConnectors.kerml:56`,
`A-3-4-OneToUnrestrictedConnectors.kerml:57`, `A-3-5-TimingForStructures.kerml:107,198`,
`Simple Tests/Connectors.kerml:34`);
`ValuePart` — 2 (`Connectors.kerml:8,9`);
`var connector` — 2 (`TimeVaryingCarDriver.kerml:57`, `TimeVaryingFeaturesEnhanced.kerml:97`);
`member connector` — 1 (`TimeVaryingCarDriver.kerml:115`).

**Failure mode.** Rejected with a diagnostic.

**Fix size.** `var`/`member` dispatch: **trivial** (widen the keyword guard the way the
`part_usage`/`kerml_feature` arms already do). N-ary ends: **small** (new `ends: Vec<…>` variant
reusing `kerml_connector_end`, which already parses `[1] name`). `redefines`/`:>`/`ValuePart`
declarations: **small once #6 exists**, otherwise medium.

---

## 5. Unnamed feature declaration that starts with keyword-spelled `subsets` / `references`

**Reproductions** (verified):

```
package P { class K { feature subsets f; }}                → unrecognized_declaration_in_scope
package P { class K { feature references a; }}             → unrecognized_declaration_in_scope
package P { class K { feature subsets f { feature g; }}}   → unrecognized_declaration_in_scope
package P { assoc A { end feature references a; }}         → unrecognized_declaration_in_scope
package P { feature subsets system; }                      → unsupported_grammar_form, opaque FeatureDecl
```

The symbol spellings work: `feature :> system;`, `feature ::> a;`, `feature : T;` and
`feature redefines f { … }` all round-trip. So the trigger is precisely the *keyword* spellings
`subsets`/`references` in the position where the parser expects a `NAME`.

**Grammar verdict.** *Legal in both.* `FeatureDeclaration` alternative 2 is a bare
`FeatureSpecializationPart` with no `Identification` (`kebnf:601-607`, `xtext:547-554`), and
`SUBSETS = ':>' | 'subsets'`, `REFERENCES = '::>' | 'references'` (`kebnf:141,143`;
`xtext:600,606`) make the two spellings interchangeable. Sources agree.

**Owning scope.** `kerml_feature` / `ast::KermlFeature`. The name slot
(`constraint.rs:1067-1080`) only skips the name when the next byte is `:`, `[` or `{`; a
following `subsets`/`references` keyword falls into `crate::parser::lex::name`, which rejects a
reserved keyword, failing the whole member.

**Occurrences.** **11.**
`Simple Tests/Connectors.kerml:10,11,17,18,25,26` (`end feature references a;` inside
connector/binding/succession bodies — masked by #3/#4 today),
`A-3-8-ChangingFeatureValues.kerml:158,166,178,190` (`feature subsets a.b.c, d.e.f chains …;`),
`Simple Tests/Redefinition.kerml:14` (`feature subsets f { … }`).

**Failure mode.** Rejected with a diagnostic (type body) / opaque `FeatureDecl` (package body).

**Fix size.** **Trivial-to-small** — extend the "name is absent" lookahead to the reserved
`FeatureSpecialization` starter keywords (`subsets`, `references`, `crosses`, `typed`,
`redefines`), which the existing `specialization_clauses` parser then consumes unchanged.

---

## 6. `FeatureSpecializationPart` is modelled as fixed slots, not an ordered sequence

**Reproductions** (verified):

*Rejected:*
```
package P { class K { feature m :> f : T; }}    → unrecognized_declaration_in_scope
package P { class K { feature m :>> f : T; }}   → unrecognized_declaration_in_scope
package P { feature x1 subsets g typed by A subsets f typed by B; } → unsupported_grammar_form
```

*Silently reordered (no diagnostic, structurally different output):*
```
package P { feature x typed by A, B references f subsets g; }
→ package P { feature x typed by A, B :> g ::> f; }              (subsets moved ahead of references)

package P { class K { feature redefines startingAt : w { feature q; }}}
→ feature : w :>> startingAt { feature q; }                       (typing moved ahead of redefinition)

package P { struct S { member feature :>> Q::driver [0]; }}
→ member feature[0] :>> Q::driver;                                (multiplicity moved ahead)
```

**Grammar verdict.** *Legal in both.* `FeatureSpecializationPart =
FeatureSpecialization+ MultiplicityPart? FeatureSpecialization*` (`kebnf:632`, `xtext:573`) is an
**ordered, repeatable** sequence over `Typings | Subsettings | References | Crosses |
Redefinitions` (`kebnf:642`, `xtext:591`). Sources agree, including on the multiplicity position
in the middle.

**Owning scope.** `kerml_feature` / `ast::KermlFeature` (`constraint.rs:1022-1205`): a fixed
pipeline `name → multiplicity → typing → multiplicity → modifiers → specialization_clauses →
multiplicity`, storing results into separate `typing` / `subsets` / `redefines` / `references` /
`crosses` fields. Order is destroyed on the way in and re-invented on the way out by
`src/emit/…`.

**Occurrences.** 4 rejected outright (`ExtendedOccurrences.kerml:9,15`,
`Simple Tests/Features.kerml:11`, `MetadataTest.kerml`-adjacent) plus **pervasive silent
reordering** in every file that writes a specialization in non-canonical order — e.g.
`Simple Tests/Features.kerml:8`, `Simple Tests/Redefinition.kerml:8`,
`Simple Tests/Expressions.kerml:31`, `TimeVaryingCarDriver.kerml:81,90`.

**Failure mode.** Rejected with a diagnostic (order the pipeline cannot express) **and** accepted
but reordered silently (order it can express but not preserve).

**Fix size.** **Large — design decision required.** Replacing five `Option<…>` slots with one
ordered `Vec<FeatureSpecialization>` is a breaking AST change: `PARSE_AST_VERSION` bump, every
exhaustive emitter/snapshot/serializer match, and the SysML usage headers that share
`specialization_clauses` (`src/parser/usage.rs:568`). This is also the natural place to introduce
the shared `feature_declaration` component named in "the meta-cause" above — do them together or
the second one re-does the first.

---

## 7. Keyword-less feature member `:>> name : T;` / `:> name : T;` in a type body

**Reproductions** (verified):

```
package P { class K { :>> self : T; }}                       → recovered_calc_body_element
package P { class K { :>> timeSlices : T [1..*]; }}          → recovered_calc_body_element
package P { class K { :>> snapshots :> timeSlices : T [1..*]; }} → recovered_calc_body_element
package P { class K { :> timeSlices : T; }}                  → recovered_calc_body_element
```

Control: `package P { class K { :>> dimension = size(c); }}` is accepted (see #9 for its emit
defect). So the `:>>` arm exists but only for the *value-binding* shape.

**Grammar verdict.** *Legal in both.* `Feature` alternative 2 —
`( EndFeaturePrefix | BasicFeaturePrefix ) FeatureDeclaration` (`kebnf:562-568`,
`xtext:537-544`) — has an entirely optional `BasicFeaturePrefix`, so a member may begin directly
with a `FeatureSpecializationPart`. Sources agree.

**Owning scope.** `calc_def_body_element` (`constraint.rs:1370-1376`): the `:>>` arm dispatches
only to `crate::parser::attribute::feature_value_binding` (`src/parser/attribute.rs:569`), which
requires a `= value`. There is no `:>` arm at all.

**Occurrences.** **6** — `Simple Tests/MetadataTest.kerml:47`,
`Variable Feature Examples/Enhancements/ExtendedOccurrences.kerml:6,10,14,15,17`.

**Related, excluded:** the *keyword* spelling of the same production, `redefines X …;`, is
gap 61 — see the gap-61 section.

**Failure mode.** Rejected into a recovery node.

**Fix size.** **Small** — route the `:>>`/`:>` arm through the same declaration parser used for
`feature :>> x : T;` (which works) instead of only through `feature_value_binding`. Best done as
part of #6; the fix must cover the `: T`, `[m]`, `:> y`, `= v` and `{ body }` variants together.

---

## 8. Conjugation (`~` / `conjugates`) in a type/classifier/feature declaration

**Reproductions** (verified):

```
package P { class B conjugates A; }                       → unsupported_grammar_form, opaque ClassifierDecl
package P { struct B conjugates A; }                      → unsupported_grammar_form, opaque ClassifierDecl
package P { feature g ~ B::f; }                           → unsupported_grammar_form, opaque FeatureDecl
package P { type B conjugates A; }                        → unrecognized_declaration_in_scope
package P { classifier T { feature fuelOutPort ~ fuelInPort; }} → unrecognized_declaration_in_scope
```

Control: the *relationship element* spelling works —
`package P { conjugation c1 conjugate C1 conjugates O; }` round-trips (when named; see #10).

**Grammar verdict.** *Legal in both.* `ConjugationPart` in `TypeDeclaration` (`kebnf:406/399`),
`ClassifierConjugationPart` in `ClassifierDeclaration` (`xtext:479/470`), and
`ConjugationPart`/`FeatureConjugationPart` in `FeatureDeclaration` (`kebnf:604`, `xtext:551/729`).
The two sources name the fragments differently but accept identical surface syntax. Sources agree.

**Owning scope.** `package_body_element` / `PackageBodyElement` for the opaque
`ClassifierDecl`/`FeatureDecl` fallbacks (`src/parser/package.rs:1062,1072`);
`calc_def_body_element` / `CalcDefBodyElement` and `kerml_feature` for the in-body case.
`ast::KermlClassifierDecl` and `ast::KermlFeature` have no conjugation field.

**Occurrences.** **5** — `Simple Tests/Conjugation.kerml:6,8`, `Simple Tests/Features.kerml:36`,
`Simple Tests/Types.kerml:28,29`. (Types.kerml:28-29 report no diagnostic of their own because an
earlier failure in the same body suppresses the cascade; they are still opaque.)

Note: `Simple Tests/Conjugation.kerml` accounts for the corpus's **only** `ClassifierDecl` opacity
hit and one of the eight `FeatureDecl` hits.

**Failure mode.** Accepted into an opaque `ClassifierDecl`/`FeatureDecl` string node (package
scope) / rejected with a diagnostic (type body).

**Fix size.** **Small-to-medium** — one `conjugates: Option<Node<…>>` field on
`KermlClassifierDecl` and on `KermlFeature`, plus a `type`-keyword arm (see #17). The `~` /
`conjugates` operator lexing already exists via `crate::parser::lex::specialization_operator`
(`src/parser/lex.rs:1683`), and `optional_typings` already tracks an `is_conjugated` flag.

---

## 9. `emit_sysml` writes a bogus `''` short name for keyword-less `:>> x = expr;`

**Reproduction** (verified):

```
package P { class K { :>> dimension = size(c); }}
→ package P { class K { '' :>> dimension = size(c); } }
```

No diagnostic, no opacity hit. The emitted `''` is a *quoted empty declaration name*, which is not
what the source said and is not re-parseable as the same model.

**Grammar verdict.** The input is *legal in both* (same production as #7). The output is the
defect.

**Owning scope.** `emit_default_reference_usage` — `src/emit/structure.rs:1556`. Line 1565 does
`w.push_str(&format_name(&usage.name))` unconditionally; with an empty `name`, `format_name`
renders `''`.

**Occurrences.** **5** — `Simple Tests/MetadataTest.kerml:48`,
`Association Examples/ProductSelection_OwnedEnds.kerml:86,90`,
`Association Examples/ProductSelection_UnownedEnds.kerml:39,43`.

They are invisible today only because all five files contain some *other* opaque node that makes
`emit_sysml` fail before this can be observed. Fixing #7 or #2 will expose all five.

**Failure mode.** Accepted but emitted as something structurally different (silent).

**Fix size.** **Trivial** — skip the name when `usage.name.is_empty()`.

---

## 10. Relationship element with the optional `Identification` omitted

**Reproductions** (verified):

```
package P { specialization subclassifier B :> A; }                  → unrecognized_declaration_in_scope
package P { specialization subset mother subsets parent; }          → unrecognized_declaration_in_scope
package P { specialization redefinition A::vin redefines B::id; }   → unrecognized_declaration_in_scope
package P { specialization subtype x :> Base::things; }             → unrecognized_declaration_in_scope
package P { specialization typing f typed by B; }                   → unrecognized_declaration_in_scope
package P { disjoining disjoint A from B; }                         → unrecognized_declaration_in_scope
package P { inverting inverse B::g of A::f; }                       → unrecognized_declaration_in_scope
package P { conjugation conjugate C1 conjugates O; }                → unrecognized_declaration_in_scope
```

Every *named* counterpart works: `specialization S subclassifier B :> A;`,
`disjoining D disjoint A from B;`, `inverting Invert inverse B::g of A::h;`. The bare forms
without the leading keyword also work: `subclassifier C specializes A;`, `inverse B::g of A::f;`.

**Grammar verdict.** *Legal in both.* Every one of these productions writes
`( 'specialization' Identification )?` / `( 'disjoining' Identification )?` etc., where the
`.kebnf` `Identification` (`kebnf:152`) is itself fully nullable, and the Pilot writes the
equivalent `Identification?` (`xtext:390, 408, 426, 486, 634, 665, 683, 712`). Sources agree.

**Owning scope.** `package_body_element` / `PackageBodyElement`. Whatever consumes the
`specialization`/`disjoining`/`inverting`/`conjugation` keyword requires a following name.

**Occurrences.** **4** in the corpus, all `specialization`:
`Simple Tests/Classifiers.kerml:6`, `Simple Tests/Features.kerml:46,69`,
`Simple Tests/Types.kerml:18`. (The `disjoining`/`inverting`/`conjugation` variants do not appear
unnamed in this corpus but fail identically and share the fix.)

**Failure mode.** Rejected with a diagnostic.

**Fix size.** **Trivial** — make the `Identification` after the leading keyword optional in the
existing relationship-element parsers; the rest of each production already parses.

---

## 11. `intersects` on a `Feature`/`Step` declaration is SILENTLY DROPPED

**Reproductions** (verified):

```
package P { feature x intersects f, g; }               → package P { feature x; }
package P { class K { feature x intersects f, g; }}    → feature x;
package P { behavior B { step s intersects f, g; }}    → step s;
package P { feature x : T intersects f, g; }           → feature x : T;
```

No diagnostic, no opacity hit — the entire clause disappears. The same corpus line loses half its
meaning:

```
Simple Tests/Features.kerml:21  feature z1 intersects f,g differences y, y1, z;
→ feature z1 differences y, y1, z;
```

`intersects` on a **classifier/type** works correctly (`classifier E specializes C intersects A, B;`
round-trips) — this is `Feature`-specific.

**Grammar verdict.** *Legal in both.* `IntersectingPart` (`kebnf:423`, `xtext:352`) is a
`TypeRelationshipPart`, and `FeatureRelationshipPart ∋ TypeRelationshipPart` (`kebnf:614`,
`xtext:556`). Sources agree.

**Owning scope.** Exactly localisable: `crate::parser::usage::specialization_clauses`
(`src/parser/usage.rs:568`) greedily consumes `intersects f, g` into
`SpecializationClauses.intersects` (`usage.rs:68`), and `kerml_feature`
(`constraint.rs:1128-1132`) reads `clauses.subsets`, `clauses.redefines`, `clauses.references`
and `clauses.crosses` — **but never `clauses.intersects`**. Consuming it there also prevents the
later, correct `kerml_type_relationship_clauses` (`src/parser/package.rs:875`, which *does*
handle `Intersects`) from ever seeing it.

**Occurrences.** 2 — `Simple Tests/Features.kerml:21` (live, silent),
`Simple Tests/FeatureChains.kerml:31` (currently masked by #12).

**Failure mode.** Accepted but shredded silently.

**Fix size.** **Trivial** — either read `clauses.intersects` into `KermlFeature.type_relationships`
in `kerml_feature`, or stop `specialization_clauses` from claiming `intersects` on this path so
that `kerml_type_relationship_clauses` gets it. The second is the more honest fix: `intersects`
is a `TypeRelationshipPart`, not a `FeatureSpecialization`, in both grammars.

---

## 12. `unions` / `differences` / `disjoint from` operands that are feature chains

**Reproductions** (verified):

```
package P { feature h1 unions f, b.f, b.a; }                                  → unsupported_grammar_form
package P { feature x differences a.b; }                                      → unsupported_grammar_form
package P { feature x disjoint from a.b; }                                    → unsupported_grammar_form
package P { feature h2 differences b.f, b.a intersects f.a, g disjoint from h1; } → unsupported_grammar_form
```

Control: `package P { feature h1 unions f, g; }` round-trips. The trigger is a dotted operand.

**Grammar verdict.** *Legal in both.* `Unioning`/`Intersecting`/`Differencing` are each
`type = [QualifiedName] | ownedRelatedElement += OwnedFeatureChain` (`kebnf:505,509,513`;
`xtext` equivalents), and `OwnedFeatureChain = FeatureChain = chaining ('.' chaining)+`
(`kebnf:713-716`). Sources agree.

**Owning scope.** `kerml_type_relationship_clauses` — `src/parser/package.rs:875`, called from
`kerml_feature`. Line 904 uses `crate::parser::lex::qualified_reference`, which parses `::`
segments but not the `.` feature-chain separator.

**Occurrences.** 2 — `Simple Tests/FeatureChains.kerml:30,31`.

**Failure mode.** Accepted into an opaque `FeatureDecl` string node (package scope).

**Fix size.** **Small** — the dotted-chain target parser already exists and is exercised by
`specialization_clauses` (see `usage.rs` test
`specialization_clauses_accept_dotted_feature_chain_targets`, `usage.rs:885`); reuse it for the
type-relationship operand list.

---

## 13. Nested `package` / `namespace` member inside a KerML type body

**Reproductions** (verified):

```
package P { class C { package Q { class D; }}}     → recovered_calc_body_element
package P { class C { package Q { }}}              → recovered_calc_body_element
package P { type T { private package Q { … }}}     → recovered_calc_body_element
package P { class C { namespace N { class G; }}}   → recovered_calc_body_element
package P { type T { package Q; }}                 → SILENT: emits  'package'; Q;
package P { class K { namespace N; }}              → SILENT: emits  namespace; N;
```

**Grammar verdict.** *Legal in both.* `TypeBodyElement ∋ NonFeatureMember` (`kebnf:434`,
`xtext:365`) → `MemberElement` → `NonFeatureElement`, and `NonFeatureElement` explicitly lists
`Namespace`, `Package` and `LibraryPackage` (`kebnf:332-350`, `xtext:234-238`). Sources agree.

**Owning scope.** `calc_def_body_element` / `CalcDefBodyElement`. There is no `package` or
`namespace` arm in the dispatch chain at `constraint.rs:1318-1470`.

**Occurrences.** 2 — `Simple Tests/Classes.kerml:22` (`package P { }` inside a class body),
`Simple Tests/Types.kerml:9` (`private package P { … }` inside a `type` body; also entry #14).

**Failure mode.** Rejected into a recovery node (bodied form) / accepted but shredded silently
(bodyless `package Q;` form — no corpus occurrence, but the shape is reachable).

**Fix size.** **Trivial-to-small** — `crate::parser::package::package_` and the namespace parser
already exist and are reachable from `package_body_element`; they are simply not dispatched in
this scope. Needs a new `CalcDefBodyElement::Package`/`::Namespace` variant plus emitter arms.

---

## 14. Visibility prefix on a nested `package` / `library package` in a package body

**Reproductions** (verified):

```
package P { package Q { private package Q1a { class G; }}}  → recovered_package_body_element
package P { private package Q; }                            → recovered_package_body_element
package P { public package Q { class G; }}                  → recovered_package_body_element
package P { private library package Q { class G; }}         → recovered_package_body_element
```

Control: `package P { private class C; }` round-trips — visibility on non-package members is fine.

**Grammar verdict.** *Legal in both.* `NonFeatureMember = MemberPrefix ownedRelatedElement +=
MemberElement` (`kebnf:270`, `xtext:153`), `MemberPrefix = ( visibility = VisibilityIndicator )?`
(`kebnf:260`, `xtext:145`), `NonFeatureElement ∋ Package | LibraryPackage` (`kebnf:332`,
`xtext:236`). Sources agree.

**Owning scope.** `package_body_element` / `PackageBodyElement` (`src/parser/package.rs:2147`) —
the package arm is not reached through `visibility_prefix`.

**Occurrences.** 2 — `Simple Tests/Imports.kerml:17`, `Simple Tests/Types.kerml:9`.
This is the corpus's only `recovered_package_body_element` diagnostic.

**Failure mode.** Rejected into a recovery node.

**Fix size.** **Trivial** — look past `visibility_prefix` before the `package`/`library` keyword
test, exactly as `calc_def_body_element` already does with its `after_visibility` local.

---

## 15. `binding` declaration forms

**Reproductions** (verified). Supported: `binding a = b;`, `binding ab of a = b;`. Rejected:

```
package P { class K { binding ab1 : AS of a = b; }}          → unexpected_keyword_in_scope
package P { class K { binding { end feature references a; }}} → unexpected_keyword_in_scope
package P { class K { binding ab1 : AS; }}                   → unexpected_keyword_in_scope
```

At package scope the same shapes hit an explicit `UnsupportedGrammar` node
("the spec-valid BindingConnectorAsUsage production is not implemented in package bodies") —
no corpus occurrence.

**Grammar verdict.** *Legal in both.* `BindingConnectorDeclaration` (`kebnf:831`, `xtext:875`)
alternative 1 is `FeatureDeclaration ('of' end '=' end)?`, which admits `ab1 : AS`; alternative 2
makes the whole end pair optional, which with `TypeBody = '{' … '}'` admits `binding { … }`.
Sources agree.

**Owning scope.** `kerml_binding_member` / `ast::KermlBindingMember` (`constraint.rs:693`).

**Occurrences.** 2 — `Simple Tests/Connectors.kerml:16,20`.

**Failure mode.** Rejected with a diagnostic.

**Fix size.** **Small** standalone; free once #6's shared `FeatureDeclaration` lands.

---

## 16. `succession flow` (`SuccessionFlow`)

**Reproduction** (verified):

```
package P { behavior K { succession flow exposure[1] of Exposure from a.x to b.y; }}
→ unexpected_keyword_in_scope (`succession` in calc body)
```

**Grammar verdict.** *Legal in both.* `SuccessionFlow = FeaturePrefix 'succession' 'flow'
FlowDeclaration TypeBody` (`kebnf:1307`, `xtext:1001`); `FeatureElement ∋ SuccessionFlow`
(`kebnf:360`, `xtext:273`). Sources agree.

**Owning scope.** `calc_def_body_element` / `CalcDefBodyElement` — the `succession` arm
(`constraint.rs:1362`) hands the input to `kerml_succession_member`, which does not know about
the `flow` continuation.

**Occurrences.** 1 — `Behavior Examples/TakePicture.kerml:14`.

**Failure mode.** Rejected with a diagnostic.

**Fix size.** **Small, but must be sequenced after gap 61** — `SuccessionFlow` shares
`FlowDeclaration` with `Flow`, which is the production gap 61 is currently implementing. Fixing it
independently would duplicate that work.

---

## 17. `type` keyword member inside a KerML type body

**Reproduction** (verified):

```
package P { class K { type T2 :> A; }}  → recovered_calc_body_element at the `:> A;` remainder
```

Control: `package P { type T { struct S :> A; }}` round-trips — `struct`, `class`, `datatype`,
`behavior`, `assoc`, `metaclass`, `function`, `predicate`, `interaction` and `classifier` are all
dispatched; only `type` is missing.

**Grammar verdict.** *Legal in both.* `NonFeatureElement ∋ Type` (`kebnf:335`, `xtext:240`),
reached from `TypeBodyElement → NonFeatureMember`. Sources agree.

**Owning scope.** `calc_def_body_element` / `CalcDefBodyElement` — the nested-classifier keyword
list at `constraint.rs:1405-1423` omits `b"type"`.

**Occurrences.** 1 — `Simple Tests/Types.kerml:10` (masked today by #13/#14 on line 9).

**Failure mode.** Rejected into a recovery node.

**Fix size.** **Trivial** — add `b"type"` to the existing keyword list; `kerml_classifier_structured`
(`src/parser/package.rs:836`) already parses the shape.

---

## 18. Repeated `chains` in one feature declaration

**Reproduction** (verified):

```
package P { feature b_f_a chains b chains f.a; }  → unsupported_grammar_form, opaque FeatureDecl
```

Control: `package P { feature b_f_a chains b.f.a; }` round-trips.

**Grammar verdict.** *Legal in both.* `FeatureRelationshipPart*` (`kebnf:607`, `xtext:553`) is a
Kleene star over a set that includes `ChainingPart`, so two `chains` clauses are admitted.
Sources agree.

**Owning scope.** `kerml_feature` / `ast::KermlFeature` — `chains` is a single
`Option<ReferencePath>` (`constraint.rs:1136-1141, 1197`).

**Occurrences.** 1 — `Simple Tests/FeatureChains.kerml:33`.

**Failure mode.** Accepted into an opaque `FeatureDecl` string node.

**Fix size.** **Small** — `Vec` instead of `Option`; naturally subsumed by #6's ordered
`FeatureRelationshipPart` list.

---

## 19. `doc <shortName> /* … */` followed by another member

**Reproductions** (verified):

```
package P { class A { doc <a> /* t */ feature q; }}          → recovered_calc_body_element
package P { class A { doc <a> /* t */ class B; }}            → recovered_calc_body_element
package P { class A { doc <a> /* t */ comment /* u */ }}     → SILENT: emits  doc <a> 'comment'  then  /* u */
```

Controls that work: `doc <a> /* t */` as the sole member; `doc d /* t */ feature q;` (long name);
`doc /* t */ feature q;` (no name); `doc locale "en_US" /* t */ …`.

**Grammar verdict.** *Legal in both.* `Documentation = 'doc' Identification ('locale' STRING)?
body = REGULAR_COMMENT` (`kebnf:208`) / `'doc' Identification? …` (`xtext:102`), with
`Identification = ('<' shortName '>')? (name)?` (`kebnf:152`) / `'<' shortName '>' (name)? | name`
(`xtext:44`). Sources agree that `doc <a> /* text */` is a complete member.

**Owning scope.** `doc_comment` — `src/parser/requirement.rs:754`. After `'<' a '>'`, the optional
`declaredName` slot inside `identification` skips comments, so it walks past this member's own
`/* t */` body and adopts the *next* member's leading token as the doc's name — the exact hazard
the neighbouring `locale` guard at `requirement.rs:760-770` was written to prevent, just not
applied to the short-name branch.

**Occurrences.** 1 — `Simple Tests/Comments.kerml:43`.

**Failure mode.** Rejected into a recovery node, or (when the next member starts with `comment`)
accepted but shredded silently.

**Fix size.** **Trivial** — use `ws` rather than `ws_and_comments` for the optional name after
`'<' NAME '>'`, mirroring the existing comment on `requirement.rs:773-778`.

---

## 20. `dependency` emit inserts an unauthored `from`

**Reproduction** (verified):

```
package P { dependency Client to Supplier; }  → package P { dependency from Client to Supplier; }
```

**Grammar verdict.** *Both spellings legal in both sources* —
`'dependency' ( Identification? 'from' )? client … 'to' supplier …` (`kebnf:168`, `xtext:66`).
Semantically identical; only the authored spelling is lost.

**Owning scope.** `src/parser/dependency.rs` / `ast::Dependency` (the AST has no "was `from`
authored?" fact) and the corresponding emitter.

**Occurrences.** 1 — `Simple Tests/Dependencies.kerml:18`. (`:11` and `:12` do write `from`.)

**Failure mode.** Accepted but re-spelled (fidelity only, not structural).

**Fix size.** **Trivial** — record the authored presence of `from` as a typed field.

---

# Known, in flight — gap 61 (excluded from the ranking)

I found these and am recording occurrences only. **No fix is proposed for them here.**

* `flow a.y to b.x1;` — `Simple Tests/Behaviors.kerml:18`; `abstract flow msg of C;` — `:20`.
  Both silently shred into bare expressions (`'flow'; a.y; 'to'; b.x1;`). These are the corpus's
  only two shredding artifacts visible in the emit sweep over the 29 diagnostic-free files.
* `redefines <name> …;` (keyword-spelled, no `feature` keyword) — **19 occurrences**:
  `Variable Feature Examples/Enhancements/Moments.kerml:8,9,10,14,15,22,27,32`;
  `A-3-8-ChangingFeatureValues.kerml:13,16,20,36,37,44,45,54,55,64,65`.

  Worth flagging for whoever is on gap 61: the variants behave differently and the fix must cover
  all of them. `redefines x [0];`, `redefines x : T;` and `redefines x = e;` are **silently
  shredded** into `'redefines'; x …;`, while `redefines x : T { … }` and
  `redefines x : T [1] subsets y;` instead produce a `recovered_calc_body_element` recovery node.
  A fix that only handles the `[m]` spelling would leave 15 of the 19 corpus occurrences broken.
* `message m of T;` — **no occurrences** in the KerML corpus. Independently confirmed: `message`
  appears nowhere in `KerML-textual-bnf.kebnf` and nowhere in `KerML.xtext`.
* Entry **#16** (`succession flow`) and entry **#7** (the `:>>` symbol spelling of #61's
  `redefines`) are adjacent to this work and should be sequenced with it.

---

# INVALID INPUT

**None.** Every construct in this inventory was checked against both `KerML-textual-bnf.kebnf`
and the Pilot `KerML.xtext`, and every one is admitted by both. That is itself the finding: for
this corpus, "the spec example file is malformed" is not an available explanation — all 29
diagnostic-carrying files are carrying parser gaps, not authoring errors.

Two shapes I actively tried to classify as INVALID INPUT and could not:

* `specialization subclassifier B :> A;` (no name) — the `.kebnf` `Identification` production is
  itself fully nullable and the Pilot writes `Identification?` at every call site. Legal.
* Bare `/* … */` as a type-body member — the `('comment' …)?` group is optional in both, leaving
  `body = REGULAR_COMMENT` as a complete `Comment`. Legal, and this turned out to be the single
  most damaging entry on the list.

One shape that *is* illegal in both but does not occur in the corpus, recorded so nobody
"fixes" it: a bare `type T;` with no specialization or conjugation — `TypeDeclaration` requires
`( SpecializationPart | ConjugationPart )` (`kebnf:396` requires one or more; `xtext:326` requires
exactly one). Every `type` declaration in this corpus carries one.

---

# Spec (`.kebnf`) vs. Pilot (`.xtext`) disagreements

None of these affect a corpus construct or an inventory verdict; recorded because the instruction
was to surface them rather than pick a side.

| # | Production | `KerML-textual-bnf.kebnf` | `KerML.xtext` | Effect |
|---|---|---|---|---|
| D1 | `TypeDeclaration` | `( SpecializationPart \| ConjugationPart )+` (`:396`) | `( SpecializationPart \| ConjugationPart )` — no quantifier (`:326`) | Spec admits `type T :> A conjugates B;`, Pilot does not. Not exercised by this corpus. |
| D2 | `Identification` | `('<' NAME '>')? (NAME)?` — nullable (`:152`) | `'<' NAME '>' (NAME)? \| NAME` — non-nullable (`:44`), but every call site writes `Identification?` | Net-equivalent. Matters only if you read the spec production in isolation and conclude a name is required (see #10). |
| D3 | `PayloadFeature` alt 4 | `OwnedMultiplicity ( OwnedFeatureTyping )?` (`:1327`) | `OwnedMultiplicity OwnedFeatureTyping` — mandatory (`:1023`) | Spec admits a payload feature with a multiplicity and no typing; Pilot does not. Not exercised. |
| D4 | `TypeFeaturingPart` | repetition assigns `ownedTypeFeaturing += …` (`:630`) — inconsistent with the first assignment on `:629` | both positions assign `ownedRelationship += …` (`:570-571`) | Apparent spec typo. Both accept `featured by A, B` identically; only the abstract-syntax binding differs. |
| D5 | naming | `Crosses`, `OwnedCrossFeature`, `OwnedCrossMultiplicity`, `ConjugationPart` (reused for features) | `Crossings`, `OwnedCrossingFeature`, `OwnedCrossingMultiplicity`, separate `FeatureConjugationPart` / `ClassifierConjugationPart` | Fragment naming only; identical surface syntax. Use the `.kebnf` names in this repo, per AGENTS.md. |

---

# Proposed burn-down order

**Wave 1 — trivial, and every one closes a silent-corruption path (do these first, in a day):**

1. **#1** bare `/* … */` dispatch in `calc_def_body_element`. One `starts_with(b"/*")` guard; the
   `annotating_member` parser is already written. Unblocks 14 silently shredded members and turns
   `A-3-7-DecisionsAndMerges.kerml`'s behavior bodies from 32 junk members into 14 correct ones.
2. **#11** `intersects` dropped by `kerml_feature`. One clause read (or better: stop
   `specialization_clauses` claiming it).
3. **#9** `''` empty-name guard in `emit_default_reference_usage`. Do it *before* #2/#7, which
   will otherwise expose five new corrupt emissions.
4. **#19** `doc <s>` name-slot lookahead (`ws` not `ws_and_comments`).
5. **#17** add `b"type"` to the nested-classifier keyword list.
6. **#20** record the authored `dependency … from`.

**Wave 2 — small, dispatch-and-slot work with the best occurrence-per-line ratio:**

7. **#2 `featured by`** — 72 occurrences, the largest single win in the corpus. New slot on
   `KermlFeature` + a `FeatureRelationshipPart` tail clause modelled on the existing
   `chains`/`inverse_of` handling.
8. **#14** visibility past `package` in `package_body_element`, and **#13** `package`/`namespace`
   arms in `calc_def_body_element`. Both are "the parser exists, it just isn't dispatched here".
9. **#5** unnamed feature starting with keyword `subsets`/`references`; **#10** optional
   `Identification` on the relationship elements. Both are lookahead widenings.
10. **#12** feature-chain operands in `unions`/`differences`/`disjoint from` — reuse the dotted
    target parser `specialization_clauses` already has.
11. **#4 (partial)** `var connector` / `member connector` dispatch, and the n-ary connector end
    list (`kerml_connector_end` already parses `[1] name`).

**Wave 3 — the structural slice (needs a design decision; see below):**

12. **#6** `FeatureSpecializationPart` as an ordered sequence, delivered together with a shared
    `feature_declaration` component owned by the `FeatureDeclaration` production
    (`kebnf:601` / `xtext:547`). This is the meta-cause; landing it closes the remainder of
    **#3** (succession), **#4** (connector declarations), **#7** (`:>>` typed member),
    **#15** (binding), **#18** (repeated `chains`) more or less for free, and removes the
    silent specialization reordering that currently affects every KerML file.
13. **#8** conjugation on type/classifier/feature declarations — best done inside the same slice,
    since `FeatureDeclaration`'s third alternative *is* `ConjugationPart`.

**Wave 4 — coordinate with gap 61:**

14. **#16** `succession flow`, and the `:>>`-symbol half of **#7**, once `FlowDeclaration` and the
    keyword-less redefinition member land from the gap-61 work.

## Items needing a design decision rather than an implementation

* **#6 / Wave 3.** Replacing `KermlFeature`'s five fixed specialization slots with one ordered
  sequence is a breaking AST change (`PARSE_AST_VERSION` bump; every exhaustive emitter,
  serializer, opacity walker and snapshot formatter; plus the SysML usage headers that share
  `crate::parser::usage::specialization_clauses`). Someone has to decide the node shape and the
  migration order before code is written.
* **Whether to introduce a shared `feature_declaration` component at all.** AGENTS.md is
  deliberately hostile to universal "declaration" nodes — but it permits factoring "at boundaries
  owned by an authoritative grammar production", and `FeatureDeclaration` is exactly that, named
  identically in both grammar sources and reused by eight productions. That still needs an
  explicit decision, plus the vertical-slice sequencing AGENTS.md requires (pin the productions
  and FIRST sets, migrate `Feature` + `Step` first, then `Connector`/`Binding`/`Succession`, then
  delete the superseded hand-rolled slots).
* **#8 conjugation.** Whether the conjugated type is a variant of the existing typing
  relationship (`optional_typings` already carries `is_conjugated`) or a separate
  `ConjugationRelationship` node. The `.kebnf` reuses `ConjugationPart` for both types and
  features while the Pilot splits it into `ClassifierConjugationPart` / `FeatureConjugationPart`
  (disagreement D5) — the node design should be made deliberately rather than inherited from
  whichever source is read first.
* **#16 `succession flow`.** Not independently implementable; it is a scoping decision on the
  gap-61 slice, not a separate piece of work.
