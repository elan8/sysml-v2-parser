# META
~~~sexpr
(snapshot (type semantic) (description "KerML const-prefixed end features inside an assoc struct body attach the const prefix to the end/feature member instead of misparsing const as a dangling feature reference (spec42 Gap 36)."))
~~~
# SOURCE
~~~sysml
package AssociationEndFeatures {
    assoc struct C {
        end [1] feature x;
        const end [1] feature a;
        const end feature b;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "association_end_features.md"
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
  (root (package (name "AssociationEndFeatures") (body brace (kerml-classifier (keyword assoc struct) (abstract false) (name "C") (specializes none) (body brace (end-member) (end-member) (kerml-feature))))))
)
~~~
