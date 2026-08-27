//! Nom-based parser for SysML v2 textual notation.
//!
//! Organized into modules (all private except [`diagnostic_catalog`]; listed here for
//! contributors reading source, not as public doc links):
//! - `lex`: whitespace, comments, names, qualified names, skip helpers
//! - `diagnostics`: nom error mapping, diagnostic classification, deduplication
//! - `recovery`: recovery error nodes for structured body parsing
//! - `collect_errors`: aggregate diagnostics from AST recovery nodes
//! - `parse`: `parse_root` and `parse_with_diagnostics` entry points
//! - `attribute`: attribute definition and usage
//! - `import`: import and relationship body
//! - `part`: part definition and part usage
//! - `package`: package and root namespace

mod action;
mod alias;
mod allocation;
mod attribute;
mod bnf_surface;
mod body;
mod case;
mod collect_errors;
mod connection;
mod connector;
mod constraint;
mod definition_header;
mod definition_prefix;
mod delimiters;
mod dependency;
pub mod diagnostic_catalog;
mod diagnostics;
mod enumeration;
mod expr;
mod feature_prefix;
mod feature_value;
mod flow;
mod grammar_scope;
mod import;
mod individual;
mod interface;
mod item;
pub(crate) mod lex;
mod metadata;
mod metadata_annotation;
mod metadata_body;
mod occurrence;
mod occurrence_body;
mod occurrence_prefix;
pub(crate) mod package;
mod parse;
mod part;
mod payload;
mod port;
mod recovery;
mod requirement;
mod span;
mod specialization;
mod stack;
mod state;
mod usage;
mod usecase;
mod view;

pub(crate) use feature_value::feature_value_part;
pub(crate) use span::{advance, node_from_to, span_from_to, with_span, Input};

pub use parse::{
    parse_root, parse_root_owned, parse_with_diagnostics, parse_with_diagnostics_owned, ParseResult,
};

pub(crate) use recovery::{build_recovery_error_node, build_recovery_error_node_from_span};
