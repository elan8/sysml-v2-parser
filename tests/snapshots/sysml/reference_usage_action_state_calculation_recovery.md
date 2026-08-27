# META
~~~sexpr
(snapshot (type recovery) (description "Malformed generic ReferenceUsage redefinitions in action, state, and calculation bodies recover at the following `ref` sibling. Each retained sibling proves that body-specific dispatch and the calculation recovery FIRST set preserve a later typed redefinition rather than swallowing it."))
~~~
# SOURCE
~~~sysml
package GenericReferenceUsageRecovery {
    action def ActionOwner {
        ref incompleteAction : Action :>> ;
        ref retainedAction : Action :>> previousAction;
    }
    state def StateOwner {
        ref incompleteStateRef : State :>> ;
        ref retainedStateRef : State :>> previousState;
    }
    calc def CalculationOwner {
        ref incompleteGeneric : Calculation :>> ;
        ref retainedGeneric : Calculation :>> previousCalculation;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "reference_usage_action_state_calculation_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 77) (line 3) (column 9) (len 44)) (message "unexpected token in action body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 77) (line 3) (column 9) (len 44)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
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
    (reference r0 (scope relative) (span (offset 142) (line 4) (column 30) (len 6)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 142) (line 4) (column 30) (len 6)))))
    (reference r1 (scope relative) (span (offset 153) (line 4) (column 41) (len 14)) (segments (segment 0 (token "previousAction") (name "previousAction") (separator none) (span (offset 153) (line 4) (column 41) (len 14)))))
    (reference r2 (scope relative) (span (offset 278) (line 8) (column 32) (len 5)) (segments (segment 0 (token "State") (name "State") (separator none) (span (offset 278) (line 8) (column 32) (len 5)))))
    (reference r3 (scope relative) (span (offset 288) (line 8) (column 42) (len 13)) (segments (segment 0 (token "previousState") (name "previousState") (separator none) (span (offset 288) (line 8) (column 42) (len 13)))))
    (reference r4 (scope relative) (span (offset 421) (line 12) (column 31) (len 11)) (segments (segment 0 (token "Calculation") (name "Calculation") (separator none) (span (offset 421) (line 12) (column 31) (len 11)))))
    (reference r5 (scope relative) (span (offset 437) (line 12) (column 47) (len 19)) (segments (segment 0 (token "previousCalculation") (name "previousCalculation") (separator none) (span (offset 437) (line 12) (column 47) (len 19)))))
  )
  (root (package (name "GenericReferenceUsageRecovery") (body brace (action-def (name "ActionOwner") (modifiers) (specializes none) (body brace (malformed (code "recovered_action_body_element") (found "ref incompleteAction : Action :>> ;") (span (offset 77) (line 3) (column 9) (len 44))) (ref (name "retainedAction") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (subsets none) (body semicolon)))) (state-def (name "StateOwner") (modifiers) (body brace (malformed (code "recovered_state_body_element") (found "ref incompleteStateRef : State :>> ;") (span (offset 210) (line 7) (column 9) (len 45))) (ref (name "retainedStateRef") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (subsets none) (body semicolon)))) (calc-def (name "CalculationOwner") (modifiers) (body brace (malformed (code "recovered_calc_body_element") (found "ref incompleteGeneric : Calculation :>> ;") (span (offset 349) (line 11) (column 9) (len 50))) (ref (name "retainedGeneric") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (subsets none) (body semicolon)))))))
)
~~~
