# META
~~~sexpr
(snapshot (type semantic) (description "Pinned SysML ActionUsage and StateUsage accept their complete `ref action` and `ref state` spellings ahead of generic ReferenceUsage in action-definition, action-usage, state, package, and requirement bodies. The shared projections retain reference prefixes, typing, multiplicity and authored modifier slots, subsets/redefinitions, and bodies."))
~~~
# SOURCE
~~~sysml
package KindedReferenceUsageRouting {
    action def DefinitionOwner {
        ref action definitionAction : ActionType[0..*] ordered nonunique :> actionSubset :>> priorAction;
        ref state definitionState : StateType[1] nonunique :>> priorState;
    }
    action UsageOwner : ActionType {
        ref action usageAction : ActionType :>> priorUsageAction;
        ref state usageState : StateType :>> priorUsageState;
    }
    state def StateOwner {
        ref action stateAction : ActionType :>> priorStateAction;
        ref state stateState : StateType[1] ordered :>> priorStateState;
    }
    ref action packageAction : ActionType :>> priorPackageAction;
    ref state packageState : StateType :>> priorPackageState;
    requirement def RequirementOwner {
        ref action requirementAction : ActionType :>> priorRequirementAction;
        ref state requirementState : StateType[1] nonunique :>> priorRequirementState;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kinded_reference_usage_routing.md"
    (diagnostics
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
    (reference r0 (scope relative) (span (offset 109) (line 3) (column 39) (len 10)) (segments (segment 0 (token "ActionType") (name "ActionType") (separator none) (span (offset 109) (line 3) (column 39) (len 10)))))
    (reference r1 (scope relative) (span (offset 147) (line 3) (column 77) (len 12)) (segments (segment 0 (token "actionSubset") (name "actionSubset") (separator none) (span (offset 147) (line 3) (column 77) (len 12)))))
    (reference r2 (scope relative) (span (offset 164) (line 3) (column 94) (len 11)) (segments (segment 0 (token "priorAction") (name "priorAction") (separator none) (span (offset 164) (line 3) (column 94) (len 11)))))
    (reference r3 (scope relative) (span (offset 213) (line 4) (column 37) (len 9)) (segments (segment 0 (token "StateType") (name "StateType") (separator none) (span (offset 213) (line 4) (column 37) (len 9)))))
    (reference r4 (scope relative) (span (offset 240) (line 4) (column 64) (len 10)) (segments (segment 0 (token "priorState") (name "priorState") (separator none) (span (offset 240) (line 4) (column 64) (len 10)))))
    (reference r5 (scope relative) (span (offset 282) (line 6) (column 25) (len 10)) (segments (segment 0 (token "ActionType") (name "ActionType") (separator none) (span (offset 282) (line 6) (column 25) (len 10)))))
    (reference r6 (scope relative) (span (offset 328) (line 7) (column 34) (len 10)) (segments (segment 0 (token "ActionType") (name "ActionType") (separator none) (span (offset 328) (line 7) (column 34) (len 10)))))
    (reference r7 (scope relative) (span (offset 343) (line 7) (column 49) (len 16)) (segments (segment 0 (token "priorUsageAction") (name "priorUsageAction") (separator none) (span (offset 343) (line 7) (column 49) (len 16)))))
    (reference r8 (scope relative) (span (offset 392) (line 8) (column 32) (len 9)) (segments (segment 0 (token "StateType") (name "StateType") (separator none) (span (offset 392) (line 8) (column 32) (len 9)))))
    (reference r9 (scope relative) (span (offset 406) (line 8) (column 46) (len 15)) (segments (segment 0 (token "priorUsageState") (name "priorUsageState") (separator none) (span (offset 406) (line 8) (column 46) (len 15)))))
    (reference r10 (scope relative) (span (offset 489) (line 11) (column 34) (len 10)) (segments (segment 0 (token "ActionType") (name "ActionType") (separator none) (span (offset 489) (line 11) (column 34) (len 10)))))
    (reference r11 (scope relative) (span (offset 504) (line 11) (column 49) (len 16)) (segments (segment 0 (token "priorStateAction") (name "priorStateAction") (separator none) (span (offset 504) (line 11) (column 49) (len 16)))))
    (reference r12 (scope relative) (span (offset 553) (line 12) (column 32) (len 9)) (segments (segment 0 (token "StateType") (name "StateType") (separator none) (span (offset 553) (line 12) (column 32) (len 9)))))
    (reference r13 (scope relative) (span (offset 578) (line 12) (column 57) (len 15)) (segments (segment 0 (token "priorStateState") (name "priorStateState") (separator none) (span (offset 578) (line 12) (column 57) (len 15)))))
    (reference r14 (scope relative) (span (offset 632) (line 14) (column 32) (len 10)) (segments (segment 0 (token "ActionType") (name "ActionType") (separator none) (span (offset 632) (line 14) (column 32) (len 10)))))
    (reference r15 (scope relative) (span (offset 647) (line 14) (column 47) (len 18)) (segments (segment 0 (token "priorPackageAction") (name "priorPackageAction") (separator none) (span (offset 647) (line 14) (column 47) (len 18)))))
    (reference r16 (scope relative) (span (offset 696) (line 15) (column 30) (len 9)) (segments (segment 0 (token "StateType") (name "StateType") (separator none) (span (offset 696) (line 15) (column 30) (len 9)))))
    (reference r17 (scope relative) (span (offset 710) (line 15) (column 44) (len 17)) (segments (segment 0 (token "priorPackageState") (name "priorPackageState") (separator none) (span (offset 710) (line 15) (column 44) (len 17)))))
    (reference r18 (scope relative) (span (offset 807) (line 17) (column 40) (len 10)) (segments (segment 0 (token "ActionType") (name "ActionType") (separator none) (span (offset 807) (line 17) (column 40) (len 10)))))
    (reference r19 (scope relative) (span (offset 822) (line 17) (column 55) (len 22)) (segments (segment 0 (token "priorRequirementAction") (name "priorRequirementAction") (separator none) (span (offset 822) (line 17) (column 55) (len 22)))))
    (reference r20 (scope relative) (span (offset 883) (line 18) (column 38) (len 9)) (segments (segment 0 (token "StateType") (name "StateType") (separator none) (span (offset 883) (line 18) (column 38) (len 9)))))
    (reference r21 (scope relative) (span (offset 910) (line 18) (column 65) (len 21)) (segments (segment 0 (token "priorRequirementState") (name "priorRequirementState") (separator none) (span (offset 910) (line 18) (column 65) (len 21)))))
  )
  (root (package (name "KindedReferenceUsageRouting") (body brace (action-def (name "DefinitionOwner") (modifiers) (specializes none) (body brace (action-usage (keyword action) (name "definitionAction") (short-name none) (prefix (abstract false) (variation false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower (expression (span (offset 120) (line 3) (column 50) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (body semicolon)) (state-usage (name "definitionState") (prefix (direction none) (derived false) (abstract false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity (lower (expression (span (offset 223) (line 4) (column 47) (len 1)) (integer 1))) (upper (expression (span (offset 223) (line 4) (column 47) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (body semicolon)))) (action-usage (keyword action) (name "UsageOwner") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (action-usage (keyword action) (name "usageAction") (short-name none) (prefix (abstract false) (variation false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (body semicolon)) (state-usage (name "usageState") (prefix (direction none) (derived false) (abstract false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (body semicolon)))) (state-def (name "StateOwner") (modifiers) (body brace (action-usage (keyword action) (name "stateAction") (short-name none) (prefix (abstract false) (variation false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (body semicolon)) (state-usage (name "stateState") (prefix (direction none) (derived false) (abstract false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity (lower (expression (span (offset 563) (line 12) (column 42) (len 1)) (integer 1))) (upper (expression (span (offset 563) (line 12) (column 42) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (body semicolon)))) (action-usage (keyword action) (name "packageAction") (short-name none) (prefix (abstract false) (variation false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (body semicolon)) (state-usage (name "packageState") (prefix (direction none) (derived false) (abstract false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (body semicolon)) (requirement-def (name "RequirementOwner") (modifiers) (body brace (action-usage (keyword action) (name "requirementAction") (short-name none) (prefix (abstract false) (variation false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19)))) (body semicolon)) (state-usage (name "requirementState") (prefix (direction none) (derived false) (abstract false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity (lower (expression (span (offset 893) (line 18) (column 48) (len 1)) (integer 1))) (upper (expression (span (offset 893) (line 18) (column 48) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r21)))) (body semicolon)))))))
)
~~~
