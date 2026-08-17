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
  )
  (root (package (name "ActionBodyDeclarations") (body brace (action-def (name "Dynamics") (specializes none) (body brace (attribute-usage) (calc-usage) (occurrence-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions))) (action-def))) (action-usage (name "mission") (short-name none)))))
)
~~~
