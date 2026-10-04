# META
~~~sexpr
(snapshot (type recovery) (description "ControlNodePrefix takes its slots in RefPrefix order, so `constant in join j;` is not a control node and recovers as a single member, while the following valid prefixed control node still parses."))
~~~
# SOURCE
~~~sysml
package ControlNodePrefixRecovery {
    action def Act {
        constant in join j;
        in fork g;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "control_node_prefix_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 65) (line 3) (column 9) (len 28)) (message "unexpected token in action body"))
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
    (reference r0 (scope relative) (span (offset 101) (line 4) (column 17) (len 1)) (segments (segment 0 (token "g") (name "g") (separator none) (span (offset 101) (line 4) (column 17) (len 1)))))
  )
  (root (package (name "ControlNodePrefixRecovery") (body brace (action-def (name "Act") (modifiers) (specializes none) (body brace (malformed (code "recovered_action_body_element") (found "constant in join j;") (span (offset 65) (line 3) (column 9) (len 28))) (fork (prefix (direction in) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 101) (line 4) (column 17) (len 1)) (ref r0)))) (body semicolon (span (span (offset 102) (line 4) (column 18) (len 1))))))))))
)
~~~
