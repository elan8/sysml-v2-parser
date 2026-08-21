# META
~~~sexpr
(snapshot (type recovery) (description "Recovery synchronizes at VariantUsageMember in each newly migrated owning body after malformed text, retaining typed variants and the `variant action` BehaviorUsageElement alternative."))
~~~
# SOURCE
~~~sysml
package VariantUsageRecovery {
    variation attribute def AttributeOwner {
        nonsense ???;
        variant attribute retainedAttribute;
    }
    variation action def ActionDefinitionOwner {
        nonsense ???;
        variant action retainedAction;
    }
    variation port def PortDefinitionOwner {
        nonsense ???;
        variant port retainedPortDefinition;
    }
    variation port PortUsageOwner {
        nonsense ???;
        variant port retainedPortUsage;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "variant_usage_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 84) (line 3) (column 9) (len 22)) (message "unrecognized declaration `nonsense` in attribute body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 206) (line 7) (column 9) (len 22)) (message "unrecognized declaration `nonsense` in action body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 318) (line 11) (column 9) (len 22)) (message "unrecognized declaration `nonsense` in port definition body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 427) (line 15) (column 9) (len 22)) (message "unrecognized declaration `nonsense` in port body"))
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
  (root (package (name "VariantUsageRecovery") (body brace (attribute-def (declaration-name "AttributeOwner") (short-name none) (modifiers (variation (span (offset 35) (line 2) (column 5) (len 9)))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "nonsense ???;") (span (offset 84) (line 3) (column 9) (len 22))) (variant-usage (target none) (usage (attribute-usage (declaration-name "retainedAttribute") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (body absent)))) (action-def (name "ActionDefinitionOwner") (modifiers (variation (span (offset 153) (line 6) (column 5) (len 9)))) (specializes none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "nonsense ???;") (span (offset 206) (line 7) (column 9) (len 22))) (variant-usage (target none) (usage (action-usage (name "retainedAction") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))) (body absent)))) (port-def (name "PortDefinitionOwner") (modifiers (variation (span (offset 269) (line 10) (column 5) (len 9)))) (specializes none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "nonsense ???;") (span (offset 318) (line 11) (column 9) (len 22))) (variant-usage (target none) (usage (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "retainedPortDefinition") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (body absent)))) (port-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "PortUsageOwner") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "nonsense ???;") (span (offset 427) (line 15) (column 9) (len 22))) (variant-usage (target none) (usage (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "retainedPortUsage") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (body absent)))))))
)
~~~
