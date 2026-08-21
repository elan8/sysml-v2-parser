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
    (reference r0 (scope relative) (span (offset 89) (line 1) (column 90) (len 13)) (segments (segment 0 (token "validateRoute") (name "validateRoute") (separator none) (span (offset 89) (line 1) (column 90) (len 13)))))
    (reference r1 (scope relative) (span (offset 108) (line 1) (column 109) (len 12)) (segments (segment 0 (token "startMission") (name "startMission") (separator none) (span (offset 108) (line 1) (column 109) (len 12)))))
  )
  (root (package (name "P") (body brace (action-def (name "ExecuteMission") (modifiers) (specializes none) (body brace (action-usage (declaration "validateRoute") (type none)) (action-usage (declaration "startMission") (type none)) (first (source (expression (span (offset 89) (line 1) (column 90) (len 13)) (ref r0))) (target (expression (span (offset 108) (line 1) (column 109) (len 12)) (ref r1))) (body semicolon (span (span (offset 120) (line 1) (column 121) (len 1))))))))))
)
~~~
