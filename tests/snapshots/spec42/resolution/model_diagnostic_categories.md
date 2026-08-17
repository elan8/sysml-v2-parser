# META
~~~sexpr
(snapshot (type semantic) (description "Category-owned diagnostics consume the frozen semantic publication"))
~~~
# SOURCE
~~~sysml
package Demo {
    part def Controller;
    part def Sensor;
    part controller : Controller;
    part sensor : Sensor;
    connect controller to sensor;

    action def Process;
    part process : Process;
    perform process;

    requirement def Requirement;
    part system;
    satisfy system;

    view def ArchitectureView;
    viewpoint def ArchitectureViewpoint;
    view architecture : ArchitectureView {
        satisfy ArchitectureViewpoint;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "model_diagnostic_categories.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Demo {
    part def Controller;
    part def Sensor;
    part controller : Controller;
    part sensor : Sensor;
    connect controller to sensor;
    action def Process;
    part process : Process;
    perform process;
    requirement def Requirement;
    part system;
    satisfy system by system;
    view def ArchitectureView;
    viewpoint def ArchitectureViewpoint;
    view architecture : ArchitectureView {
        satisfy ArchitectureViewpoint;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 83) (line 4) (column 23) (len 10)) (segments (segment 0 (token "Controller") (name "Controller") (separator none) (span (offset 83) (line 4) (column 23) (len 10)))))
    (reference r1 (scope relative) (span (offset 113) (line 5) (column 19) (len 6)) (segments (segment 0 (token "Sensor") (name "Sensor") (separator none) (span (offset 113) (line 5) (column 19) (len 6)))))
    (reference r2 (scope relative) (span (offset 133) (line 6) (column 13) (len 10)) (segments (segment 0 (token "controller") (name "controller") (separator none) (span (offset 133) (line 6) (column 13) (len 10)))))
    (reference r3 (scope relative) (span (offset 147) (line 6) (column 27) (len 6)) (segments (segment 0 (token "sensor") (name "sensor") (separator none) (span (offset 147) (line 6) (column 27) (len 6)))))
    (reference r4 (scope relative) (span (offset 199) (line 9) (column 20) (len 7)) (segments (segment 0 (token "Process") (name "Process") (separator none) (span (offset 199) (line 9) (column 20) (len 7)))))
    (reference r5 (scope relative) (span (offset 220) (line 10) (column 13) (len 7)) (segments (segment 0 (token "process") (name "process") (separator none) (span (offset 220) (line 10) (column 13) (len 7)))))
    (reference r6 (scope relative) (span (offset 397) (line 18) (column 25) (len 16)) (segments (segment 0 (token "ArchitectureView") (name "ArchitectureView") (separator none) (span (offset 397) (line 18) (column 25) (len 16)))))
    (reference r7 (scope relative) (span (offset 432) (line 19) (column 17) (len 21)) (segments (segment 0 (token "ArchitectureViewpoint") (name "ArchitectureViewpoint") (separator none) (span (offset 432) (line 19) (column 17) (len 21)))))
  )
  (root (package (name "Demo") (body brace (part-def (name "Controller") (body semicolon)) (part-def (name "Sensor") (body semicolon)) (part-usage (declaration-name "controller") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (part-usage (declaration-name "sensor") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (body semicolon)) (connect (from (expression (span (offset 133) (line 6) (column 13) (len 10)) (ref r2))) (to (expression (span (offset 147) (line 6) (column 27) (len 6)) (ref r3))) (body semicolon) (subsets none) (redefines none)) (action-def (name "Process") (specializes none) (body semicolon)) (part-usage (declaration-name "process") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (body semicolon)) (perform (declaration "") (action (ref r5)) (typing none) (subsets none) (redefines none) (body semicolon)) (requirement-def (name "Requirement") (body semicolon)) (part-usage (declaration-name "system") (typing none) (body semicolon)) (satisfy) (view-def) (viewpoint-def) (view (name "architecture") (type (ref r6)) (body brace (satisfy (viewpoint (ref r7)) (body semicolon)))))))
)
~~~
