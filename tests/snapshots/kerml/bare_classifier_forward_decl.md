# META
~~~sexpr
(snapshot (type recovery) (description "Bare classifier forward declaration retains a structured name.") (name-language kerml))
~~~
# SOURCE
~~~sysml
package ScratchClassifier {
    classifier SpatialFrame;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "bare_classifier_forward_decl.md"
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
  (root (package (name "ScratchClassifier") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "SpatialFrame") (multiplicity none) (specializes none) (conjugates none) (body semicolon)))))
)
~~~
