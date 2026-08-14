# META
~~~sexpr
(snapshot (type semantic) (description "Wildcard import statement"))
~~~
# SOURCE
~~~sysml
import ScalarValues::*;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "import_wildcard.md"
    (diagnostics
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
  (references
    (reference r0 (scope relative) (span (offset 7) (line 1) (column 8) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 7) (line 1) (column 8) (len 12)))))
  )
  (root (import (target (span (span (offset 7) (line 1) (column 8) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 19) (line 1) (column 20) (len 3))) (separator (span (offset 19) (line 1) (column 20) (len 2))) (marker (span (offset 21) (line 1) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))))
)
~~~
