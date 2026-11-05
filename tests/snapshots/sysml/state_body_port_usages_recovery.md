# META
~~~sexpr
(snapshot (type recovery) (description "A malformed port member in a state body does not consume the subsequent port, nested state or following package."))
~~~
# SOURCE
~~~sysml
package StatePortRecovery {
    port def Signal;
    state def Controller {
        port broken : ;
        port valid : Signal;
        state running;
    }
}
package Later;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_body_port_usages_recovery.md"
    (diagnostics
      (diagnostic (code "missing_type_reference") (severity error) (category parseerror) (span (offset 84) (line 4) (column 9) (len 24)) (message "expected port type after ':'"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package StatePortRecovery {
    port def Signal;
    state def Controller {
        port broken : ;
        port valid : Signal;
        state running;
    }
}

package Later;
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 121) (line 5) (column 22) (len 6)) (segments (segment 0 (token "Signal") (name "Signal") (separator none) (span (offset 121) (line 5) (column 22) (len 6)))))
  )
  (root (package (visibility none) (name "StatePortRecovery") (body brace (port-def (name "Signal") (modifiers) (specializes none) (body semicolon)) (state-def (name "Controller") (modifiers) (body brace (malformed (code "missing_type_reference") (found "port broken : ;") (span (offset 84) (line 4) (column 9) (len 24))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "valid") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (state-usage (name "running") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))) (package (visibility none) (name "Later") (body semicolon)))
)
~~~
