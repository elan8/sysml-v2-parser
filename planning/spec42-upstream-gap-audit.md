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
| 59 | Direction beside `end` rejected | Split: unauthorable per the pin; the reachable half is fixed |

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

## Gap 59 -- direction combined with an end feature

**Claim.** No spelling authors an end feature that also carries a direction, so KerML 8.3.3.3.1's
prohibition has no authorable violation; the direction prefix should be accepted alongside `end`.

**Evidence.** The pin makes the combination unauthorable by construction, in both languages:

```text
SysML BNF 284  EndUsagePrefix : Usage = isEnd ?= 'end' ( ownedRelationship += OwnedCrossFeatureMember )?
SysML BNF 298  UnextendedUsagePrefix : Usage = EndUsagePrefix | BasicUsagePrefix
KerML BNF 578  EndFeaturePrefix : Feature = ( isConstant ?= 'const' )? isEnd ?= 'end'
KerML BNF 585  FeaturePrefix = ( EndFeaturePrefix ( ... )? | BasicFeaturePrefix ) ...
```

`direction` lives only in `RefPrefix`/`BasicFeaturePrefix`, and those are the *other* alternative of
the same choice. There is no normative prefix order that spells both, which is why KerML states the
restriction as a metamodel constraint: an end feature acquires a direction by redefinition, never by
notation. Accepting `in end port p : T;` would be a deliberate deviation from
`docs/conformance-target`.

**Disposition. Split.**

- The direction-beside-`end` half is **not an upstream gap**. Rejecting it is conformant. The
  reachable and testable half of the gap's acceptance criterion -- "retain stable recovery for
  invalid combinations" -- is pinned by `tests/snapshots/spec42/end_prefix_recovery.md`, which
  authors both orders in a connection definition body and in a KerML type body and shows a stable
  diagnostic, an exact malformed span, and an untouched valid sibling after each.
- The audit did find a *reachable* direction gap next to it, and that half is **fixed**: `class`
  was the one KerML classifier keyword routed to a legacy `ClassDef` node with an attribute body
  instead of to the shared `KermlClassifierDecl`, so `class C { in feature x : T; }` reached
  recovery while `struct`, `behavior` and `type` accepted the same member, and `class C` was
  re-emitted as `class def C`, inventing a keyword no production spells. `ClassDef` is deleted and
  `class` joins its siblings.

## Deferred neighbouring debt

Found during the audit, out of scope for these commits, recorded so it is not rediscovered:

- `KermlFeatureMember` and `TypedParameterMember` split one production (`BasicFeaturePrefix`) across
  two AST nodes on whether a direction was authored. They should share one prefix component the way
  the occurrence-usage families now share `OccurrenceUsagePrefix`.
- `feature_modifiers` accepts `ordered`/`nonunique` in any order and any repetition, which is wider
  than `MultiplicityPart` (which spells at most one of each, `ordered` first or `nonunique` first).
  Narrowing it needs its own recovery slice.
- The parser accepts `derived end feature`, `abstract end feature` and `composite end feature`,
  which `EndFeaturePrefix` does not spell. Narrowing that is the mirror image of gap 59 and needs
  corpus evidence for what tolerated it.
- The `(typed-parameter)` and `(kerml-feature)` semantic projections emit far fewer fields than
  their nodes carry, so several invariants are only observable through `FORMAT`.
- `variation` is still refused on the definition kinds that store `BasicDefinitionPrefix` as a bare
  `is_abstract` bool -- `RequirementDef`, `CaseDef`, `AnalysisCaseDef`, `VerificationCaseDef`,
  `UseCaseDef`, `ConstraintDef`, `ViewDef`, `RenderingDef`, `MetadataDef`, `OccurrenceDef` -- and on
  `attribute def`, `item def`, `enum def` and `state def`, all of which the pin allows it on. Each
  needs the same one-slot conversion the six connection-like and part definitions received.
- `attribute x : Real;` at namespace level reaches `AttributeDef` and re-emits as
  `attribute def x : Real;`, inventing a `def` keyword no production spells. `AttributeUsage`
  exists and is reached by other spellings, so this is a dispatch-order question, not a missing
  node. The same shape produced the `calc` gap that Gap 53 closed.
- `feature_modifiers` accepts the two `MultiplicityPart` keyword slots in any order and any
  repetition, which is wider than the production (at most one of each, `ordered` first or
  `nonunique` first).
