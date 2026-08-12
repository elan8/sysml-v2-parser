# META
~~~sexpr
(snapshot (type semantic) (description "Unclosed comment (missing */) at file level should be preserved"))
~~~
# SOURCE
~~~sysml
/* unclosed comment without closing marker
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "unclosed_comment_at_file_level.md"
    (diagnostics
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 0) (line 1) (column 1) (len 40)) (message "expected a specific keyword or punctuation token"))
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
  (root (malformed (code "expected_keyword") (found "/* unclosed comment without closing mark") (span (offset 0) (line 1) (column 1) (len 42))))
)
~~~
