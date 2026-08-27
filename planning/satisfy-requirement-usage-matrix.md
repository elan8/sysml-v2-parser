# `SatisfyRequirementUsage` grammar matrix

Every satisfy spelling found in the parser, the checked-in tests and snapshots, and the pinned
`sysml-v2-release` corpus, classified against the pinned grammar. The pin is
`docs/conformance-target` (`release_tag=2026-04`,
`grammar_content_hash=fnv1a64:95f39e912f73b917`).

## 1. Authoritative productions

```text
SatisfyRequirementUsage =
    OccurrenceUsagePrefix 'assert' ( isNegated ?= 'not' ) 'satisfy'
    ( ownedRelationship += OwnedReferenceSubsetting
      FeatureSpecializationPart?
    | 'requirement' UsageDeclaration )
    ValuePart?
    ( 'by' ownedRelationship += SatisfactionSubjectMember )?
    RequirementBody
                                        -- SysML-textual-bnf.kebnf:1466, clause 8.2.2.21.2

SatisfactionSubjectMember       : SubjectMembership          = ownedRelatedElement += SatisfactionParameter
SatisfactionParameter           : ReferenceUsage             = ownedRelationship  += SatisfactionFeatureValue
SatisfactionFeatureValue        : FeatureValue               = ownedRelatedElement += SatisfactionReferenceExpression
SatisfactionReferenceExpression : FeatureReferenceExpression  = ownedRelationship  += FeatureChainMember
FeatureChainMember              : Membership                 = memberElement = [QualifiedName] | OwnedFeatureChainMember

OwnedReferenceSubsetting : ReferenceSubsetting = referencedFeature = [QualifiedName] | OwnedFeatureChain
UsageDeclaration         : Usage               = Identification FeatureSpecializationPart?
Identification           : Element             = ( '<' declaredShortName = NAME '>' )? ( declaredName = NAME )?
FeatureSpecializationPart: Feature             = FeatureSpecialization+ MultiplicityPart? FeatureSpecialization*
                                               | MultiplicityPart FeatureSpecialization*
FeatureSpecialization    : Feature             = Typings | Subsettings | References | Crosses | Redefinitions
ValuePart                : Feature             = ownedRelationship += FeatureValue
RequirementBody          : Type                = ';' | '{' RequirementBodyItem* '}'
```

`SatisfyRequirementUsage` is reached as
`BehaviorUsageElement → OccurrenceUsageElement → DefinitionBodyItem`/`PackageBodyElement`, so it is
an ordinary member of every definition, usage, and package body — including `RequirementBody`
itself, which is the body a satisfy usage owns.

### 1.1 Two documented departures from the printed production text

| Departure | Printed text | What the pin's own corpus writes | Resolution |
| --- | --- | --- | --- |
| `assert` is printed as required | `OccurrenceUsagePrefix 'assert' …` | `Simple Tests/RequirementTest.sysml:21` writes `satisfy r by p;` and Systems Library `Views.sysml:37` writes `satisfy requirement viewpointConformance by that { … }`, which the strict library gate requires to parse with no diagnostic | `assert` is modelled as optional |
| `not` is printed without its `?` | `( isNegated ?= 'not' )` | `RequirementTest.sysml:26` writes `not satisfy r1 by p;` with no `assert`, and line 27 writes `assert not satisfy r1 by q;` | `not` is modelled as optional and independent of `assert` |

Both are printing defects in the `.kebnf` rendering rather than grammar decisions:
`AssertConstraintUsage` (line 1386) writes the same negation group *with* its `?`, and
`LibraryPackage` (line 113) drops the `?` from `( isStandard ?= 'standard' ) 'library'`, a prefix
that is certainly optional. All four prefix combinations appear in the pinned corpus, so the
corpus is decisive.

## 2. Alternative matrix

| # | Alternative | Prefix shape | Requirement clause role | `by` cardinality | Body production | AST representation |
| --- | --- | --- | --- | --- | --- | --- |
| A | Reference | `assert`? `not`? `satisfy` | **reference** — `[QualifiedName]` or `OwnedFeatureChain` | 0..1 | `RequirementBody` | `SatisfiedRequirement::Reference { reference: QualifiedReferenceId }` |
| B | Inline declaration | `assert`? `not`? `satisfy` `requirement` | **declaration** — `Identification` (name and/or short name, possibly neither) | 0..1 | `RequirementBody` | `SatisfiedRequirement::Declaration(Node<InlineRequirementDeclaration>)` |

The alternatives are mutually exclusive and selected by the `requirement` keyword, so they are an
enum rather than two optional fields. `FeatureSpecializationPart?` and `ValuePart?` are shared by
both alternatives and live on the owning `SatisfyRequirementUsage`, since the reference alternative
spells them directly and the declaration alternative reaches the same clauses through
`UsageDeclaration`.

### 2.1 Field-by-field representation

| Grammar fact | Field | Why |
| --- | --- | --- |
| `'assert'` | `assert_span: Option<Span>` | Presence is authored syntax the emitter reproduces; the span keeps provenance instead of reducing the fact to a flag |
| `isNegated ?= 'not'` | `not_span: Option<Span>` | Same; `Some`/`None` *is* `isNegated`, so there is no second boolean |
| `'satisfy'` | `satisfy_span: Span` | Always authored; the anchor token every prefix and clause is ordered against |
| requirement clause | `requirement: SatisfiedRequirement` | Typed alternative; a declaration label is never stored as a `QualifiedReferenceId` and a reference is never synthesized from a label |
| `'requirement'` keyword | `InlineRequirementDeclaration::keyword_span` | Selects alternative B; emission writes the keyword from it |
| `Identification` | `InlineRequirementDeclaration::identification` | The repository's declaration-name node, span-backed through its `Node`. Not an owned bare `String` |
| `Typings` | `typing: Option<Node<TypingRelationship>>` | Keeps the authored spelling (`:` / `typed by` / `defined by`), conjugation and every target |
| `MultiplicityPart` | `multiplicity`, `ordered`, `nonunique` | `ordered`/`nonunique` are genuinely independent binary properties of one clause |
| `Subsettings` / `Redefinitions` / `References` / `Crosses` | `subsets`, `redefines`, `references`, `crosses` | Independent optional clauses, not alternatives of one another |
| `ValuePart?` | `value: Option<Node<FeatureValue>>` | |
| `( 'by' … )?` | `subject: Option<Node<SatisfactionSubject>>` | Absence *is* `None`; nothing is cloned in to stand for an omitted clause, and there is no mirror flag |
| `FeatureChainMember` | `SatisfactionSubject::reference: QualifiedReferenceId` | The membership chain bottoms out at a qualified name or feature chain, so the subject is a reference with typed `::`/`.` separators, not an expression |
| `'by'` keyword | `SatisfactionSubject::by_span` | Emission writes `by` only when this node exists |
| `RequirementBody` | `body: RequirementDefBody` (`Body<RequirementDefBodyElement>`) | The same member set a `requirement def` owns; `;`, `{}` and both delimiter spans stay distinct |

## 3. Legal owning scopes

`SatisfyRequirementUsage` is a `BehaviorUsageElement`, so the grammar admits it in every definition
and usage body. The scopes this parser dispatches it from:

| Scope | Element enum | Status |
| --- | --- | --- |
| package / namespace / root | `PackageBodyElement::Satisfy` | supported (all four prefix combinations) |
| `part def` body | `PartDefBodyElement::Satisfy` | supported |
| `part` usage body | `PartUsageBodyElement::Satisfy` | supported |
| occurrence body | `OccurrenceBodyElement::Satisfy` | supported |
| `view def` body | `ViewDefBodyElement::Satisfy` | supported |
| `view` usage body | `ViewBodyElement::Satisfy` | supported (previously a separate viewpoint-only node) |
| `requirement`/`concern`/`viewpoint` body | `RequirementDefBodyElement::Satisfy` | supported, including the `RequirementBody` a satisfy usage owns |
| every other definition/usage body (`action`, `state`, `calc`, `case`, `connection`, `interface`, `port`, `attribute`, …) | — | **valid but unsupported**: the member is not dispatched there and becomes an explicit recovery node. Pinned production `BehaviorUsageElement` (SysML BNF line 374) |

## 4. Corpus survey

Every `satisfy` statement in the pinned `sysml-v2-release` `.sysml` sources, classified.

| Spelling | Source | Alternative | Class |
| --- | --- | --- | --- |
| `satisfy vehicleFuelEconomyRequirements by vehicle_c1;` | `training/33. Analysis/Analysis Case Usage Example.sysml:30` | A | valid and supported |
| `satisfy 'system structure perspective';` | `training/42. Views/Views Example.sysml:7`, `validation/11a-View-Viewpoint.sysml:47` | A, no `by`, quoted | valid and supported |
| `satisfy vehicleSpecification by vehicle_design;` | `training/32. Requirements/Requirement Satisfaction.sysml:22` | A | valid and supported |
| `satisfy engineSpecification by vehicle_design.engine_v1;` | same file, line 23 | A, `.` feature chain in `by` | valid and supported |
| `satisfy r by p;` | `examples/Simple Tests/RequirementTest.sysml:21` | A | valid and supported |
| `assert satisfy r by q;` | same file, line 22 | A + `assert` | valid and supported |
| `not satisfy r1 by p;` | same file, line 26 (package scope) | A + `not` | valid and supported |
| `assert not satisfy r1 by q;` | same file, line 27 (package scope) | A + `assert` + `not` | valid and supported |
| `satisfy Requirements::engineSpecification by vehicle_b.engine{ … }` | `Vehicle Example/SysML v2 Spec Annex A SimpleVehicleModel.sysml:652` | A, `::` reference, `.` chain, braced `RequirementBody` holding `requirement … :>> … { … }` members | valid and supported |
| `satisfy Requirements::vehicleSpecification by vehicle_b{ … }` | same file, line 717 | A, braced body with nested `requirement` members | valid and supported |
| `satisfy requirement sv:SafetyViewpoint;` | same file, line 1574 | B, named + typed, no `by` | valid and supported |
| `satisfy vehicleFuelEconomyRequirementsGroup by vehicle_c1_analysized;` | `Analysis Examples/Vehicle Analysis Demo.sysml:284`, `validation/10c-Fuel Economy Analysis.sysml:164` | A | valid and supported |
| `satisfy requirement req1 : Req1 by system;` | `Requirements Examples/RequirementDerivationExample.sysml:27–29` | B, named + typed + `by` | valid and supported |
| `satisfy torqueGeneration by torqueGenerator;` | `validation/12b-Allocation-1.sysml:32` | A | valid and supported |
| `satisfy engineRqtChoice by engineChoice;` | `validation/7b-Variant Configurations.sysml:92` | A | valid and supported |
| `satisfy 'vehicle1-c1 Specification' by vehicle1_c1 { … }` | `validation/8-Requirements.sysml:185` | A, quoted, braced body | valid and supported |
| `satisfy 'engine-v1 Specification' by vehicle1_c1.engine_v1;` | same file, line 191 | A, quoted, `.` chain | valid and supported |
| `satisfy 'vehicle1-c2 Specification' by vehicle1_c2;` | same file, line 203 | A, quoted | valid and supported |
| `satisfy requirement viewpointConformance by that { … require viewpointSatisfactions { … } }` | `sysml.library/Systems Library/Views.sysml:37` | B, named, `by`, braced `RequirementBody` with a `require` member | valid and supported |

No malformed satisfy spelling occurs in the pinned corpus.

## 5. Behaviour classified as legacy permissiveness, now removed

| Former behaviour | Grammar says | Now |
| --- | --- | --- |
| `by` accepted any `OwnedExpression` (`satisfy r by (a + b);`) | `by` takes `FeatureChainMember` — a qualified name or feature chain | the expression form is refused and recovers as malformed |
| the satisfied requirement was parsed as an expression, so `satisfy r [1]` became a bracketed unit and re-emitted as `satisfy r ['1']` | `[1]` is `MultiplicityPart` | parsed as multiplicity and re-emitted as `[1]` |
| `satisfy r by p.q::s;` re-emitted as `satisfy r by p.q::s::s;` | one `OwnedFeatureChain` with typed separators | one arena reference; separators are reproduced from their segment metadata |
| `intersects` accepted through the shared usage header and discarded | not a `FeatureSpecialization` of this production | refused; the usage recovers as malformed |

## 6. Recovery and diagnostics

| Input | Outcome |
| --- | --- |
| malformed member before a satisfy usage | recovery node at its own span; the satisfy usage after it parses (snapshot `satisfy_requirement_usage_recovery.md`, `MalformedBeforeSatisfy`) |
| `satisfy Spec by ;` | `missing_expression_after_operator` / *expected target after `by`*, error severity, `ParseError` category; the whole usage is one recovery node — no empty or cloned reference stands in for the missing subject |
| `satisfy by target;` | `recovered_part_def_body_element`; refused rather than reinterpreted as an anonymous inline declaration |
| `satisfy Spec = ;` | `missing_expression_after_operator` / *expected expression after `=`* |
| malformed member inside the braced `RequirementBody` | recovery node inside the body; the valid members after it, and the valid siblings after the satisfy usage, survive |
| `}` inside a comment or a quoted name | not a body terminator; the body closes at its real `}` |
| nested braces inside the body | balanced and preserved |
| unmatched opening brace | the enclosing declaration's parse fails and the scope above keeps the text — `missing_closing_brace`. There is deliberately no "body with a missing close" state (see `planning/shared-grammar.md`) |
| malformed member before a **prefixed** satisfy usage | recovery stops at the prefix. `assert`, `not` and `satisfy` are all FIRST tokens of this production, so all three are recovery boundaries in every scope's starter table; before this seam only `satisfy` was listed (and `satisfy` itself was missing from `VIEW_DEF_BODY_STARTERS`), so a malformed member scanned past the prefix and swallowed the whole usage including its `;`. Checked for all 7 wired scopes × all 4 prefix combinations |

Starter tables updated: `REQUIREMENT_BODY_STARTERS` (+`assert`, +`not`), `PART_BODY_STARTERS`
(+`not`), `OCCURRENCE_BODY_STARTERS` (+`not`), `VIEW_BODY_STARTERS` (+`assert`, +`not`),
`VIEW_DEF_BODY_STARTERS` (+`assert`, +`not`, +`satisfy`), and `PACKAGE_BODY_GRAMMAR` (+`not`;
`assert` was already listed for `AssertConstraintUsage`, and the recovery starter slice is
generated from that table). Adding `not` at package scope changes one pinned diagnostic — the
malformed `not valid` in `tests/snapshots/qualified_references/recovery_references.md` now reports
`missing_semicolon` instead of `unexpected keyword \`not\``, at the same span, with the same
recovered content and the same surviving siblings. Recognizing a real FIRST token is worth that:
without it, recovery consumes a valid `not satisfy …` sibling.

Recovery never produces a satisfy usage: a refused parse is an explicit malformed node, and the
reference transaction that wraps `satisfy` rolls back every speculative arena entry, so a refused
head leaves the qualified-reference arena exactly as it found it.

## 7. Remaining valid-but-unsupported syntax

| Syntax | Pinned production | Note |
| --- | --- | --- |
| ~~`OccurrenceUsagePrefix` on a satisfy usage~~ | `SatisfyRequirementUsage` → `OccurrenceUsagePrefix` (SysML BNF line 564) | **Done.** Closed as a shared typed node rather than a satisfy-local one, exactly as this row anticipated: `planning/occurrence-usage-prefix-matrix.md`. `SatisfyRequirementUsage::prefix` is the same `OccurrenceUsagePrefix` the occurrence-usage and item-usage families now carry, and the `MemberPrefix` visibility is modelled beside it on `SatisfyRequirementUsage::membership` |
| A keyword abutting a comment or a quoted name (`satisfy/* c */ r`, `satisfy'quoted'`) | lexical structure, clause 8.2.2.1 | Crate-wide, not satisfy-specific: `starts_with_keyword` requires the byte after an identifier-shaped keyword to be whitespace or one of `{ : ; [`, so `part/* c */ a;` and `import/* c */ A::*;` are rejected identically. Widening it to a true identifier boundary touches every starter table, dispatch guard and recovery sync in the crate. The *separated* spelling `assert /* why */ satisfy` does parse; it did not before this seam, because the separator after `assert` was `ws1`, which stops at `/` |
| `satisfy` in body scopes other than those in §3 | `BehaviorUsageElement` | member-dispatch work per scope |
| `RequirementUsage`'s own `:>>` clause inside a satisfy body (`requirement nested :>> other;`) | `RequirementUsage` → `ConstraintUsageDeclaration` | `RequirementUsage` has no `redefines` field; the clause parses and is dropped. Pre-existing debt of that node, not of this seam |

## 8. What this seam replaced

- `Satisfy { source: Node<Expression>, target: Node<Expression>, body: ConstraintDefBody, is_negated: bool, inline_requirement: Option<InlineSatisfyRequirement> }` — deleted.
- `InlineSatisfyRequirement { name: String, type_name: Option<QualifiedReferenceId> }` — deleted; the owned `String` name is gone.
- `SatisfyViewMember { viewpoint_ref, body: Body<RelationshipBodyElement> }` — deleted; the view usage body holds the one satisfy production.
- The parser's synthesis of `Expression::FeatureRef` from the inline declaration's *name* — gone.
- The parser's `source.clone()` into `target` when `by` was absent — gone.
- The emitter's unconditional ` by ` — gone; `assert` is now emitted (it was parsed and discarded).
