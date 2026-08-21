# META
~~~sexpr
(snapshot (type semantic) (description "Exact SubjectUsage form from Requirements Examples/VehicleRequirementDerivation.sysml:17: a requirement subject with a :> subsetting header."))
~~~
# SOURCE
~~~sysml
package VehicleRequirementDerivation {
    requirement def MassRequirement {
        subject mass :> ISQ::mass;
        attribute massLimit :> ISQ::mass;
        require constraint { mass <= massLimit }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_subject_vehicle_requirement_derivation.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package VehicleRequirementDerivation {
    requirement def MassRequirement {
        subject mass :> ISQ::mass;
        attribute massLimit :> ISQ::mass;
        require constraint {
            mass <= massLimit;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 101) (line 3) (column 25) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 101) (line 3) (column 25) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 106) (line 3) (column 30) (len 4)))))
  )
  (root (package (name "VehicleRequirementDerivation") (body brace (requirement-def (name "MassRequirement") (modifiers) (body brace (subject (name "mass") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r0)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage) (require-constraint))))))
)
~~~
