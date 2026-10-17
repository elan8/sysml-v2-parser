# META
~~~sexpr
(snapshot (type semantic) (description "Preserves test_use_case_def_body_parses_members through the canonical parser projection while replacing scalar typing fields."))
~~~
# SOURCE
~~~sysml
package P { use case def U { subject s : System; actor a : Operator; objective { } } }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "typing_control_test_use_case_def_body_parses_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    use case def U {
        subject s : System;
        actor a : Operator;
        objective {
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 41) (line 1) (column 42) (len 6)) (segments (segment 0 (token "System") (name "System") (separator none) (span (offset 41) (line 1) (column 42) (len 6)))))
    (reference r1 (scope relative) (span (offset 59) (line 1) (column 60) (len 8)) (segments (segment 0 (token "Operator") (name "Operator") (separator none) (span (offset 59) (line 1) (column 60) (len 8)))))
  )
  (root (package (name "P") (body brace (use-case-def (name "U") (modifiers) (body brace (subject (name "s") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (actor (name "a") (short-name none) (type (ref r1)) (redefines none) (multiplicity none)) (objective (visibility none) (name none) (type none) (body brace)))))))
)
~~~
