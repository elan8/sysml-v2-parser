# `PortUsage` occurrence-prefix matrix

The `PortUsage` slice of the shared `OccurrenceUsagePrefix` seam, classified against the pinned
grammar, with every spelling found in the checked-in fixtures and in the pinned
`sysml-v2-release` corpus. The pin is `docs/conformance-target` (`release_tag=2026-04`,
`grammar_content_hash=fnv1a64:95f39e912f73b917`).

This document is the per-family audit `planning/occurrence-usage-prefix-matrix.md` §10 asks for
before a deferred family moves:

> `PortUsage` is the natural following structural candidate: `PortUsage = OccurrenceUsagePrefix
> 'port' Usage` (646) names the same production, and `PortUsage` already models direction,
> `is_abstract`, `is_reference` and `is_individual`, so the shape of the change is the one this
> slice just made. Its own matrix must still establish its scopes, construction paths, competing
> `ref`/`#` dispatch and recovery before any production edit, exactly as `PartUsage`'s did.

The component itself -- what `OccurrenceUsagePrefix`, `BasicUsagePrefix`, `RefPrefix` and
`UsageExtensionKeyword` are, why they nest, and which neighbouring prefixes are deliberately *not*
this production -- is audited once in `planning/occurrence-usage-prefix-matrix.md` §§1-5 and is
not restated here. This document audits only what is specific to `PortUsage`: its production, its
owning scopes, its construction path, its dispatch hazards, its corpus, and its recovery contract.

## 1. Authoritative production

Verbatim from `sysml-v2-release/bnf/SysML-textual-bnf.kebnf` at the pin.

```text
PortDefinition =                                              -- line 628, clause 8.2.2.12
    DefinitionPrefix 'port' 'def' Definition
    ownedRelationship += ConjugatedPortDefinitionMember
    { conjugatedPortDefinition.ownedPortConjugator.
        originalPortDefinition = this }

PortUsage =                                                   -- line 645, clause 8.2.2.12
    OccurrenceUsagePrefix 'port' Usage
```

The `Usage` tail, identical to the one `planning/part-usage-prefix-matrix.md` §1 quotes:

```text
Usage                     = UsageDeclaration UsageCompletion                    -- 305
UsageDeclaration : Usage  = Identification FeatureSpecializationPart?           -- 308
UsageCompletion  : Usage  = ValuePart? UsageBody                                -- 311
UsageBody        : Usage  = DefinitionBody                                      -- 314
DefinitionBody   : Type   = ';' | '{' DefinitionBodyItem* '}'
MemberPrefix : Membership = ( visibility = VisibilityIndicator )?               -- 130
```

and the membership productions that carry a `PortUsage` into a body:

```text
OccurrenceUsageMember : FeatureMembership =
    MemberPrefix ownedRelatedElement += OccurrenceUsageElement                  -- 259
OccurrenceUsageElement : Usage = StructureUsageElement | BehaviorUsageElement   -- 353
StructureUsageElement  : Usage = … | PortUsage | …                              -- 366
VariantUsageElement    : Usage = … | PortUsage | …                              -- 406
InterfaceOccurrenceUsageElement : Usage =
    DefaultInterfaceEnd | StructureUsageElement | BehaviorUsageElement          -- 749
PackageMember : OwningMembership =
    MemberPrefix ( ownedRelatedElement += DefinitionElement
                 | ownedRelatedElement = UsageElement )                         -- 133
```

### 1.1 Two other textual productions return the `PortUsage` metaclass

Neither is this slice's production, and neither builds an `ast::PortUsage` in this parser:

```text
DefaultInterfaceEnd : PortUsage = isEnd ?= 'end' Usage                          -- 752
InterfaceEnd        : PortUsage =
    ( ownedRelationship += OwnedCrossMultiplicityMember )?
    ( declaredName = NAME REFERENCES )?
    ownedRelationship += OwnedReferenceSubsetting                               -- 781
```

Both give the *abstract metaclass* `PortUsage` a textual production with **no**
`OccurrenceUsagePrefix` and **no** `'port'` keyword; this parser models both as `ast::EndDecl`
(`connector::end_decl`, dispatched from the interface and connection definition bodies). They are
therefore neither construction paths for this slice nor affected by it, exactly as `ActorUsage`
and `StakeholderUsage` were for `PartUsage` (`planning/part-usage-prefix-matrix.md` §8).

### 1.2 `PortDefinition` names a different prefix, and `def` is not optional

`PortDefinition = DefinitionPrefix 'port' 'def' Definition`. Two consequences the dispatch section
below depends on:

- `DefinitionPrefix` is `( isAbstract ?= 'abstract' | isVariation ?= 'variation' )?` --
  *not* `OccurrenceDefinitionPrefix`, so a port definition carries no `individual` and no
  `EmptyMultiplicityMember`, and certainly no direction, `derived`, `constant` or `ref`;
- `'def'` is a required literal. A `port` declaration with no `def` is a `PortUsage`, in every
  scope, with no exception in the pin.

### 1.3 Conformance identity

| Fact | Value |
| --- | --- |
| Release tag | `2026-04` |
| Grammar content hash | `fnv1a64:95f39e912f73b917` |
| SysML productions | 350 |
| KerML productions | 290 |
| `PortUsage` line | 645 |
| `PortDefinition` line | 628 |
| `OccurrenceUsagePrefix` line | 564 |

## 2. Nesting, cardinality and legal token order

`PortUsage` adds nothing to the prefix; it is `OccurrenceUsagePrefix` verbatim, then one keyword,
then `Usage`. The full slot table is `planning/occurrence-usage-prefix-matrix.md` §2; repeated
here only as the legal order this family must accept and no other:

```
[public|private|protected]                       -- MemberPrefix, on the membership, NOT the prefix
[in|out|inout] [derived] [abstract|variation] [constant] [ref] [individual]
[snapshot|timeslice] ('#' QualifiedName)*        -- OccurrenceUsagePrefix
'port'                                           -- the kind keyword
[<short>] [name] [FeatureSpecializationPart]     -- UsageDeclaration
[= expr | := expr | default …] (';' | '{' … '}') -- UsageCompletion
```

Cardinality and exclusivity, per slot:

| Slot | Cardinality | Kind |
| --- | --- | --- |
| `in` / `out` / `inout` | 0..1 | one slot, three alternatives -- mutually exclusive |
| `derived` | 0..1 | independent |
| `abstract` / `variation` | 0..1 | one slot, two alternatives -- mutually exclusive |
| `constant` | 0..1 | independent |
| `ref` | 0..1 | independent |
| `individual` | 0..1 | independent |
| `snapshot` / `timeslice` | 0..1 | one slot, two alternatives -- mutually exclusive |
| `'#' QualifiedName` | 0..* | ordered, repeatable, authored order retained |

Everything else repeated is malformed: `abstract abstract`, `in out`, `snapshot timeslice`,
`ref ref`, `individual individual`. The grammar gives no slot for the second token, so the member
is refused, not "last wins".

`then` (`SourceSuccessionMember`) precedes `OccurrenceUsageMember` and therefore precedes both the
visibility keyword and the prefix. It is **not** a prefix slot; see §11.

## 3. Legal owning scopes

Every scope this parser dispatches a `PortUsage` from, the AST enum that owns it, and the recovery
starter table that scope synchronizes on.

| # | Scope | Owning AST enum | Parser entry | Recovery starters |
| --- | --- | --- | --- | --- |
| 1 | package / namespace / root | `PackageBodyElement::PortUsage` | `package::try_package_body_structure` -> `port::port_usage` | `PACKAGE_BODY_GRAMMAR` |
| 2 | `part def` body | `PartDefBodyElement::PortUsage` | `part::body::part_def_body_element` | `PART_BODY_STARTERS` |
| 3 | `part` usage body (and `ref` bodies, which share it) | `PartUsageBodyElement::PortUsage` | `part::usage::part_usage_body_element` | `PART_BODY_STARTERS` |
| 4 | `port def` body | `PortDefBodyElement::PortUsage` | `port::port_def_body_element` | `PORT_DEF_BODY_STARTERS` |
| 5 | `port` usage body | `PortBodyElement::PortUsage` | `port::port_body_element` | `PORT_BODY_STARTERS` |
| 6 | `interface def` body | `InterfaceDefBodyElement::PortUsage` | `interface::interface_def_body_element` | `INTERFACE_DEF_BODY_STARTERS` |
| 7 | `connection def` body | `ConnectionDefBodyElement::PortUsage` | `connection::connection_def_body_element` | `CONNECTION_DEF_BODY_STARTERS` |
| 8 | `requirement def` / `concern` / `viewpoint` body | `RequirementDefBodyElement::PortUsage` | `requirement::requirement_def_body_element` | `REQUIREMENT_BODY_STARTERS` |
| 9 | `variant` member | `VariantTypedUsage::Port` | `part::usage::variant_usage` -> `port::port_usage` | owner's table |

Scope 9 is reached from wherever `variant_usage` is dispatched (part definition, part usage,
requirement and view bodies); it contributes no starter table of its own because a `variant`
member synchronizes on `variant`.

This is a *smaller* ownership surface than `PartUsage`'s thirteen, and a different one: ports are
not members of attribute, item, metadata, `perform`, occurrence, `use case def`, `calc def` or
action bodies in this parser. §11 records that as a deliberate non-goal rather than as coverage
this slice claims.

## 4. Construction paths

Every place an `ast::PortUsage` value is built, before this slice.

| Path | File | What prefix it parsed | Gap |
| --- | --- | --- | --- |
| `port_usage` | `src/parser/port.rs` | `individual`, `abstract`, direction, `derived`, `constant` -- **in that order** | no spans; wrong order (§7.3); no `variation`; no `ref`; no `snapshot`/`timeslice`; no `UsageExtensionKeyword` |

One construction path, unlike `PartUsage`'s five. That is the whole difference in shape between
the two slices: there is no head/tail split to unify, so the change at the parser is a single
substitution -- and correspondingly all of the risk is in *dispatch*, not in construction.

`connector::ref_decl` (`RefDeclKind::Port`) and `connector::end_decl` also read port-shaped
declarations, but neither constructs a `PortUsage`; see §7.

## 5. Current AST representation, and the intended one

### 5.1 Before

`ast::PortUsage` (`src/ast/structure.rs`) carried five independent fields with no spans:

| Field | Grammar slot | Defect |
| --- | --- | --- |
| `direction: Option<InOut>` | `RefPrefix.direction` | no span |
| `is_abstract: bool` | `RefPrefix`'s `abstract`/`variation` alternative | no span; a boolean cannot hold `variation`, which was simply not parsed |
| `is_derived: bool` | `RefPrefix.isDerived` | no span |
| `is_constant: bool` | `RefPrefix.isConstant` | no span |
| `is_individual: bool` | `isIndividual` | no span |
| -- | `BasicUsagePrefix.isReference` (`ref`) | **not represented at all** -- claimed by `connector::ref_decl` instead |
| -- | `PortionKind` | **not represented at all** |
| -- | `UsageExtensionKeyword*` | **not represented at all** |

The five that existed were also parsed in the wrong order and re-emitted in a *third* order; §7.3
records what that did to authored text.

### 5.2 After

```rust
pub struct PortUsage {
    pub prefix: crate::ast::OccurrenceUsagePrefix,
    // …unchanged Usage tail: name, short_name, typing, multiplicity, subsets, redefines,
    //  references, crosses, intersects, value, body, name_span, type_ref_span, membership
}
```

One field, the shared component, exactly as `OccurrenceUsage`, `ItemUsage`,
`SatisfyRequirementUsage`, `PartUsage` and `ConstraintUsage` already carry it. No port-specific
mirror and no getters reconstructing the five booleans;
`planning/occurrence-usage-prefix-matrix.md` §5.2 records why each sub-shape is what it is, and
reusing it verbatim is the point of the seam.

`PortUsage`'s hand-written `PartialEq` (which deliberately ignores `name_span`/`type_ref_span`)
keeps that behavior and compares `prefix` where it compared the five fields.

`membership` stays outside the prefix: `MemberPrefix`'s visibility belongs to the
`OccurrenceUsageMember`/`PackageMember`, not to the usage.

## 6. FIRST tokens and recovery implications, per scope

FIRST(`PortUsage`) is FIRST(`OccurrenceUsagePrefix`) ∪ {`port`}, because the whole prefix is
optional:

```
in  out  inout  derived  abstract  variation  constant  ref  individual  snapshot  timeslice  #  port
```

All thirteen are recovery boundaries in every scope of §3. Scope starter tables before this slice:

| Table | Already listed | Missing (added by this slice) |
| --- | --- | --- |
| `PACKAGE_BODY_GRAMMAR` | all thirteen | -- (complete) |
| `PART_BODY_STARTERS` | all thirteen | -- (complete, from the occurrence and part seams) |
| `PORT_DEF_BODY_STARTERS` | `abstract`, `in`, `inout`, `out`, `port`, `ref` | `#`, `constant`, `derived`, `individual`, `snapshot`, `timeslice`, `variation` |
| `PORT_BODY_STARTERS` | `in`, `inout`, `out`, `port` | `#`, `abstract`, `constant`, `derived`, `individual`, `ref`, `snapshot`, `timeslice`, `variation` |
| `INTERFACE_DEF_BODY_STARTERS` | `ref` | `port` and the remaining eleven |
| `CONNECTION_DEF_BODY_STARTERS` | all thirteen except `port` | `port` |
| `REQUIREMENT_BODY_STARTERS` | all thirteen | -- (complete, from the occurrence and part seams) |

Adding a real FIRST token is what stops a malformed member from scanning past a prefix and eating
the valid usage that follows it, exactly as recorded for `assert`/`not` in
`planning/satisfy-requirement-usage-matrix.md` §6 and for the occurrence prefix in
`planning/occurrence-usage-prefix-matrix.md` §4.

### 6.1 Four of these tables were not consulted at all

Completing a table only helps if the scope resynchronizes on it. `parse_structured_brace_members`
defaults to `BraceMemberSkip::StatementOrBlock`, which scans to the next `;` or balanced block and
**never reads the `starters` argument**; only `BraceMemberSkip::BodyElementRecover` calls
`recover_body_element(input, starters)`. Before this slice exactly four scopes opted in --
requirement, view, part definition and part usage bodies -- and all four of *this* family's brace
scopes took the default:

```
interface def I {
    %%%                          -- unterminated malformed run
    private port p;              -- consumed into the malformed node
}
```

The malformed span covered both lines. The scan stopped at the `;` of the valid member, so the
port usage disappeared -- the "recovery must not consume valid later siblings" clause of the
parsing contract, failing silently because no fixture in any of those four scopes had a member
after malformed content.

`MemberPrefix` makes it worse than it looks: `port_usage` accepts `public`/`private`/`protected`
before the occurrence prefix, so even with all thirteen prefix tokens listed, `private port p;`
still needed `private` to be a starter. Both halves are necessary and neither is sufficient --
measured by making each change alone and re-parsing the case above.

All four scopes now pass `BraceMemberSkip::BodyElementRecover`, and their tables are completed to
the member set each `*_body_element` actually dispatches, not just to FIRST(`PortUsage`):

| Table | Also added | Why |
| --- | --- | --- |
| `INTERFACE_DEF_BODY_STARTERS` | `private`, `protected`, `public`, `comment`, `rep`, `@` | `MemberPrefix`; the annotating members the scope dispatches |
| `PORT_DEF_BODY_STARTERS` | `item`, `enum`, `comment`, `rep`, `@`, `private`, `protected`, `public` | the item, enumeration and annotating members it dispatches |
| `PORT_BODY_STARTERS` | `attribute`, `item`, `comment`, `rep`, `@`, `private`, `protected`, `public` | the attribute, item and annotating members it dispatches |
| `CONNECTION_DEF_BODY_STARTERS` | `attribute`, `assert`, `succession`, `comment`, `rep`, `@`, `private`, `protected`, `public` | the attribute, assert-constraint, succession and annotating members it dispatches |

This is a pre-existing defect rather than one the slice introduced -- `%%%` followed by a bare
`port p;` was swallowed before it too -- but it is a defect in exactly the scopes whose starter
tables this slice completes, and leaving them unconsulted would make that completion decorative.
`tests/snapshots/sysml/port_usage_prefix_scope_recovery.md` pins all four.

## 7. Competing productions and parser-precedence hazards

Six productions share a FIRST token with a prefixed `PortUsage`.

| Token | Competing production | Where it wins before this slice | Resolution |
| --- | --- | --- | --- |
| `port` | `PortDefinition` (`DefinitionPrefix 'port' 'def' Definition`) | every scope tries a `port_def` parser first. In the *nested* scopes that parser is `port_def_required`, which is correct. At **package scope** it is `port_def`, which makes `def` optional and therefore claims every keyword-less `port p : T;` as a definition | `port_def` requires `def` in every scope, so package-level `port p : T;` reaches `port_usage`. §7.1 has the evidence for why this is inseparable from the slice |
| `#` | `PrefixMetadataMember` as a standalone sibling member (`metadata_keyword_prefix`), and `ExtendedDefinition`/`ExtendedUsage` (`metadata_keyword_usage`) | `metadata_keyword_prefix` claimed `#idd` and left `port APIS_HTTP { … }` as a separate unprefixed member | `port_usage` gets first refusal through `occurrence_prefix::starts_contended_prefix`, exactly as the five already-migrated families do. `metadata_keyword_usage` (the `#Tag { … }` / `#Tag def …` spellings, which own a body or a definition) keeps its place ahead of the prefix walk in the scopes that try it first |
| `ref` | this parser's kinded `connector::ref_decl`, which models `ref port x;` as a `RefDecl` with `kind_keyword = Port` | every scope of §3 -- `port_usage` could not parse a `ref` at all, so `ref port q;` was either a `RefDecl` (port/part bodies) or recovery (package scope) | `port_usage` gets first refusal through a contended pre-dispatch in scopes 2, 3, 4, 6, 7 and 8; scope 5 (the port usage body) already tries it as its first alternative and needs none, and scope 1 selects on the `port` starter, which `port_def` no longer claims. Scope 9 reaches it through the same `variant_usage` chain as before. `connector::ref_decl` keeps `RefDeclKind::Port` for the *keyword-less* `ref` members (`ref self : Port :>> Object::self;`) and every other kind it models; the `ref port …` spellings, including `ref port :>> Interface::participant, BinaryConnection::participant[2] nonunique ordered;`, are `PortUsage`s |
| `in`/`out`/`inout` | `in_out_decl` (directed parameter member) | `in_out_decl` refuses the kinded forms (`in port`, `in part`, `in item`, …) itself | unchanged |
| `abstract`/`variation` | every `*_def` parser's `DefinitionPrefix` | each scope's `*_def` arm precedes its `*_usage` sibling | unchanged. `starts_contended_prefix` reports a run of *uncontended* slots as contended only when it reaches `ref` or `#`, so `abstract port def P { … }` still reaches `port_def` -- the PAR-001 bug class a blanket reorder would reopen |
| `individual`/`snapshot`/`timeslice` | `IndividualUsage`/`PortionUsage` (the keyword-less occurrence spellings, `occurrence_usage`) | `occurrence_usage` refuses when the next word is a reserved keyword (`next_word_is_reserved`), so `individual port p;` is left to the family that owns `port` | unchanged, and load-bearing for `snapshot port …`: without that guard the portion spelling would read `port` as its declaration name |

The rule this slice keeps: **the selected `PortUsage` parser claims a prefix only when the whole
production is viable.** `port_usage` parses the prefix and then requires the `port` keyword; if it
is absent the parse fails, the reference transaction rolls back every `#tag` identity it
allocated, and the member falls through to whichever sibling really owns it.

### 7.1 Why `port def`'s optional `def` cannot stay

`src/parser/port.rs::port_def` documents its own `def`-optional behaviour as a stopgap:

> `def` is intentionally optional: the standard library uses bare, `def`-less `port` usages at
> package/namespace level … and there is no dedicated package-level `port_usage` dispatch to catch
> them instead — this parser currently folds that legal form into `PortDef`.

There *is* a package-level `port_usage` dispatch (added by PAR-002), but it is tried *after*
`port_def`, so the fold still happens for every shape `port_def` accepts. Measured on this
parser before the slice:

| Package-scope input | Parsed as | Re-emitted as |
| --- | --- | --- |
| `port p1 : T;` | `PortDef` named `p1` | `port def p1 : T;` -- a `def` nobody wrote |
| `abstract port ports : Port[0..*] nonunique :> objects;` (`Systems Library/Ports.sysml:48`) | `PortDef` named `ports` | `port def ports :> objects;` -- `abstract`, `[0..*]` and `nonunique` all discarded |
| `port :>> p1 : T;` | `PortUsage` (the one shape `port_def` cannot claim) | `port  :>> p1 : T;` -- stranded keyword space |

The middle row is a pinned Systems Library declaration losing three authored facts on a strict
gate that passes, because both sides of the round trip lose them identically and the semantic
projection of a `PortDef` never mentioned them. It is the same failure mode
`planning/shared-grammar.md` records for contentless projections: *the gate runs through the
defect*.

This slice therefore requires `def` on `PortDefinition` in every scope, which is what the pin says
(§1.2), and gives the keyword-less form to `port_usage`. Two `Usage`-tail facts have to travel
with that, because the Systems Library writes them on the declarations that change hands:

- `MultiplicityPart`'s `isOrdered`/`isNonunique` -- `port ports : Port[0..*] nonunique :> objects`;
- a multi-target subsetting clause -- `port subports : Port[0..*] :> ports, timeEnclosedOccurrences`
  (already supported by `specialization_clauses`, so nothing new).

`ordered`/`nonunique` are not prefix slots, and adding them would be scope creep if the slice did
not need them. It does: without them the declarations `port_def` stops claiming reach recovery,
which would be a regression on the pinned library. They are recorded here rather than smuggled in.

## 8. Sibling Pilot comparison

`../SysML-v2-Pilot-Implementation/org.omg.sysml.xtext/src/org/omg/sysml/xtext/SysML.xtext`, which
is newer than this repository's 2026-04 pin.

| Production | Pilot | Pin | Material? | Followed |
| --- | --- | --- | --- | --- |
| `PortUsage` | `OccurrenceUsagePrefix PortUsageKeyword Usage` (986) | `OccurrenceUsagePrefix 'port' Usage` (645) | no -- `PortKeyword : 'port'`, `PortUsageKeyword : PortKeyword`, one token either way | pin |
| `PortDefinition` | `DefinitionPrefix PortDefKeyword Definition`, `PortDefKeyword : PortKeyword 'def'` (947-957) | `DefinitionPrefix 'port' 'def' Definition` (628) | no -- `def` is required in both | both |
| `OccurrenceUsagePrefix` | admits `EndUsagePrefix` as an alternative to the whole basic prefix | does not | **yes** -- `end port p;` is grammatical there and is not at the pin | **pin.** Recorded and deliberately not followed, exactly as `planning/part-usage-prefix-matrix.md` §8 records it for `end part` |
| `RefPrefix`, `BasicUsagePrefix`, `PortionKind`, `UsageExtensionKeyword`, `PrefixMetadataMember` | identical to the pin | 275-296, 1660 | no | both |
| `DefaultInterfaceEnd` / `InterfaceEnd` | `returns SysML::PortUsage`, no prefix and no `port` keyword (1143, 1181) | identical (752, 781) | no | both -- and neither is an `ast::PortUsage` here (§1.1) |

No Pilot behaviour newer than the pin is adopted.

## 9. Corpus survey

### 9.1 Pinned `sysml-v2-release` corpus (309 `.sysml` files)

Every prefix run written immediately before a `port` kind keyword that is not `port def`, counted
at member position over comment- and string-stripped source. `#Ref` runs are normalized to `#`.

| Spelling | Count | Representative location | Class before this slice |
| --- | --- | --- | --- |
| *(no prefix)* | 255 | `sysml/src/training/10. Ports/Port Conjugation Example.sysml:14` | valid and represented -- **except at package scope**, where `port_def` claims it (§7.1) |
| `ref` | 4 | `sysml/src/examples/Simple Tests/PartTest.sysml:21` (`ref port q;`), `:46`; `sysml.library/Systems Library/Interfaces.sysml:45,70` | valid; **claimed by `connector::ref_decl`**, so the `ref` is on a `RefDecl` rather than on this production |
| `#` | 2 | `sysml/src/examples/Arrowhead Framework Example/AHFNorwayTopics.sysml:22,28` (`#idd port APIS_HTTP { … }`) | valid; **parsed as a separate sibling member**, not as this usage's prefix |
| `abstract` | 2 | `sysml.library/Systems Library/Parts.sysml:31`, `Ports.sysml:48` | valid, partly represented (no span); the `Ports.sysml` one is at package scope and is claimed by `port_def` (§7.1) |
| `variation` | 1 | `sysml/src/examples/Variability Examples/VehicleVariabilityModel.sysml:79` (`variation port :>> autoPort { … }`) | valid; **unsupported** -- recovered as malformed, taking its `variant port autoPort1/2;` members with it |
| `abstract ref` | 1 | `sysml.library/Systems Library/Ports.sysml:30` | valid; claimed by `connector::ref_decl` |

Not present anywhere in the pinned corpus on a port usage: `in`, `out`, `inout`, `derived`,
`constant`, `individual`, `snapshot`, `timeslice`, `then`, any invalid ordering, any duplicate
modifier, any malformed prefix. One `MemberPrefix` visibility does occur
(`protected port c : C;`, `Simple Tests/PartTest.sysml:7`), and one `variant` member
(`variant port autoPort1;`, `VehicleVariabilityModel.sysml:80`).

### 9.2 Checked-in fixtures, tests and snapshots

| Spelling | Where | Class |
| --- | --- | --- |
| *(no prefix)* | everywhere | valid and represented |
| `individual port` | `tests/snapshots/spec42/sysml/coverage_individual.md:30` | valid, partly represented -- the keyword is parsed, kept in a boolean and re-emitted, but the semantic projection never mentions it |
| `abstract ref port` | `tests/snapshots/sysml/ref_declaration_scopes.md:10,45` | valid; a `RefDecl` |
| `ref port` | `tests/snapshots/sysml/ref_usage_body_members.md:8,28`, `ref_usage_body_recovery.md:8,27`, `tests/gh51_connection_interface_body_gaps.rs` | valid; a `RefDecl` |
| `derived port`, `out port` | `tests/parser/structure.rs:3608-3609` | valid, partly represented |
| `variation port`, `snapshot port`, `timeslice port`, `#Tag port`, `in derived abstract constant port` | **absent** | the gaps this slice closes; covered by the four fixtures in §13 |

### 9.3 Classification of every spelling found or constructed

| Class | Spellings |
| --- | --- |
| valid and represented exactly | *(no prefix)* `port x;` in scopes 2-9 |
| valid but partially represented | `in`/`out`/`inout`, `derived`, `abstract`, `constant`, `individual` -- the keyword was accepted but no authored span was retained, the emitter re-derived it from a boolean, and the semantic projection showed only `direction`, so no snapshot could tell `individual derived port p;` from `port p;` |
| valid but parsed and discarded | `abstract` and the multiplicity part of every package-scope keyword-less port usage, via the `port_def` fold (§7.1) |
| valid but parsed with the wrong shape | *(no prefix)* `port p : T;` at package scope -- a `PortDef`, re-emitted with a `def` nobody wrote; `ref port …` in every scope -- a `RefDecl` |
| valid but unsupported | `variation port …`, `snapshot port …`, `timeslice port …`, `ref port …` at package scope, `#Tag port …` (parsed as a sibling, not a prefix), and the **only legal full order** `in derived abstract constant port y;` |
| malformed | none in the pinned corpus. Constructed for coverage: `ref derived port`, `in out port`, `abstract variation port`, `abstract abstract port`, `snapshot timeslice port`, `snapshot snapshot port`, `ref ref port`, `individual individual port`, `in derived ref;` (prefix with no `port`), `port` with no declaration, `# port`, `#A:: port`, `#$:: port` |
| permissive legacy syntax not supported by the pin | `end port …` -- Pilot-only (§8), refused here. `individual abstract in derived constant port x;` -- an **illegal order this parser accepted and silently reordered** on emission to `in derived abstract constant individual port x;` (§7.3 below) |

### 9.4 The accepted-and-reordered spelling

Before this slice, `port_usage` probed its five slots in the order `individual`, `abstract`,
direction, `derived`, `constant`, and `emit_port_usage` wrote them in the order direction,
`derived`, `abstract`, `constant`, `individual`. Neither is the grammar's. The observable result:

```
in:   individual abstract in derived constant port x;   -- accepted, no diagnostic
out:  in derived abstract constant individual port x;   -- silently reordered
```

while the one order the grammar permits was refused:

```
in:   in derived abstract constant port y;              -- recovered as malformed
```

This is the defect that makes the slice a correctness fix rather than a representation change: a
round trip changed authored text, and the whole-AST comparison could not see it because the AST
held five order-free booleans.

## 10. Recovery contract

| Input | Required outcome |
| --- | --- |
| malformed member before any prefix FIRST token | recovery stops at the *first* prefix token, not at `port`; the prefixed usage after it parses |
| malformed content between two prefix slots (`in @ derived port p;`) | one recovery node retaining the exact malformed span |
| a comment between two prefix slots (`in /* why */ derived port p;`) | trivia; the usage parses |
| invalid ordering (`ref derived port p;`, `individual ref port p;`, `port individual p;`) | refused; one recovery node retaining the exact malformed span. Never reinterpreted as a valid unprefixed usage |
| duplicate direction (`in out port p;`) | refused; recovery node |
| `abstract variation port p;` | refused; recovery node |
| duplicate `abstract` / duplicate `variation` | refused; recovery node |
| duplicate portion kind (`snapshot timeslice port p;`, `snapshot snapshot port p;`) | refused; recovery node |
| repeated independent singleton (`ref ref port p;`, `individual individual port p;`, `derived derived port p;`) | refused; recovery node |
| missing `port` after a valid prefix (`in derived ref;`) | refused; recovery node covering the prefix and its terminator; no fabricated kind keyword |
| missing declaration/completion after `port` (`port`) | refused; recovery node |
| `#;` | refused; recovery node reporting `malformed_annotation_head`. No fabricated reference |
| `# port p;`, `#Tag:: port p;`, `#$:: port p;` | refused, with the exact authored span reported and **no reference allocated at all** -- a reserved keyword is never a `NAME`, so the `#` head refuses one during its validation walk, before any arena mutation |
| malformed prefix before a named port usage | one recovery node; later siblings survive |
| malformed prefix before an anonymous or `:>>` port usage | one recovery node; later siblings survive |
| malformed port usage followed by several valid siblings | all siblings survive |
| `port def` after a usage-only prefix (`ref port def B;`) | refused as a usage; the definition branch refuses it too (a definition prefix has no `ref`), so it becomes one recovery node, never a usage named `def` |
| nested brace bodies after a prefixed usage; unmatched braces | balanced tracking unchanged from the body container |
| prefix-like words inside a quoted name, a string literal, escaped text, a line comment and a block comment | not prefix tokens |
| every materially distinct owning scope | same outcome |
| strict/editor equivalence on diagnostic-free input | identical documents |
| speculative parse that consumes extension-keyword references then fails | arena unchanged -- the whole family parse runs inside `reference_transaction` |

Recovery never fabricates a prefix component, and a refused prefix never becomes an unprefixed
usage: the refusal is at the whole production, so the member becomes one recovery node spanning
the authored text.

### 10.1 Two body scopes that the change made reachable

`port_usage` claiming `ref port …` moves the *bodies* of those declarations from `RefBody`
(`Body<PartUsageBodyElement>`) to `PortBody`, whose member set was six variants. The pinned
Systems Library writes members in both that this scope did not model, so the slice adds them
rather than regressing the library:

| Member | Evidence | Production |
| --- | --- | --- |
| `RefDecl` | `protected ref thisParticipant :>> self;` and `protected ref otherParticipants : Port[1..*] nonunique :> interfacingPorts default …;` inside `ref port :>> participant : Port [2..*] nonunique ordered { … }` (`Systems Library/Interfaces.sysml:52-54`) | `UsageBody = DefinitionBody`; `PortDefBodyElement` already models the same member for the same reason |
| `VariantUsage` | `variant port autoPort1;` and `variant port autoPort2;` inside `variation port :>> autoPort { … }` (`Variability Examples/VehicleVariabilityModel.sysml:79-81`) | `DefinitionBodyItem -> VariantUsageMember`. Only reachable once `port_usage` could spell `variation`: the whole declaration used to reach recovery | **not added -- see below** |

`RefDecl` is coverage the scope was missing, not new grammar, and is added.

`VariantUsage` is **refused by the type-level cost gate** and is therefore *not* added.
`PortBodyElement::VariantUsage(Node<VariantUsage>)` closes a new cycle -- `PortBody ->
PortBodyElement -> VariantUsage -> VariantTypedUsage::Port -> PortUsage -> PortBody` -- and
`tests/type_level_cost.rs` stops compiling: proving `ParsedDocument: Send` overflows the *default*
trait-solver recursion limit, which is exactly the consumer-visible regression that test exists to
catch (`planning/shared-grammar.md`, "Verification strategy"). Raising the limit is the workaround
that plan forbids, so the member stays unmodelled and `variant port autoPort1;` in a port usage
body is a recovery node with `unexpected_keyword_in_scope`.

That is still an improvement on the pinned file: the whole `variation port :>> autoPort { … }`
declaration used to be one recovery node covering four lines, and is now a typed port usage with
two precisely-spanned recovery members inside it. `PORT_BODY_STARTERS` gains `variant` and the
three visibility keywords either way, because recovery has to synchronize on where a member
starts whether or not the member is modelled.

Closing it needs the cycle broken -- an indirection at `VariantTypedUsage`, or a narrower variant
family for the scope -- which is a type-graph change over every owner of `VariantUsage`, not a
port-usage change. Recorded in §11.

### 10.2 The `interface def` body had no real starter table

`INTERFACE_DEF_BODY_STARTERS` was `[connect, end, ref, doc]` -- four of the members that scope
dispatches, out of a dozen -- and the scope did not read it at all. §6.1 has the mechanism and the
fix; the table is now the member set the scope dispatches, and the scope resynchronizes on it.

## 11. Explicit non-goals

- **No other usage family moves.** `ActionUsage`, `StateUsage`, `ViewUsage`, `RenderingUsage`,
  `ConnectionUsage`, `InterfaceUsage`, `AllocationUsage`, `Message`, `FlowUsage`,
  `SuccessionFlowUsage`, `CalculationUsage`, `RequirementUsage`, `ConcernUsage`, `CaseUsage`,
  `AnalysisCaseUsage`, `VerificationCaseUsage`, `UseCaseUsage`, `PerformActionUsage`,
  `ExhibitStateUsage`, `IncludeUseCaseUsage`, `AssertConstraintUsage`, `AcceptNode` and
  `SendNode` all keep whatever partial prefix fields they already had.
  `MergeNode`/`DecisionNode`/`JoinNode`/`ForkNode` name `ControlNodePrefix`, a different
  production. `planning/occurrence-usage-prefix-matrix.md` §9 remains the authoritative ledger.
- **No universal usage/header node, optional-field bag, compatibility adapter, parallel legacy
  representation, or parser-framework abstraction.**
- **`then` is not modelled on `PortUsage`.** `SourceSuccessionMember = 'then' …` (BNF 597)
  precedes `OccurrenceUsageMember`, so it precedes the visibility keyword and the whole prefix; it
  is not a prefix slot. `PartUsage` carries a `then_span` because the pinned corpus writes
  `then snapshot part vehicle_1_t1 { … }`; no `then … port` occurs anywhere in the corpus or the
  fixtures, so modelling it here would be an unexercised field. Recorded as debt, closed by
  whichever slice finds evidence for it.
- **`end port` is not accepted.** Pilot-only; see §8. `DefaultInterfaceEnd` and `InterfaceEnd`
  stay on `ast::EndDecl` (§1.1).
- **`connector::ref_decl`'s `RefDeclKind::Port` is not deleted.** `port_usage` gets first refusal
  in the scopes of §3, so every `ref port …` in the pinned corpus becomes a `PortUsage`; the kind
  keyword itself stays in `ref_decl` because the parser reaches that declaration from scopes this
  slice does not touch, and every other kind it models is untouched.
- **`PortDefinition`'s own `DefinitionPrefix` is not modelled.** `PortDefinition = DefinitionPrefix
  'port' 'def' Definition` and `ast::PortDef` has no field for the `abstract`/`variation`
  alternative, so `abstract port def Port :> Object;` (`Systems Library/Ports.sysml:11`) parses,
  discards `abstract`, and is re-emitted without it; `variation port def V;` is not recognized at
  all and becomes an `ExtendedLibraryDecl`. That is a *definition* prefix -- the neighbour this
  slice's own component is deliberately not (`planning/occurrence-usage-prefix-matrix.md` §1.1) --
  and closing it belongs to the `OccurrenceDefinitionPrefix`/`DefinitionPrefix` slice, together
  with the identical gap on `part def`, `item def` and the rest. Recorded here because making
  `port def` project its declaration in three more scopes is what surfaced it.
- **`variant` in a port usage body is not modelled.** §10.1 has the evidence and the reason: the
  member is legal (`DefinitionBodyItem -> VariantUsageMember`) and the pinned corpus writes it,
  but the variant closes a type cycle that costs a downstream consumer its default trait-solver
  recursion limit. It stays a recovery node until the cycle is broken at `VariantUsage`.
- **`MultiplicityPart`'s authored order is not retained.** `ordered nonunique` and `nonunique
  ordered` are both legal and both land in two independent booleans, so emission always writes
  `ordered nonunique`. That is the representation `PartUsage`, `AttributeUsage` and `RefDecl`
  already use; changing it is one seam over every usage family, not this one.
- **Ports are not added to scopes that do not dispatch them.** `StructureUsageElement` reaches a
  `PortUsage` from every definition body in the pin, including attribute, item, metadata,
  occurrence, `use case def`, `calc def` and action bodies, and this parser dispatches none of
  them. That is one gap per scope in a *different* production's selector, identical in kind to the
  `calc def` gap `planning/part-usage-prefix-matrix.md` §3 recorded and a later follow-up closed.
  It is recorded here, not widened.
- **`ordered`/`nonunique` on `PortUsage` are in scope only because §7.1 makes them inseparable.**
  No other `Usage`-tail fact is added.

## 11.1 Allocation and cost

Sizes, `std::mem::size_of`, measured on both sides:

| Type | Before | After |
| --- | --- | --- |
| `PortUsage` | 1208 | 1512 (+304, exactly the inline prefix) |
| `OccurrenceUsagePrefix` | 304 | 304 |
| `PackageBodyElement` | 1320 | 1320 |
| `PartDefBodyElement`, `PartUsageBodyElement`, `PortDefBodyElement`, `PortBodyElement`, `InterfaceDefBodyElement`, `ConnectionDefBodyElement` | 1240 | 1176 |
| `RequirementDefBodyElement` | 1176 | 1176 |
| `VariantTypedUsage` | 16 | 16 |

The six owning enums that `PortUsage` had been the largest variant of get *smaller*, because the
variant is now `Box<Node<PortUsage>>` and the second-largest variant dominates. That is one
allocation per authored port usage rather than 1240 bytes in every member of those scopes,
including the members that are not ports; `RequirementDefBodyElement` and `VariantTypedUsage`
already boxed it for the same reason.

Allocation inside the prefix itself is unchanged from
`planning/occurrence-usage-prefix-matrix.md` §8.2: an unauthored prefix allocates nothing (four
`Option<Span>`, three `Option<Node<_>>`, and a `Vec` with capacity 0), an authored one allocates
once and only when a `#tag` was written, and a refused prefix allocates nothing observable --
`port_usage` runs entirely inside `reference_transaction`, which
`port_usage_prefix_owning_layer::a_refused_port_prefix_leaves_no_arena_entry` pins over five
refused members.

Wall clock, `benches/parser_bench.rs`'s `snapshot_parser_corpus/all_sources` over the checked-in
snapshot corpus, the maintained benchmark. Both sides parse **identical input**: the baseline is
the pre-slice parser run against *this branch's* corpus, because the corpus grew by four fixtures
and comparing across two corpora measures the fixtures, not the parser.

```
cargo bench --bench parser_bench -- 'snapshot_parser_corpus/all_sources'
```

Environment: macOS 25.3.0 (Darwin, arm64), `cargo bench` release profile with `debug = 1`,
criterion 0.5, 100 samples per run, no other load pinned.

| Run | Baseline (pre-slice parser, this corpus) | After, first implementation | After, with the first-byte admission test |
| --- | --- | --- | --- |
| 1 | 31.343 ms | 32.368 ms | 31.481 ms |
| 2 | 31.290 ms | 32.497 ms | 31.154 ms |
| 3 | 31.295 ms | -- | -- |
| 4 | 31.412 ms | -- | -- |

The first implementation was a reproducible **+3.5%**, with 95% intervals of ±0.3% on both sides
-- narrow enough that it was a regression, not noise, and worth finding rather than reporting.

The cause is the same one `planning/occurrence-usage-prefix-matrix.md` §8.2 recorded for the seam
itself: **the failing probe is the hot one.** A sixth family now spells the prefix, and each one
walks eleven optional keyword slots before it may require its kind keyword, for a member that
almost never authored any of them. `port_usage` is attempted for every member of every scope with
no starter table, so the corpus pays that walk twice as often as it did.

The fix is one byte. Every slot of the production opens with `#` or one of `i o d a v c r s t`,
so `occurrence_usage_prefix` looks the first byte up in a table derived from the keyword list and
returns the unauthored prefix without probing a slot. It cannot drift: the table is built by a
`const fn` over the same keyword list the slots use, and
`occurrence_prefix::tests::each_slot_is_admitted_on_its_own` parses every slot alone. With it the
difference is inside the machine's own run-to-run spread, so the honest reading is **no material
change**. Nothing here claims a win from code shape: the admission test recovers a regression this
slice introduced, and every family that already spelled the prefix gets the same recovery.

## 12. Why this is one PR

Five fields, one construction path, nine owning scopes, one emitter, seven semantic-projection
sites, one visitor walk and one serialized shape are a single fact -- "which prefix did the author
write on this port usage" -- represented five ways, in the wrong order, and lost entirely at
package scope. Splitting it leaves `main` with two representations of that fact at some commit,
which `planning/shared-grammar.md`'s Phase 4 entry gate forbids ("each slice must leave `main` in
a valid final architecture with no temporary compatibility model"). The commits inside the PR are
sequenced so the AST change, the dispatch change and the consumer change are each reviewable, but
none of them is independently mergeable.

## 13. Coverage

| Evidence | What it pins |
| --- | --- |
| `tests/snapshots/sysml/port_usage_prefix_alternatives.md` | no prefix; every slot alone; all three directions; `abstract` and `variation`; `snapshot` and `timeslice`; one and several extension keywords; the full legal order; materially distinct combinations; absolute/relative/quoted extension references; named, short-name, anonymous and `:>>` shapes; typing, multiplicity with `ordered`/`nonunique`, subsets, redefines, references, crosses, intersects, values, both body forms; `MemberPrefix` visibility beside the prefix |
| `tests/snapshots/sysml/port_usage_prefix_owning_scopes.md` | a materially different prefix in every scope of §3, and identical projection for identical syntax across scopes |
| `tests/snapshots/sysml/port_usage_prefix_recovery.md` | every case in §10 except the unmatched brace, plus a valid sibling after each |
| `tests/snapshots/sysml/port_usage_prefix_scope_recovery.md` | §6.1: an unterminated malformed run in each of the four brace scopes that own a port usage and resynchronize on their own table, each followed by a visibility-prefixed port usage and two further siblings |
| `tests/snapshots/sysml/port_usage_prefix_unterminated.md` | the unmatched-brace state, which subsumes every other member and so needs its own fixture |
| `tests/port_usage_prefix_owning_layer.rs` | arena rollback after refused speculation; strict/editor equivalence; parse→format→reparse and format idempotence per slot; envelope rejection of a wrong-token span, a wrong alternative, out-of-order slots, a non-`#` sigil, a dangling extension identity and a keyword belonging to another member |
| `tests/snapshots/spec42/**` | the pinned corpus files this slice changes |
