# META
~~~sexpr
(snapshot (type malformed) (description "The action keyword selects a declaration and must not accept qualified or dotted reference targets; later state actions and package siblings survive."))
~~~
# SOURCE
~~~sysml
package P {
    state def S {
        entry action P::a;
        do action obj.a;
        exit action 'P'::'a';
        entry action retained;
    }
    action later;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_action_qualified_declarations_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_state_body_element") (severity error) (category parseerror) (span (offset 38) (line 3) (column 9) (len 27)) (message "unexpected token in state body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 38) (line 3) (column 9) (len 27)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
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
  (name-language sysml)
  (references
  )
  (root (package (name "P") (body brace (state-def (name "S") (modifiers) (body brace (malformed (code "recovered_state_body_element") (found "entry action P::a;") (span (offset 38) (line 3) (column 9) (len 27))) (malformed (code "recovered_state_body_element") (found "do action obj.a;") (span (offset 65) (line 4) (column 9) (len 25))) (malformed (code "recovered_state_body_element") (found "exit action 'P'::'a';") (span (offset 90) (line 5) (column 9) (len 30))) (entry (action-keyword true) (target none) (declared-name "retained") (type none) (redefines none) (effect false) (body semicolon)))) (action-usage (keyword action) (name "later") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))
)
~~~
