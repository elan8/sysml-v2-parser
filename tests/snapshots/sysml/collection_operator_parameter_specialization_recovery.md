# META
~~~sexpr
(snapshot (type recovery) (description "A malformed collection-operator parameter specialization recovers at the owning attribute member and preserves the later PartUsageBody sibling."))
~~~
# SOURCE
~~~sysml
package CollectionOperatorParameterSpecializationRecovery {
    part holder {
        attribute result = values.?{in selected :> ; selected > minimum};
        attribute later : Real;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "collection_operator_parameter_specialization_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_part_usage_body_element") (severity error) (category parseerror) (span (offset 86) (line 3) (column 9) (len 74)) (message "unexpected token in part usage body"))
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
    (reference r0 (scope relative) (span (offset 178) (line 4) (column 27) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 178) (line 4) (column 27) (len 4)))))
  )
  (root (package (name "CollectionOperatorParameterSpecializationRecovery") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "holder") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (malformed (code "recovered_part_usage_body_element") (found "attribute result = values.?{in selected :> ; selected > mini") (span (offset 86) (line 3) (column 9) (len 74))) (attribute-usage (declaration-name "later") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
