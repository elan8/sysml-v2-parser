# Corpus coverage burn-down

The goal this plan serves: **every construct in the pinned release corpus that either grammar
admits parses to a typed node, with no diagnostic, no opaque capture, and no silent loss.**

Three inventories sit beside this file. Read this one first — it records what each of them measured,
which of their entries are already closed, and what has to be decided before the largest items can
be worked at all.

| Inventory | Corpus | Measured at | State |
|---|---|---|---|
| [`corpus-coverage-sysml.md`](corpus-coverage-sysml.md) | `sysml/`, 251 files | `6d54b85` | **Current.** States its commit and its post-branch numbers. |
| [`corpus-coverage-kerml.md`](corpus-coverage-kerml.md) | `kerml/`, 58 files | `b6291cc` | Stale in parts; three entries verified closed, header lists them. |
| [`corpus-coverage-silent-loss.md`](corpus-coverage-silent-loss.md) | all 403 files | `b6291cc` | Stale in parts; one finding lost when the run terminated. |

## Diagnostics are not the coverage metric

`sysml.library` scores 94 files, 0 diagnostics, 0 opacity hits, and the L1+L2 conformance scorecard
passes on it. That is necessary and nowhere near sufficient.

The failure mode it cannot see is the parser accepting input and quietly turning it into something
else. Before `6d54b85`, `classifier C { flow a.y to b.x1; }` parsed with **zero diagnostics and zero
opacity hits** into four unrelated bare expression members, each keyword arriving as an ordinary
`Expression::FeatureRef` indistinguishable from a user feature of that name. Nothing in the metric
moved.

So coverage needs three conditions, not one:

1. zero diagnostics;
2. zero opacity hits; and
3. emit → reparse yields an identical span-insensitive semantic projection.

**Condition 3 is itself insufficient**, which is the most important methodological result in these
inventories. Of 16 files where an authored keyword was proven to have become an ordinary name, **8
round-trip perfectly** — a loss that is *stable* under re-parse is invisible to a round trip. Two
extra detectors were needed to see them at all:

- **KWNAME** — the emitter writes `'kw'` where the source never quoted it, proving the parser took a
  keyword as a name.
- **COMMENTLOSS** — block-comment count in source versus emitted output.

Any future "we are at 100%" claim has to be made against all five checks, not against the scorecard.

## Measured state at `3941261`

| Corpus | Files | With diagnostics | Opacity |
|---|---|---|---|
| `sysml.library` | 94 | 0 | none |
| `sysml/` | 251 | 67 | `ExtendedLibraryDecl: 13`, `ParseError: 216` |
| `kerml/` | 58 | 29 | `ClassifierDecl: 1`, `FeatureDecl: 8`, `ParseError: 102` |

Separately, and invisible to the table above: **15 authored block comments across 12 clean-parsing
files are still dropped**, `sysml.library`'s own `Occurrences.kerml`, `Requirements.sysml` and
`TradeStudies.sysml` among them.

The SysML inventory adds a second measurement the table also cannot show: of the 184 files that are
*both* diagnostic-free and opacity-free, **121 round-trip to different content**. After subtracting
verified-correct canonicalisation (`redefines`→`:>>`, `and`→`&&`, `()`→`null`, and `//` / `//*…*/`
trivia), the residue is real. **Diagnostics understate the gap by roughly 2×.**

## Nothing in the corpus is invalid input

Across 44 distinct constructs in the two corpus inventories, **zero were classified INVALID INPUT**.
Every construct was checked against both the pinned `.kebnf` and the OMG Pilot Implementation's
Xtext grammar, and both admit all of them. Every failing file is a parser gap, not an authoring
error, so 100% is a reachable target rather than an aspiration.

## Blocking decision: which grammar is authoritative

The conformance pin is the `.kebnf`. The example corpus was authored against the Pilot. They
disagree, and three SysML constructs hang on the answer:

| | `.kebnf` | Pilot | Consequence |
|---|---|---|---|
| `end port p : P;` | `OccurrenceUsagePrefix` (SysML:564) has no `EndUsagePrefix` alternative | Pilot-SysML:836 does | Parser gap if the Pilot governs; corpus bug if the `.kebnf` does |
| `end : E[1];` | `DefaultReferenceUsage` (SysML:332) has no optional `end` | Pilot-SysML:630 does | as above |
| `#Security enum secret : …` | `EnumeratedValue` admits no `UsageExtensionKeyword*` | Pilot-SysML:784 does | as above |

All three were provisionally called INVALID INPUT against the `.kebnf` alone, and **the Pilot
reversed all three**. Until this is decided, they stay unworked; the rest of the burn-down does not
depend on it.

Two lesser disagreements are recorded and change nothing: KerML `TypeDeclaration` requires
`(Specialization|Conjugation)+` in the spec but exactly one in the Pilot, and control-node
`UsageDeclaration` is optional in the Pilot but not the spec (both derive empty).

## Burn-down order

Silent-loss classes first — they corrupt models without telling anyone, and every one of them is
invisible to the conformance scorecard.

**Wave 1 — silent corruption, small fixes.**
- Package-level `attribute a : T;` parses as a *definition*, not a usage — `attribute_def(i, false)`
  at `src/parser/package.rs:1399` makes `def` optional, but `AttributeDefKeyword` includes `'def'`
  in *both* grammars, so the flag has no basis. 19 files. `part p : T;` correctly stays a usage, so
  the parser contradicts itself.
- `ref :>> system;` → `ref system;` — a redefinition silently inverted into a declared name.
- `attribute a = 10.0 [N * m];` → `attribute a[N * m] = 10.0;` — **the value becomes a
  multiplicity**. Same site also collapses `60 [SI::mm]` to a quoted name `['SI::mm']`.
- The remaining 15 dropped comments in 12 files, in whichever scopes still swallow them.
- `private inv { … }` in a KerML type body loses its visibility.

**Wave 2 — pure dispatch gaps, an existing parser simply is not wired into a scope.** Clears roughly
80 diagnostics and most cascades. `event` in port bodies (16 diagnostics), `variant`,
`rendering`/`alias`, `bind`/`part`/`package`/`import`, `subject`, keyword-less usages. Connector
ends (`::>` spelling drops end names; `references` spelling rejected although the `allocate` path
already handles it) belongs here too.

**Wave 3 — `featured by`.** 72 occurrences across 9 files, the single largest item in either corpus.
`KermlFeature` has `chains`, `inverse_of` and `type_relationships` but no `featured_by` slot.

**Wave 4 — the shared `FeatureDeclaration` component.** The grammar defines one `FeatureDeclaration`
reused by Feature/Step/Connector/Binding/Succession/Flow; the parser hand-rolls a weaker fixed-order
subset in each of `kerml_feature`, `kerml_connector_member`, `kerml_binding_member` and
`kerml_succession_member`. **This is the meta-cause behind 8 of the 20 KerML entries** and closes
succession, connector, binding, the `:>>` member form and repeated `chains` at once, killing the
pervasive silent specialization reordering. Conjugation rides along. Needs a design decision, not
just an implementation.

**Wave 5 — production widenings and the genuinely large items:** body-expressions, `ForLoopNode`,
`ExhibitStateUsage`, and the `.kebnf`-vs-Pilot trio above once the authority question is settled.

## The mechanism worth fixing before any individual spelling

`calc_def_body_element` ends in a bare `expression` fallback that reads **any** unclaimed reserved
keyword as an ordinary `FeatureRef`. That single mechanism produced gap 61, and it will keep
producing gap-61-class bugs for every keyword no arm claims — silently, and invisibly to every
metric except KWNAME. Making reserved words unavailable to that fallback converts an open-ended bug
class into honest diagnostics. Fixing spellings one at a time does not.

A related fidelity defect: `SYSML_RESERVED_KEYWORDS` (`src/parser/lex.rs`) is one merged list applied
to `.kerml` too, so `entry`, `exit`, `do`, `frame` and `accept` are over-quoted on emission despite
being keywords in neither KerML grammar.

## Method notes for whoever picks this up

- **State the commit you measured.** Both stale inventories above were produced against `main`
  because their worktrees were branched from the default branch; the disagreement between their
  results and the branch's actual behaviour is what eventually exposed it.
- The release corpus is gitignored, so a `git worktree` has neither the corpus nor the grammar.
  Symlink it: `ln -s <repo>/sysml-v2-release sysml-v2-release`.
- Grep test output for `^error` and `could not compile`, not only `FAILED` and `panicked` — a test
  target that fails to *compile* prints neither.
- The Pilot Implementation is the second authority and repeatedly changed verdicts here. Check both
  grammars before classifying anything as invalid input.
