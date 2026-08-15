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
  (root (package (name "NestedClassifiers") (body brace (part-usage) (part-def (name "PD") (body brace (kerml-classifier (keyword struct) (name "S1")) (kerml-classifier (keyword datatype) (name "D1")))) (attribute-def))))
)
~~~
