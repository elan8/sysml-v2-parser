# META
~~~sexpr
(snapshot (type semantic) (description "KerML TypeDeclaration retains disjoining clauses; the SysML definition rejection must not affect the KerML grammar.") (name-language kerml))
~~~
# SOURCE
~~~sysml
package P {
    abstract behavior A;
    abstract behavior B disjoint from A;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "classifier_disjoining_valid.md"
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
  (name-language kerml)
  (references
  )
  (root (package (name "P") (body brace (kerml-classifier (keyword behavior) (abstract true) (name "A") (multiplicity none) (specializes none) (conjugates none) (body semicolon)) (kerml-classifier (keyword behavior) (abstract true) (name "B") (multiplicity none) (specializes none) (conjugates none) (body semicolon)))))
)
~~~
