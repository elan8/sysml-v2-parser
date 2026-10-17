# META
~~~sexpr
(snapshot (type semantic) (description "Apollo state subactions own ActionBody, preserving initial nodes, nested then action usages and done successions."))
~~~
# SOURCE
~~~sysml
package P {
state def PrepareForMissionPhase :> Phase {
do action prepareForMissionPhaseOperations {
first start;
then action loadConsumablesAndPropellants : LoadConsumablesAndPropellants;
then done;
}
}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_action_apollo_control_flow.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    state def PrepareForMissionPhase :> Phase {
        do action prepareForMissionPhaseOperations {
            first start;
            then action loadConsumablesAndPropellants : LoadConsumablesAndPropellants;
            then done;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 107) (line 4) (column 7) (len 5)) (segments (segment 0 (token "start") (name "start") (separator none) (span (offset 107) (line 4) (column 7) (len 5)))))
  )
  (root (package (name "P") (body brace (state-def (name "PrepareForMissionPhase") (modifiers) (body brace (do (action (action (name "prepareForMissionPhaseOperations") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none)) (body brace (first (source (expression (span (offset 107) (line 4) (column 7) (len 5)) (ref r0))) (target none) (body semicolon (span (span (offset 112) (line 4) (column 12) (len 1))))) (then-action) (then-action))))))))
)
~~~
