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
  (root (package (name "ConnectorAll") (body (kerml-connector) (kerml-connector) (kerml-connector) (kerml-connector))))
)
~~~
