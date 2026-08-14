# META
~~~sexpr
(snapshot (type semantic) (description "Colon family operators"))
~~~
# SOURCE
~~~sysml
: :: :> ::> :>> :=
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "operators_colon_family.md"
    (diagnostics
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 0) (line 1) (column 1) (len 18)) (message "expected a specific keyword or punctuation token"))
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
  (root (malformed (code "expected_keyword") (found ": :: :> ::> :>> :=") (span (offset 0) (line 1) (column 1) (len 18))))
)
~~~
