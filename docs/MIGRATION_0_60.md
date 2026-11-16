# Migrating to v0.60.0

This guide covers upgrading from v0.59.0. The crate version is `0.60.0`, the minimum Rust version
stays **1.88**, and `PARSE_AST_VERSION` moves from **290 to 293**. The intermediate AST numbers in
commit trailers describe development changes within this release; 293 is the released schema to
target. Reparse source documents to rebuild caches. Serialized v0.59.0 envelopes are rejected
rather than upgraded by deserialization.

## Model changes

No text that parsed in v0.59.0 is rejected. These forms used to be errors and now parse:

| Text | Now |
| --- | --- |
| `attribute a : Real;` in a `view def` or `view` body | An attribute member of the view |
| `port p : P;` (and `ref port`) in a `state def` or `state` body | A port member of the state, with the typed port path |
| `connection :>> lines[3] { ... }` | A connection usage whose multiplicity follows its redefinition or subsetting |
| `action`, `state`, `exhibit state`, `calc`, `constraint` or `perform` in a `connection def` body | A member of the connection definition, as in a `part def` body |

## Consumer changes

Each change adds enum variants. Exhaustive matches over these enums fail to compile until the new
variants are handled, which is intended: decide what each consumer does with them.

- `ViewDefBodyElement` and `ViewBodyElement` gain `AttributeUsage` (AST 291).
- `StateDefBodyElement` gains `PortUsage` (AST 292). Referential ports use the same typed port path
  and complete spans as ordinary ports.
- `ConnectionDefBodyElement` gains `ActionDef`, `ActionUsage`, `StateDef`, `StateUsage`,
  `ExhibitState`, `CalcDef`, `CalcUsage`, `ConstraintDef`, `ConstraintUsage` and `Perform` (AST
  293). They carry the same node types as the part definition body members of the same name.
- A connection usage member parses its header with the shared `feature_usage_header`, so its
  typing, multiplicity and specialization clauses are filled in the same way as other usages.

The visitor, emitter and semantic snapshot projection already handle every new variant.
