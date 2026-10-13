# META
~~~sexpr
(snapshot (type semantic) (description "Wildcard import statement"))
~~~
# SOURCE
~~~sysml
private import ScalarValues::*;
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 15) (line 1) (column 16) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 15) (line 1) (column 16) (len 12)))))
  )
  (root (import (target (span (span (offset 15) (line 1) (column 16) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 27) (line 1) (column 28) (len 3))) (separator (span (offset 27) (line 1) (column 28) (len 2))) (marker (span (offset 29) (line 1) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))))
)
~~~
