# META
~~~sexpr
(snapshot (type semantic) (description "Preserves test_objective_typed_semicolon_has_no_declared_name through the canonical parser projection while replacing scalar typing fields."))
~~~
# SOURCE
~~~sysml
package P { use case def U { objective : MaximizeObjective; } }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "typing_control_test_objective_typed_semicolon_has_no_declared_name.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    use case def U {
        objective  : MaximizeObjective;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 41) (line 1) (column 42) (len 17)) (segments (segment 0 (token "MaximizeObjective") (name "MaximizeObjective") (separator none) (span (offset 41) (line 1) (column 42) (len 17)))))
  )
  (root (package (visibility none) (name "P") (body brace (use-case-def (name "U") (modifiers) (body brace (objective (visibility none) (name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)))))))
)
~~~
