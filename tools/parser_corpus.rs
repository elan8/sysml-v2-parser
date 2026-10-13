use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const SNAPSHOT_ROOT: &str = "tests/snapshots";
#[path = "snapshot_tool/metadata.rs"]
mod metadata;

pub struct SnapshotSource {
    pub name: String,
    pub text: String,
    pub name_language: sysml_v2_parser::ast::NameLanguage,
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
            let metadata::AuthoredSections {
                meta: _meta,
                source,
                name_language,
            } = metadata::authored_sections(&markdown.replace("\r\n", "\n"))
                .map_err(|message| invalid_snapshot(&path, &message))?;
            let name = path
                .strip_prefix(&root)
                .unwrap_or(&path)
                .with_extension("")
                .to_str()
                .ok_or_else(|| invalid_snapshot(&path, "snapshot filename is not UTF-8"))?
                .to_owned();
            Ok(SnapshotSource {
                name,
                text: source,
                name_language,
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
