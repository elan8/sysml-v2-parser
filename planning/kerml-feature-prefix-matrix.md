# KerML `FeaturePrefix` matrix

The `Feature`/`Step`/`Expression`/`BooleanExpression` slice of the KerML feature grammar,
classified against the pinned grammar, with every spelling found in the checked-in fixtures and in
the pinned `sysml-v2-release` corpus. The pin is `docs/conformance-target` (`release_tag=2026-04`,
`grammar_content_hash=fnv1a64:95f39e912f73b917`).

This document is the audit `planning/spec42-upstream-gap-audit.md` "Deferred neighbouring debt"
asks for before the split moves:

> `KermlFeatureMember` and `TypedParameterMember` split one production (`BasicFeaturePrefix`)
> across two AST nodes on whether a direction was authored. They should share one prefix component
> the way the occurrence-usage families now share `OccurrenceUsagePrefix`.

The *shape* of the change -- one struct per named production, one `Option<Node<_>>` per slot,
mutually exclusive alternatives as a single enum-valued slot, presence *is* the property -- is
established once in `planning/occurrence-usage-prefix-matrix.md` §§1-5 and implemented in
`src/ast/occurrence_prefix.rs`. It is not restated here. This document audits only what is
specific to the KerML feature prefix: its productions, its owning scopes, its construction paths,
its dispatch hazards, its corpus, its recovery contract, and -- because the deferred note asks a
question it does not answer -- **whether the two nodes name one production or two**.

`OccurrenceUsagePrefix` is a *different* production and is not this one. `BasicUsagePrefix`/
`RefPrefix` (SysML BNF 275/281) and `BasicFeaturePrefix` (KerML BNF 577) share only the direction
slot; `RefPrefix` continues with `abstract`|`variation` and `constant`, `BasicFeaturePrefix` with
`abstract`, `composite`|`portion` and `var`|`const`. They are not the same slot list, they are not
spelled by the same families, and neither is a sub-component of the other. §11 records that as a
non-goal rather than as a seam this slice widens.

## 1. Authoritative productions

Verbatim from `sysml-v2-release/bnf/KerML-textual-bnf.kebnf` at the pin.

```text
Feature =                                                     -- line 562, clause 8.2.4.3.1
    ( FeaturePrefix
      ( 'feature' | ownedRelationship += PrefixMetadataMember )
      FeatureDeclaration?
    | ( EndFeaturePrefix | BasicFeaturePrefix )
      FeatureDeclaration
    )
    ValuePart? TypeBody

EndFeaturePrefix : Feature =                                  -- line 573
    ( isConstant ?= 'const' { isVariable = true } )?
    isEnd ?= 'end'

BasicFeaturePrefix : Feature =                                -- line 577
    ( direction = FeatureDirection )?
    ( isDerived ?= 'derived' )?
    ( isAbstract ?= 'abstract' )?
    ( isComposite ?= 'composite' | isPortion ?= 'portion' )?
    ( isVariable ?= 'var' | isConstant ?= 'const' { isVariable = true } )?

FeaturePrefix =                                               -- line 584
    ( EndFeaturePrefix ( ownedRelationship += OwnedCrossFeatureMember )?
    | BasicFeaturePrefix
    )
    ( ownedRelationship += PrefixMetadataMember )*

OwnedCrossFeatureMember : OwningMembership =                  -- line 592
    ownedRelatedElement += OwnedCrossFeature

OwnedCrossFeature : Feature =                                 -- line 595
    BasicFeaturePrefix FeatureDeclaration

FeatureDirection : FeatureDirectionKind = 'in' | 'out' | 'inout'   -- line 598
```

The declaration tail, shared by every family below:

```text
FeatureDeclaration : Feature =                                -- line 601
    ( isSufficient ?= 'all' )?
    ( FeatureIdentification ( FeatureSpecializationPart | ConjugationPart )?
    | FeatureSpecializationPart
    | ConjugationPart )
    FeatureRelationshipPart*
ValuePart : Feature = …                                       -- line 1359
TypeBody : Type = ';' | '{' TypeBodyElement* '}'              -- line 431
PrefixMetadataMember : OwningMembership =
    '#' ownedRelatedElement = PrefixMetadataUsage             -- line 1404
```

and the memberships that carry a feature into a type body:

```text
FeatureMember : OwningMembership = TypeFeatureMember | OwnedFeatureMember       -- 519
TypeFeatureMember  : OwningMembership = MemberPrefix 'member' ownedRelatedElement += FeatureElement  -- 523
OwnedFeatureMember : FeatureMembership = MemberPrefix ownedRelatedElement += FeatureElement          -- 526
MemberPrefix : Membership = ( visibility = VisibilityIndicator )?                                    -- 260
TypeBodyElement : Type = NonFeatureMember | FeatureMember | AliasMember | Import                     -- 434
```

### 1.1 Four textual productions spell `FeaturePrefix` and nothing else differs

```text
FeatureElement : Feature =                                    -- line 360
      Feature | Step | Expression | BooleanExpression | Invariant
    | Connector | BindingConnector | Succession | Flow | SuccessionFlow

Step =              FeaturePrefix 'step' FeatureDeclaration ValuePart? TypeBody      -- 863
Expression =        FeaturePrefix 'expr' FeatureDeclaration ValuePart? FunctionBody  -- 895
BooleanExpression = FeaturePrefix 'bool' FeatureDeclaration ValuePart? FunctionBody  -- 908
```

`Feature`, `Step`, `Expression` and `BooleanExpression` are the same sentence four times:
`FeaturePrefix`, one keyword, `FeatureDeclaration`, `ValuePart?`, a body. The keyword is the only
difference in the first four columns, and the body differs only in whether a `ResultExpressionMember`
is admitted (`FunctionBody`, 876). **This is the whole grammatical basis for the merge decision in
§5.**

`Invariant` (913), `Connector` (789), `BindingConnector` (827), `Succession` (841), `Flow` (1303)
and `SuccessionFlow` (1307) also spell `FeaturePrefix`, but each adds its own declaration grammar
(`ConnectorDeclaration`, `SuccessionDeclaration`, `'inv' ('true'|'false')?`), so they are neither
this slice's node nor candidates to merge into it. §11 records them as deliberate non-goals: they
keep whatever partial prefix fields they have until their own slices.

### 1.2 `member` is on the membership, not on the prefix

`TypeFeatureMember = MemberPrefix 'member' FeatureElement` (523). `member` is the discriminator
between the two `FeatureMember` alternatives, exactly as `visibility` is `MemberPrefix`'s only
slot. It precedes the whole prefix, and the corpus proves the ordering: `member abstract feature
carSpeed : Real;` (`Variable Feature Examples/TimeVaryingCarDriver.kerml:100`). It is therefore
**not** a `BasicFeaturePrefix` slot and does not move into the component.

### 1.3 `const` appears in both alternatives and is still unambiguous

`const` is the last slot of `BasicFeaturePrefix` and the first of `EndFeaturePrefix`. Because the
two are alternatives of one choice, `const` before `end` is `EndFeaturePrefix`, and `const` with no
`end` is `BasicFeaturePrefix`'s `isVariable`/`isConstant` slot. There is no spelling in which both
apply, and no spelling in which `const` follows `end`.

`{ isVariable = true }` on both `const` slots is an abstract-syntax action, not concrete syntax:
`const` sets `isConstant` *and* `isVariable`. The component stores the authored alternative and
derives `isVariable`; it does not store a second flag that could disagree with the span.

### 1.4 Conformance identity

| Fact | Value |
| --- | --- |
| Release tag | `2026-04` |
| Grammar content hash | `fnv1a64:95f39e912f73b917` |
| SysML productions | 350 |
| KerML productions | 290 |
| `Feature` line | 562 |
| `EndFeaturePrefix` line | 573 |
| `BasicFeaturePrefix` line | 577 |
| `FeaturePrefix` line | 584 |
| `OwnedCrossFeatureMember` / `OwnedCrossFeature` lines | 592 / 595 |
| `Step` / `Expression` / `BooleanExpression` lines | 863 / 895 / 908 |
| `TypeFeatureMember` / `OwnedFeatureMember` lines | 523 / 526 |

## 2. Nesting, cardinality and legal token order

```
[public|private|protected]                      -- MemberPrefix, on the membership
[member]                                        -- TypeFeatureMember, on the membership
                                                -- then FeaturePrefix, one of:
  [const] end [ OwnedCrossFeature ]             --   EndFeaturePrefix (+ optional cross feature)
  [in|out|inout] [derived] [abstract]           --   BasicFeaturePrefix
      [composite|portion] [var|const]
('#' QualifiedName)*                            -- PrefixMetadataMember*
('feature'|'step'|'expr'|'bool'|'#' QualifiedName)  -- the keyword slot
[all] [name] [FeatureSpecializationPart] …      -- FeatureDeclaration
[= expr | := expr | default …] (';' | '{' … '}')
```

Cardinality and exclusivity, per slot:

| Slot | Cardinality | Kind |
| --- | --- | --- |
| `EndFeaturePrefix` vs `BasicFeaturePrefix` | exactly 1 | **one choice, two alternatives -- mutually exclusive** |
| `const` (within `EndFeaturePrefix`) | 0..1 | independent, *precedes* `end` |
| `end` (within `EndFeaturePrefix`) | 1 | required -- it is what selects the alternative |
| `OwnedCrossFeatureMember` | 0..1 | only on the `EndFeaturePrefix` alternative |
| `in`/`out`/`inout` | 0..1 | one slot, three alternatives -- mutually exclusive |
| `derived` | 0..1 | independent |
| `abstract` | 0..1 | independent |
| `composite` / `portion` | 0..1 | one slot, two alternatives -- mutually exclusive |
| `var` / `const` | 0..1 | one slot, two alternatives -- mutually exclusive |
| `'#' QualifiedName` | 0..* | ordered, repeatable, authored order retained |

Everything else repeated is malformed: `in out`, `derived derived`, `abstract abstract`,
`composite portion`, `var const`, `end end`. The grammar gives no slot for the second token, so the
member is refused, not "last wins".

Three consequences the rest of this document depends on:

1. **A direction beside `end` is unauthorable.** `in`/`out`/`inout` is `BasicFeaturePrefix`'s first
   slot; `end` selects the *other* alternative. `in end feature x;` and `end in feature x;` are
   both outside the grammar. Any model with `direction` and `is_end` as sibling fields makes them
   representable; a model with a two-variant choice does not.
2. **A direction beside `derived`/`composite`/`portion`/`var`/`const` is perfectly legal.** They
   are all `BasicFeaturePrefix` slots, in that order. `in derived feature x;`,
   `in composite feature x;`, `in var feature x;`, `in portion feature x;` and
   `out abstract feature x;` are all grammatical.
3. **`all` is not a prefix slot.** `FeatureDeclaration = ( isSufficient ?= 'all' )? …` (601), so it
   follows the keyword. `feature all q;` is the only order.

### 2.1 The two `PrefixMetadataMember` positions are one abstract-syntax slot

`Feature`'s first alternative writes `ownedRelationship += PrefixMetadataMember` **twice**: once as
`FeaturePrefix`'s repeatable tail (588), once as the alternative to the `feature` keyword (564).
Both append to the same `ownedRelationship` collection of the same `Feature`, so
`#Classified #Security feature z1;` and `abstract #Classified z2;` (both
`Simple Tests/MetadataTest.kerml:32,33`) differ only in whether a keyword follows the run -- not in
what the run *is*. One ordered sequence plus an optional keyword represents both exactly, and no
third state is introduced.

## 3. Legal owning scopes

Every scope this parser dispatches one of the three nodes from, the AST enum that owns it, and the
recovery starter table that scope synchronizes on.

| # | Scope | Owning AST enum | Parser entry | Recovery starters |
| --- | --- | --- | --- | --- |
| 1 | package / namespace / root, keyword-guarded | `PackageBodyElement::KermlFeatureMember` | `package::package_body_element` (`try_package_body_dispatch!`, `package.rs:1790`) | `PACKAGE_BODY_GRAMMAR` |
| 2 | package / namespace / root, unguarded late retry | `PackageBodyElement::KermlFeatureMember` | `package::package_body_element` (`package.rs:2045`) | `PACKAGE_BODY_GRAMMAR` |
| 3 | KerML type body (`struct`, `assoc`, `assoc struct`, `datatype`, `classifier`, `class`, `behavior`, `function`, `predicate`, `interaction`, `metaclass`) | `CalcDefBodyElement::KermlFeature` / `::TypedParameter` / `::EndMember` | `constraint::calc_def_body_element` | `CALC_DEF_BODY_STARTERS` |
| 4 | `calc def` / `calc` usage body | same as 3 | `constraint::calculation_body_element` -> `calc_def_body_element` | `CALC_DEF_BODY_STARTERS` |
| 5 | feature / invariant / connector / binding / succession / return member bodies (recursive) | same as 3 | `constraint::calc_def_body` | `CALC_DEF_BODY_STARTERS` |
| 6 | attribute / item / class body | `AttributeBodyElement::KermlFeature` | `attribute::attribute_body_element` (`attribute.rs:271`) | `ATTRIBUTE_BODY_STARTERS` |
| 7 | dependency / alias / import leaf body | `RelationshipBodyElement::KermlFeature` | `body::relationship_body_member` (`body.rs:70`, tried first) | `RELATIONSHIP_BODY_STARTERS` |

Scopes 3-5 are the same member set (`CalcDefBodyElement`) reached through three entry points; they
are listed separately because they synchronize independently and because only scopes 3-5 own a
`TypedParameterMember` or a `KermlEndMember` at all.

`TypedParameterMember` reaches **only** scopes 3-5. `KermlEndMember` reaches **only** scopes 3-5.
`KermlFeatureMember` reaches all seven. That asymmetry is itself evidence: the directed spelling of
a production is not owned by fewer scopes than its undirected spelling in any grammar -- it is
owned by fewer scopes *here* because a second node was added at one dispatch site instead of
widening the first node's prefix.

## 4. Construction paths

Every place one of the three values is built, before this slice.

| Path | File | Prefix it parses | Gap |
| --- | --- | --- | --- |
| `kerml_feature_member_inner` | `src/parser/constraint.rs:1102-1279` | `member` `derived` `abstract` (`composite`\|`portion`) `var` `const` `end` -- in that order, each `opt`, no spans | no direction; `var`+`const` both settable; `end` is a sibling bool; no `PrefixMetadataMember` tail; no `OwnedCrossFeature` |
| `typed_parameter_member_inner` | `src/parser/constraint.rs:546-672` | direction (**required**) `abstract` -- and nothing else | no `derived`, `composite`, `portion`, `var`, `const`; direction cannot be absent; no spans |
| `kerml_end_member_inner` | `src/parser/constraint.rs:945-997` | `const` `end`, then name/multiplicity/subsets, then delegates to `kerml_feature_member` | no `BasicFeaturePrefix` on the cross feature; no `ordered`/`nonunique`; the delegate re-offers `end`/`const` |

Three constructors, three prefixes, one production. Each is `reference_transaction`-wrapped, so a
refused attempt already rolls its arena entries back; that property is preserved.

## 5. Merge or share: the decision, and the grammar it rests on

`AGENTS.md` is explicit in both directions -- "a desire to reduce type count [is] not sufficient
evidence" for factoring, and "every syntactic fact has one representation". The question is
therefore not whether three types are worse than one, but **whether the three name one production
or three**.

### 5.1 The differences the deferred note flags are artifacts, not productions

The task framing asks whether `TypedParameterMember`'s `KermlParameterKind` and calc-body, and
`KermlFeatureMember`'s `KermlFeatureKind`, type relationships, `crosses`, `chains` and `inverse_of`,
name different productions. Field by field:

| Difference | Verdict | Evidence |
| --- | --- | --- |
| `KermlParameterKind::{Expr, Bool, Feature, Step}` vs `KermlFeatureKind::{Feature, Step, Expr, Bool}` | **the same four productions** | `Feature` (562), `Step` (863), `Expression` (895), `BooleanExpression` (908) are `FeaturePrefix <kw> FeatureDeclaration ValuePart? Body`, differing only in `<kw>` |
| `KermlParameterKind::Calc` | **a different production** | `CalculationUsage = OccurrenceUsagePrefix 'calc' ActionUsageDeclaration CalculationBody` (SysML BNF 1355). Not `FeaturePrefix`, not KerML. §5.3 |
| `KermlFeatureMember`'s `crosses`, `chains`, `inverse_of`, `type_relationships` | **`FeatureRelationshipPart*`**, the shared declaration tail | `FeatureRelationshipPart = TypeRelationshipPart \| ChainingPart \| InvertingPart \| TypeFeaturingPart` (605), reached from `FeatureDeclaration` (601), which *both* alternatives of `Feature` end with |
| `TypedParameterMember`'s calc-body vs `KermlFeatureMember`'s calc-body | **identical** -- both are `crate::ast::CalcDefBody` | `constraint.rs:646` and `constraint.rs:1250` both call `calc_def_body` |
| `TypedParameterMember`'s absent `subsets`/`references`/`typing`-by-conjugation | **not a production boundary** | it is `FeatureSpecializationPart` (620), which `FeatureDeclaration` gives to every alternative. The directed node simply parses less of it |

So the only field on either node that names a production the other does not is
`KermlParameterKind::Calc`, and it names a production that is neither `Feature`, `Step`,
`Expression` nor `BooleanExpression`, and whose prefix is `OccurrenceUsagePrefix`, not
`FeaturePrefix`.

### 5.2 The split is observably discriminated by an optional slot

Parsed today, in every scope of §3 (measured; see §9.2):

```
feature a;              -> (kerml-feature)        derived feature e;   -> (kerml-feature)
in feature b;           -> (typed-parameter)      composite step u;    -> (kerml-feature)
in expr r;              -> (typed-parameter)      abstract expr v;     -> (kerml-feature)
in step t;              -> (typed-parameter)      end bool w;          -> (kerml-feature)
```

The same four productions land on two different AST types according to whether
`BasicFeaturePrefix`'s **first optional slot** was authored. That is not a grammatical distinction;
`( direction = FeatureDirection )?` is one `?` in one production.

The cost is not theoretical. `TypedParameterMember` models only `direction` + `abstract`, so every
other legal combination of a direction with a `BasicFeaturePrefix` slot is **refused**:

| Input | Grammar | Today |
| --- | --- | --- |
| `in derived feature q;` | legal (578 then 579) | `recovered_calc_body_element`, malformed |
| `in composite feature o;` | legal (578 then 581) | malformed |
| `in var feature p;` | legal (578 then 582) | malformed |
| `in portion feature s;` | legal (578 then 581) | malformed |
| `out abstract feature r;` | legal | accepted -- the one combination the second node models |

Nothing is wrong with those inputs; there is simply no field for them on the node the direction
routed them to.

### 5.3 `in calc` is the one thing that must *leave*

`in calc scenario : NominalScenario;` (`sysml/src/validation/10-Analysis and Trades/10c-Fuel
Economy Analysis.sysml:80`) is a SysML `CalculationUsage`, whose prefix is `OccurrenceUsagePrefix`
(SysML 1355/564). It reaches `TypedParameterMember` only because `calc_def_body_element` tries the
direction-gated arm (`constraint.rs:1513`) before its `calc` arm (`constraint.rs:1540`) --
`ast::CalcUsage` already carries `direction`, `is_abstract` and `is_reference`, and
`abstract calc c2;` already parses to `(calc-usage)` in the same body. So the fifth kind is a
dispatch-order artifact, and keeping it would put an `OccurrenceUsagePrefix` production on a
`BasicFeaturePrefix`-owning node -- precisely the "factor only at boundaries owned by an
authoritative grammar production" clause. `KermlParameterKind::Calc` is therefore deleted and
`in`/`out`/`inout calc` is routed to `CalcUsage`, which is where the production already lives.

### 5.4 `KermlEndMember` is the same production a third time

`end guardedLink [0..1] feature constrainedHBLink: HappensBefore;`
(`Kernel Semantic Library/TransitionPerformances.kerml:61`) is one `Feature`: `FeaturePrefix` took
its `EndFeaturePrefix ( OwnedCrossFeatureMember )?` alternative, then `'feature'`, then
`FeatureDeclaration`. This parser models it as a `KermlEndMember` that **wraps** a
`KermlFeatureMember` -- and the wrapped node carries its own `is_end` and `is_const`. End-ness is
representable in two places at once, and `const` in two places at once, on one member.

Modelling `FeaturePrefix`'s choice faithfully therefore forces the third node too: once
`FeaturePrefix::End { prefix, cross }` exists on the merged node, `KermlEndMember` is a second
representation of the alternative that enum variant *is*.

### 5.5 Decision

**Merge.** One node, `KermlFeature`, models `Feature | Step | Expression | BooleanExpression`, and
carries one grammar-owned `prefix: FeaturePrefix`. `TypedParameterMember`, `KermlParameterKind` and
`KermlEndMember` are deleted; `CalcDefBodyElement` loses `TypedParameter` and `EndMember`.

This is not "three types were similar". It is: four textual productions with a byte-identical first
four columns, split across three Rust types by (a) whether one optional slot was authored, (b)
whether a second optional slot was authored, with the split costing five legal spellings that are
refused outright and putting two mutually-exclusive facts in independently-settable fields.

### 5.6 After

```rust
// src/ast/feature_prefix.rs -- one struct per named production
pub enum FeaturePortionKind { Composite, Portion }   // ('composite'|'portion'), BNF 581
pub enum FeatureVariability { Var, Const }           // ('var'|'const'),         BNF 582

pub struct BasicFeaturePrefix {                      // BNF 577
    pub direction:    Option<Node<InOut>>,
    pub derived_span: Option<Span>,
    pub abstract_span: Option<Span>,
    pub portioning:   Option<Node<FeaturePortionKind>>,
    pub variability:  Option<Node<FeatureVariability>>,
}

pub struct EndFeaturePrefix {                        // BNF 573
    pub constant_span: Option<Span>,
    pub end_span: Span,                              // required: the struct's existence IS isEnd
}

pub struct OwnedCrossFeature { … }                   // BNF 595: BasicFeaturePrefix FeatureDeclaration

pub enum FeaturePrefixHead {                         // BNF 584, the choice
    End  { prefix: EndFeaturePrefix, cross: Option<Box<Node<OwnedCrossFeature>>> },
    Basic(BasicFeaturePrefix),
}

pub struct FeaturePrefix {                           // BNF 584
    pub head: FeaturePrefixHead,
    pub metadata_keywords: Vec<Node<UsageExtensionKeyword>>,   // PrefixMetadataMember*, BNF 1404
}
```

Why each shape:

- `FeaturePrefixHead` is an enum, not two optional fields, because `FeaturePrefix` is a choice.
  `in end feature x;` is unrepresentable: `direction` exists only inside `BasicFeaturePrefix`, and
  the `End` variant does not contain one. `tests/…/kerml_feature_prefix_recovery.md` keeps the
  conclusion the earlier `end`-versus-direction fixture established.
- `EndFeaturePrefix::end_span` is a bare `Span`, not an `Option`, because `isEnd ?= 'end'` is the
  one *required* token in that production. There is no `EndFeaturePrefix` without `end`.
- `cross` hangs off the `End` variant only, because `OwnedCrossFeatureMember` appears only in that
  alternative. `in guardedLink [0..1] feature x;` is unrepresentable.
- `portioning` and `variability` are one enum-valued slot each, so `composite portion` and
  `var const` are unrepresentable. Today the first is refused by accident and the second is
  **accepted with both booleans set** (§9.3).
- Every independent modifier is an `Option<Span>`: presence *is* the property, so there is no
  second boolean to drift from the span, and emission writes the keyword because the author did.
- `metadata_keywords` reuses `crate::ast::UsageExtensionKeyword`, which already models
  `PrefixMetadataMember` (`'#' OwnedFeatureTyping`) for the occurrence seam. One production, one
  representation. The type keeps its SysML-derived name; §11 records the rename as debt rather than
  churning the migrated families for it.

On the node itself, `kind: KermlFeatureKind` + `has_kind_keyword: bool` collapses to
`kind: Option<Node<KermlFeatureKind>>` -- the same "presence is the property" rule, removing a
boolean that could disagree with the value beside it. `is_member` stays on the node (it is
`TypeFeatureMember`'s discriminator, §1.2), as do `is_all` (`FeatureDeclaration`, §2) and the whole
declaration tail.

## 6. FIRST tokens and recovery implications, per scope

FIRST(`Feature`) with every optional slot empty is the kind keyword; with the prefix authored it is
the union of every prefix opener:

```
member  in  out  inout  derived  abstract  composite  portion  var  const  end  #
feature  step  expr  bool
```

Sixteen tokens, all of which are recovery boundaries in every scope of §3. Scope starter tables
before this slice:

| Table | Already listed | Missing |
| --- | --- | --- |
| `CALC_DEF_BODY_STARTERS` (`lex.rs:274`) | `@ doc in out inout return calc part` | **`feature`, `step`, `expr`, `bool`, `member`, `derived`, `abstract`, `composite`, `portion`, `var`, `const`, `end`, `#`, and the three visibility keywords** |
| `ATTRIBUTE_BODY_STARTERS` (`attribute.rs:27`) | `derived abstract feature member var composite portion step expr bool end` | `const`, `#` |
| `RELATIONSHIP_BODY_STARTERS` (`lex.rs:331`) | `doc comment rep @ feature` | the other eleven prefix openers and the three other kind keywords |
| `PACKAGE_BODY_GRAMMAR` (`grammar_scope.rs:195`) | `abstract`, `derived`, `end`, `feature` | `member`, `composite`, `portion`, `var`, `const`, `step`, `expr`, `bool` |

`CALC_DEF_BODY_STARTERS` is the serious one: it is the recovery table for the scope in which
feature members are the *dominant* member kind, and it lists none of them. A malformed member
inside a KerML type body therefore scans past every following feature member until it finds a `;`
or a balanced block, which is the "recovery must not consume valid later siblings" clause of the
parsing contract failing silently. It is visible today in
`tests/snapshots/kerml/classifier_declaration_recovery.md:34` and
`tests/snapshots/spec42/kerml/types.md:108`, where a `malformed` node is followed by exactly one
surviving `kerml-feature` because the scan happened to stop there.

This is a pre-existing defect rather than one the slice introduces, but it is a defect in exactly
the scopes whose tables this slice completes, and leaving them unlisted would make the completion
decorative -- the same finding `planning/port-usage-prefix-matrix.md` §6.1 recorded for the four
brace scopes it repaired. The tables are completed to the member set each `*_body_element` actually
dispatches, not merely to FIRST(`Feature`).

## 7. Competing productions and parser-precedence hazards

| Token | Competing production | Where it wins before this slice | Resolution |
| --- | --- | --- | --- |
| `in`/`out`/`inout` | `in_out_decl` (keyword-less directed parameter, `action.rs:332`) | after the typed-parameter attempt fails | `in_out_decl` already hard-rejects `item`/`part`/`occurrence`/`expr`/`bool`/`feature`/`calc` as the following word. **`step` is missing from that list**, so `in step foo;` in an *action* body parses as an `InOutDecl` named `step`. The list gains `step` |
| `in`/`out`/`inout` + `calc` | `CalculationUsage` (SysML 1355) | never -- the typed-parameter arm claims it first | the merged node refuses `calc`, so `in calc c1;` reaches `calc_usage`, which already models direction (§5.3) |
| `abstract` | `part_usage`, every `*_def` parser's `DefinitionPrefix` | `part_usage` is tried first at `constraint.rs:1379` | unchanged -- the feature arm keeps its speculative gate (`… && kerml_feature_member(input).is_ok()`), so `abstract part def P { … }` still reaches its own family |
| `end` | `kerml_end_member` (scopes 3-5), `EndDecl` (connector/interface scopes) | the feature arm's speculative gate runs first, so plain `end feature o;` is a `KermlFeatureMember` and `end g [0..1] feature y;` falls through to `kerml_end_member` | one node claims both. `EndDecl` in connector and interface scopes is a different production (`ConnectorEnd`, 812) and is untouched |
| `const` | `kerml_end_member`, and the bare-expression fallback | speculative gate | unchanged in kind; the gate now covers one node instead of two |
| `#` | `metadata_keyword_usage` (`#Tag { … }`, `#Tag def X`), `MetadataAnnotation` | **always** -- `#Classified #Security feature z1;` parses as two standalone metadata members plus an unprefixed feature | the feature arm gets first refusal through the same speculative trial (`kerml_feature_member(input).is_ok()`), which claims `#` only when the *whole* feature member parses. `#Tag { … }` and `#Tag def X` own a body or a definition, so the trial fails and they keep their place |
| `feature` | `feature_decl`, `ReturnKindKeyword::Feature` (`return feature …`), connector-end `feature` | each is reached from a scope or position this slice does not change | unchanged |

The rule this slice keeps, unchanged from the port slice: **the selected parser claims a prefix
only when the whole production is viable.** The merged parser parses the prefix and then requires
either a kind keyword, a metadata keyword, or an authored prefix plus a declaration; if none is
present the parse fails, `reference_transaction` rolls back every `#tag` identity it allocated, and
the member falls through to whichever sibling really owns it.

## 8. Sibling Pilot comparison

`~/Documents/GitHub/SysML-v2-Pilot-Implementation/org.omg.kerml.xtext/src/org/omg/kerml/xtext/KerML.xtext`,
which is newer than this repository's 2026-04 pin.

| Production | Pilot | Pin | Material? | Followed |
| --- | --- | --- | --- | --- |
| `BasicFeaturePrefix` | `fragment` with the same five slots in the same order (514-520) | 577 | no -- the Pilot omits only the `{ isVariable = true }` action, which is abstract syntax | both |
| `EndFeaturePrefix` | `( isConstant ?= 'const')? isEnd ?= 'end'` (510-512) | 573 | no -- identical | both |
| `FeaturePrefix` | `( EndFeaturePrefix ( ownedRelationship += OwnedCrossingFeatureMember )? \| BasicFeaturePrefix ) ( ownedRelationship += PrefixMetadataMember )*` (522-527) | 584 | **no** -- `OwnedCrossFeatureMember` is renamed `OwnedCrossingFeatureMember`, same shape | pin's name |
| `Feature` | identical two-alternative body (537-545) | 562 | no | both |
| `Step` / `Expression` / `BooleanExpression` | `FeaturePrefix 'step'/'expr'/'bool' …` (912, 956, 974) | 863 / 895 / 908 | no | both |

No Pilot behaviour newer than the pin is adopted. The Pilot confirms the one structural claim this
slice rests on: `EndFeaturePrefix` and `BasicFeaturePrefix` are alternatives of a choice in both,
so a direction beside `end` is unauthorable in both.

## 9. Corpus survey

### 9.1 Pinned `sysml-v2-release` corpus (94 `.kerml` + 309 `.sysml` files)

Every prefix run written immediately before a `feature`/`step`/`expr`/`bool`/`calc` keyword at
member position, over comment- and string-stripped source. `#Ref` runs normalized to `#`.

| Spelling | Count | Representative location | Class before this slice |
| --- | --- | --- | --- |
| *(no prefix)* `feature` | 462 | `Named Collection Members Example/VehicleTanks.kerml:8` | valid and represented |
| *(no prefix)* `calc` | 151 | `Domain Libraries/Quantities and Units/MeasurementRefCalculations.sysml:14` | a `CalcUsage`; not this production |
| `var feature` | 118 | `Simple Tests/Features.kerml:59` | valid, no span |
| `end feature` | 117 | `Massed Thing Example/MassedThings.kerml:10` | valid, no span |
| `derived var feature` | 106 | `Kernel Semantic Library/KerML.kerml:12` | valid, no spans |
| `member feature` | 77 | `Variable Feature Examples/TimeVaryingFeatures.kerml:28` | valid; `member` is membership syntax (§1.2) |
| *(no prefix)* `step` | 52 | `Behavior Examples/TakePicture.kerml:11` | valid and represented |
| `derived composite var feature` | 28 | `Kernel Semantic Library/KerML.kerml:13` | valid; the longest run in the corpus |
| `in feature` | 20 | `Vehicle Example/VehicleUsages.kerml:97` | valid; **a `TypedParameterMember`** |
| `abstract feature` | 19 | `Simple Tests/Features.kerml:25` | valid, no span |
| `portion feature` | 18 | `Simple Tests/Classes.kerml:8` | valid, no span |
| `in expr` | 16 | `Kernel Function Library/ControlFunctions.kerml:24` | valid; a `TypedParameterMember` |
| `abstract calc` | 13 | `Domain Libraries/Analysis/TradeStudies.sysml:12` | a `CalcUsage`; not this production |
| `composite step` | 13 | `Simple Tests/Behaviors.kerml:5` | valid, no span |
| `abstract expr` | 12 | `Kernel Semantic Library/Performances.kerml:197` | valid, no span |
| `composite feature` | 12 | `Mass Roll-up Example/Vehicles_3.kerml:12` | valid, no span |
| `abstract step` | 7 | `Kernel Semantic Library/Performances.kerml:190` | valid, no span |
| `member step` | 6 | `Variable Feature Examples/Enhancements/TimeVaryingSteps.kerml:4` | valid |
| *(no prefix)* `expr` | 5 | `Simple Tests/Expressions.kerml:50` | valid and represented |
| `in calc` | 5 | `Domain Libraries/Analysis/TradeStudies.sysml:61` | **a `TypedParameterMember`** -- should be a `CalcUsage` (§5.3) |
| *(no prefix)* `bool` | 4 | `Kernel Semantic Library/Triggers.kerml:37` | valid and represented |
| `inout feature` | 4 | `Simple Tests/Expressions.kerml:32` | valid; a `TypedParameterMember` |
| `out feature` | 4 | `Vehicle Example/VehicleUsages.kerml:92` | valid; a `TypedParameterMember` |
| `composite var feature` | 3 | `Simple Tests/Features.kerml:53` | valid, no spans |
| `derived feature` | 3 | `Kernel Semantic Library/Objects.kerml:194` | valid, no span |
| `const end feature` | 2 | `Simple Tests/Associations.kerml:16` | valid, no spans |
| `in step` | 2 | `Simple Tests/Expressions.kerml:24` | valid; a `TypedParameterMember` |
| `# # feature` | 1 | `Simple Tests/MetadataTest.kerml:32` | **parsed as two sibling members plus an unprefixed feature** |
| `abstract var feature` | 1 | `Variable Feature Examples/TimeVaryingCarDriver.kerml:53` | valid, no spans |
| `derived abstract var feature` | 1 | `Kernel Semantic Library/KerML.kerml:86` | valid, no spans |
| `end bool` | 1 | `Kernel Semantic Library/TransitionPerformances.kerml:62` | valid -- `FeaturePrefix` before `'bool'` |
| `in bool` | 1 | `Kernel Semantic Library/Observation.kerml:78` | valid; a `TypedParameterMember` |
| `member abstract feature` | 1 | `Variable Feature Examples/TimeVaryingCarDriver.kerml:100` | valid; pins `member` before the prefix |

Separately, the `EndFeaturePrefix ( OwnedCrossFeatureMember )?` spelling -- an `end`, then a cross
feature, then the kind keyword -- occurs **46** times across 9 files:

| Cross-feature spelling | Representative location |
| --- | --- |
| `end [0..1] feature cart: ShoppingCart[1];` | `Association Examples/ProductSelection_N_ary.kerml:9` |
| `end inCart[0..1] feature cart: ShoppingCart[1];` | `Association Examples/ProductSelection_N_ary.kerml:16` |
| `end [0..*] nonunique feature selectedProduct: Product[1];` | `Association Examples/ProductSelection_OwnedEnds.kerml:12` |
| `const end [1] feature a;` | `Simple Tests/Associations.kerml:16` |
| `end guardedLink [0..1] feature constrainedHBLink: HappensBefore;` | `Kernel Semantic Library/TransitionPerformances.kerml:61` |
| `end [payloadNum] feature transferPayload references payload subsets transferSource.sourceOutput;` | `Kernel Semantic Library/Transfers.kerml:107` |
| `end happensWhile [1..*] subsets timeCoincidentOccurrences feature thatOccurrence: Occurrence redefines longerOccurrence;` | `Kernel Semantic Library/Occurrences.kerml:767` |
| `end withinBoth subsets spaceTimeCoincidentOccurrences feature thatOccurrence redefines largerOccurrence, HappensWhile::thatOccurrence;` | `Kernel Semantic Library/Occurrences.kerml:821` |

Not present anywhere in the pinned corpus: any direction combined with `derived`, `composite`,
`portion`, `var` or `const`; any `end` combined with a direction; any duplicate modifier; any
invalid ordering; any malformed prefix. Every corpus spelling is grammatical.

### 9.2 Measured behaviour of the current parser

A scratch fixture placing the full slot inventory in each of `struct`, `assoc struct`, `datatype`,
`classifier`, `behavior` and `function` bodies produces **identical** classification in all six:

```
feature a;                             (kerml-feature)
in feature b; out c; inout d;          (typed-parameter)   <- the split
derived e; abstract f; composite g;    (kerml-feature)
portion h; var i; const j;             (kerml-feature)
derived abstract composite var k;      (kerml-feature)
member m; member abstract n;           (kerml-feature)
end o; const end p;                    (kerml-feature)
in expr r; in bool s; in step t;       (typed-parameter)   <- the split
composite step u; abstract expr v;     (kerml-feature)
end bool w;                            (kerml-feature)
portion redefines x1 = 1;              (kerml-feature)     keyword-less alternative
#Tag feature z1;                       (metadata-keyword-usage) + (kerml-feature)   <- tail dropped
#Tag #Tag2 feature z2;                 (metadata-keyword-usage) x2 + (kerml-feature)
abstract #Tag z3;                      malformed
end g [0..1] feature y1 : T;           (end-member)        <- third node
end [0..*] nonunique feature y2 : T;   malformed
const end [1] feature y3;              (end-member)
end withinBoth subsets zz feature y4   (end-member)
```

Both projections are bare markers -- `(kerml-feature)` and `(typed-parameter)` carry no fields at
all in a type body -- so none of the invariants above is observable in a snapshot today. Only
`FORMAT` shows them, and `FORMAT` re-derives each keyword from a boolean, so a round trip that
loses a fact loses it identically on both sides and the gate runs straight through the defect. That
is the same failure mode `planning/port-usage-prefix-matrix.md` §7.1 recorded.

### 9.3 Classification of every spelling found or constructed

| Class | Spellings |
| --- | --- |
| valid and represented exactly | *(no prefix)* `feature`/`step`/`expr`/`bool` in every scope of §3 |
| valid but partially represented | `member`, `derived`, `abstract`, `composite`, `portion`, `var`, `const`, `end`, and all three directions -- accepted, but as spanless booleans, so no consumer can point at the authored keyword and the semantic projection shows none of them |
| valid but split across two nodes | every directed spelling: `in feature`, `out feature`, `inout feature`, `in expr`, `in bool`, `in step` |
| valid but split across a third node | every `end` with a cross feature: 46 corpus sites |
| valid but parsed with the wrong shape | `in calc …` -- a `TypedParameterMember`, not the `CalculationUsage` it is (§5.3) |
| valid but parsed and discarded | `#Classified #Security feature z1;` -- the `PrefixMetadataMember*` tail becomes two standalone sibling members and the feature loses its prefix entirely |
| valid but unsupported | `in derived …`, `in composite …`, `in var …`, `in portion …`, `in const …` (all legal, all recovered as malformed); `abstract #Classified z2;` (the metadata-as-keyword spelling); `end [0..*] nonunique feature …` (a pinned corpus spelling, `ProductSelection_OwnedEnds.kerml:12`) |
| **illegal but accepted** | `var const feature b;` -- both booleans set, no diagnostic, re-emitted verbatim; `derived end feature j;`, `abstract end feature k;`, `composite end feature l;`, `var end feature m;` -- none spelled by `EndFeaturePrefix`; `end const feature n;` -- `const` follows `end`, which no production allows |
| **illegal, accepted and mangled** | `abstract derived feature c;` -- split into a bare expression member `'abstract';` *and* a separate `derived feature c;`, so an illegal order silently becomes two members and `abstract` becomes a quoted name reference. `end in feature i;` -- re-emitted as ``end 'in' feature i;``, the direction keyword captured as a quoted `NAME` |
| malformed | none in the pinned corpus. Constructed for coverage: `composite portion`, `in out`, `var var`, `composite composite`, `feature derived`, `in end`, `#;`, `# feature`, `#A:: feature`, prefix with no keyword and no declaration |

The two "illegal, accepted and mangled" rows are why this is a correctness fix rather than a
representation change: an input outside the grammar produced a document with no diagnostic and a
different member count than the author wrote.

## 10. Recovery contract

| Input | Required outcome |
| --- | --- |
| malformed member before any prefix FIRST token | recovery stops at the *first* prefix token, not at the kind keyword; the prefixed feature after it parses |
| malformed content between two prefix slots (`derived @ var feature f;`) | one recovery node retaining the exact malformed span |
| a comment between two prefix slots (`derived /* why */ var feature f;`) | trivia; the feature parses |
| invalid ordering (`abstract derived feature x;`, `feature derived x;`, `end const feature x;`) | refused; one recovery node retaining the exact malformed span. Never silently split into two members |
| direction beside `end` (`in end feature x;`, `end in feature x;`) | refused; recovery node. Unrepresentable in the AST, so it cannot be constructed either |
| duplicate direction (`in out feature x;`) | refused; recovery node |
| `composite portion feature x;` / `var const feature x;` | refused; recovery node. Unrepresentable in the AST |
| repeated independent singleton (`derived derived`, `abstract abstract`, `end end`) | refused; recovery node |
| `derived end`, `abstract end`, `composite end`, `var end` | refused; recovery node -- `EndFeaturePrefix` spells neither |
| missing keyword *and* missing declaration after a valid prefix (`derived var;`) | refused; recovery node covering the prefix and its terminator; no fabricated kind keyword |
| `#;`, `# feature x;`, `#Tag:: feature x;` | refused, with the exact authored span reported and **no reference allocated** -- the whole parse runs inside `reference_transaction` |
| `#Tag { … }` and `#Tag def X` beside a `#Tag feature x;` | the first two stay standalone metadata members; only the third is a prefix |
| malformed prefix before a named, an anonymous, and a `:>>`-led feature | one recovery node each; later siblings survive |
| malformed feature followed by several valid siblings | all siblings survive, in every scope of §3 |
| nested brace bodies after a prefixed feature; unmatched braces | balanced tracking unchanged from the body container |
| prefix-like words inside a quoted name, a string literal, escaped text, a line comment and a block comment | not prefix tokens |
| every materially distinct owning scope | same outcome |
| strict/editor equivalence on diagnostic-free input | identical documents |
| speculative parse that consumes metadata references then fails | arena unchanged |

Recovery never fabricates a prefix component, and a refused prefix never becomes an unprefixed
feature: the refusal is at the whole production, so the member becomes one recovery node spanning
the authored text.

## 11. Explicit non-goals

- **No SysML usage family moves.** `OccurrenceUsagePrefix`, `BasicUsagePrefix`, `RefPrefix`,
  `UsagePrefix`, `ControlNodePrefix`, `DefinitionPrefix` and `OccurrenceDefinitionPrefix` are
  different productions with different slot lists (§0). `planning/occurrence-usage-prefix-matrix.md`
  §9 remains the authoritative ledger for those.
- **The other six `FeaturePrefix` families do not move.** `Invariant` (913), `Connector` (789),
  `BindingConnector` (827), `Succession` (841), `Flow` (1303) and `SuccessionFlow` (1307) spell the
  same prefix but each adds its own declaration grammar. They keep whatever partial prefix fields
  they have. Their migration is a following slice per family, exactly as
  `planning/occurrence-usage-prefix-matrix.md` §10 sequenced the usage families -- and each needs
  its own matrix first.
- **No universal feature/member/header node, optional-field bag, compatibility adapter, parallel
  legacy representation, or parser-framework abstraction.** The merged node models four productions
  that are textually identical for their first four columns; it does not grow a discriminator tag
  for families that are not.
- **`UsageExtensionKeyword` is not renamed.** It already models `PrefixMetadataMember` (KerML 1404
  / SysML 1660) and is reused verbatim, so the production keeps one representation. The name comes
  from SysML's one-line wrapper `UsageExtensionKeyword : Usage = ownedRelationship +=
  PrefixMetadataMember` (SysML 296) and is inaccurate for the KerML slot. Renaming it touches every
  migrated occurrence family for no behavioural gain; recorded as debt.
- **`ordered`/`nonunique` authored order is not retained.** Both are legal in either order and both
  land in independent booleans, so emission always writes `ordered nonunique`. That is the
  representation every usage family already uses; changing it is one seam over all of them.
  `feature_modifiers` accepting them in any repetition remains the separately-recorded debt it was.
- **`intersects` written directly after the multiplicity is still dropped.**
  `specialization_clauses` consumes it into `clauses.intersects` and the feature constructor never
  reads that field, so only an `intersects` following a `unions`/`disjoint from`/`chains`/
  `inverse of` clause survives into `type_relationships`. It is a `FeatureRelationshipPart` defect,
  not a prefix defect; newly recorded in the audit's deferred list.
- **A `#`-led feature member is still two members.** `FeaturePrefix`'s `PrefixMetadataMember*` tail
  is parsed and emitted where the feature parser owns the input -- `derived #Tag feature z3;` and
  `in var #Tag #Tag2 feature z4;` were refused outright before this slice and now carry their
  keywords in `FeaturePrefix::metadata_keywords`. But when the run *leads* the member, the owning
  scopes dispatch their `#` arm first, and that arm's contract is explicit: the
  `PrefixMetadataMember` spelling "owns no body and leaves the prefixed declaration for the next
  member iteration" (`calc_def_body_element`). So `#Tag feature z1;` remains a metadata member
  followed by a feature. Absorbing it means reordering that arm in all four scopes that dispatch
  it, against `part_usage`'s first-refusal guard (`starts_contended_prefix`) -- a metadata-seam
  change shared with every other member kind, not a `FeaturePrefix` one. `feature_prefix` refuses
  the run unless a kind keyword follows it, precisely so the seam keeps its current contract
  (without that guard, `#service port def Authorisation { ... }` is shredded into four members).
- **`Feature`'s metadata-as-keyword alternative is not modelled.** `Feature = FeaturePrefix
  ( 'feature' | ownedRelationship += PrefixMetadataMember ) FeatureDeclaration?` (562) lets the
  last metadata member stand *in place of* the `feature` keyword, so `#Tag z1;` is a feature named
  `z1`. That is the same dispatch question as the entry above and is deferred with it.
- **No broader body-recovery rewrite.** The four starter tables of §6 are completed and the scopes
  that own this production resynchronize on them; no other scope's recovery strategy changes.
- **Feature members are not added to scopes that do not dispatch them.** `TypeBodyElement ->
  FeatureMember -> FeatureElement` reaches a `Feature` from every KerML type body in the pin; this
  parser dispatches the seven scopes of §3. Widening that is one gap per scope in a different
  production's selector.

## 12. Why this is one PR

Three AST nodes, three constructors, seven owning scopes, five owning enums, six emitter sites,
seven semantic-projection sites, three visitor walks and one serialized shape are a single fact --
"which prefix did the author write on this feature" -- represented three ways, with two mutually
exclusive pairs in independently-settable booleans, five legal spellings refused, and an illegal
ordering silently rewritten into two members. Splitting the change leaves `main` with two
representations of that fact at some commit, which `planning/shared-grammar.md`'s Phase 4 entry
gate forbids. The commits inside the PR are sequenced so the audit, the AST, the parser, the
emitters and the tests are each reviewable, but none of them is independently mergeable.

## 13. Coverage

### 13.1 Landed with this slice

| Evidence | What it pins |
| --- | --- |
| `tests/snapshots/spec42/kerml_feature_prefix_slots.md` | every `BasicFeaturePrefix` slot in front of every kind keyword: all three directions; `derived`; `abstract`; both `composite`/`portion`; both `var`/`const`; the directed spellings §5.2 listed as refused; the undirected slots beside them; `EndFeaturePrefix` with and without `const`; the cross feature named and unnamed; the keyword-less `end plain;`; `in calc` routed to `calc-usage`; the metadata tail after a basic prefix and after a directed one, beside the `#`-led member that stays two members |
| `tests/snapshots/spec42/kerml_feature_prefix_owning_scopes.md` | identical projection for identical syntax across namespace level, KerML type body, function body, `calc def` body, a nested feature body and an `attribute def` body -- the property the merge has to earn, since the directed spelling previously reached only three of them |
| `tests/snapshots/spec42/end_prefix_recovery.md` | `in end feature x;` still recovers, in both orders, now as a property of the type rather than of the parser |
| `tests/snapshots/kerml/association_end_features.md` | `end`/`const end`, with and without a cross feature, all as one node |
| `src/parser/constraint.rs` unit tests | `const end` lands in `EndFeaturePrefix`'s own slot; the cross feature owns its `[1]`; `var const` is not one prefix |
| `tests/snapshots/spec42/**`, `tests/snapshots/kerml/**` | the pinned corpus files this slice changes -- five in total, each reviewed in its commit |

### 13.2 Not written, and why

The fixtures below were planned before the slice was built. Two turned out to be
covered by 13.1 under different names; the rest pin behaviour this slice did not change, so they
would be new coverage of old code rather than evidence for this change, and are better added with
the seam that touches it.

- `kerml_feature_prefix_alternatives.md` / `kerml_feature_end_prefix.md` -- landed merged as
  `kerml_feature_prefix_slots.md`. `member` before the prefix, `all` after the keyword, and the
  short-name/`:>>`-led declaration shapes are untouched by this slice and remain covered by the
  existing `kerml/` fixtures.
- `kerml_feature_prefix_recovery.md` / `kerml_feature_prefix_scope_recovery.md` -- the recovery
  contract of §10 and the starter tables of §6 are unchanged here: no scope's recovery strategy
  was touched, and `end_prefix_recovery.md` already pins the one case this slice could have
  broken.
- `tests/kerml_feature_prefix_owning_layer.rs` -- arena rollback after refused speculation is
  exercised by the corpus through `owned_cross_feature` and `metadata_keyword_run` (a leak would
  surface as duplicate references in any fixture that has them), and strict/editor equivalence is
  already a repo-wide gate rather than a per-slice one.
