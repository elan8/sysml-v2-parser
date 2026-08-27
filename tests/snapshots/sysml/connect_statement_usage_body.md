# META
~~~sexpr
(snapshot (type semantic) (description "`ConnectionUsage = OccurrenceUsagePrefix ( ... | 'connect' ConnectorPart ) UsageBody` and `UsageBody = DefinitionBody`, so a connect statement's body holds the whole usage member set. The parser routed it through `relationship_body`, which admits only the annotating subset, so an attribute or a nested part inside `connect a to b { ... }` reached recovery. The body was also a `ConnectBody` marker with no delimiter spans paired with a separate element list -- one body fact in two fields; it is now one shared `Body`."))
~~~
# SOURCE
~~~sysml
package ConnectStatementUsageBody {
    connection def Harness {
        port left;
        port right;
        connect left to right {
            doc /* the annotating subset that already parsed */
            attribute gauge;
            part shield;
            ref carrier : Conduit;
        }
        connect right to left;
        connect left to right {}
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connect_statement_usage_body.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ConnectStatementUsageBody {
    connection def Harness {
        port left;
        port right;
        connect left to right {
            doc
            /* the annotating subset that already parsed */
            attribute gauge;
            part shield;
            ref carrier : Conduit;
        }
        connect right to left;
        connect left to right {}
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 280) (line 9) (column 27) (len 7)) (segments (segment 0 (token "Conduit") (name "Conduit") (separator none) (span (offset 280) (line 9) (column 27) (len 7)))))
  )
  (root (package (name "ConnectStatementUsageBody") (body brace (connection-def (name "Harness") (modifiers) (role ordinary) (specializes none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "left") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "right") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (connect (body brace (doc (name none) (locale none) (body (span (offset 154) (line 6) (column 19) (len 43)) (normalized "the annotating subset that already parsed "))) (attribute-usage (declaration-name "gauge") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "shield") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (ref (name "carrier") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)))) (connect (body semicolon)) (connect (body brace)))))))
)
~~~
