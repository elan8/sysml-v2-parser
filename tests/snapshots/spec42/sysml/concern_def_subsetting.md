# META
~~~sexpr
(snapshot (type semantic) (description "concern def and concern usage retain :>/:>> specialization clauses"))
~~~
# SOURCE
~~~sysml
package ConcernDefSubsettingExample {
    concern def ConcernCheck :> RequirementCheck {
    }
    concern c :>> baseConcern;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "concern_def_subsetting.md"
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
    (reference r0 (scope relative) (span (offset 70) (line 2) (column 33) (len 16)) (segments (segment 0 (token "RequirementCheck") (name "RequirementCheck") (separator none) (span (offset 70) (line 2) (column 33) (len 16)))))
    (reference r1 (scope relative) (span (offset 113) (line 4) (column 19) (len 11)) (segments (segment 0 (token "baseConcern") (name "baseConcern") (separator none) (span (offset 113) (line 4) (column 19) (len 11)))))
  )
  (root (package (name "ConcernDefSubsettingExample") (body brace (concern-usage (name "ConcernCheck") (visibility none) (abstract false) (definition true) (type none) (multiplicity none) (subsets (relationship (kind subsets) (implied false) (targets (ref r0)))) (redefines none) (body brace)) (concern-usage (name "c") (visibility none) (abstract false) (definition false) (type none) (multiplicity none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (body semicolon)))))
)
~~~
