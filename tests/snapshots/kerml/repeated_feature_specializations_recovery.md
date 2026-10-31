# META
~~~sexpr
(snapshot (type recovery) (description "A malformed repeated KerML feature specialization is captured without consuming the following valid feature or leaking speculative qualified-reference arena entries.") (name-language kerml))
~~~
# SOURCE
~~~sysml
package FeatureSpecializationRecovery {
    feature broken crosses retained crosses ;
    feature later references valid;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "repeated_feature_specializations_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 44) (line 2) (column 5) (len 41)) (message "unexpected token in package body"))
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
  (name-language kerml)
  (references
    (reference r0 (scope relative) (span (offset 115) (line 3) (column 30) (len 5)) (segments (segment 0 (token "valid") (name "valid") (separator none) (span (offset 115) (line 3) (column 30) (len 5)))))
  )
  (root (package (visibility none) (name "FeatureSpecializationRecovery") (body brace (malformed (code "recovered_package_body_element") (found "feature broken crosses retained crosses ;") (span (offset 44) (line 2) (column 5) (len 41))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "later") (specializations (reference-subsetting (relationship (kind references) (implied false) (targets (ref r0))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))))
)
~~~
