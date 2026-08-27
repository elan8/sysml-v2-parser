# `ConstraintUsage` occurrence-prefix matrix

The `ConstraintUsage`/`ConstraintDefinition` slice of the shared `OccurrenceUsagePrefix` seam. The
pin is `docs/conformance-target` (`release_tag=2026-04`,
`grammar_content_hash=fnv1a64:95f39e912f73b917`).

The component itself is audited once in `planning/occurrence-usage-prefix-matrix.md` §§1-5 and is
not restated here. This document audits what is specific to the constraint family.

## 0. Why this slice, and why now

It was not planned. It is the blocker the `PartUsage` slice's follow-up ran into: completing the
semantic projection so that a part usage is visible in *every* scope that can hold one meant
projecting `ConstraintDefBody`, and doing that made the end-to-end
parse→emit→reparse check able to see two pre-existing defects it had been unable to see while the
scope projected as a contentless `(constraint-def)` marker:

```text
abstract constraint def ConstraintCheck :> BooleanEvaluation {   -- Systems Library
    ref constraint self : ConstraintCheck :>> BooleanEvaluation::self;   -- Constraints.sysml
}
```

emitted as

```text
constraint def ConstraintCheck :> BooleanEvaluation {   -- `abstract` silently dropped
    'ref';                                              -- shredded into a bare expression
    constraint self : ConstraintCheck :>> BooleanEvaluation::self;
}
```

Both are in the pinned Systems Library, so they were exercised by every corpus gate and passed:
the round-trip comparison compared two contentless markers. That is the failure mode a
"complete the projection" change exists to end, and reverting the projection to keep the gate
green would have restored it.

## 1. Authoritative productions

Verbatim from `sysml-v2-release/bnf/SysML-textual-bnf.kebnf` at the pin.

```text
ConstraintDefinition =
    OccurrenceDefinitionPrefix 'constraint' 'def'
    DefinitionDeclaration CalculationBody              -- line 1378, clause 8.2.2.20

ConstraintUsage =
    OccurrenceUsagePrefix 'constraint'
    ConstraintUsageDeclaration CalculationBody         -- line 1382

CalculationBody : Type =
      ';' | '{' CalculationBodyPart '}'                -- line 1359

CalculationBodyItem : Type =
      ActionBodyItem
    | ownedRelationship += ReturnParameterMember       -- line 1366

ActionBodyItem : Type =
      NonBehaviorBodyItem | …                          -- line 901

NonBehaviorBodyItem =
      ownedRelationship += Import
    | ownedRelationship += AliasMember
    | ownedRelationship += DefinitionMember
    | ownedRelationship += VariantUsageMember
    | ownedRelationship += NonOccurrenceUsageMember
    | ( ownedRelationship += SourceSuccessionMember )?
      ownedRelationship += StructureUsageMember        -- line 910

OccurrenceDefinitionPrefix : OccurrenceDefinition =
    BasicDefinitionPrefix?
    ( isIndividual ?= 'individual'
      ownedRelationship += EmptyMultiplicityMember )?
    DefinitionExtensionKeyword*                        -- line 541

BasicDefinitionPrefix : Definition =
    isAbstract ?= 'abstract' | isVariation ?= 'variation'
```

`ConstraintUsage` names `OccurrenceUsagePrefix` directly, exactly as `PartUsage`, `ItemUsage`,
`OccurrenceUsage` and `SatisfyRequirementUsage` do. `StructureUsageMember` is why a `PartUsage` is
legal inside a `CalculationBody` — see §6.

## 2. Legal token order

```
[public|private|protected]                       -- MemberPrefix, on the membership
[in|out|inout] [derived] [abstract|variation] [constant] [ref] [individual]
[snapshot|timeslice] ('#' QualifiedName)*        -- OccurrenceUsagePrefix
'constraint'
[<short>] [name] [FeatureSpecializationPart]     -- ConstraintUsageDeclaration
(';' | '{' … '}')                                -- CalculationBody
```

## 3. Before, and after

### 3.1 `ConstraintUsage`

| Slot | Before | After |
| --- | --- | --- |
| `abstract` | consumed by `opt(preceded(tag("abstract"), ws1))` and **discarded** | `prefix.basic.ref_prefix.variance`, with its authored span |
| `ref` | **not recognized** — the member reached the scope's expression fallback | `prefix.basic.reference_span` |
| direction, `derived`, `constant`, `individual`, `PortionKind`, `UsageExtensionKeyword*` | **not recognized** | the corresponding slot of the shared component |

`prefix: OccurrenceUsagePrefix` is purely additive: nothing was deleted, because nothing was there
— the same shape `SatisfyRequirementUsage`'s migration had.

### 3.2 `ConstraintDefinition`

`is_abstract: bool` — the fact the shared `parse_definition_prefix` helper already parsed and the
struct had nowhere to put.

A `bool` and not the two-alternative `DefinitionPrefix` enum, deliberately: that helper is the one
every definition family in this crate routes through and it recognizes only `abstract`. Making
`variation constraint def` representable means building the `OccurrenceDefinitionPrefix`
component, which is a ~20-family seam of its own and is **not** this slice. Recorded in §8.

### 3.3 `ConstraintDefBodyElement::Constraint` is boxed

The shared prefix is 304 bytes inline, which made `ConstraintUsage` much the largest member of its
own scope (800 bytes against a 368-byte second place). Boxing it leaves the enum at 368 — smaller
than before this slice, not larger.

## 4. Legal owning scopes

| Scope | Owning enum | Dispatch |
| --- | --- | --- |
| package / namespace / root | `PackageBodyElement::ConstraintUsage` | `try_package_body_dispatch!`, starter-filtered |
| `constraint def` / `constraint` usage body | `ConstraintDefBodyElement::Constraint` | `constraint_def_body_element` if-chain |
| `part def` body | `PartDefBodyElement::ConstraintUsage` | `alt` |
| `part` usage body | `PartUsageBodyElement::ConstraintUsage` | `alt` |
| attribute / item / metadata body | `AttributeBodyElement::ConstraintUsage` | `alt` |
| `requirement def` body | `RequirementDefBodyElement::Constraint` | `alt` |

Only the `constraint_def_body_element` chain needed changing: it is keyword-dispatched, and its
guard was `starts_with_keyword("constraint")`, which no prefixed spelling satisfies. Every other
scope dispatches through an `alt`, where `constraint_usage` parses its own prefix.

## 5. Corpus survey

Every prefix run written immediately before a `constraint` keyword in the pinned corpus (309
`.sysml` files), over comment- and string-stripped source:

| Spelling | Count | Representative location | Class before this slice |
| --- | --- | --- | --- |
| *(no prefix)* | 162 | `Domain Libraries/Analysis/SampledFunctions.sysml:28` | valid and represented |
| `abstract` | 6 | `Systems Library/Constraints.sysml:13,23,30,37` | valid; **parsed and discarded** on both the definition and the usage |
| `ref` | 1 | `Systems Library/Constraints.sysml:20` | valid; **shredded** into `'ref';` plus a separate usage, with no diagnostic |
| `#` | 1 | `Metadata Examples/RequirementMetadataExample.sysml:30` (`assume #goal constraint payloadMassLimit;`) | valid; claimed by the standalone `PrefixMetadataMember` parser |

No `in`/`out`/`inout`, `derived`, `constant`, `variation`, `individual`, `snapshot` or `timeslice`
occurs on a constraint anywhere in the pinned corpus, and no malformed constraint prefix does
either.

## 6. `PartUsage` in a calculation-shaped body

`CalculationBodyItem → ActionBodyItem → NonBehaviorBodyItem → StructureUsageMember → PartUsage`,
so `part p : T;` is legal in a `calc def`, a `calc` usage, a `constraint def` and a `constraint`
usage body. Neither scope had an arm for it:

- `CalcDefBodyElement` had a `PartUsage` variant, reachable only behind an `in`/`out`/`inout`
  gate, so a bare `part p;` fell through to the terminal expression arm and became `'part';` plus
  `p;` — two members, no diagnostic, and a round trip wrote both back out.
- `ConstraintDefBodyElement` had no `PartUsage` variant at all, and shredded it the same way.

Both now dispatch `part_usage` under a guard that admits the kind keyword, the uncontended prefix
slots, and `occurrence_prefix::starts_contended_prefix` for the `ref`/`#` runs. The attempt is
transactional, so a member that is really a KerML feature or a metadata annotation falls through
unchanged.

`CalcDefBody` is shared with this crate's KerML type bodies (`struct`, `classifier`, `datatype`,
…), so `part p;` in one of those is now a typed `PartUsage` rather than two bare expressions.
`part` is not a KerML production; preserving the authored member is nonetheless strictly better
than silently rewriting it, and this body already accepts `attribute`, `import`, `metadata` and
`#` members on the same basis. Recorded rather than hidden:
`tests/snapshots/sysml/part_usage_prefix_owning_scopes.md` writes one.

## 7. Coverage

| Evidence | What it pins |
| --- | --- |
| `tests/snapshots/sysml/constraint_usage_prefix_alternatives.md` | every prefix slot on a constraint usage and on a constraint definition, in every scope that owns one, with both body forms |
| `tests/snapshots/sysml/part_usage_prefix_owning_scopes.md` | a part usage in a `calc def`, a `constraint def` and a KerML type body |
| `tests/snapshots/spec42/sysml.library/constraints.md` | the pinned Systems Library file whose `abstract` and `ref` this slice stopped losing |

## 8. Explicit non-goals

- **`variation` on a definition prefix.** `BasicDefinitionPrefix` is a two-alternative slot, but
  the shared `parse_definition_prefix` helper recognizes only `abstract`, for every definition
  family in the crate. Closing it is the `OccurrenceDefinitionPrefix` component's job.
- **`individual`, `EmptyMultiplicityMember` and `DefinitionExtensionKeyword*` on a constraint
  definition.** Same helper, same seam.
- **`nonunique`/`ordered` on a constraint usage.** `ConstraintUsage` has no `MultiplicityPart`
  modifier fields, so `constraint c : T[0..*] nonunique` still loses `nonunique` on emission
  (`Systems Library/Constraints.sysml:23`). Pre-existing, unrelated to the prefix, and invisible to
  the round-trip check because nothing records it — recorded here so it is not mistaken for closed.
- **No other usage family moves.** `planning/occurrence-usage-prefix-matrix.md` §9 remains the
  authoritative ledger.
