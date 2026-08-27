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
  (root (package (name "BindingNamedMult") (body brace (binding-connector-usage) (binding-connector-usage) (binding-connector-usage) (binding-connector-usage))))
)
~~~
