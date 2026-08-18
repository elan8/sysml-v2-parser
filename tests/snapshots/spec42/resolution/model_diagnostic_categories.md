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
    satisfy system;
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
    (reference r6 (scope relative) (span (offset 292) (line 14) (column 13) (len 6)) (segments (segment 0 (token "system") (name "system") (separator none) (span (offset 292) (line 14) (column 13) (len 6)))))
    (reference r7 (scope relative) (span (offset 397) (line 18) (column 25) (len 16)) (segments (segment 0 (token "ArchitectureView") (name "ArchitectureView") (separator none) (span (offset 397) (line 18) (column 25) (len 16)))))
    (reference r8 (scope relative) (span (offset 432) (line 19) (column 17) (len 21)) (segments (segment 0 (token "ArchitectureViewpoint") (name "ArchitectureViewpoint") (separator none) (span (offset 432) (line 19) (column 17) (len 21)))))
  )
  (root (package (name "Demo") (body brace (part-def (name "Controller") (body semicolon)) (part-def (name "Sensor") (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "controller") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "sensor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (connect (from (expression (span (offset 133) (line 6) (column 13) (len 10)) (ref r2))) (to (expression (span (offset 147) (line 6) (column 27) (len 6)) (ref r3))) (body semicolon) (subsets none) (redefines none)) (action-def (name "Process") (specializes none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "process") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (perform (declaration "") (action (ref r5)) (typing none) (subsets none) (redefines none) (body semicolon)) (requirement-def (name "Requirement") (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "system") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r6))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)) (view-def (name "ArchitectureView") (short-name none) (modifiers) (specializes none) (body semicolon)) (viewpoint-def) (view (name "architecture") (short-name none) (type (ref r7)) (body brace (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r8))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)))))))
)
~~~
