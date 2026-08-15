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
  )
  (root (package (name "P") (body brace (part-usage))))
)
~~~
