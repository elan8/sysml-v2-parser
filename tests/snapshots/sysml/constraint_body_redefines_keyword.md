# META
~~~sexpr
(snapshot (type semantic) (description "A constraint usage body binds inherited parameters with the `redefines` keyword exactly as with `:>>` (training 31 Constraints Example 2): each member is one redefining attribute usage, not a refused `redefines` keyword nor a shredded expression."))
~~~
# SOURCE
~~~sysml
package P {
    part def Vehicle {
        constraint massConstraint : MassConstraint {
            redefines partMasses = (chassisMass, engineMass);
            :>> massLimit = 2500;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "constraint_body_redefines_keyword.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    part def Vehicle {
        constraint massConstraint : MassConstraint {
            :>> partMasses = (chassisMass, engineMass);
            :>> massLimit = 2500;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 71) (line 3) (column 37) (len 14)) (segments (segment 0 (token "MassConstraint") (name "MassConstraint") (separator none) (span (offset 71) (line 3) (column 37) (len 14)))))
    (reference r1 (scope relative) (span (offset 110) (line 4) (column 23) (len 10)) (segments (segment 0 (token "partMasses") (name "partMasses") (separator none) (span (offset 110) (line 4) (column 23) (len 10)))))
    (reference r2 (scope relative) (span (offset 124) (line 4) (column 37) (len 11)) (segments (segment 0 (token "chassisMass") (name "chassisMass") (separator none) (span (offset 124) (line 4) (column 37) (len 11)))))
    (reference r3 (scope relative) (span (offset 137) (line 4) (column 50) (len 10)) (segments (segment 0 (token "engineMass") (name "engineMass") (separator none) (span (offset 137) (line 4) (column 50) (len 10)))))
    (reference r4 (scope relative) (span (offset 166) (line 5) (column 17) (len 9)) (segments (segment 0 (token "massLimit") (name "massLimit") (separator none) (span (offset 166) (line 5) (column 17) (len 9)))))
  )
  (root (package (name "P") (body brace (part-def (name "Vehicle") (modifiers) (body brace (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "massConstraint") (short-name none) (type (ref r0)) (multiplicity none) (subsets none) (redefines none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 123) (line 4) (column 36) (len 25)) (sequence (sequence-list (element first (expression (span (offset 124) (line 4) (column 37) (len 11)) (ref r2))) (element comma (expression (span (offset 137) (line 4) (column 50) (len 10)) (ref r3))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 178) (line 5) (column 29) (len 4)) (integer 2500))))) (body semicolon)))))))))
)
~~~
