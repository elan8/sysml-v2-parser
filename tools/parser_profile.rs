mod parser_corpus;

use std::hint::black_box;
use std::time::Instant;

fn main() {
    let iterations = std::env::args()
        .nth(1)
        .map(|value| {
            value
                .parse::<usize>()
                .expect("iteration count must be an integer")
        })
        .unwrap_or(1_000);
    assert!(iterations > 0, "iteration count must be positive");

    let sources = parser_corpus::snapshot_sources().expect("load parser snapshot corpus");
    let corpus_bytes: usize = sources.iter().map(|source| source.text.len()).sum();

    // Populate lazy runtime state before Samply's measured steady-state loop.
    for source in &sources {
        black_box(&source.name);
        black_box(sysml_v2_parser::parse_with_diagnostics(&source.text));
    }

    let started = Instant::now();
    let mut checksum = 0usize;
    for _ in 0..iterations {
        for source in &sources {
            let result = sysml_v2_parser::parse_with_diagnostics(black_box(&source.text));
            checksum = checksum.wrapping_add(result.document.root.elements.len());
            checksum = checksum.wrapping_add(result.document.qualified_references.len());
            checksum = checksum.wrapping_add(result.errors.len());
        }
    }
    black_box(checksum);

    let elapsed = started.elapsed();
    let parsed_bytes = corpus_bytes.saturating_mul(iterations);
    eprintln!(
        "{} snapshot sources, {} corpus bytes, {} iterations: {:.3}s ({:.2} MiB/s), checksum {}",
        sources.len(),
        corpus_bytes,
        iterations,
        elapsed.as_secs_f64(),
        parsed_bytes as f64 / elapsed.as_secs_f64() / (1024.0 * 1024.0),
        checksum
    );
}
