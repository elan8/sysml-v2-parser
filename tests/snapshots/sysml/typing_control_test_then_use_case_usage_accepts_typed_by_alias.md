# META
~~~sexpr
(snapshot (type semantic) (description "Preserves test_then_use_case_usage_accepts_typed_by_alias through the canonical parser projection while replacing scalar typing fields."))
~~~
# SOURCE
~~~sysml
package P {
use case def U {
then use case step typed by Mission::StepCase;
}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "typing_control_test_then_use_case_usage_accepts_typed_by_alias.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    use case def U {
        then use case step typed by Mission::StepCase;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
  )
  (root (package (name "P") (body brace (use-case-def (name "U") (modifiers) (body brace (then-use-case))))))
)
~~~
