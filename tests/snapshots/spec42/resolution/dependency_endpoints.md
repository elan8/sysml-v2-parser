# META
~~~sexpr
(snapshot (type semantic) (description "Dependency endpoint resolution coverage"))
~~~
# SOURCE
~~~sysml
package DependencyCoverage {
    part def Source;
    part def Target;
    dependency from Source to Target;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "dependency_endpoints.md"
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
    (reference r0 (scope relative) (span (offset 91) (line 4) (column 21) (len 6)) (segments (segment 0 (token "Source") (name "Source") (separator none) (span (offset 91) (line 4) (column 21) (len 6)))))
    (reference r1 (scope relative) (span (offset 101) (line 4) (column 31) (len 6)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 101) (line 4) (column 31) (len 6)))))
  )
  (root (package (name "DependencyCoverage") (body brace (part-def (name "Source") (body semicolon)) (part-def (name "Target") (body semicolon)) (dependency (clients (ref r0)) (suppliers (ref r1)) (body semicolon)))))
)
~~~
