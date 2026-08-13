# META
~~~sexpr
(snapshot (type semantic) (description "concern def and concern usage retain :>/:>> specialization clauses"))
~~~
# SOURCE
~~~sysml
package ConcernDefSubsettingExample {
    concern def ConcernCheck :> RequirementCheck {
    }
    concern c :>> baseConcern;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "concern_def_subsetting.md"
    (diagnostics
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
  )
  (root (package (name "ConcernDefSubsettingExample") (body (concern-usage) (concern-usage))))
)
~~~
