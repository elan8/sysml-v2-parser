# META
~~~sexpr
(snapshot (type semantic) (description "KerML type-body end features: `end y;` and `end bool g;` (TransitionPerformances.kerml) are one Feature member each, not refused as misplaced keywords; `end x feature y;` carries `x` as the OwnedCrossFeature (KerML BNF 584/592/595) while `end y;` has no cross."))
~~~
# SOURCE
~~~sysml
package P {
    assoc struct A {
        end y;
        end bool g;
        end x feature z;
        end w [0..1] feature v : V;
        const end [1] feature u;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "end_feature_owned_cross_feature.md"
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
    (reference r0 (scope relative) (span (offset 126) (line 6) (column 34) (len 1)) (segments (segment 0 (token "V") (name "V") (separator none) (span (offset 126) (line 6) (column 34) (len 1)))))
  )
  (root (package (name "P") (body brace (kerml-classifier (keyword assoc struct) (abstract false) (name "A") (multiplicity none) (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind none) (member false) (all false) (name "y") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind bool) (member false) (all false) (name "g") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head end) (constant false) (cross present) (metadata)) (kind feature) (member false) (all false) (name "z") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head end) (constant false) (cross present) (metadata)) (kind feature) (member false) (all false) (name "v") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head end) (constant true) (cross present) (metadata)) (kind feature) (member false) (all false) (name "u") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))))))
)
~~~
