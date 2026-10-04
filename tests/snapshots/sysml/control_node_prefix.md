# META
~~~sexpr
(snapshot (type semantic) (description "SysML ControlNodePrefix is RefPrefix plus individual and PortionKind and UsageExtensionKeyword*, ahead of merge, decide, join and fork. A direction makes the node referential (`in fork g;`), so every slot is retained on the control node, in action definition and action usage bodies and after `then`."))
~~~
# SOURCE
~~~sysml
package ControlNodePrefixes {
    action def Act {
        fork f;
        in fork g;
        out derived join j;
        inout abstract constant merge m;
        variation individual snapshot decide d;
        timeslice #Tag fork t;
        action start;
        then in merge afterwards;
    }
    action act {
        in decide d;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "control_node_prefix.md"
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
    (reference r0 (scope relative) (span (offset 64) (line 3) (column 14) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 64) (line 3) (column 14) (len 1)))))
    (reference r1 (scope relative) (span (offset 83) (line 4) (column 17) (len 1)) (segments (segment 0 (token "g") (name "g") (separator none) (span (offset 83) (line 4) (column 17) (len 1)))))
    (reference r2 (scope relative) (span (offset 111) (line 5) (column 26) (len 1)) (segments (segment 0 (token "j") (name "j") (separator none) (span (offset 111) (line 5) (column 26) (len 1)))))
    (reference r3 (scope relative) (span (offset 152) (line 6) (column 39) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 152) (line 6) (column 39) (len 1)))))
    (reference r4 (scope relative) (span (offset 200) (line 7) (column 46) (len 1)) (segments (segment 0 (token "d") (name "d") (separator none) (span (offset 200) (line 7) (column 46) (len 1)))))
    (reference r5 (scope relative) (span (offset 222) (line 8) (column 20) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 222) (line 8) (column 20) (len 3)))))
    (reference r6 (scope relative) (span (offset 231) (line 8) (column 29) (len 1)) (segments (segment 0 (token "t") (name "t") (separator none) (span (offset 231) (line 8) (column 29) (len 1)))))
    (reference r7 (scope relative) (span (offset 278) (line 10) (column 23) (len 10)) (segments (segment 0 (token "afterwards") (name "afterwards") (separator none) (span (offset 278) (line 10) (column 23) (len 10)))))
    (reference r8 (scope relative) (span (offset 331) (line 13) (column 19) (len 1)) (segments (segment 0 (token "d") (name "d") (separator none) (span (offset 331) (line 13) (column 19) (len 1)))))
  )
  (root (package (name "ControlNodePrefixes") (body brace (action-def (name "Act") (modifiers) (specializes none) (body brace (fork (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 64) (line 3) (column 14) (len 1)) (ref r0)))) (body semicolon (span (span (offset 65) (line 3) (column 15) (len 1))))) (fork (prefix (direction in) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 83) (line 4) (column 17) (len 1)) (ref r1)))) (body semicolon (span (span (offset 84) (line 4) (column 18) (len 1))))) (join (prefix (direction out) (derived true) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 111) (line 5) (column 26) (len 1)) (ref r2)))) (body semicolon (span (span (offset 112) (line 5) (column 27) (len 1))))) (merge (prefix (direction inout) (derived false) (variance abstract) (constant true) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 152) (line 6) (column 39) (len 1)) (ref r3)))) (body semicolon (span (span (offset 153) (line 6) (column 40) (len 1))))) (decide (prefix (direction none) (derived false) (variance variation) (constant false) (individual true) (portion snapshot) (extensions)) (declaration (named (expression (span (offset 200) (line 7) (column 46) (len 1)) (ref r4)))) (body semicolon (span (span (offset 201) (line 7) (column 47) (len 1))))) (fork (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion timeslice) (extensions (ref r5))) (declaration (named (expression (span (offset 231) (line 8) (column 29) (len 1)) (ref r6)))) (body semicolon (span (span (offset 232) (line 8) (column 30) (len 1))))) (action-usage (keyword action) (name "start") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (then-control (merge (prefix (direction in) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 278) (line 10) (column 23) (len 10)) (ref r7)))) (body semicolon (span (span (offset 288) (line 10) (column 33) (len 1)))))))) (action-usage (keyword action) (name "act") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (decide (prefix (direction in) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 331) (line 13) (column 19) (len 1)) (ref r8)))) (body semicolon (span (span (offset 332) (line 13) (column 20) (len 1))))))))))
)
~~~
