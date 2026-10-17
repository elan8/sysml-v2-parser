# META
~~~sexpr
(snapshot (type semantic) (description "Preserves test_use_case_usage_accepts_typed_by_and_specialization_clauses through the canonical parser projection while replacing scalar typing fields."))
~~~
# SOURCE
~~~sysml
package P {
use case mission typed by Mission::CaseType subsets BaseCase;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "typing_control_test_use_case_usage_accepts_typed_by_and_specialization_clauses.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    use case mission typed by Mission::CaseType subsets BaseCase;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 38) (line 2) (column 27) (len 17)) (segments (segment 0 (token "Mission") (name "Mission") (separator none) (span (offset 38) (line 2) (column 27) (len 7))) (segment 1 (token "CaseType") (name "CaseType") (separator colon-colon) (span (offset 47) (line 2) (column 36) (len 8)))))
    (reference r1 (scope relative) (span (offset 64) (line 2) (column 53) (len 8)) (segments (segment 0 (token "BaseCase") (name "BaseCase") (separator none) (span (offset 64) (line 2) (column 53) (len 8)))))
  )
  (root (package (name "P") (body brace (use-case-usage (declaration (name "mission") (short-name none) (abstract false) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r1))))) (variation false) (body semicolon)))))
)
~~~
