# META
~~~sexpr
(snapshot (type semantic) (description "The canonical anonymous flow shorthand (flow from a to b;) and its succession flow sibling parse with no declared name instead of silently taking the from keyword as the flow's name, while genuinely named flows keep theirs (spec42 Gap 47)."))
~~~
# SOURCE
~~~sysml
package AnonymousFlowShorthand {
    action def Shoot {
        flow from focus.image to shoot.image;
        succession flow from focus.image to shoot.image;
        succession flow lightFlow from bulb.light to lens.light;
        flow of Exposure from focus.xrsl to shoot.xsf;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "anonymous_flow_shorthand.md"
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
  (root (package (name "AnonymousFlowShorthand") (body (action-def (name "Shoot") (specializes none) (body (flow-usage) (flow-usage) (flow-usage) (flow-usage))))))
)
~~~
