# META
~~~sexpr
(snapshot (type semantic) (description "Multiplicity and short names authored on declaration kinds previously missing an AST field or a parser slot survive parsing and typed emission: `ref`, `subject`, both `actor` spellings, and a namespace-level calculation usage (spec42 Gap 53)."))
~~~
# SOURCE
~~~sysml
package UpstreamGapMultiplicities {
    part readings : Reading[0..*] ordered nonunique;
    attribute def Samples : Real[2];
    constraint checks : Constraint[0..*];
    requirement needs : Requirement[2];
    requirement def Container {
        subject <s> subj : Vehicle;
        actor <a> stakeholders : Person[1..*];
        calc calculations : Calculation[1..*];
    }
    use case def Scenario {
        actor <u> user : Person[0..4];
    }
    ref <rd> reading : Reading[0..*] nonunique;
    calc estimate [1];
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
~~~sysml
package UpstreamGapMultiplicities {
    part readings : Reading[0..*] ordered nonunique;
    attribute def Samples : Real[2];
    constraint checks : Constraint[0..*];
    requirement needs : Requirement[2];
    requirement def Container {
        subject <s> subj : Vehicle;
        actor <a> stakeholders : Person[1..*];
        calc calculations : Calculation[1..*];
    }
    use case def Scenario {
        actor <u> user : Person[0..4];
    }
    ref <rd> reading : Reading[0..*] nonunique;
    calc estimate[1];
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 56) (line 2) (column 21) (len 7)) (segments (segment 0 (token "Reading") (name "Reading") (separator none) (span (offset 56) (line 2) (column 21) (len 7)))))
    (reference r1 (scope relative) (span (offset 117) (line 3) (column 29) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 117) (line 3) (column 29) (len 4)))))
    (reference r2 (scope relative) (span (offset 150) (line 4) (column 25) (len 10)) (segments (segment 0 (token "Constraint") (name "Constraint") (separator none) (span (offset 150) (line 4) (column 25) (len 10)))))
    (reference r3 (scope relative) (span (offset 267) (line 7) (column 28) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 267) (line 7) (column 28) (len 7)))))
    (reference r4 (scope relative) (span (offset 309) (line 8) (column 34) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 309) (line 8) (column 34) (len 6)))))
    (reference r5 (scope relative) (span (offset 429) (line 12) (column 26) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 429) (line 12) (column 26) (len 6)))))
    (reference r6 (scope relative) (span (offset 472) (line 14) (column 24) (len 7)) (segments (segment 0 (token "Reading") (name "Reading") (separator none) (span (offset 472) (line 14) (column 24) (len 7)))))
  )
  (root (package (name "UpstreamGapMultiplicities") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "readings") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower (expression (span (offset 64) (line 2) (column 29) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (subsets none) (redefines none) (value none) (body semicolon)) (attribute-def (declaration-name "Samples") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity (lower (expression (span (offset 122) (line 3) (column 34) (len 1)) (integer 2))) (upper (expression (span (offset 122) (line 3) (column 34) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "checks") (short-name none) (type (ref r2)) (multiplicity (lower (expression (span (offset 161) (line 4) (column 36) (len 1)) (integer 0))) (upper unbounded)) (subsets none) (redefines none) (body semicolon)) (requirement-usage (name "needs") (multiplicity (lower (expression (span (offset 204) (line 5) (column 37) (len 1)) (integer 2))) (upper (expression (span (offset 204) (line 5) (column 37) (len 1)) (integer 2))))) (requirement-def (name "Container") (modifiers) (body brace (subject (name "subj") (short-name "s") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (actor (name "stakeholders") (short-name "a") (type (ref r4)) (multiplicity (lower (expression (span (offset 316) (line 8) (column 41) (len 1)) (integer 1))) (upper unbounded))) (calc-usage (name "calculations") (multiplicity (lower (expression (span (offset 363) (line 9) (column 41) (len 1)) (integer 1))) (upper unbounded))))) (use-case-def (name "Scenario") (modifiers) (body brace (actor (name "user") (short-name "u") (type (ref r5)) (multiplicity (lower (expression (span (offset 436) (line 12) (column 33) (len 1)) (integer 0))) (upper (expression (span (offset 439) (line 12) (column 36) (len 1)) (integer 4))))))) (ref (name "reading") (short-name "rd") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity (lower (expression (span (offset 480) (line 14) (column 32) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (redefines none) (subsets none) (body semicolon)) (calc-usage (name "estimate") (short-name none) (direction none) (abstract false) (reference false) (type none) (multiplicity (lower (expression (span (offset 516) (line 15) (column 20) (len 1)) (integer 1))) (upper (expression (span (offset 516) (line 15) (column 20) (len 1)) (integer 1)))) (subsets none) (redefines none) (value none) (body semicolon)))))
)
~~~
