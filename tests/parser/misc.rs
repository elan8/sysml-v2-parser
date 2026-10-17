//! Parser tests: misc

use super::common::*;
use sysml_v2_parser::{parse, parse_with_diagnostics};

#[test]
fn test_parse_primitive_data_types_validation_fixture() {
    let Some(input) = primitive_data_types_fixture() else {
        return;
    };
    let result = parse(&input);
    assert!(
        result.is_ok(),
        "fixture should parse cleanly; diagnostics: {:?}",
        parse_with_diagnostics(&input).errors
    );
}
