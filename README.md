# sysml-parser

SysML v2 textual notation parser.

## Benchmarks

This repo includes Criterion benchmarks for parsing performance (editor scenario).

- Run all benches:

```bash
cargo bench
```

- Run the parser bench only:

```bash
cargo bench --bench parser_bench
```

### Fixture inputs

The primary benchmark fixture is read from:

- `C:\Git\sysml-examples\drone\sysml\SurveillanceDrone.sysml`

If the file is not present, that benchmark case is skipped.

Optional SysML v2 release fixtures are loaded from:

- `SYSML_V2_RELEASE_DIR` if set, otherwise `./sysml-v2-release`

Missing release fixtures are also skipped so `cargo bench` remains usable in minimal checkouts.

