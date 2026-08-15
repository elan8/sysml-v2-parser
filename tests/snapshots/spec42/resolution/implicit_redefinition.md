# META
~~~sexpr
(snapshot (type semantic) (description "Model diagnostics for implicit inherited feature redefinition"))
~~~
# SOURCE
~~~sysml
package P {
    part def Base {
        attribute mass : Real;
    }
    part def Child :> Base {
        attribute mass = 1200;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "implicit_redefinition.md"
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
    (reference r0 (scope relative) (span (offset 57) (line 3) (column 26) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 57) (line 3) (column 26) (len 4)))))
  )
  (root (package (name "P") (body brace (part-def (name "Base") (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Child") (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 123) (line 6) (column 26) (len 4)) (integer 1200))))) (body semicolon)))))))
)
~~~
