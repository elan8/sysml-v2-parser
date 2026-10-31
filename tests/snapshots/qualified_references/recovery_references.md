# META
~~~sexpr
(snapshot (type recovery) (description "Verifies package recovery retains valid qualified references before and after a malformed sibling and formats the recovered tree idempotently."))
~~~
# SOURCE
~~~sysml
package Recovery {
    private import Before::One::*;
    not valid
    dependency from Client::One to Supplier::Two;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "recovery_references.md"
    (diagnostics
      (diagnostic (code "missing_semicolon") (severity error) (category parseerror) (span (offset 58) (line 3) (column 5) (len 14)) (message "missing semicolon before next declaration"))
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
    (reference r0 (scope relative) (span (offset 38) (line 2) (column 20) (len 11)) (segments (segment 0 (token "Before") (name "Before") (separator none) (span (offset 38) (line 2) (column 20) (len 6))) (segment 1 (token "One") (name "One") (separator colon-colon) (span (offset 46) (line 2) (column 28) (len 3)))))
    (reference r1 (scope relative) (span (offset 88) (line 4) (column 21) (len 11)) (segments (segment 0 (token "Client") (name "Client") (separator none) (span (offset 88) (line 4) (column 21) (len 6))) (segment 1 (token "One") (name "One") (separator colon-colon) (span (offset 96) (line 4) (column 29) (len 3)))))
    (reference r2 (scope relative) (span (offset 103) (line 4) (column 36) (len 13)) (segments (segment 0 (token "Supplier") (name "Supplier") (separator none) (span (offset 103) (line 4) (column 36) (len 8))) (segment 1 (token "Two") (name "Two") (separator colon-colon) (span (offset 113) (line 4) (column 46) (len 3)))))
  )
  (root (package (visibility none) (name "Recovery") (body brace (import (target (span (span (offset 38) (line 2) (column 20) (len 14))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 49) (line 2) (column 31) (len 3))) (separator (span (offset 49) (line 2) (column 31) (len 2))) (marker (span (offset 51) (line 2) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (malformed (code "missing_semicolon") (found "not valid") (span (offset 58) (line 3) (column 5) (len 14))) (dependency (clients (ref r1)) (suppliers (ref r2)) (body semicolon)))))
)
~~~
