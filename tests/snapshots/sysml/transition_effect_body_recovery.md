# META
~~~sexpr
(snapshot (type recovery) (description "Recovery inside a transition effect ActionBody preserves the malformed member, synchronizes to the later typed action, and keeps the enclosing transition and later state sibling. SysML BNF 1324-1334; pinned Pilot EffectBehaviorUsage agrees."))
~~~
# SOURCE
~~~sysml
package TransitionEffectRecovery {
    state def S {
        transition accept signal do action {
            nonsense ???;
            action retained;
        } then Done;
        state Done;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "transition_effect_body_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 110) (line 4) (column 13) (len 26)) (message "unrecognized declaration `nonsense` in action body"))
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
    (reference r0 (scope relative) (span (offset 79) (line 3) (column 27) (len 6)) (segments (segment 0 (token "signal") (name "signal") (separator none) (span (offset 79) (line 3) (column 27) (len 6)))))
    (reference r1 (scope relative) (span (offset 168) (line 6) (column 16) (len 4)) (segments (segment 0 (token "Done") (name "Done") (separator none) (span (offset 168) (line 6) (column 16) (len 4)))))
  )
  (root (package (name "TransitionEffectRecovery") (body brace (state-def (name "S") (modifiers) (body brace (transition (name none) (source none) (initial false) (accept (shorthand (expression (span (offset 79) (line 3) (column 27) (len 6)) (ref r0)) (via none))) (guard none) (effect (perform (name none) (type none) (body (body brace (malformed (code "unrecognized_declaration_in_scope") (found "nonsense ???;") (span (offset 110) (line 4) (column 13) (len 26))) (action-usage (name "retained") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))) (target (expression (span (offset 168) (line 6) (column 16) (len 4)) (ref r1))) (body semicolon)) (state-usage (name "Done") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))))
)
~~~
