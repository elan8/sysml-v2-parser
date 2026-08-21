# META
~~~sexpr
(snapshot (type recovery) (description "A malformed SubjectUsage feature-specialization header becomes a recovery node without consuming the following valid subject declaration."))
~~~
# SOURCE
~~~sysml
package SubjectUsageFeatureSpecializationRecovery {
    requirement def R {
        subject malformed :> ;
        subject after :> Target;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "subject_usage_feature_specialization_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_requirement_body_element") (severity error) (category parseerror) (span (offset 84) (line 3) (column 9) (len 31)) (message "unexpected token in requirement body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package SubjectUsageFeatureSpecializationRecovery {
    requirement def R {
        subject malformed :> ;
        subject 'after' :> Target;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 132) (line 4) (column 26) (len 6)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 132) (line 4) (column 26) (len 6)))))
  )
  (root (package (name "SubjectUsageFeatureSpecializationRecovery") (body brace (requirement-def (name "R") (modifiers) (body brace (malformed (code "recovered_requirement_body_element") (found "subject malformed :> ;") (span (offset 84) (line 3) (column 9) (len 31))) (subject (name "after") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r0)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
