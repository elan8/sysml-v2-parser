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
                    let result = sysml_v2_parser::parse_for_editor_with_name_language(
                        black_box(&source.text),
                        source.name_language,
                    );
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
                let result = sysml_v2_parser::parse_for_editor_with_name_language(
                    black_box(&source.text),
                    source.name_language,
                );
                black_box(result.document.root.elements.len());
                black_box(result.document.qualified_references.len());
                black_box(result.errors.len());
            }
        });
    });
    corpus.finish();

    // Strict parsing of the same corpus. The recovery and malformed fixtures make this the
    // measurement for how much work the strict entry point does on invalid input.
    let mut strict = c.benchmark_group("snapshot_strict_parser_corpus");
    strict.throughput(Throughput::Bytes(corpus_bytes));
    strict.bench_function("all_sources", |b| {
        b.iter(|| {
            for source in &sources {
                let result = sysml_v2_parser::parse_with_name_language(
                    black_box(&source.text),
                    source.name_language,
                );
                black_box(result.is_ok());
            }
        });
    });
    strict.finish();

    // Opacity scanning and emission over already-parsed documents, so parsing is not measured.
    let documents: Vec<_> = sources
        .iter()
        .map(|source| {
            sysml_v2_parser::parse_for_editor_with_name_language(&source.text, source.name_language)
                .document
        })
        .collect();
    let mut emit = c.benchmark_group("snapshot_emit_corpus");
    emit.throughput(Throughput::Bytes(corpus_bytes));
    emit.bench_function("opacity_report", |b| {
        b.iter(|| {
            for document in &documents {
                black_box(sysml_v2_parser::opacity_report(black_box(&document.root)).is_clean());
            }
        });
    });
    emit.bench_function("emit_sysml", |b| {
        b.iter(|| {
            for document in &documents {
                black_box(sysml_v2_parser::emit_sysml(black_box(document)).is_ok());
            }
        });
    });
    emit.finish();
}

criterion_group!(benches, bench_snapshot_corpus);
criterion_main!(benches);
