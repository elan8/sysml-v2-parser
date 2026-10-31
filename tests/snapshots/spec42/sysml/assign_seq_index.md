# META
~~~sexpr
(snapshot (type semantic) (description "Assign node with sequence indexing operator #()"))
~~~
# SOURCE
~~~sysml
package AssignTest {
    action def A {
        assign x := seq#(i);
        assign 'var' := data#(idx);
        assign a.b := items#(0);
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "assign_seq_index.md"
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 55) (line 3) (column 16) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 55) (line 3) (column 16) (len 1)))))
    (reference r1 (scope relative) (span (offset 60) (line 3) (column 21) (len 3)) (segments (segment 0 (token "seq") (name "seq") (separator none) (span (offset 60) (line 3) (column 21) (len 3)))))
    (reference r2 (scope relative) (span (offset 65) (line 3) (column 26) (len 1)) (segments (segment 0 (token "i") (name "i") (separator none) (span (offset 65) (line 3) (column 26) (len 1)))))
    (reference r3 (scope relative) (span (offset 84) (line 4) (column 16) (len 5)) (segments (segment 0 (token "'var'") (name "var") (separator none) (span (offset 84) (line 4) (column 16) (len 5)))))
    (reference r4 (scope relative) (span (offset 93) (line 4) (column 25) (len 4)) (segments (segment 0 (token "data") (name "data") (separator none) (span (offset 93) (line 4) (column 25) (len 4)))))
    (reference r5 (scope relative) (span (offset 99) (line 4) (column 31) (len 3)) (segments (segment 0 (token "idx") (name "idx") (separator none) (span (offset 99) (line 4) (column 31) (len 3)))))
    (reference r6 (scope relative) (span (offset 120) (line 5) (column 16) (len 3)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 120) (line 5) (column 16) (len 1))) (segment 1 (token "b") (name "b") (separator dot) (span (offset 122) (line 5) (column 18) (len 1)))))
    (reference r7 (scope relative) (span (offset 127) (line 5) (column 23) (len 5)) (segments (segment 0 (token "items") (name "items") (separator none) (span (offset 127) (line 5) (column 23) (len 5)))))
  )
  (root (package (visibility none) (name "AssignTest") (body brace (action-def (name "A") (modifiers) (specializes none) (body brace (assign (then false) (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none)) (target (expression (span (offset 55) (line 3) (column 16) (len 1)) (ref r0))) (value (expression (span (offset 60) (line 3) (column 21) (len 7)) (index (base (expression (span (offset 60) (line 3) (column 21) (len 3)) (ref r1))) (operands (sequence-list (element first (expression (span (offset 65) (line 3) (column 26) (len 1)) (ref r2))))))))) (assign (then false) (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none)) (target (expression (span (offset 84) (line 4) (column 16) (len 5)) (ref r3))) (value (expression (span (offset 93) (line 4) (column 25) (len 10)) (index (base (expression (span (offset 93) (line 4) (column 25) (len 4)) (ref r4))) (operands (sequence-list (element first (expression (span (offset 99) (line 4) (column 31) (len 3)) (ref r5))))))))) (assign (then false) (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none)) (target (expression (span (offset 120) (line 5) (column 16) (len 3)) (ref r6))) (value (expression (span (offset 127) (line 5) (column 23) (len 9)) (index (base (expression (span (offset 127) (line 5) (column 23) (len 5)) (ref r7))) (operands (sequence-list (element first (expression (span (offset 134) (line 5) (column 30) (len 1)) (integer 0))))))))))))))
)
~~~
