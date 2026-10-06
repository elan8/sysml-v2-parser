# META
~~~sexpr
(snapshot (type recovery) (description "KerML FeatureSpecialization has no `specializes` spelling (that keyword belongs to classifier SuperclassingPart), so a connector written with it is recovered with an exact span and the following valid connector is preserved."))
~~~
# SOURCE
~~~sysml
package Connectors {
    classifier Holder {
        connector bad specializes Links::BinaryLink;
        connector good : Links::BinaryLink;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connector_feature_specialization_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 53) (line 3) (column 9) (len 53)) (message "unrecognized declaration `connector` in calc body"))
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
    (reference r0 (scope relative) (span (offset 123) (line 4) (column 26) (len 17)) (segments (segment 0 (token "Links") (name "Links") (separator none) (span (offset 123) (line 4) (column 26) (len 5))) (segment 1 (token "BinaryLink") (name "BinaryLink") (separator colon-colon) (span (offset 130) (line 4) (column 33) (len 10)))))
  )
  (root (package (name "Connectors") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "Holder") (multiplicity none) (specializes none) (conjugates none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "connector bad specializes Links::BinaryLink;") (span (offset 53) (line 3) (column 9) (len 53))) (kerml-connector (all false) (name "good") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (from none) (to none) (body semicolon)))))))
)
~~~
