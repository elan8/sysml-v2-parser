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
  (root (package (name "Demo") (body (enum-def) (requirement-def (name "ManagedRequirement") (body (attribute-usage))) (requirement-def (name "UserRequirement") (body semicolon)) (requirement-def (name "Need") (body semicolon)) (requirement-usage))))
)
~~~
