# META
~~~sexpr
(snapshot (type semantic) (description "Preserves gh89_9_directed_item_usage_in_part_def_body through the canonical parser projection while replacing scalar typing fields."))
~~~
# SOURCE
~~~sysml
package TimeVaryingAttribute {
            item def PwrCmd;
            part def Transport2 {
                out item pwrCmd:PwrCmd;
            }
        }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "typing_control_gh89_9_directed_item_usage_in_part_def_body.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package TimeVaryingAttribute {
    item def PwrCmd;
    part def Transport2 {
        out item pwrCmd : PwrCmd;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 126) (line 4) (column 33) (len 6)) (segments (segment 0 (token "PwrCmd") (name "PwrCmd") (separator none) (span (offset 126) (line 4) (column 33) (len 6)))))
  )
  (root (package (visibility none) (name "TimeVaryingAttribute") (body brace (item-def (name "PwrCmd") (modifiers) (individual false) (specializes none) (body semicolon)) (part-def (name "Transport2") (modifiers) (body brace (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "pwrCmd") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))
)
~~~
