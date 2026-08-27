# META
~~~sexpr
(snapshot (type recovery) (description "A malformed requirement-constraint membership in a part-definition body is recovered atomically without leaking its incomplete typing reference or consuming the following valid typed requirement constraint and ordinary sibling."))
~~~
# SOURCE
~~~sysml
package RequirementConstraintOwnerRecovery {
    part def Holder {
        require constraint broken : Missing::;
        require constraint retained : Bound;
        attribute after;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "require_constraint_invalid_owner_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 75) (line 3) (column 9) (len 47)) (message "unexpected token in part definition body"))
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
    (reference r0 (scope relative) (span (offset 152) (line 4) (column 39) (len 5)) (segments (segment 0 (token "Bound") (name "Bound") (separator none) (span (offset 152) (line 4) (column 39) (len 5)))))
  )
  (root (package (name "RequirementConstraintOwnerRecovery") (body brace (part-def (name "Holder") (modifiers) (body brace (malformed (code "recovered_part_def_body_element") (found "require constraint broken : Missing::;") (span (offset 75) (line 3) (column 9) (len 47))) (require-constraint (kind require) (constraint-keyword true) (name "retained") (target none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (attribute-usage (declaration-name "after") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
