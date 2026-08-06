//! View / calc / constraint emission. Extended as roundtrip coverage grows.

#![allow(dead_code)]

use super::writer::EmitWriter;
use super::EmitError;

pub(crate) fn unsupported(
    w: &EmitWriter<'_>,
    path: &str,
    construct: &str,
) -> Result<(), EmitError> {
    w.unsupported(path, construct)
}
