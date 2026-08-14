use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const SNAPSHOT_ROOT: &str = "tests/snapshots";
const SOURCE_START: &str = "# SOURCE\n~~~sysml\n";
const SOURCE_END: &str = "\n~~~\n# DIAGNOSTICS";

pub struct SnapshotSource {
    pub name: String,
    pub text: String,
}

pub fn snapshot_sources() -> io::Result<Vec<SnapshotSource>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(SNAPSHOT_ROOT);
    let mut paths = Vec::new();
    collect_snapshots(&root, &mut paths)?;
    paths.sort();

    let sources: Vec<_> = paths
        .into_iter()
        .map(|path| {
            let markdown = fs::read_to_string(&path)?;
            let source_start = markdown
                .find(SOURCE_START)
                .map(|offset| offset + SOURCE_START.len())
                .ok_or_else(|| invalid_snapshot(&path, "missing canonical SOURCE section"))?;
            let source_end = markdown[source_start..]
                .find(SOURCE_END)
                .map(|offset| source_start + offset)
                .ok_or_else(|| invalid_snapshot(&path, "missing canonical DIAGNOSTICS boundary"))?;
            let name = path
                .strip_prefix(&root)
                .unwrap_or(&path)
                .with_extension("")
                .to_str()
                .ok_or_else(|| invalid_snapshot(&path, "snapshot filename is not UTF-8"))?
                .to_owned();
            Ok(SnapshotSource {
                name,
                text: markdown[source_start..source_end].to_owned(),
            })
        })
        .collect::<io::Result<_>>()?;

    if sources.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("no benchmark snapshots under {SNAPSHOT_ROOT}"),
        ));
    }
    Ok(sources)
}

/// Collect every fixture under `directory`, including nested corpus subdirectories.
fn collect_snapshots(directory: &Path, paths: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_snapshots(&path, paths)?;
        } else if path.extension().is_some_and(|extension| extension == "md") {
            paths.push(path);
        }
    }
    Ok(())
}

fn invalid_snapshot(path: &Path, message: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("{}: {message}", path.display()),
    )
}
