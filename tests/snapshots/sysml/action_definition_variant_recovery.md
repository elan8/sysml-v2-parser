# META
~~~sexpr
(snapshot (type recovery) (description "Issue 200: malformed nested definitions and variants retain source, report syntax errors and preserve valid later members."))
~~~
# SOURCE
~~~sysml
package BodyRecovery {
    action def A {
        part def Bad : ;
        item def Good;
        view def BadView : ;
        view def GoodView;
    }
    use case choices {
        variant use case bad : ;
        variant use case good;
    }
    state def S {
        variant state bad : ;
        variant state good;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "action_definition_variant_recovery.md"
    (diagnostics
      (diagnostic (code "invalid_typing_operator") (severity error) (category parseerror) (span (offset 50) (line 3) (column 9) (len 25)) (message "invalid typing operator in part definition specialization: use ':>' instead of ':'"))
      (diagnostic (code "recovered_use_case_body_element") (severity error) (category parseerror) (span (offset 183) (line 9) (column 9) (len 24)) (message "unexpected token in use case body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 271) (line 13) (column 9) (len 30)) (message "unexpected keyword `variant` in state body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package BodyRecovery {
    action def A {
        part def Bad : ;
        item def Good;
        view def BadView;
        view def GoodView;
    }
    use case choices {
        variant use case bad : ;
        variant use case good;
    }
    state def S {
        variant state bad : ;
        variant state good;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
  )
  (root (package (visibility none) (name "BodyRecovery") (body brace (action-def (name "A") (modifiers) (specializes none) (body brace (malformed (code "invalid_typing_operator") (found "part def Bad : ;") (span (offset 50) (line 3) (column 9) (len 25))) (item-def (name "Good") (modifiers) (individual false) (specializes none) (body semicolon)) (view-def (name "BadView") (short-name none) (modifiers) (specializes none) (body semicolon)) (view-def (name "GoodView") (short-name none) (modifiers) (specializes none) (body semicolon)))) (use-case-usage (declaration (name "choices") (short-name none) (abstract false) (type none) (subsets none)) (variation false) (body brace (malformed (code "recovered_use_case_body_element") (found "variant use case bad : ;") (span (offset 183) (line 9) (column 9) (len 24))) (variant-usage (target none) (usage (use-case-usage (declaration (name "good") (short-name none) (abstract false) (type none) (subsets none)) (variation false) (body semicolon))) (body absent)))) (state-def (name "S") (modifiers) (body brace (malformed (code "unexpected_keyword_in_scope") (found "variant state bad : ;") (span (offset 271) (line 13) (column 9) (len 30))) (variant-usage (target none) (usage (state-usage (name "good") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))) (body absent)))))))
)
~~~
