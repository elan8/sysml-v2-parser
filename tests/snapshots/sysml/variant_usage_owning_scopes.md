# META
~~~sexpr
(snapshot (type semantic) (description "VariantUsageMember is admitted by the shared AttributeBody, ActionBody, PortBody, and PortDefBody grammar boundaries. This fixture covers each newly owning scope, the pre-existing ActionUsageBody route, the new `variant action` BehaviorUsageElement alternative, and the untyped reference form (SysML textual BNF 237-252, 374-413, and 894-917; pinned Pilot `SysML.xtext` 518-531, 679-719, and 1361-1381)."))
~~~
# SOURCE
~~~sysml
package VariantUsageOwningScopes {
    variation attribute def AttributeOwner {
        variant attribute attributeAlternative;
    }
    variation action def ActionDefinitionOwner {
        variant action actionDefinitionAlternative;
    }
    variation port def PortDefinitionOwner {
        variant port portDefinitionAlternative;
    }
    variation port PortUsageOwner {
        variant port portUsageAlternative;
    }
    variation action ActionUsageOwner {
        variant action actionUsageAlternative;
        variant existingAlternative;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "variant_usage_owning_scopes.md"
    (diagnostics
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
    (reference r0 (scope relative) (span (offset 528) (line 16) (column 17) (len 19)) (segments (segment 0 (token "existingAlternative") (name "existingAlternative") (separator none) (span (offset 528) (line 16) (column 17) (len 19)))))
  )
  (root (package (name "VariantUsageOwningScopes") (body brace (attribute-def (declaration-name "AttributeOwner") (short-name none) (modifiers (variation (span (offset 39) (line 2) (column 5) (len 9)))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (variant-usage (target none) (usage (attribute-usage (declaration-name "attributeAlternative") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (body absent)))) (action-def (name "ActionDefinitionOwner") (modifiers (variation (span (offset 138) (line 5) (column 5) (len 9)))) (specializes none) (body brace (variant-usage (target none) (usage (action-usage (name "actionDefinitionAlternative") (short-name none) (body semicolon))) (body absent)))) (port-def (name "PortDefinitionOwner") (modifiers (variation (span (offset 245) (line 8) (column 5) (len 9)))) (specializes none) (body brace (variant-usage (target none) (usage (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "portDefinitionAlternative") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (body absent)))) (port-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "PortUsageOwner") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (variant-usage (target none) (usage (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "portUsageAlternative") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (body absent)))) (action-usage (name "ActionUsageOwner") (short-name none) (body brace (variant-usage (target none) (usage (action-usage (name "actionUsageAlternative") (short-name none) (body semicolon))) (body absent)) (variant-usage (target (ref r0)) (usage none) (body absent)))))))
)
~~~
