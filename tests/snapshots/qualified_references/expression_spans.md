# META
~~~sexpr
(snapshot (type provenance) (description "Verifies nested expression nodes expose exact source spans without tests traversing Rust AST field paths."))
~~~
# SOURCE
~~~sysml
package ExpressionSpans {
    part def Container {
        attribute answer = 100;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "expression_spans.md"
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
  (root (package (name "ExpressionSpans") (body (part-def (name "Container") (body (attribute-usage (declaration-name "answer") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 78) (line 3) (column 28) (len 3)) (integer 100))))) (body semicolon)))))))
)
~~~
