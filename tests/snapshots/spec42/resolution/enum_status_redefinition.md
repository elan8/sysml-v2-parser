# META
~~~sexpr
(snapshot (type semantic) (description "Explicit enum status redefinition suppresses implicit redefinition diagnostics"))
~~~
# SOURCE
~~~sysml
package Demo {
    enum def RequirementStatusKind {
        enum approved;
    }
    requirement def ManagedRequirement {
        attribute status : RequirementStatusKind;
    }
    requirement def UserRequirement :> ManagedRequirement;
    requirement def Need :> UserRequirement;
    requirement need : Need {
        attribute :>> status = RequirementStatusKind::approved;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "enum_status_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Demo {
    enum def RequirementStatusKind {
        enum approved;
    }
    requirement def ManagedRequirement {
        attribute status : RequirementStatusKind;
    }
    requirement def UserRequirement :> ManagedRequirement;
    requirement def Need :> UserRequirement;
    requirement need : Need {
        :>> status = RequirementStatusKind::approved;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 305) (line 10) (column 24) (len 4)) (segments (segment 0 (token "Need") (name "Need") (separator none) (span (offset 305) (line 10) (column 24) (len 4)))))
  )
  (root (package (name "Demo") (body brace (enum-def (name "RequirementStatusKind") (body brace (enum-value (extensions) (enum-keyword present) (visibility none) (name "approved") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 60) (line 3) (column 9) (len 14))))) (requirement-def (name "ManagedRequirement") (modifiers) (body brace (attribute-usage))) (requirement-def (name "UserRequirement") (modifiers) (body semicolon)) (requirement-def (name "Need") (modifiers) (body semicolon)) (requirement-usage (name "need") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (abstract false) (variation false) none (subsets none) (references none) (body brace (attribute-usage))))))
)
~~~
