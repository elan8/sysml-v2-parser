# META
~~~sexpr
(snapshot (type semantic) (description "KerML FeatureDeclaration retains an ordered, repeatable FeatureRelationshipPart tail, including a comma-separated featured-by target list and a value after the tail."))
~~~
# SOURCE
~~~sysml
package P {
    class C {
        feature f : T :> S featured by A, B chains c.d unions U, V inverse of I featured by C = value;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "feature_relationship_parts.md"
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
    (reference r0 (scope relative) (span (offset 46) (line 3) (column 21) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 46) (line 3) (column 21) (len 1)))))
    (reference r1 (scope relative) (span (offset 51) (line 3) (column 26) (len 1)) (segments (segment 0 (token "S") (name "S") (separator none) (span (offset 51) (line 3) (column 26) (len 1)))))
    (reference r2 (scope relative) (span (offset 65) (line 3) (column 40) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 65) (line 3) (column 40) (len 1)))))
    (reference r3 (scope relative) (span (offset 68) (line 3) (column 43) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 68) (line 3) (column 43) (len 1)))))
    (reference r4 (scope relative) (span (offset 77) (line 3) (column 52) (len 3)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 77) (line 3) (column 52) (len 1))) (segment 1 (token "d") (name "d") (separator dot) (span (offset 79) (line 3) (column 54) (len 1)))))
    (reference r5 (scope relative) (span (offset 88) (line 3) (column 63) (len 1)) (segments (segment 0 (token "U") (name "U") (separator none) (span (offset 88) (line 3) (column 63) (len 1)))))
    (reference r6 (scope relative) (span (offset 91) (line 3) (column 66) (len 1)) (segments (segment 0 (token "V") (name "V") (separator none) (span (offset 91) (line 3) (column 66) (len 1)))))
    (reference r7 (scope relative) (span (offset 104) (line 3) (column 79) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 104) (line 3) (column 79) (len 1)))))
    (reference r8 (scope relative) (span (offset 118) (line 3) (column 93) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 118) (line 3) (column 93) (len 1)))))
    (reference r9 (scope relative) (span (offset 122) (line 3) (column 97) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 122) (line 3) (column 97) (len 5)))))
  )
  (root (package (name "P") (body brace (kerml-classifier (keyword class) (abstract false) (name "C") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "f") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (relationships (featured-by (ref r2) (ref r3)) (chains (ref r4)) (type-relationship (keyword unions) (targets (ref r5) (ref r6))) (inverse-of (ref r7)) (featured-by (ref r8))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 122) (line 3) (column 97) (len 5)) (ref r9))))) (body semicolon)))))))
)
~~~
