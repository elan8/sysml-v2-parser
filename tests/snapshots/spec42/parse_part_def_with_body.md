# META
~~~sexpr
(snapshot (type semantic) (description "Part def with nested part defs"))
~~~
# SOURCE
~~~sysml
part def Vehicle {
    part def Engine;
    part def Wheel;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parse_part_def_with_body.md"
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
  (root (part-def (name "Vehicle") (body brace (part-def (name "Engine") (body semicolon)) (part-def (name "Wheel") (body semicolon)))))
)
~~~
