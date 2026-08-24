# META
~~~sexpr
(snapshot (type recovery) (description "A malformed GuardedSuccession is one explicit action-body recovery node, and the later action sibling remains typed. The transactional GuardedSuccession dispatch runs before FirstStmt so speculative feature-chain references cannot escape into the document arena (SysML textual BNF 1180-1185; pinned Pilot SysML.xtext 1719-1725)."))
~~~
# SOURCE
~~~sysml
package GuardedSuccessionRecovery {
    action def Decision {
        first source if guard then;
        action later;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "guarded_succession_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 70) (line 3) (column 9) (len 36)) (message "unexpected token in action body"))
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
  (root (package (name "GuardedSuccessionRecovery") (body brace (action-def (name "Decision") (modifiers) (specializes none) (body brace (malformed (code "recovered_action_body_element") (found "first source if guard then;") (span (offset 70) (line 3) (column 9) (len 36))) (action-usage (keyword action) (name "later") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))))
)
~~~
