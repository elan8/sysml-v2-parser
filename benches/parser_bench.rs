use std::fs;
use std::path::{Path, PathBuf};

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

const SNAPSHOT_ROOT: &str = "tests/snapshots/qualified_references";
const SOURCE_START: &str = "# SOURCE\n~~~sysml\n";
const SOURCE_END: &str = "\n~~~\n# DIAGNOSTICS";

fn snapshot_sources() -> Vec<(String, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(SNAPSHOT_ROOT);
    let mut paths: Vec<PathBuf> = fs::read_dir(&root)
        .unwrap_or_else(|error| panic!("read {}: {error}", root.display()))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|extension| extension == "md"))
        .collect();
    paths.sort();

    let sources: Vec<_> = paths
        .into_iter()
        .map(|path| {
            let markdown = fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            let source_start = markdown
                .find(SOURCE_START)
                .unwrap_or_else(|| panic!("{} has no canonical SOURCE section", path.display()))
                + SOURCE_START.len();
            let source_end = markdown[source_start..]
                .find(SOURCE_END)
                .map(|offset| source_start + offset)
                .unwrap_or_else(|| {
                    panic!("{} has no canonical DIAGNOSTICS boundary", path.display())
                });
            let name = path
                .file_stem()
                .and_then(|name| name.to_str())
                .expect("UTF-8 snapshot filename")
                .to_owned();
            (name, markdown[source_start..source_end].to_owned())
        })
        .collect();

    assert!(
        !sources.is_empty(),
        "no benchmark snapshots under {SNAPSHOT_ROOT}"
    );
    sources
}

fn bench_snapshot_corpus(c: &mut Criterion) {
    let sources = snapshot_sources();
    let mut group = c.benchmark_group("snapshot_parse_with_diagnostics");

    for (name, source) in &sources {
        group.throughput(Throughput::Bytes(source.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), source, |b, source| {
            b.iter(|| {
                let result = sysml_v2_parser::parse_with_diagnostics(black_box(source));
                black_box(result.document.root.elements.len());
                black_box(result.document.qualified_references.len());
                black_box(result.errors.len());
            });
        });
    }
    group.finish();

    let corpus_bytes = sources.iter().map(|(_, source)| source.len() as u64).sum();
    let mut corpus = c.benchmark_group("snapshot_parser_corpus");
    corpus.throughput(Throughput::Bytes(corpus_bytes));
    corpus.bench_function("all_sources", |b| {
        b.iter(|| {
            for (_, source) in &sources {
                let result = sysml_v2_parser::parse_with_diagnostics(black_box(source));
                black_box(result.document.root.elements.len());
                black_box(result.document.qualified_references.len());
                black_box(result.errors.len());
            }
        });
    });
    corpus.finish();
}

criterion_group!(benches, bench_snapshot_corpus);
criterion_main!(benches);
