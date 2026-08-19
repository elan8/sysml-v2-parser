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
~~~sysml
package IntersectCoverage {
    part def Base;
    attribute def a;
    attribute def b;
    attribute def reading : Base;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 105) (line 5) (column 25) (len 4)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 105) (line 5) (column 25) (len 4)))))
  )
  (root (package (name "IntersectCoverage") (body brace (part-def (name "Base") (body semicolon)) (attribute-def (declaration-name "a") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "b") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "reading") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)))))
)
~~~
