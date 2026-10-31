# META
~~~sexpr
(snapshot (type recovery) (description "Issue 200 case 3: an incomplete extra typing target must recover with an error, preserving the malformed slice and later complete typing lists."))
~~~
# SOURCE
~~~sysml
package TypingRecovery {
    item broken : A, ;
    item retained : A, B;
    case broken : A, ;
    case retained : A, B;
    view report {
        render rendering broken : A, ;
        render rendering retained : A, B;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "usage_typing_lists_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 29) (line 2) (column 5) (len 18)) (message "unexpected token in package body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 29) (line 2) (column 5) (len 18)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 68) (line 3) (column 21) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 68) (line 3) (column 21) (len 1)))))
    (reference r1 (scope relative) (span (offset 71) (line 3) (column 24) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 71) (line 3) (column 24) (len 1)))))
    (reference r2 (scope relative) (span (offset 117) (line 5) (column 21) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 117) (line 5) (column 21) (len 1)))))
    (reference r3 (scope relative) (span (offset 120) (line 5) (column 24) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 120) (line 5) (column 24) (len 1)))))
    (reference r4 (scope relative) (span (offset 216) (line 8) (column 37) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 216) (line 8) (column 37) (len 1)))))
    (reference r5 (scope relative) (span (offset 219) (line 8) (column 40) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 219) (line 8) (column 40) (len 1)))))
  )
  (root (package (visibility none) (name "TypingRecovery") (body brace (malformed (code "recovered_package_body_element") (found "item broken : A, ;") (span (offset 29) (line 2) (column 5) (len 18))) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "retained") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0) (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_package_body_element") (found "case broken : A, ;") (span (offset 78) (line 4) (column 5) (len 18))) (case-usage (declaration (name "retained") (short-name none) (abstract false) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r2) (ref r3)))) (subsets none)) (body semicolon)) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "report") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (malformed (code "recovered_view_body_element") (found "render rendering broken : A, ;") (span (offset 149) (line 7) (column 9) (len 39))) (view-rendering (form inline "retained") (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r4) (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))))
)
~~~
