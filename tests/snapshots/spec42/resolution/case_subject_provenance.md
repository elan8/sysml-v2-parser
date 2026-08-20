# META
~~~sexpr
(snapshot (type semantic) (description "Case subject derived relationship retains explicit provenance"))
~~~
# SOURCE
~~~sysml
package M {
    part def P;
    analysis def A {
        subject s : P;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "case_subject_provenance.md"
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
  (root (package (name "M") (body brace (part-def (name "P") (modifiers) (body semicolon)) (analysis-case-def (modifiers)))))
)
~~~
