//! Every document in the snapshot corpus must survive serialization.
//!
//! Serializing a [`ParsedDocument`](sysml_v2_parser::ast::ParsedDocument) validates the provenance
//! it carries: reference identities resolve, import targets describe their own tokens, and body
//! delimiters slice to the `{`, `}`, or `;` they claim, lie inside the declaration that owns them,
//! and wrap their own members. A span that is merely *plausible* passes a spot check but fails
//! here, because the corpus contains every construct the parser handles.
//!
//! This guards the invariants rather than the encoding: it does not compare bytes, so it stays
//! green when the wire shape changes deliberately.

#![cfg(feature = "serde")]

use std::path::{Path, PathBuf};

fn snapshot_sources() -> Vec<(PathBuf, String)> {
    let mut sources = Vec::new();
    let mut directories = vec![PathBuf::from("tests/snapshots")];
    while let Some(directory) = directories.pop() {
        let entries = std::fs::read_dir(&directory)
            .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()));
        for entry in entries {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                directories.push(path);
                continue;
            }
            if path.extension().and_then(|extension| extension.to_str()) != Some("md") {
                continue;
            }
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            if let Some(source) = snapshot_source_section(&text) {
                sources.push((path, source));
            }
        }
    }
    sources.sort_by(|left, right| left.0.cmp(&right.0));
    sources
}

/// The authored `SOURCE` section, which is the only human-written SysML in a snapshot file.
fn snapshot_source_section(text: &str) -> Option<String> {
    let rest = text.split("# SOURCE\n~~~sysml\n").nth(1)?;
    Some(rest.split("\n~~~").next()?.to_owned())
}

fn relative(path: &Path) -> String {
    path.display().to_string()
}

#[test]
fn every_snapshot_document_serializes_with_valid_provenance() {
    let sources = snapshot_sources();
    assert!(
        sources.len() > 100,
        "expected the whole snapshot corpus, found {} documents",
        sources.len()
    );

    let failures: Vec<String> = sources
        .iter()
        .filter_map(|(path, source)| {
            let document = sysml_v2_parser::parse_for_editor(source).document;
            serde_json::to_value(&document)
                .err()
                .map(|error| format!("{}: {error}", relative(path)))
        })
        .collect();

    assert!(
        failures.is_empty(),
        "{} of {} documents carry provenance their own validation rejects:\n{}",
        failures.len(),
        sources.len(),
        failures.join("\n")
    );
}

/// The same documents must survive a round trip, not just the outbound check.
#[test]
fn every_snapshot_document_deserializes_back() {
    let failures: Vec<String> = snapshot_sources()
        .iter()
        .filter_map(|(path, source)| {
            let document = sysml_v2_parser::parse_for_editor(source).document;
            let encoded = serde_json::to_value(&document).ok()?;
            match serde_json::from_value::<sysml_v2_parser::ast::ParsedDocument>(encoded) {
                Ok(decoded) if decoded == document => None,
                Ok(_) => Some(format!("{}: decoded document differs", relative(path))),
                Err(error) => Some(format!("{}: {error}", relative(path))),
            }
        })
        .collect();

    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
