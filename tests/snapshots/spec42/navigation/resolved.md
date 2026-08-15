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
  )
  (root (package (name "P") (body brace (part-def (name "Engine") (body semicolon)) (part-usage))))
)
~~~
