# Migrating to v0.58.0

This guide covers upgrading from v0.57.0. The crate version is `0.58.0`, the minimum Rust
version is **1.88**, and `PARSE_AST_VERSION` moves from **260 to 284**. The intermediate AST
numbers in the [changelog](../CHANGELOG.md) describe development changes within this release;
284 is the released schema to target. Reparse source documents to rebuild caches. Serialized
v0.57.0 envelopes are rejected rather than upgraded by deserialization.

## Consumer changes

- Keep source, reference arenas, name language and tree together as `ParsedDocument`.
  Resolve IDs through that document; serialize its complete envelope with the `serde` feature.
- Match new body-element and typed usage variants exhaustively. Action bodies now retain views,
  nested definitions and case-family usages. Case/use-case/state bodies retain `VariantUsage`;
  extension members appear only in their permitted owning body enums.
- `EntryAction`, `DoAction` and `ExitAction` replace flattened action fields with
  `action: StateActionUsage` and `body: ActionDefBody`. Match `Empty`, `Perform { target, value }`
  and `Effect`. `PerformActionTarget::Action` holds the complete declaration, including an
  anonymous one; `Reference` holds a reference. Values and effects are boxed. The obsolete
  `StateDefBodyElement::FirstStmt` and `ThenAction` proxy variants are removed. Recovery inside
  state subactions reports action-body diagnostics.
- Read all typing targets from `typing: Option<Node<TypingRelationship>>`, including its
  operator and conjugation. This replaces scalar `type_name` fields on item, enumeration,
  requirement, concern, frame, case, analysis, verification, use case, viewpoint, view rendering
  and allocation usages. `CalcUsage` and `ConstraintUsage` also retain full typing relationships.
  Allocation's mirrored `type_is_conjugated` field is removed.
- `RequireConstraint::specializations` and `KermlConnectorMember::specializations` retain
  ordered specialization alternatives. They replace separate scalar relationship fields.
  Required constraints also retain their value; multiplicity modifiers are structured.
- `Multiplicity::upper` is a required expression; `lower` is present only for an authored range.
  `Expression::LiteralInfinity` represents `*`. Preserve the distinction between `[1]` and `[1..1]`.
- Declaration names can be absent independently of short names. Use `Option<DeclarationName>`
  where exposed, and resolve a name through the document. `MetadataUsage::type_reference` is
  required; a lone name in `metadata Tag;` names the type, while `metadata named : Tag;` declares
  `named`. Namespace-level `calc X;` and `interface X;` are usages.
- Retain typed occurrence/control prefixes, succession modifiers, action values, actor anonymous
  redefinitions and `ExtendedDefinition::membership` when lowering nodes. New KerML fields retain
  feature short names, conjugation, invariant declarations and package-level successions.
- `Node<T>` provides structurally checked auto traits for AST payloads. Downstream consumers no
  longer need a raised recursion limit solely to prove AST `Send`/`Sync` or unwind-safety traits.

The detailed changelog is the field-by-field reference for the remaining AST additions.

## Syntax diagnostics and emission

The opaque declaration variants `ExtendedLibraryDecl`, `FeatureDecl`, `ClassifierDecl`,
`KermlSemanticDecl` and `KermlFeatureDecl`, plus `OpaqueText` and its document accessor, are
removed. A keyword alone no longer makes a failed declaration valid. Typed parsers handle
supported forms; malformed syntax becomes source-preserving recovery with diagnostics.

`parse` is the strict gate. Use `parse_for_editor` or `parse_with_diagnostics` for partial trees
and diagnostics. `emit_sysml` requires a clean structured tree; `emit_recovered_sysml` preserves
malformed slices at their positions. `OpacityHit` and `EmitError::Opaque` identify source `span`
instead of a tree-path string; `OpacityKind::RawRhsString` is removed. The scorecard uses the
shared recovery traversal and reports `unstructured_nodes`.

Unquoted reserved declaration names are retained as `DeclarationNameState::ReservedKeyword`
and diagnosed. Select KerML explicitly with `NameLanguage::Kerml`; `do` is a legal KerML name
but a SysML keyword. Quoted names remain valid. Imports require explicit visibility. SysML
definition headers reject KerML `disjoint from`; result-expression semicolon diagnostics apply
only to bodies that own result expressions. These are parser syntax checks, not semantic
well-formedness validation.

## Upgrade validation

Update exhaustive consumers, rebuild caches, then run their parse/format and serialization
regressions. Check anonymous and short-name-only declarations, multi-target typing, state
subactions, variants and extension members in the contexts your application uses. Avoid
reintroducing a first-type compatibility field or opaque fallback in downstream adapters.

The parser's [conformance evidence](CONFORMANCE.md) covers the pinned `2026-04` grammar and
corpora. It does not certify name resolution, typing or semantic well-formedness in a host
such as Spec42.
