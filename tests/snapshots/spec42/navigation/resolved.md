# META
~~~sexpr
(snapshot (type semantic) (description "Resolved navigation preserves target identity and range"))
~~~
# SOURCE
~~~sysml
package P {
    part def Engine;
    part engine : Engine;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "resolved.md"
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
    (reference r0 (scope relative) (span (offset 51) (line 3) (column 19) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 51) (line 3) (column 19) (len 6)))))
  )
  (root (package (name "P") (body brace (part-def (name "Engine") (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))))
)
~~~
