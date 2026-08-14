# META
~~~sexpr
(snapshot (type semantic) (description "Inherited diamond specialization target is deduplicated"))
~~~
# SOURCE
~~~sysml
package Diamond {
    part def Base {
        part def Member;
    }
    part def Left :> Base;
    part def Right :> Base;
    part def Diamond :> Left, Right {
        part p : Member;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "inherited_diamond_dedup.md"
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
  (root (package (name "Diamond") (body (part-def (name "Base") (body (part-def (name "Member") (body semicolon)))) (part-def (name "Left") (body semicolon)) (part-def (name "Right") (body semicolon)) (part-def (name "Diamond") (body (part-usage))))))
)
~~~
