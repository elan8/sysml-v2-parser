# META
~~~sexpr
(snapshot (type recovery) (description "An anonymous then-control brace ActionBody retains malformed content, synchronizes to its later typed member, and preserves the enclosing then-control's later sibling. SysML textual BNF 898-909 and 969-998; Pilot ActionBody 1361-1368 and ControlNode 1650-1685."))
~~~
# SOURCE
~~~sysml
package ThenControlRecovery {
    action def Owner {
        then fork {
            nonsense ???;
            action retained;
        }
        then join;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "then_control_nodes_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 85) (line 4) (column 13) (len 26)) (message "unrecognized declaration `nonsense` in first/merge body"))
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
  )
  (root (package (name "ThenControlRecovery") (body brace (action-def (name "Owner") (modifiers) (specializes none) (body brace (then-control (fork (declaration anonymous) (body brace (open-brace (span (offset 71) (line 3) (column 19) (len 1))) (members (malformed (code "unrecognized_declaration_in_scope") (found "nonsense ???;") (span (offset 85) (line 4) (column 13) (len 26))) (action-usage (keyword action) (name "retained") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))) (close-brace (span (offset 136) (line 6) (column 9) (len 1)))))) (then-control (join (declaration anonymous) (body semicolon (span (span (offset 155) (line 7) (column 18) (len 1)))))))))))
)
~~~
