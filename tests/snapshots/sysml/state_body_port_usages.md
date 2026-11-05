# META
~~~sexpr
(snapshot (type semantic) (description "State definitions and state usages admit source-backed port members through StateBodyItem -> NonBehaviorBodyItem -> StructureUsageMember -> StructureUsageElement -> PortUsage; preserve visibility, conjugation, direction and nested members."))
~~~
# SOURCE
~~~sysml
package StatePorts {
    port def Signal { out attribute active : ScalarValues::Boolean; }
    state def Controller {
        private port input : ~Signal;
        out port output : Signal;
        state running {
            port nested : Signal { attribute enabled : ScalarValues::Boolean; }
        }
        entry; then running;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_body_port_usages.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package StatePorts {
    port def Signal {
        out attribute active : ScalarValues::Boolean;
    }
    state def Controller {
        private port input : ~Signal;
        out port output : Signal;
        state running {
            port nested : Signal {
                attribute enabled : ScalarValues::Boolean;
            }
        }
        entry;
        then running;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 66) (line 2) (column 46) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 66) (line 2) (column 46) (len 12))) (segment 1 (token "Boolean") (name "Boolean") (separator colon-colon) (span (offset 80) (line 2) (column 60) (len 7)))))
    (reference r1 (scope relative) (span (offset 148) (line 4) (column 31) (len 6)) (segments (segment 0 (token "Signal") (name "Signal") (separator none) (span (offset 148) (line 4) (column 31) (len 6)))))
    (reference r2 (scope relative) (span (offset 182) (line 5) (column 27) (len 6)) (segments (segment 0 (token "Signal") (name "Signal") (separator none) (span (offset 182) (line 5) (column 27) (len 6)))))
    (reference r3 (scope relative) (span (offset 240) (line 7) (column 27) (len 6)) (segments (segment 0 (token "Signal") (name "Signal") (separator none) (span (offset 240) (line 7) (column 27) (len 6)))))
    (reference r4 (scope relative) (span (offset 269) (line 7) (column 56) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 269) (line 7) (column 56) (len 12))) (segment 1 (token "Boolean") (name "Boolean") (separator colon-colon) (span (offset 283) (line 7) (column 70) (len 7)))))
    (reference r5 (scope relative) (span (offset 324) (line 9) (column 21) (len 7)) (segments (segment 0 (token "running") (name "running") (separator none) (span (offset 324) (line 9) (column 21) (len 7)))))
  )
  (root (package (visibility none) (name "StatePorts") (body brace (port-def (name "Signal") (modifiers) (specializes none) (body brace (attribute-usage (declaration-name "active") (direction out) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (state-def (name "Controller") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "input") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "output") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (state-usage (name "running") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "nested") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name "enabled") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (entry (action empty) (body semicolon)) (then (state (ref r5))))))))
)
~~~
