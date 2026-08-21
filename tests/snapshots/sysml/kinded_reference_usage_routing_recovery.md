# META
~~~sexpr
(snapshot (type recovery) (description "Malformed typed clauses on `ref action` and `ref state` recover at the following kinded usage in action, state, and requirement bodies. The retained siblings prove the new typed routing does not let a failed attempt consume a later member or strand arena-backed targets."))
~~~
# SOURCE
~~~sysml
package KindedReferenceUsageRoutingRecovery {
    action def ActionOwner {
        ref action malformedAction : ;
        ref action retainedAction : ActionType :>> priorAction;
    }
    state def StateOwner {
        ref state malformedState : ;
        ref state retainedState : StateType :>> priorState;
    }
    requirement def RequirementOwner {
        ref action malformedRequirement : ;
        ref action retainedRequirement : ActionType :>> priorRequirement;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kinded_reference_usage_routing_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 83) (line 3) (column 9) (len 39)) (message "unexpected token in action body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 83) (line 3) (column 9) (len 39)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
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
    (reference r0 (scope relative) (span (offset 150) (line 4) (column 37) (len 10)) (segments (segment 0 (token "ActionType") (name "ActionType") (separator none) (span (offset 150) (line 4) (column 37) (len 10)))))
    (reference r1 (scope relative) (span (offset 165) (line 4) (column 52) (len 11)) (segments (segment 0 (token "priorAction") (name "priorAction") (separator none) (span (offset 165) (line 4) (column 52) (len 11)))))
    (reference r2 (scope relative) (span (offset 282) (line 8) (column 35) (len 9)) (segments (segment 0 (token "StateType") (name "StateType") (separator none) (span (offset 282) (line 8) (column 35) (len 9)))))
    (reference r3 (scope relative) (span (offset 296) (line 8) (column 49) (len 10)) (segments (segment 0 (token "priorState") (name "priorState") (separator none) (span (offset 296) (line 8) (column 49) (len 10)))))
    (reference r4 (scope relative) (span (offset 438) (line 12) (column 42) (len 10)) (segments (segment 0 (token "ActionType") (name "ActionType") (separator none) (span (offset 438) (line 12) (column 42) (len 10)))))
    (reference r5 (scope relative) (span (offset 453) (line 12) (column 57) (len 16)) (segments (segment 0 (token "priorRequirement") (name "priorRequirement") (separator none) (span (offset 453) (line 12) (column 57) (len 16)))))
  )
  (root (package (name "KindedReferenceUsageRoutingRecovery") (body brace (action-def (name "ActionOwner") (modifiers) (specializes none) (body brace (malformed (code "recovered_action_body_element") (found "ref action malformedAction : ;") (span (offset 83) (line 3) (column 9) (len 39))) (action-usage (name "retainedAction") (short-name none) (prefix (abstract false) (variation false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (body semicolon)))) (state-def (name "StateOwner") (modifiers) (body brace (malformed (code "recovered_state_body_element") (found "ref state malformedState : ;") (span (offset 219) (line 7) (column 9) (len 37))) (state-usage (name "retainedState") (prefix (direction none) (derived false) (abstract false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (body semicolon)))) (requirement-def (name "RequirementOwner") (modifiers) (body brace (malformed (code "recovered_requirement_body_element") (found "ref action malformedRequirement : ;") (span (offset 361) (line 11) (column 9) (len 44))) (action-usage (name "retainedRequirement") (short-name none) (prefix (abstract false) (variation false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (body semicolon)))))))
)
~~~
