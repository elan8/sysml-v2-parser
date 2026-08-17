# META
~~~sexpr
(snapshot (type semantic) (description "`FlowDefinition = OccurrenceDefinitionPrefix ( 'flow' | 'message' ) 'def' Definition` is a DefinitionElement, so it is legal at package level and in a part definition or part usage body. It parsed into a complete typed node in all three and was reported as an unsupported construct by all three emitters, so a document containing one parsed and could not be formatted. Its body is a DefinitionBody, the same shape an allocation definition writes."))
~~~
# SOURCE
~~~sysml
package FlowDefinitionEmission {
    flow def Signal {
        doc /* a flow definition at package level */
        attribute latency;
    }
    part def Host {
        flow def Nested :> Signal;
    }
    part host {
        flow def Inner;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "flow_definition_emission.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package FlowDefinitionEmission {
    flow def Signal {
        doc
        /* a flow definition at package level */
        attribute latency;
    }
    part def Host {
        flow def Nested :> Signal;
    }
    part host {
        flow def Inner;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "FlowDefinitionEmission") (body brace (flow-def) (part-def (name "Host") (body brace (flow-def))) (part-usage))))
)
~~~
