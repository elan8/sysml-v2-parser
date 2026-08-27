# META
~~~sexpr
(snapshot (type semantic) (description "Simple package declaration"))
~~~
# SOURCE
~~~sysml
package MyPkg { }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "package_declaration.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package MyPkg {
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "MyPkg") (body brace)))
)
~~~
