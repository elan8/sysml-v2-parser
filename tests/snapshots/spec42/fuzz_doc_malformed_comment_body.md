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
      (diagnostic (code "recovered_relationship_body_element") (severity error) (category parseerror) (span (offset 24) (line 2) (column 5) (len 25)) (message "unexpected token in relationship body"))
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
    (reference r0 (scope relative) (span (offset 14) (line 1) (column 15) (len 3)) (segments (segment 0 (token "Bar") (name "Bar") (separator none) (span (offset 14) (line 1) (column 15) (len 3)))))
  )
  (root (alias (name "Foo") (target (ref r0)) (body brace (unterminated) (element-count 1))))
)
~~~
