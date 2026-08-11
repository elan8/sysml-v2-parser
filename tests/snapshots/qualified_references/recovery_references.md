# SOURCE
~~~sysml
package Recovery {
    import Before::One::*;
    not valid
    dependency from Client::One to Supplier::Two;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "recovery_references.md"
    (diagnostics
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 50) (line 3) (column 5) (len 14)) (message "unexpected keyword `not` in package body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Recovery {
    import Before::One::*;
    not valid
    dependency from Client::One to Supplier::Two;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 30) (line 2) (column 12) (len 11)) (segments (segment 0 (token "Before") (name "Before") (separator none) (span (offset 30) (line 2) (column 12) (len 6))) (segment 1 (token "One") (name "One") (separator colon-colon) (span (offset 38) (line 2) (column 20) (len 3)))))
    (reference r1 (scope relative) (span (offset 80) (line 4) (column 21) (len 11)) (segments (segment 0 (token "Client") (name "Client") (separator none) (span (offset 80) (line 4) (column 21) (len 6))) (segment 1 (token "One") (name "One") (separator colon-colon) (span (offset 88) (line 4) (column 29) (len 3)))))
    (reference r2 (scope relative) (span (offset 95) (line 4) (column 36) (len 13)) (segments (segment 0 (token "Supplier") (name "Supplier") (separator none) (span (offset 95) (line 4) (column 36) (len 8))) (segment 1 (token "Two") (name "Two") (separator colon-colon) (span (offset 105) (line 4) (column 46) (len 3)))))
  )
  (root (package (name "Recovery") (body (import (target (ref r0) (shape (namespace (recursive false))))) (malformed (code "unexpected_keyword_in_scope") (found "not valid") (span (offset 50) (line 3) (column 5) (len 14))) (dependency (clients (ref r1)) (suppliers (ref r2))))))
)
~~~
