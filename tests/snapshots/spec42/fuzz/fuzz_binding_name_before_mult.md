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
      (diagnostic (code "unsupported_grammar_form") (severity error) (category unsupportedgrammarform) (span (offset 16) (line 2) (column 5) (len 24)) (message "the spec-valid BindingConnectorAsUsage production is not implemented in package bodies"))
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
  (root (package (name "P") (body (unsupported (production binding-connector-as-usage) (code "unsupported_grammar_form") (found "binding b [5] of a = c;") (span (offset 16) (line 2) (column 5) (len 24))))))
)
~~~
