# META
~~~sexpr
(snapshot (type semantic) (description "Unresolved navigation remains an explicit outcome"))
~~~
# SOURCE
~~~sysml
package P {
    part engine : MissingEngine;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "unresolved.md"
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
    (reference r0 (scope relative) (span (offset 30) (line 2) (column 19) (len 13)) (segments (segment 0 (token "MissingEngine") (name "MissingEngine") (separator none) (span (offset 30) (line 2) (column 19) (len 13)))))
  )
  (root (package (name "P") (body brace (part-usage (declaration-name "engine") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)))))
)
~~~
