# META
~~~sexpr
(snapshot (type semantic) (description "Multiplicity authored on declaration kinds previously missing an AST field survives parsing and typed emission."))
~~~
# SOURCE
~~~sysml
package UpstreamGapMultiplicities {
    part readings : Reading[0..*] ordered nonunique;
    attribute def Samples : Real[2];
    constraint checks : Constraint[0..*];
    requirement needs : Requirement[2];
    requirement def Container {
        actor stakeholders : Person[1..*];
        calc calculations : Calculation[1..*];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_gap_multiplicities.md"
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
    (reference r0 (scope relative) (span (offset 56) (line 2) (column 21) (len 7)) (segments (segment 0 (token "Reading") (name "Reading") (separator none) (span (offset 56) (line 2) (column 21) (len 7)))))
    (reference r1 (scope relative) (span (offset 117) (line 3) (column 29) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 117) (line 3) (column 29) (len 4)))))
    (reference r2 (scope relative) (span (offset 150) (line 4) (column 25) (len 10)) (segments (segment 0 (token "Constraint") (name "Constraint") (separator none) (span (offset 150) (line 4) (column 25) (len 10)))))
    (reference r3 (scope relative) (span (offset 269) (line 7) (column 30) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 269) (line 7) (column 30) (len 6)))))
  )
  (root (package (name "UpstreamGapMultiplicities") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "readings") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower (expression (span (offset 64) (line 2) (column 29) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (subsets none) (redefines none) (value none) (body semicolon)) (attribute-def (declaration-name "Samples") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity (lower (expression (span (offset 122) (line 3) (column 34) (len 1)) (integer 2))) (upper (expression (span (offset 122) (line 3) (column 34) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "checks") (short-name none) (type (ref r2)) (multiplicity (lower (expression (span (offset 161) (line 4) (column 36) (len 1)) (integer 0))) (upper unbounded)) (subsets none) (redefines none) (body semicolon)) (requirement-usage (name "needs") (multiplicity (lower (expression (span (offset 204) (line 5) (column 37) (len 1)) (integer 2))) (upper (expression (span (offset 204) (line 5) (column 37) (len 1)) (integer 2))))) (requirement-def (name "Container") (body brace (actor (name "stakeholders") (type (ref r3)) (multiplicity (lower (expression (span (offset 276) (line 7) (column 37) (len 1)) (integer 1))) (upper unbounded))) (calc-usage (name "calculations") (multiplicity (lower (expression (span (offset 323) (line 8) (column 41) (len 1)) (integer 1))) (upper unbounded))))))))
)
~~~
