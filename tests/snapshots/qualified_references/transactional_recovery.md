# META
~~~sexpr
(snapshot (type recovery) (description "Verifies speculative references from malformed imports, aliases, and exposes roll back while later valid sibling targets remain dense and visible."))
~~~
# SOURCE
~~~sysml
package TransactionalRecovery {
    private import Ghost::ImportTarget unexpected;
    private import Live::ImportTarget;

    alias broken for Ghost::AliasTarget unexpected;
    alias live for Live::AliasTarget;

    view recoveredView {
        expose Ghost::ExposeTarget unexpected;
        expose Live::ExposeTarget;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "transactional_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 36) (line 2) (column 5) (len 46)) (message "unexpected token in package body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 36) (line 2) (column 5) (len 46)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package TransactionalRecovery {
    private import Ghost::ImportTarget unexpected;
    private import Live::ImportTarget;
    alias broken for Ghost::AliasTarget unexpected;
    alias live for Live::AliasTarget;
    view recoveredView {
        expose Ghost::ExposeTarget unexpected;
        expose Live::ExposeTarget;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 102) (line 3) (column 20) (len 18)) (segments (segment 0 (token "Live") (name "Live") (separator none) (span (offset 102) (line 3) (column 20) (len 4))) (segment 1 (token "ImportTarget") (name "ImportTarget") (separator colon-colon) (span (offset 108) (line 3) (column 26) (len 12)))))
    (reference r1 (scope relative) (span (offset 194) (line 6) (column 20) (len 17)) (segments (segment 0 (token "Live") (name "Live") (separator none) (span (offset 194) (line 6) (column 20) (len 4))) (segment 1 (token "AliasTarget") (name "AliasTarget") (separator colon-colon) (span (offset 200) (line 6) (column 26) (len 11)))))
    (reference r2 (scope relative) (span (offset 301) (line 10) (column 16) (len 18)) (segments (segment 0 (token "Live") (name "Live") (separator none) (span (offset 301) (line 10) (column 16) (len 4))) (segment 1 (token "ExposeTarget") (name "ExposeTarget") (separator colon-colon) (span (offset 307) (line 10) (column 22) (len 12)))))
  )
  (root (package (visibility none) (name "TransactionalRecovery") (body brace (malformed (code "recovered_package_body_element") (found "private import Ghost::ImportTarget unexpected;") (span (offset 36) (line 2) (column 5) (len 46))) (import (visibility private) (target (span (span (offset 102) (line 3) (column 20) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none)))) (body semicolon)) (malformed (code "recovered_package_body_element") (found "alias broken for Ghost::AliasTarget unexpected;") (span (offset 127) (line 5) (column 5) (len 47))) (alias (name "live") (target (ref r1)) (body semicolon)) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "recoveredView") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (malformed (code "recovered_view_body_element") (found "expose Ghost::ExposeTarget unexpected;") (span (offset 247) (line 9) (column 9) (len 47))) (expose (target (span (span (offset 301) (line 10) (column 16) (len 18))) (all none) (ref r2) (shape (membership (recursive-suffix none)))) (body semicolon)))))))
)
~~~
