# META
~~~sexpr
(snapshot (type semantic) (description "Issue 200 cases 5-6: StatePerformActionUsage retains complete anonymous, short-named, multiply typed and valued UsageDeclarations, and preserves empty and referenced alternatives; shared perform grammar remains typed."))
~~~
# SOURCE
~~~sysml
package StateDeclarations {
    action existing;
    action def A;
    action def B;
    state def ExistingControls {
        entry assign counter.count := 0;
        do action doAction : A :>> 'do';
        exit action entryAction :>> 'entry';
    }
    state def Named {
        entry action <e> initialize : A, B [1] = existing;
        do action ops = existing;
        exit action finish default = existing;
    }
    state def Anonymous {
        entry action : A;
        do action :>> existing;
        exit action :>> existing = existing;
    }
    state def Empty {
        entry;
        do action;
        exit action { first start; then done; }
    }
    state def References {
        entry existing = existing;
        do StateDeclarations::existing;
        exit existing redefines existing;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_action_usage_declaration.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package StateDeclarations {
    action existing;
    action def A;
    action def B;
    state def ExistingControls {
        entry assign counter.count := 0;
        do action doAction : A :>> 'do';
        exit action entryAction :>> 'entry';
    }
    state def Named {
        entry action <e> initialize : A, B[1] = existing;
        do action ops = existing;
        exit action finish default = existing;
    }
    state def Anonymous {
        entry action : A;
        do action :>> existing;
        exit action :>> existing = existing;
    }
    state def Empty {
        entry;
        do action;
        exit action {
            first start;
            then done;
        }
    }
    state def References {
        entry existing = existing;
        do StateDeclarations::existing;
        exit existing redefines existing;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 139) (line 6) (column 22) (len 7)) (segments (segment 0 (token "counter") (name "counter") (separator none) (span (offset 139) (line 6) (column 22) (len 7)))))
    (reference r1 (scope relative) (span (offset 147) (line 6) (column 30) (len 5)) (segments (segment 0 (token "count") (name "count") (separator none) (span (offset 147) (line 6) (column 30) (len 5)))))
    (reference r2 (scope relative) (span (offset 188) (line 7) (column 30) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 188) (line 7) (column 30) (len 1)))))
    (reference r3 (scope relative) (span (offset 194) (line 7) (column 36) (len 4)) (segments (segment 0 (token "'do'") (name "do") (separator none) (span (offset 194) (line 7) (column 36) (len 4)))))
    (reference r4 (scope relative) (span (offset 236) (line 8) (column 37) (len 7)) (segments (segment 0 (token "'entry'") (name "entry") (separator none) (span (offset 236) (line 8) (column 37) (len 7)))))
    (reference r5 (scope relative) (span (offset 311) (line 11) (column 39) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 311) (line 11) (column 39) (len 1)))))
    (reference r6 (scope relative) (span (offset 314) (line 11) (column 42) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 314) (line 11) (column 42) (len 1)))))
    (reference r7 (scope relative) (span (offset 322) (line 11) (column 50) (len 8)) (segments (segment 0 (token "existing") (name "existing") (separator none) (span (offset 322) (line 11) (column 50) (len 8)))))
    (reference r8 (scope relative) (span (offset 356) (line 12) (column 25) (len 8)) (segments (segment 0 (token "existing") (name "existing") (separator none) (span (offset 356) (line 12) (column 25) (len 8)))))
    (reference r9 (scope relative) (span (offset 403) (line 13) (column 38) (len 8)) (segments (segment 0 (token "existing") (name "existing") (separator none) (span (offset 403) (line 13) (column 38) (len 8)))))
    (reference r10 (scope relative) (span (offset 468) (line 16) (column 24) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 468) (line 16) (column 24) (len 1)))))
    (reference r11 (scope relative) (span (offset 493) (line 17) (column 23) (len 8)) (segments (segment 0 (token "existing") (name "existing") (separator none) (span (offset 493) (line 17) (column 23) (len 8)))))
    (reference r12 (scope relative) (span (offset 527) (line 18) (column 25) (len 8)) (segments (segment 0 (token "existing") (name "existing") (separator none) (span (offset 527) (line 18) (column 25) (len 8)))))
    (reference r13 (scope relative) (span (offset 538) (line 18) (column 36) (len 8)) (segments (segment 0 (token "existing") (name "existing") (separator none) (span (offset 538) (line 18) (column 36) (len 8)))))
    (reference r14 (scope relative) (span (offset 638) (line 23) (column 29) (len 5)) (segments (segment 0 (token "start") (name "start") (separator none) (span (offset 638) (line 23) (column 29) (len 5)))))
    (reference r15 (scope relative) (span (offset 705) (line 26) (column 15) (len 8)) (segments (segment 0 (token "existing") (name "existing") (separator none) (span (offset 705) (line 26) (column 15) (len 8)))))
    (reference r16 (scope relative) (span (offset 716) (line 26) (column 26) (len 8)) (segments (segment 0 (token "existing") (name "existing") (separator none) (span (offset 716) (line 26) (column 26) (len 8)))))
    (reference r17 (scope relative) (span (offset 737) (line 27) (column 12) (len 27)) (segments (segment 0 (token "StateDeclarations") (name "StateDeclarations") (separator none) (span (offset 737) (line 27) (column 12) (len 17))) (segment 1 (token "existing") (name "existing") (separator colon-colon) (span (offset 756) (line 27) (column 31) (len 8)))))
    (reference r18 (scope relative) (span (offset 779) (line 28) (column 14) (len 8)) (segments (segment 0 (token "existing") (name "existing") (separator none) (span (offset 779) (line 28) (column 14) (len 8)))))
    (reference r19 (scope relative) (span (offset 798) (line 28) (column 33) (len 8)) (segments (segment 0 (token "existing") (name "existing") (separator none) (span (offset 798) (line 28) (column 33) (len 8)))))
  )
  (root (package (name "StateDeclarations") (body brace (action-usage (keyword action) (name "existing") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-def (name "A") (modifiers) (specializes none) (body semicolon)) (action-def (name "B") (modifiers) (specializes none) (body semicolon)) (state-def (name "ExistingControls") (modifiers) (body brace (entry (action (assign (lhs (expression (span (offset 139) (line 6) (column 22) (len 13)) (member-access (base (expression (span (offset 139) (line 6) (column 22) (len 7)) (ref r0))) (separator dot) (member (ref r1))))) (rhs (expression (span (offset 156) (line 6) (column 39) (len 1)) (integer 0))) (body none))) (body semicolon)) (do (action (action (name "doAction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none)) (value none)) (body semicolon)) (exit (action (action (name "entryAction") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none)) (value none)) (body semicolon)))) (state-def (name "Named") (modifiers) (body brace (entry (action (action (name "initialize") (short-name "e") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5) (ref r6)))) (multiplicity (lower none) (upper (expression (span (offset 317) (line 11) (column 45) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 322) (line 11) (column 50) (len 8)) (ref r7)))))) (body semicolon)) (do (action (action (name "ops") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 356) (line 12) (column 25) (len 8)) (ref r8)))))) (body semicolon)) (exit (action (action (name "finish") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 403) (line 13) (column 38) (len 8)) (ref r9)))))) (body semicolon)))) (state-def (name "Anonymous") (modifiers) (body brace (entry (action (action (name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none)) (body semicolon)) (do (action (action (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (references none) (crosses none) (intersects none)) (value none)) (body semicolon)) (exit (action (action (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 538) (line 18) (column 36) (len 8)) (ref r13)))))) (body semicolon)))) (state-def (name "Empty") (modifiers) (body brace (entry (action empty) (body semicolon)) (do (action (action (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none)) (body semicolon)) (exit (action (action (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none)) (body brace (first (source (expression (span (offset 638) (line 23) (column 29) (len 5)) (ref r14))) (target none) (body semicolon (span (span (offset 643) (line 23) (column 34) (len 1))))) (then-action))))) (state-def (name "References") (modifiers) (body brace (entry (action (reference (action (ref r15)) (specialization none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 716) (line 26) (column 26) (len 8)) (ref r16)))))) (body semicolon)) (do (action (reference (action (ref r17)) (specialization none)) (value none)) (body semicolon)) (exit (action (reference (action (ref r18)) (specialization (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19)))) (references none) (crosses none) (intersects none))) (value none)) (body semicolon)))))))
)
~~~
