# META
~~~sexpr
(snapshot (type semantic) (description "Nested package definitions"))
~~~
# SOURCE
~~~sysml
package Outer {
    package Inner { }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parse_nested_package.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Outer {
    package Inner {
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Outer") (body (package (name "Inner") (body )))))
)
~~~
