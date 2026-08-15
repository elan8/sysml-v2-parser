# META
~~~sexpr
(snapshot (type semantic) (description "Package containing part definitions"))
~~~
# SOURCE
~~~sysml
package Vehicles {
    part def Car;
    part def Truck;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parse_package_with_parts.md"
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
  (root (package (name "Vehicles") (body brace (part-def (name "Car") (body semicolon)) (part-def (name "Truck") (body semicolon)))))
)
~~~
