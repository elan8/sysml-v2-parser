# META
~~~sexpr
(snapshot (type provenance) (description "Verifies state entry, do, and exit action targets preserve qualified, dotted, quoted, and absolute paths as source-backed semantic references."))
~~~
# SOURCE
~~~sysml
package StateActionTargets {
    state def Machine {
        entry action Actions::initialize;
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
        entry action Actions::initialize;
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
  (references
    (reference r0 (scope relative) (span (offset 74) (line 3) (column 22) (len 19)) (segments (segment 0 (token "Actions") (name "Actions") (separator none) (span (offset 74) (line 3) (column 22) (len 7))) (segment 1 (token "initialize") (name "initialize") (separator colon-colon) (span (offset 83) (line 3) (column 31) (len 10)))))
    (reference r1 (scope relative) (span (offset 106) (line 4) (column 12) (len 26)) (segments (segment 0 (token "Controller") (name "Controller") (separator none) (span (offset 106) (line 4) (column 12) (len 10))) (segment 1 (token "'provide power'") (name "provide power") (separator dot) (span (offset 117) (line 4) (column 23) (len 15)))))
    (reference r2 (scope absolute) (span (offset 160) (line 5) (column 14) (len 19)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 163) (line 5) (column 17) (len 6))) (segment 1 (token "shutdown") (name "shutdown") (separator colon-colon) (span (offset 171) (line 5) (column 25) (len 8)))))
  )
  (root (package (name "StateActionTargets") (body (state-def (name "Machine") (body (entry (action-keyword true) (target (ref r0)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (do (action-keyword false) (target (ref r1)) (declared-name none) (type none) (redefines none) (effect false) (body (inout-declaration))) (exit (action-keyword false) (target (ref r2)) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)) (entry (action-keyword false) (target none) (declared-name none) (type none) (redefines none) (effect false) (body semicolon)))))))
)
~~~
