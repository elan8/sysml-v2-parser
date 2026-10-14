# META
~~~sexpr
(snapshot (type semantic) (description "FeatureSpecializationPart = FeatureSpecialization+ MultiplicityPart? FeatureSpecialization* | MultiplicityPart FeatureSpecialization* (SysML BNF 424-426): a part usage may write its multiplicity and ordered/nonunique after its specializations, and an anonymous part usage may open its ValuePart with default."))
~~~
# SOURCE
~~~sysml
package PartMultiplicityAfterSpecializations {
    part def A {
        part x : T :> a[1];
        part y : T :> a [1] ordered;
        part z : T :> a ordered;
        part w :> a [0..*] nonunique :> b;
        part default = 3;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "part_usage_multiplicity_after_specializations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package PartMultiplicityAfterSpecializations {
    part def A {
        part x : T[1] :> a;
        part y : T[1] ordered :> a;
        part z : T ordered :> a;
        part w[0..*] nonunique :> a, b;
        part default = 3;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 81) (line 3) (column 18) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 81) (line 3) (column 18) (len 1)))))
    (reference r1 (scope relative) (span (offset 86) (line 3) (column 23) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 86) (line 3) (column 23) (len 1)))))
    (reference r2 (scope relative) (span (offset 109) (line 4) (column 18) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 109) (line 4) (column 18) (len 1)))))
    (reference r3 (scope relative) (span (offset 114) (line 4) (column 23) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 114) (line 4) (column 23) (len 1)))))
    (reference r4 (scope relative) (span (offset 146) (line 5) (column 18) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 146) (line 5) (column 18) (len 1)))))
    (reference r5 (scope relative) (span (offset 151) (line 5) (column 23) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 151) (line 5) (column 23) (len 1)))))
    (reference r6 (scope relative) (span (offset 180) (line 6) (column 19) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 180) (line 6) (column 19) (len 1)))))
    (reference r7 (scope relative) (span (offset 202) (line 6) (column 41) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 202) (line 6) (column 41) (len 1)))))
  )
  (root (package (name "PartMultiplicityAfterSpecializations") (body brace (part-def (name "A") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "x") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower none) (upper (expression (span (offset 88) (line 3) (column 25) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r1))) (value none))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "y") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity (lower none) (upper (expression (span (offset 117) (line 4) (column 26) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r3))) (value none))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "z") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r5))) (value none))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "w") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 183) (line 6) (column 22) (len 1)) (integer 0))) (upper (expression (span (offset 186) (line 6) (column 25) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r6) (ref r7))) (value none))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 228) (line 7) (column 24) (len 1)) (integer 3))))) (body semicolon)))))))
)
~~~
