//! The type-level cost gate from `planning/shared-grammar.md`.
//!
//! Generic containers and nested member families deepen the recursive type proofs the compiler has
//! to discharge, and consumers pay for them, not this crate: the library raises its own
//! `recursion_limit`, an integration test compiles as its own crate at the *default* limit. So this
//! file deliberately sets no `recursion_limit` attribute. Raising one here would hide exactly the
//! regression it exists to catch.
//!
//! It failing to *compile* is the finding. The assertions inside are incidental.
//!
//! # What each half of the gate now holds
//!
//! The longest simple path through the AST type graph is over 120 distinct types, which is more
//! than the default limit of 128 leaves room for once the `Vec`/`RawVec`/`PhantomData` plumbing of
//! each of its fourteen nested body levels is counted. Rather than cap the grammar at whatever
//! depth that arithmetic allows, [`ParsedDocument`] carries explicit `Send`/`Sync` implementations
//! so the obligation is O(1) here, and the structural proof that discharges them lives in
//! `ast::root::send_sync_structural_proof` -- inside the library, which raises `recursion_limit`
//! for precisely this reason.
//!
//! So the two halves are:
//!
//! - *this* file proves the property a consumer actually needs, at the limit a consumer actually
//!   has, and still fails to compile if those implementations are removed or narrowed; and
//! - the library-side proof fails to compile if a field is added to [`ParsedDocument`] without an
//!   assertion, or if any type anywhere in the AST stops being `Send`/`Sync` -- an `Rc`, a
//!   `RefCell`, a raw pointer. It is a `cargo check` error, so CI catches it without a lint.
//!
//! Neither half is sufficient alone: drop the first and consumers silently regress to needing
//! their own `recursion_limit`; drop the second and the `unsafe` implementations become an
//! unchecked assertion.

use sysml_v2_parser::{parse, parse_for_editor, ParseResult, ParsedDocument};

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

/// The two public entry-point results prove the auto traits a consumer needs to move a parsed
/// document across threads, without that consumer raising its recursion limit.
///
/// This is the property `ast::root`'s explicit `Send`/`Sync` implementations exist to provide. If
/// they are removed, this file stops compiling at the default limit, which is the regression the
/// gate is here to catch; the structural obligation they stand on is proved separately in the
/// library, so nothing is taken on trust by making the obligation cheap here.
#[test]
fn public_documents_prove_auto_traits_at_the_default_recursion_limit() {
    assert_send::<ParsedDocument>();
    assert_sync::<ParsedDocument>();
    assert_send::<ParseResult>();
    assert_sync::<ParseResult>();
    // `RefUnwindSafe` is deliberately not asserted: proving it over this AST exceeds the default
    // trait-solver limit today, which is the known condition the crate root documents when it
    // raises `recursion_limit` for rustdoc. Asserting it here would only pin an existing debt at
    // the wrong layer -- the gate is that `Send`/`Sync` stay reachable without a consumer
    // raising anything.
}

/// Serde derivation over the whole envelope compiles downstream, and a document actually makes the
/// round trip. The metadata seam changed the serialized shape of three types
/// (`MetadataAnnotation`, `MetadataDeclaredName`, `MetadataKeywordUsage`) and removed one
/// (`ConnectBody`), so a fixture exercising every one of them is what this proves.
#[cfg(feature = "serde")]
#[test]
fn the_serialized_envelope_round_trips_downstream() {
    let source = "package P {\n\
                  metadata def Tag;\n\
                  #Tag part def Prefixed {\n\
                  @Tag;\n\
                  @named : Tag;\n\
                  @spelled typed by Tag about Prefixed;\n\
                  #Tag { doc /* body */ }\n\
                  attribute a : Anything;\n\
                  attribute b : Anything;\n\
                  bind a = b { doc /* bind body */ }\n\
                  }\n\
                  }\n";
    let document = parse(source).expect("fixture parses");
    let encoded = serde_json::to_string(&document).expect("serialize");
    let decoded: ParsedDocument = serde_json::from_str(&encoded).expect("deserialize");
    assert_eq!(decoded, document);
}

fn nested_source(depth: usize) -> String {
    let mut source = String::from("package P {\n");
    for index in 0..depth {
        source.push_str(&format!("part def N{index} {{\n#Tag;\n@Tag;\n"));
    }
    for _ in 0..depth {
        source.push_str("}\n");
    }
    source.push_str("}\n");
    source
}

/// The narrow debug-stack budget for the largest currently supported action-body dispatch.
///
/// This is a resource contract, not an AST-shape assertion: a nested `for` / `perform action`
/// chain used to retain several wide `nom::Choice` frames and abort a 2 MiB worker stack in debug
/// builds. The public strict entry point must return normally on the same budget that a Rayon-like
/// worker commonly provides.
#[test]
fn nested_action_body_dispatch_fits_a_two_mib_worker_stack() {
    const PROBE: &str = "package P { action def A { for x in seq { perform action doStuff : DoStuff { for y in items { } } } } }";

    let worker = std::thread::Builder::new()
        .name("gap68-2mib".to_owned())
        .stack_size(2 * 1024 * 1024)
        .spawn(|| parse(PROBE))
        .expect("spawn two-MiB parser worker");
    let document = worker
        .join()
        .expect("two-MiB parser worker must not panic")
        .expect("Gap 68 probe must parse cleanly");
    assert_eq!(document.root.elements.len(), 1, "expected one package");
}

/// Parsing, walking and dropping a document nested to the supported depth is stack-safe at the
/// default limit, and exceeding that depth is a diagnostic rather than an overflow.
#[test]
fn a_deeply_nested_document_parses_and_drops_without_overflowing() {
    // One under the parser's documented `nesting_too_deep` limit, so this exercises the deepest
    // shape the parser claims to accept rather than the recovery path below it.
    let result: ParseResult = parse_for_editor(&nested_source(30));
    assert!(
        result.errors.is_empty(),
        "nested fixture should parse clean: {:?}",
        result.errors
    );
    drop(result);

    let too_deep: ParseResult = parse_for_editor(&nested_source(64));
    assert!(
        too_deep
            .errors
            .iter()
            .any(|error| error.code.as_deref() == Some("nesting_too_deep")),
        "past the limit the parser reports, rather than overflowing: {:?}",
        too_deep.errors
    );
    drop(too_deep);
}
