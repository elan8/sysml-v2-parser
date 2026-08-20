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
        approved;
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
  (references
  )
  (root (package (name "Demo") (body brace (enum-def (name "RequirementStatusKind") (body brace (enum-value (name "approved") (short-name none) (value none) (body semicolon) (span (offset 60) (line 3) (column 9) (len 14))))) (requirement-def (name "ManagedRequirement") (modifiers) (body brace (attribute-usage))) (requirement-def (name "UserRequirement") (modifiers) (body semicolon)) (requirement-def (name "Need") (modifiers) (body semicolon)) (requirement-usage (name "need") (multiplicity none)))))
)
~~~
