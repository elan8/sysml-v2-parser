# META
~~~sexpr
(snapshot (type recovery) (description "A malformed member inside a typed ForLoopNode ActionBodyParameter synchronizes to its later body sibling and the following typed for node. The grammar-owned prefix, variable declaration, in node parameter, and body are from SysML textual BNF 954-965 and 1151-1155; pinned Pilot SysML.xtext 1438-1439 and 1624-1628."))
~~~
# SOURCE
~~~sysml
package ForLoopRecovery {
    action def Owner {
        for first : Entry in entries {
            nonsense ???;
            action retainedInside;
        }
        for later : Entry in laterEntries {
            action retainedAfter;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "for_loop_nodes_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 100) (line 4) (column 13) (len 26)) (message "unrecognized declaration `nonsense` in action body"))
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
    (reference r0 (scope relative) (span (offset 69) (line 3) (column 21) (len 5)) (segments (segment 0 (token "Entry") (name "Entry") (separator none) (span (offset 69) (line 3) (column 21) (len 5)))))
    (reference r1 (scope relative) (span (offset 78) (line 3) (column 30) (len 7)) (segments (segment 0 (token "entries") (name "entries") (separator none) (span (offset 78) (line 3) (column 30) (len 7)))))
    (reference r2 (scope relative) (span (offset 179) (line 7) (column 21) (len 5)) (segments (segment 0 (token "Entry") (name "Entry") (separator none) (span (offset 179) (line 7) (column 21) (len 5)))))
    (reference r3 (scope relative) (span (offset 188) (line 7) (column 30) (len 12)) (segments (segment 0 (token "laterEntries") (name "laterEntries") (separator none) (span (offset 188) (line 7) (column 30) (len 12)))))
  )
  (root (package (name "ForLoopRecovery") (body brace (action-def (name "Owner") (modifiers) (specializes none) (body brace (for-loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (variable (for-variable (name "first") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (in (expression (span (offset 78) (line 3) (column 30) (len 7)) (ref r1))) (body-parameter (action-declaration none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "nonsense ???;") (span (offset 100) (line 4) (column 13) (len 26))) (action-usage (name "retainedInside") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))))) (for-loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (variable (for-variable (name "later") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (in (expression (span (offset 188) (line 7) (column 30) (len 12)) (ref r3))) (body-parameter (action-declaration none) (body brace (action-usage (name "retainedAfter") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))))))))))
)
~~~
