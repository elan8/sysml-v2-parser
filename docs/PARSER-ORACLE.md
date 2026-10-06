# Parser oracle

`scripts/parser-oracle.sh` parses two corpora with this parser and with an independent one,
OpenMBEE sysml-toolkit's `sysmlv2-syntax` (Apache-2.0), and records every file the two do not
both accept. It runs in CI as the `parser-oracle` job.

Neither parser is the authority. The pinned OMG grammar (`docs/conformance-target`) is. A
divergence is a lead to triage against that grammar, not a verdict.

## What runs

| Corpus | Source | Baseline |
| --- | --- | --- |
| `release` | the pinned SysML v2 release (`scripts/fetch-sysml-v2-release.sh`): standard library, examples, training, validation, KerML examples | `tests/parser_oracle_release.tsv` |
| `opensysml` | the OpenSysML fixtures vendored by the oracle at its pinned revision, mostly *invalid* models | `tests/parser_oracle_opensysml.tsv` |

The oracle is cloned into `.oracle/` (gitignored) at `oracle_rev` from
`docs/parser-oracle-target`, sparse: its syntax crate and the fixture directory, no submodules.
`tools/parser_oracle` is a standalone package, so the oracle never enters this crate's
dependency graph.

The oracle's verdict is its parse diagnostics plus its body-context check (`check::validate`):
it parses a superset grammar and reports members the grammar does not allow in a body
separately, where this parser rejects them at parse time. Named abstract-syntax rules from that
check (`validateX: ...`) are left out; they belong to the semantic layer (Spec42).

## Classes

- `ours-rejects`: only this parser reports a syntax diagnostic. On `release` (valid models)
  this is a grammar gap here. On `opensysml` it is usually a correct rejection of an invalid
  fixture that the oracle's superset parser accepts.
- `theirs-rejects`: only the oracle reports one. This parser may accept text the grammar
  rejects (a reserved word as a name, an import without visibility, a member outside its body).
- `both-reject`: both report one.

Files both parsers accept are not listed.

## The ratchet

A run fails when any file changes class against its baseline, in either direction. A regression
is caught, and a fix has to be recorded. After triaging a change:

```sh
scripts/parser-oracle.sh --update
```

Reports with the first diagnostic of each file, grouped by pattern, are written to
`conformance-out/parser-oracle-<corpus>.md` (a CI artifact).

Bump `oracle_rev` deliberately: run the oracle, triage the files that changed class, then
update the baselines in the same change.
