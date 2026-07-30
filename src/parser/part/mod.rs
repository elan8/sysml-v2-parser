//! Part definition and part usage parsing.

mod body;
mod def;
mod prelude;
mod usage;

pub(crate) use body::connection_usage_member;
pub(crate) use def::part_def_or_usage;
// `part_def`/`part_def_body` are reached in production code through `part_def_or_usage` and
// `body::part_def_body`'s own callers, not through this re-export -- it exists only so
// `package.rs`'s `#[cfg(test)]` module can call `crate::parser::part::part_def_body`/`part_def`
// directly, which is why a plain (non-test) `cargo build` sees the re-export itself as unused.
pub(crate) use usage::{
    allocate_, bind_, connect_, exhibit_state_as_state_usage, interface_usage, part_ref_usage,
    part_usage, perform_action_decl,
};
#[allow(unused_imports)]
pub(crate) use {body::part_def_body, def::part_def};

use crate::ast::{Node, PartDef, PartUsage};

/// Result of parsing either a part definition or part usage (used for package body to avoid part_def consuming "part" before part_usage can run).
#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum PartDefOrUsage {
    Def(Node<PartDef>),
    Usage(Node<PartUsage>),
}
