# META
~~~sexpr
(snapshot (type semantic) (description "The remaining behavior members of a ConnectionDefinition's DefinitionBody: action, state, calc and constraint definitions, constraint and calc usages, perform and prefixed forms, each followed by a sibling that must survive."))
~~~
# SOURCE
~~~sysml
package ConnectionDefBehaviorDefinitions {
    action def Trip;
    calc def Margin;
    connection def Line {
        end a;
        end b;
        attribute temperature : ScalarValues::Real;
        action def Sense {
            in reading : ScalarValues::Real;
        }
        state def Mode;
        calc def Headroom {
            return : ScalarValues::Real;
        }
        constraint def Limit {
            temperature < 1.0
        }
        constraint thermal {
            temperature < 2.0
        }
        calc headroom {
            1.0 - temperature
        }
        perform action trip : Trip;
        perform Trip;
        abstract action idle;
        private state standby;
        exhibit protection;
        attribute online : ScalarValues::Boolean;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connection_def_body_behavior_definitions.md"
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 245) (line 9) (column 26) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 245) (line 9) (column 26) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 259) (line 9) (column 40) (len 4)))))
    (reference r1 (scope relative) (span (offset 421) (line 16) (column 13) (len 11)) (segments (segment 0 (token "temperature") (name "temperature") (separator none) (span (offset 421) (line 16) (column 13) (len 11)))))
    (reference r2 (scope relative) (span (offset 490) (line 19) (column 13) (len 11)) (segments (segment 0 (token "temperature") (name "temperature") (separator none) (span (offset 490) (line 19) (column 13) (len 11)))))
    (reference r3 (scope relative) (span (offset 612) (line 24) (column 31) (len 4)) (segments (segment 0 (token "Trip") (name "Trip") (separator none) (span (offset 612) (line 24) (column 31) (len 4)))))
    (reference r4 (scope relative) (span (offset 634) (line 25) (column 17) (len 4)) (segments (segment 0 (token "Trip") (name "Trip") (separator none) (span (offset 634) (line 25) (column 17) (len 4)))))
    (reference r5 (scope relative) (span (offset 717) (line 28) (column 17) (len 10)) (segments (segment 0 (token "protection") (name "protection") (separator none) (span (offset 717) (line 28) (column 17) (len 10)))))
  )
  (root (package (visibility none) (name "ConnectionDefBehaviorDefinitions") (body brace (action-def (name "Trip") (modifiers) (specializes none) (body semicolon)) (calc-def (name "Margin") (modifiers) (body semicolon)) (connection-def (name "Line") (modifiers) (extensions) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "a") (span (offset 123) (line 5) (column 13) (len 1)))) (typing none) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "b") (span (offset 138) (line 6) (column 13) (len 1)))) (typing none) (references none) (multiplicity none) (redefines none) (crosses none)) (attribute-usage) (action-def (name "Sense") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "reading") (short-name none) (subsets none) (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 232) (line 9) (column 13) (len 32))))) (state-def (name "Mode") (modifiers) (body semicolon)) (calc-def) (constraint-def (name "Limit") (modifiers) (specializes none) (body brace (expression (span (offset 421) (line 16) (column 13) (len 17)) (binary (operator "<") (left (expression (span (offset 421) (line 16) (column 13) (len 11)) (ref r1))) (right (expression (span (offset 435) (line 16) (column 27) (len 3)) (real "1.0"))))))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "thermal") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (expression (span (offset 490) (line 19) (column 13) (len 17)) (binary (operator "<") (left (expression (span (offset 490) (line 19) (column 13) (len 11)) (ref r2))) (right (expression (span (offset 504) (line 19) (column 27) (len 3)) (real "2.0"))))))) (calc-usage) (perform (target (action (name "trip") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body semicolon)) (perform (target (reference (action (ref r4)) (specialization none))) (value none) (body semicolon)) (action-usage) (state-usage) (exhibit (declaration none) (short-name none) (state (ref r5))) (attribute-usage))))))
)
~~~
