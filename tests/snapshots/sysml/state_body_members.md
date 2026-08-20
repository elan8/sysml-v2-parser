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
  )
  (root (package (name "StateBodyMembers") (body brace (state-def (name "S") (modifiers) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (action-usage) (action-usage) (succession-usage) (assert-constraint))))))
)
~~~
