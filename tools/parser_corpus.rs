use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const SNAPSHOT_ROOT: &str = "tests/snapshots/qualified_references";
const SOURCE_START: &str = "# SOURCE\n~~~sysml\n";
const SOURCE_END: &str = "\n~~~\n# DIAGNOSTICS";

pub struct SnapshotSource {
    pub name: String,
    pub text: String,
}

pub fn snapshot_sources() -> io::Result<Vec<SnapshotSource>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(SNAPSHOT_ROOT);
    let mut paths: Vec<PathBuf> = fs::read_dir(&root)?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|extension| extension == "md"))
        .collect();
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
                .file_stem()
                .and_then(|name| name.to_str())
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

fn invalid_snapshot(path: &Path, message: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("{}: {message}", path.display()),
    )
}
