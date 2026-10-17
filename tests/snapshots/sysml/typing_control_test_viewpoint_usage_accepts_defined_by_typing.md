# META
~~~sexpr
(snapshot (type semantic) (description "Preserves test_viewpoint_usage_accepts_defined_by_typing through the canonical parser projection while replacing scalar typing fields."))
~~~
# SOURCE
~~~sysml
package P {
viewpoint safety defined by Mission::SafetyViewpoint { }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "typing_control_test_viewpoint_usage_accepts_defined_by_typing.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    viewpoint safety defined by Mission::SafetyViewpoint {
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 40) (line 2) (column 29) (len 24)) (segments (segment 0 (token "Mission") (name "Mission") (separator none) (span (offset 40) (line 2) (column 29) (len 7))) (segment 1 (token "SafetyViewpoint") (name "SafetyViewpoint") (separator colon-colon) (span (offset 49) (line 2) (column 38) (len 15)))))
  )
  (root (package (name "P") (body brace (viewpoint-usage (name "safety") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (body brace)))))
)
~~~
