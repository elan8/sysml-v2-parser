# META
~~~sexpr
(snapshot (type semantic) (description "The two members a connection definition body owns that emission never covered. A ConnectionDefinition's body is an ordinary DefinitionBody, so it owns occurrence usages and succession usages like any other; both parsed and neither could be re-emitted, which made this the one legal owning scope of those families with no round trip. Real usage: Systems Library `Domain Libraries/Cause and Effect/CausationConnections.sysml` writes both, an `abstract constant ref occurrence causes[1..*]` and a named succession with multiplicities on each end."))
~~~
# SOURCE
~~~sysml
package ConnectionDefBodyMembers {
    connection def CausationConnection {
        end cause;
        end effect;
        derived constant ref occurrence causes[1..*] {
            attribute startShot;
        }
        occurrence effects[1..*];
        private succession causalOrdering first [1] causes then [1] effects;
        succession unnamed first causes then effects;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connection_def_body_occurrence_and_succession.md"
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
  (root (package (name "ConnectionDefBodyMembers") (body brace (connection-def (name "CausationConnection") (modifiers) (role ordinary) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "cause") (span (offset 88) (line 3) (column 13) (len 5)))) (typing none) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "effect") (span (offset 107) (line 4) (column 13) (len 6)))) (typing none) (references none) (multiplicity none) (redefines none) (crosses none)) (occurrence (prefix (direction none) (derived true) (variance none) (constant true) (reference true) (individual false) (portion none) (extensions)) (declaration "causes") (short-name none) (target none) (body brace (attribute-usage (declaration-name "startShot") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "effects") (short-name none) (target none) (body semicolon)) (succession-usage) (succession-usage))))))
)
~~~
