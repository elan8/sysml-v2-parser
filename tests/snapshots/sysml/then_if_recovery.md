# META
~~~sexpr
(snapshot (type recovery) (description "A malformed then-if condition becomes an explicit action-body recovery node and does not consume the later valid then action. SysML textual BNF 898-909 and 1123-1141; Pilot ActionBody 1361-1368 and IfNode 1596-1612."))
~~~
# SOURCE
~~~sysml
package ThenIfRecovery {
    action def Owner {
        then if {
            action missingCondition;
        }
        then join;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "then_if_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 56) (line 3) (column 9) (len 65)) (message "unexpected token in action body"))
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
  (root (package (name "ThenIfRecovery") (body brace (action-def (name "Owner") (modifiers) (specializes none) (body brace (malformed (code "recovered_action_body_element") (found "then if {") (span (offset 56) (line 3) (column 9) (len 65))) (then-control (join (declaration anonymous) (body semicolon (span (span (offset 130) (line 6) (column 18) (len 1)))))))))))
)
~~~
