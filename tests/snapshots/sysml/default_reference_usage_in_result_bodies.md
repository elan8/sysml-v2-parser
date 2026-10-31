# META
~~~sexpr
(snapshot (type semantic) (description "`DefaultReferenceUsage = RefPrefix Usage`, so a terminated bare `name;` is a usage, not a result expression with an illegal semicolon, in case, calculation and constraint bodies. The result expression that follows keeps its own form."))
~~~
# SOURCE
~~~sysml
case def InCase {
    vehicle;
    vehicle
}
calc def InCalc {
    vehicle;
    vehicle
}
constraint def InConstraint {
    vehicle;
    vehicle
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "default_reference_usage_in_result_bodies.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
case def InCase {
    vehicle;
    vehicle
}

calc def InCalc {
    vehicle;
    vehicle
}

constraint def InConstraint {
    vehicle;
    vehicle
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 35) (line 3) (column 5) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 35) (line 3) (column 5) (len 7)))))
    (reference r1 (scope relative) (span (offset 80) (line 7) (column 5) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 80) (line 7) (column 5) (len 7)))))
    (reference r2 (scope relative) (span (offset 137) (line 11) (column 5) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 137) (line 11) (column 5) (len 7)))))
  )
  (root (case-def (name "InCase") (modifiers) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (expression (span (offset 35) (line 3) (column 5) (len 7)) (ref r0)))) (calc-def (name "InCalc") (modifiers) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (expression (expression (span (offset 80) (line 7) (column 5) (len 7)) (ref r1))))) (constraint-def (name "InConstraint") (modifiers) (specializes none) (body brace (default-reference-usage) (expression (span (offset 137) (line 11) (column 5) (len 7)) (ref r2)))))
)
~~~
