# `PartUsage` occurrence-prefix matrix

The `PartUsage` slice of the shared `OccurrenceUsagePrefix` seam, classified against the pinned
grammar, with every spelling found in the checked-in fixtures and in the pinned
`sysml-v2-release` corpus. The pin is `docs/conformance-target` (`release_tag=2026-04`,
`grammar_content_hash=fnv1a64:95f39e912f73b917`).

This document is the per-family audit `planning/occurrence-usage-prefix-matrix.md` §10 asks for
before a deferred family moves:

> `PartUsage` is the recommended next audit candidate, not a pre-approved mechanical replacement.
> It is central in the corpus and the pinned production names `OccurrenceUsagePrefix` directly,
> but its matrix extension must still enumerate every construction path, owning scope, competing
> `ref`/`#` dispatch, body form, consumer, and recovery boundary before production edits.

The component itself -- what `OccurrenceUsagePrefix`, `BasicUsagePrefix`, `RefPrefix`,
`UsageExtensionKeyword` are, why they nest, and which neighbouring prefixes are deliberately *not*
this production -- is audited once in `planning/occurrence-usage-prefix-matrix.md` §§1-5 and is
not restated here. This document audits only what is specific to `PartUsage`: its production, its
owning scopes, its construction paths, its dispatch hazards, its corpus, and its recovery
contract.

## 1. Authoritative production

Verbatim from `sysml-v2-release/bnf/SysML-textual-bnf.kebnf` at the pin.

```text
PartDefinition =
    OccurrenceDefinitionPrefix 'part' 'def' Definition        -- line 620, clause 8.2.2.11

PartUsage =
    OccurrenceUsagePrefix 'part' Usage                        -- line 623, clause 8.2.2.11
```

and everything `Usage` reaches in this context:

```text
Usage =
    UsageDeclaration UsageCompletion                          -- line 305

UsageDeclaration : Usage =
    Identification FeatureSpecializationPart?                 -- line 308

UsageCompletion : Usage =
    ValuePart? UsageBody                                      -- line 311

UsageBody : Usage =
    DefinitionBody                                            -- line 314

ValuePart : Feature =
    ownedRelationship += FeatureValue                         -- line 317

FeatureValue =
    ( '=' | isInitial ?= ':=' | isDefault ?= 'default' ( '=' | isInitial ?= ':=' )? )
    ownedRelatedElement += OwnedExpression                    -- line 319

Identification : Element =
    ( '<' declaredShortName = NAME '>' )? ( declaredName = NAME )?   -- clause 8.2.2.2

FeatureSpecializationPart : Feature =
      FeatureSpecialization+ MultiplicityPart? FeatureSpecialization*
    | MultiplicityPart FeatureSpecialization*

MultiplicityPart : Feature =
    ownedRelationship += OwnedMultiplicity
    | ( ownedRelationship += OwnedMultiplicity )?
      ( isOrdered ?= 'ordered' ( isNonunique ?= 'nonunique' )?
      | isNonunique ?= 'nonunique' ( isOrdered ?= 'ordered' )? )

FeatureSpecialization : Feature =
    Typings | Subsettings | References | Crossings | Redefinitions | Intersectings

DefinitionBody : Type =
    ';' | '{' DefinitionBodyItem* '}'

MemberPrefix : Membership =
    ( visibility = VisibilityIndicator )?                     -- line 130
```

and the two membership productions that carry a `PartUsage` into a body:

```text
DefinitionBodyItem : Type =
      ownedRelationship += DefinitionMember
    | ownedRelationship += VariantUsageMember
    | ownedRelationship += NonOccurrenceUsageMember
    | ( ownedRelationship += SourceSuccessionMember )?
      ownedRelationship += OccurrenceUsageMember              -- line 240 region
    | ownedRelationship += AliasMember
    | ownedRelationship += Import

OccurrenceUsageMember : FeatureMembership =
    MemberPrefix ownedRelatedElement += OccurrenceUsageElement    -- line 259

OccurrenceUsageElement : Usage = StructureUsageElement | BehaviorUsageElement   -- line 353
StructureUsageElement : Usage = … | PartUsage | …                               -- line 363
VariantUsageElement  : Usage = … | PartUsage | …                               -- line 403
PackageMember : OwningMembership =
    MemberPrefix ( ownedRelatedElement += DefinitionElement
                 | ownedRelatedElement = UsageElement )                        -- line 133
```

### 1.1 `PartUsageKeyword` does not exist in the pin

The pin writes the kind keyword as the literal `'part'` inside `PartUsage`. There is no
`PartUsageKeyword` production in `SysML-textual-bnf.kebnf` at `release_tag=2026-04`
(`grep -c 'PartUsageKeyword'` is 0). The sibling Pilot grammar *does* name it — see §8 — but the
two are the same single token, so nothing in this slice depends on the difference.

### 1.2 Conformance identity

| Fact | Value |
| --- | --- |
| Release tag | `2026-04` |
| Grammar content hash | `fnv1a64:95f39e912f73b917` |
| SysML productions | 350 |
| KerML productions | 290 |
| `PartUsage` line | 623 |
| `OccurrenceUsagePrefix` line | 564 |

## 2. Nesting, cardinality and legal token order

`PartUsage` adds nothing to the prefix; it is `OccurrenceUsagePrefix` verbatim, then one keyword,
then `Usage`. The full slot table is
`planning/occurrence-usage-prefix-matrix.md` §2; repeated here only as the legal order this family
must accept and no other:

```
[public|private|protected]                       -- MemberPrefix, on the membership, NOT the prefix
[in|out|inout] [derived] [abstract|variation] [constant] [ref] [individual]
[snapshot|timeslice] ('#' QualifiedName)*        -- OccurrenceUsagePrefix
'part'                                           -- the kind keyword
[<short>] [name] [FeatureSpecializationPart]     -- UsageDeclaration
[= expr | := expr | default …] (';' | '{' … '}') -- UsageCompletion
```

Cardinality and exclusivity, per slot:

| Slot | Cardinality | Kind |
| --- | --- | --- |
| `in` / `out` / `inout` | 0..1 | one slot, three alternatives — mutually exclusive |
| `derived` | 0..1 | independent |
| `abstract` / `variation` | 0..1 | one slot, two alternatives — mutually exclusive |
| `constant` | 0..1 | independent |
| `ref` | 0..1 | independent |
| `individual` | 0..1 | independent |
| `snapshot` / `timeslice` | 0..1 | one slot, two alternatives — mutually exclusive |
| `'#' QualifiedName` | 0..* | ordered, repeatable, authored order retained |

Everything else repeated is malformed: `abstract abstract`, `in out`, `snapshot timeslice`,
`ref ref`, `individual individual`. The grammar gives no slot for the second token, so the
member is refused, not "last wins".

`then` (`SourceSuccessionMember`) precedes `OccurrenceUsageMember` and therefore precedes both
the visibility keyword and the prefix. It is **not** a prefix slot and is not modelled on
`PartUsage` — see §11.

## 3. Legal owning scopes

Every scope this parser dispatches a `PartUsage` from, the AST enum that owns it, and the
recovery starter table that scope synchronizes on.

| # | Scope | Owning AST enum | Parser entry | Recovery starters |
| --- | --- | --- | --- | --- |
| 1 | package / namespace / root | `PackageBodyElement::PartUsage` | `part::part_def_or_usage` | `PACKAGE_BODY_GRAMMAR` |
| 2 | `part def` body | `PartDefBodyElement::PartUsage` | `part::part_usage` | `PART_BODY_STARTERS` |
| 3 | `part` usage body | `PartUsageBodyElement::PartUsage` | `part::part_usage` | `PART_BODY_STARTERS` |
| 4 | attribute / item def / item usage body | `AttributeBodyElement::PartUsage` | `part::part_usage` | `ATTRIBUTE_BODY_STARTERS` |
| 5 | metadata def / usage body | `AttributeBodyElement::PartUsage` | `part::part_usage` | `METADATA_BODY_STARTERS` |
| 6 | `perform` body | `PerformBodyElement::PartUsage` | `part::part_usage` | — (no recovery table; §10.3) |
| 7 | `connection def` body | `ConnectionDefBodyElement::PartUsage` | `part::part_usage` | `CONNECTION_DEF_BODY_STARTERS` |
| 8 | occurrence body | `OccurrenceBodyElement::PartUsage` | `part::part_usage` | `OCCURRENCE_BODY_STARTERS` |
| 9 | `use case def` body | `UseCaseDefBodyElement::PartUsage` | `part::part_usage` | `USE_CASE_BODY_STARTERS` |
| 10 | `calc def` / KerML type body | `CalcDefBodyElement::PartUsage` | `part::part_usage` | `CALC_DEF_BODY_STARTERS` |
| 11 | `action def` body | `ActionDefBodyElement::PartUsage` | `part::part_usage` | `ACTION_BODY_STARTERS` |
| 12 | `action` usage body | `ActionUsageBodyElement::PartUsage` | `part::part_usage` | `ACTION_BODY_STARTERS` |
| 13 | `variant` member | `VariantTypedUsage::Part` | `part::variant_usage` → `part::part_usage` | owner's table |

Scope 10 is narrower than the others by construction: `calc_def_body_element` only reaches
`part_usage` behind a `starts_with_keyword(in|out|inout)` gate, so a bare `part p;` in a
calculation body is not dispatched at all. That is a pre-existing dispatch gap in a *different*
production's scope selector, recorded in §11 rather than widened here.

## 4. Construction paths

Every place a `ast::PartUsage` value is built, before this slice.

| Path | File | What prefix it parsed | Gap |
| --- | --- | --- | --- |
| `part_usage` | `src/parser/part/usage.rs` | direction, `derived`, `abstract\|variation`, `constant`, `ref`, `individual` | no spans; no `snapshot`/`timeslice`; no `UsageExtensionKeyword` |
| `part_def_or_usage` (usage branch) | `src/parser/part/def.rs` | `abstract\|variation`, `ref`, `individual` only | additionally no direction, no `derived`, no `constant` |
| `part_usage_named` | `src/parser/part/usage.rs` | none — all six fields defaulted, then overwritten by the caller | — |
| `part_usage_redefines_only` | `src/parser/part/usage.rs` | none — same | — |
| `anonymous_part_usage` | `src/parser/part/usage.rs` | none — same | — |

The last three are tail parsers: each built a `PartUsage` with the six prefix fields at their
`false`/`None` defaults and each of the two heads then assigned over them field by field, in three
places per head. That is six writes per branch times four branches — the shape that made
`part_def_or_usage` silently disagree with `part_usage` about which slots exist.

After this slice the tails take the parsed `OccurrenceUsagePrefix` as a parameter, so a head
cannot forget a slot: there is one field and one assignment, made at construction.

`part_ref_usage` (`ReferenceUsage`) and `exhibit_state`/`exhibit_state_as_state_usage`
(`ExhibitStateUsage` → `StateUsage`) also read `RefPrefix`-shaped modifiers, but neither
constructs a `PartUsage`; they are different productions and stay on their own representations.

## 5. Current AST representation, and the intended one

### 5.1 Before

`ast::PartUsage` (`src/ast/structure.rs`) carried six independent fields with no spans:

| Field | Grammar slot | Defect |
| --- | --- | --- |
| `usage_prefix: Option<DefinitionPrefix>` | `RefPrefix`'s `abstract`/`variation` | no span |
| `is_individual: bool` | `isIndividual` | no span |
| `is_reference: bool` | `BasicUsagePrefix.isReference` | no span |
| `direction: Option<InOut>` | `RefPrefix.direction` | no span |
| `is_derived: bool` | `RefPrefix.isDerived` | no span |
| `is_constant: bool` | `RefPrefix.isConstant` | no span |
| — | `PortionKind` | **not represented at all** |
| — | `UsageExtensionKeyword*` | **not represented at all** |

### 5.2 After

```rust
pub struct PartUsage {
    pub prefix: crate::ast::OccurrenceUsagePrefix,
    // …unchanged Usage tail: name, short_name, typing, multiplicity, ordered, nonunique,
    //  subsets, redefines, value, body, name_span, type_ref_span, membership
}
```

One field, the shared component, exactly as `OccurrenceUsage`, `ItemUsage` and
`SatisfyRequirementUsage` already carry it. No part-specific mirror, no getters reconstructing the
six booleans: `planning/occurrence-usage-prefix-matrix.md` §5.2 records why each sub-shape is what
it is, and reusing it verbatim is the point of the seam.

`PartUsage`'s hand-written `PartialEq` (which deliberately ignores `name_span`/`type_ref_span`)
keeps that behavior and compares `prefix` where it compared the six fields.

`membership` stays outside the prefix: `MemberPrefix`'s visibility belongs to the
`OccurrenceUsageMember`/`PackageMember`, not to the usage.

## 6. FIRST tokens and recovery implications, per scope

FIRST(`PartUsage`) is FIRST(`OccurrenceUsagePrefix`) ∪ {`part`}, because the whole prefix is
optional:

```
in  out  inout  derived  abstract  variation  constant  ref  individual  snapshot  timeslice  #  part
```

All thirteen are recovery boundaries in every scope of §3. Scope starter tables before this slice:

| Table | Already listed | Missing (added by this slice) |
| --- | --- | --- |
| `PACKAGE_BODY_GRAMMAR` | all thirteen | — (complete) |
| `PART_BODY_STARTERS` | all thirteen | — (complete, from the occurrence seam) |
| `OCCURRENCE_BODY_STARTERS` | all thirteen | — (complete, from the occurrence seam) |
| `ATTRIBUTE_BODY_STARTERS` | `#`, `abstract`, `derived`, `part`, `ref` | `constant`, `in`, `individual`, `inout`, `out`, `snapshot`, `timeslice`, `variation` |
| `METADATA_BODY_STARTERS` | `#`(via `@`/`#` handling), `abstract`, `derived`, `part`, `ref` | `constant`, `in`, `individual`, `inout`, `out`, `snapshot`, `timeslice`, `variation` |
| `CONNECTION_DEF_BODY_STARTERS` | `ref` | `part` and the remaining eleven |
| `USE_CASE_BODY_STARTERS` | `abstract`, `in`, `out`, `part`, `ref` | `#`, `constant`, `derived`, `individual`, `inout`, `snapshot`, `timeslice`, `variation` |
| `ACTION_BODY_STARTERS` | `#`, `in`, `out`, `part`, `ref`, `snapshot`, `variation` | `abstract`, `constant`, `derived`, `individual`, `inout`, `timeslice` |
| `CALC_DEF_BODY_STARTERS` | `in`, `inout`, `out`, `part` | — (the scope's dispatch gate is `in`/`out`/`inout` only; §11) |

Adding a real FIRST token is what stops a malformed member from scanning past a prefix and eating
the valid usage that follows it, exactly as recorded for `assert`/`not` in
`planning/satisfy-requirement-usage-matrix.md` §6 and for the occurrence prefix in
`planning/occurrence-usage-prefix-matrix.md` §4.

## 7. Competing productions and parser-precedence hazards

Six productions share a FIRST token with a prefixed `PartUsage`.

| Token | Competing production | Where it wins today | Resolution |
| --- | --- | --- | --- |
| `part` | `PartDefinition` (`… 'part' 'def' Definition`) | every scope tries `part_def`/`part_def_or_usage`'s `def` branch first | unchanged: `part_usage` explicitly refuses a following `def` keyword, and `part_def_or_usage` checks for `def` before falling through to the usage tails. `ref part def …` stays a refusal in the definition branch (a definition prefix has no `ref`) and is claimed by the usage branch, which then refuses on `def` |
| `#` | `PrefixMetadataMember` as a standalone sibling member (`metadata_keyword_prefix`), and `ExtendedDefinition`/`ExtendedUsage` (`metadata_keyword_usage`) | `metadata_keyword_prefix` claimed `#logical` and left `part vehicleLogical : Vehicle { … }` as a separate unprefixed member | `part_usage` gets first refusal through `occurrence_prefix::starts_contended_prefix`, exactly as `occurrence_usage`/`item_usage`/`satisfy` already do. `metadata_keyword_usage` (the `#Tag { … }`/`#Tag def …` spellings, which own a body or a definition) keeps its place ahead of the prefix walk in the scopes that try it first |
| `ref` | `ReferenceUsage` = `(EndUsagePrefix \| RefPrefix) 'ref' Usage` (`part_ref_usage`), and this parser's kinded `ref_decl` (`connector::ref_decl`, which models `ref part x;` as a `RefDecl` with `kind_keyword = Part`) | scope-dependent: `part_usage` wins in part/occurrence/action bodies; `connector::ref_decl` wins in attribute, metadata and use-case bodies | `part_usage` gets first refusal in the contended pre-dispatch of every scope in §3. `part_ref_usage` already refuses `ref part …` explicitly, so only the kinded `connector::ref_decl` changes hands, and only for the `part` kind |
| `in`/`out`/`inout` | `in_out_decl` (directed parameter member) | `in_out_decl` refuses the kinded forms (`in part`, `in item`, `in occurrence`, …) itself | unchanged |
| `abstract`/`variation` | every `*_def` parser's `BasicDefinitionPrefix` | each scope's `*_def` arm precedes its `*_usage` sibling | unchanged. `starts_contended_prefix` deliberately reports a run of *uncontended* slots as contended only when it reaches `ref` or `#`, so `abstract part def P { … }` still reaches `part_def` — the PAR-001 bug class a blanket reorder would reopen |
| `individual`/`snapshot`/`timeslice` | `IndividualUsage`/`PortionUsage` (the keyword-less occurrence spellings, `occurrence_usage`) | `occurrence_usage` refuses when the next word is a reserved keyword (`next_word_is_reserved`), so `individual part p;` is left to the family that owns `part` | unchanged, and now load-bearing for `snapshot part …`: without that guard the portion spelling would read `part` as its declaration name |

The rule this slice keeps: **the selected `PartUsage` parser claims a prefix only when the whole
production is viable.** `part_usage` parses the prefix and then requires the `part` keyword; if it
is absent the parse fails, the reference transaction rolls back every `#tag` identity it
allocated, and the member falls through to whichever sibling really owns it.

## 8. Sibling Pilot comparison

`../SysML-v2-Pilot-Implementation/org.omg.sysml.xtext/src/org/omg/sysml/xtext/SysML.xtext`, which
is newer than this repository's 2026-04 pin.

| Production | Pilot | Pin | Material? | Followed |
| --- | --- | --- | --- | --- |
| `PartUsage` | `OccurrenceUsagePrefix PartUsageKeyword Usage` (939) | `OccurrenceUsagePrefix 'part' Usage` (623) | no — `PartKeyword : 'part'`, `PartUsageKeyword : PartKeyword`, one token either way | pin |
| `OccurrenceUsagePrefix` | `( EndUsagePrefix \| BasicUsagePrefix ( 'individual' )? ( PortionKind )? ) UsageExtensionKeyword*` (836) | `BasicUsagePrefix ( 'individual' )? ( PortionKind )? UsageExtensionKeyword*` (564) | **yes** — Pilot admits `end` as an alternative to the whole basic prefix, so `end part p;` is grammatical there and is not at the pin | **pin.** `end` is not accepted on a part usage by this slice, and `EndUsagePrefix` remains modelled only where the pin reaches it (`UsagePrefix`, i.e. `AttributeUsage::is_end`, `EnumerationUsage::is_end`) |
| `RefPrefix`, `BasicUsagePrefix`, `PortionKind`, `UsageExtensionKeyword`, `PrefixMetadataMember` | identical to the pin (556-589, 576) | 275-296, 1660 | no | both |
| `Usage` | `UsageDeclaration? UsageCompletion` (591) | `UsageDeclaration UsageCompletion` (305) | no — the pin's `UsageDeclaration` is `Identification FeatureSpecializationPart?` and `Identification` is itself all-optional, so both admit the same empty declaration | both |
| `ActorUsage` / `StakeholderUsage` | `'actor' UsageExtensionKeyword* Usage` returning `SysML::PartUsage` (2094, 2103) | `ActorUsage : PartUsage = 'actor' UsageExtensionKeyword* Usage` (1451, 1457) | no difference between them, but note both give the *abstract metaclass* `PartUsage` a textual production with **no** `OccurrenceUsagePrefix` — only `UsageExtensionKeyword*` | this parser models `ActorUsage`/`StakeholderMember` as their own AST types, not as `ast::PartUsage`, so they are neither construction paths for this slice nor affected by it |

No Pilot behaviour newer than the pin is adopted. The one material difference (`EndUsagePrefix`
inside `OccurrenceUsagePrefix`) is recorded and deliberately not followed; adopting it would be a
conformance-pin change, not a family migration.

## 9. Corpus survey

### 9.1 Pinned `sysml-v2-release` corpus (309 `.sysml` files)

Every prefix run written immediately before a `part` kind keyword that is not `part def`, counted
at member position by `scripts`-free scan over comment- and string-stripped source. `#Ref` runs
are normalized to `#`.

| Spelling | Count | Representative location | Class before this slice |
| --- | --- | --- | --- |
| *(no prefix)* | 970 | `sysml.library/Domain Libraries/Geometry/SpatialItems.sysml:50` | valid and represented |
| `ref` | 27 | `sysml.library/Systems Library/Cases.sysml:23` (`ref part actors : Part[0..*] :> parts {`) | valid, partly represented (no span); **claimed by `connector::ref_decl` instead in attribute/metadata/use-case bodies** |
| `variation` | 14 | `sysml/src/examples/Simple Tests/VariabilityTest.sysml:15` | valid, partly represented (no span) |
| `abstract` | 12 | `sysml.library/Systems Library/Items.sysml:97` | valid, partly represented (no span) |
| `in` | 10 | `sysml/src/examples/Simple Tests/TradeStudyTest.sysml:14` (`in part : Engine;` in a `calc` body) | valid, partly represented (no span); **unsupported at package scope** |
| `individual` | 6 | `sysml/src/examples/Individuals Examples/AnalysisIndividualExample.sysml:81` | valid, partly represented (no span) |
| `snapshot` | 4 | `sysml/src/training/28. Individuals/Individuals and Roles-1.sysml:14` | valid; **unsupported** — recovered as malformed |
| `out` | 1 | `sysml/src/examples/Simple Tests/ActionTest.sysml:31` (`out part target;`) | valid, partly represented (no span) |
| `#` | 1 | `sysml/src/examples/Vehicle Example/SysML v2 Spec Annex A SimpleVehicleModel.sysml:487` (`#logical part vehicleLogical:Vehicle{`) | valid; **parsed as a separate sibling member**, not as this usage's prefix |

Not present anywhere in the pinned corpus on a part usage: `inout`, `derived`, `constant`,
`timeslice`, any two-slot combination beyond the singletons above, any invalid ordering, any
duplicate modifier, any malformed prefix.

One further pinned spelling is adjacent but is **not** a prefix:
`then snapshot part vehicle_1_t1 { … }`
(`sysml/src/training/28. Individuals/Individuals and Roles-1.sysml:18`). `then` is
`SourceSuccessionMember`, which precedes the membership; see §11.

### 9.2 Checked-in fixtures, tests and snapshots

| Spelling | Where | Class |
| --- | --- | --- |
| *(no prefix)* | everywhere | valid and represented |
| `ref part` | `tests/snapshots/spec42/sysml/examples/coverage_connectors.md`, `tests/snapshots/sysml/part_ref_trailing_redefinition.md`, `tests/parser/structure.rs` | valid, partly represented |
| `variation part` | `tests/snapshots/spec42/sysml/training/36_variation_usages.md` | valid, partly represented |
| `abstract part` | `tests/snapshots/spec42/sysml/training/36_variation_usages.md` | valid, partly represented |
| `in part` | `tests/snapshots/spec42/sysml/examples/trade_study_test.md` | valid, partly represented |
| `individual part` | `tests/snapshots/spec42/sysml/coverage_individual.md` | valid, partly represented |
| `snapshot part` | `tests/snapshots/spec42/sysml/training/28_individuals_and_roles_1.md` | **malformed today** — the fixture pins the recovery diagnostic |
| `derived abstract constant ref part` | `tests/snapshots/sysml/ref_prefix_coverage.md:24` | valid, partly represented — the emitted order was the only evidence |
| `timeslice part`, `#Tag part`, `individual snapshot part`, `out part`, `inout part` | **absent** | the gaps this slice closes; covered by the four fixtures in §12 |

### 9.3 Classification of every spelling found or constructed

| Class | Spellings |
| --- | --- |
| valid and represented exactly | *(no prefix)* `part x;` in every scope |
| valid but partially represented | `in`/`out`/`inout`, `derived`, `abstract`, `variation`, `constant`, `ref`, `individual` and every combination of them — the keyword was accepted but no authored span was retained, so the emitter re-derived the keyword from a boolean and no snapshot could show provenance |
| valid but parsed and discarded | none in this production |
| valid but unsupported | `snapshot part …`, `timeslice part …`, `individual snapshot part …`, `individual timeslice part …`, `#Tag part …` (parsed as a sibling, not a prefix), and — at package scope only — `in`/`out`/`inout part …`, `derived part …`, `constant part …` |
| malformed | none in the pinned corpus. Constructed for coverage: `ref derived part`, `in out part`, `abstract variation part`, `abstract abstract part`, `snapshot timeslice part`, `snapshot snapshot part`, `ref ref part`, `individual individual part`, `in derived ref;` (prefix with no `part`), `part` with no declaration, `# part`, `#A:: part`, `#$:: part` |
| permissive legacy syntax not supported by the pin | `end part …` — Pilot-only (§8), refused here |

## 10. Recovery contract

| Input | Required outcome |
| --- | --- |
| malformed member before any prefix FIRST token | recovery stops at the *first* prefix token, not at `part`; the prefixed usage after it parses |
| malformed content between two prefix slots (`in @ derived part p;`) | one recovery node retaining the exact malformed span |
| a comment between two prefix slots (`in /* why */ derived part p;`) | trivia; the usage parses |
| invalid ordering (`ref derived part p;`, `individual ref part p;`, `part individual p;`) | refused; one recovery node retaining the exact malformed span. Never reinterpreted as a valid unprefixed usage |
| duplicate direction (`in out part p;`) | refused; recovery node |
| `abstract variation part p;` | refused; recovery node |
| duplicate `abstract` / duplicate `variation` | refused; recovery node |
| duplicate portion kind (`snapshot timeslice part p;`, `snapshot snapshot part p;`) | refused; recovery node |
| repeated independent singleton (`ref ref part p;`, `individual individual part p;`, `derived derived part p;`) | refused; recovery node |
| missing `part` after a valid prefix (`in derived ref;`) | refused; recovery node covering the prefix and its terminator; no fabricated kind keyword |
| missing declaration/completion after `part` (`part`) | refused; recovery node |
| `#;` | refused; recovery node reporting `malformed_annotation_head`. No fabricated reference |
| `# part p;`, `#Tag:: part p;`, `#$:: part p;` | refused, with the exact authored span reported and **no reference allocated at all**. `PrefixMetadataUsage`'s `OwnedFeatureTyping` is a `[QualifiedName]`, whose segments are `NAME`s, and a reserved keyword is never a `NAME`, so the `#` head refuses one during its validation walk -- before any arena mutation. Until that was fixed the standalone `PrefixMetadataMember` parser read `part` as the reference's last segment and swallowed the member behind the incomplete `#` |
| malformed prefix before a named part usage | one recovery node; later siblings survive |
| malformed prefix before an anonymous or `:>>` part usage | one recovery node; later siblings survive |
| malformed part usage followed by several valid siblings | all siblings survive |
| nested brace bodies after a prefixed usage; unmatched braces | balanced tracking unchanged from the body container |
| prefix-like words inside a quoted name, a string literal, escaped text, a line comment and a block comment | not prefix tokens |
| every materially distinct owning scope | same outcome |
| strict/editor equivalence on diagnostic-free input | identical documents |
| speculative parse that consumes extension-keyword references then fails | arena unchanged — the whole family parse runs inside `reference_transaction` |

Recovery never fabricates a prefix component, and a refused prefix never becomes an unprefixed
usage: the refusal is at the whole production, so the member becomes one recovery node spanning
the authored text.

### 10.3 `perform` body has no recovery table

Scope 6 in §3 parses its members with a bare `many0` and no starter list, so unrecognized content
ends the body rather than becoming a recovery node. That predates this slice, is shared by every
member of that scope rather than specific to `PartUsage`, and is left recorded rather than
changed here.

## 11. Explicit non-goals

- **No other usage family moves.** `PortUsage`, `ActionUsage`, `StateUsage`, `ViewUsage`,
  `RenderingUsage`, `ConnectionUsage`, `InterfaceUsage`, `AllocationUsage`, `Message`,
  `FlowUsage`, `SuccessionFlowUsage`, `CalculationUsage`, `ConstraintUsage`, `RequirementUsage`,
  `ConcernUsage`, `CaseUsage`, `AnalysisCaseUsage`, `VerificationCaseUsage`, `UseCaseUsage`,
  `PerformActionUsage`, `ExhibitStateUsage`, `IncludeUseCaseUsage`, `AssertConstraintUsage`,
  `AcceptNode`, `SendNode` all keep whatever partial prefix fields they already had.
  `MergeNode`/`DecisionNode`/`JoinNode`/`ForkNode` name `ControlNodePrefix`, a different
  production. `planning/occurrence-usage-prefix-matrix.md` §9 remains the authoritative ledger.
- **No universal usage/header node, optional-field bag, compatibility adapter, parallel legacy
  representation, or parser-framework abstraction.**
- **`then` is on `PartUsage`, but is not a prefix slot.** `SourceSuccessionMember : FeatureMembership
  = 'then' ownedRelatedElement += SourceSuccession` (BNF 597) precedes `OccurrenceUsageMember`, so
  it precedes the visibility keyword and the whole prefix. It is therefore a separate
  `then_span: Option<Span>` field, not a member of `OccurrenceUsagePrefix`. `SourceSuccession` and
  `SourceEndMember` below it contribute no further tokens, so the keyword's span is the whole
  authored fact. `OccurrenceUsage`'s pre-existing spanless `is_then: bool` moved to the same shape
  at the same time. The one pinned corpus occurrence
  (`then snapshot part vehicle_1_t1 { … }`, `training/28. Individuals/Individuals and
  Roles-1.sysml:18`) now parses, leaving that file diagnostic-free.
  `then` on the families this slice did not migrate -- `ItemUsage`, `SatisfyRequirementUsage` and
  the rest -- is still unrecognized; each closes it with its own slice.
- **`end part` is not accepted.** Pilot-only; see §8.
- ~~**`calc def` body dispatch is not widened.**~~ — closed by the follow-up. It was not merely
  narrow: `part p;` in a `calc def`, `calc` usage, `constraint def`, `constraint` usage or KerML
  type body fell through to the terminal expression arm and came apart into `'part';` plus `p;`,
  with no diagnostic and a round trip that wrote both back out.
  `planning/constraint-usage-prefix-matrix.md` §6 has the evidence.
- ~~**`item def` / `attribute def` semantic projection is not extended.**~~ — closed by the
  follow-up, together with `metadata def`/`metadata` usage, the occurrence usage body, the action
  usage body, `variant` members, KerML classifier bodies and constraint definitions and usages.
  All fifteen scopes that can hold a part usage now project their members;
  `tests/snapshots/sysml/part_usage_prefix_owning_scopes.md` shows every one, with the repeated
  member byte-identical across scopes.
- **`connector::ref_decl`'s `RefDeclKind::Part` is not deleted.** It keeps modelling `ref part`
  in the scopes where it currently wins and nothing else changes about it; only the scopes listed
  in §3 give `part_usage` first refusal.

## 11.1 Allocation and cost

Measured with `benches/parser_bench.rs` over the checked-in snapshot corpus, the maintained
benchmark. Both sides parse **identical input**: the baseline is the pre-slice parser run against
*this branch's* corpus, because the corpus itself grew by four fixtures and comparing across two
different corpora measures the fixtures, not the parser.

Command, run three times per side, alternating:

```
cargo bench --bench parser_bench -- 'snapshot_parser_corpus/all_sources'
```

Environment: macOS 25.3.0 (Darwin, arm64), `cargo bench` release profile with `debug = 1`,
criterion 0.5, 100 samples per run, no other load pinned.

| Run | Baseline (pre-slice parser, this corpus) | After |
| --- | --- | --- |
| 1 | 33.64 ms | 33.00 ms |
| 2 | 34.74 ms | 31.99 ms |
| 3 | 31.51 ms | 31.92 ms |

The machine's run-to-run spread (±10% on the baseline alone) is wider than the difference between
the two sides, so the honest reading is **no material change**, not an improvement. Nothing here
claims a win from code shape.

Per fixture, on the second measurement of each (the first run of a cold machine was discarded on
both sides after it moved a 22 µs fixture by 14%):

| Fixture | Baseline | After | Change |
| --- | --- | --- | --- |
| `sysml/part_usage_prefix_alternatives` | 172.1 µs | 126.6 µs | −26% — most of its members did not parse before, so the baseline is paying for recovery |
| `spec42/sysml/training/07_parts_example_1` (part usages and nothing else) | 22.04 µs | 22.72 µs | +3.1% |
| `sysml/ref_prefix_coverage` | 39.36 µs | 41.25 µs | +4.8% |
| `kerml/type_body_relationship_members` (no part usage at all) | 81.60 µs | 82.08 µs | +0.6%, i.e. the probe itself is not what costs |
| `spec42/sysml.library/isq_mechanics` (large, no part usage) | 1.579 ms | 1.642 ms | +4.0% |

Sizes, `std::mem::size_of`:

| Type | Before | After |
| --- | --- | --- |
| `PartUsage` | 920 | 1224 (+304, exactly the inline prefix) |
| `OccurrenceUsagePrefix` | 304 | 304 |
| `PartDefBodyElement`, `PartUsageBodyElement`, `ConnectionDefBodyElement` | 1240 | 1240 |
| `PackageBodyElement` | 1280 | 1280 |
| `AttributeBodyElement`, `OccurrenceBodyElement`, `UseCaseDefBodyElement` | 1176 | 1176 |
| `ActionDefBodyElement`, `ActionUsageBodyElement` | 984 | 984 |
| `CalcDefBodyElement`, `PerformBodyElement` | 368 | 368 |
| `VariantTypedUsage` | 16 | 16 |

No owning enum changed size: every one was already dominated by a larger variant, so the +304
bytes on `PartUsage` are absorbed.

Allocation:

- **An unauthored prefix allocates nothing.** Its four independent modifiers are `Option<Span>`,
  its three exclusive slots are `Option<Node<_>>`, and `extension_keywords` is a `Vec` whose
  default capacity is 0 -- checked directly, not assumed.
- **An authored prefix allocates once**, and only when a `#tag` was written: one `Vec` for the
  whole run, not one per keyword. No owned token `String` and no per-modifier allocation exists
  anywhere in the component.
- **Failed speculation does not grow the arena.** The whole family parse runs inside
  `reference_transaction`, pinned by
  `part_usage_prefix_owning_layer::a_refused_part_prefix_leaves_no_arena_entry` over five refused
  members, including `#Ghost 123;` (which allocates a reference before the production is refused)
  and `ref part def B;`.

## 12. Why this is one PR

The six fields, their five construction paths, thirteen owning scopes, one emitter, seven
semantic-projection sites, one visitor walk and one serialized shape are a single fact —
"which prefix did the author write on this part usage" — represented six ways. Splitting it
leaves `main` with two representations of that fact at some commit, which
`planning/shared-grammar.md`'s Phase 4 entry gate forbids ("each slice must leave `main` in a
valid final architecture with no temporary compatibility model"). The commits inside the PR are
sequenced so the AST change, the parser change and the consumer change are each reviewable, but
none of them is independently mergeable.

## 13. Coverage

| Evidence | What it pins |
| --- | --- |
| `tests/snapshots/sysml/part_usage_prefix_alternatives.md` | no prefix; every slot alone; all three directions; `abstract` and `variation`; `snapshot` and `timeslice`; one and several extension keywords; the full legal order; materially distinct combinations; absolute/relative/quoted extension references; named, short-name, anonymous and `:>>` shapes; typing, multiplicity with `ordered`/`nonunique`, subsets, redefines, values, both body forms; `MemberPrefix` visibility beside the prefix |
| `tests/snapshots/sysml/part_usage_prefix_owning_scopes.md` | a materially different prefix in every scope of §3, and identical projection for identical syntax across scopes |
| `tests/snapshots/sysml/part_usage_prefix_recovery.md` | every case in §10 except the unmatched brace, plus a valid sibling after each |
| `tests/snapshots/sysml/part_usage_prefix_unterminated.md` | the unmatched-brace state, which subsumes every other member and so needs its own fixture |
| `tests/part_usage_prefix_owning_layer.rs` | arena rollback after refused speculation; strict/editor equivalence; serde round trip; envelope rejection of a wrong-token span, a wrong alternative, out-of-order slots, a non-`#` sigil, a dangling extension identity and a keyword belonging to another member |
| `tests/snapshots/spec42/**` | the pinned corpus files this slice changes |
