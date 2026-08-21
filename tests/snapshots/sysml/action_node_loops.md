# META
~~~sexpr
(snapshot (type semantic) (description "Projects the shared ActionNodePrefix, complete action UsageDeclaration header, mandatory action body, and optional until parameter for both loop and while nodes in definition and usage action bodies. SysML textual BNF 954-965 and 1143-1149; pinned Pilot SysML.xtext 1438-1439 and 1615-1621."))
~~~
# SOURCE
~~~sysml
package ActionNodeLoops {
    action def DefinitionOwner {
        ref action <loopStep> loopStep : Step loop {
            action nested;
        } until finished;
        action whileStep : Step while running {
            action nestedWhile;
        } until complete;
    }
    action UsageOwner {
        loop {
            action nestedUsage;
        } until done;
        while ready {
            action nestedUsageWhile;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "action_node_loops.md"
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
    (reference r0 (scope relative) (span (offset 100) (line 3) (column 42) (len 4)) (segments (segment 0 (token "Step") (name "Step") (separator none) (span (offset 100) (line 3) (column 42) (len 4)))))
    (reference r1 (scope relative) (span (offset 155) (line 5) (column 17) (len 8)) (segments (segment 0 (token "finished") (name "finished") (separator none) (span (offset 155) (line 5) (column 17) (len 8)))))
    (reference r2 (scope relative) (span (offset 192) (line 6) (column 28) (len 4)) (segments (segment 0 (token "Step") (name "Step") (separator none) (span (offset 192) (line 6) (column 28) (len 4)))))
    (reference r3 (scope relative) (span (offset 203) (line 6) (column 39) (len 7)) (segments (segment 0 (token "running") (name "running") (separator none) (span (offset 203) (line 6) (column 39) (len 7)))))
    (reference r4 (scope relative) (span (offset 261) (line 8) (column 17) (len 8)) (segments (segment 0 (token "complete") (name "complete") (separator none) (span (offset 261) (line 8) (column 17) (len 8)))))
    (reference r5 (scope relative) (span (offset 364) (line 13) (column 17) (len 4)) (segments (segment 0 (token "done") (name "done") (separator none) (span (offset 364) (line 13) (column 17) (len 4)))))
    (reference r6 (scope relative) (span (offset 384) (line 14) (column 15) (len 5)) (segments (segment 0 (token "ready") (name "ready") (separator none) (span (offset 384) (line 14) (column 15) (len 5)))))
  )
  (root (package (name "ActionNodeLoops") (body brace (action-def (name "DefinitionOwner") (modifiers) (specializes none) (body brace (loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (action-declaration (name "loopStep") (short-name "loopStep") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)))) (condition none) (body-parameter (action-declaration none) (body brace (action-usage (name "nested") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (until (expression (span (offset 155) (line 5) (column 17) (len 8)) (ref r1)))) (while-loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration (name "whileStep") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)))) (condition (expression (span (offset 203) (line 6) (column 39) (len 7)) (ref r3))) (body-parameter (action-declaration none) (body brace (action-usage (name "nestedWhile") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (until (expression (span (offset 261) (line 8) (column 17) (len 8)) (ref r4)))))) (action-usage (name "UsageOwner") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (condition none) (body-parameter (action-declaration none) (body brace (action-usage (name "nestedUsage") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (until (expression (span (offset 364) (line 13) (column 17) (len 4)) (ref r5)))) (while-loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (condition (expression (span (offset 384) (line 14) (column 15) (len 5)) (ref r6))) (body-parameter (action-declaration none) (body brace (action-usage (name "nestedUsageWhile") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (until none)))))))
)
~~~
