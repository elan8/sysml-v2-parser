# `@` / `#` metadata-sigil grammar matrix

> Derived from the pinned grammar in `docs/conformance-target`
> (`sysml-v2-release/bnf/SysML-textual-bnf.kebnf`, `sysml-v2-release/bnf/KerML-textual-bnf.kebnf`,
> release `2026-04`, content hash `fnv1a64:95f39e912f73b917`).

This is the evidence for the metadata/annotation seam: the audit that retired the legacy
`Annotation` type, `AnnotationHead::Opaque(String)` and the last `ConnectBody`. It answers one
question per spelling: *which production is this, what may follow its head, and what does the AST
build?*

`planning/annotating-member-matrix.md` is the companion document. It owns `AnnotatingElement` --
`Comment | Documentation | TextualRepresentation | MetadataFeature` -- and its four alternatives'
distribution across body scopes. This document owns the two *sigils*, which is a different cut:
`@` is one alternative of that production, and `#` is not an alternative of it at all.

## The productions

Both layers define the same two, under different names for the same shapes.

```text
// The `@` spelling -- KerML 8.2.5.12 MetadataFeature, SysML 8.2.2.27 MetadataUsage
MetadataFeature =
    ( ownedRelationship += PrefixMetadataMember )*
    ( '@' | 'metadata' )
    MetadataFeatureDeclaration
    ( 'about' ownedRelationship += Annotation ( ',' ownedRelationship += Annotation )* )?
    MetadataBody

MetadataFeatureDeclaration =                  // SysML spells it MetadataUsageDeclaration
    ( Identification ( ':' | 'typed' 'by' ) )?
    ownedRelationship += OwnedFeatureTyping

// The `#` spelling -- KerML 8.2.5.12, SysML 8.2.2.27
PrefixMetadataMember     : OwningMembership = '#' ownedRelatedElement += PrefixMetadataFeature
PrefixMetadataAnnotation : Annotation       = '#' ownedRelatedElement += PrefixMetadataFeature
PrefixMetadataFeature    : MetadataFeature  = ownedRelationship += OwnedFeatureTyping

// The one production in which a `#` head owns a body of its own
ExtendedUsage      : Usage      = UnextendedUsagePrefix UsageExtensionKeyword+ Usage
ExtendedDefinition : Definition = BasicDefinitionPrefix? DefinitionExtensionKeyword+ 'def' Definition
UsageExtensionKeyword      : Usage      = ownedRelationship += PrefixMetadataMember
DefinitionExtensionKeyword : Definition = ownedRelationship += PrefixMetadataMember

// Supporting
OwnedFeatureTyping : FeatureTyping = type = [QualifiedName] | type = OwnedFeatureChain
Identification     : Element       = ( '<' declaredShortName = NAME '>' )? ( declaredName = NAME )?
Annotation                         = annotatedElement = [QualifiedName]
MetadataBody       : Type          = ';' | '{' ( DefinitionMember | MetadataBodyUsageMember
                                               | AliasMember | Import )* '}'
```

### Two facts the whole audit turns on

**1. The last qualified name after `@` is the *type*, not the head.** `MetadataFeatureDeclaration`
ends at a required `OwnedFeatureTyping`. The `Identification` in front of `:` / `typed by` is
optional and is a **declared name**, not a reference. So `@Tag;` names nothing and is typed by
`Tag`; `@t : Tag;` declares `t` and is typed by `Tag`.

The superseded `MetadataAnnotation` had this inverted -- `reference: QualifiedReferenceId` for
whatever followed `@`, `type_reference: Option<_>` for whatever followed `:` -- so `@t : Tag`
allocated an arena reference for the declaration label `t`. That is a reference synthesized from a
declaration, which `AGENTS.md` forbids outright.

**2. `#` is followed by a reference, and by nothing else.** `PrefixMetadataFeature` *is* an
`OwnedFeatureTyping`, which is `[QualifiedName]`. There is no `Identification`, no `:` / `typed by`
clause and no `about` clause anywhere in the `#` productions -- those belong to `MetadataFeature`,
which `#` does not reach in either layer.

The superseded `MetadataKeywordUsage` stored `keyword: String` (a copied `NAME`, so `#ISQ::mass`
truncated to `ISQ`), plus `type_reference` and `about_targets` fields that no `#` production can
produce.

## Spelling matrix

Every spelling reachable through a sigil, its authoritative production, and what the AST builds.

Legend for **Class**: **T** typed and supported; **U** valid but unsupported; **M** malformed;
**P** permissive legacy behaviour not justified by the pinned grammar (removed by this change).

| # | Spelling | Production | Head shape | Legal body | Class | AST |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | `@Tag;` | `MetadataFeature` | `OwnedFeatureTyping` | `;` | T | `MetadataAnnotation { declared_name: None, type_reference }` |
| 2 | `@Pkg::Tag;` | `MetadataFeature` | qualified `[QualifiedName]` | `;` | T | as row 1, multi-segment reference |
| 3 | `@$::Pkg::Tag;` | `MetadataFeature` | absolute `[QualifiedName]` | `;` | T | as row 1, `is_absolute` |
| 4 | `@Pkg::'quoted';` | `MetadataFeature` | quoted `NAME` segment | `;` | T | as row 1, decoded segment text |
| 5 | `@Tag { … }` | `MetadataFeature` | `OwnedFeatureTyping` | `MetadataBody` brace | T | as row 1, `body: Brace` |
| 6 | `@t : Tag;` | `MetadataFeature` | `Identification ':'` + typing | `;` | T | `declared_name: Some(_ , Colon)` |
| 7 | `@t typed by Tag;` | `MetadataFeature` | `Identification 'typed' 'by'` + typing | `;` | T | `declared_name: Some(_ , TypedBy)` |
| 8 | `@<s> : Tag;` | `MetadataFeature` | short-name-only `Identification` | `;` | T | `declared_name` with `short_name` only |
| 9 | `@<s> t : Tag;` | `MetadataFeature` | full `Identification` | `;` | T | `declared_name` with both halves |
| 10 | `@Tag about A, B::C;` | `MetadataFeature` | typing + `'about' Annotation*` | `;` | T | `about_targets: Vec<QualifiedReferenceId>` |
| 11 | `#Tag part def X;` | `PrefixMetadataMember` | `OwnedFeatureTyping` | none | T | `MetadataKeywordUsage { body: None }`, sibling member |
| 12 | `#Pkg::Tag part def X;` | `PrefixMetadataMember` | qualified `[QualifiedName]` | none | T | as row 11, multi-segment |
| 13 | `#A #B part def X;` | `DefinitionExtensionKeyword+` | stacked heads | none | T | one `MetadataKeywordUsage` per tag |
| 14 | `#Tag;` | `ExtendedUsage`, empty `UsageDeclaration` | `OwnedFeatureTyping` | `;` | T | `MetadataKeywordUsage { body: Some(Semicolon) }` |
| 15 | `#Tag { … }` | `ExtendedUsage`, empty `UsageDeclaration` | `OwnedFeatureTyping` | brace | T | `MetadataKeywordUsage { body: Some(Brace) }` |
| 16 | `#Tag def X;` | `ExtendedDefinition` | `DefinitionExtensionKeyword+ 'def'` | `DefinitionBody` | T | `ExtendedDefinition { prefix_keywords }` |
| 17 | `#Tag X { … }` | `ExtendedDefinition`, no `def` | as row 16 | `DefinitionBody` | T | as row 16, `has_def_keyword: false` |
| 18 | `#Tag x : T;` | `ExtendedUsage`, non-empty `UsageDeclaration` | tag + full `Usage` | `UsageBody` | T | tag as row 11, then the usage as its own member |
| 19 | `#Tag : T;` | `ExtendedUsage`, anonymous typed `Usage` | tag + `FeatureSpecializationPart` | `UsageBody` | **U** | recovery, `unsupported_annotation_syntax` |
| 20 | `#Tag :>> y;` | `ExtendedUsage`, anonymous redefining `Usage` | tag + `Redefinitions` | `UsageBody` | **U** | recovery, `unsupported_annotation_syntax` |
| 21 | `@Pkg::Tag : T;` | — | qualified name before `:` | — | **M** | recovery, `unsupported_annotation_syntax` (see note) |
| 22 | `#Tag about X;` | — | — | — | **P** | `#Tag` is a `PrefixMetadataMember`; `about X;` is then `unexpected_keyword_in_scope` |
| 23 | `@A.b;` | `MetadataFeature` | `OwnedFeatureTyping`'s `OwnedFeatureChain` half | `;` | **U** | recovery, `unsupported_annotation_syntax` |
| 24 | `#A.b;` | `PrefixMetadataFeature` | as row 23 | none | **U** | recovery, `unsupported_annotation_syntax` |
| 25 | `@ : Tag;` | `MetadataFeature` | empty `Identification` + `:` | `;` | T | `declared_name: Some(_)` with both halves `None` |
| 26 | `#;`, `@ ;`, `#::x` | — | sigil with no `[QualifiedName]` | — | **M** | recovery, `malformed_annotation_head` |

Rows 19, 20, 23 and 24 are the *valid* syntax this seam leaves unsupported; see "Left
unsupported" below. Rows 21 and 22 are spellings the superseded parser accepted and the grammar does not
produce.

Row 21 is grammatically malformed -- `Identification` is a `NAME`, never a qualified name -- but
reaches recovery as `unsupported_annotation_syntax` rather than `malformed_annotation_head`. That
is deliberate: the classifier decides on the *head*, and this head is a well-formed
`[QualifiedName]`. Separating it would mean re-deriving `MetadataFeatureDeclaration` by scanning
for a `:` after a `::` in raw text, which is exactly the substring-search recovery `AGENTS.md`
rules out. The diagnostic still says what is true -- the parser recognized a metadata head and
could not parse the continuation -- and the *span* is exact.

Row 22 is not rejected as one unit either, and should not be: `#Tag` on its own is a valid
`PrefixMetadataMember`, so it parses, and the leftover `about X;` is reported where it is written
as an unexpected keyword. The removal is of the field, not of a diagnostic: `MetadataKeywordUsage`
no longer has an `about_targets` to put those targets in.

## Owning scopes

`#` reaches every body twice -- as a prefix on any member (`PrefixMetadataMember`, via each
member's own `*ExtensionKeyword*`) and as a standalone member (`ExtendedUsage`, a
`NonOccurrenceUsageElement` and so a `DefinitionBodyItem`). `@` reaches every body once, as the
`MetadataFeature` alternative of `AnnotatingElement`; `planning/annotating-member-matrix.md` owns
that distribution and is not repeated here.

| AST scope | `@` before | `@` after | `#` before | `#` after |
| --- | --- | --- | --- | --- |
| `PackageBodyElement` | typed | unchanged | typed | typed, reference-backed |
| `PartDefBodyElement` | typed | unchanged | typed (+ opaque `Annotation` fallback) | typed, fallback removed |
| `PartUsageBodyElement` | typed | unchanged | typed (+ opaque fallback) | typed, fallback removed |
| `AttributeBodyElement` | typed | unchanged | typed | typed, reference-backed |
| `PortDefBodyElement` | typed | unchanged | typed | typed, reference-backed |
| `OccurrenceBodyElement` | typed | unchanged | **opaque `Annotation` only** | typed `MetadataKeywordUsage` (new variant) |
| `ActionDefBodyElement` | typed | unchanged | typed (+ opaque fallback) | typed, fallback removed |
| `ActionUsageBodyElement` | typed | unchanged | typed (+ opaque fallback) | typed, fallback removed |
| `StateDefBodyElement` | typed | unchanged | typed (+ opaque fallback) | typed, fallback removed |
| `RequirementDefBodyElement` | typed | unchanged | typed (+ opaque fallback) | typed, fallback removed |
| `UseCaseDefBodyElement` | typed | unchanged | typed (+ **dead** `Annotation` variant) | typed, dead variant removed |
| `CalcDefBodyElement` | typed | unchanged | **none** | typed `MetadataKeywordUsage` (new variant) |
| `ConstraintDefBodyElement` | typed | unchanged | **none** | typed `MetadataKeywordUsage` (new variant) |
| `ViewDefBodyElement` | typed | unchanged | **none** | typed `MetadataKeywordUsage` (new variant) |
| `InterfaceDefBodyElement` | typed | unchanged | **none** | typed `MetadataKeywordUsage` (new variant) |
| `ConnectionDefBodyElement` | typed | unchanged | **none** | typed `MetadataKeywordUsage` (new variant) |
| `EnumerationBodyElement` | typed | unchanged | n/a | n/a -- `EnumerationBody` admits only `AnnotatingMember` and `EnumerationUsageMember` |

Two members reached these bodies only through the opaque `#` fallback and now have their own typed
dispatch, because `DefinitionMember → DefinitionElement → Dependency` admits them:
`ActionDefBodyElement::Dependency` and `RequirementDefBodyElement::Dependency`. The Apollo model's
`#refinement dependency PerformCrewIngress to …;` is the real-usage case; it used to become one
`Annotation` node holding the copied text `refinement dependency PerformCrewIngress to …`, with
neither endpoint reaching the reference arena.

## Recovery behaviour

The sigil classification in `classify_recovery` splits into two states, because "legal syntax this
parser does not model here" and "not a metadata reference at all" are different facts and a
consumer acts differently on each:

| Condition | Code | Severity | Category |
| --- | --- | --- | --- |
| Sigil followed by something that can begin a `[QualifiedName]` (letter, `_`, `'`, `$`) | `unsupported_annotation_syntax` | warning | `UnsupportedGrammarForm` |
| Sigil followed by anything else | `malformed_annotation_head` | error | `ParseError` |

Both retain the authored span, recover through the scope's own member loop, and keep the valid
siblings before and after them. `tests/snapshots/sysml/metadata_sigil_recovery.md` pins all of it,
including a recovery region containing a `}` inside a block comment, inside a string literal, and
as an escaped character -- none of which terminate the region early.

## Representation

```rust
pub struct MetadataAnnotation {          // the `@` spelling
    pub at_span: Span,                       // syntax/provenance, never part of a name
    pub declared_name: Option<Node<MetadataDeclaredName>>,
    pub type_reference: QualifiedReferenceId,  // OwnedFeatureTyping -- required
    pub type_span: Span,
    pub about_targets: Vec<QualifiedReferenceId>,
    pub body: AttributeBody,                 // MetadataBody
}

pub struct MetadataDeclaredName {        // Identification ( ':' | 'typed' 'by' )
    pub identification: Identification,      // a declaration label, never a reference
    pub typed_by: MetadataTypedBy,           // Colon | TypedBy -- authored, reproduced verbatim
    pub typed_by_span: Span,
}

pub struct MetadataKeywordUsage {        // the `#` spelling
    pub hash_span: Span,                     // syntax/provenance, never part of the reference
    pub reference: QualifiedReferenceId,     // OwnedFeatureTyping
    pub body: Option<AttributeBody>,         // None = PrefixMetadataMember, Some = ExtendedUsage
}
```

`MetadataTypedBy` is its own two-variant enum rather than a reuse of the wider
`TypingSpelling`: `specializes` and `defined by` are unreachable from this production and must not
be representable here.

`Option<AttributeBody>` is the discriminator between the two `#` productions, not a compressed
boolean: `crate::ast::Body`'s own documentation names the `#Name` prefix as one of exactly two
grammar contexts in which no body is written, and `tests/absent_body_confinement.rs` pins that no
third scope acquires one.

### Removed

- `Annotation` (the generic `@`/`#` fallback) and its `sigil: String` field. Its `@` branch was
  unreachable -- `annotating_member` won first in all seven dispatch sites -- and its `#` branch
  was the opaque capture.
- `AnnotationHead`, including `AnnotationHead::Opaque(String)`.
- `hash_annotation` and `annotation` (parser), and the `Annotation` variant on eight body-element
  enums.
- `ConnectBody` and its parser `connect_body`, together with `OpacityKind::OpaqueConnectBrace` and
  `walk_connect_body`. Its last two owners were `Annotation.body` and `Bind`, where it sat beside a
  separate `body_elements: Vec<_>` -- one body fact in two fields, with an empty `{}` and a
  discarded `{ … }` indistinguishable. `Bind.body` is a `PartUsageBody`, matching
  `BindingConnectorAsUsage`'s `UsageBody`.
- `MetadataKeywordUsage::{keyword, keyword_span, type_reference, type_span, about_targets}`.
- `MetadataAnnotation::{reference, head_span}` in their old meanings; `type_reference` is no longer
  optional.

## Left unsupported

Rows 19 and 20: `ExtendedUsage` with an **anonymous but specialized** `UsageDeclaration` --
`#Tag : T;`, `#Tag :>> y;`, `#Tag :>> y : T { … }`.

`ExtendedUsage = UnextendedUsagePrefix UsageExtensionKeyword+ Usage`, and `Usage`'s
`FeatureSpecializationPart` is what carries the `: T` / `:>> y`. The specialization belongs to the
**anonymous usage**, not to the metadata tag, so representing it needs an `ExtendedUsage` node that
owns a declaration -- the same shape row 18 already produces when the usage has a name, at which
point the tag and the usage are separate members and both parse. Modelling the anonymous case is a
usage-declaration change, not a metadata one, so it stays with the `ExtendedUsage` seam.

The superseded parser appeared to support `#Tag : T;` only by mislabelling it: the `: T` was stored
as the metadata tag's own `type_reference`, which is a field no `#` production has. Row 20
(`#servicedd :>> serviceDiscovery : ServiceDiscoveryDD { … }`, from
`tests/snapshots/spec42/sysml/examples/ahfcore_lib.md`) reached recovery before this change and
still does, unchanged.

Rows 23 and 24: `OwnedFeatureTyping`'s second alternative, `OwnedFeatureChain` -- the dotted
`@A.b;` / `#A.b;` spelling.

Both sigils resolve their typing through `qualified_reference`, which is the `[QualifiedName]`
alternative and rejects `.`. That is deliberate rather than an oversight: a feature chain is a
different identity domain from a qualified name, parsed by `reference_path` and carrying typed
`ReferenceSeparator::Dot` segments. Accepting one here would mean either widening what the
metadata typing claims to be, or storing a chain in a slot documented as a qualified name. Closing
it is a `reference_path`-versus-`qualified_reference` decision that every `OwnedFeatureTyping` site
shares, not a metadata one.

`metadata` -- the other spelling of the same `MetadataFeature` production -- keeps its own
`MetadataUsage` AST type with `name: String` and `type_reference: Option<_>`. Unifying the two
spellings is a further step: the shapes now agree on which half is a declaration and which is a
reference, but `MetadataUsage` still models the typing as optional, which the production does not.
Recorded here rather than done, because it changes a type this seam does not otherwise touch.
