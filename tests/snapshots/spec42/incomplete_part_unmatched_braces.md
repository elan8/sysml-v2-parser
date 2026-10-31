# META
~~~sexpr
(snapshot (type semantic) (description "Incomplete part definition with unmatched braces - the unterminated bodies keep their members, and the formatter writes no compensating braces"))
~~~
# SOURCE
~~~sysml
package AyPkpowerTrain {
    part engine {
        g { }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "incomplete_part_unmatched_braces.md"
    (diagnostics
      (diagnostic (code "missing_closing_brace") (severity none) (category parseerror) (span (offset 56) (line 3) (column 14) (len 1)) (message "missing closing '}'"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package AyPkpowerTrain {
    part engine {
        g {
        }
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
  )
  (root (package (visibility none) (name "AyPkpowerTrain") (body brace (unterminated) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (unterminated) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "g") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace)))))))
)
~~~
