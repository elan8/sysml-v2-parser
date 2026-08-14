# META
~~~sexpr
(snapshot (type semantic) (description "A dependency's braced RelationshipBody accepts owned feature members alongside the annotation subset (spec42 Gap 37), and the unbodied forms stay unchanged."))
~~~
# SOURCE
~~~sysml
package DependencyBodies {
    feature x;
    feature y;
    dependency Use from x to y;
    dependency z to x, y {
        doc /* Rationale. */
        feature e;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "dependency_bodies.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package DependencyBodies {
    feature x;
    feature y;
    dependency Use from x to y;
    dependency from z to x, y {
        doc
        /* Rationale. */
        feature e;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 81) (line 4) (column 25) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 81) (line 4) (column 25) (len 1)))))
    (reference r1 (scope relative) (span (offset 86) (line 4) (column 30) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 86) (line 4) (column 30) (len 1)))))
    (reference r2 (scope relative) (span (offset 104) (line 5) (column 16) (len 1)) (segments (segment 0 (token "z") (name "z") (separator none) (span (offset 104) (line 5) (column 16) (len 1)))))
    (reference r3 (scope relative) (span (offset 109) (line 5) (column 21) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 109) (line 5) (column 21) (len 1)))))
    (reference r4 (scope relative) (span (offset 112) (line 5) (column 24) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 112) (line 5) (column 24) (len 1)))))
  )
  (root (package (name "DependencyBodies") (body (kerml-feature (name "x")) (kerml-feature (name "y")) (dependency (clients (ref r0)) (suppliers (ref r1))) (dependency (clients (ref r2)) (suppliers (ref r3) (ref r4))))))
)
~~~
