# spec42 upstream parser gap audit

Downstream `spec42` maintains `planning/UPSTREAM_PARSER_GAPS.md`, an active list of information it
claims this parser either rejects, drops, or represents ambiguously. This document audits each open
entry against the authoritative conformance pin and against the parser as it actually stands, and
records the disposition that the accompanying commits implement.

## Authority

Every claim below is checked against the pinned grammar named by `docs/conformance-target`:

```text
release_tag=2026-04
release_repo=https://github.com/Systems-Modeling/SysML-v2-Release
grammar_content_hash=fnv1a64:95f39e912f73b917
```

The BNF files are `sysml-v2-release/bnf/SysML-textual-bnf.kebnf` and
`sysml-v2-release/bnf/KerML-textual-bnf.kebnf`; the corpus is `sysml-v2-release/sysml.library` and
`sysml-v2-release/kerml`. A gap entry that asks for syntax the pin does not spell is not a gap in
this parser, and implementing it would break the conformance claim `docs/conformance-target` makes.
Such entries are dispositioned here with the production that settles them, plus a regression fixture
so the behaviour cannot drift silently.

`spec42` recorded the gaps against `204ca48`. This audit re-verified every entry against the current
tree, which is 36 commits ahead of that revision, so several entries had already been closed by
unrelated work and needed only a pinning fixture.

## Disposition summary

| Gap | Verified state at audit | Disposition |
| --- | --- | --- |
| 41 | `that` is an ordinary reference | Not a parser gap -- `that` is a declared library feature |
| 42 | Every named sub-form parses | Already closed; pinned by fixture |
| 52 | `unique`/`nonordered` consumed and discarded | Fixed: authored ordering/uniqueness keywords retained with spans |
| 53 | `ref`/`calc`/`actor` short-name and multiplicity gaps remain | Fixed: sibling-field parity |
| 55 | Bare `/* ... */` discarded as trivia | Fixed: it is a `Comment`, not trivia |
| 56 | Every body member family already retained | Already closed; pinned by fixture |
| 57 | No declared name published | Already closed; pinned by fixture |
| 58 | Prefix kept but spanless; `variation` rejected | Fixed: spanned prefix, `variation` accepted |
| 59 | Direction beside `end` rejected | Split: keyword-less `end` + `RefPrefix` fixed; keyworded forms conformantly refused |
| 64 | `conjugates` reports `unsupported_grammar_form` | Fixed: `Conjugation` node and `ConjugationPart` |
| 65 | `state def S parallel { … }` rejected outright | Fixed: shared body-modifier production |
| 67 | Restriction modifiers beside `end` rejected | Split: as Gap 59 -- keyword-less form fixed, diagnostic corrected |
| 70 | Declaration, alias and import members rejected | Fixed: all four `MetadataBody` alternatives dispatched |
| 80 | Usage-side modifier consumed and discarded | Fixed with Gap 65: `StateBodyModifier` with its span |

## Gap 41 -- implicit `that` self-reference

**Claim.** `that` should be lexically distinguished, either by adding it to
`SYSML_RESERVED_KEYWORDS` or by a dedicated `Expression::ImplicitThat` variant, because
`sysml_resolution` cannot otherwise tell it from a user declaration.

**Evidence.** `that` is not a keyword in either pinned BNF. It is a *declared feature* of the
Kernel Semantic Library:

```text
sysml-v2-release/sysml.library/Kernel Libraries/Kernel Semantic Library/Base.kerml:33
    abstract feature things: Anything [1..*] nonunique {
sysml-v2-release/sysml.library/Kernel Libraries/Kernel Semantic Library/Base.kerml:40
        feature that : Anything[1] {
            doc /* For each value of things, the "featuring instance" of that value. ... */
```

`Base::things` is the top-level feature every element is a value of, and `Anything::self` is
declared as `subsets things chains things.that`, so `that` is in scope everywhere by ordinary
inherited-member lookup. That is precisely the "implicit self-reference" semantics the gap
describes -- the library expresses it as a feature, not as a keyword.

**Disposition. Not an upstream gap.** Reserving `that` would make `Base.kerml:40`'s own
`feature that : Anything[1]` unparseable, i.e. it would break the pinned library the conformance
claim rests on. The parser is correct to lex `that` as an identifier and to publish an ordinary
arena-backed reference for it; resolving it is inherited-name lookup in `Base::things`, which is
`sysml_resolution`'s own layer.

Pinned by `tests/snapshots/spec42/that_self_reference.md`, which shows the declaration and the
bare, cast and member-access uses side by side and proves the declaration and the references are
distinct arena identities rather than one spelling.

## Gap 42 -- requirement-body member families

**Claim.** `RequirementDefBodyElement` still lacks a parameter member, a `Port`/`Allocate` variant,
a nested `requirement def`, a bare `requirement;`, and the `frame concern <name> : <Type>;`
sub-form.

**Evidence.** All five parse at the audited revision with no diagnostics, and each reaches its own
typed node rather than recovery: `in ref part`/`in calc` reach `RefDecl`/`CalcUsage`, `port` reaches
`PortUsage`, `allocate` reaches `AllocationUsage`, `requirement def <'1'> A { }` reaches a nested
`RequirementDef`, bare `requirement;` reaches an anonymous `RequirementUsage`, and
`frame concern vs : VehicleSafety;` reaches `FrameMember` with `concern_keyword` set.

**Disposition. Already closed.** Pinned by `tests/snapshots/spec42/requirement_body_members.md`,
which authors every named sub-form in one requirement definition body so source order and
per-member typing are both visible.

## Gap 52 -- `readonly`, `variable`, authored `unique`

**Claim.** `readonly`, SysML `variable`, and an authored `unique` have no representation.

**Evidence, `readonly` and `variable`.** Neither string occurs in either pinned BNF, and neither
occurs in the pinned corpus outside prose. `MultiplicityPart` (SysML BNF 495, KerML 639) spells only
`ordered` and `nonunique`; the KerML mutability slot is `( isVariable ?= 'var' | isConstant ?=
'const' )` (KerML BNF 582), whose two spellings are already carried by
`KermlFeatureMember::is_var`/`is_const`. There is no authorable `readonly` or `variable` to preserve.

**Evidence, `unique`.** `MultiplicityPart` sets `isUnique = false` from `nonunique` and never spells
a positive `unique`. The parser nevertheless *accepted* `unique` and `nonordered` and threw them
away (`feature_modifiers` in `src/parser/attribute.rs`: "recognized and consumed, but not
recorded"), so `attribute d : Real[0..*] unique;` re-emitted as `attribute d : Real[0..*];`. That is
the exact failure mode `AGENTS.md` forbids: recognized syntax silently consumed with neither a
retained structure nor a recovery node, and a formatter that invents a different document.

**Disposition. Fixed for `unique`/`nonordered`; recorded as unauthorable for `readonly`/`variable`.**
The `ordered: bool` + `nonunique: bool` pair on all thirteen nodes that carried it is replaced by
the grammar-owned `MultiplicityModifiers` component, whose two slots are
`Option<Node<MultiplicityOrdering>>` and `Option<Node<MultiplicityUniqueness>>`. Presence is the
authored fact and the node carries the keyword's exact span, so authored `unique` is now distinct
from omission, `nonordered` no longer disappears, and no combination the grammar forbids -- such as
`ordered nonordered` -- is representable. `readonly`/`variable` continue to reach recovery with a
stable diagnostic and an exact span, pinned as such.

A follow-up slice narrowed the parser to the production's actual cardinality: each slot fills at
most once per *declaration* (threaded as an accumulator through both positions
`FeatureSpecializationPart` offers, rather than folded first-wins), the multiplicity range likewise,
and any excess is left unconsumed so the member reaches the enclosing scope's recovery instead of
being swallowed. Pinned by `tests/snapshots/spec42/multiplicity_part_cardinality_legal.md`,
`multiplicity_part_repeated_slot.md` and `multiplicity_part_range_cardinality.md`.

## Gap 53 -- missing multiplicity, uniqueness and short-name fields

**Claim.** Fifteen nodes are missing a `multiplicity`, `nonunique` or `short_name` field a sibling
carries.

**Evidence.** Field-by-field re-verification at the audited revision found most already closed:
`AttributeDef`, `ConstraintUsage`, `RequirementUsage` and `RequirementActorDecl` all carry
`multiplicity`; `PartUsage` carries `nonunique`; `ActionUsage`, `OccurrenceUsage`,
`ConstraintUsage`, `EndDecl`, `ReturnDecl` and `ViewUsage` all carry `short_name`. Three did not:

- `RefDecl` had no `short_name`, so `ref <rd> rd : T;` -- legal by
  `ReferenceUsage = ( EndUsagePrefix | RefPrefix ) 'ref' Usage` with
  `UsageDeclaration = Identification FeatureSpecializationPart?` -- reached recovery.
- `CalcUsage` had no `multiplicity`, and `calc` in a namespace body did not reach `CalculationUsage`
  at all: `calc cu2 [1];` was reported as an unimplemented extended-library declaration, and
  `calc cu3 : F[1];` was parsed as a `CalculationDefinition` and re-emitted as `calc def cu3 : F;`,
  inventing a `def` keyword the author did not write and dropping the multiplicity.
- The use-case-family `ActorUsage` had no `short_name`, so `actor <sa> an : T;` reached recovery,
  while its sibling `RequirementActorDecl` accepted the same spelling.

**Disposition. Fixed** for all three, each mirroring the sibling named above.

## Gap 55 -- comment trivia and documentation fidelity

**Claim.** Plain `/* ... */`, doc-style `/** ... */` and `//` line comments are lexer trivia and
unreachable from the AST; `DocComment.text` is a raw slice with no normalization policy.

**Evidence.** The pin settles the question the gap leaves open. KerML BNF 32-39 defines *three*
lexical forms, and only two of them are notes:

```text
SINGLE_LINE_NOTE = '//' LINE_TEXT
MULTILINE_NOTE   = '//*' COMMENT_TEXT '*/'
REGULAR_COMMENT  = '/*'  COMMENT_TEXT '*/'
```

and KerML BNF 199 makes an unkeyworded `REGULAR_COMMENT` a complete `Comment`, because every
preceding group in the production is optional:

```text
Comment =
    ( 'comment' Identification ( 'about' ... )? )?
    ( 'locale' locale = STRING_VALUE )?
    body = REGULAR_COMMENT
```

`Comment` is an `AnnotatingElement` (KerML BNF 188). So a bare `/* ... */` at a member position is
*syntax*, not trivia, and `/** ... */` is simply a `Comment` whose `COMMENT_TEXT` happens to begin
with `*` -- it is not a separate production and must not become one. `//` and `//* ... */` are
notes and stay trivia.

KerML BNF 214 note 1 additionally specifies the body normalization exactly (strip the delimiters,
strip white space through the first line terminator, then per line strip leading white space, one
leading `*`, and one following space), and BNF 231 note 1 applies the same processing to
`TextualRepresentation`.

**Disposition. Fixed.** A bare `/* ... */` in a member position parses as an unkeyworded
`CommentAnnotation` rather than being discarded, so the formatter no longer deletes it, and
`/** ... */` reaches the same node -- it is not given a production of its own, because the pin does
not give it one.

The seam is one new lexical helper, `lex::ws_and_notes`, used exactly where a member may begin: the
root loop of both entry points, the two brace-member loops, and every `*_body_element` dispatcher.
Every other position keeps `ws_and_comments`, so a comment between the tokens of a declaration
stays trivia -- the grammar has no member there for it to be. A scope whose member set cannot yet
hold an annotating element falls back to consuming it as trivia, so this widening cannot turn a
previously-parsing document into a recovered one.

The corpus supports the boundary. The pinned release contains 2 857 `REGULAR_COMMENT`s. All but
thirteen either follow a `doc`/`comment`/`rep` keyword clause or sit at a member position -- after
`}`, after `;`, or at the start of the file. Of the thirteen, most are the `comment about X
/* ... */` spelling, which the keyworded parser already owned; the remainder sit between the tokens
of a declaration (`package /* ... */ Name {`, `sysml/src/examples/Simple Tests/CommentTest.sysml:5`)
and stay trivia, which is what leaving `ws_and_comments` in place everywhere but the member
boundaries preserves.

Comment, documentation and textual-representation bodies keep their authored bytes in `text` and
gain `body_span` for provenance, and `normalize_comment_body` implements the pinned processing
rules once for all three. Normalization is deliberately a view rather than a parse-time rewrite:
the formatter needs the authored bytes to reproduce the document.

Pinned by `tests/snapshots/spec42/comment_annotating_elements.md`, which authors a bare comment at
file level, between a brace and its first member, after a member, nested one body down, and closing
the file, beside the `//` and `//*` notes that stay trivia and a mid-declaration comment that also
stays trivia.

## Gap 56 -- enumeration body annotations, literal bodies, initializers

**Claim.** `EnumerationBody` is `Body<EnumeratedValue>`, so annotations, per-literal bodies and
`= expr` initializers are all discarded.

**Evidence.** Already closed at the audited revision. `EnumerationBody` is
`Body<EnumerationBodyElement>`, whose `Annotating(AnnotatingMember)` variant is the complete
`AnnotatingElement` production -- `doc`, `comment`, `rep ... language "..."` and `@Metadata` all
reach it -- interleaved with `Value(Node<EnumeratedValue>)` in source order, and an
`EnumeratedValue` keeps its own `body: PartUsageBody` and its `value: Option<Node<FeatureValue>>`
initializer.

The gap's `rep` claim did not reproduce: `TextualRepresentation` is
`( 'rep' Identification )? 'language' language = STRING_VALUE body = REGULAR_COMMENT` (KerML BNF
228), so the `language` keyword is not optional. `rep asText language "text" /* ... */` parses;
only the `language`-less spelling reaches recovery, correctly.

**Disposition. Already closed.** Pinned by `tests/snapshots/spec42/enumeration_body_members.md`,
which interleaves all four annotating forms with three enumerated values, one carrying a body, one
an initializer, and one both.

## Gap 57 -- anonymous specialization shorthand

**Claim.** `:> annotatedElement : SysML::Usage;` publishes `annotatedElement` as the member's
declared name as well as its subsetting target, so the feature subsets itself.

**Evidence.** At the audited revision the member reports `(declaration-name none)` and keeps only
the subsetting relationship; two shorthand members in one body produce two distinct arena
references and no declared name at all.

**Disposition. Already closed.** Pinned by
`tests/snapshots/spec42/anonymous_specialization_shorthand.md`, which authors repeated `:>` and
`:>>` shorthand members in one metadata definition so the absence of a declared name and the
distinctness of the reference identities are both visible.

## Gap 58 -- authored `abstract` on connection-like definitions

**Claim.** `ConnectionDef`, `FlowDef`, `AllocationDef` and `InterfaceDef` carry no abstractness
field, so `abstract connection def C { ... }` lowers with no modifier fact at all.

**Evidence.** Partly closed at the audited revision and partly not. All four nodes did carry
`definition_prefix: Option<DefinitionPrefix>`, so `abstract` reached the AST -- but spanless, so a
consumer could tell `abstract` from omitted and still not point at the keyword. And `variation`,
the *other* alternative of the same slot, was refused outright: `BasicDefinitionPrefix = isAbstract
?= 'abstract' | isVariation ?= 'variation'` (SysML BNF 219; Pilot `SysML.xtext` 490) is reached by
all four through `OccurrenceDefinitionPrefix` (BNF 541), yet `variation connection def V;` fell
through to the unimplemented extended-library declaration.

**Disposition. Fixed.** The six nodes that carried the slot -- the four connection-like ones plus
`PartDef` and `ExtendedDefinition` -- now hold `Option<Node<DefinitionPrefix>>`, and the semantic
projection shows the authored span beside the spelling, so omitted, `abstract` and `variation` are
all distinguishable. `part_def` and `extended_definition_inner` each carried their own copy of the
alternation; both call the shared parser that captures the span. Pinned by
`tests/snapshots/sysml/connection_like_definition_prefixes.md`.

A follow-up slice completed the conversion across every definition kind whose production reaches
`BasicDefinitionPrefix` -- seventeen nodes, nine converted from `is_abstract: bool` and eight that
had no field at all and were silently discarding `abstract` (the pinned library was losing it on
`abstract attribute def ScalarMeasurementReference` and a dozen siblings). `DefinitionPrefixOptions`'
two loosely-coupled booleans became one `BasicPrefixSlot` enum naming what each production spells.

Two productions genuinely differ and were not forced into uniformity: `EnumerationDefinition`
(BNF 518; Pilot 767) reaches neither `DefinitionPrefix` nor `OccurrenceDefinitionPrefix` and spells
no prefix at all, and `MetadataDefinition` (BNF 1652; Pilot 121) inlines `isAbstract ?= 'abstract'`
with no `variation` alternative, so its bool is the accurate shape. Both refuse what they do not
spell. Pinned by `tests/snapshots/sysml/definition_prefix_alternatives.md` and
`definition_prefix_refusals.md`.

## Gap 59 -- direction combined with an end feature

**Claim.** No spelling authors an end feature that also carries a direction, so KerML 8.3.3.3.1's
prohibition has no authorable violation; the direction prefix should be accepted alongside `end`.

**Evidence.** *(Corrected — the original evidence for this entry was wrong. See "What the reference
implementation actually says" below.)* The published BNF makes the combination unauthorable in the
**keyworded** forms of both languages:

```text
SysML BNF 284  EndUsagePrefix : Usage = isEnd ?= 'end' ( ownedRelationship += OwnedCrossFeatureMember )?
SysML BNF 298  UnextendedUsagePrefix : Usage = EndUsagePrefix | BasicUsagePrefix
KerML BNF 578  EndFeaturePrefix : Feature = ( isConstant ?= 'const' )? isEnd ?= 'end'
KerML BNF 585  FeaturePrefix = ( EndFeaturePrefix ( ... )? | BasicFeaturePrefix ) ...
```

`direction` lives only in `RefPrefix`/`BasicFeaturePrefix`, and those are the *other* alternative of
the same choice, so `in end port p : T;` and `end in port p : T;` are both correctly refused.

### What the reference implementation actually says

This entry previously concluded that the combination is unauthorable "by construction, in both
languages", and that the KerML constraints therefore have no authorable violation. **That conclusion
was false**, and the published BNF alone does not show why. Checked against the Pilot
Implementation:

- `org.omg.kerml.xtext/src/org/omg/kerml/xtext/KerML.xtext:510-526` is *identical* to the pinned
  `.kebnf`, including after the `OwnedCrossFeatureMember` -> `OwnedCrossingFeatureMember` rename the
  Pilot has since made. The KerML half of the conclusion holds.
- `org.omg.sysml.xtext/src/org/omg/sysml/xtext/SysML.xtext:630-633` does not:

  ```text
  DefaultReferenceUsage returns SysML::ReferenceUsage :
      ( isEnd ?= 'end' )? RefPrefix
      UsageDeclaration ValuePart? UsageBody
  ```

  The **keyword-less** reference usage spells `end` *and* a full `RefPrefix`. So
  `end derived x : T;`, `end in x : T;` and `end constant x : T;` are legal SysML, and the parser
  rejected all three.
- `org.omg.kerml.xtext/.../validation/KerMLValidator.xtend:669-677` implements both
  `validateFeatureEndNoDirection` and `validateFeatureEndNotDerivedAbstractCompositeOrPortion` as
  Xtext `@Check`s on the **textual** model -- which is only coherent because the production above
  makes them reachable from text.

So the gaps' underlying need was real; only the spelling they proposed (modifier *before* `end`, or
with a `feature`/`part` keyword) was not. The corpus is silent either way: of 403 official
`.sysml`/`.kerml` files, zero author any of these combinations, so corpus evidence neither confirmed
nor refuted the claim and must not be read as confirming it.

**Disposition. Split three ways.**

- **Fixed**: the keyword-less `DefaultReferenceUsage` form. `EndDecl` now carries a `RefPrefix`
  parsed between `end` and the declaration, retained with its spans, re-emitted by the formatter,
  and projected as `(prefix ...)`. Before this it was rejected outright; briefly, it was worse than
  that -- an earlier revision of the `end_feature_invalid_prefix` classification reported this
  *legal* syntax as a prefix violation. Pinned by `tests/snapshots/spec42/end_ref_prefix.md` and
  `tests/end_feature_prefix_diagnostic.rs`.
- The direction-beside-`end` half **in the keyworded forms** is not an upstream gap. Rejecting it is
  conformant. The
  reachable and testable half of the gap's acceptance criterion -- "retain stable recovery for
  invalid combinations" -- is pinned by `tests/snapshots/spec42/end_prefix_recovery.md`, which
  authors both orders in a connection definition body and in a KerML type body and shows a stable
  diagnostic, an exact malformed span, and an untouched valid sibling after each. That diagnostic
  is now `end_feature_invalid_prefix` and names the offending keyword; see Gap 67 below.
- The audit did find a *reachable* direction gap next to it, and that half is **fixed**: `class`
  was the one KerML classifier keyword routed to a legacy `ClassDef` node with an attribute body
  instead of to the shared `KermlClassifierDecl`, so `class C { in feature x : T; }` reached
  recovery while `struct`, `behavior` and `type` accepted the same member, and `class C` was
  re-emitted as `class def C`, inventing a keyword no production spells. `ClassDef` is deleted and
  `class` joins its siblings.

## Gap 64 -- conjugation declarations

**Claim.** Only the `~T` conjugated-typing flag is modelled; `ConjugationPart` and the
`Conjugation` relationship have no node, so `classifier One conjugates A;` is
`unsupported_grammar_form`.

**Evidence.** Confirmed. `TypeDeclaration = … ( SpecializationPart | ConjugationPart )?` (KerML BNF
455) and `ConjugationPart : Type = ( 'conjugates' | '~' ) ownedRelationship += OwnedConjugation`
(462) are a different production from `FeatureTyping`'s `~`: one conjugates the declared type, the
other the type a feature is typed by. Both KerML conjugation constraints
(`validateTypeAtMostOneConjugator`, `validateSpecializationSpecificNotConjugated`) were unauthorable.

**Disposition. Fixed.** `ast::Conjugation` is carried on `KermlClassifierDecl` as
`Option<Node<_>>`, with one target (`OwnedConjugation` is a single `[QualifiedName]`, unlike
`SpecializationPart`'s comma-separated list) and a `ConjugationSpelling` recording which of the two
interchangeable spellings was authored. Because the two parts are alternatives of one choice, the
parser reaches the conjugation only where no specialization was authored, so no declaration can
carry both. Pinned by `tests/kerml_conjugation_part.rs`.

## Gap 65 / Gap 80 -- the state body modifier

**Claim.** `parallel` is rejected on a `state def` body and accepted-then-discarded on a `state`
usage, so `StateUsage::isSubstateUsage` has nothing to read.

**Evidence.** Confirmed, and the two halves had different causes. `state def Machine parallel { … }`
failed with `missing_body_or_semicolon` across the whole declaration, while `src/parser/state.rs`
consumed the usage-side modifier and bound it to `_`.

**Disposition. Fixed.** `StateDefBody = ';' | ( isParallel ?= 'parallel' )? '{' StateBodyItem* '}'`
(SysML BNF 1192) is parsed by one shared combinator for the definition, the usage and
`ExhibitStateUsage`, which shares `StateUsageBody`. The result is `ast::StateBodyModifier` as
`Option<Node<_>>` over the authored keyword -- a node rather than a boolean, because which keyword
was written is the fact lowering needs. The keyword must be a whole word, so
`state initialState { … }` stays a usage named `initialState`. Pinned by
`tests/state_body_modifier.rs`.

## Gap 67 -- restriction modifiers alongside `end`

**Claim.** `derived`/`abstract`/`composite`/`portion`/`var` should be accepted with `end`, so
`validateFeatureEndNotDerivedAbstractCompositeOrPortion` has an authorable violation.

**Evidence.** Same structure as Gap 59, and the same correction applies: `EndFeaturePrefix` (KerML
BNF 573) spells only `( 'const' )? 'end'` and every restriction slot lives in `BasicFeaturePrefix`
(577), the *other* alternative of the same choice (584) -- but SysML's keyword-less
`DefaultReferenceUsage` (630) spells `'end'? RefPrefix`, so `end derived x : T;` and
`end constant x : T;` are legal and were being rejected. See Gap 59's "What the reference
implementation actually says".

**Disposition. Split, like Gap 59.** The keyword-less form is **fixed** (see Gap 59). Accepting the
*keyworded* combination is not an upstream gap. The other real defect was what the parser *said*: `composite`, `portion` and `var` were reported as
"`composite` is not a SysML keyword" -- flatly false -- and a direction as an anonymous "unexpected
token in calc body", naming a scope the author never wrote. Neither identified the keyword with no
derivation, so the violation was not observable from the diagnostic even though the syntax was
correctly refused.

Both now report `end_feature_invalid_prefix`, naming the offending keyword, the slot it belongs to
and the production that excludes it. The classification runs before the keyword-in-scope
classifications and walks the leading keyword run only, so a feature named `end2` and a
`var connector` are untouched. This also closes the "`abstract end feature x;` splits into two
members" item recorded under *Deferred neighbouring debt* below: the whole member is now one
recovery node with one precise diagnostic, so no keyword is turned into a reference to a feature
nobody declared. Two `connection def` port ends previously swallowed by
`recovery_cascade_suppressed` now each report their real cause. Pinned by
`tests/end_feature_prefix_diagnostic.rs` and the regenerated
`tests/snapshots/spec42/end_prefix_recovery.md`.

## Gap 70 -- named members in a metadata body

**Claim.** Named members in a metadata-feature body are rejected before lowering.

**Evidence.** Confirmed, and wider than the claim. `MetadataBody : Type = ';' | '{'
( DefinitionMember | MetadataBodyUsageMember | AliasMember | Import )* '}'` (SysML BNF 1677) has
four alternatives; the parser dispatched the annotating and usage members only, so
`attribute def X;`, `alias a for b;` and `import X::*;` all reached recovery inside `@M { … }`.
A *named* member is only reachable through `DefinitionMember`: `MetadataBodyUsage`'s
`OwnedRedefinition` is a reference to an existing feature, not a declaration.

**Disposition. Fixed.** `MetadataBodyElement` gains `Definition`, `Alias` and `Import`. The
declaration member carries an `AttributeBodyElement`, the shared type-body member set the crate
already uses for `metadata def` bodies -- a superset of `DefinitionMember`, so this over-accepts
exactly as those bodies already did rather than in a new way, and the two scopes keep one member
set instead of two that drift. The keyword-less usage member is tried first, because
`OwnedRedefinition` is a bare qualified name the declaration parsers would otherwise claim. The
semantic projection and the opacity walk each factor their attribute-body arm into one function
both scopes call. Pinned by `tests/metadata_body_members.rs`.

## Wave 2 -- gaps re-probed at the wave-1 pin

Every entry in this section was checked against the **reference implementation**
(`SysML-v2-Pilot-Implementation`, `org.omg.sysml.xtext/.../SysML.xtext` and
`org.omg.kerml.xtext/.../KerML.xtext`) before any code was written, not against the published
`.kebnf` alone. Gap 59 records why: the two artifacts agree on the KerML prefix productions but
not everywhere, and a claim checked only against the published BNF was wrong once already.

Corpus silence is *not* evidence here. Of the 403 official `.sysml`/`.kerml` files, several of the
constructs below appear zero times -- including ones that are unambiguously legal -- so "the corpus
never writes it" was treated as no information rather than as refutation.

### Fixed

- **Gap 81 (regression), calc bodies.** `calculation_body_element`'s directed-parameter branch
  committed to `in_out_decl` with `?`. A *kinded* parameter (`in expr p : T;`, `in bool redefines
  a;`, `in feature p : T;`) is a KerML `Feature` whose kind keyword names its production, and
  `in_out_decl` refuses to read a kind keyword as a parameter name, so the whole member fell to
  recovery. It now falls through to the KerML route the same function already ends in.

  The `constraint def` half is **not** fixed, deliberately: that dispatcher has no KerML
  delegation, `ConstraintBody` is a `CalculationBody`, and `'expr'` occurs zero times in
  `SysML.xtext` (which inherits only `KerMLExpressions`). Rejecting it there is conformant.

- **Gap 72.** `PerformActionUsage`'s declaration is `OwnedReferenceSubsetting
  FeatureSpecializationPart?` *or* `ActionUsageKeyword UsageDeclaration?` (`SysML.xtext:1411-1417`).
  Only the second reached an action body, so `action def G { perform L::doIt; }` was recovered while
  the part body -- the same production -- accepted it.

- **Gap 73.** `IncludeUseCaseUsage` is a choice (`SysML.xtext:2300-2306`) and only the reference
  alternative was parsed, so `include use case v;` shredded into a bare `FeatureRef` naming the
  keyword `include` plus a sibling usage, with no diagnostic. `IncludeUseCase` now carries the
  keyword span, declared name and typing, with an optional `target`.

- **Gap 74.** `ConstraintUsageDeclaration` is an ordinary `UsageDeclaration`
  (`SysML.xtext:2066-2071`), so `require constraint c : C;` declares *and* specializes.
  `RequireConstraint` gains `typing`, re-emitted by the formatter.

- **Gap 75.** `UsageBody = DefinitionBody` (604), so both port bodies reach `DefinitionBodyItem ->
  OccurrenceUsageMember -> StructureUsageMember -> PartUsage`. Both sides now accept a part member.
  The gap's `composite` modifier is **not** part of this: `'composite'` occurs zero times in
  `SysML.xtext`.

- **Gap 76, the trigger half.** `PayloadParameter`'s third alternative is `TriggerValuePart`
  (1459-1461), whose kinds are `'at' | 'after'` and `'when'`, so an accept *node* admits a trigger
  exactly as a transition does. The parser treated triggers as transition-only and carried an
  `unreachable!` justified by that assumption.

- **Gap 66, the spelling half.** Each subsetting kind has two interchangeable spellings and the AST
  carried none, so the formatter rewrote every authored keyword into its operator. Corpus fixtures
  were affected. `SubsettingRelationship` gains `spelling`.

### Not upstream gaps -- the reference grammar does not spell them

- **Gap 61, `message` in a KerML body.** `'message'` occurs only in `SysML.xtext`; a `classifier`
  body is a KerML `TypeBody`. The SysML scopes where `Message` is legal (`calc def`, `part def`)
  already accept it.
- **Gap 76, the `if ... then ... else` half.** `IfNode = ActionNodePrefix 'if'
  ExpressionParameterMember ActionBodyParameterMember ( 'else' … )?` (1596-1608) has no `then`, and
  `ActionBodyParameter` is always braced. The braced spellings, including `else if` chains, parse.
- **Gap 77, a `transition` member in an action body.** `TransitionUsageMember` appears only in
  `StateBodyItem` (1754-1770). The state-body transition-effect forms the gap also names already
  parse at this revision.
- **Gap 78, `abstract variation`.** `BasicDefinitionPrefix = isAbstract ?= 'abstract' | isVariation
  ?= 'variation'` (490-492) is a *choice*; no order spells both.
- **Gap 79, `expose` in a package body and `verify`/`render` in a part-definition body.** `Expose`
  is admitted only by `ViewBodyItem`, `ViewRenderingMember` only by `ViewDefinitionBodyItem` and
  `ViewBodyItem` (2325-2365). Admitting them elsewhere so semantics can report the owner is an
  error-tolerance policy question, not a grammar gap; see Gap 59's note on that trade-off.
- **Gap 52, the `var` spelling.** `'var'` occurs zero times in `SysML.xtext`, and `occurrence def`
  is a SysML production. `var` is the KerML variability keyword and is accepted in KerML bodies.

### Still open

- **Gap 66, clause count.** `specialization_clauses` merges repeated clauses of one kind, which is
  correct and corpus-backed for `subsets` and wrong only for `crosses`/`references`, whose KerML
  rules are "at most one *clause*". One relationship per authored clause touches ~112 read sites.
- **Gap 69, a binding connector with a `TypeBody` of ends.** `BindingConnector = FeaturePrefix
  'binding' BindingConnectorDeclaration TypeBody` and the `of … = …` clause is *optional*
  (`KerML.xtext:870-878`), so `binding b { end e1 : A; end e2 : B; }` is legal and is refused. The
  parser requires `left = right` unconditionally and gives the member a SysML usage body rather
  than a KerML `TypeBody`.
- **Gap 62, a repeated payload clause.** `FlowDeclaration`'s `( 'of' PayloadFeatureMember )?` is
  singular, so `flow of Thing of Thing …` is correctly refused; what it lacks is a *precise*
  diagnostic naming the at-most-one rule, as `end_feature_invalid_prefix` does for Gap 59.

## Wave 3 -- the corpus snapshots that still carried diagnostics

Twelve corpus-derived snapshots under `tests/snapshots/spec42/{sysml,kerml}` still recorded a
diagnostic at the wave-2 pin. Each construct was checked against the reference grammar and,
where the two disagree, against the normative model library, before anything was changed.

### Fixed

- **`individual` on every `OccurrenceDefinitionPrefix` family** (`coverage_individual`).
  `OccurrenceDefinitionPrefix = BasicDefinitionPrefix? ( isIndividual ?= 'individual' … )?`
  (SysML BNF 541; `SysML.xtext:804-810`) is reached by calc, constraint, requirement, concern,
  case, verification, use case, view, viewpoint and rendering definitions; only the structural
  and action families accepted it. The nine definition nodes and `ConcernUsage` gain
  `is_individual`; `StateDef` carried it but its projection dropped it. AST version 231.

- **`EndUsagePrefix` as the head of `OccurrenceUsagePrefix`** (`09_connections_example`,
  `11_interface_decomposition_example`, `conjugation_test`, `wheel_package_updated`).

  ```text
  OccurrenceUsagePrefix returns SysML::OccurrenceUsage :
      ( EndUsagePrefix
      | BasicUsagePrefix ( isIndividual ?= 'individual' )? ( portionKind = PortionKind )? )
      UsageExtensionKeyword*                                        -- SysML.xtext:836-843
  EndUsagePrefix : Usage = isEnd ?= 'end' ( ownedRelationship += OwnedCrossFeatureMember )?
                                                                    -- SysML BNF 285
  OwnedCrossFeature : ReferenceUsage = BasicUsagePrefix UsageDeclaration   -- SysML BNF 293
  ```

  The published `.kebnf` (564-570) spells only the second alternative, and the earlier
  `pilot_occurrence_end_prefix_recovery` fixture deliberately refused `end port` on that basis.
  The normative library disagrees with the `.kebnf`: `Interfaces.sysml:72` authors `end port
  source: Port :>> BinaryConnection::source;` and `Flows.sysml:82` `end occurrence source:
  Occurrence :>> Message::source, …;`, and both were recovering. A grammar the specification's
  own library cannot parse against is the erratum, so the parser follows the reference grammar.

  `OccurrenceUsagePrefix` gains a `head` choice -- `End(EndUsagePrefix)` or the basic slots --
  so `end individual part p;` is unrepresentable rather than merely unemitted. The owned cross
  feature is a `BasicUsagePrefix` plus a `UsageDeclaration`, retained with its span: `end [1]
  part bead : TireBead;` crosses a bare multiplicity, `end theCauses [*] occurrence theCause :>
  causes;` a named one. A prefix with no declaration (`end derived part p : T;`) is not a cross
  feature and still reports `end_feature_invalid_prefix`. Every family that owns the prefix
  (part, port, item, occurrence, action nodes, satisfy, constraint and analysis usages) accepts
  the head; the connection, interface, part-definition and occurrence bodies give those typed
  parsers first refusal on an `end`-led member, ahead of the keyword-less `EndDecl`.

  `EndDecl::nested_usage` (GH-53) was this production seen from the other side -- an `EndDecl`
  whose "target" was a complete occurrence or item usage -- and is deleted; the fixtures and
  `tests/gh53_end_decl_nested_usage.rs` now assert the usage node with its `end` head.

- **A nameless `DefaultReferenceUsage` end** (`wheel_package_updated`, `server_sequence_*`).
  `DefaultReferenceUsage = ( isEnd ?= 'end' )? RefPrefix UsageDeclaration …` (`SysML.xtext:
  630-633`) with `UsageDeclaration = Identification? …`, so `end : TireBead[1];` and `end :>>
  source ::> producer.publicationPort;` declare an end through its specialization alone.
  `EndIdentity` gains `Anonymous`; the flow-usage body already dispatched `EndDecl`.

### Fixed since

- `#Security enum secret : …` (`metadata_test`): `EnumeratedValue = UsageExtensionKeyword*
  'enum'? Usage` (`SysML.xtext:784-786`). `EnumeratedValue` gains `extension_keywords`.
- `private ref #Classified #Security z1;` (`metadata_test`): the keyword run sits between
  `ref` and the declaration; `RefDecl` gains `extension_keywords`. `abstract #Classified z2;`
  already reached the KerML feature path with its metadata keywords.
- `#systemdd name :> base { #servicedd :>> x : T { #idd y; } }` (`ahfcore_lib`): `ExtendedUsage`
  (SysML BNF 341) was a `def`-less `ExtendedDefinition` that required a name and owned a
  package body. A typed `ExtendedUsage` node now carries the prefix choice, the keyword run, a
  `UsageDeclaration`, a value and a usage body, in package and part-usage bodies;
  `ExtendedDefinition` requires `def` again. The empty-declaration `#Tag;` / `#Tag { }` spelling
  stays on `MetadataKeywordUsage` in every scope; folding it in is a body-dispatch change across
  all of them and is left as recorded debt.

- `connect bead references t.bead to mountingRim references w.rim;` (`09_connections_example`):
  `ConnectorEnd`'s `( declaredName = Name ReferencesKeyword )?`; `ConnectionEnd` gains
  `declared_name`.
- `specialization subtype x :> Base::things;`, `conjugation c1 conjugate Conjugate1 conjugates
  Original;` and `type Conjugate4 ~ Conjugate1;` (`kerml/types`): the relationship keywords are
  excluded from the declaration's identification, `Conjugation` joins the relationship
  declarations, and the classifier's parsed `ConjugationPart` is now emitted and projected --
  it had been dropped silently.
- `private y: A, '2'[0..*];` (`kerml/examples/classes`): the keyword-less member's typing is
  multi-target.
- `end port p3 : P ::> p.p1;` in an interface *usage* body (`conjugation_test`) and the
  redefinition-led `end :>> source ::> producer.publicationPort;` (`server_sequence_*`).

### Still open at this pin

- None of the corpus-derived snapshots under `tests/snapshots/spec42/{sysml,kerml,sysml.library}`
  carries a diagnostic. `coverage_connectors` is not a corpus file and authors `connector` in a
  SysML `part def`, which `SysML.xtext` does not spell at all; the fixture records a refusal.
- The snapshot corpus is a sample of the 403 release files; `planning/corpus-coverage-*.md`
  record the wider inventories, which this wave did not re-measure.

## Deferred neighbouring debt

Found during the audit, out of scope for these commits, recorded so it is not rediscovered:

- `FeatureSpecializationPart` admits one `MultiplicityPart`, but a declaration that authors two
  *disjoint* slots across its two positions (`[1] ordered : T nonunique`) still retains both. That
  over-accepts against the enclosing production rather than this one, and nothing is dropped, so it
  is not the silent-drop class the cardinality slice closed. It belongs to
  `FeatureSpecializationPart`.
- In a *package* body, a malformed member whose first keyword is in `extended_library_decl`'s
  starter list is classified `unsupported_grammar_form` at severity **warning**, with the message
  "the spec-valid extended-library declaration production is retained but not structurally
  implemented" -- which is false for an ordinary malformed `attribute`/`part` member. The text and
  the siblings are preserved, so nothing is lost, but the classification misattributes the cause.
  Pre-existing and independent of any gap here; it is a body-dispatch question.
- `EndFeaturePrefix = ( isConstant ?= 'const' )? isEnd ?= 'end'` (KerML BNF 578) spells no other
  slot, and the `FeaturePrefix` merge closed most of the over-acceptance: `derived end feature x;`
  now reaches recovery with an exact span and a surviving sibling. `abstract end feature x;` did
  not: it split into two members, the keyword becoming a bare `(expression (ref ...))` naming
  `abstract`. **Closed by Gap 67 above** -- the whole member is now one recovery node carrying one
  `end_feature_invalid_prefix` diagnostic, so no keyword becomes a reference to a feature nobody
  declared. `abstract` followed by a *non-`end`* prefix keyword (`abstract derived feature c;`) is
  a separate dispatch question and remains open; it is recorded in
  `planning/kerml-feature-prefix-matrix.md` §5.2 as pre-existing.
- The `(kerml-feature)` semantic projection is still a bare marker, so the merged node's prefix
  slots, specialization clauses and multiplicity are observable only through `FORMAT`.
  `(typed-parameter)` is gone with the node it projected.
- `attribute x : Real;` at namespace level reaches `AttributeDef` and re-emits as
  `attribute def x : Real;`, inventing a `def` keyword no production spells. `AttributeUsage`
  exists and is reached by other spellings, so this is a dispatch-order question, not a missing
  node. The same shape produced the `calc` gap that Gap 53 closed.
