//! Shared rendering and Markdown mechanics for the qualified-reference snapshot driver.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use sysml_v2_parser::ast::WriteSemanticAst;
use sysml_v2_parser::{
    emit_recovered_sysml, emit_sysml, parse, parse_for_editor, DiagnosticCategory,
    DiagnosticSeverity, ParseError,
};

pub(crate) const DEFAULT_SNAPSHOT_ROOT: &str = "tests/snapshots/qualified_references";

struct Snapshot {
    source: String,
    diagnostics: String,
    format: String,
    ast: String,
}

impl Snapshot {
    fn source(text: &str) -> Result<String, String> {
        fenced_section(text, "SOURCE")
    }

    fn write_to<W: io::Write + ?Sized>(&self, writer: &mut W) -> io::Result<()> {
        write!(
            writer,
            "# SOURCE\n~~~sysml\n{}\n~~~\n# DIAGNOSTICS\n~~~sexpr\n{}\n~~~\n# FORMAT\n~~~sysml\n{}\n~~~\n# AST\n~~~sexpr\n{}\n~~~\n",
            self.source, self.diagnostics, self.format, self.ast
        )
    }
}

fn fenced_section(text: &str, heading: &str) -> Result<String, String> {
    let marker = format!("# {heading}\n");
    let after_heading = text
        .split_once(&marker)
        .ok_or_else(|| format!("missing `{marker}` snapshot section"))?
        .1;
    let after_fence = after_heading
        .split_once('\n')
        .ok_or_else(|| format!("missing opening fence after `{marker}`"))?
        .1;
    let content = after_fence
        .split_once("\n~~~")
        .ok_or_else(|| format!("missing closing fence after `{marker}`"))?
        .0;
    Ok(content.trim_end_matches('\n').to_owned())
}

pub(crate) fn snapshot_paths(root: &Path, fixture: Option<&Path>) -> Result<Vec<PathBuf>, String> {
    let target = match fixture {
        Some(fixture) if fixture.is_absolute() => fixture.to_owned(),
        Some(fixture) if root.join(fixture).exists() => root.join(fixture),
        Some(fixture) => fixture.to_owned(),
        None => root.to_owned(),
    };
    if !target.exists() {
        return Err(format!(
            "snapshot path does not exist: {}",
            target.display()
        ));
    }
    if target.is_file() {
        return (target
            .extension()
            .is_some_and(|extension| extension == "md"))
        .then_some(vec![target.clone()])
        .ok_or_else(|| format!("snapshot is not Markdown: {}", target.display()));
    }
    let mut paths = Vec::new();
    visit_markdown(&target, &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn visit_markdown(directory: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(directory)
        .map_err(|error| format!("{}: read directory failed: {error}", directory.display()))?
    {
        let path = entry
            .map_err(|error| format!("{}: directory entry failed: {error}", directory.display()))?
            .path();
        if path.is_dir() {
            visit_markdown(&path, paths)?;
        } else if path.extension().is_some_and(|extension| extension == "md") {
            paths.push(path);
        }
    }
    Ok(())
}

fn write_quoted<W: io::Write + ?Sized>(writer: &mut W, value: &str) -> io::Result<()> {
    writer.write_all(b"\"")?;
    for character in value.chars() {
        if character == '\\' {
            writer.write_all(b"\\\\")?;
        } else if character == '"' {
            writer.write_all(b"\\\"")?;
        } else if character == '\n' {
            writer.write_all(b"\\n")?;
        } else if character == '\r' {
            writer.write_all(b"\\r")?;
        } else if character == '\t' {
            writer.write_all(b"\\t")?;
        } else {
            let mut bytes = [0; 4];
            writer.write_all(character.encode_utf8(&mut bytes).as_bytes())?;
        }
    }
    writer.write_all(b"\"")
}

fn write_optional_quoted<W: io::Write + ?Sized>(
    writer: &mut W,
    value: Option<&str>,
) -> io::Result<()> {
    if let Some(value) = value {
        write_quoted(writer, value)
    } else {
        writer.write_all(b"none")
    }
}

fn write_optional_number<W: io::Write + ?Sized, T: std::fmt::Display>(
    writer: &mut W,
    value: Option<T>,
) -> io::Result<()> {
    if let Some(value) = value {
        write!(writer, "{value}")
    } else {
        writer.write_all(b"none")
    }
}

fn write_severity<W: io::Write + ?Sized>(
    writer: &mut W,
    severity: Option<DiagnosticSeverity>,
) -> io::Result<()> {
    match severity {
        Some(DiagnosticSeverity::Error) => writer.write_all(b"error"),
        Some(DiagnosticSeverity::Warning) => writer.write_all(b"warning"),
        None => writer.write_all(b"none"),
    }
}

fn write_category<W: io::Write + ?Sized>(
    writer: &mut W,
    category: Option<DiagnosticCategory>,
) -> io::Result<()> {
    match category {
        Some(DiagnosticCategory::ParseError) => writer.write_all(b"parseerror"),
        Some(DiagnosticCategory::UnsupportedGrammarForm) => {
            writer.write_all(b"unsupportedgrammarform")
        }
        Some(DiagnosticCategory::UnresolvedSymbol) => writer.write_all(b"unresolvedsymbol"),
        None => writer.write_all(b"none"),
    }
}

fn write_diagnostics<W: io::Write + ?Sized>(
    writer: &mut W,
    name: &str,
    errors: &[ParseError],
) -> io::Result<()> {
    writer.write_all(b"(fixture-diagnostics\n  (document ")?;
    write_quoted(writer, name)?;
    writer.write_all(b"\n    (diagnostics")?;
    for error in errors {
        writer.write_all(b"\n      (diagnostic (code ")?;
        write_optional_quoted(writer, error.code.as_deref())?;
        writer.write_all(b") (severity ")?;
        write_severity(writer, error.severity)?;
        writer.write_all(b") (category ")?;
        write_category(writer, error.category)?;
        writer.write_all(b") (span (offset ")?;
        write_optional_number(writer, error.offset)?;
        writer.write_all(b") (line ")?;
        write_optional_number(writer, error.line)?;
        writer.write_all(b") (column ")?;
        write_optional_number(writer, error.column)?;
        writer.write_all(b") (len ")?;
        write_optional_number(writer, error.length)?;
        writer.write_all(b")) (message ")?;
        write_quoted(writer, &error.message)?;
        writer.write_all(b"))")?;
    }
    writer.write_all(b"\n    )\n  )\n)")
}

fn into_utf8(bytes: Vec<u8>, context: &str) -> Result<String, String> {
    String::from_utf8(bytes).map_err(|error| format!("{context} produced invalid UTF-8: {error}"))
}

fn actual_snapshot(path: &Path, source: String) -> Result<Snapshot, String> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("snapshot filename is not UTF-8: {}", path.display()))?;
    let editor = parse_for_editor(&source);
    let mut diagnostics = Vec::new();
    write_diagnostics(&mut diagnostics, file_name, &editor.errors)
        .map_err(|error| format!("{}: diagnostics write failed: {error}", path.display()))?;
    if editor.errors.is_empty() {
        let strict = parse(&source)
            .map_err(|error| format!("{}: strict parse failed: {error}", path.display()))?;
        if strict.normalize_for_test_comparison()
            != editor.document.root.normalize_for_test_comparison()
        {
            return Err(format!(
                "{}: strict/editor typed structure mismatch",
                path.display()
            ));
        }
    }
    let format = if editor.errors.is_empty() {
        emit_sysml(&editor.document)
    } else {
        emit_recovered_sysml(&editor.document)
    }
    .map_err(|error| {
        format!(
            "{}: parsed document could not be emitted: {error}",
            path.display()
        )
    })?;
    let mut ast = Vec::new();
    editor
        .document
        .write_semantic_ast(&mut ast)
        .map_err(|error| format!("{}: semantic AST write failed: {error}", path.display()))?;
    Ok(Snapshot {
        source,
        diagnostics: into_utf8(diagnostics, "diagnostics formatter")?,
        format: format.trim_end_matches('\n').to_owned(),
        ast: into_utf8(ast, "semantic AST formatter")?,
    })
}

pub(crate) fn regenerate_snapshot(fixture: &str, path: &Path) -> Result<String, String> {
    let source = Snapshot::source(&fixture.replace("\r\n", "\n"))
        .map_err(|error| format!("{}: {error}", path.display()))?;
    let snapshot = actual_snapshot(path, source)?;
    let mut rendered = Vec::new();
    snapshot
        .write_to(&mut rendered)
        .map_err(|error| format!("{}: snapshot write failed: {error}", path.display()))?;
    into_utf8(rendered, "snapshot formatter")
}
