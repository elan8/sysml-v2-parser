# META
~~~sexpr
(snapshot (type semantic) (description "KerML const-prefixed end features inside an assoc struct body attach the const prefix to EndFeaturePrefix (KerML BNF 573) instead of misparsing const as a dangling feature reference (spec42 Gap 36). All three are one Feature production: `end [1] feature x;` and `const end [1] feature a;` carry their `[1]` on the OwnedCrossFeature (592/595) that FeaturePrefix's end alternative owns, and `const end feature b;` has no cross at all, so all three are (kerml-feature) rather than a separate node wrapping a feature that offered `end` and `const` a second time."))
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
  (root (package (name "AssociationEndFeatures") (body brace (kerml-classifier (keyword assoc struct) (abstract false) (name "C") (specializes none) (body brace (kerml-feature) (kerml-feature) (kerml-feature))))))
)
~~~
