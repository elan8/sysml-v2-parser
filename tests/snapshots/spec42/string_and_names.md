# META
~~~sexpr
(snapshot (type semantic) (description "String literals and unrestricted names"))
~~~
# SOURCE
~~~sysml
"hello" 'world name' "with\nescapes"
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "string_and_names.md"
    (diagnostics
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 0) (line 1) (column 1) (len 36)) (message "expected a specific keyword or punctuation token"))
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
  (root (malformed (code "expected_keyword") (found "\"hello\" 'world name' \"with\\nescapes\"") (span (offset 0) (line 1) (column 1) (len 36))))
)
~~~
