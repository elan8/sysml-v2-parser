# META
~~~sexpr
(snapshot (type semantic) (description "RequirementBody reaches DefinitionBodyItem, including nested requirement definitions, port usages, allocation usages, and anonymous requirement usages; later valid siblings remain intact."))
~~~
# SOURCE
~~~sysml
package RequirementDefinitionBodyMembers {
    requirement def Outer {
        frame concern vs : VehicleSafety;
        requirement def Nested;
        requirement;
        port evidence : EvidencePort;
        allocate source to target;
        attribute after : Boolean;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "requirement_definition_body_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package RequirementDefinitionBodyMembers {
    requirement def Outer {
        frame concern vs : VehicleSafety;
        requirement def Nested;
        requirement ;
        port evidence : EvidencePort;
        allocate source to target;
        attribute 'after' : Boolean;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 98) (line 3) (column 28) (len 13)) (segments (segment 0 (token "VehicleSafety") (name "VehicleSafety") (separator none) (span (offset 98) (line 3) (column 28) (len 13)))))
    (reference r1 (scope relative) (span (offset 190) (line 6) (column 25) (len 12)) (segments (segment 0 (token "EvidencePort") (name "EvidencePort") (separator none) (span (offset 190) (line 6) (column 25) (len 12)))))
  )
  (root (package (name "RequirementDefinitionBodyMembers") (body brace (requirement-def (name "Outer") (modifiers) (body brace (frame (concern-keyword true) (name "vs") (short-name none) (type (ref r0)) (body semicolon)) (requirement-def (name "Nested") (modifiers) (body semicolon)) (requirement-usage (name none) (multiplicity none)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "evidence") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (allocation-usage) (attribute-usage))))))
)
~~~
