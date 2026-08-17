# `OccurrenceUsagePrefix` grammar matrix

The shared prefix every occurrence-usage family spells, classified against the pinned grammar,
with every spelling found in the checked-in fixtures and in the pinned `sysml-v2-release` corpus.
The pin is `docs/conformance-target` (`release_tag=2026-04`,
`grammar_content_hash=fnv1a64:95f39e912f73b917`).

`planning/satisfy-requirement-usage-matrix.md` §7 recorded the prefix as the one part of
`SatisfyRequirementUsage` left unmet, and said closing it "properly means a shared typed
occurrence-usage prefix node rather than a satisfy-local one, since every `OccurrenceUsageElement`
spells the same prefix". This matrix is that audit; the seam it defines is shared by
`SatisfyRequirementUsage` and by the `OccurrenceUsage` family, and the remaining families are
listed in §9 with what each still needs.

## 1. Authoritative productions

Verbatim from `sysml-v2-release/bnf/SysML-textual-bnf.kebnf` at the pin.

```text
OccurrenceUsagePrefix : OccurrenceUsage =
    BasicUsagePrefix
    ( isIndividual ?= 'individual' )?
    ( portionKind = PortionKind
      { isPortion = true }
    )?
    UsageExtensionKeyword*                          -- line 564, clause 8.2.2.9.2

BasicUsagePrefix : Usage =
    RefPrefix
    ( isReference ?= 'ref' )?                       -- line 281, clause 8.2.2.6.2

RefPrefix : Usage =
    ( direction = FeatureDirection )?
    ( isDerived ?= 'derived' )?
    ( isAbstract ?= 'abstract' | isVariation ?= 'variation' )?
    ( isConstant ?= 'constant' )?                   -- line 275, clause 8.2.2.6.2

FeatureDirection : FeatureDirectionKind =
    'in' | 'out' | 'inout'                          -- line 272

PortionKind =
    'snapshot' | 'timeslice'                        -- line 585

UsageExtensionKeyword : Usage =
    ownedRelationship += PrefixMetadataMember       -- line 296

PrefixMetadataMember : OwningMembership =
    '#' ownedRelatedElement = PrefixMetadataUsage   -- line 1660, clause 8.2.3.2

PrefixMetadataUsage : MetadataUsage =
    ownedRelationship += OwnedFeatureTyping         -- line 1663

MemberPrefix : Membership =
    ( visibility = VisibilityIndicator )?           -- line 130

VisibilityIndicator : VisibilityKind =
    'public' | 'private' | 'protected'              -- line 175
```

The four productions that *use* the prefix, and the two neighbours that do not:

```text
OccurrenceUsage         = OccurrenceUsagePrefix 'occurrence' Usage             -- line 573
IndividualUsage         : OccurrenceUsage =
                          BasicUsagePrefix isIndividual ?= 'individual'
                          UsageExtensionKeyword* Usage                         -- line 576
PortionUsage            : OccurrenceUsage =
                          BasicUsagePrefix ( isIndividual ?= 'individual' )?
                          portionKind = PortionKind
                          UsageExtensionKeyword* Usage { isPortion = true }    -- line 580
EventOccurrenceUsage    = OccurrenceUsagePrefix 'event'
                          ( ownedRelationship += OwnedReferenceSubsetting
                            FeatureSpecializationPart?
                          | 'occurrence' UsageDeclaration? )
                          UsageCompletion                                      -- line 589

SatisfyRequirementUsage = OccurrenceUsagePrefix 'assert' ( isNegated ?= 'not' )
                          'satisfy' …                                          -- line 1467

-- neighbours that are deliberately NOT this production:
UsagePrefix    : Usage = UnextendedUsagePrefix UsageExtensionKeyword*          -- line 302
                         ( UnextendedUsagePrefix = EndUsagePrefix | BasicUsagePrefix )
ControlNodePrefix : OccurrenceUsage =
                         RefPrefix ( isIndividual ?= 'individual' )?
                         ( portionKind = PortionKind )? UsageExtensionKeyword* -- line 973
OccurrenceDefinitionPrefix : OccurrenceDefinition =
                         BasicDefinitionPrefix?
                         ( isIndividual ?= 'individual'
                           ownedRelationship += EmptyMultiplicityMember )?
                         DefinitionExtensionKeyword*                           -- line 541
```

### 1.1 Similarly named prefixes are not interchangeable

| Production | Differs from `OccurrenceUsagePrefix` by | Consequence |
| --- | --- | --- |
| `UsagePrefix` | admits `EndUsagePrefix` (`end` + `OwnedCrossFeatureMember`) as an alternative to `BasicUsagePrefix`; admits **no** `individual` and **no** `PortionKind` | `end` is legal on an `AttributeUsage`, illegal on an occurrence usage; `individual`/`snapshot` are legal on an occurrence usage, illegal on an attribute usage. Two productions, not one with optional parts |
| `ControlNodePrefix` | `RefPrefix` directly, **without** `BasicUsagePrefix`'s `ref` | `ref merge m;` is not grammatical; `ref occurrence o;` is |
| `OccurrenceDefinitionPrefix` | `BasicDefinitionPrefix` (`abstract`/`variation` only — no direction, no `derived`, no `constant`, no `ref`) and `DefinitionExtensionKeyword*` | a definition prefix cannot carry a direction, and `individual` there also contributes an `EmptyMultiplicityMember` |
| `BasicDefinitionPrefix` | is exactly `RefPrefix`'s third slot, in isolation | reusing one two-alternative enum for both is grammar-backed (see §5.1); reusing the whole prefix struct is not |

`OccurrenceUsagePrefix` and `ControlNodePrefix` differ only in `ref`, and `IndividualUsage`/
`PortionUsage` inline `BasicUsagePrefix` plus the same two slots rather than naming
`OccurrenceUsagePrefix`. That inlining is what makes one shared prefix component correct for all
four occurrence-usage spellings: the slot sequence and its ordering are identical in every one of
them. It is also why `ControlNodePrefix` is *not* migrated by this change — it is a different
production, and modelling it as an `OccurrenceUsagePrefix` would make `ref merge` representable.

## 2. Nesting, cardinality and legal token order

```text
OccurrenceUsagePrefix
├── BasicUsagePrefix                       exactly 1 (all slots individually optional)
│   ├── RefPrefix                          exactly 1
│   │   ├── direction  = FeatureDirection  0..1   'in' | 'out' | 'inout'      mutually exclusive
│   │   ├── isDerived  = 'derived'         0..1   independent
│   │   ├── isAbstract | isVariation       0..1   'abstract' | 'variation'    mutually exclusive
│   │   └── isConstant = 'constant'        0..1   independent
│   └── isReference    = 'ref'             0..1   independent
├── isIndividual       = 'individual'      0..1   independent
├── portionKind        = PortionKind       0..1   'snapshot' | 'timeslice'    mutually exclusive
└── UsageExtensionKeyword                  0..*   ordered, repeatable, each '#' QualifiedName
```

Legal token order is fixed by the production and is the only legal order:

```
[in|out|inout] [derived] [abstract|variation] [constant] [ref] [individual] [snapshot|timeslice] ('#' Ref)*
```

- **Mutually exclusive alternatives:** `in`/`out`/`inout` (one slot); `abstract`/`variation` (one
  slot); `snapshot`/`timeslice` (one slot). Each is modelled as an enum in one optional slot, so
  two of a pair cannot both be recorded and a repeat is an ordinary duplicate-token parse failure.
- **Independently composable modifiers:** `derived`, `constant`, `ref`, `individual` — four
  genuinely independent binary properties, each recorded by the presence of its authored span.
- **Repetition:** only `UsageExtensionKeyword`, which is ordered and keeps authored order.
- **Everything else repeated is malformed**, including `abstract abstract`, `snapshot timeslice`
  and `in out`; the grammar gives no slot for the second token.

`MemberPrefix`'s visibility keyword precedes the whole prefix and belongs to the *membership*, not
to the usage (`OccurrenceUsageMember : FeatureMembership = MemberPrefix ownedRelatedElement +=
OccurrenceUsageElement`, line 259). It is therefore modelled separately, on `Membership`, and is
not a field of `OccurrenceUsagePrefix`. A `then` (`SourceSuccessionMember`, line 1003 region)
precedes even that, and is likewise not part of the prefix.

## 3. Declaration, reference and membership roles

| Prefix component | Identity domain | Representation |
| --- | --- | --- |
| `in` / `out` / `inout` | ordinary keyword | `Option<Node<InOut>>` — enum plus its authored span |
| `derived`, `constant`, `ref`, `individual` | ordinary keywords | `Option<Span>`; `Some`/`None` *is* the flag |
| `abstract` / `variation` | ordinary keywords | `Option<Node<DefinitionPrefix>>` |
| `snapshot` / `timeslice` | ordinary keywords | `Option<Node<OccurrencePortionKind>>` |
| `#` of a `UsageExtensionKeyword` | syntax | `UsageExtensionKeyword::hash_span` |
| the name after `#` | **reference** (`OwnedFeatureTyping = [QualifiedName]`) | `QualifiedReferenceId` into the document arena — absolute/relative scope, ordered segments and typed separators live there. `QualifiedName` is `::`-separated only, so `#a.b` is not grammatical here and no fixture asserts it; `#Lib::Tag`, `#$::P::Tag` and `#'safety critical'` all are |
| `public`/`private`/`protected` | ordinary keyword on the **membership** | `Membership::visibility` + `Membership::span` |

The extension keyword's name is a reference, never a declaration label and never copied text: the
same `'#' OwnedFeatureTyping` head the crate already owns in
`parser::metadata_annotation::metadata_keyword_head`.

## 4. FIRST tokens and recovery implications

FIRST(`OccurrenceUsagePrefix`) is every token that can open it, and — because the prefix is
optional in its entirety — the FIRST set of any production using it is the union of these with the
FIRST set of what follows the prefix:

```
in  out  inout  derived  abstract  variation  constant  ref  individual  snapshot  timeslice  #
```

Recovery must synchronize on the *first* token of a member, so all twelve are recovery boundaries
in every scope that dispatches a migrated family. Scope starter tables before this seam:

| Table | Already listed | Missing (added by this seam) |
| --- | --- | --- |
| `PACKAGE_BODY_GRAMMAR` | `#`, `abstract`, `constant`, `derived`, `in`, `inout`, `individual`, `out`, `ref`, `snapshot`, `timeslice`, `variation` | — (complete) |
| `PART_BODY_STARTERS` | `#`, `abstract`, `individual`, `ref`, `snapshot`, `timeslice`, `variation` | `constant`, `derived`, `in`, `inout`, `out` |
| `OCCURRENCE_BODY_STARTERS` | see `src/parser/occurrence_body.rs` | `constant`, `derived`, `in`, `inout`, `out`, `variation` |
| `REQUIREMENT_BODY_STARTERS` | `#`, `ref` | `abstract`, `constant`, `derived`, `in`, `individual`, `inout`, `out`, `snapshot`, `timeslice`, `variation` |
| `VIEW_BODY_STARTERS` | — | the full set (satisfy is dispatched there) |
| `VIEW_DEF_BODY_STARTERS` | `abstract`, `ref` | the rest of the set |

Adding a real FIRST token is what stops a malformed member from scanning past a prefix and eating
the valid usage that follows it, exactly as recorded for `assert`/`not` in
`planning/satisfy-requirement-usage-matrix.md` §6.

## 5. Current representation, and the intended one

### 5.1 Before

`OccurrenceUsage` (`src/ast/structure.rs`) carried the prefix as six independent fields with no
spans:

| Field | Grammar slot | Defect |
| --- | --- | --- |
| `direction: Option<InOut>` | `RefPrefix.direction` | no span |
| `is_abstract: bool` | `RefPrefix` alternative | a boolean pair with `is_variation` would be able to hold both; `variation` was simply not parsed |
| `is_constant: bool` | `RefPrefix.isConstant` | no span |
| `is_reference: bool` | `BasicUsagePrefix.isReference` | no span |
| `is_individual: bool` | `isIndividual` | no span |
| `portion_kind: Option<OccurrencePortionKind>` | `PortionKind` | no span |
| — | `RefPrefix.isDerived` | **not represented at all** |
| — | `isVariation` | **not represented at all** |
| — | `UsageExtensionKeyword*` | **not represented at all** |

`SatisfyRequirementUsage` had no prefix representation of any kind, and no `MemberPrefix` either.

The parser spelled the prefix four times — `occurrence_usage`, `individual_usage`,
`snapshot_usage`, `timeslice_usage` in `src/parser/occurrence_body.rs`, plus
`directed_occurrence_usage` for the direction — each accepting a different subset in a different
order (§7).

### 5.2 After

One shared component per production, in `src/ast/occurrence_prefix.rs`:

```rust
pub struct RefPrefix {
    pub direction: Option<Node<InOut>>,
    pub derived_span: Option<Span>,
    pub variance: Option<Node<DefinitionPrefix>>,   // abstract | variation
    pub constant_span: Option<Span>,
}

pub struct BasicUsagePrefix {
    pub ref_prefix: RefPrefix,
    pub reference_span: Option<Span>,
}

pub struct UsageExtensionKeyword {
    pub hash_span: Span,
    pub annotation: QualifiedReferenceId,
}

pub struct OccurrenceUsagePrefix {
    pub basic: BasicUsagePrefix,
    pub individual_span: Option<Span>,
    pub portion: Option<Node<OccurrencePortionKind>>,
    pub extension_keywords: Vec<Node<UsageExtensionKeyword>>,
}
```

Why these shapes:

- **Nested, not flattened.** `RefPrefix` and `BasicUsagePrefix` are separately named productions
  reached by `UsagePrefix` and `ControlNodePrefix` as well, so they are separate types; a future
  migration of either of those neighbours reuses the exact sub-component its production names
  rather than a superset. A single flat struct would have made `ref merge m;` representable.
- **`Option<Span>` rather than `bool` + span.** One representation per syntactic fact: `Some` *is*
  the flag, so a mirror boolean cannot drift from it. This is the same choice
  `SatisfyRequirementUsage::assert_span` already made.
- **`Option<Node<T>>` for the three exclusive slots.** One slot cannot hold two alternatives, so
  `in out`, `abstract variation` and `snapshot timeslice` are unrepresentable rather than
  validated away.
- **`DefinitionPrefix` reused for `abstract | variation`.** `BasicDefinitionPrefix` and
  `RefPrefix`'s third slot are the same two-alternative group written out in two productions, not
  two coincidentally similar shapes; the crate already reuses this enum for
  `AttributeUsage::usage_prefix`, which reaches it through `UsagePrefix → BasicUsagePrefix →
  RefPrefix`.
- **`OccurrencePortionKind` reused, and moved to this module.** It is `PortionKind`; the name is
  kept so the public API does not churn.
- **A dedicated `UsageExtensionKeyword` rather than `MetadataKeywordUsage`.**
  `MetadataKeywordUsage` carries `body: Option<MetadataBody>` because it also models the
  `ExtendedUsage` spelling `#Tag { … }`, which owns a body. `PrefixMetadataMember` owns nothing
  but the reference, so a two-field type makes "a prefix keyword with a body" unrepresentable.
- **`Vec` for the only repeatable slot.** An empty `Vec` performs no heap allocation, so a prefix
  with no extension keyword costs no allocation; a prefix with `n` of them costs one.

`OccurrenceUsage` keeps `is_event`, `is_then` and `has_occurrence_keyword`: `event` and
`occurrence` are kind keywords *after* the prefix, and `then` is a `SourceSuccessionMember`
*before* the membership. None of the three is a prefix slot.

## 6. Legal owning usage families and scopes

Every production that names `OccurrenceUsagePrefix` (or inlines its slots) and where this parser
dispatches it:

| Family | Production | Scopes dispatched from | Migration status |
| --- | --- | --- | --- |
| `OccurrenceUsage`, `IndividualUsage`, `PortionUsage`, `EventOccurrenceUsage` | lines 573–589 | package/namespace/root, `part def`, `part` usage, occurrence body, `action def`, `action` usage, attribute body, `connection def`, connector-nested | **migrated** |
| `SatisfyRequirementUsage` | line 1467 | package/namespace/root, `part def`, `part` usage, occurrence body, `view def`, `view` usage, requirement/concern/viewpoint body | **migrated** |
| `ItemUsage`, `PartUsage`, `PortUsage`, `ViewUsage`, `RenderingUsage`, `ConnectionUsage`, `InterfaceUsage`, `AllocationUsage`, `Message`, `FlowUsage`, `SuccessionFlowUsage` | 616, 624, 646, 1607, 1647, 668, 758, 792, 806, 826, 830 | various | deferred — §9 |
| `ActionUsage`, `CalculationUsage`, `StateUsage`, `ConstraintUsage`, `RequirementUsage`, `ConcernUsage`, `CaseUsage`, `AnalysisCaseUsage`, `VerificationCaseUsage`, `UseCaseUsage`, `PerformActionUsage`, `ExhibitStateUsage`, `IncludeUseCaseUsage`, `AssertConstraintUsage`, `AcceptNode`, `SendNode`, `ActionNodePrefix` | 938–1569 | various | deferred — §9 |
| `MergeNode`, `DecisionNode`, `JoinNode`, `ForkNode` | 973–1010 | action bodies | **not this production** — `ControlNodePrefix` (§1.1) |

### 6.1 Why `OccurrenceUsage` is the representative family

- It is the production the prefix is *named for*: `OccurrenceUsage = OccurrenceUsagePrefix
  'occurrence' Usage`. Anything the abstraction cannot express here it cannot express anywhere.
- Its three sibling productions (`IndividualUsage`, `PortionUsage`, `EventOccurrenceUsage`) inline
  the same slots, so migrating the family exercises **every** alternative of the prefix, including
  the two — `individual` and `PortionKind` — that no other family in the pinned corpus exercises
  together, and that `SatisfyRequirementUsage` does not exercise at all in the corpus.
- The corpus writes prefixed occurrence usages the parser could not parse (§7): `individual
  snapshot`, `individual timeslice`, `in individual`. Migrating fixes real, pinned input.
- It collapses four near-duplicate parsers into one, which is the abstraction's whole claim.
- Its scope coverage (nine dispatch sites) is wide enough to prove the seam is not satisfy-shaped,
  and its AST node is one struct rather than a family of them, so the migration stays reviewable.

`SatisfyRequirementUsage`, by contrast, exercises the prefix in a production whose own head is two
further optional keywords (`assert`, `not`), which is what makes the ordering and recovery
constraints interesting there. The two together cover both shapes.

## 7. Corpus survey

### 7.1 Pinned `sysml-v2-release` corpus (309 `.sysml` files)

Prefix spellings, counted at member position. Counts are occurrences of the prefix, not of the
usage kind that follows.

| Spelling | Count | Representative location | Class before this seam |
| --- | --- | --- | --- |
| `derived` | 198 | `sysml.library/Systems Library/SysML.sysml:28` (`derived item …`) | valid, partly represented (`ItemUsage::is_derived`); **unsupported on every occurrence usage** |
| `individual` | 74 | `Simple Tests/IndividualTest.sysml:30` | valid and represented |
| `snapshot` | 48 | `Simple Tests/OccurrenceTest.sysml` | valid and represented |
| `variation` | 31 | `Variability Examples/VehicleVariabilityModel.sysml:128` | valid, represented on other families; **unsupported on every occurrence usage** |
| `timeslice` | 13 | `training/28. Individuals/Individuals and Time Slices.sysml` | valid and represented |
| `ref individual` | 4 | `training/28. Individuals/Individuals and Time Slices.sysml:10`, `validation/09-Verification/9-Verification-simplified.sysml:101` | valid; **misparsed** (§7.3) |
| `in individual` | 2 | `training/34. Verification/Verification Case Usage Example.sysml:38,46` | valid; **unsupported** — recovered as malformed |
| `individual snapshot` | 2 | `Simple Tests/OccurrenceTest.sysml:9,26` | valid; **unsupported** — recovered as malformed |
| `abstract constant` | 2 | `sysml.library/Domain Libraries/Cause and Effect/CausationConnections.sysml` | valid and represented |
| `individual timeslice` | 1 | `Simple Tests/OccurrenceTest.sysml:25` | valid; **unsupported** — recovered as malformed |
| `derived constant` | 1 | `sysml.library/Systems Library/…` | valid; **unsupported on occurrence usages** |
| `constant` | 1 | `sysml.library/…` | valid and represented |
| `#Tag` before a usage | many | `Vehicle Example/…`, `14c-Language Extensions.sysml` | valid; parsed as a **separate sibling member**, not as this usage's prefix |

No `out`/`inout` occurrence-usage prefix, no `abstract variation`, and no invalid ordering occurs
anywhere in the pinned corpus. No malformed occurrence-usage prefix occurs in it either.

### 7.2 Checked-in fixtures, tests and snapshots

| Spelling | Where | Class |
| --- | --- | --- |
| `snapshot <name>`, `timeslice <name>` | `tests/snapshots/sysml/occurrence_body_members.md`, `tests/gh90_individual_timeslice.rs`, `tests/validation/**` | valid and represented |
| `individual`, `individual occurrence`, `ref individual` | `tests/gh90_individual_timeslice.rs`, `tests/validation/surveillance_drone.rs` | valid and represented |
| `abstract constant ref occurrence causes[1..*]` | `tests/snapshots/sysml/…`, GH-51 regression | valid and represented |
| `in occurrence terminatedOccurrence[1]` | Systems Library `Actions.sysml`, exercised by the library gates | valid and represented (via `directed_occurrence_usage`) |
| `derived`, `variation`, `#Tag` on an occurrence usage | **absent** before this seam | the gap it closes; covered by the four fixtures in §10 |

### 7.3 Classification of every spelling found

| Class | Spellings |
| --- | --- |
| valid and currently represented exactly | `snapshot X`, `timeslice X`, `individual X`, `individual occurrence X`, `ref individual X` *(bare, no kind keyword)*, `abstract X`, `constant X`, `abstract constant ref occurrence X`, `in occurrence X`, `event occurrence X`, `then timeslice X` |
| valid but partially represented | every one of the above: no keyword span was retained, so provenance was lost and the emitter re-derived the keyword from a boolean |
| valid but parsed and discarded | none in this production — the discarded facts were not parsed at all |
| valid but unsupported | `derived …`, `variation …`, `in individual …`, `individual snapshot …`, `individual timeslice …`, `in abstract occurrence …`, `constant snapshot …`, `#Tag <occurrence usage>` (parsed as a sibling, not a prefix), and every prefix at all on a `SatisfyRequirementUsage` |
| malformed | none in the pinned corpus. Constructed for coverage: `abstract abstract`, `snapshot timeslice`, `in out`, `ref in`, `individual ref`, `constant derived`, `#`, `#::`, `derived;` |
| permissive legacy syntax not supported by the pinned grammar | `ref individual item :>> driver : Alice;` was **accepted with the wrong shape** — `item` is `ItemUsage`'s kind keyword, and the occurrence parser took it as the declaration name, re-emitting `ref individual 'item' : Alice :>> driver;`. That is corpus input (`training/28. Individuals/Individuals and Time Slices.sysml:10`) silently changed by a round trip. Now an `ItemUsage` with a `ref individual` prefix, which is why `ItemUsage` is migrated here and not deferred: refusing the wrong reading is only an improvement if the family that owns `item` can then claim it |

## 8. Recovery contract

| Input | Required outcome |
| --- | --- |
| malformed member before any prefix starter | recovery stops at the *first* prefix token, not at the kind keyword; the prefixed usage after it parses |
| malformed content between prefix components (`in /* c */ derived occurrence o;`) | comments are trivia between slots and the usage parses; `in @ derived occurrence o;` is one recovery node |
| invalid ordering (`ref abstract occurrence o;`, `occurrence individual o;`) | refused; one recovery node retaining the exact malformed span. Never reinterpreted as a valid unprefixed usage |
| duplicate mutually exclusive modifiers (`abstract variation`, `in out`) | refused; recovery node |
| duplicate portion kinds (`snapshot timeslice`, `snapshot snapshot`) | refused; recovery node |
| missing usage after a valid prefix (`in derived ref;`) | refused; recovery node covering the prefix and its terminator |
| incomplete extension keyword (`# occurrence o;`, `#;`) | refused; recovery node. No fabricated reference |
| malformed qualified extension keyword (`#A:: occurrence o;`) | refused; recovery node, and the speculative reference allocated for `A` rolls back |
| malformed prefixed satisfy usage followed by valid siblings | siblings survive |
| malformed prefixed occurrence usage followed by several valid siblings | all survive |
| nested brace bodies after a prefixed usage; unmatched braces | balanced tracking unchanged from the body container |
| prefix-like words inside strings, quoted names, escaped text, line and block comments | not prefix tokens |
| strict/editor equivalence on diagnostic-free input | identical documents |
| speculative parse that consumes extension-keyword references then fails | arena unchanged — the whole prefix parse runs inside `reference_transaction` |

Recovery never fabricates a prefix component, and a refused prefix never becomes an unprefixed
usage: the refusal is at the whole production, so the member becomes one recovery node spanning
the authored text.

## 8.1 Dispatch precedence, and what the implementation found

Two of the prefix's FIRST tokens also head a *different* production that several scopes dispatch
before the migrated families, and neither of those parsers knows about the kind keyword that
follows:

| Token | Competing production | What happened before | Now |
| --- | --- | --- | --- |
| `#` | `PrefixMetadataMember` as a standalone sibling member (`metadata_keyword_prefix`) | `#Tag occurrence o;` became two sibling members, leaving the usage unprefixed | one member, with the tag in its prefix |
| `ref` | `ReferenceUsage = ( EndUsagePrefix \| RefPrefix ) 'ref' Usage` (`ref_decl` / `part_ref_usage` / `action_ref_decl`) | `derived ref item x;` became an anonymous `ReferenceUsage` named `item`, and `ref individual snapshot satisfy R;` one named `individual` — silently, with no diagnostic | the migrated family claims it |

`parser::occurrence_prefix::starts_contended_prefix` scans the prefix slots that head no competing
production (`in`, `out`, `inout`, `derived`, `abstract`, `variation`, `constant`, `individual`,
`snapshot`, `timeslice`) and reports whether the run reaches a `ref` or a `#`. Only then do the
migrated families get first refusal, so every scope's `*_def`-before-`*_usage` ordering is
untouched — `abstract item items : Item[0..*] nonunique :> objects { … }` still reaches
`item_def`, which is the PAR-001 bug class a blanket reordering would have reopened.

Three further findings from the migration, each fixed here:

- a `connection def` body could not *emit* an occurrence usage at all, so one legal owning scope of
  a migrated family had no round trip;
- `ConnectionDefBodyElement::OccurrenceUsage` and `PartUsageBodyElement::OccurrenceUsage`
  projected as contentless markers, as did `ItemUsage` in every scope, so a snapshot could not tell
  a prefixed usage from a bare one;
- an anonymous declaration (`ref individual :>> driver : Alice;`) left the keyword's trailing space
  stranded in front of a clause that supplies its own, emitting `ref individual  :>> driver`.

Two remain, both pre-existing and both outside this production:

- `ViewDef` projects as a contentless `(view-def)` marker in every scope, so a satisfy usage inside
  a `view def` body is covered by that fixture's `FORMAT` and `DIAGNOSTICS` sections rather than by
  its `AST` section. Extending that projection is a `ViewDef` change, not a prefix change;
- `ConnectionDefBodyElement::SuccessionUsage` still has no emitter.

## 9. Migration status, family by family

Migrated by this change — no legacy prefix field or parser path remains for either:

| Family | AST type | What moved |
| --- | --- | --- |
| `OccurrenceUsage` / `IndividualUsage` / `PortionUsage` / `EventOccurrenceUsage` | `ast::OccurrenceUsage` | `direction`, `is_abstract`, `is_constant`, `is_reference`, `is_individual`, `portion_kind` deleted; `prefix: OccurrenceUsagePrefix` added, with `derived`, `variation` and `UsageExtensionKeyword*` newly represented |
| `SatisfyRequirementUsage` | `ast::SatisfyRequirementUsage` | `prefix: OccurrenceUsagePrefix` and `membership: Membership` added; nothing deleted, because nothing was there |

Deliberately **not** migrated by this change. None of these has moved; each keeps whatever partial
prefix fields it already had, and closing each is a separate, family-sized change that reuses the
component this one defines:

| Family | Prefix facts it already models | Still missing |
| --- | --- | --- |
| `ItemUsage` | `is_derived`, `usage_prefix`, `is_individual`, direction | spans, `constant`, `ref`, `PortionKind`, extension keywords |
| `PartUsage` | `is_abstract`, `is_variation`, `is_reference`, `is_individual` | spans, direction, `derived`, `constant`, `PortionKind`, extension keywords |
| `PortUsage` | direction, `is_abstract`, `is_reference`, `is_individual` | spans, `derived`, `variation`, `constant`, `PortionKind`, extension keywords |
| `ActionUsage` | `is_abstract`, `is_variation`, `is_reference`, `is_individual` | spans, direction, `derived`, `constant`, `PortionKind`, extension keywords |
| `StateUsage`, `CalcUsage`, `ConstraintUsage`, `RequirementUsage`, `ConcernUsage`, `CaseUsage`, `AnalysisCaseUsage`, `VerificationCaseUsage`, `UseCaseUsage` | varying subsets of `abstract`/`variation`/`ref`/`individual`/direction | spans, and the slots each does not carry |
| `ViewUsage`, `RenderingUsage`, `ConnectionUsage`, `InterfaceUsage`, `AllocationUsage`, `FlowUsage`, `Message`, `SuccessionFlowUsage`, `PerformActionUsage`, `ExhibitStateUsage`, `IncludeUseCaseUsage`, `AssertConstraintUsage`, `AcceptNode`, `SendNode` | little or none | the whole prefix |
| `MergeNode`, `DecisionNode`, `JoinNode`, `ForkNode` | none | **`ControlNodePrefix`, not this production** — needs its own `RefPrefix`-rooted component, which §5.2's nesting already provides |

## 10. Coverage

| Evidence | What it pins |
| --- | --- |
| `tests/snapshots/sysml/occurrence_usage_prefix_alternatives.md` | every slot alone, the full legal order, materially different combinations, all three families, both keyword-less spellings, both body forms, extension keywords qualified/absolute/quoted, `MemberPrefix` visibility beside the prefix |
| `tests/snapshots/sysml/occurrence_usage_prefix_owning_scopes.md` | a materially different prefix in every scope a migrated family is dispatched from |
| `tests/snapshots/sysml/occurrence_usage_prefix_recovery.md` | malformed content before every FIRST token and between two prefix components; invalid orderings, both exclusive pairs, a repeated portion kind, a prefix with no usage, a malformed extension keyword; prefix words inside a quoted name, a string literal and both comment forms; a valid sibling after every case |
| `tests/snapshots/sysml/occurrence_usage_prefix_unterminated.md` | the unmatched-brace state, which subsumes every other member and therefore needs its own fixture |
| `tests/occurrence_usage_prefix_owning_layer.rs` | arena rollback after refused speculation, strict/editor equivalence, parse→format→reparse and format idempotence per slot, and envelope rejection of a wrong-token span, a wrong alternative, out-of-order slots, a non-`#` sigil, a dangling extension identity, and a keyword belonging to another member |
| `tests/snapshots/spec42/**` | the pinned corpus files this seam changed: `28_individuals_and_time_slices.md`, `john_individual_example.md`, `6_individual_and_snapshots.md`, `9_verification_simplified.md`, `14c_language_extensions.md`, `fuzz_individual_direction_prefix.md` and the action-body fixtures, each losing a diagnostic or a corrupted round trip |

The safe continuation path for an unmigrated family is: confirm from the pin which of
`OccurrenceUsagePrefix` / `UsagePrefix` / `ControlNodePrefix` its production names; give the AST
type that component in place of its current fields; route its parser through
`parser::occurrence_prefix` (or the sibling entry point for the other two productions); delete the
superseded fields, parser branches and emitter logic; and add the scope's missing FIRST tokens
from §4.
