# META
~~~sexpr
(snapshot (type semantic) (description "Entry/do/exit actions support declaring a new named, typed, or redefining nested action, and assign/send/accept effects written directly under the keyword, alongside the existing reference form (spec42 Gap 43)."))
~~~
# SOURCE
~~~sysml
package StateEntryDoExit {
    state def S {
        entry action entryAction :>> 'entry';
        do action doAction : Action :>> 'do';
        exit action exitAction : Action :>> 'exit';
    }
    state def T {
        entry assign counter.count := 0;
        do assign counter.count := counter.count + 1;
    }
    state def U {
        entry initial;
        do 'sense temperature' {
            doc /* reference form keeps its shape */
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_entry_do_exit_declarations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package StateEntryDoExit {
    state def S {
        entry action entryAction :>> 'entry';
        do action doAction : Action :>> 'do';
        exit action exitAction : Action :>> 'exit';
    }
    state def T {
        entry assign counter.count := 0;
        do assign counter.count := counter.count + 1;
    }
    state def U {
        entry initial;
        do 'sense temperature' {
            doc
            /* reference form keeps its shape */
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 82) (line 3) (column 38) (len 7)) (segments (segment 0 (token "'entry'") (name "entry") (separator none) (span (offset 82) (line 3) (column 38) (len 7)))))
    (reference r1 (scope relative) (span (offset 120) (line 4) (column 30) (len 6)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 120) (line 4) (column 30) (len 6)))))
    (reference r2 (scope relative) (span (offset 131) (line 4) (column 41) (len 4)) (segments (segment 0 (token "'do'") (name "do") (separator none) (span (offset 131) (line 4) (column 41) (len 4)))))
    (reference r3 (scope relative) (span (offset 170) (line 5) (column 34) (len 6)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 170) (line 5) (column 34) (len 6)))))
    (reference r4 (scope relative) (span (offset 181) (line 5) (column 45) (len 6)) (segments (segment 0 (token "'exit'") (name "exit") (separator none) (span (offset 181) (line 5) (column 45) (len 6)))))
    (reference r5 (scope relative) (span (offset 234) (line 8) (column 22) (len 7)) (segments (segment 0 (token "counter") (name "counter") (separator none) (span (offset 234) (line 8) (column 22) (len 7)))))
    (reference r6 (scope relative) (span (offset 242) (line 8) (column 30) (len 5)) (segments (segment 0 (token "count") (name "count") (separator none) (span (offset 242) (line 8) (column 30) (len 5)))))
    (reference r7 (scope relative) (span (offset 272) (line 9) (column 19) (len 7)) (segments (segment 0 (token "counter") (name "counter") (separator none) (span (offset 272) (line 9) (column 19) (len 7)))))
    (reference r8 (scope relative) (span (offset 280) (line 9) (column 27) (len 5)) (segments (segment 0 (token "count") (name "count") (separator none) (span (offset 280) (line 9) (column 27) (len 5)))))
    (reference r9 (scope relative) (span (offset 289) (line 9) (column 36) (len 7)) (segments (segment 0 (token "counter") (name "counter") (separator none) (span (offset 289) (line 9) (column 36) (len 7)))))
    (reference r10 (scope relative) (span (offset 297) (line 9) (column 44) (len 5)) (segments (segment 0 (token "count") (name "count") (separator none) (span (offset 297) (line 9) (column 44) (len 5)))))
    (reference r11 (scope relative) (span (offset 346) (line 12) (column 15) (len 7)) (segments (segment 0 (token "initial") (name "initial") (separator none) (span (offset 346) (line 12) (column 15) (len 7)))))
    (reference r12 (scope relative) (span (offset 366) (line 13) (column 12) (len 19)) (segments (segment 0 (token "'sense temperature'") (name "sense temperature") (separator none) (span (offset 366) (line 13) (column 12) (len 19)))))
  )
  (root (package (name "StateEntryDoExit") (body brace (state-def (name "S") (modifiers) (body brace (entry (action (action (name "entryAction") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (references none) (crosses none) (intersects none)) (value none)) (body semicolon)) (do (action (action (name "doAction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (intersects none)) (value none)) (body semicolon)) (exit (action (action (name "exitAction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none)) (value none)) (body semicolon)))) (state-def (name "T") (modifiers) (body brace (entry (action (assign (lhs (expression (span (offset 234) (line 8) (column 22) (len 13)) (member-access (base (expression (span (offset 234) (line 8) (column 22) (len 7)) (ref r5))) (separator dot) (member (ref r6))))) (rhs (expression (span (offset 251) (line 8) (column 39) (len 1)) (integer 0))) (body none))) (body semicolon)) (do (action (assign (lhs (expression (span (offset 272) (line 9) (column 19) (len 13)) (member-access (base (expression (span (offset 272) (line 9) (column 19) (len 7)) (ref r7))) (separator dot) (member (ref r8))))) (rhs (expression (span (offset 289) (line 9) (column 36) (len 17)) (binary (operator "+") (left (expression (span (offset 289) (line 9) (column 36) (len 13)) (member-access (base (expression (span (offset 289) (line 9) (column 36) (len 7)) (ref r9))) (separator dot) (member (ref r10))))) (right (expression (span (offset 305) (line 9) (column 52) (len 1)) (integer 1)))))) (body none))) (body semicolon)))) (state-def (name "U") (modifiers) (body brace (entry (action (reference (action (ref r11)) (redefines none)) (value none)) (body semicolon)) (do (action (reference (action (ref r12)) (redefines none)) (value none)) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 406) (line 14) (column 19) (len 32)) (normalized "reference form keeps its shape "))))))))))
)
~~~
