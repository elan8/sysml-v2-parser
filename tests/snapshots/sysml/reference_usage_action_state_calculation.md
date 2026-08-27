# META
~~~sexpr
(snapshot (type semantic) (description "Pinned SysML ReferenceUsage accepts a complete typed `Redefinitions` relationship after typing in action, state, and calculation bodies. The action form proves comma-separated typing and redefinition targets; state and calculation bodies route the same generic `ref` form through their typed action-member parsers."))
~~~
# SOURCE
~~~sysml
package GenericReferenceUsageHeaders {
    action def ActionOwner {
        ref actionMessage : Message, Action :>> previousMessage, previousAction;
    }
    state def StateOwner {
        ref messageState[0..*] ordered : StateMessage :>> previousStateMessage;
    }
    calc def CalculationOwner {
        ref genericCalculation : CalculationMessage :>> previousCalculationMessage;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "reference_usage_action_state_calculation.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package GenericReferenceUsageHeaders {
    action def ActionOwner {
        ref actionMessage : Message, Action :>> previousMessage, previousAction;
    }
    state def StateOwner {
        ref messageState : StateMessage[0..*] ordered :>> previousStateMessage;
    }
    calc def CalculationOwner {
        ref genericCalculation : CalculationMessage :>> previousCalculationMessage;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 96) (line 3) (column 29) (len 7)) (segments (segment 0 (token "Message") (name "Message") (separator none) (span (offset 96) (line 3) (column 29) (len 7)))))
    (reference r1 (scope relative) (span (offset 105) (line 3) (column 38) (len 6)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 105) (line 3) (column 38) (len 6)))))
    (reference r2 (scope relative) (span (offset 116) (line 3) (column 49) (len 15)) (segments (segment 0 (token "previousMessage") (name "previousMessage") (separator none) (span (offset 116) (line 3) (column 49) (len 15)))))
    (reference r3 (scope relative) (span (offset 133) (line 3) (column 66) (len 14)) (segments (segment 0 (token "previousAction") (name "previousAction") (separator none) (span (offset 133) (line 3) (column 66) (len 14)))))
    (reference r4 (scope relative) (span (offset 223) (line 6) (column 42) (len 12)) (segments (segment 0 (token "StateMessage") (name "StateMessage") (separator none) (span (offset 223) (line 6) (column 42) (len 12)))))
    (reference r5 (scope relative) (span (offset 240) (line 6) (column 59) (len 20)) (segments (segment 0 (token "previousStateMessage") (name "previousStateMessage") (separator none) (span (offset 240) (line 6) (column 59) (len 20)))))
    (reference r6 (scope relative) (span (offset 333) (line 9) (column 34) (len 18)) (segments (segment 0 (token "CalculationMessage") (name "CalculationMessage") (separator none) (span (offset 333) (line 9) (column 34) (len 18)))))
    (reference r7 (scope relative) (span (offset 356) (line 9) (column 57) (len 26)) (segments (segment 0 (token "previousCalculationMessage") (name "previousCalculationMessage") (separator none) (span (offset 356) (line 9) (column 57) (len 26)))))
  )
  (root (package (name "GenericReferenceUsageHeaders") (body brace (action-def (name "ActionOwner") (modifiers) (specializes none) (body brace (ref (name "actionMessage") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0) (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2) (ref r3)))) (subsets none) (body semicolon)))) (state-def (name "StateOwner") (modifiers) (body brace (ref (name "messageState") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity (lower (expression (span (offset 207) (line 6) (column 26) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (subsets none) (body semicolon)))) (calc-def (name "CalculationOwner") (modifiers) (body brace (ref (name "genericCalculation") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (subsets none) (body semicolon)))))))
)
~~~
