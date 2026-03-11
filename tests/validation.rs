//! Integration tests that parse SysML validation fixture files.
//!
//! Each validation .sysml file has a corresponding test module under `validation/`
//! for easier maintenance.

#[path = "validation/parts_tree_1a.rs"]
mod parts_tree_1a;

#[path = "validation/parts_interconnection_2a.rs"]
mod parts_interconnection_2a;
