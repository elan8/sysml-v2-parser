# META
~~~sexpr
(snapshot (type semantic) (description "A ConnectorEnd spans its multiplicity and target wherever it is written: the same connect [1] a to b; has the same end spans in a connection usage and as a bare connect member of a part definition."))
~~~
# SOURCE
~~~sysml
package ConnectionEndSpans {
    part def A {
        connection c connect [1] a to b;
        connect [1] a to b;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connection_end_multiplicity_span.md"
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 79) (line 3) (column 34) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 79) (line 3) (column 34) (len 1)))))
    (reference r1 (scope relative) (span (offset 84) (line 3) (column 39) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 84) (line 3) (column 39) (len 1)))))
    (reference r2 (scope relative) (span (offset 107) (line 4) (column 21) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 107) (line 4) (column 21) (len 1)))))
    (reference r3 (scope relative) (span (offset 112) (line 4) (column 26) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 112) (line 4) (column 26) (len 1)))))
  )
  (root (package (visibility none) (name "ConnectionEndSpans") (body brace (part-def (name "A") (modifiers) (body brace (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect (end-span (span (offset 75) (line 3) (column 30) (len 5))) (multiplicity (lower none) (upper (expression (span (offset 76) (line 3) (column 31) (len 1)) (integer 1)))) (expression (span (offset 79) (line 3) (column 34) (len 1)) (ref r0)) (expression (span (offset 84) (line 3) (column 39) (len 1)) (ref r1))) (body semicolon)) (connect (from (end-span (span (offset 103) (line 4) (column 17) (len 5))) (multiplicity (lower none) (upper (expression (span (offset 104) (line 4) (column 18) (len 1)) (integer 1)))) (expression (span (offset 107) (line 4) (column 21) (len 1)) (ref r2))) (to (expression (span (offset 112) (line 4) (column 26) (len 1)) (ref r3))) (body semicolon) (subsets none) (redefines none)))))))
)
~~~
