# META
~~~sexpr
(snapshot (type provenance) (description "A non-empty first/then control body retains its aggregate source span and exact opening and closing brace token spans."))
~~~
# SOURCE
~~~sysml
package ControlProvenance {
    part def Flow {
        first Actions::start then Actions::finish {
            out pin;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "first_merge_brace_provenance.md"
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
    (reference r0 (scope relative) (span (offset 62) (line 3) (column 15) (len 14)) (segments (segment 0 (token "Actions") (name "Actions") (separator none) (span (offset 62) (line 3) (column 15) (len 7))) (segment 1 (token "start") (name "start") (separator colon-colon) (span (offset 71) (line 3) (column 24) (len 5)))))
    (reference r1 (scope relative) (span (offset 82) (line 3) (column 35) (len 15)) (segments (segment 0 (token "Actions") (name "Actions") (separator none) (span (offset 82) (line 3) (column 35) (len 7))) (segment 1 (token "finish") (name "finish") (separator colon-colon) (span (offset 91) (line 3) (column 44) (len 6)))))
  )
  (root (package (name "ControlProvenance") (body (part-def (name "Flow") (body (first (source (expression (span (offset 62) (line 3) (column 15) (len 14)) (ref r0))) (target (expression (span (offset 82) (line 3) (column 35) (len 15)) (ref r1))) (body brace (span (span (offset 98) (line 3) (column 51) (len 32))) (open-brace (span (offset 98) (line 3) (column 51) (len 1))) (members (in-out (direction out) (declaration "pin") (type none) (redefines none) (value none) (span (offset 112) (line 4) (column 13) (len 8)))) (close-brace (span (offset 129) (line 5) (column 9) (len 1))))))))))
)
~~~
