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
      (diagnostic (code "expected_alt") (severity error) (category parseerror) (span (offset 0) (line 1) (column 1) (len 40)) (message "expected a package member (e.g. part, port, attribute, action, requirement, import)"))
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
  )
  (root (malformed (code "expected_alt") (found "/* unclosed comment without closing mark") (span (offset 0) (line 1) (column 1) (len 42))))
)
~~~
