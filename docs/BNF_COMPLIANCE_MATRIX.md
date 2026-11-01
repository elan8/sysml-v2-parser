# BNF coverage and structured-AST baseline

Current for **v0.59.0**, AST schema **289**. The authority for production classification is
[`bnf_coverage.map`](bnf_coverage.map), checked against both pinned BNF files identified by
[`conformance-target`](conformance-target). This document does not maintain a second set of
per-family status labels.

The map classifies all **350 SysML + 290 KerML = 640** textual productions as `implemented`.
This is a tested coverage inventory, not a proof of every legal combination or a semantic
conformance claim. See [the claim layers and corpus limits](CONFORMANCE.md).

## Structured declarations and bodies

| Area | Current representation |
| --- | --- |
| SysML definitions and usages | Construct-specific typed nodes, structured body members, prefixes and relationships |
| KerML classifiers, features, functions and connectors | Typed declaration/member nodes; the old opaque declaration fallback variants are removed |
| Extensions | Typed extended definitions and usages in their permitted owning body enums; native connector/interface ends keep scope-owned precedence |
| Actions and states | Typed case/definition/view members, variants, complete state subaction declarations and canonical action bodies |
| Typing and specialization | Typed relationships retain all targets where modeled; migrated usage families no longer expose a scalar first type |
| Malformed and unsupported input | Explicit source-backed recovery or unsupported nodes with diagnostics; a recognized keyword does not establish validity |

## Gates

- `cargo test --test bnf_compliance` checks production counts, map coverage and forbidden opaque
  body-helper patterns. Labels such as `partial`, `opaque` and `fallback` remain map-format
  vocabulary but are not current classifications.
- `CONFORMANCE_SCORECARD_L2=1 cargo test --test conformance_scorecard` verifies the 21 Systems
  Library files and all 94 standard-library files have zero diagnostics and unstructured nodes.
- `cargo test --test roundtrip_validation -- --include-ignored` checks all 56 pinned validation
  fixtures and the tracked examples inventory.
- `cargo snapshot check` verifies semantic/recovery/provenance fixtures and formatting stability.

Broader grammar combinations and consumer fidelity still need regression coverage as issues
are found. Parser acceptance does not validate name resolution, types, or SysML/KerML semantic
well-formedness; those belong to a host such as Spec42.
