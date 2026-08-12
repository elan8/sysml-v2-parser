# META
~~~sexpr
(snapshot (type semantic) (description "Expression relationship endpoints are resolved at publication"))
~~~
# SOURCE
~~~sysml
package M { part def System { part a; part b; connect a to b; } }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "expression_relationship_publication.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package M {
    part def System {
        part a;
        part b;
        connect a to b;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 1) (column 55) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 54) (line 1) (column 55) (len 1)))))
    (reference r1 (scope relative) (span (offset 59) (line 1) (column 60) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 59) (line 1) (column 60) (len 1)))))
  )
  (root (package (name "M") (body (part-def (name "System") (body (part-usage) (part-usage) (connect (from (expression (span (offset 54) (line 1) (column 55) (len 1)) (ref r0))) (to (expression (span (offset 59) (line 1) (column 60) (len 1)) (ref r1))) (body semicolon) (subsets none) (redefines none)))))))
)
~~~
