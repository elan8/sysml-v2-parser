# META
~~~sexpr
(snapshot (type semantic) (description "Diagnostics preserve canonical ordering for multiple unresolved type references"))
~~~
# SOURCE
~~~sysml
package P {
    part bad_first : MissingFirst;
    part bad_second : MissingSecond;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "diagnostic_canonical_order.md"
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
  (root (package (name "P") (body brace (part-usage) (part-usage))))
)
~~~
