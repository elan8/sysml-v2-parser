# META
~~~sexpr
(snapshot (type semantic) (description "Inherited attribute value reports both redefinition and type mismatch diagnostics"))
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
        attribute status = "approved";
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "inherited_attribute_value_type_mismatch.md"
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
        attribute status = "approved";
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
