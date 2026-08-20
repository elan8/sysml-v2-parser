# META
~~~sexpr
(snapshot (type semantic) (description "Occurrence bodies admit BindingConnectorAsUsage through DefinitionBodyItem -> NonOccurrenceUsageMember -> NonOccurrenceUsageElement (SysML textual BNF 237-247, 349-353, and 702-707; pinned Pilot SysML agrees). The typed bind retains its structured connector rather than recovery text."))
~~~
# SOURCE
~~~sysml
package OccurrenceBodyBind {
    occurrence Transfer {
        binding transferBinding : Binding bind [1] source = [0..1] target;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "occurrence_body_bind.md"
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
  (root (package (name "OccurrenceBodyBind") (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "Transfer") (short-name none) (target none) (body brace (bind))))))
)
~~~
