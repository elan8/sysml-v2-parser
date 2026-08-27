# META
~~~sexpr
(snapshot (type malformed) (description "Verifies an entirely malformed document produces explicit recovery nodes, diagnostics, and deterministic recovered formatting."))
~~~
# SOURCE
~~~sysml
not valid
also broken
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "malformed_document.md"
    (diagnostics
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 0) (line 1) (column 1) (len 9)) (message "expected a specific keyword or punctuation token"))
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 10) (line 2) (column 1) (len 11)) (message "expected a specific keyword or punctuation token"))
    )
  )
)
~~~
# FORMAT
~~~sysml
not valid

also broken
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (malformed (code "expected_keyword") (found "not valid") (span (offset 0) (line 1) (column 1) (len 10))) (malformed (code "expected_keyword") (found "also broken") (span (offset 10) (line 2) (column 1) (len 11))))
)
~~~
