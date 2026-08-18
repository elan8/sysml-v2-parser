# META
~~~sexpr
(snapshot (type semantic) (description "KerML classifier-keyword declarations (struct, classifier, behavior, datatype) nested inside part usage, part definition, and attribute definition bodies dispatch to the typed KermlClassifierDecl production (spec42 Gap 38)."))
~~~
# SOURCE
~~~sysml
package NestedClassifiers {
    part c {
        struct Car1_ {
            feature wheels : Wheel[4];
        }
        behavior Drive;
    }
    part def PD {
        struct S1 {
            feature f;
        }
        datatype D1;
    }
    attribute def AD {
        classifier C1;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "nested_classifier_declarations.md"
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
  (root (package (name "NestedClassifiers") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (kerml-classifier (keyword struct) (abstract false) (name "Car1_") (specializes none) (body brace (kerml-feature))) (kerml-classifier (keyword behavior) (abstract false) (name "Drive") (specializes none) (body semicolon)))) (part-def (name "PD") (body brace (kerml-classifier (keyword struct) (abstract false) (name "S1") (specializes none) (body brace (kerml-feature))) (kerml-classifier (keyword datatype) (abstract false) (name "D1") (specializes none) (body semicolon)))) (attribute-def (declaration-name "AD") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (kerml-classifier (keyword classifier) (abstract false) (name "C1") (specializes none) (body semicolon)))))))
)
~~~
