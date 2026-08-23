# META
~~~sexpr
(snapshot (type semantic) (description "ConcernUsage retains each authored NAME token: an intentionally quoted BASIC_NAME at package scope and an escaped UNRESTRICTED_NAME nested in a requirement body. The decoded semantic names remain visible while formatting streams the exact source-backed spelling."))
~~~
# SOURCE
~~~sysml
package ConcernUsageAuthoredNames {
    concern 'modularity';

    requirement def OwningRequirement {
        concern 'owner\'s concern';
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "concern_usage_authored_names.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ConcernUsageAuthoredNames {
    concern 'modularity';
    requirement def OwningRequirement {
        concern 'owner\'s concern';
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "ConcernUsageAuthoredNames") (body brace (concern-usage (name "modularity") (visibility none) (abstract false) (individual false) (definition false) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (requirement-def (name "OwningRequirement") (modifiers) (body brace (concern-usage (name "owner's concern") (visibility none) (abstract false) (individual false) (definition false) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)))))))
)
~~~
