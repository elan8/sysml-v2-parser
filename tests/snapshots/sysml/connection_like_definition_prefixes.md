# META
~~~sexpr
(snapshot (type semantic) (description "OccurrenceDefinitionPrefix modifiers are retained and emitted for every connection-like definition kind -- connection, interface, allocation, and flow -- with the authored keyword's exact span, so both BasicDefinitionPrefix alternatives and the unprefixed state are distinguishable (spec42 Gap 58)."))
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
    variation connection def CV;
    variation interface def IV;
    variation allocation def AV;
    variation flow def FV;
    variation individual connection def CVI;
    connection def CP;
    interface def IP;
    allocation def AP;
    flow def FP;
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
  (root (package (name "ConnectionLikeDefinitionPrefixes") (body brace (connection-def (name "C") (modifiers (abstract (span (offset 47) (line 2) (column 5) (len 8)))) (role ordinary) (specializes none) (body semicolon)) (interface-def (name "I") (modifiers (abstract (span (offset 78) (line 3) (column 5) (len 8)))) (specializes none) (body semicolon)) (allocation-def (name "A") (modifiers (abstract (span (offset 108) (line 4) (column 5) (len 8))))) (flow-def (name "F") (modifiers (abstract (span (offset 139) (line 5) (column 5) (len 8))))) (connection-def (name "CI") (modifiers individual) (role ordinary) (specializes none) (body semicolon)) (interface-def (name "II") (modifiers individual) (specializes none) (body semicolon)) (allocation-def (name "AI") (modifiers individual)) (flow-def (name "FI") (modifiers individual)) (connection-def (name "CV") (modifiers (variation (span (offset 293) (line 10) (column 5) (len 9)))) (role ordinary) (specializes none) (body semicolon)) (interface-def (name "IV") (modifiers (variation (span (offset 326) (line 11) (column 5) (len 9)))) (specializes none) (body semicolon)) (allocation-def (name "AV") (modifiers (variation (span (offset 358) (line 12) (column 5) (len 9))))) (flow-def (name "FV") (modifiers (variation (span (offset 391) (line 13) (column 5) (len 9))))) (connection-def (name "CVI") (modifiers (variation (span (offset 418) (line 14) (column 5) (len 9))) individual) (role ordinary) (specializes none) (body semicolon)) (connection-def (name "CP") (modifiers) (role ordinary) (specializes none) (body semicolon)) (interface-def (name "IP") (modifiers) (specializes none) (body semicolon)) (allocation-def (name "AP") (modifiers)) (flow-def (name "FP") (modifiers)))))
)
~~~
