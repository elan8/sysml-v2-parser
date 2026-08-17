# META
~~~sexpr
(snapshot (type semantic) (description "OccurrenceDefinitionPrefix modifiers are retained and emitted for every connection-like definition kind: connection, interface, allocation, and flow."))
~~~
# SOURCE
~~~sysml
package ConnectionLikeDefinitionPrefixes {
    abstract connection def C;
    abstract interface def I;
    abstract allocation def A;
    abstract flow def F;
    individual connection def CI;
    individual interface def II;
    individual allocation def AI;
    individual flow def FI;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connection_like_definition_prefixes.md"
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
  (root (package (name "ConnectionLikeDefinitionPrefixes") (body brace (connection-def (name "C") (modifiers abstract) (role ordinary) (specializes none) (body semicolon)) (interface-def (name "I") (modifiers abstract) (specializes none) (body semicolon)) (allocation-def (name "A") (modifiers abstract)) (flow-def (name "F") (modifiers abstract)) (connection-def (name "CI") (modifiers individual) (role ordinary) (specializes none) (body semicolon)) (interface-def (name "II") (modifiers individual) (specializes none) (body semicolon)) (allocation-def (name "AI") (modifiers individual)) (flow-def (name "FI") (modifiers individual)))))
)
~~~
