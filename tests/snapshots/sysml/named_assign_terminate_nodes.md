# META
~~~sexpr
(snapshot (type semantic) (description "AssignmentNode and TerminateNode carry the shared ActionNodePrefix, so `action stop terminate;` and `action inc assign x := 1;` are one named node each, and a `then` before `terminate` introduces a terminate node rather than a succession target named `terminate`. SysML textual BNF 8.2.2.17.1, 8.2.2.17.5 and 8.2.2.17.6."))
~~~
# SOURCE
~~~sysml
package NamedActionNodes {
    action def Definition {
        first start;
        then action stop terminate;
        action inc assign count := 1;
        then action typedInc : Step assign count := 2;
        then assign count := 3;
        then terminate;
        then terminate self;
        terminate;
    }
    action Usage {
        then action stop terminate self;
        then action inc assign count := 4;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "named_assign_terminate_nodes.md"
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
    (reference r0 (scope relative) (span (offset 69) (line 3) (column 15) (len 5)) (segments (segment 0 (token "start") (name "start") (separator none) (span (offset 69) (line 3) (column 15) (len 5)))))
    (reference r1 (scope relative) (span (offset 138) (line 5) (column 27) (len 5)) (segments (segment 0 (token "count") (name "count") (separator none) (span (offset 138) (line 5) (column 27) (len 5)))))
    (reference r2 (scope relative) (span (offset 181) (line 6) (column 32) (len 4)) (segments (segment 0 (token "Step") (name "Step") (separator none) (span (offset 181) (line 6) (column 32) (len 4)))))
    (reference r3 (scope relative) (span (offset 193) (line 6) (column 44) (len 5)) (segments (segment 0 (token "count") (name "count") (separator none) (span (offset 193) (line 6) (column 44) (len 5)))))
    (reference r4 (scope relative) (span (offset 225) (line 7) (column 21) (len 5)) (segments (segment 0 (token "count") (name "count") (separator none) (span (offset 225) (line 7) (column 21) (len 5)))))
    (reference r5 (scope relative) (span (offset 284) (line 9) (column 24) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 284) (line 9) (column 24) (len 4)))))
    (reference r6 (scope relative) (span (offset 369) (line 13) (column 36) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 369) (line 13) (column 36) (len 4)))))
  )
  (root (package (visibility none) (name "NamedActionNodes") (body brace (action-def (name "Definition") (modifiers) (specializes none) (body brace (first (source (expression (span (offset 69) (line 3) (column 15) (len 5)) (ref r0))) (target none) (body semicolon (span (span (offset 74) (line 3) (column 20) (len 1))))) (terminate (then true) (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration (name "stop") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (target none)) (assign (then false) (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration (name "inc") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (target (expression (span (offset 138) (line 5) (column 27) (len 5)) (ref r1))) (value (expression (span (offset 147) (line 5) (column 36) (len 1)) (integer 1)))) (assign (then true) (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration (name "typedInc") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (target (expression (span (offset 193) (line 6) (column 44) (len 5)) (ref r3))) (value (expression (span (offset 202) (line 6) (column 53) (len 1)) (integer 2)))) (assign (then true) (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none)) (target (expression (span (offset 225) (line 7) (column 21) (len 5)) (ref r4))) (value (expression (span (offset 234) (line 7) (column 30) (len 1)) (integer 3)))) (terminate (then true) (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none)) (target none)) (terminate (then true) (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none)) (target (expression (span (offset 284) (line 9) (column 24) (len 4)) (ref r5)))) (terminate (then false) (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none)) (target none)))) (action-usage (keyword action) (name "Usage") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (terminate (then true) (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration (name "stop") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (target (expression (span (offset 369) (line 13) (column 36) (len 4)) (ref r6)))) (assign))))))
)
~~~
