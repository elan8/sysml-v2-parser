//! Shared authored snapshot metadata; language selection is explicit, never inferred from source.

pub(crate) struct AuthoredSections {
    pub(crate) meta: String,
    pub(crate) source: String,
    pub(crate) name_language: sysml_v2_parser::ast::NameLanguage,
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

pub(crate) fn validate_meta(text: &str) -> Result<sysml_v2_parser::ast::NameLanguage, String> {
    let mut tokens = tokenize_meta(text)?;
    let mut name_language = sysml_v2_parser::ast::NameLanguage::Sysml;
    if tokens.len() >= 5 {
        let field_start = tokens.len() - 5;
        if let [MetaToken::Open, MetaToken::Symbol(field), MetaToken::Symbol(value), MetaToken::Close, MetaToken::Close] =
            &tokens[field_start..]
        {
            if field == "name-language" {
                name_language = match value.as_str() {
                    "sysml" => sysml_v2_parser::ast::NameLanguage::Sysml,
                    "kerml" => sysml_v2_parser::ast::NameLanguage::Kerml,
                    unknown => return Err(format!("unknown META name language `{unknown}`")),
                };
                tokens.drain(field_start..field_start + 4);
            }
        }
    }
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
    Ok(name_language)
}

pub(crate) fn authored_sections(text: &str) -> Result<AuthoredSections, String> {
    let (meta, remainder) = leading_fenced_section(text, "META", "sexpr")?;
    let name_language = validate_meta(&meta)?;
    let (source, _derived_sections) = leading_fenced_section(remainder, "SOURCE", "sysml")?;
    Ok(AuthoredSections {
        meta,
        source,
        name_language,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn language_is_explicit_validated_and_defaults_to_sysml() {
        use super::validate_meta;
        use sysml_v2_parser::ast::NameLanguage;
        assert_eq!(
            validate_meta("(snapshot (type semantic) (description \"Source.\"))").unwrap(),
            NameLanguage::Sysml
        );
        assert_eq!(
            validate_meta(
                "(snapshot (type semantic) (description \"Source.\") (name-language kerml))"
            )
            .unwrap(),
            NameLanguage::Kerml
        );
        assert!(validate_meta(
            "(snapshot (type semantic) (description \"Source.\") (name-language guess))"
        )
        .is_err());
        assert!(validate_meta("(snapshot (type semantic) (description \"Source.\") (name-language kerml) (name-language sysml))").is_err());
    }
}
