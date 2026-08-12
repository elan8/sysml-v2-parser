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
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 31) (line 2) (column 5) (len 141)) (message "unexpected keyword `binding` in package body"))
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
  (root (package (name "BindingNamedMult") (body (malformed (code "unexpected_keyword_in_scope") (found "binding instant[instantNum] of startShot = endShot;") (span (offset 31) (line 2) (column 5) (len 141))))))
)
~~~
