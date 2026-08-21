# META
~~~sexpr
(snapshot (type semantic) (description "Attribute, calc, event-occurrence, and nested action-def declarations inside action def/usage bodies dispatch through their typed productions (spec42 Gap 33; the opaque ActionBodyDecl fallback is retired), including an event occurrence carrying its multiplicity before the typing."))
~~~
# SOURCE
~~~sysml
package ActionBodyDeclarations {
    action def Dynamics {
        attribute mass = 5;
        calc getNextState : GetNextState {
            in input;
            return result = input;
        }
        event occurrence zeroCrossingEvents[0..*] : ZeroCrossingEventDef {
            doc /* Zero crossings anomaly. */
        }
        action def Nested {
            in signal;
        }
    }
    action mission : Dynamics {
        attribute duration = 10;
        event marker;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "action_body_declarations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ActionBodyDeclarations {
    action def Dynamics {
        attribute mass = 5;
        calc getNextState : GetNextState {
            in input;
            return result = input;
        }
        event occurrence zeroCrossingEvents : ZeroCrossingEventDef[0..*] {
            doc
            /* Zero crossings anomaly. */
        }
        action def Nested {
            in signal;
        }
    }
    action mission : Dynamics {
        attribute duration = 10;
        event marker;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 416) (line 15) (column 22) (len 8)) (segments (segment 0 (token "Dynamics") (name "Dynamics") (separator none) (span (offset 416) (line 15) (column 22) (len 8)))))
    (reference r1 (scope relative) (span (offset 474) (line 17) (column 15) (len 6)) (segments (segment 0 (token "marker") (name "marker") (separator none) (span (offset 474) (line 17) (column 15) (len 6)))))
  )
  (root (package (name "ActionBodyDeclarations") (body brace (action-def (name "Dynamics") (modifiers) (specializes none) (body brace (attribute-usage) (calc-usage) (occurrence-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions))) (action-def))) (action-usage (name "mission") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (attribute-usage (declaration-name "duration") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 456) (line 16) (column 30) (len 2)) (integer 10))))) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "") (short-name none) (target (ref r1)) (body semicolon)))))))
)
~~~
