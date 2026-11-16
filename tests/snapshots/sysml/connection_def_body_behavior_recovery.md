# META
~~~sexpr
(snapshot (type recovery) (description "A malformed behavior member in a connection definition body is reported once and does not consume the following behavior member, attribute or package; `entry`, a StateBodyItem rather than a DefinitionBodyItem, is still rejected in this scope."))
~~~
# SOURCE
~~~sysml
package ConnectionDefBehaviorRecovery {
    connection def Line {
        end a;
        action broken : ;
        state ok;
        exhibit state protection : ;
        entry action trip;
        state tail;
        constraint c { }
        attribute online : ScalarValues::Boolean;
    }
}
package Later;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connection_def_body_behavior_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_connection_def_body_element") (severity error) (category parseerror) (span (offset 89) (line 4) (column 9) (len 26)) (message "unexpected token in connection definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 89) (line 4) (column 9) (len 26)) (message "suppressed 1 cascading recovered diagnostic after earlier recovery errors"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 170) (line 7) (column 9) (len 27)) (message "unexpected keyword `entry` in connection definition body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package ConnectionDefBehaviorRecovery {
    connection def Line {
        end a;
        action broken : ;
        state ok;
        exhibit state protection : ;
        entry action trip;
        state tail;
        constraint c {
        }
        attribute online : ScalarValues::Boolean;
    }
}

package Later;
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
  )
  (root (package (visibility none) (name "ConnectionDefBehaviorRecovery") (body brace (connection-def (name "Line") (modifiers) (extensions) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "a") (span (offset 78) (line 3) (column 13) (len 1)))) (typing none) (references none) (multiplicity none) (redefines none) (crosses none)) (malformed (code "recovered_connection_def_body_element") (found "action broken : ;") (span (offset 89) (line 4) (column 9) (len 26))) (state-usage) (malformed (code "recovered_connection_def_body_element") (found "exhibit state protection : ;") (span (offset 133) (line 6) (column 9) (len 37))) (malformed (code "unexpected_keyword_in_scope") (found "entry action trip;") (span (offset 170) (line 7) (column 9) (len 27))) (state-usage) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace)) (attribute-usage))))) (package (visibility none) (name "Later") (body semicolon)))
)
~~~
