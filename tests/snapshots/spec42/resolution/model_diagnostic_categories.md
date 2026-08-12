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
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 212) (line 10) (column 5) (len 22)) (message "unexpected keyword `perform` in package body"))
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
    (reference r0 (scope relative) (span (offset 133) (line 6) (column 13) (len 10)) (segments (segment 0 (token "controller") (name "controller") (separator none) (span (offset 133) (line 6) (column 13) (len 10)))))
    (reference r1 (scope relative) (span (offset 147) (line 6) (column 27) (len 6)) (segments (segment 0 (token "sensor") (name "sensor") (separator none) (span (offset 147) (line 6) (column 27) (len 6)))))
    (reference r2 (scope relative) (span (offset 397) (line 18) (column 25) (len 16)) (segments (segment 0 (token "ArchitectureView") (name "ArchitectureView") (separator none) (span (offset 397) (line 18) (column 25) (len 16)))))
    (reference r3 (scope relative) (span (offset 432) (line 19) (column 17) (len 21)) (segments (segment 0 (token "ArchitectureViewpoint") (name "ArchitectureViewpoint") (separator none) (span (offset 432) (line 19) (column 17) (len 21)))))
  )
  (root (package (name "Demo") (body (part-def (name "Controller") (body semicolon)) (part-def (name "Sensor") (body semicolon)) (part-usage) (part-usage) (connect (from (expression (span (offset 133) (line 6) (column 13) (len 10)) (ref r0))) (to (expression (span (offset 147) (line 6) (column 27) (len 6)) (ref r1))) (body semicolon) (subsets none) (redefines none)) (action-def) (part-usage) (malformed (code "unexpected_keyword_in_scope") (found "perform process;") (span (offset 212) (line 10) (column 5) (len 22))) (requirement-def (name "Requirement") (body semicolon)) (part-usage) (satisfy) (view-def) (viewpoint-def) (view (name "architecture") (type (ref r2)) (body (satisfy (viewpoint (ref r3))))))))
)
~~~
