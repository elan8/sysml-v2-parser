# META
~~~sexpr
(snapshot (type semantic) (description "KerML binding connector with named form + multiplicity + 'of' disambiguation"))
~~~
# SOURCE
~~~sysml
package BindingNamedMult {
    binding instant[instantNum] of startShot = endShot;
    binding all startShot = endShot;
    binding x bind a = b;
    binding [0..1] a = b;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "binding_named_mult.md"
    (diagnostics
      (diagnostic (code "unsupported_grammar_form") (severity error) (category unsupportedgrammarform) (span (offset 31) (line 2) (column 5) (len 56)) (message "the spec-valid BindingConnectorAsUsage production is not implemented in package bodies"))
      (diagnostic (code "unsupported_grammar_form") (severity error) (category unsupportedgrammarform) (span (offset 87) (line 3) (column 5) (len 37)) (message "the spec-valid BindingConnectorAsUsage production is not implemented in package bodies"))
      (diagnostic (code "unsupported_grammar_form") (severity error) (category unsupportedgrammarform) (span (offset 124) (line 4) (column 5) (len 26)) (message "the spec-valid BindingConnectorAsUsage production is not implemented in package bodies"))
      (diagnostic (code "unsupported_grammar_form") (severity error) (category unsupportedgrammarform) (span (offset 150) (line 5) (column 5) (len 22)) (message "the spec-valid BindingConnectorAsUsage production is not implemented in package bodies"))
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
  (root (package (name "BindingNamedMult") (body (unsupported (production binding-connector-as-usage) (code "unsupported_grammar_form") (found "binding instant[instantNum] of startShot = endShot;") (span (offset 31) (line 2) (column 5) (len 56))) (unsupported (production binding-connector-as-usage) (code "unsupported_grammar_form") (found "binding all startShot = endShot;") (span (offset 87) (line 3) (column 5) (len 37))) (unsupported (production binding-connector-as-usage) (code "unsupported_grammar_form") (found "binding x bind a = b;") (span (offset 124) (line 4) (column 5) (len 26))) (unsupported (production binding-connector-as-usage) (code "unsupported_grammar_form") (found "binding [0..1] a = b;") (span (offset 150) (line 5) (column 5) (len 22))))))
)
~~~
