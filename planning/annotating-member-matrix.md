# Annotating-member grammar-to-scope matrix

> Derived from the pinned grammar in `docs/conformance-target`
> (`sysml-v2-release/bnf/SysML-textual-bnf.kebnf`, `sysml-v2-release/bnf/KerML-textual-bnf.kebnf`),
> not from the approximate counts in `planning/shared-grammar.md`.

This is the evidence for Phase 3's annotating-member follow-up. It answers one question per body
scope: *which alternatives of `AnnotatingElement` does the pinned grammar admit here, and which
does the parser build?*

## The production

Both layers define the same production, and neither restricts it per scope:

```
AnnotatingElement =              // KerML 8.2.3.3.1, SysML 8.2.2.4.1
      Comment                    // ( 'comment' Identification ( 'about' … )? )? ( 'locale' … )? REGULAR_COMMENT
    | Documentation              // 'doc' Identification ( 'locale' … )? REGULAR_COMMENT
    | TextualRepresentation      // ( 'rep' Identification )? 'language' STRING_VALUE REGULAR_COMMENT
    | MetadataFeature            // the '@' spelling of MetadataUsage
```

### Every route into a body reaches all four alternatives

There is no production anywhere in either grammar that admits a proper subset of
`AnnotatingElement`. Every body that admits an annotating member does so through exactly one of
these four routes, and each of them ends at the whole production:

| Route | Chain | Scopes reached |
| --- | --- | --- |
| SysML definition member | `DefinitionBodyItem → DefinitionMember → DefinitionElement → AnnotatingElement` | every `DefinitionBody`, and everything that includes `DefinitionBodyItem`: `UsageBody`, `InterfaceBody`, `ActionBody` (via `NonBehaviorBodyItem`), `StateDefBody`/`StateUsageBody` (via `NonBehaviorBodyItem`), `CalculationBody`, `RequirementBody`, `CaseBody`, `ViewDefinitionBody`, `ViewBody`, `MetadataBody` |
| SysML package member | `PackageBodyElement → PackageMember → DefinitionElement → AnnotatingElement` | `PackageBody`, `RootNamespace` |
| Owned annotation | `RelationshipBody → OwnedAnnotation → AnnotatingElement` | `alias`, `import`, `dependency`, `expose`, `first`, and every other `RelationshipBody` |
| Named annotating member | `EnumerationBody → AnnotatingMember → AnnotatingElement` | `EnumerationBody` — the only scope that names the membership production directly |
| KerML type member | `TypeBodyElement → NonFeatureMember → MemberElement → AnnotatingElement` | `TypeBody`, i.e. every KerML classifier, feature, connector and expression body |

**Consequence for the AST.** `planning/shared-grammar.md` left open how to stop a shared family
from being more permissive than a scope: split the family, parameterize it by scope, or validate
on deserialization. The matrix resolves it by removing the premise — no scope admits a subset, so
`AnnotatingMember` is *exact* everywhere it appears, and a scope enum that carries
`Annotating(AnnotatingMember)` claims neither more nor less than the grammar. No scope-dependent
family, no scope-keyed deserialization rule, and no narrower sibling family is justified by the
pinned grammar. If a future grammar release restricts one, that becomes a split at the family,
not a wildcard at the consumers.

### Two neighbouring forms that are *not* `AnnotatingElement`

These are separately confirmed against the grammar so they are not folded in by accident:

- **`#Name` prefix metadata** is `PrefixMetadataMember`/`PrefixMetadataAnnotation` (SysML
  8.2.2.27), reached from `DefinitionExtensionKeyword`, `UsageExtensionKeyword`, `FeaturePrefix`,
  `Package`, `LibraryPackage` and `Dependency`. It is a *prefix on the declaration that follows
  it*, never a member of a body, and it is not an alternative of `AnnotatingElement`. It stays
  `MetadataKeywordUsage` and keeps `body: None` for the prefix spelling.
- **`#Name;` and `#Name { … }` as a standalone member** are `ExtendedUsage`
  (`UnextendedUsagePrefix UsageExtensionKeyword+ Usage`) with an empty `UsageDeclaration` — a
  *usage* member of the enclosing body, not an annotating one. That is the spelling
  `MetadataKeywordUsage` models with `body: Some(_)`.
- **`metadata name : Type;`** is the other spelling of `MetadataUsage`; the `@` spelling is the
  one `AnnotatingElement` names. The AST keeps `MetadataUsage` separate because its ownership and
  body differ. Unifying the two spellings is a metadata-audit question, not this seam.

## Scope matrix

Every AST body scope, its owning grammar production, and its coverage. "Permitted" is the same in
every row for the reason given above, so the column records the production that grants it rather
than repeating the four alternatives.

Legend for **Before**: `D` Documentation, `C` Comment, `R` TextualRepresentation, `M` `@`
MetadataFeature, `—` none.

| # | AST scope | Owning production(s) | Grants annotating via | Before | After |
| --- | --- | --- | --- | --- | --- |
| 1 | `RelationshipBodyElement` | `RelationshipBody` (alias, import, dependency, `connect` statement) | `OwnedAnnotation` | `AnnotatingMember` | unchanged |
| 2 | `PartUsageBodyElement` (also `RefBody`) | `Usage → UsageBody = DefinitionBody` | `DefinitionMember` | `AnnotatingMember` | unchanged |
| 3 | `PackageBodyElement` | `PackageBody`, `RootNamespace` | `PackageMember` | `D C R M` | `AnnotatingMember` |
| 4 | `PartDefBodyElement` | `PartDefinition → Definition → DefinitionBody` | `DefinitionMember` | `D C M` | `AnnotatingMember` (**+R**) |
| 5 | `AttributeBodyElement` | `AttributeDefinition`/`AttributeUsage → DefinitionBody`; `MetadataBody` (`@`/`#`/`metadata` bodies) | `DefinitionMember` | `D` | `AnnotatingMember` (**+C R M**) |
| 6 | `PortDefBodyElement` | `PortDefinition → Definition → DefinitionBody` | `DefinitionMember` | `D` | `AnnotatingMember` (**+C R M**) |
| 7 | `PortBodyElement` | `PortUsage → Usage → UsageBody` | `DefinitionMember` | `D` | `AnnotatingMember` (**+C R M**) |
| 8 | `InterfaceDefBodyElement` | `InterfaceDefinition → InterfaceBody` | `DefinitionMember` (`InterfaceBodyItem` includes it) | `D` | `AnnotatingMember` (**+C R M**) |
| 9 | `InterfaceUsageBodyElement` | `InterfaceUsage → InterfaceBody` | `DefinitionMember` | `D` | `AnnotatingMember` (**+C R M**) |
| 10 | `ConnectionDefBodyElement` | `ConnectionDefinition → Definition → DefinitionBody` | `DefinitionMember` | `D` | `AnnotatingMember` (**+C R M**) |
| 11 | `OccurrenceBodyElement` | `OccurrenceDefinition`/`OccurrenceUsage → DefinitionBody`/`UsageBody` | `DefinitionMember` | `D` | `AnnotatingMember` (**+C R M**) |
| 12 | `DefinitionBodyElement` | `FlowDefinition`, `AllocationDef`, `Message`, … `→ DefinitionBody` | `DefinitionMember` | `D` (**unreachable**) | delegated to `OccurrenceBodyElement` |
| 13 | `PerformBodyElement` | `PerformActionUsage → ActionBody` | `NonBehaviorBodyItem → DefinitionMember` | `D` | `AnnotatingMember` (**+C R M**) |
| 14 | `FeatureBodyElement` | KerML `Feature → TypeBody` | `NonFeatureMember → MemberElement` | `D` | `AnnotatingMember` (**+C R M**) |
| 15 | `ActionDefBodyElement` | `ActionDefinition`/nested control nodes `→ ActionBody` | `NonBehaviorBodyItem → DefinitionMember` | `D R M` | `AnnotatingMember` (**+C**) |
| 16 | `ActionUsageBodyElement` | `ActionUsage → ActionBody` | `NonBehaviorBodyItem → DefinitionMember` | `D R M` | `AnnotatingMember` (**+C**) |
| 17 | `StateDefBodyElement` | `StateDefinition → StateDefBody`, `StateUsage → StateUsageBody` | `NonBehaviorBodyItem → DefinitionMember` | `D M` | `AnnotatingMember` (**+C R**) |
| 18 | `FirstMergeBodyElement` | `InitialNodeMember → RelationshipBody`; `MergeNode`/`DecisionNode`/`JoinNode`/`ForkNode → ActionBody` | `OwnedAnnotation`, `DefinitionMember` | `—` | via `ActionDefBodyElement` (**+D C R M**) |
| 19 | `ConstraintDefBodyElement` | `ConstraintDefinition → CalculationBody`, `RequirementConstraintMember` | `ActionBodyItem → DefinitionMember` | `D M` | `AnnotatingMember` (**+C R**) |
| 20 | `CalcDefBodyElement` | `CalculationDefinition`/`CalculationUsage → CalculationBody` | `ActionBodyItem → DefinitionMember` | `D C M` | `AnnotatingMember` (**+R**) |
| 21 | `ViewDefBodyElement` | `ViewDefinition → ViewDefinitionBody` | `DefinitionBodyItem → DefinitionMember` | `D M` | `AnnotatingMember` (**+C R**) |
| 22 | `ViewBodyElement` | `ViewUsage → ViewBody` | `DefinitionBodyItem → DefinitionMember` | `D` | `AnnotatingMember` (**+C R M**) |
| 23 | `RenderingDefBodyElement` | `RenderingDefinition → Definition → DefinitionBody` | `DefinitionMember` | `D` | `AnnotatingMember` (**+C R M**) |
| 24 | `RenderingUsageBodyElement` | `RenderingUsage → UsageBody` | `DefinitionMember` | `D` | `AnnotatingMember` (**+C R M**) |
| 25 | `RequirementDefBodyElement` | `RequirementDefinition`/`RequirementUsage`/`ConcernUsage`/`ViewpointUsage` `→ RequirementBody` | `DefinitionBodyItem → DefinitionMember` | `D R M` | `AnnotatingMember` (**+C**) |
| 26 | `UseCaseDefBodyElement` | `CaseDefinition`/`UseCaseDefinition`/`AnalysisCaseDefinition`/… `→ CaseBody` | `ActionBodyItem → DefinitionMember` | `D M` | `AnnotatingMember` (**+C R**) |
| 27 | `ReturnRefBodyElement` | `ReturnParameterMember → UsageElement → UsageBody` | `DefinitionMember` | `D` | `AnnotatingMember` (**+C R M**) |
| 28 | `EnumerationBody` (`Body<EnumeratedValue>`) | `EnumerationBody` | `AnnotatingMember` **named directly** | `D C` **parsed and discarded** | `EnumerationBodyElement::Annotating` (**+R M**, and no longer discarded) |

### Recovery behaviour required per row

Rows 3–28 all sit inside `parse_structured_brace_members`, which is the single owner of "member
failed to parse" for braced bodies: it rolls back speculative references, skips one statement or
block, records the exact skipped span as the scope's `Error` variant, and continues with the next
member. Adding an alternative to a scope's member parser therefore changes only *what parses*, not
*what happens when it does not*: malformed annotating content before or between valid members stays
a source-backed `ParseErrorNode` at its authored position, later siblings survive, and forward
progress is guaranteed by the offset check in that loop.

Row 28 is the exception and the reason it is in this change: `enumeration_body` is hand-rolled. It
matched `doc` and `comment` only to **throw them away** — no node, no diagnostic, no span — and on
any other unparseable member it ran to the closing brace and discarded everything in between. It
gains the shared loop's contract along with the family.

Row 12 is the other exception: `DefinitionBodyElement::Doc` has no construction site anywhere in
the parser. Documentation in that scope arrives as
`DefinitionBodyElement::OccurrenceMember(OccurrenceBodyElement::Doc)`. The variant is an
unreachable state in a public enum and a second representation of the same syntactic fact, so it
is removed rather than migrated.

## What the clause-level audit added

Auditing the alternatives rather than only the scopes turned up two defects inside the production
itself, both fixed with the coverage:

- **`Comment`'s `about` clause was skipped, not parsed.** `Comment = ( 'comment' Identification
  ( 'about' Annotation ( ',' Annotation )* )? )? ( 'locale' STRING_VALUE )? body`. The clause was
  consumed by `take_until("/*")` — an unbounded substring search — so the annotated elements were
  discarded, a `locale` written after them was discarded, and the scan ran past the member, past
  its enclosing `}`, and through however many later declarations it took to find a block comment.
  `CommentAnnotation::about_targets` now holds them as qualified references, in the same shape
  `MetadataAnnotation` already used for the same clause.
- **`TextualRepresentation` was not dispatched in KerML type bodies at all.** The fallback member
  parser broke `rep x language "text" /* … */` into four invented members with no diagnostic, so
  the document parsed clean and formatted back as something else.

## No annotating form remains valid-but-unsupported

`Unsupported` and `Malformed` stay distinct states, but after this change no *annotating* member
reaches the first of them, which is why the recovery fixtures have no valid-but-unsupported case
to show:

- no scope's opaque-starter list (`ATTRIBUTE_OPAQUE_STARTERS`, `METADATA_OPAQUE_STARTERS`,
  `DEFINITION_BODY_OPAQUE_STARTERS`, `VIEW_DEF_OPAQUE_STARTERS`) contains `doc`, `comment`, `rep`,
  `language` or `@`, so `unsupported_member` cannot claim one; and
- no emitter reports an `AnnotatingMember` as unsupported. The one surviving `w.unsupported` arm
  that mentions an annotation is `OccurrenceBodyElement::Annotation`, which is the legacy
  `Annotation` type — a `#`-sigil fallback with an `AnnotationHead::Opaque` head — and not an
  alternative of `AnnotatingElement`. An occurrence-body `@M about A, B;` parses and formats
  through the shared annotating path.

## Confirmed remaining gaps

These are annotating-member gaps the matrix found that this change does **not** close. Each is
recorded with the grammar evidence and the reason it is a different seam.

- **`ConnectBody` and `OpacityKind::OpaqueConnectBrace` survive only for the legacy `Annotation`.**
  Every other owner now holds a real `Body<E>`. `Annotation` is the generic `@`/`#` fallback whose
  head is an `AnnotationHead::Opaque(String)`; converting its body belongs with the metadata audit
  that owns that opaque head, not with the container work. Nothing in the 351-document snapshot
  corpus reaches `OpaqueConnectBrace` any more, and no `@`/`#` spelling tried here constructs an
  `Annotation` with a brace body at all — the variant should be deleted with `Annotation`'s
  rewrite rather than on that evidence alone.

- **(historical) `ConnectBody` marker bodies discard everything inside them.** `Allocate`, `Transition`,
  `ExposeMember`, `SatisfyViewMember` and `Annotation` hold `ConnectBody`, a two-variant marker
  (`Semicolon | Brace`) whose brace form is parsed by `advance_to_closing_brace` and retains
  nothing — not the members, not the delimiter spans. The grammar admits annotating members in
  all of them: `Expose = 'expose' ( MembershipExpose | NamespaceExpose ) RelationshipBody`,
  `AllocationUsage = OccurrenceUsagePrefix AllocationUsageDeclaration UsageBody`, and
  `TransitionUsage = 'transition' … ActionBody`. Closing it is not a variant swap: `ConnectBody`
  has no delimiter provenance at all, so each owner needs the Phase-2 `Body<E>` container before
  it can hold typed members. `Dependency` and `Satisfy` additionally pair `ConnectBody` with a
  separate `body_elements: Option<Vec<…>>`, which is the same body fact in two representations
  and must collapse into one `Body<E>` in that work.
- **A `connect` statement body is `UsageBody`, not `RelationshipBody`.**
  `ConnectionUsage = OccurrenceUsagePrefix ( … | 'connect' ConnectorPart ) UsageBody`, so
  `connect a to b { … }` legally holds every definition-body member, while the parser routes it
  through `relationship_body`. Annotating members are complete there — that is why the scope
  appears as row 1 rather than as a gap — but the non-annotating members of that body are missing.
  That is definition/usage body coverage, not this seam.

## Adjacent gaps this audit ran into

Not annotating-member gaps. They are recorded because writing the coverage fixtures found them,
and each is a member-dispatch or emission gap of the same class as the already-recorded "`ref` in
a state body only dispatches when it is typed".

- An untyped `interface i { … }` in a part usage body reaches recovery; `interface i : I { … }`
  parses. `InterfaceUsage` does not require the typing.
- A `rendering rr { … }` member in a part usage body reaches recovery, though `render r { … }`
  parses inside a view usage.
- A `first f { … }` member in a calculation body reaches recovery.
  `CalculationBodyItem = ActionBodyItem | ReturnParameterMember`, and `ActionBodyItem` includes
  `InitialNodeMember`, so it is legal there.
- ~~A `flow def` parses in a part definition body and at package level, but the emitter reports it
  as an unsupported construct in both~~ — fixed.
- `binding a = b;` (`BindingConnectorAsUsage`, a `NonOccurrenceUsageElement`) is dispatched in
  neither a package body nor a definition body: the package scope reports it as an unimplemented
  production and a part definition body reports an unexpected keyword. The AST models it and its
  body is a real `Body<PartUsageBodyElement>`, but nothing constructs one.
