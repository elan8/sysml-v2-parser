# META
~~~sexpr
(snapshot (type recovery) (description "Interface-definition recovery synchronizes at ConstraintUsage's owning `constraint` FIRST token. Malformed members before and between legal constraint usages remain explicit recovery nodes, while both the typed semicolon form and anonymous CalculationBody form are retained as typed siblings (SysML textual BNF 724-750 and 1382-1395; pinned Pilot SysML agrees)."))
~~~
# SOURCE
~~~sysml
package InterfaceDefConstraintUsageRecovery {
    interface def CheckedInterface {
        nonsense ???;
        constraint later : Limit;
        broken ???;
        constraint {
            1 == 1;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "interface_def_constraint_usage_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 91) (line 3) (column 9) (len 22)) (message "unrecognized declaration `nonsense` in interface definition body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 147) (line 5) (column 9) (len 20)) (message "unrecognized declaration `broken` in interface definition body"))
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
    (reference r0 (scope relative) (span (offset 132) (line 4) (column 28) (len 5)) (segments (segment 0 (token "Limit") (name "Limit") (separator none) (span (offset 132) (line 4) (column 28) (len 5)))))
  )
  (root (package (name "InterfaceDefConstraintUsageRecovery") (body brace (interface-def (name "CheckedInterface") (modifiers) (specializes none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "nonsense ???;") (span (offset 91) (line 3) (column 9) (len 22))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "later") (short-name none) (type (ref r0)) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (malformed (code "unrecognized_declaration_in_scope") (found "broken ???;") (span (offset 147) (line 5) (column 9) (len 20))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (expression (span (offset 192) (line 7) (column 13) (len 6)) (binary (operator "==") (left (expression (span (offset 192) (line 7) (column 13) (len 1)) (integer 1))) (right (expression (span (offset 197) (line 7) (column 18) (len 1)) (integer 1))))))))))))
)
~~~
