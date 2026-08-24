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
    (reference r0 (scope relative) (span (offset 93) (line 4) (column 30) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 93) (line 4) (column 30) (len 5)))))
  )
  (root (package (name "NestedClassifiers") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (kerml-classifier (keyword struct) (abstract false) (name "Car1_") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "wheels") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower (expression (span (offset 99) (line 4) (column 36) (len 1)) (integer 4))) (upper (expression (span (offset 99) (line 4) (column 36) (len 1)) (integer 4)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword behavior) (abstract false) (name "Drive") (specializes none) (conjugates none) (body semicolon)))) (part-def (name "PD") (modifiers) (body brace (kerml-classifier (keyword struct) (abstract false) (name "S1") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "f") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword datatype) (abstract false) (name "D1") (specializes none) (conjugates none) (body semicolon)))) (attribute-def (declaration-name "AD") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (kerml-classifier (keyword classifier) (abstract false) (name "C1") (specializes none) (conjugates none) (body semicolon)))))))
)
~~~
