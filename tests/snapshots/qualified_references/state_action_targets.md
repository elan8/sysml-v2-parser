# META
~~~sexpr
(snapshot (type provenance) (description "Verifies state entry, do, and exit action targets preserve qualified, dotted, quoted, and absolute paths as source-backed semantic references."))
~~~
# SOURCE
~~~sysml
package StateActionTargets {
    state def Machine {
        entry Actions::initialize;
        do Controller.'provide power' { out temp; }
        exit $::Safety::shutdown;
        entry;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_action_targets.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package StateActionTargets {
    state def Machine {
        entry Actions::initialize;
        do Controller.'provide power' {
            out temp;
        }
        exit $::Safety::shutdown;
        entry;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 67) (line 3) (column 15) (len 19)) (segments (segment 0 (token "Actions") (name "Actions") (separator none) (span (offset 67) (line 3) (column 15) (len 7))) (segment 1 (token "initialize") (name "initialize") (separator colon-colon) (span (offset 76) (line 3) (column 24) (len 10)))))
    (reference r1 (scope relative) (span (offset 99) (line 4) (column 12) (len 26)) (segments (segment 0 (token "Controller") (name "Controller") (separator none) (span (offset 99) (line 4) (column 12) (len 10))) (segment 1 (token "'provide power'") (name "provide power") (separator dot) (span (offset 110) (line 4) (column 23) (len 15)))))
    (reference r2 (scope absolute) (span (offset 153) (line 5) (column 14) (len 19)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 156) (line 5) (column 17) (len 6))) (segment 1 (token "shutdown") (name "shutdown") (separator colon-colon) (span (offset 164) (line 5) (column 25) (len 8)))))
  )
  (root (package (visibility none) (name "StateActionTargets") (body brace (state-def (name "Machine") (modifiers) (body brace (entry (action (reference (action (ref r0)) (specialization none)) (value none)) (body semicolon)) (do (action (reference (action (ref r1)) (specialization none)) (value none)) (body brace (in-out (direction out) (kind none) (reference false) (declaration "temp") (short-name none) (subsets none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 128) (line 4) (column 41) (len 9))))) (exit (action (reference (action (ref r2)) (specialization none)) (value none)) (body semicolon)) (entry (action empty) (body semicolon)))))))
)
~~~
