# META
~~~sexpr
(snapshot (type semantic) (description "Malformed tokens with recovery"))
~~~
# SOURCE
~~~sysml
x ` y
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "malformed_recovery.md"
    (diagnostics
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 0) (line 1) (column 1) (len 5)) (message "expected a specific keyword or punctuation token"))
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
  (root (malformed (code "expected_keyword") (found "x ` y") (span (offset 0) (line 1) (column 1) (len 5))))
)
~~~
