# META
~~~sexpr
(snapshot (type recovery) (description "A nested item usage inside an attribute def body parses as a structured item usage, not opaque text."))
~~~
# SOURCE
~~~sysml
package Messaging {
    attribute def Show {
        item picture : Picture;
    }
    item def Picture;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "attribute_item_usage.md"
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
  (root (package (name "Messaging") (body brace (attribute-def (name "Show") (multiplicity none)) (item-def))))
)
~~~
