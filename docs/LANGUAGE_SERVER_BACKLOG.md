# Language Server Readiness Backlog

This backlog captures the work needed to make the parser's error handling robust enough for language-server use.

## Goal

The parser should support editor workflows such as:

- partial AST construction in the presence of syntax errors
- accurate, stable diagnostics with useful ranges
- resilient parsing that localizes damage to the smallest possible region
- stable downstream features such as outline, symbols, hover, semantic tokens, and navigation

## Priority P0

### 1. Make recovery deterministic per grammar scope

Move recovery closer to each grammar scope instead of relying primarily on top-level recovery in [`src/parser/mod.rs`](C:\Git\sysml-parser\src\parser\mod.rs).

Target scopes:

- package bodies
- part bodies
- action bodies
- state bodies
- requirement bodies
- use case bodies

Expected outcome:

- syntax errors stay localized
- later sibling elements still parse correctly
- fewer cascaded or misplaced diagnostics

### 2. Replace heuristic error filtering with grammar-aware recovery

The current `should_report_error_inside_package()` heuristic in [`src/parser/mod.rs`](C:\Git\sysml-parser\src\parser\mod.rs) is useful but fragile.

Replace it with recovery logic based on:

- explicit synchronization tokens such as `;` and `}`
- known body-element starters for each grammar scope
- recovery decisions made by the body parser itself

Expected outcome:

- fewer missed diagnostics
- fewer false suppressions
- better behavior as support for more SysML constructs is added

### 3. Introduce error nodes in the AST

Add placeholder or error variants to the AST in [`src/ast.rs`](C:\Git\sysml-parser\src\ast.rs).

Examples:

- `PackageBodyElement::Error`
- `PartUsageBodyElement::Error`
- `RequirementDefBodyElement::Error`

Expected outcome:

- invalid regions still have a stable place in the syntax tree
- editor features can remain usable around damaged code
- recovery becomes more explicit and debuggable

### 4. Make diagnostics more specific

Improve diagnostics emitted through [`src/error.rs`](C:\Git\sysml-parser\src\error.rs) and [`src/parser/mod.rs`](C:\Git\sysml-parser\src\parser\mod.rs).

Add:

- stable error codes
- more precise `expected` messages
- targeted `suggestion` text where practical

Examples:

- `expected ';' after attribute declaration`
- `expected '}' to close package body`
- `unexpected token in requirement body`

Expected outcome:

- higher-quality LSP diagnostics
- better quick-fix integration later

## Priority P1

### 5. Add grammar-aware skip helpers

Extend [`src/parser/lex.rs`](C:\Git\sysml-parser\src\parser\lex.rs) with recovery helpers that sync to the next valid construct within a specific body type.

Examples:

- `skip_to_next_package_body_element`
- `skip_to_next_part_body_element`
- `skip_to_next_requirement_body_element`
- `skip_to_next_state_body_element`

Expected outcome:

- more accurate recovery than generic line-based skipping
- less over-skipping into later constructs

### 6. Normalize recovery patterns across parser modules

Several modules already contain custom recovery loops:

- [`src/parser/package.rs`](C:\Git\sysml-parser\src\parser\package.rs)
- [`src/parser/part.rs`](C:\Git\sysml-parser\src\parser\part.rs)
- [`src/parser/action.rs`](C:\Git\sysml-parser\src\parser\action.rs)
- [`src/parser/state.rs`](C:\Git\sysml-parser\src\parser\state.rs)
- [`src/parser/requirement.rs`](C:\Git\sysml-parser\src\parser\requirement.rs)

Unify these around shared patterns or combinators so they all:

- guarantee forward progress
- report errors consistently
- sync at the right structural boundary

Expected outcome:

- more predictable parser behavior
- lower maintenance cost

### 7. Remove recovery paths that silently reshape invalid input

Some recovery is currently tolerant in ways that help parsing continue but may hide the exact structural problem.

Review and tighten cases where the parser:

- silently accepts trailing unmatched braces
- swallows malformed blocks without producing a focused diagnostic
- accepts malformed subtrees without an AST marker

Expected outcome:

- better diagnostic fidelity
- less surprising editor behavior

### 8. Add recovery-focused tests per construct

Expand tests beyond end-to-end fixtures in [`tests/parser_tests.rs`](C:\Git\sysml-parser\tests\parser_tests.rs) and validation tests.

Add dedicated cases for malformed:

- package bodies
- part bodies
- attributes
- requirements
- use cases
- state machines
- views
- constraints

Each test should check:

- error location
- error code or expected message
- partial AST remains usable
- later siblings still parse
- no infinite loop

Expected outcome:

- confidence that recovery is stable as grammar support grows

### 9. Make spans robust under recovery

Review span generation in [`src/ast.rs`](C:\Git\sysml-parser\src\ast.rs) and parser modules.

Ensure that:

- recovered nodes have meaningful ranges
- error nodes carry useful spans
- LSP ranges remain stable even for malformed input

Expected outcome:

- reliable editor highlighting and navigation around syntax errors

## Priority P2

### 10. Separate strict parsing and resilient parsing more clearly

The API in [`src/lib.rs`](C:\Git\sysml-parser\src\lib.rs) already distinguishes `parse()` from `parse_with_diagnostics()`.

Make the internal architecture reflect that more explicitly:

- strict parse path for CI and validation
- resilient parse path for language-server/editor scenarios

Expected outcome:

- less coupling between test-suite parsing behavior and editor recovery behavior

### 11. Update the error recovery documentation

Refresh [`docs/ERROR_RECOVERY.md`](C:\Git\sysml-parser\docs\ERROR_RECOVERY.md) so it matches the current implementation.

Document:

- actual recovery flow
- sync points in use
- forward-progress invariants
- known weaknesses
- intended language-server semantics

Expected outcome:

- lower onboarding cost
- easier maintenance

### 12. Evaluate richer error infrastructure

Investigate whether to adopt:

- `nom-supreme` `ErrorTree`
- custom `expect()`-style combinators
- explicit parser state for accumulated diagnostics

Expected outcome:

- richer diagnostics
- less ad hoc error plumbing
- better foundation for long-term parser evolution

## Suggested Delivery Plan

### Phase 1

- make recovery deterministic per scope
- replace heuristic filtering with grammar-aware recovery
- add grammar-aware skip helpers

### Phase 2

- add AST error nodes
- improve diagnostics
- normalize recovery patterns across modules

### Phase 3

- expand recovery tests
- harden spans
- separate strict and resilient parse paths

### Phase 4

- update documentation
- evaluate richer error infrastructure

## Definition of Done for Language-Server Use

The parser should be considered language-server-ready when:

- malformed documents still produce a useful partial AST
- diagnostics are local, stable, and specific
- later siblings remain parseable after common syntax mistakes
- no known infinite-loop or zero-progress recovery paths remain
- recovery behavior is covered by targeted tests
- error handling architecture is documented and consistent across modules
