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
  (root (package (name "P") (body brace (part-def (name "Engine") (body semicolon)) (part-usage (declaration-name "engine") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)))))
)
~~~
