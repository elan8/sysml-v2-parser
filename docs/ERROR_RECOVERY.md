# Error Recovery in the SysML Parser

This document describes the current error recovery implementation and outlines approaches for improving it in the future.

## Current Implementation

### Overview

The parser uses a **top-level recovery loop** around the standard nom parser. When `parse_with_diagnostics()` is used (e.g. for language servers), it:

1. Parses root elements (`package` or `namespace`) one at a time
2. On success: appends the element to the partial AST and continues
3. On failure: records the error (conditionally), skips to the next sync point, and retries
4. Returns a partial AST plus a list of diagnostics

### Key Files

| File | Purpose |
|------|---------|
| `src/parser/mod.rs` | `parse_with_diagnostics()`, recovery loop, `should_report_error_inside_package()` |
| `src/parser/lex.rs` | `skip_to_next_sync_point()`, `skip_to_next_root_element()` |
| `src/parser/package.rs` | `cut()` on closing `}` in package bodies (error propagation) |

### Sync Points

- **`skip_to_next_sync_point`**: Skips to the next line (consumes until newline, then ws/comments). Used for line-by-line recovery.
- **`skip_to_next_root_element`**: Skips to the next line starting with `package ` or `namespace `, or EOF. Currently unused; was previously used when recovery jumped to the next package.

### Error Filtering

When a failure occurs **inside** a package (brace depth > 0), we only report errors that look like invalid top-level elements. The heuristic `should_report_error_inside_package()`:

- **Reports** when `found` contains ` {}` (e.g. `test {}`, `test2 {}`, `xyz {}`)
- **Skips** when `found` starts with valid keywords (`part `, `port `, `state `, `transition `, etc.)
- **Skips** when `found` looks like nested content (expressions with ` >= `, ` then `, ` first `, etc.)

This avoids a cascade of ~75 errors when the parser fails inside a package and tries to parse each subsequent line as a root element.

### Cut for Error Propagation

In `package_body`, the closing `}` parser is wrapped in `cut()`. Without this, `many0` would swallow the real error when parsing fails inside a package and report a misleading "expected `}`" or similar.

---

## Limitations

1. **Fragile heuristic**: `should_report_error_inside_package` uses a long hardcoded list that can break with new SysML constructs
2. **Recovery at wrong level**: We parse at root level but recover inside packages; the mismatch requires heuristic filtering
3. **Coarse sync points**: "Next line" is simple but not grammar-aware; skipping to the next `part `, `port `, `}`, etc. would be more precise
4. **No recovery inside nested structures**: State machines, constraint bodies, etc. are not recovered independently

---

## Research and Best Practices

### 1. Academic Approach (Medeiros & Mascarenhas 2018)

**Paper**: ["Syntax error recovery in parsing expression grammars"](https://dl.acm.org/doi/10.1145/3167132.3167261)

**Blog**: [Error recovery with parser combinators (using nom)](https://eyalkalderon.com/blog/nom-error-recovery/) by Eyal Kalderon

**Core ideas**:

- Parsing **never fails** — always produce a tree
- Use **synchronization tokens** (`)`, `}`, `;`) to skip ahead when needed
- Recovery expressions annotated with **labels** emit errors but allow parsing to continue
- Output is `(T, Vec<Error>)`, not `Result<T, Error>`

**Example** — `expect()` combinator:

```rust
fn expect<'a, F, T>(parser: F, error_msg: &str) -> impl Fn(Input) -> IResult<Option<T>>
where F: Fn(Input) -> IResult<T>
{
    move |input| match parser(input) {
        Ok((rest, out)) => Ok((rest, Some(out))),
        Err(Err::Error(e)) | Err(Err::Failure(e)) => {
            input.extra.report_error(Error(...));  // Push to external Vec
            Ok((input, None))  // Parsing continues!
        }
        Err(e) => Err(e),
    }
}
```

Errors are collected in shared state (e.g. `RefCell<Vec<Error>>` in `LocatedSpan::extra`).

### 2. Cut–Context Pattern

**Blog**: [The cut-context pattern with nom](http://blog.vorona.ca/the-cut-context-pattern-with-nom.html)

Use `cut(context("message", parser))` after commitment points to prevent misleading errors and unnecessary backtracking:

```rust
let (input, _) = tag("=")(input)?;
let (input, operator) = cut(context("Expecting operator", parse_operator))(input)?;
```

### 3. nom-supreme

**Docs**: [nom-supreme](https://docs.rs/nom-supreme/latest/nom_supreme/)

- **ErrorTree**: Tree-shaped error representation (not just a stack)
- **parse_separated_terminated**: Combinators for better error handling with separators/terminators

### 4. matklad / rust-analyzer

**Tutorial**: [Resilient LL Parsing Tutorial](https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html)

- Hand-written recursive descent with explicit recovery
- "Don't crash on first error" — localize errors
- Error nodes in the syntax tree for invalid regions

---

## Future Improvements

| Approach | Effort | Benefit |
|----------|--------|---------|
| **Grammar-level recovery** | High | Replace top-level loop with `expect()`-style combinators in package body parsing; errors collected via shared state. |
| **Grammar-aware sync points** | Medium | Instead of "next line", skip to next `part `, `port `, `}`, etc. so we don't land in the middle of nested content. |
| **nom-supreme ErrorTree** | Medium | Richer error structure and better diagnostics. |
| **Refine heuristics** | Low | Narrow `should_report_error_inside_package` to "identifier + ` {}`" pattern; avoid keyword lists where possible. |

### Recommended Next Steps

1. **Short term**: Implement grammar-aware sync points — e.g. `skip_to_next_package_body_element()` that skips until the next line starting with `part `, `port `, `attribute `, `}`, etc. This would reduce reliance on `should_report_error_inside_package`.

2. **Medium term**: Introduce `expect()`-style combinators at key points in the package body (e.g. when parsing each element in `many0`). This would integrate recovery into the grammar rather than relying on an outer loop.

3. **Long term**: Consider nom-supreme ErrorTree for richer error representation, especially if the language server needs more diagnostic detail.

---

## Test Fixtures

- **SurveillanceDrone-error.sysml**: Contains `test {}` (line 333) and `test2 {}` (line 364); expects exactly 2 errors
- **SurveillanceDrone-errors.sysml**: Multiple packages with invalid statements (`test {}`, `xyz {}`, `badstmt {}`); expects 3 errors

---

## References

- [ACM DL: Syntax error recovery in parsing expression grammars (2018)](https://dl.acm.org/doi/10.1145/3167132.3167261)
- [LPegLabel](https://github.com/sqmedeiros/lpeglabel) — reference implementation
- [Eyal Kalderon: fault-tolerant nom parser example](https://github.com/ebkalderon/example-fault-tolerant-parser)
- [matklad: Modern Parser Generator](https://matklad.github.io/2018/06/06/modern-parser-generator.html)
- [nom error management](https://github.com/Geal/nom/blob/master/doc/error_management.md)
