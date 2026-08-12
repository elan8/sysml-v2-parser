# META
~~~sexpr
(snapshot (type semantic) (description "Fuzz: binding connector formats name before multiplicity"))
~~~
# SOURCE
~~~sysml
package P {
    binding b [5] of a = c;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_binding_name_before_mult.md"
    (diagnostics
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 16) (line 2) (column 5) (len 24)) (message "unexpected keyword `binding` in package body"))
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
  (root (package (name "P") (body (malformed (code "unexpected_keyword_in_scope") (found "binding b [5] of a = c;") (span (offset 16) (line 2) (column 5) (len 24))))))
)
~~~
