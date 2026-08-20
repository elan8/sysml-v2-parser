# SysML corpus coverage inventory — `sysml-v2-release/sysml/` (251 files)

## Measured commit

**`6d54b85`** — `fix(parser): give KerML type bodies the flow and redefinition members their grammar
admits`, tip of `fix/bare-comment-dispatch-and-requirement-usage-members`, four commits ahead of
`main` (`b6291cc`). Parser v0.54.0, `PARSE_AST_VERSION` 184.

Branch commits included in this measurement:

```
6d54b85 fix(parser): give KerML type bodies the flow and redefinition members their grammar admits
e7355c6 fix(parser): a constraint body owns the `return` member its CalculationBody grants it
02fdd16 fix(docs): resolve four rustdoc failures that `-D warnings` turned into CI errors
ec47463 fix(parser): dispatch keyword-less comments, and admit the usage families a requirement body inherits
```

### Post-branch corpus numbers for `sysml/` — **identical to the pre-branch baseline**

| | `b6291cc` (main) | `6d54b85` (branch) |
|---|---|---|
| files | 251 | 251 |
| files with diagnostics | 67 | **67** |
| total diagnostics | 183 | **183** |
| `recovery_cascade_suppressed` (consequential) | 32 | **32** |
| primary diagnostics | 151 | **151** |
| opacity | `{ExtendedLibraryDecl: 13, ParseError: 216}` | **`{ExtendedLibraryDecl: 13, ParseError: 216}`** |

Diagnostic-code histogram on `6d54b85` (unchanged from main):

```
46 unexpected_keyword_in_scope        5 recovered_port_body_element
32 recovery_cascade_suppressed        4 recovered_requirement_body_element
20 unrecognized_declaration_in_scope  4 recovered_constraint_body_element
16 recovered_part_usage_body_element  3 unsupported_annotation_syntax
13 unsupported_grammar_form           3 recovered_package_body_element
 9 recovered_action_body_element      2 recovered_occurrence_body_element
 7 recovered_use_case_body_element    2 missing_body_or_semicolon
 7 recovered_connection_def_body_element  1 recovered_state_body_element
 6 missing_semicolon                  1 recovered_part_def_body_element
                                      1 recovered_interface_def_body_element
                                      1 recovered_calc_body_element
```

**I diffed the two scans field by field (file, line, column, code, cascade flag) and they are
byte-identical — 412 records each, zero differences.** So for the `sysml/` corpus specifically, none
of my counts is stale: `recovered_requirement_body_element` is still 4, `recovered_constraint_body_element`
is still 4, and `unexpected_keyword_in_scope` is still 46. The four branch commits are real fixes —
I verified each independently below — but the constructs they admit do not occur in `sysml/` in the
shapes the corpus uses. Their measurable effect must be in `kerml/` or in the snapshot fixtures.

**All 50+ minimal reproductions in this report were re-executed on `6d54b85`. Every one still
reproduces; none was dropped.**

### What the branch commits do fix (independently verified on both commits)

| Input | on `b6291cc` | on `6d54b85` |
|---|---|---|
| `package P { requirement def R { action a1; } }` | `unexpected_keyword_in_scope` | clean round-trip |
| `package P { requirement def R { part p1; } }` | `unexpected_keyword_in_scope` | clean round-trip |
| `package P { constraint def C { return x : Real; } }` | rejected | clean round-trip |
| `package P { action def A { /* note */ in x; } }` | comment dropped | comment retained |
| `package P { constraint c { /* note */ part p; } }` | comment dropped **and** `part p;` shredded to `'part'; p;` | clean round-trip |

Two residual defects in exactly this area are new entries **25** and **26** below — one of them is
the same "following member misparsed" symptom, still live in constraint bodies.

**Second measurement (new here): silent shredding.** A scanner parsed every file that is *both*
diagnostic-free and opacity-free, re-emitted it with `emit_sysml`, and compared comment-stripped token
multisets. **121 of the 184 clean files round-trip to a different token multiset**, and 3 more fail
`emit_sysml` outright. After subtracting legitimate canonicalisation (`redefines`→`:>>`,
`subsets`→`:>`, `and`→`&&`, `()`→`null`, `references`→`::>`, brace/`;` layout, `//` note trivia, and
`//*…*/` PREFIX_COMMENT trivia — all verified correct), the residue is the silent-shred entries below.
Diagnostics and opacity understate the real gap by roughly a factor of two.

**Every grammar verdict below was checked against two independent sources.** Line/column references
into the normative spec rendering are `SysML:<n>` for `sysml-v2-release/bnf/SysML-textual-bnf.kebnf`
and `KerML:<n>` for `KerML-textual-bnf.kebnf`. References into the OMG Pilot Implementation (the
Xtext reference parser the example corpus was authored against) at
`/Users/luke/Documents/GitHub/SysML-v2-Pilot-Implementation` are `Pilot-SysML:<n>` for
`org.omg.sysml.xtext/src/org/omg/sysml/xtext/SysML.xtext`, `Pilot-KerML:<n>` for
`org.omg.kerml.xtext/src/org/omg/kerml/xtext/KerML.xtext`, and `Pilot-Expr:<n>` for
`org.omg.kerml.expressions.xtext/src/org/omg/kerml/expressions/xtext/KerMLExpressions.xtext`.

**The cross-check changed three verdicts.** Everything I had provisionally classified INVALID INPUT
against the `.kebnf` alone turns out to be admitted by the Pilot grammar. There are now **zero**
INVALID INPUT entries and **three spec-vs-Pilot disagreements** — see the dedicated section at the
end. Every other verdict is confirmed by both sources.

---

## The single structural fact that explains most of this report

`UsageBody = DefinitionBody` (SysML:314) and `DefinitionBody = ';' | '{' DefinitionBodyItem* '}'`
(SysML:234). `DefinitionBodyItem` (SysML:237) is:

```
DefinitionBodyItem : Type =
      DefinitionMember | VariantUsageMember | NonOccurrenceUsageMember
    | ( SourceSuccessionMember )? OccurrenceUsageMember
    | AliasMember | Import
```

So **every** definition body and **every** usage body in SysML admits the *entire* union of
`DefinitionElement` (SysML:180), `NonOccurrenceUsageElement` (SysML:344), `OccurrenceUsageElement`
(SysML:352), `variant`, `alias`, and `import`. Only six scopes narrow or extend it:
`InterfaceBodyItem` (SysML:727), `ActionBodyItem`/`NonBehaviorBodyItem` (SysML:901/910),
`StateBodyItem` (SysML:1200), `CalculationBodyItem` (SysML:1366), `RequirementBodyItem` (SysML:1407),
`CaseBodyItem` (SysML:1513), `ViewDefinitionBodyItem`/`ViewBodyItem` (SysML:1587/1614),
`EnumerationBody` (SysML:522), `MetadataBody` (SysML:1677) — and every one of those is a *superset*
of `DefinitionBodyItem` except `InterfaceBodyItem`, `EnumerationBody` and `MetadataBody`.

**Pilot: identical.** `DefinitionBody`/`DefinitionBodyItem` at **Pilot-SysML:510-522** is the same
six-alternative rule (it names the optional successor `EmptySuccessionMember` rather than
`SourceSuccessionMember`); `UsageBody : DefinitionBody` at **Pilot-SysML:603**;
`ActionBodyItem` **Pilot-SysML:1368**, `InterfaceBodyItem` **Pilot-SysML:1113**,
`ViewBodyItem` **Pilot-SysML:2359**, `RequirementBodyItem` **Pilot-SysML:2039**,
`CalculationBodyItem` **Pilot-SysML:1956**. The two sources agree completely on the central claim.

The parser instead models each scope as a hand-written enum (`PartUsageBodyElement`,
`PortBodyElement`, `StateDefBodyElement`, …) whose arms are a proper subset of the universal
production. **46 `unexpected_keyword_in_scope` + 20 `unrecognized_declaration_in_scope` = 66 of the
151 primary diagnostics are one root cause: scope-narrowing that the grammar does not license.** They
are listed as separate entries below only where the *missing element parser* differs; where an
existing parser already exists and merely isn't dispatched, that is called out.

---

# Ranked inventory

Severity key: **S** = accepted but shredded silently (worst), **O** = accepted into an opaque /
recovery node (emit fails), **D** = rejected with a diagnostic (also becomes a `ParseError` node).

---

## 1. Package-level `attribute`/`connection` usage is parsed as a *definition* — **S** — ~19 files

### Reproduction (verified)
```sysml
package P { attribute x; }
```
emits
```sysml
package P { attribute def x; }
```
Also `attribute x : Real;`, `attribute x = 1.0;`, `attribute x : Real = 1.0;`, and `connection c;`.
`attribute x = 1.0;` *inside a part body* round-trips correctly — only the package/root scope is wrong.

### Grammar verdict
`AttributeDefinition : AttributeDefinition = DefinitionPrefix 'attribute' 'def' Definition`
(**SysML:511**) — `'def'` is **mandatory**. `AttributeUsage = UsagePrefix 'attribute' Usage`
(**SysML:514**). `ConnectionDefinition` (**SysML:674**) likewise requires `'def'`;
`ConnectionUsage` is **SysML:677**. `PackageMember = MemberPrefix (DefinitionElement | UsageElement)`
(**SysML:133**) admits both, unambiguously discriminated by the `def` keyword. The input is legal
SysML; the parser's classification is not.

**Pilot: agrees.** `AttributeDefKeyword : AttributeKeyword 'def'` (**Pilot-SysML:738**) — `'def'` is
a mandatory part of the keyword token itself, so a def-optional `attribute_def` has no basis in
either source. `AttributeDefinition` **Pilot-SysML:746**, `AttributeUsage` **Pilot-SysML:750**,
`ConnectionDefinition` **Pilot-SysML:1048**, `ConnectionUsage` **Pilot-SysML:1062**. Verdict solid.

### Owning scope
`src/parser/package.rs:1396-1414` — `try_package_body_dispatch!(… |i| attribute_def(i, false), PackageBodyElement::AttributeDef)`.
The `false` argument makes `def` **optional** in `attribute_def`, so it wins the `alt` before
`attribute_usage` (dispatched at :1408) ever runs. `PackageBodyElement::AttributeDef` vs
`::AttributeUsage` in `src/ast/package.rs:123/196`. Same pattern for the connection arm.

### Occurrences
19 files show a spurious `def` token on round-trip. Examples:
`src/validation/06-Individual and Snapshots/6-Individual and Snapshots.sysml` (+19 `def`),
`src/validation/15-Properties-Values-Expressions/15_01-Constants.sysml` (+7),
`src/examples/Simple Tests/ParameterTest.sysml` (+5).

### Failure mode
**S** — no diagnostic, no opacity hit. Every downstream consumer sees an `AttributeDef` where the
model has an `AttributeUsage`. `attribute def x : Real;` is not even re-parseable as authored
(definitions specialise with `:>`, SysML:231), so the emitted text is *also* wrong.

### Fix size
**Trivial→small.** `attribute_usage` already exists and is already wired into this scope. Change
`attribute_def(i, false)` to `attribute_def(i, true)` (require `def`) and let the existing
`attribute_usage` arm take the def-less form; do the same for the connection arm. The def-optional
flag itself should be deleted — it has no grammar basis anywhere.

---

## 2. `BracketExpression` — unit / index brackets are misparsed three different ways — **S/D** — ~13 files, ~42 lines

### Reproductions (verified)
```sysml
package P { part def A { attribute x = 60 [SI::mm]; } }   // → attribute x = 60 ['SI::mm'];
package P { part def A { attribute x = 10.0 [N * m]; } }  // → attribute x[N * m] = 10.0;
package P { requirement def R { require constraint { fuelEconomyResult > 30 [mi / gal] } } }
                                                          // → recovered_constraint_body_element
```
`60 [mm]` and `33 ['in']` (single simple operand) do round-trip correctly.

### Grammar verdict
`BracketExpression : OperatorExpression = PrimaryArgumentMember operator='[' SequenceExpressionListMember ']'`
(**KerML:1099**). The bracket operand is a full `OwnedExpression` — a qualified reference, an
operator expression, a sequence, anything. All three inputs are legal.

**Pilot: agrees, and is more explicit.** `PrimaryExpression` (**Pilot-Expr:299-323**) is
`BaseExpression` followed by a *repeatable postfix loop* whose alternatives are `#'('…')'`
(IndexExpression), **`operator = '[' operand += SequenceExpression ']'`** (**Pilot-Expr:307**,
producing a plain `OperatorExpression`), `->`, `.`-collect and `.?`-select. So `[…]` is unambiguously
a postfix *expression* operator with a full `SequenceExpression` operand — it is never a
multiplicity in that position. This makes the disambiguation rule in the "design decision" section
concrete rather than inferred.

### Owning scope
`src/parser/expr.rs` (the primary/postfix expression chain) — there is no `BracketExpression`
postfix production. `[…]` after a literal is currently absorbed either as a multiplicity
(`MultiplicityPart`, SysML:491) or as a single opaque name token.

### Occurrences
~28 lines with an operator inside the brackets across 9 files; ~14 lines with `::` across 8 files.
`src/validation/12-Dependency Relationships/12b-Allocation-1.sysml:14` (`0.0 [N*m]`),
`src/validation/15-Properties-Values-Expressions/15_10-Primitive Data Types.sysml:39-41` (`[SI::mm]`),
`src/training/33. Analysis/Analysis Case Definition Example.sysml:44` (`[mi / gal]`),
`src/examples/Geometry Examples/VehicleGeometryAndCoordinateFrames.sysml:51` (`['°']`).

### Failure mode
**S** for the first two (a qualified reference becomes a quoted single name — a direct violation of
the AGENTS.md rule that `::` is syntax, not characters; and a value expression becomes a
*multiplicity*, which changes the feature's cardinality). **D** for the operator-in-brackets form in
constraint bodies.

### Fix size
**Small→large.** Small if `expr.rs` already has a postfix-operator loop to hang `'['` off (it has
one for `#(` index and `.` chaining); large if the multiplicity-vs-bracket ambiguity needs a
lookahead design decision. Note the disambiguation rule is positional and mechanical: a
`MultiplicityPart` can only follow a `UsageDeclaration`, never an expression operand.

---

## 3. Connector ends: named ends and cross-multiplicity are dropped or rejected — **S/D** — ~12 files

### Reproductions (verified)
```sysml
// SILENT: end names and their reference-subsettings vanish
package P { part p { interface i : FuelInterface
    connect supplierPort ::> tankAssy.fuelTankPort to consumerPort ::> eng.engineFuelPort; } }
  // → interface i : FuelInterface connect tankAssy.fuelTankPort to eng.engineFuelPort;

// SILENT: cross-multiplicity vanishes
package P { part p { interface i : I connect [1] a ::> w.a to [1] b ::> h.b; } }
  // → interface i : I connect w.a to h.b;

// REJECTED: the 'references' spelling of the same production
package P { part p { connection : PressureSeat
    connect bead references t.bead to mountingRim references w.rim; } }
  // → recovered_part_usage_body_element
```

### Grammar verdict
```
ConnectorEnd : ReferenceUsage =                      // SysML:689
    ( OwnedCrossMultiplicityMember )?
    ( declaredName = NAME REFERENCES )?
    OwnedReferenceSubsetting
InterfaceEnd : PortUsage = …same shape…              // SysML:780
REFERENCES = '::>' | 'references'                     // KerML:138
```
`declaredName` is an **AST-bearing declared name**, and `OwnedCrossMultiplicity` (SysML:695) is an
owned relationship. Both inputs are legal and both carry model content.

**Pilot: agrees, character for character.** `ConnectorEnd` **Pilot-SysML:998-1002**,
`OwnedCrossMultiplicityMember` **Pilot-SysML:1004**, `InterfaceEnd` **Pilot-SysML:1170ff**,
`ReferencesKeyword` = `'references' | '::>'`. Verdict solid.

### Owning scope
`src/parser/connector.rs` (`ConnectorEndMember` / `ConnectorEnd`), consumed by
`src/parser/connection.rs`, `src/parser/interface.rs`, `src/parser/allocation.rs`.
The `references` spelling *is* supported for `allocate` (`12b-Allocation-1.sysml` round-trips
`allocate logical references torqueGenerator to …`), so the parser is inconsistent between the
`allocate` and `connect` paths.

### Occurrences
~34 lines across 12 files match a named/`::>` end. Silent examples:
`src/training/11. Interfaces/Interface Example.sysml:14-16`,
`src/training/13. Flows/Flow Interface Example.sysml:15-17`,
`src/examples/Simple Tests/ConjugationTest.sysml:39-40`.
Rejected examples: `src/training/09. Connections/Connections Example.sysml:29`,
`src/examples/Vehicle Example/SysML v2 Spec Annex A SimpleVehicleModel.sysml:968` and `:999`.

### Failure mode
**S** for `::>`-spelled named ends and cross-multiplicity (the ends are the *only* place the
connection's role names live — losing them makes the connection unresolvable);
**D** (3 diagnostics) for the `references`-spelled and multiplicity-prefixed forms.

### Fix size
**Small.** `ConnectorEnd` already parses `OwnedReferenceSubsetting`. Add the optional
`OwnedCrossMultiplicityMember` and `declaredName NAME REFERENCES` prefix to that one function and
give the node the two fields; the `allocate` path already demonstrates the `references` token
handling that can be reused.

---

## 4. `event` occurrence usage is not dispatched in port bodies — **D** — 16 occurrences, 2 files

### Reproduction (verified)
```sysml
package P { port p { event occurrence setSpeedReceived; } }
package P { port p { event sendSensedSpeed.sourceEvent; } }
```
→ `unexpected_keyword_in_scope`: "unexpected keyword `event` in port body".

### Grammar verdict
```
EventOccurrenceUsage =                                // SysML:588
    OccurrenceUsagePrefix 'event'
    ( OwnedReferenceSubsetting FeatureSpecializationPart? | 'occurrence' UsageDeclaration? )
    UsageCompletion
```
`EventOccurrenceUsage` is a `StructureUsageElement` (SysML:361), reachable from a port body via
`UsageBody = DefinitionBody` → `OccurrenceUsageMember`. Legal.

**Pilot: agrees.** `EventOccurrenceUsage` **Pilot-SysML:865-870** is the same rule;
`StructureUsageElement` **Pilot-SysML:658** lists it; `UsageBody : DefinitionBody`
**Pilot-SysML:603**. Verdict solid.

### Owning scope
`src/parser/port.rs` — `port_body_element` / `PortBodyElement`. `EventOccurrenceUsage` is already
parsed elsewhere (it appears in occurrence and part bodies); the arm is simply absent here.

### Occurrences
16. `…/SysML v2 Spec Annex A SimpleVehicleModel.sysml:788,792,800,803,806,813,825,829,837,840,844,851`;
`src/training/27. Occurrences/Interaction Realization-2.sysml:56,63,68,73`.

### Failure mode
**D**, and it cascades: 10 of the 32 `recovery_cascade_suppressed` diagnostics are downstream of
these.

### Fix size
**Trivial.** Existing `event_occurrence_usage` parser, new `PortBodyElement` variant + dispatch arm.

---

## 5. Keyword-less usages (`DefaultReferenceUsage`) not dispatched in package and attribute bodies — **D** — 15 occurrences, 5 files

### Reproductions (verified)
```sysml
package P { serviceDiscovery :~ ServiceDiscoveryDD; }   // conjugated typing
package P { causeA ::> a; }                             // reference subsetting
package P { T1 = 10.0 [N * m]; }                        // value only
package P { attribute def AD { t : TimeValue :>> domainValue; } }
```
→ `unrecognized_declaration_in_scope`.

### Grammar verdict
`DefaultReferenceUsage : ReferenceUsage = RefPrefix Usage` (**SysML:332**), where
`RefPrefix` (SysML:275) is entirely optional and `Usage = UsageDeclaration UsageCompletion`
(SysML:305). It is the first alternative of `NonOccurrenceUsageElement` (**SysML:345**), reachable
from `PackageMember` (SysML:133) and from any `DefinitionBodyItem`. All four forms are legal.
`ConjugatedPortTyping` is SysML:650; `References`/`::>` is SysML:456.

**Pilot: agrees, and is broader.** `DefaultReferenceUsage` **Pilot-SysML:630-632** is
`( isEnd ?= 'end' )? RefPrefix UsageDeclaration ValuePart? UsageBody` — it adds an optional `end`
the spec rendering omits (see disagreement D2), but the keyword-less core is identical.
`NonOccurrenceUsageElement` **Pilot-SysML:645** lists it first. Crucially,
`FeatureDeclaration` (**Pilot-KerML:547-554**) has a `FeatureSpecializationPart`-only alternative,
so a declaration with *no name at all* is explicitly supported. Verdict solid.

### Owning scope
`src/parser/package.rs` (`package_body_element` / `PackageBodyElement`) and
`src/parser/attribute.rs` (`attribute_body_element` / `AttributeBodyElement`). Note
`PackageBodyElement::Ref` already exists (`src/ast/package.rs:209`) for the `ref`-prefixed spelling —
this is the same production with the optional `ref` omitted.

### Occurrences
15. `src/examples/Arrowhead Framework Example/AHFNorwayTopics.sysml:49,71,72,105,106`;
`src/examples/Cause and Effect Examples/CauseAndEffectExample.sysml:41-44`;
`src/examples/Vehicle Example/VehicleUsages.sysml:14`;
`…/SysML v2 Spec Annex A SimpleVehicleModel.sysml:577,578,579`;
`src/examples/Analysis Examples/Vehicle Analysis Demo.sysml:166,167`.

### Fix size
**Small.** Reuse `ref_decl`'s body with the `ref` keyword made optional (that is exactly what
`DefaultReferenceUsage` is), add the variant to two body enums.

---

## 6. Modifier and prefix flags silently dropped from otherwise-clean declarations — **S** — ~100 lines, ~25 files

This is one entry because it is one architectural cause: the declaration node has no field for the
flag, so the parser consumes and discards it. Each sub-form is a separate missing field.

### Reproductions (all verified, all with zero diagnostics and zero opacity)

| # | Input | Emitted | Grammar |
|---|-------|---------|---------|
| 6a | `package P { state s parallel { state a; } }` | `state s { state a; }` | `StateUsageBody : (isParallel ?= 'parallel')?` **SysML:1263**; `StateDefBody` **SysML:1195** |
| 6b | `package P { abstract occurrence situations : Situation[*] nonunique; }` | `… : Situation[*];` | `MultiplicityPart … { isUnique = false } 'nonunique'` **SysML:491** |
| 6c | `package P { occurrence o { then timeslice ownership[0..*] ordered { attribute x; } } }` | `… ownership[0..*] { … }` | `MultiplicityPart … isOrdered ?= 'ordered'` **SysML:491** |
| 6d | `package P { part p { exhibit state vs : VS; } }` | `state vs : VS;` | `ExhibitStateUsage` **SysML:1268** is a *distinct metaclass* from `StateUsage` **SysML:1259** |
| 6e | `package P { individual occurrence ind : Ind, Occ { } }` | `individual occurrence ind : Ind { }` | `Typings = TypedBy ( ',' FeatureTyping )*` **SysML:431** |
| 6f | `package P { connection def C { end port p3 : PP; } }` | `end p3 : PP;` | **Pilot only** — `OccurrenceUsagePrefix` **Pilot-SysML:836** admits `EndUsagePrefix`; the spec rendering **SysML:564** does not. See disagreement **D1** |
| 6g | `package P { connection def C { end #original r1 : R1; } }` | `end r1 : R1;` | `ExtendedUsage = UnextendedUsagePrefix UsageExtensionKeyword+ Usage` **SysML:1699** |
| 6h | `package P { connection def C { end [*] ref cause : Situation; } }` | `end cause : Situation[*];` | `EndUsagePrefix = 'end' (OwnedCrossFeatureMember)?` **SysML:285** + `ReferenceUsage` **SysML:335** |
| 6i | `package P { enum def E { enum red; } }` | `red;` | `EnumeratedValue = 'enum'? Usage` **SysML:531** |
| 6j | `package P { port def PD { port p { in attribute engineTorque : Torque; } } }` | `in engineTorque : Torque;` | `AttributeUsage = UsagePrefix 'attribute' Usage` **SysML:514** |
| 6k | `package P { part p { ref :>> system; } }` | `ref system;` | `Redefinitions`/`Redefines` **SysML:472/475**; `Identification` all-optional **SysML:42** |
| 6l | `package P { part p :>> causes :> situations; }` | `part p :> situations :>> causes;` | `FeatureSpecializationPart = FeatureSpecialization+ …` **SysML:425** — order is authored |

**Pilot cross-check for the rest of the table — all agree.** `StateDefBody`/`StateUsageBody`
`( isParallel ?= 'parallel' )?` **Pilot-SysML:1745/1837** (6a); `MultiplicityPart` with
`isOrdered ?= 'ordered'` and `isUnique = Nonunique` **Pilot-SysML:370-380** (6b, 6c);
`ExhibitStateUsage returns SysML::ExhibitStateUsage` **Pilot-SysML:1840-1846**, a distinct
metaclass from `StateUsage` **Pilot-SysML:1832** (6d); `Typings : TypedBy ( ',' FeatureTyping )*`
**Pilot-SysML:387** (6e); `ExtendedUsage` **Pilot-SysML:728** (6g); `EndUsagePrefix` with
`OwnedCrossFeatureMember` **Pilot-SysML:567** and `ReferenceUsage` **Pilot-SysML:635** (6h);
`EnumeratedValue … EnumerationUsageKeyword? Usage` **Pilot-SysML:784** (6i);
`AttributeUsage = UsagePrefix AttributeUsageKeyword Usage` **Pilot-SysML:750** (6j);
`FeatureSpecializationPart` **Pilot-KerML:573-576** (6k, 6l).

**6k is the most dangerous of these:** `ref :>> system;` (an anonymous reference *redefining*
`system`) becomes `ref system;` (a reference *named* `system`). That is not a lost modifier, it is an
inverted meaning — exactly the "a declaration label is not a reference" rule in AGENTS.md.

**6f grammar verdict — legal per the Pilot; a gap in the spec rendering. This was my one reversed
verdict in this entry.** Reading the `.kebnf` alone I could find no production admitting `end` before
a usage keyword: `EndUsagePrefix` (SysML:285) feeds only `UnextendedUsagePrefix` (SysML:299) →
`UsagePrefix` (SysML:302), consumed only by `AttributeUsage` (514), `EnumerationUsage` (535),
`BindingConnectorAsUsage` (703), `SuccessionAsUsage` (711) and `ExtendedUsage` (1699); plus
`ReferenceUsage` (SysML:335), which requires an explicit `ref`; plus `DefaultInterfaceEnd`
(SysML:752), which is *interface-def-body only*.

The Pilot settles it: **`OccurrenceUsagePrefix` at Pilot-SysML:836-843 is**
```
fragment OccurrenceUsagePrefix returns SysML::OccurrenceUsage :
	( EndUsagePrefix
	| BasicUsagePrefix ( isIndividual ?= 'individual' )? ( portionKind = PortionKind )? )
	UsageExtensionKeyword*
;
```
— an `EndUsagePrefix` alternative that **SysML:564 does not have**. Since every occurrence usage
(`PortUsage` Pilot-SysML:986, `ItemUsage` Pilot-SysML:917, `PartUsage`, `OccurrenceUsage`,
`ActionUsage`, …) is built on `OccurrenceUsagePrefix`, `end port p : P;`, `end item a : A;` and
`end [1] item a : A { }` are all legal per the reference implementation. See disagreement **D1**.
The corpus was authored against the Pilot, so the Pilot is the operative authority here: these are
**real parser gaps**, not corpus bugs.

### Occurrences (heuristic line counts over the corpus)
`exhibit` 29 lines / 10 files; `connect`-named ends 34/12; `nonunique` 21/5; `end #meta` 21/5;
`ordered` 16/8; `parallel` 10/6; `end <kind>` 11/3; multi-typing `: A, B` 9/9.
Concrete: `src/validation/05-State-based Behavior/5-State-based Behavior-1.sysml:39,62`
(`parallel`), `src/training/26. State Exhibition/State Exhibition Example.sysml:10` (`exhibit`),
`src/validation/15-Properties-Values-Expressions/15_11-Variable Length Collection Types.sysml`
(`value` keyword → `attribute`), `src/examples/Simple Tests/OccurrenceTest.sysml:20` (`: Ind, Occ`),
`src/examples/Simple Tests/ConjugationTest.sysml:34,35,39,40` (`end port`),
`src/examples/Requirements Examples/RequirementDerivationExample.sysml:12-14,20` (`end #meta`, `ref :>>`).

### Fix size
**Small each**, and they are independent: each is one new field (bool / enum / `Vec` element) on an
existing node plus its emitter arm. 6d (`exhibit`) is **large** — `ExhibitStateUsage` is a separate
metaclass, not a flag, so it needs its own node. 6e (`Typings` list) is **small** but touches every
declaration node that currently stores a single typing.

---

## 7. `perform` action usage forms — **D** — 9 occurrences, 7 files

### Reproductions (verified)
```sysml
package P { occurrence o { perform action :>> vehicleMassTest { } } }  // unexpected_keyword_in_scope
package P { action a { perform monitorCriticalActivity; } }            // recovered_action_body_element
package P { part p { perform ActionTree::providePower redefines providePower; } }
package P { part p { perform action takePhoto[*] ordered; } }
```

### Grammar verdict
```
PerformActionUsage =                                  // SysML:944
    OccurrenceUsagePrefix 'perform' PerformActionUsageDeclaration ActionBody
PerformActionUsageDeclaration : PerformActionUsage =  // SysML:948
    ( OwnedReferenceSubsetting FeatureSpecializationPart? | 'action' UsageDeclaration ) ValuePart?
```
All four forms legal: the reference-subsetting alternative covers `perform monitorCriticalActivity;`
and `perform ActionTree::providePower redefines providePower;` (the `redefines` is a
`FeatureSpecializationPart`); the `'action' UsageDeclaration` alternative covers
`perform action :>> vehicleMassTest {…}` and `perform action takePhoto[*] ordered`.
`PerformActionUsage` is a `BehaviorUsageElement` (SysML:386), reachable from every body.

**Pilot: agrees, and is slightly broader.** `PerformActionUsage` **Pilot-SysML:1411**,
`PerformActionUsageDeclaration` **Pilot-SysML:1415-1419** — identical two-alternative shape, except
the Pilot makes the name optional (`ActionUsageKeyword UsageDeclaration?`), so
`perform action { … }` is legal too. Verdict solid.

### Owning scope
`src/parser/part/usage.rs` (`part_usage_body_element` / `PartUsageBodyElement`),
`src/parser/occurrence_body.rs` (`OccurrenceBodyElement`), `src/parser/action.rs`
(`ActionUsageBodyElement`). `Perform`/`PerformBody`/`PerformBodyElement` already exist in
`src/ast` and `emit`, so the node is present — only the declaration grammar is too narrow (it
appears to accept only `perform <name> { … }` and `perform action <name> { … }`).

### Occurrences
9. `src/examples/Interaction Sequencing Examples/ServerSequenceOutsideRealization-2.sysml:61`,
`…/ServerSequenceRealization-2.sysml:55`,
`…/SysML v2 Spec Annex A SimpleVehicleModel.sysml:569` and `:1474`,
`src/training/34. Verification/Verification Case Usage Example.sysml:37,45`,
`src/training/18. Action Performance/Action Performance Example.sysml:10`,
`src/training/19. Terminate Actions/Terminate Actions Example-1.sysml:16`,
`src/examples/Variability Examples/VehicleVariabilityModel.sysml:159`.

### Fix size
**Small.** Widen `PerformActionUsageDeclaration` to the two-alternative shape and add the missing
dispatch arms (occurrence body, part usage body).

---

## 8. `variant` members not dispatched in attribute / action / port bodies — **D** — 9 occurrences, 5 files

### Reproduction (verified)
```sysml
package P { attribute def AD; variation attribute a : AD { variant attribute diameterSmall; } }
package P { variation action a { variant action a1; } }
package P { variation port pp { variant port autoPort1; } }
```
`variation part`/`variation requirement` bodies **do** work — only these three scopes are missing.

### Grammar verdict
`VariantUsageMember : VariantMembership = MemberPrefix 'variant' VariantUsageElement`
(**SysML:250**) is a top-level alternative of `DefinitionBodyItem` (**SysML:239**) and of
`NonBehaviorBodyItem` (**SysML:914**). **Pilot: agrees** — `VariantUsageMember`
**Pilot-SysML:529-532**, listed as alternative 2 of `DefinitionBodyItem` (**Pilot-SysML:516**) and
alternative 4 of `ActionBodyItem` (**Pilot-SysML:1372**).
`VariantUsageElement` (SysML:392) includes `AttributeUsage`,
`PortUsage` and the whole of `BehaviorUsageElement`. Legal in every body.

### Owning scope
`src/parser/attribute.rs` / `AttributeBodyElement`, `src/parser/action.rs` /
`ActionUsageBodyElement`, `src/parser/port.rs` / `PortBodyElement`. A `VariantUsage` /
`VariantTypedUsage` node already exists (`src/emit/opacity.rs:15` imports both).

### Occurrences
9. `src/examples/Variability Examples/VehicleVariabilityModel.sysml:72,73,80`;
`…/SysML v2 Spec Annex A SimpleVehicleModel.sysml:1498,1499`;
`src/training/36. Variability/Variation Definitions.sysml:26,27`;
`src/examples/Simple Tests/VariabilityTest.sysml:24,25`.

### Fix size
**Trivial.** The parser exists; three enum variants + three dispatch arms.

---

## 9. `then <target>` action-target successions — **D** — 7 occurrences, 5 files

### Reproductions (verified)
```sysml
package P { action a { action a1; then accept sig after 10 [s]; } }
package P { action a { action a1; then while i > 0 { action b; } } }
package P { state def S { state wait; then state wait2; } }
package P { occurrence def O { message m1; then message sensedSpeedMessage of SensedSpeed; } }
```

### Grammar verdict
`ActionBodyItem` (**SysML:901**) alternative 3:
`( SourceSuccessionMember )? ActionBehaviorMember ( ActionTargetSuccessionMember )*`, and
`SourceSuccessionMember : FeatureMembership = 'then' SourceSuccession` (**SysML:598**). The `then`
prefix is available *in front of any structure or behavior usage member* in `DefinitionBodyItem`
(SysML:240), `NonBehaviorBodyItem` (SysML:915) and `StateBodyItem` (SysML:1201). So `then <any
usage>` is legal wherever that usage is — including `then message …` and `then state …`. The
parser supports `then timeslice …` and `then <name>;` but not the general form.

**Pilot: agrees.** The Pilot calls the same member `EmptySuccessionMember` and places it identically:
`( ownedRelationship += EmptySuccessionMember )? ownedRelationship += OccurrenceUsageMember`
(**Pilot-SysML:518-519** in `DefinitionBodyItem`), and twice more in `ActionBodyItem`
(**Pilot-SysML:1374, 1378**). Verdict solid.

### Owning scope
`src/parser/action.rs` (`ThenAction`/`ThenTarget` in `src/ast`), `src/parser/state.rs`,
`src/parser/occurrence_body.rs`.

### Occurrences
7. `src/examples/Simple Tests/ActionTest.sysml:18`,
`src/examples/Simple Tests/StructuredControlTest.sysml:24`,
`src/examples/Simple Tests/AssignmentTest.sysml:22`,
`src/training/27. Occurrences/Message Payload Example.sysml:26,29,32`,
`src/training/27. Occurrences/Interaction Example-2.sysml:31`.

### Fix size
**Small.** `ThenTarget` already exists as an enum; the fix is to make `then` a *prefix on the
existing body-element dispatch* rather than a closed list of targets — which is exactly what
`SourceSuccessionMember` is in the grammar.

---

## 10. `subject` with a specialization, multiplicity or value — **D** — 6 occurrences, 6 files

### Reproduction (verified)
```sysml
package P { requirement def R { subject mass :> ISQ::mass; } }
package P { use case uc { subject vehicle : V1 :> vehicle_c1 { } } }
package P { use case uc { subject :>> vehicle :> vehicle_large; } }
package P { use case uc { subject vehicleAlternatives[2] :> vehicle_b; } }
```

### Grammar verdict
`SubjectUsage : ReferenceUsage = 'subject' UsageExtensionKeyword* Usage` (**SysML:1418**), where
`Usage = UsageDeclaration UsageCompletion` (SysML:305) and
`UsageDeclaration = Identification FeatureSpecializationPart?` (SysML:308) with `Identification`
entirely optional (SysML:42). `SubjectMember` is an alternative of `RequirementBodyItem`
(**SysML:1409**) and `CaseBodyItem` (**SysML:1516**). All four legal — the parser evidently accepts
only `subject <name> : T`.

**Pilot: agrees, verbatim.** `SubjectUsage : 'subject' UsageExtensionKeyword* Usage`
**Pilot-SysML:2053**, `SubjectMember` **Pilot-SysML:2049**, `RequirementBodyItem`
**Pilot-SysML:2039-2047**. Verdict solid.

### Owning scope
`src/parser/requirement.rs` / `RequirementDefBodyElement`, `src/parser/usecase.rs` /
`UseCaseDefBodyElement`, `src/parser/case.rs`.

### Occurrences
6. `src/examples/Requirements Examples/VehicleRequirementDerivation.sysml:17`,
`src/examples/Individuals Examples/AnalysisIndividualExample.sysml:80`,
`src/examples/State Space Representation Examples/EVSample.sysml:282`,
`…/SysML v2 Spec Annex A SimpleVehicleModel.sysml:1166`,
`src/training/34. Verification/Verification Case Usage Example.sysml:12`,
`src/examples/Metadata Examples/RationaleMetadataExample.sysml:18`.

### Fix size
**Trivial.** Replace the hand-rolled subject declaration with the shared `usage_declaration` +
`usage_completion` helpers that `part_usage` already uses.

---

## 11. `end`-prefixed connector-end declarations in connection/interface def bodies — **D** — 6 occurrences, 6 files

### Reproductions (verified — all four give `recovered_connection_def_body_element`)
```sysml
package P { connection def C { end : E[1]; } }                 // anonymous
package P { connection def C { end [1] item a : A { } } }      // cross-multiplicity + kind
package P { connection def C { end :>> end1 ::> d1; } }        // redefinition + reference
package P { connection def C { end #cause ::> b.d; } }         // metadata prefix
```
`end a : A;` (plain, named) **is** accepted.

### Grammar verdict — all four legal (two of them only after the Pilot cross-check)
- `end #cause ::> b.d;` — **legal in both**: `ExtendedUsage = UnextendedUsagePrefix
  UsageExtensionKeyword+ Usage` (**SysML:1699** / **Pilot-SysML:728**),
  `UnextendedUsagePrefix ⊇ EndUsagePrefix` (SysML:299 / Pilot-SysML:571).
- `end [1] …` prefix multiplicity — **legal in both**:
  `EndUsagePrefix = 'end' (OwnedCrossFeatureMember)?` (**SysML:285** / **Pilot-SysML:567**),
  `OwnedCrossFeature = BasicUsagePrefix UsageDeclaration` (SysML:293 / Pilot-SysML:587) and
  `FeatureSpecializationPart` may be a bare `MultiplicityPart` (SysML:426 / Pilot-KerML:575).
- `end [1] item a : A { }` — **legal per the Pilot only**: `ItemUsage` (SysML:616 /
  **Pilot-SysML:917**) is built on `OccurrenceUsagePrefix`, and only the Pilot's
  `OccurrenceUsagePrefix` (**Pilot-SysML:836**) carries an `EndUsagePrefix` alternative.
  Disagreement **D1**. *(I had this classified INVALID INPUT before the cross-check.)*
- `end : E[1];` and `end :>> end1 ::> d1;` in a *connection def* body — **legal per the Pilot only**:
  `DefaultReferenceUsage returns SysML::ReferenceUsage : ( isEnd ?= 'end' )? RefPrefix
  UsageDeclaration ValuePart? UsageBody` (**Pilot-SysML:630-632**) carries an optional `end` that
  **SysML:332 does not have**, and `FeatureDeclaration` (**Pilot-KerML:547-554**) has a
  `FeatureSpecializationPart`-only alternative so the name may be omitted entirely.
  Disagreement **D2**. *(Also previously classified INVALID INPUT.)*

### Owning scope
`src/parser/connection.rs` (`ConnectionDefBodyElement`, `EndDecl`/`EndIdentity`/`EndNestedUsage` in
`src/ast`), `src/parser/interface.rs` (`InterfaceDefBodyElement`).

### Occurrences
6. `src/examples/v1 Spec Examples/8.4.1 Wheel Hub Assembly/Wheel Package.sysml:44`,
`…/Wheel Package - Updated.sysml:28`, `src/examples/Simple Tests/ConnectionTest.sysml:46,68`,
`src/examples/Cause and Effect Examples/MedicalDeviceFailure.sysml:17`,
`src/training/11. Interfaces/Interface Decomposition Example.sysml:10`.
Plus one **emit failure**: `src/examples/Association Examples/ProductSelection_N_ary.sysml`
(`EmitError::Unsupported { construct: "EndDecl nested_usage Item" }`) — parses clean but cannot be
emitted.

### Fix size
**Small.** Give `EndDecl` an optional cross-multiplicity and an optional `UsageExtensionKeyword*`,
and let its inner declaration go through the shared `usage_declaration` path.

---

## 12. `assume` / `require` requirement-constraint members — **D** — 4 occurrences, 2 files

### Reproduction (verified)
```sysml
package P { requirement def R { assume constraint c1 : C; } }
package P { requirement def R { assume #goal constraint payloadMassLimit; } }
package P { requirement def R { require constraint c1 :>> c; } }
package P { requirement def R { require #goal vehicleMassRequirement; } }
```

### Grammar verdict
```
RequirementConstraintMember = MemberPrefix? RequirementKind RequirementConstraintUsage  // SysML:1422
RequirementKind = 'assume' | 'require'                                                   // SysML:1426
RequirementConstraintUsage : ConstraintUsage =                                           // SysML:1430
      OwnedReferenceSubsetting FeatureSpecializationPart? RequirementBody
    | ( UsageExtensionKeyword* 'constraint' | UsageExtensionKeyword+ )
      ConstraintUsageDeclaration CalculationBody
```
All four legal. The parser appears to accept only `assume constraint { expr }` and
`require <name>;`.

**Pilot: agrees.** `RequirementConstraintUsage` **Pilot-SysML:2066-2071** is the same two-alternative
rule (the Pilot's first alternative ends in `CalculationBody` where the spec says `RequirementBody` —
immaterial here); `RequirementConstraintMember` **Pilot-SysML:2056ff**. Verdict solid.

### Owning scope
`src/parser/requirement.rs` — `RequireConstraint` / `RequirementDefBodyElement`.

### Occurrences
4. `src/examples/Simple Tests/RequirementTest.sysml:6,16`,
`src/examples/Metadata Examples/RequirementMetadataExample.sysml:30,31`.

### Fix size
**Small.** One production widened; reuses `constraint_usage_declaration` and
`prefix_metadata` which both exist.

---

## 13. `interface` usage with a non-empty body inside a part/interface body — **D/O** — 6 D + 5 O, 9 files

### Reproductions (verified)
```sysml
package P { part p { interface i : I connect a.x to b.y { part q; } } }   // recovered_part_usage_body_element
package P { interface def I { interface wheelFastenerInterface : WFI [5]; } } // unexpected_keyword_in_scope
```
The empty-body form `interface i : I connect a.x to b.y { }` parses clean.

### Grammar verdict
`InterfaceUsage = OccurrenceUsagePrefix 'interface' InterfaceUsageDeclaration InterfaceBody`
(**SysML:757**); `InterfaceBody = ';' | '{' InterfaceBodyItem* '}'` (**SysML:724**);
`InterfaceUsage` is a `StructureUsageElement` (**SysML:372**) so it nests in any body, including
another interface body. Legal.

**Pilot: agrees.** `InterfaceUsage` **Pilot-SysML:1153-1156**, `InterfaceUsageDeclaration`
**Pilot-SysML:1158-1161**, `InterfaceBody`/`InterfaceBodyItem` **Pilot-SysML:1109-1121** — the same
body production is shared by the interface *def* and the interface *usage*, which is exactly the
refactor this entry proposes. Verdict solid.

### Owning scope
`src/parser/interface.rs` — `InterfaceUsage`, `InterfaceUsageBodyElement`,
`InterfaceDefBodyElement`; dispatched from `src/parser/part/usage.rs`.
`InterfaceUsageBodyElement` exists in `src/ast` but is evidently not populated from the usage path.

### Occurrences
D: `src/examples/Arrowhead Framework Example/AHFSequences.sysml:77`,
`src/examples/Flashlight Example/Flashlight Example.sysml:22`,
`src/examples/Vehicle Example/VehicleUsages.sysml:91`,
`…/SysML v2 Spec Annex A SimpleVehicleModel.sysml:333,967,998`.
O (falls to `ExtendedLibraryDecl`): `…/ServerSequenceOutsideRealization-3.sysml:47,95`,
`…/ServerSequenceRealization-3.sysml:44,89`,
`src/training/27. Occurrences/Interaction Realization-2.sysml:10`.

### Fix size
**Small.** Route the interface *usage* body through the same `interface_body_element` list the
interface *def* body already uses.

---

## 14. `ExtendedLibraryDecl` opacity — 13 hits, 5 distinct constructs — **O**

All 13 hits carry `unsupported_grammar_form` ("the spec-valid extended-library declaration production
is retained but not structurally implemented"). Five distinct constructs land there (all verified):

**The Pilot names the exact production for every one of the five** — no `ExtendedLibraryDecl`-shaped
fallback is warranted for any of them, and the two sources agree on all five.

| Construct | Repro | Spec grammar | Pilot grammar | Hits |
|---|---|---|---|---|
| 14a. n-ary `connect` / `allocate` | `package P { connection c : MCE connect ( cause1 ::> c1, cause2 ::> c2 ); }` and `package P { allocation a2 : L2P allocate ( logical ::> l, physical ::> p ); }` | `NaryConnectorPart` **SysML:681**, `AllocationUsageDeclaration` **SysML:795** | `NaryConnectorPart` **Pilot-SysML:1079-1083**, `ConnectorPart` **Pilot-SysML:1070**, `AllocationUsageDeclaration` **Pilot-SysML:1218-1221** | 2 |
| 14b. interface usage with a body | see entry 13 | `InterfaceUsage` **SysML:757** | `InterfaceUsage` **Pilot-SysML:1153** | 5 |
| 14c. anonymous `message` with `from…to` | `package P { message : F from p to p; }` (named `message m : F from p to p;` is fine) | `MessageDeclaration` **SysML:815** — `UsageDeclaration` is optional-name | `MessageDeclaration` **Pilot-SysML:1249-1257** — `UsageDeclaration?` is *explicitly* optional | 1 |
| 14d. `enum` usage with a value | `package P { enum size : SizeChoice = 60.0; }` and `enum color1 = CK::blue;` (typed-only `enum color : ColorKind;` is fine) | `EnumerationUsage = UsagePrefix 'enum' Usage` **SysML:534**; `ValuePart` **SysML:319** | `EnumerationUsage` **Pilot-SysML:788**, `UsageCompletion = ValuePart? UsageBody` **Pilot-SysML:599** | 2 |
| 14e. `variation` of case-family definitions | `package P { variation analysis a1; }`, `variation verification v1;`, `variation use case uc1 { variant use case uc11; } }` (`variation part`/`requirement` are fine) | `BasicDefinitionPrefix : isVariation ?= 'variation'` **SysML:219**, applied to `AnalysisCaseDefinition` **SysML:1529** / `VerificationCaseDefinition` **SysML:1539** / `UseCaseDefinition` **SysML:1560** | `BasicDefinitionPrefix` **Pilot-SysML:490**, `OccurrenceDefinitionPrefix` **Pilot-SysML:804**, `AnalysisCaseDefinition` **Pilot-SysML:2232** / `VerificationCaseDefinition` **Pilot-SysML:2254** / `UseCaseDefinition` **Pilot-SysML:2292** | 3 |

Corpus locations: `src/examples/Cause and Effect Examples/CauseAndEffectExample.sysml:21`,
`src/examples/Simple Tests/AllocationTest.sysml:30`,
`…/ServerSequenceOutsideRealization-3.sysml:47,95`, `…/ServerSequenceRealization-3.sysml:44,89`,
`src/training/27. Occurrences/Interaction Realization-2.sysml:10`,
`src/examples/Simple Tests/ConnectionTest.sysml:59`,
`src/examples/Simple Tests/EnumerationTest.sysml:27,52`,
`src/examples/Simple Tests/VariabilityTest.sysml:28,33,35`.

### Fix size
14a **small** (the binary `ConnectorPart` parser exists; add the `'(' … ')'` alternative).
14c/14d **trivial** (relax an optionality). 14e **small** (`variation` prefix is already handled for
part/requirement — extend `definition_prefix.rs` coverage to the case family). 14b as entry 13.

---

## 15. Structured control actions: `until`, `for`, named `loop`, `while` — **D** — 6 occurrences, 3 files

### Reproductions (verified)
```sysml
package P { action a { loop { action b; } until c; } }            // unexpected_keyword_in_scope `until`
package P { action a { for n : Integer in (1, 2, 3) { action b; } } }
package P { action a { loop action charging { action b; } } }
package P { action a { action a1; then while i > 0 { action b; } } }
```

### Grammar verdict
```
WhileLoopNode : WhileLoopActionUsage =                 // SysML:1162
    ActionNodePrefix ( 'while' ExpressionParameterMember | 'loop' EmptyParameterMember )
    ActionBodyParameterMember ( 'until' ExpressionParameterMember ';' )?
ForLoopNode : ForLoopActionUsage =                     // SysML:1170
    ActionNodePrefix 'for' ForVariableDeclarationMember 'in' NodeParameterMember
    ActionBodyParameterMember
ActionBodyParameter : ActionUsage = ( 'action' UsageDeclaration? )? '{' ActionBodyItem* '}'  // SysML:1136
```
All legal. Note `until` is **part of the loop node**, not a body element — the parser is failing to
consume the optional trailing clause and then meets `until` at statement position.

**Pilot: agrees, verbatim.** `WhileLoopNode` **Pilot-SysML:1615-1622** (including the
`( 'until' ExpressionParameterMember ';' )?` tail), `ForLoopNode` **Pilot-SysML:1624-1629**,
`ActionBodyParameter` **Pilot-SysML:1607-1608** (`( ActionUsageKeyword UsageDeclaration? )? '{'
ActionBodyItem* '}'`). Verdict solid.

### Owning scope
`src/parser/action.rs` — `LoopStmt`, `ActionUsageBodyElement`/`ActionDefBodyElement`.

### Occurrences
6. `src/examples/Simple Tests/StructuredControlTest.sysml:22,24,30,32`,
`src/training/17. Control/Control Structures Example.sysml:14,24`.

### Fix size
**Small.** `LoopStmt` exists; add the optional `until` tail, the `action <name>` body-parameter
prefix, and a `ForLoopNode` variant (the latter is **large** — new node).

---

## 16. Anonymous / redefinition-headed members not dispatched — **D** — 5 occurrences, 5 files

### Reproductions (verified)
```sysml
package P { action a { :>> stateSpace : CartState; } }        // recovered_action_body_element
package P { constraint def C { : Vel[0..*] ordered; } }       // recovered_constraint_body_element
package P { part def A { ref redefines cylinderBR[4]; } }     // recovered_part_def_body_element
package P { port def PD { port lugNutPort :>> lugNutPort [5]; } } // recovered_port_def_body_element
package P { port p { out item redefines fuelSupply; } }       // recovered_port_body_element
```

### Grammar verdict
`Identification` is entirely optional (**SysML:42**) and
`FeatureSpecializationPart = FeatureSpecialization+ MultiplicityPart? FeatureSpecialization*`
(**SysML:425**) — so a specialization may *precede* the multiplicity and a declaration may have no
name at all. All five legal. Note the same `:>> name [n]` shape as the in-flight gap-61 item
`redefines predecessors [0];`, but in SysML-shaped bodies rather than KerML type bodies.

**Pilot: agrees.** `FeatureSpecializationPart` **Pilot-KerML:573-576** has the same
`FeatureSpecialization+ MultiplicityPart? FeatureSpecialization*` shape; `FeatureDeclaration`
**Pilot-KerML:547-554** explicitly offers a `FeatureSpecializationPart`-only (unnamed) alternative.
Verdict solid.

### Owning scope
`src/parser/action.rs`, `src/parser/constraint.rs`, `src/parser/part/def.rs`, `src/parser/port.rs`.
Root cause is shared: the `FeatureSpecializationPart` helper appears to require multiplicity-before-
specialization, and the body dispatchers key on a leading name.

### Occurrences
5. `src/examples/State Space Representation Examples/CartSample.sysml:37`,
`…/EVSample.sysml:167`, `src/examples/v1 Spec Examples/D.4.7.8 Dynamics/HSUVDynamics.sysml:26`,
`src/examples/v1 Spec Examples/8.4.5 Constraining Decomposition/Vehicle Decomposition.sysml:45`,
`…/SysML v2 Spec Annex A SimpleVehicleModel.sysml:605,944`,
`src/training/12. Binding Connectors/Binding Connectors Example-1.sysml:11` and `-2.sysml:11`.

### Fix size
**Small.** One shared `FeatureSpecializationPart` fix (allow `FeatureSpecialization+ MultiplicityPart?`)
plus name-optionality in the affected dispatchers.

---

## 17. Successions in non-action scopes (`first … then …`, conditional succession) — **D** — 4 occurrences, 4 files

### Reproductions (verified)
```sysml
package P { part p { first vehicle.doorClosed then driver.driverReady; } }      // recovered_part_usage_body_element
package P { occurrence def O { first setSpeedMessage then sensedSpeedMessage; } } // unexpected_keyword_in_scope
package P { action def A { action A1; action A2; succession S first A1 if x == 0 then A2; } } // missing_semicolon
package P { action def A { action focus; action shoot; first focus if focus.image.isWellFocused then shoot; } }
```

### Grammar verdict
`SuccessionAsUsage = UsagePrefix ('succession' UsageDeclaration)? 'first' ConnectorEndMember 'then' ConnectorEndMember UsageBody`
(**SysML:710**) is a `NonOccurrenceUsageElement` (**SysML:350**) → legal in any body.
`GuardedSuccession = ('succession' UsageDeclaration)? 'first' FeatureChainMember GuardExpressionMember 'then' TransitionSuccessionMember UsageBody`
(**SysML:1183**) is reached via `GuardedSuccessionMember`, an alternative of `ActionBodyItem`
(**SysML:908**). Both legal.

**Pilot: agrees.** `SuccessionAsUsage` **Pilot-SysML:1035ff** (listed in `NonOccurrenceUsageElement`
**Pilot-SysML:650**); `GuardedSuccessionMember` is the last alternative of `ActionBodyItem`
(**Pilot-SysML:1381**). Verdict solid.

### Owning scope
`src/parser/part/usage.rs`, `src/parser/occurrence_body.rs`, `src/parser/action.rs`
(`FirstStmt` / `FirstMergeBody` / `ThenAction` already exist).

### Occurrences
4. `…/SysML v2 Spec Annex A SimpleVehicleModel.sysml:780`,
`src/training/27. Occurrences/Interaction Example-1.sysml:21`,
`src/examples/Simple Tests/DecisionTest.sysml:17`,
`src/training/16. Conditional Succession/Conditional Succession Example-1.sysml:21`.

### Fix size
**Trivial** for the plain `first…then` (existing `FirstStmt`, missing dispatch arms);
**small** for `GuardedSuccession` (new `if <guard> then <target>` tail on the existing parser).

---

## 18. `.?{ … }` select expression, `->forAll { … }`, and calc result expressions — **D/O** — 5 occurrences, 5 files

### Reproductions (verified)
```sysml
package P { attribute def A { attribute y = sum(sub.totalMass.?{in q :> ISQ::mass; q > minMass}); } }
  // unsupported_grammar_form + OpacityKind::UnsupportedGrammar
package P { part def A { attribute n; assert constraint { (1..n)->forAll { in i : Natural; private attribute lbcf = x#(i).cf; } } } }
  // recovered_constraint_body_element
package P { calc def C { attribute a; {a == 1} } }
  // recovered_calc_body_element
```

### Grammar verdict
Body-bearing operator arguments come from `BodyExpression`/`FunctionOperationExpression` in
**KerML** (the `->`/`.?` family around KerML:939-1100); `ResultExpressionMember : ResultExpressionMembership = MemberPrefix? OwnedExpression`
is the trailing alternative of `CalculationBodyPart` (**SysML:1362**) and `CaseBody` (**SysML:1509**).
All legal.

**Pilot: agrees, and pins the shape precisely.** `PrimaryExpression` (**Pilot-Expr:299-323**) lists
`{SysML::SelectExpression.operand += current} '.?' operand += BodyExpression` (**Pilot-Expr:316-317**),
`{SysML::CollectExpression.operand += current} '.' operand += BodyExpression` (**Pilot-Expr:314-315**)
and `{SysML::InvocationExpression…} '->' InstantiatedTypeMember ( operand += BodyExpression | … )`
(**Pilot-Expr:309-313**). So the `{ … }` argument is a first-class `BodyExpression` operand of the
postfix operator — three distinct metaclasses (`SelectExpression`, `CollectExpression`,
`InvocationExpression`) share it. `CalculationBodyPart`/`ResultExpressionMember`
**Pilot-SysML:1951-1969**. Verdict solid, and the design decision has a named target.

### Owning scope
`src/parser/expr.rs` (body-expression argument), `src/parser/constraint.rs`,
`src/parser/action.rs` (calc body). This is the only place `OpacityKind::UnsupportedGrammar` appears.

### Occurrences
5. `src/examples/Mass Roll-up Example/MassRollup.sysml:23`,
`src/training/29. Expressions/MassRollup2.sysml:17`,
`src/examples/Analysis Examples/Vehicle Analysis Demo.sysml:206`,
`src/examples/Geometry Examples/VehicleGeometryAndCoordinateFrames.sysml:54`,
`…/SysML v2 Spec Annex A SimpleVehicleModel.sysml:1084`.

### Fix size
**Large.** A body-bearing expression argument is a new expression node with its own scope; the calc
result expression is a separate (small) trailing-member change.

---

## 19. `rendering` and `alias` in a view body — **D** — 4 occurrences, 2 files

### Reproduction (verified)
```sysml
package P { view v { render r; rendering r2; } }      // unexpected_keyword_in_scope `rendering`
package P { view v { alias vp1 for p1; } }            // unexpected_keyword_in_scope `alias`
```
These two make the *whole* view declaration fail: `view v : V[0..*] { expose Q::*; render r;
rendering r2; alias vp1 for p1; }` reports `missing_body_or_semicolon` on the view header.

### Grammar verdict
`ViewBodyItem = DefinitionBodyItem | ElementFilterMember | ViewRenderingMember | Expose`
(**SysML:1614**). `DefinitionBodyItem` supplies `AliasMember` (**SysML:242**) and, via
`StructureUsageElement` (**SysML:363**), `RenderingUsage` (SysML:1646). Legal.

**Pilot: agrees.** `ViewBodyItem` **Pilot-SysML:2359-2364** and `ViewDefinitionBodyItem`
**Pilot-SysML:2329-2333** both start with `DefinitionBodyItem`, which supplies `AliasMember`
(Pilot-SysML:520) and `RenderingUsage` via `StructureUsageElement` (Pilot-SysML:664). Verdict solid.

### Owning scope
`src/parser/view.rs` — `ViewBodyElement` / `ViewDefBodyElement`; `RenderingUsageBodyElement` and
`AliasDef` already exist.

### Occurrences
`src/examples/Simple Tests/ViewTest.sysml:34` (2 diagnostics collapsed into a header failure),
`src/training/42. Views/Views Example.sysml:24`.

### Fix size
**Trivial.** Two dispatch arms.

---

## 20. `bind` / `metadata` / `part` / `package` / `import` / `constraint` in scopes that omit them — **D** — 9 occurrences, 8 files

Grouped because the fix is identical in each: add the missing arm. All verified.

| Input | Diagnostic | Grammar verdict | Where |
|---|---|---|---|
| `package P { occurrence def O { bind apsp.send = forw1.mq; } }` | `unexpected_keyword_in_scope` `bind` | `BindingConnectorAsUsage` **SysML:700** ∈ `NonOccurrenceUsageElement` **SysML:349** | `occurrence.rs` ×2 (`AHFSequences.sysml:47,53`) |
| `package P { attribute a { metadata ExternalShapeRef { } } }` | `unexpected_keyword_in_scope` `metadata` | `MetadataUsage` **SysML:1666** ∈ `AnnotatingElement` **SysML:73** ∈ `DefinitionElement` **SysML:183** | `attribute.rs` ×1 (`ExternalShapeRefExample.sysml:19`) |
| `package P { state s { part counter : Counter; } }` | `unexpected_keyword_in_scope` `part` | `StateBodyItem → NonBehaviorBodyItem → StructureUsageMember` **SysML:1201/916** | `state.rs` ×1 (`AssignmentTest.sysml:19`) |
| `package P { part def A { package Q { } } }` | `unexpected_keyword_in_scope` `package` | `DefinitionBodyItem → DefinitionMember → DefinitionElement → Package` **SysML:239/247/181** | `part/def.rs` ×1 (`PartTest.sysml:23`) |
| `package P { action a { private import AnalysisTooling::*; } }` | `unexpected_keyword_in_scope` `private` | `NonBehaviorBodyItem → Import` **SysML:911** | `action.rs` ×1 (`Metadata Example-2.sysml:4`) |
| `package P { state s { constraint {electricalPower <= 500} } }` and the interface-def variant | `unexpected_keyword_in_scope` `constraint` | `StateBodyItem → BehaviorUsageMember` **SysML:1203**; `InterfaceBodyItem → InterfaceOccurrenceUsageElement → BehaviorUsageElement` **SysML:749** | `state.rs`, `interface.rs` ×3 (`AnnexA:51,328`, `Time Constraints.sysml:35`) |
| `package P { part p { accept S via x; } }` / `action a { accept S via x; }` | `recovered_*_body_element` | `AcceptNode` ∈ `ActionNode` **SysML:958**; `AcceptParameterPart` `'via'` clause | `action.rs` ×2 (`PartTest.sysml:28`, `Interaction Realization-1.sysml:13`) |

**Pilot: agrees on every row.** `BindingConnectorAsUsage` **Pilot-SysML:1018ff** ∈
`NonOccurrenceUsageElement` **Pilot-SysML:649**; `MetadataUsage` **Pilot-SysML:140-147** ∈
`AnnotatingElement` ∈ `DefinitionElement`; `StateBodyItem`→`NonBehaviorBodyItem`→
`StructureUsageMember`/`BehaviorUsageMember` **Pilot-SysML:1748ff**; `DefinitionMember`
**Pilot-SysML:524** admits `Package`; `Import` is alternative 1 of `ActionBodyItem`
**Pilot-SysML:1369**; `AcceptNode` ∈ `ActionNode` **Pilot-SysML:1450ff**. Verdicts solid.

### Fix size
**Trivial** for every row (existing parsers, missing dispatch), except the `metadata` row which also
needs the emitter arm (see entry 22).

---

## 21. `flow` / `message` usages with `of <payload>` or a redefinition head — **D** — 5 occurrences, 4 files

### Reproductions (verified)
```sysml
package P { part p { flow : FuelFlow of Fuel from a.b to c.d; } }        // recovered_part_usage_body_element
package P { part p { flow :>> publish_message from a.b to c.d; } }
package P { occurrence o { message :>> setSpeedMessage = i.m; } }        // recovered_occurrence_body_element
```

### Grammar verdict
```
FlowDeclaration : FlowUsage =                            // SysML:834
      UsageDeclaration ValuePart? ( 'of' FlowPayloadFeatureMember )?
      ( 'from' FlowEndMember 'to' FlowEndMember )?
    | FlowEndMember 'to' FlowEndMember
MessageDeclaration : FlowUsage = …same shape…            // SysML:815
```
`UsageDeclaration = Identification FeatureSpecializationPart?` — so both the anonymous form and the
`:>>`-headed form are legal, and `of <payload>` composes with `from…to`.

**Pilot: agrees, and makes the optionality explicit.** `FlowDeclaration`
**Pilot-SysML:1283-1290** and `MessageDeclaration` **Pilot-SysML:1249-1257** both begin
`UsageDeclaration? ValuePart? ( 'of' PayloadFeatureMember )? ( 'from' … 'to' … )?` — the `?` on
`UsageDeclaration` is written out rather than inferred from an all-optional `Identification`.
Verdict solid.

### Owning scope
`src/parser/flow.rs` (`FlowUsage`, `FlowUsageKind`, `PayloadFeature` all exist),
`src/parser/payload.rs`, dispatched from `src/parser/part/usage.rs` and
`src/parser/occurrence_body.rs`.

### Occurrences
5. `src/training/13. Flows/Flow Definition Example.sysml:16`,
`…/ServerSequenceOutsideRealization-3.sysml:141`, `…/ServerSequenceRealization-3.sysml:133`,
`src/training/27. Occurrences/Interaction Realization-2.sysml:78`,
`src/examples/Simple Tests/ConnectionTest.sysml:59` (as `ExtendedLibraryDecl`, entry 14c).

**Adjacent to the in-flight gap 61** (`flow a.y to b.x1;`, `message m of T;` shredded in KerML-shaped
type bodies). These occurrences are in *SysML-shaped* bodies and fail loudly rather than silently —
noted, not proposed.

### Fix size
**Small.** `PayloadFeature` exists; widen `FlowDeclaration` to the two-alternative shape.

---

## 22. Emitter gaps on nodes that parse clean — **S (emit failure)** — 3 files

`emit_sysml` returns `EmitError::Unsupported` for three constructs that parse with zero diagnostics
and zero opacity:

| Construct | Repro | File |
|---|---|---|
| `MetadataUsage` inside an action def body | `package P { action def A { metadata ToolExecution { toolName = "x"; } } }` | `src/examples/Analysis Examples/AnalysisAnnotation.sysml:7` |
| `EndDecl nested_usage Item` | `package P { connection def C { end item a : A { } } }` (n-ary form) | `src/examples/Association Examples/ProductSelection_N_ary.sysml:13` |
| `TerminateStmt` | `package P { action a { terminate; } }` | `src/training/19. Terminate Actions/Terminate Actions Example-2.sysml:15` |

Grammar: `MetadataUsage` **SysML:1666** / **Pilot-SysML:140-147**; `TerminateNode` ∈ `ActionNode`
**SysML:964** / **Pilot-SysML:1641-1647**; `EndDecl` per entry 11. Both sources agree all three are
legal, so all three are emitter bugs on correctly-parsed input.

**Fix size: trivial each** — one missing emitter arm per construct. Per AGENTS.md these should have
been compile errors; they are runtime `Unsupported` returns instead, which suggests the emitter has a
catch-all somewhere that should be removed.

---

## 23. Authored spelling of names is not preserved on emit — **S (low severity)** — ~20 files

### Reproductions (verified)
```sysml
package P { concern 'modularity' { attribute x; } }   // → concern modularity { … }
package P { part def A { attribute doc; } }           // → attribute 'doc';
```

Unquoting a quoted name and quoting a keyword-shaped name both preserve *name identity* (SysML
restricted names denote the same name as their unquoted spelling), so this is not a semantic change —
but it violates the AGENTS.md rule that "the original document is authoritative for authored spelling
and provenance". The keyword-quoting direction (`doc` → `'doc'`) is *required* for re-parseability and
should stay; the unquoting direction should not happen.

`ImportTarget`, `DeclarationName` and `QualifiedReferenceView` all carry spans, so the information is
present — the emitter just renders from a normalized string.

**Fix size: small.** Render from the authored span, not the decoded name.

---

## 24. Comment/trivia retention on emit — noted, not a defect

`emit_sysml` drops `//` notes (`SL_NOTE`, KerML:33) and `//*…*/` prefix comments (`PREFIX_COMMENT`,
KerML:36). **I verified this is correct**: neither token is referenced by any parser production —
`Comment`/`Documentation`/`TextualRepresentation` all require `REGULAR_COMMENT` (`/*…*/`,
KerML:39/SysML:88,93,99), and bare `/*…*/` comments *are* retained as `AnnotatingMember`.
Several of my initial "lost content" hits (e.g. all of `4a-Functional Allocation.sysml`'s missing
`ref action 'provide power' …` and `13a-Model Containment.sysml`'s missing
`public import VehicleSubsystems::*;`) turned out to be inside `//*…*/` blocks and are correctly
discarded. Recorded here so nobody re-chases them. The remaining AGENTS.md concern — a
*source-fidelity* formatter that retains trivia — is a separate, already-known workstream.

---

# Proposed burn-down order

Ordered by (occurrences × severity), with cheap-and-silent first because silent corruption is the
only class the current test suite cannot see.

1. **Entry 1** — package-level `attribute`/`connection` def-vs-usage. Trivial fix, 19 files, silent
   metaclass corruption. Delete the `def`-optional flag while you are there.
2. **Entry 6k + 6f + 6g** — `ref :>> system;` inverting redefinition into a declared name, and the
   `end <kind>` / `end #meta` prefix drops. Small, silent, meaning-changing.
3. **Entry 3** — connector-end names and cross-multiplicity. Small, silent, and the lost names are
   load-bearing for resolution. The `allocate` path already shows how.
4. **Entry 2** — `BracketExpression`. Silent, and the `[N*m]`→multiplicity case corrupts cardinality.
5. **Entry 6a-6c, 6e, 6i-6j, 6l** — the remaining dropped modifiers (`parallel`, `nonunique`,
   `ordered`, typing lists, `enum`, `attribute` in port params, clause order). Independent small fixes;
   batch them.
6. **Entry 22** — three trivial emitter arms; also audit for and remove the emitter catch-all that
   let them become runtime errors.
7. **Entries 4, 8, 19, 20, 17(first half), 10** — pure dispatch gaps: `event` in port bodies,
   `variant` in attribute/action/port, `rendering`/`alias` in views, `bind`/`part`/`package`/`import`/
   `constraint`/`accept`, plain `first…then`, `subject`. All trivial, ~50 diagnostics, and they clear
   most of the 32 cascade suppressions with them.
8. **Entry 5** — `DefaultReferenceUsage` in package/attribute bodies. Small, 15 diagnostics.
9. **Entries 7, 12, 13, 16, 21, 11, 14a/14c/14d/14e** — small production widenings
   (`perform`, `assume`/`require`, interface usage bodies, anonymous/redefinition heads, `flow`/
   `message` payloads, `end` prefixes, n-ary connect, anonymous message, valued `enum`, `variation`
   of cases).
10. **Entry 9, 15** — `then <target>` generalisation and the loop family (`until` tail, named loop
    body parameter).
11. **Entry 23** — authored spelling preservation.
12. **Entry D3** — prefix metadata on an enumerated value. Trivial, 1 occurrence; do it whenever the
    enumeration body is next open.
13. **Entries 18, 15(ForLoopNode), 6d** — the genuinely large ones, last.

Steps 1-6 are all silent-failure classes and none is larger than "small". Steps 7-8 clear roughly
80 of the 151 primary diagnostics and most of the 32 cascade suppressions with them.

---

# Classified INVALID INPUT

**None.** Every construct in this inventory has an admitting production in at least one of the two
authoritative sources. My three provisional INVALID INPUT classifications (made against the `.kebnf`
alone) were all **reversed** by the Pilot cross-check and are reported as spec-vs-Pilot disagreements
instead. In every one of the three, the Pilot is the more permissive source — which is the expected
direction, since the example corpus was authored and regression-tested against the Pilot.

---

# Spec-vs-Pilot disagreements

Three places where `sysml-v2-release/bnf/*.kebnf` (the normative spec rendering, and the parser's
conformance pin) and the OMG Pilot Implementation's Xtext grammar (what the reference parser actually
accepts, and what the corpus was authored against) do not agree. In all three the Pilot admits syntax
the spec rendering does not. **I am not silently picking a side**; each is stated with both readings.

### D1. `OccurrenceUsagePrefix` — the Pilot allows an `end` prefix, the spec rendering does not

| Source | Text |
|---|---|
| **SysML:564-568** | `OccurrenceUsagePrefix : OccurrenceUsage = BasicUsagePrefix ( isIndividual ?= 'individual' )? ( portionKind = PortionKind { isPortion = true } )? UsageExtensionKeyword*` |
| **Pilot-SysML:836-843** | `fragment OccurrenceUsagePrefix returns SysML::OccurrenceUsage : ( EndUsagePrefix \| BasicUsagePrefix ( isIndividual ?= 'individual' )? ( portionKind = PortionKind )? ) UsageExtensionKeyword*` |

**Consequence.** Every occurrence usage — `PortUsage` (SysML:646 / Pilot-SysML:986), `ItemUsage`
(SysML:616 / Pilot-SysML:917), `PartUsage`, `OccurrenceUsage`, `ActionUsage`, `ConnectionUsage`,
`InterfaceUsage`, … — inherits this prefix. So `end port p3 : P;`, `end item a : A;`,
`end [1] item a : A { }` and `end occurrence o;` are **legal per the Pilot and not per the spec
rendering**.

**Corpus evidence:** 11 lines across 3 files use `end <kind-keyword>`, e.g.
`src/examples/Simple Tests/ConjugationTest.sysml:34,35,39,40`,
`src/examples/Simple Tests/ConnectionTest.sysml:68`,
`src/training/11. Interfaces/Interface Decomposition Example.sysml:10`,
`src/examples/Association Examples/ProductSelection_N_ary.sysml:13`.

**Recommendation:** follow the Pilot. The parser already accepts these forms (it just shreds the
keyword, entry 6f) or rejects only the multiplicity-prefixed variant (entry 11). Retain acceptance,
fix the shred, and note the discrepancy against the conformance pin so the deviation is deliberate
and recorded rather than accidental.

### D2. `DefaultReferenceUsage` — the Pilot allows an `end` prefix, the spec rendering does not

| Source | Text |
|---|---|
| **SysML:332-333** | `DefaultReferenceUsage : ReferenceUsage = RefPrefix Usage` |
| **Pilot-SysML:630-632** | `DefaultReferenceUsage returns SysML::ReferenceUsage : ( isEnd ?= 'end' )? RefPrefix UsageDeclaration ValuePart? UsageBody` |

**Consequence.** Keyword-less `end` declarations — `end : E[1];`, `end p3 : P ::> p.p1;`,
`end :>> end1 ::> d1;` — are legal in **any** body per the Pilot. Per the spec rendering they are
legal only inside an *interface def* body, via the separate `DefaultInterfaceEnd : PortUsage =
isEnd ?= 'end' Usage` (**SysML:752** / **Pilot-SysML:1143-1144**, which both sources do have).

**Corpus evidence:** ubiquitous — `end <name> : T;` inside `connection def` bodies appears throughout,
e.g. `src/examples/v1 Spec Examples/8.4.1 Wheel Hub Assembly/Wheel Package.sysml:44`,
`src/examples/Simple Tests/ConnectionTest.sysml:46`,
`src/validation/08-Requirements/8-Requirements.sysml`. The parser already accepts the named form and
rejects only the anonymous and specialization-headed ones (entry 11).

**Recommendation:** follow the Pilot, same rationale as D1. Note that D1 and D2 together are almost
certainly one editorial omission in the spec rendering rather than two independent design choices.

### D3. `EnumeratedValue` — the Pilot allows prefix metadata, the spec rendering does not

| Source | Text |
|---|---|
| **SysML:531-532** | `EnumeratedValue : EnumerationUsage = 'enum'? Usage` |
| **Pilot-SysML:784-785** | `EnumeratedValue returns SysML::EnumerationUsage : UsageExtensionKeyword* EnumerationUsageKeyword? Usage` |

**Consequence.** `#Security enum secret : ClassificationLevel = 2;` inside an `enum def` body is
**legal per the Pilot** (`UsageExtensionKeyword* = PrefixMetadataMember*`) and **not** per the spec
rendering, whose `EnumerationUsageMember = MemberPrefix EnumeratedValue` (SysML:528) offers only a
visibility `MemberPrefix` (SysML:130) and no metadata prefix. Both sources agree that `EnumerationBody`
(SysML:522 / Pilot-SysML:772-776) admits only `AnnotatingMember | EnumerationUsageMember`, so the
`#`-prefixed `EnumerationUsage` (SysML:534 / Pilot-SysML:788) is *not* the route in.

**Corpus evidence:** `src/examples/Simple Tests/MetadataTest.sysml:9` (1 occurrence). Currently
reported as `unsupported_annotation_syntax`, which is a fair label for a genuine gap — **but the
diagnostic message should not be read as "the input is wrong"**: per the Pilot it is a parser gap.

**Recommendation:** follow the Pilot. **Fix size: trivial** — `prefix_metadata` already parses, and
`EnumeratedValue` already exists; this is one optional prefix on one production. Also note the same
line's companion form `uncl : ClassificationLevel = 0;` (entry 5 / entry 33 class) is legal in *both*
sources, so both diagnostics on that file are parser gaps.

# Needs a design decision, not just an implementation

1. **Entry 2 — `BracketExpression` vs `MultiplicityPart` disambiguation.** `[…]` is both a
   multiplicity (SysML:491 / Pilot-SysML:370) and a postfix expression operator (KerML:1099 /
   **Pilot-Expr:307**). The Pilot resolves it structurally: the bracket operator lives in
   `PrimaryExpression`'s postfix loop and produces an `OperatorExpression`, while `MultiplicityPart`
   lives in `FeatureSpecializationPart`; the two never compete at the same position. Adopting that
   split needs an explicit invariant at the expression/declaration boundary — a decision, but a
   well-specified one.
2. **Entry 6d — `exhibit` → `ExhibitStateUsage`.** Not a flag on `StateUsage`; a distinct metaclass
   with its own declaration alternatives. Both sources agree
   (SysML:1268 / **Pilot-SysML:1840**, `returns SysML::ExhibitStateUsage` vs
   `returns SysML::StateUsage` at Pilot-SysML:1832). Needs a new node and decisions about how
   `PartUsageBodyElement`/`StateDefBodyElement` name it.
3. **Entry 18 — body-bearing expression arguments** (`.?{…}`, `->forAll {…}`, `.{…}`). A nested
   declaration scope inside an expression. The Pilot names three separate metaclasses that share the
   `BodyExpression` operand — `SelectExpression`, `CollectExpression`, `InvocationExpression`
   (**Pilot-Expr:309-317**) — so the design question is whether the parser models one
   body-expression node with an operator discriminator or three. This is the one place
   `OpacityKind::UnsupportedGrammar` fires.
4. **Entry 15 — `ForLoopNode`.** New node (`ForVariableDeclarationMember` + `NodeParameterMember` +
   body parameter, SysML:1170 / **Pilot-SysML:1624**), not reusable from `LoopStmt`.
5. **D1/D2 — conformance-pin policy for the `end` prefix.** The parser pins the `.kebnf` content
   hash as its authority (`SUPPORTED_GRAMMAR`, `src/lib.rs`), but the corpus it is measured against
   was authored to the Pilot, and the Pilot is strictly more permissive here. Someone has to decide
   whether "conformance" means the spec rendering or the reference implementation, and record the
   deviation either way. This is a policy decision, not an implementation one, and it gates entries
   6f and 11.
6. **The scope-narrowing architecture itself.** 66 of 151 primary diagnostics exist because each body
   scope owns a hand-written subset of `DefinitionBodyItem` (SysML:237 / **Pilot-SysML:514**).
   AGENTS.md forbids a universal body node, so the answer is not one enum — but the *dispatch table*
   for "`DefinitionMember | VariantUsageMember | NonOccurrenceUsageMember | OccurrenceUsageMember |
   AliasMember | Import`" is genuinely shared by every scope except `InterfaceBodyItem`,
   `EnumerationBody` and `MetadataBody` — **and both grammars structure it exactly that way**, the
   Pilot literally by inlining the `DefinitionBodyItem` fragment into `RequirementBodyItem`,
   `ViewBodyItem` and `ViewDefinitionBodyItem`. Deriving that dispatch from the pinned grammar (as
   AGENTS.md's "derive scope-level FIRST sets from the pinned grammar" already requires) would close
   most of this report in one coherent slice. That is a decision, not a patch.

---

## Method notes / reproducibility

Three throwaway `examples/*.rs` scanners were used and deleted:
- a corpus scanner over `parse_for_editor` + `opacity_report` (reproduced the stated baseline exactly);
- a token-multiset round-trip differ over `emit_sysml` restricted to diagnostic-free, opacity-free
  files (found the silent-shred class);
- a snippet harness that runs each minimal reproduction and prints diagnostics, opacity and the
  emitted round-trip. **Every snippet in this report was executed through that harness**; none is
  pasted untested.

Grammar verdicts were then cross-checked line by line against the OMG Pilot Implementation's Xtext
grammar at `/Users/luke/Documents/GitHub/SysML-v2-Pilot-Implementation` (source trees only; nothing
under `target/`). 24 of 27 verdicts were confirmed unchanged by both sources; 3 were reversed and are
reported as disagreements D1-D3 above.

No repository source file was modified. The three scanner examples were deleted after use.
