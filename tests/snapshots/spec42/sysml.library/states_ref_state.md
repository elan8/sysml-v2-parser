# META
~~~sexpr
(snapshot (type semantic) (description "Direct retained context from Systems Library/States.sysml: StateAction declares the typed, multi-target redefining `ref state self`, `start`, and `done` usages. They now retain StateUsage identity rather than routing through generic RefDecl."))
~~~
# SOURCE
~~~sysml
standard library package States {
    abstract state def StateAction :> Action, StatePerformance {
        ref state self: StateAction :>> Action::self, StatePerformance::self;
        ref state start: StateAction :>> Action::start, StatePerformance::startShot;
        ref state done: StateAction :>> Action::done, StatePerformance::endShot;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "states_ref_state.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package States {
    abstract state def StateAction :> Action, StatePerformance {
        ref state self : StateAction :>> Action::self, StatePerformance::self;
        ref state start : StateAction :>> Action::start, StatePerformance::startShot;
        ref state done : StateAction :>> Action::done, StatePerformance::endShot;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 123) (line 3) (column 25) (len 11)) (segments (segment 0 (token "StateAction") (name "StateAction") (separator none) (span (offset 123) (line 3) (column 25) (len 11)))))
    (reference r1 (scope relative) (span (offset 139) (line 3) (column 41) (len 12)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 139) (line 3) (column 41) (len 6))) (segment 1 (token "self") (name "self") (separator colon-colon) (span (offset 147) (line 3) (column 49) (len 4)))))
    (reference r2 (scope relative) (span (offset 153) (line 3) (column 55) (len 22)) (segments (segment 0 (token "StatePerformance") (name "StatePerformance") (separator none) (span (offset 153) (line 3) (column 55) (len 16))) (segment 1 (token "self") (name "self") (separator colon-colon) (span (offset 171) (line 3) (column 73) (len 4)))))
    (reference r3 (scope relative) (span (offset 202) (line 4) (column 26) (len 11)) (segments (segment 0 (token "StateAction") (name "StateAction") (separator none) (span (offset 202) (line 4) (column 26) (len 11)))))
    (reference r4 (scope relative) (span (offset 218) (line 4) (column 42) (len 13)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 218) (line 4) (column 42) (len 6))) (segment 1 (token "start") (name "start") (separator colon-colon) (span (offset 226) (line 4) (column 50) (len 5)))))
    (reference r5 (scope relative) (span (offset 233) (line 4) (column 57) (len 27)) (segments (segment 0 (token "StatePerformance") (name "StatePerformance") (separator none) (span (offset 233) (line 4) (column 57) (len 16))) (segment 1 (token "startShot") (name "startShot") (separator colon-colon) (span (offset 251) (line 4) (column 75) (len 9)))))
    (reference r6 (scope relative) (span (offset 286) (line 5) (column 25) (len 11)) (segments (segment 0 (token "StateAction") (name "StateAction") (separator none) (span (offset 286) (line 5) (column 25) (len 11)))))
    (reference r7 (scope relative) (span (offset 302) (line 5) (column 41) (len 12)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 302) (line 5) (column 41) (len 6))) (segment 1 (token "done") (name "done") (separator colon-colon) (span (offset 310) (line 5) (column 49) (len 4)))))
    (reference r8 (scope relative) (span (offset 316) (line 5) (column 55) (len 25)) (segments (segment 0 (token "StatePerformance") (name "StatePerformance") (separator none) (span (offset 316) (line 5) (column 55) (len 16))) (segment 1 (token "endShot") (name "endShot") (separator colon-colon) (span (offset 334) (line 5) (column 73) (len 7)))))
  )
  (root (library-package (name "States") (standard true) (body brace (state-def (name "StateAction") (modifiers (abstract (span (offset 38) (line 2) (column 5) (len 8)))) (body brace (state-usage (name "self") (prefix (direction none) (derived false) (abstract false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1) (ref r2)))) (body semicolon)) (state-usage (name "start") (prefix (direction none) (derived false) (abstract false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4) (ref r5)))) (body semicolon)) (state-usage (name "done") (prefix (direction none) (derived false) (abstract false) (reference true) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7) (ref r8)))) (body semicolon)))))))
)
~~~
