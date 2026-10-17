# META
~~~sexpr
(snapshot (type semantic) (description "Unclosed comment with special characters should be preserved"))
~~~
# SOURCE
~~~sysml
/* isio . /% #ato
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "unclosed_comment_special_chars.md"
    (diagnostics
      (diagnostic (code "expected_alt") (severity error) (category parseerror) (span (offset 0) (line 1) (column 1) (len 17)) (message "expected a package member (e.g. part, port, attribute, action, requirement, import)"))
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
  (root (malformed (code "expected_alt") (found "/* isio . /% #ato") (span (offset 0) (line 1) (column 1) (len 17))))
)
~~~
