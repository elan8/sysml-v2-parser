# META
~~~sexpr
(snapshot (type semantic) (description "A ConnectionDefinition body is a DefinitionBody, so it owns the same behavior usage members as a part definition body: action, exhibit state, state, constraint, calc and perform usages, plus constraint and calc definitions."))
~~~
# SOURCE
~~~sysml
package ConnectionDefBehaviorMembers {
    part def Bus {
        attribute delta : ScalarValues::Real;
    }
    connection def Line {
        end a : Bus;
        end b : Bus;
        attribute online : ScalarValues::Boolean default = true;
        attribute temperature : ScalarValues::Real default = 0.0;
        action sense { }
        exhibit state protection {
            entry;
            then closed;
            state closed;
            state open {
                entry action trip;
            }
            transition overheat first closed if temperature > 1.0 then open;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connection_def_body_behavior_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ConnectionDefBehaviorMembers {
    part def Bus {
        attribute delta : ScalarValues::Real;
    }
    connection def Line {
        end a : Bus;
        end b : Bus;
        attribute online : ScalarValues::Boolean default = true;
        attribute temperature : ScalarValues::Real default = 0.0;
        action sense {
        }
        exhibit state protection {
            entry;
            then closed;
            state closed;
            state open {
                entry action trip;
            }
            transition overheat first closed if temperature > 1.0 then open;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 84) (line 3) (column 27) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 84) (line 3) (column 27) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 98) (line 3) (column 41) (len 4)))))
    (reference r1 (scope relative) (span (offset 152) (line 6) (column 17) (len 3)) (segments (segment 0 (token "Bus") (name "Bus") (separator none) (span (offset 152) (line 6) (column 17) (len 3)))))
    (reference r2 (scope relative) (span (offset 173) (line 7) (column 17) (len 3)) (segments (segment 0 (token "Bus") (name "Bus") (separator none) (span (offset 173) (line 7) (column 17) (len 3)))))
  )
  (root (package (visibility none) (name "ConnectionDefBehaviorMembers") (body brace (part-def (name "Bus") (modifiers) (body brace (attribute-usage (declaration-name "delta") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connection-def (name "Line") (modifiers) (extensions) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "a") (span (offset 148) (line 6) (column 13) (len 1)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "b") (span (offset 169) (line 7) (column 13) (len 1)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (references none) (multiplicity none) (redefines none) (crosses none)) (attribute-usage) (attribute-usage) (action-usage) (exhibit (declaration "protection") (short-name none) (state none)))))))
)
~~~
