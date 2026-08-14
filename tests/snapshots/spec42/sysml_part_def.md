# META
~~~sexpr
(snapshot (type semantic) (description "SysML part definition"))
~~~
# SOURCE
~~~sysml
part def Vehicle { }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml_part_def.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
part def Vehicle {
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (part-def (name "Vehicle") (body )))
)
~~~
