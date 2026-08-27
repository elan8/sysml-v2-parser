# META
~~~sexpr
(snapshot (type semantic) (description "Intersecting relationship resolution coverage"))
~~~
# SOURCE
~~~sysml
package IntersectCoverage {
    part def Base;
    attribute a;
    attribute b;
    attribute reading : Base intersects a, b;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "intersecting_relationship.md"
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
  (root (package (name "IntersectCoverage") (body brace (part-def (name "Base") (modifiers) (body semicolon)) (attribute-usage) (attribute-usage) (attribute-usage))))
)
~~~
