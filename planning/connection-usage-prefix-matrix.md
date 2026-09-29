# `ConnectionUsage` occurrence-prefix matrix

The `ConnectionUsage` slice of the shared `OccurrenceUsagePrefix` seam, with the two neighbouring
`#`-prefix facts it could not be separated from: `ConnectionDefinition`'s
`DefinitionExtensionKeyword*` and the end declaration's `UsageExtensionKeyword+`. Pinned grammar:
`docs/conformance-target` (`release_tag=2026-04`). The component itself is audited once in
`planning/occurrence-usage-prefix-matrix.md` §§1-5; this document audits only what is specific to
connections, as `planning/port-usage-prefix-matrix.md` did for ports.

## 1. Authoritative productions

```text
ConnectionDefinition =                                          -- 664, clause 8.2.2.13
    OccurrenceDefinitionPrefix 'connection' 'def' Definition

ConnectionUsage =                                               -- 667
    OccurrenceUsagePrefix
    ( 'connection' UsageDeclaration ValuePart? ( 'connect' ConnectorPart )?
    | 'connect' ConnectorPart )
    UsageBody

OccurrenceDefinitionPrefix : OccurrenceDefinition =             -- 541
    BasicDefinitionPrefix? ( 'individual' EmptyMultiplicityMember )?
    DefinitionExtensionKeyword*

ExtendedUsage : Usage =                                         -- 1699
    UnextendedUsagePrefix UsageExtensionKeyword+ Usage

UsageExtensionKeyword : Usage = ownedRelationship += PrefixMetadataMember   -- 296
DefinitionExtensionKeyword : Definition = ownedRelationship += PrefixMetadataMember  -- 222
PrefixMetadataMember : OwningMembership = '#' ownedRelatedElement = PrefixMetadataUsage -- 1660
```

`#derivation`, `#original`, `#derive`, `#multicausation`, `#cause`, `#effect`, `#logical`,
`#physical` are all `PrefixMetadataMember`: `'#'` and the qualified name of a metadata definition.
None is a grammar keyword or a fixed role; which metadata applies is a semantic fact of the
library that defines it (`RequirementDerivation`, `CausationConnections`, ...).

## 2. Representation, before and after

| Fact | Before | After |
| --- | --- | --- |
| `connection` usage prefix | `is_abstract: bool`, `by_reference: bool`; no direction, `derived`, `constant`, `individual`, portion, `#` | `prefix: OccurrenceUsagePrefix` |
| usage declaration | `type_reference` only, multiplicity, `:>`/`:>>` *after* the body | `short_name`, `typing`, `multiplicity`, `multiplicity_modifiers`, `subsets`, `redefines`, `references`, `crosses`, `intersects`, `value`, all before the body |
| `def`-less `connection x … { }` at package level | `ConnectionDef` (definition grammar was `def`-optional) | `ConnectionUsage` |
| `#derivation` on a definition | `derivation_role: Option<Node<DerivationConnectionRole>>` | `extension_keywords: Vec<Node<UsageExtensionKeyword>>` |
| `#original`/`#derive` on an end | `EndIdentity::Derivation(DerivationEndRole)`, replacing the name; any other `#tag name` was consumed and discarded | `EndDecl::extension_keywords`, beside `EndIdentity::{Anonymous, Declaration}` |

## 3. Owning scopes and dispatch

`connection_usage_member` is dispatched from package/namespace/root, `part def` and `part` usage
bodies, occurrence bodies and attribute bodies; `connection_def` from package/namespace/root,
`part def` and `part` usage bodies. `def` is now required everywhere, so the two parsers are
mutually exclusive by the token after `connection`.

`#` and `ref` are contended heads (`occurrence_prefix::starts_contended_prefix`): the stand-alone
`PrefixMetadataMember` and `ReferenceUsage` parsers would otherwise claim `#derivation` or `ref`
and leave `connection …` as a separate member. Every owning scope therefore gives
`connection_def`/`connection_usage_member` first refusal inside its contended-prefix block, behind
a non-allocating `kind_keyword_follows(…, b"connection")` lookahead.

## 4. Corpus (pinned `sysml-v2-release`)

- `#derivation connection` ×3, `#multicausation connection` ×3, `#derivation connection def` ×1,
  `#multicausation connection def` ×1: extension keywords lead, nothing between them and the
  keyword.
- `end #…`: `#cause` ×6, `#effect` ×5, `#derive` ×5, `#original` ×3, `#physical` ×1, `#logical` ×1
  -- six distinct metadata names, which a fixed-role model cannot represent.
- `def`-less library usages: `abstract connection x : T[*] nonunique (:> …)? ( ; | { … } )`
  (`Connections.sysml`, `CausationConnections.sysml`, `DerivationConnections.sysml`, ...).

`Cause and Effect Examples/MedicalDeviceFailure.sysml` now round-trips and is promoted into
`EXAMPLES_ROUNDTRIP_PASS`; the library node-type gates stay green.

## 5. Retained non-BNF compatibility

A `:>`/`:>>` clause after a connection usage body, terminated by `;` (`connection : T { … } :>
capabilityToGoals;`, Apollo 11 model, `tests/apollo_regressions.rs`), still fills the `subsets`/
`redefines` slot when the header did not author it. It is documented at its parse site; emission
writes the clause in its BNF header position.

## 6. Recovery

No new recovery path. `end #mystery ::> X;` used to be a malformed member because only
`#original`/`#derive` were accepted; it is valid syntax and now parses
(`tests/snapshots/sysml/derivation_end_role_recovery.md`, retyped `semantic`).
`connection :>> c connect a to b;` in an occurrence body, previously
`unsupported_grammar_form`, parses (`occurrence_body_members.md`).

## 7. Coverage

- `tests/snapshots/sysml/connection_usage_prefix_owning_scopes.md`: every owning scope, the
  library declaration shape, `#` on usages, definitions and named/anonymous ends.
- Connection usages are projected structurally (`connection-usage …`) instead of as a marker, in
  every scope.
