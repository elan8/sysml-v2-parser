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
  (root (package (name "P") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))))
)
~~~
