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
  (root (package (name "M") (body brace (part-def (name "System") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "a") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "b") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (connect (from (expression (span (offset 54) (line 1) (column 55) (len 1)) (ref r0))) (to (expression (span (offset 59) (line 1) (column 60) (len 1)) (ref r1))) (body semicolon) (subsets none) (redefines none)))))))
)
~~~
