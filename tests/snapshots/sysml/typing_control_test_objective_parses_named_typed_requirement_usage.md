# META
~~~sexpr
(snapshot (type semantic) (description "Preserves test_objective_parses_named_typed_requirement_usage through the canonical parser projection while replacing scalar typing fields."))
~~~
# SOURCE
~~~sysml
package P { use case def U { objective missionObjective : MaximizeObjective; } }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "typing_control_test_objective_parses_named_typed_requirement_usage.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    use case def U {
        objective missionObjective  : MaximizeObjective;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 58) (line 1) (column 59) (len 17)) (segments (segment 0 (token "MaximizeObjective") (name "MaximizeObjective") (separator none) (span (offset 58) (line 1) (column 59) (len 17)))))
  )
  (root (package (name "P") (body brace (use-case-def (name "U") (modifiers) (body brace (objective (visibility none) (name "missionObjective") (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)))))))
)
~~~
