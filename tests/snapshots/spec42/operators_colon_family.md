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
      (diagnostic (code "expected_alt") (severity error) (category parseerror) (span (offset 0) (line 1) (column 1) (len 18)) (message "expected a package member (e.g. part, port, attribute, action, requirement, import)"))
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
  (root (malformed (code "expected_alt") (found ": :: :> ::> :>> :=") (span (offset 0) (line 1) (column 1) (len 18))))
)
~~~
