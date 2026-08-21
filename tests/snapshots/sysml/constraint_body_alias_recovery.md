# META
~~~sexpr
(snapshot (type recovery) (description "Constraint-body recovery synchronizes at AliasMember after malformed content, retaining the recovery node and the following typed alias in the shared CalculationBody of both constraint declaration forms."))
~~~
# SOURCE
~~~sysml
package ConstraintBodyAliasRecovery {
    constraint def RecoveringDefinition {
        nonsense ???;
        alias retainedDefinitionAlias for Target::definition;
    }
    constraint RecoveringUsage {
        broken ???;
        alias retainedUsageAlias for Target::usage;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "constraint_body_alias_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 88) (line 3) (column 9) (len 22)) (message "unrecognized declaration `nonsense` in constraint body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 211) (line 7) (column 9) (len 20)) (message "unrecognized declaration `broken` in constraint body"))
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
    (reference r0 (scope relative) (span (offset 144) (line 4) (column 43) (len 18)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 144) (line 4) (column 43) (len 6))) (segment 1 (token "definition") (name "definition") (separator colon-colon) (span (offset 152) (line 4) (column 51) (len 10)))))
    (reference r1 (scope relative) (span (offset 260) (line 8) (column 38) (len 13)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 260) (line 8) (column 38) (len 6))) (segment 1 (token "usage") (name "usage") (separator colon-colon) (span (offset 268) (line 8) (column 46) (len 5)))))
  )
  (root (package (name "ConstraintBodyAliasRecovery") (body brace (constraint-def (name "RecoveringDefinition") (modifiers) (specializes none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "nonsense ???;") (span (offset 88) (line 3) (column 9) (len 22))) (alias (name "retainedDefinitionAlias") (target (ref r0)) (body semicolon)))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "RecoveringUsage") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "broken ???;") (span (offset 211) (line 7) (column 9) (len 20))) (alias (name "retainedUsageAlias") (target (ref r1)) (body semicolon)))))))
)
~~~
