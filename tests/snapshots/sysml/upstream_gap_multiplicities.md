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
    (reference r1 (scope relative) (span (offset 269) (line 7) (column 30) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 269) (line 7) (column 30) (len 6)))))
  )
  (root (package (name "UpstreamGapMultiplicities") (body brace (part-usage (declaration-name "readings") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity-modifiers (ordered true) (nonunique true)) (body semicolon)) (attribute-def (name "Samples") (multiplicity (lower (expression (span (offset 122) (line 3) (column 34) (len 1)) (integer 2))) (upper (expression (span (offset 122) (line 3) (column 34) (len 1)) (integer 2))))) (constraint-usage (name "checks") (short-name none) (multiplicity (lower (expression (span (offset 161) (line 4) (column 36) (len 1)) (integer 0))) (upper unbounded))) (requirement-usage (name "needs") (multiplicity (lower (expression (span (offset 204) (line 5) (column 37) (len 1)) (integer 2))) (upper (expression (span (offset 204) (line 5) (column 37) (len 1)) (integer 2))))) (requirement-def (name "Container") (body brace (actor (name "stakeholders") (type (ref r1)) (multiplicity (lower (expression (span (offset 276) (line 7) (column 37) (len 1)) (integer 1))) (upper unbounded))) (calc-usage (name "calculations") (multiplicity (lower (expression (span (offset 323) (line 8) (column 41) (len 1)) (integer 1))) (upper unbounded))))))))
)
~~~
