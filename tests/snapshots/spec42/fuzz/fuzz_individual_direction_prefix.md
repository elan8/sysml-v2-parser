# META
~~~sexpr
(snapshot (type semantic) (description "Fuzz: individual usage with direction prefix preserves 'individual' keyword"))
~~~
# SOURCE
~~~sysml
in individual it;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_individual_direction_prefix.md"
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
  (root (occurrence (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "it") (short-name none) (target none) (body semicolon)))
)
~~~
