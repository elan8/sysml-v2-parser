# BNF coverage gate

This repo now keeps a machine-readable coverage map for the SysML/KerML textual BNF:

- coverage map: `docs/bnf_coverage.map`
- gate: `tests/bnf_compliance.rs`
- normative BNF source: `SYSML_V2_RELEASE_DIR/bnf/*.kebnf`, defaulting to `./sysml-v2-release` in the repo root
- pinned release tag + production counts: [`docs/conformance-target`](./conformance-target) (see [`CONFORMANCE.md`](./CONFORMANCE.md))

The coverage status labels are:

- `implemented`: grammar production has dedicated parser coverage and should not rely on opaque body skipping.
- `partial`: parser accepts known/common forms, but grammar depth is not yet complete.
- `opaque`: parser recognizes the construct but skips brace-body contents.
- `fallback`: production is represented by modeled fallback nodes rather than a construct-specific AST.
- `untested`: production is known in the BNF but still needs coverage classification work.
- `not_supported`: intentionally out of scope for this textual parser.

Run the gate with:

```powershell
cargo test --test bnf_compliance -- --nocapture
```

The test fails when a BNF production is not covered by the map, when two equally-specific rules assign conflicting statuses, or when the BNF production counts drift from the current SysML v2 release baseline.

Additional gate tests (see `tests/bnf_compliance.rs`):

- `coverage_map_rules_use_no_partial_status` — every map rule must be `implemented` (no `partial` rows).
- `all_textual_bnf_productions_are_implemented` — all 640 SysML/KerML productions resolve to `implemented`.
- `implemented_wildcard_patterns_do_not_target_opaque_body_helper_families` — wildcard `Flow*` / `Allocation*` / `Metadata*` rules must not claim `implemented` while bodies are opaque.
