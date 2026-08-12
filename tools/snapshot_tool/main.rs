//! Check or update Markdown parser snapshots.

mod support;

use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Copy)]
enum Command {
    Check,
    Update,
}

struct Cli {
    command: Command,
    root: PathBuf,
    fixture: Option<PathBuf>,
}

fn usage() -> &'static str {
    "usage: cargo run --bin snapshot_tool -- <check|update> [--root PATH] [--fixture PATH]"
}

fn parse_cli() -> Result<Cli, String> {
    let mut args = env::args().skip(1);
    let command = match args.next().as_deref() {
        Some("check") => Command::Check,
        Some("update") => Command::Update,
        Some("-h" | "--help") => return Err(usage().to_owned()),
        Some(other) => return Err(format!("unknown command `{other}`\n{}", usage())),
        None => return Err(usage().to_owned()),
    };
    let mut root = PathBuf::from(support::DEFAULT_SNAPSHOT_ROOT);
    let mut fixture = None;
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--root" => {
                root = PathBuf::from(
                    args.next()
                        .ok_or_else(|| "--root requires a path".to_owned())?,
                );
            }
            "--fixture" => {
                fixture = Some(PathBuf::from(
                    args.next()
                        .ok_or_else(|| "--fixture requires a path".to_owned())?,
                ));
            }
            "-h" | "--help" => return Err(usage().to_owned()),
            other => return Err(format!("unknown option `{other}`\n{}", usage())),
        }
    }
    Ok(Cli {
        command,
        root,
        fixture,
    })
}

fn run() -> Result<(), String> {
    let cli = parse_cli()?;
    let paths = support::snapshot_paths(&cli.root, cli.fixture.as_deref())?;
    if paths.is_empty() {
        return Err(format!(
            "no Markdown snapshots found under {}",
            cli.root.display()
        ));
    }

    let mut stale = Vec::new();
    for snapshot in support::regenerate_snapshots(&paths)? {
        if snapshot.rendered == snapshot.original.replace("\r\n", "\n") {
            continue;
        }
        match cli.command {
            Command::Check => stale.push(snapshot.path),
            Command::Update => {
                fs::write(&snapshot.path, snapshot.rendered).map_err(|error| {
                    format!("{}: write failed: {error}", snapshot.path.display())
                })?;
                println!("updated {}", snapshot.path.display());
            }
        }
    }

    if stale.is_empty() {
        return Ok(());
    }
    eprintln!("stale snapshots (run `cargo run --bin snapshot_tool -- update`):");
    for path in stale {
        eprintln!("  {}", path.display());
    }
    Err("snapshot check failed".to_owned())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
