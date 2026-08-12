//! End-to-end Markdown snapshots for the parsed-document qualified-reference contract.

#[path = "../tools/snapshot_tool/support.rs"]
mod support;

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
    let snapshots = support::regenerate_snapshots(&paths).expect("regenerate snapshots");
    for snapshot in snapshots {
        assert_eq!(
            snapshot.original.replace("\r\n", "\n"),
            snapshot.rendered,
            "qualified-reference snapshot mismatch: {}",
            snapshot.path.display()
        );
    }
}
