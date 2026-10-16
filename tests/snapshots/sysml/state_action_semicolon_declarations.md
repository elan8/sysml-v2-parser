# META
~~~sexpr
(snapshot (type semantic) (description "Spec42 287: entry/do/exit action NAME semicolon forms declare names; bare NAME forms keep action references, including quoted and qualified references."))
~~~
# SOURCE
~~~sysml
package E {
    action existing;
    state def S {
        entry; then a;
        state a {
            entry action transmit;
            do action blink;
            exit action stop;
        }
        state b {
            entry existing;
            do E::existing;
            exit 'existing';
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_action_semicolon_declarations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package E {
    action existing;
    state def S {
        entry;
        then a;
        state a {
            entry action transmit;
            do action blink;
            exit action stop;
        }
        state b {
            entry existing;
            do E::existing;
            exit 'existing';
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 71) (line 4) (column 21) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 71) (line 4) (column 21) (len 1)))))
    (reference r1 (scope relative) (span (offset 232) (line 11) (column 19) (len 8)) (segments (segment 0 (token "existing") (name "existing") (separator none) (span (offset 232) (line 11) (column 19) (len 8)))))
    (reference r2 (scope relative) (span (offset 257) (line 12) (column 16) (len 11)) (segments (segment 0 (token "E") (name "E") (separator none) (span (offset 257) (line 12) (column 16) (len 1))) (segment 1 (token "existing") (name "existing") (separator colon-colon) (span (offset 260) (line 12) (column 19) (len 8)))))
    (reference r3 (scope relative) (span (offset 287) (line 13) (column 18) (len 10)) (segments (segment 0 (token "'existing'") (name "existing") (separator none) (span (offset 287) (line 13) (column 18) (len 10)))))
  )
  (root (package (name "E") (body brace (action-usage (keyword action) (name "existing") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (state-def (name "S") (modifiers) (body brace (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (then (state (ref r0))) (state-usage (name "a") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (entry (action-keyword true) (target none) (declared-name "transmit") (type none) (redefines none) (effect false) (body semicolon)) (do (action-keyword true) (target none) (declared-name "blink") (type none) (redefines none) (effect false) (body semicolon)) (exit (action-keyword true) (target none) (declared-name "stop") (type none) (redefines none) (effect false) (body semicolon)))) (state-usage (name "b") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (entry (action-keyword false) (target (ref r1)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (do (action-keyword false) (target (ref r2)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (exit (action-keyword false) (target (ref r3)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)))))))))
)
~~~
