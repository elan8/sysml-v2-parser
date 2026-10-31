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
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    binding b[5] of a = c;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
  )
  (root (package (visibility none) (name "P") (body brace (binding-connector-usage))))
)
~~~
