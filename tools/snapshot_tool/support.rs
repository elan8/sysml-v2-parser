//! Shared rendering and Markdown mechanics for the qualified-reference snapshot driver.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::thread;

use sysml_v2_parser::ast::WriteSemanticAst;
use sysml_v2_parser::{
    emit_recovered_sysml, emit_sysml, parse, parse_for_editor, DiagnosticCategory,
    DiagnosticSeverity, EmitError, ParseError,
};

pub(crate) const DEFAULT_SNAPSHOT_ROOT: &str = "tests/snapshots";

struct Snapshot {
    meta: String,
    source: String,
    diagnostics: String,
    format: FormatSection,
    ast: String,
}

enum FormatSection {
    /// The formatter reproduced the canonical SOURCE payload byte-for-byte.
    StableIdempotent,
    /// The formatter changed the document, so retain the complete emitted SysML for review.
    Sysml(String),
    /// The typed tree intentionally retains syntax for which canonical emission is unavailable.
    UnavailableOpaqueAst,
}

impl FormatSection {
    fn from_output(source: &str, output: String) -> Self {
        if output.as_bytes() == source.as_bytes() {
            Self::StableIdempotent
        } else {
            Self::Sysml(output)
        }
    }

    fn from_emit_result(
        source: &str,
        result: Result<String, EmitError>,
    ) -> Result<Self, EmitError> {
        match result {
            Ok(output) => Ok(Self::from_output(
                source,
                output.trim_end_matches('\n').to_owned(),
            )),
            Err(EmitError::Opaque { .. }) => Ok(Self::UnavailableOpaqueAst),
            Err(error @ EmitError::Unsupported { .. }) => Err(error),
            Err(error @ EmitError::InvalidQualifiedReference { .. }) => Err(error),
            Err(error @ EmitError::InvalidSpan { .. }) => Err(error),
        }
    }

    fn write_to<W: io::Write + ?Sized>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Self::StableIdempotent => {
                writer.write_all(b"# FORMAT\n~~~sexpr\n(stable-idempotent)\n~~~\n")
            }
            Self::Sysml(output) => {
                writer.write_all(b"# FORMAT\n~~~sysml\n")?;
                writer.write_all(output.as_bytes())?;
                writer.write_all(b"\n~~~\n")
            }
            Self::UnavailableOpaqueAst => {
                writer.write_all(b"# FORMAT\n~~~sexpr\n(unavailable (reason opaque-ast))\n~~~\n")
            }
        }
    }
}

impl Snapshot {
    fn write_to<W: io::Write + ?Sized>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b"# META\n~~~sexpr\n")?;
        writer.write_all(self.meta.as_bytes())?;
        writer.write_all(b"\n~~~\n# SOURCE\n~~~sysml\n")?;
        writer.write_all(self.source.as_bytes())?;
        writer.write_all(b"\n~~~\n# DIAGNOSTICS\n~~~sexpr\n")?;
        writer.write_all(self.diagnostics.as_bytes())?;
        writer.write_all(b"\n~~~\n")?;
        self.format.write_to(writer)?;
        writer.write_all(b"# AST\n~~~sexpr\n")?;
        writer.write_all(self.ast.as_bytes())?;
        writer.write_all(b"\n~~~\n")
    }
}

struct AuthoredSections {
    meta: String,
    source: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SnapshotType {
    Semantic,
    Provenance,
    Recovery,
    Malformed,
}

#[derive(Debug, PartialEq, Eq)]
enum MetaToken {
    Open,
    Close,
    Symbol(String),
    String(String),
}

fn leading_fenced_section<'a>(
    text: &'a str,
    heading: &str,
    fence: &str,
) -> Result<(String, &'a str), String> {
    let marker = format!("# {heading}\n");
    let after_heading = text
        .strip_prefix(&marker)
        .ok_or_else(|| format!("expected `{marker}` as the next snapshot section"))?;
    let (opening_fence, after_fence) = after_heading
        .split_once('\n')
        .ok_or_else(|| format!("missing opening fence after `{marker}`"))?;
    let expected_fence = format!("~~~{fence}");
    if opening_fence != expected_fence {
        return Err(format!(
            "`{marker}` must use `{expected_fence}`, found `{opening_fence}`"
        ));
    }
    let (content, remainder) = after_fence
        .split_once("\n~~~\n")
        .ok_or_else(|| format!("missing closing fence after `{marker}`"))?;
    Ok((content.to_owned(), remainder))
}

fn tokenize_meta(text: &str) -> Result<Vec<MetaToken>, String> {
    let mut tokens = Vec::new();
    let mut characters = text.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            character if character.is_whitespace() => {}
            '(' => tokens.push(MetaToken::Open),
            ')' => tokens.push(MetaToken::Close),
            '"' => {
                let mut value = String::new();
                let mut closed = false;
                while let Some(character) = characters.next() {
                    match character {
                        '"' => {
                            closed = true;
                            break;
                        }
                        '\\' => {
                            let escaped = characters.next().ok_or_else(|| {
                                "META description ends after an escape".to_owned()
                            })?;
                            match escaped {
                                '\\' => value.push('\\'),
                                '"' => value.push('"'),
                                'n' => value.push('\n'),
                                'r' => value.push('\r'),
                                't' => value.push('\t'),
                                other => {
                                    return Err(format!(
                                        "META description has unsupported escape `\\{other}`"
                                    ));
                                }
                            }
                        }
                        other => value.push(other),
                    }
                }
                if !closed {
                    return Err("META description is missing its closing quote".to_owned());
                }
                tokens.push(MetaToken::String(value));
            }
            first => {
                let mut symbol = String::from(first);
                while let Some(next) = characters.peek().copied() {
                    if next.is_whitespace() || matches!(next, '(' | ')' | '"') {
                        break;
                    }
                    symbol.push(next);
                    characters.next();
                }
                tokens.push(MetaToken::Symbol(symbol));
            }
        }
    }
    Ok(tokens)
}

fn validate_meta(text: &str) -> Result<(), String> {
    let tokens = tokenize_meta(text)?;
    let [MetaToken::Open, MetaToken::Symbol(snapshot), MetaToken::Open, MetaToken::Symbol(type_field), MetaToken::Symbol(snapshot_type), MetaToken::Close, MetaToken::Open, MetaToken::Symbol(description_field), MetaToken::String(description), MetaToken::Close, MetaToken::Close] =
        tokens.as_slice()
    else {
        return Err("META must be `(snapshot (type <type>) (description \"...\"))`".to_owned());
    };
    if snapshot != "snapshot" || type_field != "type" || description_field != "description" {
        return Err("META must be `(snapshot (type <type>) (description \"...\"))`".to_owned());
    }
    let parsed_type = match snapshot_type.as_str() {
        "semantic" => SnapshotType::Semantic,
        "provenance" => SnapshotType::Provenance,
        "recovery" => SnapshotType::Recovery,
        "malformed" => SnapshotType::Malformed,
        unknown => {
            return Err(format!(
                "unknown META snapshot type `{unknown}`; expected semantic, provenance, recovery, or malformed"
            ));
        }
    };
    match parsed_type {
        SnapshotType::Semantic
        | SnapshotType::Provenance
        | SnapshotType::Recovery
        | SnapshotType::Malformed => {}
    }
    if description.trim().is_empty() {
        return Err("META description must not be empty".to_owned());
    }
    Ok(())
}

fn authored_sections(text: &str) -> Result<AuthoredSections, String> {
    let (meta, remainder) = leading_fenced_section(text, "META", "sexpr")?;
    validate_meta(&meta)?;
    let (source, _derived_sections) = leading_fenced_section(remainder, "SOURCE", "sysml")?;
    Ok(AuthoredSections { meta, source })
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

fn actual_snapshot(path: &Path, meta: String, source: String) -> Result<Snapshot, String> {
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
    let emitted = if editor.errors.is_empty() {
        emit_sysml(&editor.document)
    } else {
        emit_recovered_sysml(&editor.document)
    };
    let format = FormatSection::from_emit_result(&source, emitted).map_err(|error| {
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
        meta,
        source,
        diagnostics: into_utf8(diagnostics, "diagnostics formatter")?,
        format,
        ast: into_utf8(ast, "semantic AST formatter")?,
    })
}

pub(crate) fn regenerate_snapshot(fixture: &str, path: &Path) -> Result<String, String> {
    let normalized = fixture.replace("\r\n", "\n");
    let authored =
        authored_sections(&normalized).map_err(|error| format!("{}: {error}", path.display()))?;
    let snapshot = actual_snapshot(path, authored.meta, authored.source)?;
    let mut rendered = Vec::new();
    snapshot
        .write_to(&mut rendered)
        .map_err(|error| format!("{}: snapshot write failed: {error}", path.display()))?;
    into_utf8(rendered, "snapshot formatter")
}

pub(crate) struct RegeneratedSnapshot {
    pub(crate) path: PathBuf,
    pub(crate) original: String,
    pub(crate) rendered: String,
}

/// Regenerates independent fixtures concurrently while retaining sorted-path result order.
pub(crate) fn regenerate_snapshots(paths: &[PathBuf]) -> Result<Vec<RegeneratedSnapshot>, String> {
    if paths.is_empty() {
        return Ok(Vec::new());
    }
    let workers = thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1)
        .min(paths.len());
    let chunk_size = paths.len().div_ceil(workers);
    let mut results = thread::scope(|scope| {
        let handles = paths
            .chunks(chunk_size)
            .enumerate()
            .map(|(chunk_index, chunk)| {
                scope.spawn(move || {
                    chunk
                        .iter()
                        .enumerate()
                        .map(|(item_index, path)| {
                            let original = fs::read_to_string(path).map_err(|error| {
                                format!("{}: read failed: {error}", path.display())
                            })?;
                            let rendered = regenerate_snapshot(&original, path)?;
                            Ok((
                                chunk_index * chunk_size + item_index,
                                RegeneratedSnapshot {
                                    path: path.clone(),
                                    original,
                                    rendered,
                                },
                            ))
                        })
                        .collect::<Result<Vec<_>, String>>()
                })
            });
        let mut completed = Vec::with_capacity(paths.len());
        for handle in handles {
            completed.extend(
                handle
                    .join()
                    .map_err(|_| "snapshot worker panicked".to_owned())??,
            );
        }
        Ok::<_, String>(completed)
    })?;
    results.sort_unstable_by_key(|(index, _)| *index);
    Ok(results.into_iter().map(|(_, result)| result).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_format(format: &FormatSection) -> String {
        let mut rendered = Vec::new();
        format
            .write_to(&mut rendered)
            .expect("write format section");
        String::from_utf8(rendered).expect("format section is UTF-8")
    }

    #[test]
    fn byte_identical_format_uses_explicit_sentinel() {
        let section = FormatSection::from_output("package P;", "package P;".to_owned());
        assert_eq!(
            render_format(&section),
            "# FORMAT\n~~~sexpr\n(stable-idempotent)\n~~~\n"
        );
    }

    #[test]
    fn changed_format_retains_complete_sysml() {
        let section = FormatSection::from_output("package P;", "package P {\n}".to_owned());
        assert_eq!(
            render_format(&section),
            "# FORMAT\n~~~sysml\npackage P {\n}\n~~~\n"
        );
    }

    #[test]
    fn opaque_ast_format_uses_explicit_unavailable_sentinel() {
        let section = FormatSection::from_emit_result(
            "package P;",
            Err(EmitError::Opaque {
                path: "root.elements[0]".to_owned(),
                kind: sysml_v2_parser::OpacityKind::OpaqueConnectBrace,
            }),
        )
        .expect("opacity is a representable FORMAT outcome");

        assert_eq!(
            render_format(&section),
            "# FORMAT\n~~~sexpr\n(unavailable (reason opaque-ast))\n~~~\n"
        );
    }

    #[test]
    fn non_opacity_emit_error_still_aborts_regeneration() {
        let result = FormatSection::from_emit_result(
            "package P;",
            Err(EmitError::Unsupported {
                path: "root.elements[0]".to_owned(),
                construct: "test construct".to_owned(),
            }),
        );

        assert!(matches!(
            result,
            Err(EmitError::Unsupported { path, construct })
                if path == "root.elements[0]" && construct == "test construct"
        ));
    }

    #[test]
    fn comparison_is_byte_exact() {
        let section = FormatSection::from_output("package P;", "package P; ".to_owned());
        assert!(matches!(section, FormatSection::Sysml(_)));
    }

    #[test]
    fn regeneration_derives_format_from_source_not_existing_payload() {
        let meta = "# META\n~~~sexpr\n(snapshot (type semantic) (description \"Checks stable package formatting.\"))\n~~~\n";
        let sentinel_fixture = format!(
            "{meta}# SOURCE\n~~~sysml\npackage P;\n~~~\n# FORMAT\n~~~sexpr\n(stable-idempotent)\n~~~\n"
        );
        let stale_sysml_fixture = format!(
            "{meta}# SOURCE\n~~~sysml\npackage P;\n~~~\n# FORMAT\n~~~sysml\nnot the source\n~~~\n"
        );
        let path = Path::new("derived-format-contract.md");

        let from_sentinel =
            regenerate_snapshot(&sentinel_fixture, path).expect("regenerate sentinel fixture");
        let from_stale_sysml = regenerate_snapshot(&stale_sysml_fixture, path)
            .expect("regenerate stale SysML fixture");

        assert_eq!(from_sentinel, from_stale_sysml);
        assert!(from_sentinel.contains("# FORMAT\n~~~sexpr\n(stable-idempotent)\n~~~"));
    }

    #[test]
    fn regeneration_retains_valid_opaque_ast_with_unavailable_format() {
        let fixture = "# META\n~~~sexpr\n(snapshot (type semantic) (description \"Retains a parsed opaque transition body without claiming canonical emission.\"))\n~~~\n# SOURCE\n~~~sysml\npackage P { state def S { transition t then next { } } }\n~~~\n";
        let regenerated = regenerate_snapshot(fixture, Path::new("opaque-format.md"))
            .expect("opaque AST must not abort snapshot regeneration");

        assert!(regenerated.contains("# FORMAT\n~~~sexpr\n(unavailable (reason opaque-ast))\n~~~"));
        assert!(regenerated.contains("# AST\n~~~sexpr\n"));
    }

    #[test]
    fn authored_meta_is_preserved_byte_for_byte() {
        let meta = "(snapshot\n  (type provenance)\n  (description \"Preserves authored spacing and intent.\"))";
        let fixture =
            format!("# META\n~~~sexpr\n{meta}\n~~~\n# SOURCE\n~~~sysml\npackage P;\n~~~\n");
        let regenerated = regenerate_snapshot(&fixture, Path::new("meta-preservation.md"))
            .expect("regenerate fixture with authored META");
        assert!(regenerated.starts_with(&format!("# META\n~~~sexpr\n{meta}\n~~~\n# SOURCE\n")));
    }

    #[test]
    fn meta_requires_known_type_and_nonempty_description() {
        for snapshot_type in ["semantic", "provenance", "recovery", "malformed"] {
            validate_meta(&format!(
                "(snapshot (type {snapshot_type}) (description \"Meaningful intent.\"))"
            ))
            .unwrap_or_else(|error| panic!("{snapshot_type} should be accepted: {error}"));
        }

        let unknown = validate_meta(
            "(snapshot (type performance) (description \"Not a parser snapshot kind.\"))",
        )
        .expect_err("unknown type must fail");
        assert!(unknown.contains("unknown META snapshot type `performance`"));

        let empty = validate_meta("(snapshot (type semantic) (description \"   \"))")
            .expect_err("blank description must fail");
        assert_eq!(empty, "META description must not be empty");
    }

    #[test]
    fn missing_or_misordered_meta_is_rejected_instead_of_generated() {
        let missing = "# SOURCE\n~~~sysml\npackage P;\n~~~\n";
        let error = regenerate_snapshot(missing, Path::new("missing-meta.md"))
            .expect_err("missing META must fail");
        assert!(error.contains("expected `# META\n` as the next snapshot section"));

        let misordered = "# SOURCE\n~~~sysml\npackage P;\n~~~\n# META\n~~~sexpr\n(snapshot (type semantic) (description \"Too late.\"))\n~~~\n";
        let error = regenerate_snapshot(misordered, Path::new("misordered-meta.md"))
            .expect_err("META after SOURCE must fail");
        assert!(error.contains("expected `# META\n` as the next snapshot section"));
    }
}
