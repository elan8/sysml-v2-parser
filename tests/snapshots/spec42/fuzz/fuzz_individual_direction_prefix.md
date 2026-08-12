# META
~~~sexpr
(snapshot (type semantic) (description "Fuzz: individual usage with direction prefix preserves 'individual' keyword"))
~~~
# SOURCE
~~~sysml
in individual it;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_individual_direction_prefix.md"
    (diagnostics
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 0) (line 1) (column 1) (len 17)) (message "expected a specific keyword or punctuation token"))
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
  (root (malformed (code "expected_keyword") (found "in individual it;") (span (offset 0) (line 1) (column 1) (len 17))))
)
~~~
