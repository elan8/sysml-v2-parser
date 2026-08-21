# META
~~~sexpr
(snapshot (type recovery) (description "A malformed member inside a typed loop ActionBodyParameter synchronizes to its later body sibling and the following typed while node. The while/loop ActionNodePrefix and until tail are from SysML textual BNF 954-965 and 1143-1149; pinned Pilot SysML.xtext 1438-1439 and 1615-1621."))
~~~
# SOURCE
~~~sysml
package ActionNodeLoopRecovery {
    action def Owner {
        loop {
            nonsense ???;
            action retainedInside;
        } until done;
        while ready {
            action retainedAfter;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "action_node_loops_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 83) (line 4) (column 13) (len 26)) (message "unrecognized declaration `nonsense` in action body"))
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
    (reference r0 (scope relative) (span (offset 148) (line 6) (column 17) (len 4)) (segments (segment 0 (token "done") (name "done") (separator none) (span (offset 148) (line 6) (column 17) (len 4)))))
    (reference r1 (scope relative) (span (offset 168) (line 7) (column 15) (len 5)) (segments (segment 0 (token "ready") (name "ready") (separator none) (span (offset 168) (line 7) (column 15) (len 5)))))
  )
  (root (package (name "ActionNodeLoopRecovery") (body brace (action-def (name "Owner") (modifiers) (specializes none) (body brace (loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (condition none) (body-parameter (action-declaration none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "nonsense ???;") (span (offset 83) (line 4) (column 13) (len 26))) (action-usage (name "retainedInside") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (until (expression (span (offset 148) (line 6) (column 17) (len 4)) (ref r0)))) (while-loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (condition (expression (span (offset 168) (line 7) (column 15) (len 5)) (ref r1))) (body-parameter (action-declaration none) (body brace (action-usage (name "retainedAfter") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (until none)))))))
)
~~~
