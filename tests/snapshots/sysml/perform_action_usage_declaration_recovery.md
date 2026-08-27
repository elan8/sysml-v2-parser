# META
~~~sexpr
(snapshot (type recovery) (description "A malformed declared perform action is recovered as one part-body member while its following sibling remains typed. The PerformActionUsageDeclaration parse is transactional, so the failed declaration does not leak its partial relationship reference (SysML textual BNF 944-952; pinned Pilot SysML.xtext 1411-1418)."))
~~~
# SOURCE
~~~sysml
package PerformActionUsageDeclarationRecovery {
    part host {
        perform action named :> target.;
        part retained;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "perform_action_usage_declaration_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_part_usage_body_element") (severity error) (category parseerror) (span (offset 72) (line 3) (column 9) (len 41)) (message "unexpected token in part usage body"))
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
  (root (package (name "PerformActionUsageDeclarationRecovery") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "host") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (malformed (code "recovered_part_usage_body_element") (found "perform action named :> target.;") (span (offset 72) (line 3) (column 9) (len 41))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "retained") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))
)
~~~
