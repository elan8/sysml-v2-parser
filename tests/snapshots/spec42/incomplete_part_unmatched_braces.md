# META
~~~sexpr
(snapshot (type semantic) (description "Incomplete part definition with unmatched braces - formatter adds compensating braces"))
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
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (malformed (code "missing_closing_brace") (found none) (span (offset 0) (line 1) (column 1) (len 56))))
)
~~~
