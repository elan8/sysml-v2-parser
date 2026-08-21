# META
~~~sexpr
(snapshot (type semantic) (description "State def bodies accept general usage members: attribute redefinitions, anonymous action redefinitions, per-end-multiplicity successions, and assert constraints (Systems Library States.sysml; spec42 Gap 42)."))
~~~
# SOURCE
~~~sysml
package StateBodyMembers {
    state def S {
        attribute :>> isTriggerDuring;
        action :>> subactions :> middle;
        action substates : StateAction [0..*] :> stateActions, subactions;
        succession stateSequencing first [0..1] exclusiveStates then [0..1] exclusiveStates;
        assert constraint { notEmpty(exclusiveStates) }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_body_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package StateBodyMembers {
    state def S {
        attribute :>> isTriggerDuring;
        action :> middle :>> subactions;
        action substates : StateAction[0..*] :> stateActions, subactions;
        succession stateSequencing first [0..1] exclusiveStates then [0..1] exclusiveStates;
        assert constraint {
            notEmpty(exclusiveStates);
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 67) (line 3) (column 23) (len 15)) (segments (segment 0 (token "isTriggerDuring") (name "isTriggerDuring") (separator none) (span (offset 67) (line 3) (column 23) (len 15)))))
    (reference r1 (scope relative) (span (offset 117) (line 4) (column 34) (len 6)) (segments (segment 0 (token "middle") (name "middle") (separator none) (span (offset 117) (line 4) (column 34) (len 6)))))
    (reference r2 (scope relative) (span (offset 103) (line 4) (column 20) (len 10)) (segments (segment 0 (token "subactions") (name "subactions") (separator none) (span (offset 103) (line 4) (column 20) (len 10)))))
    (reference r3 (scope relative) (span (offset 152) (line 5) (column 28) (len 11)) (segments (segment 0 (token "StateAction") (name "StateAction") (separator none) (span (offset 152) (line 5) (column 28) (len 11)))))
    (reference r4 (scope relative) (span (offset 174) (line 5) (column 50) (len 12)) (segments (segment 0 (token "stateActions") (name "stateActions") (separator none) (span (offset 174) (line 5) (column 50) (len 12)))))
    (reference r5 (scope relative) (span (offset 188) (line 5) (column 64) (len 10)) (segments (segment 0 (token "subactions") (name "subactions") (separator none) (span (offset 188) (line 5) (column 64) (len 10)))))
  )
  (root (package (name "StateBodyMembers") (body brace (state-def (name "S") (modifiers) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (action-usage (name "") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (body semicolon)) (action-usage (name "substates") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity (lower (expression (span (offset 165) (line 5) (column 41) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r4) (ref r5)))) (redefines none) (body semicolon)) (succession-usage) (assert-constraint))))))
)
~~~
