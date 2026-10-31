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
  (name-language sysml)
  (references
  )
  (root (package (visibility none) (name "Outer") (body brace (package (visibility none) (name "Inner") (body brace)))))
)
~~~
