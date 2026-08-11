//! End-to-end Markdown snapshots for the parsed-document qualified-reference contract.

#[path = "../tools/snapshot_tool/support.rs"]
mod support;

use std::fs;
use std::path::Path;

#[test]
fn qualified_reference_markdown_snapshots() {
    let root = Path::new(support::DEFAULT_SNAPSHOT_ROOT);
    let paths = support::snapshot_paths(root, None).expect("discover snapshot fixtures");
    assert!(
        !paths.is_empty(),
        "no snapshots found under {}",
        root.display()
    );
    for path in paths {
        let expected = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let actual = support::regenerate_snapshot(&expected, &path)
            .unwrap_or_else(|error| panic!("regenerate {}: {error}", path.display()));
        assert_eq!(
            expected.replace("\r\n", "\n"),
            actual,
            "qualified-reference snapshot mismatch: {}",
            path.display()
        );
    }
}
