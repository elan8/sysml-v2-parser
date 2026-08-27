# META
~~~sexpr
(snapshot (type recovery) (description "A malformed repeated KerML feature specialization is captured without consuming the following valid feature or leaking speculative qualified-reference arena entries."))
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
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 44) (line 2) (column 5) (len 41)) (message "the spec-valid KerML feature declaration production is retained but not structurally implemented"))
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
    (reference r0 (scope relative) (span (offset 115) (line 3) (column 30) (len 5)) (segments (segment 0 (token "valid") (name "valid") (separator none) (span (offset 115) (line 3) (column 30) (len 5)))))
  )
  (root (package (name "FeatureSpecializationRecovery") (body brace (feature-declaration) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "later") (specializations (reference-subsetting (relationship (kind references) (implied false) (targets (ref r0))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))))
)
~~~
