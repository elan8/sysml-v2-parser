# META
~~~sexpr
(snapshot (type semantic) (description "Preserves gh87_4_bare_item_usage_in_occurrence_def_body through the canonical parser projection while replacing scalar typing fields."))
~~~
# SOURCE
~~~sysml
package P {
            occurrence def Occ {
                item x;
                part y;
            }
        }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "typing_control_gh87_4_bare_item_usage_in_occurrence_def_body.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    occurrence def Occ {
        item x;
        part y;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
  )
  (root (package (name "P") (body brace (occurrence-def (name "Occ") (short-name none) (modifiers) (specializes none) (body brace (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "x") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "y") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
