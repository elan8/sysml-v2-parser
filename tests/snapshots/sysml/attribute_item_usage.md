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
    (reference r0 (scope relative) (span (offset 68) (line 3) (column 24) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 68) (line 3) (column 24) (len 7)))))
  )
  (root (package (name "Messaging") (body brace (attribute-def (declaration-name "Show") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "picture") (short-name none) (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (item-def (name "Picture") (modifiers) (individual false) (specializes none) (body semicolon)))))
)
~~~
