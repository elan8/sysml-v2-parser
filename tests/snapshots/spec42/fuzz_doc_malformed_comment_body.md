# META
~~~sexpr
(snapshot (type semantic) (description "Documentation node with malformed comment body should close the comment when formatting"))
~~~
# SOURCE
~~~sysml
alias Foo for Bar {
    doc /* unclosed comment
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_doc_malformed_comment_body.md"
    (diagnostics
      (diagnostic (code "expected_alt") (severity error) (category parseerror) (span (offset 0) (line 1) (column 1) (len 19)) (message "expected a package member (e.g. part, port, attribute, action, requirement, import)"))
      (diagnostic (code "missing_closing_brace") (severity none) (category parseerror) (span (offset 49) (line 3) (column 2) (len 1)) (message "missing closing '}'"))
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
  (root (malformed (code "expected_alt") (found "alias Foo for Bar {") (span (offset 0) (line 1) (column 1) (len 49))))
)
~~~
