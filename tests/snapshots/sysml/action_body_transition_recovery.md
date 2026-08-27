# META
~~~sexpr
(snapshot (type recovery) (description "A malformed transition in an action body is recovered atomically without leaking its speculative source reference or consuming the following valid typed transition and action sibling."))
~~~
# SOURCE
~~~sysml
package ActionBodyTransitionRecovery {
    action def Flow {
        transition first leaked accept when then ;
        transition first retained then target;
        action after;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "action_body_transition_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 69) (line 3) (column 9) (len 51)) (message "unexpected token in action body"))
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 137) (line 4) (column 26) (len 8)) (segments (segment 0 (token "retained") (name "retained") (separator none) (span (offset 137) (line 4) (column 26) (len 8)))))
    (reference r1 (scope relative) (span (offset 151) (line 4) (column 40) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 151) (line 4) (column 40) (len 6)))))
  )
  (root (package (name "ActionBodyTransitionRecovery") (body brace (action-def (name "Flow") (modifiers) (specializes none) (body brace (malformed (code "recovered_action_body_element") (found "transition first leaked accept when then ;") (span (offset 69) (line 3) (column 9) (len 51))) (transition (name none) (source (expression (span (offset 137) (line 4) (column 26) (len 8)) (ref r0))) (initial true) (accept none) (guard none) (effect none) (target (expression (span (offset 151) (line 4) (column 40) (len 6)) (ref r1))) (body semicolon)) (action-usage (keyword action) (name "after") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))))
)
~~~
