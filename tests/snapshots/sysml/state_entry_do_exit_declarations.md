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
  (references
    (reference r0 (scope relative) (span (offset 82) (line 3) (column 38) (len 7)) (segments (segment 0 (token "'entry'") (name "entry") (separator none) (span (offset 82) (line 3) (column 38) (len 7)))))
    (reference r1 (scope relative) (span (offset 120) (line 4) (column 30) (len 6)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 120) (line 4) (column 30) (len 6)))))
    (reference r2 (scope relative) (span (offset 131) (line 4) (column 41) (len 4)) (segments (segment 0 (token "'do'") (name "do") (separator none) (span (offset 131) (line 4) (column 41) (len 4)))))
    (reference r3 (scope relative) (span (offset 170) (line 5) (column 34) (len 6)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 170) (line 5) (column 34) (len 6)))))
    (reference r4 (scope relative) (span (offset 181) (line 5) (column 45) (len 6)) (segments (segment 0 (token "'exit'") (name "exit") (separator none) (span (offset 181) (line 5) (column 45) (len 6)))))
    (reference r5 (scope relative) (span (offset 346) (line 12) (column 15) (len 7)) (segments (segment 0 (token "initial") (name "initial") (separator none) (span (offset 346) (line 12) (column 15) (len 7)))))
    (reference r6 (scope relative) (span (offset 366) (line 13) (column 12) (len 19)) (segments (segment 0 (token "'sense temperature'") (name "sense temperature") (separator none) (span (offset 366) (line 13) (column 12) (len 19)))))
  )
  (root (package (name "StateEntryDoExit") (body (state-def (name "S") (body (entry (action-keyword true) (target none) (declared-name "entryAction") (type none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (effect false) (body semicolon)) (do (action-keyword true) (target none) (declared-name "doAction") (type (ref r1)) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (effect false) (body semicolon)) (exit (action-keyword true) (target none) (declared-name "exitAction") (type (ref r3)) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (effect false) (body semicolon)))) (state-def (name "T") (body (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect true) (body semicolon)) (do (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect true) (body semicolon)))) (state-def (name "U") (body (entry (action-keyword false) (target (ref r5)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (do (action-keyword false) (target (ref r6)) (declared-name none) (type none) (redefines none) (effect false) (body (doc))))))))
)
~~~
