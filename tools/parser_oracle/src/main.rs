//! Differential parse against an independent SysML v2 parser.
//!
//! Every `.sysml` / `.kerml` file under the corpus root is parsed by this crate and by OpenMBEE
//! sysml-toolkit's `sysmlv2-syntax`. A file lands in one of four classes:
//!
//! - `agree`: neither parser reports a syntax diagnostic;
//! - `ours-rejects`: only this parser reports one -- a lead for a grammar gap here;
//! - `theirs-rejects`: only the oracle reports one -- a lead for over-acceptance here, or a gap
//!   in the oracle;
//! - `both-reject`: both report one.
//!
//! Neither parser is the authority; the pinned OMG grammar is. A divergence is a lead to triage
//! against it. The non-`agree` classes are a ratchet: the run fails when the set differs from
//! the checked-in baseline in either direction, so a regression is caught and a fix is recorded.
//!
//! Usage: `parser_oracle <corpus-root> --baseline <file> [--update] [--report <file>]`

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Class {
    OursRejects,
    TheirsRejects,
    BothReject,
}

impl Class {
    fn as_str(self) -> &'static str {
        match self {
            Self::OursRejects => "ours-rejects",
            Self::TheirsRejects => "theirs-rejects",
            Self::BothReject => "both-reject",
        }
    }

    fn parse(text: &str) -> Option<Self> {
        match text {
            "ours-rejects" => Some(Self::OursRejects),
            "theirs-rejects" => Some(Self::TheirsRejects),
            "both-reject" => Some(Self::BothReject),
            _ => None,
        }
    }
}

/// First diagnostic of one parser for one file, `None` when it reported nothing.
struct Outcome {
    ours: Option<String>,
    theirs: Option<String>,
}

struct Args {
    root: PathBuf,
    baseline: PathBuf,
    update: bool,
    report: Option<PathBuf>,
}

fn parse_args() -> Result<Args, String> {
    let mut args = std::env::args().skip(1);
    let mut root = None;
    let mut baseline = None;
    let mut update = false;
    let mut report = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--baseline" => baseline = args.next().map(PathBuf::from),
            "--report" => report = args.next().map(PathBuf::from),
            "--update" => update = true,
            _ if root.is_none() => root = Some(PathBuf::from(arg)),
            other => return Err(format!("unexpected argument `{other}`")),
        }
    }
    Ok(Args {
        root: root.ok_or("missing <corpus-root>")?,
        baseline: baseline.ok_or("missing --baseline <file>")?,
        update,
        report,
    })
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("parser_oracle: {error}");
            eprintln!(
                "usage: parser_oracle <corpus-root> --baseline <file> [--update] [--report <file>]"
            );
            return ExitCode::from(2);
        }
    };
    // Both parsers recurse on nesting depth; give the whole run one generous stack.
    let run = std::thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(move || run(&args))
        .expect("spawn oracle thread");
    run.join().unwrap_or(ExitCode::FAILURE)
}

fn run(args: &Args) -> ExitCode {
    let mut files = Vec::new();
    collect(&args.root, &mut files);
    files.sort();
    if files.is_empty() {
        eprintln!(
            "parser_oracle: no .sysml/.kerml files under {}",
            args.root.display()
        );
        return ExitCode::from(2);
    }

    let mut outcomes = BTreeMap::new();
    for path in &files {
        let Ok(source) = std::fs::read_to_string(path) else {
            eprintln!("parser_oracle: cannot read {}", path.display());
            return ExitCode::from(2);
        };
        let kerml = path
            .extension()
            .is_some_and(|extension| extension == "kerml");
        let relative = path
            .strip_prefix(&args.root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        outcomes.insert(relative, parse_both(&source, kerml));
    }

    let observed: BTreeMap<String, Class> = outcomes
        .iter()
        .filter_map(|(path, outcome)| {
            let class = match (&outcome.ours, &outcome.theirs) {
                (None, None) => return None,
                (Some(_), None) => Class::OursRejects,
                (None, Some(_)) => Class::TheirsRejects,
                (Some(_), Some(_)) => Class::BothReject,
            };
            Some((path.clone(), class))
        })
        .collect();

    let report = render_report(&outcomes, &observed);
    print!("{report}");
    if let Some(report_path) = &args.report {
        if let Err(error) = std::fs::write(report_path, &report) {
            eprintln!(
                "parser_oracle: cannot write {}: {error}",
                report_path.display()
            );
            return ExitCode::from(2);
        }
    }

    if args.update {
        if let Err(error) = std::fs::write(&args.baseline, render_baseline(&observed)) {
            eprintln!(
                "parser_oracle: cannot write {}: {error}",
                args.baseline.display()
            );
            return ExitCode::from(2);
        }
        println!("baseline written: {}", args.baseline.display());
        return ExitCode::SUCCESS;
    }

    let baseline = match read_baseline(&args.baseline) {
        Ok(baseline) => baseline,
        Err(error) => {
            eprintln!("parser_oracle: {error}");
            return ExitCode::from(2);
        }
    };
    compare(&baseline, &observed)
}

fn parse_both(source: &str, kerml: bool) -> Outcome {
    let ours = std::panic::catch_unwind(|| {
        let result = sysml_v2_parser::parse_for_editor(source);
        result.errors.first().map(|error| {
            let line = error.line.map(|line| line.to_string()).unwrap_or_default();
            format!("{}:{}", line, error.message)
        })
    })
    .unwrap_or_else(|_| Some("panic".to_owned()));
    let theirs = std::panic::catch_unwind(|| {
        let parse = if kerml {
            sysmlv2_syntax::parser::parse_kerml_source(source)
        } else {
            sysmlv2_syntax::parser::parse_source(source)
        };
        // The oracle parses a superset grammar and reports body-context legality (a member the
        // grammar does not allow in its body) in a separate syntax-crate pass; this parser
        // rejects those at parse time, so both are its syntax verdict. That pass also checks a
        // few named abstract-syntax rules (`validateCaseUsageOnlyOneSubject: ...`); those belong
        // to the semantic layer here (Spec42), not to the parser, so they are left out.
        parse
            .diagnostics
            .first()
            .map(|diagnostic| format!("{diagnostic:?}"))
            .or_else(|| {
                sysmlv2_syntax::check::validate(&parse.unit)
                    .iter()
                    .find(|diagnostic| !is_named_semantic_rule(&diagnostic.message))
                    .map(|diagnostic| format!("{diagnostic:?}"))
            })
    })
    .unwrap_or_else(|_| Some("panic".to_owned()));
    Outcome { ours, theirs }
}

/// `validateX: ...` / `checkX: ...`: a diagnostic of a named KerML/SysML abstract-syntax rule.
fn is_named_semantic_rule(message: &str) -> bool {
    message.split_once(": ").is_some_and(|(rule, _)| {
        (rule.starts_with("validate") || rule.starts_with("check"))
            && rule.chars().all(|ch| ch.is_ascii_alphanumeric())
    })
}

/// A diagnostic message with its identifiers removed, so that one grammar gap reported in many
/// files is counted once (`unexpected token in action body`, not one line per file).
fn pattern(message: &str) -> String {
    let message = message.split_once(':').map_or(message, |(_, rest)| rest);
    let mut out = String::new();
    let mut in_quote = false;
    for ch in message.chars() {
        match ch {
            '`' | '\'' => {
                in_quote = !in_quote;
                if in_quote {
                    out.push_str("`…`");
                }
            }
            _ if in_quote => {}
            _ => out.push(ch),
        }
    }
    out.trim().to_owned()
}

fn render_report(
    outcomes: &BTreeMap<String, Outcome>,
    observed: &BTreeMap<String, Class>,
) -> String {
    let mut counts = BTreeMap::<&str, usize>::new();
    counts.insert("agree", outcomes.len() - observed.len());
    for class in observed.values() {
        *counts.entry(class.as_str()).or_default() += 1;
    }
    let mut out = String::new();
    let _ = writeln!(out, "# Parser oracle: sysml-v2-parser vs sysmlv2-syntax\n");
    let _ = writeln!(out, "{} files\n", outcomes.len());
    let _ = writeln!(out, "| class | files |\n| --- | ---: |");
    for (class, count) in &counts {
        let _ = writeln!(out, "| {class} | {count} |");
    }

    let mut patterns = BTreeMap::<String, Vec<&str>>::new();
    for (path, class) in observed {
        if matches!(class, Class::OursRejects | Class::BothReject) {
            let message = outcomes[path].ours.as_deref().unwrap_or_default();
            patterns.entry(pattern(message)).or_default().push(path);
        }
    }
    let mut patterns: Vec<_> = patterns.into_iter().collect();
    patterns.sort_by(|left, right| right.1.len().cmp(&left.1.len()).then(left.0.cmp(&right.0)));
    let _ = writeln!(
        out,
        "\n## This parser rejects: first diagnostic per file, by pattern\n"
    );
    for (pattern, paths) in &patterns {
        let _ = writeln!(out, "- **{pattern}** ({} files)", paths.len());
        for path in paths {
            let message = outcomes[*path].ours.as_deref().unwrap_or_default();
            let line = message.split_once(':').map_or("", |(line, _)| line);
            let _ = writeln!(out, "  - `{path}` line {line}");
        }
    }

    let _ = writeln!(out, "\n## Only the oracle rejects\n");
    for (path, class) in observed {
        if *class == Class::TheirsRejects {
            let message = outcomes[path].theirs.as_deref().unwrap_or_default();
            let short: String = message.chars().take(200).collect();
            let _ = writeln!(out, "- `{path}`: {short}");
        }
    }
    out
}

fn render_baseline(observed: &BTreeMap<String, Class>) -> String {
    let mut out = String::from(
        "# Parser-oracle ratchet: every corpus file the two parsers do not both accept.\n\
         # Regenerate with `scripts/parser-oracle.sh --update` after triaging the change.\n",
    );
    for (path, class) in observed {
        let _ = writeln!(out, "{}\t{path}", class.as_str());
    }
    out
}

fn read_baseline(path: &Path) -> Result<BTreeMap<String, Class>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("cannot read baseline {}: {error}", path.display()))?;
    let mut baseline = BTreeMap::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let (class, file) = line.split_once('\t').ok_or_else(|| {
            format!(
                "{}:{}: expected `<class>\\t<path>`",
                path.display(),
                index + 1
            )
        })?;
        let class = Class::parse(class)
            .ok_or_else(|| format!("{}:{}: unknown class `{class}`", path.display(), index + 1))?;
        baseline.insert(file.to_owned(), class);
    }
    Ok(baseline)
}

fn compare(baseline: &BTreeMap<String, Class>, observed: &BTreeMap<String, Class>) -> ExitCode {
    let paths: BTreeSet<&String> = baseline.keys().chain(observed.keys()).collect();
    let mut changes = Vec::new();
    for path in paths {
        let before = baseline.get(path).map_or("agree", |class| class.as_str());
        let after = observed.get(path).map_or("agree", |class| class.as_str());
        if before != after {
            changes.push(format!("  {path}: {before} -> {after}"));
        }
    }
    if changes.is_empty() {
        println!("\nparser oracle: matches the baseline");
        return ExitCode::SUCCESS;
    }
    eprintln!(
        "\nparser oracle: {} file(s) changed class against the baseline:",
        changes.len()
    );
    for change in &changes {
        eprintln!("{change}");
    }
    eprintln!(
        "Triage each change against the pinned grammar, then record it with \
         `scripts/parser-oracle.sh --update`."
    );
    ExitCode::FAILURE
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "sysml" || extension == "kerml")
        {
            out.push(path);
        }
    }
}
