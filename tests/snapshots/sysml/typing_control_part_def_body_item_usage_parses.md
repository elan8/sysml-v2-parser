# META
~~~sexpr
(snapshot (type semantic) (description "Preserves part_def_body_item_usage_parses through the canonical parser projection while replacing scalar typing fields."))
~~~
# SOURCE
~~~sysml
package P {
part def Stakeholder {
item concerns[*]: Concern;
}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "typing_control_part_def_body_item_usage_parses.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    part def Stakeholder {
        item concerns : Concern[*];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 53) (line 3) (column 19) (len 7)) (segments (segment 0 (token "Concern") (name "Concern") (separator none) (span (offset 53) (line 3) (column 19) (len 7)))))
  )
  (root (package (name "P") (body brace (part-def (name "Stakeholder") (modifiers) (body brace (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "concerns") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower none) (upper (expression (span (offset 49) (line 3) (column 15) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))
)
~~~
