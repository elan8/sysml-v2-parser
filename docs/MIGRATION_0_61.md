# Migrating to v0.61.0

This guide covers upgrading from v0.60.0. The crate version is `0.61.0`, the minimum Rust version
stays **1.88**, and `PARSE_AST_VERSION` moves from **293 to 295**. Schema 294 was an intermediate
development version; 295 is the released schema to target. Reparse source documents to rebuild
caches. Serialized v0.60.0 envelopes are rejected rather than upgraded by deserialization.

## Model changes

| Text | v0.61.0 behavior |
| --- | --- |
| `item x ::> original;` and the equivalent `references` spelling | Retains the relationship instead of silently dropping it; also applies to action, state, constraint, allocation and enumeration usages |
| `calc x ::> original;`, `interface x references original;`, `rendering ::> original;` | Accepts and preserves the relationship, including anonymous declarations and interfaces with `connect` |
| `ref attribute :>> quantity = value;` inside a `part def` | Parses as a typed reference attribute usage with its redefinition and value |
| A `binding` member following an unsupported KerML connector body | Survives local recovery as a separate typed sibling |
| Prefixed, nested and other state-body members | Outer member spans cover the complete authored member |
| `assert;` or `assert not;` without a referenced constraint | Reports a syntax error; editor parsing retains the malformed member and continues with later siblings |

To repair an incomplete assertion, name the intended constraint (`assert not target;`) or write
an inline constraint (`assert constraint { true }`). Existing valid assertions remain accepted.

## Consumer changes

The following fields are added; update struct literals and exhaustive variant destructuring:

| AST type | Added field |
| --- | --- |
| `ItemUsage`, `ActionUsage`, `StateUsage`, `ConstraintUsage`, `AllocationUsage`, `EnumerationUsage` | `references: Option<Node<SubsettingRelationship>>` (schema 294) |
| `CalcUsage`, `RenderingUsage` | `references: Option<Node<SubsettingRelationship>>` (schema 295) |
| `InterfaceUsage::TypedConnect`, `InterfaceUsage::Declaration` | `references: Option<Node<SubsettingRelationship>>` (schema 295) |

The owning visitor, emitter and semantic snapshot projection handle the new fields. Consumers
with custom projections should process these relationships too. Keep reference IDs with their
owning `ParsedDocument` and resolve them through its reference arena.

State-body member spans are now non-empty and include prefixes and nested bodies. Consumers
that used the old zero-length wrapper spans should use the corrected full member range.

## Remaining limits

Reference (`::>` / `references`) and cross (`=>` / `crosses`) subsettings still accept comma lists
and merge repeated clauses into one relationship. The grammar permits one target per clause;
the metamodel also limits the number of these relationships owned by a feature. This pre-existing
behavior is tracked in [#236](https://github.com/elan8/sysml-v2-parser/issues/236). Avoid these
forms when producing conforming source; successful parsing does not establish semantic validity.

The OMG grammar pin is unchanged. The optional toolkit reference audit is supplementary evidence,
not a switch to the toolkit's Pilot grammar. See [CONFORMANCE.md](CONFORMANCE.md) for corpus limits
and the two pinned library recovery exceptions.
