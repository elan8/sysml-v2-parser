//! Stack headroom for the recursive-descent body grammar.
//!
//! Only one construct recurses without bound in the parser: a brace body whose members are
//! themselves declarations with brace bodies. Expressions use an explicit heap stack, and
//! [`MAX_SYNTAX_NESTING`](super::parse::MAX_SYNTAX_NESTING) caps how deep body nesting may go
//! before parsing starts. Headroom is therefore acquired where the recursion happens -- at each
//! brace-body entry and again before each member, only when the remaining stack is genuinely
//! short -- rather than by mapping a worst-case segment at the parse entry point on behalf of
//! documents that never nest.

/// Stack one brace body may consume between headroom checks.
///
/// One level costs a few kilobytes in release builds and roughly two orders of magnitude more in
/// unoptimized builds, where every combinator frame survives. That cost is not uniform: an
/// `individual part` header (`individual part x : T :> a, b`) plus a nested `snapshot` and a
/// `doc` member each cost more than a bare `part`, and together they can spend more than this
/// margin inside a single level. A check only at body entry cannot
/// see that remaining work -- the header is already on the stack, the member is still ahead -- so
/// callers also probe before each member. 1.75 MiB covers the heavier of those pieces in a debug
/// build. A fresh 2 MiB thread still has about 2.0 MiB left at the first check, so a flat or
/// shallow document does not allocate.
const NESTED_BODY_RED_ZONE: usize = 1792 * 1024;

/// Segment size acquired when a nesting level cannot fit in [`NESTED_BODY_RED_ZONE`].
///
/// Sized so a single segment carries the remaining levels of a `MAX_SYNTAX_NESTING`-deep document
/// even in a debug build, making growth a once-per-deep-document event rather than a per-level one.
const NESTED_BODY_GROWTH: usize = 16 * 1024 * 1024;

/// Run one brace-body entry or one body member, acquiring a new stack segment only if that work
/// would not otherwise fit in [`NESTED_BODY_RED_ZONE`].
///
/// Callers invoke this at body entry and again immediately before each member. The second probe
/// is what keeps an expensive header and an expensive member (a `doc` comment chain) from sharing
/// one unchecked gap.
pub(crate) fn with_nested_body_stack<R>(f: impl FnOnce() -> R) -> R {
    stacker::maybe_grow(NESTED_BODY_RED_ZONE, NESTED_BODY_GROWTH, f)
}

#[cfg(test)]
mod tests {
    use crate::parse;

    /// The nested `individual part` / `snapshot` / `doc` shape from
    /// `Vehicle Example/VehicleIndividuals.sysml`, on an explicit 2 MiB thread. A debug build
    /// aborts that thread when headroom is checked only at body entry. The spawn is required:
    /// `cargo test`'s own thread is not always 2 MiB.
    #[test]
    fn individual_redefinition_with_doc_body_does_not_overflow() {
        let src = r#"
package VehicleIndividuals {
    package IndividualConfigurations {
        individual part vehicle1_C2: Vehicle1 :> vehicle_C2, vehicle1 {
            doc
            /* lifetime */
            snapshot vehicle1_C2_t0 :> vehicle1_t0 {
                doc
                /* snapshot */
                individual axleAssembly1_t0: AxleAssembly1 :>> frontAxleAssembly {
                    doc
                    /* axle */
                    individual leftFrontWheel_t0: Wheel1 :>> leftFrontWheel {
                        doc
                        /* wheel */
                    }
                }
            }
        }
    }
}
"#;
        let src = src.to_owned();
        std::thread::Builder::new()
            .stack_size(2 * 1024 * 1024)
            .spawn(move || {
                parse(&src).unwrap_or_else(|e| panic!("parse repro: {e}"));
            })
            .unwrap()
            .join()
            .unwrap();
    }

    /// `occurrence_usage_body_brace` (`src/parser/occurrence_body.rs`, the brace body of a bare
    /// `individual`/`portion` occurrence usage) was the one nesting loop the fix above missed: it
    /// had no `with_nested_body_stack` call at all, neither at entry nor per member, unlike
    /// `parse_structured_brace_members_inner`/`package_body_brace_inner`. This repro -- 30 levels
    /// of nested bare `individual x : T { doc /* ... */ ... }` (well within `MAX_SYNTAX_NESTING`,
    /// no `part`/`snapshot` at any level) -- overflows an explicit 8 MiB thread's stack in a debug
    /// build without that loop's own entry and per-member probes.
    #[test]
    fn nested_bare_individual_usages_with_doc_bodies_do_not_overflow() {
        let depth = 30;
        let mut src = String::from("package Demo {\n");
        for level in 0..depth {
            src.push_str(&format!(
                "individual x{level} : T {{\n\tdoc /* level {level} */\n"
            ));
        }
        for _ in 0..depth {
            src.push_str("}\n");
        }
        src.push('}');
        std::thread::Builder::new()
            .stack_size(8 * 1024 * 1024)
            .spawn(move || {
                parse(&src).unwrap_or_else(|e| panic!("parse repro: {e}"));
            })
            .unwrap()
            .join()
            .unwrap();
    }
}
