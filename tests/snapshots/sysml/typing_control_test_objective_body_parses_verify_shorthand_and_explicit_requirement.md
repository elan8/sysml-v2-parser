# META
~~~sexpr
(snapshot (type semantic) (description "Preserves test_objective_body_parses_verify_shorthand_and_explicit_requirement through the canonical parser projection while replacing scalar typing fields."))
~~~
# SOURCE
~~~sysml
package P { use case def U { objective O { verify vehicleMassRequirement; verify requirement vehicleMassRequirement : VehicleMassRequirement; } } }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "typing_control_test_objective_body_parses_verify_shorthand_and_explicit_requirement.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    use case def U {
        objective O {
            verify vehicleMassRequirement;
            verify requirement vehicleMassRequirement : VehicleMassRequirement;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 50) (line 1) (column 51) (len 22)) (segments (segment 0 (token "vehicleMassRequirement") (name "vehicleMassRequirement") (separator none) (span (offset 50) (line 1) (column 51) (len 22)))))
    (reference r1 (scope relative) (span (offset 118) (line 1) (column 119) (len 22)) (segments (segment 0 (token "VehicleMassRequirement") (name "VehicleMassRequirement") (separator none) (span (offset 118) (line 1) (column 119) (len 22)))))
  )
  (root (package (visibility none) (name "P") (body brace (use-case-def (name "U") (modifiers) (body brace (objective (visibility none) (name "O") (type none) (body brace (verify (explicit-requirement false) (requirement none) (target (ref r0)) (redefines none)) (verify (explicit-requirement true) (requirement (name "vehicleMassRequirement") (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (body semicolon)) (target none) (redefines none)))))))))
)
~~~
