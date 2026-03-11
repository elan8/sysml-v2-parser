//! Nom-based parser for SysML v2 textual notation.
//!
//! Organized into modules:
//! - [lex]: whitespace, comments, names, qualified names, skip helpers
//! - [attribute]: attribute definition and usage
//! - [import]: import and relationship body
//! - [part]: part definition and part usage
//! - [package]: package and root namespace

mod action;
mod alias;
mod attribute;
mod expr;
mod import;
mod interface;
mod lex;
mod package;
mod part;
mod port;

use crate::ast::RootNamespace;
use crate::error::ParseError;

/// Parse full input; must consume entire input. Strips UTF-8 BOM if present.
pub fn parse_root(input: &str) -> Result<RootNamespace, ParseError> {
    let bytes = input
        .strip_prefix('\u{FEFF}')
        .map(str::as_bytes)
        .unwrap_or_else(|| input.as_bytes());
    match package::root_namespace(bytes) {
        Ok((rest, root)) => {
            if rest.is_empty() {
                Ok(root)
            } else {
                let offset = bytes.len() - rest.len();
                Err(ParseError::new("expected end of input").with_offset(offset))
            }
        }
        Err(nom::Err::Error(e)) => {
            let offset = bytes.len() - e.input.len();
            Err(ParseError::new(format!("parse error: {:?}", e.code)).with_offset(offset))
        }
        Err(nom::Err::Failure(e)) => {
            let offset = bytes.len() - e.input.len();
            Err(ParseError::new(format!("parse error: {:?}", e.code)).with_offset(offset))
        }
        Err(nom::Err::Incomplete(_)) => Err(ParseError::new("unexpected end of input")),
    }
}
