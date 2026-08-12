# META
~~~sexpr
(snapshot (type semantic) (description "Generic flow builder endpoints resolve canonically"))
~~~
# SOURCE
~~~sysml
package P { action def ExecuteMission { action validateRoute; action startMission; first validateRoute then startMission; } }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "generic_flow_publication.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    action def ExecuteMission {
        action validateRoute;
        action startMission;
        first validateRoute then startMission;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "P") (body (action-def))))
)
~~~
