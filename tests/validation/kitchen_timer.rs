//! Validation test for `tests/fixtures/KitchenTimer.sysml`.

use std::path::Path;
use sysml_v2_parser::{parse, parse_with_diagnostics, RootElement};

fn kitchen_timer_fixture_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("KitchenTimer.sysml")
}

#[test]
fn test_parse_kitchen_timer() {
    super::init_log();
    let path = kitchen_timer_fixture_path();
    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read fixture {}: {}", path.display(), e));
    let input = input.replace("\r\n", "\n").replace('\r', "\n");

    // Two root members: the file's leading `/* ... */` header block, which is the `Comment`
    // production's keyword-less spelling and therefore a member rather than trivia, and the
    // package itself.
    let strict = parse(&input).expect_err(
        "pinned SysML/Pilot grammar rejects the fixture's Pilot-only `end port` interface members",
    );
    assert_eq!(
        strict.code.as_deref(),
        Some("recovered_interface_def_body_element")
    );
    assert_eq!(strict.line, Some(65));
    assert_eq!(
        strict.found.as_deref(),
        Some("end port source : BuzzerCommandPort;")
    );

    let result = parse_with_diagnostics(&input);
    assert_eq!(
        result.document.root.elements.len(),
        2,
        "fixture should produce its header comment and one root package"
    );
    match &result.document.root.elements[0].value {
        RootElement::Member(member) => match &member.value {
            sysml_v2_parser::ast::PackageBodyElement::Annotating(
                sysml_v2_parser::ast::AnnotatingMember::Comment(comment),
            ) => {
                assert!(comment.value.keyword_span.is_none());
                assert!(comment.value.text.contains("Kitchen Timer"));
            }
            other => panic!("expected the header comment, got {other:?}"),
        },
        other => panic!("expected the header comment, got {other:?}"),
    }
    match &result.document.root.elements[1].value {
        RootElement::Package(pkg) => {
            assert_eq!(pkg.identification.simple_name(), Some("KitchenTimer"));
        }
        other => panic!("expected root element to be package, got {:?}", other),
    }

    let recovered_end_ports: Vec<_> = result
        .errors
        .iter()
        .filter(|error| {
            error.code.as_deref() == Some("recovered_interface_def_body_element")
                && error
                    .found
                    .as_deref()
                    .is_some_and(|found| found.starts_with("end port "))
        })
        .collect();
    assert_eq!(
        recovered_end_ports.len(),
        1,
        "the first unsupported `end port` stays pinned-policy recovery, got {:?}",
        result.errors
    );
    let cascade = result
        .errors
        .iter()
        .find(|error| error.code.as_deref() == Some("recovery_cascade_suppressed"))
        .expect("the remaining same-body end-port diagnostics should be intentionally coalesced");
    assert_eq!(cascade.line, Some(65));
    assert_eq!(
        cascade.found.as_deref(),
        Some("end port source : BuzzerCommandPort;")
    );
    assert!(
        cascade
            .message
            .contains("suppressed 11 cascading recovered diagnostics"),
        "coalesced recovery should document the exact suppressed tail: {cascade:?}"
    );
}
