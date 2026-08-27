# META
~~~sexpr
(snapshot (type semantic) (description "SysML 8.3.18.9 validates that a transition with trigger actions has a StateUsage source. The parser retains a complete transition in action definition and usage bodies so semantic consumers can diagnose the invalid owner/source, while the legal state-body spelling demonstrates the Pilot grammar's effect-before-target order. SysML 2.0 clauses 8.2.2.18.3 and 8.3.18.9; Pilot SysML.xtext 1818-1927."))
~~~
# SOURCE
~~~sysml
package ActionBodyTransitions {
    state def Machine {
        state idle;
        state running;
        action notify;
        transition first idle do notify then running;
    }
    action def Flow {
        action step;
        action done;
        transition first step accept when true then done;
    }
    action execution {
        action begin;
        action finish;
        transition first begin then finish;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "action_body_transition_invalid_owner.md"
    (diagnostics
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
    (reference r0 (scope relative) (span (offset 147) (line 6) (column 26) (len 4)) (segments (segment 0 (token "idle") (name "idle") (separator none) (span (offset 147) (line 6) (column 26) (len 4)))))
    (reference r1 (scope relative) (span (offset 155) (line 6) (column 34) (len 6)) (segments (segment 0 (token "notify") (name "notify") (separator none) (span (offset 155) (line 6) (column 34) (len 6)))))
    (reference r2 (scope relative) (span (offset 167) (line 6) (column 46) (len 7)) (segments (segment 0 (token "running") (name "running") (separator none) (span (offset 167) (line 6) (column 46) (len 7)))))
    (reference r3 (scope relative) (span (offset 271) (line 11) (column 26) (len 4)) (segments (segment 0 (token "step") (name "step") (separator none) (span (offset 271) (line 11) (column 26) (len 4)))))
    (reference r4 (scope relative) (span (offset 298) (line 11) (column 53) (len 4)) (segments (segment 0 (token "done") (name "done") (separator none) (span (offset 298) (line 11) (column 53) (len 4)))))
    (reference r5 (scope relative) (span (offset 403) (line 16) (column 26) (len 5)) (segments (segment 0 (token "begin") (name "begin") (separator none) (span (offset 403) (line 16) (column 26) (len 5)))))
    (reference r6 (scope relative) (span (offset 414) (line 16) (column 37) (len 6)) (segments (segment 0 (token "finish") (name "finish") (separator none) (span (offset 414) (line 16) (column 37) (len 6)))))
  )
  (root (package (name "ActionBodyTransitions") (body brace (state-def (name "Machine") (modifiers) (body brace (state-usage (name "idle") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (state-usage (name "running") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (keyword action) (name "notify") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source (expression (span (offset 147) (line 6) (column 26) (len 4)) (ref r0))) (initial true) (accept none) (guard none) (effect (expression (expression (span (offset 155) (line 6) (column 34) (len 6)) (ref r1)))) (target (expression (span (offset 167) (line 6) (column 46) (len 7)) (ref r2))) (body semicolon)))) (action-def (name "Flow") (modifiers) (specializes none) (body brace (action-usage (keyword action) (name "step") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (keyword action) (name "done") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source (expression (span (offset 271) (line 11) (column 26) (len 4)) (ref r3))) (initial true) (accept (time-trigger when (expression (span (offset 288) (line 11) (column 43) (len 4)) (boolean true)))) (guard none) (effect none) (target (expression (span (offset 298) (line 11) (column 53) (len 4)) (ref r4))) (body semicolon)))) (action-usage (keyword action) (name "execution") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (action-usage (keyword action) (name "begin") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (keyword action) (name "finish") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (transition (name none) (source (expression (span (offset 403) (line 16) (column 26) (len 5)) (ref r5))) (initial true) (accept none) (guard none) (effect none) (target (expression (span (offset 414) (line 16) (column 37) (len 6)) (ref r6))) (body semicolon)))))))
)
~~~
