#[path = "../tools/parser_corpus.rs"]
mod parser_corpus;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench_snapshot_corpus(c: &mut Criterion) {
    let sources = parser_corpus::snapshot_sources().expect("load parser snapshot corpus");
    let mut group = c.benchmark_group("snapshot_parse_with_diagnostics");

    for source in &sources {
        group.throughput(Throughput::Bytes(source.text.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(&source.name),
            source,
            |b, source| {
                b.iter(|| {
                    let result = sysml_v2_parser::parse_with_diagnostics(black_box(&source.text));
                    black_box(result.document.root.elements.len());
                    black_box(result.document.qualified_references.len());
                    black_box(result.errors.len());
                });
            },
        );
    }
    group.finish();

    let corpus_bytes = sources.iter().map(|source| source.text.len() as u64).sum();
    let mut corpus = c.benchmark_group("snapshot_parser_corpus");
    corpus.throughput(Throughput::Bytes(corpus_bytes));
    corpus.bench_function("all_sources", |b| {
        b.iter(|| {
            for source in &sources {
                let result = sysml_v2_parser::parse_with_diagnostics(black_box(&source.text));
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
