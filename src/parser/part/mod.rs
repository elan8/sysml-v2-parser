//! Part definition and part usage parsing.
#![allow(dead_code, unused_imports)]

mod body;
mod def;
mod prelude;
mod usage;

pub(crate) use body::{connection_usage_member, part_def_body};
pub(crate) use def::{part_def, part_def_or_usage};
pub(crate) use usage::{bind_, part_ref_usage, part_usage, perform_action_decl};

use crate::ast::{Node, PartDef, PartUsage};

/// Result of parsing either a part definition or part usage (used for package body to avoid part_def consuming "part" before part_usage can run).
#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum PartDefOrUsage {
    Def(Node<PartDef>),
    Usage(Node<PartUsage>),
}
