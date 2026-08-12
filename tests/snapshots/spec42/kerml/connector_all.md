# META
~~~sexpr
(snapshot (type semantic) (description "KerML connector with 'all' keyword (stdlib patterns from OccurrenceFunctions/TransitionPerformances)"))
~~~
# SOURCE
~~~sysml
package ConnectorAll {
    connector all during: HappensDuring from self to occ;
    connector all guardConstraint: TPCGuardConstraint[*] from transitionLink to guard;
    connector all x from a to b;
    connector all from a to b;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connector_all.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 27) (line 2) (column 5) (len 205)) (message "unrecognized declaration `connector` in package body"))
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
  (root (package (name "ConnectorAll") (body (malformed (code "unrecognized_declaration_in_scope") (found "connector all during: HappensDuring from self to occ;") (span (offset 27) (line 2) (column 5) (len 205))))))
)
~~~
