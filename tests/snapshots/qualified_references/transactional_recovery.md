# META
~~~sexpr
(snapshot (type recovery) (description "Verifies speculative references from malformed imports, aliases, and exposes roll back while later valid sibling targets remain dense and visible."))
~~~
# SOURCE
~~~sysml
package TransactionalRecovery {
    import Ghost::ImportTarget unexpected;
    import Live::ImportTarget;

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
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 36) (line 2) (column 5) (len 43)) (message "unexpected token in package body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 36) (line 2) (column 5) (len 43)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package TransactionalRecovery {
    import Ghost::ImportTarget unexpected;
    import Live::ImportTarget;
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
  (references
    (reference r0 (scope relative) (span (offset 86) (line 3) (column 12) (len 18)) (segments (segment 0 (token "Live") (name "Live") (separator none) (span (offset 86) (line 3) (column 12) (len 4))) (segment 1 (token "ImportTarget") (name "ImportTarget") (separator colon-colon) (span (offset 92) (line 3) (column 18) (len 12)))))
    (reference r1 (scope relative) (span (offset 178) (line 6) (column 20) (len 17)) (segments (segment 0 (token "Live") (name "Live") (separator none) (span (offset 178) (line 6) (column 20) (len 4))) (segment 1 (token "AliasTarget") (name "AliasTarget") (separator colon-colon) (span (offset 184) (line 6) (column 26) (len 11)))))
    (reference r2 (scope relative) (span (offset 285) (line 10) (column 16) (len 18)) (segments (segment 0 (token "Live") (name "Live") (separator none) (span (offset 285) (line 10) (column 16) (len 4))) (segment 1 (token "ExposeTarget") (name "ExposeTarget") (separator colon-colon) (span (offset 291) (line 10) (column 22) (len 12)))))
  )
  (root (package (name "TransactionalRecovery") (body brace (malformed (code "recovered_package_body_element") (found "import Ghost::ImportTarget unexpected;") (span (offset 36) (line 2) (column 5) (len 43))) (import (target (span (span (offset 86) (line 3) (column 12) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (malformed (code "recovered_package_body_element") (found "alias broken for Ghost::AliasTarget unexpected;") (span (offset 111) (line 5) (column 5) (len 52))) (alias (name "live") (target (ref r1)) (body semicolon)) (view (name "recoveredView") (type none) (body brace (malformed (code "recovered_view_body_element") (found "expose Ghost::ExposeTarget unexpected;") (span (offset 231) (line 9) (column 9) (len 47))) (expose (target (span (span (offset 285) (line 10) (column 16) (len 18))) (all none) (ref r2) (shape (membership (recursive-suffix none))))))))))
)
~~~
